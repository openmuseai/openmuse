// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
use util::kfc::r#impl::map::WIntObjectHashMap::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.io.IOThreadSign
use crate::cn::wps::moffice::writer::core::io::IOThreadSign::*;
// Java: import cn.wps.moffice.writer.core.shape.KShapeRange
use crate::cn::wps::moffice::writer::core::shape::KShapeRange::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::{PLC, Node as PlcNode};
// Java: import cn.wps.moffice.writer.data.PLCSection
use crate::cn::wps::moffice::writer::data::PLCSection::*;
// Java: import cn.wps.moffice.writer.data.table.CellInfo
use crate::cn::wps::moffice::writer::data::table::CellInfo::*;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
use crate::cn::wps::moffice::writer::data::table::RowInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfos
use crate::cn::wps::moffice::writer::data::table::TableInfos::*;
// Java: import cn.wps.util.concurrent.locks.RangeWriteLock
use crate::cn::wps::util::concurrent::locks::RangeWriteLock::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.insert.InsertEnv
use crate::cn::wps::moffice::writer::insert::InsertEnv::*;
// Java: import cn.wps.moffice.writer.insert.InsertDocument
// CARGO_BLOCKER_UNUSED_MODULE: InsertDocument 未改写未挂载（既有），use 裁剪
// use crate::cn::wps::moffice::writer::insert::InsertDocument::*;
// Java: import java.lang.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;
// Java: import cn.wps.io.dom.Node
use io_wps_io::io::dom::Node as DomNode;

// java public class InsertFile
#[derive(Default)]
pub struct InsertFile {
    // java InsertEnv mEnv;
    pub(crate) mEnv: Option<InsertEnv>,
}

// java public class InsertFile
impl InsertFile {
    // java public boolean Insert(InsertEnv env)
    pub fn Insert(&mut self, env: &InsertEnv) -> bool {
      // java this.mEnv = env;
      self.mEnv = env;
      // java if (!this.loadDocument())
      if !self.loadDocument() {
        // java this.mEnv = null;
        self.mEnv = None;
        // java return false;
        return false;
      }
      // java else
      else {
            // java RangeWriteLock lock = this.mEnv.mInsertRange.getMainDocument().writeLock();
            let mut lock: RangeWriteLock = self.mEnv.mInsertRange.getMainDocument().writeLock();
                // java this._start();
                self._start();
                // java this.adjustRange();
                self.adjustRange();
                // java this.mEnv.mInsertRange.deleteRange();
                self.mEnv.mInsertRange.deleteRange();
                // java Document subDocument = this.mEnv.mInsertRange.getSubDocument();
                let mut subDocument: Box<dyn Document> = self.mEnv.mInsertRange.getSubDocument();
                // java int type = subDocument.getType();
                let mut r#type: i32 = subDocument.getType();
                // java IPapxTable.Entry paprEntry = subDocument.getPapxTable().seek(this.mEnv.mInsertRange.getStart());
                let mut paprEntry: Box<dyn Entry> = subDocument.getPapxTable().seek(self.mEnv.mInsertRange.getStart());
                // java if (paprEntry.getProperty().getBoolean(223, false))
                if paprEntry.getProperty().getBoolean(223, false) {
                  // java this.mEnv.mTableDepth = paprEntry.getProperty().getInt(224, 0);
                  self.mEnv.mTableDepth = paprEntry.getProperty().getInt(224, 0);
                  // java this.mEnv.mTableInfo = subDocument.getTableInfos().getTableInfoByLevel(this.mEnv.mInsertRange.getStart(), env.mTableDepth);
                  self.mEnv.mTableInfo = subDocument.getTableInfos().getTableInfoByLevel(self.mEnv.mInsertRange.getStart(), env.mTableDepth);
                }
                // java if (type == 5)
                if r#type == 5 {
                  // java this.mEnv.mJustText = true;
                  self.mEnv.mJustText = true;
                }
                // java this.insertDocuments();
                self.insertDocuments();
                // java this.addRsidToDestDocument();
                self.addRsidToDestDocument();
                // java this.addDrawingRIDMapToDestDocument();
                self.addDrawingRIDMapToDestDocument();
                  // java lock.unlock();
                  lock.unlock();
                  // java this._commit("insert File Content!");
                  self._commit(&"insert File Content!");
            // java this.mEnv.mSrcTextDocument.close();
            self.mEnv.mSrcTextDocument.close();
            // java return true;
            return true;
            // java Exception e
            todo!();
              // java e.printStackTrace();
              e.printStackTrace();
              // java return false;
              return false;
      }
    }

    // java private void addDrawingRIDMapToDestDocument()
    pub fn addDrawingRIDMapToDestDocument(&self) {
      // java TextDocument srcTextDocument = this.mEnv.mSrcTextDocument;
      let mut srcTextDocument: TextDocument = self.mEnv.mSrcTextDocument;
      // java WIntObjectHashMap<String> srcRidMap = srcTextDocument.getShapeId2DrawingRID();
      let mut srcRidMap: WIntObjectHashMap = srcTextDocument.getShapeId2DrawingRID();
      // java TextDocument destTextDocument = this.mEnv.mDestTextDocument;
      let mut destTextDocument: TextDocument = self.mEnv.mDestTextDocument;
      // java WIntObjectHashMap<String> destRidMap = destTextDocument.getShapeId2DrawingRID();
      let mut destRidMap: WIntObjectHashMap = destTextDocument.getShapeId2DrawingRID();
      // java destRidMap.putAll(srcRidMap);
      destRidMap.putAll(srcRidMap);
    }

    // java private void addRsidToDestDocument()
    pub fn addRsidToDestDocument(&self) {
      // java TextDocument srcTextDocument = this.mEnv.mSrcTextDocument;
      let mut srcTextDocument: TextDocument = self.mEnv.mSrcTextDocument;
      // java ArrayList<Integer> srcRsids = srcTextDocument.getRsids();
      let mut srcRsids: Vec<i32> = srcTextDocument.getRsids();
      // java TextDocument destTextDocument = this.mEnv.mDestTextDocument;
      let mut destTextDocument: TextDocument = self.mEnv.mDestTextDocument;
      // java ArrayList<Integer> destRsids = destTextDocument.getRsids();
      let mut destRsids: Vec<i32> = destTextDocument.getRsids();
      // java destRsids.addAll(srcRsids);
      destRsids.addAll(srcRsids);
    }

    // java private void adjustRange()
    pub fn adjustRange(&mut self) {
      // java if (this.mEnv.mInsertRange.getStart() != this.mEnv.mInsertRange.getEnd())
      if self.mEnv.mInsertRange.getStart() != self.mEnv.mInsertRange.getEnd() {
        // java this.adjustRangeWithTable();
        self.adjustRangeWithTable();
      }
    }

    // java private void adjustRangeWithTable()
    pub fn adjustRangeWithTable(&mut self) {
      // java int startCp = this.mEnv.mInsertRange.getStart();
      let mut startCp: i32 = self.mEnv.mInsertRange.getStart();
      // java int endCp = this.mEnv.mInsertRange.getEnd();
      let mut endCp: i32 = self.mEnv.mInsertRange.getEnd();
      // java Document subDocument = this.mEnv.mInsertRange.getSubDocument();
      let mut subDocument: Box<dyn Document> = self.mEnv.mInsertRange.getSubDocument();
      // java TableInfos tableInfos = subDocument.getTableInfos();
      let mut tableInfos: Box<dyn TableInfos> = subDocument.getTableInfos();
      let mut i: i32 = 0;
      // java for(int i = 0; i < tableInfos.size(); ++i)
      while i < (tableInfos.len() as i32) {
        // java TableInfo table = tableInfos.getTableByIndex(i);
        let mut table: TableInfo = tableInfos.getTableByIndex(i);
        // java int tableStart = table.getStartCP();
        let mut tableStart: i32 = table.getStartCP();
        // java int tableEnd = table.getEndCP();
        let mut tableEnd: i32 = table.getEndCP();
        // java RowInfo rowInfo = table.getRowByCP(startCp);
        let mut rowInfo: RowInfo = table.getRowByCP(startCp);
        // java if (null != rowInfo)
        if rowInfo.is_some() {
          // java CellInfo cellInfo = rowInfo.getCellByCP(startCp);
          let mut cellInfo: CellInfo = rowInfo.getCellByCP(startCp);
          // java if (startCp >= tableStart && endCp < tableEnd)
          if startCp >= tableStart && endCp < tableEnd {
            // java if (startCp >= cellInfo.getStartCP() && cellInfo.getEndCP() <= endCp)
            if startCp >= cellInfo.getStartCP() && cellInfo.getEndCP() <= endCp {
              // java this.mEnv.mInsertRange.setRange(startCp, startCp);
              self.mEnv.mInsertRange.setRange(startCp, startCp);
              // java break;
              break;
            }
          }
          // java else if (startCp >= tableStart && endCp == tableEnd)
          else if startCp >= tableStart && endCp == tableEnd {
            // java this.mEnv.mInsertRange.setRange(cellInfo.getStartCP(), cellInfo.getEndCP());
            self.mEnv.mInsertRange.setRange(cellInfo.getStartCP(), cellInfo.getEndCP());
          }
        }
      }
    }

    // java private boolean loadDocument()
    pub fn loadDocument(&mut self) -> bool {
      // java if (null != this.mEnv.mSrcTextDocument)
      if self.mEnv.mSrcTextDocument.is_some() {
        // java return true;
        return true;
      }
      // java else
      else {
        // java this.mEnv.mSrcTextDocument = new TextDocument();
        self.mEnv.mSrcTextDocument = TextDocument::new();
        // java this.mEnv.mSrcTextDocument.openDocument(this.mEnv.mInsertFile, this.mEnv.mInsertFilePassword);
        self.mEnv.mSrcTextDocument.openDocument(self.mEnv.mInsertFile, self.mEnv.mInsertFilePassword);
        // java int nRet = 0;
        let mut nRet: i32 = 0;
            // java nRet = this.mEnv.mSrcTextDocument.startLoading(this.mEnv.mIoListener, new IOThreadSign());
            nRet = self.mEnv.mSrcTextDocument.startLoading(self.mEnv.mIoListener, IOThreadSign::new());
            // java IOException e
            todo!();
              // java e.printStackTrace();
              e.printStackTrace();
              // java this.mEnv.mSrcTextDocument = null;
              self.mEnv.mSrcTextDocument = None;
              // java return false;
              return false;
        // java return nRet == 0;
        return nRet == 0;
      }
    }

    // java private void insertDocuments()
    pub fn insertDocuments(&mut self) {
      // java this.mEnv.mDestTextDocument.start();
      self.mEnv.mDestTextDocument.start();
          // java this.insertFontTable();
          self.insertFontTable();
          // java InsertDocument insert = new InsertDocument(this.mEnv);
          let mut insert: InsertDocument = InsertDocument::new(self.mEnv);
          // java insert.Insert(this.mEnv.mSrcTextDocument.getMainDocument(), this.mEnv.mInsertRange.getSubDocument());
          insert.Insert(self.mEnv.mSrcTextDocument.getMainDocument(), self.mEnv.mInsertRange.getSubDocument());
          // java if (this.mEnv.mInsertRange.getSubDocument().getType() == 0)
          if self.mEnv.mInsertRange.getSubDocument().getType() == 0 {
            // java this.insertHeaderDocument();
            self.insertHeaderDocument();
          }
            // java this.mEnv.mDestTextDocument.commit("insert file");
            self.mEnv.mDestTextDocument.commit("insert file");
    }

    // java private void insertFontTable()
    pub fn insertFontTable(&self) {
      // java this.mEnv.mDestTextDocument.getFontTable().add(this.mEnv.mSrcTextDocument.getFontTable());
      self.mEnv.mDestTextDocument.getFontTable().add(self.mEnv.mSrcTextDocument.getFontTable());
    }

    // java private void insertHeaderDocument()
    pub fn insertHeaderDocument(&mut self) {
      // java Document srcDocument = this.mEnv.mSrcTextDocument.getSubdocument(2);
      let mut srcDocument: Box<dyn Document> = self.mEnv.mSrcTextDocument.getSubdocument(2);
      // java Document destDocument = this.mEnv.mDestTextDocument.getSubdocument(2);
      let mut destDocument: Box<dyn Document> = self.mEnv.mDestTextDocument.getSubdocument(2);
      // java InsertEnv env = new InsertEnv();
      let mut env: InsertEnv = InsertEnv::new();
      // java env.mDestDocument = destDocument;
      env.mDestDocument = destDocument;
      // java env.mSrcDocument = srcDocument;
      env.mSrcDocument = srcDocument;
      // java env.mDestTextDocument = this.mEnv.mDestTextDocument;
      env.mDestTextDocument = self.mEnv.mDestTextDocument;
      // java env.mSrcTextDocument = this.mEnv.mSrcTextDocument;
      env.mSrcTextDocument = self.mEnv.mSrcTextDocument;
      // java if (srcDocument.getLength() > 0 && destDocument.getLength() < 1)
      if srcDocument.getLength() > 0 && destDocument.getLength() < 1 {
        // java this.copyHeaderSection();
        self.copyHeaderSection();
        // java env.mInsertRange = new KRange(destDocument, 0);
        env.mInsertRange = KRange::new(destDocument, 0);
        // java env.mShapeRange = new KShapeRange(destDocument, env.mInsertRange);
        env.mShapeRange = KShapeRange::new(destDocument, env.mInsertRange);
        // java InsertDocument insert = new InsertDocument(env);
        let mut insert: InsertDocument = InsertDocument::new(env);
        // java insert.Insert(srcDocument, destDocument);
        insert.Insert(srcDocument, destDocument);
      }
    }

    // java private void copyHeaderSection()
    pub fn copyHeaderSection(&mut self) {
      // java PLCSection sectionSrc = this.mEnv.mSrcTextDocument.getMainDocument().getPLCSection();
      let mut sectionSrc: PLCSection = self.mEnv.mSrcTextDocument.getMainDocument().getPLCSection();
      // java PLCSection sectionDest = this.mEnv.mDestTextDocument.getMainDocument().getPLCSection();
      let mut sectionDest: PLCSection = self.mEnv.mDestTextDocument.getMainDocument().getPLCSection();
      // java sectionDest.reset();
      sectionDest.reset();
      // java if (!sectionSrc.isEmpty())
      if !sectionSrc.is_empty() {
        // java PLC.Iterator iterSrc = sectionSrc.createIterator();
        let mut iterSrc: Vec<()> = sectionSrc.createIterator();
        // java while (iterSrc.hasNext())
        while iterSrc.hasNext() {
          // java PLCSection.SectionNode nodeSrc = (PLCSection.SectionNode)iterSrc.next();
          let mut nodeSrc: PLCSection::SectionNode = (iterSrc.next()).downcast_ref::<PLCSection::SectionNode>().unwrap();
          // java PLCSection.SectionNode nodeDest = sectionDest.addSection(iterSrc.getStart());
          let mut nodeDest: PLCSection::SectionNode = sectionDest.addSection(iterSrc.getStart());
          // java this.copySectionNode(nodeSrc, nodeDest);
          self.copySectionNode(&nodeSrc, &mut nodeDest);
        }
      }
    }

    // java private void copySectionNode(PLCSection.SectionNode src, PLCSection.SectionNode dest)
    pub fn copySectionNode(&mut self, src: &PLCSection::SectionNode, dest: &mut PLCSection::SectionNode) {
          // java dest.sepx = src.sepx.clone();
          dest.sepx = src.sepx.clone();
          // java CloneNotSupportedException var5
          todo!();
      // java PLC.Node nodeSrc = src.getEvenHeader();
      let mut nodeSrc: Box<dyn DomNode> = src.getEvenHeader();
      // java PLC.Node nodeDest = new PLC.Node();
      let mut nodeDest: Box<dyn DomNode> = DomDomDomNode::new();
      // java nodeDest.moveRight(nodeSrc.getCP());
      nodeDest.moveRight(nodeSrc.getCP());
      // java dest.setEvenHeader(nodeDest);
      dest.setEvenHeader(nodeDest);
      // java nodeSrc = src.getOddHeader();
      nodeSrc = src.getOddHeader();
      // java nodeDest = new PLC.Node();
      nodeDest = DomDomDomNode::new();
      // java nodeDest.moveRight(nodeSrc.getCP());
      nodeDest.moveRight(nodeSrc.getCP());
      // java dest.setOddHeader(nodeDest);
      dest.setOddHeader(nodeDest);
      // java nodeSrc = src.getEvenFooter();
      nodeSrc = src.getEvenFooter();
      // java nodeDest = new PLC.Node();
      nodeDest = DomDomDomNode::new();
      // java nodeDest.moveRight(nodeSrc.getCP());
      nodeDest.moveRight(nodeSrc.getCP());
      // java dest.setEvenFooter(nodeDest);
      dest.setEvenFooter(nodeDest);
      // java nodeSrc = src.getOddFooter();
      nodeSrc = src.getOddFooter();
      // java nodeDest = new PLC.Node();
      nodeDest = DomDomDomNode::new();
      // java nodeDest.moveRight(nodeSrc.getCP());
      nodeDest.moveRight(nodeSrc.getCP());
      // java dest.setOddFooter(nodeDest);
      dest.setOddFooter(nodeDest);
      // java nodeSrc = src.getFirstHeader();
      nodeSrc = src.getFirstHeader();
      // java nodeDest = new PLC.Node();
      nodeDest = DomDomDomNode::new();
      // java nodeDest.moveRight(nodeSrc.getCP());
      nodeDest.moveRight(nodeSrc.getCP());
      // java dest.setFirstHeader(nodeDest);
      dest.setFirstHeader(nodeDest);
      // java nodeSrc = src.getFirstFooter();
      nodeSrc = src.getFirstFooter();
      // java nodeDest = new PLC.Node();
      nodeDest = DomDomDomNode::new();
      // java nodeDest.moveRight(nodeSrc.getCP());
      nodeDest.moveRight(nodeSrc.getCP());
      // java dest.setFirstFooter(nodeDest);
      dest.setFirstFooter(nodeDest);
    }

    // java  void _start()
    pub fn _start(&mut self) {
      // java this.mEnv.mDestTextDocument.start();
      self.mEnv.mDestTextDocument.start();
    }

    // java  void _commit(String description)
    pub fn _commit(&mut self, description: &str) {
      // java this.mEnv.mDestTextDocument.commit(description);
      self.mEnv.mDestTextDocument.commit(description);
    }

}
