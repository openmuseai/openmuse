#![allow(non_snake_case)]
#![allow(dead_code)]

use super::KmoTipStamp::KmoTipStamp;

// Java: public class KmoTipStampImpl implements KmoTipStamp
pub struct KmoTipStampImpl {
    // Java field: int tip
    pub(crate) tip: i32,
}

// Java: public class KmoTipStampImpl implements KmoTipStamp
impl KmoTipStampImpl {
    // Java: KmoTipStampImpl(int tip)
    pub fn new(tip: i32) -> Self {
      // Java: super();
      // Java: this.tip = tip;
      KmoTipStampImpl { tip }
    }

    // Java: int tip()
    pub fn tip(&self) -> i32 {
      // Java: return this.tip;
      self.tip
    }

}

impl KmoTipStamp for KmoTipStampImpl {
    fn tip(&self) -> i32 {
        self.tip
    }
}
