#![allow(non_snake_case, dead_code)]

// Java: public interface IColorTransform
pub trait IColorTransform {
    // Java: abstract int type()
    fn r#type(&self) -> i32;

    // Java: abstract double value()
    fn value(&self) -> f64;

}
