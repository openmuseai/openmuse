//! Production admission gate for arbitrary-program sandbox execution.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityProfile {
    pub rootless: bool,
    pub read_only_image: bool,
    pub seccomp_enforced: bool,
    pub capabilities_dropped: bool,
    pub proc_hidepid: bool,
    pub max_cpu_millis: u64,
    pub max_memory_bytes: u64,
    pub max_pids: u32,
    pub max_disk_bytes: u64,
    pub max_runtime_ms: u64,
    pub egress_allowlist: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestedCeiling {
    pub cpu_millis: u64,
    pub memory_bytes: u64,
    pub pids: u32,
    pub disk_bytes: u64,
    pub runtime_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactAttestation {
    pub digest: String,
    pub signature_valid: bool,
    pub sbom_digest: String,
    pub revoked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionAdmissionRequest {
    pub actor_ref: String,
    pub caller_ref: String,
    pub target_ref: String,
    pub artifact: ArtifactAttestation,
    pub draft_ref: String,
    pub expected_revision: String,
    pub requested: RequestedCeiling,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExecutionAuditReceipt {
    pub sequence: u64,
    pub actor_ref: String,
    pub caller_ref: String,
    pub target_ref: String,
    pub artifact_digest: String,
    pub draft_ref: String,
    pub expected_revision: String,
    pub previous_hash: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SecurityError {
    #[error("production profile is incomplete")]
    ProfileIncomplete,
    #[error("artifact is not admitted")]
    ArtifactDenied,
    #[error("requested resource ceiling exceeds the outer sandbox")]
    QuotaExceeded,
    #[error("network egress denied")]
    EgressDenied,
    #[error("filesystem entry denied")]
    PathDenied,
    #[error("archive expansion denied")]
    ArchiveDenied,
    #[error("untrusted output denied")]
    OutputDenied,
}

pub type Result<T> = std::result::Result<T, SecurityError>;

pub struct ProductionSecurityGate {
    profile: SecurityProfile,
    audit: Vec<ExecutionAuditReceipt>,
}

impl ProductionSecurityGate {
    pub fn new(profile: SecurityProfile) -> Result<Self> {
        if !profile.rootless
            || !profile.read_only_image
            || !profile.seccomp_enforced
            || !profile.capabilities_dropped
            || !profile.proc_hidepid
            || profile.max_cpu_millis == 0
            || profile.max_memory_bytes == 0
            || profile.max_pids == 0
            || profile.max_disk_bytes == 0
            || profile.max_runtime_ms == 0
        {
            return Err(SecurityError::ProfileIncomplete);
        }
        Ok(Self {
            profile,
            audit: Vec::new(),
        })
    }

    pub fn admit(&mut self, request: ExecutionAdmissionRequest) -> Result<ExecutionAuditReceipt> {
        if request.actor_ref.is_empty()
            || request.caller_ref.is_empty()
            || request.target_ref.is_empty()
            || request.draft_ref.is_empty()
            || request.expected_revision.is_empty()
            || !valid_digest(&request.artifact.digest)
            || !valid_digest(&request.artifact.sbom_digest)
            || !request.artifact.signature_valid
            || request.artifact.revoked
        {
            return Err(SecurityError::ArtifactDenied);
        }
        if request.requested.cpu_millis > self.profile.max_cpu_millis
            || request.requested.memory_bytes > self.profile.max_memory_bytes
            || request.requested.pids > self.profile.max_pids
            || request.requested.disk_bytes > self.profile.max_disk_bytes
            || request.requested.runtime_ms > self.profile.max_runtime_ms
        {
            return Err(SecurityError::QuotaExceeded);
        }
        let previous_hash = self
            .audit
            .last()
            .map(|receipt| receipt.receipt_hash.clone())
            .unwrap_or_else(|| format!("sha256:{}", "0".repeat(64)));
        let sequence = self.audit.len() as u64 + 1;
        let canonical = serde_json::to_vec(&(
            sequence,
            &request.actor_ref,
            &request.caller_ref,
            &request.target_ref,
            &request.artifact.digest,
            &request.draft_ref,
            &request.expected_revision,
            &previous_hash,
        ))
        .expect("audit tuple is serializable");
        let receipt = ExecutionAuditReceipt {
            sequence,
            actor_ref: request.actor_ref,
            caller_ref: request.caller_ref,
            target_ref: request.target_ref,
            artifact_digest: request.artifact.digest,
            draft_ref: request.draft_ref,
            expected_revision: request.expected_revision,
            previous_hash,
            receipt_hash: format!("sha256:{:x}", Sha256::digest(canonical)),
        };
        self.audit.push(receipt.clone());
        Ok(receipt)
    }

    pub fn authorize_egress(&self, destination: &str) -> Result<()> {
        let url = Url::parse(destination).map_err(|_| SecurityError::EgressDenied)?;
        let host = url.host_str().ok_or(SecurityError::EgressDenied)?;
        if url.scheme() != "https" || !self.profile.egress_allowlist.contains(host) {
            return Err(SecurityError::EgressDenied);
        }
        Ok(())
    }

    pub fn audit(&self) -> &[ExecutionAuditReceipt] {
        &self.audit
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File { hard_links: u64 },
    Directory,
    Symlink,
}

pub fn validate_workspace_entry(path: &str, kind: EntryKind) -> Result<()> {
    if !path.starts_with("/workspace/")
        || path.split('/').any(|part| part == "..")
        || matches!(
            kind,
            EntryKind::Symlink | EntryKind::File { hard_links: 2.. }
        )
    {
        return Err(SecurityError::PathDenied);
    }
    Ok(())
}

pub fn validate_archive(compressed_bytes: u64, expanded_bytes: u64, entries: u64) -> Result<()> {
    if compressed_bytes == 0
        || expanded_bytes > 512 * 1024 * 1024
        || expanded_bytes / compressed_bytes > 100
        || entries > 10_000
    {
        return Err(SecurityError::ArchiveDenied);
    }
    Ok(())
}

pub fn sanitize_child_env(environment: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    environment
        .iter()
        .filter(|(key, _)| {
            let upper = key.to_ascii_uppercase();
            !upper.contains("SECRET")
                && !upper.contains("TOKEN")
                && !upper.contains("PASSWORD")
                && !upper.contains("ACCESS_KEY")
        })
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

pub fn validate_untrusted_output(output: &[u8], max_bytes: usize) -> Result<()> {
    if output.len() > max_bytes
        || output.iter().any(|byte| matches!(byte, 0 | 0x1b))
        || std::str::from_utf8(output).is_err()
    {
        return Err(SecurityError::OutputDenied);
    }
    Ok(())
}

fn valid_digest(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(|hex| {
        hex.len() == 64
            && hex
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    })
}
