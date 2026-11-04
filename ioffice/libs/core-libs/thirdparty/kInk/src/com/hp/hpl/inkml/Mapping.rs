#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import java.util.ArrayList
// Java: import java.util.HashMap
use std::collections::HashMap;

use super::InkMLSerializer::InkMLWriter;
use super::URI::URI;

// Java: public enum MappingType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum MappingType {
    // Java: MappingType()

    identity,
    product,
    table,
    affine,
    mathml,
    unknown,
}

impl std::fmt::Display for MappingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MappingType::identity => write!(f, "identity"),
            MappingType::product => write!(f, "product"),
            MappingType::table => write!(f, "table"),
            MappingType::affine => write!(f, "affine"),
            MappingType::mathml => write!(f, "mathml"),
            MappingType::unknown => write!(f, "unknown"),
        }
    }
}

// Java: public class Mapping implements DefinitionElement, MappingElement, Cloneable
pub struct Mapping {
    // Java field: String id
    pub(crate) id: String,
    // Java field: MappingType type
    pub(crate) r#type: MappingType,
    // Java field: URI mappingRef
    pub(crate) mappingRef: Option<URI>,
    // Java field: ArrayList<MappingElement> children
    pub(crate) children: Vec<Mapping>,
}

// Java: public class Mapping implements DefinitionElement, MappingElement, Cloneable
impl Mapping {
    // Java: Mapping()
    pub fn Mapping() -> Self {
        // Java: super();
        // Java: this.id = "";
        // Java: this.type = Mapping.MappingType.unknown;
        // Java: this.children = new ArrayList();
        Self {
            id: String::new(),
            r#type: MappingType::unknown,
            mappingRef: None,
            children: Vec::new(),
        }
    }

    // Java: Mapping(String paramString)
    pub fn Mapping_type(paramString: &str) -> Self {
        // Java: super();
        let mut s = Self::Mapping();
        // Java: this.setId("");
        s.setId("");
        // Java: this.setType(paramString);
        s.setType(paramString);
        // Java: this.children = new ArrayList();
        s
    }

    // Java: Mapping(String paramString1, String paramString2)
    pub fn Mapping_id_type(paramString1: &str, paramString2: &str) -> Self {
        // Java: super();
        let mut s = Self::Mapping();
        // Java: this.id = paramString1;
        s.id = paramString1.to_string();
        // Java: this.setType(paramString2);
        s.setType(paramString2);
        // Java: this.children = new ArrayList();
        s
    }

    // Java: String getId()
    pub fn getId(&self) -> String {
        // Java: return this.id;
        self.id.clone()
    }

    // Java: void addChildElement(MappingElement paramMappingElement)
    pub fn addChildElement(&mut self, paramMappingElement: Option<Mapping>) {
        // Java: if (paramMappingElement != null)
        if let Some(me) = paramMappingElement {
            // Java: this.children.add(paramMappingElement);
            self.children.push(me);
        }
    }

    // Java: void addChildElements(ArrayList<MappingElement> paramArrayList)
    pub fn addChildElements(&mut self, paramArrayList: Vec<Mapping>) {
        // Java: for(MappingElement localMappingElement : paramArrayList)
        for me in paramArrayList {
            // Java: this.addChildElement(localMappingElement);
            self.children.push(me);
        }
    }

    // Java: ArrayList<MappingElement> getChildElements()
    pub fn getChildElements(&self) -> &Vec<Mapping> {
        // Java: return this.children;
        &self.children
    }

    // Java: String getInkElementType()
    pub fn getInkElementType(&self) -> String {
        // Java: return Mapping.class.getSimpleName();
        "Mapping".to_string()
    }

    // Java: static Mapping getDefaultMapping()
    pub fn getDefaultMapping() -> Mapping {
        // Java: return new Mapping();
        Mapping::Mapping()
    }

    // Java: boolean equals2(Mapping paramMapping)
    pub fn equals2(&self, paramMapping: Option<&Mapping>) -> bool {
        // Java: if (paramMapping == null)
        let pm = match paramMapping {
            None => {
                // Java: return false;
                return false;
            }
            Some(p) => p,
        };
        // Java: else if (this.type != paramMapping.getType())
        if self.r#type != pm.getType() {
            // Java: return false;
            return false;
        // Java: else if (this.children.size() == 0 && paramMapping.children.size() == 0)
        } else if self.children.is_empty() && pm.children.is_empty() {
            // Java: return true;
            true
        // Java: else
        } else {
            // Java: return this.children.size() != paramMapping.children.size() ? false : this.children.containsAll(paramMapping.children);
            self.children.len() == pm.children.len()
        }
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: StringBuffer localStringBuffer = new StringBuffer();
        let mut buf = String::new();
        // Java: localStringBuffer.append("<mapping");
        buf += "<mapping";
        // Java: if (!"".equals(this.id) && this.id != null)
        if !self.id.is_empty() {
            // Java: localStringBuffer.append(" xml:id=\"" + this.id + "\"");
            buf += &format!(" xml:id=\"{}\"", self.id);
        }
        // Java: if (this.type != Mapping.MappingType.unknown && this.type != null)
        if self.r#type != MappingType::unknown {
            // Java: localStringBuffer.append(" type=\"" + this.type.toString() + "\"");
            buf += &format!(" type=\"{}\"", self.r#type);
        }
        // Java: if (this.mappingRef != null && !"".equals(this.mappingRef.getURIString()))
        if let Some(ref mref) = self.mappingRef {
            if !mref.getURIString().is_empty() {
                // Java: localStringBuffer.append(" mappingRef=\"" + this.mappingRef.getURIString() + "\"");
                buf += &format!(" mappingRef=\"{}\"", mref.getURIString());
            }
        }
        // Java: if (this.type == Mapping.MappingType.identity)
        if self.r#type == MappingType::identity {
            // Java: localStringBuffer.append(" />");
            buf += " />";
        // Java: else
        } else {
            // Java: localStringBuffer.append(">");
            buf += ">";
            // Java: for(MappingElement localMappingElement : this.children)
            for child in &self.children {
                // Java: localStringBuffer.append(localMappingElement.toInkML());
                buf += &child.toInkML();
            }
            // Java: localStringBuffer.append("</mapping>");
            buf += "</mapping>";
        }
        // Java: return localStringBuffer.toString();
        buf
    }

    // Java: MappingType getType()
    pub fn getType(&self) -> MappingType {
        // Java: return this.type;
        self.r#type
    }

    // Java: void setType(String paramString)
    pub fn setType(&mut self, paramString: &str) {
        // Java: if (paramString != null)
        {
            // Java: if (paramString.equalsIgnoreCase("identity"))
            if paramString.eq_ignore_ascii_case("identity") {
                // Java: this.type = Mapping.MappingType.identity;
                self.r#type = MappingType::identity;
            // Java: else if (paramString.equalsIgnoreCase("product"))
            } else if paramString.eq_ignore_ascii_case("product") {
                // Java: this.type = Mapping.MappingType.product;
                self.r#type = MappingType::product;
            // Java: else if (paramString.equalsIgnoreCase("table"))
            } else if paramString.eq_ignore_ascii_case("table") {
                // Java: this.type = Mapping.MappingType.table;
                self.r#type = MappingType::table;
            // Java: else if (paramString.equalsIgnoreCase("affine"))
            } else if paramString.eq_ignore_ascii_case("affine") {
                // Java: this.type = Mapping.MappingType.affine;
                self.r#type = MappingType::affine;
            // Java: else if (paramString.equalsIgnoreCase("mathml"))
            } else if paramString.eq_ignore_ascii_case("mathml") {
                // Java: this.type = Mapping.MappingType.mathml;
                self.r#type = MappingType::mathml;
            // Java: else if (paramString.equalsIgnoreCase("unknown"))
            } else if paramString.eq_ignore_ascii_case("unknown") {
                // Java: this.type = Mapping.MappingType.unknown;
                self.r#type = MappingType::unknown;
            // Java: else
            } else {
                // Java: try
                {
                    // Java: this.type = Mapping.MappingType.unknown;
                    self.r#type = MappingType::unknown;
                    // Java: throw new InkMLException("Failed to set mapping type --- invalid type");
                }
                // Java: catch (InkMLException localInkMLException)
                {
                    // Java: localInkMLException.printStackTrace();
                }
            }
        }
    }

    // Java: void setId(String paramString)
    pub fn setId(&mut self, paramString: &str) {
        // Java: this.id = paramString;
        self.id = paramString.to_string();
    }

    // Java: void writeXML(InkMLWriter paramInkMLWriter)
    pub fn writeXML(&self, paramInkMLWriter: &mut InkMLWriter) {
        // Java: HashMap localHashMap = new HashMap();
        let mut localHashMap: HashMap<String, String> = HashMap::new();
        // Java: if (this.id != null && !"".equals(this.id))
        if !self.id.is_empty() {
            // Java: localHashMap.put("xml:id", this.id);
            localHashMap.insert("xml:id".to_string(), self.id.clone());
        }
        // Java: if (this.type != null && this.type != Mapping.MappingType.unknown)
        if self.r#type != MappingType::unknown {
            // Java: localHashMap.put("type", this.type.toString());
            localHashMap.insert("type".to_string(), self.r#type.to_string());
        }
        // Java: if (this.mappingRef != null && !"".equals(this.mappingRef.getURIString()))
        if let Some(ref mref) = self.mappingRef {
            if !mref.getURIString().is_empty() {
                // Java: localHashMap.put("mappingRef", this.mappingRef.getURIString());
                localHashMap.insert("mappingRef".to_string(), mref.getURIString().to_string());
            }
        }
        // Java: if (this.getChildElements().isEmpty())
        if self.children.is_empty() {
            // Java: paramInkMLWriter.writeEmptyStartTag("mapping", localHashMap);
        // Java: else
        } else {
            // Java: paramInkMLWriter.writeStartTag("mapping", localHashMap);
            // Java: paramInkMLWriter.incrementTagLevel();
            paramInkMLWriter.incrementTagLevel();
            // Java: for(MappingElement localMappingElement : this.children)
            for child in &self.children {
                // Java: localMappingElement.writeXML(paramInkMLWriter);
                child.writeXML(paramInkMLWriter);
            }
            // Java: paramInkMLWriter.decrementTagLevel();
            paramInkMLWriter.decrementTagLevel();
            // Java: paramInkMLWriter.writeEndTag("mapping");
            paramInkMLWriter.writeEndTag("mapping");
        }
    }

    // Java: URI getMappingRef()
    pub fn getMappingRef(&self) -> Option<&URI> {
        // Java: return this.mappingRef;
        self.mappingRef.as_ref()
    }

    // Java: void setMappingRef(URI paramURI)
    pub fn setMappingRef(&mut self, paramURI: URI) {
        // Java: this.mappingRef = paramURI;
        self.mappingRef = Some(paramURI);
    }

    // Java: void setMappingRef(String paramString)
    pub fn setMappingRef_str(&mut self, paramString: &str) {
        // Java: this.mappingRef = new URI(paramString);
        self.mappingRef = Some(URI::URI(paramString));
    }

    // Java: Mapping clone()
    pub fn clone_mapping(&self) -> Mapping {
        // Java: Mapping ret = new Mapping();
        let mut ret = Mapping::Mapping();
        // Java: ret.children = this.cloneChildren();
        ret.children = self.cloneChildren();
        // Java: if (null != this.id)
        {
            // Java: ret.id = new String(this.id);
            ret.id = self.id.clone();
        }
        // Java: if (null != this.mappingRef)
        if let Some(ref mref) = self.mappingRef {
            // Java: ret.mappingRef = new URI(this.mappingRef.getURIString());
            ret.mappingRef = Some(URI::URI(mref.getURIString()));
        }
        // Java: ret.type = this.type;
        ret.r#type = self.r#type;
        // Java: return ret;
        ret
    }

    // Java: private ArrayList<MappingElement> cloneChildren()
    fn cloneChildren(&self) -> Vec<Mapping> {
        // Java: if (null == this.children)
        if self.children.is_empty() {
            // Java: return null;
            return Vec::new();
        // Java: else
        } else {
            // Java: ArrayList<MappingElement> ret = new ArrayList();
            let mut ret = Vec::new();
            // Java: int size = this.children.size();
            let size = self.children.len();
            // Java: for(int i = 0; i < size; ++i)
            for i in 0..size {
                // Java: MappingElement e = (MappingElement)this.children.get(i);
                let e = &self.children[i];
                // Java: if (e instanceof Mapping)
                {
                    // Java: Mapping mapping = (Mapping)e;
                    // Java: ret.add(mapping.clone());
                    ret.push(e.clone_mapping());
                }
            }
            // Java: return ret;
            ret
        }
    }
}
