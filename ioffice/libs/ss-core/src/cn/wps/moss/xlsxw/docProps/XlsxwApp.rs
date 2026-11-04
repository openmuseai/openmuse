use kfc_kfc_alpha::kfc::ooxfw::OxfwAppPart::OxfwAppPart;
use crate::cn::wps::moss::core::book::KmoAppSettings::KmoAppSettings;
// java public class XlsxwApp {
pub struct XlsxwApp;
impl XlsxwApp {
    // java public static void write(OxfwAppPart part, KmoAppSettings settings) {
    pub fn write(part:&mut OxfwAppPart,settings:&KmoAppSettings){
        // java part.setAppName(settings.getAppName());
        part.setAppName(settings.getAppName());
        // java part.setAppVersion(settings.getAppVersion());
        part.setAppVersion(settings.getAppVersion());
        // java part.setManager(settings.getManager());
        part.setManager(settings.getManager());
        // java part.setCompany(settings.getCompany());
        part.setCompany(settings.getCompany());
        // java part.setHyperlinkBase(settings.getHyperlinkBase());
        part.setHyperlinkBase(settings.getHyperlinkBase());
        // java part.setDocSecurity(settings.getSecurity());
        part.setDocSecurity(settings.getSecurity());
        // java part.setScale(settings.getScale());
        part.setScale(settings.getScale());
        // java part.setLinkDirty(settings.getLinkDirty());
        part.setLinkDirty(settings.getLinkDirty());
    }
}
