#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.util.KSLog
use util::cn::wps::moffice::util::KSLog::KSLog;
// Java: import java.util.Collection
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Map
// Java: import java.util.Set
// Java: import java.util.StringTokenizer

use super::InkElement::InkElement;
use super::InkMLSerializer::InkMLWriter;

// Java field: String TAG
const TAG: &str = "Definitions";

// Java: public class Definitions implements Cloneable
pub struct Definitions {
    // Java field: HashMap<String, InkElement> directChildrenMap
    pub(crate) directChildrenMap: HashMap<String, Box<dyn InkElement>>,
    // Java field: HashMap<String, InkElement> indirectChildrenMap
    indirectChildrenMap: HashMap<String, Box<dyn InkElement>>,
}

// Java: public class Definitions implements Cloneable
impl Definitions {
    // Java: Definitions()
    pub fn Definitions() -> Self {
        // Java: super();
        // Java: InkElement[] defaultContextObjects = new InkElement[7];
        // Java: defaultContextObjects[0] = Canvas.getDefaultCanvas();
        // Java: defaultContextObjects[1] = CanvasTransform.getDefaultCanvasTransform();
        // Java: defaultContextObjects[2] = TraceFormat.getDefaultTraceFormat();
        // Java: defaultContextObjects[3] = InkSource.getDefaultInkSource();
        // Java: defaultContextObjects[4] = Brush.getDefaultBrush();
        // Java: defaultContextObjects[5] = Timestamp.getDefaultTimestamp();
        // Java: defaultContextObjects[6] = Context.getDefaultContext();
        // Java: this.addToIndirectChildrenMap(defaultContextObjects);
        Self {
            directChildrenMap: HashMap::new(),
            indirectChildrenMap: HashMap::new(),
        }
    }

    // Java: String addToDirectChildrenMap(InkElement inkElement)
    pub fn addToDirectChildrenMap_single(&mut self, inkElement: Box<dyn InkElement>) -> String {
        // Java: String id = "";
        let mut id = String::new();
        // Java: try
        {
            // Java: id = inkElement.getId();
            id = inkElement.getId();
            // Java: if (!"".equals(id))
            if !id.is_empty() {
                // Java: if (this.directChildrenMap.containsKey(id))
                if self.directChildrenMap.contains_key(&id) {
                    // Java: KSLog.w(TAG, "The Ink Element already exist in the definitions.Ignoring to save it again in definitions.\nElement ID: " + id);
                    KSLog::w(TAG, &format!("The Ink Element already exist in the definitions.Ignoring to save it again in definitions.\nElement ID: {}", id));
                    // Java: return id;
                    return id;
                }
                // Java: this.directChildrenMap.put(id, inkElement);
                self.directChildrenMap.insert(id.clone(), inkElement);
            // Java: else
            } else {
                // Java: KSLog.w(TAG, "The Ink Element does not have value for id; It is not added to definitions.\nElement: " + inkElement);
                KSLog::w(TAG, "The Ink Element does not have value for id; It is not added to definitions.");
            }
        }
        // Java: catch (NullPointerException var4)
        {
            // Java: KSLog.w(TAG, "addToDirectChildrenMap: the InkElement in the argument is null.");
        }
        // Java: return id;
        id
    }

    // Java: String addToIndirectChildrenMap(InkElement inkElement)
    pub fn addToIndirectChildrenMap_single(&mut self, inkElement: Box<dyn InkElement>) -> String {
        // Java: String id = inkElement.getId();
        let id = inkElement.getId();
        // Java: if (!"".equals(id))
        if !id.is_empty() {
            // Java: if (this.indirectChildrenMap.containsKey(id))
            if self.indirectChildrenMap.contains_key(&id) {
                // Java: return id;
                return id;
            }
            // Java: this.indirectChildrenMap.put(id, inkElement);
            self.indirectChildrenMap.insert(id.clone(), inkElement);
        }
        // Java: return id;
        id
    }

    // Java: void addToDirectChildrenMap(InkElement[] inkElements)
    pub fn addToDirectChildrenMap(&mut self, inkElements: Vec<Box<dyn InkElement>>) {
        // Java: for(int index = 0; index < inkElements.length; ++index)
        for element in inkElements {
            // Java: String id = inkElements[index].getId();
            let id = element.getId();
            // Java: if (!"".equals(id))
            if !id.is_empty() {
                // Java: if (this.directChildrenMap.containsKey(id))
                if self.directChildrenMap.contains_key(&id) {
                    // Java: KSLog.w(TAG, "The Ink Element already exist in the definitions.Ignoring to save it again in definitions.\nElement ID: " + id);
                    KSLog::w(TAG, &format!("The Ink Element already exist in the definitions.Ignoring to save it again in definitions.\nElement ID: {}", id));
                    // Java: return;
                    return;
                }
                // Java: this.directChildrenMap.put(id, inkElements[index]);
                self.directChildrenMap.insert(id, element);
            // Java: else
            } else {
                // Java: KSLog.w(TAG, "The Ink Element does not have value for id; It will be ignored.\n" + inkElements[index]);
                KSLog::w(TAG, "The Ink Element does not have value for id; It will be ignored.");
            }
        }
    }

    // Java: void addToIndirectChildrenMap(InkElement[] inkElements)
    pub fn addToIndirectChildrenMap(&mut self, inkElements: Vec<Box<dyn InkElement>>) {
        // Java: for(int index = 0; index < inkElements.length; ++index)
        for element in inkElements {
            // Java: String id = inkElements[index].getId();
            let id = element.getId();
            // Java: if (!"".equals(id))
            if !id.is_empty() {
                // Java: if (this.indirectChildrenMap.containsKey(id))
                if self.indirectChildrenMap.contains_key(&id) {
                    // Java: KSLog.w(TAG, "The Ink Element already exist in the definitions.Ignoring to save it again in definitions.\nElement ID: " + id);
                    KSLog::w(TAG, &format!("The Ink Element already exist in the definitions.Ignoring to save it again in definitions.\nElement ID: {}", id));
                    // Java: return;
                    return;
                }
                // Java: this.indirectChildrenMap.put(id, inkElements[index]);
                self.indirectChildrenMap.insert(id, element);
            // Java: else
            } else {
                // Java: KSLog.w(TAG, "The Ink Element does not have value for id; It will be ignored.\n" + inkElements[index]);
                KSLog::w(TAG, "The Ink Element does not have value for id; It will be ignored.");
            }
        }
    }

    // Java: InkElement getReferredInkElement(String refAttribute)
    fn getReferredInkElement(&self, refAttribute: &str) -> Result<&dyn InkElement, String> {
        // Java: InkElement inkElement = null;
        // Java: StringTokenizer st = new StringTokenizer(refAttribute, "#");
        let tokens: Vec<&str> = refAttribute.split('#').collect();
        // Java: if (0 != st.countTokens() && 3 > st.countTokens())
        if !tokens.is_empty() && tokens.len() < 3 {
            // Java: if (2 == st.countTokens())
            if tokens.len() == 2 {
                // Java: throw new InkMLException("Feature not supported. Yet to implement reference outside the current Ink documents.");
                return Err("Feature not supported. Yet to implement reference outside the current Ink documents.".to_string());
            // Java: else
            } else {
                // Java: String key = st.nextToken();
                let key = tokens[0];
                // Java: inkElement = (InkElement)this.directChildrenMap.get(key);
                if let Some(el) = self.directChildrenMap.get(key) {
                    return Ok(el.as_ref());
                }
                // Java: if (null == inkElement)
                {
                    // Java: inkElement = (InkElement)this.indirectChildrenMap.get(key);
                    if let Some(el) = self.indirectChildrenMap.get(key) {
                        return Ok(el.as_ref());
                    }
                }
                // Java: if (null == inkElement)
                {
                    // Java: throw new InkMLException("\nError: There is no element exist with the given id, " + key);
                    // Java: else
                    // Java: return inkElement;
                    return Err(format!("\nError: There is no element exist with the given id, {}", key));
                }
            }
        // Java: else
        } else {
            // Java: throw new InkMLException("\nError: Invalid attribute value given, " + refAttribute + "\nUsage: [url]#elementId\n");
            return Err(format!("\nError: Invalid attribute value given, {}\nUsage: [url]#elementId\n", refAttribute));
        }
    }

    // Java: Context getContextRefElement(String contextRef)
    pub fn getContextRefElement(&self, contextRef: &str) -> Result<&dyn InkElement, String> {
        // Java: InkElement inkElement = this.getReferredInkElement(contextRef);
        let inkElement = self.getReferredInkElement(contextRef)?;
        // Java: if (!"Context".equals(inkElement.getInkElementType()))
        if inkElement.getInkElementType() != "Context" {
            // Java: throw new InkMLException("The given Reference attribute value, " + contextRef + "is not the 'id' of a Context Element");
            return Err(format!("The given Reference attribute value, {}is not the 'id' of a Context Element", contextRef));
        // Java: else
        } else {
            // Java: return new Context((Context)inkElement);
            Ok(inkElement)
        }
    }

    // Java: IBrush getBrushRefElement(String brushRef)
    pub fn getBrushRefElement(&self, brushRef: &str) -> Result<&dyn InkElement, String> {
        // Java: IBrush brush = null;
        // Java: InkElement inkElement = this.getReferredInkElement(brushRef);
        let inkElement = self.getReferredInkElement(brushRef)?;
        // Java: if (!"Brush".equals(inkElement.getInkElementType()))
        if inkElement.getInkElementType() != "Brush" {
            // Java: throw new InkMLException("The given Reference attribute value, " + brushRef + "is not the 'id' of a Brush Element");
            return Err(format!("The given Reference attribute value, {}is not the 'id' of a Brush Element", brushRef));
        // Java: else
        } else {
            // Java: brush = (IBrush)inkElement;
            // Java: return brush;
            Ok(inkElement)
        }
    }

    // Java: TraceFormat getTraceFormatRefElement(String traceFormatRef)
    pub fn getTraceFormatRefElement(&self, traceFormatRef: &str) -> Result<&dyn InkElement, String> {
        // Java: TraceFormat traceFormat = null;
        // Java: InkElement inkElement = this.getReferredInkElement(traceFormatRef);
        let inkElement = self.getReferredInkElement(traceFormatRef)?;
        // Java: if (!"TraceFormat".equals(inkElement.getInkElementType()))
        if inkElement.getInkElementType() != "TraceFormat" {
            // Java: throw new InkMLException("The given Reference attribute value, " + traceFormatRef + "is not the 'id' of a TraceFormat Element");
            return Err(format!("The given Reference attribute value, {}is not the 'id' of a TraceFormat Element", traceFormatRef));
        // Java: else
        } else {
            // Java: traceFormat = (TraceFormat)inkElement;
            // Java: return traceFormat;
            Ok(inkElement)
        }
    }

    // Java: TraceDataElement getTraceDataRefElement(String traceDataRef)
    pub fn getTraceDataRefElement(&self, traceDataRef: &str) -> Result<&dyn InkElement, String> {
        // Java: TraceDataElement traceDataElement = null;
        // Java: InkElement inkElement = this.getReferredInkElement(traceDataRef);
        let inkElement = self.getReferredInkElement(traceDataRef)?;
        // Java: if ("Trace".equals(inkElement.getInkElementType()))
        if inkElement.getInkElementType() == "Trace" {
            // Java: traceDataElement = (Trace)inkElement;
            Ok(inkElement)
        // Java: else if ("TraceGroup".equals(inkElement.getInkElementType()))
        } else if inkElement.getInkElementType() == "TraceGroup" {
            // Java: traceDataElement = (TraceGroup)inkElement;
            Ok(inkElement)
        // Java: else
        } else {
            // Java: if (!"TraceView".equals(inkElement.getInkElementType()))
            if inkElement.getInkElementType() != "TraceView" {
                // Java: throw new InkMLException("The given Reference attribute value, " + traceDataRef + "is not the 'id' of a Trace Data Element");
                return Err(format!("The given Reference attribute value, {}is not the 'id' of a Trace Data Element", traceDataRef));
            }
            // Java: traceDataElement = (TraceView)inkElement;
            Ok(inkElement)
        }
        // Java: return traceDataElement;
    }

    // Java: InkSource getInkSourceRefElement(String inkSourceRef)
    pub fn getInkSourceRefElement(&self, inkSourceRef: &str) -> Result<&dyn InkElement, String> {
        // Java: InkSource inkSource = null;
        // Java: InkElement inkElement = this.getReferredInkElement(inkSourceRef);
        let inkElement = self.getReferredInkElement(inkSourceRef)?;
        // Java: if (!"InkSource".equals(inkElement.getInkElementType()))
        if inkElement.getInkElementType() != "InkSource" {
            // Java: throw new InkMLException("The given Reference attribute value, " + inkSourceRef + "is not the 'id' of a InkSource Element");
            return Err(format!("The given Reference attribute value, {}is not the 'id' of a InkSource Element", inkSourceRef));
        // Java: else
        } else {
            // Java: inkSource = (InkSource)inkElement;
            // Java: return inkSource;
            Ok(inkElement)
        }
    }

    // Java: Canvas getCanvasRefElement(String canvasRef)
    pub fn getCanvasRefElement(&self, canvasRef: &str) -> Result<&dyn InkElement, String> {
        // Java: Canvas canvas = null;
        // Java: InkElement inkElement = this.getReferredInkElement(canvasRef);
        let inkElement = self.getReferredInkElement(canvasRef)?;
        // Java: if (!"Canvas".equals(inkElement.getInkElementType()))
        if inkElement.getInkElementType() != "Canvas" {
            // Java: throw new InkMLException("The given Reference attribute value, " + canvasRef + "is not the 'id' of a Canvas Element");
            return Err(format!("The given Reference attribute value, {}is not the 'id' of a Canvas Element", canvasRef));
        // Java: else
        } else {
            // Java: canvas = (Canvas)inkElement;
            // Java: return canvas;
            Ok(inkElement)
        }
    }

    // Java: CanvasTransform getCanvasTransformRefElement(String canvasTransformRef)
    pub fn getCanvasTransformRefElement(&self, canvasTransformRef: &str) -> Result<&dyn InkElement, String> {
        // Java: CanvasTransform canvasTransform = null;
        // Java: InkElement inkElement = this.getReferredInkElement(canvasTransformRef);
        let inkElement = self.getReferredInkElement(canvasTransformRef)?;
        // Java: if (!"CanvasTransform".equals(inkElement.getInkElementType()))
        if inkElement.getInkElementType() != "CanvasTransform" {
            // Java: throw new InkMLException("The given Reference attribute value, " + canvasTransformRef + "is not the 'id' of a CanvasTransform Element");
            return Err(format!("The given Reference attribute value, {}is not the 'id' of a CanvasTransform Element", canvasTransformRef));
        // Java: else
        } else {
            // Java: canvasTransform = (CanvasTransform)inkElement;
            // Java: return canvasTransform;
            Ok(inkElement)
        }
    }

    // Java: Timestamp getTimestampRefElement(String timestampRef)
    pub fn getTimestampRefElement(&self, timestampRef: &str) -> Result<&dyn InkElement, String> {
        // Java: Timestamp timestamp = null;
        // Java: InkElement inkElement = this.getReferredInkElement(timestampRef);
        let inkElement = self.getReferredInkElement(timestampRef)?;
        // Java: if (!"Timestamp".equals(inkElement.getInkElementType()))
        if inkElement.getInkElementType() != "Timestamp" {
            // Java: throw new InkMLException("The given Reference attribute value, " + timestampRef + "is not the 'id' of a Timestamp Element");
            return Err(format!("The given Reference attribute value, {}is not the 'id' of a Timestamp Element", timestampRef));
        // Java: else
        } else {
            // Java: timestamp = (Timestamp)inkElement;
            // Java: return timestamp;
            Ok(inkElement)
        }
    }

    // Java: Mapping getMappingRefElement(String mappingRef)
    pub fn getMappingRefElement(&self, mappingRef: &str) -> Result<&dyn InkElement, String> {
        // Java: Mapping mapping = null;
        // Java: InkElement inkElement = this.getReferredInkElement(mappingRef);
        let inkElement = self.getReferredInkElement(mappingRef)?;
        // Java: if (!"Mapping".equals(inkElement.getInkElementType()))
        if inkElement.getInkElementType() != "Mapping" {
            // Java: throw new InkMLException("The given Reference attribute value, " + mappingRef + "is not the 'id' of a Mapping Element");
            return Err(format!("The given Reference attribute value, {}is not the 'id' of a Mapping Element", mappingRef));
        // Java: else
        } else {
            // Java: mapping = (Mapping)inkElement;
            // Java: return mapping;
            Ok(inkElement)
        }
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: if (this.directChildrenMap != null && this.directChildrenMap.size() != 0)
        if !self.directChildrenMap.is_empty() {
            // Java: String definitionsElement = "<definitions>";
            let mut definitionsElement = String::from("<definitions>");
            // Java: for(Map.Entry<String, InkElement> child : this.directChildrenMap.entrySet())
            for (_key, child) in &self.directChildrenMap {
                // Java: InkElement inkObject = (InkElement)child.getValue();
                // Java: definitionsElement = definitionsElement + "\n" + inkObject.toInkML();
                definitionsElement = definitionsElement + "\n" + &child.toInkML();
            }
            // Java: definitionsElement = definitionsElement + "\n</definitions>";
            definitionsElement = definitionsElement + "\n</definitions>";
            // Java: return definitionsElement;
            definitionsElement
        // Java: else
        } else {
            // Java: return "";
            String::new()
        }
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: if (this.directChildrenMap != null && this.directChildrenMap.size() > 0)
        if !self.directChildrenMap.is_empty() {
            // Java: return false;
            return false;
        // Java: else
        } else {
            // Java: return this.indirectChildrenMap == null || this.indirectChildrenMap.size() <= 0;
            self.indirectChildrenMap.is_empty()
        }
    }

    // Java: Collection<InkElement> getChildrenList()
    pub fn getChildrenList(&self) -> Vec<&dyn InkElement> {
        // Java: return this.directChildrenMap.values();
        self.directChildrenMap.values().map(|v| v.as_ref()).collect()
    }

    // Java: AnnotationXML getAnnotationXMLRefElement(String href)
    pub fn getAnnotationXMLRefElement(&self, href: &str) -> Result<&dyn InkElement, String> {
        // Java: AnnotationXML annotationXML = null;
        // Java: InkElement inkElement = this.getReferredInkElement(href);
        let inkElement = self.getReferredInkElement(href)?;
        // Java: if (!"AnnotationXML".equals(inkElement.getInkElementType()))
        if inkElement.getInkElementType() != "AnnotationXML" {
            // Java: throw new InkMLException("The given Reference attribute value, " + href + "is not the 'id' of a AnnotationXML Element");
            return Err(format!("The given Reference attribute value, {}is not the 'id' of a AnnotationXML Element", href));
        // Java: else
        } else {
            // Java: annotationXML = (AnnotationXML)inkElement;
            // Java: return annotationXML;
            Ok(inkElement)
        }
    }

    // Java: boolean contains(String id)
    pub fn contains(&self, id: &str) -> bool {
        // Java: Set directChildrenKeys = this.directChildrenMap.keySet();
        // Java: if (!directChildrenKeys.contains(id))
        if !self.directChildrenMap.contains_key(id) {
            // Java: Set indirectChildrenKeys = this.indirectChildrenMap.keySet();
            // Java: if (!indirectChildrenKeys.contains(id))
            if !self.indirectChildrenMap.contains_key(id) {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        true
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
        // Java: int nChildren = this.directChildrenMap.size();
        let nChildren = self.directChildrenMap.len();
        // Java: if (0 != nChildren)
        if 0 != nChildren {
            // Java: writer.writeStartTag("definitions", (HashMap)null);
            writer.writeStartTag("definitions", None);
            // Java: writer.incrementTagLevel();
            writer.incrementTagLevel();
            // Java: for(InkElement child : this.directChildrenMap.values())
            for child in self.directChildrenMap.values() {
                // Java: child.writeXML(writer);
                child.writeXML(writer);
            }
            // Java: writer.decrementTagLevel();
            writer.decrementTagLevel();
            // Java: writer.writeEndTag("definitions");
            writer.writeEndTag("definitions");
        }
    }

    // Java: void setBrushProperty(String id, String key, String value)
    pub fn setBrushProperty3(&mut self, id: &str, key: &str, value: &str) {
        // Java: if (this.directChildrenMap.containsKey(id))
        if self.directChildrenMap.contains_key(id) {
            // Java: InkElement element = (InkElement)this.directChildrenMap.get(id);
            // Java: if (element instanceof Brush)
            {
                // Java: Brush brush = (Brush)element;
                // Java: brush.setBrushProperty(key, value);
            }
        }
    }

    // Java: void setAllBrushColorProperty(String color)
    pub fn setAllBrushColorProperty(&mut self, color: &str) {
        // Java: for(Map.Entry<String, InkElement> entry : this.directChildrenMap.entrySet())
        for (_key, _entry) in &self.directChildrenMap {
            // Java: if (entry.getValue() instanceof Brush)
            {
                // Java: Brush brush = (Brush)entry.getValue();
                // Java: brush.setBrushProperty("color", color);
            }
        }
    }

    // Java: void setBrushProperty(String id, String key, String value, String unit)
    pub fn setBrushProperty4(&mut self, id: &str, key: &str, value: &str, unit: &str) {
        // Java: if (this.directChildrenMap.containsKey(id))
        if self.directChildrenMap.contains_key(id) {
            // Java: InkElement element = (InkElement)this.directChildrenMap.get(id);
            // Java: if (element instanceof Brush)
            {
                // Java: Brush brush = (Brush)element;
                // Java: brush.setBrushProperty(key, value, unit);
            }
        }
    }

    // Java: Definitions clone()
    pub fn clone_defs(&self) -> Definitions {
        // Java: Definitions ret = new Definitions();
        let mut ret = Definitions::Definitions();
        // Java: ret.directChildrenMap = this.cloneDirectChildrenMap();
        ret.directChildrenMap = self.cloneDirectChildrenMap();
        // Java: ret.indirectChildrenMap = this.cloneIndirectChildrenMap();
        ret.indirectChildrenMap = self.cloneIndirectChildrenMap();
        // Java: return ret;
        ret
    }

    // Java: private HashMap<String, InkElement> cloneDirectChildrenMap()
    fn cloneDirectChildrenMap(&self) -> HashMap<String, Box<dyn InkElement>> {
        // Java: if (null == this.directChildrenMap)
        if self.directChildrenMap.is_empty() {
            // Java: return null;
            return HashMap::new();
        // Java: else
        } else {
            // Java: HashMap<String, InkElement> ret = new HashMap();
            let ret: HashMap<String, Box<dyn InkElement>> = HashMap::new();
            // Java: for(String k : this.directChildrenMap.keySet())
            for (_k, _e) in &self.directChildrenMap {
                // Java: InkElement e = (InkElement)this.directChildrenMap.get(k);
                // Java: if (e instanceof BrushProperty)
                {
                    // Java: BrushProperty bp = (BrushProperty)e;
                    // Java: ret.put(new String(k), bp);
                }
                // Java: else if (e instanceof Channel)
                {
                    // Java: Channel c = (Channel)e;
                    // Java: ret.put(new String(k), c);
                }
                // Java: else if (e instanceof Context)
                {
                    // Java: Context ctx = (Context)e;
                    // Java: ret.put(new String(k), ctx.clone());
                }
                // Java: else if (e instanceof Brush)
                {
                    // Java: Brush b = (Brush)e;
                    // Java: ret.put(new String(k), b.clone());
                }
                // Java: else if (e instanceof Canvas)
                {
                    // Java: Canvas c = (Canvas)e;
                    // Java: ret.put(new String(k), c.clone());
                }
                // Java: else if (e instanceof CanvasTransform)
                {
                    // Java: CanvasTransform ct = (CanvasTransform)e;
                    // Java: ret.put(new String(k), ct.clone());
                }
                // Java: else if (e instanceof InkSource)
                {
                    // Java: InkSource is = (InkSource)e;
                    // Java: ret.put(new String(k), is.clone());
                }
                // Java: else if (e instanceof Timestamp)
                {
                    // Java: Timestamp ts = (Timestamp)e;
                    // Java: ret.put(new String(k), ts.clone());
                }
                // Java: else if (e instanceof TraceFormat)
                {
                    // Java: TraceFormat tf = (TraceFormat)e;
                    // Java: ret.put(new String(k), tf.clone());
                }
                // Java: else if (e instanceof Mapping)
                {
                    // Java: Mapping m = (Mapping)e;
                    // Java: ret.put(new String(k), m.clone());
                }
                // Java: else if (e instanceof TraceGroup)
                {
                    // Java: TraceGroup traceGroup = (TraceGroup)e;
                    // Java: ret.put(new String(k), traceGroup.clone());
                }
                // Java: else if (e instanceof Trace)
                {
                    // Java: Trace trace = (Trace)e;
                    // Java: ret.put(new String(k), trace.clone());
                }
                // Java: else if (e instanceof TraceView)
                {
                    // Java: TraceView traceView = (TraceView)e;
                    // Java: ret.put(new String(k), traceView.clone());
                }
            }
            // Java: return ret;
            ret
        }
    }

    // Java: private HashMap<String, InkElement> cloneIndirectChildrenMap()
    fn cloneIndirectChildrenMap(&self) -> HashMap<String, Box<dyn InkElement>> {
        // Java: if (null == this.indirectChildrenMap)
        if self.indirectChildrenMap.is_empty() {
            // Java: return null;
            return HashMap::new();
        // Java: else
        } else {
            // Java: HashMap<String, InkElement> ret = new HashMap();
            let ret: HashMap<String, Box<dyn InkElement>> = HashMap::new();
            // Java: for(String k : this.indirectChildrenMap.keySet())
            for (_k, _e) in &self.indirectChildrenMap {
                // Java: InkElement e = (InkElement)this.indirectChildrenMap.get(k);
                // Java: if (e instanceof BrushProperty)
                {
                    // Java: BrushProperty bp = (BrushProperty)e;
                    // Java: ret.put(new String(k), bp);
                }
                // Java: else if (e instanceof Channel)
                {
                    // Java: Channel c = (Channel)e;
                    // Java: ret.put(new String(k), c);
                }
                // Java: else if (e instanceof Context)
                {
                    // Java: Context ctx = (Context)e;
                    // Java: ret.put(new String(k), ctx.clone());
                }
                // Java: else if (e instanceof Brush)
                {
                    // Java: Brush b = (Brush)e;
                    // Java: ret.put(new String(k), b.clone());
                }
                // Java: else if (e instanceof Canvas)
                {
                    // Java: Canvas c = (Canvas)e;
                    // Java: ret.put(new String(k), c.clone());
                }
                // Java: else if (e instanceof CanvasTransform)
                {
                    // Java: CanvasTransform ct = (CanvasTransform)e;
                    // Java: ret.put(new String(k), ct.clone());
                }
                // Java: else if (e instanceof InkSource)
                {
                    // Java: InkSource is = (InkSource)e;
                    // Java: ret.put(new String(k), is.clone());
                }
                // Java: else if (e instanceof Timestamp)
                {
                    // Java: Timestamp ts = (Timestamp)e;
                    // Java: ret.put(new String(k), ts.clone());
                }
                // Java: else if (e instanceof TraceFormat)
                {
                    // Java: TraceFormat tf = (TraceFormat)e;
                    // Java: ret.put(new String(k), tf.clone());
                }
                // Java: else if (e instanceof Mapping)
                {
                    // Java: Mapping m = (Mapping)e;
                    // Java: ret.put(new String(k), m.clone());
                }
                // Java: else if (e instanceof TraceGroup)
                {
                    // Java: TraceGroup traceGroup = (TraceGroup)e;
                    // Java: ret.put(new String(k), traceGroup.clone());
                }
                // Java: else if (e instanceof Trace)
                {
                    // Java: Trace trace = (Trace)e;
                    // Java: ret.put(new String(k), trace.clone());
                }
                // Java: else if (e instanceof TraceView)
                {
                    // Java: TraceView traceView = (TraceView)e;
                    // Java: ret.put(new String(k), traceView.clone());
                }
            }
            // Java: return ret;
            ret
        }
    }
}
