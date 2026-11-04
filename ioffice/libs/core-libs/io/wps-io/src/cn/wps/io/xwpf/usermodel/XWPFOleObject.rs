#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackageRelationship
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.opc.model.POIXMLException
// Java: import cn.wps.io.xwpf.ChartSrc
// Java: import cn.wps.util.IOUtils
// Java: import java.io.IOException
// Java: import java.util.UUID

// Java: public class XWPFOleObject extends POIXMLDocumentPart implements ChartSrc
pub struct XWPFOleObject {
    _private: (),
}

// Java: public class XWPFOleObject extends POIXMLDocumentPart implements ChartSrc
impl XWPFOleObject {
    // Java: XWPFOleObject()
    pub fn new() -> Self {
        // Java: super();
        XWPFOleObject { _private: () }
    }

    // Java: XWPFOleObject(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn new_with_parts(_parent: Box<dyn std::any::Any + Send + Sync>, _part: Box<dyn std::any::Any + Send + Sync>, _rel: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(parent, part, rel);
        XWPFOleObject { _private: () }
    }

    // Java: static XWPFOleObject startPart(XWPFChartData chartData, XWPFRelation descriptor, int idx)
    pub fn startPart(_chart_data: Box<dyn std::any::Any + Send + Sync>, _descriptor: Box<dyn std::any::Any + Send + Sync>, _idx: i32) -> Self {
        // Java: IOAssert.assertNotNull("chartData should not be null.", chartData);
        // Java: POIXMLDocumentPart part = POIXMLDocumentPart.startPart(chartData, descriptor, idx, XWPFFactory.getInstance());
        // Java: IOAssert.assertTrue("part should be instanceof XWPFChartSrc", part instanceof XWPFOleObject);
        // Java: return (XWPFOleObject)part;
        todo!()
    }

    // Java: static XWPFOleObject startPart(XWPFChartData chartData, XWPFRelation descriptor, int idx, String rId)
    pub fn startPart_with_rid(_chart_data: Box<dyn std::any::Any + Send + Sync>, _descriptor: Box<dyn std::any::Any + Send + Sync>, _idx: i32, _r_id: &str) -> Self {
        // Java: IOAssert.assertNotNull("chartData should not be null.", chartData);
        // Java: POIXMLDocumentPart part = POIXMLDocumentPart.startPart(chartData, descriptor, idx, rId, XWPFFactory.getInstance());
        // Java: IOAssert.assertTrue("part should be instanceof XWPFOleObject", part instanceof XWPFOleObject);
        // Java: return (XWPFOleObject)part;
        todo!()
    }

    // Java: byte[] getData()
    pub fn getData(&self) -> Vec<u8> {
        // Java: try
        // Java: return IOUtils.toByteArray(this.getPackagePart().getInputStream());
        // Java: catch (IOException e)
        // Java: throw new POIXMLException(e);
        todo!()
    }

    // Java: String getUniqueFileName()
    pub fn getUniqueFileName(&self) -> String {
        // Java: return UUID.randomUUID().toString() + "." + this.getPackagePart().getPartName().getExtension();
        todo!()
    }
}
