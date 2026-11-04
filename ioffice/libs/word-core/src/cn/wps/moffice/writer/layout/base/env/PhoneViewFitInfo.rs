// Java: import java.util.ArrayList
// Java: import java.lang.Class
use stubs::java::lang::Class::Class;

// java public class PhoneViewFitInfo
#[derive(Default)]
pub struct PhoneViewFitInfo {
    // java private ArrayList<RangeInfo> mList = new ArrayList();
    mList: Vec<RangeInfo>,
}

// java public abstract static class RangeInfo
#[derive(Default)]
pub struct RangeInfo {
    // java public int mStartCp;
    pub(crate) mStartCp: i32,
    // java public int mEndCp;
    pub(crate) mEndCp: i32,
}

// java public class PhoneViewFitInfo
impl PhoneViewFitInfo {
    // java public void clear()
    pub fn clear(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mList.clear();
      self.mList.clear();
      */
    }

    // java public int indexForCP(int cp)
    pub fn indexForCP(&self, cp: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int size = this.mList.size();
      let mut size: i32 = (self.mList.len() as i32);
      // java int l = 0;
      let mut l: i32 = 0;
      // java int u = size - 1;
      let mut u: i32 = size - 1;
      // java while (l <= u)
      while l <= u {
        // java int mid = (l + u) / 2;
        let mut mid: i32 = l + u / 2;
        // java RangeInfo item = (RangeInfo)this.mList.get(mid);
        let mut item: RangeInfo = (self.mList.get(mid)).downcast_ref::<RangeInfo>().unwrap();
        // java if (cp >= item.mEndCp)
        if cp >= item.mEndCp {
          // java l = mid + 1;
          l = mid + 1;
        }
        // java else
        else {
          // java if (cp >= item.mStartCp)
          if cp >= item.mStartCp {
            // java return mid;
            return mid;
          }
          // java u = mid - 1;
          u = mid - 1;
        }
      }
      // java return ~l;
      return !l;
      */
    }

    // java public RangeInfo getItemByStartCp(int startCp)
    pub fn getItemByStartCp(&self, startCp: i32) -> Option<RangeInfo> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int index = this.indexForCP(startCp);
      let mut index: i32 = self.indexForCP(startCp);
      // java return index >= 0 ? (RangeInfo)this.mList.get(index) : null;
      return Some((if index >= 0 { Some((self.mList.get(index)).downcast_ref::<RangeInfo>().unwrap()) } else { None }));
      */
    }

    // java public T getItemByStartCp(int startCp, Class<T> tClass)
    pub fn getItemByStartCp__int_classt(&self, startCp: i32, tClass: &dyn std::any::Any + Send + Sync) -> Option<T> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java RangeInfo range = this.getItemByStartCp(startCp);
      let mut range: RangeInfo = self.getItemByStartCp__int(startCp).unwrap();
      // java return (T)(range != null && range.getClass() == tClass ? range : null);
      return ((if true && range.getClass() == tClass { Some(range) } else { None })).downcast_ref::<T>().unwrap();
      */
    }

    // java public T getItemOrNeighbour(int startCp, int endCp, Class<T> tClass)
    pub fn getItemOrNeighbour(&self, startCp: i32, endCp: i32, tClass: ()) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int index = this.indexForCP(startCp);
      let mut index: i32 = self.indexForCP(startCp);
      // java if (index >= 0)
      if index >= 0 {
        // java RangeInfo cur = (RangeInfo)this.mList.get(index);
        let mut cur: RangeInfo = (self.mList.get(index)).downcast_ref::<RangeInfo>().unwrap();
        // java if (cur.getClass() == tClass)
        if cur.getClass() == tClass {
          // java return (T)cur;
          return { let __x: () = todo!("no align strategy matched"); __x };
        }
      }
      // java else
      else {
        // java index = ~index;
        index = !index;
        // java int size = this.mList.size();
        let mut size: i32 = (self.mList.len() as i32);
        // java if (index > 0 && index - 1 < size)
        if index > 0 && index - 1 < size {
          // java RangeInfo pre = (RangeInfo)this.mList.get(index - 1);
          let mut pre: RangeInfo = (self.mList.get(index - 1)).downcast_ref::<RangeInfo>().unwrap();
          // java if (pre.mEndCp == startCp && pre.getClass() == tClass)
          if pre.mEndCp == startCp && pre.getClass() == tClass {
            // java return (T)pre;
            return { let __x: () = todo!("no align strategy matched"); __x };
          }
        }
        // java if (index < size)
        if index < size {
          // java RangeInfo next = (RangeInfo)this.mList.get(index);
          let mut next: RangeInfo = (self.mList.get(index)).downcast_ref::<RangeInfo>().unwrap();
          // java if (endCp == next.mStartCp && next.getClass() == tClass)
          if endCp == next.mStartCp && next.getClass() == tClass {
            // java return (T)next;
            return { let __x: () = todo!("no align strategy matched"); __x };
          }
        }
      }
      // java return null;
      return { let __x: () = todo!("no align strategy matched"); __x };
      */
    }

    // java public boolean insertItem(RangeInfo info)
    pub fn insertItem(&mut self, info: RangeInfo) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int index = this.indexForCP(info.mStartCp);
      let mut index: i32 = self.indexForCP(info.mStartCp);
      // java if (index >= 0)
      if index >= 0 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java index = ~index;
        index = !index;
        // java this.mList.add(index, info);
        self.mList.add(index, info);
        // java return true;
        return true;
      }
      */
    }

}

// java public abstract static class RangeInfo
impl RangeInfo {
    // java protected abstract boolean merge(RangeInfo var1)
    pub fn merge(&mut self, var1: &RangeInfo) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      panic!("RangeInfo::merge not implemented");
      */
    }

    // java public int indentPos()
    pub fn indentPos(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return -1;
      return -1;
      */
    }

}
