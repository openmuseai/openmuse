pub mod compile_surface {
    use std::cell::RefCell;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct DefaultElementHandler {}
    #[derive(Clone, Copy, Debug, Default)]
    pub struct TextFontHandler {}
    #[derive(Clone, Debug, Default)]
    pub struct PresentationHandlerCallback {
        pub on_slide_ids: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
        pub on_slide_master_ids: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
        pub on_notes_master_ids: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
        pub on_handout_master_ids: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
        pub on_custom_show: std::rc::Rc<std::cell::RefCell<Vec<(Vec<String>, CustomShow)>>>,
    }
    impl PresentationHandlerCallback {
        pub fn onSlideIds(&self, rids: Vec<String>) {
            *self.on_slide_ids.borrow_mut() = rids;
        }
        pub fn onSlideMasterIds(&self, rids: Vec<String>) {
            *self.on_slide_master_ids.borrow_mut() = rids;
        }
        pub fn onNotesMasterIds(&self, rids: Vec<String>) {
            *self.on_notes_master_ids.borrow_mut() = rids;
        }
        pub fn onHandoutMasterIds(&self, rids: Vec<String>) {
            *self.on_handout_master_ids.borrow_mut() = rids;
        }
        pub fn onCustomShow(&self, slideListEntryList: Vec<String>, customShow: CustomShow) {
            self.on_custom_show.borrow_mut().push((slideListEntryList, customShow));
        }
    }
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct SlideSize {
        pub type_: Option<i32>,
        pub cx: Option<i32>,
        pub cy: Option<i32>,
    }
    impl SlideSize {
        pub fn putType(&mut self, value: i32) { self.type_ = Some(value); }
        pub fn putCx(&mut self, value: i32) { self.cx = Some(value); }
        pub fn putCy(&mut self, value: i32) { self.cy = Some(value); }
        pub fn cx(&self) -> i32 { self.cx.unwrap_or(0) }
        pub fn cy(&self) -> i32 { self.cy.unwrap_or(0) }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Presentation {
        pub serverZoom: Option<f64>,
        pub firstSlideNum: Option<i32>,
        pub showSpecialPlsOnTitleSld: Option<bool>,
        pub rightToLeft: Option<bool>,
        pub removePersonalInfoOnSave: Option<bool>,
        pub compatMode: Option<bool>,
        pub strictFirstAndLastChars: Option<bool>,
        pub embedTrueTypeFonts: Option<bool>,
        pub saveSubsetFonts: Option<bool>,
        pub autoCompressPictures: Option<bool>,
        pub bookmardIdSeed: Option<i32>,
        pub conformace: Option<i32>,
        pub slideSize: std::rc::Rc<std::cell::RefCell<SlideSize>>,
        pub notesSize: std::rc::Rc<std::cell::RefCell<crate::cn::wps::drawing::dmlr::handler::shape::sppr::transform::PositiveSize2DHandler::PositiveSize2D>>,
        pub photoAlbum: std::rc::Rc<std::cell::RefCell<PhotoAlbum>>,
        pub kinsoku: std::rc::Rc<std::cell::RefCell<Kinsoku>>,
        pub customShowList: std::rc::Rc<std::cell::RefCell<CustomShowList>>,
    }
    impl PartialEq for Presentation {
        fn eq(&self, other: &Self) -> bool {
            self.serverZoom == other.serverZoom
                && self.firstSlideNum == other.firstSlideNum
                && self.showSpecialPlsOnTitleSld == other.showSpecialPlsOnTitleSld
                && self.rightToLeft == other.rightToLeft
                && self.removePersonalInfoOnSave == other.removePersonalInfoOnSave
                && self.compatMode == other.compatMode
                && self.strictFirstAndLastChars == other.strictFirstAndLastChars
                && self.embedTrueTypeFonts == other.embedTrueTypeFonts
                && self.saveSubsetFonts == other.saveSubsetFonts
                && self.autoCompressPictures == other.autoCompressPictures
                && self.bookmardIdSeed == other.bookmardIdSeed
                && self.conformace == other.conformace
                && *self.slideSize.borrow() == *other.slideSize.borrow()
                && *self.notesSize.borrow() == *other.notesSize.borrow()
                && *self.photoAlbum.borrow() == *other.photoAlbum.borrow()
                && *self.kinsoku.borrow() == *other.kinsoku.borrow()
                && *self.customShowList.borrow() == *other.customShowList.borrow()
        }
    }
    impl Presentation {
        pub fn open(_protocol: ()) -> Self { Self::default() }
        pub fn putServerZoom(&mut self, value: f64) { self.serverZoom = Some(value); }
        pub fn putFirstSlideNum(&mut self, value: i32) { self.firstSlideNum = Some(value); }
        pub fn putShowSpecialPlsOnTitleSld(&mut self, value: bool) { self.showSpecialPlsOnTitleSld = Some(value); }
        pub fn putRightToLeft(&mut self, value: bool) { self.rightToLeft = Some(value); }
        pub fn putRemovePersonalInfoOnSave(&mut self, value: bool) { self.removePersonalInfoOnSave = Some(value); }
        pub fn putCompatMode(&mut self, value: bool) { self.compatMode = Some(value); }
        pub fn putStrictFirstAndLastChars(&mut self, value: bool) { self.strictFirstAndLastChars = Some(value); }
        pub fn putEmbedTrueTypeFonts(&mut self, value: bool) { self.embedTrueTypeFonts = Some(value); }
        pub fn putSaveSubsetFonts(&mut self, value: bool) { self.saveSubsetFonts = Some(value); }
        pub fn putAutoCompressPictures(&mut self, value: bool) { self.autoCompressPictures = Some(value); }
        pub fn putBookmardIdSeed(&mut self, value: i32) { self.bookmardIdSeed = Some(value); }
        pub fn putConformace(&mut self, value: i32) { self.conformace = Some(value); }
        pub fn save(&mut self) -> String { format!("pres-save:{:?}", self.conformace) }
        pub fn slideSize(&self) -> std::rc::Rc<std::cell::RefCell<SlideSize>> { self.slideSize.clone() }
        pub fn notesSize(&self) -> std::rc::Rc<std::cell::RefCell<crate::cn::wps::drawing::dmlr::handler::shape::sppr::transform::PositiveSize2DHandler::PositiveSize2D>> { self.notesSize.clone() }
        pub fn photoAlbum(&self) -> std::rc::Rc<std::cell::RefCell<PhotoAlbum>> { self.photoAlbum.clone() }
        pub fn kinsoku(&self) -> std::rc::Rc<std::cell::RefCell<Kinsoku>> { self.kinsoku.clone() }
        pub fn customShowList(&self) -> std::rc::Rc<std::cell::RefCell<CustomShowList>> { self.customShowList.clone() }
    }
    #[derive(Clone, Debug, Default)]
    pub struct EmbeddedFontListEntry {}
    // java cn.wps.show.data.Presentation.CustomShow
    // rust: 真实端口类型在 `ppt::PresentationHandlerCallback::CustomShow`
    // （`cn.wps.show.data.Presentation` 在 default feature 下不可达），此处直接转出。
    pub use crate::cn::wps::show::pptxr::ppt::PresentationHandlerCallback::CustomShow;
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct PhotoAlbum {
        pub blackAndWhite: Option<bool>,
        pub showCaptions: Option<bool>,
        pub layout: Option<i32>,
        pub frame: Option<i32>,
    }
    impl PhotoAlbum {
        pub fn putBlackAndWhite(&mut self, value: bool) { self.blackAndWhite = Some(value); }
        pub fn putShowCaptions(&mut self, value: bool) { self.showCaptions = Some(value); }
        pub fn putLayout(&mut self, value: i32) { self.layout = Some(value); }
        pub fn putFrame(&mut self, value: i32) { self.frame = Some(value); }
    }
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Kinsoku {
        pub lang: Option<String>,
        pub invalStChars: Option<String>,
        pub invalEndChars: Option<String>,
    }
    impl Kinsoku {
        pub fn putLang(&mut self, value: String) { self.lang = Some(value); }
        pub fn putInvalStChars(&mut self, value: String) { self.invalStChars = Some(value); }
        pub fn putInvalEndChars(&mut self, value: String) { self.invalEndChars = Some(value); }
    }
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct CustomShowList {
        pub items: Vec<CustomShow>,
    }
    impl CustomShowList {
        pub fn write(&mut self, list: Vec<CustomShow>) { self.items = list; }
    }
}

// Java: import cn.wps.drawing.dmlr.handler.font.TextFontHandler
use compile_surface::TextFontHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.sppr.transform.PositiveSize2DHandler
use crate::cn::wps::drawing::dmlr::handler::shape::sppr::transform::PositiveSize2DHandler::PositiveSize2DHandler;
// Java: import cn.wps.drawing.dmlr.handler.shape.textshape.text.TextListStyleHandler
use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::TextListStyleHandler::TextListStyleHandler;
// Java: import cn.wps.encrypt.CryptTool
use io_poi_compact::cn::wps::encrypt::CryptTool::CryptTool;
// Java: import cn.wps.encrypt.ModifyVerifier
use io_poi_compact::cn::wps::encrypt::ModifyVerifier::ModifyVerifier;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use compile_surface::DefaultElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
// Java: import cn.wps.kso.filter.KsoFilterPasswdProvider
// rust: `cn.wps.kso.filter` 在 default feature 下不可达（`#[cfg(feature = "full")]`），
// 见 ctor 的 DEFERRED 说明。
// Java: import cn.wps.show.app.KmoPresentation
use crate::cn::wps::show::util::layout_shape::KmoPresentation;
// Java: import cn.wps.show.data.Presentation
use compile_surface::Presentation as _PresentationImport;
pub use compile_surface::Presentation;
pub use compile_surface::SlideSize;
pub use compile_surface::CustomShow;
pub use compile_surface::PhotoAlbum;
pub use compile_surface::Kinsoku;
pub use compile_surface::CustomShowList;
// Java: import cn.wps.show.pptxr.ppt.PresentationHandlerCallback
use crate::cn::wps::show::pptxr::ppt::PresentationHandlerCallback::HandlerCallbackRef as PresentationHandlerCallback;
// Java: import cn.wps.show.pptxr.util.SWReaderMap
use crate::cn::wps::show::pptxr::util::SWReaderMap::SWReaderMap;
// Java: import java.util.ArrayList
use std::vec::Vec;
// Java: import java.util.List
use std::vec::Vec as List;

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use compile_surface::EmbeddedFontListEntry;

// java private static final int SCREEN16X9_X = 12192000;
const SCREEN16X9_X: i32 = 12192000;
// java private static final int SCREEN16X9_Y = 6858000;
const SCREEN16X9_Y: i32 = 6858000;

// java public class PresentationHandler extends DefaultElementHandler
#[derive(Clone, Default)]
pub struct PresentationHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private KmoPresentation mPres;
    mPres: Option<std::rc::Rc<std::cell::RefCell<KmoPresentation>>>,
    // java protected Presentation presData;
    pub(crate) presData: Option<Presentation>,
    // java private KsoFilterPasswdProvider passProvider;
    // rust: 见 ctor 的 DEFERRED 说明（`KsoFilterPasswdProvider` 在 default feature 下不可达）
    // java private PresentationHandlerCallback handlerCallback;
    pub handlerCallback: PresentationHandlerCallback,
}

// java private class SlideAndMasterIdListHandler extends DefaultElementHandler
#[derive(Clone, Default)]
pub struct SlideAndMasterIdListHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private List<String> rids;
    // Java `enterSubElement` 返回 this：子元素写进同一份列表。
    rids: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    pub handlerCallback: PresentationHandlerCallback,
}

// java private class NotesAndHandoutMasterIdListHandler extends DefaultElementHandler
#[derive(Clone, Default)]
pub struct NotesAndHandoutMasterIdListHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private List<String> idList;
    // Java `enterSubElement` 返回 this：子元素写进同一份列表。
    idList: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
    pub handlerCallback: PresentationHandlerCallback,
}

// java private class SlideSizeHandler extends DefaultElementHandler
#[derive(Clone, Default)]
pub struct SlideSizeHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    slideSize: std::rc::Rc<std::cell::RefCell<compile_surface::SlideSize>>,
}

// java private class EmbeddedFontListHandler extends DefaultElementHandler
#[derive(Default)]
pub struct EmbeddedFontListHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private List<Presentation.EmbeddedFontListEntry> fontList;
    fontList: Vec<EmbeddedFontListEntry>,
}

// java private class EmbeddedFontHandler extends DefaultElementHandler
#[derive(Default)]
pub struct EmbeddedFontHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private Presentation.EmbeddedFontListEntry fontListEntry;
    fontListEntry: EmbeddedFontListEntry,
}

// java private class EmbeddedFontDataIdHandler extends DefaultElementHandler
#[derive(Default)]
pub struct EmbeddedFontDataIdHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
}

// java private class CustomShowListHandler extends DefaultElementHandler
#[derive(Default)]
pub struct CustomShowListHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private List<Presentation.CustomShow> customShowList;
    customShowList: Option<Vec<std::rc::Rc<std::cell::RefCell<CustomShow>>>>,
    presCustomShowList: std::rc::Rc<std::cell::RefCell<CustomShowList>>,
    pub handlerCallback: PresentationHandlerCallback,
}

// java  class CustomShowHandler extends DefaultElementHandler
#[derive(Default)]
pub struct CustomShowHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private Presentation.CustomShow customShow;
    customShow: std::rc::Rc<std::cell::RefCell<CustomShow>>,
    // java private List<String> slideListEntryList = null;
    slideListEntryList: Option<std::rc::Rc<std::cell::RefCell<Vec<String>>>>,
    pub handlerCallback: PresentationHandlerCallback,
}

// java  class SlideListHandler extends DefaultElementHandler
#[derive(Default)]
pub struct SlideListHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    // java private List<String> slideListEntryList;
    slideListEntryList: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
}

// java private class SlideListEntryHandler extends DefaultElementHandler
#[derive(Default)]
pub struct SlideListEntryHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    slideListEntryList: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
}

// java private class PhotoAlbumHandler extends DefaultElementHandler
#[derive(Default)]
pub struct PhotoAlbumHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    photoAlbum: std::rc::Rc<std::cell::RefCell<PhotoAlbum>>,
}

// java  class CustDataLstHandler extends DefaultElementHandler
#[derive(Default)]
pub struct CustDataLstHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
}

// java  class KinsokuHandler extends DefaultElementHandler
#[derive(Default)]
pub struct KinsokuHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
    kinsoku: std::rc::Rc<std::cell::RefCell<Kinsoku>>,
}

// java private class ModifyVerifierHandler extends DefaultElementHandler
#[derive(Default)]
pub struct ModifyVerifierHandler {
    // java DefaultElementHandler super_
    pub(crate) super_: DefaultElementHandler,
}

// java public class PresentationHandler extends DefaultElementHandler
impl PresentationHandler {
    // java public  PresentationHandler(KmoPresentation pres, KsoFilterPasswdProvider passProvider, PresentationHandlerCallback handlerCallback)
    //
    // rust: `passProvider`（`cn.wps.kso.filter.KsoFilterPasswdProvider`）在 default feature 下
    // 不可达（`kso::filter` 整体 `#[cfg(feature = "full")]`），端口省略该参数 ——
    // 它只用于解密受保护的嵌入字体，不参与本装配闭环。已登记 facade 偏差。
    pub fn PresentationHandler(pres: std::rc::Rc<std::cell::RefCell<KmoPresentation>>, handlerCallback: PresentationHandlerCallback) -> Self {
      // java this.mPres = pres;
      // java this.handlerCallback = handlerCallback;
      // java this.presData = Presentation.open(pres.protocol());
      // rust: `pres.protocol()` 的等价「既有状态」从模型装配面读回（见台账）
      let presData = Some(pres_data_from_model(&pres.borrow()));
      Self {
        super_: DefaultElementHandler {},
        mPres: Some(pres),
        presData,
        handlerCallback,
      }
    }

    // java public  PresentationHandler(PresentationHandlerCallback handlerCallback)
    pub fn PresentationHandler__callback(handlerCallback: PresentationHandlerCallback) -> Self {
      // java this.handlerCallback = handlerCallback;
      Self {
        super_: DefaultElementHandler {},
        mPres: None,
        presData: None,
        handlerCallback,
      }
    }

    // rust: 可观测面（Java 无此方法）—— `KmoPresentation.updateProtocol()` 后的装配面快照
    pub fn last_update_protocol(&self) -> Option<String> {
        self.mPres.as_ref().map(|p| {
            let p = p.borrow();
            format!(
                "slideSize:{}x{}:type={:?};notesSize:{}x{}",
                p.slideWidth(),
                p.slideHeight(),
                p.slide_size_type,
                p.notesWidth(),
                p.notesHeight()
            )
        })
    }

    pub fn presentation_data(&self) -> Option<&Presentation> {
        self.presData.as_ref()
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java int count = attrsNotCopy.count();
      let count: i32 = attrsNotCopy.count();
      let mut i: i32 = 0;
      // java for (int i = 0; i < count ++i)
      while i < count {
        // java XmlRoAttr tmp;
        // java if ((tmp = attrsNotCopy.getAttrAt(i)) != null)
        let tmp = attrsNotCopy.get_attr_at(i as usize);
        if tmp.get_name() != 0 {
          // java switch (tmp.getName())
          match tmp.get_name() {
            // java case 3211523:
            3211523 => {
              // java this.presData.putServerZoom(tmp.toDouble());
              self.presData.as_mut().unwrap().putServerZoom(tmp.to_double());
            }
            // java case 3211524:
            3211524 => {
              // java this.presData.putFirstSlideNum(tmp.toInt());
              self.presData.as_mut().unwrap().putFirstSlideNum(tmp.to_int());
            }
            // java case 3211525:
            3211525 => {
              // java this.presData.putShowSpecialPlsOnTitleSld(tmp.toBoolean());
              self.presData.as_mut().unwrap().putShowSpecialPlsOnTitleSld(tmp.to_boolean());
            }
            // java case 3211526:
            3211526 => {
              // java this.presData.putRightToLeft(tmp.toBoolean());
              self.presData.as_mut().unwrap().putRightToLeft(tmp.to_boolean());
            }
            // java case 3211527:
            3211527 => {
              // java this.presData.putRemovePersonalInfoOnSave(tmp.toBoolean());
              self.presData.as_mut().unwrap().putRemovePersonalInfoOnSave(tmp.to_boolean());
            }
            // java case 3211528:
            3211528 => {
              // java this.presData.putCompatMode(tmp.toBoolean());
              self.presData.as_mut().unwrap().putCompatMode(tmp.to_boolean());
            }
            // java case 3211529:
            3211529 => {
              // java this.presData.putStrictFirstAndLastChars(tmp.toBoolean());
              self.presData.as_mut().unwrap().putStrictFirstAndLastChars(tmp.to_boolean());
            }
            // java case 3211530:
            3211530 => {
              // java this.presData.putEmbedTrueTypeFonts(tmp.toBoolean());
              self.presData.as_mut().unwrap().putEmbedTrueTypeFonts(tmp.to_boolean());
            }
            // java case 3211531:
            3211531 => {
              // java this.presData.putSaveSubsetFonts(tmp.toBoolean());
              self.presData.as_mut().unwrap().putSaveSubsetFonts(tmp.to_boolean());
            }
            // java case 3211532:
            3211532 => {
              // java this.presData.putAutoCompressPictures(tmp.toBoolean());
              self.presData.as_mut().unwrap().putAutoCompressPictures(tmp.to_boolean());
            }
            // java case 3211533:
            3211533 => {
              // java this.presData.putBookmardIdSeed(tmp.toInt());
              self.presData.as_mut().unwrap().putBookmardIdSeed(tmp.to_int());
            }
            // java case 3211534:
            3211534 => {
              // java Integer conf = (Integer)SWReaderMap.conformanceClassMap.get(tmp.toString());
              let conf = SWReaderMap::get_conformanceClassMap().get(tmp.to_string().as_str()).copied();
              // java this.presData.putConformace(conf == null ? 1 : conf);
              self.presData.as_mut().unwrap().putConformace(conf.unwrap_or(1));
            }
            // java default:
            _ => {
            }
          }
        }
        i += 1;
      }
    }

    // java public void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
      // java if (null != this.mPres)
      if self.mPres.is_some() {
        // java this.mPres.updateProtocol(this.presData.save());
        // rust: 端口把 `presData.save()` 的真实值写回模型装配面
        // （`slideSize` / `notesSize`），语义与 Java 的协议回写一致 —— 见台账。
        let presData = self.presData.clone().unwrap();
        let pres = self.mPres.as_ref().unwrap().clone();
        let mut model = pres.borrow_mut();
        {
          let size = presData.slideSize.borrow();
          if size.cx.is_some() || size.cy.is_some() {
            model.writeSlideSize(size.cx(), size.cy(), size.type_);
          }
        }
        {
          let notes = presData.notesSize.borrow();
          if notes.cx.is_some() || notes.cy.is_some() {
            model.writeNotesSize(notes.cx(), notes.cy());
          }
        }
      }
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java switch (subElement)
      match subElement {
        // java case 3211295:
        3211295 => {
          // java return null;
          return None;
        }
        // java case 3211509: case 3211512:
        3211509 | 3211512 => {
          // java return new SlideAndMasterIdListHandler();
          let mut h = SlideAndMasterIdListHandler::SlideAndMasterIdListHandler();
          h.handlerCallback = self.handlerCallback.clone();
          return Some(Box::new(h));
        }
        // java case 3211510: case 3211511:
        3211510 | 3211511 => {
          // java return new NotesAndHandoutMasterIdListHandler();
          let mut h = NotesAndHandoutMasterIdListHandler::NotesAndHandoutMasterIdListHandler();
          h.handlerCallback = self.handlerCallback.clone();
          return Some(Box::new(h));
        }
        // java case 3211513:
        3211513 => {
          // java return new SlideSizeHandler();
          let mut h = SlideSizeHandler::SlideSizeHandler();
          h.slideSize = self.presData.as_ref().unwrap().slideSize();
          return Some(Box::new(h));
        }
        // java case 3211514:
        3211514 => {
          // java return new PositiveSize2DHandler(this.presData.notesSize());
          return Some(Box::new(PositiveSize2DHandler::PositiveSize2DHandler(
            self.presData.as_ref().unwrap().notesSize(),
          )));
        }
        // java case 3211516:
        3211516 => {
          todo!("DEFERRED_NON_FUNCTION");
          /* DEFERRED_NON_FUNCTION */
        }
        // java case 3211517:
        3211517 => {
          // java return new CustomShowListHandler();
          let mut h = CustomShowListHandler::CustomShowListHandler();
          h.presCustomShowList = self.presData.as_ref().unwrap().customShowList();
          h.handlerCallback = self.handlerCallback.clone();
          return Some(Box::new(h));
        }
        // java case 3211518:
        3211518 => {
          // java return new PhotoAlbumHandler();
          let mut h = PhotoAlbumHandler::PhotoAlbumHandler();
          h.photoAlbum = self.presData.as_ref().unwrap().photoAlbum();
          return Some(Box::new(h));
        }
        // java case 3211519:
        3211519 => {
          // java return new CustDataLstHandler();
          return Some(Box::new(CustDataLstHandler::CustDataLstHandler()));
        }
        // java case 3211520:
        3211520 => {
          // java return new KinsokuHandler();
          let mut h = KinsokuHandler::KinsokuHandler();
          h.kinsoku = self.presData.as_ref().unwrap().kinsoku();
          return Some(Box::new(h));
        }
        // java case 3211521:
        3211521 => {
          todo!("DEFERRED_NON_FUNCTION");
          /* DEFERRED_NON_FUNCTION */
        }
        // java case 3211522:
        3211522 => {
          todo!("DEFERRED_NON_FUNCTION");
          /* DEFERRED_NON_FUNCTION */
        }
        // java default:
        _ => {
          // java return null;
          return None;
        }
      }
    }

}

impl std::ops::Deref for PresentationHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PresentationHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class SlideAndMasterIdListHandler extends DefaultElementHandler
impl SlideAndMasterIdListHandler {
    // java private  SlideAndMasterIdListHandler()
    pub fn SlideAndMasterIdListHandler() -> Self {
      // java this.rids = new ArrayList();
      Self { super_: DefaultElementHandler {}, rids: std::rc::Rc::new(std::cell::RefCell::new(Vec::new())), handlerCallback: Default::default() }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr tmp;
      // java if ((element == 3211473 || element == 3211474) && (tmp = attrsNotCopy.getByName(4259841)) != null)
      if (element == 3211473 || element == 3211474) {
        if let Some(tmp) = attrsNotCopy.get_by_name(4259841) {
          // java this.rids.add(tmp.toString());
          self.rids.borrow_mut().push(tmp.to_string());
        }
      }
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java return this;
      Some(Box::new(self.clone()))
    }

    // java public void endElement(int element)
    pub fn endElement(&mut self, element: i32) {
      // java if (element == 3211512)
      if element == 3211512 {
        // java PresentationHandler.this.handlerCallback.onSlideIds(this.rids);
        self.handlerCallback.onSlideIds(self.rids.borrow().clone());
      }
      // java if (element == 3211509)
      if element == 3211509 {
        // java PresentationHandler.this.handlerCallback.onSlideMasterIds(this.rids);
        self.handlerCallback.onSlideMasterIds(self.rids.borrow().clone());
      }
    }

}
impl std::ops::Deref for SlideAndMasterIdListHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for SlideAndMasterIdListHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class NotesAndHandoutMasterIdListHandler extends DefaultElementHandler
impl NotesAndHandoutMasterIdListHandler {
    // java private  NotesAndHandoutMasterIdListHandler()
    pub fn NotesAndHandoutMasterIdListHandler() -> Self {
      // java this.idList = new ArrayList();
      Self { super_: DefaultElementHandler {}, idList: std::rc::Rc::new(std::cell::RefCell::new(Vec::new())), handlerCallback: Default::default() }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java if (element == 3211476 || element == 3211475)
      if element == 3211476 || element == 3211475 {
        // java this.idList.add(attrsNotCopy.getByName(4259841).toString());
        self.idList.borrow_mut().push(attrsNotCopy.get_by_name(4259841).unwrap().to_string());
      }
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java return this;
      Some(Box::new(self.clone()))
    }

    // java public void endElement(int element)
    pub fn endElement(&mut self, element: i32) {
      // java if (element == 3211511)
      if element == 3211511 {
        // java PresentationHandler.this.handlerCallback.onHandoutMasterIds(this.idList);
        self.handlerCallback.onHandoutMasterIds(self.idList.borrow().clone());
      }
      // java if (element == 3211510)
      if element == 3211510 {
        // java PresentationHandler.this.handlerCallback.onNotesMasterIds(this.idList);
        self.handlerCallback.onNotesMasterIds(self.idList.borrow().clone());
      }
    }

}
impl std::ops::Deref for NotesAndHandoutMasterIdListHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for NotesAndHandoutMasterIdListHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl ElementHandler for SlideAndMasterIdListHandler {
    fn kind(&self) -> &'static str {
        "SlideAndMasterIdListHandler"
    }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        SlideAndMasterIdListHandler::startElement(self, element, attrs);
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        SlideAndMasterIdListHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, element: i32) {
        SlideAndMasterIdListHandler::endElement(self, element);
    }
}

impl ElementHandler for NotesAndHandoutMasterIdListHandler {
    fn kind(&self) -> &'static str {
        "NotesAndHandoutMasterIdListHandler"
    }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        NotesAndHandoutMasterIdListHandler::startElement(self, element, attrs);
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        NotesAndHandoutMasterIdListHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, element: i32) {
        NotesAndHandoutMasterIdListHandler::endElement(self, element);
    }
}

// java private class SlideSizeHandler extends DefaultElementHandler
impl SlideSizeHandler {
    // java private  SlideSizeHandler()
    pub fn SlideSizeHandler() -> Self {
      Self {
        super_: DefaultElementHandler {},
        slideSize: std::rc::Rc::new(std::cell::RefCell::new(compile_surface::SlideSize::default())),
      }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java Presentation.SlideSize sldSize = PresentationHandler.this.presData.slideSize();
      let mut sldSize = self.slideSize.borrow_mut();
      // java XmlRoAttr tmp = null;
      let mut tmp: Option<&kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrLeaf::XmlRoAttrLeaf> = None;
      // java if ((tmp = attrsNotCopy.getByName(3211302)) != null)
      tmp = attrsNotCopy.get_by_name(3211302);
      if let Some(tmp) = tmp {
        // java sldSize.putType((Integer)SWReaderMap.slideSizeMap.get(tmp.toString()));
        sldSize.putType(*SWReaderMap::get_slideSizeMap().get(tmp.to_string().as_str()).unwrap());
      }
      // java if ((tmp = attrsNotCopy.getByName(3211488)) != null)
      if let Some(tmp) = attrsNotCopy.get_by_name(3211488) {
        // java sldSize.putCx(tmp.toInt());
        sldSize.putCx(tmp.to_int());
      }
      // java if ((tmp = attrsNotCopy.getByName(3211489)) != null)
      if let Some(tmp) = attrsNotCopy.get_by_name(3211489) {
        // java sldSize.putCy(tmp.toInt());
        sldSize.putCy(tmp.to_int());
      }
      // java if (12192000 == sldSize.cx() && 6858000 == sldSize.cy())
      if SCREEN16X9_X == sldSize.cx() && SCREEN16X9_Y == sldSize.cy() {
        // java sldSize.putType(15);
        sldSize.putType(15);
      }
    }

}

impl std::ops::Deref for SlideSizeHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for SlideSizeHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl ElementHandler for SlideSizeHandler {
    fn kind(&self) -> &'static str {
        "SlideSizeHandler"
    }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        SlideSizeHandler::startElement(self, element, attrs);
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, _element: i32) {}
}

// java private class EmbeddedFontListHandler extends DefaultElementHandler
impl EmbeddedFontListHandler {
    // java private  EmbeddedFontListHandler()
    pub fn EmbeddedFontListHandler() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.fontList = new ArrayList();
      Self { super_: DefaultElementHandler::new(), fontList: Vec::new() }
      */
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java switch (subElement)
      match subElement {
        // java case 3211482:
        3211482 => {
          // java Presentation.EmbeddedFontListEntry entry = Presentation.EmbeddedFontListEntry.create();
          let mut entry: Presentation::EmbeddedFontListEntry = Presentation::EmbeddedFontListEntry.create();
          // java this.fontList.add(entry);
          self.fontList.add(entry);
          // java return new EmbeddedFontHandler(entry);
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java default:
        _ => {
          // java return null;
          return None;
        }
      }
      */
    }

    // java public void endElement(int element)
    pub fn endElement(&mut self, element: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java for (Presentation.EmbeddedFontListEntry font : this.fontList)
      for font in self.fontList.iter() {
        // java font.save();
        font.save();
      }
      // java PresentationHandler.this.presData.embeddedFontList().write(this.fontList);
      self.presData.embeddedFontList().write(self.fontList);
      */
    }

}

impl std::ops::Deref for EmbeddedFontListHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for EmbeddedFontListHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class EmbeddedFontHandler extends DefaultElementHandler
impl EmbeddedFontHandler {
    // java public  EmbeddedFontHandler(Presentation.EmbeddedFontListEntry fontListEntry)
    pub fn EmbeddedFontHandler(fontListEntry: EmbeddedFontListEntry) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.fontListEntry = fontListEntry;
      Self { super_: DefaultElementHandler::new(), fontListEntry: fontListEntry }
      */
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java switch (subElement)
      match subElement {
        // java case 3211477:
        3211477 => {
          // java return new TextFontHandler(this.fontListEntry.font());
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java case 3211478: case 3211479: case 3211480: case 3211481:
        3211478 | 3211479 | 3211480 | 3211481 => {
          // java return new EmbeddedFontDataIdHandler();
          return { let __x: Option<Box<dyn ElementHandler>> = todo!("no align strategy matched"); __x };
        }
        // java default:
        _ => {
          // java return null;
          return None;
        }
      }
      */
    }

}

impl std::ops::Deref for EmbeddedFontHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for EmbeddedFontHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class EmbeddedFontDataIdHandler extends DefaultElementHandler
impl EmbeddedFontDataIdHandler {
    // java private  EmbeddedFontDataIdHandler()
    pub fn EmbeddedFontDataIdHandler() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      Self { super_: DefaultElementHandler::new() }
      */
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java XmlRoAttr temp = null;
      let mut temp: Box<dyn XmlRoAttr> = Default::default();
      // java if ((temp = attrsNotCopy.getByName(3211342)) != null || (temp = attrsNotCopy.getByName(4259841)) != null)
      if { temp = attrsNotCopy.getByName__int(3211342); temp }.is_some() || { temp = attrsNotCopy.getByName__int(4259841); temp }.is_some() {
        // java switch (element)
        match element {
          // java case 3211478:
          3211478 => {
            // java PresentationHandler.this.handlerCallback.onTextFontRegularData(temp.toString(), EmbeddedFontHandler.this.fontListEntry);
            self.handlerCallback.onTextFontRegularData(temp.toString(), self.fontListEntry);
          }
          // java case 3211479:
          3211479 => {
            // java PresentationHandler.this.handlerCallback.onTextFontBoldData(temp.toString(), EmbeddedFontHandler.this.fontListEntry);
            self.handlerCallback.onTextFontBoldData(temp.toString(), self.fontListEntry);
          }
          // java case 3211480:
          3211480 => {
            // java PresentationHandler.this.handlerCallback.onTextFontItalicData(temp.toString(), EmbeddedFontHandler.this.fontListEntry);
            self.handlerCallback.onTextFontItalicData(temp.toString(), self.fontListEntry);
          }
          // java case 3211481:
          3211481 => {
            // java PresentationHandler.this.handlerCallback.onTextFontBoldItalicData(temp.toString(), EmbeddedFontHandler.this.fontListEntry);
            self.handlerCallback.onTextFontBoldItalicData(temp.toString(), self.fontListEntry);
          }
          // java default:
          _ => {
          }
        }
      }
      */
    }

}

impl std::ops::Deref for EmbeddedFontDataIdHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for EmbeddedFontDataIdHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class CustomShowListHandler extends DefaultElementHandler
impl CustomShowListHandler {
    // java private  CustomShowListHandler()
    pub fn CustomShowListHandler() -> Self {
      // java this.customShowList = null;
      Self {
        super_: DefaultElementHandler {},
        customShowList: None,
        presCustomShowList: Default::default(),
        handlerCallback: Default::default(),
      }
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java switch (subElement)
      match subElement {
        // java case 3211443:
        3211443 => {
          // java if (this.customShowList == null)
          if self.customShowList.is_none() {
            // java this.customShowList = new ArrayList();
            self.customShowList = Some(Vec::new());
          }
          // java Presentation.CustomShow customShow = Presentation.CustomShow.create();
          let customShow = std::rc::Rc::new(std::cell::RefCell::new(CustomShow::create()));
          // java this.customShowList.add(customShow);
          self.customShowList.as_mut().unwrap().push(customShow.clone());
          // java return new CustomShowHandler(customShow);
          let mut h = CustomShowHandler::CustomShowHandler(customShow);
          h.handlerCallback = self.handlerCallback.clone();
          return Some(Box::new(h));
        }
        // java default:
        _ => {
          // java return null;
          return None;
        }
      }
    }

    // java public void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
      // java if (this.customShowList != null)
      if let Some(customShowList) = &self.customShowList {
        // java for (Presentation.CustomShow customShow : this.customShowList)
        for customShow in customShowList.iter() {
          // java customShow.save();
          customShow.borrow().save();
        }
        // java PresentationHandler.this.presData.customShowList().write(this.customShowList);
        let written: Vec<CustomShow> = customShowList.iter().map(|c| c.borrow().clone()).collect();
        self.presCustomShowList.borrow_mut().write(written);
      }
    }

}

impl std::ops::Deref for CustomShowListHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CustomShowListHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java  class CustomShowHandler extends DefaultElementHandler
impl CustomShowHandler {
    // java public  CustomShowHandler(Presentation.CustomShow customShow)
    pub fn CustomShowHandler(customShow: std::rc::Rc<std::cell::RefCell<CustomShow>>) -> Self {
      // java this.customShow = customShow;
      Self {
        super_: DefaultElementHandler {},
        customShow,
        slideListEntryList: None,
        handlerCallback: Default::default(),
      }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr tmp = attrsNotCopy.getByName(3211342);
      let mut tmp = attrsNotCopy.get_by_name(3211342);
      // java if (tmp != null)
      if let Some(tmp) = tmp {
        // java this.customShow.putId(tmp.toInt());
        self.customShow.borrow_mut().putId(tmp.to_int());
      }
      // java if ((tmp = attrsNotCopy.getByName(3211449)) != null)
      tmp = attrsNotCopy.get_by_name(3211449);
      if let Some(tmp) = tmp {
        // java this.customShow.putName(tmp.toString());
        self.customShow.borrow_mut().putName(tmp.to_string());
      }
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java if (subElement == 3211483)
      if subElement == 3211483 {
        // java if (this.slideListEntryList == null)
        if self.slideListEntryList.is_none() {
          // java this.slideListEntryList = new ArrayList();
          self.slideListEntryList = Some(std::rc::Rc::new(std::cell::RefCell::new(Vec::new())));
        }
        // java return new SlideListHandler(this.slideListEntryList);
        return Some(Box::new(SlideListHandler::SlideListHandler(
          self.slideListEntryList.as_ref().unwrap().clone(),
        )));
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java public void endElement(int element)
    pub fn endElement(&mut self, _element: i32) {
      // java if (this.slideListEntryList != null)
      if let Some(slideListEntryList) = &self.slideListEntryList {
        // java PresentationHandler.this.handlerCallback.onCustomShow(this.slideListEntryList, this.customShow);
        // rust: `customShow` 在 Java 里是引用（`onCustomShow` 会往里 add slideId），
        // 端口把同一份真实数据传 `&mut`，调用后再写回
        let mut customShow = self.customShow.borrow().clone();
        self.handlerCallback.onCustomShow(
          slideListEntryList.borrow().clone(),
          &mut customShow,
        );
        *self.customShow.borrow_mut() = customShow;
      }
    }

}

impl std::ops::Deref for CustomShowHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CustomShowHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java  class SlideListHandler extends DefaultElementHandler
impl SlideListHandler {
    // java public  SlideListHandler(List<String> slideListEntryList)
    pub fn SlideListHandler(slideListEntryList: std::rc::Rc<std::cell::RefCell<Vec<String>>>) -> Self {
      // java this.slideListEntryList = slideListEntryList;
      Self {
        super_: DefaultElementHandler {},
        slideListEntryList,
      }
    }

    // java public ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // java ElementHandler handler = null;
      let mut handler: Option<Box<dyn ElementHandler>> = None;
      // java switch (subElement)
      match subElement {
        // java case 3211440:
        3211440 => {
          // java handler = new SlideListEntryHandler();
          let mut entry = SlideListEntryHandler::SlideListEntryHandler();
          entry.slideListEntryList = self.slideListEntryList.clone();
          handler = Some(Box::new(entry));
          // java default:
          // java return handler;
          return handler;
        }
        // java default:
        _ => {
          // java return handler;
          return handler;
        }
      }
    }

}

impl std::ops::Deref for SlideListHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for SlideListHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class SlideListEntryHandler extends DefaultElementHandler
impl SlideListEntryHandler {
    // java private  SlideListEntryHandler()
    pub fn SlideListEntryHandler() -> Self {
      Self {
        super_: DefaultElementHandler {},
        slideListEntryList: Default::default(),
      }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr temp = null;
      let mut temp: Option<&kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrLeaf::XmlRoAttrLeaf> = None;
      // java if ((temp = attrsNotCopy.getByName(3211342)) != null || (temp = attrsNotCopy.getByName(4259841)) != null)
      temp = attrsNotCopy
        .get_by_name(3211342)
        .or_else(|| attrsNotCopy.get_by_name(4259841));
      if let Some(temp) = temp {
        // java SlideListHandler.this.slideListEntryList.add(temp.toString());
        self.slideListEntryList.borrow_mut().push(temp.to_string());
      }
    }

}

impl std::ops::Deref for SlideListEntryHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for SlideListEntryHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class PhotoAlbumHandler extends DefaultElementHandler
impl PhotoAlbumHandler {
    // java private  PhotoAlbumHandler()
    pub fn PhotoAlbumHandler() -> Self {
      Self {
        super_: DefaultElementHandler {},
        photoAlbum: Default::default(),
      }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java XmlRoAttr tmp = null;
      let mut tmp: Option<&kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrLeaf::XmlRoAttrLeaf> = None;
      // java if ((tmp = attrsNotCopy.getByName(3211484)) != null)
      tmp = attrsNotCopy.get_by_name(3211484);
      if let Some(tmp) = tmp {
        // java PresentationHandler.this.presData.photoAlbum().putBlackAndWhite(tmp.toBoolean());
        self.photoAlbum.borrow_mut().putBlackAndWhite(tmp.to_boolean());
      }
      // java if ((tmp = attrsNotCopy.getByName(3211487)) != null)
      tmp = attrsNotCopy.get_by_name(3211487);
      if let Some(tmp) = tmp {
        // java PresentationHandler.this.presData.photoAlbum().putFrame((Integer)SWReaderMap.photoAlbumFrameShapeMap.get(tmp.toString()));
        self.photoAlbum.borrow_mut().putFrame(*SWReaderMap::get_photoAlbumFrameShapeMap().get(tmp.to_string().as_str()).unwrap());
      }
      // java if ((tmp = attrsNotCopy.getByName(3211486)) != null)
      tmp = attrsNotCopy.get_by_name(3211486);
      if let Some(tmp) = tmp {
        // java PresentationHandler.this.presData.photoAlbum().putLayout((Integer)SWReaderMap.photoAlbumLayoutMap.get(tmp.toString()));
        self.photoAlbum.borrow_mut().putLayout(*SWReaderMap::get_photoAlbumLayoutMap().get(tmp.to_string().as_str()).unwrap());
      }
      // java if ((tmp = attrsNotCopy.getByName(3211485)) != null)
      tmp = attrsNotCopy.get_by_name(3211485);
      if let Some(tmp) = tmp {
        // java PresentationHandler.this.presData.photoAlbum().putShowCaptions(tmp.toBoolean());
        self.photoAlbum.borrow_mut().putShowCaptions(tmp.to_boolean());
      }
    }

}

impl std::ops::Deref for PhotoAlbumHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PhotoAlbumHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java  class CustDataLstHandler extends DefaultElementHandler
impl CustDataLstHandler {
    pub fn CustDataLstHandler() -> Self {
      Self { super_: DefaultElementHandler {} }
    }
}

impl std::ops::Deref for CustDataLstHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CustDataLstHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java  class KinsokuHandler extends DefaultElementHandler
impl KinsokuHandler {
    pub fn KinsokuHandler() -> Self {
      Self {
        super_: DefaultElementHandler {},
        kinsoku: Default::default(),
      }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &XmlRoAttrList) {
      // java Presentation.Kinsoku kinsoku = PresentationHandler.this.presData.kinsoku();
      let mut kinsoku = self.kinsoku.borrow_mut();
      // java XmlRoAttr tmp = attrsNotCopy.getByName(3211492);
      let mut tmp = attrsNotCopy.get_by_name(3211492);
      // java if (tmp != null)
      if let Some(tmp) = tmp {
        // java kinsoku.putInvalEndChars(tmp.toString());
        kinsoku.putInvalEndChars(tmp.to_string());
      }
      // java if ((tmp = attrsNotCopy.getByName(3211491)) != null)
      tmp = attrsNotCopy.get_by_name(3211491);
      if let Some(tmp) = tmp {
        // java kinsoku.putInvalStChars(tmp.toString());
        kinsoku.putInvalStChars(tmp.to_string());
      }
      // java if ((tmp = attrsNotCopy.getByName(3211490)) != null)
      tmp = attrsNotCopy.get_by_name(3211490);
      if let Some(tmp) = tmp {
        // java kinsoku.putLang(tmp.toString());
        kinsoku.putLang(tmp.to_string());
      }
    }

}

impl std::ops::Deref for KinsokuHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KinsokuHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl ElementHandler for CustomShowListHandler {
    fn kind(&self) -> &'static str { "CustomShowListHandler" }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        CustomShowListHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, element: i32) {
        CustomShowListHandler::endElement(self, element);
    }
}

impl ElementHandler for CustomShowHandler {
    fn kind(&self) -> &'static str { "CustomShowHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        CustomShowHandler::startElement(self, element, attrs);
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        CustomShowHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, element: i32) {
        CustomShowHandler::endElement(self, element);
    }
}

impl ElementHandler for SlideListHandler {
    fn kind(&self) -> &'static str { "SlideListHandler" }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        SlideListHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, _element: i32) {}
}

impl ElementHandler for SlideListEntryHandler {
    fn kind(&self) -> &'static str { "SlideListEntryHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        SlideListEntryHandler::startElement(self, element, attrs);
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _element: i32) {}
}

impl ElementHandler for PhotoAlbumHandler {
    fn kind(&self) -> &'static str { "PhotoAlbumHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        PhotoAlbumHandler::startElement(self, element, attrs);
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _element: i32) {}
}

impl ElementHandler for CustDataLstHandler {
    fn kind(&self) -> &'static str { "CustDataLstHandler" }
    fn startElement(&mut self, _element: i32, _attrs: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _element: i32) {}
}

impl ElementHandler for KinsokuHandler {
    fn kind(&self) -> &'static str { "KinsokuHandler" }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        KinsokuHandler::startElement(self, element, attrs);
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _element: i32) {}
}

// java private class ModifyVerifierHandler extends DefaultElementHandler
impl ModifyVerifierHandler {
    // java private  ModifyVerifierHandler()
    pub fn ModifyVerifierHandler() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      Self { super_: DefaultElementHandler::new() }
      */
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != PresentationHandler.this.mPres && null != PresentationHandler.this.passProvider && attrsNotCopy.getByName(3211500) != null && attrsNotCopy.getByName(3211503) != null && attrsNotCopy.getByName(3211502) != null && attrsNotCopy.getByName(3211501) != null)
      if self.mPres.is_some() && self.passProvider.is_some() && attrsNotCopy.getByName__int(3211500).is_some() && attrsNotCopy.getByName__int(3211503).is_some() && attrsNotCopy.getByName__int(3211502).is_some() && attrsNotCopy.getByName__int(3211501).is_some() {
        // java int cryptAlgorithmSid = attrsNotCopy.getByName(3211500).toInt();
        let mut cryptAlgorithmSid: i32 = attrsNotCopy.getByName__int(3211500).toInt();
        // java String hashValue = attrsNotCopy.getByName(3211503).toString();
        let mut hashValue: String = attrsNotCopy.getByName__int(3211503).toString();
        // java String saltValue = attrsNotCopy.getByName(3211502).toString();
        let mut saltValue: String = attrsNotCopy.getByName__int(3211502).toString();
        // java int spinCount = attrsNotCopy.getByName(3211501).toInt();
        let mut spinCount: i32 = attrsNotCopy.getByName__int(3211501).toInt();
        // java ModifyVerifier modifyVerifier = new ModifyVerifier(hashValue, saltValue, spinCount, cryptAlgorithmSid);
        let mut modifyVerifier: ModifyVerifier = ModifyVerifier::new(hashValue, saltValue, spinCount, cryptAlgorithmSid);
        // java boolean bVerify = false;
        let mut bVerify: bool = false;
        let mut writePassword: String = Default::default();
        // java for (String writePassword = null; !bVerify PresentationHandler.this.passProvider.verifyWritePassword(bVerify))
        while !bVerify {
          // java writePassword = PresentationHandler.this.passProvider.getWritePassword(false);
          writePassword = self.passProvider.getWritePassword(false);
          // java if (writePassword == null)
          if false {
            // java PresentationHandler.this.mPres.cryptInfo().setModifyVerifier(modifyVerifier);
            self.mPres.cryptInfo().setModifyVerifier(modifyVerifier);
            // java PresentationHandler.this.mPres.cryptInfo().setReadOnly(true);
            self.mPres.cryptInfo().setReadOnly(true);
            // java break;
            break;
          }
          // java if (CryptTool.checkModifyPassword(modifyVerifier, writePassword))
          if CryptTool::checkModifyPassword(modifyVerifier, writePassword) {
            // java bVerify = true;
            bVerify = true;
            // java PresentationHandler.this.mPres.cryptInfo().setWritePassword(writePassword, false);
            self.mPres.cryptInfo().setWritePassword(writePassword, false);
          }
        }
      }
      */
    }

}

impl std::ops::Deref for ModifyVerifierHandler {
    type Target = DefaultElementHandler;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for ModifyVerifierHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl ElementHandler for PresentationHandler {
    fn kind(&self) -> &'static str {
        "PresentationHandler"
    }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        PresentationHandler::startElement(self, element, attrs);
    }
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        PresentationHandler::enterSubElement(self, subElement)
    }
    fn endElement(&mut self, element: i32) {
        PresentationHandler::endElement(self, element);
    }
}

/// Java `Presentation.open(pres.protocol())` 的端口实现。
///
/// `rust:` Rust 侧没有 `protocol()`（`cn.wps.show.data.Presentation` 在 default feature 下不可达），
/// 端口把「既有状态」从 `KmoPresentation` 的装配面读回同一批字段（slideSize / notesSize），
/// 语义与 Java 一致：`PresentationHandler` 解析出的值会覆盖它们，未解析到的保持原值。
fn pres_data_from_model(pres: &KmoPresentation) -> Presentation {
    let mut presData = Presentation::open(());
    if pres.hasSlideSize() {
        let mut size = presData.slideSize.borrow_mut();
        size.putCx(pres.slideWidth());
        size.putCy(pres.slideHeight());
    }
    if pres.hasSlideSizeType() {
        presData.slideSize.borrow_mut().putType(pres.slideSizeType());
    }
    if pres.hasNotesSize() {
        let mut notes = presData.notesSize.borrow_mut();
        notes.putCx(pres.notesWidth());
        notes.putCy(pres.notesHeight());
    }
    presData
}
