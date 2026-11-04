#![allow(non_snake_case, dead_code)]

// Java: public class TransformOperation
pub struct TransformOperation {
}

// Java field: byte alpha
pub(crate) const alpha: i8 = 1;
// Java field: byte alphaMod
pub(crate) const alphaMod: i8 = 2;
// Java field: byte alphaOff
pub(crate) const alphaOff: i8 = 3;
// Java field: byte blue
pub(crate) const blue: i8 = 4;
// Java field: byte blueMod
pub(crate) const blueMod: i8 = 5;
// Java field: byte blueOff
pub(crate) const blueOff: i8 = 6;
// Java field: byte comp
pub(crate) const comp: i8 = 7;
// Java field: byte gamma
pub(crate) const gamma: i8 = 8;
// Java field: byte gray
pub(crate) const gray: i8 = 9;
// Java field: byte green
pub(crate) const green: i8 = 10;
// Java field: byte greenMod
pub(crate) const greenMod: i8 = 11;
// Java field: byte greenOff
pub(crate) const greenOff: i8 = 12;
// Java field: byte hue
pub(crate) const hue: i8 = 13;
// Java field: byte hueMod
pub(crate) const hueMod: i8 = 14;
// Java field: byte hueOff
pub(crate) const hueOff: i8 = 15;
// Java field: byte inv
pub(crate) const inv: i8 = 16;
// Java field: byte invGamma
pub(crate) const invGamma: i8 = 17;
// Java field: byte lum
pub(crate) const lum: i8 = 18;
// Java field: byte lumMod
pub(crate) const lumMod: i8 = 19;
// Java field: byte lumOff
pub(crate) const lumOff: i8 = 20;
// Java field: byte red
pub(crate) const red: i8 = 21;
// Java field: byte redMod
pub(crate) const redMod: i8 = 22;
// Java field: byte redOff
pub(crate) const redOff: i8 = 23;
// Java field: byte sat
pub(crate) const sat: i8 = 24;
// Java field: byte satMod
pub(crate) const satMod: i8 = 25;
// Java field: byte satOff
pub(crate) const satOff: i8 = 26;
// Java field: byte shade
pub(crate) const shade: i8 = 27;
// Java field: byte tint
pub(crate) const tint: i8 = 28;

// Java: public class CoreColorTransform implements Cloneable
pub struct CoreColorTransform {
    // Java field: byte transOp
    pub(crate) transOp: i8,
    // Java field: double value
    pub(crate) value: f64,
}

pub trait Cloneable {}
impl Cloneable for CoreColorTransform {}

// Java: public class CoreColorTransform implements Cloneable
impl CoreColorTransform {
    // Java: CoreColorTransform(byte op, double val)
    pub fn new(op: i8, val: f64) -> Self {
      // Java: super();
      let mut instance = CoreColorTransform { transOp: 0, value: 0.0 };
      instance.init(op, val);
      instance
    }

    fn init(&mut self, op: i8, val: f64) {
      // Java: this.transOp = op;
      self.transOp = op;
      // Java: this.value = val;
      self.value = val;
    }

    // Java: CoreColorTransform clone()
    pub fn clone(&self) -> CoreColorTransform {
      // Java: CoreColorTransform clone = new CoreColorTransform(this.transOp, this.value);
      let clone = CoreColorTransform::new(self.transOp, self.value);
      // Java: return clone;
      clone
    }

}

// Java: public class TransformOperation
impl TransformOperation {
    // Java: TransformOperation()
    pub fn new() -> Self {
         // Java: super();
         TransformOperation {}
    }

}
