use crate::kfc::{opcw::{OpcwPackagePart::OpcwPackagePart,native::{SharedZip,RelationRef}},ooxfr::OxfRelations};
// java public final class OxfwCustomPart extends OpcwPackagePart {
pub struct OxfwCustomPart{pub(crate) base:OpcwPackagePart}
impl OxfwCustomPart{
    // java public OxfwCustomPart(ZipOutputStream zipStream) { super(zipStream, OxfRelations.CUSTOM, -1); }
    pub fn OxfwCustomPart(zipStream:Option<SharedZip>)->Self{Self{base:OpcwPackagePart::OpcwPackagePart(zipStream,RelationRef::Static(&*OxfRelations::CUSTOM),-1)}}
}
impl std::ops::Deref for OxfwCustomPart{type Target=OpcwPackagePart;fn deref(&self)->&Self::Target{&self.base}}
impl std::ops::DerefMut for OxfwCustomPart{fn deref_mut(&mut self)->&mut Self::Target{&mut self.base}}
