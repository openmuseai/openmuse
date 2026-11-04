#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.dom.Element
// Java: import cn.wps.io.dom.Namespace
// Java: import cn.wps.io.dom.tree.DefaultDocument
// Java: import cn.wps.io.opc.CustomPackageProperties
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.internal.CustomPackagePropertiesPart
// Java: import cn.wps.io.opc.internal.PartMarshaller
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import java.io.OutputStream
// Java: import java.text.SimpleDateFormat
// Java: import java.util.Date
// Java: import java.util.List
// Java: import java.util.TimeZone

use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::internal::CustomPackagePropertiesPart::CustomPackagePropertiesPart;

// Java field: Namespace namespaceCustomProperties
// Java field: Namespace namespaceVT

// Java field: String KEYWORD_PROPERTIE
pub(crate) const KEYWORD_PROPERTIE: &str = "property";
// Java field: String KEYWORD_ATTRIBUTE_FMTID
pub(crate) const KEYWORD_ATTRIBUTE_FMTID: &str = "fmtid";
// Java field: String KEYWORD_ATTRIBUTE_PID
pub(crate) const KEYWORD_ATTRIBUTE_PID: &str = "pid";
// Java field: String KEYWORD_ATTRIBUTE_NAME
pub(crate) const KEYWORD_ATTRIBUTE_NAME: &str = "name";
// Java field: String KEYWORD_ATTRIBUTE_LINKTARGET
pub(crate) const KEYWORD_ATTRIBUTE_LINKTARGET: &str = "linkTarget";
// Java field: String KEYWORD_ATTRIBUTE_FMTID_ID
pub(crate) const KEYWORD_ATTRIBUTE_FMTID_ID: &str = "{D5CDD505-2E9C-101B-9397-08002B2CF9AE}";
// Java field: String KEYWORD_LPWSTR
pub(crate) const KEYWORD_LPWSTR: &str = "lpwstr";
// Java field: String KEYWORD_I4
pub(crate) const KEYWORD_I4: &str = "i4";
// Java field: String KEYWORD_R8
pub(crate) const KEYWORD_R8: &str = "r8";
// Java field: String KEYWORD_BOOL
pub(crate) const KEYWORD_BOOL: &str = "bool";
// Java field: String KEYWORD_FILETIME
pub(crate) const KEYWORD_FILETIME: &str = "filetime";

// Java: public class CustomPackagePropertiesMarshaller implements PartMarshaller
pub struct CustomPackagePropertiesMarshaller {
    // Java field: int PID_START
    pub(crate) PID_START: i32,
    // Java field: CustomPackagePropertiesPart propsPart
    pub(crate) propsPart: Option<CustomPackagePropertiesPart>,
    // Java field: Document xmlDoc
    pub(crate) xmlDoc: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class CustomPackagePropertiesMarshaller implements PartMarshaller
impl CustomPackagePropertiesMarshaller {
    // Java: CustomPackagePropertiesMarshaller()
    pub fn new() -> Self {
        // Java: super();
        CustomPackagePropertiesMarshaller {
            PID_START: 2,
            propsPart: None,
            xmlDoc: None,
        }
    }

    // Java: boolean marshall(PackagePart part, OutputStream out)
    pub fn marshall(&mut self, _part: &PackagePart, _out: &mut dyn std::io::Write) -> bool {
        // Java: if (!(part instanceof CustomPackagePropertiesPart))
        // Java: throw new IllegalArgumentException("'part' must be a CustomPackagePropertiesPart instance.");
        // Java: else
        // Java: this.propsPart = (CustomPackagePropertiesPart)part;
        // Java: this.xmlDoc = new DefaultDocument();
        // Java: Element root = this.xmlDoc.addElement(namespaceCustomProperties.getURI(), namespaceCustomProperties.getPrefix(), "Properties");
        // Java: root.addNamespace("vt", "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes");
        // Java: this.addPropertyArray(root);
        // Java: return true;
        todo!()
    }

    // Java: void finish()
    pub fn finish(&self) {
        // Java: if (null != this.xmlDoc)
        // Java: Element root = this.xmlDoc.getRootElement();
        // Java: if (null != root)
        // Java: root.detachRecursively();
        todo!()
    }

    // Java: void addPropertyArray(Element root)
    fn addPropertyArray(&self, _root: &dyn std::any::Any) {
        todo!()
    }

    // Java: void addProperty(Element root, CustomPackageProperties.Property property, int pid)
    fn addProperty(&self, _root: &dyn std::any::Any, _property: &dyn std::any::Any, _pid: i32) {
        todo!()
    }

    // Java: void addChoice(Element elem, CustomPackageProperties.Property property)
    fn addChoice(&self, _elem: &dyn std::any::Any, _property: &dyn std::any::Any) {
        todo!()
    }

    // Java: void addLpwstr(Element rootElem, String value)
    fn addLpwstr(&self, _root_elem: &dyn std::any::Any, _value: &str) {
        todo!()
    }

    // Java: void addBool(Element rootElem, boolean value)
    fn addBool(&self, _root_elem: &dyn std::any::Any, _value: bool) {
        todo!()
    }

    // Java: void addI4(Element rootElem, int value)
    fn addI4(&self, _root_elem: &dyn std::any::Any, _value: i32) {
        todo!()
    }

    // Java: void addR8(Element rootElem, double value)
    fn addR8(&self, _root_elem: &dyn std::any::Any, _value: f64) {
        todo!()
    }

    // Java: void addFileTime(Element rootElem, Date value)
    fn addFileTime(&self, _root_elem: &dyn std::any::Any, _value: &str) {
        todo!()
    }

    // Java: static String getDateValue(Date date)
    fn getDateValue(_date: &str) -> String {
        todo!()
    }
}
