// Java: import cn.wps.collections.KList
use util::collections::KList::KList;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteRef
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::PLCFootnoteEndnoteRef;

// java public class FootNoteCacheItem extends KList.RecycableNode
#[derive(Default)]
pub struct FootNoteCacheItem {
    // java KList.RecycableNode super_
    pub(crate) super_: RecycableNode,
    // java public int cp;
    pub(crate) cp: i32,
    // java public String numStr;
    pub(crate) numStr: Option<String>,
    // java public int footnote;
    pub(crate) footnote: i32,
    // java public PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode refNote;
    pub(crate) refNote: Option<RefNoteHolder>,
}

// java public static class Alloc extends KList.RecycableNodeAlloc<FootNoteCacheItem>
#[derive(Default)]
pub struct Alloc {
    // java KList.RecycableNodeAlloc<FootNoteCacheItem> super_
    pub(crate) super_: RecycableNodeAlloc<FootNoteCacheItem>,
}

// java public class FootNoteCacheItem extends KList.RecycableNode
impl FootNoteCacheItem {
    // java private  FootNoteCacheItem()
    pub fn FootNoteCacheItem() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      Self { super_: RecycableNode::new(), cp: Default::default(), numStr: Default::default(), footnote: Default::default(), refNote: Default::default() }
      */
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.numStr = null;
      self.numStr = None;
      // java this.refNote = null;
      self.refNote = None;
      // java super.dispose();
      self.super_.dispose();
      */
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java StringBuffer sb = new StringBuffer();
      let mut sb: String = { let __x: String = todo!("no align strategy matched"); __x };
      // java return sb.toString();
      return sb.toString();
      */
    }

}

impl std::ops::Deref for FootNoteCacheItem {
    type Target = RecycableNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for FootNoteCacheItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class Alloc extends KList.RecycableNodeAlloc<FootNoteCacheItem>
impl Alloc {
    // java public FootNoteCacheItem alloc()
    pub fn alloc(&self) -> FootNoteCacheItem {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return new FootNoteCacheItem();
      return FootNoteCacheItem::new();
      */
    }

    // java public void init(FootNoteCacheItem node)
    pub fn init(&self, node: &mut FootNoteCacheItem) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super.init(node);
      self.super_.init(node);
      // java node.cp = -1;
      node.cp = -1;
      // java node.numStr = null;
      node.numStr = None;
      // java node.footnote = 0;
      node.footnote = 0;
      // java node.refNote = null;
      node.refNote = None;
      */
    }

}

impl std::ops::Deref for Alloc {
    type Target = RecycableNodeAlloc<FootNoteCacheItem>;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for Alloc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
