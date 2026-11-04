// Java: public class Nullable
pub struct Nullable<E> {
    // Java field: E value
    pub(crate) value: Option<E>,
}

// Java: public class Nullable
impl<E> Nullable<E> {
    // Java: Nullable()
    pub fn new() -> Self {
        // Java: super();
        Nullable { value: None }
    }

    // Java: Nullable(E value)
    pub fn new_with_value(value: E) -> Self {
        // Java: super();
        Nullable {
            // Java: this.value = value;
            value: Some(value),
        }
    }

    // Java: E getValue()
    pub fn getValue(&self) -> Option<&E> {
        // Java: return this.value;
        self.value.as_ref()
    }

    // Java: boolean hasValue()
    pub fn hasValue(&self) -> bool {
        // Java: return this.value != null;
        self.value.is_some()
    }

    // Java: void nullify()
    pub fn nullify(&mut self) {
        // Java: this.value = null;
        self.value = None;
    }
}
