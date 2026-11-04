#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackageRelationship
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.xwpf.usermodel.XWPFInkData

// Java: public class GVmlInkData extends XWPFInkData
pub struct GVmlInkData {
    _private: (),
}

// Java: public class GVmlInkData extends XWPFInkData
impl GVmlInkData {
    // Java: GVmlInkData(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn new_with_parts(_parent: Box<dyn std::any::Any + Send + Sync>, _part: Box<dyn std::any::Any + Send + Sync>, _rel: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(parent, part, rel);
        GVmlInkData { _private: () }
    }

    // Java: GVmlInkData()
    pub fn new() -> Self {
        // Java: super();
        GVmlInkData { _private: () }
    }
}
