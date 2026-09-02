use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentContentV2 {
  pub media_type: String,
  pub text: String,
  pub truncated: bool,
  pub byte_length: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSnapshotV2 {
  pub protocol: String,
  pub resource_ref: String,
  pub revision: String,
  pub content: DocumentContentV2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DocumentOriginV2 { Ui, Agent, Collaboration, Recovery }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentChangeV2 { pub kind: String }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentCommitEventV2 {
  pub protocol: String,
  pub event_id: String,
  pub cursor: String,
  pub resource_ref: String,
  pub revision: String,
  pub command_ref: String,
  pub origin: DocumentOriginV2,
  pub change: DocumentChangeV2,
  pub occurred_at: u64,
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn rust_round_trips_shared_v2_fixture() {
    let raw = include_str!("../../fixtures/v2/roundtrip.json");
    let value: serde_json::Value = serde_json::from_str(raw).unwrap();
    let snapshot: DocumentSnapshotV2 = serde_json::from_value(value["snapshot"].clone()).unwrap();
    let event: DocumentCommitEventV2 = serde_json::from_value(value["event"].clone()).unwrap();
    assert_eq!(serde_json::to_value(snapshot).unwrap(), value["snapshot"]);
    assert_eq!(serde_json::to_value(event).unwrap(), value["event"]);
  }
}
