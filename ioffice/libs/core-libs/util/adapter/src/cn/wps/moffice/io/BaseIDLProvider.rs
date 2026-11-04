use super::IDLProvider::IDLProvider;

// Java: public class BaseIDLProvider implements IDLProvider
pub struct BaseIDLProvider {
}

// Java: public class BaseIDLProvider implements IDLProvider
impl BaseIDLProvider {
    // Java: BaseIDLProvider()
    pub fn new() -> Self {
      // Java: super();
      BaseIDLProvider {}
    }

}

impl IDLProvider for BaseIDLProvider {
    // Java: void setOpenForceQuit(boolean isForceQuit)
    fn setOpenForceQuit(&mut self, _isForceQuit: bool) {
    }

    // Java: boolean isOpenForceQuit()
    fn isOpenForceQuit(&self) -> bool {
      // Java: return false;
      false
    }

    // Java: boolean isOpenFirstPageForceQuit()
    fn isOpenFirstPageForceQuit(&self) -> bool {
      // Java: return false;
      false
    }

    // Java: Integer getReadingLimit()
    fn getReadingLimit(&self) -> Option<i32> {
      // Java: return null;
      None
    }

    // Java: String getNotePath()
    fn getNotePath(&self) -> String {
      // Java: return null;
      String::new()
    }

    // Java: String getWritePassword()
    fn getWritePassword(&self) -> String {
      // Java: return null;
      String::new()
    }

    // Java: boolean isOpenOnReadOnly()
    fn isOpenOnReadOnly(&self) -> bool {
      // Java: return false;
      false
    }
}
