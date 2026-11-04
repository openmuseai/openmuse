#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
use core_libs_util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.opc.PackagePart
use crate::io::opc::PackagePart::PackagePart;
// Java: import cn.wps.io.opc.PackageRelationship
use crate::io::opc::PackageRelationship::PackageRelationship;
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
// Java: import cn.wps.io.opc.model.POIXMLException
// Java: import cn.wps.io.opt.XmlReader
use crate::io::opt::XmlReader::XmlReader;
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.dml.ThemeHandler
use crate::io::xwpf::element_handler::dml::ThemeHandler::ThemeHandler;
use crate::io::simple::ElementHandlerSimple::ElementHandlerRef;
use std::rc::Rc;
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
// Java: import java.io.IOException

// Java: public class XWPFTheme extends POIXMLDocumentPart
pub struct XWPFTheme {
    pub(crate) base: Rc<POIXMLDocumentPart>,
    // Java field: ThemeHandler mThemeHandler
    pub(crate) mThemeHandler: Option<ElementHandlerRef>,
}

impl XWPFTheme {
    // Java: public XWPFTheme(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn XWPFTheme__parent_part_rel(
        parent: Option<&POIXMLDocumentPart>,
        part: Option<Box<PackagePart>>,
        rel: Option<Box<PackageRelationship>>,
    ) -> Self {
        // Java: super(parent, part, rel);
        XWPFTheme {
            base: Rc::new(POIXMLDocumentPart::POIXMLDocumentPart__parent(parent, part, rel)),
            mThemeHandler: None,
        }
    }

    // rust: ★M-STAGE1 P1-e —— 复刻 Java 的**继承**门面。
    //   Java 侧 `XWPFTheme extends POIXMLDocumentPart` ⇒ `getPackagePart()` 继承而来、
    //   调用方直接可用；Rust 侧 `base` 是 `pub(crate)` ⇒ 门面缺口。按同一签名转发。
    pub fn getPackagePart(&self) -> &Option<Box<crate::io::opc::PackagePart::PackagePart>> {
        self.base.getPackagePart()
    }

    // Java: public XWPFTheme()
    pub fn XWPFTheme() -> Self {
        // Java: super();
        XWPFTheme {
            base: Rc::new(POIXMLDocumentPart::new()),
            mThemeHandler: None,
        }
    }

    // Java: void setDocumentImporter(IDocumentImporter importer)
    pub fn setDocumentImporter(&mut self, importer: Rc<dyn IDocumentImporter>) {
        // Java: this.mThemeHandler = new ThemeHandler(importer, this);
        self.mThemeHandler = Some(ThemeHandler::as_handler_ref(importer, self.base.clone()));
    }

    // Java: public void parse()
    pub fn parse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Java: try {
        // Java: XmlReader xmlReader = new XmlReader();
        let xmlReader = XmlReader::new();
        // Java: xmlReader.parse(this.getPackagePart().getInputStream(), this.mThemeHandler, "utf-8", -1, 0);
        if let Some(ref part_box) = self.base.getPackagePart() {
            let mut input = part_box.getInputStream_default()?;
            if let Some(ref mut handler) = self.mThemeHandler {
                return xmlReader.parse(&mut *input, handler, "utf-8", -1, 0);
            }
        }
        // Java: } catch (IOException e) {
        // Java: IOLog.e(TAG, "IOException: ", e);
        // Java: IOAssert.fail("it should not reach here");
        // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
        // Java: throw new POIXMLException(e);
        Ok(())
    }
}
