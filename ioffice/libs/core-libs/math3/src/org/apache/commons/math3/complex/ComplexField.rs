// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field;
// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement;

use super::Complex::Complex;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = -6130362688700788798;
// Java field: ComplexField INSTANCE
static INSTANCE: std::sync::OnceLock<ComplexField> = std::sync::OnceLock::new();

// Java: public class is
pub struct is {
}

// Java: public class ComplexField
#[derive(Clone, Debug)]
pub struct ComplexField {
}

// Java: public class LazyHolder
pub struct LazyHolder {
}

// Java: public class is
impl is {
}

// Java: public class ComplexField
impl ComplexField {
    // Java: ComplexField()
    pub fn ComplexField() -> Self {
        ComplexField {}
    }

    // Java: static ComplexField getInstance()
    pub fn getInstance() -> &'static ComplexField {
        // Java: return LazyHolder.INSTANCE;
        INSTANCE.get_or_init(|| ComplexField {})
    }

    // Java: Complex getOne()
    pub fn getOne(&self) -> Complex {
        // Java: return Complex.ONE;
        Complex::new(1.0, 0.0)
    }

    // Java: Complex getZero()
    pub fn getZero(&self) -> Complex {
        // Java: return Complex.ZERO;
        Complex::new(0.0, 0.0)
    }

    // Java: public Class<? extends FieldElement<Complex>> getRuntimeClass()
    pub fn getRuntimeClass(&self) -> std::any::TypeId {
        // Java: return Complex.class;
        std::any::TypeId::of::<Complex>()
    }

    // Java: Object readResolve()
    pub fn readResolve(&self) -> &'static ComplexField {
        // Java: // return the singleton instance
        // Java: return LazyHolder.INSTANCE;
        Self::getInstance()
    }
}

// Java: public class LazyHolder
impl LazyHolder {
}
