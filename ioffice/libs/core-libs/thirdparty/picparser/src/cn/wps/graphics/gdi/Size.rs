#![allow(non_snake_case, dead_code)]

// Java: public class Size implements Cloneable
#[derive(Clone)]
pub struct Size {
    // Java field: float height
    pub(crate) height: f32,
    // Java field: float width
    pub(crate) width: f32,
}

// Java: public class Size implements Cloneable
impl Size {
    // Java: Size(float width, float height)
    pub fn new(width: f32, height: f32) -> Self {
        // Java: super();
        // Java: this.width = width;
        // Java: this.height = height;
        Size { width, height }
    }

    // Java: float getHeight()
    pub fn getHeight(&self) -> f32 {
        // Java: return this.height;
        self.height
    }

    // Java: float getWidth()
    pub fn getWidth(&self) -> f32 {
        // Java: return this.width;
        self.width
    }

    // Java: void setSize(float width, float height)
    pub fn setSize(&mut self, width: f32, height: f32) {
        // Java: this.width = width;
        self.width = width;
        // Java: this.height = height;
        self.height = height;
    }

    // Java: Size clone()
    pub fn clone_size(&self) -> Size {
        // Java: return new Size(this.width, this.height);
        Size::new(self.width, self.height)
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "Size = (" + this.width + ", " + this.height + ")";
        format!("Size = ({}, {})", self.width, self.height)
    }
}
