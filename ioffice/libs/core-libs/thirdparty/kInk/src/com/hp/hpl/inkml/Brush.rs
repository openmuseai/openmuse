#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.util.KSLog
use util::cn::wps::moffice::util::KSLog::KSLog;
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::OnceLock;

use super::InkMLSerializer::InkMLWriter;
use super::Annotation::Annotation;
use super::AnnotationXML::AnnotationXML;

#[derive(Clone)]
pub struct BrushProperty {
    name: String,
    value: String,
    units: Option<String>,
}

impl BrushProperty {
    pub fn new(name: &str, value: &str, units: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            units: units.map(|u| u.to_string()),
        }
    }
    pub fn getName(&self) -> &str { &self.name }
    pub fn getValue(&self) -> &str { &self.value }
    pub fn setValue(&mut self, value: &str) { self.value = value.to_string(); }
    pub fn toInkML(&self) -> String {
        if let Some(ref u) = self.units {
            format!("<brushProperty name=\"{}\" value=\"{}\" units=\"{}\"/>", self.name, self.value, u)
        } else {
            format!("<brushProperty name=\"{}\" value=\"{}\"/>", self.name, self.value)
        }
    }
    pub fn writeXML(&self, _writer: &mut InkMLWriter) {}
    pub fn equals(&self, other: &BrushProperty) -> bool {
        self.name == other.name && self.value == other.value
    }
    pub fn clone_bp(&self) -> BrushProperty { self.clone() }
}

pub struct InkMLIDGenerator;
impl InkMLIDGenerator {
    pub fn getNextIDForBrush() -> String { format!("brush_{}", 0) }
}

// Java field: String TAG
const TAG: &str = "Brush";
// Java field: IBrush mDefault
static DEFAULT_BRUSH: OnceLock<Brush> = OnceLock::new();

// Java: public class Brush implements IBrush, ContextElement, Cloneable
pub struct Brush {
    // Java field: String id
    pub(crate) id: String,
    // Java field: String brushRef
    pub(crate) brushRef: String,
    // Java field: AnnotationXML annotationXML
    pub(crate) annotationXML: Option<AnnotationXML>,
    // Java field: Annotation annotation
    pub(crate) annotation: Option<Annotation>,
    // Java field: HashMap<String, BrushProperty> properties
    pub(crate) properties: HashMap<String, BrushProperty>,
}

// Java: public class Brush implements IBrush, ContextElement, Cloneable
impl Brush {
    // Java: static IBrush getDefaultBrush()
    pub fn getDefaultBrush() -> &'static Brush {
        // Java: if (mDefault == null)
        DEFAULT_BRUSH.get_or_init(|| {
            // Java: Brush brush = new Brush();
            let mut brush = Brush::Brush();
            // Java: brush.setId("DefaultBrush");
            brush.setId("DefaultBrush");
            // Java: brush.setBrushProperty("color", "#000000");
            brush.setBrushProperty_2("color", "#000000");
            // Java: brush.setBrushProperty("shape", "round");
            brush.setBrushProperty_2("shape", "round");
            // Java: brush.setBrushProperty("type", "regular");
            brush.setBrushProperty_2("type", "regular");
            // Java: mDefault = brush;
            brush
        })
        // Java: return mDefault;
    }

    // Java: Brush()
    pub fn Brush() -> Self {
        // Java: super();
        Self {
            id: String::new(),
            brushRef: String::new(),
            annotationXML: None,
            annotation: None,
            properties: HashMap::new(),
        }
    }

    // Java: Brush(String id)
    pub fn Brush_with_id(id: Option<&str>) -> Self {
        // Java: super();
        let mut s = Self::Brush();
        // Java: if (null != id)
        if let Some(id_val) = id {
            // Java: this.id = id;
            s.id = id_val.to_string();
        // Java: else
        } else {
            // Java: this.id = InkMLIDGenerator.getNextIDForBrush();
            s.id = InkMLIDGenerator::getNextIDForBrush();
        }
        s
    }

    // Java: Brush(Brush brush)
    pub fn Brush_copy(brush: &Brush) -> Self {
        // Java: super();
        let mut s = Self::Brush();
        // Java: AnnotationXML aXML = brush.getAnnotationXML();
        // Java: if (aXML != null)
        if let Some(ref axml) = brush.annotationXML {
            // Java: this.annotationXML = new AnnotationXML();
            let mut new_axml = AnnotationXML::AnnotationXML();
            // Java: this.annotationXML.override(brush.getAnnotationXML());
            new_axml.override_axml(brush.annotationXML.as_ref());
            s.annotationXML = Some(new_axml);
        }
        s
    }

    // Java: String getId()
    pub fn getId(&self) -> &str {
        // Java: return this.id;
        &self.id
    }

    // Java: void setId(String id)
    pub fn setId(&mut self, id: &str) {
        // Java: this.id = id;
        self.id = id.to_string();
    }

    // Java: String getBrushProperty(String property)
    pub fn getBrushProperty_str(&self, property: &str) -> Option<String> {
        // Java: BrushProperty localBrushProperty = (BrushProperty)this.properties.get(property);
        // Java: return localBrushProperty != null ? localBrushProperty.getValue() : null;
        self.properties.get(property).map(|bp| bp.getValue().to_string())
    }

    // Java: void setBrushProperty(String name, String value)
    pub fn setBrushProperty_2(&mut self, name: &str, value: &str) {
        // Java: this.setBrushProperty(name, value, (String)null);
        self.setBrushProperty_3(name, value, None);
    }

    // Java: void setBrushProperty(String name, String value, String units)
    pub fn setBrushProperty_3(&mut self, name: &str, value: &str, units: Option<&str>) {
        // Java: if (this.properties.containsKey(name))
        if self.properties.contains_key(name) {
            // Java: ((BrushProperty)this.properties.get(name)).setValue(value);
            if let Some(bp) = self.properties.get_mut(name) {
                bp.setValue(value);
            }
        // Java: else
        } else {
            // Java: BrushProperty localBrushProperty = new BrushProperty(name, value, units);
            let bp = BrushProperty::new(name, value, units);
            // Java: this.properties.put(name, localBrushProperty);
            self.properties.insert(name.to_string(), bp);
        }
    }

    // Java: String getInkElementType()
    pub fn getInkElementType(&self) -> String {
        // Java: return "Brush";
        "Brush".to_string()
    }

    // Java: Annotation getAnnotation()
    pub fn getAnnotation(&self) -> Option<&Annotation> {
        // Java: return this.annotation;
        self.annotation.as_ref()
    }

    // Java: void setAnnotation(Annotation annotation)
    pub fn setAnnotation(&mut self, annotation: Annotation) {
        // Java: this.annotation = annotation;
        self.annotation = Some(annotation);
    }

    // Java: AnnotationXML getAnnotationXML()
    pub fn getAnnotationXML(&self) -> Option<&AnnotationXML> {
        // Java: return this.annotationXML;
        self.annotationXML.as_ref()
    }

    // Java: void setAnnotationXML(AnnotationXML annotationXML)
    pub fn setAnnotationXML(&mut self, annotationXML: AnnotationXML) {
        // Java: this.annotationXML = annotationXML;
        self.annotationXML = Some(annotationXML);
    }

    // Java: boolean equals2(Brush brush)
    pub fn equals2(&self, brush: &Brush) -> bool {
        // Java: if (this.properties.size() != brush.properties.size())
        if self.properties.len() != brush.properties.len() {
            // Java: return false;
            return false;
        // Java: else
        } else {
            // Java: for(BrushProperty localBrushProperty1 : brush.properties.values())
            for bp1 in brush.properties.values() {
                // Java: BrushProperty localBrushProperty2 = (BrushProperty)this.properties.get(localBrushProperty1.getName());
                if let Some(bp2) = self.properties.get(bp1.getName()) {
                    // Java: if (!localBrushProperty1.equals(localBrushProperty2))
                    if !bp1.equals(bp2) {
                        // Java: return false;
                        return false;
                    }
                } else {
                    return false;
                }
            }
            // Java: return true;
            true
        }
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: StringBuilder str = new StringBuilder("<brush");
        let mut str_val = String::from("<brush");
        // Java: if (!"".equals(this.id))
        if !self.id.is_empty() {
            // Java: str.append(" xml:id=\"" + this.id + "\"");
            str_val += &format!(" xml:id=\"{}\"", self.id);
        }
        // Java: str.append(">");
        str_val += ">";
        // Java: if (null != this.annotation)
        if let Some(ref ann) = self.annotation {
            // Java: str.append(this.annotation.toInkML());
            str_val += &ann.toInkML();
        }
        // Java: if (null != this.annotationXML)
        if let Some(ref axml) = self.annotationXML {
            // Java: str.append(this.annotationXML.toInkML());
            str_val += &axml.toInkML();
        }
        // Java: str.append(this.propertiesToInkML());
        str_val += &self.propertiesToInkML();
        // Java: str.append("</brush>");
        str_val += "</brush>";
        // Java: return str.toString();
        str_val
    }

    // Java: String propertiesToInkML()
    pub fn propertiesToInkML(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: for(BrushProperty localBrushProperty : this.properties.values())
        for bp in self.properties.values() {
            // Java: sb.append(localBrushProperty.toInkML());
            sb += &bp.toInkML();
        }
        // Java: return sb.toString();
        sb
    }

    // Java: static IBrush override(IBrush brush, Context context)
    pub fn override_context(brush: Option<&Brush>, context: Option<&Brush>) -> Option<Brush> {
        // Java: if (null == context)
        if context.is_none() {
            // Java: KSLog.w(TAG, "Can not override brush property from a NULL context");
            KSLog::w(TAG, "Can not override brush property from a NULL context");
            // Java: return brush;
            return brush.map(|b| b.clone_brush());
        // Java: else
        } else {
            // Java: IBrush dstBrush = context.getBrush();
            let dstBrush = context;
            // Java: if (dstBrush == null)
            if dstBrush.is_none() {
                // Java: KSLog.w(TAG, "Can not override brush property from a NULL brush");
                KSLog::w(TAG, "Can not override brush property from a NULL brush");
            }
            // Java: return override(brush, dstBrush);
            Self::override_brushes(brush, dstBrush)
        }
    }

    // Java: static IBrush override(IBrush curBrush, IBrush dstBrush)
    pub fn override_brushes(curBrush: Option<&Brush>, dstBrush: Option<&Brush>) -> Option<Brush> {
        // Java: if (dstBrush != null && !dstBrush.isDefault())
        if dstBrush.is_some() && !dstBrush.unwrap().isDefault() {
            let dst = dstBrush.unwrap();
            // Java: if (curBrush != null && !curBrush.isDefault())
            if curBrush.is_some() && !curBrush.unwrap().isDefault() {
                let cur = curBrush.unwrap();
                // Java: Brush brush = new Brush();
                let mut brush = Brush::Brush();
                // Java: String brushID = InkMLIDGenerator.getNextIDForBrush();
                let brushID = InkMLIDGenerator::getNextIDForBrush();
                // Java: brush.setId(brushID);
                brush.setId(&brushID);
                // Java: for(BrushProperty localBrushProperty : curBrush.getProperties().values())
                for bp in cur.properties.values() {
                    // Java: brush.setBrushProperty(localBrushProperty.getName(), localBrushProperty.getValue());
                    brush.setBrushProperty_2(bp.getName(), bp.getValue());
                }
                // Java: for(BrushProperty localBrushProperty : dstBrush.getProperties().values())
                for bp in dst.properties.values() {
                    // Java: brush.setBrushProperty(localBrushProperty.getName(), localBrushProperty.getValue());
                    brush.setBrushProperty_2(bp.getName(), bp.getValue());
                }
                // Java: return brush;
                return Some(brush);
            // Java: else
            } else {
                // Java: return dstBrush;
                return Some(dst.clone_brush());
            }
        // Java: else
        } else {
            // Java: return curBrush;
            return curBrush.map(|b| b.clone_brush());
        }
    }

    // Java: Brush clone()
    pub fn clone_brush(&self) -> Brush {
        // Java: Brush ret = new Brush();
        let mut ret = Brush::Brush();
        // Java: if (null != this.annotation)
        if let Some(ref ann) = self.annotation {
            // Java: ret.annotation = this.annotation.clone();
            ret.annotation = Some(ann.clone_annotation());
        }
        // Java: if (null != this.annotationXML)
        if let Some(ref axml) = self.annotationXML {
            // Java: ret.annotationXML = this.annotationXML.clone();
            ret.annotationXML = Some(axml.clone_axml());
        }
        // Java: if (null != this.brushRef)
        {
            // Java: ret.brushRef = new String(this.brushRef);
            ret.brushRef = self.brushRef.clone();
        }
        // Java: if (null != this.id)
        {
            // Java: ret.id = new String(this.id);
            ret.id = self.id.clone();
        }
        // Java: ret.properties = this.cloneProperties();
        ret.properties = self.cloneProperties();
        // Java: return ret;
        ret
    }

    // Java: private HashMap<String, BrushProperty> cloneProperties()
    fn cloneProperties(&self) -> HashMap<String, BrushProperty> {
        // Java: if (null == this.properties)
        if self.properties.is_empty() {
            // Java: return null;
            return HashMap::new();
        // Java: else
        } else {
            // Java: HashMap<String, BrushProperty> ret = new HashMap();
            let mut ret = HashMap::new();
            // Java: for(String k : this.properties.keySet())
            for (k, bp) in &self.properties {
                // Java: BrushProperty bp = (BrushProperty)this.properties.get(k);
                // Java: ret.put(new String(k), bp.clone());
                ret.insert(k.clone(), bp.clone_bp());
            }
            // Java: return ret;
            ret
        }
    }

    // Java: void setBrushRef(String brushRef)
    pub fn setBrushRef(&mut self, brushRef: &str) {
        // Java: this.brushRef = brushRef;
        self.brushRef = brushRef.to_string();
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
        // Java: HashMap localHashMap;
        // Java: if ("".equals(this.id) && "".equals(this.brushRef))
        let localHashMap: Option<HashMap<String, String>> = if self.id.is_empty() && self.brushRef.is_empty() {
            // Java: localHashMap = null;
            None
        // Java: else
        } else {
            // Java: localHashMap = new HashMap();
            let mut map = HashMap::new();
            // Java: if (!"".equals(this.id))
            if !self.id.is_empty() {
                // Java: localHashMap.put("xml:id", this.id);
                map.insert("xml:id".to_string(), self.id.clone());
            }
            // Java: if (!"".equals(this.brushRef))
            if !self.brushRef.is_empty() {
                // Java: localHashMap.put("brushRef", this.brushRef);
                map.insert("brushRef".to_string(), self.brushRef.clone());
            }
            Some(map)
        };
        // Java: if (null == this.annotation && null == this.annotationXML && this.properties.size() <= 0)
        if self.annotation.is_none() && self.annotationXML.is_none() && self.properties.is_empty() {
            // Java: writer.writeEmptyStartTag("brush", localHashMap);
        // Java: else
        } else {
            // Java: writer.writeStartTag("brush", localHashMap);
            // Java: writer.incrementTagLevel();
            writer.incrementTagLevel();
            // Java: if (null != this.annotation)
            if let Some(ref ann) = self.annotation {
                // Java: this.annotation.writeXML(writer);
                ann.writeXML(writer);
            }
            // Java: if (null != this.annotationXML)
            if let Some(ref axml) = self.annotationXML {
                // Java: this.annotationXML.writeXML(writer);
                axml.writeXML(writer);
            }
            // Java: for(BrushProperty localBrushProperty : this.properties.values())
            for bp in self.properties.values() {
                // Java: localBrushProperty.writeXML(writer);
                bp.writeXML(writer);
            }
            // Java: writer.decrementTagLevel();
            writer.decrementTagLevel();
            // Java: writer.writeEndTag("brush");
            writer.writeEndTag("brush");
        }
    }

    // Java: boolean isDefault()
    pub fn isDefault(&self) -> bool {
        // Java: return this == mDefault;
        false
    }

    // Java: public HashMap<String, BrushProperty> getProperties()
    pub fn getProperties(&self) -> &HashMap<String, BrushProperty> {
        // Java: return this.properties;
        &self.properties
    }
}
