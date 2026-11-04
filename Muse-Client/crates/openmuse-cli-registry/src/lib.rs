//! Resolve installed Plugin Agent CLI contributions into a frozen Lease snapshot.

use openmuse_plugin_protocol::{
    AgentCliContribution, ArtifactKind, Permission, PluginManifestV2, TargetTriple,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactEvidence {
    pub artifact_id: String,
    pub digest: String,
    pub signature_valid: bool,
    pub revoked: bool,
}

#[derive(Debug, Clone)]
pub struct InstalledPlugin {
    pub manifest: PluginManifestV2,
    pub granted_permissions: BTreeSet<Permission>,
    pub artifacts: Vec<ArtifactEvidence>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResolvedCommand {
    pub identity: String,
    pub plugin_id: String,
    pub plugin_version: String,
    pub command: AgentCliContribution,
    pub worker_artifact_id: String,
    pub worker_digest: String,
    pub abi: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UnavailableCapability {
    pub plugin_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CliRegistrySnapshot {
    pub generation: u64,
    pub digest: String,
    pub target: TargetTriple,
    pub commands: Vec<ResolvedCommand>,
    pub unavailable: Vec<UnavailableCapability>,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RegistryError {
    #[error("namespace conflict: {0}")]
    NamespaceConflict(String),
    #[error("invalid manifest: {0}")]
    InvalidManifest(String),
}

pub struct CliRegistryResolver {
    target: TargetTriple,
    accepted_abis: BTreeSet<String>,
    accepted_licenses: BTreeSet<String>,
}

impl CliRegistryResolver {
    pub fn new(
        target: TargetTriple,
        accepted_abis: BTreeSet<String>,
        accepted_licenses: BTreeSet<String>,
    ) -> Self {
        Self {
            target,
            accepted_abis,
            accepted_licenses,
        }
    }

    pub fn resolve(
        &self,
        generation: u64,
        installed: &[InstalledPlugin],
        workspace_requirements: &BTreeSet<String>,
    ) -> Result<CliRegistrySnapshot, RegistryError> {
        let mut commands = BTreeMap::<String, ResolvedCommand>::new();
        let mut unavailable = Vec::new();

        let mut plugins = installed.iter().collect::<Vec<_>>();
        plugins.sort_by(|a, b| a.manifest.id.0.cmp(&b.manifest.id.0));
        for plugin in plugins {
            let plugin_id = plugin.manifest.id.0.clone();
            if !workspace_requirements.is_empty() && !workspace_requirements.contains(&plugin_id) {
                continue;
            }
            if let Err(error) = plugin.manifest.validate() {
                return Err(RegistryError::InvalidManifest(error.to_string()));
            }
            let Some((artifact, evidence)) = plugin
                .manifest
                .artifacts
                .iter()
                .filter(|artifact| {
                    artifact.target == self.target && artifact.kind == ArtifactKind::SandboxWorker
                })
                .find_map(|artifact| {
                    plugin
                        .artifacts
                        .iter()
                        .find(|evidence| {
                            evidence.artifact_id == artifact.id
                                && evidence.digest == artifact.digest.value
                                && evidence.signature_valid
                                && !evidence.revoked
                        })
                        .map(|evidence| (artifact, evidence))
                })
            else {
                unavailable.push(UnavailableCapability {
                    plugin_id,
                    reason: "no verified sandbox worker for target".into(),
                });
                continue;
            };
            if !self.accepted_abis.contains(&artifact.abi)
                || !self.accepted_licenses.contains(&artifact.license)
            {
                unavailable.push(UnavailableCapability {
                    plugin_id,
                    reason: "worker ABI or license is not admitted".into(),
                });
                continue;
            }
            for command in &plugin.manifest.contributes.agent_cli {
                if !command
                    .required_permissions
                    .is_subset(&plugin.granted_permissions)
                {
                    continue;
                }
                let identity = format!(
                    "{}/{}/{}",
                    command.group, command.namespace, command.command
                );
                let resolved = ResolvedCommand {
                    identity: identity.clone(),
                    plugin_id: plugin.manifest.id.0.clone(),
                    plugin_version: plugin.manifest.version.clone(),
                    command: command.clone(),
                    worker_artifact_id: artifact.id.clone(),
                    worker_digest: evidence.digest.clone(),
                    abi: artifact.abi.clone(),
                };
                if commands.insert(identity.clone(), resolved).is_some() {
                    return Err(RegistryError::NamespaceConflict(identity));
                }
            }
        }
        for requirement in workspace_requirements {
            if !installed
                .iter()
                .any(|plugin| plugin.manifest.id.0 == *requirement)
            {
                unavailable.push(UnavailableCapability {
                    plugin_id: requirement.clone(),
                    reason: "workspace requirement is not an installed Plugin".into(),
                });
            }
        }
        unavailable.sort_by(|a, b| a.plugin_id.cmp(&b.plugin_id));
        let command_values = commands.into_values().collect::<Vec<_>>();
        let canonical =
            serde_json::to_vec(&(generation, self.target, &command_values, &unavailable))
                .expect("registry values are serializable");
        Ok(CliRegistrySnapshot {
            generation,
            digest: format!("sha256:{:x}", Sha256::digest(canonical)),
            target: self.target,
            commands: command_values,
            unavailable,
        })
    }
}
