use crate::{java_runtime::Rc,cn::wps::moss::{app::KmoBook::KmoBook,io::except::PasswordCancelException::PasswordCancelException,xlsxr::handler::docProps::PropertiesHandler::PropertiesHandler}};
use kfc_kfc_alpha::kfc::{ooxfr::OxfrCustomPart::OxfrCustomPart,xpxml::hlxml::HLXmlReader::HLXmlReader};
// java public final class XlsxrCustom {
pub struct XlsxrCustom;
impl XlsxrCustom {
    // java private static final String TAG = null;
    const TAG:Option<&'static str>=None;
    // java public static void open(KmoBook book, OxfrCustomPart part) throws IOException {
    pub fn open(book:Rc<KmoBook>,part:Option<&OxfrCustomPart>)->std::io::Result<()> {
        // java if (part != null) {
        if let Some(part)=part {
            // java InputStream stream = part.getInputStream();
            let stream=part.getInputStream_nullable(false)?;
            // java if (stream != null) {
            if let Some(mut stream)=stream {
                // java try { HLXmlReader.parser(stream, new PropertiesHandler(book)); }
                let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||HLXmlReader::parser_str_handler(&mut stream,Box::new(PropertiesHandler::PropertiesHandler(book)))));
                // java catch (Throwable e) {
                match result {Ok(Ok(()))=>(),Ok(Err(e))=>{
                    // java if (e instanceof PasswordCancelException) { throw (PasswordCancelException)e; }
                    if e.get_ref().is_some_and(|e|e.is::<PasswordCancelException>()){return Err(e);}
                    // java IOLog.d(TAG, "Throwable", e);
                    eprintln!("{:?}: Throwable: {e}",Self::TAG);
                },Err(e)=>{
                    // java if (e instanceof PasswordCancelException) { throw (PasswordCancelException)e; }
                    if e.is::<PasswordCancelException>(){std::panic::resume_unwind(e);}
                    // java IOLog.d(TAG, "Throwable", e);
                    eprintln!("{:?}: Throwable",Self::TAG);
                }}
            }
        }Ok(())
    }
}
