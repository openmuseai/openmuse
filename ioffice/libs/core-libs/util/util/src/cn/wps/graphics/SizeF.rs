// Java: public class SizeF
pub struct SizeF {
    // Java field: float height
    pub height: f32,
    // Java field: float width
    pub width: f32,
}

// Java: public class SizeF
impl SizeF {
    // Java: SizeF()
    pub fn SizeF() -> Self {
        // Java: super();
        Self { height: 0.0, width: 0.0 }
    }

    // Java: SizeF(float width, float height)
    pub fn SizeF_new_with(width: f32, height: f32) -> Self {
        // Java: super();
        // Java: this.width = width;
        // Java: this.height = height;
        Self { width, height }
    }

    pub fn SizeF_wh(width: f32, height: f32) -> Self {
        Self::SizeF_new_with(width, height)
    }

    // Java: void set(float width, float height)
    pub fn set(&mut self, width: f32, height: f32) {
        // Java: this.width = width;
        self.width = width;
        // Java: this.height = height;
        self.height = height;
    }

    // Java: boolean equals(Object r)
    pub fn equals(&self, r: &SizeF) -> bool {
        // Java: if (!(r instanceof SizeF))
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: SizeF p = (SizeF)r;
            // Java: return this.width == p.width && this.height == p.height;
            self.width == r.width && self.height == r.height
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return (int)(this.width + this.height);
        (self.width + self.height) as i32
    }
}
