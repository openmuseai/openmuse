// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import cn.wps.moffice.writer.core.KRevision
use crate::cn::wps::moffice::writer::core::KRevision::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;

// java public class KTableRevision extends KRevision
#[derive(Default)]
pub struct KTableRevision {
    // java KRevision super_
    pub(crate) super_: KRevision,
}

// java public class KTableRevision extends KRevision
impl KTableRevision {
    // java public  KTableRevision(Document document)
    pub fn KTableRevision(document: &dyn Document) -> Self {
      // java super(document.getTextDocument());
      // java Assert.assertNotNull("document should not be null !", document);
      Assert::assertNotNull__string_object(&"document should not be null !", Some(&document));
    }

    // java public PropertySet setTableRevision(PropertySet prop)
    pub fn setTableRevision(&mut self, prop: &PropertySet) -> Option<PropertySet> {
      // java Assert.assertNotNull("prop should not be null !", prop);
      Assert::assertNotNull__string_object(&"prop should not be null !", Some(&prop));
      // java Revision revision = (Revision)prop.getValue(363);
      let mut revision: Revision = (prop.getValue(363)).downcast_ref::<Revision>().unwrap();
      // java return null != revision ? this.modifyTableRevision(revision) : this.getNewTableRevision(prop);
      return (if revision.is_some() { Some(self.modifyTableRevision(&revision)) } else { Some(self.getNewTableRevision(&prop)) });
    }

    // java private PropertySet modifyTableRevision(Revision revision)
    pub fn modifyTableRevision(&mut self, revision: &Revision) -> PropertySet {
      // java Assert.assertNotNull("revision should not be null !", revision);
      Assert::assertNotNull__string_object(&"revision should not be null !", Some(&revision));
      // java MutablePropertySet mutaProp = new MutablePropertySet();
      let mut mutaProp: MutablePropertySet = MutablePropertySet::new();
          // java Revision newRevision = revision.clone();
          let mut newRevision: Revision = revision.clone();
          // java this.setRevision(newRevision);
          self.super_.setRevision(&mut newRevision);
          // java mutaProp.setValue(363, newRevision);
          mutaProp.setValue(363, newRevision);
          // java CloneNotSupportedException e
          todo!();
            // java Assert.fail("revision clone fail!" + e);
            Assert::fail__string(&"revision clone fail!" + e);
      // java return mutaProp.freeze();
      return mutaProp.freeze();
    }

    // java private PropertySet getNewTableRevision(PropertySet originalProp)
    pub fn getNewTableRevision(&mut self, originalProp: &PropertySet) -> PropertySet {
      // java Assert.assertNotNull("originalProp should not be null !", originalProp);
      Assert::assertNotNull__string_object(&"originalProp should not be null !", Some(&originalProp));
      // java MutablePropertySet mutaProp = new MutablePropertySet();
      let mut mutaProp: MutablePropertySet = MutablePropertySet::new();
      // java Revision revision = new Revision(3);
      let mut revision: Revision = Revision::new(3);
      // java this.setRevision(revision);
      self.super_.setRevision(&mut revision);
      // java int tableStyle = originalProp.getInt(292, 105);
      let mut tableStyle: i32 = originalProp.getInt__int_int(292, 105);
      // java if (105 != tableStyle && 4095 != tableStyle)
      if 105 != tableStyle && 4095 != tableStyle {
        // java MutablePropertySet revisionMutaProp = new MutablePropertySet();
        let mut revisionMutaProp: MutablePropertySet = MutablePropertySet::new();
        // java revisionMutaProp.addLimit(originalProp, 291, 376);
        revisionMutaProp.addLimit(originalProp, 291, 376);
        // java revision.setProperty(revisionMutaProp.freeze());
        revision.setProperty(revisionMutaProp.freeze());
      }
      // java mutaProp.setValue(363, revision);
      mutaProp.setValue(363, revision);
      // java mutaProp.setValue(364, true);
      mutaProp.setValue(364, true);
      // java mutaProp.setInt(365, this.assignRsid());
      mutaProp.setInt(365, self.super_.assignRsid());
      // java return mutaProp.freeze();
      return mutaProp.freeze();
    }

}

impl std::ops::Deref for KTableRevision {
    type Target = KRevision;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KTableRevision {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
