// Java: import java.util.LinkedList
use std::collections::VecDeque;
use super::IPointProcess::IPointProcess;

// Java field: float SPEED_UP_FACTOR
pub(crate) const SPEED_UP_FACTOR: f32 = 1.1111112;

struct RawPoint {
    x: f32,
    y: f32,
    pressure: f32,
    size: f32,
    speed: f32,
}

impl RawPoint {
    fn obtain(x: f32, y: f32, pressure: f32, size: f32, speed: f32) -> Self {
        Self { x, y, pressure, size, speed }
    }
    fn recycle(&self) {}
    fn clear() {}
}

// Java: public class FixSpeedProcess implements IPointProcess
pub struct FixSpeedProcess {
    // Java field: LinkedList<RawPoint> mSpeedDownPoints
    mSpeedDownPoints: VecDeque<RawPoint>,
    // Java field: IPointProcess mNextHandler
    mNextHandler: Option<Box<dyn IPointProcess>>,
    // Java field: float mBeforeSpeedDownSpeed
    mBeforeSpeedDownSpeed: f32,
    // Java field: float mLastSpeed
    mLastSpeed: f32,
}

// Java: public class FixSpeedProcess implements IPointProcess
impl FixSpeedProcess {
    // Java: FixSpeedProcess(IPointProcess nextHandler)
    pub fn FixSpeedProcess(next_handler: Box<dyn IPointProcess>) -> Self {
      // Java: super();
      // Java: this.mNextHandler = nextHandler;
      // Java: this.mSpeedDownPoints = new LinkedList();
      Self {
          mSpeedDownPoints: VecDeque::new(),
          mNextHandler: Some(next_handler),
          mBeforeSpeedDownSpeed: 0.0,
          mLastSpeed: 0.0,
      }
    }

    // Java: void process(float x, float y, float pressure, float size, float speed, int type)
    pub fn process(&mut self, x: f32, y: f32, pressure: f32, size: f32, speed: f32, r#type: i32) {
      // Java: if (0 == type)
      if 0 == r#type
      {
         // Java: this.mNextHandler.process(x, y, pressure, size, speed, 0);
         if let Some(ref mut handler) = self.mNextHandler {
             handler.process(x, y, pressure, size, speed, 0);
         }
         // Java: this.mBeforeSpeedDownSpeed = this.mLastSpeed = speed;
         self.mLastSpeed = speed;
         self.mBeforeSpeedDownSpeed = speed;
      }
      // Java: else if (2 == type)
      else if 2 == r#type
      {
         // Java: float speedUp = this.mBeforeSpeedDownSpeed;
         let mut speed_up = self.mBeforeSpeedDownSpeed;
         // Java: for(RawPoint point : this.mSpeedDownPoints)
         for point in self.mSpeedDownPoints.iter()
         {
            // Java: speedUp *= 1.1111112F;
            speed_up *= SPEED_UP_FACTOR;
            // Java: this.mNextHandler.process(point.x, point.y, point.pressure, point.size, speedUp, 3);
            if let Some(ref mut handler) = self.mNextHandler {
                handler.process(point.x, point.y, point.pressure, point.size, speed_up, 3);
            }
         }
         // Java: this.mSpeedDownPoints.clear();
         self.mSpeedDownPoints.clear();
         // Java: this.mNextHandler.process(x, y, pressure, size, speedUp, 2);
         if let Some(ref mut handler) = self.mNextHandler {
             handler.process(x, y, pressure, size, speed_up, 2);
         }
         // Java: this.mLastSpeed = 0.0F;
         self.mLastSpeed = 0.0;
      }
      // Java: else
      else
      {
         // Java: if (speed <= this.mLastSpeed)
         if speed <= self.mLastSpeed
         {
            // Java: this.mSpeedDownPoints.add(RawPoint.obtain(x, y, pressure, size, speed));
            self.mSpeedDownPoints.push_back(RawPoint::obtain(x, y, pressure, size, speed));
         }
         // Java: else
         else
         {
            // Java: if (!this.mSpeedDownPoints.isEmpty())
            if !self.mSpeedDownPoints.is_empty()
            {
               // Java: for(RawPoint point : this.mSpeedDownPoints)
               for point in self.mSpeedDownPoints.iter()
               {
                  // Java: this.mNextHandler.process(point.x, point.y, point.pressure, point.size, point.speed, 1);
                  if let Some(ref mut handler) = self.mNextHandler {
                      handler.process(point.x, point.y, point.pressure, point.size, point.speed, 1);
                  }
                  // Java: point.recycle();
                  point.recycle();
               }
               // Java: this.mSpeedDownPoints.clear();
               self.mSpeedDownPoints.clear();
            }
            // Java: this.mNextHandler.process(x, y, pressure, size, speed, 1);
            if let Some(ref mut handler) = self.mNextHandler {
                handler.process(x, y, pressure, size, speed, 1);
            }
            // Java: this.mBeforeSpeedDownSpeed = speed;
            self.mBeforeSpeedDownSpeed = speed;
         }
         // Java: this.mLastSpeed = speed;
         self.mLastSpeed = speed;
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.mSpeedDownPoints.clear();
      self.mSpeedDownPoints.clear();
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: RawPoint.clear();
      RawPoint::clear();
    }

}

impl IPointProcess for FixSpeedProcess {
    fn process(&mut self, var1: f32, var2: f32, var3: f32, var4: f32, var5: f32, var6: i32) {
        FixSpeedProcess::process(self, var1, var2, var3, var4, var5, var6);
    }
    fn clear(&mut self) {
        FixSpeedProcess::clear(self);
    }
}
