use kfc_kfc_alpha::kfc::ooxfw::OxfwCorePart::OxfwCorePart;
use util_adapter::cn::wps::core::runtime::Platform::Platform;
use crate::cn::wps::moss::core::book::KmoAppSettings::KmoAppSettings;
// java public class XlsxwCore {
pub struct XlsxwCore;
impl XlsxwCore {
    // java public static void write(OxfwCorePart part, KmoAppSettings settings) {
    pub fn write(part:&mut OxfwCorePart,settings:&KmoAppSettings){
        // java part.setTitle(settings.getTitle());
        part.setTitle(settings.getTitle());
        // java part.setSubject(settings.getSubject());
        part.setSubject(settings.getSubject());
        // java String creator = settings.getCreator();
        let mut creator=settings.getCreator();
        // java if (null == creator || creator.length() <= 0) { creator = Platform.getUserName(); }
        if creator.as_ref().is_none_or(String::is_empty){creator=Platform::getUserName();}
        // java part.setCreator(creator);
        part.setCreator(creator);
        // java part.setKeywords(settings.getKeywords());
        part.setKeywords(settings.getKeywords());
        // java part.setDescription(settings.getDescription());
        part.setDescription(settings.getDescription());
        // java part.setLastModifiedBy(settings.getLastModifiedBy());
        part.setLastModifiedBy(settings.getLastModifiedBy());
        // java part.setCreated(settings.getCreated());
        part.setCreated(settings.getCreated());
        // java part.setModified(settings.getModified());
        part.setModified(settings.getModified());
        // java part.setCategory(settings.getCategory());
        part.setCategory(settings.getCategory());
        // java part.setContentStatus(settings.getContentStatus());
        part.setContentStatus(settings.getContentStatus());
    }
}
