//! Cloud execution-world reference control plane.
//!
//! The real container/microVM substrate remains behind this port. This crate
//! freezes tenant isolation, attachment, image identity and cleanup semantics.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

pub const FS_PROVIDER: &str = "@openmuse/dsh-workspace-runtime/fs";
pub const SUBPROCESS_PROVIDER: &str = "@openmuse/dsh-workspace-runtime/subprocess";
pub const SANDBOX_PROVIDER: &str = "@openmuse/dsh-workspace-runtime/sandbox";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuntimeImageIdentity {
    pub image_digest: String,
    pub version: String,
    pub sbom_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloudAllocationRequest {
    pub tenant_ref: String,
    pub task_ref: String,
    pub lease_ref: String,
    pub checkout_handle_ref: String,
    pub image: RuntimeImageIdentity,
    pub generation: u64,
    pub now_ms: u64,
    pub ttl_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CloudRuntimeDescriptor {
    pub runtime_ref: String,
    pub tenant_ref: String,
    pub task_ref: String,
    pub lease_ref: String,
    pub image: RuntimeImageIdentity,
    pub generation: u64,
    pub expires_at_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ControlAttachment {
    pub token_ref: String,
    pub runtime_ref: String,
    pub audience: String,
    pub generation: u64,
    pub expires_at_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeEnd {
    Cancelled,
    Crashed,
    Expired,
    Orphaned,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RuntimeMetrics {
    pub allocations: u64,
    pub cold_starts: u64,
    pub cancellations: u64,
    pub crashes: u64,
    pub orphan_cleanups: u64,
    pub processes_terminated: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum CloudRuntimeError {
    #[error("policy denied")]
    PolicyDenied,
    #[error("runtime not found")]
    NotFound,
    #[error("attachment expired")]
    Expired,
    #[error("stale generation")]
    StaleGeneration,
    #[error("runtime unavailable")]
    Unavailable,
}

type Result<T> = std::result::Result<T, CloudRuntimeError>;

#[derive(Debug, Clone)]
struct RuntimeState {
    descriptor: CloudRuntimeDescriptor,
    checkout_handle_ref: String,
    files: BTreeMap<String, Vec<u8>>,
    active_processes: u32,
    last_heartbeat_ms: u64,
    available: bool,
}

#[derive(Default)]
pub struct CloudRuntimePool {
    runtimes: HashMap<String, RuntimeState>,
    attachments: HashMap<String, ControlAttachment>,
    metrics: RuntimeMetrics,
    next_runtime: u64,
    next_attachment: u64,
}

impl CloudRuntimePool {
    pub fn allocate(&mut self, request: CloudAllocationRequest) -> Result<CloudRuntimeDescriptor> {
        if request.tenant_ref.is_empty()
            || request.task_ref.is_empty()
            || request.lease_ref.is_empty()
            || request.checkout_handle_ref.is_empty()
            || request.generation == 0
            || request.ttl_ms == 0
            || !valid_digest(&request.image.image_digest)
            || !valid_digest(&request.image.sbom_digest)
        {
            return Err(CloudRuntimeError::PolicyDenied);
        }
        self.next_runtime += 1;
        let runtime_ref = format!("cloud-runtime.{}", self.next_runtime);
        let descriptor = CloudRuntimeDescriptor {
            runtime_ref: runtime_ref.clone(),
            tenant_ref: request.tenant_ref,
            task_ref: request.task_ref,
            lease_ref: request.lease_ref,
            image: request.image,
            generation: request.generation,
            expires_at_ms: request.now_ms.saturating_add(request.ttl_ms),
        };
        self.runtimes.insert(
            runtime_ref,
            RuntimeState {
                descriptor: descriptor.clone(),
                checkout_handle_ref: request.checkout_handle_ref,
                files: BTreeMap::new(),
                active_processes: 0,
                last_heartbeat_ms: request.now_ms,
                available: true,
            },
        );
        self.metrics.allocations += 1;
        self.metrics.cold_starts += 1;
        Ok(descriptor)
    }

    pub fn attach(
        &mut self,
        runtime_ref: &str,
        tenant_ref: &str,
        audience: &str,
        generation: u64,
        now_ms: u64,
        ttl_ms: u64,
    ) -> Result<ControlAttachment> {
        let runtime = self.runtime(runtime_ref, tenant_ref, generation, now_ms)?;
        if audience.is_empty() || ttl_ms == 0 {
            return Err(CloudRuntimeError::PolicyDenied);
        }
        let expires_at_ms = now_ms
            .saturating_add(ttl_ms)
            .min(runtime.descriptor.expires_at_ms);
        self.next_attachment += 1;
        let attachment = ControlAttachment {
            token_ref: format!("cloud-attachment.{}", self.next_attachment),
            runtime_ref: runtime_ref.into(),
            audience: audience.into(),
            generation,
            expires_at_ms,
        };
        self.attachments
            .insert(attachment.token_ref.clone(), attachment.clone());
        Ok(attachment)
    }

    pub fn authenticate(
        &self,
        token_ref: &str,
        tenant_ref: &str,
        audience: &str,
        generation: u64,
        now_ms: u64,
    ) -> Result<&CloudRuntimeDescriptor> {
        let attachment = self
            .attachments
            .get(token_ref)
            .ok_or(CloudRuntimeError::NotFound)?;
        if attachment.audience != audience {
            return Err(CloudRuntimeError::PolicyDenied);
        }
        if attachment.generation != generation {
            return Err(CloudRuntimeError::StaleGeneration);
        }
        if now_ms >= attachment.expires_at_ms {
            return Err(CloudRuntimeError::Expired);
        }
        Ok(&self
            .runtime(&attachment.runtime_ref, tenant_ref, generation, now_ms)?
            .descriptor)
    }

    pub fn fs_write(
        &mut self,
        runtime_ref: &str,
        tenant_ref: &str,
        path: &str,
        data: &[u8],
    ) -> Result<()> {
        let path = workspace_path(path)?;
        self.runtime_mut(runtime_ref, tenant_ref)?
            .files
            .insert(path, data.to_vec());
        Ok(())
    }

    pub fn fs_read(&self, runtime_ref: &str, tenant_ref: &str, path: &str) -> Result<Vec<u8>> {
        let path = workspace_path(path)?;
        self.runtime_any_generation(runtime_ref, tenant_ref)?
            .files
            .get(&path)
            .cloned()
            .ok_or(CloudRuntimeError::NotFound)
    }

    pub fn bash_read(&self, runtime_ref: &str, tenant_ref: &str, path: &str) -> Result<Vec<u8>> {
        self.fs_read(runtime_ref, tenant_ref, path)
    }

    pub fn bash_write(
        &mut self,
        runtime_ref: &str,
        tenant_ref: &str,
        path: &str,
        data: &[u8],
    ) -> Result<()> {
        self.fs_write(runtime_ref, tenant_ref, path, data)
    }

    pub fn pty_round_trip(
        &self,
        runtime_ref: &str,
        tenant_ref: &str,
        input: &[u8],
    ) -> Result<Vec<u8>> {
        self.runtime_any_generation(runtime_ref, tenant_ref)?;
        Ok(input.to_vec())
    }

    pub fn lsp_round_trip(
        &self,
        runtime_ref: &str,
        tenant_ref: &str,
        frame: &[u8],
    ) -> Result<Vec<u8>> {
        self.runtime_any_generation(runtime_ref, tenant_ref)?;
        if !frame.starts_with(b"Content-Length:") {
            return Err(CloudRuntimeError::PolicyDenied);
        }
        Ok(frame.to_vec())
    }

    pub fn spawn_process(&mut self, runtime_ref: &str, tenant_ref: &str) -> Result<()> {
        self.runtime_mut(runtime_ref, tenant_ref)?.active_processes += 1;
        Ok(())
    }

    pub fn heartbeat(&mut self, runtime_ref: &str, tenant_ref: &str, now_ms: u64) -> Result<()> {
        self.runtime_mut(runtime_ref, tenant_ref)?.last_heartbeat_ms = now_ms;
        Ok(())
    }

    pub fn end(&mut self, runtime_ref: &str, reason: RuntimeEnd) -> Result<u32> {
        let runtime = self
            .runtimes
            .get_mut(runtime_ref)
            .ok_or(CloudRuntimeError::NotFound)?;
        let terminated = runtime.active_processes;
        runtime.active_processes = 0;
        runtime.available = false;
        runtime.files.clear();
        match reason {
            RuntimeEnd::Cancelled => self.metrics.cancellations += 1,
            RuntimeEnd::Crashed => self.metrics.crashes += 1,
            RuntimeEnd::Orphaned => self.metrics.orphan_cleanups += 1,
            RuntimeEnd::Expired => {}
        }
        self.metrics.processes_terminated += u64::from(terminated);
        self.attachments
            .retain(|_, item| item.runtime_ref != runtime_ref);
        Ok(terminated)
    }

    pub fn sweep_orphans(&mut self, now_ms: u64, heartbeat_timeout_ms: u64) -> Vec<String> {
        let stale: Vec<String> = self
            .runtimes
            .iter()
            .filter(|(_, runtime)| {
                runtime.available
                    && (now_ms >= runtime.descriptor.expires_at_ms
                        || now_ms.saturating_sub(runtime.last_heartbeat_ms) >= heartbeat_timeout_ms)
            })
            .map(|(runtime_ref, _)| runtime_ref.clone())
            .collect();
        for runtime_ref in &stale {
            let _ = self.end(runtime_ref, RuntimeEnd::Orphaned);
        }
        stale
    }

    pub fn metrics(&self) -> &RuntimeMetrics {
        &self.metrics
    }

    pub fn opaque_checkout_bound(&self, runtime_ref: &str) -> bool {
        self.runtimes
            .get(runtime_ref)
            .is_some_and(|runtime| !runtime.checkout_handle_ref.is_empty())
    }

    fn runtime(
        &self,
        runtime_ref: &str,
        tenant_ref: &str,
        generation: u64,
        now_ms: u64,
    ) -> Result<&RuntimeState> {
        let runtime = self.runtime_any_generation(runtime_ref, tenant_ref)?;
        if runtime.descriptor.generation != generation {
            return Err(CloudRuntimeError::StaleGeneration);
        }
        if now_ms >= runtime.descriptor.expires_at_ms {
            return Err(CloudRuntimeError::Expired);
        }
        Ok(runtime)
    }

    fn runtime_any_generation(&self, runtime_ref: &str, tenant_ref: &str) -> Result<&RuntimeState> {
        let runtime = self
            .runtimes
            .get(runtime_ref)
            .ok_or(CloudRuntimeError::NotFound)?;
        if runtime.descriptor.tenant_ref != tenant_ref {
            return Err(CloudRuntimeError::PolicyDenied);
        }
        if !runtime.available {
            return Err(CloudRuntimeError::Unavailable);
        }
        Ok(runtime)
    }

    fn runtime_mut(&mut self, runtime_ref: &str, tenant_ref: &str) -> Result<&mut RuntimeState> {
        let runtime = self
            .runtimes
            .get_mut(runtime_ref)
            .ok_or(CloudRuntimeError::NotFound)?;
        if runtime.descriptor.tenant_ref != tenant_ref {
            return Err(CloudRuntimeError::PolicyDenied);
        }
        if !runtime.available {
            return Err(CloudRuntimeError::Unavailable);
        }
        Ok(runtime)
    }
}

fn workspace_path(path: &str) -> Result<String> {
    if !path.starts_with("/workspace/")
        || path.split('/').any(|part| part == "..")
        || path.contains('\0')
    {
        return Err(CloudRuntimeError::PolicyDenied);
    }
    Ok(path.into())
}

fn valid_digest(value: &str) -> bool {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return false;
    };
    hex.len() == 64
        && hex
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}
