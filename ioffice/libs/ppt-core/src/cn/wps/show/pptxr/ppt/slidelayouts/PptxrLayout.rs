#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

// Java: import cn.wps.base.log.IOLog
use compile_surface::IOLog;
// Java: import cn.wps.drawing.app.DgStore
use compile_surface::DgStore;
// Java: import cn.wps.drawing.app.diagrams.DiagramPair
use compile_surface::DiagramPair;
// Java: import cn.wps.drawing.data.shape.Shape
use compile_surface::Shape;
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
// Java: import cn.wps.show.app.Ink.KmoInkLib
use compile_surface::KmoInkLib;
// Java: import cn.wps.show.app.media.KmoMediaLib
use compile_surface::KmoMediaLib;
// Java: import cn.wps.show.app.slide.KmoSlide
use compile_surface::KmoSlide;
// Java: import cn.wps.show.app.slide.KmoSlideLayout
use compile_surface::KmoSlideLayout;
// Java: import cn.wps.show.app.theme.KmoTheme
use compile_surface::KmoTheme;
// Java: import cn.wps.show.pptxr.handler.slidelayouts.SlideLayoutHandler
use compile_surface::SlideLayoutHandler;
// Java: import cn.wps.show.pptxr.ppt.PptxrPresentation
use compile_surface::PptxrPresentation;
// Java: import cn.wps.show.pptxr.ppt.charts.PptxrChart
use compile_surface::PptxrChart;
// Java: import cn.wps.show.pptxr.ppt.diagram.PptxrDiagramParts
use compile_surface::PptxrDiagramParts;
// Java: import cn.wps.show.pptxr.ppt.drawings.PptxrVmlDrawing
use compile_surface::PptxrVmlDrawing;
// Java: import cn.wps.show.pptxr.ppt.media.PptxrMedias
use compile_surface::PptxrMedias;
// Java: import cn.wps.show.pptxr.ppt.slides.PptxrSlides
use compile_surface::PptxrSlides;
// Java: import cn.wps.show.pptxr.ppt.slides.SlideHandlerCallbackImpl
use compile_surface::SlideHandlerCallbackImpl;
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
// Java: import java.util.Iterator
// Java: import java.util.Map
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
    pub struct DgStore;
    #[derive(Clone, Debug, Default)]
    pub struct DiagramPair;
    #[derive(Clone, Debug, Default)]
    pub struct Shape;
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
        pub fn themeOverride() -> OpcRelation {
            OpcRelation { rels_type: "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride".into() }
        }
        pub fn vmlDrawing() -> OpcRelation {
            OpcRelation { rels_type: "vmlDrawing".into() }
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct OpcrRelationship {
        pub rel_type: String,
    }
    impl OpcrRelationship {
        pub fn getType(&self) -> String {
            self.rel_type.clone()
        }
        pub fn getPackagePart(&self) -> () {}
    }
    #[derive(Clone, Debug, Default)]
    pub struct OpcrRelationships {
        pub by_type: HashMap<String, OpcrRelationship>,
        pub items: Vec<OpcrRelationship>,
    }
    impl OpcrRelationships {
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
        pub fail_rels: bool,
        pub iolog_count: Rc<Cell<i32>>,
        pub stream_calls: Rc<Cell<i32>>,
    }
    impl OpcrPackagePart {
        pub fn getRelationships(&self) -> Result<Option<OpcrRelationships>, ()> {
            if self.fail_rels {
                self.iolog_count.set(self.iolog_count.get() + 1);
                Err(())
            } else {
                Ok(self.relationships.clone())
            }
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
        pub fn getSlideLayout(&self, _t: i32) -> Option<Vec<u8>> {
            Some(br#"<p:sldLayout xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"/>"#.to_vec())
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct KmoPresentation {
        pub drawingStore: DgStore,
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
    }
    #[derive(Clone, Debug, Default)]
    pub struct KmoInkLib;
    #[derive(Clone, Debug, Default)]
    pub struct KmoMediaLib;
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
        pub film: Film,
        pub io_reading: bool,
    }
    impl KmoSlideLayout {
        pub fn new(_p: Option<KmoPresentation>) -> Self {
            Self::default()
        }
        pub fn film(&mut self) -> &mut Film {
            &mut self.film
        }
        pub fn setIOReading(&mut self, v: bool) {
            self.io_reading = v;
        }
        pub fn spTree(&self) -> () {}
    }
    #[derive(Clone, Debug, Default)]
    pub struct SlideLayoutHandler;
    impl SlideLayoutHandler {
        pub fn new(_layout: Option<KmoSlideLayout>, _dg: Option<DgStore>, _cb: Option<SlideHandlerCallbackImpl>) -> Self {
            Self
        }
    }
    impl ElementHandler for SlideLayoutHandler {
        fn startElement(&self, _var1: i32, _var2: &dyn XmlRoAttr) {}
        fn endElement(&self, _var1: i32) {}
        fn characters(&self, _var1: &str) {}
        fn enterSubElement(&self, _var1: i32) -> Option<Box<dyn ElementHandler>> {
            None
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
    pub struct PptxrChart;
    impl PptxrChart {
        pub fn new(_a: Option<KmoTheme>, _b: Option<KmoPresentation>, _c: DgStore, _d: HashMap<String, Shape>, _e: &OpcrPackagePart, _f: Option<PptxrThemes>, _g: Option<PptxrMedias>, _h: Option<OpcrRelationship>, _i: Option<SlideHandlerCallbackImpl>) -> Self {
            Self
        }
        pub fn open(&self) {}
    }
    #[derive(Clone, Debug, Default)]
    pub struct PptxrDiagramParts;
    impl PptxrDiagramParts {
        pub fn new(_a: Option<KmoPresentation>, _b: &OpcrPackagePart, _c: Option<PptxrMedias>) -> Self {
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
        pub fn getChartMap(&self) -> HashMap<String, Shape> {
            HashMap::new()
        }
        pub fn getDiagramPairs(&self) -> HashMap<i32, DiagramPair> {
            HashMap::new()
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

// java private static final String TAG = null;
const TAG: Option<&str> = None;

// java public class PptxrLayout
#[derive(Default)]
pub struct PptxrLayout {
    // java protected KmoPresentation mPresentation;
    pub(crate) mPresentation: Option<KmoPresentation>,
    // java private KmoSlideLayout mLayout;
    mLayout: Option<KmoSlideLayout>,
    // java private OpcrPackagePart mPart;
    mPart: Option<OpcrPackagePart>,
    // java protected boolean bOpen = false;
    pub bOpen: bool,
    // java private SlideHandlerCallbackImpl slideHandlerCallback;
    slideHandlerCallback: Option<SlideHandlerCallbackImpl>,
    // java private PptxrMedias mMedias;
    mMedias: Option<PptxrMedias>,
    // java private PptxrThemes mThemes;
    mThemes: Option<PptxrThemes>,
    // java private PptxrSlides mSlides;
    mSlides: Option<PptxrSlides>,
    // java private int mMode;
    mMode: i32,
}

// java public class PptxrLayout
impl PptxrLayout {
    // java public  PptxrLayout(KmoPresentation presentation, KmoSlideLayout layout, OpcrPackagePart part, PptxrMedias medias, PptxrThemes themes, PptxrSlides slides, int mode)
    pub fn PptxrLayout(presentation: Option<KmoPresentation>, layout: Option<KmoSlideLayout>, part: Option<OpcrPackagePart>, medias: Option<PptxrMedias>, themes: Option<PptxrThemes>, slides: Option<PptxrSlides>, mode: i32) -> Self {
      // java this.mPresentation = presentation;
      Self { mPresentation: presentation, mLayout: layout, mPart: part, bOpen: Default::default(), slideHandlerCallback: Default::default(), mMedias: medias, mThemes: themes, mSlides: slides, mMode: mode }
      // java this.mLayout = layout;
      // java this.mPart = part;
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
        // java if (this.mPart != null)
        if self.mPart.is_some() {
          // java OpcrRelationships relses = null;
          let mut relses: Option<OpcrRelationships> = None;
          match self.mPart.as_ref().unwrap().getRelationships() {
              Ok(r) => {
              // java relses = this.mPart.getRelationships();
              relses = r;
              }
              Err(()) => {
              // java Throwable e
              // rust: no-stmt
                // java IOLog.d(TAG, "Throwable", e);
                IOLog::d(TAG, "Throwable", ());
                // java return;
                return;
              }
          }
          // java KmoTheme themeOverride = null;
          let mut themeOverride: Option<KmoTheme> = None;
          // java OpcrRelationship themeOverrideRels = relses.getRelationshipByType(OxfPptRelations.themeOverride().getRelsType());
          let themeOverrideRels: Option<OpcrRelationship> = relses.as_ref().and_then(|r| r.getRelationshipByType(OxfPptRelations::themeOverride().getRelsType()));
          // java if (themeOverrideRels != null)
          if let Some(themeOverrideRels) = themeOverrideRels {
            // java themeOverride = this.mThemes.open(this.mPresentation, themeOverrideRels, this.mMedias);
            if let Some(themes) = self.mThemes.as_ref() {
                themeOverride = Some(themes.open(self.mPresentation.clone(), themeOverrideRels, self.mMedias.clone()));
            }
            // java this.mLayout.film().setTheme(themeOverride);
            if let Some(layout) = self.mLayout.as_mut() {
                if let Some(theme) = themeOverride.clone() {
                    layout.film().setTheme(theme);
                }
            }
          }
          let xml_try = (|| {
              // java InputStream stream = this.mPart.getInputStream();
              let stream = self.mPart.as_ref().unwrap().getInputStream();
              // java this.slideHandlerCallback = new SlideHandlerCallbackImpl(this.mPresentation.drawingStore(), this.mPresentation.mediaLib(), this.mPresentation.inkLib(), this.mPart.getRelationships(), this.mMedias, this.mSlides, this.mMode);
              let pres = self.mPresentation.clone().unwrap_or_default();
              let rels2 = self.mPart.as_ref().unwrap().getRelationships().ok().flatten();
              self.slideHandlerCallback = Some(SlideHandlerCallbackImpl::new(Some(pres.drawingStore()), pres.mediaLib(), pres.inkLib(), rels2, self.mMedias.clone(), self.mSlides.clone(), self.mMode));
              // java SlideLayoutHandler handler = new SlideLayoutHandler(this.mLayout, this.mPresentation.drawingStore(), this.slideHandlerCallback);
              let handler: SlideLayoutHandler = SlideLayoutHandler::new(self.mLayout.clone(), Some(pres.drawingStore()), self.slideHandlerCallback.clone());
              // java UnifiedXmlReader.read((InputStream)stream, handler);
              let mut cursor = Cursor::new(stream);
              UnifiedXmlReader::read__inputstream_elementhandler(&mut cursor, Box::new(handler)).map_err(|_| ())?;
              // java this.slideHandlerCallback.load();
              if let Some(cb) = self.slideHandlerCallback.as_ref() {
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
          /* DEFERRED_NON_FUNCTION chart/vml/diagram
          // java Map<String, Shape> chartMap = this.slideHandlerCallback.getChartMap();
          let mut chartMap: HashMap<String, Shape> = self.slideHandlerCallback.as_ref().unwrap().getChartMap();
          // java if (null != chartMap && chartMap.size() > 0)
          if true && (chartMap.len() as i32) > 0 {
            // java PptxrChart pptxrChart = new PptxrChart(themeOverride, this.mPresentation, this.mPresentation.drawingStore(), chartMap, this.mPart, this.mThemes, this.mMedias, (OpcrRelationship)null, this.slideHandlerCallback);
            let mut pptxrChart: PptxrChart = PptxrChart::new(themeOverride, self.mPresentation.clone(), self.mPresentation.clone().unwrap_or_default().drawingStore(), chartMap, self.mPart.as_ref().unwrap(), self.mThemes.clone(), self.mMedias.clone(), None, self.slideHandlerCallback.clone());
            // java pptxrChart.open();
            pptxrChart.open();
          }
          // java OpcrRelationship vmlDrawingRels = this.getVmlDrawingRels();
          let mut vmlDrawingRels: OpcrRelationship = self.getVmlDrawingRels().unwrap();
          // java if (vmlDrawingRels != null)
          if true {
            // java PptxrVmlDrawing pptxrVmlDrawing = new PptxrVmlDrawing(this.mLayout.spTree(), vmlDrawingRels.getPackagePart(), this.mMedias);
            let mut pptxrVmlDrawing: PptxrVmlDrawing = PptxrVmlDrawing::new(self.mLayout.as_ref().unwrap().spTree(), vmlDrawingRels.getPackagePart(), self.mMedias.clone());
            // java pptxrVmlDrawing.open();
            pptxrVmlDrawing.open();
          }
          // java Map<Integer, DiagramPair> diagramPairs = this.slideHandlerCallback.getDiagramPairs();
          let mut diagramPairs: HashMap<i32, DiagramPair> = self.slideHandlerCallback.as_ref().unwrap().getDiagramPairs();
          // java if (diagramPairs != null && diagramPairs.size() > 0)
          if true && (diagramPairs.len() as i32) > 0 {
            // java PptxrDiagramParts diagramParts = new PptxrDiagramParts(this.mPresentation, this.mPart, this.mMedias);
            let mut diagramParts: PptxrDiagramParts = PptxrDiagramParts::new(self.mPresentation.clone(), self.mPart.as_ref().unwrap(), self.mMedias.clone());
            // java diagramParts.setDiagramPairs(diagramPairs);
            diagramParts.setDiagramPairs(diagramPairs);
            // java diagramParts.open((KmoSlide)null);
            diagramParts.open(None);
          }
          */
          // java this.mLayout.setIOReading(false);
          if let Some(layout) = self.mLayout.as_mut() {
              layout.setIOReading(false);
          }
        }
      }
    }

    // java public static KmoSlideLayout buildDefaultLayout(KmoPresentation presentation, PptxrMedias medias, int mode)
    pub fn buildDefaultLayout(presentation: Option<KmoPresentation>, medias: Option<PptxrMedias>, mode: i32) -> Result<KmoSlideLayout, FileNotFoundException> {
      // java int slideSizeType = PptxrPresentation.getPresentationSlideSizeType(presentation);
      let mut slideSizeType: i32 = PptxrPresentation::getPresentationSlideSizeType(presentation.clone());
      // java InputStream stream = PptResourceFactory.getResPrivider().getSlideLayout(slideSizeType);
      let stream: Option<Vec<u8>> = PptResourceFactory::getResPrivider().getSlideLayout(slideSizeType);
      // java if (stream == null)
      if stream.is_none() {
        // java throw new FileNotFoundException("The resources is not exist.");
        return Err(FileNotFoundException::new("The resources is not exist."));
      }
      // java else
      else {
        let xml = stream.unwrap();
        // java KmoSlideLayout defaultLayout = new KmoSlideLayout(presentation);
        let mut defaultLayout: KmoSlideLayout = KmoSlideLayout::new(presentation.clone());
        // java defaultLayout.setIOReading(true);
        defaultLayout.setIOReading(true);
        // java SlideHandlerCallbackImpl slideHandlerCallback = new SlideHandlerCallbackImpl(presentation.drawingStore(), presentation.mediaLib(), presentation.inkLib(), (OpcrRelationships)null, medias, (PptxrSlides)null, mode);
        let pres = presentation.clone().unwrap_or_default();
        let mut slideHandlerCallback: SlideHandlerCallbackImpl = SlideHandlerCallbackImpl::new(Some(pres.drawingStore()), pres.mediaLib(), pres.inkLib(), None, medias, None, mode);
        // java SlideLayoutHandler handler = new SlideLayoutHandler(defaultLayout, presentation.drawingStore(), slideHandlerCallback);
        let mut handler: SlideLayoutHandler = SlideLayoutHandler::new(Some(defaultLayout.clone()), Some(pres.drawingStore()), Some(slideHandlerCallback.clone()));
            // java UnifiedXmlReader.read((InputStream)stream, handler);
            let mut cursor = Cursor::new(xml);
            let _ = UnifiedXmlReader::read__inputstream_elementhandler(&mut cursor, Box::new(handler));
              // java stream.close();
              // rust: no-stmt
        // java slideHandlerCallback.load();
        slideHandlerCallback.load();
        // java defaultLayout.setIOReading(false);
        defaultLayout.setIOReading(false);
        // java return defaultLayout;
        return Ok(defaultLayout);
      }
    }

    // java public KmoSlideLayout getLayout()
    pub fn getLayout(&self) -> Option<KmoSlideLayout> {
      // java return this.mLayout;
      return self.mLayout.clone();
    }

    // java public static void open(KmoSlideLayout layout, String xmlpath, PptxrMedias medias)
    pub fn open__kmoslidelayout_string_pptxrmedias(layout: &KmoSlideLayout, xmlpath: &str, medias: &PptxrMedias) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java SlideHandlerCallbackImpl callback = new SlideHandlerCallbackImpl((DgStore)null, (KmoMediaLib)null, (KmoInkLib)null, (OpcrRelationships)null, medias, (PptxrSlides)null, 0);
      let mut callback: SlideHandlerCallbackImpl = SlideHandlerCallbackImpl::new(None, None, None, None, Some(medias.clone()), None, 0);
      // java SlideLayoutHandler handler = new SlideLayoutHandler(layout, (DgStore)null, callback);
      let mut handler: SlideLayoutHandler = SlideLayoutHandler::new(Some(layout.clone()), None, Some(callback));
      // java UnifiedXmlReader.read((String)xmlpath, handler);
      UnifiedXmlReader::read__string_elementhandler(xmlpath, Box::new(handler));
      */
    }

    // java public static void open(KmoSlideLayout layout, InputStream xmlstream, PptxrMedias medias)
    pub fn open__kmoslidelayout_inputstream_pptxrmedias(layout: Option<KmoSlideLayout>, xmlstream: Box<Box<dyn std::io::Read>>, medias: Option<PptxrMedias>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java SlideHandlerCallbackImpl callback = new SlideHandlerCallbackImpl((DgStore)null, (KmoMediaLib)null, (KmoInkLib)null, (OpcrRelationships)null, medias, (PptxrSlides)null, 0);
      let mut callback: SlideHandlerCallbackImpl = SlideHandlerCallbackImpl::new(None, None, None, None, medias, None, 0);
      // java SlideLayoutHandler handler = new SlideLayoutHandler(layout, (DgStore)null, callback);
      let mut handler: SlideLayoutHandler = SlideLayoutHandler::new(layout, None, Some(callback));
      // java UnifiedXmlReader.read((InputStream)xmlstream, handler);
      UnifiedXmlReader::read__inputstream_elementhandler(todo!("cast → Box<dyn std::io::Read>"), handler);
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
        // java OpcrRelationships relsCol = this.mPart.getRelationships();
        let mut relsCol: OpcrRelationships = self.mPart.getRelationships();
        // java if (relsCol == null)
        if false {
          // java return null;
          return None;
        }
        let mut it: Vec<OpcrRelationship> = relsCol.iterator();
        // java for (Iterator<OpcrRelationship> it = relsCol.iterator(); it.hasNext())
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
      */
    }

}
