#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackageRelationship

use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::PackageRelationship::PackageRelationship;
use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;

// Java: public class POIXMLFactory
pub trait POIXMLFactory {
    // Java: POIXMLFactory()
    // Java: super();
    // Java: abstract POIXMLDocumentPart createDocumentPart(POIXMLDocumentPart var1, PackageRelationship var2, PackagePart var3)
    fn createDocumentPart(&self, _var1: &POIXMLDocumentPart, _var2: &PackageRelationship, _var3: &PackagePart) -> Option<POIXMLDocumentPart>;

    // Java: abstract POIXMLDocumentPart newDocumentPart(POIXMLRelation var1)
    fn newDocumentPart(&self, _var1: &dyn std::any::Any) -> Option<POIXMLDocumentPart>;
}
