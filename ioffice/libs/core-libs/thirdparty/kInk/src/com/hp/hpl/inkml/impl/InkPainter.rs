use std::sync::{Arc, Mutex};
// Java: import android.graphics.Bitmap
// Java: import android.graphics.Canvas
// Java: import android.graphics.Paint
// Java: import android.graphics.Path
// Java: import android.graphics.PointF
// Java: import android.graphics.RectF
// Java: import android.graphics.Bitmap.CompressFormat
// Java: import android.graphics.Bitmap.Config
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.graphics.SizeF
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.moffice.util.KSLog
// Java: import com.hp.hpl.inkml.Ink
use crate::com::hp::hpl::inkml::Ink::Ink;
// Java: import com.hp.hpl.inkml.TraceDataList
// Java: import com.hp.hpl.inkml.constant.Converters
// Java: import com.hp.hpl.inkml.impl.painter.IPainter
// Java: import com.hp.hpl.inkml.impl.painter.PainterFactory
// Java: import com.hp.hpl.inkml.impl.painter.WritingBrush
// Java: import java.io.FileNotFoundException
// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.List

pub struct Canvas;
pub struct Paint;
pub struct Path;
pub struct Bitmap;
pub struct WritingBrush;

// Java field: String TAG
pub(crate) const TAG: &str = "InkPainter";

// Java: public class InkPainter
pub struct InkPainter {
    // Java field: ArrayList<WritingBrush> mWritingBrushList
    mWritingBrushList: Option<Vec<WritingBrush>>,
    // Java field: Ink mInk
    pub(crate) mInk: Arc<Mutex<Ink>>,
    // Java field: boolean mIsNightMode
    mIsNightMode: bool,
    // Java field: boolean mNeedColorFilter
    mNeedColorFilter: bool,
}

// Java: public class InkPainter
impl InkPainter {
    // Java: static String createPng(Ink ink, int width, int height)
    pub fn createPng_simple(_ink: &Ink, _width: i32, _height: i32) -> Option<String> {
      // Java: return createPng(ink, width, height, true);
      InkPainter::createPng(_ink, _width, _height, true)
    }

    // Java: static String createPng(Ink ink, int width, int height, boolean scale)
    pub fn createPng(_ink: &Ink, _width: i32, _height: i32, _scale: bool) -> Option<String> {
      // Java: Bitmap bitmap = createBitmap(ink, width, height, scale);
      // Java: File file = Platform.createTempFile("ink-capture-", ".png");
      // Java: FileOutputStream fos = null;
      // Java: String result;
      // Java: try
      {
         // Java: fos = new FileOutputStream(file);
         // Java: if (!bitmap.compress(CompressFormat.PNG, 30, fos))
         {
            // Java: return null;
         }
         // Java: result = file.getAbsolutePath();
      }
      // Java: catch (FileNotFoundException e)
      {
         // Java: KSLog.e(TAG, "IOException", e);
         // Java: return null;
      }
      // Java: finally
      {
         // Java: if (null != bitmap)
         {
            // Java: bitmap.recycle();
         }
      }
      // Java: return result;
      None
    }

    // Java: static Bitmap createBitmap(Ink ink, int width, int height)
    pub fn createBitmap_simple(_ink: &Ink, _width: i32, _height: i32) -> Option<Bitmap> {
      // Java: return createBitmap(ink, width, height, true);
      InkPainter::createBitmap(_ink, _width, _height, true)
    }

    // Java: static Bitmap createBitmap(Ink ink, int width, int height, boolean scale)
    pub fn createBitmap(_ink: &Ink, _width: i32, _height: i32, _scale: bool) -> Option<Bitmap> {
      // Java: try
      {
         // Java: return (new InkPainter(ink)).draw(width, height, scale);
      }
      // Java: catch (Exception var5)
      {
         // Java: return null;
      }
      None
    }

    // Java: InkPainter(Ink ink)
    pub fn InkPainter_simple(ink: Ink) -> Self {
      // Java: this(ink, false);
      InkPainter::InkPainter(ink, false)
    }

    // Java: InkPainter(Ink ink, boolean isNightMode)
    pub fn InkPainter(ink: Ink, is_night_mode: bool) -> Self {
      // Java: super();
      // Java: this.mNeedColorFilter = true;
      // Java: this.mInk = ink;
      // Java: this.mIsNightMode = isNightMode;
      // Java: ink.buildTraces();
      Self {
          mWritingBrushList: None,
          mInk: Arc::new(Mutex::new(ink)),
          mIsNightMode: is_night_mode,
          mNeedColorFilter: true,
      }
    }

    // Java: InkPainter(Ink ink, boolean isNightMode, boolean needColorFilter)
    pub fn InkPainter_full(ink: Ink, is_night_mode: bool, need_color_filter: bool) -> Self {
      // Java: this(ink, isNightMode);
      // Java: this.mNeedColorFilter = needColorFilter;
      let mut p = InkPainter::InkPainter(ink, is_night_mode);
      p.mNeedColorFilter = need_color_filter;
      p
    }

    // Java: RectF getRect()
    pub fn getRect(&self) {
      // Java: return this.mInk.getInkRect();
    }

    // Java: Bitmap draw(int width, int height)
    pub fn draw_simple(&self, _width: i32, _height: i32) -> Option<Bitmap> {
      // Java: return this.draw(width, height, true);
      None
    }

    // Java: Bitmap draw(int width, int height, boolean scale)
    pub fn draw(&self, _width: i32, _height: i32, _scale: bool) -> Option<Bitmap> {
      // Java: Bitmap bitmap = null;
      // Java: bitmap = Bitmap.createBitmap(width, height, Config.ARGB_8888);
      // Java: Canvas canvas = new Canvas(bitmap);
      // Java: this.draw(canvas, width, height, scale);
      // Java: return bitmap;
      None
    }

    // Java: void draw(Canvas canvas, int width, int height, boolean scale)
    pub fn draw_canvas(&self, _canvas: &mut Canvas, _width: i32, _height: i32, _scale: bool) {
      // Java: this.draw(canvas, width, height, scale, 0.0F);
    }

    // Java: void draw(Canvas canvas, int width, int height, boolean scale, float minStrokeWidth)
    pub fn draw_canvas_full(&self, _canvas: &mut Canvas, _width: i32, _height: i32, _scale: bool, _min_stroke_width: f32) {
      // Java: Paint paint = new Paint(1);
      // Java: Path path = new Path();
      // Java: float drawWidth = (float)(width * 2 / 3);
      // Java: float drawHeight = (float)(height * 2 / 3);
      // Java: if (!scale)
      {
         // Java: drawWidth = (float)width;
         // Java: drawHeight = (float)height;
      }
      // Java: float sx = 0.037795275F;
      // Java: float sy = 0.037795275F;
      // Java: RectF inkRect = this.getRect();
      // Java: if (inkRect.width() > 0.0F)
      {
         // Java: sx = drawWidth / inkRect.width();
         // Java: drawHeight = (float)((int)(inkRect.height() * sx));
      }
      // Java: else if (inkRect.height() > 0.0F)
      {
         // Java: sy = drawHeight / inkRect.height();
         // Java: drawWidth = (float)((int)(inkRect.width() * sy));
      }
      // Java: canvas.save();
      // Java: canvas.translate(((float)width - drawWidth) / 2.0F, ((float)height - drawHeight) / 2.0F);
      // Java: this.drawPound(canvas, paint, path, drawWidth, drawHeight, minStrokeWidth, false);
      // Java: canvas.restore();
    }

    // Java: void drawPound(Canvas canvas, Paint paint, Path path, float width, float height)
    pub fn drawPound_2(&self) {
      // Java: this.drawPound(canvas, paint, path, width, height, 1.0F);
    }

    // Java: void drawPound(Canvas canvas, Paint paint, Path path, float width, float height, float minStrokeWidth)
    pub fn drawPound_3(&self) {
      // Java: this.drawPound(canvas, paint, path, width, height, minStrokeWidth, false);
    }

    // Java: void drawPound(Canvas canvas, Paint paint, Path path, float width, float height, float minStrokeWidth, boolean forceMinStroke)
    pub fn drawPound_4(&self) {
      // Java: this.drawPound(canvas, paint, path, width, height, minStrokeWidth, forceMinStroke, false);
    }

    // Java: void drawPound(Canvas canvas, Paint paint, Path path, float width, float height, float minStrokeWidth, boolean forceMinStroke, boolean scaleBrush)
    pub fn drawPound_5(&self) {
      // Java: this.drawPound(canvas, paint, path, width, height, minStrokeWidth, forceMinStroke, false, false);
    }

    // Java: void drawPound(Canvas canvas, Paint paint, Path path, float width, float height, float minStrokeWidth, boolean forceMinStroke, boolean scaleBrush, boolean scaleInk)
    pub fn drawPound_6(&self) {
      // Java: this.drawPound(canvas, paint, path, (List)null, width, height, minStrokeWidth, forceMinStroke, scaleBrush, scaleInk, true);
    }

    // Java: void drawPound(Canvas canvas, Paint paint, Path path, float width, float height, float minStrokeWidth, boolean forceMinStroke, boolean scaleBrush, boolean scaleInk, boolean resetPath)
    pub fn drawPound_7(&self) {
      // Java: this.drawPound(canvas, paint, path, (List)null, width, height, minStrokeWidth, forceMinStroke, scaleBrush, scaleInk, resetPath);
    }

    // Java: void drawPound(Canvas canvas, Paint paint, Path path, List<PointF> points, float width, float height, float minStrokeWidth, boolean forceMinStroke, boolean scaleBrush, boolean scaleInk, boolean resetPath)
    pub fn drawPound(&self) {
      // Java: RectF inkRect = this.getRect();
      // Java: SizeF scaleXY = Ink.calScaleXY(inkRect, width, height);
      // Java: float scaleX = scaleXY.width;
      // Java: float scaleY = scaleXY.height;
      // Java: canvas.save();
      // Java: canvas.translate(-inkRect.left * scaleX, -inkRect.top * scaleY);
      // Java: if (scaleInk)
      {
         // Java: canvas.scale(scaleX, scaleY);
      }
      // Java: this.draw(canvas, paint, path, points, false, minStrokeWidth, scaleX, scaleY, forceMinStroke, scaleBrush, scaleInk, resetPath);
      // Java: canvas.restore();
    }

    // Java: void drawPoundWithPathCache(Canvas canvas, float width, float height, float minStrokeWidth, boolean forceMinStroke)
    pub fn drawPoundWithPathCache(&self) {
      // Java: InkPathCache pathCache = this.mInk.getPathCache();
      // Java: if (pathCache != null && pathCache.canUsePath(width, height))
      {
         // Java: pathCache.drawPath(canvas);
      }
      // Java: else
      {
         // Java: Paint paint = new Paint();
         // Java: Path path = new Path();
         // Java: List<PointF> points = new ArrayList();
         // Java: boolean bSetPath = InkPathCache.canSetPath(this.mInk);
         // Java: this.drawPound(canvas, paint, path, points, width, height, minStrokeWidth, forceMinStroke, false, false, !bSetPath);
         // Java: if (bSetPath)
         {
            // Java: pathCache = new InkPathCache();
            // Java: if (pathCache.setPath(this.mInk, path, points, paint, width, height))
            {
               // Java: this.mInk.setPathCache(pathCache);
            }
         }
      }
    }

    // Java: void draw(Canvas canvas, Paint paint, Path path, float width, float height)
    pub fn draw_with_path(&self) {
      // Java: RectF inkRect = this.getRect();
      // Java: canvas.save();
      // Java: SizeF scaleXY = Ink.calScaleXY(inkRect, width, height);
      // Java: float scaleX = scaleXY.width;
      // Java: float scaleY = scaleXY.height;
      // Java: canvas.translate(-inkRect.left * scaleX, -inkRect.top * scaleY);
      // Java: float mMinStrokeWidth = 1.0F / scaleX;
      // Java: this.draw(canvas, paint, path, (List)null, false, mMinStrokeWidth, scaleX, scaleY, false, false, false, true);
      // Java: canvas.restore();
    }

    // Java: void scaleTraceBursh(float scale)
    pub fn scaleTraceBursh(&self, _scale: f32) {
      // Java: ArrayList<TraceAdapter> traces = this.mInk.getTraces();
      // Java: int traceSize = traces.size();
      // Java: for(int traceIndex = 0; traceIndex < traceSize; ++traceIndex)
      {
         // Java: TraceAdapter trace = (TraceAdapter)traces.get(traceIndex);
         // Java: Brush brush = trace.getBrush();
         // Java: if (brush != null)
         {
            // Java: float newWidth = brush.getWidth() * scale;
            // Java: brush.setWidth(newWidth);
         }
      }
    }

    // Java: float getBurshWidth()
    pub fn getBurshWidth(&self) -> f32 {
      // Java: ArrayList<TraceAdapter> traces = this.mInk.getTraces();
      // Java: int traceSize = traces.size();
      // Java: for(int traceIndex = 0; traceIndex < traceSize; ++traceIndex)
      {
         // Java: TraceAdapter trace = (TraceAdapter)traces.get(traceIndex);
         // Java: Brush brush = trace.getBrush();
         // Java: if (brush != null)
         {
            // Java: return brush.getWidth();
         }
      }
      // Java: return 0.0F;
      0.0
    }

    // Java: float getBurshOriWidth()
    pub fn getBurshOriWidth(&self) -> f32 {
      // Java: ArrayList<TraceAdapter> traces = this.mInk.getTraces();
      // Java: int traceSize = traces.size();
      // Java: for(int traceIndex = 0; traceIndex < traceSize; ++traceIndex)
      {
         // Java: TraceAdapter trace = (TraceAdapter)traces.get(traceIndex);
         // Java: Brush brush = trace.getBrush();
         // Java: if (brush != null)
         {
            // Java: return brush.getWidth() / trace.getScaleX();
         }
      }
      // Java: return 0.0F;
      0.0
    }

    // Java: void draw(Canvas canvas, Paint paint, Path path, List<PointF> points, boolean ingoreMaskPen, float minStrokeWidth, float scaleX, float scaleY, boolean forceMinStroke, boolean scaleBrush, boolean scaleInk, boolean resetPath)
    pub fn draw_full(&self) {
      // Java: ArrayList<TraceAdapter> traces = this.mInk.getTraces();
      // Java: int traceSize = traces.size();
      // Java: for(int traceIndex = 0; traceIndex < traceSize; ++traceIndex)
      {
         // Java: TraceAdapter trace = (TraceAdapter)traces.get(traceIndex);
         // Java: float traceScaleX = trace.getScaleX();
         // Java: float traceScaleY = trace.getScaleY();
         // Java: TraceDataList list = trace.getTraceData();
         // Java: Brush brush = trace.getBrush();
         // Java: IPainter painter = null;
         // Java: boolean useCachedPainter = null != this.mWritingBrushList && traceIndex < this.mWritingBrushList.size();
         // Java: if (useCachedPainter)
         {
            // Java: painter = (IPainter)this.mWritingBrushList.get(traceIndex);
         }
         // Java: else
         {
            // Java: painter = PainterFactory.create(brush);
         }
         // Java: float scale = Math.max(traceScaleX, traceScaleY);
         // Java: float oldWidth = brush.getWidth();
         // Java: float oldHeight = brush.getHeight();
         // Java: if (!scaleBrush)
         {
            // Java: if (brush.getShape() != Brush.Shape.rectangle)
            {
               // Java: float xScale = 26.458334F / traceScaleX;
               // Java: float yScale = 26.458334F / traceScaleY;
               // Java: if (scaleInk)
               {
                  // Java: xScale *= scaleX;
                  // Java: yScale *= scaleY;
               }
               // Java: float width = oldWidth / Math.max(xScale, yScale);
               // Java: float minWidth = 0.25F * traceScaleX;
               // Java: if (width != 0.0F && minStrokeWidth == 0.4F && width < minWidth)
               {
                  // Java: width = minWidth;
               }
               // Java: brush.setWidth(width);
            }
            // Java: else
            {
               // Java: float xScale = 26.458334F / traceScaleX;
               // Java: float yScale = 26.458334F / traceScaleY;
               // Java: if (scaleInk)
               {
                  // Java: xScale *= scaleX;
                  // Java: yScale *= scaleY;
               }
               // Java: float width = oldWidth / xScale;
               // Java: float heigth = oldHeight / yScale;
               // Java: brush.setWidth(width);
               // Java: brush.setHeight(heigth);
            }
         }
         // Java: Converters.convertPaint(paint, brush, minStrokeWidth * scale, ingoreMaskPen, forceMinStroke);
         // Java: int color = paint.getColor();
         // Java: if (this.mIsNightMode && (this.mNeedColorFilter || color == -1 || color == -16777216))
         {
            // Java: paint.setColor(ColorTool.changeColorAtNightMode(color));
         }
         // Java: if (scaleInk)
         {
            // Java: drawDataList(canvas, paint, path, points, painter, list, brush, traceScaleX, traceScaleY, 1.0F, 1.0F, true, useCachedPainter, resetPath);
         }
         // Java: else
         {
            // Java: drawDataList(canvas, paint, path, points, painter, list, brush, traceScaleX, traceScaleY, scaleX, scaleY, true, useCachedPainter, resetPath);
         }
         // Java: brush.setWidth(oldWidth);
         // Java: brush.setHeight(oldHeight);
      }
    }

    // Java: static void drawDataList(Canvas canvas, Paint paint, Path path, IPainter painter, TraceDataList list, Brush brush, float traceScaleX, float traceScaleY, float scaleX, float scaleY)
    pub fn drawDataList_simple() {
      // Java: drawDataList(canvas, paint, path, painter, list, brush, traceScaleX, traceScaleY, scaleX, scaleY, true);
    }

    // Java: static void drawDataList(Canvas canvas, Paint paint, Path path, IPainter painter, TraceDataList list, Brush brush, float traceScaleX, float traceScaleY, float scaleX, float scaleY, boolean completed)
    pub fn drawDataList_completed() {
      // Java: drawDataList(canvas, paint, path, painter, list, brush, traceScaleX, traceScaleY, scaleX, scaleY, completed, false);
    }

    // Java: static void drawDataList(Canvas canvas, Paint paint, Path path, IPainter painter, TraceDataList list, Brush brush, float traceScaleX, float traceScaleY, float scaleX, float scaleY, boolean completed, boolean useCachedPainter)
    pub fn drawDataList_cached() {
      // Java: drawDataList(canvas, paint, path, (List)null, painter, list, brush, traceScaleX, traceScaleY, scaleX, scaleY, completed, useCachedPainter, true);
    }

    // Java: static void drawDataList(Canvas canvas, Paint paint, Path path, List<PointF> points, IPainter painter, TraceDataList list, Brush brush, float traceScaleX, float traceScaleY, float scaleX, float scaleY, boolean completed, boolean useCachedPainter, boolean resetPath)
    pub fn drawDataList() {
      // Java: canvas.save();
      // Java: canvas.scale(1.0F / traceScaleX, 1.0F / traceScaleY);
      // Java: int size = list.size();
      // Java: if (size > 1)
      {
         // Java: if (useCachedPainter)
         {
            // Java: drawPathByCachedPainter(canvas, paint, path, (WritingBrush)painter, list, brush, scaleX, scaleY, completed, resetPath);
         }
         // Java: else
         {
            // Java: drawPath(canvas, paint, path, painter, list, brush, scaleX, scaleY, completed, resetPath);
         }
      }
      // Java: else if (size == 1)
      {
         // Java: drawPoint(canvas, points, paint, list, 0, brush, scaleX, scaleY);
      }
      // Java: canvas.restore();
    }

    // Java: static void drawPath(Canvas canvas, Paint paint, Path path, IPainter painter, TraceDataList list, Brush brush, float scaleX, float scaleY, boolean completed, boolean resetPath)
    pub fn drawPath() {
      // Java: if (resetPath)
      {
         // Java: path.reset();
      }
      // Java: int xIndex = list.getXIndex();
      // Java: int yIndex = list.getYIndex();
      // Java: int fIndex = list.getFIndex();
      // Java: float maxFChannel = brush.getMaxFChannel();
      // Java: boolean hasFChannel = maxFChannel > 0.0F && fIndex >= 0;
      // Java: float f = 1.0F;
      // Java: Iterator<Object[]> iterator = list.iterator();
      // Java: Object[] traceFragment = iterator.next();
      // Java: float x = (Float)traceFragment[xIndex] * scaleX;
      // Java: float y = (Float)traceFragment[yIndex] * scaleY;
      // Java: if (hasFChannel)
      {
         // Java: f = Math.abs((Float)traceFragment[fIndex] / maxFChannel);
      }
      // Java: painter.begin(path, brush, x, y, Math.min(1.0F, f));
      // Java: for(; iterator.hasNext(); painter.move(x, y, Math.min(1.0F, f)))
      {
         // Java: traceFragment = iterator.next();
         // Java: x = (Float)traceFragment[xIndex] * scaleX;
         // Java: y = (Float)traceFragment[yIndex] * scaleY;
         // Java: if (hasFChannel)
         {
            // Java: f = Math.abs((Float)traceFragment[fIndex] / maxFChannel);
         }
      }
      // Java: painter.end(completed);
      // Java: painter.draw(canvas, paint);
    }

    // Java: static void drawPathByCachedPainter(Canvas canvas, Paint paint, Path path, WritingBrush writingBrush, TraceDataList list, Brush brush, float scaleX, float scaleY, boolean completed, boolean resetPath)
    pub fn drawPathByCachedPainter() {
      // Java: if (resetPath)
      {
         // Java: path.reset();
      }
      // Java: int xIndex = list.getXIndex();
      // Java: int yIndex = list.getYIndex();
      // Java: int fIndex = list.getFIndex();
      // Java: float maxFChannel = brush.getMaxFChannel();
      // Java: boolean hasFChannel = maxFChannel > 0.0F && fIndex >= 0;
      // Java: float f = 1.0F;
      // Java: int size = list.size();
      // Java: Object[] traceFragment = list.get(0);
      // Java: float x = (Float)traceFragment[xIndex] * scaleX;
      // Java: float y = (Float)traceFragment[yIndex] * scaleY;
      // Java: if (hasFChannel)
      {
         // Java: f = (Float)traceFragment[fIndex] / maxFChannel;
      }
      // Java: int cacheSize = writingBrush.getCacheSize();
      // Java: if (0 == cacheSize)
      {
         // Java: writingBrush.begin(path, brush, x, y, f);
      }
      // Java: else
      {
         // Java: writingBrush.drawCache();
      }
      // Java: int escapedTraces = writingBrush.getEscapedTraces();
      // Java: for(int i = cacheSize + escapedTraces + 1; i < size; ++i)
      {
         // Java: traceFragment = list.get(i);
         // Java: x = (Float)traceFragment[xIndex] * scaleX;
         // Java: y = (Float)traceFragment[yIndex] * scaleY;
         // Java: if (hasFChannel)
         {
            // Java: f = (Float)traceFragment[fIndex] / maxFChannel;
         }
         // Java: writingBrush.move(x, y, f);
      }
      // Java: writingBrush.end(completed);
      // Java: writingBrush.draw(canvas, paint);
    }

    // Java: static void drawPoint(Canvas canvas, List<PointF> points, Paint paint, TraceDataList list, int index, Brush brush, float scaleX, float scaleY)
    pub fn drawPoint() {
      // Java: Object[] traceFragment = list.get(index);
      // Java: int xIndex = list.getXIndex();
      // Java: int yIndex = list.getYIndex();
      // Java: float x = (Float)traceFragment[xIndex] * scaleX;
      // Java: float y = (Float)traceFragment[yIndex] * scaleY;
      // Java: paint.setStrokeWidth(brush.getWidth() / 4.0F);
      // Java: if (points != null)
      {
         // Java: points.add(new PointF(x, y));
      }
      // Java: canvas.drawPoint(x, y, paint);
    }

    // Java: void setWritingBrushList(ArrayList<WritingBrush> writingBrushList)
    pub fn setWritingBrushList(&mut self, writing_brush_list: Vec<WritingBrush>) {
      // Java: this.mWritingBrushList = writingBrushList;
      self.mWritingBrushList = Some(writing_brush_list);
    }

    // Java: boolean isNitePen()
    pub fn isNitePen(&self) -> bool {
      // Java: ArrayList<TraceAdapter> traces = this.mInk.getTraces();
      // Java: if (traces.size() > 0)
      {
         // Java: TraceAdapter trace = (TraceAdapter)traces.get(0);
         // Java: Brush brush = trace.getBrush();
         // Java: return brush.getRasterOp() == Brush.RasterOp.maskPen;
      }
      // Java: else
      {
         // Java: return false;
      }
      false
    }

    // Java: void setColor(int color)
    pub fn setColor(&self, _color: i32) {
      // Java: if (null != this.mInk)
      {
         // Java: for(TraceAdapter adapter : this.mInk.getTraces())
         {
            // Java: adapter.getBrush().setColor(color);
         }
      }
    }

    // Java: void setSize(float size)
    pub fn setSize(&self, _size: f32) {
      // Java: if (null != this.mWritingBrushList)
      {
         // Java: for(WritingBrush brush : this.mWritingBrushList)
         {
            // Java: brush.clear();
         }
      }
      // Java: if (null != this.mInk)
      {
         // Java: for(TraceAdapter adapter : this.mInk.getTraces())
         {
            // Java: adapter.getBrush().setWidth(size * 1000.0F);
            // Java: adapter.getBrush().setHeight(size * 1000.0F);
         }
      }
    }

}
