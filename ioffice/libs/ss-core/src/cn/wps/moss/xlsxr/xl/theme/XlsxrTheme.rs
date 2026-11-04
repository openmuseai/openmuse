use std::io;
use kfc_kfc_alpha::kfc::opcr::OpcrRelationship::OpcrRelationship;
use crate::cn::wps::moss::xlsxr::{handler::theme::ThemeHandler::ThemeHandler,util::UnifiedXmlReader::UnifiedXmlReader};
// java public class XlsxrTheme extends HandlerCallback
// rust: Theme media/protocol persistence is not in the M2 ColorElement read closure.
pub struct XlsxrTheme;
impl XlsxrTheme {
// java public void open() throws IOException { if (this.mThemeRels != null) { OpcrPackagePart part = this.mThemeRels.getPackagePart(); if (part != null) { ... ThemeHandler handler = new ThemeHandler(this.mTheme); handler.init(this); UnifiedXmlReader.read((InputStream)stream, handler); ... } } }
pub fn open(rels:Option<&mut OpcrRelationship>)->io::Result<()> {if let Some(rels)=rels{if let Some(part)=rels.getPackagePart(){let mut stream=part.getInputStream_default()?;UnifiedXmlReader::read(&mut stream,Box::new(ThemeHandler))?;}}Ok(())}
}
