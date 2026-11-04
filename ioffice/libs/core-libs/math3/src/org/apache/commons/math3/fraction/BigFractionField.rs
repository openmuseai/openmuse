// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field::Field;
// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement::FieldElement;

#[derive(Clone, Debug)]
pub struct BigFraction;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;
// Java field: BigFractionField INSTANCE
static INSTANCE: std::sync::OnceLock<BigFractionField> = std::sync::OnceLock::new();

// Java: public class is
pub struct is {
}

// Java: public class BigFractionField
pub struct BigFractionField {
}

// Java: public class LazyHolder
pub struct LazyHolder {
}

// Java: public class is
impl is {
}

// Java: public class BigFractionField
impl BigFractionField {
    // Java: BigFractionField()
    pub fn BigFractionField(&self) {
    }

    // Java: static BigFractionField getInstance()
    pub fn getInstance(&self) -> BigFractionField {
        // Java: return LazyHolder.INSTANCE;
        todo!();
    }

    // Java: BigFraction getOne()
    pub fn getOne(&self) -> BigFraction {
        // Java: return BigFraction.ONE;
        todo!();
    }

    // Java: BigFraction getZero()
    pub fn getZero(&self) -> BigFraction {
        // Java: return BigFraction.ZERO;
        todo!();
    }

    // Java: public Class<? extends FieldElement<BigFraction>> getRuntimeClass()
    pub fn getRuntimeClass(&self) -> std::any::TypeId {
        // Java: return BigFraction.class;
        todo!();
    }

    // Java: Object readResolve()
    pub fn readResolve(&self) -> &BigFractionField {
        // Java: // return the singleton instance
        // Java: return LazyHolder.INSTANCE;
        todo!();
    }

}

// Java: public class LazyHolder
impl LazyHolder {
}
