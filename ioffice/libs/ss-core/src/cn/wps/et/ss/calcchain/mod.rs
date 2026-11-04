pub mod FormulaHelper;
pub mod CalcChainNode;
pub mod ThreadCalcStack;
pub mod CellFmlEvaluator;
pub mod CalcChain;

// java CalcChain.recalcLock = new ReentrantLock();
// Shared with CellRefMap and Static*RefMap. A plain Mutex would deadlock when
// the calculating thread enters a nested Java lock site.
pub static recalcLock: RecalcLock = RecalcLock::new();

struct LockState {
    owner: Option<std::thread::ThreadId>,
    depth: usize,
}

pub struct RecalcLock {
    state: std::sync::Mutex<LockState>,
    released: std::sync::Condvar,
}

pub struct RecalcLockGuard<'a> {
    lock: &'a RecalcLock,
    // Java unlock is performed by the owning thread.
    _not_send: std::marker::PhantomData<std::rc::Rc<()>>,
}

impl RecalcLock {
    pub const fn new() -> Self {
        Self {
            state: std::sync::Mutex::new(LockState { owner: None, depth: 0 }),
            released: std::sync::Condvar::new(),
        }
    }

    // java recalcLock.lock()
    pub fn lock(&self) -> RecalcLockGuard<'_> {
        let current = std::thread::current().id();
        let mut state = self.state.lock().unwrap();
        // java ReentrantLock waits for a different owner, but allows its owner to reenter.
        while state.owner.is_some() && state.owner != Some(current) {
            state = self.released.wait(state).unwrap();
        }
        state.owner = Some(current);
        state.depth += 1;
        RecalcLockGuard { lock: self, _not_send: std::marker::PhantomData }
    }
}

impl Drop for RecalcLockGuard<'_> {
    fn drop(&mut self) {
        // java recalcLock.unlock() in finally.
        let mut state = self.lock.state.lock().unwrap();
        assert_eq!(state.owner, Some(std::thread::current().id()));
        state.depth -= 1;
        if state.depth == 0 {
            state.owner = None;
            self.lock.released.notify_one();
        }
    }
}

pub mod threadpool;
pub mod listener;
pub mod marktype;
pub mod NameFmlEvaluator;
