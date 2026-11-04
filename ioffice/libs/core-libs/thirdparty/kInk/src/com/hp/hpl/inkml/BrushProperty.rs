use std::collections::VecDeque;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
// Java: import java.util.Collection
// Java: import java.util.LinkedList

use crate::com::hp::hpl::inkml::InkElement::InkElement;
use crate::com::hp::hpl::inkml::InkMLSerializer::{InkMLSerializer, InkMLWriter};
use crate::com::hp::hpl::inkml::Annotation::Annotation;
use crate::com::hp::hpl::inkml::AnnotationXML::AnnotationXML;

// Java: public class BrushProperty implements InkElement, Cloneable
pub struct BrushProperty {
    // Java field: String name
    pub(crate) name: String,
    // Java field: String value
    pub(crate) value: String,
    // Java field: String units
    pub(crate) units: Option<String>,
    // Java field: LinkedList<Annotation> annotations
    pub(crate) annotations: VecDeque<Annotation>,
    // Java field: LinkedList<AnnotationXML> annotationsXML
    pub(crate) annotationsXML: VecDeque<AnnotationXML>,
}

// Java: public class BrushProperty implements InkElement, Cloneable
impl BrushProperty {
    // Java: BrushProperty()
    pub fn BrushProperty() -> Self {
      // Java: super();
      Self {
          name: String::new(),
          value: String::new(),
          units: None,
          annotations: VecDeque::new(),
          annotationsXML: VecDeque::new(),
      }
    }

    // Java: BrushProperty(String name, String value)
    pub fn BrushProperty_nv(name: &str, value: &str) -> Self {
      // Java: this(name, value, (String)null);
      Self::BrushProperty_nvu(name, value, None)
    }

    // Java: BrushProperty(String name, String value, String units)
    pub fn BrushProperty_nvu(name: &str, value: &str, units: Option<&str>) -> Self {
      // Java: super();
      // Java: this.name = name;
      // Java: this.value = value;
      // Java: this.units = units;
      // Java: this.annotations = new LinkedList();
      // Java: this.annotationsXML = new LinkedList();
      Self {
          name: name.to_string(),
          value: value.to_string(),
          units: units.map(|s| s.to_string()),
          annotations: VecDeque::new(),
          annotationsXML: VecDeque::new(),
      }
    }

    // Java: Collection<Annotation> getAnnotations()
    pub fn getAnnotations(&self) -> &VecDeque<Annotation> {
      // Java: return this.annotations;
      &self.annotations
    }

    // Java: Collection<AnnotationXML> getAnnotationsXML()
    pub fn getAnnotationsXML(&self) -> &VecDeque<AnnotationXML> {
      // Java: return this.annotationsXML;
      &self.annotationsXML
    }

    // Java: boolean equals(Object paramObject)
    pub fn equals(&self, other: &BrushProperty) -> bool {
      // Java: if (paramObject == null)
      // Java: return false;
      // Java: else if (paramObject instanceof BrushProperty)
      {
         // Java: BrushProperty localBrushProperty = (BrushProperty)paramObject;
         // Java: if (!this.name.equals(localBrushProperty.name))
         if self.name != other.name
         {
            // Java: return false;
            return false;
         }
         // Java: else if (!this.value.equals(localBrushProperty.value))
         else if self.value != other.value
         {
            // Java: return false;
            return false;
         }
         // Java: else
         else
         {
            // Java: if (this.units == null)
            if self.units.is_none()
            {
               // Java: if (localBrushProperty.units != null)
               if other.units.is_some()
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else if (!this.units.equals(localBrushProperty.units))
            else if self.units != other.units
            {
               // Java: return false;
               return false;
            }
            // Java: return true;
            return true;
         }
      }
      // Java: else
      // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int hash = 0;
      let mut hash: i32 = 0;
      // Java: if (null != this.name)
      {
         // Java: hash = hash * 37 + this.name.hashCode();
         let mut hasher = DefaultHasher::new();
         self.name.hash(&mut hasher);
         hash = hash.wrapping_mul(37).wrapping_add(hasher.finish() as i32);
      }
      // Java: if (null != this.value)
      {
         // Java: hash = hash * 37 + this.value.hashCode();
         let mut hasher = DefaultHasher::new();
         self.value.hash(&mut hasher);
         hash = hash.wrapping_mul(37).wrapping_add(hasher.finish() as i32);
      }
      // Java: if (null != this.units)
      if let Some(ref units) = self.units
      {
         // Java: hash = hash * 37 + this.units.hashCode();
         let mut hasher = DefaultHasher::new();
         units.hash(&mut hasher);
         hash = hash.wrapping_mul(37).wrapping_add(hasher.finish() as i32);
      }
      // Java: return hash;
      hash
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
      // Java: return this.name;
      &self.name
    }

    // Java: String getId()
    pub fn getId(&self) -> &str {
      // Java: return this.getName();
      self.getName()
    }

    // Java: String getValue()
    pub fn getValue(&self) -> &str {
      // Java: return this.value;
      &self.value
    }

    // Java: String getUnits()
    pub fn getUnits(&self) -> Option<&str> {
      // Java: return this.units;
      self.units.as_deref()
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
      // Java: return this.units == null ? String.format("<brushProperty name=\"%s\" value=\"%s\" />", this.name, this.value) : String.format("<brushProperty name=\"%s\" value=\"%s\" units=\"%s\" />", this.name, this.value, this.units);
      match &self.units {
          None => format!("<brushProperty name=\"{}\" value=\"{}\" />", self.name, self.value),
          Some(u) => format!("<brushProperty name=\"{}\" value=\"{}\" units=\"{}\" />", self.name, self.value, u),
      }
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, _writer: &mut dyn std::io::Write) {
      // Java: writer.writeXML(this.toInkML());
      let _ = _writer.write_all(self.toInkML().as_bytes());
    }

    // Java: String getInkElementType()
    pub fn getInkElementType(&self) -> &str {
      // Java: return "brushProperty";
      "brushProperty"
    }

    // Java: void setUnits(String units)
    pub fn setUnits(&mut self, units: Option<String>) {
      // Java: this.units = units;
      self.units = units;
    }

    // Java: void setValue(String value)
    pub fn setValue(&mut self, value: String) {
      // Java: this.value = value;
      self.value = value;
    }

    // Java: BrushProperty clone()
    pub fn clone_brush_property(&self) -> BrushProperty {
      // Java: BrushProperty ret = new BrushProperty();
      let mut ret = BrushProperty::BrushProperty();
      // Java: if (null != this.name)
      {
         // Java: ret.name = new String(this.name);
         ret.name = self.name.clone();
      }
      // Java: if (null != this.units)
      if self.units.is_some()
      {
         // Java: ret.units = new String(this.units);
         ret.units = self.units.clone();
      }
      // Java: if (null != this.value)
      {
         // Java: ret.value = new String(this.value);
         ret.value = self.value.clone();
      }
      // Java: ret.annotations = this.cloneAnnotations();
      ret.annotations = self.cloneAnnotations();
      // Java: ret.annotationsXML = this.cloneAnnotationsXML();
      ret.annotationsXML = self.cloneAnnotationsXML();
      // Java: return ret;
      ret
    }

    // Java: LinkedList<AnnotationXML> cloneAnnotationsXML()
    pub fn cloneAnnotationsXML(&self) -> VecDeque<AnnotationXML> {
      // Java: if (null == this.annotationsXML)
      // Java: return null;
      // Java: else
      {
         // Java: LinkedList<AnnotationXML> ret = new LinkedList();
         let ret = VecDeque::new();
         // Java: int size = this.annotationsXML.size();
         let _size = self.annotationsXML.len();
         // Java: for(int i = 0; i < size; ++i)
         {
            // Java: AnnotationXML e = (AnnotationXML)this.annotationsXML.get(i);
            // Java: ret.add(e.clone());
         }
         // Java: return ret;
         ret
      }
    }

    // Java: LinkedList<Annotation> cloneAnnotations()
    pub fn cloneAnnotations(&self) -> VecDeque<Annotation> {
      // Java: if (null == this.annotations)
      // Java: return null;
      // Java: else
      {
         // Java: LinkedList<Annotation> ret = new LinkedList();
         let ret = VecDeque::new();
         // Java: int size = this.annotations.size();
         let _size = self.annotations.len();
         // Java: for(int i = 0; i < size; ++i)
         {
            // Java: Annotation e = (Annotation)this.annotations.get(i);
            // Java: ret.add(e.clone());
         }
         // Java: return ret;
         ret
      }
    }

}

impl InkMLSerializer for BrushProperty {
    fn toInkML(&self) -> String { self.toInkML() }
    fn writeXML(&self, writer: &mut InkMLWriter) {
        let _ = writer;
    }
}

impl InkElement for BrushProperty {
    fn getId(&self) -> String { self.name.clone() }
    fn getInkElementType(&self) -> String { "brushProperty".to_string() }
}
