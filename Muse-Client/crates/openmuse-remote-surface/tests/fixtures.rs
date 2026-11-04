use openmuse_remote_surface::validate_message;
use serde_json::Value;
use std::path::PathBuf;

#[test]
fn fixtures_match_the_closed_loop_contract() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../contracts/openmuse-remote-surface/v1/messages.json");
    let fixtures: Vec<Value> =
        serde_json::from_str(&std::fs::read_to_string(path).expect("fixture file")).expect("json");
    assert_eq!(fixtures.len(), 18);
    for fixture in fixtures {
        let name = fixture["name"].as_str().expect("name");
        let kind = fixture["kind"].as_str().expect("kind");
        let valid = fixture["valid"].as_bool().expect("valid");
        let result = validate_message(kind, &fixture["value"]);
        assert_eq!(result.is_ok(), valid, "{name}: {result:?}");
    }
}
