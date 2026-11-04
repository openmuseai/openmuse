use crate::cn::wps::moss::app::KmoBook::KmoBook;
use kfc_kfc_alpha::kfc::ooxfr::OxfrCorePart::OxfrCorePart;
// java public final class XlsxrCore {
pub struct XlsxrCore;
impl XlsxrCore {
    // java private static final String TAG = null;
    const TAG:Option<&'static str>=None;
    // java public static void open(KmoBook book, OxfrCorePart part) throws IOException {
    pub fn open(book:&KmoBook,part:Option<&mut OxfrCorePart>)->std::io::Result<()> {
        // java if (part != null) {
        if let Some(part)=part {
            // java try { part.parse(); } catch (Throwable e) { IOLog.e(TAG, "Throwable", e); }
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(||part.parse())) {Ok(Ok(()))=>(),Ok(Err(e))=>eprintln!("{:?}: Throwable: {e}",Self::TAG),Err(_)=>eprintln!("{:?}: Throwable",Self::TAG)}
            // java KmoAppSettings settings = book.getAppSettings();
            let settings=book.getAppSettings();let mut settings=settings.borrow_mut();
            // java settings.setTitle(part.getTitle());
            settings.setTitle(part.getTitle());
            // java settings.setSubject(part.getSubject());
            settings.setSubject(part.getSubject());
            // java settings.setCreator(part.getCreator());
            settings.setCreator(part.getCreator());
            // java settings.setKeywords(part.getKeywords());
            settings.setKeywords(part.getKeywords());
            // java settings.setDescription(part.getDescription());
            settings.setDescription(part.getDescription());
            // java settings.setLastModifiedBy(part.getLastModifiedBy());
            settings.setLastModifiedBy(part.getLastModifiedBy());
            // java settings.setCreated(part.getCreated());
            settings.setCreated(part.getCreated());
            // java settings.setModified(part.getModified());
            settings.setModified(part.getModified());
            // java settings.setCategory(part.getCategory());
            settings.setCategory(part.getCategory());
            // java settings.setContentStatus(part.getContentStatus());
            settings.setContentStatus(part.getContentStatus());
        }Ok(())
    }
}
