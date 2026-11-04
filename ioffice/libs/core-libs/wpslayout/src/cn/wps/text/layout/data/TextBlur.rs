// Java: public class TextBlur
#[derive(Clone)]
pub struct TextBlur {
    // Java field: int mBlurColor
    pub(crate) mBlurColor: i32,
    // Java field: float mBlurSize
    pub(crate) mBlurSize: f32,
    // Java field: float mColorAlpha
    pub(crate) mColorAlpha: f32,
}

// Java: public class TextBlur
impl TextBlur {
    // Java: TextBlur()
    pub fn new() -> Self {
      // Java: super();
      TextBlur {
          mBlurColor: 0,
          mBlurSize: 0.0,
          mColorAlpha: 0.0,
      }
    }

    // Java: float radius()
    pub fn radius(&self) -> f32 {
      // Java: return this.mBlurSize;
      self.mBlurSize
    }

    // Java: void setRadius(float radius)
    pub fn setRadius(&mut self, radius: f32) {
      // Java: this.mBlurSize = radius;
      self.mBlurSize = radius;
    }

    // Java: void setColor(int color, float alpha)
    pub fn setColor(&mut self, color: i32, alpha: f32) {
      // Java: this.mBlurColor = color;
      self.mBlurColor = color;
      // Java: this.mColorAlpha = alpha;
      self.mColorAlpha = alpha;
    }

    // Java: int blurColor()
    pub fn blurColor(&self) -> i32 {
      // Java: return this.mBlurColor;
      self.mBlurColor
    }

    // Java: float colorAlpha()
    pub fn colorAlpha(&self) -> f32 {
      // Java: return this.mColorAlpha;
      self.mColorAlpha
    }

    // Java: boolean equals(TextBlur txtBlur)
    pub fn equals(&self, txtBlur: Option<&TextBlur>) -> bool {
      // Java: if (txtBlur == null)
      let txtBlur = match txtBlur {
          None =>
      {
         // Java: return false;
         return false;
      }
      // Java: else
          Some(t) => t,
      };
      {
         // Java: return this.mBlurColor == txtBlur.mBlurColor && this.mBlurSize == txtBlur.mBlurSize && this.mColorAlpha == txtBlur.mColorAlpha;
         self.mBlurColor == txtBlur.mBlurColor && self.mBlurSize == txtBlur.mBlurSize && self.mColorAlpha == txtBlur.mColorAlpha
      }
    }

}
