//! Rust ownership for Java object references and mutable fields.
//!
//! Java's workbook, formula and snapshot references may cross calculation
//! threads. Arc retains the same object identity; these field containers use
//! locks so that sharing that graph requires no unsafe Send impl.

pub use std::sync::{Arc as Rc, Weak};
pub mod DecimalText;
pub mod Charset;
pub use kfc_kfc_alpha::native_runtime::{Date,JavaDateArg,currentTimeMillis,withClock};
pub use parking_lot::{MappedRwLockReadGuard as Ref, MappedRwLockWriteGuard as RefMut};
use parking_lot::{Mutex, RwLock};

#[derive(Default, Debug)]
pub struct RefCell<T: ?Sized> {
    value: RwLock<T>,
}

impl<T> RefCell<T> {
    pub const fn new(value: T) -> Self { Self { value: RwLock::new(value) } }
    pub fn into_inner(self) -> T { self.value.into_inner() }
}

impl<T: ?Sized> RefCell<T> {
    // Identity only; dereferencing this pointer still requires the normal guards.
    pub fn as_ptr(&self) -> *mut T { self.value.data_ptr() }
    pub fn borrow(&self) -> Ref<'_, T> { parking_lot::RwLockReadGuard::map(self.value.read_recursive(), |value| value) }
    pub fn borrow_mut(&self) -> RefMut<'_, T> { parking_lot::RwLockWriteGuard::map(self.value.write(), |value| value) }
    pub fn get_mut(&mut self) -> &mut T { self.value.get_mut() }
}

impl<T: Clone> Clone for RefCell<T> {
    fn clone(&self) -> Self { Self::new(self.borrow().clone()) }
}

#[derive(Default, Debug)]
pub struct Cell<T> {
    value: Mutex<T>,
}

impl<T> Cell<T> {
    pub const fn new(value: T) -> Self { Self { value: Mutex::new(value) } }
    pub fn set(&self, value: T) { *self.value.lock() = value; }
    pub fn replace(&self, value: T) -> T { std::mem::replace(&mut *self.value.lock(), value) }
}

impl<T: Copy> Cell<T> {
    pub fn get(&self) -> T { *self.value.lock() }
}

impl<T: Copy> Clone for Cell<T> {
    fn clone(&self) -> Self { Self::new(self.get()) }
}
