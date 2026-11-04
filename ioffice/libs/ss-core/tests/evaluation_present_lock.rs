use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

use ss_core::cn::wps::moss::engine::eval::EvaluationPresent::EvaluationPresent;

#[test]
fn reentrant_lock_tracks_owner_and_blocks_other_threads() {
    let present = Arc::new(EvaluationPresent::new());
    present.lock();
    present.lock();
    assert!(present.isLock());
    let (ready_tx, ready_rx) = mpsc::channel();
    let (acquired_tx, acquired_rx) = mpsc::channel();
    let worker_present = present.clone();
    let worker = thread::spawn(move || {
        ready_tx.send(()).unwrap();
        worker_present.lock();
        acquired_tx.send(()).unwrap();
        worker_present.unlock();
    });
    ready_rx.recv().unwrap();
    assert!(acquired_rx.recv_timeout(Duration::from_millis(30)).is_err());
    present.unlock();
    assert!(present.isLock());
    assert!(acquired_rx.recv_timeout(Duration::from_millis(30)).is_err());
    present.unlock();
    acquired_rx.recv_timeout(Duration::from_secs(1)).unwrap();
    worker.join().unwrap();
    assert!(!present.isLock());
}
