// Java: import cn.wps.moffice.io.IDocument

use super::ISlimToolManager::ISlimToolManager;

// Java: public class SlimToolMgrNull extends ISlimToolManager
pub struct SlimToolMgrNull {
    pub(crate) base: ISlimToolManager,
}

// Java: public class SlimToolMgrNull extends ISlimToolManager
impl SlimToolMgrNull {
    // Java: SlimToolMgrNull()
    pub fn new() -> Self {
      // Java: super((IDocument)null, sNullListener);
      SlimToolMgrNull {
          base: ISlimToolManager::new(None, None),
      }
    }

}
