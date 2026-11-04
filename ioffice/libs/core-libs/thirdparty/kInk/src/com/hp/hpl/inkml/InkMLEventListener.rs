use crate::com::hp::hpl::inkml::IBrush::IBrush;
use crate::com::hp::hpl::inkml::TraceFormat::TraceFormat;
use crate::com::hp::hpl::inkml::Canvas::Canvas;
use crate::com::hp::hpl::inkml::CanvasTransform::CanvasTransform;
use crate::com::hp::hpl::inkml::InkSource::InkSource;
use crate::com::hp::hpl::inkml::Timestamp::Timestamp;
use crate::com::hp::hpl::inkml::Context::Context;
use crate::com::hp::hpl::inkml::Trace::Trace;
use crate::com::hp::hpl::inkml::TraceView::TraceView;
use crate::com::hp::hpl::inkml::TraceGroup::TraceGroup;

// Java: public interface InkMLEventListener
pub trait InkMLEventListener {
    // Java: abstract void brushChangedEvent(IBrush var1)
    fn brushChangedEvent(&self, var1: &dyn IBrush);

    // Java: abstract void traceFormatChangedEvent(TraceFormat var1)
    fn traceFormatChangedEvent(&self, var1: &TraceFormat);

    // Java: abstract void canvasChangedEvent(Canvas var1)
    fn canvasChangedEvent(&self, var1: &Canvas);

    // Java: abstract void canvasTransformChangedEvent(CanvasTransform var1)
    fn canvasTransformChangedEvent(&self, var1: &CanvasTransform);

    // Java: abstract void inkSourceChangedEvent(InkSource var1)
    fn inkSourceChangedEvent(&self, var1: &InkSource);

    // Java: abstract void timestampChangedEvent(Timestamp var1)
    fn timestampChangedEvent(&self, var1: &Timestamp);

    // Java: abstract void contextChangedEvent(Context var1)
    fn contextChangedEvent(&self, var1: &Context);

    // Java: abstract void traceReceivedEvent(Trace var1)
    fn traceReceivedEvent(&self, var1: &Trace);

    // Java: abstract void traceViewReceivedEvent(TraceView var1)
    fn traceViewReceivedEvent(&self, var1: &TraceView);

    // Java: abstract void traceGroupReceivedEvent(TraceGroup var1)
    fn traceGroupReceivedEvent(&self, var1: &TraceGroup);

}
