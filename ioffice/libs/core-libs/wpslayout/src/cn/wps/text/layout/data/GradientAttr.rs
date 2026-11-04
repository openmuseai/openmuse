// Java: import android.graphics.RectF
#[derive(Clone, PartialEq)]
pub struct RectF {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl RectF {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        RectF { left, top, right, bottom }
    }
}

// Java: public enum GradType
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GradType {
    SHAPE,
    CIRCLE,
    RECT,
    LINEAR,
}

// Java: public class GradientAttr
#[derive(Clone)]
pub struct GradientAttr {
    // Java field: GradType gradType
    pub(crate) gradType: GradType,
    // Java field: int fillAngle
    pub(crate) fillAngle: i32,
    // Java field: int[] colors
    pub(crate) colors: Vec<i32>,
    // Java field: float[] colorAlpha
    pub(crate) colorAlpha: Vec<f32>,
    // Java field: float[] positions
    pub(crate) positions: Vec<f32>,
    // Java field: RectF fillRect
    pub(crate) fillRect: Option<RectF>,
    // Java field: RectF titleRect
    pub(crate) titleRect: Option<RectF>,
}

// Java: public class GradientAttr
impl GradientAttr {
    // Java: GradientAttr(GradType gradType, int fillAngle, int[] colors, float[] colorAlpha, float[] positions)
    pub fn new(gradType: GradType, fillAngle: i32, colors: Vec<i32>, colorAlpha: Vec<f32>, positions: Vec<f32>) -> Self {
      // Java: super();
      // Java: this.gradType = GradientAttr.GradType.LINEAR;
      // Java: this.fillRect = null;
      // Java: this.titleRect = null;
      // Java: this.gradType = gradType;
      // Java: this.fillAngle = fillAngle;
      // Java: this.colors = colors;
      // Java: this.colorAlpha = colorAlpha;
      // Java: this.positions = positions;
      GradientAttr {
          gradType,
          fillAngle,
          colors,
          colorAlpha,
          positions,
          fillRect: None,
          titleRect: None,
      }
    }

    // Java: int angle()
    pub fn angle(&self) -> i32 {
      // Java: return this.fillAngle;
      self.fillAngle
    }

    // Java: int[] colors()
    pub fn colors(&self) -> &[i32] {
      // Java: return this.colors;
      &self.colors
    }

    // Java: float[] colorAlpha()
    pub fn colorAlpha(&self) -> &[f32] {
      // Java: return this.colorAlpha;
      &self.colorAlpha
    }

    // Java: float[] positions()
    pub fn positions(&self) -> &[f32] {
      // Java: return this.positions;
      &self.positions
    }

    // Java: GradType gradType()
    pub fn gradType(&self) -> GradType {
      // Java: return this.gradType;
      self.gradType
    }

    // Java: RectF fillRect()
    pub fn fillRect(&self) -> Option<&RectF> {
      // Java: return this.fillRect;
      self.fillRect.as_ref()
    }

    // Java: void setFillRect(float left, float top, float right, float bottom)
    pub fn setFillRect(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
      // Java: this.fillRect = new RectF(left, top, right, bottom);
      self.fillRect = Some(RectF::new(left, top, right, bottom));
    }

    // Java: RectF titleRect()
    pub fn titleRect(&self) -> Option<&RectF> {
      // Java: return this.titleRect;
      self.titleRect.as_ref()
    }

    // Java: void setTitleRect(float left, float top, float right, float bottom)
    pub fn setTitleRect(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
      // Java: this.titleRect = new RectF(left, top, right, bottom);
      self.titleRect = Some(RectF::new(left, top, right, bottom));
    }

    // Java: boolean equals(GradientAttr gradAttr)
    pub fn equals(&self, gradAttr: Option<&GradientAttr>) -> bool {
      // Java: if (gradAttr == null)
      let gradAttr = match gradAttr {
          None =>
      {
         // Java: return false;
         return false;
      }
      // Java: else
          Some(g) => g,
      };
      {
         // Java: return this.gradType == gradAttr.gradType && this.fillAngle == gradAttr.fillAngle && Arrays.equals(this.colors, gradAttr.colors) && Arrays.equals(this.colorAlpha, gradAttr.colorAlpha) && Arrays.equals(this.positions, gradAttr.positions) && (this.fillRect == null && gradAttr.fillRect == null || this.fillRect != null && this.fillRect.equals(gradAttr.fillRect)) && (this.titleRect == null && gradAttr.titleRect == null || this.titleRect != null && this.titleRect.equals(gradAttr.titleRect));
         self.gradType == gradAttr.gradType
             && self.fillAngle == gradAttr.fillAngle
             && self.colors == gradAttr.colors
             && self.colorAlpha == gradAttr.colorAlpha
             && self.positions == gradAttr.positions
             && self.fillRect == gradAttr.fillRect
             && self.titleRect == gradAttr.titleRect
      }
    }

}

// Java: public enum GradType
impl GradType {
    // Java: GradType()
}
