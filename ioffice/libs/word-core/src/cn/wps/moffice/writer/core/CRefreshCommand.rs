// Java: import cn.wps.moffice.transaction.CommandBase
use util::moffice::transaction::CommandBase::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.DocumentEvent
use crate::cn::wps::moffice::writer::core::DocumentEvent::*;

// java public class CRefreshCommand extends CommandBase
#[derive(Default)]
pub struct CRefreshCommand {
    // java CommandBase super_
    pub(crate) super_: CommandBase,
    // java Document mDocument;
    pub(crate) mDocument: *mut dyn Document,
}

// java public class CRefreshCommand extends CommandBase
impl CRefreshCommand {
    // java public  CRefreshCommand(Document doc)
    pub fn CRefreshCommand(doc: &dyn Document) -> Self {
      // java this.mDocument = doc;
      Self { super_: CommandBase::new(), mDocument: doc as *const dyn Document as *mut dyn Document }
    }

    // java public void execute()
    pub fn execute(&self) {
      // java if (this.mDocument != null)
      if !self.mDocument.is_null() {
        let mDocument = unsafe { &*self.mDocument };
        // java DocumentEvent e = DefaultModelEvent.obtain(this.mDocument, 0, 0, 7);
        let mut e: Box<dyn DocumentEvent> = DefaultModelEvent::obtain__document_int_int_int(&mDocument, 0, 0, 7);
        // java this.mDocument.getDispatcher().update(e);
        mDocument.getDispatcher().update(e);
        // java e.decreaseRef();
        e.decreaseRef();
      }
    }

    // java public void unexecute()
    pub fn unexecute(&mut self) {
      // java this.execute();
      self.execute();
    }

}

impl std::ops::Deref for CRefreshCommand {
    type Target = CommandBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CRefreshCommand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
