use crate::cn::wps::moss::app::KmoBook::KmoBook;
use kfc_kfc_alpha::kfc::ooxfr::OxfrAppPart::OxfrAppPart;
// java public final class XlsxrApp {
pub struct XlsxrApp;
impl XlsxrApp {
    // java private static final String TAG = null;
    const TAG:Option<&'static str>=None;
    // java public static void open(KmoBook book, OxfrAppPart part) throws IOException {
    pub fn open(book:&KmoBook,part:Option<&mut OxfrAppPart>)->std::io::Result<()> {
        // java if (part != null) {
        if let Some(part)=part {
            // java try { part.parse(); } catch (Throwable e) { IOLog.e(TAG, "Throwable", e); }
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(||part.parse())) {Ok(Ok(()))=>(),Ok(Err(e))=>eprintln!("{:?}: Throwable: {e}",Self::TAG),Err(_)=>eprintln!("{:?}: Throwable",Self::TAG)}
            // java KmoAppSettings settings = book.getAppSettings();
            let settings=book.getAppSettings();let mut settings=settings.borrow_mut();
            // java settings.setAppName(part.getAppName());
            settings.setAppName(part.getAppName());
            // java settings.setAppVersion(part.getAppVersion());
            settings.setAppVersion(part.getAppVersion());
            // java settings.setManager(part.getManager());
            settings.setManager(part.getManager());
            // java settings.setCompany(part.getCompany());
            settings.setCompany(part.getCompany());
            // java settings.setHyperlinkBase(part.getHyperlinkBase());
            settings.setHyperlinkBase(part.getHyperlinkBase());
            // java settings.setSecurity(part.getDocSecurity());
            settings.setSecurity(part.getDocSecurity());
            // java settings.setScale(part.getScale());
            settings.setScale(part.getScale());
            // java settings.setLinkDirty(part.getLinkDirty());
            settings.setLinkDirty(part.getLinkDirty());
        }Ok(())
    }
}
