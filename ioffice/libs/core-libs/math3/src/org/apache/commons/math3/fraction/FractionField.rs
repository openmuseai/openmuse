// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field;
// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement;

// Java field: long serialVersionUID
// Java field: FractionField INSTANCE
static INSTANCE: std::sync::OnceLock<FractionField> = std::sync::OnceLock::new();

// Java: public class is
pub struct r#is {
}

// Java: public class FractionField
pub struct FractionField {
}

// Java: public class LazyHolder
pub struct LazyHolder {
}

// Java: public class is
impl r#is {
}

// Java: public class FractionField
impl FractionField {
    // Java: FractionField()
    pub fn new() -> Self {
        FractionField {}
    }

    // Java: static FractionField getInstance()
    pub fn getInstance() -> &'static FractionField {
        // Java: return LazyHolder.INSTANCE;
        INSTANCE.get_or_init(|| FractionField::new())
    }

    // Java: Fraction getOne()
    pub fn getOne(&self) -> Box<dyn std::any::Any> {
        // Java: return Fraction.ONE;
        todo!("FractionField::getOne")
    }

    // Java: Fraction getZero()
    pub fn getZero(&self) -> Box<dyn std::any::Any> {
        // Java: return Fraction.ZERO;
        todo!("FractionField::getZero")
    }

    // Java: public Class<? extends FieldElement<Fraction>> getRuntimeClass()
    pub fn getRuntimeClass(&self) -> Box<dyn std::any::Any> {
        // Java: return Fraction.class;
        todo!("FractionField::getRuntimeClass")
    }

    // Java: Object readResolve()
    pub fn readResolve(&self) -> &'static FractionField {
        // Java: // return the singleton instance
        // Java: return LazyHolder.INSTANCE;
        FractionField::getInstance()
    }

}

// Java: public class LazyHolder
impl LazyHolder {
}
