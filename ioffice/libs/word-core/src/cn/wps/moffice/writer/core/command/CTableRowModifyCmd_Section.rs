// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.command.CTableRowModifyCmd
use crate::cn::wps::moffice::writer::core::command::CTableRowModifyCmd::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java public class CTableRowModifyCmd_Section extends CTableRowModifyCmd
#[derive(Default)]
pub struct CTableRowModifyCmd_Section {
    // java CTableRowModifyCmd super_
    pub(crate) super_: CTableRowModifyCmd,
}

// java public class CTableRowModifyCmd_Section extends CTableRowModifyCmd
impl CTableRowModifyCmd_Section {
    // java public  CTableRowModifyCmd_Section(Document document, long range, int id, Object propOld, Object propNew)
    pub fn CTableRowModifyCmd_Section(document: &dyn Document, range: i64, id: i32, propOld: &dyn Object, propNew: &dyn Object) -> Self {
      // java super(document, range, id, propOld, propNew);
      Self { super_: CTableRowModifyCmd::new(&document, range, id, &propOld, &propNew) }
    }

    // java public void execute()
    pub fn execute(&mut self) {
      // java super.execute();
      self.super_.execute();
    }

    // java public void unexecute()
    pub fn unexecute(&mut self) {
      // java this.execute();
      self.execute();
    }

}

impl std::ops::Deref for CTableRowModifyCmd_Section {
    type Target = CTableRowModifyCmd;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CTableRowModifyCmd_Section {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
