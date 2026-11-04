use ss_core::cn::wps::et::ss::calcchain::recalcLock;

#[test]
fn java_calc_chain_lock_allows_same_thread_reentry() {
    let _outer = recalcLock.lock();
    let _inner = recalcLock.lock();
}
