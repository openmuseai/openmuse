use openmuse_cloud_execution_runtime::*;

fn image() -> RuntimeImageIdentity {
    RuntimeImageIdentity {
        image_digest: format!("sha256:{}", "a".repeat(64)),
        version: "2026.09.29".into(),
        sbom_digest: format!("sha256:{}", "b".repeat(64)),
    }
}

fn allocate(pool: &mut CloudRuntimePool, tenant: &str, task: &str) -> CloudRuntimeDescriptor {
    pool.allocate(CloudAllocationRequest {
        tenant_ref: tenant.into(),
        task_ref: task.into(),
        lease_ref: format!("lease:{task}"),
        checkout_handle_ref: format!("opaque:{task}"),
        image: image(),
        generation: 1,
        now_ms: 100,
        ttl_ms: 1_000,
    })
    .unwrap()
}

#[test]
fn provider_group_shares_one_execution_world() {
    let mut pool = CloudRuntimePool::default();
    let runtime = allocate(&mut pool, "tenant:a", "task:1");
    pool.fs_write(
        &runtime.runtime_ref,
        "tenant:a",
        "/workspace/a.txt",
        b"from-fs",
    )
    .unwrap();
    assert_eq!(
        pool.bash_read(&runtime.runtime_ref, "tenant:a", "/workspace/a.txt")
            .unwrap(),
        b"from-fs"
    );
    pool.bash_write(
        &runtime.runtime_ref,
        "tenant:a",
        "/workspace/b.txt",
        b"from-bash",
    )
    .unwrap();
    assert_eq!(
        pool.fs_read(&runtime.runtime_ref, "tenant:a", "/workspace/b.txt")
            .unwrap(),
        b"from-bash"
    );
    assert_eq!(
        pool.pty_round_trip(&runtime.runtime_ref, "tenant:a", b"hello")
            .unwrap(),
        b"hello"
    );
    let frame = b"Content-Length: 2\r\n\r\n{}";
    assert_eq!(
        pool.lsp_round_trip(&runtime.runtime_ref, "tenant:a", frame)
            .unwrap(),
        frame
    );
}

#[test]
fn outer_sandbox_cannot_be_bypassed_by_file_effect_escalation() {
    let mut pool = CloudRuntimePool::default();
    let runtime = allocate(&mut pool, "tenant:a", "task:1");
    for path in ["/etc/passwd", "/workspace/../etc/passwd", "relative"] {
        assert_eq!(
            pool.bash_write(&runtime.runtime_ref, "tenant:a", path, b"x")
                .unwrap_err(),
            CloudRuntimeError::PolicyDenied
        );
    }
}

#[test]
fn tenants_processes_volumes_and_attachments_are_isolated() {
    let mut pool = CloudRuntimePool::default();
    let a = allocate(&mut pool, "tenant:a", "task:1");
    let _b = allocate(&mut pool, "tenant:b", "task:2");
    pool.fs_write(&a.runtime_ref, "tenant:a", "/workspace/secret", b"a")
        .unwrap();
    assert_eq!(
        pool.fs_read(&a.runtime_ref, "tenant:b", "/workspace/secret")
            .unwrap_err(),
        CloudRuntimeError::PolicyDenied
    );
    let token = pool
        .attach(&a.runtime_ref, "tenant:a", "dsh", 1, 200, 100)
        .unwrap();
    assert_eq!(
        pool.authenticate(&token.token_ref, "tenant:b", "dsh", 1, 250)
            .unwrap_err(),
        CloudRuntimeError::PolicyDenied
    );
    assert_eq!(
        pool.authenticate(&token.token_ref, "tenant:a", "worker", 1, 250)
            .unwrap_err(),
        CloudRuntimeError::PolicyDenied
    );
    assert_eq!(
        pool.authenticate(&token.token_ref, "tenant:a", "dsh", 1, 300)
            .unwrap_err(),
        CloudRuntimeError::Expired
    );
}

#[test]
fn cancellation_crash_and_orphan_cleanup_are_measured() {
    let mut pool = CloudRuntimePool::default();
    let cancelled = allocate(&mut pool, "tenant:a", "task:cancel");
    pool.spawn_process(&cancelled.runtime_ref, "tenant:a")
        .unwrap();
    pool.spawn_process(&cancelled.runtime_ref, "tenant:a")
        .unwrap();
    assert_eq!(
        pool.end(&cancelled.runtime_ref, RuntimeEnd::Cancelled)
            .unwrap(),
        2
    );
    let crashed = allocate(&mut pool, "tenant:a", "task:crash");
    pool.spawn_process(&crashed.runtime_ref, "tenant:a")
        .unwrap();
    pool.end(&crashed.runtime_ref, RuntimeEnd::Crashed).unwrap();
    let orphan = allocate(&mut pool, "tenant:a", "task:orphan");
    pool.spawn_process(&orphan.runtime_ref, "tenant:a").unwrap();
    assert_eq!(pool.sweep_orphans(500, 300), vec![orphan.runtime_ref]);
    let metrics = pool.metrics();
    assert_eq!(metrics.allocations, 3);
    assert_eq!(metrics.cold_starts, 3);
    assert_eq!(metrics.cancellations, 1);
    assert_eq!(metrics.crashes, 1);
    assert_eq!(metrics.orphan_cleanups, 1);
    assert_eq!(metrics.processes_terminated, 4);
}

#[test]
fn image_and_sbom_are_pinned_and_checkout_stays_opaque() {
    let mut pool = CloudRuntimePool::default();
    let runtime = allocate(&mut pool, "tenant:a", "task:1");
    assert!(pool.opaque_checkout_bound(&runtime.runtime_ref));
    assert_eq!(runtime.image, image());
    let encoded = serde_json::to_string(&runtime).unwrap();
    assert!(!encoded.contains("opaque:task:1"));
    assert!(!encoded.contains("/Users/"));
}
