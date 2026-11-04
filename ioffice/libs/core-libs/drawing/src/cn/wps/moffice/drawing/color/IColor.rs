#![allow(non_snake_case, dead_code, unused_imports)]

use super::IColorTransform::IColorTransform;

// Java: public interface IColor
pub trait IColor {
    // Java: abstract int type()
    fn r#type(&self) -> i32;

    // Java: abstract int srgbR()
    fn srgbR(&self) -> i32;

    // Java: abstract int srgbG()
    fn srgbG(&self) -> i32;

    // Java: abstract int srgbB()
    fn srgbB(&self) -> i32;

    // Java: abstract double scrgbR()
    fn scrgbR(&self) -> f64;

    // Java: abstract double scrgbG()
    fn scrgbG(&self) -> f64;

    // Java: abstract double scrgbB()
    fn scrgbB(&self) -> f64;

    // Java: abstract double hslH()
    fn hslH(&self) -> f64;

    // Java: abstract double hslS()
    fn hslS(&self) -> f64;

    // Java: abstract double hslL()
    fn hslL(&self) -> f64;

    // Java: abstract int prst()
    fn prst(&self) -> i32;

    // Java: abstract int scheme()
    fn scheme(&self) -> i32;

    // Java: abstract int sys()
    fn sys(&self) -> i32;

    // Java: abstract int sysLastClr()
    fn sysLastClr(&self) -> i32;

    // Java: abstract boolean hasTransformLst()
    fn hasTransformLst(&self) -> bool;

    // Java: abstract List<IColorTransform> colorTransforms()
    fn colorTransforms(&self) -> Vec<Box<dyn IColorTransform>>;

}
