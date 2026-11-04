// Java: public class FontMetrics
pub struct FontMetrics {
    // Java field: float mMaxAscent
    pub(crate) mMaxAscent: f32,
    // Java field: float mMaxDescent
    pub(crate) mMaxDescent: f32,
    // Java field: float mBaseline
    pub(crate) mBaseline: f32,
    // Java field: float mHeight
    pub(crate) mHeight: f32,
    // Java field: float mLeading
    pub(crate) mLeading: f32,
}

// Java: public class FontMetrics
impl FontMetrics {
    // Java: FontMetrics()
    pub fn new() -> Self {
        // Java: super();
        FontMetrics {
            mMaxAscent: 0.0,
            mMaxDescent: 0.0,
            mBaseline: 0.0,
            mHeight: 0.0,
            mLeading: 0.0,
        }
    }

    // Java: void init()
    pub fn init(&mut self) {
        // Java: this.mMaxAscent = 0.0F;
        self.mMaxAscent = 0.0;
        // Java: this.mMaxDescent = 0.0F;
        self.mMaxDescent = 0.0;
        // Java: this.mBaseline = 0.0F;
        self.mBaseline = 0.0;
        // Java: this.mHeight = 0.0F;
        self.mHeight = 0.0;
    }

    // Java: float getMaxAscent()
    pub fn getMaxAscent(&self) -> f32 {
        // Java: return this.mMaxAscent;
        self.mMaxAscent
    }

    // Java: void setMaxAscent(float maxAscent)
    pub fn setMaxAscent(&mut self, max_ascent: f32) {
        // Java: this.mMaxAscent = maxAscent;
        self.mMaxAscent = max_ascent;
    }

    // Java: float getMaxDescent()
    pub fn getMaxDescent(&self) -> f32 {
        // Java: return this.mMaxDescent;
        self.mMaxDescent
    }

    // Java: void setMaxDescent(float maxDescent)
    pub fn setMaxDescent(&mut self, max_descent: f32) {
        // Java: this.mMaxDescent = maxDescent;
        self.mMaxDescent = max_descent;
    }

    // Java: float getBaseline()
    pub fn getBaseline(&self) -> f32 {
        // Java: return this.mBaseline;
        self.mBaseline
    }

    // Java: void setBaseline(float baseline)
    pub fn setBaseline(&mut self, baseline: f32) {
        // Java: this.mBaseline = baseline;
        self.mBaseline = baseline;
    }

    // Java: float getHeight()
    pub fn getHeight(&self) -> f32 {
        // Java: return this.mHeight;
        self.mHeight
    }

    // Java: void setHeight(float height)
    pub fn setHeight(&mut self, height: f32) {
        // Java: this.mHeight = height;
        self.mHeight = height;
    }
}
