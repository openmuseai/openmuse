#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

use std::collections::{HashMap, VecDeque};

// Java: import cn.wps.collections.KList
// Java: import java.util.HashMap
// Java: import java.util.Map

use crate::cn::wps::graphics::bitmap::PictureMemoryPool::PictureMemoryPool;

pub struct BitmapRefNode {
    pub bmpRef: i32,
}

pub struct KList<T> {
    list: VecDeque<T>,
}
impl<T> KList<T> {
    pub fn new() -> Self { KList { list: VecDeque::new() } }
    pub fn dispose(&mut self) { self.list.clear(); }
    pub fn addLast(&mut self, _item: T) {}
    pub fn remove(&mut self, _item: &T) {}
    pub fn isEmpty(&self) -> bool { self.list.is_empty() }
    pub fn firstElement(&self) -> Option<&T> { self.list.front() }
}

// Java: public class BitmapLruManager extends PictureMemoryPool
pub struct BitmapLruManager {
    pub base: PictureMemoryPool,
    // Java field: Map<String, BitmapRefNode> bmpMap
    bmpMap: Option<HashMap<String, BitmapRefNode>>,
    // Java field: KList<BitmapRefNode> bmpQueue
    bmpQueue: Option<KList<BitmapRefNode>>,
}

// Java: public class BitmapLruManager extends PictureMemoryPool
impl BitmapLruManager {
    // Java: BitmapLruManager()
    pub fn new() -> Self {
      // Java: super();
      BitmapLruManager {
          base: PictureMemoryPool::new(),
          bmpMap: Some(HashMap::new()),
          bmpQueue: Some(KList::new()),
      }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: if (null != this.bmpQueue)
      if self.bmpQueue.is_some()
      {
         // Java: this.bmpQueue.dispose();
         if let Some(ref mut queue) = self.bmpQueue {
             queue.dispose();
         }
         // Java: this.bmpQueue = null;
         self.bmpQueue = None;
      }
      // Java: if (null != this.bmpMap)
      if self.bmpMap.is_some()
      {
         // Java: this.bmpMap.clear();
         if let Some(ref mut map) = self.bmpMap {
             map.clear();
         }
         // Java: this.bmpMap = null;
         self.bmpMap = None;
      }
      // Java: if (null != this.reuseBitmap)
      {
         // Java: this.reuseBitmap.clear();
         self.base.reuseBitmap.clear();
      }
      // Java: this.currentBmpSize = 0;
      self.base.currentBmpSize = 0;
      // Java: this.reuseBmpSize = 0;
      self.base.reuseBmpSize = 0;
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.recycle();
      self.recycle();
      // Java: this.currentBmpSize = 0;
      self.base.currentBmpSize = 0;
    }

    // Java: void recycle()
    pub fn recycle(&mut self) {
      // Java: if (null != this.bmpMap)
      if self.bmpMap.is_some()
      {
         // Java: this.bmpMap.clear();
         if let Some(ref mut map) = self.bmpMap {
             map.clear();
         }
      }
      // Java: if (null != this.bmpQueue)
      if self.bmpQueue.is_some()
      {
         // Java: this.bmpQueue.dispose();
         if let Some(ref mut queue) = self.bmpQueue {
             queue.dispose();
         }
         // Java: this.bmpQueue = new KList();
         self.bmpQueue = Some(KList::new());
      }
      // Java: if (null != this.reuseBitmap)
      {
         // Java: this.reuseBitmap.clear();
         self.base.reuseBitmap.clear();
         // Java: this.reuseBmpSize = 0;
         self.base.reuseBmpSize = 0;
      }
    }

    // Java: BitmapRefNode getBitmapRefNode(String key)
    pub fn getBitmapRefNode(&self, key: &str) -> Option<&BitmapRefNode> {
      // Java: BitmapRefNode node = null != this.bmpMap ? (BitmapRefNode)this.bmpMap.get(key) : null;
      let node = self.bmpMap.as_ref().and_then(|map| map.get(key));
      // Java: if (null != node)
      if node.is_some()
      {
         // Java: this.bmpQueue.remove(node);
         // Java: this.bmpQueue.addLast(node);
      }
      // Java: return node;
      node
    }

    // Java: boolean add(String key, BitmapRefNode node, int size)
    pub fn add(&mut self, key: String, node: BitmapRefNode, size: i32) -> bool {
      // Java: this.check(size);
      self.check(size);
      // Java: this.bmpMap.put(key, node);
      if let Some(ref mut map) = self.bmpMap {
          map.insert(key, node);
      }
      // Java: this.bmpQueue.addLast(node);
      // Java: this.currentBmpSize += size;
      self.base.currentBmpSize += size;
      // Java: return true;
      true
    }

    // Java: boolean remove(BitmapRefNode node, boolean addDiskCache)
    pub fn remove_node(&mut self, _node: &BitmapRefNode, addDiskCache: bool) -> bool {
      // Java: IBitmapRef ref = node.bmpRef;
      // Java: this.bmpQueue.remove(node);
      // Java: this.bmpMap.remove(ref.getPicture().getPath());
      // Java: return this.recycleBmp(ref, addDiskCache);
      self.base.recycleBmp(0, addDiskCache)
    }

    // Java: boolean check(int requestSize)
    pub fn check(&mut self, requestSize: i32) -> bool {
      // Java: return this.check(requestSize, true);
      self.check_with_reuse(requestSize, true)
    }

    // Java: boolean check(int requestSize, boolean bReuse)
    pub fn check_with_reuse(&mut self, requestSize: i32, bReuse: bool) -> bool {
      // Java: if (requestSize > this.maxMemorySize)
      if requestSize > self.base.maxMemorySize
      {
         // Java: return false;
         return false;
      }
      // Java: else
      else
      {
         // Java: if (bReuse && this.currentBmpSize + requestSize + this.reuseBmpSize > this.maxMemorySize)
         if bReuse && self.base.currentBmpSize + requestSize + self.base.reuseBmpSize > self.base.maxMemorySize
         {
            // Java: this.reuseBitmap.clear();
            self.base.reuseBitmap.clear();
            // Java: this.reuseBmpSize = 0;
            self.base.reuseBmpSize = 0;
         }
         // Java: while(this.currentBmpSize + requestSize > this.maxMemorySize && !this.bmpQueue.isEmpty())
         while self.base.currentBmpSize + requestSize > self.base.maxMemorySize
             && self.bmpQueue.as_ref().map_or(true, |q| !q.isEmpty())
         {
            // Java: BitmapRefNode firstNode = (BitmapRefNode)this.bmpQueue.firstElement();
            // Java: if (null != firstNode)
            {
               // Java: this.remove(firstNode, true);
            }
            break;
         }
         // Java: return true;
         return true;
      }
    }

}
