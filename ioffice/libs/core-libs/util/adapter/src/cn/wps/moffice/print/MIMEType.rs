// Java: import cn.wps.io.file.FileFormatEnum

// Java field: String TEXT_ANY
pub(crate) const TEXT_ANY: &str = "text/*";
// Java field: String TXT
pub(crate) const TXT: &str = "text/plain";
// Java field: String HTML
pub(crate) const HTML: &str = "text/html";
// Java field: String MHT
pub(crate) const MHT: &str = "message/rfc822";
// Java field: String POSTSCRIPT
pub(crate) const POSTSCRIPT: &str = "application/postscript";
// Java field: String PDF
pub(crate) const PDF: &str = "application/pdf";
// Java field: String DOC
pub(crate) const DOC: &str = "application/msword";
// Java field: String XLS
pub(crate) const XLS: &str = "application/vnd.ms-excel";
// Java field: String PPT
pub(crate) const PPT: &str = "application/vnd.ms-powerpoint";
// Java field: String DOCX
pub(crate) const DOCX: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
// Java field: String XLSX
pub(crate) const XLSX: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
// Java field: String PPTX
pub(crate) const PPTX: &str = "application/vnd.openxmlformats-officedocument.presentationml.presentation";
// Java field: String COPY_PNG
pub(crate) const COPY_PNG: &str = "copy/png";

// Java: public class MIMEType
pub struct MIMEType {
}

// Java: public class MIMEType
impl MIMEType {
    // Java: MIMEType()
    pub fn new() -> Self {
      // Java: super();
      MIMEType {}
    }

    // Java: static String getMIMEType(FileFormatEnum fileFormat)
    pub fn getMIMEType(_fileFormat: &str) -> Option<&'static str> {
      // Java: switch (fileFormat)
      match _fileFormat {
         // Java: case TXT:
         "TXT" => {
            // Java: return "text/plain";
            Some("text/plain")
         }
         // Java: case HTM:
         // Java: case HTML:
         "HTM" | "HTML" => {
            // Java: return "text/html";
            Some("text/html")
         }
         // Java: case DOC:
         // Java: case DOT:
         // Java: case WPS:
         // Java: case WPT:
         "DOC" | "DOT" | "WPS" | "WPT" => {
            // Java: return "application/msword";
            Some("application/msword")
         }
         // Java: case DOCX:
         // Java: case DOTX:
         "DOCX" | "DOTX" => {
            // Java: return "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
            Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document")
         }
         // Java: case PDF:
         "PDF" => {
            // Java: return "application/pdf";
            Some("application/pdf")
         }
         // Java: case XLS:
         // Java: case XLT:
         // Java: case ET:
         // Java: case ETT:
         "XLS" | "XLT" | "ET" | "ETT" => {
            // Java: return "application/vnd.ms-excel";
            Some("application/vnd.ms-excel")
         }
         // Java: case XLSX:
         // Java: case XLTX:
         "XLSX" | "XLTX" => {
            // Java: return "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
            Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
         }
         // Java: case PPT:
         // Java: case PPS:
         // Java: case POT:
         // Java: case DPS:
         // Java: case DPT:
         "PPT" | "PPS" | "POT" | "DPS" | "DPT" => {
            // Java: return "application/vnd.ms-powerpoint";
            Some("application/vnd.ms-powerpoint")
         }
         // Java: case PPTX:
         // Java: case PPSX:
         // Java: case POTX:
         "PPTX" | "PPSX" | "POTX" => {
            // Java: return "application/vnd.openxmlformats-officedocument.presentationml.presentation";
            Some("application/vnd.openxmlformats-officedocument.presentationml.presentation")
         }
         // Java: default:
         _ => {
            // Java: return null;
            None
         }
      }
    }

}
