// Java: public class Point
pub struct Point {
    // Java field: float x
    pub(crate) x: f32,
    // Java field: float y
    pub(crate) y: f32,
}

// Java: public class Point
impl Point {
    // Java: static float getDst(Point p1, Point p2)
    pub fn getDst(&self, Point p1, Point p2) -> f32 {
      // Java: return equals(p1, p2) ? 0.0F : (float)Math.sqrt(Math.pow((double)(p1.x - p2.x), (double)2.0F) + Math.pow((double)(p1.y - p2.y), (double)2.0F));
      // TODO_RUST_CODE
    }

    // Java: float getSquareDst(Point point)
    pub fn getSquareDst(&self, Point point) -> f32 {
      // Java: return equals(this, point) ? 0.0F : (point.x - this.x) * (point.x - this.x) + (point.y - this.y) * (point.y - this.y);
      // TODO_RUST_CODE
    }

    // Java: Point()
    pub fn Point(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: Point(float x, float y)
    pub fn Point(&self, float x, float y) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.x = x;
      // TODO_RUST_CODE
      // Java: this.y = y;
      // TODO_RUST_CODE
    }

    // Java: Point(Point point)
    pub fn Point(&self, Point point) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.x = point.x;
      // TODO_RUST_CODE
      // Java: this.y = point.y;
      // TODO_RUST_CODE
    }

    // Java: void set(Point point)
    pub fn set(&self, Point point) {
      // Java: this.x = point.x;
      // TODO_RUST_CODE
      // Java: this.y = point.y;
      // TODO_RUST_CODE
    }

    // Java: void offset(float offsetX, float offsetY)
    pub fn offset(&self, float offsetX, float offsetY) {
      // Java: this.x += offsetX;
      // TODO_RUST_CODE
      // Java: this.y += offsetY;
      // TODO_RUST_CODE
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[" + this.x + "," + this.y + "]";
      // TODO_RUST_CODE
    }

    // Java: static boolean equals(Point p1, Point p2)
    pub fn equals(&self, Point p1, Point p2) -> bool {
      // Java: return p1 == p2 || null != p1 && null != p2 && p1.x == p2.x && p1.y == p2.y;
      // TODO_RUST_CODE
    }

}
