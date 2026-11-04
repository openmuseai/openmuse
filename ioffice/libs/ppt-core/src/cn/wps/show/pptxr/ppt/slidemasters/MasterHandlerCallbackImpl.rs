#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

// Java: import cn.wps.drawing.app.DgStore
use crate::cn::wps::show::pptxr::ppt::slidemasters::PptxrMaster::compile_surface::DgStore;
// Java: import cn.wps.kfc.opcr.OpcrRelationships
use crate::cn::wps::show::pptxr::ppt::slidemasters::PptxrMaster::compile_surface::OpcrRelationships;
// Java: import cn.wps.show.app.Ink.KmoInkLib
use crate::cn::wps::show::pptxr::ppt::slidemasters::PptxrMaster::compile_surface::KmoInkLib;
// Java: import cn.wps.show.app.media.KmoMediaLib
use crate::cn::wps::show::pptxr::ppt::slidemasters::PptxrMaster::compile_surface::KmoMediaLib;
// Java: import cn.wps.show.pptxr.ppt.media.PptxrMedias
use crate::cn::wps::show::pptxr::ppt::slidemasters::PptxrMaster::compile_surface::PptxrMedias;
// Java: import cn.wps.show.pptxr.ppt.slides.PptxrSlides
use crate::cn::wps::show::pptxr::ppt::slidemasters::PptxrMaster::compile_surface::PptxrSlides;
// Java: import cn.wps.show.pptxr.ppt.slides.SlideHandlerCallbackImpl
use crate::cn::wps::show::pptxr::ppt::slidemasters::PptxrMaster::compile_surface::SlideHandlerCallbackImpl;
// Java: import java.io.IOException
use crate::cn::wps::show::pptxr::ppt::slidemasters::PptxrMaster::compile_surface::IOException;
// Java: import java.util.Map
// Java: import cn.wps.show.pptxr.ppt.slidemasters.PptxrMaster
use crate::cn::wps::show::pptxr::ppt::slidemasters::PptxrMaster::PptxrMaster;
use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::RefCell;

// java public class MasterHandlerCallbackImpl extends SlideHandlerCallbackImpl
#[derive(Default, Clone)]
pub struct MasterHandlerCallbackImpl {
    // java SlideHandlerCallbackImpl super_
    pub(crate) super_: SlideHandlerCallbackImpl,
    // java private PptxrMaster masterReader;
    masterReader: Option<Rc<RefCell<PptxrMaster>>>,
}

// java public class MasterHandlerCallbackImpl extends SlideHandlerCallbackImpl
impl MasterHandlerCallbackImpl {
    // java public  MasterHandlerCallbackImpl(DgStore dgStore, KmoMediaLib mediaLib, KmoInkLib inkLib, OpcrRelationships relationships, PptxrMaster masterReader, PptxrMedias medias, PptxrSlides slides, int mode)
    pub fn MasterHandlerCallbackImpl(dgStore: Option<DgStore>, mediaLib: Option<KmoMediaLib>, inkLib: Option<KmoInkLib>, relationships: Option<OpcrRelationships>, masterReader: Option<Rc<RefCell<PptxrMaster>>>, medias: Option<PptxrMedias>, slides: Option<PptxrSlides>, mode: i32) -> Self {
      // java super(dgStore, mediaLib, inkLib, relationships, medias, slides, mode);
      // java this.masterReader = masterReader;
      Self { super_: SlideHandlerCallbackImpl::new(dgStore, mediaLib, inkLib, relationships, medias, slides, mode), masterReader: masterReader }
    }

    pub fn load(&self) {
        self.super_.load();
    }

    // java public void onSlideLayoutIds(Map<String, Long> ids)
    pub fn onSlideLayoutIds(&mut self, ids: std::collections::HashMap<String, i64>) {
      // java if (this.masterReader != null)
      if let Some(masterReader) = self.masterReader.as_ref() {
            // java this.masterReader.onSlideLayoutIds(ids);
            masterReader.borrow_mut().onSlideLayoutIds(ids);
            // java IOException e
            // rust: no-stmt
              // java e.printStackTrace();
              // rust: no-stmt
      }
    }

}
