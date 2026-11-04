#[allow(unused_imports)]
use std::sync::{Arc, Mutex};
// Java: import android.graphics.Canvas
// Java: import android.graphics.Color
// Java: import android.graphics.Paint
// Java: import android.graphics.Path
// Java: import android.graphics.RectF
// Java: import com.hp.hpl.inkml.Channel
// Java: import com.hp.hpl.inkml.ChannelProperties
// Java: import com.hp.hpl.inkml.ChannelProperty
// Java: import com.hp.hpl.inkml.Context
// Java: import com.hp.hpl.inkml.Definitions
// Java: import com.hp.hpl.inkml.Ink
use crate::com::hp::hpl::inkml::Ink::Ink;
// Java: import com.hp.hpl.inkml.InkElement
// Java: import com.hp.hpl.inkml.InkMLException
// Java: import com.hp.hpl.inkml.InkSource
// Java: import com.hp.hpl.inkml.Trace
// Java: import com.hp.hpl.inkml.TraceDataList
use crate::com::hp::hpl::inkml::TraceDataList::TraceDataList;
// Java: import com.hp.hpl.inkml.TraceFormat
// Java: import com.hp.hpl.inkml.constant.Converters
// Java: import com.hp.hpl.inkml.impl.painter.IPainter
use crate::com::hp::hpl::inkml::r#impl::painter::PainterFactory::IPainter;
// Java: import com.hp.hpl.inkml.impl.painter.PainterFactory
// Java: import com.hp.hpl.inkml.impl.painter.WritingBrush
// Java: import java.util.ArrayList

use crate::com::hp::hpl::inkml::r#impl::Brush::Brush;
use crate::com::hp::hpl::inkml::r#impl::InkPainter::InkPainter;
use util::graphics::RectF::RectF;

pub struct WritingBrush;
pub struct ChannelType;

// Java: public class InkCreator
pub struct InkCreator {
    // Java field: String DEFAULT_CONTEXT_REF
    DEFAULT_CONTEXT_REF: String,
    // Java field: String DEFAULT_ROURCE_ID
    DEFAULT_ROURCE_ID: String,
    // Java field: Channel.ChannelType DEFAULT_CHANNELTYPE
    DEFAULT_CHANNELTYPE: i32,
    // Java field: int DEFAULT_X_INDEX
    DEFAULT_X_INDEX: i32,
    // Java field: int DEFAULT_Y_INDEX
    DEFAULT_Y_INDEX: i32,
    // Java field: int DEFAULT_F_INDEX
    DEFAULT_F_INDEX: i32,
    // Java field: int DEFAULT_CHANNEL_COUNT
    DEFAULT_CHANNEL_COUNT: i32,
    // Java field: String DEFAULT_BRUSH_REF_PRE
    DEFAULT_BRUSH_REF_PRE: String,
    // Java field: boolean mFitToCurve
    mFitToCurve: bool,
    // Java field: TraceDataList mDataList
    pub(crate) mDataList: Option<TraceDataList>,
    // Java field: ArrayList<WritingBrush> mWritingBrushList
    mWritingBrushList: Option<Vec<WritingBrush>>,
    // Java field: ArrayList<com.hp.hpl.inkml.Brush> mBrushList
    mBrushList: Vec<i32>,
    // Java field: int mCurrentBrushIndex
    mCurrentBrushIndex: i32,
    // Java field: Ink mInk
    pub(crate) mInk: Option<Ink>,
    // Java field: RectF mSelectRect
    pub(crate) mSelectRect: RectF,
    // Java field: boolean mIsBegin
    pub(crate) mIsBegin: bool,
    // Java field: IPainter mPainter
    pub(crate) mPainter: Option<Box<dyn IPainter>>,
    // Java field: Brush mBrush
    pub(crate) mBrush: Option<Brush>,
    // Java field: InkPainter mInkPainter
    pub(crate) mInkPainter: Option<InkPainter>,
    // Java field: boolean isNightMode
    isNightMode: bool,
    // Java field: boolean mNeedColorFilter
    mNeedColorFilter: bool,
}

// Java: public class InkCreator
impl InkCreator {
    // Java: InkCreator()
    pub fn InkCreator() -> Self {
      // Java: super();
      // Java: this.DEFAULT_CHANNELTYPE = Channel.ChannelType.DECIMAL;
      // Java: this.DEFAULT_X_INDEX = 0;
      // Java: this.DEFAULT_Y_INDEX = 1;
      // Java: this.DEFAULT_F_INDEX = 2;
      // Java: this.DEFAULT_CHANNEL_COUNT = 2;
      // Java: this.DEFAULT_BRUSH_REF_PRE = "br";
      // Java: this.mFitToCurve = true;
      // Java: this.mBrushList = new ArrayList();
      // Java: this.mCurrentBrushIndex = 0;
      // Java: this.mSelectRect = new RectF();
      // Java: this.mIsBegin = true;
      // Java: this.mPainter = null;
      // Java: this.mBrush = null;
      Self {
          DEFAULT_CONTEXT_REF: String::new(),
          DEFAULT_ROURCE_ID: String::new(),
          DEFAULT_CHANNELTYPE: 0,
          DEFAULT_X_INDEX: 0,
          DEFAULT_Y_INDEX: 1,
          DEFAULT_F_INDEX: 2,
          DEFAULT_CHANNEL_COUNT: 2,
          DEFAULT_BRUSH_REF_PRE: "br".to_string(),
          mFitToCurve: true,
          mDataList: None,
          mWritingBrushList: None,
          mBrushList: Vec::new(),
          mCurrentBrushIndex: 0,
          mInk: None,
          mSelectRect: RectF::new(),
          mIsBegin: true,
          mPainter: None,
          mBrush: None,
          mInkPainter: None,
          isNightMode: false,
          mNeedColorFilter: true,
      }
    }

    // Java: void setRect(RectF selectRect)
    pub fn setRect(&mut self, _select_rect: &RectF) {
      // Java: this.mSelectRect.set(selectRect);
      // Java: this.getInk().onEdit();
    }

    // Java: void setSelectRect(RectF selectRect)
    pub fn setSelectRect_rect(&mut self, _select_rect: &RectF) {
      // Java: this.mSelectRect.set(selectRect);
    }

    // Java: void begin(int color, float strokeWidth, boolean maskPen, Brush.Shape shape)
    pub fn begin_simple(&mut self, color: i32, stroke_width: f32, mask_pen: bool, shape: i32) {
      // Java: this.begin(color, strokeWidth, maskPen, shape, false);
      self.begin(color, stroke_width, mask_pen, shape, false);
    }

    // Java: void begin(int color, float strokeWidth, boolean maskPen, Brush.Shape shape, boolean hasFChannel)
    pub fn begin(&mut self, _color: i32, _stroke_width: f32, _mask_pen: bool, _shape: i32, _has_f_channel: bool) {
      // Java: if (this.mInk == null)
      if self.mInk.is_none()
      {
         // Java: this.mInk = new Ink();
         // Java: Context context = this.createContext(hasFChannel);
         // Java: this.mInk.getDefinitions().addToDirectChildrenMap((InkElement)context);
         // Java: this.mIsBegin = true;
         self.mIsBegin = true;
      }
      // Java: int fIndex = 2;
      // Java: if (!hasFChannel)
      {
         // Java: fIndex = -1;
      }
      // Java: com.hp.hpl.inkml.Brush brush = this.prepareBrush(color, strokeWidth, maskPen, shape);
      // Java: if (this.mDataList != null)
      {
         // Java: this.addTraceData();
      }
      // Java: this.mDataList = new TraceDataList(this.DEFAULT_CHANNELTYPE, 0, 1, fIndex);
      // Java: if (null != brush)
      {
         // Java: this.mBrush = Brush.fromInkML(brush);
         // Java: this.mBrush.setWidth(this.mBrush.getWidth() * 1000.0F);
         // Java: this.mBrush.setHeight(this.mBrush.getHeight() * 1000.0F);
         // Java: this.mBrush.setMaxFChannel(hasFChannel);
         // Java: this.mPainter = PainterFactory.create(this.mBrush);
      }
      // Java: else if (PainterFactory.isWritingBrush(this.mBrush))
      {
         // Java: this.mPainter = PainterFactory.create(this.mBrush);
      }
      // Java: if (hasFChannel != this.mBrush.hasMaxFChannel())
      {
         // Java: this.mBrush.setMaxFChannel(hasFChannel);
      }
      // Java: if (PainterFactory.isWritingBrush(this.mBrush))
      {
         // Java: if (null == this.mWritingBrushList)
         {
            // Java: this.mWritingBrushList = new ArrayList();
         }
         // Java: ((WritingBrush)this.mPainter).setCreating(true);
         // Java: this.mWritingBrushList.add((WritingBrush)this.mPainter);
      }
      // Java: InkEstimate inkEstimate = InkEstimate.getInstance();
      // Java: if (inkEstimate != null)
      {
         // Java: inkEstimate.clear();
      }
    }

    // Java: Ink getInk()
    pub fn getInk(&self) -> Option<&Ink> {
      // Java: return this.mInk;
      self.mInk.as_ref()
    }

    // Java: InkPainter getInkPainter()
    pub fn getInkPainter(&self) -> Option<&InkPainter> {
      // Java: if (null == this.mInkPainter)
      {
         // Java: this.mInkPainter = new InkPainter(this.mInk);
      }
      // Java: return this.mInkPainter;
      self.mInkPainter.as_ref()
    }

    // Java: boolean hasData()
    pub fn hasData(&self) -> bool {
      // Java: if (this.mInk == null)
      if self.mInk.is_none()
      {
         // Java: return false;
         return false;
      }
      // Java: else if (this.mInk.getTraceDataList() != null && this.mInk.getTraceDataList().size() > 0)
      // Java: return true;
      // Java: else
      {
         // Java: return this.mDataList != null && this.mDataList.size() > 0;
         return self.mDataList.is_some();
      }
    }

    // Java: float getScale()
    pub fn getScale(&self) -> f32 {
      // Java: return 26.458334F;
      26.458334
    }

    // Java: void move(float x, float y)
    pub fn move_xy(&mut self, x: f32, y: f32) {
      // Java: this.move(x, y, 0.0F);
      self.move_xyf(x, y, 0.0);
    }

    // Java: void move(float x, float y, float f)
    pub fn move_xyf(&mut self, _x: f32, _y: f32, _f: f32) {
      // Java: if (this.mDataList != null)
      {
         // Java: int channelCount = 2;
         // Java: int fIndex = this.mDataList.getFIndex();
         // Java: if (-1 != fIndex)
         {
            // Java: ++channelCount;
         }
         // Java: Object[] traceFragment = new Object[channelCount];
         // Java: traceFragment[0] = x * this.getScale();
         // Java: traceFragment[1] = y * this.getScale();
         // Java: if (-1 != fIndex)
         {
            // Java: traceFragment[2] = f <= 0.0F ? 0.0F : Math.abs(f * this.mBrush.getMaxFChannel());
         }
         // Java: this.mDataList.add(traceFragment);
         // Java: this.setSelectRect(x, y);
      }
    }

    // Java: void move(float px, float py, float ix, float iy)
    pub fn move_pxpy(&mut self, px: f32, py: f32, ix: f32, iy: f32) {
      // Java: this.move(px, py, ix, iy, 0.0F);
      self.move_pxpyf(px, py, ix, iy, 0.0);
    }

    // Java: void move(float px, float py, float ix, float iy, float f)
    pub fn move_pxpyf(&mut self, _px: f32, _py: f32, _ix: f32, _iy: f32, _f: f32) {
      // Java: if (this.mDataList != null)
      {
         // Java: int channelCount = 2;
         // Java: int fIndex = this.mDataList.getFIndex();
         // Java: if (-1 != fIndex)
         {
            // Java: ++channelCount;
         }
         // Java: Object[] traceFragment = new Object[channelCount];
         // Java: traceFragment[0] = ix * this.getScale();
         // Java: traceFragment[1] = iy * this.getScale();
         // Java: if (-1 != fIndex)
         {
            // Java: traceFragment[2] = Math.abs(f);
         }
         // Java: this.mDataList.add(traceFragment);
         // Java: this.setSelectRect(px, py);
      }
    }

    // Java: void end()
    pub fn end(&mut self) {
      // Java: this.addTraceData();
      self.addTraceData();
    }

    // Java: RectF getRect()
    pub fn getRect(&self) -> &RectF {
      // Java: return this.mSelectRect;
      &self.mSelectRect
    }

    // Java: void getRect(RectF rect)
    pub fn getRect_into(&self, _rect: &mut RectF) {
      // Java: rect.set(this.mSelectRect.left, this.mSelectRect.top, this.mSelectRect.right, this.mSelectRect.bottom);
    }

    // Java: void setFitToCurve(boolean value)
    pub fn setFitToCurve(&mut self, value: bool) {
      // Java: this.mFitToCurve = value;
      self.mFitToCurve = value;
    }

    // Java: void addTraceData()
    pub fn addTraceData(&mut self) {
      // Java: if (this.mDataList != null && this.mDataList.size() >= 1)
      {
         // Java: Trace trace = new Trace();
         // Java: trace.setAttribute("brushRef", "#" + this.getBrushRef());
         // Java: trace.setAttribute("contextRef", "#ctx0");
         // Java: Context currCtx = this.mInk.getCurrentContext();
         // Java: Definitions defs = this.mInk.getDefinitions();
         // Java: try
         {
            // Java: trace.resolveAssociatedContext(currCtx, defs);
         }
         // Java: catch (InkMLException var5)
         {
         }
         // Java: trace.setTraceData(this.mDataList);
         // Java: trace.mScaleX = this.getScale();
         // Java: trace.mScaleY = this.getScale();
         // Java: this.mInk.addTrace(trace);
         // Java: this.mDataList = null;
         self.mDataList = None;
         // Java: InkEstimate inkEstimate = InkEstimate.getInstance();
         // Java: if (inkEstimate != null)
         {
            // Java: inkEstimate.clear();
         }
      }
    }

    // Java: Context createContext(boolean createFChannel)
    pub fn createContext(&self, _create_f_channel: bool) {
      // Java: TraceFormat traceFormat = new TraceFormat();
      // Java: Channel channelX = new Channel("X", Channel.ChannelType.DECIMAL);
      // Java: channelX.setUnits("cm");
      // Java: traceFormat.addChannel(channelX);
      // Java: Channel channelY = new Channel("Y", Channel.ChannelType.DECIMAL);
      // Java: channelY.setUnits("cm");
      // Java: traceFormat.addChannel(channelY);
      // Java: if (createFChannel)
      {
         // Java: Channel channelF = new Channel("F", Channel.ChannelType.DECIMAL);
         // Java: channelF.setUnits("dev");
         // Java: channelF.setMax(Integer.toString(1023));
         // Java: traceFormat.addChannel(channelF);
      }
      // Java: ChannelProperties cps = new ChannelProperties();
      // Java: ChannelProperty cpx = new ChannelProperty("X", "resolution", 28.36041, "1/cm");
      // Java: cps.addChannelProperty(cpx);
      // Java: ChannelProperty cpy = new ChannelProperty("Y", "resolution", 28.36041, "1/cm");
      // Java: cps.addChannelProperty(cpy);
      // Java: if (createFChannel)
      {
         // Java: ChannelProperty cpf = new ChannelProperty("F", "resolution", (double)1.0F, "1/dev");
         // Java: cps.addChannelProperty(cpf);
      }
      // Java: InkSource inkSource = new InkSource();
      // Java: inkSource.setTraceFormat(traceFormat);
      // Java: inkSource.setChannelProperties(cps);
      // Java: inkSource.setId("inkSrc0");
      // Java: Context context = new Context();
      // Java: context.setId("ctx0");
      // Java: context.setInkSource(inkSource);
      // Java: return context;
    }

    // Java: int getIndexInBrushList(com.hp.hpl.inkml.Brush brush)
    pub fn getIndexInBrushList(&self) -> i32 {
      // Java: for(int i = 0; i < this.mBrushList.size(); ++i)
      {
         // Java: if (((com.hp.hpl.inkml.Brush)this.mBrushList.get(i)).propertiesToInkML().equals(brush.propertiesToInkML()))
         {
            // Java: return i;
         }
      }
      // Java: return -1;
      -1
    }

    // Java: void setSelectRect(float x, float y)
    pub fn setSelectRect(&mut self, x: f32, y: f32) {
      // Java: if (this.mIsBegin)
      if self.mIsBegin
      {
         // Java: this.mIsBegin = false;
         self.mIsBegin = false;
         // Java: this.mSelectRect.left = this.mSelectRect.right = x;
         self.mSelectRect.left = x;
         self.mSelectRect.right = x;
         // Java: this.mSelectRect.top = this.mSelectRect.bottom = y;
         self.mSelectRect.top = y;
         self.mSelectRect.bottom = y;
      }
      // Java: else
      else
      {
         // Java: if (this.mSelectRect.left > x)
         if self.mSelectRect.left > x
         {
            // Java: this.mSelectRect.left = x;
            self.mSelectRect.left = x;
         }
         // Java: else if (this.mSelectRect.right < x)
         else if self.mSelectRect.right < x
         {
            // Java: this.mSelectRect.right = x;
            self.mSelectRect.right = x;
         }
         // Java: if (this.mSelectRect.top > y)
         if self.mSelectRect.top > y
         {
            // Java: this.mSelectRect.top = y;
            self.mSelectRect.top = y;
         }
         // Java: else if (this.mSelectRect.bottom < y)
         else if self.mSelectRect.bottom < y
         {
            // Java: this.mSelectRect.bottom = y;
            self.mSelectRect.bottom = y;
         }
      }
    }

    // Java: String getBrushRef()
    pub fn getBrushRef(&self) -> String {
      // Java: return "br" + this.mCurrentBrushIndex;
      format!("br{}", self.mCurrentBrushIndex)
    }

    // Java: void draw(Canvas canvas, Paint paint, Path path, float minStrokeWidth, boolean forceMinStroke)
    pub fn draw_simple(&self) {
      // Java: this.draw(canvas, paint, path, minStrokeWidth, forceMinStroke, false);
    }

    // Java: void draw(Canvas canvas, Paint paint, Path path, float minStrokeWidth, boolean forceMinStroke, boolean resetPath)
    pub fn draw_reset(&self) {
      // Java: RectF rect = this.getRect();
      // Java: canvas.save();
      // Java: canvas.translate(rect.left, rect.top);
      // Java: if (null == this.mInkPainter)
      {
         // Java: this.mInkPainter = new InkPainter(this.mInk, this.isNightMode, this.mNeedColorFilter);
      }
      // Java: InkPainter painter = this.mInkPainter;
      // Java: if (PainterFactory.isWritingBrush(this.mBrush))
      {
         // Java: painter.setWritingBrushList(this.mWritingBrushList);
      }
      // Java: painter.drawPound(canvas, paint, path, rect.width(), rect.height(), minStrokeWidth, forceMinStroke, false, true, resetPath);
      // Java: canvas.restore();
    }

    // Java: void draw(Canvas canvas, Paint paint, Path path, float minStrokeWidth, boolean forceMinStroke, float scaleX, float scaleY)
    pub fn draw_scale(&self) {
      // Java: if (null == this.mDataList)
      {
         // Java: this.draw(canvas, paint, path, minStrokeWidth, forceMinStroke);
      }
      // Java: else if (null != this.mBrush && this.mDataList.size() > 2)
      {
         // Java: float traceScale = this.getScale();
         // Java: Converters.convertPaint(paint, this.mBrush, minStrokeWidth * traceScale, false, forceMinStroke);
         // Java: InkPainter.drawDataList(canvas, paint, path, this.mPainter, this.mDataList, this.mBrush, traceScale, traceScale, scaleX, scaleY, false, null != this.mWritingBrushList);
         // Java: InkEstimate inkEstimate = InkEstimate.getInstance();
         // Java: if (inkEstimate != null)
         {
            // Java: inkEstimate.draw(canvas, paint, path, this.mPainter, this.mDataList, this.mBrush, traceScale, traceScale, scaleX, scaleY);
         }
      }
    }

    // Java: void setNightMode(boolean isNightMOde)
    pub fn setNightMode(&mut self, is_night_mode: bool) {
      // Java: this.isNightMode = isNightMOde;
      self.isNightMode = is_night_mode;
    }

    // Java: void setNeedColorFilter(boolean need)
    pub fn setNeedColorFilter(&mut self, need: bool) {
      // Java: this.mNeedColorFilter = need;
      self.mNeedColorFilter = need;
    }

    // Java: void setColor(int color)
    pub fn setColor(&self, _color: i32) {
      // Java: if (null != this.mInkPainter)
      {
         // Java: this.mInkPainter.setColor(color);
      }
      // Java: if (this.mInk != null)
      {
         // Java: String sColor = Integer.toHexString(color);
         // Java: sColor = sColor.substring(sColor.length() - 6);
         // Java: this.mInk.getDefinitions().setAllBrushColorProperty("#" + sColor);
      }
    }

    // Java: void setSize(float size)
    pub fn setSize(&self, _size: f32) {
      // Java: float size_lm = size * 26.458334F / 1000.0F;
      // Java: if (null != this.mInkPainter)
      {
         // Java: this.mInkPainter.setSize(size_lm);
      }
      // Java: if (this.mInk != null)
      {
         // Java: this.mInk.getDefinitions().setBrushProperty("br1", "width", String.valueOf(size_lm), "cm");
         // Java: this.mInk.getDefinitions().setBrushProperty("br1", "height", String.valueOf(size_lm), "cm");
      }
    }

}
