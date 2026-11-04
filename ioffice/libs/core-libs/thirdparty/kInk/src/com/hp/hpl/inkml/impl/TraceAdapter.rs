#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import android.graphics.RectF
use util::graphics::RectF::RectF;
// Java: import com.hp.hpl.inkml.Channel
use crate::com::hp::hpl::inkml::Channel::ChannelType;
// Java: import com.hp.hpl.inkml.ChannelProperties
// Java: import com.hp.hpl.inkml.ChannelProperty
// Java: import com.hp.hpl.inkml.Context
use crate::com::hp::hpl::inkml::Context::Context;
// Java: import com.hp.hpl.inkml.Trace
use crate::com::hp::hpl::inkml::Trace::Trace;
// Java: import com.hp.hpl.inkml.TraceDataList
use crate::com::hp::hpl::inkml::TraceDataList::TraceDataList;
// Java: import com.hp.hpl.inkml.TraceFormat
use crate::com::hp::hpl::inkml::TraceFormat::TraceFormat;

use super::Brush::Brush;

// Java: public class TraceAdapter
pub struct TraceAdapter {
    // Java field: double normalizationWeight
    pub(crate) normalizationWeight: f64,
    // Java field: double coefX0
    pub(crate) coefX0: f64,
    // Java field: double coefY0
    pub(crate) coefY0: f64,
    // Java field: Brush mBrush
    pub(crate) mBrush: Brush,
    // Java field: Trace mTrace
    pub(crate) mTrace: Trace,
}

// Java: public class TraceAdapter
impl TraceAdapter {
    // Java: TraceAdapter(Trace trace)
    pub fn TraceAdapter(trace: Trace) -> Self {
        // Java: super();
        // Java: this.mTrace = trace;
        Self {
            normalizationWeight: 0.0,
            coefX0: 0.0,
            coefY0: 0.0,
            mBrush: Brush::Brush(),
            mTrace: trace,
        }
    }

    // Java: long getStartTime()
    pub fn getStartTime(&self) -> i64 {
        // Java: return (long)this.mTrace.getTimeOffset();
        self.mTrace.getTimeOffset() as i64
    }

    // Java: Brush getBrush()
    pub fn getBrush(&self) -> &Brush {
        // Java: return this.mBrush;
        &self.mBrush
    }

    // Java: long getDuration()
    pub fn getDuration(&self) -> i64 {
        // Java: return this.mTrace.getDuration();
        self.mTrace.getDuration()
    }

    // Java: static TraceAdapter fromInkML(Trace srcTrace, Context currentContext)
    pub fn fromInkML(srcTrace: Trace, _currentContext: Option<&Context>) -> TraceAdapter {
        // Java: TraceAdapter dstTrace = new TraceAdapter(srcTrace);
        let mut dstTrace = TraceAdapter::TraceAdapter(srcTrace);
        // Java: Context context = srcTrace.getAssociatedContext();
        // Java: context = null == context ? currentContext : context;
        // Java: Brush brush = Brush.fromInkML(context.getBrush());
        let brush = Brush::Brush();
        // Java: if (srcTrace.getTraceData().getFIndex() >= 0)
        // Java: brush.setMaxFChannel(getMaxFChannel(context));
        // Java: dstTrace.mBrush = brush;
        dstTrace.mBrush = brush;
        // Java: ChannelProperties cps = srcTrace.getChannelProperties();
        // Java: if (cps != null)
        // Java: ChannelProperty cp = cps.getChannelPropertyByChannel("X");
        // Java: if (cp.isResolution())
        // Java: Channel channel = srcTrace.getAssociatedContext().getTraceFormat().getChannel("X");
        // Java: if (null == channel)
        // Java: return dstTrace;
        // Java: String channelUnits = channel.getUnits();
        // Java: if (("1/" + channelUnits).equals(cp.getUnits()))
        // Java: float width = dstTrace.getBrush().getWidth();
        // Java: width *= 1000.0F;
        // Java: dstTrace.getBrush().setWidth(width);
        // Java: float height = dstTrace.getBrush().getHeight();
        // Java: height *= 1000.0F;
        // Java: dstTrace.getBrush().setHeight(height);
        // Java: return dstTrace;
        dstTrace
    }

    // Java: static float getMaxFChannel(Context context)
    pub fn getMaxFChannel(_context: Option<&Context>) -> f32 {
        // Java: float maxFChannel = 0.0F;
        let maxFChannel = 0.0f32;
        // Java: if (null == context)
        if _context.is_none() {
            // Java: return maxFChannel;
            return maxFChannel;
        // Java: else
        } else {
            // Java: TraceFormat traceFormat = context.getTraceFormat();
            // Java: if (null == traceFormat)
            // Java: return maxFChannel;
            // Java: else
            // Java: Channel fChannel = traceFormat.getChannel("F");
            // Java: if (null == fChannel)
            // Java: return maxFChannel;
            // Java: else
            // Java: String max = fChannel.getMax();
            // Java: if (null != max && max.length() != 0)
            // Java: Channel.ChannelType type = fChannel.getChannelType();
            // Java: switch (type)
            // Java: case DECIMAL:
            // Java: case INTEGER:
            // Java: maxFChannel = new Float(max);
            // Java: default:
            // Java: return maxFChannel;
            // Java: else
            // Java: return 1023.0F;
            return 1023.0;
        }
    }

    // Java: void setNormalizationWeight(double paramDouble)
    pub fn setNormalizationWeight(&mut self, paramDouble: f64) {
        // Java: this.normalizationWeight = paramDouble;
        self.normalizationWeight = paramDouble;
    }

    // Java: double getNormalizationWeight()
    pub fn getNormalizationWeight(&self) -> f64 {
        // Java: return this.normalizationWeight;
        self.normalizationWeight
    }

    // Java: void setCoefX0(double paramDouble)
    pub fn setCoefX0(&mut self, paramDouble: f64) {
        // Java: this.coefX0 = paramDouble;
        self.coefX0 = paramDouble;
    }

    // Java: double getCoefX0()
    pub fn getCoefX0(&self) -> f64 {
        // Java: return this.coefX0;
        self.coefX0
    }

    // Java: void setCoefY0(double paramDouble)
    pub fn setCoefY0(&mut self, paramDouble: f64) {
        // Java: this.coefY0 = paramDouble;
        self.coefY0 = paramDouble;
    }

    // Java: double getCoefY0()
    pub fn getCoefY0(&self) -> f64 {
        // Java: return this.coefY0;
        self.coefY0
    }

    // Java: int compareTo(TraceAdapter paramTrace)
    pub fn compareTo(&self, paramTrace: Option<&TraceAdapter>) -> i32 {
        // Java: if (paramTrace == null)
        if paramTrace.is_none() {
            // Java: throw new NullPointerException("The trace to compare to was null.");
            return 0;
        // Java: else if (this.getStartTime() == 0L && paramTrace.getStartTime() == 0L)
        } else if self.getStartTime() == 0 && paramTrace.unwrap().getStartTime() == 0 {
            // Java: if (this.getStartTime() == 0L && paramTrace.getStartTime() == 0L)
            if self.getStartTime() == 0 && paramTrace.unwrap().getStartTime() == 0 {
                // Java: return 0;
                return 0;
            // Java: else
            } else {
                // Java: return this.getStartTime() == 0L ? -1 : 1;
                return if self.getStartTime() == 0 { -1 } else { 1 };
            }
        // Java: else if (this.getStartTime() < paramTrace.getStartTime())
        } else if self.getStartTime() < paramTrace.unwrap().getStartTime() {
            // Java: return -1;
            -1
        // Java: else
        } else {
            // Java: return this.getStartTime() > paramTrace.getStartTime() ? 1 : 0;
            if self.getStartTime() > paramTrace.unwrap().getStartTime() { 1 } else { 0 }
        }
    }

    // Java: TraceDataList getTraceData()
    pub fn getTraceData(&self) -> &TraceDataList {
        // Java: return this.mTrace.getTraceData();
        self.mTrace.getTraceData()
    }

    // Java: RectF getRect()
    pub fn getRect(&self) -> RectF {
        // Java: return this.mTrace.getRectAfterTransform();
        self.mTrace.getRectAfterTransform()
    }

    // Java: float getScaleX()
    pub fn getScaleX(&self) -> f32 {
        // Java: return this.mTrace.mScaleX;
        self.mTrace.mScaleX
    }

    // Java: float getScaleY()
    pub fn getScaleY(&self) -> f32 {
        // Java: return this.mTrace.mScaleY;
        self.mTrace.mScaleY
    }

    // Java: float getTranslateX()
    pub fn getTranslateX(&self) -> f32 {
        // Java: return this.mTrace.mTranslateX;
        self.mTrace.mTranslateX
    }

    // Java: float getTranslateY()
    pub fn getTranslateY(&self) -> f32 {
        // Java: return this.mTrace.mTranslateY;
        self.mTrace.mTranslateY
    }
}
