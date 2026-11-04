#![allow(non_snake_case, dead_code, unused_variables)]

use super::atom_reflect::AtomBackupValue;
use super::IUndoable::IUndoable;
use super::Transaction::Transaction;

// Java: public class IUndoableImpl implements IUndoable, Cloneable
// rust: ★M-STAGE2 P1-D1 / T6 —— 补 `Default`：`Atom` 现 `#[derive(Clone, Default)]`，
//   其字段 `base: IUndoableImpl` 需同样实现 `Default`（Java 侧为隐式无参构造器）。
#[derive(Clone, Default)]
pub struct IUndoableImpl;

// Java: public class IUndoableImpl implements IUndoable, Cloneable
impl IUndoableImpl {
    // Java: IUndoableImpl()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: void onSwapped(Object[] prev, Object arg)
    pub fn onSwapped(&self, _prev: &[AtomBackupValue], _arg: &dyn std::any::Any) {
    }

    // Java: void writeLockAtom(Transaction transaction)
    pub fn writeLockAtom(&self, transaction: Option<&mut Transaction>) {
        // Java: if (transaction != null)
        if let Some(transaction) = transaction {
            // Java: transaction.writeLockAtom(this);
            transaction.writeLockAtom_no_arg(self);
        }
    }
}

impl IUndoable for IUndoableImpl {
    fn onSwapped(&self, items: &[AtomBackupValue], target: &dyn std::any::Any) {
        IUndoableImpl::onSwapped(self, items, target);
    }
}
