//! Security boundary and AWS-SDK-backed implementation for S3-compatible storage.

use async_trait::async_trait;
use openmuse_storage_contract::{
    AddressingStyle, ProviderKind, StorageError, StorageErrorCode, StorageResult,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fmt;
use std::net::IpAddr;
use url::Url;

mod adapter;

pub use adapter::{
    AwsS3BlobStore, OperationEndpointCheck, PresignedPutRequest, RevalidatingEndpointCheck,
};

#[derive(Clone)]
pub struct SensitiveUrl(String);

impl SensitiveUrl {
    pub fn expose(&self) -> &str {
        &self.0
    }

    fn new(value: String) -> Self {
        Self(value)
    }
}

impl fmt::Debug for SensitiveUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SensitiveUrl([REDACTED])")
    }
}

pub fn build_client_from_lease(
    config: &S3ProviderConfig,
    endpoint: &ValidatedEndpoint,
    lease: &openmuse_storage_credentials::CredentialLease,
    now_ms: u64,
    expected_generation: u64,
) -> StorageResult<aws_sdk_s3::Client> {
    build_client_from_lease_with_ca_bundle(
        config,
        endpoint,
        lease,
        now_ms,
        expected_generation,
        None,
    )
}

pub fn build_client_from_lease_with_ca_bundle(
    config: &S3ProviderConfig,
    endpoint: &ValidatedEndpoint,
    lease: &openmuse_storage_credentials::CredentialLease,
    now_ms: u64,
    expected_generation: u64,
    ca_bundle_pem: Option<&[u8]>,
) -> StorageResult<aws_sdk_s3::Client> {
    use aws_sdk_s3::config::{Credentials, Region};
    lease.ensure_live(now_ms, expected_generation)?;
    let url = config.validate_static()?;
    endpoint.ensure_matches(config, &url)?;
    let secret = std::str::from_utf8(lease.secret_access_key.expose())
        .map_err(|_| denied("S3 secret access key is not UTF-8"))?;
    let token = lease
        .session_token
        .as_ref()
        .map(|value| std::str::from_utf8(value.expose()))
        .transpose()
        .map_err(|_| denied("S3 session token is not UTF-8"))?;
    let credentials = Credentials::new(
        lease.access_key_id.clone(),
        secret,
        token.map(str::to_owned),
        None,
        "openmuse-credential-lease",
    );
    let mut sdk = aws_sdk_s3::Config::builder()
        .behavior_version_latest()
        .credentials_provider(credentials)
        .region(Region::new(config.region.clone()))
        .endpoint_url(&config.endpoint)
        .force_path_style(matches!(config.addressing_style, AddressingStyle::Path));
    use aws_smithy_http_client::{
        Builder,
        tls::{Provider, TlsContext, TrustStore, rustls_provider::CryptoMode},
    };
    let mut trust = TrustStore::default();
    if let Some(ca_bundle_pem) = ca_bundle_pem {
        if ca_bundle_pem.is_empty() {
            return Err(denied("S3 CA bundle is empty"));
        }
        trust = trust.with_pem_certificate(ca_bundle_pem);
    }
    let context = TlsContext::builder()
        .with_trust_store(trust)
        .build()
        .map_err(|_| denied("S3 CA bundle is invalid"))?;
    let http_client = Builder::new()
        .tls_provider(Provider::Rustls(CryptoMode::AwsLc))
        .tls_context(context)
        .build_with_resolver(PinnedDnsResolver {
            host: url.host_str().expect("validated host").to_owned(),
            addresses: endpoint.addresses.clone(),
        });
    sdk = sdk.http_client(http_client);
    Ok(aws_sdk_s3::Client::from_conf(sdk.build()))
}

#[derive(Debug, Clone)]
struct PinnedDnsResolver {
    host: String,
    addresses: Vec<IpAddr>,
}

impl aws_smithy_runtime_api::client::dns::ResolveDns for PinnedDnsResolver {
    fn resolve_dns<'a>(
        &'a self,
        name: &'a str,
    ) -> aws_smithy_runtime_api::client::dns::DnsFuture<'a> {
        let addresses = if name.eq_ignore_ascii_case(&self.host) {
            self.addresses.clone()
        } else {
            Vec::new()
        };
        aws_smithy_runtime_api::client::dns::DnsFuture::ready(Ok(addresses))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointMode {
    PublicInternet,
    LoopbackDevelopment,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct S3ProviderConfig {
    pub provider_ref: String,
    pub provider_kind: ProviderKind,
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub prefix: String,
    pub addressing_style: AddressingStyle,
    pub credential_ref: String,
    pub endpoint_mode: EndpointMode,
}

impl S3ProviderConfig {
    pub fn validate_static(&self) -> StorageResult<Url> {
        for value in [
            self.provider_ref.as_str(),
            self.endpoint.as_str(),
            self.region.as_str(),
            self.bucket.as_str(),
            self.credential_ref.as_str(),
        ] {
            if value.is_empty() {
                return Err(denied("S3 provider config is incomplete"));
            }
        }
        if self.prefix.starts_with('/') || self.prefix.contains("..") {
            return Err(denied("S3 provider prefix is not canonical"));
        }
        let url =
            Url::parse(&self.endpoint).map_err(|_| denied("S3 endpoint is not a valid URL"))?;
        if !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
        {
            return Err(denied(
                "S3 endpoint must not embed userinfo, query, or fragment",
            ));
        }
        if url.path() != "/" && !url.path().is_empty() {
            return Err(denied("S3 endpoint must not contain a path"));
        }
        match self.endpoint_mode {
            EndpointMode::PublicInternet if url.scheme() != "https" => {
                return Err(denied("public S3 endpoints require HTTPS"));
            }
            EndpointMode::LoopbackDevelopment if !matches!(url.scheme(), "http" | "https") => {
                return Err(denied("development S3 endpoint scheme is invalid"));
            }
            _ => {}
        }
        if url.host_str().is_none() {
            return Err(denied("S3 endpoint has no host"));
        }
        Ok(url)
    }

    pub fn object_key(&self, object_ref: &str) -> StorageResult<String> {
        if object_ref.is_empty()
            || object_ref.starts_with('/')
            || object_ref.split('/').any(|part| part == "..")
        {
            return Err(denied("object ref is not canonical"));
        }
        if self.prefix.is_empty() {
            Ok(object_ref.to_owned())
        } else {
            Ok(format!(
                "{}/{}",
                self.prefix.trim_end_matches('/'),
                object_ref
            ))
        }
    }
}

#[async_trait]
pub trait EndpointResolver: Send + Sync {
    async fn resolve(&self, host: &str, port: u16) -> StorageResult<Vec<IpAddr>>;
}

pub struct SystemEndpointResolver;

#[async_trait]
impl EndpointResolver for SystemEndpointResolver {
    async fn resolve(&self, host: &str, port: u16) -> StorageResult<Vec<IpAddr>> {
        let addresses = tokio::net::lookup_host((host, port))
            .await
            .map_err(|_| unavailable("S3 endpoint DNS resolution failed"))?;
        Ok(addresses.map(|value| value.ip()).collect())
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValidatedEndpoint {
    pub authority: String,
    pub addresses: Vec<IpAddr>,
    pub resolution_digest: String,
}

impl ValidatedEndpoint {
    fn ensure_matches(&self, config: &S3ProviderConfig, url: &Url) -> StorageResult<()> {
        validate_addresses(config.endpoint_mode, &self.addresses)?;
        let host = url
            .host_str()
            .ok_or_else(|| denied("S3 endpoint has no host"))?;
        let port = url
            .port_or_known_default()
            .ok_or_else(|| denied("S3 endpoint has no port"))?;
        if *self != validated_endpoint(host, port, self.addresses.clone()) {
            return Err(denied("validated S3 endpoint does not match config"));
        }
        Ok(())
    }
}

impl fmt::Debug for ValidatedEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ValidatedEndpoint")
            .field("authority", &self.authority)
            .field("addresses", &self.addresses)
            .field("resolution_digest", &self.resolution_digest)
            .finish()
    }
}

pub struct EndpointGuard<R> {
    resolver: R,
}

impl<R> EndpointGuard<R>
where
    R: EndpointResolver,
{
    pub fn new(resolver: R) -> Self {
        Self { resolver }
    }

    pub async fn validate(&self, config: &S3ProviderConfig) -> StorageResult<ValidatedEndpoint> {
        let url = config.validate_static()?;
        let host = url
            .host_str()
            .ok_or_else(|| denied("S3 endpoint has no host"))?;
        let port = url
            .port_or_known_default()
            .ok_or_else(|| denied("S3 endpoint has no port"))?;
        let addresses = self.resolver.resolve(host, port).await?;
        validate_addresses(config.endpoint_mode, &addresses)?;
        Ok(validated_endpoint(host, port, addresses))
    }

    /// Re-resolves before an operation and rejects a changed address set.
    /// ST1's HTTP connector must additionally pin this set for the request to
    /// close the DNS-rebinding time-of-check/time-of-use gap.
    pub async fn revalidate(
        &self,
        config: &S3ProviderConfig,
        previous: &ValidatedEndpoint,
    ) -> StorageResult<ValidatedEndpoint> {
        let current = self.validate(config).await?;
        if current.authority != previous.authority
            || current.resolution_digest != previous.resolution_digest
        {
            return Err(denied("S3 endpoint DNS binding changed"));
        }
        Ok(current)
    }
}

pub fn redact_url(value: &str) -> String {
    let Ok(mut url) = Url::parse(value) else {
        return "[REDACTED_URL]".to_owned();
    };
    let _ = url.set_username("");
    let _ = url.set_password(None);
    url.set_query(None);
    url.set_fragment(None);
    url.to_string()
}

fn validate_addresses(mode: EndpointMode, values: &[IpAddr]) -> StorageResult<()> {
    if values.is_empty() {
        return Err(unavailable("S3 endpoint DNS returned no addresses"));
    }
    for value in values {
        let allowed = match mode {
            EndpointMode::PublicInternet => is_public(*value),
            EndpointMode::LoopbackDevelopment => value.is_loopback(),
        };
        if !allowed {
            return Err(denied("S3 endpoint resolved to a forbidden address"));
        }
    }
    Ok(())
}

fn is_public(value: IpAddr) -> bool {
    match value {
        IpAddr::V4(value) => {
            let raw = u32::from(value);
            ![
                (0x0000_0000, 8),
                (0x0a00_0000, 8),
                (0x6440_0000, 10),
                (0x7f00_0000, 8),
                (0xa9fe_0000, 16),
                (0xac10_0000, 12),
                (0xc000_0000, 24),
                (0xc000_0200, 24),
                (0xc0a8_0000, 16),
                (0xc612_0000, 15),
                (0xc633_6400, 24),
                (0xcb00_7100, 24),
                (0xe000_0000, 4),
                (0xf000_0000, 4),
            ]
            .iter()
            .any(|(network, prefix)| in_ipv4_network(raw, *network, *prefix))
        }
        IpAddr::V6(value) => {
            let segments = value.segments();
            (segments[0] & 0xe000) == 0x2000
                && !(segments[0] == 0x2001 && (segments[1] == 0x0db8 || segments[1] == 0x0002))
        }
    }
}

fn in_ipv4_network(value: u32, network: u32, prefix: u32) -> bool {
    let mask = u32::MAX << (32 - prefix);
    value & mask == network & mask
}

fn validated_endpoint(host: &str, port: u16, addresses: Vec<IpAddr>) -> ValidatedEndpoint {
    let ordered = addresses.into_iter().collect::<BTreeSet<_>>();
    let joined = ordered
        .iter()
        .map(IpAddr::to_string)
        .collect::<Vec<_>>()
        .join(",");
    ValidatedEndpoint {
        authority: format!("{host}:{port}"),
        addresses: ordered.into_iter().collect(),
        resolution_digest: format!("{:x}", Sha256::digest(joined.as_bytes())),
    }
}

fn denied(message: &'static str) -> StorageError {
    StorageError::new(StorageErrorCode::Denied, message, false)
}

fn unavailable(message: &'static str) -> StorageError {
    StorageError::new(StorageErrorCode::Unavailable, message, true)
}

#[cfg(test)]
mod tests {
    use super::SensitiveUrl;

    #[test]
    fn sensitive_url_debug_never_exposes_the_value() {
        let value = SensitiveUrl::new(
            "https://s3.example.test/object?X-Amz-Signature=top-secret".to_owned(),
        );

        let rendered = format!("{value:?}");
        assert_eq!(rendered, "SensitiveUrl([REDACTED])");
        assert!(!rendered.contains("top-secret"));
    }
}
