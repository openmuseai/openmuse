// Java: import cn.wps.moffice.transaction.CommandUpdate
use util::moffice::transaction::CommandUpdate::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.table.grid.GridResizeRequest
use crate::cn::wps::moffice::writer::data::table::grid::GridResizeRequest::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java public class CTableResizeCmd extends CommandUpdate
#[derive(Default)]
pub struct CTableResizeCmd {
    // java CommandUpdate super_
    pub(crate) super_: CommandUpdate,
    // java private Document document;
    document: Option<Box<dyn Document>>,
    // java private long range;
    range: i64,
}

// java public class CTableResizeCmd extends CommandUpdate
impl CTableResizeCmd {
    // java public  CTableResizeCmd(Document document, long range)
    pub fn CTableResizeCmd(document: &dyn Document, range: i64) -> Self {
      // java this.document = document;
      Self { super_: CommandUpdate::new(), document: document, range: range }
      // java this.range = range;
    }

    // java public void update(Object object)
    pub fn update(&mut self, object: &dyn Object) {
    }

    // java public void execute()
    pub fn execute(&self) {
      // java super.execute();
      self.super_.execute();
      // java if (this.document != null)
      if let Some(document) = self.document.as_ref() {
        // java GridResizeRequest grr = this.document.getTextDocument().getGridResizeRequest();
        let mut grr: Box<dyn GridResizeRequest> = document.getTextDocument().getGridResizeRequest();
        // java if (grr != null)
        if grr.is_some() {
          // java grr.requestWithReTypo(this.range, this.document);
          grr.requestWithReTypo(self.range, document);
        }
      }
    }

    // java public void unexecute()
    pub fn unexecute(&self) {
      // java super.unexecute();
      self.super_.unexecute();
      // java if (this.document != null)
      if let Some(document) = self.document.as_ref() {
        // java GridResizeRequest grr = this.document.getTextDocument().getGridResizeRequest();
        let mut grr: Box<dyn GridResizeRequest> = document.getTextDocument().getGridResizeRequest();
        // java if (grr != null)
        if grr.is_some() {
          // java grr.requestWithReTypo(this.range, this.document);
          grr.requestWithReTypo(self.range, document);
        }
      }
    }

}

impl std::ops::Deref for CTableResizeCmd {
    type Target = CommandUpdate;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CTableResizeCmd {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
