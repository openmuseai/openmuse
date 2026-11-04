// Java: import com.parser.IInkStroker
use crate::com::parser::IInkStroker::IInkStroker;

// Java: public class LerpStroker implements IInkStroker
pub struct LerpStroker {
    // Java field: IInkStroker mBaseStroker
    mBaseStroker: Option<Box<dyn IInkStroker>>,
    // Java field: float mLastX
    mLastX: f32,
    // Java field: float mLastY
    mLastY: f32,
    // Java field: float mLastRadius
    mLastRadius: f32,
}

// Java: public class LerpStroker implements IInkStroker
impl LerpStroker {
    // Java: LerpStroker(IInkStroker baseStroker)
    pub fn LerpStroker(base_stroker: Box<dyn IInkStroker>) -> Self {
      // Java: super();
      // Java: this.mBaseStroker = baseStroker;
      Self {
          mBaseStroker: Some(base_stroker),
          mLastX: 0.0,
          mLastY: 0.0,
          mLastRadius: 0.0,
      }
    }

    // Java: void onStart(float x, float y, float r)
    pub fn onStart(&mut self, x: f32, y: f32, r: f32) {
      // Java: this.mBaseStroker.onStart(x, y, r);
      if let Some(ref mut stroker) = self.mBaseStroker {
          stroker.onStart(x, y, r);
      }
      // Java: this.mLastX = x;
      self.mLastX = x;
      // Java: this.mLastY = y;
      self.mLastY = y;
      // Java: this.mLastRadius = r;
      self.mLastRadius = r;
    }

    // Java: void onMove(float x, float y, float r)
    pub fn onMove(&mut self, x: f32, y: f32, r: f32) {
      // Java: float deltaR = r - this.mLastRadius;
      let delta_r = r - self.mLastRadius;
      // Java: int step = (int)Math.abs(deltaR / 0.5F);
      let mut step = (delta_r / 0.5).abs() as i32;
      // Java: int dis = (int)this.dist(x, y, this.mLastX, this.mLastY);
      let dis = self.dist(x, y, self.mLastX, self.mLastY) as i32;
      // Java: if (dis > step)
      if dis > step
      {
         // Java: step = dis;
         step = dis;
      }
      // Java: if (0 == step)
      if 0 == step
      {
         // Java: this.mBaseStroker.onMove(x, y, r);
         if let Some(ref mut stroker) = self.mBaseStroker {
             stroker.onMove(x, y, r);
         }
         // Java: this.mLastX = x;
         self.mLastX = x;
         // Java: this.mLastY = y;
         self.mLastY = y;
         // Java: this.mLastRadius = r;
         self.mLastRadius = r;
      }
      // Java: else
      else
      {
         // Java: float stepX = (x - this.mLastX) / (float)step;
         let step_x = (x - self.mLastX) / step as f32;
         // Java: float stepY = (y - this.mLastY) / (float)step;
         let step_y = (y - self.mLastY) / step as f32;
         // Java: for(int i = 0; i < step; ++i)
         for i in 0..step
         {
            // Java: this.mBaseStroker.onMove(this.mLastX + stepX * (float)i, this.mLastY + stepY * (float)i, this.mLastRadius + deltaR / (float)step * (float)i);
            if let Some(ref mut stroker) = self.mBaseStroker {
                stroker.onMove(
                    self.mLastX + step_x * i as f32,
                    self.mLastY + step_y * i as f32,
                    self.mLastRadius + delta_r / step as f32 * i as f32,
                );
            }
         }
         // Java: this.mBaseStroker.onMove(x, y, r);
         if let Some(ref mut stroker) = self.mBaseStroker {
             stroker.onMove(x, y, r);
         }
         // Java: this.mLastX = x;
         self.mLastX = x;
         // Java: this.mLastY = y;
         self.mLastY = y;
         // Java: this.mLastRadius = r;
         self.mLastRadius = r;
      }
    }

    // Java: void onFinish()
    pub fn onFinish(&mut self) {
      // Java: this.mBaseStroker.onFinish();
      if let Some(ref mut stroker) = self.mBaseStroker {
          stroker.onFinish();
      }
      // Java: this.mLastX = 0.0F;
      self.mLastX = 0.0;
      // Java: this.mLastY = 0.0F;
      self.mLastY = 0.0;
      // Java: this.mLastRadius = 0.0F;
      self.mLastRadius = 0.0;
    }

    // Java: float dist(float x1, float y1, float x2, float y2)
    pub fn dist(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
      // Java: x2 -= x1;
      let dx = x2 - x1;
      // Java: y2 -= y1;
      let dy = y2 - y1;
      // Java: return (float)Math.sqrt((double)(x2 * x2 + y2 * y2));
      ((dx * dx + dy * dy) as f64).sqrt() as f32
    }

    // Java: float lerp(float a, float b, float f)
    pub fn lerp(&self, a: f32, b: f32, f: f32) -> f32 {
      // Java: return a + f * (b - a);
      a + f * (b - a)
    }

    // Java: float getStrokeWidth()
    pub fn getStrokeWidth(&self) -> f32 {
      // Java: return 0.0F;
      0.0
    }

}

impl IInkStroker for LerpStroker {
    fn onStart(&mut self, var1: f32, var2: f32, var3: f32) {
        LerpStroker::onStart(self, var1, var2, var3);
    }
    fn onMove(&mut self, var1: f32, var2: f32, var3: f32) {
        LerpStroker::onMove(self, var1, var2, var3);
    }
    fn onFinish(&mut self) {
        LerpStroker::onFinish(self);
    }
    fn getStrokeWidth(&self) -> f32 {
        LerpStroker::getStrokeWidth(self)
    }
}
