#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;
use super::AttributesImpl::AttributesImpl;
use super::XmlStringBuilder::XmlStringBuilder;

// Java field: int INVALID
const INVALID: i32 = -1;

// Java: public class Token
pub struct Token {
    // Java field: Attributes1 attributes
    pub(crate) attributes: AttributesImpl,
    // Java field: int attrNameSpacingId
    attrNameSpacingId: i32,
    // Java field: HashMap<Integer, String> nameSpacingMap
    // (owned by attributes.nameSpacingMap — shared as in Java)
    // Java field: HashMap<Integer, String> mHash2Prefix
    // (owned by attributes.mHash2Prefix — shared as in Java)
    // Java field: String localName
    pub(crate) localName: Option<String>,
    // Java field: String qualifiedName
    pub(crate) qualifiedName: Option<String>,
    // Java field: int elementId
    pub(crate) elementId: i32,
    // Java field: int elementNameSpacing
    pub(crate) elementNameSpacing: i32,
    // Java field: XmlStringBuilder attrName
    attrName: XmlStringBuilder,
    // Java field: boolean isNameSpacing
    isNameSpacing: bool,
    // Java field: int shapeLevel
    shapeLevel: i32,
    // Java field: boolean isShape
    isShape: bool,
    // Java field: boolean isIgnore
    isIgnore: bool,
    // Java field: int ignoreLevel
    ignoreLevel: i32,
    // Java field: int ignoreElementId
    ignoreElementId: i32,
    // Java field: boolean isIgnoreAttr
    isIgnoreAttr: bool,
}

// Java: public class Token
impl Token {
    // Java: Token()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.attributes = new AttributesImpl(this.nameSpacingMap, this.mHash2Prefix);
        Token {
            attributes: AttributesImpl::new_with_maps(
                HashMap::with_capacity(20),
                Some(HashMap::with_capacity(20)),
            ),
            attrNameSpacingId: INVALID,
            localName: None,
            qualifiedName: None,
            elementId: INVALID,
            elementNameSpacing: 0,
            attrName: XmlStringBuilder::new(),
            isNameSpacing: false,
            shapeLevel: 0,
            isShape: false,
            isIgnore: false,
            ignoreLevel: 0,
            ignoreElementId: INVALID,
            isIgnoreAttr: false,
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.attributes.clear();
        self.attributes.clear();
        // Java: this.elementId = -1;
        self.elementId = -1;
        // Java: this.localName = null;
        self.localName = None;
        // Java: this.attrName.clear();
        self.attrName.clear();
    }

    // Java: void setIgnoreElementId(int elementId)
    pub fn setIgnoreElementId(&mut self, element_id: i32) {
        // Java: this.ignoreElementId = elementId;
        self.ignoreElementId = element_id;
    }

    // Java: void addElementName(XmlStringBuilder tagName, int mode)
    pub fn addElementName(&mut self, tag_name: &mut XmlStringBuilder, mode: i32) {
        // Java: this.elementId = tagName.getNameHashCode();
        self.elementId = tag_name.getNameHashCode();
        // Java: if (!this.isIgnore(this.elementId))
        if !self.isIgnore_check_mut(self.elementId) {
            // Java: if (tagName.prefixLength() > 0)
            if tag_name.prefixLength() > 0 {
                // Java: this.elementNameSpacing = tagName.getPrefixHashCode();
                self.elementNameSpacing = tag_name.getPrefixHashCode();
                // Java: this.elementId = convertElementId(this.elementNameSpacing, this.elementId);
                self.elementId = Self::convertElementId(self.elementNameSpacing, self.elementId);
            }
            // Java: this.isShape = this.isShape();
            self.isShape = self.isShape_check();
            // Java: if (this.isShape)
            if self.isShape {
                // Java: if (tagName.prefixLength() > 0)
                if tag_name.prefixLength() > 0 {
                    // Java: this.qualifiedName = tagName.toString();
                    self.qualifiedName = Some(tag_name.to_string());
                    // Java: this.localName = tagName.getName();
                    self.localName = Some(tag_name.getName());
                }
                // Java: else
                else {
                    // Java: this.qualifiedName = tagName.toString();
                    self.qualifiedName = Some(tag_name.to_string());
                    // Java: this.localName = this.qualifiedName;
                    self.localName = self.qualifiedName.clone();
                }
            }
            // Java: else if (1 == mode)
            else if mode == 1 {
                // Java: this.qualifiedName = tagName.toString();
                self.qualifiedName = Some(tag_name.to_string());
            }
            // Java: else if (3 == mode)
            else if mode == 3 {
                // Java: if (tagName.prefixLength() > 0)
                if tag_name.prefixLength() > 0 {
                    // Java: this.qualifiedName = tagName.toString();
                    self.qualifiedName = Some(tag_name.to_string());
                    // Java: this.localName = tagName.getName();
                    self.localName = Some(tag_name.getName());
                }
                // Java: else
                else {
                    // Java: this.qualifiedName = tagName.toString();
                    self.qualifiedName = Some(tag_name.to_string());
                    // Java: this.localName = this.qualifiedName;
                    self.localName = self.qualifiedName.clone();
                }
            }
            // rust: `SAXReader` 通路需要元素名（Java 侧由 JDK `SAXParser` 提供，本 crate 的 `XmlParse`
            //       只在 mode 1/3 或 shape 下记录）⇒ 任何模式下都补齐 qualifiedName/localName（登记 §1.7.1 ①）
            {
                let full = tag_name.to_string();
                self.localName = Some(match full.find(':') {
                    Some(i) => full[i + 1..].to_string(),
                    None => full.clone(),
                });
                self.qualifiedName = Some(full);
            }
            // Java: tagName.clear();
            tag_name.clear();
        }
    }

    // Java: void addAttrName(XmlStringBuilder tagName, int mode)
    pub fn addAttrName(&mut self, tag_name: &mut XmlStringBuilder, mode: i32) {
        // Java: this.isIgnoreAttr = this.isIgnoreAttribute(tagName);
        self.isIgnoreAttr = self.isIgnoreAttribute(tag_name);
        // Java: if (!this.isIgnoreAttr)
        if !self.isIgnoreAttr {
            // Java: if (tagName.prefixLength() > 0)
            if tag_name.prefixLength() > 0 {
                // Java: if (tagName.isPrefixEqual("xmlns"))
                if tag_name.isPrefixEqual("xmlns") {
                    // Java: this.isNameSpacing = true;
                    self.isNameSpacing = true;
                    // Java: this.attrNameSpacingId = tagName.getNameHashCode();
                    self.attrNameSpacingId = tag_name.getNameHashCode();
                    // Java: if (1 == mode)
                    if mode == 1 {
                        // Java: this.mHash2Prefix.put(this.attrNameSpacingId, tagName.toString());
                        self.attributes
                            .mHash2Prefix
                            .as_mut()
                            .unwrap()
                            .insert(self.attrNameSpacingId, tag_name.to_string());
                    }
                }
                // Java: else
                else {
                    // Java: this.attributes.addAttriteName(tagName);
                    self.attributes.addAttriteName(&*tag_name);
                }
            }
            // Java: else
            else {
                // Java: this.attributes.addAttriteName(tagName);
                self.attributes.addAttriteName(&*tag_name);
            }
        }
    }

    // Java: boolean isIgnoreAttribute(XmlStringBuilder tagName)
    fn isIgnoreAttribute(&mut self, _tag_name: &XmlStringBuilder) -> bool {
        // Java: return this.isIgnoreElementAttribute(this.elementId) || this.isIgnore(this.elementId);
        self.isIgnoreElementAttribute(self.elementId) || self.isIgnore_check_mut(self.elementId)
    }

    // Java: void addAttribute(XmlStringBuilder tagName)
    pub fn addAttribute(&mut self, tag_name: &XmlStringBuilder) {
        // Java: if (!this.isIgnoreAttr)
        if !self.isIgnoreAttr {
            // Java: if (this.isNameSpacing)
            if self.isNameSpacing {
                // Java: this.nameSpacingMap.put(this.attrNameSpacingId, tagName.toString());
                self.attributes
                    .nameSpacingMap
                    .insert(self.attrNameSpacingId, tag_name.to_string());
                // Java: this.isNameSpacing = false;
                self.isNameSpacing = false;
            }
            // Java: else
            else {
                // Java: this.attributes.addAttriteValue(tagName);
                self.attributes.addAttriteValue(&*tag_name);
            }
            // Java: this.attrNameSpacingId = -1;
            self.attrNameSpacingId = -1;
        }
    }

    // Java: boolean isIgnoreElementAttribute(int elementId)
    fn isIgnoreElementAttribute(&self, element_id: i32) -> bool {
        // Java: return elementId == 114;
        element_id == 114
    }

    // Java: boolean isShape()
    fn isShape_check(&self) -> bool {
        // Java: if (this.shapeLevel > 0)
        if self.shapeLevel > 0 {
            // Java: return true;
            return true;
        }
        // Java: else
        // Java: switch (this.elementId)
        match self.elementId {
            // Java: case -1332194002:
            // Java: case -1023368385:
            // Java: case 3440682:
            // Java: case 1590178495:
            // Java: case 1913009182:
            -1332194002 | -1023368385 | 3440682 | 1590178495 | 1913009182 => {
                // Java: return true;
                true
            }
            // Java: default:
            _ => {
                // Java: return isW14(this.elementNameSpacing);
                Self::isW14(self.elementNameSpacing)
            }
        }
    }

    // Java: boolean isIgnore(int elementId)
    fn isIgnore_check_mut(&mut self, element_id: i32) -> bool {
        // Java: if (this.ignoreLevel > 0)
        if self.ignoreLevel > 0 {
            // Java: return true;
            return true;
        }
        // Java: else if (elementId == -1)
        else if element_id == -1 {
            // Java: return this.isIgnore;
            return self.isIgnore;
        }
        // Java: else
        // Java: if (elementId == this.ignoreElementId)
        if element_id == self.ignoreElementId {
            // Java: this.isIgnore = true;
            self.isIgnore = true;
        }
        // Java: else
        else {
            // Java: this.isIgnore = false;
            self.isIgnore = false;
        }
        // Java: return this.isIgnore;
        self.isIgnore
    }

    // Java: static boolean isW14(int nameSpacing)
    fn isW14(name_spacing: i32) -> bool {
        // Java: int w14 = 115930;
        // Java: switch (nameSpacing)
        match name_spacing {
            // Java: case 115930:
            115930 => {
                // Java: return true;
                true
            }
            // Java: default:
            _ => {
                // Java: return false;
                false
            }
        }
    }

    // Java: static int convertElementId(int nameSpacing, int elementId)
    fn convertElementId(name_spacing: i32, element_id: i32) -> i32 {
        // Java: int w14 = 115930;
        // Java: int m = 109;
        // Java: switch (nameSpacing)
        match name_spacing {
            // Java: case 109:
            109 => {
                // Java: return convertMElementId(elementId);
                Self::convertMElementId(element_id)
            }
            // Java: case 115930:
            115930 => {
                // Java: return convertW14ElementId(elementId);
                Self::convertW14ElementId(element_id)
            }
            // Java: default:
            _ => {
                // Java: return elementId;
                element_id
            }
        }
    }

    // Java: static int convertMElementId(int elementId)
    fn convertMElementId(element_id: i32) -> i32 {
        // Java: switch (elementId)
        match element_id {
            // Java: case 3385:
            3385 => {
                // Java: return 43;
                43
            }
            // Java: case 112148:
            112148 => {
                // Java: return 44;
                44
            }
            // Java: default:
            _ => {
                // Java: return elementId;
                element_id
            }
        }
    }

    // Java: static int convertW14ElementId(int elementId)
    fn convertW14ElementId(element_id: i32) -> i32 {
        // Java: switch (elementId)
        match element_id {
            // Java: case -2001340054:
            -2001340054_i32 => {
                // Java: return 19;
                19
            }
            // Java: case -1946182059:
            -1946182059_i32 => {
                // Java: return 14;
                14
            }
            // Java: case -1004056496:
            -1004056496_i32 => {
                // Java: return 15;
                15
            }
            // Java: case -520163875:
            -520163875_i32 => {
                // Java: return 20;
                20
            }
            // Java: case -309119999:
            -309119999_i32 => {
                // Java: return 17;
                17
            }
            // Java: case 486258120:
            486258120 => {
                // Java: return 18;
                18
            }
            // Java: case 1002557153:
            1002557153 => {
                // Java: return 21;
                21
            }
            // Java: case 1914645533:
            1914645533 => {
                // Java: return 16;
                16
            }
            // Java: default:
            _ => {
                // Java: return elementId;
                element_id
            }
        }
    }

    // Java: String getElementNameSpacing()
    pub fn getElementNameSpacing(&self) -> Option<&str> {
        // Java: switch (this.elementNameSpacing)
        match self.elementNameSpacing {
            // Java: case -1:
            -1 => {
                // Java: return null;
                None
            }
            // Java: default:
            _ => {
                // Java: return (String)this.nameSpacingMap.get(this.elementNameSpacing);
                self.attributes.nameSpacingMap.get(&self.elementNameSpacing).map(|s| s.as_str())
            }
        }
    }

    // Java: void startElement()
    pub fn startElement(&mut self) {
        // Java: if (this.isShape)
        if self.isShape {
            // Java: ++this.shapeLevel;
            self.shapeLevel += 1;
        }
        // Java: else if (this.isIgnore)
        else if self.isIgnore {
            // Java: ++this.ignoreLevel;
            self.ignoreLevel += 1;
        }
    }

    // Java: void endElement()
    pub fn endElement(&mut self) {
        // Java: if (this.isShape)
        if self.isShape {
            // Java: --this.shapeLevel;
            self.shapeLevel -= 1;
        }
        // Java: else if (this.isIgnore)
        else if self.isIgnore {
            // Java: --this.ignoreLevel;
            self.ignoreLevel -= 1;
        }
    }

    // Java: void addNamespace(String prefix, String namespace)
    pub fn addNamespace(&mut self, prefix: &str, namespace: &str) {
        // Java: if (this.nameSpacingMap != null)
        // Java: this.nameSpacingMap.put(prefix.hashCode(), namespace);
        let hash = XmlStringBuilder::hashcode_char(&prefix.chars().collect::<Vec<_>>(), 0, prefix.len());
        self.attributes.nameSpacingMap.insert(hash, namespace.to_string());
    }
}
