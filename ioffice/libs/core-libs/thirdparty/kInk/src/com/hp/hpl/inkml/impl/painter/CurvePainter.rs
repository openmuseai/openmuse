#[allow(unused_imports)]
use crate::com::hp::hpl::inkml::r#impl::InkPainter::Canvas;
#[allow(unused_imports)]
use crate::com::hp::hpl::inkml::r#impl::InkPainter::Paint;
#[allow(unused_imports)]
use crate::com::hp::hpl::inkml::r#impl::InkPainter::Path;

// Java: public class CurvePainter extends IPainter
pub struct CurvePainter {
    // Java field: Path mPath
    mPath: Option<Path>,
    // Java field: float mLastX
    mLastX: f32,
    // Java field: float mLastY
    mLastY: f32,
    // Java field: float mX
    mX: f32,
    // Java field: float mY
    mY: f32,
    // Java field: float mStroke2
    mStroke2: f32,
}

// Java: public class CurvePainter extends IPainter
impl CurvePainter {
    // Java: CurvePainter()
    pub fn CurvePainter() -> Self {
      // Java: super();
      Self { mPath: None, mLastX: 0.0, mLastY: 0.0, mX: 0.0, mY: 0.0, mStroke2: 0.0 }
    }

    // Java: void begin(Path path, float width, float height, float x, float y, float f)
    pub fn begin(&mut self, path: Path, width: f32, _height: f32, x: f32, y: f32, _f: f32) {
      // Java: this.mPath = path;
      self.mPath = Some(path);
      // Java: this.mStroke2 = width * width;
      self.mStroke2 = width * width;
      // Java: path.moveTo(x, y);
      // Java: this.mX = this.mLastX = x;
      self.mLastX = x;
      self.mX = x;
      // Java: this.mY = this.mLastY = y;
      self.mLastY = y;
      self.mY = y;
    }

    // Java: void move(float x, float y, float f)
    pub fn move_to(&mut self, x: f32, y: f32, _f: f32) {
      // Java: this.mX = x;
      self.mX = x;
      // Java: this.mY = y;
      self.mY = y;
      // Java: float absX = Math.abs(this.mLastX - x);
      let abs_x = (self.mLastX - x).abs();
      // Java: float absY = Math.abs(this.mLastY - y);
      let abs_y = (self.mLastY - y).abs();
      // Java: float dist2 = absX * absX + absY * absY;
      let dist2 = abs_x * abs_x + abs_y * abs_y;
      // Java: if (!(dist2 < this.mStroke2 * 0.01F))
      if !(dist2 < self.mStroke2 * 0.01)
      {
         // Java: this.mPath.quadTo(this.mLastX, this.mLastY, (x + this.mLastX) / 2.0F, (y + this.mLastY) / 2.0F);
         // Java: this.mLastX = x;
         self.mLastX = x;
         // Java: this.mLastY = y;
         self.mLastY = y;
      }
    }

    // Java: void end()
    pub fn end(&mut self) {
      // Java: this.mPath.lineTo(this.mLastX, this.mLastY);
    }

    // Java: void end(boolean completed)
    pub fn end_completed(&mut self, completed: bool) {
      // Java: if (completed)
      if completed
      {
         // Java: if (this.mX != this.mLastX && this.mY != this.mLastY)
         if self.mX != self.mLastX && self.mY != self.mLastY
         {
            // Java: this.mPath.quadTo(this.mLastX, this.mLastY, (this.mX + this.mLastX) / 2.0F, (this.mY + this.mLastY) / 2.0F);
         }
         // Java: this.mPath.lineTo(this.mX, this.mY);
      }
    }

    // Java: void draw(Canvas canvas, Paint paint)
    pub fn draw(&self, _canvas: &Canvas, _paint: &Paint) {
      // Java: canvas.drawPath(this.mPath, paint);
    }

}
