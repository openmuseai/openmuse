#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackageRelationship
// Java: import cn.wps.io.opc.PackageRelationshipCollection
// Java: import cn.wps.io.opc.TargetMode
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.opc.model.POIXMLException
// Java: import cn.wps.io.util.URI
// Java: import cn.wps.io.xwpf.ChartData
// Java: import cn.wps.io.xwpf.ChartSrc
// Java: import cn.wps.io.xwpf.DmlRelation
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
// Java: import cn.wps.util.IOUtils
// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.UUID

// Java: public class XWPFChartData extends POIXMLDocumentPart implements ChartData
pub struct XWPFChartData {
    // Java field: String TAG
    // Java field: String srcId
    srcId: Option<String>,
    // Java field: URI target
    target: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: TargetMode targetMode
    targetMode: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: String relationshipType
    relationshipType: Option<String>,
    // Java field: String drawingXmlRelationIds
    drawingXmlRelationIds: Option<String>,
    // Java field: String chartStyleRelationIds
    chartStyleRelationIds: Option<String>,
    // Java field: String colorStyleRelationIds
    colorStyleRelationIds: Option<String>,
    // Java field: String themeOverrideRelationIds
    themeOverrideRelationIds: Option<String>,
    // Java field: ArrayList<String> chartImageIds
    chartImageIds: Vec<String>,
}

// Java: public class XWPFChartData extends POIXMLDocumentPart implements ChartData
impl XWPFChartData {
    // Java: XWPFChartData()
    pub fn new() -> Self {
        // Java: super();
        XWPFChartData {
            srcId: None,
            target: None,
            targetMode: None,
            relationshipType: None,
            drawingXmlRelationIds: None,
            chartStyleRelationIds: None,
            colorStyleRelationIds: None,
            themeOverrideRelationIds: None,
            chartImageIds: Vec::new(),
        }
    }

    // Java: XWPFChartData(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn new_with_parts(_parent: Box<dyn std::any::Any + Send + Sync>, _part: Box<dyn std::any::Any + Send + Sync>, _rel: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(parent, part, rel);
        // Java: this.chartImageIds = new ArrayList();
        // Java: boolean isFindChartSrc = false;
        // Java: try
        // Java: PackageRelationshipCollection relations = this.getPackagePart().getRelationships();
        // Java: int i = 0;
        // Java: for(int size = relations.size(); i < size; ++i)
        // Java: PackageRelationship relation = relations.getRelationship(i);
        // Java: String relationshipType = relation.getRelationshipType();
        // Java: if (isContainWorkbook(relationshipType) && !isFindChartSrc)
        // Java: this.srcId = relation.getId();
        // Java: this.target = relation.getTargetURI();
        // Java: this.targetMode = relation.getTargetMode();
        // Java: this.relationshipType = relationshipType;
        // Java: isFindChartSrc = true;
        // Java: else if (isDrawingXmlRelation(relationshipType))
        // Java: this.drawingXmlRelationIds = relation.getId();
        // Java: else if (this.isChartImage(relationshipType))
        // Java: this.chartImageIds.add(relation.getId());
        // Java: else if (this.isStyleChart(relationshipType))
        // Java: this.chartStyleRelationIds = relation.getId();
        // Java: else if (this.isColorChart(relationshipType))
        // Java: this.colorStyleRelationIds = relation.getId();
        // Java: else if (this.isThemeOverride(relationshipType))
        // Java: this.themeOverrideRelationIds = relation.getId();
        // Java: IOAssert.logNotNull("Cannot find embedding workbook for this chart.xml in docx.", this.srcId);
        // Java: catch (InvalidFormatException e)
        // Java: IOLog.e(TAG, "InvalidFormatException:", e);
        todo!()
    }

    // Java: static boolean isContainWorkbook(String relType)
    fn isContainWorkbook(_rel_type: &str) -> bool {
        // Java: if (null == relType)
        // Java: return false;
        // Java: else
        // Java: return relType.equals(XWPFRelation.CHART_SRC.getRelation()) || relType.equals(XWPFRelationPurl.CHART_SRC.getRelation()) || relType.equals(XWPFRelation.OLE_OBJECT.getRelation()) || relType.equals(XWPFRelationPurl.CHART_EXTERNAL_SRC.getRelation());
        todo!()
    }

    // Java: boolean isChartImage(String relationType)
    fn isChartImage(&self, _relation_type: &str) -> bool {
        // Java: return XWPFRelation.IMAGE_JPEG.getRelation().equals(relationType);
        todo!()
    }

    // Java: static boolean isDrawingXmlRelation(String relationType)
    fn isDrawingXmlRelation(_relation_type: &str) -> bool {
        // Java: return XWPFRelation.CHART_EXTERNAL_XML.getRelation().equals(relationType) || XWPFRelationPurl.CHART_EXTERNAL_XML.getRelation().equals(relationType);
        todo!()
    }

    // Java: boolean isStyleChart(String relationType)
    fn isStyleChart(&self, _relation_type: &str) -> bool {
        // Java: return XWPFRelation.CHART_STYLE.getRelation().equals(relationType);
        todo!()
    }

    // Java: boolean isColorChart(String relationType)
    fn isColorChart(&self, _relation_type: &str) -> bool {
        // Java: return XWPFRelation.CHART_COLOR.getRelation().equals(relationType);
        todo!()
    }

    // Java: boolean isThemeOverride(String relationType)
    fn isThemeOverride(&self, _relation_type: &str) -> bool {
        // Java: return XWPFRelation.THEME_OVERRIDE.getRelation().equals(relationType);
        todo!()
    }

    // Java: static XWPFChartData startPart(POIXMLDocumentPart parent, int idx)
    pub fn startPart(_parent: Box<dyn std::any::Any + Send + Sync>, _idx: i32) -> Self {
        // Java: IOAssert.assertNotNull("parent should not be null.", parent);
        // Java: POIXMLDocumentPart part = POIXMLDocumentPart.startPart(parent, DmlRelation.chart(), idx, XWPFFactory.getInstance());
        // Java: IOAssert.assertTrue("part should be instanceof XWPFChartData", part instanceof XWPFChartData);
        // Java: return (XWPFChartData)part;
        todo!()
    }

    // Java: byte[] getData()
    pub fn getData(&self) -> Option<Vec<u8>> {
        // Java: try
        // Java: return IOUtils.toByteArray(this.getPackagePart().getInputStream());
        // Java: catch (IOException e)
        // Java: IOLog.e(TAG, "IOException:", e);
        // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
        // Java: throw new POIXMLException(e);
        // Java: else
        // Java: return null;
        todo!()
    }

    // Java: ChartSrc getSrc()
    pub fn getSrc(&self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: if (null != this.srcId && (null == this.targetMode || TargetMode.INTERNAL == this.targetMode))
        // Java: POIXMLDocumentPart relatedPart = this.getRelationById(this.srcId);
        // Java: return !(relatedPart instanceof ChartSrc) ? null : (ChartSrc)relatedPart;
        // Java: else
        // Java: return null;
        todo!()
    }

    // Java: String getUniqueFileName()
    pub fn getUniqueFileName(&self) -> String {
        // Java: return UUID.randomUUID().toString() + "." + this.getPackagePart().getPartName().getExtension();
        todo!()
    }

    // Java: String getSrcRelId()
    pub fn getSrcRelId(&self) -> Option<&str> {
        // Java: return this.srcId;
        self.srcId.as_deref()
    }

    // Java: TargetMode getTargetMode()
    pub fn getTargetMode(&self) -> &Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: return this.targetMode;
        &self.targetMode
    }

    // Java: URI getTarget()
    pub fn getTarget(&self) -> &Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: return this.target;
        &self.target
    }

    // Java: String getRelationshipType()
    pub fn getRelationshipType(&self) -> Option<&str> {
        // Java: return this.relationshipType;
        self.relationshipType.as_deref()
    }

    // Java: String getDrawingXmlPartID()
    pub fn getDrawingXmlPartID(&self) -> Option<&str> {
        // Java: return this.drawingXmlRelationIds;
        self.drawingXmlRelationIds.as_deref()
    }

    // Java: ArrayList<String> getChartImageIds()
    pub fn getChartImageIds(&self) -> &Vec<String> {
        // Java: return this.chartImageIds;
        &self.chartImageIds
    }

    // Java: POIXMLDocumentPart getPart(String id)
    pub fn getPart(&self, _id: &str) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: return super.getRelationById(id);
        todo!()
    }

    // Java: String getChartStyleXmlPartID()
    pub fn getChartStyleXmlPartID(&self) -> Option<&str> {
        // Java: return this.chartStyleRelationIds;
        self.chartStyleRelationIds.as_deref()
    }

    // Java: String getColorStyleXmlPartID()
    pub fn getColorStyleXmlPartID(&self) -> Option<&str> {
        // Java: return this.colorStyleRelationIds;
        self.colorStyleRelationIds.as_deref()
    }

    // Java: String getThemeOverridePartID()
    pub fn getThemeOverridePartID(&self) -> Option<&str> {
        // Java: return this.themeOverrideRelationIds;
        self.themeOverrideRelationIds.as_deref()
    }
}
