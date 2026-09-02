use muse_host_bridge_contract::{
    ContractError, JsonLimits, NegotiatedProtocol, ProtocolFragment, ProtocolLimits,
    ProtocolSchemas, canonicalize, digest_grant, digest_input, digest_schema, ids::HostSessionId,
};
use serde::Deserialize;
use serde_json::Value;
use std::{fs, path::PathBuf, str::FromStr};

#[derive(Debug, Deserialize)]
struct ConformanceCase {
    name: String,
    valid: bool,
    category: Option<String>,
    negotiated: bool,
    message: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CanonicalCase {
    name: String,
    value: Value,
    canonical: String,
    schema_digest: String,
    input_digest: String,
    grant_digest: String,
}

fn fixture_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("fixtures")
        .join("v1")
        .join(relative)
}

fn read_cases(relative: &str) -> Vec<ConformanceCase> {
    let bytes = fs::read(fixture_path(relative)).expect("fixture should be readable");
    serde_json::from_slice(&bytes).expect("fixture should be valid JSON")
}

fn negotiated() -> NegotiatedProtocol {
    NegotiatedProtocol {
        major: 1,
        minor: 0,
        host_session_id: HostSessionId::from_str("host-session.1").expect("valid fixture ID"),
    }
}

#[test]
fn shared_message_fixtures_have_the_same_verdicts() {
    let schemas = ProtocolSchemas::new().expect("packaged schemas should compile");
    let limits = ProtocolLimits::default();
    let negotiated = negotiated();
    let cases = [
        "valid/messages.json",
        "invalid/messages.json",
        "compatibility/messages.json",
    ]
    .into_iter()
    .flat_map(read_cases);

    for case in cases {
        let bytes = serde_json::to_vec(&case.message).expect("fixture message should encode");
        let result = schemas.decode_message(&bytes, case.negotiated.then_some(&negotiated), limits);
        if case.valid {
            let envelope = result.unwrap_or_else(|error| panic!("{}: {error}", case.name));
            let encoded = schemas
                .encode_message(&envelope, limits)
                .expect("decoded message should re-encode");
            schemas
                .decode_message(&encoded, case.negotiated.then_some(&negotiated), limits)
                .unwrap_or_else(|error| panic!("{} roundtrip: {error}", case.name));
        } else {
            let error = result.unwrap_err();
            assert_eq!(
                Some(error.bridge_code()),
                case.category.as_deref(),
                "{}",
                case.name
            );
        }
    }
}

#[test]
fn shared_canonical_and_digest_fixtures_match() {
    let bytes =
        fs::read(fixture_path("canonical.json")).expect("canonical fixture should be readable");
    let cases: Vec<CanonicalCase> =
        serde_json::from_slice(&bytes).expect("canonical fixture should parse");
    let limits = JsonLimits {
        max_depth: 64,
        max_container_children: 10_000,
    };

    for case in cases {
        assert_eq!(
            String::from_utf8(canonicalize(&case.value, limits).expect("canonicalize"))
                .expect("UTF-8"),
            case.canonical,
            "{}",
            case.name
        );
        assert_eq!(
            digest_schema(&case.value, limits).expect("schema digest"),
            case.schema_digest,
            "{}",
            case.name
        );
        assert_eq!(
            digest_input(&case.value, limits).expect("input digest"),
            case.input_digest,
            "{}",
            case.name
        );
        assert_eq!(
            digest_grant(&case.value, limits).expect("grant digest"),
            case.grant_digest,
            "{}",
            case.name
        );
    }
}

#[test]
fn rust_rejects_negative_zero() {
    let schemas = ProtocolSchemas::new().expect("packaged schemas should compile");
    let error = schemas
        .decode_envelope(br#"-0"#, ProtocolLimits::default())
        .expect_err("negative zero must be rejected");
    assert!(matches!(error, ContractError::Lossless(_)));
}

#[test]
fn public_fragment_validators_reuse_the_wire_contract() {
    let schemas = ProtocolSchemas::new().expect("packaged schemas should compile");
    let cases = read_cases("valid/messages.json");
    let descriptor = serde_json::json!({
        "descriptorId": "descriptor.1", "revision": "revision.1", "familyId": "sample.echo",
        "contractVersion": {"major": 1, "minor": 0}, "providerInstanceId": "provider.1",
        "operations": [{
            "operationId": "sample.echo", "effect": "read",
            "inputSchema": {"sha256": format!("sha256:{}", "1".repeat(64)), "byteLength": 2, "draft": "2020-12", "inline": {}},
            "outputSchema": {"sha256": format!("sha256:{}", "2".repeat(64)), "byteLength": 2, "draft": "2020-12", "inline": {}},
            "cancellable": true, "idempotency": "optional"
        }],
        "events": []
    });
    schemas
        .validate_fragment(ProtocolFragment::DiscoverDescriptor, &descriptor)
        .unwrap();
    let binding = cases
        .iter()
        .find(|case| case.name == "bind response")
        .unwrap();
    schemas
        .validate_fragment(
            ProtocolFragment::Binding,
            &binding.message["payload"]["value"],
        )
        .unwrap();
}
