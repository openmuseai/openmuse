#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackageRelationship
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.xwpf.PictureData
// Java: import cn.wps.io.xwpf.usermodel.XWPFPictureData

// Java: public class GVmlPictureData extends XWPFPictureData implements PictureData
pub struct GVmlPictureData {
    // Java field: String TAG
    _private: (),
}

// Java: public class GVmlPictureData extends XWPFPictureData implements PictureData
impl GVmlPictureData {
    // Java: GVmlPictureData()
    pub fn new() -> Self {
        // Java: super();
        GVmlPictureData { _private: () }
    }

    // Java: GVmlPictureData(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn new_with_parts(_parent: Box<dyn std::any::Any + Send + Sync>, _part: Box<dyn std::any::Any + Send + Sync>, _rel: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(parent, part, rel);
        GVmlPictureData { _private: () }
    }
}
