// Java: import cn.wps.collections.KPList
use util::collections::KPList::{KPList, KPNode};
// Java: import cn.wps.kfc.impl.arrayList.WIntArrayList
use util::kfc::r#impl::arrayList::WIntArrayList::WIntArrayList;
// Java: import cn.wps.moffice.writer.layout.base.graph.Graphs
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
// Java: import cn.wps.moffice.writer.layout.base.line.charborder.BdrEnv
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::line::charborder::BdrEnv::BdrEnv;
// Java: import cn.wps.io.dom.Node
// CARGO_OFF_SURFACE use io_wps_io::cn::wps::io::dom::Node::Node;
// Java: import cn.wps.moffice.writer.layout.base.line.LineCompressEnv
use crate::cn::wps::moffice::writer::layout::base::line::LineCompressEnv::LineCompressEnv;

#[derive(Default)]
pub struct BdrEnv;
impl BdrEnv {
    pub fn reset(&mut self) {}
}
pub type GraphsHolder = Graphs;
pub type LineCompressEnvHolder = LineCompressEnv;
pub type WIntArrayListHolder = WIntArrayList;
pub type BdrEnvHolder = BdrEnv;

// java public class SubLineEnv extends KPList.Node
pub struct SubLineEnv {
    // java KPList.Node super_
    pub(crate) super_: KPNode,
    // java public int left;
    pub left: i32,
    // java public int right;
    pub right: i32,
    // java public int layoutLeft;
    pub layoutLeft: i32,
    // java public int layoutRight;
    pub layoutRight: i32,
    // java public boolean isUniqueSubline;
    pub(crate) isUniqueSubline: bool,
    // java public int wrapRectType;
    pub(crate) wrapRectType: i32,
    // java public boolean enableLeftIndent;
    pub(crate) enableLeftIndent: bool,
    // java public boolean enableRightIndent;
    pub(crate) enableRightIndent: bool,
    // java public boolean atLeastOneChar;
    pub atLeastOneChar: bool,
    // java public boolean valid;
    pub(crate) valid: bool,
    // java public Graphs firstGraphs;
    pub firstGraphs: Option<GraphsHolder>,
    // java public Graphs lastGraphs;
    pub lastGraphs: Option<GraphsHolder>,
    // Java firstGraphs/lastGraphs are references to the graph-pool nodes.
    // Keep their identity alongside the legacy value snapshots until every
    // consumer can use references directly.
    pub firstGraphNode: Option<*mut Graphs>,
    pub lastGraphNode: Option<*mut Graphs>,
    // java public Graphs alignRightGraphs;
    // rust: ★wr-29 05 —— Java 对象引用 → 池节点指针
    pub(crate) alignRightGraphs: Option<*mut Graphs>,
    // java public Graphs firstGraphForExtends;
    // Java keeps the identity of the graph pool node; a cloned graph loses
    // identity when AlignDistributeHandler later visits that same node.
    pub(crate) firstGraphForExtends: Option<*mut Graphs>,
    // java public Graphs lastGraphForExtends;
    pub(crate) lastGraphForExtends: Option<*mut Graphs>,
    // java public int charsCount;
    pub(crate) charsCount: i32,
    // java public int spaceCount;
    pub(crate) spaceCount: i32,
    // java public int firstGraphIndexInCP;
    pub(crate) firstGraphIndexInCP: i32,
    // java public LineCompressEnv lineCompressEnv;
    pub lineCompressEnv: Option<LineCompressEnvHolder>,
    // java public WIntArrayList needExtendsLastGraphsForBothAlign;
    pub(crate) needExtendsLastGraphsForBothAlign: Option<WIntArrayListHolder>,
    // java public BdrEnv mBdrEnv;
    pub(crate) mBdrEnv: Option<BdrEnvHolder>,
    // java public boolean mStopBothAlign;
    pub(crate) mStopBothAlign: bool,
    // java public boolean mAutoHyphenation;
    pub(crate) mAutoHyphenation: bool,
    // java public int mHyphenationZone;
    pub(crate) mHyphenationZone: i32,
    // java public int mDrawHyphenationAfter;
    pub(crate) mDrawHyphenationAfter: i32,
    // java public int mHyphenCharWidth;
    pub(crate) mHyphenCharWidth: i32,
}

// java public static class Alloc implements KPList.INodeAlloc<SubLineEnv>
#[derive(Default)]
pub struct Alloc {
}

// java public class SubLineEnv extends KPList.Node
impl SubLineEnv {
    // java public void init()
    pub fn init(&mut self) {
      // java this.clear();
      self.clear();
    }

    // java private  SubLineEnv()
    // rust: ★wr-29 05 —— 真实（Java 私有 ctor；firstGraphs/lastGraphs Java null）
    pub fn SubLineEnv() -> Self {
      Self::__new()
    }

    fn __new() -> Self {
      SubLineEnv {
        super_: KPNode::new(),
        left: 0, right: 0, layoutLeft: 0, layoutRight: 0, isUniqueSubline: false, wrapRectType: 0,
        enableLeftIndent: true, enableRightIndent: true, atLeastOneChar: true, valid: false,
        firstGraphs: None, lastGraphs: None, firstGraphNode: None, lastGraphNode: None,
        alignRightGraphs: None,
        firstGraphForExtends: None, lastGraphForExtends: None,
        charsCount: 0, spaceCount: 0, firstGraphIndexInCP: 0,
        lineCompressEnv: Some(LineCompressEnv::LineCompressEnv()),
        needExtendsLastGraphsForBothAlign: Some(WIntArrayList::new()),
        mBdrEnv: Some(BdrEnv), mStopBothAlign: false, mAutoHyphenation: false,
        mHyphenationZone: 0, mDrawHyphenationAfter: -1, mHyphenCharWidth: 0,
      }
    }

    // java public void clear()
    pub fn clear(&mut self) {
      // java this.left = 0;
      self.left = 0;
      // java this.right = 0;
      self.right = 0;
      // java this.layoutLeft = 0;
      self.layoutLeft = 0;
      // java this.layoutRight = 0;
      self.layoutRight = 0;
      // java this.isUniqueSubline = false;
      self.isUniqueSubline = false;
      // java this.wrapRectType = 0;
      self.wrapRectType = 0;
      // java this.enableLeftIndent = true;
      self.enableLeftIndent = true;
      // java this.enableRightIndent = true;
      self.enableRightIndent = true;
      // java this.atLeastOneChar = true;
      self.atLeastOneChar = true;
      // java this.valid = false;
      self.valid = false;
      // java this.charsCount = 0;
      self.charsCount = 0;
      // java this.spaceCount = 0;
      self.spaceCount = 0;
      // java this.firstGraphs = null;
      self.firstGraphs = None;
      // java this.lastGraphs = null;
      self.lastGraphs = None;
      self.firstGraphNode = None;
      self.lastGraphNode = None;
      // java this.alignRightGraphs = null;
      self.alignRightGraphs = None;
      // java this.firstGraphForExtends = null;
      self.firstGraphForExtends = None;
      // java this.lastGraphForExtends = null;
      self.lastGraphForExtends = None;
      // java this.firstGraphIndexInCP = 0;
      self.firstGraphIndexInCP = 0;
      // java this.lineCompressEnv.reset();
      if let Some(value) = self.lineCompressEnv.as_mut() { value.reset(); }
      // java this.needExtendsLastGraphsForBothAlign.resetQuick();
      if let Some(value) = self.needExtendsLastGraphsForBothAlign.as_mut() { value.resetQuick(); }
      // java this.mBdrEnv.reset();
      if let Some(value) = self.mBdrEnv.as_mut() { value.reset(); }
      // java this.mStopBothAlign = false;
      self.mStopBothAlign = false;
      // java this.mAutoHyphenation = false;
      self.mAutoHyphenation = false;
      // java this.mHyphenationZone = 0;
      self.mHyphenationZone = 0;
      // java this.mDrawHyphenationAfter = -1;
      self.mDrawHyphenationAfter = -1;
      // java this.mHyphenCharWidth = 0;
      self.mHyphenCharWidth = 0;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.firstGraphs = null;
      self.firstGraphs = None;
      // java this.lastGraphs = null;
      self.lastGraphs = None;
      self.firstGraphNode = None;
      self.lastGraphNode = None;
      // java this.alignRightGraphs = null;
      self.alignRightGraphs = None;
      // java this.firstGraphForExtends = null;
      self.firstGraphForExtends = None;
      // java this.lastGraphForExtends = null;
      self.lastGraphForExtends = None;
      // java if (null != this.lineCompressEnv)
      if let Some(lineCompressEnv) = self.lineCompressEnv.as_mut() {
        // java this.lineCompressEnv.dispose();
        lineCompressEnv.dispose();
        // java this.lineCompressEnv = null;
      }
      self.lineCompressEnv = None;
      // java super.dispose();
      self.super_.dispose();
    }

    // java public boolean hasGraphs()
    pub fn hasGraphs(&self) -> bool {
      // java return null != this.lastGraphs;
      self.lastGraphs.is_some()
    }

    // java public void addGraphs(Graphs run)
    // rust: ★wr-29 05 —— 真实；run 池节点 → first/last 引用语义（拷贝镜像）
    pub fn addGraphs(&mut self, run: *mut Graphs) {
      // java if (null == this.firstGraphs)
      if self.firstGraphs.is_none() {
        // java this.firstGraphs = run;
        self.firstGraphs = Some(Clone::clone(unsafe { &*run }));
        self.firstGraphNode = Some(run);
      }
      // java this.lastGraphs = run;
      self.lastGraphs = Some(Clone::clone(unsafe { &*run }));
      self.lastGraphNode = Some(run);
    }

}

impl std::ops::Deref for SubLineEnv {
    type Target = KPNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for SubLineEnv {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl Default for SubLineEnv {
    fn default() -> Self {
        Self {
            super_: KPNode::new(),
            left: 0,
            right: 0,
            layoutLeft: 0,
            layoutRight: 0,
            isUniqueSubline: false,
            wrapRectType: 0,
            enableLeftIndent: true,
            enableRightIndent: true,
            atLeastOneChar: true,
            valid: false,
            firstGraphs: None,
            lastGraphs: None,
            firstGraphNode: None,
            lastGraphNode: None,
            alignRightGraphs: None,
            firstGraphForExtends: None,
            lastGraphForExtends: None,
            charsCount: 0,
            spaceCount: 0,
            firstGraphIndexInCP: 0,
            lineCompressEnv: Some(LineCompressEnv::LineCompressEnv()),
            needExtendsLastGraphsForBothAlign: None,
            mBdrEnv: None,
            mStopBothAlign: false,
            mAutoHyphenation: false,
            mHyphenationZone: 0,
            mDrawHyphenationAfter: -1,
            mHyphenCharWidth: 0,
        }
    }
}

// java public static class Alloc implements KPList.INodeAlloc<SubLineEnv>
impl Alloc {
    // java public SubLineEnv alloc()
    pub fn alloc(&self) -> SubLineEnv {
      // java return new SubLineEnv();
      SubLineEnv::SubLineEnv()
    }

}

impl Clone for SubLineEnv {
    fn clone(&self) -> Self {
        // rust: mirror 深拷贝（super_ 链表不链）
        let mut c = SubLineEnv::SubLineEnv();
        c.left = self.left; c.right = self.right; c.layoutLeft = self.layoutLeft; c.layoutRight = self.layoutRight;
        c.isUniqueSubline = self.isUniqueSubline; c.wrapRectType = self.wrapRectType;
        c.enableLeftIndent = self.enableLeftIndent; c.enableRightIndent = self.enableRightIndent;
        c.atLeastOneChar = self.atLeastOneChar; c.valid = self.valid;
        c.firstGraphs = self.firstGraphs.clone(); c.lastGraphs = self.lastGraphs.clone();
        c.firstGraphNode = self.firstGraphNode; c.lastGraphNode = self.lastGraphNode;
        c.alignRightGraphs = self.alignRightGraphs;
        c.firstGraphForExtends = self.firstGraphForExtends.clone(); c.lastGraphForExtends = self.lastGraphForExtends.clone();
        c.charsCount = self.charsCount; c.spaceCount = self.spaceCount;
        c.firstGraphIndexInCP = self.firstGraphIndexInCP;
        c.lineCompressEnv = None;
        c.needExtendsLastGraphsForBothAlign = None;
        c.mBdrEnv = None;
        c.mStopBothAlign = self.mStopBothAlign; c.mAutoHyphenation = self.mAutoHyphenation;
        c.mHyphenationZone = self.mHyphenationZone; c.mDrawHyphenationAfter = self.mDrawHyphenationAfter;
        c.mHyphenCharWidth = self.mHyphenCharWidth;
        c
    }
}
