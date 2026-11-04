use async_trait::async_trait;
use openmuse_storage_contract::{BlobDigest, BlobReadStream, DigestAlgorithm};
use openmuse_storage_tck::VecBlobStream;
use openmuse_workspace_sync::{
    CloudChange, CloudChangePage, CloudCommitReceipt, CloudResourceSyncPort, ConflictDomain,
    InMemorySyncStateStore, LocalChange, LocalChangePage, LocalWorkspaceSyncPort, MigrationPhase,
    StagedCloudContent, SyncError, SyncErrorCode, SyncPolicy, SyncResult, SyncStateStorePort,
    WorkspaceAuthorityPlacement, WorkspaceSyncEngine, WorkspaceSyncState,
};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

fn digest(seed: char) -> BlobDigest {
    BlobDigest {
        algorithm: DigestAlgorithm::Sha256,
        value: std::iter::repeat_n(seed, 64).collect(),
    }
}

fn local_change(change_ref: &str, cursor: &str, revision: &str) -> LocalChange {
    LocalChange {
        change_ref: change_ref.to_owned(),
        cursor: cursor.to_owned(),
        workspace_ref: "workspace.a".to_owned(),
        resource_ref: "resource.document".to_owned(),
        local_revision: revision.to_owned(),
        expected_cloud_revision: Some("cloud.0".to_owned()),
        digest: digest('a'),
        size: 7,
        media_type: "text/plain".to_owned(),
    }
}

fn cloud_change(base_local_revision: &str, cloud_revision: &str) -> CloudChange {
    CloudChange {
        change_ref: "cloud-change.1".to_owned(),
        cursor: "cloud-cursor.1".to_owned(),
        workspace_ref: "workspace.a".to_owned(),
        resource_ref: "resource.document".to_owned(),
        cloud_revision: cloud_revision.to_owned(),
        base_local_revision: base_local_revision.to_owned(),
        digest: digest('b'),
        size: 8,
        media_type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            .to_owned(),
    }
}

#[derive(Default)]
struct FakeLocal {
    changes: Mutex<Vec<LocalChange>>,
    caught_up: Mutex<bool>,
    current_revisions: Mutex<HashMap<String, String>>,
    materialize_calls: Mutex<usize>,
    applied: Mutex<Vec<CloudChange>>,
}

impl FakeLocal {
    fn with_changes(changes: Vec<LocalChange>) -> Self {
        Self {
            changes: Mutex::new(changes),
            caught_up: Mutex::new(true),
            ..Self::default()
        }
    }

    fn set_current_revision(&self, resource_ref: &str, revision: &str) {
        self.current_revisions
            .lock()
            .unwrap()
            .insert(resource_ref.to_owned(), revision.to_owned());
    }
}

#[async_trait]
impl LocalWorkspaceSyncPort for FakeLocal {
    async fn changes_after(
        &self,
        _workspace_ref: &str,
        cursor: Option<&str>,
        limit: usize,
    ) -> SyncResult<LocalChangePage> {
        let changes = self.changes.lock().unwrap();
        let start = cursor
            .and_then(|cursor| changes.iter().position(|change| change.cursor == cursor))
            .map_or(0, |index| index + 1);
        Ok(LocalChangePage {
            changes: changes.iter().skip(start).take(limit).cloned().collect(),
            caught_up: *self.caught_up.lock().unwrap(),
        })
    }

    async fn materialize(&self, _change: &LocalChange) -> SyncResult<Box<dyn BlobReadStream>> {
        *self.materialize_calls.lock().unwrap() += 1;
        Ok(Box::new(VecBlobStream::new(b"content".to_vec(), 3)))
    }

    async fn current_revision(
        &self,
        _workspace_ref: &str,
        resource_ref: &str,
    ) -> SyncResult<String> {
        self.current_revisions
            .lock()
            .unwrap()
            .get(resource_ref)
            .cloned()
            .ok_or_else(|| SyncError::new(SyncErrorCode::Unavailable, "missing revision", false))
    }

    async fn apply_cloud_change(&self, change: &CloudChange) -> SyncResult<()> {
        self.applied.lock().unwrap().push(change.clone());
        self.current_revisions
            .lock()
            .unwrap()
            .insert(change.resource_ref.clone(), change.cloud_revision.clone());
        Ok(())
    }
}

#[derive(Default)]
struct FakeCloud {
    upload_calls: Mutex<Vec<String>>,
    commit_calls: Mutex<Vec<(String, bool)>>,
    changes_calls: Mutex<usize>,
    cloud_changes: Mutex<Vec<CloudChange>>,
    fail_commit_once: Mutex<bool>,
    verify_results: Mutex<VecDeque<bool>>,
    activate_calls: Mutex<usize>,
}

impl FakeCloud {
    fn fail_next_commit(&self) {
        *self.fail_commit_once.lock().unwrap() = true;
    }

    fn set_verify_results(&self, results: impl IntoIterator<Item = bool>) {
        *self.verify_results.lock().unwrap() = results.into_iter().collect();
    }
}

#[async_trait]
impl CloudResourceSyncPort for FakeCloud {
    async fn upload(
        &self,
        change: &LocalChange,
        body: &mut dyn BlobReadStream,
        idempotency_key: &str,
    ) -> SyncResult<StagedCloudContent> {
        while body
            .next_chunk(1024)
            .await
            .map_err(|error| {
                SyncError::new(SyncErrorCode::IntegrityFailed, error.to_string(), false)
            })?
            .is_some()
        {}
        self.upload_calls
            .lock()
            .unwrap()
            .push(idempotency_key.to_owned());
        Ok(StagedCloudContent {
            stage_ref: format!("stage.{}", change.change_ref),
            resource_ref: change.resource_ref.clone(),
            digest: change.digest.clone(),
            size: change.size,
            provider_receipt_ref: format!("provider.{}", change.change_ref),
        })
    }

    async fn commit(
        &self,
        change: &LocalChange,
        stage: &StagedCloudContent,
        idempotency_key: &str,
        cloud_writable: bool,
    ) -> SyncResult<CloudCommitReceipt> {
        self.commit_calls
            .lock()
            .unwrap()
            .push((idempotency_key.to_owned(), cloud_writable));
        let mut fail = self.fail_commit_once.lock().unwrap();
        if *fail {
            *fail = false;
            return Err(SyncError::new(
                SyncErrorCode::Unavailable,
                "cloud commit temporarily unavailable",
                true,
            ));
        }
        Ok(CloudCommitReceipt {
            receipt_ref: format!("receipt.{}", change.change_ref),
            resource_ref: change.resource_ref.clone(),
            cloud_revision: format!("cloud.{}", change.local_revision),
            provider_receipt_ref: stage.provider_receipt_ref.clone(),
            replayed: false,
        })
    }

    async fn changes_after(
        &self,
        _workspace_ref: &str,
        cursor: Option<&str>,
        limit: usize,
    ) -> SyncResult<CloudChangePage> {
        *self.changes_calls.lock().unwrap() += 1;
        let changes = self.cloud_changes.lock().unwrap();
        let start = cursor
            .and_then(|cursor| changes.iter().position(|change| change.cursor == cursor))
            .map_or(0, |index| index + 1);
        Ok(CloudChangePage {
            changes: changes.iter().skip(start).take(limit).cloned().collect(),
        })
    }

    async fn verify_migration(&self, _workspace_ref: &str) -> SyncResult<bool> {
        Ok(self
            .verify_results
            .lock()
            .unwrap()
            .pop_front()
            .unwrap_or(false))
    }

    async fn activate_cloud_authority(&self, _workspace_ref: &str) -> SyncResult<()> {
        *self.activate_calls.lock().unwrap() += 1;
        Ok(())
    }
}

fn fixture(
    local: Arc<FakeLocal>,
    cloud: Arc<FakeCloud>,
) -> (Arc<InMemorySyncStateStore>, WorkspaceSyncEngine) {
    let state = Arc::new(InMemorySyncStateStore::default());
    state
        .insert(WorkspaceSyncState::unconfigured("workspace.a"))
        .unwrap();
    let engine = WorkspaceSyncEngine::new(state.clone(), local, cloud);
    (state, engine)
}

#[test]
fn policies_expose_the_prd_direction_and_writability() {
    let local_only = SyncPolicy::LocalOnly.capabilities(MigrationPhase::NotApplicable);
    assert!(!local_only.uploads_local_changes);
    assert!(!local_only.downloads_cloud_changes);
    assert!(!local_only.cloud_writable);
    assert!(!local_only.mobile_available_when_desktop_offline);

    let snapshot = SyncPolicy::Snapshot.capabilities(MigrationPhase::NotApplicable);
    assert!(snapshot.uploads_local_changes);
    assert!(!snapshot.downloads_cloud_changes);
    assert!(!snapshot.cloud_writable);
    assert!(snapshot.mobile_available_when_desktop_offline);

    let mirror = SyncPolicy::Mirror.capabilities(MigrationPhase::NotApplicable);
    assert!(mirror.uploads_local_changes);
    assert!(mirror.downloads_cloud_changes);
    assert!(mirror.cloud_writable);
    assert!(mirror.mobile_available_when_desktop_offline);

    let migrating = SyncPolicy::Migrate.capabilities(MigrationPhase::Verifying);
    assert!(migrating.uploads_local_changes);
    assert!(!migrating.downloads_cloud_changes);
    assert!(!migrating.cloud_writable);
    assert!(!migrating.mobile_available_when_desktop_offline);
    let migrated = SyncPolicy::Migrate.capabilities(MigrationPhase::Complete);
    assert!(migrated.cloud_writable);
    assert!(migrated.mobile_available_when_desktop_offline);
}

#[test]
fn no_confirmed_policy_and_local_only_never_upload() {
    futures::executor::block_on(async {
        let local = Arc::new(FakeLocal::with_changes(vec![local_change(
            "change.1", "cursor.1", "local.1",
        )]));
        let cloud = Arc::new(FakeCloud::default());
        let (state, engine) = fixture(local.clone(), cloud.clone());

        let error = engine.run_local_cycle("workspace.a", 10).await.unwrap_err();
        assert_eq!(error.code, SyncErrorCode::PolicyNotConfirmed);
        assert!(cloud.upload_calls.lock().unwrap().is_empty());
        assert_eq!(*local.materialize_calls.lock().unwrap(), 0);

        let error = engine
            .configure_policy("workspace.a", SyncPolicy::Snapshot, false)
            .await
            .unwrap_err();
        assert_eq!(error.code, SyncErrorCode::PolicyNotConfirmed);
        engine
            .configure_policy("workspace.a", SyncPolicy::LocalOnly, true)
            .await
            .unwrap();
        assert_eq!(engine.run_local_cycle("workspace.a", 10).await.unwrap(), 0);
        assert!(cloud.upload_calls.lock().unwrap().is_empty());
        assert_eq!(
            state.load("workspace.a").await.unwrap().local_cursor,
            Some("cursor.1".to_owned())
        );
    });
}

#[test]
fn snapshot_is_one_way_read_only_and_emits_authoritative_receipt() {
    futures::executor::block_on(async {
        let local = Arc::new(FakeLocal::with_changes(vec![local_change(
            "change.1", "cursor.1", "local.1",
        )]));
        let cloud = Arc::new(FakeCloud::default());
        let (state, engine) = fixture(local, cloud.clone());
        engine
            .configure_policy("workspace.a", SyncPolicy::Snapshot, true)
            .await
            .unwrap();

        assert_eq!(engine.run_local_cycle("workspace.a", 10).await.unwrap(), 1);
        assert!(!cloud.commit_calls.lock().unwrap()[0].1);
        let snapshot = state.load("workspace.a").await.unwrap();
        assert_eq!(snapshot.local_cursor.as_deref(), Some("cursor.1"));
        assert_eq!(snapshot.receipts.len(), 1);
        assert_eq!(snapshot.receipts[0].cloud_revision, "cloud.local.1");

        let error = engine.run_cloud_cycle("workspace.a", 10).await.unwrap_err();
        assert_eq!(error.code, SyncErrorCode::Denied);
        assert_eq!(*cloud.changes_calls.lock().unwrap(), 0);
    });
}

#[test]
fn staged_upload_resumes_after_process_restart_without_duplicate_upload() {
    futures::executor::block_on(async {
        let local = Arc::new(FakeLocal::with_changes(vec![local_change(
            "change.1", "cursor.1", "local.1",
        )]));
        let cloud = Arc::new(FakeCloud::default());
        cloud.fail_next_commit();
        let (state, first_engine) = fixture(local.clone(), cloud.clone());
        first_engine
            .configure_policy("workspace.a", SyncPolicy::Mirror, true)
            .await
            .unwrap();
        let error = first_engine
            .run_local_cycle("workspace.a", 10)
            .await
            .unwrap_err();
        assert!(error.retryable);
        assert_eq!(cloud.upload_calls.lock().unwrap().len(), 1);
        assert_eq!(state.load("workspace.a").await.unwrap().work.len(), 1);

        let restarted = WorkspaceSyncEngine::new(state.clone(), local, cloud.clone());
        assert_eq!(
            restarted.run_local_cycle("workspace.a", 10).await.unwrap(),
            1
        );
        assert_eq!(cloud.upload_calls.lock().unwrap().len(), 1);
        let commits = cloud.commit_calls.lock().unwrap();
        assert_eq!(commits.len(), 2);
        assert_eq!(commits[0].0, commits[1].0);
    });
}

#[test]
fn mirror_split_brain_preserves_both_revisions_without_overwrite() {
    futures::executor::block_on(async {
        let local = Arc::new(FakeLocal::default());
        local.set_current_revision("resource.document", "local.diverged");
        let cloud = Arc::new(FakeCloud::default());
        cloud
            .cloud_changes
            .lock()
            .unwrap()
            .push(cloud_change("local.base", "cloud.diverged"));
        let (state, engine) = fixture(local.clone(), cloud);
        engine
            .configure_policy("workspace.a", SyncPolicy::Mirror, true)
            .await
            .unwrap();

        assert_eq!(engine.run_cloud_cycle("workspace.a", 10).await.unwrap(), 0);
        assert!(local.applied.lock().unwrap().is_empty());
        let snapshot = state.load("workspace.a").await.unwrap();
        assert_eq!(snapshot.conflicts.len(), 1);
        assert_eq!(snapshot.conflicts[0].local_revision, "local.diverged");
        assert_eq!(snapshot.conflicts[0].cloud_revision, "cloud.diverged");
        assert_eq!(snapshot.conflicts[0].domain, ConflictDomain::Office);
        assert_eq!(snapshot.cloud_cursor.as_deref(), Some("cloud-cursor.1"));
    });
}

#[test]
fn mirror_applies_cloud_change_only_when_base_revision_matches() {
    futures::executor::block_on(async {
        let local = Arc::new(FakeLocal::default());
        local.set_current_revision("resource.document", "local.base");
        let cloud = Arc::new(FakeCloud::default());
        cloud
            .cloud_changes
            .lock()
            .unwrap()
            .push(cloud_change("local.base", "cloud.1"));
        let (_, engine) = fixture(local.clone(), cloud);
        engine
            .configure_policy("workspace.a", SyncPolicy::Mirror, true)
            .await
            .unwrap();

        assert_eq!(engine.run_cloud_cycle("workspace.a", 10).await.unwrap(), 1);
        assert_eq!(local.applied.lock().unwrap().len(), 1);
    });
}

#[test]
fn migrate_keeps_local_authority_until_validation_then_switches_once() {
    futures::executor::block_on(async {
        let local = Arc::new(FakeLocal::with_changes(vec![local_change(
            "change.1", "cursor.1", "local.1",
        )]));
        let cloud = Arc::new(FakeCloud::default());
        cloud.set_verify_results([false, true]);
        let (state, engine) = fixture(local, cloud.clone());
        engine
            .configure_policy("workspace.a", SyncPolicy::Migrate, true)
            .await
            .unwrap();

        assert_eq!(engine.run_local_cycle("workspace.a", 10).await.unwrap(), 1);
        let verifying = state.load("workspace.a").await.unwrap();
        assert_eq!(verifying.migration_phase, MigrationPhase::Verifying);
        assert_eq!(verifying.authority, WorkspaceAuthorityPlacement::Local);
        assert!(
            !SyncPolicy::Migrate
                .capabilities(verifying.migration_phase)
                .cloud_writable
        );
        assert_eq!(*cloud.activate_calls.lock().unwrap(), 0);

        assert_eq!(engine.run_local_cycle("workspace.a", 10).await.unwrap(), 0);
        let complete = state.load("workspace.a").await.unwrap();
        assert_eq!(complete.migration_phase, MigrationPhase::Complete);
        assert_eq!(complete.authority, WorkspaceAuthorityPlacement::Cloud);
        assert_eq!(*cloud.activate_calls.lock().unwrap(), 1);

        assert_eq!(engine.run_local_cycle("workspace.a", 10).await.unwrap(), 0);
        assert_eq!(*cloud.activate_calls.lock().unwrap(), 1);
    });
}
