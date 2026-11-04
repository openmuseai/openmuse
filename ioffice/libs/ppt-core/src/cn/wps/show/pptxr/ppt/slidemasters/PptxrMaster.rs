#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

// Java: import cn.wps.base.log.IOLog
use compile_surface::IOLog;
// Java: import cn.wps.drawing.app.diagrams.DiagramPair
use compile_surface::DiagramPair;
// Java: import cn.wps.kfc.ooxfr.OxfPptRelations
use compile_surface::OxfPptRelations;
// Java: import cn.wps.kfc.opcr.OpcRelation
use compile_surface::OpcRelation;
// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use compile_surface::OpcrPackagePart;
// Java: import cn.wps.kfc.opcr.OpcrRelationship
use compile_surface::OpcrRelationship;
// Java: import cn.wps.kfc.opcr.OpcrRelationships
use compile_surface::OpcrRelationships;
// Java: import cn.wps.show.PptResourceFactory
use compile_surface::PptResourceFactory;
// Java: import cn.wps.show.app.KmoPresentation
use compile_surface::KmoPresentation;
// Java: import cn.wps.show.app.slide.KmoSlide
use compile_surface::KmoSlide;
// Java: import cn.wps.show.app.slide.KmoSlideLayout
use compile_surface::KmoSlideLayout;
// Java: import cn.wps.show.app.slide.KmoSlideMaster
use compile_surface::KmoSlideMaster;
// Java: import cn.wps.show.app.theme.KmoTheme
use compile_surface::KmoTheme;
// Java: import cn.wps.show.data.slidemasters.SlideMaster
use compile_surface::SlideMaster;
// Java: import cn.wps.show.pptxr.handler.slidemasters.SlideMasterHandler
use compile_surface::SlideMasterHandler;
// Java: import cn.wps.show.pptxr.handler.slidemasters.TextStylesHandler
use compile_surface::TextStylesHandler;
// Java: import cn.wps.show.pptxr.ppt.PptxrPresentation
use compile_surface::PptxrPresentation;
// Java: import cn.wps.show.pptxr.ppt.diagram.PptxrDiagramParts
use compile_surface::PptxrDiagramParts;
// Java: import cn.wps.show.pptxr.ppt.drawings.PptxrVmlDrawing
use compile_surface::PptxrVmlDrawing;
// Java: import cn.wps.show.pptxr.ppt.media.PptxrMedias
use compile_surface::PptxrMedias;
// Java: import cn.wps.show.pptxr.ppt.slidelayouts.PptxrLayout
use compile_surface::PptxrLayout;
// Java: import cn.wps.show.pptxr.ppt.slides.PptxrSlides
use compile_surface::PptxrSlides;
// Java: import cn.wps.show.pptxr.ppt.slides.SlideHandlerCallbackImpl
use compile_surface::SlideHandlerCallbackImpl;
// Java: import cn.wps.show.pptxr.ppt.theme.PptxrTheme
use compile_surface::PptxrTheme;
// Java: import cn.wps.show.pptxr.ppt.theme.PptxrThemes
use compile_surface::PptxrThemes;
// Java: import cn.wps.show.pptxr.util.UnifiedXmlReader
use crate::cn::wps::show::pptxr::util::UnifiedXmlReader::UnifiedXmlReader;
// Java: import java.io.FileNotFoundException
use compile_surface::FileNotFoundException;
// Java: import java.io.IOException
use compile_surface::IOException;
// Java: import java.io.InputStream
use compile_surface::InputStream;
// Java: import java.util.Collection
// Java: import java.util.HashMap
// Java: import java.util.Iterator
// Java: import java.util.Map
// Java: import cn.wps.show.pptxr.ppt.slidemasters.MasterHandlerCallbackImpl
use crate::cn::wps::show::pptxr::ppt::slidemasters::MasterHandlerCallbackImpl::MasterHandlerCallbackImpl;
use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use std::cell::Cell;
use std::collections::HashMap;
use std::io::Cursor;
use std::rc::Rc;

pub mod compile_surface {
    use super::*;

    #[derive(Clone, Debug, Default)]
    pub struct IOLog;
    impl IOLog {
        pub fn d(_tag: Option<&str>, _m: &str, _e: ()) {}
    }
    #[derive(Clone, Debug, Default)]
    pub struct DiagramPair;
    #[derive(Clone, Debug, Default)]
    pub struct OpcRelation {
        pub rels_type: String,
    }
    impl OpcRelation {
        pub fn getRelsType(&self) -> String {
            self.rels_type.clone()
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct OxfPptRelations;
    impl OxfPptRelations {
        pub fn theme() -> OpcRelation {
            OpcRelation { rels_type: "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme".into() }
        }
        pub fn vmlDrawing() -> OpcRelation {
            OpcRelation { rels_type: "vmlDrawing".into() }
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct OpcrRelationship {
        pub id: String,
        pub part_name: String,
        pub part: Option<OpcrPackagePart>,
    }
    impl OpcrRelationship {
        pub fn getPackagePart(&self) -> Option<OpcrPackagePart> {
            self.part.clone()
        }
        pub fn getPartName(&self) -> String {
            self.part_name.clone()
        }
        pub fn getType(&self) -> String {
            String::new()
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct OpcrRelationships {
        pub by_id: HashMap<String, OpcrRelationship>,
        pub by_type: HashMap<String, OpcrRelationship>,
        pub items: Vec<OpcrRelationship>,
    }
    impl OpcrRelationships {
        pub fn getRelationship(&self, id: &str) -> Option<OpcrRelationship> {
            self.by_id.get(id).cloned()
        }
        pub fn getRelationshipByType(&self, rels_type: String) -> Option<OpcrRelationship> {
            self.by_type.get(&rels_type).cloned()
        }
        pub fn iterator(&self) -> Vec<OpcrRelationship> {
            self.items.clone()
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct OpcrPackagePart {
        pub relationships: Option<OpcrRelationships>,
        pub xml: Vec<u8>,
        pub stream_calls: Rc<Cell<i32>>,
    }
    impl OpcrPackagePart {
        pub fn getRelationships(&self) -> Option<OpcrRelationships> {
            self.relationships.clone()
        }
        pub fn getInputStream(&self) -> Vec<u8> {
            self.stream_calls.set(self.stream_calls.get() + 1);
            self.xml.clone()
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct PptResourceFactory;
    impl PptResourceFactory {
        pub fn getResPrivider() -> Self {
            Self
        }
        pub fn getSlideMaster(&self, _t: i32) -> Option<Vec<u8>> {
            Some(br#"<p:sldMaster xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"/>"#.to_vec())
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct DgStore;
    #[derive(Clone, Debug, Default)]
    pub struct KmoInkLib;
    #[derive(Clone, Debug, Default)]
    pub struct KmoMediaLib;
    #[derive(Clone, Debug, Default)]
    pub struct KmoPresentation {
        pub drawingStore: DgStore,
        pub add_theme: Rc<Cell<i32>>,
    }
    impl KmoPresentation {
        pub fn drawingStore(&self) -> DgStore {
            self.drawingStore.clone()
        }
        pub fn mediaLib(&self) -> Option<KmoMediaLib> {
            None
        }
        pub fn inkLib(&self) -> Option<KmoInkLib> {
            None
        }
        pub fn _addTheme(&self, _t: KmoTheme) {
            self.add_theme.set(self.add_theme.get() + 1);
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct KmoSlide;
    #[derive(Clone, Debug, Default)]
    pub struct KmoTheme;
    #[derive(Clone, Debug, Default)]
    pub struct Film {
        pub theme: Option<KmoTheme>,
    }
    impl Film {
        pub fn setTheme(&mut self, theme: KmoTheme) {
            self.theme = Some(theme);
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct KmoSlideLayout {
        pub io_reading: bool,
        pub has_master: bool,
    }
    impl KmoSlideLayout {
        pub fn new(_p: Option<KmoPresentation>) -> Self {
            Self::default()
        }
        pub fn setIOReading(&mut self, v: bool) {
            self.io_reading = v;
        }
        pub fn setMaster(&mut self, _m: Option<KmoSlideMaster>) {
            self.has_master = true;
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct KmoSlideMaster {
        pub film: Film,
        pub io_reading: bool,
        pub layouts: Rc<Cell<i32>>,
        pub from_default: bool,
    }
    impl KmoSlideMaster {
        pub fn new(_p: Option<KmoPresentation>) -> Self {
            Self { from_default: true, ..Self::default() }
        }
        pub fn film(&mut self) -> &mut Film {
            &mut self.film
        }
        pub fn setIOReading(&mut self, v: bool) {
            self.io_reading = v;
        }
        pub fn _addLayout(&self, _l: KmoSlideLayout) {
            self.layouts.set(self.layouts.get() + 1);
        }
        pub fn spTree(&self) -> () {}
    }
    #[derive(Clone, Debug, Default)]
    pub struct SlideMaster;
    #[derive(Clone, Debug, Default)]
    pub struct SlideMasterHandler;
    impl SlideMasterHandler {
        pub fn new(_m: Option<KmoSlideMaster>, _dg: DgStore, _cb: Option<MasterHandlerCallbackImpl>) -> Self {
            Self
        }
    }
    impl ElementHandler for SlideMasterHandler {
        fn startElement(&self, _var1: i32, _var2: &dyn XmlRoAttr) {}
        fn endElement(&self, _var1: i32) {}
        fn characters(&self, _var1: &str) {}
        fn enterSubElement(&self, _var1: i32) -> Option<Box<dyn ElementHandler>> {
            None
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct TextStylesHandler;
    impl TextStylesHandler {
        pub fn new(_m: SlideMaster, _cb: SlideHandlerCallbackImpl) -> Self {
            Self
        }
    }
    /* DEFERRED_NON_FUNCTION (影子结构，owner: pp-10 layout/master · wave: S3):
     * Java `PptxrPresentation.getPresentationSlideSizeType(presentation)` 需要 `KmoPresentation`
     * 的 slideSize（type/cx/cy）。本文件的 `compile_surface::KmoPresentation` 端口面只有
     * `drawingStore`，没有 slide size 状态，因此无法接真实实现 —— 这里的 `0` 是端口占位，
     * **不是** Java 语义（Java 在 cx=cy=0 时返回 15）。
     * 真实实现在 `ppt::PptxrPresentation::getPresentationSlideSizeType`（已真实改写并测试）。
     * 本端口面的合并属于 layout/master 改写，不属 Importer 装配闭环。 */
    #[derive(Clone, Debug, Default)]
    pub struct PptxrPresentation;
    impl PptxrPresentation {
        pub fn getPresentationSlideSizeType(_p: Option<KmoPresentation>) -> i32 {
            0
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct PptxrDiagramParts;
    impl PptxrDiagramParts {
        pub fn new(_a: Option<KmoPresentation>, _b: Option<OpcrPackagePart>, _c: Option<PptxrMedias>) -> Self {
            Self
        }
        pub fn setDiagramPairs(&mut self, _p: HashMap<i32, DiagramPair>) {}
        pub fn open(&self, _s: Option<KmoSlide>) {}
    }
    #[derive(Clone, Debug, Default)]
    pub struct PptxrVmlDrawing;
    impl PptxrVmlDrawing {
        pub fn new(_a: (), _b: (), _c: Option<PptxrMedias>) -> Self {
            Self
        }
        pub fn open(&self) {}
    }
    #[derive(Clone, Debug, Default)]
    pub struct PptxrMedias;
    #[derive(Clone, Debug, Default)]
    pub struct PptxrSlides;
    #[derive(Clone, Debug, Default)]
    pub struct PptxrTheme;
    impl PptxrTheme {
        pub fn buildDefaultTheme() -> KmoTheme {
            KmoTheme::default()
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct PptxrThemes {
        pub open_count: Rc<Cell<i32>>,
    }
    impl PptxrThemes {
        pub fn open(&self, _pres: Option<KmoPresentation>, _rel: OpcrRelationship, _medias: Option<PptxrMedias>) -> KmoTheme {
            self.open_count.set(self.open_count.get() + 1);
            KmoTheme::default()
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct SlideHandlerCallbackImpl {
        pub load_count: Rc<Cell<i32>>,
    }
    impl SlideHandlerCallbackImpl {
        pub fn new(_dg: Option<DgStore>, _ml: Option<KmoMediaLib>, _ink: Option<KmoInkLib>, _rels: Option<OpcrRelationships>, _medias: Option<PptxrMedias>, _slides: Option<PptxrSlides>, _mode: i32) -> Self {
            Self { load_count: Rc::new(Cell::new(0)) }
        }
        pub fn load(&self) {
            self.load_count.set(self.load_count.get() + 1);
        }
        pub fn getDiagramPairs(&self) -> HashMap<i32, DiagramPair> {
            HashMap::new()
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct PptxrLayout {
        pub layout: Option<KmoSlideLayout>,
        pub opened: Rc<Cell<i32>>,
    }
    impl PptxrLayout {
        pub fn PptxrLayout(_pres: Option<KmoPresentation>, layout: Option<KmoSlideLayout>, _part: Option<OpcrPackagePart>, _medias: Option<PptxrMedias>, _themes: Option<PptxrThemes>, _slides: Option<PptxrSlides>, _mode: i32) -> Self {
            Self { layout, opened: Rc::new(Cell::new(0)) }
        }
        pub fn open(&mut self) {
            self.opened.set(self.opened.get() + 1);
        }
        pub fn getLayout(&self) -> Option<KmoSlideLayout> {
            self.layout.clone()
        }
        pub fn buildDefaultLayout(_p: Option<KmoPresentation>, _m: Option<PptxrMedias>, _mode: i32) -> Result<KmoSlideLayout, FileNotFoundException> {
            Ok(KmoSlideLayout::default())
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct FileNotFoundException {
        pub msg: String,
    }
    impl FileNotFoundException {
        pub fn new(msg: &str) -> Self {
            Self { msg: msg.into() }
        }
    }
    #[derive(Clone, Copy, Debug, Default)]
    pub struct IOException;
    #[derive(Clone, Debug, Default)]
    pub struct InputStream;
}

use compile_surface::DgStore;
use compile_surface::KmoInkLib;
use compile_surface::KmoMediaLib;

// java private static final String TAG = null;
const TAG: Option<&str> = None;

// java public class PptxrMaster
#[derive(Default)]
pub struct PptxrMaster {
    // java private KmoSlideMaster mMaster;
    mMaster: Option<KmoSlideMaster>,
    // java protected KmoPresentation mPresentation;
    pub(crate) mPresentation: Option<KmoPresentation>,
    // java private OpcrPackagePart mPart;
    mPart: Option<OpcrPackagePart>,
    // java protected boolean bOpen = false;
    pub bOpen: bool,
    // java private Map<String, PptxrLayout> mLayoutsReader;
    pub mLayoutsReader: HashMap<String, PptxrLayout>,
    // java private MasterHandlerCallbackImpl masterHandlerCallback;
    masterHandlerCallback: Option<MasterHandlerCallbackImpl>,
    // java private PptxrMedias mMedias;
    mMedias: Option<PptxrMedias>,
    // java private PptxrThemes mThemes;
    mThemes: Option<PptxrThemes>,
    // java private PptxrSlides mSlides;
    mSlides: Option<PptxrSlides>,
    // java private int mMode;
    mMode: i32,
}

// java public class PptxrMaster
impl PptxrMaster {
    // java public  PptxrMaster(KmoPresentation pres, KmoSlideMaster master, OpcrPackagePart part, PptxrMedias medias, PptxrThemes themes, PptxrSlides slides, int mode)
    pub fn PptxrMaster(pres: Option<KmoPresentation>, master: Option<KmoSlideMaster>, part: Option<OpcrPackagePart>, medias: Option<PptxrMedias>, themes: Option<PptxrThemes>, slides: Option<PptxrSlides>, mode: i32) -> Self {
      // java this.mPresentation = pres;
      Self { mMaster: master, mPresentation: pres, mPart: part, bOpen: Default::default(), mLayoutsReader: HashMap::new(), masterHandlerCallback: Default::default(), mMedias: medias, mThemes: themes, mSlides: slides, mMode: mode }
      // java this.mMaster = master;
      // java this.mPart = part;
      // java this.mLayoutsReader = new HashMap();
      // java this.mMedias = medias;
      // java this.mThemes = themes;
      // java this.mSlides = slides;
      // java this.mMode = mode;
    }

    // java public void open()
    pub fn open(&mut self) {
      // java if (!this.bOpen)
      if !self.bOpen {
        // java this.bOpen = true;
        self.bOpen = true;
        // java if (this.mPart == null)
        if self.mPart.is_none() {
          // java this.mMaster = buildDefaultSlideMaster(this.mPresentation, this.mMedias, this.mMode);
          self.mMaster = Self::buildDefaultSlideMaster(self.mPresentation.clone(), self.mMedias.clone(), self.mMode).ok();
        }
        // java else
        else {
          // java OpcrRelationships relses = this.mPart.getRelationships();
          let relses: Option<OpcrRelationships> = self.mPart.as_ref().and_then(|p| p.getRelationships());
          // java OpcrRelationship themeRels = null != relses ? relses.getRelationshipByType(OxfPptRelations.theme().getRelsType()) : null;
          let themeRels: Option<OpcrRelationship> = relses.as_ref().and_then(|r| r.getRelationshipByType(OxfPptRelations::theme().getRelsType()));
          // java if (themeRels != null)
          if let Some(themeRels) = themeRels {
            // java KmoTheme theme = this.mThemes.open(this.mPresentation, themeRels, this.mMedias);
            let theme: KmoTheme = self.mThemes.as_ref().map(|t| t.open(self.mPresentation.clone(), themeRels, self.mMedias.clone())).unwrap_or_default();
            // java this.mMaster.film().setTheme(theme);
            if let Some(m) = self.mMaster.as_mut() {
                m.film().setTheme(theme);
            }
          }
          // java else
          else {
            // java KmoTheme theme = PptxrTheme.buildDefaultTheme();
            let theme: KmoTheme = PptxrTheme::buildDefaultTheme();
            // java this.mMaster.film().setTheme(theme);
            if let Some(m) = self.mMaster.as_mut() {
                m.film().setTheme(theme.clone());
            }
            // java this.mPresentation._addTheme(theme);
            if let Some(p) = self.mPresentation.as_ref() {
                p._addTheme(theme);
            }
          }
          let xml_try = (|| {
              // java InputStream stream = this.mPart.getInputStream();
              let stream = self.mPart.as_ref().unwrap().getInputStream();
              // java this.masterHandlerCallback = new MasterHandlerCallbackImpl(this.mPresentation.drawingStore(), this.mPresentation.mediaLib(), this.mPresentation.inkLib(), this.mPart.getRelationships(), this, this.mMedias, this.mSlides, this.mMode);
              let pres = self.mPresentation.clone().unwrap_or_default();
              self.masterHandlerCallback = Some(MasterHandlerCallbackImpl::MasterHandlerCallbackImpl(Some(pres.drawingStore()), pres.mediaLib(), pres.inkLib(), self.mPart.as_ref().and_then(|p| p.getRelationships()), None, self.mMedias.clone(), self.mSlides.clone(), self.mMode));
              // java SlideMasterHandler handler = new SlideMasterHandler(this.mMaster, this.mPresentation.drawingStore(), this.masterHandlerCallback);
              let handler: SlideMasterHandler = SlideMasterHandler::new(self.mMaster.clone(), pres.drawingStore(), self.masterHandlerCallback.clone());
              // java UnifiedXmlReader.read((InputStream)stream, handler);
              let mut cursor = Cursor::new(stream);
              UnifiedXmlReader::read__inputstream_elementhandler(&mut cursor, Box::new(handler)).map_err(|_| ())?;
              // java this.masterHandlerCallback.load();
              if let Some(cb) = self.masterHandlerCallback.as_ref() {
                  cb.load();
              }
              Ok::<(), ()>(())
          })();
          if xml_try.is_err() {
              // java Throwable e
              // rust: no-stmt
                // java IOLog.d(TAG, "Throwable", e);
                IOLog::d(TAG, "Throwable", ());
          }
          /* DEFERRED_NON_FUNCTION vml/diagram
          // java OpcrRelationship vmlDrawingRels = this.getVmlDrawingRels();
          let mut vmlDrawingRels: OpcrRelationship = self.getVmlDrawingRels().unwrap();
          // java if (vmlDrawingRels != null)
          if true {
            // java PptxrVmlDrawing pptxrVmlDrawing = new PptxrVmlDrawing(this.mMaster.spTree(), vmlDrawingRels.getPackagePart(), this.mMedias);
            let mut pptxrVmlDrawing: PptxrVmlDrawing = PptxrVmlDrawing::new(self.mMaster.as_ref().unwrap().spTree(), vmlDrawingRels.getPackagePart().unwrap_or(()), self.mMedias.clone());
            // java pptxrVmlDrawing.open();
            pptxrVmlDrawing.open();
          }
          // java Map<Integer, DiagramPair> diagramPairs = this.masterHandlerCallback.getDiagramPairs();
          let mut diagramPairs: HashMap<i32, DiagramPair> = HashMap::new();
          // java if (diagramPairs != null && diagramPairs.size() > 0)
          if true && (diagramPairs.len() as i32) > 0 {
            // java PptxrDiagramParts diagramParts = new PptxrDiagramParts(this.mPresentation, this.mPart, this.mMedias);
            let mut diagramParts: PptxrDiagramParts = PptxrDiagramParts::new(self.mPresentation.clone(), self.mPart.clone(), self.mMedias.clone());
            // java diagramParts.setDiagramPairs(diagramPairs);
            diagramParts.setDiagramPairs(diagramPairs);
            // java diagramParts.open((KmoSlide)null);
            diagramParts.open(None);
          }
          */
        }
        // java this.mMaster.setIOReading(false);
        if let Some(m) = self.mMaster.as_mut() {
            m.setIOReading(false);
        }
      }
    }

    // java public KmoSlideMaster getMaster()
    pub fn getMaster(&self) -> Option<KmoSlideMaster> {
      // java return this.mMaster;
      return self.mMaster.clone();
    }

    // java public void onSlideLayoutIds(Map<String, Long> rids)
    pub fn onSlideLayoutIds(&mut self, rids: std::collections::HashMap<String, i64>) {
      // java if (this.mPart != null)
      if self.mPart.is_some() {
        // java OpcrRelationships relses = this.mPart.getRelationships();
        let relses: Option<OpcrRelationships> = self.mPart.as_ref().and_then(|p| p.getRelationships());
        // java Iterator<String> iter = rids.keySet().iterator();
        let keys: Vec<String> = rids.keys().cloned().collect();
        // java while (iter.hasNext())
        for key in keys {
          // java OpcrRelationship rels = relses.getRelationship((String)iter.next());
          let rels: Option<OpcrRelationship> = relses.as_ref().and_then(|r| r.getRelationship(&key));
          // java if (rels != null)
          if let Some(rels) = rels {
            // java OpcrPackagePart part = rels.getPackagePart();
            let part: Option<OpcrPackagePart> = rels.getPackagePart();
            // java if (part != null)
            if let Some(part) = part {
              // java KmoSlideLayout layout = new KmoSlideLayout(this.mPresentation);
              let mut layout: KmoSlideLayout = KmoSlideLayout::new(self.mPresentation.clone());
              // java layout.setIOReading(true);
              layout.setIOReading(true);
              // java PptxrLayout layoutReader = this.createPptxrLayout(this.mPresentation, layout, part, this.mMedias, this.mThemes, this.mSlides, this.mMode);
              let layoutReader: PptxrLayout = self.createPptxrLayout(self.mPresentation.clone(), Some(layout.clone()), Some(part), self.mMedias.clone(), self.mThemes.clone(), self.mSlides.clone(), self.mMode);
              // java this.mLayoutsReader.put(rels.getPartName(), layoutReader);
              self.mLayoutsReader.insert(rels.getPartName(), layoutReader);
              // java this.mMaster._addLayout(layout);
              if let Some(m) = self.mMaster.as_ref() {
                  m._addLayout(layout.clone());
              }
              // java layout.setMaster(this.mMaster);
              layout.setMaster(self.mMaster.clone());
            }
          }
        }
      }
    }

    // java protected PptxrLayout createPptxrLayout(KmoPresentation presentation, KmoSlideLayout layout, OpcrPackagePart part, PptxrMedias medias, PptxrThemes themes, PptxrSlides slides, int mode)
    pub fn createPptxrLayout(&self, presentation: Option<KmoPresentation>, layout: Option<KmoSlideLayout>, part: Option<OpcrPackagePart>, medias: Option<PptxrMedias>, themes: Option<PptxrThemes>, slides: Option<PptxrSlides>, mode: i32) -> PptxrLayout {
      // java return new PptxrLayout(presentation, layout, part, medias, themes, slides, mode);
      return PptxrLayout::PptxrLayout(presentation, layout, part, medias, themes, slides, mode);
    }

    // java public KmoSlideLayout openLayout(OpcrRelationship layoutRels)
    pub fn openLayout(&mut self, layoutRels: OpcrRelationship) -> Option<KmoSlideLayout> {
      // java if (!this.bOpen)
      if !self.bOpen {
        // java this.open();
        self.open();
      }
      // java PptxrLayout layoutReader = (PptxrLayout)this.mLayoutsReader.get(layoutRels.getPartName());
      let layoutReader: Option<PptxrLayout> = self.mLayoutsReader.get(&layoutRels.getPartName()).cloned();
      // java if (layoutReader != null)
      if let Some(mut layoutReader) = layoutReader {
        // java layoutReader.open();
        layoutReader.open();
        // java return layoutReader.getLayout();
        return layoutReader.getLayout();
      }
      // java else
      else {
        // java KmoSlideLayout defaultLayout = PptxrLayout.buildDefaultLayout(this.mPresentation, this.mMedias, this.mMode);
        let mut defaultLayout: KmoSlideLayout = PptxrLayout::buildDefaultLayout(self.mPresentation.clone(), self.mMedias.clone(), self.mMode).unwrap_or_default();
        // java defaultLayout.setMaster(this.mMaster);
        defaultLayout.setMaster(self.mMaster.clone());
        // java return defaultLayout;
        return Some(defaultLayout);
      }
    }

    // java public void openAllDatas()
    pub fn openAllDatas(&mut self) {
      // java if (!this.bOpen)
      if !self.bOpen {
        // java this.open();
        self.open();
      }
      // java Collection<PptxrLayout> layoutsReader = this.mLayoutsReader.values();
      let mut layoutsReader: Vec<PptxrLayout> = self.mLayoutsReader.values().cloned().collect();
      // java if (layoutsReader.size() == 0)
      if (layoutsReader.len() as i32) == 0 {
        // java KmoSlideLayout layout = PptxrLayout.buildDefaultLayout(this.mPresentation, this.mMedias, this.mMode);
        let mut layout: KmoSlideLayout = PptxrLayout::buildDefaultLayout(self.mPresentation.clone(), self.mMedias.clone(), self.mMode).unwrap_or_default();
        // java this.mMaster._addLayout(layout);
        if let Some(m) = self.mMaster.as_ref() {
            m._addLayout(layout.clone());
        }
        // java layout.setMaster(this.mMaster);
        layout.setMaster(self.mMaster.clone());
      }
      // java else
      else {
        // java for (PptxrLayout layoutReader : layoutsReader)
        for layoutReader in layoutsReader.iter_mut() {
          // java layoutReader.open();
          layoutReader.open();
        }
      }
    }

    // java public static KmoSlideMaster buildDefaultSlideMaster(KmoPresentation presentation, PptxrMedias medias, int mode)
    pub fn buildDefaultSlideMaster(presentation: Option<KmoPresentation>, medias: Option<PptxrMedias>, mode: i32) -> Result<KmoSlideMaster, FileNotFoundException> {
      // java KmoSlideMaster defaultSlideMaster = new KmoSlideMaster(presentation);
      let mut defaultSlideMaster: KmoSlideMaster = KmoSlideMaster::new(presentation.clone());
      // java KmoTheme theme = PptxrTheme.buildDefaultTheme();
      let mut theme: KmoTheme = PptxrTheme::buildDefaultTheme();
      // java defaultSlideMaster.film().setTheme(theme);
      defaultSlideMaster.film().setTheme(theme.clone());
      // java presentation._addTheme(theme);
      if let Some(p) = presentation.as_ref() {
          p._addTheme(theme);
      }
      // java int slideSizeType = PptxrPresentation.getPresentationSlideSizeType(presentation);
      let mut slideSizeType: i32 = PptxrPresentation::getPresentationSlideSizeType(presentation.clone());
      // java InputStream stream = PptResourceFactory.getResPrivider().getSlideMaster(slideSizeType);
      let stream: Option<Vec<u8>> = PptResourceFactory::getResPrivider().getSlideMaster(slideSizeType);
      // java if (stream == null)
      if stream.is_none() {
        // java throw new FileNotFoundException("The resources is not exist.");
        return Err(FileNotFoundException::new("The resources is not exist."));
      }
      // java else
      else {
        let xml = stream.unwrap();
        // java MasterHandlerCallbackImpl masterHandlerCallback = new MasterHandlerCallbackImpl(presentation.drawingStore(), presentation.mediaLib(), presentation.inkLib(), (OpcrRelationships)null, (PptxrMaster)null, medias, (PptxrSlides)null, mode);
        let pres = presentation.clone().unwrap_or_default();
        let mut masterHandlerCallback: MasterHandlerCallbackImpl = MasterHandlerCallbackImpl::MasterHandlerCallbackImpl(Some(pres.drawingStore()), pres.mediaLib(), pres.inkLib(), None, None, medias.clone(), None, mode);
        // java SlideMasterHandler handler = new SlideMasterHandler(defaultSlideMaster, presentation.drawingStore(), masterHandlerCallback);
        let mut handler: SlideMasterHandler = SlideMasterHandler::new(Some(defaultSlideMaster.clone()), pres.drawingStore(), Some(masterHandlerCallback.clone()));
            // java UnifiedXmlReader.read((InputStream)stream, handler);
            let mut cursor = Cursor::new(xml);
            let _ = UnifiedXmlReader::read__inputstream_elementhandler(&mut cursor, Box::new(handler));
              // java stream.close();
              // rust: no-stmt
        // java masterHandlerCallback.load();
        masterHandlerCallback.load();
        // java KmoSlideLayout layout = PptxrLayout.buildDefaultLayout(presentation, medias, mode);
        let mut layout: KmoSlideLayout = PptxrLayout::buildDefaultLayout(presentation.clone(), medias, mode).unwrap_or_default();
        // java defaultSlideMaster._addLayout(layout);
        defaultSlideMaster._addLayout(layout.clone());
        // java layout.setMaster(defaultSlideMaster);
        layout.setMaster(Some(defaultSlideMaster.clone()));
        // java return defaultSlideMaster;
        return Ok(defaultSlideMaster);
      }
    }

    // java public static void open(KmoPresentation pres, SlideMaster masterData, OpcrPackagePart part, PptxrMedias medias)
    pub fn open__kmopresentation_slidemaster_opcrpackagepart_pptxrmedias(pres: Option<KmoPresentation>, masterData: SlideMaster, part: Option<OpcrPackagePart>, medias: Option<PptxrMedias>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (part != null)
      if let Some(part) = part {
        // java InputStream stream = part.getInputStream();
        let mut stream: Box<dyn std::io::Read> = part.getInputStream();
        // java if (stream != null)
        if true {
          // java SlideHandlerCallbackImpl callback = new SlideHandlerCallbackImpl(pres.drawingStore(), pres.mediaLib(), pres.inkLib(), part.getRelationships(), medias, (PptxrSlides)null, 0);
          let mut callback: SlideHandlerCallbackImpl = SlideHandlerCallbackImpl::new(None, None, None, None, medias, None, 0);
          // java TextStylesHandler handler = new TextStylesHandler(masterData, callback);
          let mut handler: TextStylesHandler = TextStylesHandler::new(masterData, callback);
          // java UnifiedXmlReader.read((InputStream)stream, handler);
          UnifiedXmlReader::read__inputstream_elementhandler(todo!("cast → Box<dyn std::io::Read>"), handler);
          // java callback.load();
          callback.load();
        }
      }
      */
    }

    // java public OpcrRelationship getVmlDrawingRels()
    pub fn getVmlDrawingRels(&self) -> Option<OpcrRelationship> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getRels(OxfPptRelations.vmlDrawing());
      return self.getRels(OxfPptRelations::vmlDrawing());
      */
    }

    // java private OpcrRelationship getRels(OpcRelation relation)
    pub fn getRels(&self, relation: OpcRelation) -> Option<OpcrRelationship> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mPart == null)
      if self.mPart.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java OpcrRelationships relses = this.mPart.getRelationships();
        let mut relses: OpcrRelationships = self.mPart.getRelationships();
        // java if (null == relses)
        if false {
          // java return null;
          return None;
        }
        // java else
        else {
          let mut it: Vec<OpcrRelationship> = relses.iterator();
          // java for (Iterator<OpcrRelationship> it = relses.iterator(); it.hasNext())
          while it.hasNext() {
            // java OpcrRelationship rels = it.next();
            let mut rels: OpcrRelationship = it.next();
            // java if (relation.getRelsType().endsWith(rels.getType()))
            if relation.getRelsType().endsWith(rels.getType()) {
              // java return rels;
              return Some(rels);
            }
          }
          // java return null;
          return None;
        }
      }
      */
    }

}
