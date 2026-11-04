#![allow(dead_code)]

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{LazyLock, Mutex};

use super::AtomException::AtomException;
use super::Transaction::Transaction;

/// Java `Object[]` backup slot for `@AtomMember` fields.
pub type AtomBackupValue = Option<Box<dyn Any + Send + Sync>>;

/// Java `@AtomMember` backup policy (`value()`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtomMemberPolicy {
    Share = 0,
    CloneOnBackup = 1,
}

impl From<i32> for AtomMemberPolicy {
    fn from(value: i32) -> Self {
        if value == 1 {
            Self::CloneOnBackup
        } else {
            Self::Share
        }
    }
}

/// Compile-time metadata replacing Java `Field` + `@AtomMember`.
#[derive(Clone, Copy, Debug)]
pub struct AtomFieldMeta {
    pub field_type: TypeId,
    pub policy: AtomMemberPolicy,
}

impl AtomFieldMeta {
    pub fn new<T: 'static>(policy: AtomMemberPolicy) -> Self {
        Self {
            field_type: TypeId::of::<T>(),
            policy,
        }
    }
}

static ATOM_FIELD_LAYOUTS: LazyLock<Mutex<HashMap<TypeId, Vec<AtomFieldMeta>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Register `@AtomMember` field layout for an Atom owner type (replaces Java `getAtomFields` cache).
pub fn register_atom_layout(owner: TypeId, fields: Vec<AtomFieldMeta>) {
    ATOM_FIELD_LAYOUTS
        .lock()
        .expect("atom layout registry poisoned")
        .insert(owner, fields);
}

/// Java `AtomImpl.getAtomFields(Class)` — returns registered layout or empty vec.
pub fn atom_field_layout(owner: TypeId) -> Vec<AtomFieldMeta> {
    ATOM_FIELD_LAYOUTS
        .lock()
        .expect("atom layout registry poisoned")
        .get(&owner)
        .cloned()
        .unwrap_or_default()
}

/// Java `Atom.getFieldIndex(Class, Class)` — match by field **type**, not name.
pub fn atom_field_index(owner: TypeId, field_type: TypeId) -> i32 {
    for (index, meta) in atom_field_layout(owner).iter().enumerate() {
        if meta.field_type == field_type {
            return index as i32;
        }
    }
    -1
}

/// Optional per-type field backup/swap hooks (implemented on concrete Atom owners).
pub trait AtomFieldOps {
    fn backup_atom_fields(&self) -> Option<Vec<AtomBackupValue>>;
    fn swap_atom_fields(
        &mut self,
        data: &mut Vec<AtomBackupValue>,
    ) -> Result<(), AtomException>;
    fn propagate_atom_transaction(
        &mut self,
        transaction: Option<Rc<RefCell<Transaction>>>,
    );
}

/// Java `AtomImpl.cloneObject` — `Clone` when `@AtomMember(1)`, otherwise share.
pub fn clone_for_atom_backup(value: &dyn Any) -> AtomBackupValue {
    clone_any_if_possible(value)
}

fn clone_any_if_possible(value: &dyn Any) -> AtomBackupValue {
    macro_rules! try_clone {
        ($t:ty) => {
            if let Some(v) = value.downcast_ref::<$t>() {
                return Some(Box::new(v.clone()));
            }
        };
    }
    try_clone!(String);
    try_clone!(i32);
    try_clone!(i64);
    try_clone!(bool);
    None
}

pub fn backup_value(
    original: AtomBackupValue,
    policy: AtomMemberPolicy,
) -> AtomBackupValue {
    match policy {
        AtomMemberPolicy::Share => original,
        AtomMemberPolicy::CloneOnBackup => match &original {
            Some(value) => clone_for_atom_backup(value.as_ref()).or(original),
            None => None,
        },
    }
}
