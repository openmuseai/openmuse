#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackageRelationship
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.xwpf.usermodel.XWPFChartData

use crate::io::xwpf::usermodel::XWPFChartData::XWPFChartData;

// Java: public class GVmlChartData extends XWPFChartData
pub struct GVmlChartData {
    pub(crate) base: XWPFChartData,
}

// Java: public class GVmlChartData extends XWPFChartData
impl GVmlChartData {
    // Java: GVmlChartData()
    pub fn new() -> Self {
        // Java: super();
        GVmlChartData {
            base: XWPFChartData::new(),
        }
    }

    // Java: GVmlChartData(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn new_with_parts(parent: Box<dyn std::any::Any + Send + Sync>, part: Box<dyn std::any::Any + Send + Sync>, rel: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(parent, part, rel);
        GVmlChartData {
            base: XWPFChartData::new_with_parts(parent, part, rel),
        }
    }
}
