#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

// Java: import cn.wps.base.log.IOLog
use compile_surface::IOLog;
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
// Java: import cn.wps.show.app.KmoPresentation
use compile_surface::KmoPresentation;
// Java: import cn.wps.show.app.slide.KmoNoteSlide
use compile_surface::KmoNoteSlide;
// Java: import cn.wps.show.app.slide.KmoSlide
use compile_surface::KmoSlide;
// Java: import cn.wps.show.app.theme.KmoTheme
use compile_surface::KmoTheme;
// Java: import cn.wps.show.pptxr.handler.slides.SlideHandler
use compile_surface::SlideHandler;
// Java: import cn.wps.show.pptxr.ppt.CommentIdContext
use compile_surface::CommentIdContext;
// Java: import cn.wps.show.pptxr.ppt.charts.PptxrChart
use compile_surface::PptxrChart;
// Java: import cn.wps.show.pptxr.ppt.comments.PptxrComment
use compile_surface::PptxrComment;
// Java: import cn.wps.show.pptxr.ppt.diagram.PptxrDiagramParts
use compile_surface::PptxrDiagramParts;
// Java: import cn.wps.show.pptxr.ppt.drawings.PptxrVmlDrawing
use compile_surface::PptxrVmlDrawing;
// Java: import cn.wps.show.pptxr.ppt.media.PptxrMedias
use compile_surface::PptxrMedias;
// Java: import cn.wps.show.pptxr.ppt.notesslides.PptxrNotesSlide
use compile_surface::PptxrNotesSlide;
// Java: import cn.wps.show.pptxr.ppt.theme.PptxrThemes
use compile_surface::PptxrThemes;
// Java: import cn.wps.show.pptxr.util.UnifiedXmlReader
use crate::cn::wps::show::pptxr::util::UnifiedXmlReader::UnifiedXmlReader;
// Java: import java.io.IOException
use compile_surface::IOException;
// Java: import java.io.InputStream
use compile_surface::InputStream;
// Java: import java.util.Iterator
// Java: import java.util.Map
// Java: import cn.wps.show.pptxr.ppt.slides.SlideHandlerCallback
use compile_surface::SlideHandlerCallback;
// Java: import cn.wps.show.pptxr.ppt.slides.PptxrSlides
use compile_surface::PptxrSlides;
// Java: import cn.wps.show.pptxr.ppt.slides.SlideHandlerCallbackImpl
use compile_surface::SlideHandlerCallbackImpl;
use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use std::cell::Cell;
use std::collections::HashMap;
use std::io::Cursor;
use std::rc::Rc;

pub mod compile_surface {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[derive(Clone, Debug, Default)]
    pub struct IOLog {
        pub calls: Rc<Cell<i32>>,
    }
    impl IOLog {
        pub fn d(_tag: Option<&str>, _m: &str, _e: ()) {
            // rust: no-stmt — tests inject counting via Cell on the part
        }
        pub fn d_count(count: &Rc<Cell<i32>>) {
            count.set(count.get() + 1);
        }
    }
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
        pub fn slideLayout() -> OpcRelation {
            OpcRelation { rels_type: "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout".into() }
        }
        pub fn notesSlide() -> OpcRelation {
            OpcRelation { rels_type: "notesSlide".into() }
        }
        pub fn comment() -> OpcRelation {
            OpcRelation { rels_type: "comment".into() }
        }
        pub fn vmlDrawing() -> OpcRelation {
            OpcRelation { rels_type: "vmlDrawing".into() }
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct OpcrRelationship {
        pub rel_type: String,
        pub id: String,
    }
    impl OpcrRelationship {
        pub fn getType(&self) -> String {
            self.rel_type.clone()
        }
        pub fn getPackagePart(&self) -> () {}
    }
    #[derive(Clone, Debug, Default)]
    pub struct OpcrRelationships {
        pub items: Vec<OpcrRelationship>,
        pub by_type: HashMap<String, OpcrRelationship>,
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
        pub fail: bool,
        pub iolog_count: Rc<Cell<i32>>,
    }
    impl OpcrPackagePart {
        pub fn getRelationships(&self) -> Option<OpcrRelationships> {
            self.relationships.clone()
        }
        pub fn getInputStream(&self) -> Result<Vec<u8>, ()> {
            if self.fail {
                self.iolog_count.set(self.iolog_count.get() + 1);
                Err(())
            } else {
                Ok(self.xml.clone())
            }
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct DgStore;
    #[derive(Clone, Debug, Default)]
    pub struct KmoPresentation {
        pub drawingStore: DgStore,
        pub mediaLib: (),
        pub inkLib: (),
    }
    impl KmoPresentation {
        pub fn drawingStore(&self) -> DgStore {
            self.drawingStore.clone()
        }
        pub fn mediaLib(&self) -> () {
            self.mediaLib
        }
        pub fn inkLib(&self) -> () {
            self.inkLib
        }
        pub fn selection(&self) -> Selection {
            Selection::default()
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Selection;
    impl Selection {
        pub fn activedSlide(&self) -> KmoSlide {
            KmoSlide::default()
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct KmoTheme;
    #[derive(Clone, Debug, Default)]
    pub struct KmoSlide {
        pub theme_override: Option<KmoTheme>,
        pub update_field_text: Rc<Cell<i32>>,
        pub xml_fill: Rc<Cell<i32>>,
    }
    impl KmoSlide {
        pub fn setThemeOverride(&mut self, themeOverride: KmoTheme) {
            self.theme_override = Some(themeOverride);
        }
        pub fn updateFieldText(&self) {
            self.update_field_text.set(self.update_field_text.get() + 1);
        }
        pub fn addNoteSlide(&mut self, _n: KmoNoteSlide) {}
        pub fn getThemeOverride(&self) -> KmoTheme {
            self.theme_override.clone().unwrap_or_default()
        }
        pub fn spTree(&self) -> () {}
    }
    #[derive(Clone, Debug, Default)]
    pub struct KmoNoteSlide;
    impl KmoNoteSlide {
        pub fn new(_p: Option<KmoPresentation>) -> Self {
            Self
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct SlideHandler {
        pub slide_xml_fill: Rc<Cell<i32>>,
    }
    impl SlideHandler {
        pub fn new(slide: Option<KmoSlide>, _dg: DgStore, _cb: Option<SlideHandlerCallbackImpl>) -> Self {
            Self {
                slide_xml_fill: slide.map(|s| s.xml_fill).unwrap_or_default(),
            }
        }
    }
    impl ElementHandler for SlideHandler {
        fn startElement(&self, _var1: i32, _var2: &dyn XmlRoAttr) {
            self.slide_xml_fill.set(self.slide_xml_fill.get() + 1);
        }
        fn endElement(&self, _var1: i32) {}
        fn characters(&self, _var1: &str) {}
        fn enterSubElement(&self, _var1: i32) -> Option<Box<dyn ElementHandler>> {
            None
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct CommentIdContext;
    #[derive(Clone, Debug, Default)]
    pub struct PptxrChart;
    impl PptxrChart {
        pub fn new(_a: KmoTheme, _b: Option<KmoPresentation>, _c: DgStore, _d: HashMap<String, Shape>, _e: &OpcrPackagePart, _f: Option<PptxrThemes>, _g: Option<PptxrMedias>, _h: Option<OpcrRelationship>, _i: Option<SlideHandlerCallbackImpl>) -> Self {
            Self
        }
        pub fn open(&self) {}
    }
    #[derive(Clone, Debug, Default)]
    pub struct PptxrComment;
    impl PptxrComment {
        pub fn new(_a: Option<KmoSlide>, _b: (), _c: Option<CommentIdContext>) -> Self {
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
    pub struct PptxrNotesSlide;
    impl PptxrNotesSlide {
        pub fn new(_a: KmoNoteSlide, _b: Option<KmoPresentation>, _c: (), _d: Option<PptxrMedias>, _e: Option<PptxrSlides>, _f: i32) -> Self {
            Self
        }
        pub fn open(&self) {}
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
    #[derive(Clone, Copy, Debug, Default)]
    pub struct IOException;
    #[derive(Clone, Debug, Default)]
    pub struct InputStream;
    #[derive(Clone, Debug, Default)]
    pub struct SlideHandlerCallback;
    #[derive(Clone, Debug, Default)]
    pub struct PptxrSlides;
    #[derive(Clone, Debug, Default)]
    pub struct SlideHandlerCallbackImpl {
        pub load_count: Rc<Cell<i32>>,
    }
    impl SlideHandlerCallbackImpl {
        pub fn new(_dg: DgStore, _ml: (), _ink: (), _rels: OpcrRelationships, _medias: Option<PptxrMedias>, _slides: Option<PptxrSlides>, _mode: i32) -> Self {
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
}

// java private static final String TAG = null;
const TAG: Option<&str> = None;

// java public class PptxrSlide
#[derive(Default)]
pub struct PptxrSlide {
    // java protected KmoSlide mSlide;
    pub(crate) mSlide: Option<KmoSlide>,
    // java private OpcrPackagePart mPart;
    mPart: Option<OpcrPackagePart>,
    // java protected KmoPresentation mPresentation;
    pub(crate) mPresentation: Option<KmoPresentation>,
    // java private SlideHandlerCallback slideHandlerCallback;
    slideHandlerCallback: Option<SlideHandlerCallbackImpl>,
    // java private PptxrMedias mMedias;
    mMedias: Option<PptxrMedias>,
    // java private PptxrThemes mThemes;
    mThemes: Option<PptxrThemes>,
    // java private PptxrSlides mSlides;
    mSlides: Option<PptxrSlides>,
    // java private int mMode;
    mMode: i32,
    // java private CommentIdContext mCommentIdCxt;
    mCommentIdCxt: Option<CommentIdContext>,
}

// java public class PptxrSlide
impl PptxrSlide {
    // java public  PptxrSlide(KmoSlide slide, KmoPresentation presentation, OpcrPackagePart part, PptxrMedias medias, PptxrThemes themes, PptxrSlides slides, int mode, CommentIdContext commentIdCxt)
    pub fn PptxrSlide(slide: Option<KmoSlide>, presentation: Option<KmoPresentation>, part: Option<OpcrPackagePart>, medias: Option<PptxrMedias>, themes: Option<PptxrThemes>, slides: Option<PptxrSlides>, mode: i32, commentIdCxt: Option<CommentIdContext>) -> Self {
      // java this.mSlide = slide;
      Self { mSlide: slide, mPart: part, mPresentation: presentation, slideHandlerCallback: Default::default(), mMedias: medias, mThemes: themes, mSlides: slides, mMode: mode, mCommentIdCxt: commentIdCxt }
      // java this.mPart = part;
      // java this.mPresentation = presentation;
      // java this.mMedias = medias;
      // java this.mThemes = themes;
      // java this.mSlides = slides;
      // java this.mMode = mode;
      // java this.mCommentIdCxt = commentIdCxt;
    }

    // java public void open()
    pub fn open(&mut self) {
      // java if (this.mPart != null)
      if self.mPart.is_some() {
        // java OpcrRelationships relationships = this.mPart.getRelationships();
        let relationships: Option<OpcrRelationships> = self.mPart.as_ref().and_then(|p| p.getRelationships());
        // java if (relationships != null)
        if let Some(relationships) = relationships {
          // java KmoTheme theme = null;
          let mut theme: Option<KmoTheme> = None;
          // java if (null != this.mThemes)
          if let Some(mThemes) = self.mThemes.as_ref() {
            // java OpcrRelationship themeOverrideRels = relationships.getRelationshipByType(OxfPptRelations.themeOverride().getRelsType());
            let themeOverrideRels: Option<OpcrRelationship> = relationships.getRelationshipByType(OxfPptRelations::themeOverride().getRelsType());
            // java if (themeOverrideRels != null)
            if let Some(themeOverrideRels) = themeOverrideRels {
              // java theme = this.mThemes.open(this.mPresentation, themeOverrideRels, this.mMedias);
              theme = Some(mThemes.open(self.mPresentation.clone(), themeOverrideRels, self.mMedias.clone()));
              // java this.mSlide.setThemeOverride(theme);
              if let Some(slide) = self.mSlide.as_mut() {
                  slide.setThemeOverride(theme.clone().unwrap());
              }
            }
          }
          // java else
          else {
            // java theme = this.mPresentation.selection().activedSlide().getThemeOverride();
            theme = self.mPresentation.as_ref().map(|p| p.selection().activedSlide().getThemeOverride());
          }
              // java InputStream stream = this.mPart.getInputStream();
              let stream = self.mPart.as_ref().unwrap().getInputStream();
              match stream {
                Ok(xml) => {
              // java this.slideHandlerCallback = new SlideHandlerCallbackImpl(this.mPresentation.drawingStore(), this.mPresentation.mediaLib(), this.mPresentation.inkLib(), relationships, this.mMedias, this.mSlides, this.mMode);
              let pres = self.mPresentation.clone().unwrap_or_default();
              self.slideHandlerCallback = Some(SlideHandlerCallbackImpl::new(pres.drawingStore(), pres.mediaLib(), pres.inkLib(), relationships, self.mMedias.clone(), self.mSlides.clone(), self.mMode));
              // java SlideHandler handler = new SlideHandler(this.mSlide, this.mPresentation.drawingStore(), this.slideHandlerCallback);
              let handler: SlideHandler = SlideHandler::new(self.mSlide.clone(), pres.drawingStore(), self.slideHandlerCallback.clone());
              // java UnifiedXmlReader.read((InputStream)stream, handler);
              let mut cursor = Cursor::new(xml);
              if UnifiedXmlReader::read__inputstream_elementhandler(&mut cursor, Box::new(handler)).is_err() {
              // java Throwable e
              // rust: no-stmt
                // java IOLog.d(TAG, "Throwable", e);
                IOLog::d(TAG, "Throwable", ());
                // java return;
                return;
              }
                }
                Err(()) => {
                // java IOLog.d(TAG, "Throwable", e);
                IOLog::d(TAG, "Throwable", ());
                // java return;
                return;
                }
              }
          // java this.slideHandlerCallback.load();
          if let Some(cb) = self.slideHandlerCallback.as_ref() {
              cb.load();
          }
          // java this.mSlide.updateFieldText();
          if let Some(slide) = self.mSlide.as_ref() {
              slide.updateFieldText();
          }
          /* DEFERRED_NON_FUNCTION notes/comment/chart/vml/diagram (post-MVP)
          // java OpcrRelationship notesSlideRels = this.getNotesSlideRels();
          let mut notesSlideRels: OpcrRelationship = self.getNotesSlideRels().unwrap();
          // java if (notesSlideRels != null)
          if true {
            // java KmoNoteSlide noteSlide = new KmoNoteSlide(this.mPresentation);
            let mut noteSlide: KmoNoteSlide = KmoNoteSlide::new(self.mPresentation.clone());
            // java this.mSlide.addNoteSlide(noteSlide);
            self.mSlide.as_mut().unwrap().addNoteSlide(noteSlide.clone());
            // java PptxrNotesSlide pptxrNotesSlide = new PptxrNotesSlide(noteSlide, this.mPresentation, notesSlideRels.getPackagePart(), this.mMedias, this.mSlides, this.mMode);
            let mut pptxrNotesSlide: PptxrNotesSlide = PptxrNotesSlide::new(noteSlide, self.mPresentation.clone(), notesSlideRels.getPackagePart(), self.mMedias.clone(), self.mSlides.clone(), self.mMode);
            // java pptxrNotesSlide.open();
            pptxrNotesSlide.open();
          }
          // java OpcrRelationship commentRels = this.getCommentRels();
          let mut commentRels: OpcrRelationship = self.getCommentRels().unwrap();
          // java if (commentRels != null)
          if true {
            // java PptxrComment pptxrComment = new PptxrComment(this.mSlide, commentRels.getPackagePart(), this.mCommentIdCxt);
            let mut pptxrComment: PptxrComment = PptxrComment::new(self.mSlide.clone(), commentRels.getPackagePart(), self.mCommentIdCxt.clone());
            // java pptxrComment.open();
            pptxrComment.open();
          }
          // java Map<String, Shape> chartMap = this.slideHandlerCallback.getChartMap();
          let mut chartMap: HashMap<String, Shape> = self.slideHandlerCallback.as_ref().unwrap().getChartMap();
          // java if (null != chartMap && chartMap.size() > 0)
          if true && (chartMap.len() as i32) > 0 {
            // java PptxrChart pptxrChart = new PptxrChart(theme, this.mPresentation, this.mPresentation.drawingStore(), chartMap, this.mPart, this.mThemes, this.mMedias, (OpcrRelationship)null, this.slideHandlerCallback);
            let mut pptxrChart: PptxrChart = PptxrChart::new(theme.unwrap_or_default(), self.mPresentation.clone(), self.mPresentation.clone().unwrap_or_default().drawingStore(), chartMap, self.mPart.as_ref().unwrap(), self.mThemes.clone(), self.mMedias.clone(), None, self.slideHandlerCallback.clone());
            // java pptxrChart.open();
            pptxrChart.open();
          }
          // java OpcrRelationship vmlDrawingRels = this.getVmlDrawingRels();
          let mut vmlDrawingRels: OpcrRelationship = self.getVmlDrawingRels().unwrap();
          // java if (vmlDrawingRels != null)
          if true {
            // java PptxrVmlDrawing pptxrVmlDrawing = new PptxrVmlDrawing(this.mSlide.spTree(), vmlDrawingRels.getPackagePart(), this.mMedias);
            let mut pptxrVmlDrawing: PptxrVmlDrawing = PptxrVmlDrawing::new(self.mSlide.as_ref().unwrap().spTree(), vmlDrawingRels.getPackagePart(), self.mMedias.clone());
            // java pptxrVmlDrawing.open();
            pptxrVmlDrawing.open();
          }
          // java Map<Integer, DiagramPair> diagramPairs = this.slideHandlerCallback.getDiagramPairs();
          let mut diagramPairs: HashMap<i32, DiagramPair> = self.slideHandlerCallback.as_ref().unwrap().getDiagramPairs();
          // java if (null != diagramPairs && diagramPairs.size() > 0)
          if true && (diagramPairs.len() as i32) > 0 {
            // java PptxrDiagramParts diagramParts = new PptxrDiagramParts(this.mPresentation, this.mPart, this.mMedias);
            let mut diagramParts: PptxrDiagramParts = PptxrDiagramParts::new(self.mPresentation.clone(), self.mPart.as_ref().unwrap(), self.mMedias.clone());
            // java diagramParts.setDiagramPairs(diagramPairs);
            diagramParts.setDiagramPairs(diagramPairs);
            // java diagramParts.open(this.mSlide);
            diagramParts.open(self.mSlide.clone());
          }
          */
        }
      }
    }

    // java private OpcrRelationship getRels(OpcRelation relation)
    pub fn getRels(&self, relation: OpcRelation) -> Option<OpcrRelationship> {
      // java if (this.mPart == null)
      if self.mPart.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java OpcrRelationships relses = this.mPart.getRelationships();
        let relses: Option<OpcrRelationships> = self.mPart.as_ref().and_then(|p| p.getRelationships());
        // java if (relses == null)
        if relses.is_none() {
          // java return null;
          return None;
        }
        // java else
        else {
          let it: Vec<OpcrRelationship> = relses.unwrap().iterator();
          // java for (Iterator<OpcrRelationship> it = relses.iterator(); it.hasNext())
          for rels in it {
            // java OpcrRelationship rels = it.next();
            // rust: no-stmt
            // java String relsType = relation.getRelsType();
            let relsType: String = relation.getRelsType();
            // java String type = rels.getType();
            let r#type: String = rels.getType();
            // java if (null != type && type.length() > 0 && relsType.endsWith(type))
            if !r#type.is_empty() && relsType.ends_with(&r#type) {
              // java return rels;
              return Some(rels);
            }
          }
          // java return null;
          return None;
        }
      }
    }

    // java public OpcrRelationship getLayoutRels()
    pub fn getLayoutRels(&self) -> Option<OpcrRelationship> {
      // java return this.getRels(OxfPptRelations.slideLayout());
      return self.getRels(OxfPptRelations::slideLayout());
    }

    // java public OpcrRelationship getNotesSlideRels()
    pub fn getNotesSlideRels(&self) -> Option<OpcrRelationship> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getRels(OxfPptRelations.notesSlide());
      return self.getRels(OxfPptRelations::notesSlide());
      */
    }

    // java public OpcrRelationship getCommentRels()
    pub fn getCommentRels(&self) -> Option<OpcrRelationship> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getRels(OxfPptRelations.comment());
      return self.getRels(OxfPptRelations::comment());
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

    // java public KmoSlide getSlide()
    pub fn getSlide(&self) -> Option<KmoSlide> {
      // java return this.mSlide;
      return self.mSlide.clone();
    }

}
