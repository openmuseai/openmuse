use muse_host_context::{
    ContextInput, ContextStoreConfig, ContextStoreError, SurfaceBinding, SurfaceContextStore,
    SurfaceLifecycle,
};
use muse_host_events::{EventHubConfig, HostEventHub};
use serde_json::json;

fn store() -> SurfaceContextStore {
    SurfaceContextStore::new(
        ContextStoreConfig {
            max_surfaces: 4,
            max_contexts_per_surface: 2,
            max_payload_bytes: 1024,
            max_ttl_ms: 100,
        },
        HostEventHub::new(EventHubConfig::default()).unwrap(),
    )
    .unwrap()
}

fn binding(resource: &str) -> SurfaceBinding {
    SurfaceBinding {
        plugin_id: "muse.test".into(),
        facet_instance_ref: format!("facet.{resource}"),
        surface_kind: "test.editor".into(),
        actor_ref: "actor.1".into(),
        workspace_ref: "workspace.1".into(),
        scope_ref: resource.into(),
        window_ref: "window.1".into(),
    }
}

fn context(revision: u64, epoch: &str) -> ContextInput {
    ContextInput {
        context_type: "test.selection".into(),
        schema_digest: format!("sha256:{}", "a".repeat(64)),
        revision,
        epoch_ref: epoch.into(),
        ttl_ms: 1_000,
        payload: json!({"opaque": true}),
    }
}

#[test]
fn latest_wins_ttl_is_clamped_and_epoch_resets_revision() {
    let store = store();
    let lease = store.bind_surface(binding("doc.1"), 1).unwrap();
    store
        .publish(
            &lease,
            "actor.1",
            "workspace.1",
            "doc.1",
            context(2, "epoch.1"),
            10,
        )
        .unwrap();
    assert_eq!(
        store.publish(
            &lease,
            "actor.1",
            "workspace.1",
            "doc.1",
            context(1, "epoch.1"),
            11,
        ),
        Err(ContextStoreError::StaleRevision)
    );
    store
        .publish(
            &lease,
            "actor.1",
            "workspace.1",
            "doc.1",
            context(1, "epoch.2"),
            12,
        )
        .unwrap();
    assert_eq!(store.snapshot("actor.1", "workspace.1", None, 111).len(), 1);
    assert!(store.snapshot("actor.1", "workspace.1", None, 113)[0]
        .contexts
        .is_empty());
}

#[test]
fn actor_scope_and_lease_are_fail_closed() {
    let store = store();
    let lease = store.bind_surface(binding("doc.1"), 1).unwrap();
    assert_eq!(
        store.publish(
            &lease,
            "actor.2",
            "workspace.1",
            "doc.1",
            context(1, "epoch.1"),
            2,
        ),
        Err(ContextStoreError::ScopeMismatch)
    );
    store.close(&lease, 3).unwrap();
    assert_eq!(
        store.transition(&lease, SurfaceLifecycle::Active, 4),
        Err(ContextStoreError::InvalidLease)
    );
}

#[test]
fn focus_is_exclusive_per_workspace_window_and_events_are_observable() {
    let events = HostEventHub::new(EventHubConfig::default()).unwrap();
    let store = SurfaceContextStore::new(ContextStoreConfig::default(), events.clone()).unwrap();
    let one = store.bind_surface(binding("doc.1"), 1).unwrap();
    let two = store.bind_surface(binding("doc.2"), 2).unwrap();
    store.transition(&one, SurfaceLifecycle::Focused, 3).unwrap();
    store.transition(&two, SurfaceLifecycle::Focused, 4).unwrap();
    let snapshots = store.snapshot("actor.1", "workspace.1", None, 4);
    assert_eq!(
        snapshots
            .iter()
            .filter(|surface| surface.lifecycle == SurfaceLifecycle::Focused)
            .count(),
        1
    );
    assert_eq!(events.head_cursor(), 4);
}
