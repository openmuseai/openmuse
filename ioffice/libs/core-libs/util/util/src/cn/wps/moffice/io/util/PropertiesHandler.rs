#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_unsafe)]

// Java: import cn.wps.kfc.opcr.OpcUtil
use kfc_kfc_alpha::kfc::opcr::OpcUtil::OpcUtil;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementStrHandler
use kfc_kfc_alpha::kfc::xpxml::reader::DefaultElementStrHandler::DefaultElementStrHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementStrHandler
use kfc_kfc_alpha::kfc::xpxml::reader::ElementStrHandler::ElementStrHandler;
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Map
// LLM JavaLine Aggregation: java.util.Map is represented by HashMap in Rust
// Java: import org.apache.poi.hpsf.Property
use io_poi_compact::org::apache::poi::hpsf::Property::{Property, PropertyValue};

use std::any::Any;
use std::ops::{Deref, DerefMut};

struct OuterRef {
    mCustomProps_ptr: *mut Option<HashMap<Option<String>, Option<Property>>>,
}

impl OuterRef {
    fn mCustomProps(&self) -> &mut HashMap<Option<String>, Option<Property>> {
        unsafe { (*self.mCustomProps_ptr).as_mut().unwrap() }
    }
}

struct ParentRef {
    mType: *const i64,
    mValue: *mut Option<Box<dyn Any>>,
}

// Java: public class PropertiesHandler extends DefaultElementStrHandler
pub struct PropertiesHandler {
    parent: DefaultElementStrHandler,
    // Java field: Map<String, Property> mCustomProps
    pub(crate) mCustomProps: Option<HashMap<Option<String>, Option<Property>>>,
    // Java field: PropertyHandler mHandler
    mHandler: Option<PropertyHandler>,
}

impl Deref for PropertiesHandler {
    type Target = DefaultElementStrHandler;
    fn deref(&self) -> &Self::Target { &self.parent }
}

impl DerefMut for PropertiesHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.parent }
}

// Java: public class PropertyHandler
pub struct PropertyHandler {
    parent: DefaultElementStrHandler,
    _PropertiesHandler_this: OuterRef,
    // Java field: String mName
    pub(crate) mName: Option<String>,
    // Java field: long mType
    pub(crate) mType: i64,
    // Java field: Object mValue
    pub(crate) mValue: Option<Box<dyn Any>>,
    // Java field: PropertyValueHandler mValueHandler
    pub(crate) mValueHandler: Option<PropertyValueHandler>,
}

impl Deref for PropertyHandler {
    type Target = DefaultElementStrHandler;
    fn deref(&self) -> &Self::Target { &self.parent }
}

impl DerefMut for PropertyHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.parent }
}

// Java: public class PropertyValueHandler
pub struct PropertyValueHandler {
    parent: DefaultElementStrHandler,
    _PropertyHandler_this: ParentRef,
}

impl Deref for PropertyValueHandler {
    type Target = DefaultElementStrHandler;
    fn deref(&self) -> &Self::Target { &self.parent }
}

impl DerefMut for PropertyValueHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.parent }
}

// Java: public class PropertiesHandler extends DefaultElementStrHandler
impl PropertiesHandler {
    pub fn new(customProps: HashMap<String, Property>) -> Self {
        let wrapped: HashMap<Option<String>, Option<Property>> = customProps.into_iter()
            .map(|(k, v)| (Some(k), Some(v)))
            .collect();
        let mut s = PropertiesHandler {
            parent: DefaultElementStrHandler::new(),
            mCustomProps: None,
            mHandler: None,
        };
        s.PropertiesHandler_init(wrapped);
        s
    }

    // Java: PropertiesHandler(Map<String, Property> customProps)
    pub fn PropertiesHandler_init(&mut self, customProps: HashMap<Option<String>, Option<Property>>) {
      // Java: super();
      self.parent = DefaultElementStrHandler::new();
      // Java: this.mCustomProps = customProps;
      self.mCustomProps = Some(customProps);
      // Java: this.mHandler = new PropertyHandler();
      let props_ptr = &mut self.mCustomProps as *mut Option<HashMap<Option<String>, Option<Property>>>;
      self.mHandler = Some(PropertyHandler::new_with_outer(props_ptr));
    }

    // Java: ElementStrHandler enterSubElement(String subLabel)
    pub fn enterSubElement(&mut self, subLabel: &str) -> Option<&mut PropertyHandler> {
      // Java: return subLabel.equals("property") ? this.mHandler : null;
      if subLabel == "property" {
          self.mHandler.as_mut()
      } else {
          None
      }
    }

}

// Java: public class PropertyHandler
impl PropertyHandler {
    fn new_with_outer(outer_custom_props: *mut Option<HashMap<Option<String>, Option<Property>>>) -> Self {
        let mut handler = PropertyHandler {
            parent: DefaultElementStrHandler::new(),
            _PropertiesHandler_this: OuterRef { mCustomProps_ptr: outer_custom_props },
            mName: None,
            mType: -1,
            mValue: None,
            mValueHandler: None,
        };
        let type_ptr = &handler.mType as *const i64;
        let value_ptr = &mut handler.mValue as *mut Option<Box<dyn Any>>;
        handler.mValueHandler = Some(PropertyValueHandler {
            parent: DefaultElementStrHandler::new(),
            _PropertyHandler_this: ParentRef { mType: type_ptr, mValue: value_ptr },
        });
        handler
    }

    fn to_property_value(&self, any_val: Box<dyn Any>) -> PropertyValue {
        match self.mType {
            11 => {
                match any_val.downcast::<bool>() {
                    Ok(v) => PropertyValue::Bool(*v),
                    Err(_) => PropertyValue::None,
                }
            }
            3 => {
                match any_val.downcast::<i32>() {
                    Ok(v) => PropertyValue::Int(*v),
                    Err(_) => PropertyValue::None,
                }
            }
            64 => {
                match any_val.downcast::<i64>() {
                    Ok(v) => PropertyValue::Long(*v),
                    Err(_) => PropertyValue::None,
                }
            }
            30 | 31 => {
                match any_val.downcast::<String>() {
                    Ok(v) => PropertyValue::Str(*v),
                    Err(_) => PropertyValue::None,
                }
            }
            _ => PropertyValue::None,
        }
    }

    // Java: void startElement(String label)
    pub fn startElement(&mut self, label: &str) {
         // Java: this.mName = null;
         self.mName = None;
         // Java: this.mType = -1L;
         self.mType = -1;
         // Java: this.mValue = null;
         self.mValue = None;
    }

    // Java: void addAttr(String name, String value)
    pub fn addAttr(&mut self, name: &str, value: &str) {
         // Java: if (name.equals("name"))
         if name == "name" {
            // Java: this.mName = value;
            self.mName = Some(value.to_string());
         }
    }

    // Java: void endElement(String label)
    pub fn endElement(&mut self, label: &str) {
         // Java: if (this.mName != null && this.mType != -1L && this.mValue != null)
         if self.mName.is_some() && self.mType != -1 && self.mValue.is_some() {
            let name = self.mName.as_ref().unwrap().clone();
            let any_val = self.mValue.take().unwrap();
            let pv = self.to_property_value(any_val);
            // Java: PropertiesHandler.this.mCustomProps.put(this.mName, new Property(-1L, this.mType, this.mValue));
            self._PropertiesHandler_this.mCustomProps().insert(Some(name), Some(Property::new(-1, self.mType, pv)));
         }
    }

    // Java: ElementStrHandler enterSubElement(String subLabel)
    pub fn enterSubElement(&mut self, subLabel: &str) -> Option<&mut PropertyValueHandler> {
         // Java: if (subLabel.equals("vt:bool"))
         if subLabel == "vt:bool" {
            // Java: this.mType = 11L;
            self.mType = 11;
            // Java: return this.mValueHandler;
            return self.mValueHandler.as_mut();
         }
         // Java: else if (subLabel.equals("vt:lpwstr"))
         else if subLabel == "vt:lpwstr" {
            // Java: this.mType = 31L;
            self.mType = 31;
            // Java: return this.mValueHandler;
            return self.mValueHandler.as_mut();
         }
         // Java: else if (subLabel.equals("vt:lpstr"))
         else if subLabel == "vt:lpstr" {
            // Java: this.mType = 30L;
            self.mType = 30;
            // Java: return this.mValueHandler;
            return self.mValueHandler.as_mut();
         }
         // Java: else if (subLabel.equals("vt:i4"))
         else if subLabel == "vt:i4" {
            // Java: this.mType = 3L;
            self.mType = 3;
            // Java: return this.mValueHandler;
            return self.mValueHandler.as_mut();
         }
         // Java: else if (subLabel.equals("vt:filetime"))
         else if subLabel == "vt:filetime" {
            // Java: this.mType = 64L;
            self.mType = 64;
            // Java: return this.mValueHandler;
            return self.mValueHandler.as_mut();
         }
         // Java: else
         else {
            // Java: return null;
            return None;
         }
    }

}

// Java: public class PropertyValueHandler
impl PropertyValueHandler {
    // Java: void characters(String chars)
    pub fn characters(&mut self, chars: &str) {
            unsafe {
            let m_type = *self._PropertyHandler_this.mType;
            // Java: if (PropertyHandler.this.mType == 11L)
            if m_type == 11 {
               // Java: PropertyHandler.this.mValue = chars.equals("1") || chars.toLowerCase().equals("true") || chars.toLowerCase().equals("on");
               let val = chars == "1" || chars.to_lowercase() == "true" || chars.to_lowercase() == "on";
               *self._PropertyHandler_this.mValue = Some(Box::new(val));
            }
            // Java: else if (PropertyHandler.this.mType != 31L && PropertyHandler.this.mType != 30L)
            else if m_type != 31 && m_type != 30 {
               // Java: if (PropertyHandler.this.mType == 3L)
               if m_type == 3 {
                  // Java: PropertyHandler.this.mValue = Integer.parseInt(chars);
                  let parsed = chars.parse().unwrap_or(0i32);
                  *self._PropertyHandler_this.mValue = Some(Box::new(parsed));
               }
               // Java: else if (PropertyHandler.this.mType == 64L)
               else if m_type == 64 {
                  // Java: PropertyHandler.this.mValue = OpcUtil.getDate(chars);
                  let date = OpcUtil::getDate(chars);
                  if date.is_some() {
                      *self._PropertyHandler_this.mValue = Some(Box::new(date.unwrap()));
                  }
               }
            }
            // Java: else
            else {
               // Java: PropertyHandler.this.mValue = chars;
               *self._PropertyHandler_this.mValue = Some(Box::new(chars.to_string()));
            }
            }
    }

}
