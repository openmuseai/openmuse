#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.util.KSLog
use util::cn::wps::moffice::util::KSLog::KSLog;
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::OnceLock;

use super::InkMLSerializer::InkMLWriter;
use super::Mapping::Mapping;

// Java field: String TAG
const TAG: &str = "CanvasTransform";
// Java field: CanvasTransform mDefault
static DEFAULT_CT: OnceLock<CanvasTransform> = OnceLock::new();

// Java: public class CanvasTransform implements ContextElement, Cloneable
pub struct CanvasTransform {
    // Java field: HashMap<String, String> attributesMap
    pub(crate) attributesMap: HashMap<String, String>,
    // Java field: Mapping forwardMapping
    pub(crate) forwardMapping: Option<Mapping>,
    // Java field: Mapping reverseMapping
    pub(crate) reverseMapping: Option<Mapping>,
}

// Java: public class CanvasTransform implements ContextElement, Cloneable
impl CanvasTransform {
    // Java: static CanvasTransform getDefaultCanvasTransform()
    pub fn getDefaultCanvasTransform() -> &'static CanvasTransform {
        // Java: return _getDefaultCanvasTransform();
        Self::_getDefaultCanvasTransform()
    }

    // Java: private static synchronized CanvasTransform _getDefaultCanvasTransform()
    fn _getDefaultCanvasTransform() -> &'static CanvasTransform {
        // Java: if (mDefault == null)
        DEFAULT_CT.get_or_init(|| {
            // Java: mDefault = new CanvasTransform();
            let mut ct = CanvasTransform::CanvasTransform();
            // Java: mDefault.setId("DefaultCanvasTransform");
            ct.setId("DefaultCanvasTransform");
            ct
        })
        // Java: return mDefault;
    }

    // Java: CanvasTransform()
    pub fn CanvasTransform() -> Self {
        // Java: super();
        Self {
            attributesMap: HashMap::new(),
            forwardMapping: Some(Mapping::getDefaultMapping()),
            reverseMapping: Some(Mapping::getDefaultMapping()),
        }
    }

    // Java: String getId()
    pub fn getId(&self) -> String {
        // Java: String idAttr = (String)this.attributesMap.get("id");
        // Java: return idAttr != null ? idAttr : "";
        self.attributesMap.get("id").cloned().unwrap_or_default()
    }

    // Java: String getInkElementType()
    pub fn getInkElementType(&self) -> String {
        // Java: return "CanvasTransform";
        "CanvasTransform".to_string()
    }

    // Java: boolean equals2(CanvasTransform transform)
    pub fn equals2(&self, transform: Option<&CanvasTransform>) -> bool {
        // Java: if (transform == null)
        let t = match transform {
            None => {
                // Java: return false;
                return false;
            }
            Some(tr) => tr,
        };
        // Java: else if (transform == this)
        // Java: return true;
        // Java: else
        {
            // Java: boolean isEqual = true;
            let is_equal = true;
            // Java: if (this.isInvertible() != transform.isInvertible())
            if self.isInvertible() != t.isInvertible() {
                // Java: return false;
                return false;
            // Java: else if ((this.forwardMapping != null || this.reverseMapping == null) && (this.forwardMapping == null || this.reverseMapping != null))
            } else if (self.forwardMapping.is_some() || self.reverseMapping.is_none())
                && (self.forwardMapping.is_none() || self.reverseMapping.is_some()) {
                // Java: if (this.forwardMapping != null && !this.forwardMapping.equals2(transform.forwardMapping))
                if self.forwardMapping.is_some() && !self.forwardMapping.as_ref().unwrap().equals2(t.forwardMapping.as_ref()) {
                    // Java: return false;
                    return false;
                // Java: else
                } else {
                    // Java: return this.reverseMapping != null && !this.reverseMapping.equals2(transform.reverseMapping) ? false : isEqual;
                    if self.reverseMapping.is_some() && !self.reverseMapping.as_ref().unwrap().equals2(t.reverseMapping.as_ref()) {
                        false
                    } else {
                        is_equal
                    }
                }
            // Java: else
            } else {
                // Java: return false;
                false
            }
        }
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: String canvasTransform = "<canvasTransform ";
        let mut ct = String::from("<canvasTransform ");
        // Java: String id = this.getId();
        let id = self.getId();
        // Java: if (!"".equals(id))
        if !id.is_empty() {
            // Java: canvasTransform = canvasTransform + "id='" + id + "' ";
            ct += &format!("id='{}' ", id);
        }
        // Java: boolean isInvertible = this.isInvertible();
        let is_invertible = self.isInvertible();
        // Java: if (isInvertible)
        if is_invertible {
            // Java: canvasTransform = canvasTransform + "invertible='" + isInvertible + "' ";
            ct += &format!("invertible='{}' ", is_invertible);
        }
        // Java: canvasTransform = canvasTransform + ">";
        ct += ">";
        // Java: if (null != this.forwardMapping)
        if let Some(ref fm) = self.forwardMapping {
            // Java: canvasTransform = canvasTransform + this.forwardMapping.toInkML();
            ct += &fm.toInkML();
        // Java: else
        } else {
            // Java: canvasTransform = canvasTransform + "<mapping type='unknown'/>";
            ct += "<mapping type='unknown'/>";
        }
        // Java: if (null != this.reverseMapping)
        if let Some(ref rm) = self.reverseMapping {
            // Java: canvasTransform = canvasTransform + this.reverseMapping.toInkML();
            ct += &rm.toInkML();
        }
        // Java: canvasTransform = canvasTransform + "</canvasTransform>";
        ct += "</canvasTransform>";
        // Java: return canvasTransform;
        ct
    }

    // Java: boolean isInvertible()
    pub fn isInvertible(&self) -> bool {
        // Java: String invertibleAttr = (String)this.attributesMap.get("invertible");
        let invertibleAttr = self.attributesMap.get("invertible");
        // Java: if (invertibleAttr != null)
        if let Some(attr) = invertibleAttr {
            // Java: try
            {
                // Java: boolean status = new Boolean(invertibleAttr);
                // Java: return status;
                if let Ok(status) = attr.parse::<bool>() {
                    return status;
                }
            }
            // Java: catch (Exception var3)
            {
                // Java: KSLog.i(TAG, "Improper value to 'invertible' attribute, value = " + invertibleAttr + ". Returning the default value of false.");
            }
        }
        // Java: return false;
        false
    }

    // Java: void setId(String id)
    pub fn setId(&mut self, id: &str) {
        // Java: this.attributesMap.put("id", id);
        self.attributesMap.insert("id".to_string(), id.to_string());
    }

    // Java: Mapping getForwardMapping()
    pub fn getForwardMapping(&self) -> Option<&Mapping> {
        // Java: return this.forwardMapping;
        self.forwardMapping.as_ref()
    }

    // Java: Mapping getReverseMapping()
    pub fn getReverseMapping(&self) -> Option<&Mapping> {
        // Java: return this.reverseMapping;
        self.reverseMapping.as_ref()
    }

    // Java: void writeXML(InkMLWriter writer)
    pub fn writeXML(&self, writer: &mut InkMLWriter) {
        // Java: writer.writeStartTag("canvasTransform", this.attributesMap);
        // Java: writer.incrementTagLevel();
        writer.incrementTagLevel();
        // Java: if (null != this.forwardMapping)
        if let Some(ref fm) = self.forwardMapping {
            // Java: this.forwardMapping.writeXML(writer);
            fm.writeXML(writer);
        // Java: else
        } else {
            // Java: writer.writeXML("<mapping type='unknown'/>");
        }
        // Java: if (null != this.reverseMapping)
        if let Some(ref rm) = self.reverseMapping {
            // Java: this.reverseMapping.writeXML(writer);
            rm.writeXML(writer);
        }
    }

    // Java: CanvasTransform clone()
    pub fn clone_ct(&self) -> CanvasTransform {
        // Java: CanvasTransform ret = new CanvasTransform();
        let mut ret = CanvasTransform::CanvasTransform();
        // Java: ret.attributesMap = this.cloneAttributesMap();
        ret.attributesMap = self.cloneAttributesMap();
        // Java: if (null != this.forwardMapping)
        if let Some(ref fm) = self.forwardMapping {
            // Java: ret.forwardMapping = this.forwardMapping.clone();
            ret.forwardMapping = Some(fm.clone_mapping());
        }
        // Java: if (null != this.reverseMapping)
        if let Some(ref rm) = self.reverseMapping {
            // Java: ret.reverseMapping = this.reverseMapping.clone();
            ret.reverseMapping = Some(rm.clone_mapping());
        }
        // Java: return ret;
        ret
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
            let mut ret = HashMap::new();
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
