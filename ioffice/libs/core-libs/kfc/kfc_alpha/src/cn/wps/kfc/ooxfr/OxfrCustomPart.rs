use crate::kfc::opcr::{OpcrPackagePart::OpcrPackagePart,ZipFile::{ZipFile,ZipEntry}};
// java public final class OxfrCustomPart extends OpcrPackagePart {
pub struct OxfrCustomPart{base:OpcrPackagePart}
impl OxfrCustomPart {
    // java public OxfrCustomPart(ZipFile zipFile, ZipEntry entry) { super(zipFile, entry); }
    pub fn OxfrCustomPart(zipFile:&ZipFile,entry:&ZipEntry)->Self{Self{base:OpcrPackagePart::OpcrPackagePart(zipFile,entry)}}
}
impl std::ops::Deref for OxfrCustomPart{type Target=OpcrPackagePart;fn deref(&self)->&Self::Target{&self.base}}
impl std::ops::DerefMut for OxfrCustomPart{fn deref_mut(&mut self)->&mut Self::Target{&mut self.base}}
