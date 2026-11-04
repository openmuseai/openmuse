use std::collections::HashMap;
// Java: import java.util.HashMap

use crate::com::hp::hpl::inkml::Mapping::Mapping;

// Java: public class CanvasTransformLoader extends CanvasTransform
pub struct CanvasTransformLoader {
    attributesMap: Option<HashMap<String, String>>,
    forwardMapping: Option<Mapping>,
    reverseMapping: Option<Mapping>,
}

// Java: public class CanvasTransformLoader extends CanvasTransform
impl CanvasTransformLoader {
    // Java: CanvasTransformLoader()
    pub fn CanvasTransformLoader() -> Self {
      // Java: super();
      Self {
          attributesMap: None,
          forwardMapping: None,
          reverseMapping: None,
      }
    }

    // Java: void setAttribute(String attrName, String attrValue)
    pub fn setAttribute(&mut self, attr_name: &str, attr_value: &str) {
      // Java: if (this.attributesMap == null)
      if self.attributesMap.is_none()
      {
         // Java: this.attributesMap = new HashMap();
         self.attributesMap = Some(HashMap::new());
      }
      // Java: this.attributesMap.put(attrName, attrValue);
      if let Some(ref mut map) = self.attributesMap {
          map.insert(attr_name.to_string(), attr_value.to_string());
      }
    }

    // Java: void setForwardMapping(Mapping mapping)
    pub fn setForwardMapping(&mut self, mapping: Mapping) {
      // Java: this.forwardMapping = mapping;
      self.forwardMapping = Some(mapping);
    }

    // Java: void setReverseMapping(Mapping mapping)
    pub fn setReverseMapping(&mut self, mapping: Mapping) {
      // Java: this.reverseMapping = mapping;
      self.reverseMapping = Some(mapping);
    }

    // Java: void resetForwardMappingToDefault(Mapping mapping)
    pub fn resetForwardMappingToDefault(&mut self, _mapping: Mapping) {
      // Java: this.forwardMapping = Mapping.getDefaultMapping();
      self.forwardMapping = Some(Mapping::getDefaultMapping());
    }

    // Java: void resetReverseMappingToDefault(Mapping mapping)
    pub fn resetReverseMappingToDefault(&mut self, _mapping: Mapping) {
      // Java: this.reverseMapping = Mapping.getDefaultMapping();
      self.reverseMapping = Some(Mapping::getDefaultMapping());
    }

    // Java: void setInvertible(boolean isInvertible)
    pub fn setInvertible(&mut self, is_invertible: bool) {
      // Java: this.attributesMap.put("invertible", String.valueOf(isInvertible));
      if let Some(ref mut map) = self.attributesMap {
          map.insert("invertible".to_string(), is_invertible.to_string());
      }
    }

}
