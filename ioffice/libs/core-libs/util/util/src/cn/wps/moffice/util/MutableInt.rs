// Java field: int value

// Java: public class MutableInt
#[derive(Clone, Debug, Default)]
pub struct MutableInt {
    pub value: i32,
}

// Java: public class MutableInt
impl MutableInt {
    // Java: MutableInt()
    pub fn new() -> Self {
        // Java: super();
        Self { value: 0 }
    }

    // Java: MutableInt(int value)
    pub fn new_with_value(value: i32) -> Self {
        // Java: super();
        // Java: this.value = value;
        Self { value }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.value = 0;
        self.value = 0;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.value;
        self.value
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, obj: &dyn std::any::Any) -> bool {
        // Java: return obj instanceof MutableInt && ((MutableInt)obj).value == this.value;
        if let Some(other) = obj.downcast_ref::<MutableInt>() {
            other.value == self.value
        } else {
            false
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return Integer.toString(this.value);
        self.value.to_string()
    }
}
