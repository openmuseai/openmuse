#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports)]

use super::AdjustHandler::AdjustHandler;
use super::AdjustData::AdjustData;

// Java: public class SingleAdjustHandler extends AdjustHandler
pub struct SingleAdjustHandler {
    pub(crate) base: AdjustHandler,
    // Java field: AdjustData adjustData
    pub(crate) adjustData: AdjustData,
}

// Java: public class SingleAdjustHandler extends AdjustHandler
impl SingleAdjustHandler {
    // Java: SingleAdjustHandler(int type)
    pub fn new(r#type: i32) -> Self {
      // Java: super(type);
      SingleAdjustHandler {
          base: AdjustHandler::new(r#type),
          adjustData: AdjustData::new(),
      }
    }

}
