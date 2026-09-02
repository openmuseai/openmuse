use muse_plugin_facets::{FacetSchemaKind, schema_digest, validate};
use serde_json::Value;

fn kind(value: &str) -> FacetSchemaKind {
    match value {
        "plugin-descriptor" => FacetSchemaKind::PluginDescriptor,
        "context-contribution" => FacetSchemaKind::ContextContribution,
        "domain-change" => FacetSchemaKind::DomainChange,
        "presentation-intent" => FacetSchemaKind::PresentationIntent,
        "presentation-intent-result" => FacetSchemaKind::PresentationIntentResult,
        other => panic!("unknown fixture kind {other}"),
    }
}

#[test]
fn rust_matches_shared_fixture_verdicts() {
    let fixtures: Value =
        serde_json::from_str(include_str!("../../fixtures/v1/messages.json")).unwrap();
    for fixture in fixtures.as_array().unwrap() {
        let expected = fixture["valid"].as_bool().unwrap();
        let actual = validate(kind(fixture["kind"].as_str().unwrap()), &fixture["value"]).is_ok();
        assert_eq!(actual, expected, "fixture {}", fixture["name"]);
    }
}

#[test]
fn all_public_schema_digests_are_unique_and_stable_shaped() {
    let kinds = [
        FacetSchemaKind::PluginDescriptor,
        FacetSchemaKind::ContextContribution,
        FacetSchemaKind::DomainChange,
        FacetSchemaKind::PresentationIntent,
        FacetSchemaKind::PresentationIntentResult,
    ];
    let mut digests = std::collections::BTreeSet::new();
    let expected: Value =
        serde_json::from_str(include_str!("../../fixtures/v1/schema-digests.json")).unwrap();
    for kind in kinds {
        let digest = schema_digest(kind).unwrap();
        assert!(digest.starts_with("sha256:"));
        assert_eq!(digest.len(), 71);
        assert!(digests.insert(digest));
    }
    assert_eq!(
        schema_digest(FacetSchemaKind::PluginDescriptor).unwrap(),
        expected["plugin-descriptor"]
    );
    assert_eq!(
        schema_digest(FacetSchemaKind::ContextContribution).unwrap(),
        expected["context-contribution"]
    );
    assert_eq!(
        schema_digest(FacetSchemaKind::DomainChange).unwrap(),
        expected["domain-change"]
    );
    assert_eq!(
        schema_digest(FacetSchemaKind::PresentationIntent).unwrap(),
        expected["presentation-intent"]
    );
    assert_eq!(
        schema_digest(FacetSchemaKind::PresentationIntentResult).unwrap(),
        expected["presentation-intent-result"]
    );
}
