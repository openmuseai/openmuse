//! Durable draft transaction, content scan and expected-base checkpoint.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftState {
    Open,
    Quiescent,
    Prepared,
    Committed,
    Conflict,
    Released,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangedEntry {
    pub path: String,
    pub digest: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PreparedDraft {
    pub draft_ref: String,
    pub workspace_ref: String,
    pub expected_base_revision: String,
    pub manifest_digest: String,
    pub changes: Vec<ChangedEntry>,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CheckpointReceipt {
    pub receipt_ref: String,
    pub workspace_ref: String,
    pub before_revision: String,
    pub after_revision: String,
    pub manifest_digest: String,
    pub replayed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum DraftError {
    #[error("draft is not quiescent")]
    NotQuiescent,
    #[error("workspace head changed")]
    Conflict,
    #[error("authority unavailable")]
    Unavailable,
    #[error("draft state does not permit this operation")]
    InvalidState,
    #[error("path is outside the workspace")]
    PathDenied,
}

pub type Result<T> = std::result::Result<T, DraftError>;

pub trait RevisionAuthorityPort: Send + Sync {
    fn commit(&self, prepared: &PreparedDraft) -> Result<CheckpointReceipt>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DraftTransaction {
    draft_ref: String,
    workspace_ref: String,
    expected_base_revision: String,
    base_files: BTreeMap<String, Vec<u8>>,
    overlay_files: BTreeMap<String, Option<Vec<u8>>>,
    watcher_hints: BTreeSet<String>,
    active_writers: u32,
    state: DraftState,
    prepared: Option<PreparedDraft>,
    receipt: Option<CheckpointReceipt>,
}

impl DraftTransaction {
    pub fn new(
        draft_ref: impl Into<String>,
        workspace_ref: impl Into<String>,
        expected_base_revision: impl Into<String>,
        base_files: BTreeMap<String, Vec<u8>>,
    ) -> Self {
        Self {
            draft_ref: draft_ref.into(),
            workspace_ref: workspace_ref.into(),
            expected_base_revision: expected_base_revision.into(),
            base_files,
            overlay_files: BTreeMap::new(),
            watcher_hints: BTreeSet::new(),
            active_writers: 0,
            state: DraftState::Open,
            prepared: None,
            receipt: None,
        }
    }

    pub fn state(&self) -> DraftState {
        self.state
    }

    pub fn write(&mut self, path: &str, content: Vec<u8>, watcher_observed: bool) -> Result<()> {
        self.ensure_open()?;
        let path = normalize(path)?;
        self.overlay_files.insert(path.clone(), Some(content));
        if watcher_observed {
            self.watcher_hints.insert(path);
        }
        Ok(())
    }

    pub fn remove(&mut self, path: &str, watcher_observed: bool) -> Result<()> {
        self.ensure_open()?;
        let path = normalize(path)?;
        self.overlay_files.insert(path.clone(), None);
        if watcher_observed {
            self.watcher_hints.insert(path);
        }
        Ok(())
    }

    pub fn set_active_writers(&mut self, count: u32) {
        self.active_writers = count;
    }

    pub fn quiesce(&mut self, terminate: bool) -> Result<u32> {
        self.ensure_open()?;
        if self.active_writers > 0 && !terminate {
            return Err(DraftError::NotQuiescent);
        }
        let terminated = self.active_writers;
        self.active_writers = 0;
        self.state = DraftState::Quiescent;
        Ok(terminated)
    }

    pub fn prepare(&mut self) -> Result<PreparedDraft> {
        if self.state != DraftState::Quiescent || self.active_writers != 0 {
            return Err(DraftError::NotQuiescent);
        }
        let changes = self.content_scan();
        let manifest_digest = manifest_digest(&changes);
        let prepared = PreparedDraft {
            draft_ref: self.draft_ref.clone(),
            workspace_ref: self.workspace_ref.clone(),
            expected_base_revision: self.expected_base_revision.clone(),
            idempotency_key: format!("checkpoint:{}:{manifest_digest}", self.draft_ref),
            manifest_digest,
            changes,
        };
        self.prepared = Some(prepared.clone());
        self.state = DraftState::Prepared;
        Ok(prepared)
    }

    pub fn checkpoint(
        &mut self,
        authority: &dyn RevisionAuthorityPort,
    ) -> Result<CheckpointReceipt> {
        if let Some(receipt) = &self.receipt {
            return Ok(receipt.clone());
        }
        if self.state != DraftState::Prepared {
            return Err(DraftError::InvalidState);
        }
        let prepared = self.prepared.as_ref().ok_or(DraftError::InvalidState)?;
        match authority.commit(prepared) {
            Ok(receipt) => {
                self.receipt = Some(receipt.clone());
                self.state = DraftState::Committed;
                Ok(receipt)
            }
            Err(DraftError::Conflict) => {
                self.state = DraftState::Conflict;
                Err(DraftError::Conflict)
            }
            Err(error) => Err(error),
        }
    }

    pub fn journal(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("DraftTransaction is serializable")
    }

    pub fn recover(journal: &[u8]) -> std::result::Result<Self, serde_json::Error> {
        serde_json::from_slice(journal)
    }

    fn ensure_open(&self) -> Result<()> {
        if self.state == DraftState::Open {
            Ok(())
        } else {
            Err(DraftError::InvalidState)
        }
    }

    fn content_scan(&self) -> Vec<ChangedEntry> {
        self.overlay_files
            .iter()
            .filter_map(|(path, overlay)| {
                let base = self.base_files.get(path);
                if overlay.as_ref() == base {
                    return None;
                }
                Some(match overlay {
                    Some(content) => ChangedEntry {
                        path: path.clone(),
                        digest: Some(hex_digest(content)),
                        size: Some(content.len() as u64),
                    },
                    None => ChangedEntry {
                        path: path.clone(),
                        digest: None,
                        size: None,
                    },
                })
            })
            .collect()
    }
}

fn normalize(path: &str) -> Result<String> {
    let path = path.strip_prefix("/workspace/").unwrap_or(path);
    if path.is_empty()
        || path.starts_with('/')
        || path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(DraftError::PathDenied);
    }
    Ok(path.into())
}

fn hex_digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn manifest_digest(changes: &[ChangedEntry]) -> String {
    let bytes = serde_json::to_vec(changes).expect("ChangedEntry is serializable");
    hex_digest(&bytes)
}

#[derive(Debug)]
struct AuthorityState {
    heads: HashMap<String, String>,
    receipts: HashMap<String, CheckpointReceipt>,
    next_revision: u64,
    fail_response_once: bool,
}

#[derive(Debug)]
pub struct InMemoryRevisionAuthority {
    state: Mutex<AuthorityState>,
}

impl InMemoryRevisionAuthority {
    pub fn new(workspace_ref: impl Into<String>, head: impl Into<String>) -> Self {
        Self {
            state: Mutex::new(AuthorityState {
                heads: HashMap::from([(workspace_ref.into(), head.into())]),
                receipts: HashMap::new(),
                next_revision: 1,
                fail_response_once: false,
            }),
        }
    }

    pub fn move_head(&self, workspace_ref: &str, revision: &str) {
        self.state
            .lock()
            .unwrap()
            .heads
            .insert(workspace_ref.into(), revision.into());
    }

    pub fn fail_response_once(&self) {
        self.state.lock().unwrap().fail_response_once = true;
    }

    pub fn head(&self, workspace_ref: &str) -> Option<String> {
        self.state.lock().unwrap().heads.get(workspace_ref).cloned()
    }
}

impl RevisionAuthorityPort for InMemoryRevisionAuthority {
    fn commit(&self, prepared: &PreparedDraft) -> Result<CheckpointReceipt> {
        let mut state = self.state.lock().unwrap();
        if let Some(receipt) = state.receipts.get(&prepared.idempotency_key) {
            let mut receipt = receipt.clone();
            receipt.replayed = true;
            return Ok(receipt);
        }
        if state.heads.get(&prepared.workspace_ref) != Some(&prepared.expected_base_revision) {
            return Err(DraftError::Conflict);
        }
        let after_revision = format!("revision:{}", state.next_revision);
        state.next_revision += 1;
        let receipt = CheckpointReceipt {
            receipt_ref: format!("checkpoint-receipt:{}", prepared.idempotency_key),
            workspace_ref: prepared.workspace_ref.clone(),
            before_revision: prepared.expected_base_revision.clone(),
            after_revision: after_revision.clone(),
            manifest_digest: prepared.manifest_digest.clone(),
            replayed: false,
        };
        state
            .heads
            .insert(prepared.workspace_ref.clone(), after_revision);
        state
            .receipts
            .insert(prepared.idempotency_key.clone(), receipt.clone());
        if state.fail_response_once {
            state.fail_response_once = false;
            return Err(DraftError::Unavailable);
        }
        Ok(receipt)
    }
}
