use std::ops::Deref;

pub struct Externalizable;

// Java: public interface PropBaseSerializable extends Externalizable
pub struct PropBaseSerializable {
    pub(crate) _externalizable: Externalizable,
}

impl Deref for PropBaseSerializable {
    type Target = Externalizable;
    fn deref(&self) -> &Self::Target {
        &self._externalizable
    }
}

// Java: public interface PropBaseSerializable extends Externalizable
pub trait PropBaseSerializableTrait {}
