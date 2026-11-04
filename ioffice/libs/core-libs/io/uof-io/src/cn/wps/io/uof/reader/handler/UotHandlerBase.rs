use super::UofHandlerBase::UofHandlerBase;

// Java: public class UotHandlerBase extends UofHandlerBase
pub struct UotHandlerBase {
    pub(crate) base: UofHandlerBase,
}

// Java: public class UotHandlerBase extends UofHandlerBase
impl UotHandlerBase {
    // Java: UotHandlerBase()
    pub fn UotHandlerBase() -> Self {
      // Java: super();
      UotHandlerBase {
          base: UofHandlerBase::UofHandlerBase(),
      }
    }

}
