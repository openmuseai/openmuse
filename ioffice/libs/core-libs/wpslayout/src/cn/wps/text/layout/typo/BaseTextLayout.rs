// Java: import android.graphics.Paint
// Java: import android.graphics.PointF
// Java: import android.graphics.RectF
// Java: import android.text.TextPaint
// Java: import cn.wps.text.layout.ITextLayout
// Java: import cn.wps.text.layout.data.ParaInfo
// Java: import cn.wps.text.layout.data.TextBoxData
// Java: import java.util.ArrayList
// Java: import java.util.List

use std::sync::atomic::{AtomicI32, Ordering};

static ID_BASE: AtomicI32 = AtomicI32::new(i32::MIN);

#[derive(Clone)]
pub struct PointF { pub x: f32, pub y: f32 }

pub struct RectF { pub left: f32, pub top: f32, pub right: f32, pub bottom: f32 }
impl RectF {
    pub fn new() -> Self { RectF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 } }
}

pub struct TextPaint;

// Java field: int NONE
pub const NONE: i32 = 0;
// Java field: int DOC
pub const DOC: i32 = 1;
// Java field: int DOCX
pub const DOCX: i32 = 2;
// Java field: float ITALIC_MARGIN_FACTOR
pub(crate) const ITALIC_MARGIN_FACTOR: f32 = 0.2;

// Java: public abstract class BaseTextLayout implements ITextLayout
pub struct BaseTextLayout {
    // Java field: int mID
    pub(crate) mID: i32,
    // Java field: List<PointF> mTypoOffsets
    pub(crate) mTypoOffsets: Vec<PointF>,
    // Java field: TextPaint mTextPaint
    pub(crate) mTextPaint: TextPaint,
    // Java field: TextBoxData mTextBoxData
    pub(crate) mTextBoxData: Option<()>,
    // Java field: RectF mTextRect
    pub(crate) mTextRect: RectF,
    // Java field: RectF mTypoRect
    pub(crate) mTypoRect: RectF,
    // Java field: float mTypoMaxWidth
    pub(crate) mTypoMaxWidth: f32,
    // Java field: float mTypoMaxHeight
    pub(crate) mTypoMaxHeight: f32,
    // Java field: int mTypoState
    pub(crate) mTypoState: i32,
}

// Java: public abstract class BaseTextLayout implements ITextLayout
impl BaseTextLayout {
    // Java: BaseTextLayout()
    pub fn new() -> Self {
      // Java: super();
      BaseTextLayout {
          mID: ID_BASE.fetch_add(1, Ordering::Relaxed),
          mTypoOffsets: Vec::new(),
          mTextPaint: TextPaint,
          mTextBoxData: None,
          mTextRect: RectF::new(),
          mTypoRect: RectF::new(),
          mTypoMaxWidth: i32::MAX as f32,
          mTypoMaxHeight: i32::MAX as f32,
          mTypoState: -1,
      }
    }

    pub fn id(&self) -> i32 { self.mID }
    pub fn state(&self) -> i32 { self.mTypoState }
}
