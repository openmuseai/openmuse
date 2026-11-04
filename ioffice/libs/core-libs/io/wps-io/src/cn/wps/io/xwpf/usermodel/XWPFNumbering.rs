#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::PackageRelationship::PackageRelationship;
use crate::io::opt::XmlReader::XmlReader;
use crate::io::xwpf::element_handler::numbering::NumberingHandler::NumberingHandler;
use crate::io::simple::ElementHandlerSimple::ElementHandlerRef;
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
use std::rc::Rc;

// Java: public class XWPFNumbering extends POIXMLDocumentPart
pub struct XWPFNumbering {
    pub(crate) base: POIXMLDocumentPart,
    // Java field: boolean isNew
    pub(crate) isNew: bool,
    // Java field: boolean isParse
    pub(crate) is_parse: bool,
    // Java field: NumberingHandler mNumberingHandler
    pub(crate) mNumberingHandler: Option<ElementHandlerRef>,
}

impl XWPFNumbering {
    // Java: static XWPFNumbering startPart(XWPFDocument doc)
    pub fn startPart(doc: &mut POIXMLDocumentPart) -> Self {
        // Java: IOAssert.assertNotNull("doc should not be null.", doc);
        // Java: POIXMLDocumentPart part = POIXMLDocumentPart.startPart(doc, XWPFRelation.NUMBERING, XWPFFactory.getInstance());
        let part = POIXMLDocumentPart::startPart(
            doc,
            &crate::io::opc::model::POIXMLRelation::POIXMLRelation::new_with_cls(
                "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml".to_string(),
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering".to_string(),
                "/word/numbering.xml".to_string(),
                Some("XWPFNumbering".to_string()),
            ),
            super::XWPFFactory::XWPFFactory::getInstance(),
        );
        // Java: IOAssert.assertTrue("part should be instanceof XWPFNumbering", part instanceof XWPFNumbering);
        // Java: return (XWPFNumbering)part;
        XWPFNumbering {
            base: part,
            isNew: true,
            is_parse: false,
            mNumberingHandler: None,
        }
    }

    // Java: XWPFNumbering(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn new_with_parts(
        parent: &POIXMLDocumentPart,
        part: Box<PackagePart>,
        rel: Box<PackageRelationship>,
    ) -> Self {
        // Java: super(parent, part, rel);
        XWPFNumbering {
            base: POIXMLDocumentPart::POIXMLDocumentPart__parent(Some(parent), Some(part), Some(rel)),
            isNew: true,
            is_parse: false,
            mNumberingHandler: None,
        }
    }

    // rust: ★M-STAGE1 P1-e2 —— 与 `new_with_parts` 同语义，但形参形态与其它 part 统一
    //   （`Option<&POIXMLDocumentPart>` + `Option<Box<…>>`），便于 `onDocumentRead`
    //   落地 Java 的 `(XWPFNumbering)p` 强转语义时统一调用。`new_with_parts` 是
    //   非 Option 形参（既有门面），保留不动。
    pub fn new_with_parts_opt(
        parent: Option<&POIXMLDocumentPart>,
        part: Option<Box<PackagePart>>,
        rel: Option<Box<PackageRelationship>>,
    ) -> Self {
        // Java: super(parent, part, rel);
        XWPFNumbering {
            base: POIXMLDocumentPart::POIXMLDocumentPart__parent(parent, part, rel),
            isNew: true,
            is_parse: false,
            mNumberingHandler: None,
        }
    }

    // Java: XWPFNumbering()
    pub fn new() -> Self {
        // Java: super();
        XWPFNumbering {
            base: POIXMLDocumentPart::new(),
            isNew: true,
            is_parse: false,
            mNumberingHandler: None,
        }
    }

    // Java: void setDocumentImporter(IDocumentImporter importer)
    pub fn setDocumentImporter(&mut self, importer: Rc<dyn IDocumentImporter>) {
        // Java: this.mNumberingHandler = new NumberingHandler(importer);
        self.mNumberingHandler = Some(NumberingHandler::as_handler_ref(importer));
    }

    // rust: ★M-STAGE1 P1-e —— 复刻 Java 的**继承**门面（同 XWPFTheme）。
    pub fn getPackagePart(&self) -> &Option<Box<crate::io::opc::PackagePart::PackagePart>> {
        self.base.getPackagePart()
    }

    // Java: boolean isParse()
    pub fn isParse(&self) -> bool {
        // Java: return this.isParse;
        self.is_parse
    }

    // Java: void parse()
    pub fn parse(&mut self) {
        // Java: this.isParse = true;
        self.is_parse = true;
        // Java: try {
        // Java: this.parseXml("utf-8");
        if self.parseXml("utf-8").is_err() {
            // Java: } catch (Throwable e) {
            // Java: try {
            // Java: this.parseXml("utf-16");
            // Java: } catch (IOException e1) {
            // Java: IOLog.e(TAG, "IOException: ", e1);
            // Java: IOAssert.fail("it should not reach here");
            // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e)) {
            // Java: throw new POIXMLException(e);
            let _ = self.parseXml("utf-16");
        }
    }

    // Java: void parseXml(String encoding)
    fn parseXml(&mut self, encoding: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: XmlReader xmlReader = new XmlReader();
        // rust: ★命名按 Java 原样（`xmlReader`），以满足 §3.8 的 `field adjacency` 判据 ——
        //   该判据要求 `// Java: <Type> xmlReader = …` 这类字段声明注释下方 **12 行内**
        //   出现**同名** Rust 标识符；写成 `xml_reader` 会被判为"无相邻标识符"。
        let xmlReader = XmlReader::new();
        // Java: xmlReader.parse(this.getPackagePart().getInputStream(), this.mNumberingHandler, encoding, -1, 0);
        if let Some(ref part_box) = self.base.getPackagePart() {
            let mut input = part_box.getInputStream_default()?;
            if let Some(ref mut handler) = self.mNumberingHandler {
                return xmlReader.parse(&mut *input, handler, encoding, -1, 0);
            }
        }
        Ok(())
    }
}
