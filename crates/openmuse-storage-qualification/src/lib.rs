//! Fail-closed qualification policy for managed object-storage rollout.
//!
//! This crate evaluates evidence. It deliberately cannot manufacture elapsed
//! soak time or turn a compatibility TCK into production qualification.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const REQUIRED_SOAK_HOURS: u64 = 90 * 24;
pub const MINIMUM_SHADOW_READS: u64 = 1_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillKind {
    NodeLoss,
    DriveLoss,
    NetworkPartition,
    DiskFull,
    Recovery,
    RollingUpgrade,
    UpgradeRollback,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DrillEvidence {
    pub kind: DrillKind,
    pub passed: bool,
    pub recovery_point_loss_seconds: u64,
    pub recovery_time_seconds: u64,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FallbackProviderEvidence {
    pub provider_ref: String,
    pub profile_tck_passed: bool,
    pub migration_rehearsed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RustfsQualificationEvidence {
    pub schema: String,
    pub recorded_at_ms: u64,
    pub provider_version: String,
    pub image_digest: String,
    pub artifact_signature_verified: bool,
    pub sbom_digest: String,
    pub security_critical_open: u32,
    pub security_high_open: u32,
    pub cluster_nodes: u16,
    pub drives_per_node: u16,
    pub rpo_objective_seconds: u64,
    pub rto_objective_seconds: u64,
    pub drills: Vec<DrillEvidence>,
    pub production_equivalent_soak_hours: u64,
    pub digest_scrub_objects: u64,
    pub digest_mismatches: u64,
    pub canary_tenants: u32,
    pub shadow_reads: u64,
    pub shadow_read_mismatches: u64,
    pub canary_rollback_rehearsed: bool,
    pub fallback_providers: Vec<FallbackProviderEvidence>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationStatus {
    Eligible,
    BlockedBySoak,
    Ineligible,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QualificationDecision {
    pub status: QualificationStatus,
    pub reasons: Vec<String>,
}

pub fn evaluate(evidence: &RustfsQualificationEvidence) -> QualificationDecision {
    let mut blockers = Vec::new();
    if evidence.schema != "openmuse.rustfs-qualification@1" {
        blockers.push("unsupported qualification evidence schema".to_owned());
    }
    if evidence.provider_version.trim().is_empty() {
        blockers.push("provider version is not pinned".to_owned());
    }
    if !is_sha256(&evidence.image_digest) {
        blockers.push("container image digest is missing or invalid".to_owned());
    }
    if !evidence.artifact_signature_verified {
        blockers.push("artifact signature has not been verified".to_owned());
    }
    if !is_sha256(&evidence.sbom_digest) {
        blockers.push("SBOM digest is missing or invalid".to_owned());
    }
    if evidence.security_critical_open > 0 || evidence.security_high_open > 0 {
        blockers.push("high or critical security findings remain open".to_owned());
    }
    if evidence.cluster_nodes < 4 || evidence.drives_per_node < 2 {
        blockers
            .push("qualification topology is below four nodes and two drives per node".to_owned());
    }

    let required = BTreeSet::from([
        DrillKind::NodeLoss,
        DrillKind::DriveLoss,
        DrillKind::NetworkPartition,
        DrillKind::DiskFull,
        DrillKind::Recovery,
        DrillKind::RollingUpgrade,
        DrillKind::UpgradeRollback,
    ]);
    let passed: BTreeSet<_> = evidence
        .drills
        .iter()
        .filter(|drill| {
            drill.passed
                && !drill.evidence_ref.is_empty()
                && drill.recovery_point_loss_seconds <= evidence.rpo_objective_seconds
                && drill.recovery_time_seconds <= evidence.rto_objective_seconds
        })
        .map(|drill| drill.kind)
        .collect();
    if !required.is_subset(&passed) {
        blockers.push(
            "required failure, recovery, upgrade and rollback drills are incomplete".to_owned(),
        );
    }
    if evidence.digest_scrub_objects == 0 || evidence.digest_mismatches > 0 {
        blockers.push("digest scrub is missing or contains mismatches".to_owned());
    }
    if evidence.canary_tenants == 0
        || evidence.shadow_reads < MINIMUM_SHADOW_READS
        || evidence.shadow_read_mismatches > 0
        || !evidence.canary_rollback_rehearsed
    {
        blockers
            .push("shadow-read/canary/rollback evidence is incomplete or mismatched".to_owned());
    }
    let fallback_ready = evidence
        .fallback_providers
        .iter()
        .filter(|provider| provider.profile_tck_passed && provider.migration_rehearsed)
        .count();
    if fallback_ready < 2 {
        blockers.push("at least two tested fallback providers are required".to_owned());
    }

    if !blockers.is_empty() {
        return QualificationDecision {
            status: QualificationStatus::Ineligible,
            reasons: blockers,
        };
    }
    if evidence.production_equivalent_soak_hours < REQUIRED_SOAK_HOURS {
        return QualificationDecision {
            status: QualificationStatus::BlockedBySoak,
            reasons: vec![format!(
                "production-equivalent soak is {}h; {}h is required",
                evidence.production_equivalent_soak_hours, REQUIRED_SOAK_HOURS
            )],
        };
    }
    QualificationDecision {
        status: QualificationStatus::Eligible,
        reasons: Vec::new(),
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn passing() -> RustfsQualificationEvidence {
        RustfsQualificationEvidence {
            schema: "openmuse.rustfs-qualification@1".to_owned(),
            recorded_at_ms: 1,
            provider_version: "1.0.0".to_owned(),
            image_digest: "a".repeat(64),
            artifact_signature_verified: true,
            sbom_digest: "b".repeat(64),
            security_critical_open: 0,
            security_high_open: 0,
            cluster_nodes: 4,
            drives_per_node: 2,
            rpo_objective_seconds: 60,
            rto_objective_seconds: 600,
            drills: [
                DrillKind::NodeLoss,
                DrillKind::DriveLoss,
                DrillKind::NetworkPartition,
                DrillKind::DiskFull,
                DrillKind::Recovery,
                DrillKind::RollingUpgrade,
                DrillKind::UpgradeRollback,
            ]
            .into_iter()
            .map(|kind| DrillEvidence {
                kind,
                passed: true,
                recovery_point_loss_seconds: 0,
                recovery_time_seconds: 100,
                evidence_ref: format!("evidence.{kind:?}"),
            })
            .collect(),
            production_equivalent_soak_hours: REQUIRED_SOAK_HOURS,
            digest_scrub_objects: 1_000_000,
            digest_mismatches: 0,
            canary_tenants: 10,
            shadow_reads: MINIMUM_SHADOW_READS,
            shadow_read_mismatches: 0,
            canary_rollback_rehearsed: true,
            fallback_providers: vec![
                FallbackProviderEvidence {
                    provider_ref: "provider.minio".to_owned(),
                    profile_tck_passed: true,
                    migration_rehearsed: true,
                },
                FallbackProviderEvidence {
                    provider_ref: "provider.aws-s3".to_owned(),
                    profile_tck_passed: true,
                    migration_rehearsed: true,
                },
            ],
        }
    }

    #[test]
    fn complete_evidence_is_eligible() {
        assert_eq!(evaluate(&passing()).status, QualificationStatus::Eligible);
    }

    #[test]
    fn elapsed_soak_cannot_be_inferred_or_waived() {
        let mut evidence = passing();
        evidence.production_equivalent_soak_hours = REQUIRED_SOAK_HOURS - 1;
        assert_eq!(
            evaluate(&evidence).status,
            QualificationStatus::BlockedBySoak
        );
    }

    #[test]
    fn digest_mismatch_and_missing_drill_are_ineligible() {
        let mut evidence = passing();
        evidence.digest_mismatches = 1;
        evidence.drills.pop();
        let decision = evaluate(&evidence);
        assert_eq!(decision.status, QualificationStatus::Ineligible);
        assert_eq!(decision.reasons.len(), 2);
    }

    #[test]
    fn fallback_requires_tck_and_migration_rehearsal() {
        let mut evidence = passing();
        evidence.fallback_providers[0].migration_rehearsed = false;
        assert_eq!(evaluate(&evidence).status, QualificationStatus::Ineligible);
    }
}
