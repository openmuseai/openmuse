// Java: import android.graphics.RectF
use util::cn::wps::graphics::RectF;
// Java: import java.util.Arrays
// TODO_RUST_CODE use ???; // java.util.Arrays

// Java: public class GradientAttr
pub struct GradientAttr {
    // Java field: GradType gradType
    pub(crate) gradType: Arc<Mutex<GradType>>,
    // Java field: int fillAngle
    pub(crate) fillAngle: i32,
    // Java field: int[] colors
    pub(crate) colors: Arc<Mutex<Vec<i32>>>,
    // Java field: float[] colorAlpha
    pub(crate) colorAlpha: Arc<Mutex<Vec<f32>>>,
    // Java field: float[] positions
    pub(crate) positions: Arc<Mutex<Vec<f32>>>,
    // Java field: RectF fillRect
    pub(crate) fillRect: Arc<Mutex<RectF>>,
    // Java field: RectF titleRect
    pub(crate) titleRect: Arc<Mutex<RectF>>,
}

// Java: public enum GradType
pub struct GradType {
}

// Java: public class GradientAttr
impl GradientAttr {
    // Java: GradientAttr(GradType gradType, int fillAngle, int[] colors, float[] colorAlpha, float[] positions)
    pub fn GradientAttr(&self, GradType gradType, int fillAngle, int[] colors, float[] colorAlpha, float[] positions) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.gradType = GradientAttr.GradType.LINEAR;
      // TODO_RUST_CODE
      // Java: this.fillRect = null;
      // TODO_RUST_CODE
      // Java: this.titleRect = null;
      // TODO_RUST_CODE
      // Java: this.gradType = gradType;
      // TODO_RUST_CODE
      // Java: this.fillAngle = fillAngle;
      // TODO_RUST_CODE
      // Java: this.colors = colors;
      // TODO_RUST_CODE
      // Java: this.colorAlpha = colorAlpha;
      // TODO_RUST_CODE
      // Java: this.positions = positions;
      // TODO_RUST_CODE
    }

    // Java: int angle()
    pub fn angle(&self) -> i32 {
      // Java: return this.fillAngle;
      // TODO_RUST_CODE
    }

    // Java: int[] colors()
    pub fn colors(&self) -> int[] {
      // Java: return this.colors;
      // TODO_RUST_CODE
    }

    // Java: float[] colorAlpha()
    pub fn colorAlpha(&self) -> float[] {
      // Java: return this.colorAlpha;
      // TODO_RUST_CODE
    }

    // Java: float[] positions()
    pub fn positions(&self) -> float[] {
      // Java: return this.positions;
      // TODO_RUST_CODE
    }

    // Java: GradType gradType()
    pub fn gradType(&self) -> GradType {
      // Java: return this.gradType;
      // TODO_RUST_CODE
    }

    // Java: RectF fillRect()
    pub fn fillRect(&self) -> RectF {
      // Java: return this.fillRect;
      // TODO_RUST_CODE
    }

    // Java: void setFillRect(float left, float top, float right, float bottom)
    pub fn setFillRect(&self, float left, float top, float right, float bottom) {
      // Java: this.fillRect = new RectF(left, top, right, bottom);
      // TODO_RUST_CODE
    }

    // Java: RectF titleRect()
    pub fn titleRect(&self) -> RectF {
      // Java: return this.titleRect;
      // TODO_RUST_CODE
    }

    // Java: void setTitleRect(float left, float top, float right, float bottom)
    pub fn setTitleRect(&self, float left, float top, float right, float bottom) {
      // Java: this.titleRect = new RectF(left, top, right, bottom);
      // TODO_RUST_CODE
    }

    // Java: boolean equals(GradientAttr gradAttr)
    pub fn equals(&self, GradientAttr gradAttr) -> bool {
      // Java: if (gradAttr == null)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return this.gradType == gradAttr.gradType && this.fillAngle == gradAttr.fillAngle && Arrays.equals(this.colors, gradAttr.colors) && Arrays.equals(this.colorAlpha, gradAttr.colorAlpha) && Arrays.equals(this.positions, gradAttr.positions) && (this.fillRect == null && gradAttr.fillRect == null || this.fillRect != null && this.fillRect.equals(gradAttr.fillRect)) && (this.titleRect == null && gradAttr.titleRect == null || this.titleRect != null && this.titleRect.equals(gradAttr.titleRect));
         // TODO_RUST_CODE
      }
    }

}

// Java: public enum GradType
impl GradType {
    // Java: GradType()
    pub fn GradType(&self) {
    // TODO_RUST_CODE
    }

}
