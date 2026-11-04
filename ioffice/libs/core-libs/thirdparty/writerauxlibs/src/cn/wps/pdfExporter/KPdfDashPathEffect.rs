// Java: import android.graphics.DashPathEffect
// (no direct Rust equivalent)

// Java: public class KPdfDashPathEffect extends DashPathEffect
pub struct KPdfDashPathEffect {
    // Java field: float[] mIntervals
    pub(crate) mIntervals: Vec<f32>,
    // Java field: float mPhase
    pub(crate) mPhase: f32,
}

// Java: public class KPdfDashPathEffect extends DashPathEffect
#[allow(non_snake_case)]
impl KPdfDashPathEffect {
    // Java: KPdfDashPathEffect(float[] intervals, float phase)
    pub fn new(intervals: Vec<f32>, phase: f32) -> Self {
      // Java: super(intervals, phase);
      // Java: this.mIntervals = intervals;
      // Java: this.mPhase = phase;
      Self { mIntervals: intervals, mPhase: phase }
    }

    // Java: float[] getIntervals()
    pub fn getIntervals(&self) -> &[f32] {
      // Java: return this.mIntervals;
      &self.mIntervals
    }

    // Java: float getPhase()
    pub fn getPhase(&self) -> f32 {
      // Java: return this.mPhase;
      self.mPhase
    }

}
