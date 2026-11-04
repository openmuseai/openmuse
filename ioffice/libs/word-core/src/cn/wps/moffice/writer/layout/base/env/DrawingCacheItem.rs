// Java: import cn.wps.collections.KList
use util::collections::KList::{KList, RecycableNode, RecycableNodeAlloc};
// Java: import cn.wps.moffice.drawing.Shape
use std::sync::{Arc, Mutex};
type ShapeHolder = Arc<Mutex<drawing::cn::wps::moffice::drawing::Shape::Shape>>;

// java public class DrawingCacheItem extends KList.RecycableNode
pub struct DrawingCacheItem {
    // java KList.RecycableNode super_
    pub(crate) super_: RecycableNode,
    // java public Shape shape;
    pub(crate) shape: Option<ShapeHolder>,
    // java public int cp;
    pub(crate) cp: i32,
    // java public int drawing;
    pub(crate) drawing: i32,
    // java public boolean hasFlowed;
    pub(crate) hasFlowed: bool,
    // java public int anchor;
    pub(crate) anchor: i32,
    // java public int relX;
    pub(crate) relX: i32,
    // java public int relY;
    pub(crate) relY: i32,
    // java public int docType;
    pub(crate) docType: i32,
}

// java public static class Alloc extends KList.RecycableNodeAlloc<DrawingCacheItem>
pub struct Alloc {
    // java KList.RecycableNodeAlloc<DrawingCacheItem> super_
    pub(crate) super_: RecycableNodeAlloc,
}

impl DrawingCacheItem {
    pub fn new() -> Self {
        Self {
            super_: RecycableNode::new(),
            shape: None,
            cp: -1,
            drawing: 0,
            hasFlowed: false,
            anchor: 0,
            relX: 0,
            relY: 0,
            docType: 0,
        }
    }

    // java private DrawingCacheItem()
    pub fn DrawingCacheItem() -> Self {
      Self::new()
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      self.shape = None;
      self.super_.dispose();
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      format!("cp = {}, hasFlowed = {}", self.cp, self.hasFlowed)
    }

}

impl std::ops::Deref for DrawingCacheItem {
    type Target = RecycableNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for DrawingCacheItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl Alloc {
    pub fn new() -> Self {
        Self { super_: RecycableNodeAlloc::new() }
    }

    // java public DrawingCacheItem alloc()
    pub fn alloc(&self) -> DrawingCacheItem {
      DrawingCacheItem::new()
    }

    // java public void init(DrawingCacheItem item)
    pub fn init(&self, item: &mut DrawingCacheItem) {
      item.shape = None;
      item.cp = -1;
      item.drawing = 0;
      item.hasFlowed = false;
      item.anchor = 0;
      item.relX = 0;
      item.relY = 0;
      item.docType = 0;
    }

}

impl std::ops::Deref for Alloc {
    type Target = RecycableNodeAlloc;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for Alloc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
