use openmuse_contract::{
    ContractErrorCode, ContractViolation, LifecycleSnapshot, RequestEnvelope, ResponseEnvelope,
};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas/fixtures/contract/v1")
}

fn fixture(name: &str) -> (Vec<u8>, Value) {
    let bytes = fs::read(fixture_root().join(name)).expect("fixture must exist");
    let value = serde_json::from_slice(&bytes).expect("fixture must be JSON");
    (bytes, value)
}

#[test]
fn shared_success_and_error_envelopes_round_trip() {
    for name in [
        "request.success.json",
        "response.success.json",
        "response.denied.json",
        "response.expired.json",
        "response.conflict.json",
    ] {
        let (bytes, expected) = fixture(name);
        let actual = if name.starts_with("request.") {
            serde_json::to_value(
                RequestEnvelope::from_json_slice(&bytes, Some(3)).expect("valid request fixture"),
            )
            .unwrap()
        } else {
            serde_json::to_value(
                ResponseEnvelope::from_json_slice(&bytes, Some(3)).expect("valid response fixture"),
            )
            .unwrap()
        };
        assert_eq!(actual, expected, "{name}");
    }
}

#[test]
fn stale_generation_fails_closed() {
    let (bytes, _) = fixture("response.stale-generation.json");
    assert!(matches!(
        ResponseEnvelope::from_json_slice(&bytes, Some(3)),
        Err(ContractViolation::StaleGeneration {
            expected: 3,
            actual: 2
        })
    ));
}

#[test]
fn unknown_major_fails_closed() {
    let (bytes, _) = fixture("request.unknown-major.json");
    assert!(matches!(
        RequestEnvelope::from_json_slice(&bytes, Some(3)),
        Err(ContractViolation::IncompatibleProtocol {
            actual_major: 2,
            actual_minor: 0
        })
    ));
}

#[test]
fn lifecycle_values_round_trip() {
    let (bytes, expected) = fixture("lifecycle.snapshot.json");
    let snapshot: LifecycleSnapshot = serde_json::from_slice(&bytes).unwrap();
    snapshot.validate().unwrap();
    assert_eq!(serde_json::to_value(snapshot).unwrap(), expected);
}

#[test]
fn deadline_and_error_vocabulary_are_frozen() {
    let (bytes, _) = fixture("request.success.json");
    let request = RequestEnvelope::from_json_slice(&bytes, None).unwrap();
    assert!(matches!(
        request.ensure_live_at(request.deadline_at_ms),
        Err(ContractViolation::Expired { .. })
    ));

    let actual = [
        ContractErrorCode::Denied,
        ContractErrorCode::NotFound,
        ContractErrorCode::Conflict,
        ContractErrorCode::Expired,
        ContractErrorCode::StaleGeneration,
        ContractErrorCode::Unavailable,
        ContractErrorCode::Transient,
        ContractErrorCode::IntegrityFailed,
    ]
    .map(|code| serde_json::to_value(code).unwrap());
    assert_eq!(
        actual,
        [
            "DENIED",
            "NOT_FOUND",
            "CONFLICT",
            "EXPIRED",
            "STALE_GENERATION",
            "UNAVAILABLE",
            "TRANSIENT",
            "INTEGRITY_FAILED",
        ]
        .map(Value::from)
    );
}
