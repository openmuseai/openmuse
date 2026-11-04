#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

// Java: import cn.wps.drawing.app.DgStore
use crate::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::compile_surface::DgStore;
// Java: import cn.wps.drawing.app.diagrams.DiagramPair
use crate::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::compile_surface::DiagramPair;
// Java: import cn.wps.drawing.data.blip.Blip
use crate::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::compile_surface::Blip;
// Java: import cn.wps.drawing.data.hyperlink.Hyperlink
use crate::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::compile_surface::Hyperlink;
// Java: import cn.wps.drawing.data.media.Media
use crate::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::compile_surface::{AudioFile, EmbeddedWAVAudioFile, Media, QuickTimeFile, VideoFile};
// Java: import cn.wps.drawing.data.model3d.Model3D
use crate::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::compile_surface::Model3D;
// Java: import cn.wps.drawing.data.shape.Shape
use crate::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::compile_surface::Shape;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
// rust: no-stmt — HandlerCallbackImpl owns the impl; this class forwards
// Java: import cn.wps.kfc.opcr.OpcrRelationships
use crate::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::compile_surface::OpcrRelationships;
// Java: import cn.wps.show.app.Ink.KmoInkLib
use crate::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::compile_surface::KmoInkLib;
// Java: import cn.wps.show.app.media.KmoMediaLib
use crate::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::compile_surface::KmoMediaLib;
// Java: import cn.wps.show.data.custdatalst.CustDataLst
use crate::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::compile_surface::TagLst;
// Java: import cn.wps.show.pptxr.ppt.media.PptxrMedias
use crate::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::compile_surface::PptxrMedias;
// Java: import cn.wps.util.manager.FileManager
use crate::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::compile_surface::FileManager;
// Java: import java.util.Map
// Java: import cn.wps.show.pptxr.ppt.slides.SlideHandlerCallback
// rust: W6c-1 去假面（形态 G）——原先指向 `HandlerCallbackImpl::compile_surface` 的内联影子，
//       现指向真实类型 `ppt/slides/SlideHandlerCallback.rs`（Java 9 行真身）。
use crate::cn::wps::show::pptxr::ppt::slides::SlideHandlerCallback::SlideHandlerCallback;
// Java: import cn.wps.show.pptxr.ppt.slides.PptxrSlides
use crate::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::compile_surface::PptxrSlides;
// Java: import cn.wps.show.pptxr.ppt.slides.HandlerCallbackImpl
use crate::cn::wps::show::pptxr::ppt::slides::HandlerCallbackImpl::HandlerCallbackImpl;
use std::collections::{HashMap};

// java public class SlideHandlerCallbackImpl extends SlideHandlerCallback
#[derive(Default)]
pub struct SlideHandlerCallbackImpl {
    // java SlideHandlerCallback super_
    pub(crate) super_: SlideHandlerCallback,
    // java private HandlerCallback handlerCallbackImpl;
    handlerCallbackImpl: HandlerCallbackImpl,
}

// java public class SlideHandlerCallbackImpl extends SlideHandlerCallback
impl SlideHandlerCallbackImpl {
    // java public  SlideHandlerCallbackImpl(DgStore dgStore, KmoMediaLib mediaLib, KmoInkLib inkLib, OpcrRelationships relationships, PptxrMedias medias, PptxrSlides slides, int mode)
    pub fn SlideHandlerCallbackImpl(dgStore: Option<DgStore>, mediaLib: Option<KmoMediaLib>, inkLib: Option<KmoInkLib>, relationships: Option<OpcrRelationships>, medias: Option<PptxrMedias>, slides: Option<PptxrSlides>, mode: i32) -> Self {
      // java super(mode);
      // java this.handlerCallbackImpl = new HandlerCallbackImpl(dgStore, mediaLib, inkLib, relationships, medias, slides, mode);
      Self { super_: SlideHandlerCallback::new(mode), handlerCallbackImpl: HandlerCallbackImpl::HandlerCallbackImpl(dgStore, mediaLib, inkLib, relationships, medias, slides, mode) }
    }

    // java public void load()
    pub fn load(&mut self) {
      // java this.handlerCallbackImpl.load();
      self.handlerCallbackImpl.load();
    }

    // java public void onTags(String id, CustDataLst.TagLst tagLst)
    pub fn onTags(&mut self, id: String, tagLst: TagLst) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.handlerCallbackImpl.onTags(id, tagLst);
      self.handlerCallbackImpl.onTags(id, tagLst);
      */
    }

    // java public void onBlipEmbed(String embed, Blip blip)
    pub fn onBlipEmbed(&mut self, embed: String, blip: &mut Blip) {
      // java this.handlerCallbackImpl.onBlipEmbed(embed, blip);
      self.handlerCallbackImpl.onBlipEmbed(embed, blip);
    }

    // java public void onBlipLink(String link, Blip blip)
    pub fn onBlipLink(&mut self, link: String, blip: &mut Blip) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.handlerCallbackImpl.onBlipLink(link, blip);
      self.handlerCallbackImpl.onBlipLink(link, blip);
      */
    }

    // java public void onAudioFileLink(String link, Media.AudioFile audioFile)
    pub fn onAudioFileLink(&mut self, link: String, audioFile: &mut AudioFile) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.handlerCallbackImpl.onAudioFileLink(link, audioFile);
      self.handlerCallbackImpl.onAudioFileLink(link, audioFile);
      */
    }

    // java public void onExtMediaFileLink(String link, Media media)
    pub fn onExtMediaFileLink(&mut self, link: String, media: &mut Media) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.handlerCallbackImpl.onExtMediaFileLink(link, media);
      self.handlerCallbackImpl.onExtMediaFileLink(link, media);
      */
    }

    // java public void onQuickTimeFileLink(String link, Media.QuickTimeFile quickTimeFile)
    pub fn onQuickTimeFileLink(&mut self, link: String, quickTimeFile: &mut QuickTimeFile) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.handlerCallbackImpl.onQuickTimeFileLink(link, quickTimeFile);
      self.handlerCallbackImpl.onQuickTimeFileLink(link, quickTimeFile);
      */
    }

    // java public void onVideoFileLink(String link, Media.VideoFile videoFile)
    pub fn onVideoFileLink(&mut self, link: String, videoFile: &mut VideoFile) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.handlerCallbackImpl.onVideoFileLink(link, videoFile);
      self.handlerCallbackImpl.onVideoFileLink(link, videoFile);
      */
    }

    // java public void onEmbeddedWAVAudioFileEmbed(String embed, Media.EmbeddedWAVAudioFile embeddedWAVAudioFile)
    pub fn onEmbeddedWAVAudioFileEmbed(&mut self, embed: String, embeddedWAVAudioFile: &mut EmbeddedWAVAudioFile) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.handlerCallbackImpl.onEmbeddedWAVAudioFileEmbed(embed, embeddedWAVAudioFile);
      self.handlerCallbackImpl.onEmbeddedWAVAudioFileEmbed(embed, embeddedWAVAudioFile);
      */
    }

    // java public void onHyperlinkRid(String rId, Hyperlink hyperlink)
    pub fn onHyperlinkRid(&mut self, rId: String, hyperlink: &mut Hyperlink) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.handlerCallbackImpl.onHyperlinkRid(rId, hyperlink);
      self.handlerCallbackImpl.onHyperlinkRid(rId, hyperlink);
      */
    }

    // java public void onHyperlinkAction(String action, Hyperlink hyperlink)
    pub fn onHyperlinkAction(&mut self, action: String, hyperlink: &mut Hyperlink) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.handlerCallbackImpl.onHyperlinkAction(action, hyperlink);
      self.handlerCallbackImpl.onHyperlinkAction(action, hyperlink);
      */
    }

    // java public void onShapeId(Integer originShapeId, Integer coreShapeId)
    pub fn onShapeId(&mut self, originShapeId: i32, coreShapeId: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.handlerCallbackImpl.onShapeId(originShapeId, coreShapeId);
      self.handlerCallbackImpl.onShapeId(originShapeId, coreShapeId);
      */
    }

    // java public Integer getCoreShapeId(Integer originShapeId)
    pub fn getCoreShapeId(&self, originShapeId: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.handlerCallbackImpl.getCoreShapeId(originShapeId);
      return self.handlerCallbackImpl.getCoreShapeId(originShapeId);
      */
    }

    // java public void onConnectorShape(Integer connectorShapeId)
    pub fn onConnectorShape(&mut self, connectorShapeId: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.handlerCallbackImpl.onConnectorShape(connectorShapeId);
      self.handlerCallbackImpl.onConnectorShape(connectorShapeId);
      */
    }

    // java public int onOleObject(String rId)
    pub fn onOleObject(&mut self, rId: String) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.handlerCallbackImpl.onOleObject(rId);
      return self.handlerCallbackImpl.onOleObject(rId);
      */
    }

    // java public void onChartRelationShip(String rId, Shape shape)
    pub fn onChartRelationShip(&mut self, rId: String, shape: Shape) {
      // java this.handlerCallbackImpl.onChartRelationShip(rId, shape);
      self.handlerCallbackImpl.onChartRelationShip(rId, shape);
    }

    // java public Map<String, Shape> getChartMap()
    pub fn getChartMap(&self) -> &std::collections::HashMap<String, Shape> {
      // java return this.handlerCallbackImpl.getChartMap();
      return self.handlerCallbackImpl.getChartMap();
    }

    // java public Map<Integer, DiagramPair> getDiagramPairs()
    pub fn getDiagramPairs(&self) -> &std::collections::HashMap<i32, DiagramPair> {
      // java return this.handlerCallbackImpl.getDiagramPairs();
      return self.handlerCallbackImpl.getDiagramPairs();
    }

    // java public void addDiagramPair(int smartArtId, String colorsId, String dataId, String layoutId, String quickId)
    pub fn addDiagramPair(&mut self, smartArtId: i32, colorsId: String, dataId: String, layoutId: String, quickId: String) {
      // java this.handlerCallbackImpl.addDiagramPair(smartArtId, colorsId, dataId, layoutId, quickId);
      self.handlerCallbackImpl.addDiagramPair(smartArtId, colorsId, dataId, layoutId, quickId);
    }

    // java public int onInk(String rId, FileManager fileManager)
    pub fn onInk(&mut self, rId: String, fileManager: FileManager) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.handlerCallbackImpl.onInk(rId, fileManager);
      return self.handlerCallbackImpl.onInk(rId, fileManager);
      */
    }

    // java public void onModel3D(String rId, Model3D model3D)
    pub fn onModel3D(&mut self, rId: String, model3D: &mut Model3D) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.handlerCallbackImpl.onModel3D(rId, model3D);
      self.handlerCallbackImpl.onModel3D(rId, model3D);
      */
    }

}

impl std::ops::Deref for SlideHandlerCallbackImpl {
    type Target = SlideHandlerCallback;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for SlideHandlerCallbackImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
