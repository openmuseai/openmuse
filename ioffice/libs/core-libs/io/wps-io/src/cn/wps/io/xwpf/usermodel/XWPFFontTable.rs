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
// Java: import cn.wps.io.xwpf.element_handler.font.FontsHandler
use crate::io::xwpf::element_handler::font::FontsHandler::FontsHandler;
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
// Java: import java.io.IOException

// Java: public class XWPFFontTable extends POIXMLDocumentPart
pub struct XWPFFontTable {
    pub(crate) base: POIXMLDocumentPart,
    // Java field: FontsHandler mFontsHandler
    pub(crate) mFontsHandler: Option<FontsHandler>,
}

impl XWPFFontTable {
    // Java: public static XWPFFontTable startPart(XWPFDocument doc)
    pub fn startPart(doc: &mut POIXMLDocumentPart) -> Self {
        // Java: IOAssert.assertNotNull("doc should not be null.", doc);
        IOAssert::assertNotNull_msg("doc should not be null.", &Some(()));
        // Java: POIXMLDocumentPart part = POIXMLDocumentPart.startPart(doc, XWPFRelation.FONT_TABLE, XWPFFactory.getInstance());
        // rust: XWPFRelation.FONT_TABLE 的描述符在 Rust 侧未暴露公开访问器（静态量私有）；
        //       沿用 XWPFSettings::startPart 的既有做法，就地构造等价描述符（Java 串逐字相同）。
        let part = POIXMLDocumentPart::startPart(
            doc,
            &crate::io::opc::model::POIXMLRelation::POIXMLRelation::new_with_cls(
                "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml".to_string(),
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable".to_string(),
                "/word/fontTable.xml".to_string(),
                Some("XWPFFontTable".to_string()),
            ),
            super::XWPFFactory::XWPFFactory::getInstance(),
        );
        // Java: IOAssert.assertTrue("part should be instanceof XWPFFontTable", part instanceof XWPFFontTable);
        // rust: no-stmt（Java 用 instanceof 断言工厂返回类型；Rust 由返回类型静态保证）
        // Java: return (XWPFFontTable)part;
        XWPFFontTable {
            base: part,
            mFontsHandler: None,
        }
    }

    // Java: public XWPFFontTable()
    pub fn XWPFFontTable() -> Self {
        // Java: super();
        XWPFFontTable {
            base: POIXMLDocumentPart::new(),
            mFontsHandler: None,
        }
    }

    // Java: public XWPFFontTable(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel) throws IOException, OpenXML4JException
    pub fn XWPFFontTable__parent_part_rel(
        parent: Option<&POIXMLDocumentPart>,
        part: Option<Box<PackagePart>>,
        rel: Option<Box<PackageRelationship>>,
    ) -> Self {
        // Java: super(parent, part, rel);
        XWPFFontTable {
            base: POIXMLDocumentPart::POIXMLDocumentPart__parent(parent, part, rel),
            mFontsHandler: None,
        }
    }

    // Java: void setDocumentImporter(IDocumentImporter importer)
    pub fn setDocumentImporter(&mut self, _importer: &dyn std::any::Any) {
        // Java: this.mFontsHandler = new FontsHandler(importer);
        self.mFontsHandler = Some(FontsHandler::new());
    }

    // rust: ★M-STAGE1 P1-e —— 复刻 Java 的**继承**门面。
    //   Java 侧 `XWPFFontTable extends POIXMLDocumentPart` ⇒ `getPackagePart()` 继承而来、
    //   调用方直接可用；Rust 侧 `base` 是 `pub(crate)`，外部不可见 ⇒ 门面缺口。
    //   按同一签名转发，保持门面一致（并让「子 part 是否真绑定 packagePart」可被外部观测）。
    pub fn getPackagePart(&self) -> &Option<Box<crate::io::opc::PackagePart::PackagePart>> {
        self.base.getPackagePart()
    }

    // Java: public void parse()
    pub fn parse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Java: try {
        // Java: XmlReader xmlReader = new XmlReader();
        let xmlReader = XmlReader::new();
        // Java: xmlReader.parse(this.getPackagePart().getInputStream(), this.mFontsHandler, "utf-8", -1, 0);
        if let Some(ref part_box) = self.base.getPackagePart() {
            let mut input = part_box.getInputStream_default()?;
            if let Some(ref mut handler) = self.mFontsHandler {
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
