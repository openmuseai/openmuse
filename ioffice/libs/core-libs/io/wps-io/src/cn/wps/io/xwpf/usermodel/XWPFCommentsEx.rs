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
// Java: import cn.wps.io.xwpf.element_handler.CommentsExHandler
use crate::io::xwpf::element_handler::CommentsExHandler::CommentsExHandler;
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
// Java: import java.io.IOException

// Java: public class XWPFCommentsEx extends POIXMLDocumentPart
pub struct XWPFCommentsEx {
    pub(crate) base: POIXMLDocumentPart,
    // Java field: CommentsExHandler mCommentsExHandler
    pub(crate) mCommentsExHandler: Option<CommentsExHandler>,
    // Java field: boolean mIsParse
    pub(crate) mIsParse: bool,
}

impl XWPFCommentsEx {
    // Java: public XWPFCommentsEx(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel) throws IOException
    pub fn XWPFCommentsEx__parent_part_rel(
        parent: Option<&POIXMLDocumentPart>,
        part: Option<Box<PackagePart>>,
        rel: Option<Box<PackageRelationship>>,
    ) -> Self {
        // Java: super(parent, part, rel);
        XWPFCommentsEx {
            base: POIXMLDocumentPart::POIXMLDocumentPart__parent(parent, part, rel),
            mCommentsExHandler: None,
            mIsParse: false,
        }
    }

    // Java: public XWPFCommentsEx()
    pub fn XWPFCommentsEx() -> Self {
        // Java: super();
        XWPFCommentsEx {
            base: POIXMLDocumentPart::new(),
            mCommentsExHandler: None,
            mIsParse: false,
        }
    }

    // Java: public static XWPFCommentsEx startPart(XWPFDocument doc)
    pub fn startPart(doc: &mut POIXMLDocumentPart) -> Self {
        // Java: IOAssert.assertNotNull("doc should not be null.", doc);
        IOAssert::assertNotNull_msg("doc should not be null.", &Some(()));
        // Java: POIXMLDocumentPart part = POIXMLDocumentPart.startPart(doc, XWPFRelation.COMMENTS_EX, XWPFFactory.getInstance());
        // rust: XWPFRelation.COMMENTS_EX 的描述符在 Rust 侧未暴露公开访问器（静态量私有）；
        //       沿用 XWPFSettings::startPart 的既有做法，就地构造等价描述符（Java 串逐字相同）。
        let part = POIXMLDocumentPart::startPart(
            doc,
            &crate::io::opc::model::POIXMLRelation::POIXMLRelation::new_with_cls(
                "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml".to_string(),
                "http://schemas.microsoft.com/office/2011/relationships/commentsExtended".to_string(),
                "/word/commentsExtended.xml".to_string(),
                Some("XWPFCommentsEx".to_string()),
            ),
            super::XWPFFactory::XWPFFactory::getInstance(),
        );
        // Java: IOAssert.assertTrue("part should be instanceof XWPFCommentsEx", part instanceof XWPFCommentsEx);
        // rust: no-stmt（Java 用 instanceof 断言工厂返回类型；Rust 由返回类型静态保证）
        // Java: return (XWPFCommentsEx)part;
        XWPFCommentsEx {
            base: part,
            mCommentsExHandler: None,
            mIsParse: false,
        }
    }

    // Java: public void setDocumentImporter(IDocumentImporter importer)
    pub fn setDocumentImporter(&mut self, _importer: &dyn std::any::Any) {
        // Java: this.mCommentsExHandler = new CommentsExHandler(importer, this);
        // rust: Java 第二实参是宿主 part 自身（`this`）；Rust 侧 CommentsExHandler 尚未接线 importer
        //       （`CommentsExHandler::new` 为停车状态），故此处只保位次，不伪造 parent 传递。
        self.mCommentsExHandler = Some(CommentsExHandler::new());
    }

    // Java: public void parse()
    pub fn parse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Java: if (!this.mIsParse)
        if !self.mIsParse {
            // Java: this.mIsParse = true;
            self.mIsParse = true;

            // Java: try {
            // Java: XmlReader xmlReader = new XmlReader();
            let xmlReader = XmlReader::new();
            // Java: xmlReader.parse(this.getPackagePart().getInputStream(), this.mCommentsExHandler, "utf-8", -1, 0);
            if let Some(ref part_box) = self.base.getPackagePart() {
                let mut input = part_box.getInputStream_default()?;
                if let Some(ref mut handler) = self.mCommentsExHandler {
                    return xmlReader.parse(&mut *input, handler, "utf-8", -1, 0);
                }
            }
            // Java: } catch (IOException e) {
            // Java: IOLog.e(TAG, "IOException: ", e);
            // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
            // Java: throw new POIXMLException(e);
        }
        Ok(())
    }
}
