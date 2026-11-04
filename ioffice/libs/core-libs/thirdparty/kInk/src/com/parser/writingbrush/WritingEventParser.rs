use std::collections::VecDeque;
use super::IPointProcess::IPointProcess;
use super::StrokeNormalizer::StrokeNormalizer;
use crate::com::parser::IInkStroker::IInkStroker;
// Java: import android.os.Build.VERSION
// Java: import android.view.MotionEvent
// Java: import android.view.VelocityTracker
// Java: import com.hp.hpl.inkml.impl.InkEstimate
// Java: import com.parser.IEventParser
// Java: import com.parser.IInkStroker
// Java: import java.util.LinkedList

pub struct MotionEvent;
pub struct VelocityTracker;

struct WritingPoint {
    x: f32,
    y: f32,
    f: f32,
}

impl WritingPoint {
    fn obtain(x: f32, y: f32, f: f32) -> WritingPoint {
        WritingPoint { x, y, f }
    }
    fn recycle(&self) {}
    fn clear() {}
}

// Java field: int MAX_STORED_SIZE
pub(crate) const MAX_STORED_SIZE: i32 = 10;
// Java field: float POSITION_DECAY
pub(crate) const POSITION_DECAY: f32 = 0.65;
// Java field: float PRESSURE_DECAY
pub(crate) const PRESSURE_DECAY: f32 = 0.9;
// Java field: float DRAG_LENGTH
pub(crate) const DRAG_LENGTH: f32 = 1.2;
// Java field: float DRAG_DEGREE_DELTA
pub(crate) const DRAG_DEGREE_DELTA: f32 = 0.4;
// Java field: float COS_DELTA
pub(crate) const COS_DELTA: f32 = 0.9486833;
// Java field: float SIN_DELTA
pub(crate) const SIN_DELTA: f32 = 0.31622776;
// Java field: float LOW_STROKE_F
pub(crate) const LOW_STROKE_F: f32 = 0.25;
// Java field: float HIGH_STROKE_F
pub(crate) const HIGH_STROKE_F: f32 = 0.75;

// Java: public class WritingEventParser implements IEventParser, IPointProcess
pub struct WritingEventParser {
    // Java field: LinkedList<WritingPoint> mStoredPoints
    mStoredPoints: VecDeque<WritingPoint>,
    // Java field: StrokeNormalizer mStrokeNormalizer
    mStrokeNormalizer: StrokeNormalizer,
    // Java field: VelocityTracker mVelocityTracker
    mVelocityTracker: VelocityTracker,
    // Java field: IInkStroker mStroker
    mStroker: Box<dyn IInkStroker>,
    // Java field: float mLowStroke
    mLowStroke: f32,
    // Java field: float mHighStroke
    mHighStroke: f32,
    // Java field: float mDip
    mDip: f32,
    // Java field: float mLastSpeed
    mLastSpeed: f32,
    // Java field: float mDx
    mDx: f32,
    // Java field: float mDy
    mDy: f32,
    // Java field: float mLastX
    mLastX: f32,
    // Java field: float mLastY
    mLastY: f32,
    // Java field: float mLastF
    mLastF: f32,
    // Java field: boolean mIsStrokeLimitOnFinish
    pub(crate) mIsStrokeLimitOnFinish: bool,
    // Java field: boolean mIsStylus
    mIsStylus: bool,
}

// Java: public class WritingEventParser implements IEventParser, IPointProcess
impl WritingEventParser {
    // Java: WritingEventParser(IInkStroker stroker, float dip)
    pub fn WritingEventParser(stroker: Box<dyn IInkStroker>, dip: f32) -> Self {
      // Java: super();
      // Java: this.mDip = dip;
      // Java: this.mStroker = stroker;
      // Java: this.mVelocityTracker = VelocityTracker.obtain();
      // Java: this.mStoredPoints = new LinkedList();
      // Java: this.mStrokeNormalizer = new StrokeNormalizer();
      Self {
          mStoredPoints: VecDeque::new(),
          mStrokeNormalizer: StrokeNormalizer::StrokeNormalizer(),
          mVelocityTracker: VelocityTracker,
          mStroker: stroker,
          mLowStroke: 0.25,
          mHighStroke: 0.75,
          mDip: dip,
          mLastSpeed: 0.0,
          mDx: 0.0,
          mDy: 0.0,
          mLastX: -1.0,
          mLastY: -1.0,
          mLastF: 0.0,
          mIsStrokeLimitOnFinish: false,
          mIsStylus: false,
      }
    }

    // Java: void setLowStrokeF(float val)
    pub fn setLowStrokeF(&mut self, val: f32) {
      // Java: this.mLowStroke = val;
      self.mLowStroke = val;
    }

    // Java: void setHighStrokeF(float val)
    pub fn setHighStrokeF(&mut self, val: f32) {
      // Java: this.mHighStroke = val;
      self.mHighStroke = val;
    }

    // Java: void parseEvent(MotionEvent event)
    pub fn parseEvent(&mut self, _event: &MotionEvent) {
      // Java: if (2 == (event.getAction() & 255))
      {
         // Java: int i = 0;
         // Java: for(int historySize = event.getHistorySize(); i < historySize; ++i)
         {
            // Java: MotionEvent historyEvent = this.obtainFromHistory(event, i);
            // Java: this.doParseEvent(historyEvent);
            // Java: historyEvent.recycle();
         }
      }
      // Java: this.doParseEvent(event);
    }

    // Java: void doParseEvent(MotionEvent event)
    pub fn doParseEvent(&mut self, _event: &MotionEvent) {
      // Java: int action = event.getAction() & 255;
      let action: i32 = 0;
      // Java: boolean isDown = action == 0;
      let is_down = action == 0;
      // Java: boolean isMove = action == 2;
      let is_move = action == 2;
      // Java: if (isDown)
      if is_down
      {
         // Java: this.mIsStylus = VERSION.SDK_INT > 14 && event.getToolType(0) == 2;
         self.mIsStylus = false;
      }
      // Java: int type = isDown ? 0 : (isMove ? 1 : 2);
      let r#type = if is_down { 0 } else { if is_move { 1 } else { 2 } };
      // Java: if (this.mIsStylus)
      if self.mIsStylus
      {
         // Java: this.process(event.getX(), event.getY(), event.getPressure(), event.getSize(), 0.0F, type);
         self.process(0.0, 0.0, 0.0, 0.0, 0.0, r#type);
      }
      // Java: else
      else
      {
         // Java: float speed = this.getSpeed(event);
         let speed = self.getSpeed(_event);
         // Java: this.process(event.getX(), event.getY(), 0.0F, event.getSize(), speed, type);
         self.process(0.0, 0.0, 0.0, 0.0, speed, r#type);
      }
      // Java: InkEstimate inkEstimate = InkEstimate.getInstance();
      // Java: if (inkEstimate != null)
      {
         // Java: if (isMove)
         if is_move
         {
            // Java: inkEstimate.addPoint(event);
         }
         // Java: else if (isDown)
         else if is_down
         {
            // Java: inkEstimate.onTouchDown(event);
         }
      }
    }

    // Java: MotionEvent obtainFromHistory(MotionEvent event, int historyIndex)
    pub fn obtainFromHistory(&self, _event: &MotionEvent, _history_index: i32) -> MotionEvent {
      // Java: return MotionEvent.obtain(event.getDownTime(), event.getHistoricalEventTime(historyIndex), 2, event.getHistoricalX(historyIndex), event.getHistoricalY(historyIndex), event.getHistoricalPressure(historyIndex), event.getHistoricalSize(historyIndex), event.getMetaState(), event.getXPrecision(), event.getYPrecision(), event.getDeviceId(), event.getEdgeFlags());
      MotionEvent
    }

    // Java: void process(float x, float y, float pressure, float size, float speed, int type)
    pub fn process(&mut self, x: f32, y: f32, pressure: f32, _size: f32, speed: f32, r#type: i32) {
      // Java: boolean isNormal = !this.mIsStylus;
      let is_normal = !self.mIsStylus;
      // Java: float f = this.mStrokeNormalizer.getAdjustedWidth(isNormal, pressure, speed);
      let mut f = self.mStrokeNormalizer.getAdjustedWidth(is_normal, pressure, speed);
      // Java: if (type == 0 || type == 1 || this.mIsStrokeLimitOnFinish)
      if r#type == 0 || r#type == 1 || self.mIsStrokeLimitOnFinish
      {
         // Java: float lowStrokeF = isNormal ? this.mLowStroke : this.mLowStroke / 2.0F;
         let low_stroke_f = if is_normal { self.mLowStroke } else { self.mLowStroke / 2.0 };
         // Java: f = lowStrokeF + f * (1.0F - lowStrokeF);
         f = low_stroke_f + f * (1.0 - low_stroke_f);
      }
      // Java: float highStorkeF = type == 0 && isNormal ? this.mHighStroke / 3.0F : this.mHighStroke;
      let high_storke_f = if r#type == 0 && is_normal { self.mHighStroke / 3.0 } else { self.mHighStroke };
      // Java: if (f > highStorkeF)
      if f > high_storke_f
      {
         // Java: f = highStorkeF + (f - highStorkeF) / 3.0F;
         f = high_storke_f + (f - high_storke_f) / 3.0;
      }
      // Java: WritingPoint newPoint = WritingPoint.obtain(x, y, f);
      let new_point = WritingPoint::obtain(x, y, f);
      // Java: while(this.mStoredPoints.size() >= 10)
      while self.mStoredPoints.len() >= 10
      {
         // Java: ((WritingPoint)this.mStoredPoints.removeFirst()).recycle();
         if let Some(p) = self.mStoredPoints.pop_front() { p.recycle(); }
      }
      // Java: this.mStoredPoints.add(newPoint);
      self.mStoredPoints.push_back(new_point);
      // Java: WritingPoint generatedPoint = this.generateFromStoredPoints();
      let generated_point = self.generateFromStoredPoints();
      // Java: if (0 == type)
      if 0 == r#type
      {
         // Java: this.mStroker.onStart(generatedPoint.x, generatedPoint.y, generatedPoint.f);
         self.mStroker.onStart(generated_point.x, generated_point.y, generated_point.f);
         // Java: generatedPoint.recycle();
         generated_point.recycle();
      }
      // Java: else
      else
      {
         // Java: this.onMove(generatedPoint.x, generatedPoint.y, generatedPoint.f);
         let gx = generated_point.x;
         let gy = generated_point.y;
         let gf = generated_point.f;
         // Java: generatedPoint.recycle();
         generated_point.recycle();
         self.on_move(gx, gy, gf);
         // Java: if (2 == type)
         if 2 == r#type
         {
            // Java: this.onFinish();
            self.on_finish();
         }
      }
    }

    // Java: void touchCancel()
    pub fn touchCancel(&mut self) {
      // Java: this.clear();
      self.mStoredPoints.clear();
      // Java: if (this.mStroker != null)
      {
         // Java: this.mStroker.onFinish();
         self.mStroker.onFinish();
      }
      // Java: if (this.mVelocityTracker != null)
      {
         // Java: this.mVelocityTracker.clear();
      }
      // Java: this.mLastSpeed = 0.0F;
      self.mLastSpeed = 0.0;
      // Java: this.mDx = 0.0F;
      self.mDx = 0.0;
      // Java: this.mDy = 0.0F;
      self.mDy = 0.0;
      // Java: this.mLastX = -1.0F;
      self.mLastX = -1.0;
      // Java: this.mLastY = -1.0F;
      self.mLastY = -1.0;
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.mStoredPoints.clear();
      self.mStoredPoints.clear();
    }

    // Java: void onFinish()
    fn on_finish(&mut self) {
      // Java: while(this.mStoredPoints.size() > 0)
      while self.mStoredPoints.len() > 0
      {
         // Java: WritingPoint point = this.generateFromStoredPoints();
         let point = self.generateFromStoredPoints();
         // Java: this.onMove(point.x, point.y, point.f);
         let px = point.x;
         let py = point.y;
         let pf = point.f;
         // Java: point.recycle();
         point.recycle();
         self.on_move(px, py, pf);
         // Java: ((WritingPoint)this.mStoredPoints.removeFirst()).recycle();
         if let Some(p) = self.mStoredPoints.pop_front() { p.recycle(); }
      }
      // Java: float dxy = (float)Math.sqrt((double)(this.mDx * this.mDx + this.mDy * this.mDy));
      let dxy = ((self.mDx * self.mDx + self.mDy * self.mDy) as f64).sqrt() as f32;
      // Java: float lastSin = 0.0F;
      let mut last_sin: f32 = 0.0;
      // Java: float lastCos = 0.0F;
      let mut last_cos: f32 = 0.0;
      // Java: if (0.0F != dxy)
      if 0.0 != dxy
      {
         // Java: lastSin = -this.mDy / dxy;
         last_sin = -self.mDy / dxy;
         // Java: lastCos = this.mDx / dxy;
         last_cos = self.mDx / dxy;
      }
      // Java: float cos = lastCos * COS_DELTA + lastSin * SIN_DELTA;
      let cos = last_cos * COS_DELTA + last_sin * SIN_DELTA;
      // Java: float sin = lastSin * COS_DELTA - lastCos * SIN_DELTA;
      let sin = last_sin * COS_DELTA - last_cos * SIN_DELTA;
      // Java: float x = this.mLastX + 1.2F * this.mStroker.getStrokeWidth() * this.mLastF * cos;
      let x = self.mLastX + 1.2 * self.mStroker.getStrokeWidth() * self.mLastF * cos;
      // Java: float y = this.mLastY - 1.2F * this.mStroker.getStrokeWidth() * this.mLastF * sin;
      let y = self.mLastY - 1.2 * self.mStroker.getStrokeWidth() * self.mLastF * sin;
      // Java: this.quadTo(this.mLastX, this.mLastY, this.mLastF, x, y, this.mLastF / 100.0F);
      self.quadTo(self.mLastX, self.mLastY, self.mLastF, x, y, self.mLastF / 100.0);
      // Java: this.mStroker.onFinish();
      self.mStroker.onFinish();
      // Java: this.mVelocityTracker.clear();
      // Java: this.mLastSpeed = 0.0F;
      self.mLastSpeed = 0.0;
      // Java: this.mDx = 0.0F;
      self.mDx = 0.0;
      // Java: this.mDy = 0.0F;
      self.mDy = 0.0;
      // Java: this.mLastX = -1.0F;
      self.mLastX = -1.0;
      // Java: this.mLastY = -1.0F;
      self.mLastY = -1.0;
    }

    // Java: void quadTo(float x1, float y1, float f1, float x2, float y2, float f2)
    pub fn quadTo(&mut self, x1: f32, y1: f32, f1: f32, x2: f32, y2: f32, f2: f32) {
      // Java: for(float t = 0.2F; t < 1.1F; t += 0.1F)
      let mut t: f32 = 0.2;
      while t < 1.1
      {
         // Java: float x = x1 + (x2 - x1) * t;
         let x = x1 + (x2 - x1) * t;
         // Java: float y = y1 + (y2 - y1) * t;
         let y = y1 + (y2 - y1) * t;
         // Java: float f = f1 + (f2 - f1) * t * t * t;
         let f = f1 + (f2 - f1) * t * t * t;
         // Java: this.mStroker.onMove(x, y, f);
         self.mStroker.onMove(x, y, f);
         t += 0.1;
      }
    }

    // Java: void onMove(float x, float y, float f)
    fn on_move(&mut self, x: f32, y: f32, f: f32) {
      // Java: if (this.mLastX < 0.0F && this.mLastY < 0.0F)
      if self.mLastX < 0.0 && self.mLastY < 0.0
      {
         // Java: this.mDx = 0.0F;
         self.mDx = 0.0;
         // Java: this.mDy = 0.0F;
         self.mDy = 0.0;
         // Java: this.mLastX = x;
         self.mLastX = x;
         // Java: this.mLastY = y;
         self.mLastY = y;
         // Java: this.mLastF = f;
         self.mLastF = f;
         // Java: this.mStroker.onMove(x, y, f);
         self.mStroker.onMove(x, y, f);
      }
      // Java: else
      else
      {
         // Java: float dx = x - this.mLastX;
         let dx = x - self.mLastX;
         // Java: float dy = y - this.mLastY;
         let dy = y - self.mLastY;
         // Java: float lastStroke = this.mLastF * this.mStroker.getStrokeWidth();
         let last_stroke = self.mLastF * self.mStroker.getStrokeWidth();
         // Java: if (!(dx * dx + dy * dy < lastStroke * lastStroke))
         if !(dx * dx + dy * dy < last_stroke * last_stroke)
         {
            // Java: this.mDx = dx;
            self.mDx = dx;
            // Java: this.mDy = dy;
            self.mDy = dy;
            // Java: this.mLastX = x;
            self.mLastX = x;
            // Java: this.mLastY = y;
            self.mLastY = y;
            // Java: this.mLastF = f;
            self.mLastF = f;
            // Java: this.mStroker.onMove(x, y, f);
            self.mStroker.onMove(x, y, f);
         }
      }
    }

    // Java: float getSpeed(MotionEvent event)
    pub fn getSpeed(&mut self, _event: &MotionEvent) -> f32 {
      // Java: this.mVelocityTracker.addMovement(event);
      // Java: this.mVelocityTracker.computeCurrentVelocity(1000);
      // Java: float speed = (float)Math.sqrt(Math.pow((double)this.mVelocityTracker.getXVelocity(), (double)2.0F) + Math.pow((double)this.mVelocityTracker.getYVelocity(), (double)2.0F)) / this.mDip;
      let speed: f32 = 0.0;
      // Java: if (speed == 0.0F)
      if speed == 0.0
      {
         // Java: return this.mLastSpeed;
         return self.mLastSpeed;
      }
      // Java: else
      else
      {
         // Java: this.mLastSpeed = speed;
         self.mLastSpeed = speed;
         // Java: return speed;
         return speed;
      }
    }

    // Java: WritingPoint generateFromStoredPoints()
    pub fn generateFromStoredPoints(&self) -> WritingPoint {
      // Java: float wi = 1.0F;
      let mut wi: f32 = 1.0;
      // Java: float wiSum = 0.0F;
      let mut wi_sum: f32 = 0.0;
      // Java: float ri = 1.0F;
      let mut ri: f32 = 1.0;
      // Java: float riSum = 0.0F;
      let mut ri_sum: f32 = 0.0;
      // Java: float x = 0.0F;
      let mut x: f32 = 0.0;
      // Java: float y = 0.0F;
      let mut y: f32 = 0.0;
      // Java: float r = 0.0F;
      let mut r: f32 = 0.0;
      // Java: for(int i = this.mStoredPoints.size() - 1; i >= 0; --i)
      for i in (0..self.mStoredPoints.len()).rev()
      {
         // Java: WritingPoint point = (WritingPoint)this.mStoredPoints.get(i);
         let point = &self.mStoredPoints[i];
         // Java: x += point.x * wi;
         x += point.x * wi;
         // Java: y += point.y * wi;
         y += point.y * wi;
         // Java: r += point.f * ri;
         r += point.f * ri;
         // Java: wiSum += wi;
         wi_sum += wi;
         // Java: wi *= 0.65F;
         wi *= 0.65;
         // Java: riSum += ri;
         ri_sum += ri;
         // Java: ri *= 0.9F;
         ri *= 0.9;
      }
      // Java: return WritingPoint.obtain(x / wiSum, y / wiSum, r / riSum);
      WritingPoint::obtain(x / wi_sum, y / wi_sum, r / ri_sum)
    }

    // Java: boolean isIsStrokeLimitOnFinish()
    pub fn isIsStrokeLimitOnFinish(&self) -> bool {
      // Java: return this.mIsStrokeLimitOnFinish;
      self.mIsStrokeLimitOnFinish
    }

    // Java: void setIsStrokeLimitOnFinish(boolean isStrokeLimitOnFinish)
    pub fn setIsStrokeLimitOnFinish(&mut self, is_stroke_limit_on_finish: bool) {
      // Java: this.mIsStrokeLimitOnFinish = isStrokeLimitOnFinish;
      self.mIsStrokeLimitOnFinish = is_stroke_limit_on_finish;
    }

    // Java: void dispose()
    pub fn dispose(&self) {
      // Java: WritingPoint.clear();
      WritingPoint::clear();
    }

}

impl IPointProcess for WritingEventParser {
    fn process(&mut self, var1: f32, var2: f32, var3: f32, var4: f32, var5: f32, var6: i32) {
        self.process(var1, var2, var3, var4, var5, var6);
    }
    fn clear(&mut self) {
        self.clear();
    }
}
