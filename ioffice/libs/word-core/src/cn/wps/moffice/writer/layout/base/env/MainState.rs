// Java: import cn.wps.collections.KList
use util::collections::KList::KList;
// Java: import cn.wps.kfc.impl.arrayList.WIntArrayList
use util::kfc::r#impl::arrayList::WIntArrayList::WIntArrayList;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.cache.TypoDrawingLayers
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoDrawingLayers::TypoDrawingLayers;
// Java: import cn.wps.moffice.writer.cache.TypoLayoutPage
use crate::cn::wps::moffice::writer::cache::TypoLayoutPage::TypoLayoutPage;
// Java: import cn.wps.moffice.writer.cache.TypoList
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
// Java: import cn.wps.moffice.writer.cache.TypoRevisions
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::cache::TypoRevisions::TypoRevisions;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::cache::TypoDrawingLayers::TypoDrawingLayers;
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.core.pool.Reusable
use crate::cn::wps::moffice::writer::core::pool::Reusable::Reusable;
// Java: import cn.wps.moffice.writer.data.PLCSection
use crate::cn::wps::moffice::writer::data::PLCSection::{PLCSection, SectionNode};
// Java: import cn.wps.moffice.writer.layout.base.edit.Event
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::edit::Event::Event;
// Java: import cn.wps.moffice.writer.layout.base.edit.GridSizeLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::edit::GridSizeLayouter::GridSizeLayouter;
// Java: import cn.wps.moffice.writer.layout.base.env.DrawingCacheItem
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::DrawingCacheItem;
// Java: import cn.wps.moffice.writer.layout.base.env.FootNoteCache
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::FootNoteCache::FootNoteCache;
// Java: import cn.wps.moffice.writer.layout.base.env.Pools
use crate::cn::wps::moffice::writer::layout::base::env::Pools::Pools;


// CARGO_OFF_SURFACE_TYPES
#[derive(Default, Clone)]
pub struct TypoRevisions;
#[derive(Default, Clone)]
pub struct DrawingCacheItem;
#[derive(Default, Clone)]
pub struct Event;
#[derive(Default, Clone)]
pub struct GridSizeLayouter;
// java public final class MainState implements Reusable
#[derive(Default)]
pub struct MainState {
    // java public TypoLayoutPage layoutPage = TypoLayoutPage.newInstance();
    pub(crate) layoutPage: TypoLayoutPage,
    // java public TypoRevisions revisions = TypoRevisions.newInstance();
    pub(crate) revisions: RevisionsHolder,
    // java public int curPageTop;
    pub(crate) curPageTop: i32,
    // java public boolean pageOrientationChanged = false;
    pub(crate) pageOrientationChanged: bool,
    // java public boolean needTableFit = false;
    pub(crate) needTableFit: bool,
    // java public int curPageIndex = 0;
    pub(crate) curPageIndex: i32,
    // java public int curSecPageIndex = 1;
    pub(crate) curSecPageIndex: i32,
    // java public int curSecMsoNfc = 0;
    pub(crate) curSecMsoNfc: i32,
    // java public PLCSection.SectionNode sectionNode;
    pub(crate) sectionNode: Option<SectionNode>,
    // java public PropertySet secPropSet;
    pub(crate) secPropSet: Option<PropertySet>,
    // java public boolean firstUseDefaultSec = false;
    pub(crate) firstUseDefaultSec: bool,
    // java public int secIndex;
    pub(crate) secIndex: i32,
    // java public int secStart;
    pub(crate) secStart: i32,
    // java public int secEnd;
    pub(crate) secEnd: i32,
    // java public int secBreakCode = 2;
    pub(crate) secBreakCode: i32,
    // java public int columns;
    pub(crate) columns: i32,
    // java public KList<DrawingCacheItem> columnsFloatsCache = new KList();
    pub(crate) columnsFloatsCache: Vec<crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::DrawingCacheItem>,
    // java public KList<DrawingCacheItem> floatsCache = new KList();
    pub(crate) floatsCache: Vec<crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::DrawingCacheItem>,
    // java public WIntArrayList layoutPageRelParaQuasiFloatFCs = new WIntArrayList();
    pub(crate) layoutPageRelParaQuasiFloatFCs: Vec<i32>,
    // java public WIntArrayList layoutPageRelParaQuasiFloats = new WIntArrayList();
    pub(crate) layoutPageRelParaQuasiFloats: Vec<i32>,
    // java public TypoSnapshot snapshot;
    pub snapshot: TypoSnapshot,
    // java public TypoSnapshot mainSnapshot;
    pub(crate) mainSnapshot: TypoSnapshot,
    // java public TypoDrawingLayers drawingLayers = TypoDrawingLayers.newInstance();
    pub(crate) drawingLayers: TypoDrawingLayers,
    // java public TypoDrawingLayers headerFooterDrawingLayers = TypoDrawingLayers.newInstance();
    pub(crate) headerFooterDrawingLayers: TypoDrawingLayers,
    // java public KList<Event> eventsForReflowField = new KList();
    pub(crate) eventsForReflowField: Vec<EventHolder>,
    // java public FootNoteCache footEndNoteCache;
    pub(crate) footEndNoteCache: Option<FootNoteCache>,
    // java public boolean hasFieldPage;
    pub(crate) hasFieldPage: bool,
    // java public boolean hasFieldNumPages;
    pub(crate) hasFieldNumPages: bool,
    // java public boolean hasFieldSectionPages;
    pub(crate) hasFieldSectionPages: bool,
    // java public WIntArrayList fieldRanges = new WIntArrayList();
    pub(crate) fieldRanges: Vec<i32>,
    // java public WIntArrayList curLineBallooItem = new WIntArrayList();
    pub(crate) curLineBallooItem: Vec<i32>,
    // java public int endEmbedPage = 0;
    pub(crate) endEmbedPage: i32,
    // java public GridSizeLayouter mGridSizeLayouter;
    pub(crate) mGridSizeLayouter: Option<GridSizeLayouterHolder>,
    // java public boolean isInUiCard;
    pub(crate) isInUiCard: bool,
}

// java public interface Reusable
impl Reusable for MainState {
    // java public boolean reuseClean()
    fn reuseClean(&mut self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.layoutPage.clear();
      self.layoutPage.clear();
      // java this.revisions.clear();
      self.revisions.clear();
      // java this.drawingLayers.clear();
      self.drawingLayers.clear();
      // java this.headerFooterDrawingLayers.clear();
      self.headerFooterDrawingLayers.clear();
      // java if (this.snapshot != null)
      if true {
        // java this.snapshot.release();
        self.snapshot.release();
        // java this.snapshot = null;
        self.snapshot = None;
      }
      // java if (this.mainSnapshot != null)
      if true {
        // java this.mainSnapshot.release();
        self.mainSnapshot.release();
        // java this.mainSnapshot = null;
        self.mainSnapshot = None;
      }
      // java this.footEndNoteCache = null;
      self.footEndNoteCache = None;
      // java this.curPageIndex = 0;
      self.curPageIndex = 0;
      // java this.curSecPageIndex = 1;
      self.curSecPageIndex = 1;
      // java this.curSecMsoNfc = 0;
      self.curSecMsoNfc = 0;
      // java this.sectionNode = null;
      self.sectionNode = None;
      // java this.secPropSet = null;
      self.secPropSet = None;
      // java this.secBreakCode = 2;
      self.secBreakCode = 2;
      // java this.layoutPageRelParaQuasiFloatFCs.resetQuick();
      self.layoutPageRelParaQuasiFloatFCs.resetQuick();
      // java this.layoutPageRelParaQuasiFloats.resetQuick();
      self.layoutPageRelParaQuasiFloats.resetQuick();
      // java this.fieldRanges.resetQuick();
      self.fieldRanges.resetQuick();
      // java this.isInUiCard = false;
      self.isInUiCard = false;
      // java return true;
      return true;
      */
    }

    // java public void reuseInit()
    fn reuseInit(&mut self) {
    todo!("DEFERRED_NON_FUNCTION");
    /* DEFERRED_NON_FUNCTION

    */
    }

}

// java public final class MainState implements Reusable
impl MainState {
    // java public void reset()
    pub fn reset(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.layoutPage.clear();
      self.layoutPage.clear();
      // java this.drawingLayers.clear();
      self.drawingLayers.clear();
      // java this.headerFooterDrawingLayers.clear();
      self.headerFooterDrawingLayers.clear();
      // java this.revisions.clear();
      self.revisions.clear();
      // java this.firstUseDefaultSec = false;
      self.firstUseDefaultSec = false;
      // java this.isInUiCard = false;
      self.isInUiCard = false;
      */
    }

    // java public boolean reuseClean(Pools pools)
    pub fn reuseClean__pools(&mut self, pools: &mut Pools) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.reuseClean();
      self.reuseClean();
      // java pools.recycleDrawingCacheItems(this.columnsFloatsCache);
      pools.recycleDrawingCacheItems(self.columnsFloatsCache);
      // java pools.recycleDrawingCacheItems(this.floatsCache);
      pools.recycleDrawingCacheItems(self.floatsCache);
      // java pools.recycleEvents(this.eventsForReflowField);
      pools.recycleEvents(self.eventsForReflowField);
      // java this.disposeGridSizeLayouter();
      self.disposeGridSizeLayouter();
      // java return true;
      return true;
      */
    }

    // java public void disposeGridSizeLayouter()
    pub fn disposeGridSizeLayouter(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mGridSizeLayouter != null)
      if let Some(mGridSizeLayouter) = self.mGridSizeLayouter.as_mut() {
        // java this.mGridSizeLayouter.dispose();
        mGridSizeLayouter.dispose();
        // java this.mGridSizeLayouter = null;
        mGridSizeLayouter = None;
      }
      */
    }

    // java public int getWrapDrawingCount(Document document)
    pub fn getWrapDrawingCount(&self, document: &dyn Document) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int count = 0;
      let mut count: i32 = 0;
      // java TypoDrawingLayers drawingLayers = this.getDrawingLayers(document);
      let mut drawingLayers: TypoDrawingLayers = { let __x: TypoDrawingLayers = todo!("no align strategy matched"); __x };
      // java int wraps = drawingLayers == null ? 0 : this.getDrawingLayers(document).getWraps();
      let mut wraps: i32 = (if false { 0 } else { self.getDrawingLayers__document(document).getWraps() });
      // java if (0 != wraps)
      if 0 != wraps {
        // java count = TypoList.size(wraps, this.snapshot);
        count = TypoList::size__int_typosnapshot(wraps, self.snapshot);
      }
      // java if (document.getType() == 0)
      if document.getType() == 0 {
        // java wraps = this.headerFooterDrawingLayers.getWraps();
        wraps = self.headerFooterDrawingLayers.getWraps();
        // java if (0 != wraps)
        if 0 != wraps {
          // java count += TypoList.size(wraps, this.snapshot);
          count = count + TypoList::size__int_typosnapshot(wraps, self.snapshot);
        }
      }
      // java return count;
      return count;
      */
    }

    // java public boolean hasWrapDrawing(Document doc)
    pub fn hasWrapDrawing(&self, doc: &dyn Document) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoDrawingLayers drawingLayers = this.getDrawingLayers(doc);
      let mut drawingLayers: TypoDrawingLayers = { let __x: TypoDrawingLayers = todo!("no align strategy matched"); __x };
      // java int wraps = drawingLayers == null ? 0 : this.getDrawingLayers(doc).getWraps();
      let mut wraps: i32 = (if false { 0 } else { self.getDrawingLayers__document(doc).getWraps() });
      // java boolean ret = false;
      let mut ret: bool = false;
      // java if (0 != wraps)
      if 0 != wraps {
        // java ret = TypoList.size(wraps, this.snapshot) > 0;
        ret = TypoList::size__int_typosnapshot(wraps, self.snapshot) > 0;
      }
      // java if (!ret && doc.getType() == 0)
      if !ret && doc.getType() == 0 {
        // java wraps = this.headerFooterDrawingLayers.getWraps();
        wraps = self.headerFooterDrawingLayers.getWraps();
        // java if (0 != wraps)
        if 0 != wraps {
          // java ret = TypoList.size(wraps, this.snapshot) > 0;
          ret = TypoList::size__int_typosnapshot(wraps, self.snapshot) > 0;
        }
      }
      // java return ret;
      return ret;
      */
    }

    // java public boolean hasFloats()
    pub fn hasFloats(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return !this.floatsCache.isEmpty();
      return !self.floatsCache.is_empty();
      */
    }

    // java public TypoDrawingLayers getDrawingLayers(Document doc)
    pub fn getDrawingLayers<'a>(&'a mut self, doc: &dyn Document) -> Option<&'a mut TypoDrawingLayers> {
      self.getDrawingLayers__document_boolean(doc, false)
    }

    // java public TypoDrawingLayers getDrawingLayers(Document doc, boolean fource)
    pub fn getDrawingLayers__document_boolean<'a>(&'a mut self, doc: &dyn Document, fource: bool) -> Option<&'a mut TypoDrawingLayers> {
      let doc_type = doc.getType__on_IDocument();
      if doc_type != 0 && doc_type != 5 {
        if doc_type != 2 && doc_type != 6 {
          if fource || self.drawingLayers.hasDocTag(doc_type) {
            Some(&mut self.drawingLayers)
          } else {
            None
          }
        } else {
          Some(&mut self.headerFooterDrawingLayers)
        }
      } else {
        Some(&mut self.drawingLayers)
      }
    }
      /* DEFERRED_NON_FUNCTION
      // java int docType = doc.getType();
      let mut docType: i32 = doc.getType();
      // java TypoDrawingLayers drawingLayers = null;
      let mut drawingLayers: TypoDrawingLayers = Default::default();
      // java if (docType != 0 && docType != 5)
      if docType != 0 && docType != 5 {
        // java if (docType != 2 && docType != 6)
        if docType != 2 && docType != 6 {
          // java if (fource || this.drawingLayers.hasDocTag(docType))
          if fource || self.drawingLayers.hasDocTag__int(docType) {
            // java drawingLayers = this.drawingLayers;
            drawingLayers = self.drawingLayers;
          }
        }
        // java else
        else {
          // java drawingLayers = this.headerFooterDrawingLayers;
          drawingLayers = self.headerFooterDrawingLayers;
        }
      }
      // java else
      else {
        // java drawingLayers = this.drawingLayers;
        drawingLayers = self.drawingLayers;
      }
      // java return drawingLayers;
      return drawingLayers;
      */

}

// java public class TypoRevisions
// Each slot is docType * 6 + revisionType. An item is cpRef, page, version, authorId.
#[derive(Default, Clone)]
pub struct RevisionsHolder {
    slots: Vec<Vec<i32>>,
}

impl RevisionsHolder {
    // java private IntArrayList getData(int docType, int revisionType)
    fn slot_mut(&mut self, doc_type: i32, revision_type: i32) -> &mut Vec<i32> {
        let index = (doc_type * 6 + revision_type) as usize;
        if self.slots.len() <= index {
            self.slots.resize(index + 1, Vec::new());
        }
        &mut self.slots[index]
    }

    fn slot(&self, doc_type: i32, revision_type: i32) -> &[i32] {
        let index = (doc_type * 6 + revision_type) as usize;
        self.slots.get(index).map(|slot| slot.as_slice()).unwrap_or(&[])
    }

    // java public void clear()
    pub fn clear(&mut self) {
        for slot in &mut self.slots {
            slot.clear();
        }
    }

    // java private int search(int cp, IntArrayList revision)
    fn search(cp: i32, data: &[i32], snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
        let mut low = 0i32;
        let mut high = (data.len() as i32) / 4;
        while low < high {
            let mid = (low + high) / 2;
            let cur_cp = snapshot.getCPRef(data[(mid as usize) * 4]);
            if cp < cur_cp {
                high = mid;
            } else if cp <= cur_cp {
                return mid;
            } else {
                low = mid + 1;
            }
        }
        -(low + 1)
    }

    // java public void addRevisionDeleteTable(Document document, int cp, int authorId, int pPage)
    pub fn addRevisionDeleteTable(
        &mut self,
        document: &dyn crate::cn::wps::moffice::writer::core::Document::Document,
        cp: i32,
        author_id: i32,
        page: i32,
        snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot,
        author_of: &mut dyn FnMut(&str) -> i32,
    ) {
        let doc_type = document.getType__on_IDocument();
        let item = crate::cn::wps::moffice::writer::cache::RevisionCache::get(document, cp, 1, &mut snapshot.revision_slots, author_of);
        let Some(item) = item else { return; };
        if cp != item.start {
            return;
        }
        let item_index = Self::search(cp, self.slot(doc_type, 1), snapshot);
        if item_index >= 0 {
            let version_at = (item_index as usize) * 4 + 2;
            let data = self.slot_mut(doc_type, 1);
            data[version_at] = data[version_at].wrapping_add(1);
        } else {
            let cp_ref = snapshot.addRefCP(doc_type, cp);
            let index = ((-item_index - 1) * 4) as usize;
            let data = self.slot_mut(doc_type, 1);
            data.insert(index, cp_ref);
            data.insert(index + 1, page);
            data.insert(index + 2, 0);
            data.insert(index + 3, author_id);
        }
    }

    // java public void addRevisionChangeTable(Document document, int cp, int tableLevel, int authorId, int pPage)
    pub fn addRevisionChangeTable(
        &mut self,
        document: &dyn crate::cn::wps::moffice::writer::core::Document::Document,
        cp: i32,
        table_level: i32,
        author_id: i32,
        page: i32,
        snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot,
        author_of: &mut dyn FnMut(&str) -> i32,
    ) {
        let doc_type = document.getType__on_IDocument();
        let item = crate::cn::wps::moffice::writer::cache::RevisionCache::get_table_revision(document, cp, table_level, &mut snapshot.revision_slots, author_of);
        let Some(item) = item else { return; };
        if cp != item.start {
            return;
        }
        let item_index = Self::search(cp, self.slot(doc_type, 4), snapshot);
        if item_index >= 0 {
            let page_at = (item_index as usize) * 4 + 1;
            let same_page = self.slot(doc_type, 4).get(page_at).copied() == Some(page);
            if same_page {
                let version_at = (item_index as usize) * 4 + 2;
                let data = self.slot_mut(doc_type, 4);
                data[version_at] = data[version_at].wrapping_add(1);
            } else {
                let cp_ref = snapshot.addRefCP(doc_type, cp);
                let index = ((item_index) * 4) as usize;
                let data = self.slot_mut(doc_type, 4);
                data.insert(index, cp_ref);
                data.insert(index + 1, page);
                data.insert(index + 2, 0);
                data.insert(index + 3, author_id);
            }
        } else {
            let cp_ref = snapshot.addRefCP(doc_type, cp);
            let index = ((-item_index - 1) * 4) as usize;
            let data = self.slot_mut(doc_type, 4);
            let index = index.min(data.len());
            data.insert(index, cp_ref);
            data.insert(index + 1, page);
            data.insert(index + 2, 0);
            data.insert(index + 3, author_id);
        }
    }
}
#[derive(Default, Clone)]
pub struct SectionNodeHolder;
#[derive(Default, Clone)]
pub struct DrawingCacheItemHolder;
#[derive(Default, Clone)]
pub struct DrawingLayersHolder;
#[derive(Default, Clone)]
pub struct EventHolder;
#[derive(Default, Clone)]
pub struct FootNoteCacheHolder;

// java public class FootNoteCache extends KList<FootNoteCacheItem>
pub struct FootNoteCache {
    items: Vec<FootNoteCacheItem>,
}

// java public class FootNoteCacheItem extends KList.RecycableNode
pub struct FootNoteCacheItem {
    // java public int cp;
    pub cp: i32,
    // java public String numStr;
    pub num_str: Option<String>,
    // java public int footnote;
    pub footnote: i32,
    // java public PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode refNote;
    pub ref_note: crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode,
}

impl FootNoteCache {
    // java public FootNoteCache(Pools pools)
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    // java public FootNoteCacheItem addFootEndNoteCache(int cp, FootnoteEndnoteRefNode footEndNote, String numStr)
    pub fn addFootEndNoteCache(
        &mut self,
        cp: i32,
        foot_end_note: crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode,
        num_str: Option<String>,
    ) -> Option<String> {
        let stored = num_str.clone();
        let item = FootNoteCacheItem { cp, num_str, footnote: 0, ref_note: foot_end_note };
        if self.items.is_empty() {
            self.items.push(item);
        } else {
            let mut pos = 0i32;
            let mut i = self.items.len() as i32 - 1;
            while i >= 0 {
                if self.items[i as usize].cp <= cp {
                    pos = i + 1;
                    break;
                }
                i -= 1;
            }
            self.items.insert(pos as usize, item);
        }
        stored
    }

    // java public void clear()
    pub fn clear(&mut self) {
        self.items.clear();
    }

    // java public void removeFootEndNoteCache(int startCP, int endCP)
    pub fn removeFootEndNoteCache(&mut self, start_cp: i32, end_cp: i32) {
        self.items.retain(|item| item.cp < start_cp || item.cp >= end_cp);
    }

    // java public int size()
    pub fn size(&self) -> i32 { self.items.len() as i32 }
}
#[derive(Default, Clone)]
pub struct GridSizeLayouterHolder;
