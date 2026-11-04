// Java: import android.graphics.Typeface
// Java: import cn.wps.text.layout.data.SpanInfo
// Java: import java.util.Map
// Java: import java.util.WeakHashMap
use crate::cn::wps::text::layout::data::SpanInfo::SpanInfo;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub struct Typeface;

// Java field: Map<String, Typeface> mStringTypeface
pub static M_STRING_TYPEFACE: OnceLock<Mutex<HashMap<String, Typeface>>> = OnceLock::new();

// Java field: short SPAN_INITIAL_SIZE
pub(crate) const SPAN_INITIAL_SIZE: i16 = 3;

// Java field: short SPAN_INCREASE_SIZE
pub(crate) const SPAN_INCREASE_SIZE: i16 = 2;

// Java field: DataAllocator allocator
static ALLOCATOR: OnceLock<Mutex<DataAllocator>> = OnceLock::new();

fn get_allocator() -> &'static Mutex<DataAllocator> {
    ALLOCATOR.get_or_init(|| Mutex::new(DataAllocator::new_internal()))
}

// Java: public class DataAllocator
pub struct DataAllocator {
    // Java field: SpanInfo[] mSpanPool
    mSpanPool: Vec<Option<SpanInfo>>,
    // Java field: int mSpanPoolSize
    mSpanPoolSize: i32,
}

// Java: public class DataAllocator
impl DataAllocator {
    // Java: static DataAllocator getInstance()
    pub fn getInstance() -> &'static Mutex<DataAllocator> {
      // Java: if (allocator == null)
      // Java: allocator = new DataAllocator();
      // Java: return allocator;
      get_allocator()
    }

    // Java: DataAllocator()
    fn new_internal() -> Self {
      // Java: super();
      let pool_size = SPAN_INITIAL_SIZE as i32;
      let mut pool: Vec<Option<SpanInfo>> = Vec::with_capacity(pool_size as usize);
      // Java: for(int i = 0; i < this.mSpanPoolSize; ++i)
      for _i in 0..pool_size
      {
         // Java: this.mSpanPool[i] = new SpanInfo();
         pool.push(Some(SpanInfo::new()));
      }
      DataAllocator { mSpanPool: pool, mSpanPoolSize: pool_size }
    }

    // Java: SpanInfo allocSpan()
    pub fn allocSpan(&mut self) -> SpanInfo {
      // Java: for(int i = this.mSpanPoolSize - 1; i >= 0; --i)
      for i in (0..self.mSpanPoolSize as usize).rev()
      {
         // Java: if (this.mSpanPool[i] != null)
         if self.mSpanPool[i].is_some()
         {
            // Java: SpanInfo span = this.mSpanPool[i];
            // Java: this.mSpanPool[i] = null;
            let span = self.mSpanPool[i].take().unwrap();
            // Java: return span;
            return span;
         }
      }
      // Java: return new SpanInfo();
      SpanInfo::new()
    }

    // Java: void disposeSpan(SpanInfo spanInfo)
    pub fn disposeSpan(&mut self, spanInfo: SpanInfo) {
      // Java: for(int i = this.mSpanPoolSize - 1; i >= 0; --i)
      for i in (0..self.mSpanPoolSize as usize).rev()
      {
         // Java: if (this.mSpanPool[i] == null)
         if self.mSpanPool[i].is_none()
         {
            // Java: this.mSpanPool[i] = spanInfo;
            self.mSpanPool[i] = Some(spanInfo);
            // Java: return;
            return;
         }
      }
      // Java: SpanInfo[] pool = new SpanInfo[this.mSpanPoolSize + 2];
      let new_size = self.mSpanPoolSize + SPAN_INCREASE_SIZE as i32;
      let mut pool: Vec<Option<SpanInfo>> = Vec::with_capacity(new_size as usize);
      // Java: System.arraycopy(this.mSpanPool, 0, pool, 0, this.mSpanPoolSize);
      for item in self.mSpanPool.drain(..) {
          pool.push(item);
      }
      // Java: pool[this.mSpanPoolSize] = spanInfo;
      pool.push(Some(spanInfo));
      // Java: for(int i = this.mSpanPoolSize + 1; i < pool.length; ++i)
      while pool.len() < new_size as usize
      {
         // Java: pool[i] = new SpanInfo();
         pool.push(Some(SpanInfo::new()));
      }
      // Java: this.mSpanPool = pool;
      self.mSpanPool = pool;
      // Java: this.mSpanPoolSize = this.mSpanPool.length;
      self.mSpanPoolSize = self.mSpanPool.len() as i32;
    }

}
