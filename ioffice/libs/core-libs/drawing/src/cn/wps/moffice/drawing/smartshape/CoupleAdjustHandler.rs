#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports)]

use super::AdjustHandler::AdjustHandler;
use super::AdjustData::AdjustData;

// Java: public class CoupleAdjustHandler extends AdjustHandler
pub struct CoupleAdjustHandler {
    pub(crate) base: AdjustHandler,
    // Java field: AdjustData firstAdjustData
    pub(crate) firstAdjustData: AdjustData,
    // Java field: AdjustData secondAdjustData
    pub(crate) secondAdjustData: AdjustData,
}

// Java: public class CoupleAdjustHandler extends AdjustHandler
impl CoupleAdjustHandler {
    // Java: CoupleAdjustHandler(int type)
    pub fn new(r#type: i32) -> Self {
      // Java: super(type);
      CoupleAdjustHandler {
          base: AdjustHandler::new(r#type),
          firstAdjustData: AdjustData::new(),
          secondAdjustData: AdjustData::new(),
      }
    }

}
