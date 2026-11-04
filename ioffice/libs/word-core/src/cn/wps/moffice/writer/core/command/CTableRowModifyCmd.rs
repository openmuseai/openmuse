// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.transaction.CommandUpdate
use util::moffice::transaction::CommandUpdate::*;
// Java: import cn.wps.moffice.writer.core.DefaultModelEvent
use crate::cn::wps::moffice::writer::core::DefaultModelEvent::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.DocumentEvent
use crate::cn::wps::moffice::writer::core::DocumentEvent::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import cn.wps.moffice.writer.core.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;

// java public class CTableRowModifyCmd extends CommandUpdate
#[derive(Default)]
pub struct CTableRowModifyCmd {
    // java CommandUpdate super_
    pub(crate) super_: CommandUpdate,
    // java Document mDocument;
    pub(crate) mDocument: *mut dyn Document,
    // java long mRange;
    pub(crate) mRange: i64,
    // java int mPropID;
    pub(crate) mPropID: i32,
    // java Object mPropOld;
    pub(crate) mPropOld: Option<Box<dyn Object>>,
    // java Object mPropNew;
    pub(crate) mPropNew: Option<Box<dyn Object>>,
    // java boolean mModifyRevision = true;
    pub(crate) mModifyRevision: bool,
}

// java public class CTableRowModifyCmd extends CommandUpdate
impl CTableRowModifyCmd {
    // java public  CTableRowModifyCmd(Document document, long range, int id, Object propOld, Object propNew)
    pub fn CTableRowModifyCmd(document: &dyn Document, range: i64, id: i32, propOld: &dyn Object, propNew: &dyn Object) -> Self {
      // java this.mDocument = document;
      Self { super_: CommandUpdate::new(), mDocument: document, mRange: range, mPropID: id, mPropOld: propOld, mPropNew: propNew, mModifyRevision: Default::default() }
      // java this.mRange = range;
      // java this.mPropID = id;
      // java this.mPropOld = propOld;
      // java this.mPropNew = propNew;
    }

    // java public void update(Object propNew)
    pub fn update(&mut self, propNew: &dyn Object) {
      // java if (this.mPropNew == null || this.mPropNew.getClass().isInstance(propNew))
      if self.mPropNew.is_none() || self.mPropNew.getClass().isInstance(propNew) {
        // java this.mPropNew = propNew;
        self.mPropNew = propNew;
        // java this.execute(false);
        self.execute__boolean(false);
      }
    }

    // java public void execute()
    pub fn execute(&mut self) {
      // java this.execute(false);
      self.execute__boolean(false);
    }

    // java public void unexecute()
    pub fn unexecute(&mut self) {
      // java this.execute(true);
      self.execute__boolean(true);
    }

    // java private void execute(boolean bReverse)
    pub fn execute__boolean(&self, bReverse: bool) {
      // java DocumentEvent e = DefaultModelEvent.obtain(this.mDocument, this.mDocument.getTextRope().cp2fc(RANGEFactory.getEnd(this.mRange) - 1), 1, 5);
      let mut e: Box<dyn DocumentEvent> = DefaultModelEvent::obtain__document_int_int_int(&self.mDocument, self.mDocument.getTextRope().cp2fc(RANGEFactory::getEnd(self.mRange) - 1), 1, 5);
      // java this.mDocument.getDispatcher().update(e);
      self.mDocument.getDispatcher().update(e);
      // java e.decreaseRef();
      e.decreaseRef();
      // java if (!bReverse)
      if !bReverse {
        // java Object prop = this.mPropNew;
        let mut prop: Box<dyn Object> = self.mPropNew;
        // java IPapxTable pt = this.mDocument.getPapxTable();
        let mut pt: Box<dyn IPapxTable> = self.mDocument.getPapxTable();
        // java IPapxTable.Entry rowEndEntry = pt.seek(RANGEFactory.getEnd(this.mRange) - 1);
        let mut rowEndEntry: Box<dyn Entry> = pt.seek(RANGEFactory::getEnd(self.mRange) - 1);
        // java PropertySet rowProp = rowEndEntry.getProperty();
        let mut rowProp: PropertySet = rowEndEntry.getProperty();
        // java Revision revision = (Revision)rowProp.getValue(363);
        let mut revision: Revision = (rowProp.getValue(363)).downcast_ref::<Revision>().unwrap();
        // java MutablePropertySet mPropertySet = new MutablePropertySet(rowProp);
        let mut mPropertySet: MutablePropertySet = MutablePropertySet::new(rowProp);
        // java if (this.mModifyRevision && null != revision)
        if self.mModifyRevision && revision.is_some() {
          // java MutablePropertySet newRevisonProp = new MutablePropertySet(revision.getProperty());
          let mut newRevisonProp: MutablePropertySet = MutablePropertySet::new(revision.getProperty());
          // java newRevisonProp.setValue(this.mPropID, prop);
          newRevisonProp.setValue(self.mPropID, prop);
              // java Revision newRevision = revision.clone();
              let mut newRevision: Revision = revision.clone();
              // java newRevision.setProperty(newRevisonProp.freeze());
              newRevision.setProperty(newRevisonProp.freeze());
              // java mPropertySet.setValue(363, newRevision);
              mPropertySet.setValue(363, newRevision);
              // java CloneNotSupportedException var11
              todo!();
        }
        // java else
        else {
          // java mPropertySet.setValue(this.mPropID, prop);
          mPropertySet.setValue(self.mPropID, prop);
        }
        // java rowEndEntry.setProperty(mPropertySet.freeze());
        rowEndEntry.setProperty(mPropertySet.freeze());
      }
    }

    // java public void setModifyRevision(boolean b)
    pub fn setModifyRevision(&mut self, b: bool) {
      // java this.mModifyRevision = b;
      self.mModifyRevision = b;
    }

}

impl std::ops::Deref for CTableRowModifyCmd {
    type Target = CommandUpdate;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CTableRowModifyCmd {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
