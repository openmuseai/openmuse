#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.gvml.GVmlPackage
// Java: import cn.wps.io.gvml.IGVmlImporter
// Java: import cn.wps.io.opc.OpenXmlNamespaceHelper
// Java: import cn.wps.io.opc.PackageAccess
// Java: import cn.wps.io.opc.PackagePartName
// Java: import cn.wps.io.opc.PackageRelationshipCollection
// Java: import cn.wps.io.opc.PackagingURIHelper
// Java: import cn.wps.io.opc.TargetMode
// Java: import cn.wps.io.opc.model.POIXMLDocument
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.opc.model.POIXMLException
// Java: import cn.wps.io.opt.XmlReader
// Java: import cn.wps.io.xwpf.element_handler.dml.GraphicHandler
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
// Java: import cn.wps.moffice.kfs.File
// Java: import java.io.IOException
// Java: import java.util.List

// Java: public class GVmlDrawing extends POIXMLDocument
pub struct GVmlDrawing {
    // Java field: String TAG
    // Java field: GraphicHandler mGraphicHandler
    mGraphicHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: GVmlTheme mTheme
    mTheme: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class GVmlDrawing extends POIXMLDocument
impl GVmlDrawing {
    // Java: GVmlDrawing(GVmlPackage pkg)
    pub fn new(_pkg: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(pkg);
        // Java: OpenXmlNamespaceHelper.OpenXmlFileType fileType = OpenXmlNamespaceHelper.OpenXmlFileType.TRANSITIONAL;
        // Java: OpenXmlNamespaceHelper.initTagNamespace(fileType);
        // Java: PackageRelationshipCollection coreRels = pkg.getRelationshipsByType(GVmlRelation.GVML_DRAWING.getRelation());
        // Java: if (coreRels.size() > 0)
        // Java: this.packageRel = coreRels.getRelationship(0);
        // Java: this.packagePart = pkg.getPart(this.packageRel);
        // Java: this.load(GVmlFactory.getInstance());
        todo!()
    }

    // Java: GVmlDrawing(File file, boolean createNew)
    pub fn new_with_file(_file: Box<dyn std::any::Any + Send + Sync>, _create_new: bool) -> Self {
        // Java: this(createNew ? newPackage(file) : GVmlPackage.open(file.getAbsolutePath(), PackageAccess.READ));
        todo!()
    }

    // Java: static GVmlDrawing startPackage(File file)
    pub fn startPackage(_file: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: return new GVmlDrawing(file, true);
        todo!()
    }

    // Java: void onDocumentRead()
    pub fn onDocumentRead(&mut self) {
        // Java: List<POIXMLDocumentPart> relations = this.getRelations();
        // Java: int size = relations.size();
        // Java: for(int i = 0; i < size; ++i)
        // Java: POIXMLDocumentPart p = (POIXMLDocumentPart)relations.get(i);
        // Java: String relation = p.getPackageRelationship().getRelationshipType();
        // Java: if (relation.equals(GVmlRelation.GVML_THEME.getRelation()))
        // Java: this.mTheme = (GVmlTheme)p;
        todo!()
    }

    // Java: static GVmlPackage newPackage(File file)
    fn newPackage(_file: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
        // Java: try
        // Java: GVmlPackage pkg = GVmlPackage.create(file);
        // Java: PackagePartName drawingPartName = PackagingURIHelper.createPartName(GVmlRelation.GVML_DRAWING.getDefaultFileName());
        // Java: pkg.addRelationship(drawingPartName, TargetMode.INTERNAL, GVmlRelation.GVML_DRAWING.getRelation());
        // Java: pkg.createPart(drawingPartName, GVmlRelation.GVML_DRAWING.getContentType());
        // Java: return pkg;
        // Java: catch (Exception e)
        // Java: throw new POIXMLException(e);
        todo!()
    }

    // Java: void setDocumentImporter(IGVmlImporter importer, SubDocType subDocType)
    pub fn setDocumentImporter(&mut self, _importer: Box<dyn std::any::Any + Send + Sync>, _sub_doc_type: Box<dyn std::any::Any + Send + Sync>) {
        // Java: if (null != this.mTheme)
        // Java: this.mTheme.setGVmlImporter(importer);
        // Java: this.mGraphicHandler = new GraphicHandler(importer, this, subDocType, 0, 0);
        todo!()
    }

    // Java: void parse()
    pub fn parse(&self) {
        // Java: if (null != this.mTheme)
        // Java: this.mTheme.parse();
        // Java: try
        // Java: XmlReader xmlReader = new XmlReader();
        // Java: xmlReader.parse(this.getPackagePart().getInputStream(), this.mGraphicHandler, "utf-8", -1, 2);
        // Java: catch (IOException e)
        // Java: IOLog.e(TAG, "IOException: ", e);
        // Java: IOAssert.fail("it should not reach here");
        // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
        // Java: throw new POIXMLException(e);
        todo!()
    }

    // Java: void close()
    pub fn close(&self) {
        // Java: try
        // Java: this.getPackage().close();
        // Java: catch (IOException e)
        // Java: IOLog.e(TAG, "IOException", e);
        // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
        // Java: throw new POIXMLException(e);
        todo!()
    }
}
