#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.gvml.IGVmlImporter
// Java: import cn.wps.io.gvml.handler.ClipboardThemeHandler
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackageRelationship
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.opc.model.POIXMLException
// Java: import cn.wps.io.opt.XmlReader
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
// Java: import java.io.IOException

// Java: public class GVmlTheme extends POIXMLDocumentPart
pub struct GVmlTheme {
    // Java field: String TAG
    // Java field: ClipboardThemeHandler mGVmlThemeHandler
    mGVmlThemeHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class GVmlTheme extends POIXMLDocumentPart
impl GVmlTheme {
    // Java: GVmlTheme(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn new_with_parts(_parent: Box<dyn std::any::Any + Send + Sync>, _part: Box<dyn std::any::Any + Send + Sync>, _rel: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(parent, part, rel);
        GVmlTheme {
            mGVmlThemeHandler: None,
        }
    }

    // Java: GVmlTheme()
    pub fn new() -> Self {
        // Java: super();
        GVmlTheme {
            mGVmlThemeHandler: None,
        }
    }

    // Java: void setGVmlImporter(IGVmlImporter importer)
    pub fn setGVmlImporter(&mut self, _importer: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.mGVmlThemeHandler = new ClipboardThemeHandler(importer);
        // Java: importer.setTheme(this.dump());
        todo!()
    }

    // Java: void parse()
    pub fn parse(&self) {
        // Java: try
        // Java: XmlReader xmlReader = new XmlReader();
        // Java: xmlReader.parse(this.getPackagePart().getInputStream(), this.mGVmlThemeHandler, "utf-8", -1, 2);
        // Java: catch (IOException e)
        // Java: IOLog.e(TAG, "IOException: ", e);
        // Java: IOAssert.fail("it should not reach here");
        // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
        // Java: throw new POIXMLException(e);
        todo!()
    }
}
