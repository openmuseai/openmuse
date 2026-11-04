// Java: public class RawPoint
pub struct RawPoint {
    // Java field: float x
    pub(crate) x: f32,
    // Java field: float y
    pub(crate) y: f32,
    // Java field: float pressure
    pub(crate) pressure: f32,
    // Java field: float size
    pub(crate) size: f32,
    // Java field: float speed
    pub(crate) speed: f32,
    // Java field: RawPoint mNext
    mNext: Option<Box<RawPoint>>,
    // Java field: RawPoint sTop
    _sTop: Option<i32>,
}

// Java: public class RawPoint
impl RawPoint {
    // Java: RawPoint(float x, float y, float pressure, float size, float speed)
    pub fn RawPoint(x: f32, y: f32, pressure: f32, size: f32, speed: f32) -> Self {
      // Java: super();
      // Java: this.x = x;
      // Java: this.y = y;
      // Java: this.pressure = pressure;
      // Java: this.size = size;
      // Java: this.speed = speed;
      Self {
          x,
          y,
          pressure,
          size,
          speed,
          mNext: None,
          _sTop: None,
      }
    }

    // Java: void recycle()
    pub fn recycle(&mut self) {
      // Java: if (null == this.mNext)
      if self.mNext.is_none()
      {
         // Java: this.mNext = sTop;
         // Java: sTop = this;
      }
    }

    // Java: static RawPoint obtain(float x, float y, float pressure, float size, float speed)
    pub fn obtain(x: f32, y: f32, pressure: f32, size: f32, speed: f32) -> RawPoint {
      // Java: if (null == sTop)
      // Java: return new RawPoint(x, y, pressure, size, speed);
      // Java: else
      // Java: RawPoint top = sTop;
      // Java: sTop = sTop.mNext;
      // Java: top.x = x;
      // Java: top.y = y;
      // Java: top.pressure = pressure;
      // Java: top.size = size;
      // Java: top.speed = speed;
      // Java: top.mNext = null;
      // Java: return top;
      RawPoint::RawPoint(x, y, pressure, size, speed)
    }

    // Java: static void clear()
    pub fn clear() {
      // Java: sTop = null;
    }

}
