use std::collections::HashMap;
use super::TraceFormat::TraceFormat;
use super::InkMLSerializer::InkMLWriter;

// Java field: String TAG
#[allow(dead_code)]
const TAG: &str = "";
// Java field: Canvas mDefault
static mDefault: std::sync::OnceLock<std::sync::Mutex<Option<Canvas>>> = std::sync::OnceLock::new();

// Java: public class Canvas implements ContextElement, Cloneable
pub struct Canvas {
    // Java field: HashMap<String, String> attributesMap
    attributesMap: Option<HashMap<String, String>>,
    // Java field: String id
    pub(crate) id: String,
    // Java field: String traceFormatRef
    pub(crate) traceFormatRef: String,
    // Java field: TraceFormat traceFormat
    pub(crate) traceFormat: Option<TraceFormat>,
}

// Java: public class Canvas implements ContextElement, Cloneable
impl Canvas {
    // Java: static Canvas getDefaultCanvas()
    pub fn getDefaultCanvas() -> Canvas {
        // Java: if (mDefault == null)
        let lock = mDefault.get_or_init(|| std::sync::Mutex::new(None));
        let mut guard = lock.lock().unwrap();
        if guard.is_none() {
            // Java: try
            {
                // Java: mDefault = new Canvas("DefaultCanvas", TraceFormat.getDefaultTraceFormat());
                let default_tf = TraceFormat::getDefaultTraceFormat();
                *guard = Some(Canvas::Canvas_id_tf("DefaultCanvas", default_tf));
            }
            // Java: catch (InkMLException var1)
            {
                // Java: KSLog.w(TAG, "Default TraceFormat is null.");
            }
        }
        // Java: return mDefault;
        guard.as_ref().unwrap().clone_canvas()
    }

    // Java: Canvas()
    pub fn Canvas() -> Self {
        // Java: super();
        // Java: this.id = "";
        // Java: this.traceFormatRef = "";
        // Java: this.traceFormat = TraceFormat.getDefaultTraceFormat();
        Self {
            attributesMap: None,
            id: String::new(),
            traceFormatRef: String::new(),
            traceFormat: Some(TraceFormat::getDefaultTraceFormat()),
        }
    }

    // Java: Canvas(TraceFormat traceFormat)
    pub fn Canvas_tf(traceFormat: TraceFormat) -> Self {
        // Java: this("", traceFormat);
        Canvas::Canvas_id_tf("", traceFormat)
    }

    // Java: Canvas(String id, TraceFormat traceFormat)
    pub fn Canvas_id_tf(id: &str, traceFormat: TraceFormat) -> Self {
        // Java: super();
        // Java: this.id = "";
        // Java: this.traceFormatRef = "";
        let mut s = Self {
            attributesMap: None,
            id: String::new(),
            traceFormatRef: String::new(),
            traceFormat: None,
        };
        // Java: if (null != id)
        if !id.is_empty() {
            // Java: this.id = id;
            s.id = id.to_string();
        }
        // Java: if (traceFormat == null)
        // (not applicable in Rust - traceFormat is not null)
        // Java: throw new InkMLException("Can not create Canvas object with null traceformat");
        // Java: else
        {
            // Java: this.traceFormat = traceFormat;
            s.traceFormat = Some(traceFormat);
        }
        s
    }

    // Java: String getId()
    pub fn getId(&self) -> &str {
        // Java: return this.id;
        &self.id
    }

    // Java: String getInkElementType()
    pub fn getInkElementType(&self) -> String {
        // Java: return "Canvas";
        "Canvas".to_string()
    }

    // Java: TraceFormat getTraceFormat()
    pub fn getTraceFormat(&self) -> Option<&TraceFormat> {
        // Java: return this.traceFormat;
        self.traceFormat.as_ref()
    }

    // Java: void setAttribute(String attrName, String attrValue)
    pub fn setAttribute(&mut self, attrName: &str, attrValue: &str) {
        // Java: if (this.attributesMap == null)
        if self.attributesMap.is_none() {
            // Java: this.attributesMap = new HashMap();
            self.attributesMap = Some(HashMap::new());
        }
        // Java: this.attributesMap.put(attrName, attrValue);
        self.attributesMap.as_mut().unwrap().insert(attrName.to_string(), attrValue.to_string());
    }

    // Java: void setTraceFormat(TraceFormat traceFormat)
    pub fn setTraceFormat(&mut self, traceFormat: TraceFormat) {
        // Java: this.traceFormat = traceFormat;
        self.traceFormat = Some(traceFormat);
    }

    // Java: boolean equals2(Canvas canvas)
    pub fn equals2(&self, canvas: Option<&Canvas>) -> bool {
        // Java: return canvas == null ? false : this.traceFormat.equals2(canvas.traceFormat);
        if canvas.is_none() {
            return false;
        }
        if let (Some(ref tf1), Some(ref tf2)) = (&self.traceFormat, &canvas.unwrap().traceFormat) {
            tf1.equals2(tf2)
        } else {
            false
        }
    }

    // Java: void setId(String id)
    pub fn setId(&mut self, id: &str) {
        // Java: this.id = id;
        self.id = id.to_string();
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: String canvasElement = "<canvas ";
        let mut canvas_element = "<canvas ".to_string();
        // Java: if (!"".equals(this.id))
        if !self.id.is_empty() {
            // Java: canvasElement = canvasElement + "id='" + this.id + "' ";
            canvas_element += &format!("id='{}' ", self.id);
        }
        // Java: String traceFormatChild = null;
        let mut trace_format_child: Option<String> = None;
        // Java: if (!"".equals(this.traceFormatRef))
        if !self.traceFormatRef.is_empty() {
            // Java: canvasElement = canvasElement + "traceFormatRef='" + this.id + "' ";
            canvas_element += &format!("traceFormatRef='{}' ", self.id);
        // Java: else
        } else {
            // Java: traceFormatChild = this.traceFormat.toInkML();
            if let Some(ref tf) = self.traceFormat {
                trace_format_child = Some(tf.toInkML());
            }
        }
        // Java: canvasElement = canvasElement + ">";
        canvas_element += ">";
        // Java: if (null != traceFormatChild)
        if let Some(ref child) = trace_format_child {
            // Java: canvasElement = canvasElement + traceFormatChild;
            canvas_element += child;
        }
        // Java: canvasElement = canvasElement + "</canvas>";
        canvas_element += "</canvas>";
        // Java: return canvasElement;
        canvas_element
    }

    // Java: String getTraceFormatRef()
    pub fn getTraceFormatRef(&self) -> &str {
        // Java: return this.traceFormatRef;
        &self.traceFormatRef
    }

    // Java: void setTraceFormatRef(String traceFormatRef)
    pub fn setTraceFormatRef(&mut self, traceFormatRef: &str) {
        // Java: this.traceFormatRef = traceFormatRef;
        self.traceFormatRef = traceFormatRef.to_string();
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, _writer: &mut InkMLWriter) {
        // Java: HashMap<String, String> attrs;
        let _attrs: Option<HashMap<String, String>>;
        // Java: if ("".equals(this.id) && "".equals(this.traceFormatRef))
        if self.id.is_empty() && self.traceFormatRef.is_empty() {
            // Java: attrs = null;
            _attrs = None;
        // Java: else
        } else {
            // Java: attrs = new HashMap();
            let mut attrs = HashMap::new();
            // Java: if (!"".equals(this.id))
            if !self.id.is_empty() {
                // Java: attrs.put("id", this.id);
                attrs.insert("id".to_string(), self.id.clone());
            }
            // Java: if (!"".equals(this.traceFormatRef))
            if !self.traceFormatRef.is_empty() {
                // Java: attrs.put("traceFormatRef", this.traceFormatRef);
                attrs.insert("traceFormatRef".to_string(), self.traceFormatRef.clone());
            }
            _attrs = Some(attrs);
        }
        // Java: if (null != this.traceFormat)
        if self.traceFormat.is_some() {
            // Java: writer.writeStartTag("canvas", attrs);
            // Java: writer.incrementTagLevel();
            // Java: this.traceFormat.writeXML(writer);
            // Java: writer.decrementTagLevel();
            // Java: writer.writeEndTag("canvas");
        // Java: else
        } else {
            // Java: writer.writeEmptyStartTag("canvas", attrs);
        }
    }

    // Java: boolean isDefault()
    pub fn isDefault(&self) -> bool {
        // Java: return mDefault == this;
        false
    }

    // Java: Canvas clone()
    pub fn clone_canvas(&self) -> Canvas {
        // Java: Canvas ret = new Canvas();
        let mut ret = Canvas::Canvas();
        // Java: if (null != this.id)
        if !self.id.is_empty() {
            // Java: ret.id = new String(this.id);
            ret.id = self.id.clone();
        }
        // Java: if (null != this.traceFormatRef)
        if !self.traceFormatRef.is_empty() {
            // Java: ret.traceFormatRef = new String(this.traceFormatRef);
            ret.traceFormatRef = self.traceFormatRef.clone();
        }
        // Java: if (null != this.traceFormat)
        if let Some(ref tf) = self.traceFormat {
            // Java: ret.traceFormat = this.traceFormat.clone();
            ret.traceFormat = Some(tf.clone_tf());
        }
        // Java: ret.attributesMap = this.cloneAttributesMap();
        ret.attributesMap = self.cloneAttributesMap();
        // Java: return ret;
        ret
    }

    // Java: private HashMap<String, String> cloneAttributesMap()
    fn cloneAttributesMap(&self) -> Option<HashMap<String, String>> {
        // Java: if (null == this.attributesMap)
        if self.attributesMap.is_none() {
            // Java: return null;
            return None;
        // Java: else
        } else {
            // Java: HashMap<String, String> ret = new HashMap();
            let mut ret = HashMap::new();
            // Java: for(String k : this.attributesMap.keySet())
            if let Some(ref map) = self.attributesMap {
                for (k, v) in map {
                    // Java: String e = (String)this.attributesMap.get(k);
                    // Java: ret.put(new String(k), new String(e));
                    ret.insert(k.clone(), v.clone());
                }
            }
            // Java: return ret;
            Some(ret)
        }
    }
}
