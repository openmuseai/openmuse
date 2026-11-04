#[allow(unused_imports)]
use util::cn::wps::moffice::util::KSLog;
// Java: import cn.wps.moffice.util.KSLog
// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.io.UnsupportedEncodingException
// Java: import java.io.Writer
// Java: import java.util.ArrayList

use crate::com::hp::hpl::inkml::Ink::Ink;
use crate::com::hp::hpl::inkml::InkMLSerializer::InkMLWriter;
use crate::com::hp::hpl::inkml::Context::Context;
use crate::com::hp::hpl::inkml::Trace::Trace;
use crate::com::hp::hpl::inkml::TraceView::TraceView;
use crate::com::hp::hpl::inkml::InkSource::InkSource;
use crate::com::hp::hpl::inkml::TraceFormat::TraceFormat;
use crate::com::hp::hpl::inkml::Canvas::Canvas;
use crate::com::hp::hpl::inkml::CanvasTransform::CanvasTransform;
use crate::com::hp::hpl::inkml::Timestamp::Timestamp;
use crate::com::hp::hpl::inkml::IBrush::IBrush;

pub trait InkMLEventListener {
    fn traceReceivedEvent(&self, _trace: &Trace) {}
    fn traceViewReceivedEvent(&self, _trace_view: &TraceView) {}
    fn traceGroupReceivedEvent(&self, _trace_group: &crate::com::hp::hpl::inkml::TraceGroup::TraceGroup) {}
    fn brushChangedEvent(&self, _brush: &dyn IBrush) {}
    fn inkSourceChangedEvent(&self, _ink_source: &InkSource) {}
    fn traceFormatChangedEvent(&self, _trace_format: &TraceFormat) {}
    fn canvasChangedEvent(&self, _canvas: &Canvas) {}
    fn canvasTransformChangedEvent(&self, _canvas_transform: &CanvasTransform) {}
    fn timestampChangedEvent(&self, _timestamp: &Timestamp) {}
}

// Java field: String TAG
pub(crate) const TAG: &str = "InkMLProcessor";

// Java: public class InkMLProcessor
pub struct InkMLProcessor {
    // Java field: ArrayList<InkMLEventListener> listenerList
    listenerList: Vec<Box<dyn InkMLEventListener>>,
    // Java field: Ink ink
    pub(crate) ink: Option<Ink>,
}

// Java: public class InkMLProcessor
impl InkMLProcessor {
    // Java: InkMLProcessor()
    pub fn InkMLProcessor() -> Self {
      // Java: super();
      Self {
          listenerList: Vec::new(),
          ink: None,
      }
    }

    // Java: void addInkMLEventListener(InkMLEventListener listener)
    pub fn addInkMLEventListener(&mut self, listener: Box<dyn InkMLEventListener>) {
      // Java: this.listenerList.add(listener);
      self.listenerList.push(listener);
    }

    // Java: void removeInkMLEventListener(InkMLEventListener listener)
    pub fn removeInkMLEventListener(&mut self, _listener: Box<dyn InkMLEventListener>) {
      // Java: this.listenerList.remove(listener);
    }

    // Java: void parseInkMLFile(String inkmlFileName)
    pub fn parseInkMLFile(&mut self, _inkml_file_name: &str) {
      // Java: this.ink = new Ink();
      self.ink = Some(Ink::Ink());
      // Java: InkMLParser parser = new InkMLDOMParser(this);
      // Java: parser.parseInkMLFile(inkmlFileName);
    }

    // Java: void parseInkMLString(String inkmlString)
    pub fn parseInkMLString(&mut self, _inkml_string: &str) {
      // Java: if (this.ink == null)
      if self.ink.is_none()
      {
         // Java: throw new InkMLException("ParseInk operation terminated. Reason: No active Ink session available.");
         panic!("ParseInk operation terminated. Reason: No active Ink session available.");
      }
      // Java: else
      else
      {
         // Java: InkMLParser parser = new InkMLDOMParser(this);
         // Java: parser.parseInkMLString(inkmlString);
      }
    }

    // Java: void notifyContextChanged(Context context, ArrayList<Ink.contextChangeStatus> ctxChanges)
    pub fn notifyContextChanged(&self, _context: &Context, _ctx_changes: &Vec<i32>) {
      // Java: KSLog.w(TAG, "To notify - context changed");
      // Java: if (0 != this.listenerList.size())
      if !self.listenerList.is_empty()
      {
         // Java: if (0 != ctxChanges.size())
         if !_ctx_changes.is_empty()
         {
            // Java: for(int i = 0; i < ctxChanges.size(); ++i)
            for _i in 0.._ctx_changes.len()
            {
               // Java: if (ctxChanges.get(i) == Ink.contextChangeStatus.isBrushChanged)
               {
                  // Java: this.notifyBrushChangedEvent(context.getBrush());
               }
               // Java: else if (ctxChanges.get(i) == Ink.contextChangeStatus.isCanvasChanged)
               {
                  // Java: this.notifyCanvasChangedEvent(context.getCanvas());
               }
               // Java: else if (ctxChanges.get(i) == Ink.contextChangeStatus.isCanvasTransformChanged)
               {
                  // Java: this.notifyCanvasTransformChangedEvent(context.getCanvasTransform());
               }
               // Java: else if (ctxChanges.get(i) == Ink.contextChangeStatus.isInkSourceChanged)
               {
                  // Java: this.notifyInkSourceChangedEvent(context.getInkSource());
               }
               // Java: else if (ctxChanges.get(i) == Ink.contextChangeStatus.isTimestampChanged)
               {
                  // Java: this.notifyTimestampChangedEvent(context.getTimestamp());
               }
               // Java: else if (ctxChanges.get(i) == Ink.contextChangeStatus.isTraceFormatChanged)
               {
                  // Java: this.notifyTraceFormatChangedEvent(context.getTraceFormat());
               }
            }
         }
      }
    }

    // Java: void notifyTraceReceived(Trace trace)
    pub fn notifyTraceReceived(&self, trace: &Trace) {
      // Java: KSLog.w(TAG, "To notify - trace received");
      // Java: if (0 != this.listenerList.size())
      if !self.listenerList.is_empty()
      {
         // Java: InkMLEventListener[] listeners = (InkMLEventListener[])this.listenerList.toArray(new InkMLEventListener[this.listenerList.size()]);
         // Java: for(int i = 0; i < listeners.length; ++i)
         for listener in &self.listenerList
         {
            // Java: listeners[i].traceReceivedEvent(trace);
            listener.traceReceivedEvent(trace);
         }
      }
    }

    // Java: void notifyTraceViewReceived(TraceView traceView)
    pub fn notifyTraceViewReceived(&self, trace_view: &TraceView) {
      // Java: if (0 != this.listenerList.size())
      if !self.listenerList.is_empty()
      {
         // Java: InkMLEventListener[] listeners = (InkMLEventListener[])this.listenerList.toArray(new InkMLEventListener[this.listenerList.size()]);
         // Java: for(int i = 0; i < listeners.length; ++i)
         for listener in &self.listenerList
         {
            // Java: listeners[i].traceViewReceivedEvent(traceView);
            listener.traceViewReceivedEvent(trace_view);
         }
      }
    }

    // Java: void notifyTraceGroupReceived(TraceGroup traceGroup)
    pub fn notifyTraceGroupReceived(&self, trace_group: &crate::com::hp::hpl::inkml::TraceGroup::TraceGroup) {
      // Java: if (0 != this.listenerList.size())
      if !self.listenerList.is_empty()
      {
         // Java: InkMLEventListener[] listeners = (InkMLEventListener[])this.listenerList.toArray(new InkMLEventListener[this.listenerList.size()]);
         // Java: for(int i = 0; i < listeners.length; ++i)
         for listener in &self.listenerList
         {
            // Java: listeners[i].traceGroupReceivedEvent(traceGroup);
            listener.traceGroupReceivedEvent(trace_group);
         }
      }
    }

    // Java: void notifyBrushChangedEvent(IBrush brush)
    pub fn notifyBrushChangedEvent(&self, brush: &dyn IBrush) {
      // Java: KSLog.w(TAG, "To notify - brush changed");
      // Java: InkMLEventListener[] listeners = (InkMLEventListener[])this.listenerList.toArray(new InkMLEventListener[this.listenerList.size()]);
      // Java: for(int i = 0; i < listeners.length; ++i)
      for listener in &self.listenerList
      {
         // Java: listeners[i].brushChangedEvent(brush);
         listener.brushChangedEvent(brush);
      }
    }

    // Java: void notifyInkSourceChangedEvent(InkSource inkSource)
    pub fn notifyInkSourceChangedEvent(&self, ink_source: &InkSource) {
      // Java: InkMLEventListener[] listeners = (InkMLEventListener[])this.listenerList.toArray(new InkMLEventListener[this.listenerList.size()]);
      // Java: for(int i = 0; i < listeners.length; ++i)
      for listener in &self.listenerList
      {
         // Java: listeners[i].inkSourceChangedEvent(inkSource);
         listener.inkSourceChangedEvent(ink_source);
      }
    }

    // Java: void notifyTraceFormatChangedEvent(TraceFormat traceFormat)
    pub fn notifyTraceFormatChangedEvent(&self, trace_format: &TraceFormat) {
      // Java: InkMLEventListener[] listeners = (InkMLEventListener[])this.listenerList.toArray(new InkMLEventListener[this.listenerList.size()]);
      // Java: for(int i = 0; i < listeners.length; ++i)
      for listener in &self.listenerList
      {
         // Java: listeners[i].traceFormatChangedEvent(traceFormat);
         listener.traceFormatChangedEvent(trace_format);
      }
    }

    // Java: void notifyCanvasChangedEvent(Canvas canvas)
    pub fn notifyCanvasChangedEvent(&self, canvas: &Canvas) {
      // Java: InkMLEventListener[] listeners = (InkMLEventListener[])this.listenerList.toArray(new InkMLEventListener[this.listenerList.size()]);
      // Java: for(int i = 0; i < listeners.length; ++i)
      for listener in &self.listenerList
      {
         // Java: listeners[i].canvasChangedEvent(canvas);
         listener.canvasChangedEvent(canvas);
      }
    }

    // Java: void notifyCanvasTransformChangedEvent(CanvasTransform canvasTransform)
    pub fn notifyCanvasTransformChangedEvent(&self, canvas_transform: &CanvasTransform) {
      // Java: InkMLEventListener[] listeners = (InkMLEventListener[])this.listenerList.toArray(new InkMLEventListener[this.listenerList.size()]);
      // Java: for(int i = 0; i < listeners.length; ++i)
      for listener in &self.listenerList
      {
         // Java: listeners[i].canvasTransformChangedEvent(canvasTransform);
         listener.canvasTransformChangedEvent(canvas_transform);
      }
    }

    // Java: void notifyTimestampChangedEvent(Timestamp timestamp)
    pub fn notifyTimestampChangedEvent(&self, timestamp: &Timestamp) {
      // Java: InkMLEventListener[] listeners = (InkMLEventListener[])this.listenerList.toArray(new InkMLEventListener[this.listenerList.size()]);
      // Java: for(int i = 0; i < listeners.length; ++i)
      for listener in &self.listenerList
      {
         // Java: listeners[i].timestampChangedEvent(timestamp);
         listener.timestampChangedEvent(timestamp);
      }
    }

    // Java: boolean isInkNull()
    pub fn isInkNull(&self) -> bool {
      // Java: return this.ink == null;
      self.ink.is_none()
    }

    // Java: Ink getInk()
    pub fn getInk(&self) -> Option<&Ink> {
      // Java: return this.ink;
      self.ink.as_ref()
    }

    // Java: void loadInkSession(Ink ink)
    pub fn loadInkSession(&mut self, ink: Ink) {
      // Java: this.ink = ink;
      self.ink = Some(ink);
    }

    // Java: void beginInkSession()
    pub fn beginInkSession(&mut self) {
      // Java: this.ink = new Ink();
      self.ink = Some(Ink::Ink());
    }

    // Java: void endInkSession()
    pub fn endInkSession(&mut self) {
      // Java: this.ink = null;
      self.ink = None;
    }

    // Java: void resetInkSession()
    pub fn resetInkSession(&mut self) {
      // Java: this.ink = new Ink();
      self.ink = Some(Ink::Ink());
    }

    // Java: void saveInkSession(String fileName)
    pub fn saveInkSession_filename(&self, _file_name: &str) {
      // Java: if (this.ink == null)
      if self.ink.is_none()
      {
         // Java: throw new InkMLException("SaveInkSession operation terminated. Reason: No active Ink session available.");
         panic!("SaveInkSession operation terminated. Reason: No active Ink session available.");
      }
      // Java: else
      else
      {
         // Java: InkMLWriter writer = new InkMLWriter(fileName);
         // Java: writer.writeProcessingInstruction();
         // Java: this.ink.writeXML(writer);
         // Java: writer.close();
      }
    }

    // Java: void saveInkSession(String fileName, String encoding)
    pub fn saveInkSession_filename_encoding(&self, _file_name: &str, _encoding: &str) {
      // Java: if (this.ink == null)
      if self.ink.is_none()
      {
         // Java: throw new InkMLException("SaveInkSession operation terminated. Reason: No active Ink session available.");
         panic!("SaveInkSession operation terminated. Reason: No active Ink session available.");
      }
      // Java: else
      else
      {
         // Java: InkMLWriter writer = new InkMLWriter(fileName, encoding);
         // Java: writer.writeProcessingInstruction();
         // Java: this.ink.writeXML(writer);
         // Java: writer.close();
      }
    }

    // Java: void saveInkSession(Writer writer)
    pub fn saveInkSession_writer(&self, _writer: &mut dyn std::io::Write) {
      // Java: if (this.ink == null)
      if self.ink.is_none()
      {
         // Java: throw new InkMLException("SaveInkSession operation terminated. Reason: No active Ink session available.");
         panic!("SaveInkSession operation terminated. Reason: No active Ink session available.");
      }
      // Java: else
      else
      {
         // Java: InkMLWriter inkMLWriter = new InkMLWriter(writer);
         // Java: inkMLWriter.writeProcessingInstruction();
         // Java: this.ink.writeXML(inkMLWriter);
         // Java: inkMLWriter.close();
      }
    }

    // Java: void saveInkSession(OutputStream stream, String encoding)
    pub fn saveInkSession_stream_encoding(&self, _stream: &mut dyn std::io::Write, _encoding: &str) {
      // Java: if (this.ink == null)
      if self.ink.is_none()
      {
         // Java: throw new InkMLException("SaveInkSession operation terminated. Reason: No active Ink session available.");
         panic!("SaveInkSession operation terminated. Reason: No active Ink session available.");
      }
      // Java: else
      else
      {
         // Java: InkMLWriter inkMLWriter = new InkMLWriter(stream, encoding);
         // Java: inkMLWriter.writeProcessingInstruction();
         // Java: this.ink.writeXML(inkMLWriter);
         // Java: inkMLWriter.close();
      }
    }

    // Java: void saveInkSession(OutputStream stream)
    pub fn saveInkSession_stream(&self, _stream: &mut dyn std::io::Write) {
      // Java: if (this.ink == null)
      if self.ink.is_none()
      {
         // Java: throw new InkMLException("SaveInkSession operation terminated. Reason: No active Ink session available.");
         panic!("SaveInkSession operation terminated. Reason: No active Ink session available.");
      }
      // Java: else
      else
      {
         // Java: InkMLWriter inkMLWriter = new InkMLWriter(stream);
         // Java: inkMLWriter.writeProcessingInstruction();
         // Java: this.ink.writeXML(inkMLWriter);
         // Java: inkMLWriter.close();
      }
    }

}
