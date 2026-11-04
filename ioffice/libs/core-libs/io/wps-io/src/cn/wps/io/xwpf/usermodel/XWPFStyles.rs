#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.opc.OpenXmlNamespaceHelper
use crate::io::opc::OpenXmlNamespaceHelper::OpenXmlNamespaceHelper;
// Java: import cn.wps.io.opc.PackagePart
use crate::io::opc::PackagePart::PackagePart;
// Java: import cn.wps.io.opc.PackageRelationship
use crate::io::opc::PackageRelationship::PackageRelationship;
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
// Java: import cn.wps.io.opc.model.POIXMLException
use crate::io::opc::model::POIXMLException::POIXMLException;
// Java: import cn.wps.io.opt.XmlReader
use crate::io::opt::XmlReader::XmlReader;
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.element_handler.XmlSimpleNodeElementHandler
use crate::io::xwpf::element_handler::XmlSimpleNodeElementHandler::XmlSimpleNodeElementHandler;
// Java: import cn.wps.io.xwpf.element_handler.styles.StylesHandler
use crate::io::xwpf::element_handler::styles::StylesHandler::StylesHandler;
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
use crate::io::xwpf::usermodel::exceptions::OpenXML4JException::OpenXML4JException;
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
// Java: import java.io.IOException
// Java: import java.io.InputStream

use std::any::Any;

// Java: public class XWPFStyles extends POIXMLDocumentPart
pub struct XWPFStyles {
    pub base: POIXMLDocumentPart,
    // Java field: String TAG
    // Java field: StylesHandler mStylesHandler
    pub(crate) mStylesHandler: Option<StylesHandler>,
}

// Java: public class XWPFStyles extends POIXMLDocumentPart
impl XWPFStyles {
    // Java: static XWPFStyles startPart(XWPFDocument doc)
    pub fn startPart(doc: &mut POIXMLDocumentPart) -> XWPFStyles {
      // Java: IOAssert.assertNotNull("doc should not be null.", doc);
      // Java: POIXMLDocumentPart part = POIXMLDocumentPart.startPart(doc, XWPFRelation.STYLES, XWPFFactory.getInstance());
      let _ = super::XWPFRelation::XWPFRelation::getInstance("http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles");
      let part = POIXMLDocumentPart::startPart(doc, &crate::io::opc::model::POIXMLRelation::POIXMLRelation::new_with_cls(
          "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml".to_string(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles".to_string(),
          "/word/styles.xml".to_string(),
          Some("XWPFStyles".to_string()),
      ), super::XWPFFactory::XWPFFactory::getInstance());
      // Java: IOAssert.assertTrue("part should be instanceof XWPFStyles", part instanceof XWPFStyles);
      // Java: return (XWPFStyles)part;
      XWPFStyles {
          base: part,
          mStylesHandler: None,
      }
    }

    // Java: XWPFStyles(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn new_with_parent(parent: Option<Box<POIXMLDocumentPart>>, part: Option<Box<dyn Any + Send + Sync>>, rel: Option<Box<dyn Any + Send + Sync>>) -> Self {
      // Java: super(parent, part, rel);
      XWPFStyles {
          base: POIXMLDocumentPart::POIXMLDocumentPart__parent(parent, part, rel),
          mStylesHandler: None,
      }
    }

    // Java: XWPFStyles()
    pub fn new() -> Self {
      // Java: super();
      XWPFStyles {
          base: POIXMLDocumentPart::new(),
          mStylesHandler: None,
      }
    }

    // Java: void setDocumentImporter(IDocumentImporter importer)
    pub fn setDocumentImporter(&mut self, _importer: &dyn Any) {
      // Java: this.mStylesHandler = new StylesHandler(importer);
      self.mStylesHandler = Some(StylesHandler::new());
    }

    // Java: void parse()
    pub fn parse(&mut self) {
      // Java: try
      // Java: this.parseXml("utf-8");
      if self.parseXml("utf-8").is_err() {
          // Java: catch (Throwable e)
          // Java: try
          // Java: this.parseXml("utf-16");
          if let Err(_e1) = self.parseXml("utf-16") {
              // Java: catch (IOException e1)
              // Java: IOLog.e(TAG, "IOException: ", e1);
              // Java: IOAssert.fail("it should not reach here");
              // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
              // Java: throw new POIXMLException(e);
          }
      }
    }

    // Java: void parseXml(String encoding)
    fn parseXml(&mut self, encoding: &str) -> Result<(), Box<dyn std::error::Error>> {
      // Java: int latentStyles = 154284782;
      let _latent_styles: i32 = 154284782;
      // Java: XmlReader reader = new XmlReader();
      let reader = XmlReader::new();
      // Java: reader.parse(this.getPackagePart().getInputStream(), this.mStylesHandler, encoding, 154284782, 0);
      if let Some(ref part_box) = self.base.getPackagePart() {
          if let Some(part) = part_box.downcast_ref::<PackagePart>() {
              let mut input = part.getInputStream_default()?;
              if let Some(ref mut handler) = self.mStylesHandler {
                  return reader.parse(&mut *input, handler, encoding, 154284782, 0);
              }
          }
      }
      Ok(())
    }

    // Java: static void parse4BuiltinTblStyles(InputStream in, XmlSimpleNodeElementHandler handler)
    pub fn parse4BuiltinTblStyles(input: &mut dyn std::io::Read, handler: &mut dyn Any) {
      // Java: IOAssert.assertNotNull("in should not be null", in);
      // Java: IOAssert.assertNotNull("handler should not be null", handler);
      // Java: try
      // Java: OpenXmlNamespaceHelper.initTagNamespace(OpenXmlNamespaceHelper.OpenXmlFileType.TRANSITIONAL);
      OpenXmlNamespaceHelper::initTagNamespace(&crate::io::opc::OpenXmlNamespaceHelper::OpenXmlFileType::TRANSITIONAL);
      // Java: XmlReader xmlReader = new XmlReader();
      let xml_reader = XmlReader::new();
      // Java: xmlReader.parse(in, handler, "utf-8", -1, 0);
      let _ = xml_reader.parse(input, handler, "utf-8", -1, 0);
      // Java: catch (IOException var3)
      let _ = handler;
    }
}
