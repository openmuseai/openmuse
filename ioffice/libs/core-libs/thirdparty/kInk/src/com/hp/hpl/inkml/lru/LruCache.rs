use std::collections::HashMap;
use std::sync::Mutex;

// Java: public class LruCache
pub struct LruCache<T: Clone + Eq + std::hash::Hash> {
    // Java field: Model<T> head
    pub(crate) head: Option<usize>,
    // Java field: Model<T> end
    end: Option<usize>,
    // Java field: Map<T, Model<T>> map
    map: HashMap<T, usize>,
    // Java field: int size
    size: i32,
    // Java field: ReentrantLock lock
    lock: Mutex<()>,
    nodes: Vec<Model<T>>,
}

// Java: public class Model
pub struct Model<T: Clone> {
    // Java field: T mEntry
    pub mEntry: T,
    // Java field: Model<T> pre
    pub pre: Option<usize>,
    // Java field: Model<T> next
    pub next: Option<usize>,
}

// Java: public class LruCache
impl<T: Clone + Eq + std::hash::Hash> LruCache<T> {
    // Java: LruCache()
    pub fn LruCache() -> Self {
      // Java: super();
      Self {
          head: None,
          end: None,
          map: HashMap::new(),
          size: 0,
          lock: Mutex::new(()),
          nodes: Vec::new(),
      }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: ReentrantLock lock = this.lock;
      // Java: lock.lock();
      let _guard = self.lock.lock().unwrap();
      // Java: int var2;
      let var2;
      // Java: try
      // Java: var2 = this.size;
      var2 = self.size;
      // Java: finally
      // Java: lock.unlock();
      // Java: return var2;
      var2
    }

    // Java: T add(T o)
    pub fn add(&mut self, o: T) -> Option<T> {
      // Java: ReentrantLock lock = this.lock;
      // Java: lock.lock();
      // Java: try
      // Java: if (o != null)
      {
         // Java: Model<T> model = (Model)this.map.get(o);
         // Java: if (model != null)
         if let Some(&idx) = self.map.get(&o) {
            // Java: this.addExistence(model);
            self.addExistence(idx);
            // Java: Object var9 = null;
            // Java: return (T)var9;
            return None;
         }
         // Java: Object var4 = this.addNoExistence(o);
         let var4 = self.addNoExistence(o);
         // Java: return (T)var4;
         return var4;
      }
      // Java: return null;
      // Java: finally
      // Java: lock.unlock();
    }

    // Java: Model<T> getHead()
    pub fn getHead(&self) -> Option<&Model<T>> {
      // Java: return this.head;
      self.head.map(|idx| &self.nodes[idx])
    }

    // Java: T addNoExistence(T o)
    fn addNoExistence(&mut self, o: T) -> Option<T> {
      // Java: Model<T> model = new Model<T>(o);
      let model = Model { mEntry: o.clone(), pre: None, next: None };
      let model_idx = self.nodes.len();
      self.nodes.push(model);
      // Java: this.map.put(o, model);
      self.map.insert(o, model_idx);
      // Java: if (this.size == 0)
      if self.size == 0
      {
         // Java: this.head = model;
         self.head = Some(model_idx);
         // Java: ++this.size;
         self.size += 1;
         // Java: return null;
         return None;
      }
      // Java: else if (this.size == 1)
      else if self.size == 1
      {
         // Java: this.head.next = model;
         if let Some(head_idx) = self.head {
             self.nodes[head_idx].next = Some(model_idx);
         }
         // Java: model.pre = this.head;
         self.nodes[model_idx].pre = self.head;
         // Java: this.end = model;
         self.end = Some(model_idx);
         // Java: ++this.size;
         self.size += 1;
         // Java: return null;
         return None;
      }
      // Java: else if (!this.isExceedMemory())
      else if !self.isExceedMemory()
      {
         // Java: this.end.next = model;
         if let Some(end_idx) = self.end {
             self.nodes[end_idx].next = Some(model_idx);
         }
         // Java: model.pre = this.end;
         self.nodes[model_idx].pre = self.end;
         // Java: this.end = model;
         self.end = Some(model_idx);
         // Java: ++this.size;
         self.size += 1;
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: Model<T> temp = this.head;
         let temp_idx = self.head.unwrap();
         // Java: this.head = temp.next;
         self.head = self.nodes[temp_idx].next;
         // Java: temp.next = null;
         self.nodes[temp_idx].next = None;
         // Java: temp.pre = null;
         self.nodes[temp_idx].pre = None;
         // Java: this.end.next = model;
         if let Some(end_idx) = self.end {
             self.nodes[end_idx].next = Some(model_idx);
         }
         // Java: model.pre = this.end;
         self.nodes[model_idx].pre = self.end;
         // Java: this.end = model;
         self.end = Some(model_idx);
         // Java: this.map.put(temp.getEntry(), null);
         let evicted_entry = self.nodes[temp_idx].mEntry.clone();
         self.map.remove(&evicted_entry);
         // Java: return temp.mEntry;
         return Some(evicted_entry);
      }
    }

    // Java: boolean isExceedMemory()
    pub fn isExceedMemory(&self) -> bool {
      // Java: Runtime runtime = Runtime.getRuntime();
      // Java: return runtime.maxMemory() - runtime.totalMemory() + runtime.freeMemory() < 125829120L;
      false
    }

    // Java: void addExistence(Model<T> model)
    fn addExistence(&mut self, model_idx: usize) {
      // Java: if (this.size != 1 && model != this.end)
      if self.size != 1 && Some(model_idx) != self.end
      {
         // Java: Model<T> modelPre = model.pre;
         let model_pre = self.nodes[model_idx].pre;
         // Java: Model<T> modelNext = model.next;
         let model_next = self.nodes[model_idx].next;
         // Java: if (modelPre != null)
         if let Some(pre_idx) = model_pre
         {
            // Java: modelPre.next = modelNext;
            self.nodes[pre_idx].next = model_next;
         }
         // Java: else
         else
         {
            // Java: this.head = modelNext;
            self.head = model_next;
         }
         // Java: if (modelNext != null)
         if let Some(next_idx) = model_next
         {
            // Java: modelNext.pre = modelPre;
            self.nodes[next_idx].pre = model_pre;
         }
         // Java: model.next = null;
         self.nodes[model_idx].next = None;
         // Java: model.pre = this.end;
         self.nodes[model_idx].pre = self.end;
         // Java: this.end.next = model;
         if let Some(end_idx) = self.end {
             self.nodes[end_idx].next = Some(model_idx);
         }
         // Java: this.end = model;
         self.end = Some(model_idx);
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.lock.lock();
      // Java: try
      {
         // Java: this.map.clear();
         self.map.clear();
         // Java: this.head = null;
         self.head = None;
         // Java: this.end = null;
         self.end = None;
         // Java: this.size = 0;
         self.size = 0;
         self.nodes.clear();
      }
      // Java: finally
      {
         // Java: this.lock.unlock();
      }
    }

}

// Java: public class Model
impl<T: Clone> Model<T> {
    // Java: Model(T entry)
    pub fn Model(entry: T) -> Self {
         // Java: super();
         // Java: this.mEntry = entry;
         Self { mEntry: entry, pre: None, next: None }
    }

    // Java: T getEntry()
    pub fn getEntry(&self) -> &T {
         // Java: return this.mEntry;
         &self.mEntry
    }

    // Java: Model<T> getNext()
    pub fn getNext(&self) -> Option<usize> {
         // Java: return this.next;
         self.next
    }

}
