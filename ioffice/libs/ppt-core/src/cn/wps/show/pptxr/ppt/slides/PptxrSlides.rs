use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod slides_cs {
    use std::collections::HashMap;

    #[derive(Clone, Default)]
    pub struct OpcrPackagePart {
        pub relationships: Option<OpcrRelationships>,
        /// 真实 part 的 XML 字节（来自 kfc `getInputStream`）。
        pub bytes: Vec<u8>,
    }
    impl OpcrPackagePart {
        pub fn getRelationships(&self) -> Option<OpcrRelationships> {
            self.relationships.clone()
        }
    }

    #[derive(Clone, Default)]
    pub struct OpcrRelationship {
        pub target: String,
        pub part_name: String,
        pub rels_type: String,
        pub part: Option<OpcrPackagePart>,
    }
    impl OpcrRelationship {
        pub fn getTarget(&self) -> String {
            self.target.clone()
        }
        pub fn getPartName(&self) -> String {
            self.part_name.clone()
        }
        pub fn getType(&self) -> String {
            self.rels_type.clone()
        }
        pub fn getPackagePart(&self) -> Option<OpcrPackagePart> {
            self.part.clone()
        }
    }

    #[derive(Clone, Default)]
    pub struct OpcrRelationships {
        pub by_rid: HashMap<String, OpcrRelationship>,
    }
    impl OpcrRelationships {
        pub fn getRelationship(&self, rid: &str) -> Option<OpcrRelationship> {
            self.by_rid.get(rid).cloned()
        }
        pub fn put(&mut self, rid: String, rel: OpcrRelationship) {
            self.by_rid.insert(rid, rel);
        }
        pub fn all(&self) -> Vec<OpcrRelationship> {
            self.by_rid.values().cloned().collect()
        }
    }
    #[derive(Default)]
    pub struct OxfPptRelations {}
    impl OxfPptRelations {
        pub fn slideMaster() -> Self {
            Self {}
        }
        pub fn getRelsType(&self) -> String {
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster"
                .into()
        }
    }
    #[derive(Default)]
    pub struct ForceQuitException {}
    #[derive(Clone, Default)]
    pub struct IDLProvider {
        pub force_quit: bool,
    }
    impl IDLProvider {
        pub fn isOpenForceQuit(&self) -> bool {
            self.force_quit
        }
    }
    #[derive(Clone, Default)]
    pub struct KmoPresOpenEventListener {
        pub finishes: std::rc::Rc<std::cell::RefCell<Vec<i32>>>,
    }
    impl KmoPresOpenEventListener {
        pub fn slideDidFinishReadingEverything(&self, slideIndex: i32) {
            self.finishes.borrow_mut().push(slideIndex);
            super::push_open_event(format!("finish:{slideIndex}"));
        }
    }
    #[derive(Default)]
    pub struct KmoHandoutMaster {}
    #[derive(Default)]
    pub struct KmoNoteMaster {}
    #[derive(Default)]
    pub struct KmoSlideLayout {}
    #[derive(Default)]
    pub struct KmoSlideMaster {}
    #[derive(Default)]
    pub struct KmoTextOperator {}
    #[derive(Default)]
    pub struct HeaderFooter {}
    #[derive(Default)]
    pub struct CommentIdContext {}
    #[derive(Default)]
    pub struct PptxrHandoutMaster {}
    #[derive(Clone, Default)]
    pub struct PptxrMedias {}
    impl PptxrMedias {
        pub fn priorityReadMedia(&self, _pres: &super::KmoPresentation) {
            super::push_open_event("priorityReadMedia".into());
        }
    }
    #[derive(Default)]
    pub struct PptxrNotesMaster {}
    impl PptxrNotesMaster {
        pub fn buildDefaultNotesMaster(
            _pres: &std::rc::Rc<std::cell::RefCell<super::KmoPresentation>>,
            _medias: &Option<PptxrMedias>,
            _mode: i32,
        ) -> () {
            ()
        }
        pub fn open(&self) {}
    }
    #[derive(Default)]
    pub struct PptxrNotesSlide {}
    #[derive(Default)]
    pub struct PptxrLayout {}
    impl PptxrLayout {
        pub fn buildDefaultLayout(
            _pres: &std::rc::Rc<std::cell::RefCell<super::KmoPresentation>>,
            _medias: &Option<PptxrMedias>,
            _mode: i32,
        ) -> super::KmoSlideLayout {
            super::push_open_event("buildDefaultLayout".into());
            super::KmoSlideLayout::default()
        }
    }
    #[derive(Clone, Default)]
    pub struct PptxrMaster {
        pub part_name: String,
    }
    impl PptxrMaster {
        pub fn open(&mut self) {
            super::push_open_event("master.open".into());
        }
        pub fn openAllDatas(&mut self) {
            super::push_open_event("master.openAllDatas".into());
        }
        pub fn openLayout(
            &mut self,
            _layoutRels: Option<OpcrRelationship>,
        ) -> super::KmoSlideLayout {
            super::push_open_event("openLayout".into());
            super::KmoSlideLayout::default()
        }
        pub fn buildDefaultSlideMaster(
            _pres: &std::rc::Rc<std::cell::RefCell<super::KmoPresentation>>,
            _medias: &Option<PptxrMedias>,
            _mode: i32,
        ) -> super::KmoSlideMaster {
            super::push_open_event("buildDefaultSlideMaster".into());
            super::KmoSlideMaster {
                id: -1,
                layouts: vec![super::KmoSlideLayout::default()],
                ..Default::default()
            }
        }
    }
    #[derive(Clone, Default)]
    pub struct PptxrSlide {
        pub layout_rels: Option<OpcrRelationship>,
        pub slide_index: i32,
        /// 该页 `slideN.xml` 的原文。`open()` 把它写进模型里的 `KmoSlide`。
        pub xml: Option<String>,
    }
    impl PptxrSlide {
        pub fn getLayoutRels(&self) -> Option<OpcrRelationship> {
            self.layout_rels.clone()
        }
        pub fn open(&mut self) {
            super::push_open_event("slide.open".into());
        }
    }
    #[derive(Default)]
    pub struct PptxrThemes {}
    #[derive(Default)]
    pub struct IOException {}
}

// Java: import cn.wps.kfc.ooxfr.OxfPptRelations
use slides_cs::OxfPptRelations;
// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use slides_cs::OpcrPackagePart as _;
pub use slides_cs::OpcrPackagePart;
// Java: import cn.wps.kfc.opcr.OpcrRelationship
use slides_cs::OpcrRelationship as _;
pub use slides_cs::OpcrRelationship;
// Java: import cn.wps.kfc.opcr.OpcrRelationships
use slides_cs::OpcrRelationships as _;
pub use slides_cs::OpcrRelationships;
// Java: import cn.wps.moffice.io.ForceQuitException
use slides_cs::ForceQuitException;
// Java: import cn.wps.moffice.io.IDLProvider
use slides_cs::IDLProvider as _;
pub use slides_cs::IDLProvider;
// Java: import cn.wps.show.app.KmoPresentation
use crate::cn::wps::show::util::layout_shape::KmoPresentation;
// Java: import cn.wps.show.app.event.KmoPresOpenEventListener
use slides_cs::KmoPresOpenEventListener as _;
pub use slides_cs::KmoPresOpenEventListener;
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::util::layout_shape::KmoShowShape;
// Java: import cn.wps.show.app.slide.KmoHandoutMaster
use slides_cs::KmoHandoutMaster;
// Java: import cn.wps.show.app.slide.KmoNoteMaster
use slides_cs::KmoNoteMaster;
// Java: import cn.wps.show.app.slide.KmoSlide
use crate::cn::wps::show::util::layout_shape::KmoSlide;
// Java: import cn.wps.show.app.slide.KmoSlideLayout
use crate::cn::wps::show::util::layout_shape::KmoSlideLayout;
// Java: import cn.wps.show.app.slide.KmoSlideMaster
use crate::cn::wps::show::util::layout_shape::KmoSlideMaster;
// Java: import cn.wps.show.app.text.KmoTextOperator
use crate::cn::wps::show::util::layout_textbox::KmoTextOperator;
// Java: import cn.wps.show.data.HeaderFooter
use crate::cn::wps::show::util::layout_shape::HeaderFooter;
// Java: import cn.wps.show.pptxr.ppt.CommentIdContext
use slides_cs::CommentIdContext;
// Java: import cn.wps.show.pptxr.ppt.handoutmaster.PptxrHandoutMaster
use slides_cs::PptxrHandoutMaster;
// Java: import cn.wps.show.pptxr.ppt.media.PptxrMedias
use slides_cs::PptxrMedias as _;
pub use slides_cs::PptxrMedias;
// Java: import cn.wps.show.pptxr.ppt.notesmasters.PptxrNotesMaster
use slides_cs::PptxrNotesMaster;
// Java: import cn.wps.show.pptxr.ppt.notesslides.PptxrNotesSlide
use slides_cs::PptxrNotesSlide;
// Java: import cn.wps.show.pptxr.ppt.slidelayouts.PptxrLayout
use slides_cs::PptxrLayout;
// Java: import cn.wps.show.pptxr.ppt.slidemasters.PptxrMaster
use slides_cs::PptxrMaster;
// Java: import cn.wps.show.pptxr.ppt.theme.PptxrThemes
use slides_cs::PptxrThemes;
// Java: import java.io.IOException
use slides_cs::IOException;
// Java: import java.util.ArrayList
use std::vec::Vec as ArrayList;
// Java: import java.util.HashMap
use std::collections::HashMap as JavaHashMap;
// Java: import java.util.Iterator
use std::vec::IntoIter as Iterator;
// Java: import java.util.List
use std::vec::Vec as List;
// Java: import java.util.Map
use std::collections::HashMap as Map;
// Java: import cn.wps.show.pptxr.ppt.slides.PptxrSlide
use slides_cs::PptxrSlide as _;
pub use slides_cs::PptxrSlide;

// java public static boolean OPLogDumper_FirstSlideIOFinished = false;
pub(crate) static OPLogDumper_FirstSlideIOFinished: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
// java private static Map<String, Long> mSlideIdMap = null;
thread_local! {
    static mSlideIdMap: RefCell<Option<HashMap<String, i64>>> = RefCell::new(None);
    static OPEN_EVENTS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

fn push_open_event(s: String) {
    OPEN_EVENTS.with(|e| e.borrow_mut().push(s));
}

pub fn take_open_events() -> Vec<String> {
    OPEN_EVENTS.with(|e| e.replace(Vec::new()))
}

pub fn set_first_slide_io_finished(v: bool) {
    OPLogDumper_FirstSlideIOFinished.store(v, std::sync::atomic::Ordering::SeqCst);
}

// java public final class PptxrSlides
#[derive(Default)]
pub struct PptxrSlides {
    // java private KmoPresentation mPres;
    mPres: Rc<RefCell<KmoPresentation>>,
    // java private OpcrRelationships mPresRelses;
    mPresRelses: Option<OpcrRelationships>,
    // java private KmoPresOpenEventListener mOpenListener;
    mOpenListener: Option<KmoPresOpenEventListener>,
    // java private IDLProvider mDlProvider;
    mDlProvider: Option<IDLProvider>,
    // java private List<PptxrSlide> mSlidesReader = new ArrayList();
    mSlidesReader: Vec<PptxrSlide>,
    // java private Map<String, PptxrMaster> mMastersReader = new HashMap();
    mMastersReader: Option<HashMap<String, PptxrMaster>>,
    // java private Map<String, PptxrNotesMaster> mNotesMastersReader = new HashMap();
    mNotesMastersReader: Option<HashMap<String, PptxrNotesMaster>>,
    // java private Map<String, PptxrHandoutMaster> mHandoutMastersReader = new HashMap();
    mHandoutMastersReader: Option<HashMap<String, PptxrHandoutMaster>>,
    // java private CommentIdContext mCommentIdCxt;
    mCommentIdCxt: Option<CommentIdContext>,
    // java private List<String> slideRids = new ArrayList();
    slideRids: Vec<String>,
    // java private int createSlideCount = 0;
    createSlideCount: i32,
    // java private PptxrMedias mMedias;
    mMedias: Option<PptxrMedias>,
    // java private PptxrThemes mThemes;
    mThemes: Option<PptxrThemes>,
    // java private int mMode;
    mMode: i32,
}

// java public static class UnReadSlides
#[derive(Default)]
pub struct UnReadSlides {
    // java private boolean[] mUnReadSlides;
    mUnReadSlides: Vec<bool>,
    // java private int mTop = 0;
    mTop: i32,
}

// java public final class PptxrSlides
impl PptxrSlides {
    // java public static void initialize()
    pub fn initialize() {
      // java if (mSlideIdMap == null)
      mSlideIdMap.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.is_none() {
          // java mSlideIdMap = new HashMap();
          *slot = Some(HashMap::new());
          let _ = stringify!(mSlideIdMap);
        }
        // java else
        else {
          // java mSlideIdMap.clear();
          slot.as_mut().unwrap().clear();
          let _ = stringify!(mSlideIdMap);
        }
      });
    }

    // java public static long getSlideIdByFileName(String fileName)
    pub fn getSlideIdByFileName(fileName: Option<String>) -> i64 {
      // java Long slideId = (Long)mSlideIdMap.get(fileName);
      let slideId: Option<i64> = mSlideIdMap.with(|cell| {
        let slot = cell.borrow();
        let map = slot.as_ref().expect("mSlideIdMap");
        match &fileName {
          Some(k) => map.get(k).copied(),
          None => None,
        }
      });
      // java return slideId != null ? slideId : 0L;
      return (if let Some(slideId) = slideId { slideId } else { 0 });
    }

    pub fn put_slide_id(fileName: String, slideId: i64) {
      mSlideIdMap.with(|cell| {
        let mut g = cell.borrow_mut();
        if g.is_none() {
          *g = Some(HashMap::new());
        }
        g.as_mut().unwrap().insert(fileName, slideId);
      });
    }

    // java public long getSlideIdByFileNameNotAdd(String fileName)
    pub fn getSlideIdByFileNameNotAdd(&mut self, fileName: Option<String>) -> i64 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java long slideId = getSlideIdByFileName(fileName);
      let mut slideId: i64 = Self::getSlideIdByFileName(fileName);
      // java if (0L == slideId)
      if 0 == slideId {
        // java this.createSlidesByFileName(fileName);
        self.createSlidesByFileName(fileName.unwrap());
        // java slideId = getSlideIdByFileName(fileName);
        slideId = Self::getSlideIdByFileName(fileName);
      }
      // java return slideId;
      return slideId;
      */
    }

    // java public static String getRelationShipTargetFileName(OpcrRelationship relationship)
    pub fn getRelationShipTargetFileName(relationship: Option<&OpcrRelationship>) -> String {
      // java if (relationship == null)
      if relationship.is_none() {
        // java return "";
        return "".to_string();
      }
      // java else
      else {
        // java String target = relationship.getTarget();
        let mut target: String = relationship.unwrap().getTarget();
        // java int index = target.lastIndexOf("/");
        let mut index: i32 = match target.rfind('/') {
          Some(i) => i as i32,
          None => -1,
        };
        // java if (index >= 0 && index < target.length())
        if index >= 0 && index < (target.len() as i32) {
          // java ++index;
          index += 1;
        }
        // java else
        else {
          // java index = 0;
          index = 0;
        }
        // java return target.substring(index);
        return target[index as usize..].to_string();
      }
    }

    // java public  PptxrSlides(KmoPresentation pres, OpcrRelationships presRelses, KmoPresOpenEventListener listener, IDLProvider dlProvider, PptxrMedias medias, PptxrThemes themes, int mode, CommentIdContext commentIdCxt)
    pub fn PptxrSlides(pres: Rc<RefCell<KmoPresentation>>, presRelses: Option<OpcrRelationships>, listener: Option<KmoPresOpenEventListener>, dlProvider: Option<IDLProvider>, medias: Option<PptxrMedias>, themes: Option<PptxrThemes>, mode: i32, commentIdCxt: Option<CommentIdContext>) -> Self {
      // java this.mPres = pres;
      Self {
        mPres: pres,
        mPresRelses: presRelses,
        mOpenListener: listener,
        mDlProvider: dlProvider,
        mSlidesReader: Default::default(),
        mMastersReader: Some(HashMap::new()),
        mNotesMastersReader: Some(HashMap::new()),
        mHandoutMastersReader: Default::default(),
        mCommentIdCxt: commentIdCxt,
        slideRids: Default::default(),
        createSlideCount: Default::default(),
        mMedias: medias,
        mThemes: themes,
        mMode: mode,
      }
      // java this.mPresRelses = presRelses;
      // java this.mOpenListener = listener;
      // java this.mDlProvider = dlProvider;
      // java this.mMedias = medias;
      // java this.mThemes = themes;
      // java this.mMode = mode;
      // java this.mCommentIdCxt = commentIdCxt;
    }

    fn reader_for(part: &OpcrPackagePart) -> PptxrSlide {
        let xml = String::from_utf8(part.bytes.clone()).ok().filter(|s| !s.is_empty());
        PptxrSlide {
            xml,
            ..PptxrSlide::default()
        }
    }

    pub fn slides_reader_len(&self) -> usize {
        self.mSlidesReader.len()
    }
    pub fn slide_rids(&self) -> &[String] {
        &self.slideRids
    }
    pub fn create_slide_count(&self) -> i32 {
        self.createSlideCount
    }
    pub fn masters_reader_len(&self) -> usize {
        self.mMastersReader.as_ref().map(|m| m.len()).unwrap_or(0)
    }
    pub fn masters_part_names(&self) -> Vec<String> {
        self.mMastersReader
            .as_ref()
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default()
    }
    pub fn slides_reader_mut(&mut self) -> &mut Vec<PptxrSlide> {
        &mut self.mSlidesReader
    }
    pub fn pres(&self) -> Rc<RefCell<KmoPresentation>> {
        self.mPres.clone()
    }

    // java public void open()
    pub fn open(&mut self) -> Result<(), std::io::Error> {
      // java int slideIndex = -1;
      let mut slideIndex: i32 = -1;
      // java UnReadSlides slides = new UnReadSlides(this.mSlidesReader.size());
      let mut slides: UnReadSlides = UnReadSlides::UnReadSlides(self.mSlidesReader.len() as i32);
      // java boolean bFirstSlide = true;
      let mut bFirstSlide: bool = true;
      // rust: ★★M-PPT-KMOSLIDE §1.3 目标 3 —— **对象图真实**。
      //
      //   Java 侧每个 handler 都经 `shape.parent().getPresentation().drawingStore()` 取仓，
      //   而 `getPresentation()` 就是本 reader 的 `this.mPres`
      //   ⇒ 它们拿到的是 `mPres.canvas.store()` —— **演示文稿级唯一仓**。
      //
      //   修复前：本处调用 `ImportHost::leak()`，它**自己新 leak 一个空 DgStore**，
      //   于是 handler 写入的仓 ≠ `mPres.drawingStore()` 的仓 ⇒
      //   `commit_drawing` 的 `attachSpPr` 结果在 `mPres` 侧**完全不可见**
      //   （实测：真树 695 个节点，`mPres.drawingStore().shapePr(id)` 命中 **0** 个）。
      //
      //   修复：把 `mPres` 的**真实仓**交给宿主。
      //   `drawingStorePtr()` 在 `store_handle` 未装配时会先建好（保证非空且稳定），
      //   故宿主与其后所有 `mPres.drawingStore()` 调用解析到**同一个仓**。
      //
      //   owner: ppt.show.pptxr.ppt.slides.PptxrSlides    wave: M-PPT-KMOSLIDE
      let import_host = crate::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::ImportHost::leak_with_store(
        self.mPres.borrow_mut().drawingStorePtr(),
      );
      // java while ((slideIndex = slides.next(this.mPres)) != -1)
      loop {
        slideIndex = slides.next(&self.mPres.borrow());
        if slideIndex == -1 {
          break;
        }
        // java PptxrSlide slideReader = (PptxrSlide)this.mSlidesReader.get(slideIndex);
        let slide_i = slideIndex as usize;
        // java PptxrMaster masterReader = this.getMasterReader(slideReader);
        let masterReader: Option<PptxrMaster> = self.getMasterReader(&self.mSlidesReader[slide_i]);
        // java if (masterReader != null)
        if let Some(mut masterReader) = masterReader {
          // java masterReader.open();
          masterReader.open();
          // java KmoSlideLayout layout = masterReader.openLayout(slideReader.getLayoutRels());
          let layout: KmoSlideLayout = masterReader.openLayout(self.mSlidesReader[slide_i].getLayoutRels());
          // java slideReader.getSlide().setLayout(layout);
          let si = self.mSlidesReader[slide_i].slide_index as usize;
          self.mPres.borrow_mut().slides[si].setLayout(layout);
          push_open_event("setLayout".into());
        }
        // java else
        else {
          // java KmoSlideMaster defaultSlideMatser = PptxrMaster.buildDefaultSlideMaster(this.mPres, this.mMedias, this.mMode);
          let defaultSlideMatser: KmoSlideMaster = PptxrMaster::buildDefaultSlideMaster(&self.mPres, &self.mMedias, self.mMode);
          // java KmoSlideLayout layout = PptxrLayout.buildDefaultLayout(this.mPres, this.mMedias, this.mMode);
          let mut layout: KmoSlideLayout = PptxrLayout::buildDefaultLayout(&self.mPres, &self.mMedias, self.mMode);
          // java layout.setMaster(defaultSlideMatser);
          layout.setMaster(defaultSlideMatser.clone());
          // java slideReader.getSlide().setLayout(layout);
          let si = self.mSlidesReader[slide_i].slide_index as usize;
          self.mPres.borrow_mut().slides[si].setLayout(layout);
          push_open_event("setLayout".into());
          // java if (this.mPres.indexOfMaster(defaultSlideMatser) == -1)
          if self.mPres.borrow().indexOfMaster(&defaultSlideMatser) == -1 {
            // java this.mPres._addMaster(defaultSlideMatser);
            self.mPres.borrow_mut()._addMaster(defaultSlideMatser);
          }
        }
        // java if (null != this.mDlProvider && this.mDlProvider.isOpenForceQuit())
        if let Some(mDlProvider) = self.mDlProvider.as_ref() {
          if mDlProvider.isOpenForceQuit() {
            // java throw new ForceQuitException();
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "ForceQuitException"));
          }
        }
        // java slideReader.open();
        self.mSlidesReader[slide_i].open();
        if let Some(xml) = self.mSlidesReader[slide_i].xml.clone() {
            let si = self.mSlidesReader[slide_i].slide_index as usize;
            if let Some(slide) = self.mPres.borrow_mut().slides.get_mut(si) {
                let mut handler = crate::cn::wps::show::pptxr::handler::slides::SlideHandler::SlideHandler::SlideHandler(
                    import_host,
                );
                let mut cursor = std::io::Cursor::new(xml.into_bytes());
                let _ = crate::cn::wps::show::pptxr::util::UnifiedXmlReader::UnifiedXmlReader::read__stream_elementhandler(
                    &mut cursor,
                    &mut handler,
                );
                // rust: ★★M-PPT-KMOSLIDE §1.3 A3 —— 注入**本页所属的真实 presentation**。
                //   Java: `KmoSlide.getPresentation()` 返回构造期注入的 `this.pres`
                //   （即本 reader 的 `this.mPres`），而 `KmoSlideLayout`/`KmoSlideMaster`
                //   经母版链反查同一个对象。修复前 `KmoSlide::getPresentation()` **现造**
                //   一个 `themes: Vec::new()` 的假 presentation，消费方拿到"没有主题的演示文稿"。
                //   owner: ppt.show.pptxr.ppt.slides.PptxrSlides    wave: M-PPT-KMOSLIDE
                // rust: `slide` 已是 `self.mPres` 的 `&mut` 借出（上面的 `borrow_mut()`），
                //   故此处直接写入，**不再二次借用**（否则 `RefCell already borrowed`）。
                slide.pres_handle = Some(self.mPres.clone());
                slide.kmo_sp_root = Some(handler.take_root());
                slide.cnvpr_count = handler.cnvpr_count();
                if let Some(show_master) = handler.show_master_sp() {
                    slide.show_master_shape = show_master;
                }
            }
        }
        // java slideReader.getSlide().setIOReading(false);
        let si = self.mSlidesReader[slide_i].slide_index as usize;
        self.mPres.borrow_mut().slides[si].setIOReading(false);
        // java this.mOpenListener.slideDidFinishReadingEverything(slideIndex);
        if let Some(listener) = self.mOpenListener.as_ref() {
          listener.slideDidFinishReadingEverything(slideIndex);
        }
        // java if (bFirstSlide)
        if bFirstSlide {
          // java this.delayCreateSlides();
          self.delayCreateSlides();
          // java slides.resetSlides(this.mSlidesReader.size());
          slides.resetSlides(self.mSlidesReader.len() as i32);
          // java this.mPres.eventListeners().onSlideListChanged();
          self.mPres.borrow_mut().eventListeners().onSlideListChanged();
          // java bFirstSlide = false;
          bFirstSlide = false;
        }
        // java if (OPLogDumper_FirstSlideIOFinished)
        if OPLogDumper_FirstSlideIOFinished.load(std::sync::atomic::Ordering::SeqCst) {
          // java return;
          return Ok(());
        }
        // java this.mMedias.priorityReadMedia(this.mPres);
        if let Some(medias) = self.mMedias.as_ref() {
          medias.priorityReadMedia(&self.mPres.borrow());
        }
      }
      // java for (PptxrMaster masterReader : this.mMastersReader.values())
      if let Some(map) = self.mMastersReader.as_mut() {
        for masterReader in map.values_mut() {
          // java masterReader.open();
          masterReader.open();
          // java masterReader.openAllDatas();
          masterReader.openAllDatas();
        }
      }
      // java if (this.mSlidesReader.size() == 0)
      if self.mSlidesReader.is_empty() {
        // java KmoSlideLayout layout = null;
        let mut layout: KmoSlideLayout = Default::default();
        // java if (0 == this.mPres.masterCount())
        if 0 == self.mPres.borrow().masterCount() {
          // java KmoSlideMaster defaultSlideMatser = PptxrMaster.buildDefaultSlideMaster(this.mPres, this.mMedias, this.mMode);
          let defaultSlideMatser: KmoSlideMaster = PptxrMaster::buildDefaultSlideMaster(&self.mPres, &self.mMedias, self.mMode);
          // java layout = defaultSlideMatser.layoutAt(0);
          layout = defaultSlideMatser.layoutAt(0);
          // java this.mPres.addMaster(defaultSlideMatser);
          self.mPres.borrow_mut().addMaster(defaultSlideMatser);
        }
        // java else
        else {
          // java layout = this.mPres.masterAt(0).layoutAt(0);
          layout = self.mPres.borrow().masterAt(0).layoutAt(0);
        }
        // java KmoSlide slide = this.mPres.slideAt(0);
        let mut slide: KmoSlide = self.mPres.borrow().slideAt(0);
        // java changeLayout(slide, layout);
        Self::changeLayout(&mut slide, &layout);
        // java this.mOpenListener.slideDidFinishReadingEverything(0);
        if let Some(listener) = self.mOpenListener.as_ref() {
          listener.slideDidFinishReadingEverything(0);
        }
      }
      // rust: no-stmt — notes/handout masters deferred (DEFERRED_NON_FUNCTION bodies remain below in original template comments)
      let _ = ForceQuitException {};
      Ok(())
    }

    // java private void delayCreateSlides()
    pub fn delayCreateSlides(&mut self) {
      let mut idx: i32 = self.createSlideCount;
      // java for (int idx = this.createSlideCount; idx < this.slideRids.size() ++idx)
      while idx < (self.slideRids.len() as i32) {
        // java OpcrRelationship rels = this.mPresRelses.getRelationship((String)this.slideRids.get(idx));
        let rels: Option<OpcrRelationship> = self
          .mPresRelses
          .as_ref()
          .and_then(|rr| rr.getRelationship(&self.slideRids[idx as usize]));
        // java if (rels != null)
        if let Some(rels) = rels {
          // java OpcrPackagePart part = rels.getPackagePart();
          let part: Option<OpcrPackagePart> = rels.getPackagePart();
          // java if (part != null)
          if part.is_some() {
            // java ++this.createSlideCount;
            self.createSlideCount += 1;
            // java KmoSlide slide = new KmoSlide(this.mPres);
            let mut slide: KmoSlide = self.mPres.borrow_mut().alloc_slide();
            // java slide.setIOReading(true);
            slide.setIOReading(true);
            // java PptxrSlide slideReader = new PptxrSlide(slide, this.mPres, part, this.mMedias, this.mThemes, this, this.mMode, this.mCommentIdCxt);
            let mut slideReader: PptxrSlide = Self::reader_for(part.as_ref().unwrap());
            // java this.mSlidesReader.add(slideReader);
            self.mSlidesReader.push(slideReader);
            // java this.mPres.addSlide(slide);
            let slide_id = slide.id();
            self.mPres.borrow_mut().addSlide(slide);
            if let Some(last) = self.mSlidesReader.last_mut() {
              last.slide_index = self.mPres.borrow().slideCount() - 1;
            }
            // java String fileName = getRelationShipTargetFileName(rels);
            let fileName: String = Self::getRelationShipTargetFileName(Some(&rels));
            // java mSlideIdMap.put(fileName, slide.id());
            Self::put_slide_id(fileName, slide_id);
            let _ = stringify!(mSlideIdMap);
          }
        }
        idx += 1;
      }
    }

    // java private void createSlidesByFileName(String neededFileName)
    pub fn createSlidesByFileName(&mut self, neededFileName: String) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      let mut idx: i32 = self.createSlideCount;
      // java for (int idx = this.createSlideCount; idx < this.slideRids.size() ++idx)
      while idx < (self.slideRids.len() as i32) {
        // java OpcrRelationship rels = this.mPresRelses.getRelationship((String)this.slideRids.get(idx));
        let mut rels: OpcrRelationship = self.mPresRelses.getRelationship__string(self.slideRids.get(idx).to_string());
        // java if (rels != null)
        if true {
          // java OpcrPackagePart part = rels.getPackagePart();
          let mut part: OpcrPackagePart = rels.getPackagePart();
          // java if (part != null)
          if true {
            // java ++this.createSlideCount;
            { let _tmp = self.createSlideCount; self.createSlideCount += 1; _tmp };
            // java KmoSlide slide = new KmoSlide(this.mPres);
            let mut slide: KmoSlide = KmoSlide::new(self.mPres);
            // java slide.setIOReading(true);
            slide.setIOReading(true);
            // java PptxrSlide slideReader = new PptxrSlide(slide, this.mPres, part, this.mMedias, this.mThemes, this, this.mMode, this.mCommentIdCxt);
            let mut slideReader: PptxrSlide = PptxrSlide::new(slide, self.mPres, part, self.mMedias, self.mThemes, self, self.mMode, self.mCommentIdCxt);
            // java this.mSlidesReader.add(slideReader);
            self.mSlidesReader.add(slideReader);
            // java this.mPres.addSlide(slide);
            self.mPres.addSlide(slide);
            // java String fileName = getRelationShipTargetFileName(rels);
            let mut fileName: String = Self::getRelationShipTargetFileName(Some(&rels));
            // java mSlideIdMap.put(fileName, slide.id());
            mSlideIdMap.put(fileName, slide.id());
            // java if (neededFileName.equals(fileName))
            if neededFileName.equals(fileName) {
              // java break;
              break;
            }
          }
        }
      }
      */
    }

    // java public void onSlideIds(List<String> rids)
    pub fn onSlideIds(&mut self, rids: Vec<String>) {
      // java this.slideRids = rids;
      self.slideRids = rids.clone();
      // java this.createSlideCount = PptxrSlides.UnReadSlides.getCreateSlideCount(this.mPres, rids.size());
      self.createSlideCount = UnReadSlides::getCreateSlideCount(&self.mPres.borrow(), rids.len() as i32);
      let mut idx: i32 = 0;
      // java for (int idx = 0; idx < this.createSlideCount ++idx)
      while idx < self.createSlideCount {
        // java OpcrRelationship rels = this.mPresRelses.getRelationship((String)rids.get(idx));
        let rels: Option<OpcrRelationship> = self
          .mPresRelses
          .as_ref()
          .and_then(|rr| rr.getRelationship(&rids[idx as usize]));
        // java if (rels != null)
        if let Some(rels) = rels {
          // java OpcrPackagePart part = rels.getPackagePart();
          let part: Option<OpcrPackagePart> = rels.getPackagePart();
          // java if (part != null)
          if part.is_some() {
            // java KmoSlide slide = new KmoSlide(this.mPres);
            let mut slide: KmoSlide = self.mPres.borrow_mut().alloc_slide();
            // java slide.setIOReading(true);
            slide.setIOReading(true);
            // java PptxrSlide slideReader = new PptxrSlide(slide, this.mPres, part, this.mMedias, this.mThemes, this, this.mMode, this.mCommentIdCxt);
            let mut slideReader: PptxrSlide = Self::reader_for(part.as_ref().unwrap());
            // java this.mSlidesReader.add(slideReader);
            self.mSlidesReader.push(slideReader);
            // java this.mPres.addSlide(slide);
            let slide_id = slide.id();
            self.mPres.borrow_mut().addSlide(slide);
            if let Some(last) = self.mSlidesReader.last_mut() {
                last.slide_index = self.mPres.borrow().slideCount() - 1;
            }
            // java String fileName = getRelationShipTargetFileName(rels);
            let fileName: String = Self::getRelationShipTargetFileName(Some(&rels));
            // java mSlideIdMap.put(fileName, slide.id());
            Self::put_slide_id(fileName, slide_id);
            let _ = stringify!(mSlideIdMap);
          }
        }
        idx += 1;
      }
      // java if (this.mPres.slideCount() > 0)
      if self.mPres.borrow().slideCount() > 0 {
        // java this.mPres.selection().init();
        self.mPres.borrow_mut().selection_mut().init();
        // java int historySlideIndex = this.mPres.selection().getHistorySlideIndex();
        let historySlideIndex: i32 = self.mPres.borrow().selection().getHistorySlideIndex();
        // java if (historySlideIndex > 0 && historySlideIndex < this.mPres.slideCount())
        if historySlideIndex > 0 && historySlideIndex < self.mPres.borrow().slideCount() {
          // java this.mPres.selection().selectSlide(historySlideIndex);
          self.mPres.borrow_mut().selection_mut().selectSlide(historySlideIndex);
        }
      }
    }

    // java public void onSlideMasterIds(List<String> rids)
    pub fn onSlideMasterIds(&mut self, rids: Vec<String>) {
      // java for (String rid : rids)
      for rid in rids.iter() {
        // java OpcrRelationship rels = this.mPresRelses.getRelationship(rid);
        let rels: Option<OpcrRelationship> = self
          .mPresRelses
          .as_ref()
          .and_then(|rr| rr.getRelationship(rid));
        // java if (rels != null)
        if let Some(rels) = rels {
          // java OpcrPackagePart part = rels.getPackagePart();
          let part: Option<OpcrPackagePart> = rels.getPackagePart();
          // java if (part != null)
          if part.is_some() {
            // java KmoSlideMaster master = new KmoSlideMaster(this.mPres);
            let mut master: KmoSlideMaster = KmoSlideMaster::default();
            // java master.setIOReading(true);
            master.setIOReading(true);
            // java PptxrMaster masterReader = new PptxrMaster(this.mPres, master, part, this.mMedias, this.mThemes, this, this.mMode);
            let masterReader: PptxrMaster = PptxrMaster::default();
            // java this.mMastersReader.put(rels.getPartName(), masterReader);
            self.mMastersReader
              .get_or_insert_with(HashMap::new)
              .insert(rels.getPartName(), masterReader);
            // java this.mPres._addMaster(master);
            self.mPres.borrow_mut()._addMaster(master);
          }
        }
      }
    }

    // java public void onNotesMasterIds(List<String> ids)
    pub fn onNotesMasterIds(&mut self, ids: Vec<String>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java for (String notesMasterRid : ids)
      for notesMasterRid in ids.iter() {
        // java OpcrRelationship rels = this.mPresRelses.getRelationship(notesMasterRid);
        let mut rels: OpcrRelationship = self.mPresRelses.getRelationship__int(notesMasterRid);
        // java if (rels != null)
        if true {
          // java OpcrPackagePart part = rels.getPackagePart();
          let mut part: OpcrPackagePart = rels.getPackagePart();
          // java if (part != null)
          if true {
            // java KmoNoteMaster notesMaster = new KmoNoteMaster(this.mPres);
            let mut notesMaster: KmoNoteMaster = KmoNoteMaster::new(self.mPres);
            // java PptxrNotesMaster pptxrNotesMaster = new PptxrNotesMaster(notesMaster, this.mPres, part, this.mMedias, this.mThemes, this, this.mMode);
            let mut pptxrNotesMaster: PptxrNotesMaster = PptxrNotesMaster::new(notesMaster, self.mPres, part, self.mMedias, self.mThemes, self, self.mMode);
            // java this.mNotesMastersReader.put(rels.getPartName(), pptxrNotesMaster);
            self.mNotesMastersReader.put(rels.getPartName(), pptxrNotesMaster);
            // java this.mPres.setNoteMaster(notesMaster);
            self.mPres.setNoteMaster(notesMaster);
          }
          // java else
          else {
            // java KmoNoteMaster notesMaster = PptxrNotesMaster.buildDefaultNotesMaster(this.mPres, this.mMedias, this.mMode);
            let mut notesMaster: KmoNoteMaster = PptxrNotesMaster::buildDefaultNotesMaster(self.mPres, self.mMedias, self.mMode);
            // java this.mPres.setNoteMaster(notesMaster);
            self.mPres.setNoteMaster(notesMaster);
          }
        }
      }
      */
    }

    // java public void onHandoutMasterIds(List<String> ids)
    pub fn onHandoutMasterIds(&mut self, ids: Vec<String>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java for (String handoutMasterRid : ids)
      for handoutMasterRid in ids.iter() {
        // java OpcrRelationship rels = this.mPresRelses.getRelationship(handoutMasterRid);
        let mut rels: OpcrRelationship = self.mPresRelses.getRelationship__int(handoutMasterRid);
        // java if (rels != null)
        if true {
          // java OpcrPackagePart part = rels.getPackagePart();
          let mut part: OpcrPackagePart = rels.getPackagePart();
          // java if (part != null)
          if true {
            // java KmoHandoutMaster handoutMaster = new KmoHandoutMaster(this.mPres);
            let mut handoutMaster: KmoHandoutMaster = KmoHandoutMaster::new(self.mPres);
            // java PptxrHandoutMaster pptxrHandoutMaster = new PptxrHandoutMaster(handoutMaster, this.mPres, part, this.mMedias, this.mThemes, this, this.mMode);
            let mut pptxrHandoutMaster: PptxrHandoutMaster = PptxrHandoutMaster::new(handoutMaster, self.mPres, part, self.mMedias, self.mThemes, self, self.mMode);
            // java this.mHandoutMastersReader.put(rels.getPartName(), pptxrHandoutMaster);
            self.mHandoutMastersReader.put(rels.getPartName(), pptxrHandoutMaster);
            // java this.mPres.setHandoutMaster(handoutMaster);
            self.mPres.setHandoutMaster(handoutMaster);
          }
        }
      }
      */
    }

    // java private PptxrMaster getMasterReader(PptxrSlide slideReader)
    pub fn getMasterReader(&self, slideReader: &PptxrSlide) -> Option<PptxrMaster> {
      // java OpcrRelationship layoutRels = slideReader.getLayoutRels();
      let layoutRels: Option<OpcrRelationship> = slideReader.getLayoutRels();
      // java if (layoutRels == null)
      if layoutRels.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        let layoutRels = layoutRels.unwrap();
        // java OpcrPackagePart layoutPart = layoutRels.getPackagePart();
        let layoutPart: Option<OpcrPackagePart> = layoutRels.getPackagePart();
        // java if (layoutPart != null)
        if let Some(layoutPart) = layoutPart {
          // java OpcrRelationships layoutRelses = layoutPart.getRelationships();
          let layoutRelses: Option<OpcrRelationships> = layoutPart.getRelationships();
          // java if (null == layoutRelses)
          if layoutRelses.is_none() {
            // java return null;
            return None;
          }
          let layoutRelses = layoutRelses.unwrap();
          // java for (Iterator<OpcrRelationship> it = layoutRelses.iterator(); it.hasNext())
          for rels in layoutRelses.all() {
            // java OpcrRelationship rels = it.next();
            // java if (OxfPptRelations.slideMaster().getRelsType().endsWith(rels.getType()))
            if OxfPptRelations::slideMaster().getRelsType().ends_with(&rels.getType()) {
              // java return (PptxrMaster)this.mMastersReader.get(rels.getPartName());
              return self
                .mMastersReader
                .as_ref()
                .and_then(|m| m.get(&rels.getPartName()).cloned());
            }
          }
        }
        // java return null;
        return None;
      }
    }

    // java private PptxrNotesMaster getNotesMasterReader(PptxrNotesSlide notesSlideReader)
    pub fn getNotesMasterReader(&self, notesSlideReader: &PptxrNotesSlide) -> Option<PptxrNotesMaster> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OpcrRelationship notesMasterRels = notesSlideReader.getNotesMasterRels();
      let mut notesMasterRels: OpcrRelationship = notesSlideReader.getNotesMasterRels();
      // java return notesMasterRels == null ? null : (PptxrNotesMaster)this.mNotesMastersReader.get(notesMasterRels.getPartName());
      return Some((if false { None } else { Some((self.mNotesMastersReader.get(notesMasterRels.getPartName())).downcast_ref::<PptxrNotesMaster>().unwrap()) }));
      */
    }

    // java public static void changeLayout(KmoSlide emptySlide, KmoSlideLayout layout)
    pub fn changeLayout(emptySlide: &mut KmoSlide, layout: &KmoSlideLayout) {
      // java emptySlide.setLayout(layout);
      emptySlide.setLayout(layout.clone());
      // java emptySlide.spTree().initSpTreeGroupTransform();
      emptySlide.spTree_mut().initSpTreeGroupTransform();
      // java ArrayList<KmoShowShape> layoutPhs = layout.spTree().getAllPh();
      let layoutPhs: Vec<KmoShowShape> = layout
        .spTree()
        .map(|t| t.getAllPh())
        .unwrap_or_default();
      // java if (!layoutPhs.isEmpty())
      if !layoutPhs.is_empty() {
        // java ArrayList<KmoShowShape> addShapes = new ArrayList();
        let mut addShapes: Vec<KmoShowShape> = Vec::new();
        // java for (KmoShowShape layoutPh : layoutPhs)
        for layoutPh in layoutPhs.iter() {
          // java if (shapeNeedCopyFromLayout(layout, layoutPh))
          if Self::shapeNeedCopyFromLayout(layout, layoutPh) {
            // java KmoShowShape newShape = new KmoShowShape(emptySlide, layoutPh.type());
            let mut newShape: KmoShowShape = KmoShowShape::new(emptySlide, layoutPh.r#type());
            // java newShape.setNoGrp(true);
            newShape.setNoGrp(true);
            // java newShape.writePh(layoutPh.ph());
            newShape.writePh(layoutPh.ph());
            // java if (shapeNeedClearContentText(newShape))
            if Self::shapeNeedClearContentText(&newShape) {
              // java newShape.clearTextContents();
              newShape.clearTextContents();
            }
            // java else
            else {
              // java KmoTextOperator textOperator = newShape.getTextOperator();
              let mut textOperator: KmoTextOperator = newShape.getTextOperator();
              // java textOperator.getTextBody().updateProtocol(layoutPh.getTextOperator().getTextBody().protocol());
              let src_proto = layoutPh.getTextOperator().text_body.protocol();
              textOperator.getTextBody().updateProtocol(src_proto);
              // java textOperator.save(newShape);
              textOperator.save();
              newShape.text_op = Some(textOperator);
              newShape.text_saved = true;
            }
            // java addShapes.add(newShape);
            addShapes.push(newShape);
          }
        }
        // java emptySlide.spTree().insertShapes(addShapes, 0);
        emptySlide.spTree_mut().insertShapes(addShapes, 0);
      }
    }

    // java private static boolean shapeNeedCopyFromLayout(KmoSlideLayout layout, KmoShowShape shape)
    pub fn shapeNeedCopyFromLayout(layout: &KmoSlideLayout, shape: &KmoShowShape) -> bool {
      // java if (shape.type() != 1)
      if shape.r#type() != 1 {
        // java return false;
        return false;
      }
      // java else if (!shape.isPlaceHolder())
      else if !shape.isPlaceHolder() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java switch (shape.ph().type())
        match shape.ph().r#type() {
          // java case 4:
          4 => {
            let master = layout.master().expect("layout.master");
            // java if (!layout.master().hasHeaderFooter())
            if !master.hasHeaderFooter() {
              // java return false;
              return false;
            }
            // java HeaderFooter hf = layout.master().headerFooter();
            let hf: HeaderFooter = master.headerFooter();
            // java return !hf.hasDateTime() || hf.isDateTime();
            return !hf.hasDateTime() || hf.isDateTime();
          }
          // java case 5:
          5 => {
            let master = layout.master().expect("layout.master");
            // java if (!layout.master().hasHeaderFooter())
            if !master.hasHeaderFooter() {
              // java return false;
              return false;
            }
            // java HeaderFooter hf = layout.master().headerFooter();
            let hf: HeaderFooter = master.headerFooter();
            // java return !hf.hasSlideNum() || hf.isSlideNum();
            return !hf.hasSlideNum() || hf.isSlideNum();
          }
          // java case 6:
          6 => {
            let master = layout.master().expect("layout.master");
            // java if (!layout.master().hasHeaderFooter())
            if !master.hasHeaderFooter() {
              // java return false;
              return false;
            }
            // java HeaderFooter hf = layout.master().headerFooter();
            let hf: HeaderFooter = master.headerFooter();
            // java return !hf.hasFooter() || hf.isFooter();
            return !hf.hasFooter() || hf.isFooter();
          }
          // java default:
          _ => {
            // java return true;
            return true;
          }
        }
      }
    }

    // java private static boolean shapeNeedClearContentText(KmoShowShape shape)
    pub fn shapeNeedClearContentText(shape: &KmoShowShape) -> bool {
      // java int phType = shape.ph().type();
      let phType: i32 = shape.ph().r#type();
      // java return phType != 4 && phType != 5 && phType != 6;
      return phType != 4 && phType != 5 && phType != 6;
    }

}

// java public static class UnReadSlides
impl UnReadSlides {
    // java public  UnReadSlides(int slideCount)
    pub fn UnReadSlides(slideCount: i32) -> Self {
      // java this.mUnReadSlides = new boolean[slideCount];
      let n = if slideCount < 0 { 0 } else { slideCount as usize };
      let mut this = Self { mUnReadSlides: vec![false; n], mTop: 0 };
      let mut idx: i32 = 0;
      while idx < slideCount {
        // java this.mUnReadSlides[idx] = true;
        this.mUnReadSlides[idx as usize] = true;
        idx += 1;
      }
      this
    }

    // java public void resetSlides(int slideCount)
    pub fn resetSlides(&mut self, slideCount: i32) {
      // java if (slideCount > this.mUnReadSlides.length)
      if slideCount > (self.mUnReadSlides.len() as i32) {
        // java boolean[] tmps = new boolean[slideCount];
        let mut tmps: Vec<bool> = vec![Default::default(); (slideCount) as usize];
        let mut idx: i32 = 0;
        // java for (int idx = 0; idx < this.mUnReadSlides.length ++idx)
        while idx < (self.mUnReadSlides.len() as i32) {
          // java tmps[idx] = this.mUnReadSlides[idx];
          tmps[(idx) as usize] = self.mUnReadSlides[(idx) as usize];
          idx += 1;
        }
        let mut idx: i32 = (self.mUnReadSlides.len() as i32);
        // java for (int idx = this.mUnReadSlides.length; idx < slideCount ++idx)
        while idx < slideCount {
          // java tmps[idx] = true;
          tmps[(idx) as usize] = true;
          idx += 1;
        }
        // java this.mUnReadSlides = tmps;
        self.mUnReadSlides = tmps;
      }
    }

    // java public int next(KmoPresentation pres)
    pub fn next(&mut self, pres: &KmoPresentation) -> i32 {
      // java int sldIdx = -1;
      let mut sldIdx: i32 = -1;
      // java ArrayList<Integer> mPriorityList = pres.selection().getPriorityList();
      let mPriorityList = pres.selection().getPriorityList();
      // java if (mPriorityList != null && mPriorityList.size() > 0)
      if let Some(mPriorityList) = mPriorityList {
        if (mPriorityList.len() as i32) > 0 {
        let mut idx: i32 = 0;
        // java for (int idx = 0; idx < mPriorityList.size() ++idx)
        while idx < (mPriorityList.len() as i32) {
          // java int id = (Integer)mPriorityList.get(idx);
          let id: i32 = mPriorityList[idx as usize];
          // java if (id >= 0 && id < this.mUnReadSlides.length && this.mUnReadSlides[id])
          if id >= 0 && id < (self.mUnReadSlides.len() as i32) && self.mUnReadSlides[(id) as usize] {
            // java this.mUnReadSlides[id] = false;
            self.mUnReadSlides[(id) as usize] = false;
            // java return id;
            return id;
          }
          idx += 1;
        }
        }
      }
      // java sldIdx = pres.selection().activedSlideIndex();
      sldIdx = pres.selection().activedSlideIndex();
      // java if (sldIdx != -1)
      if sldIdx != -1 {
        // java if (sldIdx >= 0 && sldIdx < this.mUnReadSlides.length && this.mUnReadSlides[sldIdx])
        if sldIdx >= 0 && sldIdx < (self.mUnReadSlides.len() as i32) && self.mUnReadSlides[(sldIdx) as usize] {
          // java this.mUnReadSlides[sldIdx] = false;
          self.mUnReadSlides[(sldIdx) as usize] = false;
          // java return sldIdx;
          return sldIdx;
        }
        // java ++sldIdx;
        { let _tmp = sldIdx; sldIdx += 1; _tmp };
        // java if (sldIdx >= 0 && sldIdx < this.mUnReadSlides.length && this.mUnReadSlides[sldIdx])
        if sldIdx >= 0 && sldIdx < (self.mUnReadSlides.len() as i32) && self.mUnReadSlides[(sldIdx) as usize] {
          // java this.mUnReadSlides[sldIdx] = false;
          self.mUnReadSlides[(sldIdx) as usize] = false;
          // java return sldIdx;
          return sldIdx;
        }
        // java sldIdx -= 2;
        sldIdx = sldIdx - 2;
        // java if (sldIdx >= 0 && sldIdx < this.mUnReadSlides.length && this.mUnReadSlides[sldIdx])
        if sldIdx >= 0 && sldIdx < (self.mUnReadSlides.len() as i32) && self.mUnReadSlides[(sldIdx) as usize] {
          // java this.mUnReadSlides[sldIdx] = false;
          self.mUnReadSlides[(sldIdx) as usize] = false;
          // java return sldIdx;
          return sldIdx;
        }
        // java sldIdx += 3;
        sldIdx = sldIdx + 3;
        // java if (sldIdx >= 0 && sldIdx < this.mUnReadSlides.length && this.mUnReadSlides[sldIdx])
        if sldIdx >= 0 && sldIdx < (self.mUnReadSlides.len() as i32) && self.mUnReadSlides[(sldIdx) as usize] {
          // java this.mUnReadSlides[sldIdx] = false;
          self.mUnReadSlides[(sldIdx) as usize] = false;
          // java return sldIdx;
          return sldIdx;
        }
        // java sldIdx -= 4;
        sldIdx = sldIdx - 4;
        // java if (sldIdx >= 0 && sldIdx < this.mUnReadSlides.length && this.mUnReadSlides[sldIdx])
        if sldIdx >= 0 && sldIdx < (self.mUnReadSlides.len() as i32) && self.mUnReadSlides[(sldIdx) as usize] {
          // java this.mUnReadSlides[sldIdx] = false;
          self.mUnReadSlides[(sldIdx) as usize] = false;
          // java return sldIdx;
          return sldIdx;
        }
        // java sldIdx += 5;
        sldIdx = sldIdx + 5;
        // java if (sldIdx >= 0 && sldIdx < this.mUnReadSlides.length && this.mUnReadSlides[sldIdx])
        if sldIdx >= 0 && sldIdx < (self.mUnReadSlides.len() as i32) && self.mUnReadSlides[(sldIdx) as usize] {
          // java this.mUnReadSlides[sldIdx] = false;
          self.mUnReadSlides[(sldIdx) as usize] = false;
          // java return sldIdx;
          return sldIdx;
        }
        // java sldIdx -= 6;
        sldIdx = sldIdx - 6;
        // java if (sldIdx >= 0 && sldIdx < this.mUnReadSlides.length && this.mUnReadSlides[sldIdx])
        if sldIdx >= 0 && sldIdx < (self.mUnReadSlides.len() as i32) && self.mUnReadSlides[(sldIdx) as usize] {
          // java this.mUnReadSlides[sldIdx] = false;
          self.mUnReadSlides[(sldIdx) as usize] = false;
          // java return sldIdx;
          return sldIdx;
        }
      }
      // java sldIdx = pres.selection().thumbnailSlideIndex();
      sldIdx = pres.selection().thumbnailSlideIndex();
      // java if (sldIdx != -1)
      if sldIdx != -1 {
        // java if (sldIdx >= 0 && sldIdx < this.mUnReadSlides.length && this.mUnReadSlides[sldIdx])
        if sldIdx >= 0 && sldIdx < (self.mUnReadSlides.len() as i32) && self.mUnReadSlides[(sldIdx) as usize] {
          // java this.mUnReadSlides[sldIdx] = false;
          self.mUnReadSlides[(sldIdx) as usize] = false;
          // java return sldIdx;
          return sldIdx;
        }
        // java ++sldIdx;
        { let _tmp = sldIdx; sldIdx += 1; _tmp };
        // java if (sldIdx >= 0 && sldIdx < this.mUnReadSlides.length && this.mUnReadSlides[sldIdx])
        if sldIdx >= 0 && sldIdx < (self.mUnReadSlides.len() as i32) && self.mUnReadSlides[(sldIdx) as usize] {
          // java this.mUnReadSlides[sldIdx] = false;
          self.mUnReadSlides[(sldIdx) as usize] = false;
          // java return sldIdx;
          return sldIdx;
        }
        // java sldIdx -= 2;
        sldIdx = sldIdx - 2;
        // java if (sldIdx >= 0 && sldIdx < this.mUnReadSlides.length && this.mUnReadSlides[sldIdx])
        if sldIdx >= 0 && sldIdx < (self.mUnReadSlides.len() as i32) && self.mUnReadSlides[(sldIdx) as usize] {
          // java this.mUnReadSlides[sldIdx] = false;
          self.mUnReadSlides[(sldIdx) as usize] = false;
          // java return sldIdx;
          return sldIdx;
        }
      }
      // java sldIdx = pres.selection().getHistorySlideIndex();
      sldIdx = pres.selection().getHistorySlideIndex();
      // java if (sldIdx != -1)
      if sldIdx != -1 {
        // java if (sldIdx >= 0 && sldIdx < this.mUnReadSlides.length && this.mUnReadSlides[sldIdx])
        if sldIdx >= 0 && sldIdx < (self.mUnReadSlides.len() as i32) && self.mUnReadSlides[(sldIdx) as usize] {
          // java this.mUnReadSlides[sldIdx] = false;
          self.mUnReadSlides[(sldIdx) as usize] = false;
          // java return sldIdx;
          return sldIdx;
        }
        // java ++sldIdx;
        { let _tmp = sldIdx; sldIdx += 1; _tmp };
        // java if (sldIdx >= 0 && sldIdx < this.mUnReadSlides.length && this.mUnReadSlides[sldIdx])
        if sldIdx >= 0 && sldIdx < (self.mUnReadSlides.len() as i32) && self.mUnReadSlides[(sldIdx) as usize] {
          // java this.mUnReadSlides[sldIdx] = false;
          self.mUnReadSlides[(sldIdx) as usize] = false;
          // java return sldIdx;
          return sldIdx;
        }
        // java sldIdx -= 2;
        sldIdx = sldIdx - 2;
        // java if (sldIdx >= 0 && sldIdx < this.mUnReadSlides.length && this.mUnReadSlides[sldIdx])
        if sldIdx >= 0 && sldIdx < (self.mUnReadSlides.len() as i32) && self.mUnReadSlides[(sldIdx) as usize] {
          // java this.mUnReadSlides[sldIdx] = false;
          self.mUnReadSlides[(sldIdx) as usize] = false;
          // java return sldIdx;
          return sldIdx;
        }
      }
      let mut idx: i32 = self.mTop;
      // java for (int idx = this.mTop; idx < this.mUnReadSlides.length ++idx)
      while idx < (self.mUnReadSlides.len() as i32) {
        // java ++this.mTop;
        { let _tmp = self.mTop; self.mTop += 1; _tmp };
        // java if (this.mUnReadSlides[idx])
        if self.mUnReadSlides[(idx) as usize] {
          // java this.mUnReadSlides[idx] = false;
          self.mUnReadSlides[(idx) as usize] = false;
          // java return idx;
          return idx;
        }
        idx += 1;
      }
      // java return -1;
      return -1;
    }

    // java public static int getCreateSlideCount(KmoPresentation pres, int slideCount)
    pub fn getCreateSlideCount(pres: &KmoPresentation, slideCount: i32) -> i32 {
      // java int count = slideCount > 10 ? 10 : slideCount;
      let mut count: i32 = (if slideCount > 10 { 10 } else { slideCount });
      // java int sldIdx = pres.selection().activedSlideIndex();
      let mut sldIdx: i32 = pres.selection().activedSlideIndex();
      // java if (sldIdx != -1 && count <= sldIdx && sldIdx < slideCount)
      if sldIdx != -1 && count <= sldIdx && sldIdx < slideCount {
        let mut idx: i32 = 0;
        // java for (int idx = 0; idx < 10 ++idx)
        while idx < 10 {
          // java if (sldIdx + 1 <= slideCount)
          if sldIdx + 1 <= slideCount {
            // java ++sldIdx;
            sldIdx += 1;
          }
          idx += 1;
        }
        // java return sldIdx;
        return sldIdx;
      }
      // java else
      else {
        // java sldIdx = pres.selection().thumbnailSlideIndex();
        sldIdx = pres.selection().thumbnailSlideIndex();
        // java if (sldIdx != -1 && count <= sldIdx && sldIdx < slideCount)
        if sldIdx != -1 && count <= sldIdx && sldIdx < slideCount {
          let mut idx: i32 = 0;
          // java for (int idx = 0; idx < 10 ++idx)
        while idx < 10 {
          // java if (sldIdx + 1 <= slideCount)
          if sldIdx + 1 <= slideCount {
            // java ++sldIdx;
            sldIdx += 1;
          }
          idx += 1;
        }
        // java return sldIdx;
        return sldIdx;
      }
      // java else
      else {
        // java sldIdx = pres.selection().getHistorySlideIndex();
          sldIdx = pres.selection().getHistorySlideIndex();
          // java if (sldIdx != -1 && count <= sldIdx && sldIdx < slideCount)
          if sldIdx != -1 && count <= sldIdx && sldIdx < slideCount {
            let mut idx: i32 = 0;
            // java for (int idx = 0; idx < 10 ++idx)
            while idx < 10 {
              // java if (sldIdx + 1 <= slideCount)
              if sldIdx + 1 <= slideCount {
                // java ++sldIdx;
                sldIdx += 1;
              }
              idx += 1;
            }
            // java return sldIdx;
            return sldIdx;
          }
          // java else
          else {
            // java return count;
            return count;
          }
        }
      }
    }

}