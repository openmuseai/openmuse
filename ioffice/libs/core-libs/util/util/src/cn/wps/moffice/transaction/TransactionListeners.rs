// Java: import java.util.ArrayList
use super::Transaction::Listener;

// Java: public class TransactionListeners implements Transaction.Listener

// Java: public class TransactionListeners implements Transaction.Listener
pub struct TransactionListeners {
    // Java field: ArrayList<Transaction.Listener> mListeners
    mListeners: Vec<Box<dyn Listener>>,
}

// Java: public class TransactionListeners implements Transaction.Listener
impl TransactionListeners {
    // Java: TransactionListeners()
    pub fn new() -> Self {
        // Java: super();
        TransactionListeners { mListeners: Vec::new() }
    }

    // Java: void add(Transaction.Listener listener)
    pub fn add(&mut self, listener: Box<dyn Listener>) {
        // Java: if (!this.mListeners.contains(listener))
        // Java: this.mListeners.add(listener);
        self.mListeners.push(listener);
    }

    // Java: void remove(Transaction.Listener listener)
    pub fn remove(&mut self, index: usize) {
        // Java: this.mListeners.remove(listener);
        if index < self.mListeners.len() {
            self.mListeners.remove(index);
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mListeners.clear();
        self.mListeners.clear();
    }

    // Java: void beforeStart(String description)
    pub fn beforeStart(&mut self, description: &str) {
        // Java: for (Listener listener : this.mListeners) listener.beforeStart(description);
        for listener in self.mListeners.iter_mut() {
            listener.beforeStart(description);
        }
    }

    // Java: void afterCommit(String description)
    pub fn afterCommit(&mut self, description: &str) {
        for listener in self.mListeners.iter_mut() {
            listener.afterCommit(description);
        }
    }

    // Java: void afterRollback()
    pub fn afterRollback(&mut self) {
        for listener in self.mListeners.iter_mut() {
            listener.afterRollback();
        }
    }

    // Java: void beforeUndo(String description)
    pub fn beforeUndo(&mut self, description: &str) {
        for listener in self.mListeners.iter_mut() {
            listener.beforeUndo(description);
        }
    }

    // Java: void afterUndo(String description)
    pub fn afterUndo(&mut self, description: &str) {
        for listener in self.mListeners.iter_mut() {
            listener.afterUndo(description);
        }
    }

    // Java: void beforeRedo(String description)
    pub fn beforeRedo(&mut self, description: &str) {
        for listener in self.mListeners.iter_mut() {
            listener.beforeRedo(description);
        }
    }

    // Java: void afterRedo(String description)
    pub fn afterRedo(&mut self, description: &str) {
        for listener in self.mListeners.iter_mut() {
            listener.afterRedo(description);
        }
    }
}

impl Listener for TransactionListeners {
    fn beforeStart(&mut self, description: &str) {
        TransactionListeners::beforeStart(self, description);
    }
    fn afterCommit(&mut self, description: &str) {
        TransactionListeners::afterCommit(self, description);
    }
    fn afterRollback(&mut self) {
        TransactionListeners::afterRollback(self);
    }
    fn beforeUndo(&mut self, description: &str) {
        TransactionListeners::beforeUndo(self, description);
    }
    fn afterUndo(&mut self, description: &str) {
        TransactionListeners::afterUndo(self, description);
    }
    fn beforeRedo(&mut self, description: &str) {
        TransactionListeners::beforeRedo(self, description);
    }
    fn afterRedo(&mut self, description: &str) {
        TransactionListeners::afterRedo(self, description);
    }
    fn addListener(&mut self, listener: Box<dyn Listener>) {
        // Java: this.mListeners.add(listener);
        self.add(listener);
    }
}
