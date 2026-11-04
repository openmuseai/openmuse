use crate::kfc::opcw::native::RelationRef;
use super::{IOxfRelationsBase::IOxfRelationsBase,IOxfXlsRelations::IOxfXlsRelations,OxfXlRelations as R};
// java public class OxfXlsRelationsImpl implements IOxfXlsRelations {
pub struct OxfXlsRelationsImpl;
impl OxfXlsRelationsImpl{
    // java public OxfXlsRelationsImpl() { super(); }
    pub fn OxfXlsRelationsImpl()->Self{Self}
}
impl IOxfRelationsBase for OxfXlsRelationsImpl{
    // java public OpcRelation chart() { return OxfXlRelations.CHART; }
    fn chart(&mut self)->RelationRef{RelationRef::Static(&*R::CHART)}
    // java public OpcRelation chartColor() { return OxfXlRelations.CHART_COLOR; }
    fn chartColor(&mut self)->RelationRef{RelationRef::Static(&*R::CHART_COLOR)}
    // java public OpcRelation chartStyle() { return OxfXlRelations.CHART_STYLE; }
    fn chartStyle(&mut self)->RelationRef{RelationRef::Static(&*R::CHART_STYLE)}
    // java public OpcRelation chartExternalSrc() { return OxfXlRelations.BIN_SRC; }
    fn chartExternalSrc(&mut self)->RelationRef{RelationRef::Static(&*R::BIN_SRC)}
    // java public OpcRelation diagramQuickStyle() { return OxfXlRelations.DIAGRAMSQUICKSTYLE; }
    fn diagramQuickStyle(&mut self)->RelationRef{RelationRef::Static(&*R::DIAGRAMSQUICKSTYLE)}
    // java public OpcRelation diagramData() { return OxfXlRelations.DIAGRAMSDATA; }
    fn diagramData(&mut self)->RelationRef{RelationRef::Static(&*R::DIAGRAMSDATA)}
    // java public OpcRelation diagramLayout() { return OxfXlRelations.DIAGRAMSLAYOUT; }
    fn diagramLayout(&mut self)->RelationRef{RelationRef::Static(&*R::DIAGRAMSLAYOUT)}
    // java public OpcRelation diagramColors() { return OxfXlRelations.DIAGRAMSCOLORS; }
    fn diagramColors(&mut self)->RelationRef{RelationRef::Static(&*R::DIAGRAMSCOLORS)}
    // java public OpcRelation diagramDrawing() { return OxfXlRelations.DIAGRAMSDRAWING; }
    fn diagramDrawing(&mut self)->RelationRef{RelationRef::Static(&*R::DIAGRAMSDRAWING)}
    // java public OpcRelation oleObjectDocx() { return OxfXlRelations.DOCX_SRC; }
    fn oleObjectDocx(&mut self)->RelationRef{RelationRef::Static(&*R::DOCX_SRC)}
    // java public OpcRelation oleObjectDoc() { return OxfXlRelations.DOC_SRC; }
    fn oleObjectDoc(&mut self)->RelationRef{RelationRef::Static(&*R::DOC_SRC)}
    // java public OpcRelation oleObjectDocm() { return OxfXlRelations.DOCM_SRC; }
    fn oleObjectDocm(&mut self)->RelationRef{RelationRef::Static(&*R::DOCM_SRC)}
    // java public OpcRelation imageEmf() { return OxfXlRelations.IMAGE_EMF; }
    fn imageEmf(&mut self)->RelationRef{RelationRef::Static(&*R::IMAGE_EMF)}
    // java public OpcRelation imageWmf() { return OxfXlRelations.IMAGE_WMF; }
    fn imageWmf(&mut self)->RelationRef{RelationRef::Static(&*R::IMAGE_WMF)}
    // java public OpcRelation imagePict() { return OxfXlRelations.IMAGE_PICT; }
    fn imagePict(&mut self)->RelationRef{RelationRef::Static(&*R::IMAGE_PICT)}
    // java public OpcRelation imageJpeg() { return OxfXlRelations.IMAGE_JPEG; }
    fn imageJpeg(&mut self)->RelationRef{RelationRef::Static(&*R::IMAGE_JPEG)}
    // java public OpcRelation imagePng() { return OxfXlRelations.IMAGE_PNG; }
    fn imagePng(&mut self)->RelationRef{RelationRef::Static(&*R::IMAGE_PNG)}
    // java public OpcRelation imageDib() { return OxfXlRelations.IMAGE_DIB; }
    fn imageDib(&mut self)->RelationRef{RelationRef::Static(&*R::IMAGE_DIB)}
    // java public OpcRelation imageGif() { return OxfXlRelations.IMAGE_GIF; }
    fn imageGif(&mut self)->RelationRef{RelationRef::Static(&*R::IMAGE_GIF)}
    // java public OpcRelation imageBmp() { return OxfXlRelations.IMAGE_BMP; }
    fn imageBmp(&mut self)->RelationRef{RelationRef::Static(&*R::IMAGE_BMP)}
    // java public OpcRelation wdp() { return OxfXlRelations.HDPHOTO; }
    fn wdp(&mut self)->RelationRef{RelationRef::Static(&*R::HDPHOTO)}
    // java public OpcRelation theme() { return OxfXlRelations.THEME; }
    fn theme(&mut self)->RelationRef{RelationRef::Static(&*R::THEME)}
    // java public OpcRelation themeOverride() { return OxfXlRelations.THEMEOVERRIDE; }
    fn themeOverride(&mut self)->RelationRef{RelationRef::Static(&*R::THEMEOVERRIDE)}
    // java public OpcRelation ink() { return OxfXlRelations.INK; }
    fn ink(&mut self)->RelationRef{RelationRef::Static(&*R::INK)}
    // java public OpcRelation oleObjectBin() { return OxfXlRelations.BIN_SRC; }
    fn oleObjectBin(&mut self)->RelationRef{RelationRef::Static(&*R::BIN_SRC)}
}
impl IOxfXlsRelations for OxfXlsRelationsImpl{}
