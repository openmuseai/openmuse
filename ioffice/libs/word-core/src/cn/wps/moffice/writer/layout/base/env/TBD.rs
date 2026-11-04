// Java: import cn.wps.collections.KList
use util::collections::KList::{KList, RecycableNode, RecycableNodeAlloc, KListNode};

// java public class TBD extends KList.RecycableNode
#[derive()]
pub struct TBD {
    // java KList.RecycableNode super_
    pub(crate) super_: RecycableNode,
    // java public int tabJC;
    pub(crate) tabJC: i32,
    // java public int tabFontCharType;
    pub(crate) tabFontCharType: i32,
    // java public int tabStopPos;
    pub(crate) tabStopPos: i32,
}

// java public static class Alloc extends KList.RecycableNodeAlloc<TBD>
#[derive()]
pub struct Alloc {
    // java KList.RecycableNodeAlloc<TBD> super_
    pub(crate) super_: RecycableNodeAlloc,
}

// java public class TBD extends KList.RecycableNode
impl TBD {
    // java private  TBD()
    pub fn TBD() -> Self {
      // java this.tabJC = 0;
      Self { super_: RecycableNode::new(), tabJC: 0, tabFontCharType: 0, tabStopPos: Default::default() }
      // java this.tabFontCharType = 0;
    }

}

impl std::ops::Deref for TBD {
    type Target = RecycableNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TBD {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class Alloc extends KList.RecycableNodeAlloc<TBD>
impl Alloc {
    // java public TBD alloc()
    pub fn alloc(&self) -> TBD {
      // java return new TBD();
      TBD::TBD()
    }

    // java public void init(TBD tbd)
    pub fn init(&self, tbd: &mut TBD) {
      // java super.init(tbd);
      tbd.super_ = RecycableNode::new();
      // java tbd.tabJC = 0;
      tbd.tabJC = 0;
      // java tbd.tabFontCharType = 0;
      tbd.tabFontCharType = 0;
      // java tbd.tabStopPos = 0;
      tbd.tabStopPos = 0;
    }

}

impl Default for TBD {
    fn default() -> Self {
        Self {
            super_: RecycableNode {
                base: KListNode {
                    next: std::ptr::null_mut(),
                    prev: std::ptr::null_mut(),
                    is_head: false,
                },
            },
            tabJC: 0,
            tabFontCharType: 0,
            tabStopPos: 0,
        }
    }
}

impl Default for Alloc {
    fn default() -> Self {
        Self {
            super_: RecycableNodeAlloc {},
        }
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
