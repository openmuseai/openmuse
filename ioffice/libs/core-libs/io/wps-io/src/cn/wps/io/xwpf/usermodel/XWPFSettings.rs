#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::PackageRelationship::PackageRelationship;
use crate::io::opt::XmlReader::XmlReader;
use crate::io::xwpf::element_handler::settings::SettingsHandler::SettingsHandler;
use crate::io::simple::ElementHandlerSimple::ElementHandlerRef;
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use std::rc::Rc;

// Java: public class XWPFSettings
pub struct XWPFSettings {
    pub(crate) base: POIXMLDocumentPart,
    pub(crate) mSettingsHandler: Option<ElementHandlerRef>,
}

impl XWPFSettings {
    // Java: public static XWPFSettings startPart(XWPFDocument doc)
    pub fn startPart(doc: &mut POIXMLDocumentPart) -> Self {
        // Java: IOAssert.assertNotNull("doc should not be null.", doc);
        // Java: POIXMLDocumentPart part = POIXMLDocumentPart.startPart(doc, XWPFRelation.SETTINGS, XWPFFactory.getInstance());
        let part = POIXMLDocumentPart::startPart(
            doc,
            &crate::io::opc::model::POIXMLRelation::POIXMLRelation::new_with_cls(
                "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml".to_string(),
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings".to_string(),
                "/word/settings.xml".to_string(),
                Some("XWPFSettings".to_string()),
            ),
            super::XWPFFactory::XWPFFactory::getInstance(),
        );
        // Java: IOAssert.assertTrue("part should be instanceof XWPFSettings", part instanceof XWPFSettings);
        // Java: return (XWPFSettings)part;
        XWPFSettings {
            base: part,
            mSettingsHandler: None,
        }
    }

    // Java: public XWPFSettings(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel) throws IOException
    pub fn XWPFSettings__parent_part_rel(
        parent: Option<&POIXMLDocumentPart>,
        part: Option<Box<PackagePart>>,
        rel: Option<Box<PackageRelationship>>,
    ) -> Self {
        // Java: super(parent, part, rel);
        XWPFSettings {
            base: POIXMLDocumentPart::POIXMLDocumentPart__parent(parent, part, rel),
            mSettingsHandler: None,
        }
    }

    // Java: XWPFSettings()
    pub fn new() -> Self {
        // Java: super();
        XWPFSettings {
            base: POIXMLDocumentPart::new(),
            mSettingsHandler: None,
        }
    }

    // Java: void setDocumentImporter(IDocumentImporter importer)
    pub fn setDocumentImporter(&mut self, importer: Rc<dyn IDocumentImporter>) {
        // Java: this.mSettingsHandler = new SettingsHandler(importer, this);
        self.mSettingsHandler = Some(SettingsHandler::as_handler_ref(importer, self as *const XWPFSettings));
    }

    // rust: ★M-STAGE1 P1-e —— 复刻 Java 的**继承**门面。
    //   Java 侧 `XWPFSettings extends POIXMLDocumentPart` ⇒ `getPackagePart()` 是
    //   **继承来的 public 方法**，调用方（含测试）可直接使用；Rust 侧改用 `base` 组合，
    //   而 `base` 是 `pub(crate)`，外部不可见 ⇒ 门面缺口。此处按同一签名转发，保持门面一致。
    pub fn getPackagePart(&self) -> &Option<Box<crate::io::opc::PackagePart::PackagePart>> {
        self.base.getPackagePart()
    }

    // Java: XWPFSettings inherits POIXMLDocumentPart.getRelationShipById(String).
    // Rust stores the parent class as `base`, so expose the same public facade.
    pub fn getRelationShipById(&self, id: &str) -> Option<PackageRelationship> {
        self.base.getRelationShipById(id)
    }

    // Java: public void parse() throws IOException
    pub fn parse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Java: int rsid = 108811479;
        let _rsid: i32 = 108811479;
        // Java: try {
        // Java: XmlReader reader = new XmlReader();
        let reader = XmlReader::new();
        // Java: reader.parse(this.getPackagePart().getInputStream(), this.mSettingsHandler, "utf-8", -1, 0);
        if let Some(ref part_box) = self.base.getPackagePart() {
            let mut input = part_box.getInputStream_default()?;
            if let Some(ref mut handler) = self.mSettingsHandler {
                return reader.parse(&mut *input, handler, "utf-8", -1, 0);
            }
        }
        // Java: } catch (IOException e) {
        // Java: if (e instanceof ForceQuitException) {
        // Java: throw e;
        // Java: IOLog.e(TAG, "IOException: ", e);
        // Java: IOAssert.fail("it should not reach here");
        // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e)) {
        // Java: throw new POIXMLException(e);
        Ok(())
    }
}
