use futures::executor::block_on;
use openmuse_storage_tck::{InMemoryBlobStore, TckConfig, run_provider_tck};

#[test]
fn in_memory_provider_passes_the_complete_profile() {
    let provider = InMemoryBlobStore::new("provider.fake.primary");
    let report = block_on(run_provider_tck(
        &provider,
        &provider,
        &TckConfig {
            run_ref: "fake.complete".to_owned(),
            workspace_ref: "workspace.tck".to_owned(),
            generation: 1,
            deadline_at_ms: 4_102_444_800_000,
            run_large_object: true,
        },
    ));
    assert!(report.certified(), "{:#?}", report.failures);
    assert!(report.passed.contains(&"multipart-100-mib-plus".to_owned()));
    assert_eq!(report.passed.len(), 9);

    let json = serde_json::to_value(&report).expect("capability report serializes");
    assert_eq!(
        json["capabilitySnapshot"]["profile"],
        "openmuse.s3-data-plane"
    );
    let wire = serde_json::to_string(&json).expect("capability report JSON");
    for forbidden in ["endpoint", "credential", "aws_sdk_s3", "ByteStream"] {
        assert!(!wire.contains(forbidden), "report leaked {forbidden}");
    }
}
