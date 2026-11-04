#[allow(unused_imports)]
use crate::com::hp::hpl::inkml::r#impl::InkPainter::Canvas;
#[allow(unused_imports)]
use crate::com::hp::hpl::inkml::r#impl::InkPainter::Paint;
#[allow(unused_imports)]
use crate::com::hp::hpl::inkml::r#impl::InkPainter::Path;

// Java: public class RectPainter extends IPainter
pub struct RectPainter {
    // Java field: Path mPath
    mPath: Option<Path>,
    // Java field: float halfStrokeWidth
    halfStrokeWidth: f32,
    // Java field: float halfStrokeHeight
    halfStrokeHeight: f32,
    // Java field: float mLastX
    mLastX: f32,
    // Java field: float mLastY
    mLastY: f32,
}

// Java: public class RectPainter extends IPainter
impl RectPainter {
    // Java: RectPainter()
    pub fn RectPainter() -> Self {
      // Java: super();
      Self { mPath: None, halfStrokeWidth: 0.0, halfStrokeHeight: 0.0, mLastX: 0.0, mLastY: 0.0 }
    }

    // Java: void begin(Path path, float width, float height, float x, float y, float f)
    pub fn begin(&mut self, path: Path, width: f32, height: f32, x: f32, y: f32, _f: f32) {
      // Java: this.mPath = path;
      self.mPath = Some(path);
      // Java: this.mPath.setFillType(FillType.WINDING);
      // Java: this.halfStrokeWidth = width / 2.0F;
      self.halfStrokeWidth = width / 2.0;
      // Java: this.halfStrokeHeight = height / 2.0F;
      self.halfStrokeHeight = height / 2.0;
      // Java: this.mLastX = x;
      self.mLastX = x;
      // Java: this.mLastY = y;
      self.mLastY = y;
      // Java: this.addRect(x, y, this.halfStrokeWidth, this.halfStrokeHeight, path);
    }

    // Java: void move(float x, float y, float f)
    pub fn move_to(&mut self, x: f32, y: f32, _f: f32) {
      // Java: this.fillPath(this.mLastX, this.mLastY, x, y, this.halfStrokeWidth, this.halfStrokeHeight, this.mPath);
      // Java: this.mLastX = x;
      self.mLastX = x;
      // Java: this.mLastY = y;
      self.mLastY = y;
    }

    // Java: void end()
    pub fn end(&self) {
    }

    // Java: void draw(Canvas canvas, Paint paint)
    pub fn draw(&self, _canvas: &Canvas, _paint: &Paint) {
      // Java: canvas.drawPath(this.mPath, paint);
    }

    // Java: void fillPath(float xStart, float yStart, float xEnd, float yEnd, float halfStrokeWidth, float halfStrokeHeight, Path path)
    fn fillPath(&self, x_start: f32, y_start: f32, x_end: f32, y_end: f32, half_stroke_width: f32, half_stroke_height: f32, _path: &Path) {
      // Java: float xAdd = xEnd - xStart;
      let _x_add = x_end - x_start;
      // Java: float yAdd = yEnd - yStart;
      let _y_add = y_end - y_start;
      // Java: float rectLeft = xStart - halfStrokeWidth;
      let _rect_left = x_start - half_stroke_width;
      // Java: float rectTop = yStart - halfStrokeHeight;
      let _rect_top = y_start - half_stroke_height;
      // Java: float rectRight = xStart + halfStrokeWidth;
      let _rect_right = x_start + half_stroke_width;
      // Java: float rectBottom = yStart + halfStrokeHeight;
      let _rect_bottom = y_start + half_stroke_height;
      // Java: if (xEnd > xStart)
      if x_end > x_start
      {
         // Java: if (yEnd > yStart)
         if y_end > y_start
         {
            // Java: path.moveTo(rectLeft, rectBottom);
            // Java: path.lineTo(rectRight, rectBottom);
            // Java: path.lineTo(rectRight, rectTop);
            // Java: path.lineTo(rectRight + xAdd, rectTop + yAdd);
            // Java: path.lineTo(rectRight + xAdd, rectBottom + yAdd);
            // Java: path.lineTo(rectLeft + xAdd, rectBottom + yAdd);
         }
         // Java: else
         else
         {
            // Java: path.moveTo(rectRight, rectBottom);
            // Java: path.lineTo(rectRight, rectTop);
            // Java: path.lineTo(rectLeft, rectTop);
            // Java: path.lineTo(rectLeft + xAdd, rectTop + yAdd);
            // Java: path.lineTo(rectRight + xAdd, rectTop + yAdd);
            // Java: path.lineTo(rectRight + xAdd, rectBottom + yAdd);
         }
      }
      // Java: else if (yEnd > yStart)
      else if y_end > y_start
      {
         // Java: path.moveTo(rectLeft, rectTop);
         // Java: path.lineTo(rectLeft, rectBottom);
         // Java: path.lineTo(rectRight, rectBottom);
         // Java: path.lineTo(rectRight + xAdd, rectBottom + yAdd);
         // Java: path.lineTo(rectLeft + xAdd, rectBottom + yAdd);
         // Java: path.lineTo(rectLeft + xAdd, rectTop + yAdd);
      }
      // Java: else
      else
      {
         // Java: path.moveTo(rectRight, rectTop);
         // Java: path.lineTo(rectLeft, rectTop);
         // Java: path.lineTo(rectLeft, rectBottom);
         // Java: path.lineTo(rectLeft + xAdd, rectBottom + yAdd);
         // Java: path.lineTo(rectLeft + xAdd, rectTop + yAdd);
         // Java: path.lineTo(rectRight + xAdd, rectTop + yAdd);
      }
      // Java: path.close();
    }

    // Java: void addRect(float x, float y, float halfStrokeWidth, float halfStrokeHeight, Path path)
    fn addRect(&self, x: f32, y: f32, half_stroke_width: f32, half_stroke_height: f32, _path: &Path) {
      // Java: float rectLeft = x - halfStrokeWidth;
      let _rect_left = x - half_stroke_width;
      // Java: float rectTop = y - halfStrokeHeight;
      let _rect_top = y - half_stroke_height;
      // Java: float rectRight = x + halfStrokeWidth;
      let _rect_right = x + half_stroke_width;
      // Java: float rectBottom = y + halfStrokeHeight;
      let _rect_bottom = y + half_stroke_height;
      // Java: path.addRect(rectLeft, rectTop, rectRight, rectBottom, Direction.CW);
    }

}
