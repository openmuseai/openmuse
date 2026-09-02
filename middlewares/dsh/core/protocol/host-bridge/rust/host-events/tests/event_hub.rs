use muse_host_events::{EventHubConfig, EventHubError, EventRead, HostEventHub};
use serde_json::json;

fn hub(retention: usize, subscriber_capacity: usize) -> HostEventHub {
    HostEventHub::new(EventHubConfig {
        retention,
        subscriber_capacity,
        max_payload_bytes: 1024,
    })
    .unwrap()
}

#[tokio::test]
async fn replay_is_ordered_and_cursor_expiry_is_explicit() {
    let hub = hub(2, 8);
    hub.publish(1, json!({"event": 1})).unwrap();
    hub.publish(2, json!({"event": 2})).unwrap();
    hub.publish(3, json!({"event": 3})).unwrap();
    assert_eq!(hub.head_cursor(), 3);
    assert!(matches!(hub.subscribe(Some(0)), Err(EventHubError::CursorExpired)));

    let mut subscription = hub.subscribe(Some(1)).unwrap();
    assert_eq!(subscription.retention_from_cursor, 2);
    assert!(matches!(subscription.next().await, EventRead::Event(record) if record.cursor == 2));
    assert!(matches!(subscription.next().await, EventRead::Event(record) if record.cursor == 3));
}

#[tokio::test]
async fn a_slow_subscriber_observes_a_gap_without_blocking_publish() {
    let hub = hub(32, 2);
    let mut subscription = hub.subscribe(None).unwrap();
    for value in 0..8 {
        hub.publish(value, json!({"event": value})).unwrap();
    }
    assert!(matches!(subscription.next().await, EventRead::Gap { missed } if missed > 0));
}

#[test]
fn payload_and_configuration_are_bounded() {
    assert!(HostEventHub::new(EventHubConfig {
        retention: 0,
        subscriber_capacity: 1,
        max_payload_bytes: 1,
    })
    .is_err());
    let hub = HostEventHub::new(EventHubConfig {
        retention: 1,
        subscriber_capacity: 1,
        max_payload_bytes: 8,
    })
    .unwrap();
    assert_eq!(
        hub.publish(1, json!({"payload": "too large"})),
        Err(EventHubError::InvalidPayload)
    );
}

#[test]
fn metrics_are_content_free_and_bounded() {
    let hub = HostEventHub::new(EventHubConfig {
        retention: 1,
        subscriber_capacity: 1,
        max_payload_bytes: 16,
    })
    .unwrap();
    hub.publish(1, json!({"ok": true})).unwrap();
    assert!(hub.publish(2, json!({"secret": "this payload is too large"})).is_err());
    let metrics = hub.metrics();
    assert_eq!(metrics.published, 1);
    assert_eq!(metrics.rejected, 1);
    assert_eq!(metrics.retained, 1);
}
