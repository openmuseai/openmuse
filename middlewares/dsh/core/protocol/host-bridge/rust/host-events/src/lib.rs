use std::{
    collections::VecDeque,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

use serde_json::Value;
use tokio::sync::broadcast;

#[derive(Debug, Clone, PartialEq)]
pub struct HostEventRecord {
    pub cursor: u64,
    pub occurred_at_ms: u64,
    pub data: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventHubConfig {
    pub retention: usize,
    pub subscriber_capacity: usize,
    pub max_payload_bytes: usize,
}

impl Default for EventHubConfig {
    fn default() -> Self {
        Self {
            retention: 1024,
            subscriber_capacity: 256,
            max_payload_bytes: 256 * 1024,
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum EventHubError {
    #[error("event payload is invalid or exceeds the configured bound")]
    InvalidPayload,
    #[error("event cursor is outside the retained window")]
    CursorExpired,
}

struct State {
    next_cursor: u64,
    records: VecDeque<HostEventRecord>,
}

struct Inner {
    config: EventHubConfig,
    state: Mutex<State>,
    live: broadcast::Sender<HostEventRecord>,
    published: AtomicU64,
    rejected: AtomicU64,
    subscriber_gaps: AtomicU64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HostEventMetrics {
    pub published: u64,
    pub rejected: u64,
    pub subscriber_gaps: u64,
    pub head_cursor: u64,
    pub retained: usize,
}

#[derive(Clone)]
pub struct HostEventHub {
    inner: Arc<Inner>,
}

impl HostEventHub {
    pub fn new(config: EventHubConfig) -> Result<Self, EventHubError> {
        if config.retention == 0
            || config.subscriber_capacity == 0
            || config.max_payload_bytes == 0
        {
            return Err(EventHubError::InvalidPayload);
        }
        let (live, _) = broadcast::channel(config.subscriber_capacity);
        Ok(Self {
            inner: Arc::new(Inner {
                config,
                state: Mutex::new(State {
                    next_cursor: 1,
                    records: VecDeque::with_capacity(config.retention),
                }),
                live,
                published: AtomicU64::new(0),
                rejected: AtomicU64::new(0),
                subscriber_gaps: AtomicU64::new(0),
            }),
        })
    }

    pub fn publish(
        &self,
        occurred_at_ms: u64,
        data: Value,
    ) -> Result<HostEventRecord, EventHubError> {
        let size = match serde_json::to_vec(&data) {
            Ok(bytes) => bytes.len(),
            Err(_) => {
                self.inner.rejected.fetch_add(1, Ordering::Relaxed);
                return Err(EventHubError::InvalidPayload);
            }
        };
        if size == 0 || size > self.inner.config.max_payload_bytes {
            self.inner.rejected.fetch_add(1, Ordering::Relaxed);
            return Err(EventHubError::InvalidPayload);
        }
        let record = {
            let mut state = self.inner.state.lock().expect("event hub state poisoned");
            let record = HostEventRecord {
                cursor: state.next_cursor,
                occurred_at_ms,
                data,
            };
            state.next_cursor = state.next_cursor.saturating_add(1);
            if state.records.len() == self.inner.config.retention {
                state.records.pop_front();
            }
            state.records.push_back(record.clone());
            record
        };
        let _ = self.inner.live.send(record.clone());
        self.inner.published.fetch_add(1, Ordering::Relaxed);
        Ok(record)
    }

    pub fn subscribe(&self, after_cursor: Option<u64>) -> Result<HostEventSubscription, EventHubError> {
        let receiver = self.inner.live.subscribe();
        let state = self.inner.state.lock().expect("event hub state poisoned");
        let head_cursor = state.next_cursor.saturating_sub(1);
        let retention_from_cursor = state.records.front().map_or(head_cursor, |record| record.cursor);
        if let Some(after) = after_cursor {
            if after > head_cursor || (after < retention_from_cursor.saturating_sub(1)) {
                return Err(EventHubError::CursorExpired);
            }
        }
        let backlog = state
            .records
            .iter()
            .filter(|record| after_cursor.is_none_or(|after| record.cursor > after))
            .cloned()
            .collect();
        Ok(HostEventSubscription {
            backlog,
            receiver,
            head_cursor,
            retention_from_cursor,
            metrics: self.inner.clone(),
        })
    }

    pub fn head_cursor(&self) -> u64 {
        self.inner
            .state
            .lock()
            .expect("event hub state poisoned")
            .next_cursor
            .saturating_sub(1)
    }

    pub fn metrics(&self) -> HostEventMetrics {
        let state = self.inner.state.lock().expect("event hub state poisoned");
        HostEventMetrics {
            published: self.inner.published.load(Ordering::Relaxed),
            rejected: self.inner.rejected.load(Ordering::Relaxed),
            subscriber_gaps: self.inner.subscriber_gaps.load(Ordering::Relaxed),
            head_cursor: state.next_cursor.saturating_sub(1),
            retained: state.records.len(),
        }
    }
}

pub enum EventRead {
    Event(HostEventRecord),
    Gap { missed: u64 },
    Closed,
}

pub struct HostEventSubscription {
    backlog: VecDeque<HostEventRecord>,
    receiver: broadcast::Receiver<HostEventRecord>,
    pub head_cursor: u64,
    pub retention_from_cursor: u64,
    metrics: Arc<Inner>,
}

impl HostEventSubscription {
    pub async fn next(&mut self) -> EventRead {
        if let Some(record) = self.backlog.pop_front() {
            return EventRead::Event(record);
        }
        match self.receiver.recv().await {
            Ok(record) => EventRead::Event(record),
            Err(broadcast::error::RecvError::Lagged(missed)) => {
                self.metrics.subscriber_gaps.fetch_add(1, Ordering::Relaxed);
                EventRead::Gap { missed }
            }
            Err(broadcast::error::RecvError::Closed) => EventRead::Closed,
        }
    }
}
