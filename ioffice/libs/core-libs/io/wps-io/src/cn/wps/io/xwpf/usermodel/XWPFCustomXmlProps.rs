#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackageRelationship
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart

// Java: public class XWPFCustomXmlProps extends POIXMLDocumentPart
pub struct XWPFCustomXmlProps {
    _private: (),
}

// Java: public class XWPFCustomXmlProps extends POIXMLDocumentPart
impl XWPFCustomXmlProps {
    // Java: XWPFCustomXmlProps()
    pub fn new() -> Self {
        // Java: super();
        XWPFCustomXmlProps { _private: () }
    }

    // Java: XWPFCustomXmlProps(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn new_with_parts(_parent: Box<dyn std::any::Any + Send + Sync>, _part: Box<dyn std::any::Any + Send + Sync>, _rel: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(parent, part, rel);
        XWPFCustomXmlProps { _private: () }
    }

    // Java: static XWPFCustomXmlProps startPart(XWPFChartData chartData, XWPFRelation descriptor, int idx, String rId)
    pub fn startPart(_chart_data: Box<dyn std::any::Any + Send + Sync>, _descriptor: Box<dyn std::any::Any + Send + Sync>, _idx: i32, _r_id: &str) -> Self {
        // Java: IOAssert.assertNotNull("chartData should not be null.", chartData);
        // Java: POIXMLDocumentPart part = POIXMLDocumentPart.startPart(chartData, descriptor, idx, rId, XWPFFactory.getInstance());
        // Java: IOAssert.assertTrue("part should be instanceof XWPFChartSrc", part instanceof XWPFCustomXmlProps);
        // Java: return (XWPFCustomXmlProps)part;
        todo!()
    }
}
