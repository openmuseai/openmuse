// Java: public interface IUndoable
use super::atom_reflect::AtomBackupValue;
use super::AtomException::AtomException;
use super::Transaction::Transaction;
use std::cell::RefCell;
use std::rc::Rc;

pub trait IUndoable: std::any::Any {
    // Java: abstract void onSwapped(Object[] var1, Object var2)
    fn onSwapped(&self, items: &[AtomBackupValue], target: &dyn std::any::Any);

    /// Replaces Java reflective `AtomImpl.backupAtomData`.
    fn backup_atom_fields(&self) -> Option<Vec<AtomBackupValue>> {
        None
    }

    /// Replaces Java reflective `AtomImpl.swapAtom`.
    fn swap_atom_fields(&mut self, _data: &mut Vec<AtomBackupValue>) -> Result<(), AtomException> {
        Err(AtomException::new())
    }

    /// Replaces Java reflective nested-`Atom` propagation in `setTransaction`.
    fn propagate_atom_transaction(&mut self, _transaction: Option<Rc<RefCell<Transaction>>>) {}
}
