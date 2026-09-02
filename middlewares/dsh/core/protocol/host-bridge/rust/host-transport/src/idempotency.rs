use std::{
    collections::BTreeMap,
    sync::atomic::{AtomicU64, Ordering},
    sync::{Arc, Mutex},
};

use crate::TransportError;

#[derive(Debug, Clone)]
pub struct IdempotencyConfig {
    pub max_entries: usize,
    pub ttl_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdempotencyIdentity {
    pub runtime_instance_id: String,
    pub binding_id: String,
    pub operation_id: String,
    pub key: String,
    pub input_fingerprint: String,
}

enum Entry {
    Pending {
        fingerprint: String,
        execution_id: u64,
    },
    Complete {
        fingerprint: String,
        response: Vec<u8>,
        expires_at_ms: u64,
    },
}

struct Inner {
    config: IdempotencyConfig,
    counter: AtomicU64,
    entries: Mutex<BTreeMap<(String, String, String, String), Entry>>,
}

#[derive(Clone)]
pub struct IdempotencyRegistry {
    inner: Arc<Inner>,
}

pub enum IdempotencyDecision {
    Execute(IdempotencyPermit),
    Replay(Vec<u8>),
}

impl IdempotencyRegistry {
    pub fn new(config: IdempotencyConfig) -> Result<Self, TransportError> {
        if config.max_entries == 0 || config.ttl_ms == 0 {
            return Err(TransportError::InvalidFrame);
        }
        Ok(Self {
            inner: Arc::new(Inner {
                config,
                counter: AtomicU64::new(1),
                entries: Mutex::new(BTreeMap::new()),
            }),
        })
    }

    pub fn begin(
        &self,
        identity: IdempotencyIdentity,
        now_ms: u64,
    ) -> Result<IdempotencyDecision, TransportError> {
        let key = key_of(&identity);
        let mut entries = self
            .inner
            .entries
            .lock()
            .expect("idempotency state poisoned");
        entries.retain(|_, entry| !matches!(entry, Entry::Complete { expires_at_ms, .. } if *expires_at_ms <= now_ms));
        if let Some(entry) = entries.get(&key) {
            let fingerprint = match entry {
                Entry::Pending { fingerprint, .. } | Entry::Complete { fingerprint, .. } => {
                    fingerprint
                }
            };
            if fingerprint != &identity.input_fingerprint {
                return Err(TransportError::InvalidFrame);
            }
            return match entry {
                Entry::Pending { .. } => Err(TransportError::RateLimited),
                Entry::Complete { response, .. } => {
                    Ok(IdempotencyDecision::Replay(response.clone()))
                }
            };
        }
        if entries.len() >= self.inner.config.max_entries {
            let completed = entries.iter().find_map(|(key, entry)| {
                matches!(entry, Entry::Complete { .. }).then(|| key.clone())
            });
            if let Some(completed) = completed {
                entries.remove(&completed);
            } else {
                return Err(TransportError::RateLimited);
            }
        }
        let execution_id = self.inner.counter.fetch_add(1, Ordering::Relaxed);
        entries.insert(
            key.clone(),
            Entry::Pending {
                fingerprint: identity.input_fingerprint,
                execution_id,
            },
        );
        Ok(IdempotencyDecision::Execute(IdempotencyPermit {
            inner: self.inner.clone(),
            key,
            execution_id,
            completed: false,
        }))
    }
}

pub struct IdempotencyPermit {
    inner: Arc<Inner>,
    key: (String, String, String, String),
    execution_id: u64,
    completed: bool,
}

impl IdempotencyPermit {
    pub fn complete(mut self, response: Vec<u8>, now_ms: u64) -> Result<(), TransportError> {
        if response.is_empty() {
            return Err(TransportError::Handler);
        }
        let mut entries = self
            .inner
            .entries
            .lock()
            .expect("idempotency state poisoned");
        let fingerprint = match entries.get(&self.key) {
            Some(Entry::Pending {
                fingerprint,
                execution_id,
            }) if *execution_id == self.execution_id => fingerprint.clone(),
            _ => return Err(TransportError::Unavailable),
        };
        entries.insert(
            self.key.clone(),
            Entry::Complete {
                fingerprint,
                response,
                expires_at_ms: now_ms.saturating_add(self.inner.config.ttl_ms),
            },
        );
        self.completed = true;
        Ok(())
    }
}

impl Drop for IdempotencyPermit {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        let mut entries = self
            .inner
            .entries
            .lock()
            .expect("idempotency state poisoned");
        if matches!(entries.get(&self.key), Some(Entry::Pending { execution_id, .. }) if *execution_id == self.execution_id)
        {
            entries.remove(&self.key);
        }
    }
}

fn key_of(identity: &IdempotencyIdentity) -> (String, String, String, String) {
    (
        identity.runtime_instance_id.clone(),
        identity.binding_id.clone(),
        identity.operation_id.clone(),
        identity.key.clone(),
    )
}
