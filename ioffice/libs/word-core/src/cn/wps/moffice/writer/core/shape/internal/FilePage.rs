// Java: cn.wps.moffice.writer.core.shape.internal.FilePage
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Sentinel index used by `KShapePool`'s active-page ring. Java uses a dummy `FilePage`.
pub const ACTIVE_SENTINEL: i32 = -1;

// java class FilePage
pub struct FilePage {
    // java WIntObjectHashMap<Shape> mShapes
    mShapes: HashMap<i32, Arc<Mutex<Shape>>>,
    // java int mIndex
    pub mIndex: i32,
    // java FilePage mPrevActive / mNextActive. -1 is the sentinel page.
    pub mPrevActive: i32,
    pub mNextActive: i32,
    // java boolean mDirty
    pub mDirty: bool,
    // java FilePageContent != null. Properties stay on Shape; this flag is the content slot.
    pub mContentLive: bool,
}

impl FilePage {
    // java FilePage()
    pub fn new_empty() -> Self {
        Self {
            mShapes: HashMap::new(),
            mIndex: ACTIVE_SENTINEL,
            mPrevActive: ACTIVE_SENTINEL,
            mNextActive: ACTIVE_SENTINEL,
            mDirty: false,
            mContentLive: false,
        }
    }

    // java FilePage(int index)
    pub fn new(index: i32) -> Self {
        // java this.mFilePageContent = new FilePageContent(this);
        // java this.mShapes = new WIntObjectHashMap();
        // java this.mIndex = index;
        Self {
            mShapes: HashMap::new(),
            mIndex: index,
            mPrevActive: ACTIVE_SENTINEL,
            mNextActive: ACTIVE_SENTINEL,
            mDirty: false,
            mContentLive: true,
        }
    }

    // java Shape findShapeById(int shapeId)
    pub fn findShapeById(&self, shapeId: i32) -> Option<Arc<Mutex<Shape>>> {
        // java return (Shape)this.mShapes.get(shapeId);
        self.mShapes.get(&shapeId).cloned()
    }

    // java void clear()
    pub fn clear(&mut self) {
        // java if (this.mFilePageContent != null) this.mFilePageContent.clear();
        // java if (null != this.mShapes) this.mShapes.clear();
        self.mShapes.clear();
        self.mContentLive = false;
    }

    // java void put(Integer shapeId, Shape shape)
    pub fn put(&mut self, shapeId: i32, shape: Arc<Mutex<Shape>>) {
        // java this.mFilePageContent.put(shapeId, shape.mProperty);
        // java this.mShapes.put(shapeId, shape);
        // Property bytes stay on the Shape object. The content map in Java is the
        // paged copy of those bytes; while the page is live they are the same object.
        self.mContentLive = true;
        self.mShapes.insert(shapeId, shape);
        self.mDirty = true;
    }

    // java FilePageContent.remove(int key) also removes mShapes
    pub fn remove(&mut self, shapeId: i32) -> Option<Arc<Mutex<Shape>>> {
        let removed = self.mShapes.remove(&shapeId);
        self.mDirty = true;
        removed
    }

    // java int size()
    pub fn size(&self) -> i32 {
        // java return null != this.mFilePageContent ? this.mFilePageContent.size() : 0;
        if self.mContentLive {
            self.mShapes.len() as i32
        } else {
            0
        }
    }

    pub fn shape_count(&self) -> i32 {
        self.mShapes.len() as i32
    }

    // java Collection<? extends Shape> getShapes()
    pub fn getShapes(&self) -> Vec<Arc<Mutex<Shape>>> {
        // java return this.mShapes.valueCollection();
        self.mShapes.values().cloned().collect()
    }
}
