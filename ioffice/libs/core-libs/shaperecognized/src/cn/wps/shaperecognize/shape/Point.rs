// Java: public class Point
#[derive(Clone)]
pub struct Point {
    // Java field: float x
    pub(crate) x: f32,
    // Java field: float y
    pub(crate) y: f32,
}

// Java: public class Point
impl Point {
    // Java: static float getDst(Point p1, Point p2)
    pub fn getDst(p1: &Point, p2: &Point) -> f32 {
        // Java: return equals(p1, p2) ? 0.0F : (float)Math.sqrt(Math.pow((double)(p1.x - p2.x), (double)2.0F) + Math.pow((double)(p1.y - p2.y), (double)2.0F));
        if Point::equals(Some(p1), Some(p2)) {
            0.0
        } else {
            (((p1.x - p2.x) as f64).powi(2) + ((p1.y - p2.y) as f64).powi(2)).sqrt() as f32
        }
    }

    // Java: float getSquareDst(Point point)
    pub fn getSquareDst(&self, point: &Point) -> f32 {
        // Java: return equals(this, point) ? 0.0F : (point.x - this.x) * (point.x - this.x) + (point.y - this.y) * (point.y - this.y);
        if Point::equals(Some(self), Some(point)) {
            0.0
        } else {
            (point.x - self.x) * (point.x - self.x) + (point.y - self.y) * (point.y - self.y)
        }
    }

    // Java: Point()
    pub fn new() -> Self {
        // Java: super();
        Point { x: 0.0, y: 0.0 }
    }

    // Java: Point(float x, float y)
    pub fn new_with_xy(x: f32, y: f32) -> Self {
        // Java: super();
        // Java: this.x = x;
        // Java: this.y = y;
        Point { x, y }
    }

    // Java: Point(Point point)
    pub fn new_from_point(point: &Point) -> Self {
        // Java: super();
        // Java: this.x = point.x;
        // Java: this.y = point.y;
        Point {
            x: point.x,
            y: point.y,
        }
    }

    // Java: void set(Point point)
    pub fn set(&mut self, point: &Point) {
        // Java: this.x = point.x;
        self.x = point.x;
        // Java: this.y = point.y;
        self.y = point.y;
    }

    // Java: void offset(float offsetX, float offsetY)
    pub fn offset(&mut self, offsetX: f32, offsetY: f32) {
        // Java: this.x += offsetX;
        self.x += offsetX;
        // Java: this.y += offsetY;
        self.y += offsetY;
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "[" + this.x + "," + this.y + "]";
        format!("[{},{}]", self.x, self.y)
    }

    // Java: static boolean equals(Point p1, Point p2)
    pub fn equals(p1: Option<&Point>, p2: Option<&Point>) -> bool {
        // Java: return p1 == p2 || null != p1 && null != p2 && p1.x == p2.x && p1.y == p2.y;
        match (p1, p2) {
            (None, None) => true,
            (Some(p1), Some(p2)) => std::ptr::eq(p1, p2) || (p1.x == p2.x && p1.y == p2.y),
            _ => false,
        }
    }
}
