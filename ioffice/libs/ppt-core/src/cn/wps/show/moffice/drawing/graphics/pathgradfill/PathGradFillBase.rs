// Java: import android.graphics.Canvas
use thirdparty_kInk::com::hp::hpl::inkml::Canvas;
// Java: import android.graphics.LinearGradient
use crate::android::graphics::LinearGradient;
// Java: import android.graphics.Matrix
use crate::android::graphics::Matrix;
// Java: import android.graphics.Paint
use crate::android::graphics::Paint;
// Java: import android.graphics.Path
use crate::android::graphics::Path;
// Java: import android.graphics.PointF
use util::cn::wps::graphics::PointF;
// Java: import android.graphics.RadialGradient
use crate::android::graphics::RadialGradient;
// Java: import android.graphics.RectF
use util::cn::wps::graphics::RectF;
// Java: import android.graphics.Shader
use crate::android::graphics::Shader;
// Java: import android.graphics.Paint.Style
use crate::android::graphics::Paint::Style;
// Java: import android.graphics.Path.Direction
use crate::android::graphics::Path::Direction;
// Java: import android.graphics.Shader.TileMode
use crate::android::graphics::Shader::TileMode;
// Java: import cn.wps.show.moffice.drawing.util.AnyObjPool
use crate::cn::wps::show::moffice::drawing::util::AnyObjPool;

// Java: public class PathGradFillBase
pub struct PathGradFillBase {
    // Java field: AnyObjPool objPool
    pub(crate) objPool: Arc<Mutex<AnyObjPool>>,
    // Java field: Path path
    pub(crate) path: Arc<Mutex<Path>>,
    // Java field: Path adjustPath
    pub(crate) adjustPath: Arc<Mutex<Path>>,
    // Java field: Canvas canvas
    pub(crate) canvas: Arc<Mutex<Canvas>>,
    // Java field: Paint fillPaint
    pub(crate) fillPaint: Arc<Mutex<Paint>>,
    // Java field: int[] colors
    pub(crate) colors: Arc<Mutex<Vec<i32>>>,
    // Java field: float[] positions
    pub(crate) positions: Arc<Mutex<Vec<f32>>>,
    // Java field: RectF dstRect
    pub(crate) dstRect: Arc<Mutex<RectF>>,
    // Java field: RectF fillToRect
    pub(crate) fillToRect: Arc<Mutex<RectF>>,
    // Java field: RectF tileRect
    pub(crate) tileRect: Arc<Mutex<RectF>>,
    // Java field: RectF oriDstRect
    oriDstRect: Option<RectF>,
    // Java field: RectF oriFillToRect
    oriFillToRect: Option<RectF>,
    // Java field: RectF oriTileRect
    oriTileRect: Option<RectF>,
    // Java field: RectF shapeRect
    pub(crate) shapeRect: Arc<Mutex<RectF>>,
    // Java field: int shapeType
    pub(crate) shapeType: i32,
    // Java field: PointF[] points
    pub(crate) points: Arc<Mutex<Vec<PointF>>>,
    // Java field: PointF tempCenterF
    tempCenterF: Option<PointF>,
    // Java field: float rotation
    pub(crate) rotation: f32,
    // Java field: boolean rotWithShape
    pub(crate) rotWithShape: bool,
    // Java field: boolean useBgFill
    pub(crate) useBgFill: bool,
}

// Java: public class PathGradFillBase
impl PathGradFillBase {
    // Java: void init(Path path, Canvas canvas, Paint fillPaint, RectF dstRect, RectF fillToRect, RectF tileRect, RectF shapeRect, int[] colors, float[] positions, boolean useBgFill)
    pub fn init(&self, Path path, Canvas canvas, Paint fillPaint, RectF dstRect, RectF fillToRect, RectF tileRect, RectF shapeRect, int[] colors, float[] positions, boolean useBgFill) {
      // Java: this.objPool = AnyObjPool.getPool();
      // TODO_RUST_CODE
      // Java: this.path = path;
      // TODO_RUST_CODE
      // Java: this.canvas = canvas;
      // TODO_RUST_CODE
      // Java: this.fillPaint = fillPaint;
      // TODO_RUST_CODE
      // Java: this.colors = colors;
      // TODO_RUST_CODE
      // Java: this.positions = positions;
      // TODO_RUST_CODE
      // Java: this.oriDstRect = dstRect;
      // TODO_RUST_CODE
      // Java: this.oriFillToRect = fillToRect;
      // TODO_RUST_CODE
      // Java: this.oriTileRect = tileRect;
      // TODO_RUST_CODE
      // Java: this.useBgFill = useBgFill;
      // TODO_RUST_CODE
      // Java: this.shapeRect = shapeRect;
      // TODO_RUST_CODE
      // Java: if (this.shapeRect == null && this.oriDstRect != null)
        // TODO_RUST_CODE
      {
         // Java: this.shapeRect = this.objPool.getRectF(this.oriDstRect.left, this.oriDstRect.top, this.oriDstRect.right, this.oriDstRect.bottom);
         // TODO_RUST_CODE
      }
      // Java: this.rotWithShape = !useBgFill;
      // TODO_RUST_CODE
      // Java: this.rotation = 0.0F;
      // TODO_RUST_CODE
      // Java: this.shapeType = 0;
      // TODO_RUST_CODE
    }

    // Java: void dispose()
    pub fn dispose(&self) {
      // Java: this.objPool.tryReuse(this.dstRect);
      // TODO_RUST_CODE
      // Java: this.dstRect = null;
      // TODO_RUST_CODE
      // Java: this.objPool.tryReuse(this.fillToRect);
      // TODO_RUST_CODE
      // Java: this.fillToRect = null;
      // TODO_RUST_CODE
      // Java: this.objPool.tryReuse(this.tileRect);
      // TODO_RUST_CODE
      // Java: this.tileRect = null;
      // TODO_RUST_CODE
      // Java: this.objPool.tryReuse(this.oriDstRect);
      // TODO_RUST_CODE
      // Java: this.oriDstRect = null;
      // TODO_RUST_CODE
      // Java: this.objPool.tryReuse(this.oriFillToRect);
      // TODO_RUST_CODE
      // Java: this.oriFillToRect = null;
      // TODO_RUST_CODE
      // Java: this.objPool.tryReuse(this.oriTileRect);
      // TODO_RUST_CODE
      // Java: this.oriTileRect = null;
      // TODO_RUST_CODE
      // Java: this.objPool.tryReuse(this.shapeRect);
      // TODO_RUST_CODE
      // Java: this.shapeRect = null;
      // TODO_RUST_CODE
      // Java: this.objPool.tryReuse(this.tempCenterF);
      // TODO_RUST_CODE
      // Java: this.tempCenterF = null;
      // TODO_RUST_CODE
      // Java: this.reUsePoints(this.points);
      // TODO_RUST_CODE
      // Java: this.points = null;
      // TODO_RUST_CODE
      // Java: if (this.adjustPath != this.path)
        // TODO_RUST_CODE
      {
         // Java: this.objPool.tryReuse(this.adjustPath);
         // TODO_RUST_CODE
      }
      // Java: this.adjustPath = null;
      // TODO_RUST_CODE
      // Java: this.path = null;
      // TODO_RUST_CODE
      // Java: this.canvas = null;
      // TODO_RUST_CODE
      // Java: this.fillPaint = null;
      // TODO_RUST_CODE
      // Java: this.colors = null;
      // TODO_RUST_CODE
      // Java: this.positions = null;
      // TODO_RUST_CODE
      // Java: this.objPool = null;
      // TODO_RUST_CODE
    }

    // Java: void reUsePoints(PointF[] points)
    pub fn reUsePoints(&self, PointF[] points) {
      // Java: if (points != null)
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < points.length; ++i)
            // TODO_RUST_CODE
         {
            // Java: this.objPool.tryReuse(points[i]);
            // TODO_RUST_CODE
         }
      }
    }

    // Java: void gradFill()
    pub fn gradFill(&self) {
      // Java: this.beginFill();
      // TODO_RUST_CODE
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: this.doFill();
         // TODO_RUST_CODE
      }
      // Java: finally
        // TODO_RUST_CODE
      {
         // Java: this.afterFill();
         // TODO_RUST_CODE
      }
    }

    // Java: abstract void doFill()
    pub fn doFill(&self) {
    // TODO_RUST_CODE
    }

    // Java: boolean isStrokeFill()
    pub fn isStrokeFill(&self) -> bool {
      // Java: return this.fillPaint.getStyle() == Style.STROKE;
      // TODO_RUST_CODE
    }

    // Java: void beginFill()
    pub fn beginFill(&self) {
      // Java: this.adjustParams(true);
      // TODO_RUST_CODE
      // Java: this.canvas.save();
      // TODO_RUST_CODE
      // Java: if (!this.isStrokeFill() && this.path != this.adjustPath)
        // TODO_RUST_CODE
      {
         // Java: this.canvas.clipPath(this.path);
         // TODO_RUST_CODE
      }
      // Java: if ((!this.rotWithShape || this.useBgFill) && this.rotation != 0.0F)
        // TODO_RUST_CODE
      {
         // Java: Matrix m = this.getRotationMatric(this.shapeRect.centerX(), this.shapeRect.centerY(), -this.rotation);
         // TODO_RUST_CODE
         // Java: this.canvas.concat(m);
         // TODO_RUST_CODE
         // Java: this.objPool.tryReuse(m);
         // TODO_RUST_CODE
      }
    }

    // Java: void afterFill()
    pub fn afterFill(&self) {
      // Java: this.canvas.restore();
      // TODO_RUST_CODE
    }

    // Java: void setShapeType(int shapeType)
    pub fn setShapeType(&self, int shapeType) {
      // Java: this.shapeType = shapeType;
      // TODO_RUST_CODE
    }

    // Java: void setPoints(PointF[] points)
    pub fn setPoints(&self, PointF[] points) {
      // Java: this.points = points;
      // TODO_RUST_CODE
    }

    // Java: PointF[] getPoints()
    pub fn getPoints(&self) -> PointF[] {
      // Java: return this.points;
      // TODO_RUST_CODE
    }

    // Java: void setRotParam(boolean rotWithShape, float rotation)
    pub fn setRotParam(&self, boolean rotWithShape, float rotation) {
      // Java: if (!this.useBgFill)
        // TODO_RUST_CODE
      {
         // Java: this.rotWithShape = rotWithShape;
         // TODO_RUST_CODE
      }
      // Java: this.rotation = rotation;
      // TODO_RUST_CODE
    }

    // Java: void adjustParams(boolean adustDstRectForRotation)
    pub fn adjustParams(&self, boolean adustDstRectForRotation) {
      // Java: if (this.oriFillToRect == null)
        // TODO_RUST_CODE
      {
         // Java: this.oriFillToRect = this.objPool.getRectF(0.0F, 0.0F, 1.0F, 1.0F);
         // TODO_RUST_CODE
      }
      // Java: if (this.oriTileRect == null)
        // TODO_RUST_CODE
      {
         // Java: this.oriTileRect = this.objPool.getRectF();
         // TODO_RUST_CODE
      }
      // Java: if (this.useBgFill)
        // TODO_RUST_CODE
      {
         // Java: this.dstRect = this.objPool.getRectF(this.oriDstRect.left, this.oriDstRect.top, this.oriDstRect.right, this.oriDstRect.bottom);
         // TODO_RUST_CODE
         // Java: if (this.rotation != 0.0F)
            // TODO_RUST_CODE
         {
            // Java: this.adjustPath = this.objPool.getPath();
            // TODO_RUST_CODE
            // Java: Matrix m = this.getRotationMatric(this.shapeRect.centerX(), this.shapeRect.centerY(), this.rotation);
            // TODO_RUST_CODE
            // Java: this.path.transform(m, this.adjustPath);
            // TODO_RUST_CODE
            // Java: this.objPool.tryReuse(m);
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: this.adjustPath = this.path;
            // TODO_RUST_CODE
         }
      }
      // Java: else if (adustDstRectForRotation && !this.rotWithShape && this.rotation != 0.0F)
        // TODO_RUST_CODE
      {
         // Java: this.dstRect = getRotationRect(this.oriDstRect, this.rotation);
         // TODO_RUST_CODE
         // Java: this.adjustPath = this.objPool.getPath();
         // TODO_RUST_CODE
         // Java: this.adjustPath.addRect(this.dstRect, Direction.CW);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.dstRect = this.objPool.getRectF(this.oriDstRect.left, this.oriDstRect.top, this.oriDstRect.right, this.oriDstRect.bottom);
         // TODO_RUST_CODE
         // Java: this.adjustPath = this.path;
         // TODO_RUST_CODE
      }
      // Java: this.fillToRect = this.transPercentageRect(this.oriFillToRect, this.dstRect);
      // TODO_RUST_CODE
      // Java: this.tileRect = this.transPercentageRect(this.oriTileRect, this.dstRect);
      // TODO_RUST_CODE
    }

    // Java: PointF getCenter()
    pub fn getCenter(&self) -> PointF {
      // Java: if (this.tempCenterF == null)
        // TODO_RUST_CODE
      {
         // Java: this.tempCenterF = new PointF(this.fillToRect.centerX(), this.fillToRect.centerY());
         // TODO_RUST_CODE
      }
      // Java: return this.tempCenterF;
      // TODO_RUST_CODE
    }

    // Java: boolean haveMoreTile()
    pub fn haveMoreTile(&self) -> bool {
      // Java: return this.tileRect.left > this.dstRect.left || this.tileRect.top > this.dstRect.top || this.tileRect.right < this.dstRect.right || this.tileRect.bottom < this.dstRect.bottom;
      // TODO_RUST_CODE
    }

    // Java: void gradFillForLinesPath(PointF gradCenter, boolean closePath)
    pub fn gradFillForLinesPath(&self, PointF gradCenter, boolean closePath) {
      // Java: if (this.points != null && this.points.length >= 2)
        // TODO_RUST_CODE
      {
         // Java: PointF starF = this.points[0];
         // TODO_RUST_CODE
         // Java: for(int i = 1; i < this.points.length; ++i)
            // TODO_RUST_CODE
         {
            // Java: PointF endF = this.points[i];
            // TODO_RUST_CODE
            // Java: this.gradFillForTriangle(gradCenter, starF, endF);
            // TODO_RUST_CODE
            // Java: starF = endF;
            // TODO_RUST_CODE
         }
         // Java: if (closePath)
            // TODO_RUST_CODE
         {
            // Java: this.gradFillForTriangle(gradCenter, starF, this.points[0]);
            // TODO_RUST_CODE
         }
      }
    }

    // Java: void tileGradFill(ITileFillAction action)
    pub fn tileGradFill(&self, ITileFillAction action) {
      // Java: if (action != null)
        // TODO_RUST_CODE
      {
         // Java: if (this.tileRect.left > this.dstRect.left)
            // TODO_RUST_CODE
         {
            // Java: this.rotateToTile(action, this.tileRect.left, this.tileRect.top, -1.0F, 1.0F);
            // TODO_RUST_CODE
         }
         // Java: if (this.tileRect.top > this.dstRect.top)
            // TODO_RUST_CODE
         {
            // Java: this.rotateToTile(action, this.tileRect.left, this.tileRect.top, 1.0F, -1.0F);
            // TODO_RUST_CODE
         }
         // Java: if (this.tileRect.left > this.dstRect.left && this.tileRect.top > this.dstRect.top)
            // TODO_RUST_CODE
         {
            // Java: this.rotateToTile(action, this.tileRect.left, this.tileRect.top, -1.0F, -1.0F);
            // TODO_RUST_CODE
         }
         // Java: if (this.tileRect.bottom < this.dstRect.bottom)
            // TODO_RUST_CODE
         {
            // Java: this.rotateToTile(action, this.tileRect.right, this.tileRect.bottom, 1.0F, -1.0F);
            // TODO_RUST_CODE
         }
         // Java: if (this.tileRect.right < this.dstRect.right)
            // TODO_RUST_CODE
         {
            // Java: this.rotateToTile(action, this.tileRect.right, this.tileRect.bottom, -1.0F, 1.0F);
            // TODO_RUST_CODE
         }
         // Java: if (this.tileRect.bottom < this.dstRect.bottom && this.tileRect.right < this.dstRect.right)
            // TODO_RUST_CODE
         {
            // Java: this.rotateToTile(action, this.tileRect.right, this.tileRect.bottom, -1.0F, -1.0F);
            // TODO_RUST_CODE
         }
         // Java: if (this.tileRect.top > this.dstRect.top && this.tileRect.right < this.dstRect.right)
            // TODO_RUST_CODE
         {
            // Java: this.rotateToTile(action, this.tileRect.right, this.tileRect.top, -1.0F, -1.0F);
            // TODO_RUST_CODE
         }
         // Java: if (this.tileRect.bottom < this.dstRect.bottom && this.tileRect.left > this.dstRect.left)
            // TODO_RUST_CODE
         {
            // Java: this.rotateToTile(action, this.tileRect.left, this.tileRect.bottom, -1.0F, -1.0F);
            // TODO_RUST_CODE
         }
      }
    }

    // Java: void rotateToTile(ITileFillAction action, float px, float py, float sx, float sy)
    pub fn rotateToTile(&self, ITileFillAction action, float px, float py, float sx, float sy) {
      // Java: if (action != null)
        // TODO_RUST_CODE
      {
         // Java: this.canvas.save();
         // TODO_RUST_CODE
         // Java: Matrix m = this.getScaleMatric(px, py, sx, sy);
         // TODO_RUST_CODE
         // Java: this.canvas.concat(m);
         // TODO_RUST_CODE
         // Java: this.objPool.tryReuse(m);
         // TODO_RUST_CODE
         // Java: action.tileAction();
         // TODO_RUST_CODE
         // Java: this.canvas.restore();
         // TODO_RUST_CODE
      }
    }

    // Java: Matrix getScaleMatric(float px, float py, float sx, float sy)
    pub fn getScaleMatric(&self, float px, float py, float sx, float sy) -> Matrix {
      // Java: Matrix matrix = this.objPool.getMatrix();
      // TODO_RUST_CODE
      // Java: matrix.preTranslate(px, py);
      // TODO_RUST_CODE
      // Java: matrix.preScale(sx, sy);
      // TODO_RUST_CODE
      // Java: matrix.preTranslate(-px, -py);
      // TODO_RUST_CODE
      // Java: return matrix;
      // TODO_RUST_CODE
    }

    // Java: Matrix getRotationMatric(float px, float py, float degrees)
    pub fn getRotationMatric(&self, float px, float py, float degrees) -> Matrix {
      // Java: Matrix matrix = this.objPool.getMatrix();
      // TODO_RUST_CODE
      // Java: matrix.preTranslate(px, py);
      // TODO_RUST_CODE
      // Java: matrix.preRotate(degrees);
      // TODO_RUST_CODE
      // Java: matrix.preTranslate(-px, -py);
      // TODO_RUST_CODE
      // Java: return matrix;
      // TODO_RUST_CODE
    }

    // Java: void gradFillForTriangle(PointF gradCenter, PointF lineStart, PointF lineEnd)
    pub fn gradFillForTriangle(&self, PointF gradCenter, PointF lineStart, PointF lineEnd) {
      // Java: PointF footPointF = getFootPoint(gradCenter, lineStart, lineEnd);
      // TODO_RUST_CODE
      // Java: if (footPointF != null && !isSamePoint(gradCenter.x, gradCenter.y, footPointF.x, footPointF.y))
        // TODO_RUST_CODE
      {
         // Java: Path trianglePath = this.objPool.getPath();
         // TODO_RUST_CODE
         // Java: trianglePath.moveTo(gradCenter.x, gradCenter.y);
         // TODO_RUST_CODE
         // Java: trianglePath.lineTo(lineStart.x, lineStart.y);
         // TODO_RUST_CODE
         // Java: trianglePath.lineTo(lineEnd.x, lineEnd.y);
         // TODO_RUST_CODE
         // Java: trianglePath.close();
         // TODO_RUST_CODE
         // Java: this.lineGradFill(trianglePath, gradCenter, footPointF, 0.0F, 0.0F);
         // TODO_RUST_CODE
         // Java: this.objPool.tryReuse(footPointF);
         // TODO_RUST_CODE
         // Java: this.objPool.tryReuse(trianglePath);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.objPool.tryReuse(footPointF);
         // TODO_RUST_CODE
      }
    }

    // Java: void lineGradFill(Path clippath, PointF start, PointF end, float dx, float dy)
    pub fn lineGradFill(&self, Path clippath, PointF start, PointF end, float dx, float dy) {
      // Java: this.canvas.save();
      // TODO_RUST_CODE
      // Java: LinearGradient lg = new LinearGradient(start.x, start.y, end.x, end.y, this.colors, this.positions, TileMode.MIRROR);
      // TODO_RUST_CODE
      // Java: if (dx != 0.0F || dy != 0.0F)
        // TODO_RUST_CODE
      {
         // Java: Matrix matrix = new Matrix();
         // TODO_RUST_CODE
         // Java: matrix.preTranslate(dx, dy);
         // TODO_RUST_CODE
         // Java: lg.setLocalMatrix(matrix);
         // TODO_RUST_CODE
      }
      // Java: this.fillPaint.setShader(lg);
      // TODO_RUST_CODE
      // Java: if (clippath != null)
        // TODO_RUST_CODE
      {
         // Java: this.canvas.clipPath(clippath);
         // TODO_RUST_CODE
      }
      // Java: this.canvas.drawPath(this.adjustPath, this.fillPaint);
      // TODO_RUST_CODE
      // Java: this.fillPaint.setShader((Shader)null);
      // TODO_RUST_CODE
      // Java: this.canvas.restore();
      // TODO_RUST_CODE
    }

    // Java: void fillForCircle(PointF centerF, float radius, boolean haveMoreTile)
    pub fn fillForCircle(&self, PointF centerF, float radius, boolean haveMoreTile) {
      // Java: if (radius != 0.0F)
        // TODO_RUST_CODE
      {
         // Java: RadialGradient shader = new RadialGradient(centerF.x, centerF.y, radius, this.colors, this.positions, TileMode.CLAMP);
         // TODO_RUST_CODE
         // Java: this.canvas.save();
         // TODO_RUST_CODE
         // Java: this.fillPaint.setShader(shader);
         // TODO_RUST_CODE
         // Java: if (haveMoreTile)
            // TODO_RUST_CODE
         {
            // Java: Path clipPath = new Path();
            // TODO_RUST_CODE
            // Java: clipPath.addCircle(this.tileRect.centerX(), this.tileRect.centerY(), this.tileRect.width() / 2.0F, Direction.CW);
            // TODO_RUST_CODE
            // Java: this.canvas.clipPath(clipPath);
            // TODO_RUST_CODE
         }
         // Java: this.canvas.drawPath(this.adjustPath, this.fillPaint);
         // TODO_RUST_CODE
         // Java: this.fillPaint.setShader((Shader)null);
         // TODO_RUST_CODE
         // Java: this.canvas.restore();
         // TODO_RUST_CODE
      }
    }

    // Java: RectF transPercentageRect(RectF percentRect, RectF dstRect)
    pub fn transPercentageRect(&self, RectF percentRect, RectF dstRect) -> RectF {
      // Java: float left = dstRect.left + dstRect.width() * percentRect.left;
      // TODO_RUST_CODE
      // Java: float top = dstRect.top + dstRect.height() * percentRect.top;
      // TODO_RUST_CODE
      // Java: float right = dstRect.right - dstRect.width() * percentRect.right;
      // TODO_RUST_CODE
      // Java: float bottom = dstRect.bottom - dstRect.height() * percentRect.bottom;
      // TODO_RUST_CODE
      // Java: return this.objPool.getRectF(left, top, right, bottom);
      // TODO_RUST_CODE
    }

    // Java: static PointF getFootPoint(PointF C, PointF A, PointF B)
    pub fn getFootPoint(&self, PointF C, PointF A, PointF B) -> PointF {
      // Java: PointF destPoint = null;
      // TODO_RUST_CODE
      // Java: if (A.x == B.x)
        // TODO_RUST_CODE
      {
         // Java: destPoint = AnyObjPool.getPool().getPoinF(A.x, C.y);
         // TODO_RUST_CODE
      }
      // Java: else if (A.y == B.y)
        // TODO_RUST_CODE
      {
         // Java: destPoint = AnyObjPool.getPool().getPoinF(C.x, A.y);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: float k = (B.y - A.y) / (B.x - A.x);
         // TODO_RUST_CODE
         // Java: destPoint = getFootPoint(k, A, C);
         // TODO_RUST_CODE
      }
      // Java: return destPoint;
      // TODO_RUST_CODE
    }

    // Java: static PointF getFootPoint(float k, PointF A, PointF C)
    pub fn getFootPoint(&self, float k, PointF A, PointF C) -> PointF {
      // Java: float k1 = (C.y - A.y) / (C.x - A.x);
      // TODO_RUST_CODE
      // Java: if (k1 == k)
        // TODO_RUST_CODE
      {
         // Java: return AnyObjPool.getPool().getPoinF(C.x, C.y);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: float x = (k * A.x + C.x / k + C.y - A.y) / (1.0F / k + k);
         // TODO_RUST_CODE
         // Java: float y = k * (x - A.x) + A.y;
         // TODO_RUST_CODE
         // Java: return AnyObjPool.getPool().getPoinF(x, y);
         // TODO_RUST_CODE
      }
    }

    // Java: static boolean isSamePoint(float x1, float y1, float x2, float y2)
    pub fn isSamePoint(&self, float x1, float y1, float x2, float y2) -> bool {
      // Java: float precision = 1.0E-4F;
      // TODO_RUST_CODE
      // Java: return Math.abs(x1 - x2) < precision && Math.abs(y1 - y2) < precision;
      // TODO_RUST_CODE
    }

    // Java: static RectF getRotationRect(RectF sourceRect, float rotation)
    pub fn getRotationRect(&self, RectF sourceRect, float rotation) -> RectF {
      // Java: float centerX = sourceRect.centerX();
      // TODO_RUST_CODE
      // Java: float centerY = sourceRect.centerY();
      // TODO_RUST_CODE
      // Java: float cosValue = Math.abs((float)Math.cos((double)rotation * Math.PI / (double)180.0F));
      // TODO_RUST_CODE
      // Java: float sinValue = Math.abs((float)Math.sin((double)rotation * Math.PI / (double)180.0F));
      // TODO_RUST_CODE
      // Java: float height = sourceRect.height() * cosValue + sourceRect.width() * sinValue;
      // TODO_RUST_CODE
      // Java: float width = sourceRect.width() * cosValue + sourceRect.height() * sinValue;
      // TODO_RUST_CODE
      // Java: return AnyObjPool.getPool().getRectF(centerX - width / 2.0F, centerY - height / 2.0F, centerX + width / 2.0F, centerY + height / 2.0F);
      // TODO_RUST_CODE
    }

}
