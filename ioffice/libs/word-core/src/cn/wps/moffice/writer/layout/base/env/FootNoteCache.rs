// Java: import cn.wps.collections.KList
use util::collections::KList::KList;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteRef
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::PLCFootnoteEndnoteRef;
// Java: import cn.wps.moffice.writer.layout.base.env.FootNoteCacheItem
use crate::cn::wps::moffice::writer::layout::base::env::FootNoteCacheItem::FootNoteCacheItem;
// Java: import cn.wps.moffice.writer.layout.base.env.Pools
use crate::cn::wps::moffice::writer::layout::base::env::Pools::Pools;

// java public class FootNoteCache extends KList<FootNoteCacheItem>
#[derive(Default)]
pub struct FootNoteCache {
    // java KList<FootNoteCacheItem> super_
    pub(crate) super_: KList<FootNoteCacheItem>,
    // java private Pools mPools;
    mPools: Option<Pools>,
}

// java public class FootNoteCache extends KList<FootNoteCacheItem>
impl FootNoteCache {
    // java public  FootNoteCache(Pools pools)
    pub fn FootNoteCache(pools: &Pools) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mPools = pools;
      Self { super_: KList<FootNoteCacheItem>::new(), mPools: pools }
      */
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mPools = null;
      self.mPools = None;
      // java super.dispose();
      self.super_.dispose();
      */
    }

    // java public void clear()
    pub fn clear(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mPools.recycleFootEndNoteCacheItems(this);
      self.mPools.recycleFootEndNoteCacheItems(self);
      */
    }

    // java public FootNoteCacheItem addFootEndNoteCache(int cp, PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode footEndNote, String numStr)
    pub fn addFootEndNoteCache(&mut self, cp: i32, footEndNote: ()) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java FootNoteCacheItem footNoteCache = this.createItem(cp, footEndNote, numStr);
      let mut footNoteCache: FootNoteCacheItem = self.createItem(cp, footEndNote, numStr);
      // java if (this.isEmpty())
      if self.isEmpty() {
        // java this.add(footNoteCache);
        self.add(footNoteCache);
      }
      // java else
      else {
        // java int size = this.size();
        let mut size: i32 = self.size();
        // java int pos = 0;
        let mut pos: i32 = 0;
        let mut i: i32 = size - 1;
        // java for(int i = size - 1; i >= 0 --i)
        while i >= 0 {
          // java FootNoteCacheItem item = (FootNoteCacheItem)this.get(i);
          let mut item: FootNoteCacheItem = (self.get(i)).downcast_ref::<FootNoteCacheItem>().unwrap();
          // java if (item.cp <= cp)
          if item.cp <= cp {
            // java pos = i + 1;
            pos = i + 1;
            // java break;
            break;
          }
        }
        // java this.add(pos, footNoteCache);
        self.add(pos, footNoteCache);
      }
      // java return footNoteCache;
      return { let __x: () = todo!("no align strategy matched"); __x };
      */
    }

    // java private FootNoteCacheItem createItem(int cp, PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode footEndNote, String numStr)
    pub fn createItem(&mut self, cp: i32, footEndNote: ()) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java FootNoteCacheItem item = this.mPools.obtainFootEndCacheItem();
      let mut item: FootNoteCacheItem = self.mPools.obtainFootEndCacheItem();
      // java item.cp = cp;
      item.cp = cp;
      // java item.numStr = numStr;
      item.numStr = numStr;
      // java item.refNote = footEndNote;
      item.refNote = footEndNote;
      // java return item;
      return { let __x: () = todo!("no align strategy matched"); __x };
      */
    }

    // java public void removeFootEndNoteCache(int startCP, int endCP)
    pub fn removeFootEndNoteCache(&mut self, startCP: i32, endCP: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.isEmpty())
      if !self.isEmpty() {
        // java KList.Iterator<FootNoteCacheItem> it = this.iterator();
        let mut it: Vec<FootNoteCacheItem> = self.iterator();
        // java it.seekBegin();
        it.seekBegin();
        // java while (!it.isEnd())
        while !it.isEnd() {
          // java FootNoteCacheItem item = (FootNoteCacheItem)it.next();
          let mut item: FootNoteCacheItem = (it.next()).downcast_ref::<FootNoteCacheItem>().unwrap();
          // java if (item.cp >= startCP && item.cp < endCP)
          if item.cp >= startCP && item.cp < endCP {
            // java this.mPools.recycleFootEndNoteCacheItem((FootNoteCacheItem)this.remove(item));
            self.mPools.recycleFootEndNoteCacheItem((self.remove(item)).downcast_ref::<FootNoteCacheItem>().unwrap());
          }
        }
        // java it.recycle();
        it.recycle();
      }
      */
    }

}

impl std::ops::Deref for FootNoteCache {
    type Target = KList<FootNoteCacheItem>;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for FootNoteCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
