// Java: import cn.wps.moffice.writer.cache.TypoColumnPage
use crate::cn::wps::moffice::writer::cache::TypoColumnPage::TypoColumnPage;
// Java: import cn.wps.moffice.writer.cache.TypoColumns
use crate::cn::wps::moffice::writer::cache::TypoColumns::TypoColumns;
// Java: import cn.wps.moffice.writer.cache.TypoDrawing
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
// Java: import cn.wps.moffice.writer.cache.TypoDrawingLayers
use crate::cn::wps::moffice::writer::cache::TypoDrawingLayers::TypoDrawingLayers;
// Java: import cn.wps.moffice.writer.cache.TypoFootEndNote
use crate::cn::wps::moffice::writer::cache::TypoFootEndNote::TypoFootEndNote;
// Java: import cn.wps.moffice.writer.cache.TypoFootNotes
use crate::cn::wps::moffice::writer::cache::TypoFootNotes::TypoFootNotes;
// Java: import cn.wps.moffice.writer.cache.TypoHeaderFooterPage
use crate::cn::wps::moffice::writer::cache::TypoHeaderFooterPage::TypoHeaderFooterPage;
// Java: import cn.wps.moffice.writer.cache.TypoLayoutPage
use crate::cn::wps::moffice::writer::cache::TypoLayoutPage::TypoLayoutPage;
// Java: import cn.wps.moffice.writer.cache.TypoList
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
// Java: import cn.wps.moffice.writer.cache.TypoMemoryObj
use crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj;
// Java: import cn.wps.moffice.writer.cache.TypoObjBase
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
// Java: import cn.wps.moffice.writer.cache.TypoPage
use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
// Java: import cn.wps.moffice.writer.cache.TypoRevisions
use crate::cn::wps::moffice::writer::cache::TypoRevisions::TypoRevisions;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.cache.TypoTableRow
use crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow;
// Java: import cn.wps.moffice.writer.cache.TypoTextLine
use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.layout.base.drawing.DrawingCacheTool
use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCacheTool::DrawingCacheTool;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.Pools
use crate::cn::wps::moffice::writer::layout::base::env::Pools::Pools;

// java static final String TAG = null;
pub(crate) const TAG: i32 = 0;

// java public final class CacheCleaner
#[derive(Default)]
pub struct CacheCleaner {
    // java private MainState mMainState;
    mMainState: Option<MainStateHolder>,
    // java private DocumentEnv mDocEnv;
    mDocEnv: Option<DocumentEnvHolder>,
}

// java public final class CacheCleaner
impl CacheCleaner {
    // java public void init(MainState mainState, DocumentEnv docEnv)
    pub fn init(&mut self, mainState: Option<MainStateHolder>, docEnv: Option<DocumentEnvHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMainState = mainState;
      self.mMainState = mainState;
      // java this.mDocEnv = docEnv;
      self.mDocEnv = docEnv;
      */
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMainState = null;
      self.mMainState = None;
      // java this.mDocEnv = null;
      self.mDocEnv = None;
      */
    }

    // java public void cleanAfter(LayoutState env)
    pub fn cleanAfter(&self, env: Option<LayoutStateHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Pools pools = this.mDocEnv.pools;
      let mut pools: Pools = self.mDocEnv.pools;
      // java int docType = env.document.getType();
      let mut docType: i32 = env.document.getType();
      // java int footNotesPage = docType == 0 ? this.getFootNotesPage(env.curPage.getPointer()) : 0;
      let mut footNotesPage: i32 = (if docType == 0 { self.getFootNotesPage(env.curPage.getPointer()) } else { 0 });
      // java int lines = env.curPage.getLines();
      let mut lines: i32 = env.curPage.getLines();
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java TypoRevisions revisions = this.mMainState.revisions;
      let mut revisions: TypoRevisions = self.mMainState.revisions;
      // java int limit = env.linePos;
      let mut limit: i32 = env.linePos;
      // java int needDelWrapTableDrawing = 0;
      let mut needDelWrapTableDrawing: i32 = 0;
      // java if (this.mMainState.footEndNoteCache.size() > 0)
      if (self.mMainState.footEndNoteCache.len() as i32) > 0 {
        // java int lastLine = TypoList.last(lines, snapshot);
        let mut lastLine: i32 = TypoList::last__int_typosnapshot(lines, snapshot);
        // java if (TypoMemoryObj.getType(lastLine, snapshot) == 5 && TypoTableRow.isWrap(lastLine, snapshot))
        if TypoMemoryObj::getType__int_typosnapshot(lastLine, snapshot) == 5 && TypoTableRow::isWrap__int_typosnapshot(lastLine, snapshot) {
          // java TypoLayoutPage layoutPage = this.mMainState.layoutPage;
          let mut layoutPage: TypoLayoutPage = self.mMainState.layoutPage;
          // java int wrapDrawing = TypoDrawingLayers.getWrapTableByCP(TypoTableRow.getStartCP(lastLine, snapshot), layoutPage.getPointer(), layoutPage.getDrawingLayers(), snapshot);
          let mut wrapDrawing: i32 = TypoDrawingLayers::getWrapTableByCP__int_int_int_typosnapshot(TypoTableRow::getStartCP(lastLine, snapshot), layoutPage.getPointer(), layoutPage.getDrawingLayers(), snapshot);
          // java if (wrapDrawing != 0 && TypoDrawing.isNoKeepCacheWhenRollBack(wrapDrawing, snapshot))
          if wrapDrawing != 0 && TypoDrawing::isNoKeepCacheWhenRollBack(wrapDrawing, snapshot) {
            // java needDelWrapTableDrawing = wrapDrawing;
            needDelWrapTableDrawing = wrapDrawing;
          }
        }
      }
      let mut delIndex: i32 = TypoList::size__int_typosnapshot(lines, snapshot) - 1;
      // java for(int delIndex = TypoList.size(lines, snapshot) - 1; delIndex >= limit --delIndex)
      while delIndex >= limit {
        // java int delObj = TypoList.quickGet(delIndex, lines, snapshot);
        let mut delObj: i32 = TypoList::quickGet__int_int_typosnapshot(delIndex, lines, snapshot);
        // java switch (TypoObjBase.getType(delObj, snapshot))
        match TypoObjBase::getType(delObj, snapshot) {
          // java case 3:
          3 => {
            // java this.recycleFootNote(TypoTextLine.getStartCP(delObj, snapshot), TypoTextLine.getEndCP(delObj, snapshot), footNotesPage);
            self.recycleFootNote(TypoTextLine::getStartCP__int_typosnapshot(delObj, snapshot), TypoTextLine::getEndCP__int_typosnapshot(delObj, snapshot), footNotesPage);
          }
          // java case 5:
          5 => {
            // java this.recycleRow(delObj, this.mMainState.getDrawingLayers(env.document));
            self.recycleRow__int_typodrawinglayers(delObj, Some(self.mMainState.getDrawingLayers__document(env.document)));
          }
          // java case 11:
          11 => {
            // java this.recyclePageContent(delObj);
            self.recyclePageContent(delObj);
          }
          // java case 13:
          13 => {
            // java this.recycleColumns(delObj);
            self.recycleColumns(delObj);
          }
          // java default:
          _ => {
          }
        }
        // java if (0 != footNotesPage)
        if 0 != footNotesPage {
          // java footNotesPage = this.getFootNotesPage(env.curPage.getPointer());
          footNotesPage = self.getFootNotesPage(env.curPage.getPointer());
        }
      }
      // java TypoList.setSize(env.linePos, lines, snapshot);
      TypoList::setSize(env.linePos, lines, snapshot);
      // java this.clearRedundantEmbeds(env);
      self.clearRedundantEmbeds(env);
      // java Document document = env.document;
      let mut document: Box<dyn Document> = env.document;
      // java if (null != document)
      if true {
        // java TypoDrawingLayers drawingLayers = this.mMainState.getDrawingLayers(env.document);
        let mut drawingLayers: TypoDrawingLayers = self.mMainState.getDrawingLayers__document(env.document);
        // java if (null != drawingLayers)
        if true {
          // java boolean isInColumns = 14 == env.curPage.getType();
          let mut isInColumns: bool = 14 == env.curPage.getType();
          // java int floatsPage = isInColumns ? this.mMainState.layoutPage.getPointer() : env.curPage.getPointer();
          let mut floatsPage: i32 = (if isInColumns { self.mMainState.layoutPage.getPointer() } else { env.curPage.getPointer() });
          // java int drawings = drawingLayers.getDrawins();
          let mut drawings: i32 = drawingLayers.getDrawins();
          // java int drawingSize = TypoList.size(drawings, snapshot);
          let mut drawingSize: i32 = TypoList::size__int_typosnapshot(drawings, snapshot);
          // java int index = 0;
          let mut index: i32 = 0;
          // java while (index < drawingSize)
          while index < drawingSize {
            // java int drawing = TypoList.get(index, drawings, snapshot);
            let mut drawing: i32 = TypoList::get__int_int_typosnapshot(index, drawings, snapshot);
            // java if (TypoDrawing.getParent(drawing, snapshot) != floatsPage)
            if TypoDrawing::getParent(drawing, snapshot) != floatsPage {
              // java ++index;
              { let _tmp = index; index += 1; _tmp };
            }
            // java else
            else {
              // java int drawingCP = TypoDrawing.getCP(drawing, snapshot);
              let mut drawingCP: i32 = TypoDrawing::getCP__int_typosnapshot(drawing, snapshot);
              // java if (drawingCP < env.cp)
              if drawingCP < env.cp {
                // java ++index;
                { let _tmp = index; index += 1; _tmp };
              }
              // java else if (TypoDrawing.isWrap(drawing, snapshot) && needDelWrapTableDrawing != drawing)
              else if TypoDrawing::isWrap__int_typosnapshot(drawing, snapshot) && needDelWrapTableDrawing != drawing {
                // java ++index;
                { let _tmp = index; index += 1; _tmp };
              }
              // java else
              else {
                // java drawingLayers.removeDrawing(drawing);
                drawingLayers.removeDrawing(drawing);
                // java DrawingCacheTool.addToSortedCacheItems(this.mMainState.floatsCache, pools, this, document, drawing, drawingLayers, this.mMainState.snapshot);
                DrawingCacheTool::addToSortedCacheItems__klistdrawingcacheitem_pools_cachecleaner_document_int_typodrawinglayers_typosnapshot(self.mMainState.floatsCache, pools, self, document, drawing, drawingLayers, self.mMainState.snapshot);
                // java --drawingSize;
                { let _tmp = drawingSize; drawingSize -= 1; _tmp };
              }
            }
          }
        }
        // java if (docType == 0)
        if docType == 0 {
          // java this.adjustFootNotesPage(this.getFootNotesPage(env.curPage.getPointer()));
          self.adjustFootNotesPage(self.getFootNotesPage(env.curPage.getPointer()));
        }
      }
      */
    }

    // java protected void clearRedundantEmbeds(LayoutState layoutState)
    pub fn clearRedundantEmbeds(&self, layoutState: Option<LayoutStateHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (layoutState.hasEmbed() && layoutState.typoPara.getEmbedSize() > 0)
      if layoutState.hasEmbed() && layoutState.typoPara.getEmbedSize() > 0 {
        // java layoutState.typoPara.clearRedundantEmbeds(layoutState.cp);
        layoutState.typoPara.clearRedundantEmbeds(layoutState.cp);
      }
      */
    }

    // java private int getFootNotesPage(int curPage)
    pub fn getFootNotesPage(&self, curPage: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      let mut pageType: i32 = TypoPage::getType(curPage, snapshot);
      // java for(int pageType = TypoPage.getType(curPage, snapshot); pageType != 2 && pageType != 14 pageType = TypoPage.getType(curPage, snapshot))
      while pageType != 2 && pageType != 14 {
        // java curPage = TypoPage.getPage(curPage, snapshot);
        curPage = TypoPage::getPage(curPage, snapshot);
      }
      // java return 0 != curPage ? TypoColumnPage.getFootnotes(curPage, snapshot) : 0;
      return (if 0 != curPage { TypoColumnPage::getFootnotes__int_typosnapshot(curPage, snapshot) } else { 0 });
      */
    }

    // java private void adjustFootNotesPage(int footNotes)
    pub fn adjustFootNotesPage(&self, footNotes: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (0 != footNotes)
      if 0 != footNotes {
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java int parentPage = TypoFootNotes.getParent(footNotes, snapshot);
        let mut parentPage: i32 = TypoFootNotes::getParent(footNotes, snapshot);
        // java int footNotesSize = TypoFootNotes.getSize(footNotes, snapshot);
        let mut footNotesSize: i32 = TypoFootNotes::getSize__int_typosnapshot(footNotes, snapshot);
        // java if (footNotesSize > 1)
        if footNotesSize > 1 {
          // java int curTop = 0;
          let mut curTop: i32 = 0;
          let mut i: i32 = 0;
          // java for(int i = 0; i < footNotesSize; ++i)
          while i < footNotesSize {
            // java int curLine = TypoFootNotes.getFootNote(i, footNotes, snapshot);
            let mut curLine: i32 = TypoFootNotes::getFootNote__int_int_typosnapshot(i, footNotes, snapshot);
            // java TypoObjBase.offsetYTo(curTop, curLine, snapshot);
            TypoObjBase::offsetYTo__int_int_typosnapshot(curTop, curLine, snapshot);
            // java curTop = TypoObjBase.getBottom(curLine, snapshot);
            curTop = TypoObjBase::getBottom__int_typosnapshot(curLine, snapshot);
          }
          // java TypoFootNotes.setHeight(curTop, footNotes, snapshot);
          TypoFootNotes::setHeight(curTop, footNotes, snapshot);
          // java int textFlow = TypoPage.getTextFlow(parentPage, snapshot);
          let mut textFlow: i32 = TypoPage::getTextFlow__int_typosnapshot(parentPage, snapshot);
          // java int maxHeight = 0 == textFlow ? TypoPage.getCoreHeight(parentPage, snapshot) : TypoPage.width(parentPage, snapshot) - TypoPage.getMarginTop(parentPage, snapshot) - TypoPage.getMarginBottom(parentPage, snapshot);
          let mut maxHeight: i32 = (if 0 == textFlow { TypoPage::getCoreHeight__int_typosnapshot(parentPage, snapshot) } else { TypoPage::width(parentPage, snapshot) - TypoPage::getMarginTop__int_typosnapshot(parentPage, snapshot) - TypoPage::getMarginBottom__int_typosnapshot(parentPage, snapshot) });
          // java if (TypoPage.getType(parentPage, snapshot) == 2 && 0 == textFlow)
          if TypoPage::getType(parentPage, snapshot) == 2 && 0 == textFlow {
            // java int footerPage = TypoLayoutPage.getFooterPage(parentPage, snapshot);
            let mut footerPage: i32 = TypoLayoutPage::getFooterPage__int_typosnapshot(parentPage, snapshot);
            // java if (0 != footerPage)
            if 0 != footerPage {
              // java maxHeight = Math.min(maxHeight, TypoHeaderFooterPage.getTop(footerPage, snapshot) - TypoLayoutPage.getMarginTop(parentPage, snapshot));
              maxHeight = (maxHeight).min(TypoHeaderFooterPage::getTop(footerPage, snapshot) - TypoLayoutPage::getMarginTop(parentPage, snapshot));
            }
          }
          // java TypoFootNotes.offsetYTo(maxHeight - curTop, footNotes, snapshot);
          TypoFootNotes::offsetYTo(maxHeight - curTop, footNotes, snapshot);
        }
        // java else
        else {
          // java TypoColumnPage.setFootnotes(parentPage, 0, snapshot);
          TypoColumnPage::setFootnotes__int_int_typosnapshot(parentPage, 0, snapshot);
        }
      }
      */
    }

    // java public void recycleLine(TypoTextLine line, LayoutState env)
    pub fn recycleLine(&mut self, line: Option<TypoTextLineHolder>, env: Option<LayoutStateHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int footNotesPage = env.document.getType() == 0 ? this.getFootNotesPage(env.curPage.getPointer()) : 0;
      let mut footNotesPage: i32 = (if env.document.getType() == 0 { self.getFootNotesPage(env.curPage.getPointer()) } else { 0 });
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java int pLine = line.getPointer();
      let mut pLine: i32 = line.getPointer();
      // java Pools pools = this.mDocEnv.pools;
      let mut pools: Pools = self.mDocEnv.pools;
      // java this.recycleFootNote(line.getStartCP(), line.getEndCP(), footNotesPage);
      self.recycleFootNote(line.getStartCP(), line.getEndCP(), footNotesPage);
      */
    }

    // java public void recycleRow(int row)
    pub fn recycleRow__int(&mut self, row: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.recycleRow(row, (TypoDrawingLayers)null);
      self.recycleRow__int_typodrawinglayers(row, None);
      */
    }

    // java public void recycleRow(int row, TypoDrawingLayers needRecycleDrawingsInLayers)
    pub fn recycleRow__int_typodrawinglayers(&mut self, row: i32, needRecycleDrawingsInLayers: Option<&mut TypoDrawingLayers>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java DrawingCacheTool.saveQuasiFloats(row, snapshot);
      DrawingCacheTool::saveQuasiFloats(row, snapshot);
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = TypoTableRow::getCellsCount__int_typosnapshot(row, snapshot);
      // java for(int size = TypoTableRow.getCellsCount(row, snapshot); i < size; ++i)
      while i < size {
        // java int cell = TypoTableRow.getCell(i, row, snapshot);
        let mut cell: i32 = TypoTableRow::getCell__int_int_typosnapshot(i, row, snapshot);
        // java if (needRecycleDrawingsInLayers != null)
        if let Some(needRecycleDrawingsInLayers) = needRecycleDrawingsInLayers {
          // java needRecycleDrawingsInLayers.removeDrawinghsForParent(cell);
          needRecycleDrawingsInLayers.removeDrawinghsForParent(cell);
        }
        // java this.recyclePageContent(cell);
        self.recyclePageContent(cell);
      }
      */
    }

    // java public void recyclePageContent(int page)
    pub fn recyclePageContent(&mut self, page: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (0 != page)
      if 0 != page {
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java int lines = TypoPage.getLines(page, snapshot);
        let mut lines: i32 = TypoPage::getLines__int_typosnapshot(page, snapshot);
        // java int docType = TypoPage.getDocumentType(page, snapshot);
        let mut docType: i32 = TypoPage::getDocumentType__int_typosnapshot(page, snapshot);
        // java int footNotesPage = docType == 0 ? this.getFootNotesPage(page) : 0;
        let mut footNotesPage: i32 = (if docType == 0 { self.getFootNotesPage(page) } else { 0 });
        let mut i: i32 = TypoList::size__int_typosnapshot(lines, snapshot) - 1;
        // java for(int i = TypoList.size(lines, snapshot) - 1; i >= 0 --i)
        while i >= 0 {
          // java int line = TypoList.quickGet(i, lines, snapshot);
          let mut line: i32 = TypoList::quickGet__int_int_typosnapshot(i, lines, snapshot);
          // java switch (TypoMemoryObj.getType(line, snapshot))
          match TypoMemoryObj::getType__int_typosnapshot(line, snapshot) {
            // java case 3:
            3 => {
              // java this.recycleFootNote(TypoTextLine.getStartCP(line, snapshot), TypoTextLine.getEndCP(line, snapshot), footNotesPage);
              self.recycleFootNote(TypoTextLine::getStartCP__int_typosnapshot(line, snapshot), TypoTextLine::getEndCP__int_typosnapshot(line, snapshot), footNotesPage);
            }
            // java case 5:
            5 => {
              // java this.recycleRow(line);
              self.recycleRow__int(line);
            }
            // java case 11:
            11 => {
              // java this.recyclePageContent(line);
              self.recyclePageContent(line);
            }
            // java case 13:
            13 => {
              // java this.recycleColumns(line);
              self.recycleColumns(line);
            }
            // java default:
            _ => {
            }
          }
          // java if (0 != footNotesPage)
          if 0 != footNotesPage {
            // java footNotesPage = this.getFootNotesPage(page);
            footNotesPage = self.getFootNotesPage(page);
          }
        }
        // java if (docType == 0)
        if docType == 0 {
          // java this.adjustFootNotesPage(this.getFootNotesPage(page));
          self.adjustFootNotesPage(self.getFootNotesPage(page));
        }
        // java if (lines != 0)
        if lines != 0 {
          // java TypoList.setSize(0, lines, snapshot);
          TypoList::setSize(0, lines, snapshot);
        }
      }
      */
    }

    // java private void recycleFootNote(int startCP, int endCP, int footNotesPage)
    pub fn recycleFootNote(&self, startCP: i32, endCP: i32, footNotesPage: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMainState.footEndNoteCache.removeFootEndNoteCache(startCP, endCP);
      self.mMainState.footEndNoteCache.removeFootEndNoteCache(startCP, endCP);
      // java if (0 != footNotesPage)
      if 0 != footNotesPage {
        // java TypoSnapshot snapshot = this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        let mut i: i32 = TypoFootNotes::getSize__int_typosnapshot(footNotesPage, snapshot) - 1;
        // java for(int i = TypoFootNotes.getSize(footNotesPage, snapshot) - 1; i >= 0 --i)
        while i >= 0 {
          // java int footNote = TypoFootNotes.getFootNote(i, footNotesPage, snapshot);
          let mut footNote: i32 = TypoFootNotes::getFootNote__int_int_typosnapshot(i, footNotesPage, snapshot);
          // java if (footNote != 0)
          if footNote != 0 {
            // java int refCP = TypoFootEndNote.getRefCP(footNote, snapshot);
            let mut refCP: i32 = TypoFootEndNote::getRefCP__int_typosnapshot(footNote, snapshot);
            // java if (refCP >= startCP && refCP < endCP)
            if refCP >= startCP && refCP < endCP {
              // java this.recyclePageContent(footNote);
              self.recyclePageContent(footNote);
              // java TypoFootNotes.remove(i, footNotesPage, snapshot);
              TypoFootNotes::remove(i, footNotesPage, snapshot);
            }
          }
        }
      }
      */
    }

    // java public void recycleDrawing(int remove)
    pub fn recycleDrawing__int(&mut self, remove: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.recycleDrawing(remove, (TypoDrawingLayers)null);
      self.recycleDrawing__int_typodrawinglayers(remove, None);
      */
    }

    // java public void recycleDrawing(int remove, TypoDrawingLayers drawingLayers)
    pub fn recycleDrawing__int_typodrawinglayers(&mut self, remove: i32, drawingLayers: Option<&mut TypoDrawingLayers>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != drawingLayers)
      if let Some(drawingLayers) = drawingLayers {
        // java drawingLayers.removeDrawing(remove);
        drawingLayers.removeDrawing(remove);
      }
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java int childDrawins = TypoDrawing.getChildDrawings(remove, snapshot);
      let mut childDrawins: i32 = TypoDrawing::getChildDrawings__int_typosnapshot(remove, snapshot);
      let mut index: i32 = TypoList::size__int_typosnapshot(childDrawins, snapshot) - 1;
      // java for(int index = TypoList.size(childDrawins, snapshot) - 1; index >= 0 --index)
      while index >= 0 {
        // java int childDrawing = TypoList.quickGet(index, childDrawins, snapshot);
        let mut childDrawing: i32 = TypoList::quickGet__int_int_typosnapshot(index, childDrawins, snapshot);
        // java this.recycleDrawing(childDrawing, (TypoDrawingLayers)null);
        self.recycleDrawing__int_typodrawinglayers(childDrawing, None);
      }
      // java int pTextBox = TypoDrawing.getTextbox(remove, snapshot);
      let mut pTextBox: i32 = TypoDrawing::getTextbox__int_typosnapshot(remove, snapshot);
      // java this.recyclePageContent(pTextBox);
      self.recyclePageContent(pTextBox);
      // java if (drawingLayers != null && TypoDrawing.isTable(remove, snapshot))
      if let Some(drawingLayers) = drawingLayers {
        if TypoDrawing::isTable__int_typosnapshot(remove, snapshot) {
          // java drawingLayers.removeDrawinghsForParent(pTextBox);
          drawingLayers.removeDrawinghsForParent(pTextBox);
        }
      }
      */
    }

    // java private void recycleColumns(int typoColumns)
    pub fn recycleColumns(&self, typoColumns: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mMainState.snapshot;
      let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = TypoColumns::getColumnsCount__int_typosnapshot(typoColumns, snapshot);
      // java for(int size = TypoColumns.getColumnsCount(typoColumns, snapshot); i < size; ++i)
      while i < size {
        // java int columnPage = TypoColumns.getColumnPage(i, typoColumns, snapshot);
        let mut columnPage: i32 = TypoColumns::getColumnPage__int_int_typosnapshot(i, typoColumns, snapshot);
        // java int footNotes = TypoColumnPage.getFootnotes(columnPage, snapshot);
        let mut footNotes: i32 = TypoColumnPage::getFootnotes__int_typosnapshot(columnPage, snapshot);
        // java if (0 != footNotes)
        if 0 != footNotes {
          // java int j = 0;
          let mut j: i32 = 0;
          let mut footNoteSize: i32 = TypoFootNotes::getSize__int_typosnapshot(footNotes, snapshot);
          // java for(int footNoteSize = TypoFootNotes.getSize(footNotes, snapshot); j < footNoteSize ++j)
          while j < footNoteSize {
            // java int footNote = TypoFootNotes.getFootNote(j, footNotes, snapshot);
            let mut footNote: i32 = TypoFootNotes::getFootNote__int_int_typosnapshot(j, footNotes, snapshot);
            // java this.recyclePageContent(footNote);
            self.recyclePageContent(footNote);
          }
          // java TypoColumnPage.setFootnotes(columnPage, 0, snapshot);
          TypoColumnPage::setFootnotes__int_int_typosnapshot(columnPage, 0, snapshot);
        }
        // java this.recyclePageContent(columnPage);
        self.recyclePageContent(columnPage);
      }
      */
    }

}
