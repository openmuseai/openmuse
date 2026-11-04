//! Credential references and short-lived leases for storage adapters.
//!
//! Secret material is intentionally non-serializable and redacted from Debug.

use async_trait::async_trait;
use openmuse_storage_contract::{StorageError, StorageErrorCode, StorageResult};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use zeroize::Zeroize;

#[derive(Clone, PartialEq, Eq)]
pub struct SecretBytes(Vec<u8>);

impl SecretBytes {
    pub fn new(value: impl Into<Vec<u8>>) -> StorageResult<Self> {
        let value = value.into();
        if value.is_empty() {
            return Err(StorageError::new(
                StorageErrorCode::Denied,
                "credential secret is empty",
                false,
            ));
        }
        Ok(Self(value))
    }

    pub fn expose(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for SecretBytes {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SecretBytes([REDACTED])")
    }
}

impl Drop for SecretBytes {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

#[derive(Clone)]
pub struct CredentialLease {
    pub credential_ref: String,
    pub access_key_id: String,
    pub secret_access_key: SecretBytes,
    pub session_token: Option<SecretBytes>,
    pub generation: u64,
    pub expires_at_ms: u64,
}

impl fmt::Debug for CredentialLease {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CredentialLease")
            .field("credential_ref", &self.credential_ref)
            .field("access_key_id", &"[REDACTED]")
            .field("secret_access_key", &"[REDACTED]")
            .field(
                "session_token",
                &self.session_token.as_ref().map(|_| "[REDACTED]"),
            )
            .field("generation", &self.generation)
            .field("expires_at_ms", &self.expires_at_ms)
            .finish()
    }
}

impl CredentialLease {
    pub fn ensure_live(&self, now_ms: u64, expected_generation: u64) -> StorageResult<()> {
        if self.generation != expected_generation {
            return Err(StorageError::new(
                StorageErrorCode::StaleGeneration,
                "credential generation is stale",
                false,
            ));
        }
        if now_ms >= self.expires_at_ms {
            return Err(StorageError::new(
                StorageErrorCode::Expired,
                "credential lease expired",
                false,
            ));
        }
        Ok(())
    }
}

#[async_trait]
pub trait CredentialVaultPort: Send + Sync {
    async fn lease(
        &self,
        credential_ref: &str,
        audience: &str,
        now_ms: u64,
        ttl_ms: u64,
    ) -> StorageResult<CredentialLease>;
}

#[derive(Clone, Default)]
pub struct InMemoryCredentialVault {
    state: Arc<Mutex<HashMap<String, StoredCredential>>>,
}

#[derive(Clone)]
struct StoredCredential {
    access_key_id: String,
    secret_access_key: SecretBytes,
    session_token: Option<SecretBytes>,
    audience: String,
    generation: u64,
    revoked: bool,
}

impl InMemoryCredentialVault {
    pub fn rotate(
        &self,
        credential_ref: impl Into<String>,
        audience: impl Into<String>,
        access_key_id: impl Into<String>,
        secret_access_key: SecretBytes,
        session_token: Option<SecretBytes>,
    ) -> StorageResult<u64> {
        let credential_ref = credential_ref.into();
        let audience = audience.into();
        let access_key_id = access_key_id.into();
        if credential_ref.is_empty() || audience.is_empty() || access_key_id.is_empty() {
            return Err(StorageError::new(
                StorageErrorCode::Denied,
                "credential ref and audience are required",
                false,
            ));
        }
        let mut state = self.lock()?;
        let generation = state
            .get(&credential_ref)
            .map_or(1, |existing| existing.generation.saturating_add(1));
        state.insert(
            credential_ref,
            StoredCredential {
                access_key_id,
                secret_access_key,
                session_token,
                audience,
                generation,
                revoked: false,
            },
        );
        Ok(generation)
    }

    pub fn revoke(&self, credential_ref: &str) -> StorageResult<()> {
        let mut state = self.lock()?;
        let value = state.get_mut(credential_ref).ok_or_else(not_found)?;
        value.revoked = true;
        value.generation = value.generation.saturating_add(1);
        Ok(())
    }

    fn lock(&self) -> StorageResult<std::sync::MutexGuard<'_, HashMap<String, StoredCredential>>> {
        self.state.lock().map_err(|_| {
            StorageError::new(
                StorageErrorCode::Unavailable,
                "credential vault state is poisoned",
                true,
            )
        })
    }
}

#[async_trait]
impl CredentialVaultPort for InMemoryCredentialVault {
    async fn lease(
        &self,
        credential_ref: &str,
        audience: &str,
        now_ms: u64,
        ttl_ms: u64,
    ) -> StorageResult<CredentialLease> {
        if audience.is_empty() || ttl_ms == 0 {
            return Err(StorageError::new(
                StorageErrorCode::Denied,
                "credential lease audience and TTL are required",
                false,
            ));
        }
        let state = self.lock()?;
        let value = state.get(credential_ref).ok_or_else(not_found)?;
        if value.revoked || value.audience != audience {
            return Err(StorageError::new(
                StorageErrorCode::Denied,
                "credential lease is revoked or has a different audience",
                false,
            ));
        }
        Ok(CredentialLease {
            credential_ref: credential_ref.to_owned(),
            access_key_id: value.access_key_id.clone(),
            secret_access_key: value.secret_access_key.clone(),
            session_token: value.session_token.clone(),
            generation: value.generation,
            expires_at_ms: now_ms.saturating_add(ttl_ms),
        })
    }
}

fn not_found() -> StorageError {
    StorageError::new(
        StorageErrorCode::NotFound,
        "credential reference not found",
        false,
    )
}
