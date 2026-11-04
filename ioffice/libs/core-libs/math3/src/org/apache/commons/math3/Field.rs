// Java: public interface will
#[allow(non_camel_case_types)]
pub trait will {
}

// Java: public interface Field
pub trait Field {
    type T;

    // Java: abstract T getZero()
    fn getZero(&self) -> Self::T;

    // Java: abstract T getOne()
    fn getOne(&self) -> Self::T;

    // Java: abstract Class<? extends FieldElement<T>> getRuntimeClass()
    fn getRuntimeClass(&self) -> std::any::TypeId;
}
