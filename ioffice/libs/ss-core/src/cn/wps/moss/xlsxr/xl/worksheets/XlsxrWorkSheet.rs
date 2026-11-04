// java package cn.wps.moss.xlsxr.xl.worksheets;

// java import cn.wps.base.log.IOLog;
// rust: no-stmt
// java import cn.wps.kfc.ooxfr.OxfXlRelations;
// rust: no-stmt — DRAWINGS 用 getRelsType() 原文。登记 §1.7.1。
// java import cn.wps.kfc.opcr.OpcrPackagePart;
use kfc_kfc_alpha::kfc::opcr::OpcrPackage::OpcrPackage;
use kfc_kfc_alpha::kfc::opcr::OpcrPackagePart::OpcrPackagePart;
use kfc_kfc_alpha::kfc::opcr::ZipFile::ZipFile;
// java import cn.wps.kfc.opcr.OpcrRelationship;
// rust: no-stmt
// java import cn.wps.kfc.opcr.OpcrRelationships;
// rust: no-stmt
// java import cn.wps.moffice.drawing.OLE;
// rust: no-stmt
// java import cn.wps.moffice.drawing.graphics.FillBase;
// rust: no-stmt
// java import cn.wps.moffice.drawing.line.LineProperty;
// rust: no-stmt
// java import cn.wps.moffice.drawing.media.MediaTypeEnum;
// rust: no-stmt
// java import cn.wps.moffice.kfs.File;
// rust: no-stmt
// java import cn.wps.moffice.slim.ISlimToolManager;
// rust: no-stmt
// java import cn.wps.moffice.util.StringUtil;
// rust: no-stmt
// java import cn.wps.moss.app.KmoSheet;
use crate::java_runtime::{Cell, RefCell};
use std::io::{self, Read};
use crate::java_runtime::Rc;

use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
// java import cn.wps.moss.app.OleObject;
// rust: no-stmt
// java import cn.wps.moss.core.drawing.KmoShape;
// rust: no-stmt
// java import cn.wps.moss.io.ole.KmoOle;
// rust: no-stmt
// java import cn.wps.moss.io.ole.OleHelper;
// rust: no-stmt
// java import cn.wps.moss.xlsxr.handler.sheet.SparklineDelayHandler;
// rust: no-stmt
// java import cn.wps.moss.xlsxr.handler.sheet.WorksheetDelayHandler;
// rust: no-stmt
// java import cn.wps.moss.xlsxr.handler.sheet.WorksheetHandler;
use crate::cn::wps::moss::xlsxr::handler::sheet::WorksheetHandler::WorksheetHandler;
// java import cn.wps.moss.xlsxr.handler.sheet.table.TableHandler;
// rust: no-stmt
// java import cn.wps.moss.xlsxr.model.StylesTable;
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
// java import cn.wps.moss.xlsxr.util.UnifiedXmlReader;
use crate::cn::wps::moss::xlsxr::util::UnifiedXmlReader::UnifiedXmlReader;
// java import cn.wps.moss.xlsxr.util.XlsxrFileCache;
// rust: no-stmt
// java import cn.wps.moss.xlsxr.util.XlsxrPackage;
// rust: no-stmt
// java import cn.wps.moss.xlsxr.xl.XlsxrComments;
// rust: no-stmt
// java import cn.wps.moss.xlsxr.xl.XlsxrWorkbook;
// rust: no-stmt
// java import cn.wps.moss.xlsxr.xl.drawings.XlsxrDrawing;
// rust: no-stmt
// java import cn.wps.moss.xlsxr.xl.drawings.XlsxrVmlDrawing;
// rust: no-stmt
// java import cn.wps.moss.xlsxr.xl.worksheets.block.XlsxrRowsBlockManager;
// rust: no-stmt
// java import cn.wps.moss.xlsxr.xl.worksheets.block.XlsxrSheetTraversal;
// rust: no-stmt
// java import java.io.IOException;
// rust: no-stmt
// java import java.io.InputStream;
// rust: no-stmt
// java import java.util.ArrayList;
// rust: no-stmt
// java import java.util.List;
// rust: no-stmt
// java import java.util.Map;
// rust: no-stmt
// java import org.apache.poi.ddf.BlipType;
// rust: no-stmt
// java import org.apache.poi.hwpf.ole.OleClsTypeMatcher;
// rust: no-stmt
// java import cn.wps.moss.xlsxr.xl.worksheets.XlsxrSheet;
// rust: no-stmt

use std::collections::HashMap;

// java private static final String TAG = null;
const TAG: &str = "";
// java private static final int SMALL_SHEET = 65536;
const SMALL_SHEET: usize = 65536;

const DRAWINGS_RELS: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing";

// java public final class XlsxrWorkSheet implements XlsxrSheet
pub struct XlsxrWorkSheet {
    // java private XlsxrWorkbook mBookReader;
    mBookReader:crate::java_runtime::Weak<super::super::XlsxrWorkbook::XlsxrWorkbook>,
    // java private OpcrRelationship mSheetRels;
    mSheetRels: String,
    // java private KmoSheet mSheet;
    mSheet: Rc<RefCell<KmoSheet>>,
    // java private int mSheetIndex;
    mSheetIndex: i32,
    // java private boolean bActiveSheet;
    bActiveSheet: Cell<bool>,
    // java private boolean isMIndex;
    isMIndex: Cell<bool>,
    // java private boolean isMIndex2;
    isMIndex2: Cell<bool>,
    // java private boolean isFinishRows;
    isFinishRows: Cell<bool>,
    // java XlsxrRowsBlockManager mBlocksManager;
    mBlocksManager: Option<()>,
    // java private StylesTable mStylesTable;
    mStylesTable: Option<Rc<RefCell<StylesTable>>>,
    // java private WorksheetHandler mSheetHandler;
    mSheetHandler: RefCell<Option<WorksheetHandler>>,
    // java private int mBackupNotifyBeginRow = -1;
    mBackupNotifyBeginRow: Cell<i32>,
    // java private int mBackupNotifyEndRow = -1;
    mBackupNotifyEndRow: Cell<i32>,
    // java private String mBackgroundRid = null;
    mBackgroundRid: Option<String>,
    // java private boolean bHasDataValidations = false;
    bHasDataValidations: Cell<bool>,
    // java private boolean bHasConditionalFormatting = false;
    bHasConditionalFormatting: Cell<bool>,
    // java private boolean bHasSparklineGroups = false;
    bHasSparklineGroups: Cell<bool>,
    // java public boolean isFirstScreen = false;
    pub isFirstScreen: Cell<bool>,
    // java private final ArrayList<OleObject> mOleObjects = new ArrayList();
    mOleObjects: Vec<String>,
    // java private List<String> mTableParts;
    mTableParts: Option<Vec<String>>,
    // rust: workbook 部件的 ZipFile，和 mSheetRels 的 part name 一起重建 sheet 部件。
    zip: Rc<RefCell<Option<ZipFile>>>,
}

// java public final class XlsxrWorkSheet implements XlsxrSheet
impl XlsxrWorkSheet {
    // java public XlsxrWorkSheet(KmoSheet sheet, OpcrRelationship rels, XlsxrWorkbook bookReader, int sheetIndex, StylesTable stylesTable)
    pub fn XlsxrWorkSheet(
        sheet: Rc<RefCell<KmoSheet>>,
        part_name: String,
        zip: Rc<RefCell<Option<ZipFile>>>,
        bookReader:crate::java_runtime::Weak<super::super::XlsxrWorkbook::XlsxrWorkbook>,
        sheetIndex: i32,
        stylesTable: Option<Rc<RefCell<StylesTable>>>,
    ) -> Rc<Self> {
        // java this.mSheet = sheet;
        // java this.mSheetRels = rels;
        // java this.mSheet.setSheetType((byte)0);
        sheet.borrow().setSheetType(0);
        // java this.mBookReader = bookReader;
        // Native weak back-reference to the original enclosing reader.
        // java this.mSheetIndex = sheetIndex;
        // java this.isMIndex = false;
        // java this.isMIndex2 = false;
        // java this.isFinishRows = false;
        // java this.mStylesTable = stylesTable;
        // java this.bActiveSheet = false;
        Rc::new(Self {
            mBookReader:bookReader,
            mSheetRels: part_name,
            mSheet: sheet,
            mSheetIndex: sheetIndex,
            bActiveSheet: Cell::new(false),
            isMIndex: Cell::new(false),
            isMIndex2: Cell::new(false),
            isFinishRows: Cell::new(false),
            mBlocksManager: None,
            mStylesTable: stylesTable,
            mSheetHandler: RefCell::new(None),
            mBackupNotifyBeginRow: Cell::new(-1),
            mBackupNotifyEndRow: Cell::new(-1),
            mBackgroundRid: None,
            bHasDataValidations: Cell::new(false),
            bHasConditionalFormatting: Cell::new(false),
            bHasSparklineGroups: Cell::new(false),
            isFirstScreen: Cell::new(false),
            mOleObjects: Vec::new(),
            mTableParts: None,
            zip,
        })
    }

    fn package_part(&self) -> Option<OpcrPackagePart> {
        let zip = self.zip.borrow().clone()?;
        let entry = OpcrPackage::getEntry(&zip, Some(self.mSheetRels.as_str()))?;
        Some(OpcrPackagePart::OpcrPackagePart(&zip, &entry))
    }

    // java public void open() throws IOException
    pub fn open(&self) -> io::Result<()> {
        // java if (!this.isMIndex || !this.isMIndex2 || !this.isFinishRows)
        if !self.isMIndex.get() || !self.isMIndex2.get() || !self.isFinishRows.get() {
            // java if (!this.isMIndex)
            if !self.isMIndex.get() {
                // java OpcrPackagePart sheetPart = this.mSheetRels.getPackagePart();
                let part = self.package_part();
                if part.is_none() {
                    panic!("NullPointerException");
                }
                let mut sheetPart = part.unwrap();
                // java this.checkSheetHasDrawingXml(sheetPart);
                self.checkSheetHasDrawingXml(&mut sheetPart)?;
                // java if (this.isLargeSheet(sheetPart))
                if self.isLargeSheet(Some(&sheetPart)) {
                    // java boolean openedFirstScreen = this.openFirstScreen(sheetPart);
                    // rust: no-stmt — openFirstScreen 未实现，按 false 走 openAllSheetData。登记 §1.7.1。
                    let openedFirstScreen = false;
                    // java if (!openedFirstScreen)
                    if !openedFirstScreen {
                        // java this.openAllSheetData(sheetPart);
                        self.openAllSheetData(Some(&sheetPart))?;
                    }
                // java else
                } else {
                    // java this.openAllSheetData(sheetPart);
                    self.openAllSheetData(Some(&sheetPart))?;
                }
                // java ISlimToolManager.instance().getSlimTool(33).checkSlim(this.mSheet);
                // rust: no-stmt
                let _ = TAG;
            // java else
            } else {
                // java if (!this.isMIndex2)
                // rust: no-stmt — 块读取留给大表。登记 §1.7.1。
                // java this.mBlocksManager.mIndex2();
                // rust: no-stmt
                // java this.isMIndex2 = true;
                // rust: no-stmt
                // java if (!this.isFinishRows)
                // rust: no-stmt
                // java if (this.mBackupNotifyBeginRow != -1 && this.mBackupNotifyEndRow != -1)
                // rust: no-stmt
                // java this.mBlocksManager.readGridRange(this.mBackupNotifyBeginRow, this.mBackupNotifyEndRow);
                // rust: no-stmt
                // java this.mBackupNotifyBeginRow = -1;
                // rust: no-stmt
                // java this.mBackupNotifyEndRow = -1;
                // rust: no-stmt
                // java this.sheetUIUpdate();
                // rust: no-stmt
                // java if (!this.mBlocksManager.readNextBlock())
                // rust: no-stmt
                // java this.isFinishRows = true;
                // rust: no-stmt
                // java this.mBookReader.getBookOpenListener().sheetDidFinishReadingEverything(this.mSheetIndex);
                // rust: no-stmt
                // java ISlimToolManager.instance().getSlimTool(33).checkSlim(this.mSheet);
                // rust: no-stmt
            }
        }
        Ok(())
    }

    // java public boolean hasFinish()
    pub fn hasFinish(&self) -> bool {
        // java return this.isMIndex && this.isMIndex2 && this.isFinishRows;
        self.isMIndex.get() && self.isMIndex2.get() && self.isFinishRows.get()
    }

    // java public boolean hasMIndex()
    pub fn hasMIndex(&self) -> bool {
        // java return this.isMIndex && this.isMIndex2;
        self.isMIndex.get() && self.isMIndex2.get()
    }

    // java public void setActiveSheet(boolean bActive)
    pub fn setActiveSheet(&self, bActive: bool) {
        // java this.bActiveSheet = bActive;
        self.bActiveSheet.set(bActive);
    }

    // java public boolean getActiveSheet()
    pub fn getActiveSheet(&self) -> bool {
        // java return this.bActiveSheet;
        self.bActiveSheet.get()
    }

    // java private void checkSheetHasDrawingXml(OpcrPackagePart sheetPart) throws IOException
    pub fn checkSheetHasDrawingXml(&self, sheetPart: &mut OpcrPackagePart) -> io::Result<()> {
        // java OpcrRelationships sheetPartRels = sheetPart.getRelationships();
        let sheetPartRels = sheetPart.getRelationships()?;
        // java if (null != sheetPartRels)
        if let Some(sheetPartRels) = sheetPartRels {
            // java OpcrRelationship drawingRel = sheetPartRels.getRelationshipByType(OxfXlRelations.DRAWINGS.getRelsType());
            let drawingRel = sheetPartRels.getRelationshipByType(DRAWINGS_RELS);
            // java if (null != drawingRel)
            if drawingRel.is_some() {
                // java this.mSheet.setHasDrawing(true);
                self.mSheet.borrow().setHasDrawing(true);
            }
        }
        Ok(())
    }

    // java private boolean isLargeSheet(OpcrPackagePart part) throws IOException
    pub fn isLargeSheet(&self, part: Option<&OpcrPackagePart>) -> bool {
        // java if (part == null)
        if part.is_none() {
            // java return false;
            return false;
        }
        // java else
        let part = part.unwrap();
        // java InputStream stream = null;
        let mut stream = part.getInputStream_default();
        // java try
        // java stream = part.getInputStream();
        // java catch (Throwable e)
        // java IOLog.e(TAG, "Throwable", e);
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(_) => {
                let _ = TAG;
                return false;
            }
        };
        // java if (stream == null)
        // rust: no-stmt — 空流按 0 字节，不大于 SMALL_SHEET。
        // java return stream.available() > 65536;
        let mut buf = Vec::new();
        if stream.read_to_end(&mut buf).is_err() {
            return false;
        }
        let _ = SMALL_SHEET;
        buf.len() > SMALL_SHEET
    }

    // java private void openAllSheetData(OpcrPackagePart part) throws IOException
    pub fn openAllSheetData(&self, part: Option<&OpcrPackagePart>) -> io::Result<()> {
        // java this.isFirstScreen = false;
        self.isFirstScreen.set(false);
        // java if (part != null)
        if let Some(part) = part {
            // java InputStream stream = part.getInputStream();
            let mut stream = part.getInputStream_default()?;
            // java if (stream != null)
            // rust: no-stmt — 失败已返回。空字节仍交给 readSheetXml。
            // java this.mSheetHandler = new WorksheetHandler(this);
            let handler = WorksheetHandler::WorksheetHandler(
                Rc::clone(&self.mSheet),
                self.mStylesTable.clone(),
            );
            // java this.mSheetHandler = new WorksheetHandler(this);
            handler.setActiveSheet(self.bActiveSheet.get());
            *self.mSheetHandler.borrow_mut() = Some(handler.clone());
            // java UnifiedXmlReader.readSheetXml((InputStream)stream, this.mSheetHandler);
            UnifiedXmlReader::readSheetXml(&mut *stream, Box::new(handler));
        }
        // java this.isMIndex = true;
        self.isMIndex.set(true);
        // java this.isMIndex2 = true;
        self.isMIndex2.set(true);
        // java this.isFinishRows = true;
        self.isFinishRows.set(true);
        // java if (this.bActiveSheet)
        if self.bActiveSheet.get() {
            // java int valueIdx = this.mSheetHandler != null ? this.mSheetHandler.getMaxValueIndex() : 0;
            let valueIdx=self.mSheetHandler.borrow().as_ref().map_or(0,|h|h.getMaxValueIndex());
            // java this.mBookReader.onFinishActiveSheet(valueIdx);
            self.mBookReader.upgrade().expect("NullPointerException").onFinishActiveSheet(valueIdx)?;
            // java if (this.bHasConditionalFormatting || this.bHasDataValidations)
            // rust: no-stmt
            // java UnifiedXmlReader.readSheetXml((InputStream)part.getInputStream(), new WorksheetDelayHandler(this));
            // rust: no-stmt
            let has_cf=self.mSheetHandler.borrow().as_ref().is_some_and(|h|h.hasConditionalFormatting());self.bHasConditionalFormatting.set(has_cf);
            if has_cf {if let(Some(part),Some(styles))=(part,self.mStylesTable.as_ref()){let mut stream=part.getInputStream_default()?;UnifiedXmlReader::readSheetXml(&mut stream,Box::new(crate::cn::wps::moss::xlsxr::handler::sheet::WorksheetDelayHandler::WorksheetDelayHandler::new(self.mSheet.clone(),styles.clone())));}}
        }
        // java this.mSheet.changeListenerGroup().sheetDidFinishInitializing();
        self.mSheet.borrow().changeListenerGroup().sheetDidFinishInitializing();
        // java this.mBookReader.getBookOpenListener().sheetDidFinishReadingEverything(this.mSheetIndex);
        self.mBookReader.upgrade().expect("NullPointerException").getBookOpenListener().expect("NullPointerException").sheetDidFinishReadingEverything(self.mSheetIndex);
        Ok(())
    }

    // java public KmoSheet getSheet()
    pub fn getSheet(&self) -> Rc<RefCell<KmoSheet>> {
        // java return this.mSheet;
        Rc::clone(&self.mSheet)
    }

    // java public StylesTable getStylesTable()
    pub fn getStylesTable(&self) -> Option<Rc<RefCell<StylesTable>>> {
        // java return this.mStylesTable;
        self.mStylesTable.clone()
    }
}
impl XlsxrWorkSheet {
    // java public boolean notifyGridRange(int beginRow, int endRow)
    fn notifyGridRange(&mut self, beginRow: i32, endRow: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mBlocksManager != null && !this.mBlocksManager.hasUnReadGridRange(beginRow, endRow))
      if let Some(mBlocksManager) = self.mBlocksManager.as_mut() {
        if !self.mBlocksManager.hasUnReadGridRange(beginRow, endRow) {
          // java return false;
          return false;
        }
      }
      // java else
      else {
        // java this.mBackupNotifyBeginRow = beginRow;
        self.mBackupNotifyBeginRow = beginRow;
        // java this.mBackupNotifyEndRow = endRow;
        self.mBackupNotifyEndRow = endRow;
        // java return true;
        return true;
      }
      */
    }

    // java public boolean readGridRange(int beginRow, int endRow)
    fn readGridRange(&mut self, beginRow: i32, endRow: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.hasMIndex() && this.mBlocksManager != null)
      if let Some(mBlocksManager) = self.mBlocksManager.as_mut() {
        if self.hasMIndex() {
          // java return this.mBlocksManager.readGridRange(beginRow, endRow);
          return mBlocksManager.readGridRange(beginRow, endRow);
        }
      }
      // java else
      else {
        // java this.mBackupNotifyBeginRow = beginRow;
        self.mBackupNotifyBeginRow = beginRow;
        // java this.mBackupNotifyEndRow = endRow;
        self.mBackupNotifyEndRow = endRow;
        // java return false;
        return false;
      }
      */
    }
    // java public void restore()
    fn restore(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSheet.getGrid().getMatrix().clearAllRowsMemory();
      self.mSheet.getGrid().getMatrix().clearAllRowsMemory();
      // java if (this.mBlocksManager != null)
      if let Some(mBlocksManager) = self.mBlocksManager.as_mut() {
        // java this.mBlocksManager.restore();
        mBlocksManager.restore();
      }
      // java else
      else {
        // java this.isMIndex = false;
        self.isMIndex = false;
        // java this.isMIndex2 = false;
        self.isMIndex2 = false;
        // java this.isFinishRows = false;
        self.isFinishRows = false;
      }
      */
    }
    // java private boolean openFirstScreen(OpcrPackagePart part)
    pub fn openFirstScreen(&mut self, part: &OpcrPackagePart) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.isFirstScreen = true;
      self.isFirstScreen = true;
      // java InputStream stream = part.getInputStream();
      let mut stream: Box<dyn std::io::Read> = part.getInputStream();
      // java String sheetCachePath = XlsxrFileCache.dumpTempFile(stream, new File(this.mSheet.getBook().getTmpPath()));
      let mut sheetCachePath: String = XlsxrFileCache::dumpTempFile__inputstream_file(Some(&stream), &File::new(self.mSheet.getBook().getTmpPath()));
      // java XlsxrSheetTraversal sheetTraversal = new XlsxrSheetTraversal(sheetCachePath);
      let mut sheetTraversal: XlsxrSheetTraversal = XlsxrSheetTraversal::new(sheetCachePath);
      // java try
          // java sheetTraversal.traversal();
          sheetTraversal.traversal();
        // java catch (Exception var7)
          // java this.openAllSheetData(part);
          self.openAllSheetData(Some(&part));
          // java return true;
          return true;
      // java this.mSheetHandler = new WorksheetHandler(this);
      self.mSheetHandler = WorksheetHandler::new(self);
      // java UnifiedXmlReader.readSheetXml(sheetCachePath, 0, sheetTraversal.getBeginSheetPos(), this.mSheetHandler, true);
      UnifiedXmlReader::readSheetXml__string_int_int_elementhandler_boolean(&sheetCachePath, 0, sheetTraversal.getBeginSheetPos(), &self.mSheetHandler, true);
      // java UnifiedXmlReader.readSheetXml(sheetCachePath, sheetTraversal.getEndSheetPos(), sheetTraversal.getFileSize(), this.mSheetHandler, false);
      UnifiedXmlReader::readSheetXml__string_int_int_elementhandler_boolean(&sheetCachePath, sheetTraversal.getEndSheetPos(), sheetTraversal.getFileSize(), &self.mSheetHandler, false);
      // java int topRow = this.mSheet.getWindow2().getTopRow();
      let mut topRow: i32 = self.mSheet.getWindow2().getTopRow();
      // java this.mBlocksManager = new XlsxrRowsBlockManager(sheetCachePath, this.mSheetHandler.getSheetDataHandler());
      self.mBlocksManager = XlsxrRowsBlockManager::new(sheetCachePath, self.mSheetHandler.getSheetDataHandler());
      // java this.mBlocksManager.mIndex(sheetTraversal.getBeginSheetPos(), sheetTraversal.getEndSheetPos(), topRow);
      self.mBlocksManager.mIndex(sheetTraversal.getBeginSheetPos(), sheetTraversal.getEndSheetPos(), topRow);
      // java boolean isReadFirstScreen = this.mBlocksManager.readGridRange(topRow, topRow + 40);
      let mut isReadFirstScreen: bool = self.mBlocksManager.readGridRange(topRow, topRow + 40);
      // java if (!isReadFirstScreen)
      if !isReadFirstScreen {
        // java return false;
        return false;
      }
      // java else
      else {
        // java if (this.bActiveSheet)
        if self.bActiveSheet {
          // java this.mBookReader.onFinishActiveSheet(this.mSheetHandler.getMaxValueIndex());
          self.mBookReader.onFinishActiveSheet(self.mSheetHandler.getMaxValueIndex());
          // java if (this.bHasConditionalFormatting || this.bHasDataValidations)
          if self.bHasConditionalFormatting || self.bHasDataValidations {
            // java UnifiedXmlReader.readSheetXml(sheetCachePath, sheetTraversal.getEndSheetPos(), sheetTraversal.getFileSize(), new WorksheetDelayHandler(this), false);
            UnifiedXmlReader::readSheetXml__string_int_int_elementhandler_boolean(&sheetCachePath, sheetTraversal.getEndSheetPos(), sheetTraversal.getFileSize(), &WorksheetDelayHandler::new(self), false);
          }
        }
        // java this.mSheet.changeListenerGroup().sheetDidFinishInitializing();
        self.mSheet.changeListenerGroup().sheetDidFinishInitializing();
        // java this.isMIndex = true;
        self.isMIndex = true;
        // java return true;
        return true;
      }
      */
    }

    // java public void openDrawingPart()
    pub fn openDrawingPart(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OpcrPackagePart part = this.mSheetRels.getPackagePart();
      let mut part: OpcrPackagePart = self.mSheetRels.getPackagePart();
      // java if (part != null)
      if part.is_some() {
        // java if (part.getRelationships() != null)
        if part.getRelationships().is_some() {
          // java OpcrRelationship relationship = part.getRelationships().getRelationshipByType(OxfXlRelations.DRAWINGS.getRelsType());
          let mut relationship: OpcrRelationship = part.getRelationships().getRelationshipByType(OxfXlRelations::DRAWINGS.getRelsType());
          // java if (relationship != null)
          if relationship.is_some() {
            // java XlsxrDrawing drawReader = new XlsxrDrawing(this.mSheet, relationship);
            let mut drawReader: XlsxrDrawing = XlsxrDrawing::new(self.mSheet, relationship);
            // java drawReader.open();
            drawReader.open();
          }
          // java this.openCommentPart(part);
          self.openCommentPart(&part);
          // java this.openVMLPart(part);
          self.openVMLPart(&part);
        }
      }
      */
    }

    // java private void openCommentPart(OpcrPackagePart sheetPart)
    pub fn openCommentPart(&self, sheetPart: &OpcrPackagePart) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OpcrRelationship relationship = sheetPart.getRelationships().getRelationshipByType(OxfXlRelations.SHEET_COMMENTS.getRelsType());
      let mut relationship: OpcrRelationship = sheetPart.getRelationships().getRelationshipByType(OxfXlRelations::SHEET_COMMENTS.getRelsType());
      // java if (relationship != null)
      if relationship.is_some() {
        // java XlsxrComments commentRead = new XlsxrComments(this.mSheet.getSheetDrawingAgg(), relationship);
        let mut commentRead: XlsxrComments = XlsxrComments::new(self.mSheet.getSheetDrawingAgg(), relationship);
        // java commentRead.open();
        commentRead.open();
      }
      */
    }

    // java public void addOleObject(OleObject oleObject)
    pub fn addOleObject(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mOleObjects.add(oleObject);
      self.mOleObjects.add(oleObject);
      */
    }

    // java public void importOle()
    pub fn importOle(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.importOleObjects();
      self.importOleObjects();
      */
    }

    // java public void importOleObjects()
    pub fn importOleObjects(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mOleObjects != null && this.mOleObjects.size() >= 1)
      if let Some(mOleObjects) = self.mOleObjects.as_ref() {
        if (self.mOleObjects.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) >= 1 {
          let mut oleIndex: i32 = 0;
          // java for(int oleIndex = 0; oleIndex < this.mOleObjects.size(); ++oleIndex)
          while oleIndex < (mOleObjects.len() as i32) {
            // java OleObject oleObject = (OleObject)this.mOleObjects.get(oleIndex);
            let mut oleObject: OleObject = (mOleObjects.get(oleIndex)).as_any().downcast_ref::<OleObject>().expect("Java cast to OleObject failed");
            // java KmoShape shape = this.getShapeByName("_x0000_s" + oleObject.getShapeId());
            let mut shape: KmoShape = self.getShapeByName(&"_x0000_s" + oleObject.getShapeId());
            // java if (null != shape)
            if shape.is_some() {
              // java if (oleObject.getAnchor() != null)
              if oleObject.getAnchor().is_some() {
                // java shape.setAnchor(oleObject.getAnchor());
                shape.setAnchor(oleObject.getAnchor());
              }
              // java OpcrPackagePart part = this.mSheetRels.getPackagePart();
              let mut part: OpcrPackagePart = self.mSheetRels.getPackagePart();
              // java OpcrRelationship relationship = null;
              let mut relationship: OpcrRelationship = OpcrRelationship::default();
              // java try
                  // java relationship = part.getRelationships().getRelationship(oleObject.getwId());
                  relationship = part.getRelationships().getRelationship(oleObject.getwId());
                // java catch (IOException e)
                  // java IOLog.d(TAG, "importOleObjects :", e);
                  IOLog::d__string_string_throwable(&TAG, &"importOleObjects :", &e);
              // java if (null != relationship && !StringUtil.isEmpty(relationship.getTarget()))
              if relationship.is_some() && !StringUtil::isEmpty(Some(&mut relationship.getTarget())) {
                // java String target = relationship.getTarget();
                let mut target: String = relationship.getTarget();
                // java target = target.replace("..", "xl");
                target = target.replace("..", "xl");
                // java File tempFile = OleHelper.createTmpOleFile();
                let mut tempFile: std::path::PathBuf = OleHelper::createTmpOleFile();
                // java String binPath = tempFile.getAbsolutePath();
                let mut binPath: String = tempFile.getAbsolutePath();
                // java int mediaId = this.mSheet.getBook().getOleMediaLib().addMedia(binPath, binPath, MediaTypeEnum.OLE);
                let mut mediaId: i32 = self.mSheet.getBook().getOleMediaLib().addMedia(binPath, binPath, MediaTypeEnum::OLE);
                // java OLE ole = new OLE(mediaId);
                let mut ole: OLE = OLE::new(mediaId);
                // java String progId = OleClsTypeMatcher.convertProgId(oleObject.getProgId(), target);
                let mut progId: String = OleClsTypeMatcher::convertProgId(&oleObject.getProgId(), Some(&mut target));
                // java ole.setProgId(progId);
                ole.setProgId(progId);
                // java ole.setIsIcon(oleObject.isIcon());
                ole.setIsIcon(oleObject.isIcon());
                // java shape.setOLE(ole);
                shape.setOLE(ole);
                // java shape.getGRF().setOleShape(true);
                shape.getGRF().setOleShape(true);
                // java shape.setFill((FillBase)null);
                shape.setFill(None);
                // java shape.setLineProperty((LineProperty)null);
                shape.setLineProperty(None);
                // java if (null == shape.getShapeName())
                if shape.getShapeName().is_none() {
                  // java shape.setShapeName("_x0000_s" + shape.getShapeID());
                  shape.setShapeName("_x0000_s" + shape.getShapeID());
                }
                // java this.mSheet.getBook().addOle(mediaId, new KmoOle(target, KmoOle.Type.XLSX));
                self.mSheet.getBook().addOle(mediaId, KmoOle::new(target, KmoOle::Type.XLSX));
              }
            }
          }
        }
      }
      */
    }

    // java private KmoShape getShapeByName(String shapeName)
    pub fn getShapeByName(&self, shapeName: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java List<KmoShape> shapeList = this.mSheet.getSheetDrawingAgg().getShapes();
      let mut shapeList: Vec<KmoShape> = self.mSheet.getSheetDrawingAgg().getShapes();
      // java if (!StringUtil.isEmpty(shapeName) && shapeList != null && shapeList.size() >= 1)
      if !StringUtil::isEmpty(Some(&mut shapeName)) && shapeList.is_some() && (shapeList.len() as i32) >= 1 {
        let mut index: i32 = 0;
        // java for(int index = 0; index < shapeList.size(); ++index)
        while index < (shapeList.len() as i32) {
          // java KmoShape shape = (KmoShape)shapeList.get(index);
          let mut shape: KmoShape = (shapeList.get(index)).as_any().downcast_ref::<KmoShape>().expect("Java cast to KmoShape failed");
          // java if (shape.getShapeName() != null && shape.getShapeName().equals(shapeName))
          if shape.getShapeName().is_some() && shape.getShapeName().equals(shapeName) {
            // java return shape;
            return shape;
          }
        }
        // java return null;
        return None;
      }
      // java else
      else {
        // java return null;
        return None;
      }
      */
    }

    // java private void openVMLPart(OpcrPackagePart sheetPart)
    pub fn openVMLPart(&self, sheetPart: &OpcrPackagePart) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OpcrRelationship relationship = sheetPart.getRelationships().getRelationshipByType(OxfXlRelations.VML_DRAWINGS.getRelsType());
      let mut relationship: OpcrRelationship = sheetPart.getRelationships().getRelationshipByType(OxfXlRelations::VML_DRAWINGS.getRelsType());
      // java if (relationship != null)
      if relationship.is_some() {
        // java XlsxrVmlDrawing vmlDrawing = new XlsxrVmlDrawing(this.mSheet.getSheetDrawingAgg(), relationship);
        let mut vmlDrawing: XlsxrVmlDrawing = XlsxrVmlDrawing::new(self.mSheet.getSheetDrawingAgg(), relationship);
        // java vmlDrawing.open();
        vmlDrawing.open();
      }
      */
    }

    // java public void openBackgroundImage()
    pub fn openBackgroundImage(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mBackgroundRid != null && this.mBackgroundRid.length() != 0)
      if let Some(mBackgroundRid) = self.mBackgroundRid.as_mut() {
        if self.mBackgroundRid.length() != 0 {
          // java OpcrPackagePart sheetPart = this.mSheetRels.getPackagePart();
          let mut sheetPart: OpcrPackagePart = self.mSheetRels.getPackagePart();
          // java if (sheetPart != null)
          if sheetPart.is_some() {
            // java OpcrPackagePart part = XlsxrPackage.getChildPartByRid(sheetPart, this.mBackgroundRid);
            let mut part: OpcrPackagePart = XlsxrPackage::getChildPartByRid(Some(&sheetPart), &mBackgroundRid);
            // java if (part != null)
            if part.is_some() {
              // java InputStream stream = part.getInputStream();
              let mut stream: Box<dyn std::io::Read> = part.getInputStream();
              // java if (stream != null)
              if stream.is_some() {
                // java int size = part.getPartSize();
                let mut size: i32 = part.getPartSize();
                // java int off = 0;
                let mut off: i32 = 0;
                // java byte[] buffer;
                let mut buffer: Vec<u8> = vec![];
                // java int l;
                let mut l: i32;
                buffer = vec![Default::default(); (size) as usize];
                // java for(buffer = new byte[size]; stream.available() > 0 && off < size; off += l)
                while stream.available() > 0 && off < size {
                  // java l = stream.read(buffer, off, size - off);
                  l = stream.read(buffer, off, size - off);
                }
                // java this.mSheet.setBackgroundImage2(buffer, this.getTypeByName(part.getPartName()));
                self.mSheet.setBackgroundImage2(&buffer, self.getTypeByName(&mut part.getPartName()));
              }
            }
          }
        }
      }
      */
    }

    // java public void openSparkLines()
    pub fn openSparkLines(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.bHasSparklineGroups)
      if self.bHasSparklineGroups {
        // java OpcrPackagePart sheetPart = this.mSheetRels.getPackagePart();
        let mut sheetPart: OpcrPackagePart = self.mSheetRels.getPackagePart();
        // java InputStream stream = sheetPart.getInputStream();
        let mut stream: Box<dyn std::io::Read> = sheetPart.getInputStream();
        // java String sheetCachePath = XlsxrFileCache.dumpTempFile(stream, new File(this.mSheet.getBook().getTmpPath()));
        let mut sheetCachePath: String = XlsxrFileCache::dumpTempFile__inputstream_file(Some(&stream), &File::new(self.mSheet.getBook().getTmpPath()));
        // java XlsxrSheetTraversal sheetTraversal = new XlsxrSheetTraversal(sheetCachePath);
        let mut sheetTraversal: XlsxrSheetTraversal = XlsxrSheetTraversal::new(sheetCachePath);
        // java UnifiedXmlReader.readSheetXml(sheetCachePath, sheetTraversal.getEndSheetPos(), sheetTraversal.getFileSize(), new SparklineDelayHandler(this), false);
        UnifiedXmlReader::readSheetXml__string_int_int_elementhandler_boolean(&sheetCachePath, sheetTraversal.getEndSheetPos(), sheetTraversal.getFileSize(), &SparklineDelayHandler::new(self), false);
      }
      */
    }

    // java private void sheetUIUpdate()
    pub fn sheetUIUpdate(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSheet.changeListenerGroup().sheetContentsDidFinishChangingNonBlockable();
      self.mSheet.changeListenerGroup().sheetContentsDidFinishChangingNonBlockable();
      */
    }

    // java public long cleanBlockByGridRange(int topRow)
    pub fn cleanBlockByGridRange(&mut self, topRow: i32) -> Option<i64> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.isMIndex && this.mBlocksManager != null ? this.mBlocksManager.cleanBlockByGridRange(topRow) : -1L;
      return (if self.isMIndex && self.mBlocksManager.is_some() { Some(self.mBlocksManager.cleanBlockByGridRange(topRow)) } else { Some(-1) });
      */
    }

    pub fn formulaImportKinds(&self) -> Vec<&'static str> {
        self.mSheetHandler
            .borrow()
            .as_ref()
            .map(|handler| handler.getSheetDataHandler().fmlHandler().import_order())
            .unwrap_or_default()
    }

    // java public void delayedImportFormula()
    pub fn delayedImportFormula(&self) {
      // java if (this.mSheetHandler != null)
      if let Some(mSheetHandler) = self.mSheetHandler.borrow().as_ref() {
        // java this.mSheetHandler.getSheetDataHandler().delayedImportFormula(this.mSheet);
        mSheetHandler
            .getSheetDataHandler()
            .delayedImportFormula(&self.mSheet.borrow());
      }
    }

    // java public void setBackgroundRId(String rid)
    pub fn setBackgroundRId(&mut self, rid: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mBackgroundRid = rid;
      self.mBackgroundRid = rid;
      */
    }

    // java private int getTypeByName(String name)
    pub fn getTypeByName(&self, name: &mut str) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java String suffix = "";
      let mut suffix: String = "".to_string();
      // java int index = -1;
      let mut index: i32 = -1;
      // java if ((index = name.lastIndexOf(".")) != -1)
      if { index = name.lastIndexOf("."); index } != -1 {
        // java suffix = name.substring(index + 1, name.length()).toLowerCase();
        suffix = name.substring(index + 1, (name.len() as i32)).toLowerCase();
      }
      // java return BlipType.getBlipType(suffix);
      return BlipType::getBlipType(&mut suffix);
      */
    }

    // java public void setHasDataValidations(boolean b)
    pub fn setHasDataValidations(&mut self, b: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.bHasDataValidations = b;
      self.bHasDataValidations = b;
      */
    }

    // java public void setHasConditionalFormatting(boolean b)
    pub fn setHasConditionalFormatting(&mut self, b: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.bHasConditionalFormatting = b;
      self.bHasConditionalFormatting = b;
      */
    }

    // java public void setHasSparklineGroups(boolean b)
    pub fn setHasSparklineGroups(&mut self, b: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.bHasSparklineGroups = b;
      self.bHasSparklineGroups = b;
      */
    }

    // java public Map<String, String> getHyperTargetMap()
    pub fn getHyperTargetMap(&self) -> Option<HashMap<String, String>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OpcrPackagePart sheetPart = this.mSheetRels.getPackagePart();
      let mut sheetPart: OpcrPackagePart = self.mSheetRels.getPackagePart();
      // java if (sheetPart == null)
      if sheetPart.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java try
            // java return XlsxrPackage.getHyperTargetMap(sheetPart);
            return XlsxrPackage::getHyperTargetMap(&sheetPart);
          // java catch (IOException e)
            // java throw new RuntimeException(e);
            panic!("{}", RuntimeException::new(e));
      }
      */
    }

    // java public void setTableParts(List<String> tableParts)
    pub fn setTableParts(&mut self, tableParts: &Vec<String>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mTableParts = tableParts;
      self.mTableParts = tableParts;
      */
    }

    // java public void openTables()
    pub fn openTables(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != this.mTableParts && !this.mTableParts.isEmpty())
      if let Some(mTableParts) = self.mTableParts.as_ref() {
        if !self.mTableParts.is_empty() {
          let mut index: i32 = 0;
          // java for(int index = 0; index < this.mTableParts.size(); ++index)
          while index < (mTableParts.len() as i32) {
            // java OpcrPackagePart sheetPart = this.mSheetRels.getPackagePart();
            let mut sheetPart: OpcrPackagePart = self.mSheetRels.getPackagePart();
            // java if (sheetPart == null)
            if sheetPart.is_none() {
              // java return;
              return;
            }
            // java OpcrPackagePart part = null;
            let mut part: OpcrPackagePart = OpcrPackagePart::default();
            // java try
                // java part = XlsxrPackage.getChildPartByRid(sheetPart, (String)this.mTableParts.get(index));
                part = XlsxrPackage::getChildPartByRid(Some(&sheetPart), &mTableParts.get(index).to_string());
                // java if (part == null)
                if part.is_none() {
                  // java return;
                  return;
                }
                // java InputStream stream = part.getInputStream();
                let mut stream: Box<dyn std::io::Read> = part.getInputStream();
                // java TableHandler tableHandler = new TableHandler(this.mSheet, this.getStylesTable());
                let mut tableHandler: TableHandler = TableHandler::new(self.mSheet, self.getStylesTable());
                // java UnifiedXmlReader.read((InputStream)stream, tableHandler);
                UnifiedXmlReader::read__inputstream_elementhandler(&todo!("cast → Box<dyn std::io::Read>"), &tableHandler);
              // java catch (IOException var6)
          }
        }
      }
      */
    }

}
