// Java: public class CurvePoint
struct CurvePoint {
    // Java field: float x1
    x1: f32,
    // Java field: float y1
    y1: f32,
    // Java field: float x2
    x2: f32,
    // Java field: float y2
    y2: f32,
}

// Java: public class WritingBrush extends IPainter
pub struct WritingBrush {
    // Java field: Path mPath
    mPath: Vec<(u8, f32, f32, f32, f32)>,
    // Java field: boolean mIsBegin
    mIsBegin: bool,
    // Java field: float mLastX
    mLastX: f32,
    // Java field: float mLastY
    mLastY: f32,
    // Java field: float mStroke
    mStroke: f32,
    // Java field: float mLastStroke
    mLastStroke: f32,
    // Java field: float mLastPointX
    mLastPointX: f32,
    // Java field: float mLastPointY
    mLastPointY: f32,
    // Java field: float mLastPoint2X
    mLastPoint2X: f32,
    // Java field: float mLastPoint2Y
    mLastPoint2Y: f32,
    // Java field: PointF mLastPoint
    mLastPointPx: f32,
    mLastPointPy: f32,
    // Java field: PointF mEndControlPoint
    mEndControlPointX: f32,
    mEndControlPointY: f32,
    mEndControlPointValid: bool,
    // Java field: Vector<PointF> mClosePoints
    mClosePoints: Vec<(f32, f32)>,
    // Java field: float mMoveToX
    mMoveToX: f32,
    // Java field: float mMoveToY
    mMoveToY: f32,
    // Java field: boolean mIsCreating
    mIsCreating: bool,
    // Java field: ArrayList<CurvePoint> mCurvePointCaches
    mCurvePointCaches: Vec<CurvePoint>,
    // Java field: int mEscapedTraces
    mEscapedTraces: i32,
}

// Java: public class WritingBrush extends IPainter
impl WritingBrush {
    // Java: WritingBrush()
    pub fn WritingBrush() -> Self {
        // Java: super();
        Self {
            mPath: Vec::new(), mIsBegin: false, mLastX: 0.0, mLastY: 0.0,
            mStroke: 0.0, mLastStroke: 0.0, mLastPointX: 0.0, mLastPointY: 0.0,
            mLastPoint2X: 0.0, mLastPoint2Y: 0.0, mLastPointPx: 0.0, mLastPointPy: 0.0,
            mEndControlPointX: 0.0, mEndControlPointY: 0.0, mEndControlPointValid: false,
            mClosePoints: Vec::with_capacity(20), mMoveToX: 0.0, mMoveToY: 0.0,
            mIsCreating: false, mCurvePointCaches: Vec::new(), mEscapedTraces: 0,
        }
    }

    // Java: void begin(Path path, float width, float height, float x, float y, float f)
    pub fn begin(&mut self, width: f32, _height: f32, x: f32, y: f32, f: f32) {
        // Java: this.mIsBegin = true;
        self.mIsBegin = true;
        // Java: this.mLastX = x;
        self.mLastX = x;
        // Java: this.mLastY = y;
        self.mLastY = y;
        // Java: this.mStroke = width;
        self.mStroke = width;
        // Java: this.mLastStroke = this.mStroke * f;
        self.mLastStroke = self.mStroke * f;
        // Java: this.mPath = path;
        self.mPath.clear();
        // Java: this.mPath.setFillType(FillType.WINDING);
        // Java: this.mEndControlPoint = null;
        self.mEndControlPointValid = false;
    }

    // Java: void move(float x, float y, float f)
    pub fn move_xyf(&mut self, x: f32, y: f32, f: f32) {
        // Java: if (this.mLastX != x || this.mLastY != y)
        if self.mLastX != x || self.mLastY != y {
            // Java: float dx = x - this.mLastX;
            let dx = x - self.mLastX;
            // Java: float dy = y - this.mLastY;
            let dy = y - self.mLastY;
            // Java: float dxdx_dydy = dx * dx + dy * dy;
            let dxdx_dydy = dx * dx + dy * dy;
            // Java: if (dxdx_dydy < this.mLastStroke * this.mLastStroke / 4.0F)
            if dxdx_dydy < self.mLastStroke * self.mLastStroke / 4.0 {
                // Java: if (this.mIsCreating)
                if self.mIsCreating {
                    // Java: ++this.mEscapedTraces;
                    self.mEscapedTraces += 1;
                }
            // Java: else
            } else {
                // Java: float dxy = (float)Math.sqrt((double)dxdx_dydy);
                let dxy = dxdx_dydy.sqrt();
                // Java: float sin = dy / dxy;
                let sin = dy / dxy;
                // Java: float cos = -dx / dxy;
                let cos = -dx / dxy;
                // Java: float curLastStroke = this.mStroke * f;
                let cur_last_stroke = self.mStroke * f;
                // Java: float deltaX = this.mLastStroke * sin;
                let delta_x = self.mLastStroke * sin;
                // Java: float deltaY = this.mLastStroke * cos;
                let delta_y = self.mLastStroke * cos;
                // Java: float deltaX2 = curLastStroke * sin;
                let delta_x2 = cur_last_stroke * sin;
                // Java: float deltaY2 = curLastStroke * cos;
                let delta_y2 = cur_last_stroke * cos;
                // Java: float p1x = this.mLastX - deltaX;
                let p1x = self.mLastX - delta_x;
                // Java: float p1y = this.mLastY - deltaY;
                let p1y = self.mLastY - delta_y;
                // Java: float p2x = this.mLastX + deltaX;
                let p2x = self.mLastX + delta_x;
                // Java: float p2y = this.mLastY + deltaY;
                let p2y = self.mLastY + delta_y;
                // Java: if (this.mIsBegin)
                if self.mIsBegin {
                    // Java: this.mPath.moveTo(p2x, p2y);
                    self.mPath.push((0, p2x, p2y, 0.0, 0.0));
                    // Java: float x1 = this.mLastX + 2.0F * deltaY;
                    let x1 = self.mLastX + 2.0 * delta_y;
                    // Java: float y1 = this.mLastY - 2.0F * deltaX;
                    let y1 = self.mLastY - 2.0 * delta_x;
                    // Java: if (this.mIsCreating)
                    if self.mIsCreating {
                        // Java: this.mMoveToX = p2x;
                        self.mMoveToX = p2x;
                        // Java: this.mMoveToY = p2y;
                        self.mMoveToY = p2y;
                        // Java: this.mCurvePointCaches.add(new CurvePoint(x1, y1, p1x, p1y));
                        self.mCurvePointCaches.push(CurvePoint { x1, y1, x2: p1x, y2: p1y });
                    }
                    // Java: this.mPath.quadTo(x1, y1, p1x, p1y);
                    self.mPath.push((1, x1, y1, p1x, p1y));
                    // Java: this.mLastPointX = p1x;
                    self.mLastPointX = p1x;
                    // Java: this.mLastPointY = p1y;
                    self.mLastPointY = p1y;
                    // Java: this.mIsBegin = false;
                    self.mIsBegin = false;
                    // Java: this.mClosePoints.addElement(new PointF(p2x, p2y));
                    self.mClosePoints.push((p2x, p2y));
                // Java: else
                } else {
                    // Java: if (this.mEndControlPoint == null)
                    if !self.mEndControlPointValid {
                        // Java: this.mEndControlPoint = new PointF();
                        self.mEndControlPointValid = true;
                    }
                    // Java: this.mEndControlPoint.x = x - 2.0F * deltaY2;
                    self.mEndControlPointX = x - 2.0 * delta_y2;
                    // Java: this.mEndControlPoint.y = y + 2.0F * deltaX2;
                    self.mEndControlPointY = y + 2.0 * delta_x2;
                    // Java: this.mLastPoint.x = (this.mLastPoint.x + p2x) / 2.0F;
                    self.mLastPointPx = (self.mLastPointPx + p2x) / 2.0;
                    // Java: this.mLastPoint.y = (this.mLastPoint.y + p2y) / 2.0F;
                    self.mLastPointPy = (self.mLastPointPy + p2y) / 2.0;
                    // Java: float cx = (this.mLastPoint2X + p1x) / 2.0F;
                    let cx = (self.mLastPoint2X + p1x) / 2.0;
                    // Java: float cy = (this.mLastPoint2Y + p1y) / 2.0F;
                    let cy = (self.mLastPoint2Y + p1y) / 2.0;
                    // Java: float x2 = (this.mLastPointX + cx) / 2.0F;
                    let x2 = (self.mLastPointX + cx) / 2.0;
                    // Java: float y2 = (this.mLastPointY + cy) / 2.0F;
                    let y2 = (self.mLastPointY + cy) / 2.0;
                    // Java: if (this.mIsCreating)
                    if self.mIsCreating {
                        // Java: this.mCurvePointCaches.add(new CurvePoint(this.mLastPointX, this.mLastPointY, x2, y2));
                        self.mCurvePointCaches.push(CurvePoint { x1: self.mLastPointX, y1: self.mLastPointY, x2, y2 });
                    }
                    // Java: this.mPath.quadTo(this.mLastPointX, this.mLastPointY, x2, y2);
                    self.mPath.push((1, self.mLastPointX, self.mLastPointY, x2, y2));
                    // Java: this.mLastPointX = cx;
                    self.mLastPointX = cx;
                    // Java: this.mLastPointY = cy;
                    self.mLastPointY = cy;
                }
                // Java: this.mLastPoint2X = x - deltaX2;
                self.mLastPoint2X = x - delta_x2;
                // Java: this.mLastPoint2Y = y - deltaY2;
                self.mLastPoint2Y = y - delta_y2;
                // Java: this.mLastPoint = new PointF(x + deltaX2, y + deltaY2);
                self.mLastPointPx = x + delta_x2;
                self.mLastPointPy = y + delta_y2;
                // Java: this.mClosePoints.addElement(this.mLastPoint);
                self.mClosePoints.push((self.mLastPointPx, self.mLastPointPy));
                // Java: this.mLastStroke = curLastStroke;
                self.mLastStroke = cur_last_stroke;
                // Java: this.mLastX = x;
                self.mLastX = x;
                // Java: this.mLastY = y;
                self.mLastY = y;
            }
        }
    }

    // Java: static boolean isAngleLessThan30(float pointA_x, float pointA_y, float pointB_x, float pointB_y, float pointC_x, float pointC_y)
    pub fn isAngleLessThan30(pointA_x: f32, pointA_y: f32, pointB_x: f32, pointB_y: f32, pointC_x: f32, pointC_y: f32) -> bool {
        // Java: double dotProduct = (double)((pointB_x - pointA_x) * (pointC_x - pointA_x) + (pointB_y - pointA_y) * (pointC_y - pointA_y));
        let dot_product = ((pointB_x - pointA_x) * (pointC_x - pointA_x) + (pointB_y - pointA_y) * (pointC_y - pointA_y)) as f64;
        // Java: double magnitudeAB = Math.sqrt(Math.pow((double)(pointB_x - pointA_x), (double)2.0F) + Math.pow((double)(pointB_y - pointA_y), (double)2.0F));
        let magnitude_ab = (((pointB_x - pointA_x) as f64).powi(2) + ((pointB_y - pointA_y) as f64).powi(2)).sqrt();
        // Java: double magnitudeAC = Math.sqrt(Math.pow((double)(pointC_x - pointA_x), (double)2.0F) + Math.pow((double)(pointC_y - pointA_y), (double)2.0F));
        let magnitude_ac = (((pointC_x - pointA_x) as f64).powi(2) + ((pointC_y - pointA_y) as f64).powi(2)).sqrt();
        // Java: double cosAngle = dotProduct / (magnitudeAB * magnitudeAC);
        let cos_angle = dot_product / (magnitude_ab * magnitude_ac);
        // Java: double angle = Math.acos(cosAngle) * (180D / Math.PI);
        let angle = cos_angle.acos() * (180.0 / std::f64::consts::PI);
        // Java: return angle < (double)45.0F;
        angle < 45.0
    }

    // Java: void end(boolean completed)
    pub fn end_completed(&mut self, completed: bool) {
        // Java: this.mPath.quadTo(this.mLastPointX, this.mLastPointY, this.mLastPoint2X, this.mLastPoint2Y);
        self.mPath.push((1, self.mLastPointX, self.mLastPointY, self.mLastPoint2X, self.mLastPoint2Y));
        // Java: if (!this.mClosePoints.isEmpty())
        if !self.mClosePoints.is_empty() {
            // Java: float lastPointX = this.mLastX;
            let mut last_point_x = self.mLastX;
            // Java: float lastPointY = this.mLastY;
            let mut last_point_y = self.mLastY;
            // Java: PointF p = null;
            // Java: int size = this.mClosePoints.size();
            let size = self.mClosePoints.len() as i32;
            // Java: int index = size - 1;
            let mut index = size - 1;
            // Java: if (completed && size >= 2 && this.mEndControlPoint != null)
            if completed && size >= 2 && self.mEndControlPointValid {
                // Java: PointF var12 = (PointF)this.mClosePoints.elementAt(size - 1);
                let (cur_px, cur_py) = self.mClosePoints[(size - 1) as usize];
                // Java: float curPointX = var12.x;
                // Java: float curPointY = var12.y;
                // Java: this.mPath.quadTo(this.mEndControlPoint.x, this.mEndControlPoint.y, curPointX, curPointY);
                self.mPath.push((1, self.mEndControlPointX, self.mEndControlPointY, cur_px, cur_py));
                // Java: p = (PointF)this.mClosePoints.elementAt(size - 2);
                let (px, py) = self.mClosePoints[(size - 2) as usize];
                // Java: lastPointX = p.x;
                last_point_x = px;
                // Java: lastPointY = p.y;
                last_point_y = py;
                // Java: index = size - 3;
                index = size - 3;
            }
            // Java: for(int i = index; i >= 0; --i)
            let mut i = index;
            let mut last_px = last_point_x;
            let mut last_py = last_point_y;
            while i >= 0 {
                // Java: p = (PointF)this.mClosePoints.elementAt(i);
                let (cur_px, cur_py) = self.mClosePoints[i as usize];
                // Java: float curPointX = p.x;
                // Java: float curPointY = p.y;
                // Java: this.mPath.quadTo(lastPointX, lastPointY, (lastPointX + curPointX) / 2.0F, (lastPointY + curPointY) / 2.0F);
                self.mPath.push((1, last_point_x, last_point_y, (last_point_x + cur_px) / 2.0, (last_point_y + cur_py) / 2.0));
                // Java: lastPointX = curPointX;
                last_point_x = cur_px;
                // Java: lastPointY = curPointY;
                last_point_y = cur_py;
                last_px = cur_px;
                last_py = cur_py;
                i -= 1;
            }
            // Java: if (!this.mIsCreating)
            if !self.mIsCreating {
                // Java: this.mClosePoints.clear();
                self.mClosePoints.clear();
                // Java: this.mLastPointX = lastPointX;
                self.mLastPointX = last_point_x;
                // Java: this.mLastPointY = lastPointY;
                self.mLastPointY = last_point_y;
            }
            // Java: this.mPath.lineTo(p.x, p.y);
            self.mPath.push((2, last_px, last_py, 0.0, 0.0));
        }
    }

    // Java: void draw(Canvas canvas, Paint paint)
    pub fn draw(&self, _canvas: i64, _paint: i64) {
        // Java: if (!this.mIsBegin)
        if !self.mIsBegin {
            // Java: canvas.drawPath(this.mPath, paint);
        }
    }

    // Java: void setCreating(boolean isCreating)
    pub fn setCreating(&mut self, isCreating: bool) {
        // Java: this.mIsCreating = isCreating;
        self.mIsCreating = isCreating;
        // Java: this.mCurvePointCaches = new ArrayList(20);
        self.mCurvePointCaches = Vec::with_capacity(20);
        // Java: this.mEscapedTraces = 0;
        self.mEscapedTraces = 0;
    }

    // Java: int getCacheSize()
    pub fn getCacheSize(&self) -> i32 {
        // Java: return this.mCurvePointCaches.size();
        self.mCurvePointCaches.len() as i32
    }

    // Java: int getEscapedTraces()
    pub fn getEscapedTraces(&self) -> i32 {
        // Java: return this.mEscapedTraces;
        self.mEscapedTraces
    }

    // Java: void drawCache()
    pub fn drawCache(&mut self) {
        // Java: this.mPath.moveTo(this.mMoveToX, this.mMoveToY);
        self.mPath.push((0, self.mMoveToX, self.mMoveToY, 0.0, 0.0));
        // Java: int cacheSize = this.mCurvePointCaches.size();
        let cache_size = self.mCurvePointCaches.len();
        // Java: CurvePoint p = null;
        // Java: for(int i = 0; i < cacheSize; ++i)
        for i in 0..cache_size {
            // Java: p = (CurvePoint)this.mCurvePointCaches.get(i);
            let p = &self.mCurvePointCaches[i];
            // Java: this.mPath.quadTo(p.x1, p.y1, p.x2, p.y2);
            self.mPath.push((1, p.x1, p.y1, p.x2, p.y2));
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mClosePoints.clear();
        self.mClosePoints.clear();
        // Java: if (null != this.mCurvePointCaches)
        // Java: this.mCurvePointCaches.clear();
        self.mCurvePointCaches.clear();
    }
}

impl CurvePoint {
    // Java: CurvePoint(float x1, float y1, float x2, float y2)
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        // Java: super();
        // Java: this.x1 = x1;
        // Java: this.y1 = y1;
        // Java: this.x2 = x2;
        // Java: this.y2 = y2;
        Self { x1, y1, x2, y2 }
    }
}
