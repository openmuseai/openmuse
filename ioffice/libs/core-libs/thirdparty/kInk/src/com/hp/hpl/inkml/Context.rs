#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.util.KSLog
use util::cn::wps::moffice::util::KSLog::KSLog;
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Iterator
// Java: import java.util.Map
// Java: import java.util.Set
// Java: import java.util.TreeMap

use super::InkMLSerializer::InkMLWriter;
use super::Brush::Brush;
use super::Canvas::Canvas;
use super::CanvasTransform::CanvasTransform;
use super::InkSource::InkSource;
use super::TraceFormat::TraceFormat;
use super::Timestamp::Timestamp;
use super::Definitions::Definitions;

// Java field: String TAG
const TAG: &str = "Context";

// Java: public class Context implements InkElement, Cloneable
pub struct Context {
    // Java field: HashMap<String, String> attributesMap
    pub(crate) attributesMap: HashMap<String, String>,
    // Java field: HashMap<String, ContextElement> ctxElementList
    pub(crate) ctxElementList: HashMap<String, String>,
    // Java field: IBrush brush
    pub(crate) brush: Option<Brush>,
    // Java field: TraceFormat traceFormat
    pub(crate) traceFormat: Option<TraceFormat>,
    // Java field: InkSource inkSource
    pub(crate) inkSource: Option<InkSource>,
    // Java field: Canvas canvas
    pub(crate) canvas: Option<Canvas>,
    // Java field: CanvasTransform canvasTransform
    pub(crate) canvasTransform: Option<CanvasTransform>,
    // Java field: Timestamp timestamp
    pub(crate) timestamp: Option<Timestamp>,
}

// Java: public class Context implements InkElement, Cloneable
impl Context {
    // Java: Context()
    pub fn Context() -> Self {
        // Java: super();
        // Java: this.attributesMap = new HashMap();
        // Java: this.ctxElementList = new HashMap();
        Self {
            attributesMap: HashMap::new(),
            ctxElementList: HashMap::new(),
            brush: None,
            traceFormat: None,
            inkSource: None,
            canvas: None,
            canvasTransform: None,
            timestamp: None,
        }
    }

    // Java: void setAttribute(String attrName, String attrValue)
    pub fn setAttribute(&mut self, attrName: &str, attrValue: &str) {
        // Java: this.attributesMap.put(attrName, attrValue);
        self.attributesMap.insert(attrName.to_string(), attrValue.to_string());
    }

    // Java: String getBrushRef()
    pub fn getBrushRef(&self) -> String {
        // Java: String reference = (String)this.attributesMap.get("brushRef");
        // Java: return null == reference ? "" : reference;
        self.attributesMap.get("brushRef").cloned().unwrap_or_default()
    }

    // Java: void setBrushRef(String brushRef)
    pub fn setBrushRef(&mut self, brushRef: &str) {
        // Java: this.attributesMap.put("brushRef", brushRef);
        self.attributesMap.insert("brushRef".to_string(), brushRef.to_string());
    }

    // Java: String getCanvasRef()
    pub fn getCanvasRef(&self) -> String {
        // Java: String reference = (String)this.attributesMap.get("canvasRef");
        // Java: return null == reference ? "" : reference;
        self.attributesMap.get("canvasRef").cloned().unwrap_or_default()
    }

    // Java: void setCanvasRef(String canvasRef)
    pub fn setCanvasRef(&mut self, canvasRef: &str) {
        // Java: this.attributesMap.put("canvasRef", canvasRef);
        self.attributesMap.insert("canvasRef".to_string(), canvasRef.to_string());
    }

    // Java: String getCanvasTransformRef()
    pub fn getCanvasTransformRef(&self) -> String {
        // Java: String reference = (String)this.attributesMap.get("canvasTransformRef");
        // Java: return null == reference ? "" : reference;
        self.attributesMap.get("canvasTransformRef").cloned().unwrap_or_default()
    }

    // Java: void setCanvasTransformRef(String canvasTransformRef)
    pub fn setCanvasTransformRef(&mut self, canvasTransformRef: &str) {
        // Java: this.attributesMap.put("canvasTransformRef", canvasTransformRef);
        self.attributesMap.insert("canvasTransformRef".to_string(), canvasTransformRef.to_string());
    }

    // Java: String getInkSourceRef()
    pub fn getInkSourceRef(&self) -> String {
        // Java: String reference = (String)this.attributesMap.get("inkSourceRef");
        // Java: return null == reference ? "" : reference;
        self.attributesMap.get("inkSourceRef").cloned().unwrap_or_default()
    }

    // Java: void setInkSourceRef(String inkSourceRef)
    pub fn setInkSourceRef(&mut self, inkSourceRef: &str) {
        // Java: this.attributesMap.put("inkSourceRef", inkSourceRef);
        self.attributesMap.insert("inkSourceRef".to_string(), inkSourceRef.to_string());
    }

    // Java: String getTimestampRef()
    pub fn getTimestampRef(&self) -> String {
        // Java: String reference = (String)this.attributesMap.get("timestampRef");
        // Java: return null == reference ? "" : reference;
        self.attributesMap.get("timestampRef").cloned().unwrap_or_default()
    }

    // Java: void setTimestampRef(String timestampRef)
    pub fn setTimestampRef(&mut self, timestampRef: &str) {
        // Java: this.attributesMap.put("timestampRef", timestampRef);
        self.attributesMap.insert("timestampRef".to_string(), timestampRef.to_string());
    }

    // Java: String getTraceFormatRef()
    pub fn getTraceFormatRef(&self) -> String {
        // Java: String reference = (String)this.attributesMap.get("traceFormatRef");
        // Java: return null == reference ? "" : reference;
        self.attributesMap.get("traceFormatRef").cloned().unwrap_or_default()
    }

    // Java: void setTraceFormatRef(String traceFormatRef)
    pub fn setTraceFormatRef(&mut self, traceFormatRef: &str) {
        // Java: this.attributesMap.put("traceFormatRef", traceFormatRef);
        self.attributesMap.insert("traceFormatRef".to_string(), traceFormatRef.to_string());
    }

    // Java: void setCanvasTransform(CanvasTransform canvasTransform)
    pub fn setCanvasTransform(&mut self, canvasTransform: CanvasTransform) {
        // Java: this.canvasTransform = canvasTransform;
        self.canvasTransform = Some(canvasTransform);
    }

    // Java: void setContextRef(String contextRef)
    pub fn setContextRef(&mut self, contextRef: &str) {
        // Java: this.attributesMap.put("contextRef", contextRef);
        self.attributesMap.insert("contextRef".to_string(), contextRef.to_string());
    }

    // Java: void setTimestamp(Timestamp timestamp)
    pub fn setTimestamp(&mut self, timestamp: Timestamp) {
        // Java: this.timestamp = timestamp;
        self.timestamp = Some(timestamp);
    }

    // Java: Context(Context context)
    pub fn Context_copy(context: &Context) -> Self {
        // Java: this();
        let mut s = Self::Context();
        // Java: this.brush = context.getBrush();
        s.brush = context.brush.as_ref().map(|b| b.clone_brush());
        // Java: this.traceFormat = context.getTraceFormat();
        s.traceFormat = context.traceFormat.as_ref().map(|tf| tf.clone_tf());
        // Java: this.inkSource = context.getInkSource();
        s.inkSource = context.inkSource.as_ref().map(|is| is.clone_is());
        // Java: this.canvas = context.getCanvas();
        s.canvas = context.canvas.as_ref().map(|c| c.clone_canvas());
        // Java: this.canvasTransform = context.getCanvasTransform();
        s.canvasTransform = context.canvasTransform.as_ref().map(|ct| ct.clone_ct());
        // Java: this.timestamp = context.getTimestamp();
        s.timestamp = context.timestamp.as_ref().map(|t| t.clone_ts());
        s
    }

    // Java: static Context getDefaultContext()
    pub fn getDefaultContext() -> Context {
        // Java: Context defaultCtx = new Context();
        let mut defaultCtx = Context::Context();
        // Java: defaultCtx.setId("DefaultContext");
        defaultCtx.setId("DefaultContext");
        // Java: defaultCtx.setCanvasRef("#DefaultCanvas");
        defaultCtx.setCanvasRef("#DefaultCanvas");
        // Java: defaultCtx.setCanvas(Canvas.getDefaultCanvas());
        // Java: defaultCtx.setCanvasTransformRef("#DefaultCanvasTransform");
        defaultCtx.setCanvasTransformRef("#DefaultCanvasTransform");
        // Java: defaultCtx.setCanvasTransform(CanvasTransform.getDefaultCanvasTransform());
        // Java: defaultCtx.setTraceFormatRef("#DefaultTraceFormat");
        defaultCtx.setTraceFormatRef("#DefaultTraceFormat");
        // Java: defaultCtx.setTraceFormat(TraceFormat.getDefaultTraceFormat());
        // Java: defaultCtx.setInkSourceRef("#DefaultInkSource");
        defaultCtx.setInkSourceRef("#DefaultInkSource");
        // Java: defaultCtx.setInkSource(InkSource.getDefaultInkSource());
        // Java: defaultCtx.setBrushRef("#DefaultBrush");
        defaultCtx.setBrushRef("#DefaultBrush");
        // Java: defaultCtx.setBrush(Brush.getDefaultBrush());
        // Java: defaultCtx.setTimestampRef("#DefaultTimestamp");
        defaultCtx.setTimestampRef("#DefaultTimestamp");
        // Java: defaultCtx.setTimestamp(Timestamp.getDefaultTimestamp());
        // Java: return defaultCtx;
        defaultCtx
    }

    // Java: Canvas getCanvas()
    pub fn getCanvas(&self) -> Option<&Canvas> {
        // Java: return this.canvas;
        self.canvas.as_ref()
    }

    // Java: InkSource getInkSource()
    pub fn getInkSource(&self) -> Option<&InkSource> {
        // Java: return this.inkSource;
        self.inkSource.as_ref()
    }

    // Java: CanvasTransform getCanvasTransform()
    pub fn getCanvasTransform(&self) -> Option<&CanvasTransform> {
        // Java: return this.canvasTransform;
        self.canvasTransform.as_ref()
    }

    // Java: Timestamp getTimestamp()
    pub fn getTimestamp(&self) -> Option<&Timestamp> {
        // Java: return this.timestamp;
        self.timestamp.as_ref()
    }

    // Java: TraceFormat getTraceFormat()
    pub fn getTraceFormat(&self) -> Option<&TraceFormat> {
        // Java: if (this.traceFormat != null && !TraceFormat.isDefaultTraceFormat(this.traceFormat))
        if self.traceFormat.is_some() {
            // Java: return this.traceFormat;
            return self.traceFormat.as_ref();
        // Java: else
        } else {
            // Java: return this.inkSource != null && this.inkSource.getTraceFormat() != null ? this.inkSource.getTraceFormat() : this.traceFormat;
            self.traceFormat.as_ref()
        }
    }

    // Java: String getId()
    pub fn getId(&self) -> String {
        // Java: String id = (String)this.attributesMap.get("xml:id");
        let mut id = self.attributesMap.get("xml:id").cloned();
        // Java: if (id == null)
        if id.is_none() {
            // Java: String str = (String)this.attributesMap.get("id");
            let str_val = self.attributesMap.get("id").cloned();
            // Java: if (str != null)
            if let Some(s) = str_val {
                // Java: id = str;
                id = Some(s);
                // Java: KSLog.w(TAG, "Resolving ID for element without namespace.");
                KSLog::w(TAG, "Resolving ID for element without namespace.");
            }
        }
        // Java: return id == null ? "" : id;
        id.unwrap_or_default()
    }

    // Java: IBrush getBrush()
    pub fn getBrush(&self) -> Option<&Brush> {
        // Java: return this.brush;
        self.brush.as_ref()
    }

    // Java: String getInkElementType()
    pub fn getInkElementType(&self) -> String {
        // Java: return "Context";
        "Context".to_string()
    }

    // Java: String getContextRef()
    pub fn getContextRef(&self) -> String {
        // Java: String value = (String)this.attributesMap.get("contextRef");
        // Java: return null == value ? "" : value;
        self.attributesMap.get("contextRef").cloned().unwrap_or_default()
    }

    // Java: void setBrush(IBrush brush)
    pub fn setBrush(&mut self, brush: Brush) {
        // Java: this.brush = brush;
        self.brush = Some(brush);
    }

    // Java: void setTraceFormat(TraceFormat traceFormat)
    pub fn setTraceFormat(&mut self, traceFormat: TraceFormat) {
        // Java: this.traceFormat = traceFormat;
        self.traceFormat = Some(traceFormat);
        // Java: this.ctxElementList.put(TraceFormat.class.getSimpleName(), traceFormat);
        self.ctxElementList.insert("TraceFormat".to_string(), "TraceFormat".to_string());
    }

    // Java: void setCanvas(Canvas canvas)
    pub fn setCanvas(&mut self, canvas: Canvas) {
        // Java: this.canvas = canvas;
        self.canvas = Some(canvas);
        // Java: this.ctxElementList.put(Canvas.class.getSimpleName(), canvas);
        self.ctxElementList.insert("Canvas".to_string(), "Canvas".to_string());
    }

    // Java: void setInkSource(InkSource inkSource)
    pub fn setInkSource(&mut self, inkSource: InkSource) {
        // Java: this.inkSource = inkSource;
        self.inkSource = Some(inkSource);
        // Java: this.ctxElementList.put(InkSource.class.getSimpleName(), inkSource);
        self.ctxElementList.insert("InkSource".to_string(), "InkSource".to_string());
    }

    // Java: boolean equals2(Context context)
    pub fn equals2(&self, _context: Option<&Context>) -> bool {
        // Java: if (context == null)
        // Java: return false;
        // Java: else if (!this.brush.equals(context.brush))
        // Java: return false;
        // Java: else if (!this.traceFormat.equals2(context.traceFormat))
        // Java: return false;
        // Java: else if (!this.inkSource.equals2(context.inkSource))
        // Java: return false;
        // Java: else if (!this.canvas.equals2(context.canvas))
        // Java: return false;
        // Java: else if (!this.canvasTransform.equals2(context.canvasTransform))
        // Java: return false;
        // Java: else
        // Java: return this.timestamp.equals(context.timestamp);
        false
    }

    // Java: void setId(String id)
    pub fn setId(&mut self, id: &str) {
        // Java: this.attributesMap.put("id", id);
        self.attributesMap.insert("id".to_string(), id.to_string());
    }

    // Java: void addToContextElementList(ContextElement ctxChild)
    pub fn addToContextElementList(&mut self, _ctxChild: &str) {
        // Java: if (ctxChild != null)
        // Java: this.ctxElementList.put(ctxChild.getInkElementType(), ctxChild);
        // Java: String str = ctxChild.getInkElementType();
        // Java: if (str.equals(IBrush.class.getSimpleName()))
        // Java: this.brush = (IBrush)ctxChild;
        // Java: else if (str.equals(TraceFormat.class.getSimpleName()))
        // Java: this.traceFormat = (TraceFormat)ctxChild;
        // Java: else if (str.equals(InkSource.class.getSimpleName()))
        // Java: this.inkSource = (InkSource)ctxChild;
        // Java: else if (str.equals(Canvas.class.getSimpleName()))
        // Java: this.canvas = (Canvas)ctxChild;
        // Java: else if (str.equals(CanvasTransform.class.getSimpleName()))
        // Java: this.canvasTransform = (CanvasTransform)ctxChild;
        // Java: else if (str.equals(Timestamp.class.getSimpleName()))
        // Java: this.timestamp = (Timestamp)ctxChild;
        // Java: else
        // Java: KSLog.w(TAG, "Failed to add context element --- invalid type: " + str);
    }

    // Java: void deriveContextualChildrenData(Definitions defs, Context currentCtx)
    pub fn deriveContextualChildrenData(&mut self, _defs: &Definitions, _currentCtx: &Context) -> Result<(), String> {
        // Java: String ctxRef = this.getContextRef();
        let _ctxRef = self.getContextRef();
        // Java: if (!"".equals(ctxRef))
        // Java: this.copyFromContextReference(defs, ctxRef);
        // Java: String brushRef = this.getBrushRef();
        let _brushRef = self.getBrushRef();
        // Java: if (!"".equals(brushRef))
        // Java: IBrush refBrush = defs.getBrushRefElement(brushRef);
        // Java: if (this.brush == null)
        // Java: this.brush = refBrush;
        // Java: else
        // Java: this.brush = Brush.override(this.brush, refBrush);
        // Java: String inkSourceRef = this.getInkSourceRef();
        let _inkSourceRef = self.getInkSourceRef();
        // Java: if (!"".equals(inkSourceRef))
        // Java: this.inkSource = defs.getInkSourceRefElement(inkSourceRef);
        // Java: this.traceFormat = this.inkSource.getTraceFormat();
        // Java: String traceFormatRef = this.getTraceFormatRef();
        let _traceFormatRef = self.getTraceFormatRef();
        // Java: if (!"".equals(traceFormatRef))
        // Java: this.traceFormat = defs.getTraceFormatRefElement(traceFormatRef);
        // Java: int nChildren = this.ctxElementList.keySet().size();
        let nChildren = self.ctxElementList.len();
        // Java: KSLog.w(TAG, "CTX child List size: " + nChildren);
        KSLog::w(TAG, &format!("CTX child List size: {}", nChildren));
        // Java: if (0 != nChildren)
        // Java: for(ContextElement child : this.ctxElementList.values())
        // Java: String type = child.getInkElementType();
        // Java: if ("Brush".equals(type))
        // Java: KSLog.w(TAG, "CTX Brush child");
        // Java: IBrush currBrush = currentCtx.getBrush();
        // Java: this.brush = Brush.override(this.brush, (IBrush)child);
        // Java: else if ("InkSource".equalsIgnoreCase(type))
        // Java: this.inkSource = (InkSource)child;
        // Java: this.traceFormat = this.inkSource.getTraceFormat();
        // Java: else if ("TraceFormat".equals(type))
        // Java: if (((TraceFormat)child).channelMap.size() == 0)
        // Java: if (this.traceFormat == null)
        // Java: this.traceFormat = currentCtx.getTraceFormat();
        // Java: else
        // Java: KSLog.w(TAG, "overriding TF");
        // Java: this.traceFormat.override((TraceFormat)child);
        // Java: this.traceFormat = (TraceFormat)child;
        // Java: else if ("Canvas".equalsIgnoreCase(type))
        // Java: this.canvas = (Canvas)child;
        // Java: else if ("CanvasTransform".equalsIgnoreCase(type))
        // Java: this.canvasTransform = (CanvasTransform)child;
        // Java: else if ("Timestamp".equalsIgnoreCase(type))
        // Java: this.timestamp = (Timestamp)child;
        Ok(())
    }

    // Java: void copyFromContext(Context context)
    fn copyFromContext(&mut self, context: &Context) {
        // Java: this.brush = context.getBrush().clone();
        self.brush = context.brush.as_ref().map(|b| b.clone_brush());
        // Java: this.canvas = context.getCanvas();
        self.canvas = context.canvas.as_ref().map(|c| c.clone_canvas());
        // Java: this.canvasTransform = context.getCanvasTransform();
        self.canvasTransform = context.canvasTransform.as_ref().map(|ct| ct.clone_ct());
        // Java: this.inkSource = context.getInkSource();
        self.inkSource = context.inkSource.as_ref().map(|is| is.clone_is());
        // Java: this.traceFormat = context.getTraceFormat();
        self.traceFormat = context.traceFormat.as_ref().map(|tf| tf.clone_tf());
        // Java: this.timestamp = context.getTimestamp();
        self.timestamp = context.timestamp.as_ref().map(|t| t.clone_ts());
    }

    // Java: void copyFromContextReference(Definitions defs, String ctxRef)
    fn copyFromContextReference(&mut self, _defs: &Definitions, _ctxRef: &str) -> Result<(), String> {
        // Java: Context refferedContext = defs.getContextRefElement(ctxRef);
        // Java: this.copyFromContext(refferedContext);
        Ok(())
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: StringBuffer elementStrBuff = new StringBuffer("<context");
        let mut buf = String::from("<context");
        // Java: if (this.attributesMap != null)
        if !self.attributesMap.is_empty() {
            // Java: Map<String, String> sortedAttrs = new TreeMap(this.attributesMap);
            // Java: Set<String> keys = sortedAttrs.keySet();
            // Java: Iterator<String> itr = keys.iterator();
            let mut keys: Vec<&String> = self.attributesMap.keys().collect();
            keys.sort();
            // Java: while(itr.hasNext())
            for key in keys {
                // Java: elementStrBuff.append(' ');
                buf += " ";
                // Java: String key = (String)itr.next();
                // Java: if (key.equals("id"))
                if key == "id" {
                    // Java: elementStrBuff.append("xml:" + key);
                    buf += "xml:id";
                // Java: else
                } else {
                    // Java: elementStrBuff.append(key);
                    buf += key;
                }
                // Java: elementStrBuff.append("=\"");
                buf += "=\"";
                // Java: elementStrBuff.append((String)this.attributesMap.get(key));
                if let Some(v) = self.attributesMap.get(key) {
                    buf += v;
                }
                // Java: elementStrBuff.append('"');
                buf += "\"";
            }
        }
        // Java: int size = this.ctxElementList.keySet().size();
        let size = self.ctxElementList.len();
        // Java: if (size != 0)
        if size != 0 {
            // Java: elementStrBuff.append(">");
            buf += ">";
            // Java: ContextElement[] contextElements = new ContextElement[6];
            // Java: contextElements[0] = (ContextElement)this.ctxElementList.get(Canvas.class.getSimpleName());
            // Java: contextElements[1] = (ContextElement)this.ctxElementList.get(CanvasTransform.class.getSimpleName());
            // Java: contextElements[2] = (ContextElement)this.ctxElementList.get(TraceFormat.class.getSimpleName());
            // Java: contextElements[3] = (ContextElement)this.ctxElementList.get(InkSource.class.getSimpleName());
            // Java: contextElements[4] = (ContextElement)this.ctxElementList.get(IBrush.class.getSimpleName());
            // Java: contextElements[5] = (ContextElement)this.ctxElementList.get(Timestamp.class.getSimpleName());
            // Java: for(ContextElement element : contextElements)
            {
                // Java: if (element != null)
                {
                    // Java: elementStrBuff.append(element.toInkML());
                }
            }
            // Java: elementStrBuff.append("</context>");
            buf += "</context>";
        // Java: else
        } else {
            // Java: elementStrBuff.append(" />");
            buf += " />";
        }
        // Java: return elementStrBuff.toString();
        buf
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
        // Java: int size = this.ctxElementList.keySet().size();
        let size = self.ctxElementList.len();
        // Java: if (size != 0)
        if size != 0 {
            // Java: writer.writeStartTag("context", this.attributesMap);
            writer.writeStartTag("context", Some(&self.attributesMap));
            // Java: writer.incrementTagLevel();
            writer.incrementTagLevel();
            // Java: ContextElement[] contextElements = new ContextElement[]{(ContextElement)this.ctxElementList.get(Canvas.class.getSimpleName()), (ContextElement)this.ctxElementList.get(CanvasTransform.class.getSimpleName()), (ContextElement)this.ctxElementList.get(TraceFormat.class.getSimpleName()), (ContextElement)this.ctxElementList.get(InkSource.class.getSimpleName()), (ContextElement)this.ctxElementList.get(IBrush.class.getSimpleName()), (ContextElement)this.ctxElementList.get(Timestamp.class.getSimpleName())};
            // Java: for(ContextElement element : contextElements)
            {
                // Java: if (element != null)
                {
                    // Java: element.writeXML(writer);
                }
            }
            // Java: writer.decrementTagLevel();
            writer.decrementTagLevel();
            // Java: writer.writeEndTag("context");
            writer.writeEndTag("context");
        // Java: else
        } else {
            // Java: writer.writeEmptyStartTag("context", this.attributesMap);
            writer.writeEmptyStartTag("context", Some(&self.attributesMap));
        }
    }

    // Java: void resolveImplicitReferenceWithDefaultContext()
    pub fn resolveImplicitReferenceWithDefaultContext(&mut self) {
        // Java: Context defaultCtx = getDefaultContext();
        let defaultCtx = Context::getDefaultContext();
        // Java: if (null == this.brush)
        if self.brush.is_none() {
            // Java: this.brush = defaultCtx.getBrush();
            self.brush = defaultCtx.brush;
        }
        // Java: if (null == this.traceFormat)
        if self.traceFormat.is_none() {
            // Java: this.traceFormat = defaultCtx.getTraceFormat();
            self.traceFormat = defaultCtx.traceFormat;
        }
        // Java: if (null == this.inkSource)
        if self.inkSource.is_none() {
            // Java: this.inkSource = defaultCtx.getInkSource();
            self.inkSource = defaultCtx.inkSource;
        }
        // Java: if (null == this.canvas)
        if self.canvas.is_none() {
            // Java: this.canvas = defaultCtx.getCanvas();
            self.canvas = defaultCtx.canvas;
        }
        // Java: if (null == this.canvasTransform)
        if self.canvasTransform.is_none() {
            // Java: this.canvasTransform = defaultCtx.getCanvasTransform();
            self.canvasTransform = defaultCtx.canvasTransform;
        }
        // Java: if (null == this.timestamp)
        if self.timestamp.is_none() {
            // Java: this.timestamp = defaultCtx.getTimestamp();
            self.timestamp = defaultCtx.timestamp;
        }
    }

    // Java: Context clone()
    pub fn clone_context(&self) -> Context {
        // Java: Context ret = new Context();
        let mut ret = Context::Context();
        // Java: if (null != this.inkSource)
        if let Some(ref is) = self.inkSource {
            // Java: ret.inkSource = this.inkSource.clone();
            ret.inkSource = Some(is.clone_is());
        }
        // Java: if (null != this.traceFormat)
        if let Some(ref tf) = self.traceFormat {
            // Java: ret.traceFormat = this.traceFormat.clone();
            ret.traceFormat = Some(tf.clone_tf());
        }
        // Java: if (null != this.brush)
        if let Some(ref b) = self.brush {
            // Java: ret.brush = this.brush.clone();
            ret.brush = Some(b.clone_brush());
        }
        // Java: if (null != this.canvas)
        if let Some(ref c) = self.canvas {
            // Java: ret.canvas = this.canvas.clone();
            ret.canvas = Some(c.clone_canvas());
        }
        // Java: if (null != this.canvasTransform)
        if let Some(ref ct) = self.canvasTransform {
            // Java: ret.canvasTransform = this.canvasTransform.clone();
            ret.canvasTransform = Some(ct.clone_ct());
        }
        // Java: if (null != this.timestamp)
        if let Some(ref ts) = self.timestamp {
            // Java: ret.timestamp = this.timestamp.clone();
            ret.timestamp = Some(ts.clone_ts());
        }
        // Java: ret.attributesMap = this.cloneAttributesMap();
        ret.attributesMap = self.cloneAttributesMap();
        // Java: ret.ctxElementList = this.cloneCtxElementList();
        ret.ctxElementList = self.cloneCtxElementList();
        // Java: return ret;
        ret
    }

    // Java: private HashMap<String, ContextElement> cloneCtxElementList()
    fn cloneCtxElementList(&self) -> HashMap<String, String> {
        // Java: if (null == this.ctxElementList)
        if self.ctxElementList.is_empty() {
            // Java: return null;
            return HashMap::new();
        // Java: else
        } else {
            // Java: HashMap<String, ContextElement> ret = new HashMap();
            let mut ret = HashMap::new();
            // Java: for(String k : this.ctxElementList.keySet())
            for (k, v) in &self.ctxElementList {
                // Java: ContextElement e = (ContextElement)this.ctxElementList.get(k);
                // Java: if (e instanceof Brush)
                // Java: Brush b = (Brush)e;
                // Java: ret.put(new String(k), b.clone());
                // Java: else if (e instanceof Canvas)
                // Java: Canvas c = (Canvas)e;
                // Java: ret.put(new String(k), c.clone());
                // Java: else if (e instanceof CanvasTransform)
                // Java: CanvasTransform ct = (CanvasTransform)e;
                // Java: ret.put(new String(k), ct.clone());
                // Java: else if (e instanceof InkSource)
                // Java: InkSource is = (InkSource)e;
                // Java: ret.put(new String(k), is.clone());
                // Java: else if (e instanceof Timestamp)
                // Java: Timestamp ts = (Timestamp)e;
                // Java: ret.put(new String(k), ts.clone());
                // Java: else if (e instanceof TraceFormat)
                // Java: TraceFormat tf = (TraceFormat)e;
                // Java: ret.put(new String(k), tf.clone());
                ret.insert(k.clone(), v.clone());
            }
            // Java: return ret;
            ret
        }
    }

    // Java: private HashMap<String, String> cloneAttributesMap()
    fn cloneAttributesMap(&self) -> HashMap<String, String> {
        // Java: if (null == this.attributesMap)
        if self.attributesMap.is_empty() {
            // Java: return null;
            return HashMap::new();
        // Java: else
        } else {
            // Java: HashMap<String, String> ret = new HashMap();
            let mut ret: HashMap<String, String> = HashMap::new();
            // Java: for(String k : this.attributesMap.keySet())
            for (k, v) in &self.attributesMap {
                // Java: String e = (String)this.attributesMap.get(k);
                // Java: ret.put(new String(k), new String(e));
                ret.insert(k.clone(), v.clone());
            }
            // Java: return ret;
            ret
        }
    }
}
