// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.transaction.Atom
use util::moffice::transaction::Atom::*;
// Java: import cn.wps.moffice.transaction.AtomMember
use util::moffice::transaction::AtomMember::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.list.ListParagraphs
use crate::cn::wps::moffice::writer::core::list::ListParagraphs::*;
// Java: import cn.wps.moffice.writer.core.list.ListStyles
use crate::cn::wps::moffice::writer::core::list::ListStyles::*;
// Java: import cn.wps.moffice.writer.core.list.Lists
use crate::cn::wps::moffice::writer::core::list::Lists::*;
// Java: import cn.wps.moffice.writer.data.autonum.AutoNumTable
use crate::cn::wps::moffice::writer::data::autonum::AutoNumTable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoData
use crate::cn::wps::moffice::writer::data::autonum::LfoData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoDataTable
use crate::cn::wps::moffice::writer::data::autonum::LfoDataTable::*;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import cn.wps.moffice.writer.core.list.impl.KList
use crate::cn::wps::moffice::writer::core::list::r#impl::KList::*;
use std::collections::{HashMap};

// java public final class KLists extends Atom implements Lists
#[derive(Default)]
pub struct KLists {
    // java Atom super_
    pub(crate) super_: Atom,
    // java private TextDocument mTextDocument;
    mTextDocument: Option<TextDocument>,
    // java private TextDocument.UUNumberingId mUUid;
    // rust: Java 的嵌套类 `TextDocument.UUNumberingId` 在 Rust 侧是独立模块
    //   （`core::UUNumberingId`，已登记）⇒ 用其真实路径。
    mUUid: Option<crate::cn::wps::moffice::writer::core::UUNumberingId::UUNumberingId>,
    // java private LfoDataTable mLfoTable;
    mLfoTable: Option<LfoDataTable>,
    // java @AtomMember(1) private ArrayList<KList> mLists;
    mLists: Option<Vec<KList>>,
}

// java public interface Lists
impl Lists for KLists {
    // java public KList add()
    fn add(&mut self) -> KList {
      // java int DEFAULT_LIST_LEVEL_OVERRIDES_COUNT = 0;
      let mut DEFAULT_LIST_LEVEL_OVERRIDES_COUNT: i32 = 0;
      // java return this.add(0);
      return self.add__int(0);
    }

    // java public KList add(int lvlOverridesCount)
    fn add__int(&mut self, lvlOverridesCount: i32) -> KList {
      // java KList list = new KList(this.mTextDocument, lvlOverridesCount);
      let mut list: KList = KList::new(self.mTextDocument, lvlOverridesCount);
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.mLists.add(list);
      self.mLists.add(list);
      // java return list;
      return list;
    }

}

// java public final class KLists extends Atom implements Lists
impl KLists {
    // java public  KLists(TextDocument textDocument)
    pub fn KLists(textDocument: &TextDocument) -> Self {
      // java Assert.assertNotNull("textDocument should not be null.", textDocument);
      Assert::assertNotNull__string_object(&"textDocument should not be null.", Some(&textDocument));
      // java this.mTextDocument = textDocument;
      let mut this = Self { super_: Atom::new(), mTextDocument: textDocument, mUUid: textDocument.getUUNumberingId(), mLfoTable: lfoTable, mLists: Vec::new() };
      // java this.setTransaction(this.mTextDocument.getTransaction());
      this.super_.setTransaction__transaction(&this.mTextDocument.getTransaction());
      // java this.mUUid = textDocument.getUUNumberingId();
      // java Assert.assertNotNull("mUUid should not be null.", this.mUUid);
      Assert::assertNotNull__string_object(&"mUUid should not be null.", Some(&this.mUUid));
      // java AutoNumTable autoNumTable = textDocument.getAutoNumTable();
      let mut autoNumTable: AutoNumTable = textDocument.getAutoNumTable();
      // java Assert.assertNotNull("autoNumTable should not be null.", autoNumTable);
      Assert::assertNotNull__string_object(&"autoNumTable should not be null.", Some(&autoNumTable));
      // java LfoDataTable lfoTable = autoNumTable.getLfoDataTable();
      let mut lfoTable: LfoDataTable = autoNumTable.getLfoDataTable();
      // java Assert.assertNotNull("lfoTable should not be null.", lfoTable);
      Assert::assertNotNull__string_object(&"lfoTable should not be null.", Some(&lfoTable));
      // java this.mLfoTable = lfoTable;
      // java this.mLists = new ArrayList();
      // java this.init();
      this.init();
      this
    }

    // java private void init()
    pub fn init(&mut self) {
      // java Assert.assertNotNull("mLfoTable should not be null.", this.mLfoTable);
      Assert::assertNotNull__string_object(&"mLfoTable should not be null.", Some(&self.mLfoTable));
      // java Assert.assertNotNull("mLists should not be null.", this.mLists);
      Assert::assertNotNull__string_object(&"mLists should not be null.", Some(&self.mLists));
      // java Assert.assertNotNull("mTextDocument should not be null.", this.mTextDocument);
      Assert::assertNotNull__string_object(&"mTextDocument should not be null.", Some(&self.mTextDocument));
      // java HashMap<Integer, LfoData> map = this.mLfoTable.getLfoDataMap();
      let mut map: HashMap<i32, LfoData> = self.mLfoTable.getLfoDataMap();
      // java for(Integer numId : map.keySet())
      for numId in map.keySet().iter() {
        // java Assert.assertNotNull("numId should not be null.", numId);
        Assert::assertNotNull__string_object(&"numId should not be null.", Some(&numId));
        // java LfoData lfoData = (LfoData)map.get(numId);
        let mut lfoData: LfoData = (map.get(numId)).downcast_ref::<LfoData>().unwrap();
        // java Assert.assertNotNull("lfoData should not be null.", lfoData);
        Assert::assertNotNull__string_object(&"lfoData should not be null.", Some(&lfoData));
        // java this.mLists.add(new KList(this.mTextDocument, numId, lfoData));
        self.mLists.add(KList::new(self.mTextDocument, numId, lfoData));
      }
    }

    // java public int count()
    pub fn count(&self) -> i32 {
      // java return this.mLists.size();
      return (self.mLists.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
    }

    // java public KList item(int index)
    pub fn item(&self, index: i32) -> KList {
      // java return (KList)this.mLists.get(index);
      return (self.mLists.get(index)).downcast_ref::<KList>().unwrap();
    }

    // java public KList removeIfNotReferenced(int numId)
    pub fn removeIfNotReferenced__int(&mut self, numId: i32) -> Option<KList> {
      // java KList list = this.find(numId);
      let mut list: KList = self.find(numId);
      // java return null != list && this.removeIfNotReferenced(list) ? list : null;
      return (if list.is_some() && self.removeIfNotReferenced__int(list) { Some(list) } else { None });
    }

    // java public boolean removeIfNotReferenced(KList list)
    pub fn removeIfNotReferenced__klist(&mut self, list: &KList) -> bool {
      // java Assert.assertNotNull("list should not be null.", list);
      Assert::assertNotNull__string_object(&"list should not be null.", Some(&list));
      // java ListParagraphs listParas = list.getListParagraphs();
      let mut listParas: Box<dyn ListParagraphs> = list.getListParagraphs();
      // java if (null != listParas && listParas.count() > 0)
      if listParas.is_some() && listParas.count() > 0 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java ListStyles listStyles = list.getListStyles();
        let mut listStyles: Box<dyn ListStyles> = list.getListStyles();
        // java if (null != listStyles && listStyles.count() > 0)
        if listStyles.is_some() && listStyles.count() > 0 {
          // java return false;
          return false;
        }
        // java else
        else {
          // java this.writeLockAtom();
          self.super_.writeLockAtom();
          // java boolean removed = this.mLists.remove(list);
          let mut removed: bool = self.mLists.remove(list);
          // java Assert.assertTrue("removed should be true.", removed);
          Assert::assertTrue__string_boolean(&"removed should be true.", removed);
          // java if (removed)
          if removed {
            // java LfoData removedLfo = this.mLfoTable.remove(list.getNumId());
            let mut removedLfo: LfoData = self.mLfoTable.remove(list.getNumId());
            // java Assert.assertNotNull("removedLfo should not be null.", removedLfo);
            Assert::assertNotNull__string_object(&"removedLfo should not be null.", Some(&removedLfo));
          }
          // java return removed;
          return removed;
        }
      }
    }

    // java public KList find(int numId)
    pub fn find(&self, numId: i32) -> Option<KList> {
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (self.mLists.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java for(int size = this.mLists.size(); i < size; ++i)
      while i < size {
        // java KList list = (KList)this.mLists.get(i);
        let mut list: KList = (self.mLists.get(i)).downcast_ref::<KList>().unwrap();
        // java Assert.assertNotNull("list should not be null.", list);
        Assert::assertNotNull__string_object(&"list should not be null.", Some(&list));
        // java if (list.getNumId() == numId)
        if list.getNumId() == numId {
          // java return list;
          return list;
        }
      }
      // java return null;
      return None;
    }

    // java public KList[] getListReferenceTemplate(int lsid)
    pub fn getListReferenceTemplate(&self, lsid: i32) -> Option<Vec<KList>> {
      // java ArrayList<KList> listArray = new ArrayList();
      let mut listArray: Vec<KList> = Vec::new();
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (self.mLists.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java for(int size = this.mLists.size(); i < size; ++i)
      while i < size {
        // java KList list = (KList)this.mLists.get(i);
        let mut list: KList = (self.mLists.get(i)).downcast_ref::<KList>().unwrap();
        // java Assert.assertNotNull("list should not be null.", list);
        Assert::assertNotNull__string_object(&"list should not be null.", Some(&list));
        // java if (list.getLsid() == lsid)
        if list.getLsid() == lsid {
          // java listArray.add(list);
          listArray.add(list);
        }
      }
      // java i = listArray.size();
      i = (listArray.len() as i32);
      // java return i > 0 ? (KList[])listArray.toArray(new KList[i]) : null;
      return (if i > 0 { Some(todo!("cast → Vec<KList>")) } else { None });
    }

}

impl std::ops::Deref for KLists {
    type Target = Atom;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KLists {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
