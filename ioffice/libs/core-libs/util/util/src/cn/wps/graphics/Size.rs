#![allow(non_snake_case, dead_code, unused_variables)]

// Java: public class Size
#[derive(Clone)]
pub struct Size {
    // Java field: int height
    pub height: i32,
    // Java field: int width
    pub width: i32,
}

// Java: public class Size
impl Size {
    // Java: Size()
    pub fn new() -> Self {
        // Java: super();
        Self { width: 0, height: 0 }
    }

    // Java: Size(int width, int height)
    pub fn new_with(width: i32, height: i32) -> Self {
        // Java: super();
        // Java: this.width = width;
        // Java: this.height = height;
        Self { width, height }
    }

    // Java: Size(Size r)
    pub fn new_from(r: &Size) -> Self {
        // Java: this(r.width, r.height);
        Self { width: r.width, height: r.height }
    }

    // Java: boolean isValid()
    pub fn isValid(&self) -> bool {
        // Java: return this.width > 0 && this.height > 0;
        self.width > 0 && self.height > 0
    }

    // Java: Object clone()
    pub fn clone_size(&self) -> Size {
        // Java: return new Size(this);
        Self::new_from(self)
    }

    // Java: boolean equals(Object r)
    pub fn equals(&self, r: &Size) -> bool {
        // Java: if (!(r instanceof Size))
        // Java: return false;
        // Java: else if (this == r)
        // Java: return true;
        // Java: else
        // Java: Size p = (Size)r;
        // Java: return this.width == p.width && this.height == p.height;
        self.width == r.width && self.height == r.height
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.width + this.height;
        self.width + self.height
    }
}
