// Java: import cn.wps.io.xwpf.usermodel.PictureType
use crate::io::xwpf::usermodel::PictureType::PictureType;

// Java field: String EXTENDED_PROPERTIES_PART
pub(crate) const EXTENDED_PROPERTIES_PART: &str = "application/vnd.openxmlformats-officedocument.extended-properties+xml";
// Java field: String CORE_PROPERTIES_PART
pub(crate) const CORE_PROPERTIES_PART: &str = "application/vnd.openxmlformats-package.core-properties+xml";
// Java field: String CUSTOM_PROPERTIES_PART
pub(crate) const CUSTOM_PROPERTIES_PART: &str = "application/vnd.openxmlformats-officedocument.custom-properties+xml";
// Java field: String DIGITAL_SIGNATURE_CERTIFICATE_PART
pub(crate) const DIGITAL_SIGNATURE_CERTIFICATE_PART: &str = "application/vnd.openxmlformats-package.digital-signature-certificate";
// Java field: String DIGITAL_SIGNATURE_ORIGIN_PART
pub(crate) const DIGITAL_SIGNATURE_ORIGIN_PART: &str = "application/vnd.openxmlformats-package.digital-signature-origin";
// Java field: String DIGITAL_SIGNATURE_XML_SIGNATURE_PART
pub(crate) const DIGITAL_SIGNATURE_XML_SIGNATURE_PART: &str = "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml";
// Java field: String RELATIONSHIPS_PART
pub(crate) const RELATIONSHIPS_PART: &str = "application/vnd.openxmlformats-package.relationships+xml";
// Java field: String CUSTOM_XML_PART
pub(crate) const CUSTOM_XML_PART: &str = "application/vnd.openxmlformats-officedocument.customXmlProperties+xml";
// Java field: String PLAIN_OLD_XML
pub(crate) const PLAIN_OLD_XML: &str = "application/xml";
// Java field: String XML
pub(crate) const XML: &str = "text/xml";
// Java field: String EXTENSION_XML
pub(crate) const EXTENSION_XML: &str = "xml";

// Java: public class ContentTypes
pub struct ContentTypes {
}

// Java: public class ContentTypes
impl ContentTypes {
    // Java field: String EXTENDED_PROPERTIES_PART
    // Java field: String CORE_PROPERTIES_PART
    // Java field: String CUSTOM_PROPERTIES_PART
    // Java field: String DIGITAL_SIGNATURE_CERTIFICATE_PART
    // Java field: String DIGITAL_SIGNATURE_ORIGIN_PART
    // Java field: String DIGITAL_SIGNATURE_XML_SIGNATURE_PART
    // Java field: String RELATIONSHIPS_PART
    // Java field: String CUSTOM_XML_PART
    // Java field: String PLAIN_OLD_XML
    // Java field: String XML
    // Java field: String EXTENSION_XML

    // Java: ContentTypes()
    pub fn new() -> Self {
      // Java: super();
      ContentTypes {}
    }

    // Rust-native: Java 构造器 `ContentTypes()` 的**同名入口**（仓内 ctor 约定，与 `new` 同实现；
    // Java 侧该构造器是 `public`，Rust 侧 `new` 是惯用名 ⇒ 两个都保留）。
    pub fn ContentTypes() -> Self {
      // Java: super();
      ContentTypes {}
    }

    // Java: static String guessContentTypeByExtension(String extension)
    pub fn guessContentTypeByExtension(extension: &str) -> Option<String> {
      // Java: PictureType pictureType = PictureType.getPictureType(extension);
      let picture_type = PictureType::getPictureType(extension);
      // Java: return null != pictureType ? pictureType.contentType : null;
      // Rust: Java 这里读的是**字段** `contentType`（`PictureType.java:27`）⇒ 用字段等价的
      //       `content_type()`（`&'static str`）再转 `String`，不走 `getContentType()` 那一层。
      picture_type.map(|pt| pt.content_type().to_string())
    }
}
