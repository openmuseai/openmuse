use crate::{
    CommandContribution, Contributions, EditorContribution, PROTOCOL_VERSION, PanelContribution,
    Permission, PluginId, PluginManifest, ProtocolVersion, RuntimeKind, ServiceContribution,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};

pub const MANIFEST_VERSION_V2: u16 = 2;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PluginManifestV2 {
    pub manifest_version: u16,
    pub id: PluginId,
    pub name: String,
    pub version: String,
    pub protocol: ProtocolVersion,
    pub ui_runtime: UiRuntime,
    pub execution_connector: ExecutionConnector,
    pub compatibility: Compatibility,
    pub artifacts: Vec<PluginArtifact>,
    #[serde(default)]
    pub activation_events: Vec<String>,
    #[serde(default)]
    pub requested_permissions: BTreeSet<Permission>,
    pub presentation: Presentation,
    #[serde(default)]
    pub contributes: ContributionsV2,
}

impl PluginManifestV2 {
    pub fn from_json_slice(bytes: &[u8]) -> Result<Self, ManifestV2Error> {
        let manifest: Self = serde_json::from_slice(bytes)
            .map_err(|error| ManifestV2Error::InvalidJson(error.to_string()))?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), ManifestV2Error> {
        if self.manifest_version != MANIFEST_VERSION_V2 {
            return Err(ManifestV2Error::UnsupportedManifestVersion(
                self.manifest_version,
            ));
        }
        if !PROTOCOL_VERSION.accepts(self.protocol) {
            return Err(ManifestV2Error::IncompatibleProtocol {
                major: self.protocol.major,
                minor: self.protocol.minor,
            });
        }
        require_nonempty("plugin id", &self.id.0)?;
        require_nonempty("name", &self.name)?;
        require_nonempty("version", &self.version)?;
        self.ui_runtime.validate()?;
        self.execution_connector.validate()?;
        self.presentation.validate()?;

        let mut decisions = BTreeMap::new();
        for decision in &self.compatibility.targets {
            decision.validate()?;
            if decisions.insert(decision.target, decision).is_some() {
                return Err(ManifestV2Error::DuplicateTarget(decision.target));
            }
        }
        if decisions.is_empty() {
            return Err(ManifestV2Error::InvalidField("compatibility.targets"));
        }

        let mut artifact_ids = BTreeSet::new();
        let mut supported_artifact_count: BTreeMap<TargetTriple, usize> = BTreeMap::new();
        for artifact in &self.artifacts {
            artifact.validate()?;
            if !artifact_ids.insert(&artifact.id) {
                return Err(ManifestV2Error::DuplicateArtifact(artifact.id.clone()));
            }
            match decisions.get(&artifact.target) {
                Some(decision) if decision.status == TargetStatus::Supported => {
                    *supported_artifact_count.entry(artifact.target).or_default() += 1;
                }
                _ => {
                    return Err(ManifestV2Error::ArtifactTargetsUnsupported(
                        artifact.id.clone(),
                    ));
                }
            }
        }
        for decision in decisions.values() {
            if decision.status == TargetStatus::Supported
                && supported_artifact_count
                    .get(&decision.target)
                    .copied()
                    .unwrap_or_default()
                    == 0
            {
                return Err(ManifestV2Error::MissingArtifact(decision.target));
            }
        }
        self.contributes.validate()
    }

    pub fn resolve_artifacts(
        &self,
        target: TargetTriple,
    ) -> Result<Vec<&PluginArtifact>, ManifestV2Error> {
        self.validate()?;
        let decision = self
            .compatibility
            .targets
            .iter()
            .find(|decision| decision.target == target)
            .ok_or(ManifestV2Error::UnsupportedTarget(target))?;
        if decision.status != TargetStatus::Supported {
            return Err(ManifestV2Error::UnsupportedTarget(target));
        }
        let artifacts = self
            .artifacts
            .iter()
            .filter(|artifact| artifact.target == target)
            .collect::<Vec<_>>();
        if artifacts.is_empty() {
            return Err(ManifestV2Error::MissingArtifact(target));
        }
        Ok(artifacts)
    }

    pub fn verify_sandbox_worker<V: ArtifactSignatureVerifier>(
        &self,
        artifact_id: &str,
        bytes: &[u8],
        verifier: &V,
    ) -> Result<VerifiedArtifact, ManifestV2Error> {
        self.validate()?;
        let artifact = self
            .artifacts
            .iter()
            .find(|artifact| artifact.id == artifact_id)
            .ok_or_else(|| ManifestV2Error::UnknownArtifact(artifact_id.to_owned()))?;
        if artifact.kind != ArtifactKind::SandboxWorker {
            return Err(ManifestV2Error::NotSandboxWorker(artifact.id.clone()));
        }
        let actual_digest = sha256_hex(bytes);
        if actual_digest != artifact.digest.value {
            return Err(ManifestV2Error::DigestMismatch(artifact.id.clone()));
        }
        let signature = artifact
            .signature
            .as_ref()
            .ok_or_else(|| ManifestV2Error::SignatureRequired(artifact.id.clone()))?;
        if !verifier.verify(artifact, bytes, signature) {
            return Err(ManifestV2Error::SignatureRejected(artifact.id.clone()));
        }
        Ok(VerifiedArtifact {
            plugin_id: self.id.clone(),
            artifact_id: artifact.id.clone(),
            target: artifact.target,
            sha256: actual_digest,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UiRuntimeKind {
    None,
    Flutter,
    WebView,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UiRuntime {
    pub kind: UiRuntimeKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
}

impl UiRuntime {
    fn validate(&self) -> Result<(), ManifestV2Error> {
        if let Some(entrypoint) = &self.entrypoint {
            require_nonempty("ui_runtime.entrypoint", entrypoint)?;
        }
        if self.kind == UiRuntimeKind::None && self.entrypoint.is_some() {
            return Err(ManifestV2Error::InvalidField("ui_runtime.entrypoint"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExecutionConnectorKind {
    None,
    HostProcess,
    RemoteDsh,
    SandboxProvider,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionConnector {
    pub kind: ExecutionConnectorKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

impl ExecutionConnector {
    fn validate(&self) -> Result<(), ManifestV2Error> {
        if let Some(protocol) = &self.protocol {
            require_nonempty("execution_connector.protocol", protocol)?;
        }
        if self.kind == ExecutionConnectorKind::None && self.protocol.is_some() {
            return Err(ManifestV2Error::InvalidField(
                "execution_connector.protocol",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TargetOs {
    Macos,
    Windows,
    Linux,
    Android,
    Ios,
    Web,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TargetArch {
    Aarch64,
    #[serde(rename = "x86_64")]
    X86_64,
    Wasm32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TargetLibc {
    Darwin,
    Msvc,
    Gnu,
    Musl,
    Bionic,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetTriple {
    pub os: TargetOs,
    pub arch: TargetArch,
    pub libc: TargetLibc,
}

impl Display for TargetTriple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}/{:?}/{:?}", self.os, self.arch, self.libc)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TargetStatus {
    Supported,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetDecision {
    pub target: TargetTriple,
    pub status: TargetStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl TargetDecision {
    fn validate(&self) -> Result<(), ManifestV2Error> {
        validate_target(self.target)?;
        match (self.status, &self.reason) {
            (TargetStatus::Unsupported, Some(reason)) => {
                require_nonempty("compatibility.targets.reason", reason)
            }
            (TargetStatus::Unsupported, None) => Err(ManifestV2Error::InvalidField(
                "compatibility.targets.reason",
            )),
            (TargetStatus::Supported, Some(_)) => Err(ManifestV2Error::InvalidField(
                "compatibility.targets.reason",
            )),
            (TargetStatus::Supported, None) => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Compatibility {
    pub targets: Vec<TargetDecision>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ArtifactKind {
    HostBundle,
    WebBundle,
    NativeExecutable,
    RuntimeClosure,
    SandboxWorker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DigestAlgorithm {
    Sha256,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactDigest {
    pub algorithm: DigestAlgorithm,
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SignatureAlgorithm {
    Ed25519,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactSignature {
    pub algorithm: SignatureAlgorithm,
    pub key_id: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PluginArtifact {
    pub id: String,
    pub kind: ArtifactKind,
    pub target: TargetTriple,
    pub digest: ArtifactDigest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<ArtifactSignature>,
    pub license: String,
    pub abi: String,
}

impl PluginArtifact {
    fn validate(&self) -> Result<(), ManifestV2Error> {
        require_nonempty("artifacts.id", &self.id)?;
        validate_target(self.target)?;
        if self.digest.value.len() != 64
            || !self
                .digest
                .value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(ManifestV2Error::InvalidDigest(self.id.clone()));
        }
        if let Some(signature) = &self.signature {
            require_nonempty("artifacts.signature.key_id", &signature.key_id)?;
            require_nonempty("artifacts.signature.value", &signature.value)?;
        }
        require_nonempty("artifacts.license", &self.license)?;
        require_nonempty("artifacts.abi", &self.abi)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PresentationSurface {
    Editor,
    LeftSidebar,
    RightSidebar,
    BottomPanel,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Presentation {
    #[serde(default)]
    pub surfaces: BTreeSet<PresentationSurface>,
    #[serde(default)]
    pub remote_capable: bool,
}

impl Presentation {
    fn validate(&self) -> Result<(), ManifestV2Error> {
        if self.surfaces.is_empty() && self.remote_capable {
            return Err(ManifestV2Error::InvalidField("presentation.surfaces"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContributionsV2 {
    #[serde(default)]
    pub commands: Vec<CommandContribution>,
    #[serde(default)]
    pub services: Vec<ServiceContribution>,
    #[serde(default)]
    pub editors: Vec<EditorContribution>,
    #[serde(default)]
    pub panels: Vec<PanelContribution>,
    #[serde(default)]
    pub agent_cli: Vec<AgentCliContribution>,
}

impl ContributionsV2 {
    fn validate(&self) -> Result<(), ManifestV2Error> {
        let mut identities = BTreeSet::new();
        for command in &self.agent_cli {
            command.validate()?;
            let identity = (
                command.group.as_str(),
                command.namespace.as_str(),
                command.command.as_str(),
            );
            if !identities.insert(identity) {
                return Err(ManifestV2Error::DuplicateAgentCli(format!(
                    "{}/{}/{}",
                    command.group, command.namespace, command.command
                )));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentCliContribution {
    pub group: String,
    pub namespace: String,
    pub command: String,
    pub schema: Value,
    #[serde(default)]
    pub required_permissions: BTreeSet<Permission>,
    #[serde(default)]
    pub effects: BTreeSet<String>,
}

impl AgentCliContribution {
    fn validate(&self) -> Result<(), ManifestV2Error> {
        require_nonempty("agent_cli.group", &self.group)?;
        require_nonempty("agent_cli.namespace", &self.namespace)?;
        require_nonempty("agent_cli.command", &self.command)?;
        if !self.schema.is_object() {
            return Err(ManifestV2Error::InvalidField("agent_cli.schema"));
        }
        if self.effects.iter().any(|effect| effect.is_empty()) {
            return Err(ManifestV2Error::InvalidField("agent_cli.effects"));
        }
        Ok(())
    }
}

pub trait ArtifactSignatureVerifier {
    fn verify(
        &self,
        artifact: &PluginArtifact,
        bytes: &[u8],
        signature: &ArtifactSignature,
    ) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedArtifact {
    pub plugin_id: PluginId,
    pub artifact_id: String,
    pub target: TargetTriple,
    pub sha256: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ManifestV2Error {
    #[error("invalid manifest JSON: {0}")]
    InvalidJson(String),
    #[error("unsupported manifest version {0}")]
    UnsupportedManifestVersion(u16),
    #[error("incompatible plugin protocol {major}.{minor}")]
    IncompatibleProtocol { major: u16, minor: u16 },
    #[error("invalid manifest field {0}")]
    InvalidField(&'static str),
    #[error("duplicate target decision {0}")]
    DuplicateTarget(TargetTriple),
    #[error("duplicate artifact {0}")]
    DuplicateArtifact(String),
    #[error("duplicate Agent CLI command {0}")]
    DuplicateAgentCli(String),
    #[error("unsupported target {0}")]
    UnsupportedTarget(TargetTriple),
    #[error("supported target {0} has no artifact")]
    MissingArtifact(TargetTriple),
    #[error("artifact {0} targets an unsupported platform")]
    ArtifactTargetsUnsupported(String),
    #[error("artifact {0} has an invalid sha256 digest")]
    InvalidDigest(String),
    #[error("unknown artifact {0}")]
    UnknownArtifact(String),
    #[error("artifact {0} is not a sandbox worker")]
    NotSandboxWorker(String),
    #[error("artifact {0} digest mismatch")]
    DigestMismatch(String),
    #[error("sandbox worker {0} requires a signature")]
    SignatureRequired(String),
    #[error("sandbox worker {0} signature was rejected")]
    SignatureRejected(String),
}

pub fn migrate_manifest_v1(manifest: &PluginManifest) -> PluginManifestV2 {
    let (ui_kind, connector_kind) = match manifest.runtime {
        RuntimeKind::BuiltIn => (UiRuntimeKind::Flutter, ExecutionConnectorKind::None),
        RuntimeKind::NativeProcess => (UiRuntimeKind::Flutter, ExecutionConnectorKind::HostProcess),
        RuntimeKind::WebView => (UiRuntimeKind::WebView, ExecutionConnectorKind::None),
    };
    let mut surfaces = BTreeSet::new();
    if !manifest.contributes.editors.is_empty() {
        surfaces.insert(PresentationSurface::Editor);
    }
    for panel in &manifest.contributes.panels {
        match panel.region.as_str() {
            "left-sidebar" => surfaces.insert(PresentationSurface::LeftSidebar),
            "right-sidebar" => surfaces.insert(PresentationSurface::RightSidebar),
            "bottom-panel" => surfaces.insert(PresentationSurface::BottomPanel),
            "editor" => surfaces.insert(PresentationSurface::Editor),
            _ => false,
        };
    }
    PluginManifestV2 {
        manifest_version: MANIFEST_VERSION_V2,
        id: manifest.id.clone(),
        name: manifest.name.clone(),
        version: manifest.version.clone(),
        protocol: manifest.protocol,
        ui_runtime: UiRuntime {
            kind: ui_kind,
            entrypoint: None,
        },
        execution_connector: ExecutionConnector {
            kind: connector_kind,
            protocol: None,
        },
        compatibility: Compatibility {
            targets: known_targets()
                .into_iter()
                .map(|target| TargetDecision {
                    target,
                    status: TargetStatus::Unsupported,
                    reason: Some(
                        "legacy v1 manifest has no signed target artifact metadata".into(),
                    ),
                })
                .collect(),
        },
        artifacts: Vec::new(),
        activation_events: manifest.activation_events.clone(),
        requested_permissions: manifest.permissions.clone(),
        presentation: Presentation {
            surfaces,
            remote_capable: false,
        },
        contributes: ContributionsV2 {
            commands: manifest.contributes.commands.clone(),
            services: manifest.contributes.services.clone(),
            editors: manifest.contributes.editors.clone(),
            panels: manifest.contributes.panels.clone(),
            agent_cli: Vec::new(),
        },
    }
}

pub fn known_targets() -> [TargetTriple; 6] {
    [
        TargetTriple {
            os: TargetOs::Macos,
            arch: TargetArch::Aarch64,
            libc: TargetLibc::Darwin,
        },
        TargetTriple {
            os: TargetOs::Macos,
            arch: TargetArch::X86_64,
            libc: TargetLibc::Darwin,
        },
        TargetTriple {
            os: TargetOs::Windows,
            arch: TargetArch::X86_64,
            libc: TargetLibc::Msvc,
        },
        TargetTriple {
            os: TargetOs::Linux,
            arch: TargetArch::X86_64,
            libc: TargetLibc::Gnu,
        },
        TargetTriple {
            os: TargetOs::Android,
            arch: TargetArch::Aarch64,
            libc: TargetLibc::Bionic,
        },
        TargetTriple {
            os: TargetOs::Ios,
            arch: TargetArch::Aarch64,
            libc: TargetLibc::Darwin,
        },
    ]
}

fn validate_target(target: TargetTriple) -> Result<(), ManifestV2Error> {
    let valid = match target.os {
        TargetOs::Macos | TargetOs::Ios => target.libc == TargetLibc::Darwin,
        TargetOs::Windows => target.libc == TargetLibc::Msvc,
        TargetOs::Android => target.libc == TargetLibc::Bionic,
        TargetOs::Linux => matches!(target.libc, TargetLibc::Gnu | TargetLibc::Musl),
        TargetOs::Web => target.arch == TargetArch::Wasm32 && target.libc == TargetLibc::None,
    };
    if valid {
        Ok(())
    } else {
        Err(ManifestV2Error::InvalidField("target"))
    }
}

fn require_nonempty(field: &'static str, value: &str) -> Result<(), ManifestV2Error> {
    if value.trim().is_empty() {
        return Err(ManifestV2Error::InvalidField(field));
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

impl From<&Contributions> for ContributionsV2 {
    fn from(value: &Contributions) -> Self {
        Self {
            commands: value.commands.clone(),
            services: value.services.clone(),
            editors: value.editors.clone(),
            panels: value.panels.clone(),
            agent_cli: Vec::new(),
        }
    }
}
