use super::ClipRelationsBase::ClipRelationsBase;
use crate::kfc::{ooxfr::{IOxfRelationsBase::IOxfRelationsBase,IOxfXlsRelations::IOxfXlsRelations},opcw::native::RelationRef};
// java public class XlsClipRelationsImpl extends ClipRelationsBase implements IOxfXlsRelations {
pub struct XlsClipRelationsImpl{base:ClipRelationsBase}
impl XlsClipRelationsImpl{
    // java public XlsClipRelationsImpl() { super(); }
    pub fn XlsClipRelationsImpl()->Self{Self{base:ClipRelationsBase::ClipRelationsBase()}}
}
impl std::ops::Deref for XlsClipRelationsImpl{type Target=ClipRelationsBase;fn deref(&self)->&Self::Target{&self.base}}
impl std::ops::DerefMut for XlsClipRelationsImpl{fn deref_mut(&mut self)->&mut Self::Target{&mut self.base}}
// Native virtual inheritance forwarding, with no added Java subclass decisions.
macro_rules! inherit{($($method:ident),*$(,)?)=>{impl IOxfRelationsBase for XlsClipRelationsImpl{$(fn $method(&mut self)->RelationRef{self.base.$method()})*}};}
inherit!{chart,chartColor,chartStyle,chartExternalSrc,diagramQuickStyle,diagramData,diagramLayout,diagramColors,diagramDrawing,oleObjectDocx,oleObjectDoc,oleObjectDocm,imageEmf,imageWmf,imagePict,imageJpeg,imagePng,imageDib,imageGif,imageBmp,wdp,theme,themeOverride,ink,oleObjectBin}
impl IOxfXlsRelations for XlsClipRelationsImpl{}
