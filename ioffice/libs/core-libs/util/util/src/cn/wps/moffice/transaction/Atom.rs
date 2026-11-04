#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.lang.reflect.Field
use super::atom_reflect::atom_field_index;
use super::atom_reflect::AtomFieldOps;
use super::IUndoableImpl::IUndoableImpl;
use super::Transaction::Transaction;
use std::any::TypeId;
use std::cell::RefCell;
use std::rc::Rc;

// Java: public class Atom extends IUndoableImpl
// rust: ★M-STAGE2 P1-D1 / T6 —— 补 `Clone` + `Default`。
//   Java 侧 `Atom` 的子类（`LfoLvlData` / `LSTF` / `LFO` 等）声明 `implements Cloneable`
//   并调用 `super.clone()`（即 `Object.clone()` 浅拷贝）；Rust 侧 `IUndoableImpl`
//   已 `#[derive(Clone)]`，`Option<Rc<RefCell<Transaction>>>` 亦可克隆 ⇒ 直接派生。
//   `Default` 对应 Java 的隐式无参构造器 `Atom()`。
#[derive(Clone, Default)]
pub struct Atom {
    pub base: IUndoableImpl,
    // Java field: Transaction mTransaction
    pub(crate) mTransaction: Option<Rc<RefCell<Transaction>>>,
}

// Java: public class Atom extends IUndoableImpl
impl Atom {
    // Java: Atom()
    pub fn new() -> Self {
        // Java: super();
        Self {
            base: IUndoableImpl::new(),
            mTransaction: None,
        }
    }

    // Java: Atom(Transaction transaction)
    pub fn new_with_transaction(transaction: Option<Rc<RefCell<Transaction>>>) -> Self {
        // Java: super();
        // Java: this.mTransaction = transaction;
        Self {
            base: IUndoableImpl::new(),
            mTransaction: transaction,
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: if (this.mTransaction != null)
        if let Some(ref t) = self.mTransaction
        {
            // Java: this.mTransaction.removeListener();
            t.borrow_mut().removeListener();
            // Java: this.mTransaction = null;
        }
        self.mTransaction = None;
    }

    // Java: int getFieldIndex(Class<?> pClass, Class<?> pField)
    pub fn getFieldIndex(&self, p_class: TypeId, p_field: TypeId) -> i32 {
        // Java: Field[] pFields = AtomImpl.getAtomFields(pClass);
        let _p_fields = super::AtomImpl::AtomImpl::get_atom_fields(p_class);
        // Java: for(int i = 0; i < pFields.length; ++i)
        // Java: Field field = pFields[i];
        // Java: if (field.getType() == pField)
        // Java: return i;
        atom_field_index(p_class, p_field)
        // Java: return -1;
    }

    // Java: Transaction getTransaction()
    pub fn getTransaction(&self) -> Option<Rc<RefCell<Transaction>>> {
        // Java: return this.mTransaction;
        self.mTransaction.clone()
    }

    /// Convenience for callers that match Java's non-null getTransaction() chaining.
    pub fn transaction(&self) -> Rc<RefCell<Transaction>> {
        self.mTransaction
            .clone()
            .expect("Atom.mTransaction is null")
    }

    // Java: void setTransaction(Transaction transaction)
    pub fn setTransaction(&mut self, transaction: Option<Rc<RefCell<Transaction>>>) {
        // Java: this.setTransaction(transaction, true);
        self.setTransaction_recursive(transaction, true);
    }

    // Java: void setTransaction(Transaction transaction, boolean bRecursive)
    pub fn setTransaction_recursive(
        &mut self,
        transaction: Option<Rc<RefCell<Transaction>>>,
        b_recursive: bool,
    ) {
        // Java: if (this.mTransaction != transaction)
        let same = match (&self.mTransaction, &transaction) {
            (Some(a), Some(b)) => Rc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        };
        if !same {
            // Java: this.mTransaction = transaction;
            self.mTransaction = transaction;
        }
        let _ = b_recursive;
    }

    /// Leaf Atom owners call this to also propagate into nested `Atom` `@AtomMember` fields.
    pub fn setTransaction_recursive_with_owner<A: AtomFieldOps>(
        &mut self,
        owner: &mut A,
        transaction: Option<Rc<RefCell<Transaction>>>,
        b_recursive: bool,
    ) {
        let same = match (&self.mTransaction, &transaction) {
            (Some(a), Some(b)) => Rc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        };
        if !same {
            self.mTransaction = transaction.clone();
            if b_recursive {
                owner.propagate_atom_transaction(transaction);
            }
        }
    }

    // Java: void writeLockAtom()
    pub fn writeLockAtom(&mut self) {
        // Java: if (this.mTransaction != null)
        if let Some(t) = self.mTransaction.clone()
        {
            // Java: this.mTransaction.writeLockAtom(this);
            t.borrow_mut()
                .writeLockAtom_no_arg(self as &dyn std::any::Any);
        }
    }
}
