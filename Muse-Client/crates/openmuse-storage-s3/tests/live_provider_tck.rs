use openmuse_storage_contract::{AddressingStyle, ProviderKind};
use openmuse_storage_credentials::{CredentialLease, SecretBytes};
use openmuse_storage_s3::{
    AwsS3BlobStore, EndpointGuard, EndpointMode, RevalidatingEndpointCheck, S3ProviderConfig,
    SystemEndpointResolver, build_client_from_lease_with_ca_bundle,
};
use openmuse_storage_tck::{TckConfig, run_provider_tck};
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct QualificationReport<'a> {
    generated_at_unix_ms: u128,
    provider_version: String,
    adapter_revision: String,
    adapter_source_digest: String,
    endpoint_mode: EndpointMode,
    addressing_style: AddressingStyle,
    tck: &'a openmuse_storage_tck::TckReport,
}

/// Opt-in live certification. It is ignored by the normal hermetic gate and
/// never prints credentials or a presigned URL.
#[test]
#[ignore = "requires an isolated real S3/MinIO/RustFS bucket"]
fn live_s3_provider_passes_the_same_tck() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Tokio runtime");
    runtime.block_on(async {
        let endpoint = required("OPENMUSE_TCK_S3_ENDPOINT");
        let provider_kind = match required("OPENMUSE_TCK_S3_KIND").as_str() {
            "aws_s3" => ProviderKind::AwsS3,
            "minio" => ProviderKind::Minio,
            "rustfs" => ProviderKind::Rustfs,
            _ => panic!("OPENMUSE_TCK_S3_KIND must be aws_s3, minio, or rustfs"),
        };
        let mode = if endpoint.contains("://localhost") || endpoint.contains("://127.0.0.1") {
            EndpointMode::LoopbackDevelopment
        } else {
            EndpointMode::PublicInternet
        };
        let config = S3ProviderConfig {
            provider_ref: format!("provider.live.{provider_kind:?}"),
            provider_kind,
            endpoint: endpoint.clone(),
            region: required("OPENMUSE_TCK_S3_REGION"),
            bucket: required("OPENMUSE_TCK_S3_BUCKET"),
            prefix: required("OPENMUSE_TCK_S3_PREFIX"),
            addressing_style: AddressingStyle::Path,
            credential_ref: "credential.env.live-tck".to_owned(),
            endpoint_mode: mode,
        };
        let validated = EndpointGuard::new(SystemEndpointResolver)
            .validate(&config)
            .await
            .expect("endpoint policy");
        let lease = CredentialLease {
            credential_ref: config.credential_ref.clone(),
            access_key_id: required("OPENMUSE_TCK_S3_ACCESS_KEY_ID"),
            secret_access_key: SecretBytes::new(
                required("OPENMUSE_TCK_S3_SECRET_ACCESS_KEY").into_bytes(),
            )
            .expect("secret"),
            session_token: std::env::var("OPENMUSE_TCK_S3_SESSION_TOKEN")
                .ok()
                .map(|value| SecretBytes::new(value.into_bytes()).expect("session token")),
            generation: 1,
            expires_at_ms: u64::MAX,
        };
        let ca_bundle = std::env::var("OPENMUSE_TCK_S3_CA_BUNDLE")
            .ok()
            .map(|path| std::fs::read(path).expect("read S3 CA bundle"));
        let client = build_client_from_lease_with_ca_bundle(
            &config,
            &validated,
            &lease,
            0,
            1,
            ca_bundle.as_deref(),
        )
        .expect("S3 client");
        if optional_flag("OPENMUSE_TCK_S3_CREATE_BUCKET") {
            client
                .create_bucket()
                .bucket(&config.bucket)
                .send()
                .await
                .expect("create isolated TCK bucket");
        }
        let check = RevalidatingEndpointCheck::new(
            EndpointGuard::new(SystemEndpointResolver),
            config.clone(),
            validated,
        );
        let provider =
            AwsS3BlobStore::with_endpoint_check(client, config, std::sync::Arc::new(check))
                .expect("S3 adapter");
        let report = run_provider_tck(
            &provider,
            &provider,
            &TckConfig {
                run_ref: format!("live-{}", std::process::id()),
                workspace_ref: "workspace.live-tck".to_owned(),
                generation: 1,
                deadline_at_ms: 4_102_444_800_000,
                run_large_object: true,
            },
        )
        .await;
        if let Ok(path) = std::env::var("OPENMUSE_TCK_REPORT_PATH") {
            write_report(&path, mode, &report);
        }
        assert!(report.certified(), "{:#?}", report.failures);
    });
}

fn required(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

fn optional_flag(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"))
}

fn write_report(path: &str, endpoint_mode: EndpointMode, report: &openmuse_storage_tck::TckReport) {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create TCK report directory");
    }
    let envelope = QualificationReport {
        generated_at_unix_ms: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock")
            .as_millis(),
        provider_version: required("OPENMUSE_TCK_PROVIDER_VERSION"),
        adapter_revision: required("OPENMUSE_TCK_ADAPTER_REVISION"),
        adapter_source_digest: required("OPENMUSE_TCK_ADAPTER_SOURCE_DIGEST"),
        endpoint_mode,
        addressing_style: AddressingStyle::Path,
        tck: report,
    };
    let bytes = serde_json::to_vec_pretty(&envelope).expect("serialize TCK report");
    std::fs::write(path, bytes).expect("write TCK report");
}
