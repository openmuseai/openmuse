#![allow(non_snake_case, dead_code, unused_imports)]

// Java: import android.graphics.Matrix
// Java: import android.graphics.Rect

use super::Size::Size;

pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Rect { left, top, right, bottom }
    }
    pub fn width(&self) -> i32 { self.right - self.left }
    pub fn height(&self) -> i32 { self.bottom - self.top }
}

pub struct Matrix {
    values: [f32; 9],
}

impl Matrix {
    pub fn getValues(&self, out: &mut [f32; 9]) {
        *out = self.values;
    }
}

// Java: public class Rectangle
#[derive(Clone)]
pub struct Rectangle {
    // Java field: int x
    pub(crate) x: i32,
    // Java field: int y
    pub(crate) y: i32,
    // Java field: int width
    pub(crate) width: i32,
    // Java field: int height
    pub(crate) height: i32,
}

// Java: public class Rectangle
impl Rectangle {
    // Java: Rectangle(int x, int y, int width, int height)
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        // Java: super();
        // Java: this.x = x;
        // Java: this.y = y;
        // Java: this.width = width;
        // Java: this.height = height;
        Rectangle { x, y, width, height }
    }

    // Java: Rectangle(Rectangle rect)
    pub fn from_rect(rect: &Rectangle) -> Self {
        // Java: super();
        // Java: this.x = rect.x;
        // Java: this.y = rect.y;
        // Java: this.width = rect.width;
        // Java: this.height = rect.height;
        Rectangle {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
        }
    }

    // Java: Rectangle()
    pub fn default() -> Self {
        // Java: super();
        // Java: this.x = this.y = this.width = this.height = 0;
        Rectangle { x: 0, y: 0, width: 0, height: 0 }
    }

    // Java: Rect getRect()
    pub fn getRect(&self) -> Rect {
        // Java: return new Rect(this.x, this.y, this.x + this.width, this.y + this.height);
        Rect::new(self.x, self.y, self.x + self.width, self.y + self.height)
    }

    // Java: Size getSize()
    pub fn getSize(&self) -> Size {
        // Java: return new Size((float)this.width, (float)this.height);
        Size::new(self.width as f32, self.height as f32)
    }

    // Java: static Rectangle createRectangle(Rect rect)
    pub fn createRectangle(rect: &Rect) -> Rectangle {
        // Java: return new Rectangle(rect.left, rect.top, rect.width(), rect.height());
        Rectangle::new(rect.left, rect.top, rect.width(), rect.height())
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "(" + this.x + ", " + this.y + " - " + this.width + ", " + this.height + ")";
        format!("({}, {} - {}, {})", self.x, self.y, self.width, self.height)
    }

    // Java: Rectangle preConcat(Matrix matrix)
    pub fn preConcat(&self, matrix: &Matrix) -> Rectangle {
        // Java: float[] matrixFloatValues = new float[9];
        let mut matrixFloatValues = [0.0f32; 9];
        // Java: matrix.getValues(matrixFloatValues);
        matrix.getValues(&mut matrixFloatValues);
        // Java: float scaleX = matrixFloatValues[0];
        let scaleX = matrixFloatValues[0];
        // Java: float skewX = matrixFloatValues[1];
        let skewX = matrixFloatValues[1];
        // Java: float transX = matrixFloatValues[2];
        let transX = matrixFloatValues[2];
        // Java: float skewY = matrixFloatValues[3];
        let skewY = matrixFloatValues[3];
        // Java: float scaleY = matrixFloatValues[4];
        let scaleY = matrixFloatValues[4];
        // Java: float transY = matrixFloatValues[5];
        let transY = matrixFloatValues[5];
        // Java: Rectangle rect = new Rectangle();
        let mut rect = Rectangle::default();
        // Java: rect.x = Math.round((float)this.x * scaleX + (float)this.y * skewX + transX);
        rect.x = (self.x as f32 * scaleX + self.y as f32 * skewX + transX).round() as i32;
        // Java: rect.y = Math.round((float)this.x * skewY + (float)this.y * scaleY + transY);
        rect.y = (self.x as f32 * skewY + self.y as f32 * scaleY + transY).round() as i32;
        // Java: rect.width = Math.round((float)this.width * scaleX + (float)this.height * skewX);
        rect.width = (self.width as f32 * scaleX + self.height as f32 * skewX).round() as i32;
        // Java: rect.height = Math.round((float)this.width * skewY + (float)this.height * scaleY);
        rect.height = (self.width as f32 * skewY + self.height as f32 * scaleY).round() as i32;
        // Java: return rect;
        rect
    }
}
