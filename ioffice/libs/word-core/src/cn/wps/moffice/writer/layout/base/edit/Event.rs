// Java: import cn.wps.collections.KList
use util::collections::KList::KList;
// Java: import cn.wps.moffice.util.BoolBitSet
use crate::cn::wps::moffice::util::BoolBitSet::BoolBitSet;
// Java: import cn.wps.moffice.util.IntBitSet
use crate::cn::wps::moffice::util::IntBitSet::IntBitSet;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.util.KeepNotProguard
use util::util::KeepNotProguard::KeepNotProguard;
// Java: import java.util.Arrays

// java public static final int EVENT_REPLACE = 12;
pub(crate) const EVENT_REPLACE: i32 = 12;
// java public static final int EVENT_MERGE_TABLE_CHANGE = 13;
pub(crate) const EVENT_MERGE_TABLE_CHANGE: i32 = 13;
// java public static final int MAX_BUF = 128;
pub(crate) const MAX_BUF: i32 = 128;
// java private static final BoolBitSet FLAG_SHAPE_NEED_REFLOW = new BoolBitSet(0);
static FLAG_SHAPE_NEED_REFLOW: std::sync::LazyLock<i32> =
                    std::sync::LazyLock::new(|| BoolBitSet::new(0));
// java private static final BoolBitSet FLAG_QUICK_FLOWABLE_DRAWING = new BoolBitSet(1);
static FLAG_QUICK_FLOWABLE_DRAWING: std::sync::LazyLock<i32> =
                    std::sync::LazyLock::new(|| BoolBitSet::new(1));
// java private static final BoolBitSet FLAG_EDIT_DRAWING_RANGE_FORMAT = new BoolBitSet(2);
static FLAG_EDIT_DRAWING_RANGE_FORMAT: std::sync::LazyLock<i32> =
                    std::sync::LazyLock::new(|| BoolBitSet::new(2));
// java private static final BoolBitSet FLAG_SHAPE_ROTATING = new BoolBitSet(3);
static FLAG_SHAPE_ROTATING: std::sync::LazyLock<i32> =
                    std::sync::LazyLock::new(|| BoolBitSet::new(3));
// java private static final IntBitSet FLAG_ANCHOR_CHPX_EVENT_COUNT = new IntBitSet(4, 6);
static FLAG_ANCHOR_CHPX_EVENT_COUNT: std::sync::LazyLock<i32> =
                    std::sync::LazyLock::new(|| IntBitSet::new(4, 6));
// java private static final BoolBitSet FLAG_SHAPE_CLIPING = new BoolBitSet(7);
static FLAG_SHAPE_CLIPING: std::sync::LazyLock<i32> =
                    std::sync::LazyLock::new(|| BoolBitSet::new(7));

// java @KeepNotProguard public class Event extends KList.RecycableNode
#[derive(Default)]
pub struct Event {
    // java KList.RecycableNode super_
    pub(crate) super_: RecycableNode,
    // java public int type;
    pub(crate) r#type: i32,
    // java public Document document;
    pub(crate) document: Option<DocumentHolder>,
    // java public int cp;
    pub(crate) cp: i32,
    // java public int length;
    pub(crate) length: i32,
    // java public int replaceLength;
    pub(crate) replaceLength: i32,
    // java public int shapeID;
    pub(crate) shapeID: i32,
    // java public char[] data;
    pub(crate) data: Vec<char>,
    // java public int mFlag;
    pub(crate) mFlag: i32,
}

// java public static class Alloc extends KList.RecycableNodeAlloc<Event>
#[derive(Default)]
pub struct Alloc {
    // java KList.RecycableNodeAlloc<Event> super_
    pub(crate) super_: RecycableNodeAlloc<Event>,
}

// java @KeepNotProguard public class Event extends KList.RecycableNode
impl Event {
    // java private  Event()
    pub fn Event() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.type = 0;
      Self { super_: RecycableNode::new(), type: 0, document: Default::default(), cp: -1, length: 0, replaceLength: 0, shapeID: -1, data: vec![Default::default(); 128usize], mFlag: 0 }
      // java this.cp = -1;
      // java this.length = 0;
      // java this.replaceLength = 0;
      // java this.shapeID = -1;
      // java this.data = new char[128];
      // java this.mFlag = 0;
      */
    }

    // java public final boolean isShapeNeedReflow()
    pub fn isShapeNeedReflow(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return FLAG_SHAPE_NEED_REFLOW.get(this.mFlag);
      return FLAG_SHAPE_NEED_REFLOW::get(self.mFlag);
      */
    }

    // java public final void setShapeNeedReflow(boolean val)
    pub fn setShapeNeedReflow(&mut self, val: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFlag = FLAG_SHAPE_NEED_REFLOW.set(this.mFlag, val);
      self.mFlag = FLAG_SHAPE_NEED_REFLOW::set(self.mFlag, val);
      */
    }

    // java public final boolean isQuickFlowableDrawing()
    pub fn isQuickFlowableDrawing(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return FLAG_QUICK_FLOWABLE_DRAWING.get(this.mFlag);
      return FLAG_QUICK_FLOWABLE_DRAWING::get(self.mFlag);
      */
    }

    // java public final void setQuickFlowableDrawing(boolean val)
    pub fn setQuickFlowableDrawing(&mut self, val: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFlag = FLAG_QUICK_FLOWABLE_DRAWING.set(this.mFlag, val);
      self.mFlag = FLAG_QUICK_FLOWABLE_DRAWING::set(self.mFlag, val);
      */
    }

    // java public final void setEditDrawingRangeFormat(boolean val)
    pub fn setEditDrawingRangeFormat(&mut self, val: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFlag = FLAG_EDIT_DRAWING_RANGE_FORMAT.set(this.mFlag, val);
      self.mFlag = FLAG_EDIT_DRAWING_RANGE_FORMAT::set(self.mFlag, val);
      */
    }

    // java public final boolean isEditDrawingRangeFormat()
    pub fn isEditDrawingRangeFormat(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return FLAG_EDIT_DRAWING_RANGE_FORMAT.get(this.mFlag);
      return FLAG_EDIT_DRAWING_RANGE_FORMAT::get(self.mFlag);
      */
    }

    // java public final void setShapeRotating(boolean val)
    pub fn setShapeRotating(&mut self, val: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFlag = FLAG_SHAPE_ROTATING.set(this.mFlag, val);
      self.mFlag = FLAG_SHAPE_ROTATING::set(self.mFlag, val);
      */
    }

    // java public final boolean isShapeRotating()
    pub fn isShapeRotating(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return FLAG_SHAPE_ROTATING.get(this.mFlag);
      return FLAG_SHAPE_ROTATING::get(self.mFlag);
      */
    }

    // java public final void setShapeCliping(boolean val)
    pub fn setShapeCliping(&mut self, val: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFlag = FLAG_SHAPE_CLIPING.set(this.mFlag, val);
      self.mFlag = FLAG_SHAPE_CLIPING::set(self.mFlag, val);
      */
    }

    // java public final boolean isShapeCliping()
    pub fn isShapeCliping(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return FLAG_SHAPE_CLIPING.get(this.mFlag);
      return FLAG_SHAPE_CLIPING::get(self.mFlag);
      */
    }

    // java public final int getAnchorChpxEventCount()
    pub fn getAnchorChpxEventCount(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return FLAG_ANCHOR_CHPX_EVENT_COUNT.get(this.mFlag);
      return FLAG_ANCHOR_CHPX_EVENT_COUNT::get(self.mFlag);
      */
    }

    // java public final void setAnchorChpxEventCount(int val)
    pub fn setAnchorChpxEventCount(&mut self, val: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFlag = FLAG_ANCHOR_CHPX_EVENT_COUNT.set(this.mFlag, val);
      self.mFlag = FLAG_ANCHOR_CHPX_EVENT_COUNT::set(self.mFlag, val);
      */
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.document = null;
      self.document = None;
      // java this.data = null;
      self.data = None;
      // java super.dispose();
      self.super_.dispose();
      */
    }

    // java public String toString()
    pub fn toString(&self) -> Option<String> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java StringBuffer strBuf = new StringBuffer();
      let mut strBuf: String = { let __x: String = todo!("no align strategy matched"); __x };
      // java strBuf.append("doc=" + this.document.getType());
      strBuf.append("doc=" + self.document.getType());
      // java strBuf.append(" type=" + this.type);
      strBuf.append(" type=" + self.r#type);
      // java strBuf.append(" [" + this.cp + ", " + (this.cp + this.length) + ")");
      strBuf.append(" [" + self.cp + ", " + self.cp + (self.len() as i32) + ")");
      // java if (this.type == 12)
      if self.r#type == 12 {
        // java strBuf.append(" replaceLen=" + this.replaceLength);
        strBuf.append(" replaceLen=" + self.replaceLength);
      }
      // java return strBuf.toString();
      return strBuf.toString();
      */
    }

}

impl std::ops::Deref for Event {
    type Target = RecycableNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for Event {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class Alloc extends KList.RecycableNodeAlloc<Event>
impl Alloc {
    // java public Event alloc()
    pub fn alloc(&self) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return new Event();
      return { let __x: Self = todo!("no align strategy matched"); __x };
      */
    }

    // java public void init(Event event)
    pub fn init(&mut self, event: Option<EventHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super.init(event);
      self.super_.init(event);
      // java event.type = 0;
      event.r#type = 0;
      // java event.document = null;
      event.document = None;
      // java event.cp = -1;
      event.cp = -1;
      // java event.length = 0;
      (event.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) = 0;
      // java event.replaceLength = 0;
      event.replaceLength = 0;
      // java event.shapeID = -1;
      event.shapeID = -1;
      // java event.mFlag = 0;
      event.mFlag = 0;
      // java Arrays.fill(event.data, '\u0000');
      Arrays::fill(event.data, '\u{0000}');
      */
    }

}

impl std::ops::Deref for Alloc {
    type Target = RecycableNodeAlloc<Event>;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for Alloc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
