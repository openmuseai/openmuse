#![allow(non_snake_case, dead_code, unused_imports)]
// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
// Java: import cn.wps.moffice.writer.core.io.IOListener
use crate::cn::wps::moffice::writer::core::io::IOListener::IOListener;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteTxt
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteTxt::{FootnoteEndnoteTxtNode, PLCFootnoteEndnoteTxt};
// Java: import cn.wps.moffice.writer.helper.CoreHelper
use crate::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.NumIdRedirection
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::NumIdRedirection;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::StyleId2Sti;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.document.SubDocumentImporter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::SubDocumentImporter;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::{Attributes, ConvertHelper};
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.UUFootnoteEndnoteId
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::UUFootnoteEndnoteId::UUFootnoteEndnoteId;
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::Xml03Context;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
// Java: import java.util.HashMap
// Java: import java.util.HashSet
// Java: import java.util.Set
use std::collections::{HashMap, HashSet};

// java public abstract class FootnoteEndnoteDocumentImporter extends SubDocumentImporter
pub struct FootnoteEndnoteDocumentImporter {
    // java SubDocumentImporter super_
    pub(crate) super_: SubDocumentImporter,
    // java protected UUFootnoteEndnoteId mFootEndnoteIdMaker = new UUFootnoteEndnoteId();
    mFootEndnoteIdMaker: Option<UUFootnoteEndnoteId>,
    // java private HashMap<Integer, PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode> mId2TxtNode = new HashMap();
    mId2TxtNode: HashMap<i32, FootnoteEndnoteTxtNode>,
    // java private HashMap<Integer, Integer> mOld2NewId = new HashMap();
    mOld2NewId: HashMap<i32, i32>,
    // java private HashMap<Integer, Integer> mRepeat2Id = new HashMap();
    mRepeat2Id: HashMap<i32, i32>,
    // java private Set<Integer> mMainId = new HashSet();
    mMainId: HashSet<i32>,
    // java private int mFootEndNoteRepeat = 0;
    mFootEndNoteRepeat: i32,
    // java private int mMainRepeat = 0;
    mMainRepeat: i32,
}

impl Default for FootnoteEndnoteDocumentImporter {
    fn default() -> Self {
        Self {
            super_: SubDocumentImporter::default(),
            mFootEndnoteIdMaker: Some(UUFootnoteEndnoteId::UUFootnoteEndnoteId()),
            mId2TxtNode: HashMap::new(),
            mOld2NewId: HashMap::new(),
            mRepeat2Id: HashMap::new(),
            mMainId: HashSet::new(),
            mFootEndNoteRepeat: 0,
            mMainRepeat: 0,
        }
    }
}

// java public abstract class FootnoteEndnoteDocumentImporter extends SubDocumentImporter
impl FootnoteEndnoteDocumentImporter {
    // java protected  FootnoteEndnoteDocumentImporter(SubDocument document, StyleId2Sti styleId2Sti, NumIdRedirection numIdRedirection, IOListener ioListener, boolean initDrawingContainer, int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn FootnoteEndnoteDocumentImporter(document: &mut SubDocument, styleId2Sti: StyleId2Sti, numIdRedirection: NumIdRedirection, ioListener: Option<Box<dyn IOListener>>, initDrawingContainer: bool, mode: i32, xml07Context: Option<Xml07Context>, xml03Context: Option<Xml03Context>) -> Self {
      // java super(document, styleId2Sti, numIdRedirection, ioListener, true, mode, xml07Context, xml03Context);
      // rust: ★M-STAGE2 P1-D1 —— Java 原文此处**硬编码 `true`**（忽略入参
      //   `initDrawingContainer`），如实复现。
      let _ = initDrawingContainer;
      Self {
        super_: SubDocumentImporter::SubDocumentImporter(document, styleId2Sti, numIdRedirection, ioListener, true, mode, xml07Context, xml03Context),
        ..Default::default()
      }
    }

    // java public void onImportFootnoteEndNoteStart(PLCFootnoteEndnoteTxt plcfootnoteEndnoteTxt, Attributes attributes)
    pub fn onImportFootnoteEndNoteStart(&mut self, plcfootnoteEndnoteTxt: &mut PLCFootnoteEndnoteTxt, attributes: &dyn Attributes) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java IOAssert.assertNotNull("plcfootnoteEndnoteTxt should not be null!", plcfootnoteEndnoteTxt);
      // java IOAssert.assertNotNull("attributes should not be null!", attributes);
      // java Integer oldId = ConvertHelper.getInt(attributes, "id");
      let mut oldId: Option<i32> = ConvertHelper::getInt(attributes, "id");
      // java IOAssert.assertNotNull("oldId should not be null", oldId);
      IOAssert::assertNotNull__string_object("oldId should not be null", Some(&oldId));
      // java if (null != oldId)
      if oldId.is_some() {
        // java int newId = this.getFootEndNodeNewId(oldId);
        let mut newId: i32 = self.getFootEndNodeNewId(oldId.unwrap());
        // java int cpStart = this.mDocument.getLength();
        let mut cpStart: i32 = self.super_.mDocument.map_or(0, |d| unsafe { (*d).getLength() });
        // java if (!plcfootnoteEndnoteTxt.isHead(plcfootnoteEndnoteTxt.find(cpStart)))
        // java CoreHelper.appendParagraph(this.mDocument, '\r', PropertySet.empty, PropertySet.empty);
        // java cpStart = this.mDocument.getLength();
        cpStart = self.super_.mDocument.map_or(0, |d| unsafe { (*d).getLength() });
        // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode txtNode = plcfootnoteEndnoteTxt.addFootnoteEndnoteTxt(cpStart);
        let mut txtNode: FootnoteEndnoteTxtNode = plcfootnoteEndnoteTxt.addFootnoteEndnoteTxt(cpStart);
        // java this.mId2TxtNode.put(newId, txtNode);
        self.mId2TxtNode.insert(newId, txtNode);
      }
      // rust: ★M-STAGE2 P1-D1 —— 脚注/尾注导入属**非管线核心**：
      //   语料 C1/C2a/C2b/C6′ 的 `<w:footnoteReference>` / `<w:endnoteReference>` 均为 0，
      //   `DocumentImporter` 的分发分支永不进入。按用户指令合法延迟。
      */
    }

    // java private int getFootEndNodeNewId(int oldId)
    fn getFootEndNodeNewId(&mut self, oldId: i32) -> i32 {
      // java IOAssert.assertNotNull("mFootEndnoteIdMaker should not be null!", this.mFootEndnoteIdMaker);
      IOAssert::assertNotNull__string_object("mFootEndnoteIdMaker should not be null!", Some(&self.mFootEndnoteIdMaker));
      // java int newId = this.mFootEndnoteIdMaker.getUUFootnoteEndnoteId();
      let mut newId: i32 = self.mFootEndnoteIdMaker.as_mut().map_or(0, |m| m.getUUFootnoteEndnoteId());
      // java if (this.mOld2NewId.containsKey(oldId))
      if self.mOld2NewId.contains_key(&oldId) {
        // java this.mRepeat2Id.put(++this.mFootEndNoteRepeat, newId);
        self.mFootEndNoteRepeat += 1;
        let k = self.mFootEndNoteRepeat;
        self.mRepeat2Id.insert(k, newId);
      }
      // java else
      else {
        // java this.mOld2NewId.put(oldId, newId);
        self.mOld2NewId.insert(oldId, newId);
      }
      // java return newId;
      return newId;
    }

    // java public PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode getEndnoteStartCpbyId(int id)
    pub fn getEndnoteStartCpbyId(&self, id: i32) -> Option<FootnoteEndnoteTxtNode> {
      // java IOAssert.assertNotNull("mId2TxtNode should not be null", this.mId2TxtNode);
      // java return (PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode)this.mId2TxtNode.get(id);
      return self.mId2TxtNode.get(&id).cloned();
    }

    // java public Integer getNewMainId(Integer mainId)
    pub fn getNewMainId(&mut self, mainId: Option<i32>) -> Option<i32> {
      // java if (null == mainId)
      if mainId.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        let id = mainId.unwrap();
        // java return this.mMainId.add(mainId) ? (Integer)this.mOld2NewId.get(mainId) : (Integer)this.mRepeat2Id.get(++this.mMainRepeat);
        if self.mMainId.insert(id) {
          return self.mOld2NewId.get(&id).copied();
        } else {
          self.mMainRepeat += 1;
          let k = self.mMainRepeat;
          return self.mRepeat2Id.get(&k).copied();
        }
      }
    }

    // java public void clear()
    pub fn clear(&mut self) {
      // java this.mId2TxtNode.clear();
      self.mId2TxtNode.clear();
      // java this.mOld2NewId.clear();
      self.mOld2NewId.clear();
      // java this.mRepeat2Id.clear();
      self.mRepeat2Id.clear();
      // java this.mMainId.clear();
      self.mMainId.clear();
      // java this.mFootEndnoteIdMaker = null;
      self.mFootEndnoteIdMaker = None;
    }
}

impl std::ops::Deref for FootnoteEndnoteDocumentImporter {
    type Target = SubDocumentImporter;
    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for FootnoteEndnoteDocumentImporter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
