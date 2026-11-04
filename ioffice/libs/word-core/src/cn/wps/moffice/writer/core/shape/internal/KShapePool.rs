// Java: cn.wps.moffice.writer.core.shape.internal.KShapePool
use super::FilePage::{FilePage, ACTIVE_SENTINEL};
use drawing::cn::wps::moffice::drawing::IShapePool::IShapePool;
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::graphics::SolidFill::SolidFill;
use std::sync::{Arc, Mutex};
use std::any::Any;
use util::kfc::api::procedure::IObjectProcedure::IObjectProcedure;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// java public static final int MAX_ACTIVE_SHAPE_COUNT = 3072;
const MAX_ACTIVE_SHAPE_COUNT: i32 = 3072;

struct PoolInner {
    // java List<FilePage> filePages
    file_pages: Vec<Option<FilePage>>,
    // java int mActiveTotalShapeCount
    mActiveTotalShapeCount: i32,
    // java FilePage mActiveFilePages ring head. -1 means the sentinel.
    sentinel_next: i32,
    sentinel_prev: i32,
}

impl PoolInner {
    fn new() -> Self {
        Self {
            file_pages: Vec::new(),
            mActiveTotalShapeCount: 0,
            // Java sentinel's next/prev start as the sentinel itself (empty ring).
            sentinel_next: ACTIVE_SENTINEL,
            sentinel_prev: ACTIVE_SENTINEL,
        }
    }

    // java private int getFilePageIndexByShapeId(int shapeId)
    fn getFilePageIndexByShapeId(shapeId: i32) -> i32 {
        // java int index = (shapeId - 1024) / 1024;
        // java return index < 0 ? 0 : index;
        let index = (shapeId - 1024) / 1024;
        if index < 0 { 0 } else { index }
    }

    // java private FilePage findFilePage(int shapeId)
    fn findFilePage(&self, shapeId: i32) -> Option<usize> {
        // java int index = this.getFilePageIndexByShapeId(shapeId);
        // java return index < this.filePages.size() ? (FilePage)this.filePages.get(index) : null;
        let index = Self::getFilePageIndexByShapeId(shapeId) as usize;
        if index < self.file_pages.len() && self.file_pages[index].is_some() {
            Some(index)
        } else {
            None
        }
    }

    // java private FilePage insert_filePage(Integer shapeId)
    fn insert_filePage(&mut self, shapeId: i32) -> usize {
        // java int index = this.getFilePageIndexByShapeId(shapeId);
        // java FilePage filePage = new FilePage(index);
        let index = Self::getFilePageIndexByShapeId(shapeId);
        let file_page = FilePage::new(index);
        let idx = index as usize;
        // java if (filePage.mIndex >= this.filePages.size()) { pad nulls; add }
        // java else this.filePages.set(filePage.mIndex, filePage);
        if idx >= self.file_pages.len() {
            self.file_pages.resize_with(idx, || None);
            self.file_pages.push(Some(file_page));
        } else {
            self.file_pages[idx] = Some(file_page);
        }
        idx
    }

    fn page_mut(&mut self, index: i32) -> &mut FilePage {
        self.file_pages[index as usize].as_mut().expect("file page")
    }

    // java private void active_remove(FilePage filePage)
    fn active_remove(&mut self, index: i32) {
        let (prev, next) = {
            let page = self.page_mut(index);
            let prev = page.mPrevActive;
            let next = page.mNextActive;
            page.mPrevActive = ACTIVE_SENTINEL;
            page.mNextActive = ACTIVE_SENTINEL;
            (prev, next)
        };
        // Detached pages keep sentinel links; removing them again is a no-op.
        if prev == ACTIVE_SENTINEL && next == ACTIVE_SENTINEL
            && self.sentinel_next != index && self.sentinel_prev != index {
            return;
        }
        if prev == ACTIVE_SENTINEL {
            self.sentinel_next = next;
        } else {
            self.page_mut(prev).mNextActive = next;
        }
        if next == ACTIVE_SENTINEL {
            self.sentinel_prev = prev;
        } else {
            self.page_mut(next).mPrevActive = prev;
        }
    }

    // java private void active_add(FilePage filePage)
    fn active_add(&mut self, index: i32) {
        // java this.active_remove(filePage);
        self.active_remove(index);
        // java while (this.mActiveTotalShapeCount > 3072) this.active_kill_one();
        self.evict_overflow();
        // java filePage.mPrevActive = sentinel; filePage.mNextActive = sentinel.mNextActive;
        let next = self.sentinel_next;
        {
            let page = self.page_mut(index);
            page.mPrevActive = ACTIVE_SENTINEL;
            page.mNextActive = next;
        }
        if next == ACTIVE_SENTINEL {
            self.sentinel_prev = index;
        } else {
            self.page_mut(next).mPrevActive = index;
        }
        self.sentinel_next = index;
        if self.page_mut(index).mNextActive == ACTIVE_SENTINEL {
            self.sentinel_prev = index;
        }
    }

    // java private void active_kill_one()
    fn active_kill_one(&mut self) {
        // java FilePage filePage = this.mActiveFilePages.mPrevActive;
        let lru = self.sentinel_prev;
        if lru == ACTIVE_SENTINEL {
            self.mActiveTotalShapeCount = 0;
            return;
        }
        // java this.active_remove(filePage);
        self.active_remove(lru);
        // java this.unLockfilePageContent(filePage);
        self.unLockfilePageContent(lru);
    }

    // java private int unLockfilePageContent(FilePage filePage)
    fn unLockfilePageContent(&mut self, index: i32) -> i32 {
        let page = self.page_mut(index);
        // java if (null != filePage.mFilePageContent)
        if page.mContentLive {
            // java int nRemoved = filePage.size();
            let n_removed = page.size();
            // java if (filePage.mDirty) this.mMemberyPool.write(filePage);
            // The memory-pool file is the eviction copy. Shapes stay in mShapes,
            // which is what findShapeById reads. Property bytes stay on the Shape
            // so a later lock does not have to reload a missing file.
            page.mDirty = false;
            // java this.mActiveTotalShapeCount -= nRemoved;
            self.mActiveTotalShapeCount -= n_removed;
            n_removed
        } else {
            // java return 0;
            0
        }
    }

    fn evict_overflow(&mut self) {
        let mut guard = 0;
        // java while (this.mActiveTotalShapeCount > 3072) this.active_kill_one();
        while self.mActiveTotalShapeCount > MAX_ACTIVE_SHAPE_COUNT {
            let before = self.mActiveTotalShapeCount;
            self.active_kill_one();
            guard += 1;
            if self.mActiveTotalShapeCount >= before || guard > 64 {
                break;
            }
        }
    }

    // java private FilePage lockFilePageContent(FilePage filePage)
    fn lockFilePageContent(&mut self, index: usize) {
        let index = index as i32;
        // java this.active_add(filePage);
        self.active_add(index);
        // java if (filePage.mFilePageContent != null) return filePage;
        if self.page_mut(index).mContentLive {
            return;
        }
        // java else read from mMemberyPool and add filePage.size() to the active count.
        // Content was never dropped from mShapes, so the page is live again.
        let n = {
            let page = self.page_mut(index);
            page.mContentLive = true;
            page.mDirty = false;
            page.shape_count()
        };
        self.mActiveTotalShapeCount += n;
    }
}

// java public class KShapePool implements IShapePool
pub struct KShapePool {
    inner: Mutex<PoolInner>,
}

impl KShapePool {
    // java KShapePool(Document document)
    pub fn new() -> Self {
        // java this.mMemberyPool = new KShapePropMemoryPool(document);
        // java this.mActiveFilePages = new FilePage(); ring to itself
        Self { inner: Mutex::new(PoolInner::new()) }
    }

    fn lock_inner(&self) -> std::sync::MutexGuard<'_, PoolInner> {
        self.inner.lock().expect("KShapePool")
    }

    pub fn for_each_shape(&self, mut visit: impl FnMut(&Arc<Mutex<Shape>>) -> bool) -> bool {
        let inner = self.lock_inner();
        for page in inner.file_pages.iter().flatten() {
            for shape in page.getShapes() {
                if !visit(&shape) {
                    return false;
                }
            }
        }
        true
    }

    // java public void analytics(boolean[] result)
    pub fn analytics(&self, result: &mut [bool]) {
        let _guard = self.lock_inner();
        // Walk without holding the pool lock across shape locks: copy arcs first.
        let shapes: Vec<Arc<Mutex<Shape>>> = _guard.file_pages.iter().flatten()
            .flat_map(|page| page.getShapes()).collect();
        drop(_guard);
        for shape in shapes {
            let mut shape = shape.lock().expect("shape");
            let has_inks = Self::analyticInks(result, &mut shape);
            let has_pictures = Self::analyticPictures(result, &mut shape);
            let has_white = Self::analyticWhiteBackground(result, &mut shape);
            if has_inks && has_pictures && has_white {
                return;
            }
        }
    }

    // java private boolean analyticInks(boolean[] result, Shape shape)
    fn analyticInks(result: &mut [bool], shape: &mut Shape) -> bool {
        // java if (shape.hasInk()) result[8] = true;
        if shape.hasInk() && result.len() > 8 {
            result[8] = true;
        }
        // java return result[8];
        result.get(8).copied().unwrap_or(false)
    }

    // java private boolean analyticPictures(boolean[] result, Shape shape)
    fn analyticPictures(result: &mut [bool], shape: &mut Shape) -> bool {
        // java if (shape.isPicture()) result[0] = true;
        if shape.isPicture() && !result.is_empty() {
            result[0] = true;
        }
        // java return result[0];
        result.first().copied().unwrap_or(false)
    }

    // java private boolean analyticWhiteBackground(boolean[] result, Shape shape)
    fn analyticWhiteBackground(result: &mut [bool], shape: &mut Shape) -> bool {
        // java SolidFill fill = shape.getSolidFill();
        let fill = shape.getSolidFill();
        // java if (shape.isBackground() && null != fill && fill.getColor() != 16777215) result[7] = true;
        if shape.isBackground() {
            if let Some(fill) = fill.as_ref().and_then(|v| v.downcast_ref::<SolidFill>()) {
                if fill.base.getColor() != 16777215 && result.len() > 7 {
                    result[7] = true;
                }
            }
        }
        // java return result[7];
        result.get(7).copied().unwrap_or(false)
    }

    // java public boolean hasArtText()
    pub fn hasArtText(&self) -> bool {
        let shapes: Vec<Arc<Mutex<Shape>>> = {
            let inner = self.lock_inner();
            inner.file_pages.iter().flatten().flat_map(|page| page.getShapes()).collect()
        };
        for shape in shapes {
            // java if (shape.isArtText()) return true;
            if shape.lock().expect("shape").isArtText() {
                return true;
            }
        }
        false
    }
}

impl Default for KShapePool {
    fn default() -> Self { Self::new() }
}

fn to_element(shape: Arc<Mutex<Shape>>) -> Element {
    Some(shape as Arc<dyn Any + Send + Sync>)
}

fn shape_arc(element: &Element) -> Option<Arc<Mutex<Shape>>> {
    element.as_ref()?.clone().downcast::<Mutex<Shape>>().ok()
}

impl IShapePool for KShapePool {
    // java public void lock()
    fn lock(&self) {
        // Each map method takes the same mutex. A separate lock here would deadlock
        // a non-reentrant mutex, so the Java ReentrantLock is the mutex inside those methods.
    }

    // java public void unlock()
    fn unlock(&self) {}

    // java public void dispose()
    fn dispose(&mut self) {
        let mut inner = self.lock_inner();
        // java if (null != this.filePages) { this.filePages.clear(); this.filePages = null; }
        inner.file_pages.clear();
        inner.mActiveTotalShapeCount = 0;
        inner.sentinel_next = ACTIVE_SENTINEL;
        inner.sentinel_prev = ACTIVE_SENTINEL;
    }

    // java public void addToShapeMap(Integer shapeId, Shape shape)
    fn addToShapeMap(&mut self, shapeId: i32, shape: Element) {
        let Some(shape) = shape_arc(&shape) else { return };
        let mut inner = self.lock_inner();
        // java FilePage filePage = this.findFilePage(shapeId);
        // java if (null == filePage) filePage = this.insert_filePage(shapeId);
        let index = if let Some(index) = inner.findFilePage(shapeId) {
            index
        } else {
            inner.insert_filePage(shapeId)
        };
        // java this.lockFilePageContent(filePage);
        inner.lockFilePageContent(index);
        // java filePage.put(shapeId, shape);
        inner.page_mut(index as i32).put(shapeId, shape);
        // java ++this.mActiveTotalShapeCount;
        inner.mActiveTotalShapeCount += 1;
        // java filePage.mDirty = true;  (put already marks dirty)
        // java while (this.mActiveTotalShapeCount > 3072) this.active_kill_one();
        inner.evict_overflow();
    }

    // java public void delFromShapeMap(int shapeId)
    fn delFromShapeMap(&mut self, shapeId: i32) {
        let mut inner = self.lock_inner();
        // java FilePage filePage = this.findFilePage(shapeId);
        let Some(index) = inner.findFilePage(shapeId) else { return };
        // java this.lockFilePageContent(filePage);
        inner.lockFilePageContent(index);
        // java filePageContent.remove(shapeId);
        inner.page_mut(index as i32).remove(shapeId);
        // java --this.mActiveTotalShapeCount;
        inner.mActiveTotalShapeCount -= 1;
    }

    // java public Shape findShapeById(int shapeId)
    fn findShapeById(&self, shapeId: i32) -> Element {
        let inner = self.lock_inner();
        // java FilePage filePage = this.findFilePage(shapeId);
        // java if (null != filePage) return filePage.findShapeById(shapeId);
        // java return null;
        match inner.findFilePage(shapeId) {
            Some(index) => inner.file_pages[index].as_ref()
                .and_then(|page| page.findShapeById(shapeId))
                .map(to_element)
                .unwrap_or(None),
            None => None,
        }
    }

    // java public Collection<? extends Shape> getAllShapesWithChild()
    fn getAllShapesWithChild(&self) -> Vec<Element> {
        let inner = self.lock_inner();
        let mut shapes = Vec::new();
        // java for each file page: shapes.addAll(filePage.getShapes());
        for page in inner.file_pages.iter().flatten() {
            for shape in page.getShapes() {
                shapes.push(to_element(shape));
            }
        }
        shapes
    }

    // java public void propChanged(Shape shape, boolean finalModified)
    fn propChanged(&self, shape: Element, finalModified: bool) {
        let Some(arc) = shape_arc(&shape) else { return };
        let shape_id = arc.lock().expect("shape").getShapeID();
        let mut inner = self.lock_inner();
        // java if (shapeId > 0 && null != filePage) { lock; filePage.mDirty = finalModified; }
        if shape_id > 0 {
            if let Some(index) = inner.findFilePage(shape_id) {
                inner.lockFilePageContent(index);
                inner.page_mut(index as i32).mDirty = finalModified;
            }
        }
    }

    // java public MutablePropertyMap getProperty(PropBase propBase)
    fn getProperty(&self, shape: Element) -> Element {
        // Properties stay on the Shape while the page is live, so there is no
        // detached map to return. A missing page is Java's null file page.
        let Some(arc) = shape_arc(&shape) else { return None };
        let shape_id = arc.lock().expect("shape").getShapeID();
        let inner = self.lock_inner();
        if inner.findFilePage(shape_id).is_none() {
            return None;
        }
        None
    }
}
