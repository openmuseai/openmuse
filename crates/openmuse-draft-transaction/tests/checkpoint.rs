use openmuse_draft_transaction::*;
use std::collections::BTreeMap;

fn draft() -> DraftTransaction {
    DraftTransaction::new(
        "draft:1",
        "workspace:1",
        "revision:base",
        BTreeMap::from([("known.txt".into(), b"before".to_vec())]),
    )
}

#[test]
fn content_scan_detects_changes_even_when_watcher_misses_them() {
    let mut draft = draft();
    draft.write("known.txt", b"after".to_vec(), false).unwrap();
    draft.write("new.txt", b"new".to_vec(), false).unwrap();
    draft.quiesce(false).unwrap();
    let prepared = draft.prepare().unwrap();
    assert_eq!(prepared.changes.len(), 2);
    assert_eq!(prepared.changes[0].path, "known.txt");
    assert_eq!(prepared.changes[1].path, "new.txt");
}

#[test]
fn active_background_writer_rejects_or_is_terminated_before_checkpoint() {
    let mut draft = draft();
    draft.set_active_writers(2);
    assert_eq!(draft.quiesce(false).unwrap_err(), DraftError::NotQuiescent);
    assert_eq!(draft.quiesce(true).unwrap(), 2);
    assert!(draft.prepare().is_ok());
}

#[test]
fn expected_base_cas_preserves_conflicting_draft() {
    let authority = InMemoryRevisionAuthority::new("workspace:1", "revision:base");
    let mut draft = draft();
    draft.write("new.txt", b"new".to_vec(), false).unwrap();
    draft.quiesce(false).unwrap();
    draft.prepare().unwrap();
    authority.move_head("workspace:1", "revision:concurrent");
    assert_eq!(
        draft.checkpoint(&authority).unwrap_err(),
        DraftError::Conflict
    );
    assert_eq!(draft.state(), DraftState::Conflict);
}

#[test]
fn uncertain_network_response_never_reports_false_success_and_retry_is_idempotent() {
    let authority = InMemoryRevisionAuthority::new("workspace:1", "revision:base");
    authority.fail_response_once();
    let mut draft = draft();
    draft.write("new.txt", b"new".to_vec(), false).unwrap();
    draft.quiesce(false).unwrap();
    draft.prepare().unwrap();
    assert_eq!(
        draft.checkpoint(&authority).unwrap_err(),
        DraftError::Unavailable
    );
    assert_eq!(draft.state(), DraftState::Prepared);
    let receipt = draft.checkpoint(&authority).unwrap();
    assert!(receipt.replayed);
    assert_eq!(draft.state(), DraftState::Committed);
}

#[test]
fn crash_recovery_restores_prepared_draft_and_commits_once() {
    let authority = InMemoryRevisionAuthority::new("workspace:1", "revision:base");
    let mut draft = draft();
    draft.remove("known.txt", false).unwrap();
    draft.quiesce(false).unwrap();
    let before = draft.prepare().unwrap();
    let mut recovered = DraftTransaction::recover(&draft.journal()).unwrap();
    let receipt = recovered.checkpoint(&authority).unwrap();
    assert_eq!(receipt.manifest_digest, before.manifest_digest);
    assert_eq!(recovered.state(), DraftState::Committed);
}
