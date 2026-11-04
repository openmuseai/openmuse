// Java: public class WritingPoint
pub struct WritingPoint {
    // Java field: float x
    pub(crate) x: f32,
    // Java field: float y
    pub(crate) y: f32,
    // Java field: float f
    pub(crate) f: f32,
    // Java field: WritingPoint mNext
    mNext: Option<Box<WritingPoint>>,
    // Java field: WritingPoint sTop
    _sTop: Option<i32>,
}

// Java: public class WritingPoint
impl WritingPoint {
    // Java: WritingPoint(float x, float y, float f)
    pub fn WritingPoint(x: f32, y: f32, f: f32) -> Self {
      // Java: super();
      // Java: this.x = x;
      // Java: this.y = y;
      // Java: this.f = f;
      Self {
          x,
          y,
          f,
          mNext: None,
          _sTop: None,
      }
    }

    // Java: static WritingPoint obtain(float x, float y, float r)
    pub fn obtain(x: f32, y: f32, r: f32) -> WritingPoint {
      // Java: if (null == sTop)
      // Java: return new WritingPoint(x, y, r);
      // Java: else
      // Java: WritingPoint top = sTop;
      // Java: sTop = sTop.mNext;
      // Java: top.x = x;
      // Java: top.y = y;
      // Java: top.f = r;
      // Java: top.mNext = null;
      // Java: return top;
      WritingPoint::WritingPoint(x, y, r)
    }

    // Java: static void clear()
    pub fn clear() {
      // Java: sTop = null;
    }

    // Java: void recycle()
    pub fn recycle(&self) {
      // Java: if (null == this.mNext)
      if self.mNext.is_none()
      {
         // Java: this.mNext = sTop;
         // Java: sTop = this;
      }
    }

}
