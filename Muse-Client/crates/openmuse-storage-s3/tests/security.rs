use async_trait::async_trait;
use futures::executor::block_on;
use openmuse_storage_contract::{AddressingStyle, ProviderKind, StorageErrorCode, StorageResult};
use openmuse_storage_credentials::{CredentialVaultPort, InMemoryCredentialVault, SecretBytes};
use openmuse_storage_s3::{
    EndpointGuard, EndpointMode, EndpointResolver, S3ProviderConfig, build_client_from_lease,
    redact_url,
};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct FakeResolver {
    values: Arc<Mutex<Vec<IpAddr>>>,
}

#[async_trait]
impl EndpointResolver for FakeResolver {
    async fn resolve(&self, _host: &str, _port: u16) -> StorageResult<Vec<IpAddr>> {
        Ok(self.values.lock().unwrap().clone())
    }
}

fn config(endpoint: &str, mode: EndpointMode) -> S3ProviderConfig {
    S3ProviderConfig {
        provider_ref: "provider.byos".to_owned(),
        provider_kind: ProviderKind::OtherS3,
        endpoint: endpoint.to_owned(),
        region: "us-east-1".to_owned(),
        bucket: "openmuse".to_owned(),
        prefix: "tenant-a".to_owned(),
        addressing_style: AddressingStyle::Path,
        credential_ref: "credential.byos".to_owned(),
        endpoint_mode: mode,
    }
}

#[test]
fn public_endpoint_requires_https_and_public_dns() {
    block_on(async {
        let values = Arc::new(Mutex::new(vec![IpAddr::V4(Ipv4Addr::new(
            93, 184, 216, 34,
        ))]));
        let guard = EndpointGuard::new(FakeResolver {
            values: values.clone(),
        });
        let approved = guard
            .validate(&config(
                "https://s3.example.test",
                EndpointMode::PublicInternet,
            ))
            .await
            .unwrap();
        assert_eq!(approved.addresses.len(), 1);

        *values.lock().unwrap() = vec![IpAddr::V4(Ipv4Addr::new(169, 254, 169, 254))];
        assert_eq!(
            guard
                .validate(&config(
                    "https://s3.example.test",
                    EndpointMode::PublicInternet
                ))
                .await
                .unwrap_err()
                .code,
            StorageErrorCode::Denied
        );
        assert_eq!(
            guard
                .revalidate(
                    &config("https://s3.example.test", EndpointMode::PublicInternet),
                    &approved,
                )
                .await
                .unwrap_err()
                .code,
            StorageErrorCode::Denied
        );
        assert_eq!(
            config("http://s3.example.test", EndpointMode::PublicInternet)
                .validate_static()
                .unwrap_err()
                .code,
            StorageErrorCode::Denied
        );
    });
}

#[test]
fn development_mode_is_loopback_only_and_keys_are_prefixed() {
    block_on(async {
        let guard = EndpointGuard::new(FakeResolver {
            values: Arc::new(Mutex::new(vec![IpAddr::V4(Ipv4Addr::LOCALHOST)])),
        });
        let config = config("http://localhost:9000", EndpointMode::LoopbackDevelopment);
        guard.validate(&config).await.unwrap();
        assert_eq!(
            config.object_key("v1/blob"),
            Ok("tenant-a/v1/blob".to_owned())
        );
        assert!(config.object_key("../escape").is_err());
    });
}

#[test]
fn signed_url_logging_is_redacted() {
    let redacted = redact_url(
        "https://access:secret@s3.example.test/bucket/key?X-Amz-Credential=secret&X-Amz-Signature=abc#fragment",
    );
    assert_eq!(redacted, "https://s3.example.test/bucket/key");
    assert!(!redacted.contains("secret"));
    assert!(!redacted.contains("Signature"));
}

#[test]
fn client_requires_a_live_expected_credential_generation() {
    block_on(async {
        let vault = InMemoryCredentialVault::default();
        vault
            .rotate(
                "credential.byos",
                "adapter.s3",
                "access",
                SecretBytes::new(b"secret".to_vec()).unwrap(),
                None,
            )
            .unwrap();
        let lease = vault
            .lease("credential.byos", "adapter.s3", 1000, 100)
            .await
            .unwrap();
        let config = config("http://localhost:9000", EndpointMode::LoopbackDevelopment);
        let endpoint = EndpointGuard::new(FakeResolver {
            values: Arc::new(Mutex::new(vec![IpAddr::V4(Ipv4Addr::LOCALHOST)])),
        })
        .validate(&config)
        .await
        .unwrap();
        assert!(build_client_from_lease(&config, &endpoint, &lease, 1001, 1).is_ok());
        let mut forged_endpoint = endpoint.clone();
        forged_endpoint.resolution_digest = "forged".to_owned();
        assert_eq!(
            build_client_from_lease(&config, &forged_endpoint, &lease, 1001, 1)
                .unwrap_err()
                .code,
            StorageErrorCode::Denied
        );
        assert_eq!(
            build_client_from_lease(&config, &endpoint, &lease, 1001, 2)
                .unwrap_err()
                .code,
            StorageErrorCode::StaleGeneration
        );
    });
}
