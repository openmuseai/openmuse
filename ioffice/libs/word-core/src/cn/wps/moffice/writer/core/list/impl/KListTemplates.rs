// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.transaction.Atom
use util::moffice::transaction::Atom::*;
// Java: import cn.wps.moffice.transaction.AtomMember
use util::moffice::transaction::AtomMember::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument as WriterTextDocument;
// Java: import cn.wps.moffice.writer.core.list.ListTemplates
use crate::cn::wps::moffice::writer::core::list::ListTemplates::*;
// Java: import cn.wps.moffice.writer.data.autonum.AutoNumTable
use crate::cn::wps::moffice::writer::data::autonum::AutoNumTable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LstData
use crate::cn::wps::moffice::writer::data::autonum::LstData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LstDataTable
use crate::cn::wps::moffice::writer::data::autonum::LstDataTable::*;
// Java: import java.security.SecureRandom
use stubs::java::security::SecureRandom::*;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import cn.wps.moffice.writer.core.list.impl.KListTemplate
use crate::cn::wps::moffice::writer::core::list::r#impl::KListTemplate::*;
use std::collections::{HashMap};

// java public final class KListTemplates extends Atom implements ListTemplates
#[derive(Default)]
pub struct KListTemplates {
    // java Atom super_
    pub(crate) super_: Atom,
    // java private TextDocument.UULsid mUUID;
    mUUID: Option<crate::cn::wps::moffice::writer::core::TextDocument::UULsid>,
    // java private SecureRandom mTplcGenerator;
    mTplcGenerator: Option<SecureRandom>,
    // java private TextDocument mTextDocument;
    mTextDocument: Option<WriterTextDocument>,
    // java private LstDataTable mLstTable;
    mLstTable: Option<LstDataTable>,
    // java @AtomMember(1) private ArrayList<KListTemplate> mTemplates;
    mTemplates: Option<Vec<KListTemplate>>,
}

// java public interface ListTemplates
impl ListTemplates for KListTemplates {
    // java public KListTemplate add()
    fn add(&mut self) -> KListTemplate {
      // java KListTemplate added = new KListTemplate(this.mTextDocument, this.mLstTable, this.mUUID, this.mTplcGenerator, 9);
      let mut added: KListTemplate = KListTemplate::new(self.mTextDocument, self.mLstTable, self.mUUID, self.mTplcGenerator, 9);
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mTemplates.add(added);
      self.mTemplates.add(added);
      // java return added;
      return added;
    }

}

// java public final class KListTemplates extends Atom implements ListTemplates
impl KListTemplates {
    // java public  KListTemplates(TextDocument textDocument)
    pub fn KListTemplates(textDocument: &WriterTextDocument) -> Self {
      // java Assert.assertNotNull("textDocument should not be null.", textDocument);
      Assert::assertNotNull__string_object(&"textDocument should not be null.", Some(&textDocument));
      // java AutoNumTable autoNumTable = textDocument.getAutoNumTable();
      let mut autoNumTable: AutoNumTable = textDocument.getAutoNumTable();
      // java this.mTextDocument = textDocument;
      let mut this = Self { super_: Atom::new(), mUUID: textDocument.getUULsid(), mTplcGenerator: SecureRandom::new(), mTextDocument: Some(textDocument.clone()), mLstTable: autoNumTable.getLstDataTable(), mTemplates: Some(Vec::new()) };
      // java this.setTransaction(this.mTextDocument.getTransaction());
      this.super_.setTransaction__transaction(&this.mTextDocument.as_ref().unwrap().getTransaction());
      // java Assert.assertNotNull("autoNumTable should not be null.", autoNumTable);
      Assert::assertNotNull__string_object(&"autoNumTable should not be null.", Some(&autoNumTable));
      // java this.mLstTable = autoNumTable.getLstDataTable();
      // java Assert.assertNotNull("mLstTable should not be null.", this.mLstTable);
      Assert::assertNotNull__string_object(&"mLstTable should not be null.", Some(&this.mLstTable));
      // java this.mUUID = textDocument.getUULsid();
      // java Assert.assertNotNull("mUUID should not be null.", this.mUUID);
      Assert::assertNotNull__string_object(&"mUUID should not be null.", Some(&this.mUUID));
      // java this.mTplcGenerator = new SecureRandom();
      // java this.mTemplates = new ArrayList();
      // java this.init();
      this.init();
      this
    }

    // java private void init()
    pub fn init(&mut self) {
      // java Assert.assertNotNull("mLfoTable should not be null.", this.mLstTable);
      Assert::assertNotNull__string_object(&"mLfoTable should not be null.", Some(&self.mLstTable));
      // java Assert.assertNotNull("mTemplates should not be null.", this.mTemplates);
      Assert::assertNotNull__string_object(&"mTemplates should not be null.", Some(&self.mTemplates));
      // java Assert.assertNotNull("mUUID should not be null.", this.mUUID);
      Assert::assertNotNull__string_object(&"mUUID should not be null.", Some(&self.mUUID));
      // java HashMap<Integer, LstData> map = this.mLstTable.getLstMap();
      let mut map: HashMap<i32, LstData> = self.mLstTable.getLstMap();
      // java for(Integer lsid : map.keySet())
      for lsid in map.keySet().iter() {
        // java Assert.assertNotNull("numId should not be null.", lsid);
        Assert::assertNotNull__string_object(&"numId should not be null.", Some(&lsid));
        // java LstData lstData = (LstData)map.get(lsid);
        let mut lstData: LstData = (map.get(lsid)).downcast_ref::<LstData>().unwrap();
        // java Assert.assertNotNull("lstData should not be null.", lstData);
        Assert::assertNotNull__string_object(&"lstData should not be null.", Some(&lstData));
        // java this.mTemplates.add(new KListTemplate(this.mTextDocument, lstData, this.mUUID, this.mTplcGenerator));
        self.mTemplates.add(KListTemplate::new(self.mTextDocument, lstData, self.mUUID, self.mTplcGenerator));
      }
    }

    // java public KListTemplate remove(int lsid)
    pub fn remove__int(&mut self, lsid: i32) -> Option<KListTemplate> {
      // java KListTemplate template = this.find(lsid);
      let mut template: KListTemplate = self.find(lsid);
      // java return null != template && this.remove(template) ? template : null;
      return (if template.is_some() && self.remove__int(template) { Some(template) } else { None });
    }

    // java public boolean remove(KListTemplate template)
    pub fn remove__klisttemplate(&mut self, template: &KListTemplate) -> bool {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java boolean removed = this.mTemplates.remove(template);
      let mut removed: bool = self.mTemplates.remove(template);
      // java Assert.assertTrue("removed should be true.", removed);
      Assert::assertTrue__string_boolean(&"removed should be true.", removed);
      // java if (removed)
      if removed {
        // java LstData removedLstData = this.mLstTable.remove(template.getLsid());
        let mut removedLstData: LstData = self.mLstTable.remove(template.getLsid());
        // java Assert.assertNotNull("removedLstData should not be null.", removedLstData);
        Assert::assertNotNull__string_object(&"removedLstData should not be null.", Some(&removedLstData));
      }
      // java return removed;
      return removed;
    }

    // java public int count()
    pub fn count(&self) -> i32 {
      // java return this.mTemplates.size();
      return (self.mTemplates.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
    }

    // java public KListTemplate item(int index)
    pub fn item(&self, index: i32) -> KListTemplate {
      // java return (KListTemplate)this.mTemplates.get(index);
      return (self.mTemplates.get(index)).downcast_ref::<KListTemplate>().unwrap();
    }

    // java public KListTemplate newInstance()
    pub fn newInstance(&self) -> KListTemplate {
      // java KListTemplate template = new KListTemplate(this.mTextDocument, this.mTplcGenerator, 9);
      let mut template: KListTemplate = KListTemplate::new(self.mTextDocument, self.mTplcGenerator, 9);
      // java return template;
      return template;
    }

    // java public boolean contains(KListTemplate listTemplate)
    pub fn contains(&self, listTemplate: &KListTemplate) -> bool {
      // java LstData lstData = listTemplate.getLstData();
      let mut lstData: LstData = listTemplate.getLstData();
      // java Assert.assertNotNull("lstData should not be null.", lstData);
      Assert::assertNotNull__string_object(&"lstData should not be null.", Some(&lstData));
      // java boolean isInLstTable = null != this.mLstTable.getDataByLsid(lstData.getlsid());
      let mut isInLstTable: bool = self.mLstTable.getDataByLsid(lstData.getlsid()).is_some();
      // java return isInLstTable;
      return isInLstTable;
    }

    // java public void add(KListTemplate listTemplate)
    pub fn add__klisttemplate(&mut self, listTemplate: &mut KListTemplate) {
      // java if (!this.mTemplates.contains(listTemplate))
      if !self.mTemplates.contains(listTemplate) {
        // java LstData lstData = listTemplate.getLstData();
        let mut lstData: LstData = listTemplate.getLstData();
        // java Assert.assertNotNull("lstData should not be null.", lstData);
        Assert::assertNotNull__string_object(&"lstData should not be null.", Some(&lstData));
        // java listTemplate.setLsid(this.mUUID);
        listTemplate.setLsid__int(self.mUUID);
        // java this.writeLockAtom();
        self.super_.writeLockAtom();
        // java this.mTemplates.add(listTemplate);
        self.mTemplates.add(listTemplate);
        // java this.mLstTable.add(lstData);
        self.mLstTable.add(&mut lstData);
      }
    }

    // java public void add(KListTemplate listTemplate, int lsid)
    pub fn add__klisttemplate_int(&mut self, listTemplate: &mut KListTemplate, lsid: i32) {
      // java LstData lstData = listTemplate.getLstData();
      let mut lstData: LstData = listTemplate.getLstData();
      // java Assert.assertNotNull("lstData should not be null.", lstData);
      Assert::assertNotNull__string_object(&"lstData should not be null.", Some(&lstData));
      // java listTemplate.setLsid(lsid);
      listTemplate.setLsid__int(lsid);
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mTemplates.add(listTemplate);
      self.mTemplates.add(listTemplate);
      // java this.mLstTable.add(lstData);
      self.mLstTable.add(&mut lstData);
    }

    // java public KListTemplate find(int lsid)
    pub fn find(&self, lsid: i32) -> Option<KListTemplate> {
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (self.mTemplates.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java for(int size = this.mTemplates.size(); i < size; ++i)
      while i < size {
        // java KListTemplate template = (KListTemplate)this.mTemplates.get(i);
        let mut template: KListTemplate = (self.mTemplates.get(i)).downcast_ref::<KListTemplate>().unwrap();
        // java Assert.assertNotNull("template should not be null.", template);
        Assert::assertNotNull__string_object(&"template should not be null.", Some(&template));
        // java if (template.getLsid() == lsid)
        if template.getLsid() == lsid {
          // java return template;
          return template;
        }
      }
      // java return null;
      return None;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java if (null != this.mTemplates)
      if let Some(mTemplates) = self.mTemplates.as_mut() {
        // java int i = 0;
        let mut i: i32 = 0;
        let mut size: i32 = (mTemplates.len() as i32);
        // java for(int size = this.mTemplates.size(); i < size; ++i)
        while i < size {
          // java KListTemplate listTemplate = (KListTemplate)this.mTemplates.get(i);
          let mut listTemplate: KListTemplate = (mTemplates.get(i)).downcast_ref::<KListTemplate>().unwrap();
          // java if (null != listTemplate)
          if listTemplate.is_some() {
            // java listTemplate.dispose();
            listTemplate.dispose();
          }
        }
        // java this.mTemplates.clear();
        mTemplates.clear();
        // java this.mTemplates = null;
        mTemplates = None;
      }
    }

}

impl std::ops::Deref for KListTemplates {
    type Target = Atom;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KListTemplates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
