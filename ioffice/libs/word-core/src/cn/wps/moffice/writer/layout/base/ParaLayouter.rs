// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::SHD;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::{PropertySet, PropertyValue};
// Java: import cn.wps.moffice.util.UnitUtil
use util::moffice::util::UnitUtil::UnitUtil;
// Java: import cn.wps.moffice.writer.base.ViewMode
use crate::cn::wps::moffice::writer::base::ViewMode::ViewMode;
// Java: import cn.wps.moffice.writer.cache.TypoColumns
use crate::cn::wps::moffice::writer::cache::TypoColumns::TypoColumns;
// Java: import cn.wps.moffice.writer.cache.TypoList
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
// Java: import cn.wps.moffice.writer.cache.TypoMemoryObj
use crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj;
// Java: import cn.wps.moffice.writer.cache.TypoPage
use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
// Java: import cn.wps.moffice.writer.cache.TypoTextLine
use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
// Java: import cn.wps.moffice.writer.cache.TypoToolsManager
use crate::cn::wps::moffice::writer::cache::TypoToolsManager::TypoToolsManager;
// Java: import cn.wps.moffice.writer.core.CharFontTool
use crate::cn::wps::moffice::writer::core::CharFontTool::CharFontTool;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::KStyle;
// Java: import cn.wps.moffice.writer.core.RangeTool
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::core::RangeTool::RangeTool;
// Java: import cn.wps.moffice.writer.data.DefaultValue
use crate::cn::wps::moffice::writer::data::DefaultValue::DefaultValue;
use crate::cn::wps::moffice::writer::data::DefaultValue::paragraph_lineSpacing;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::IChpxTable;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
// Java: import cn.wps.moffice.writer.data.value.DropCap
use crate::cn::wps::moffice::writer::data::value::DropCap::DropCap;
// Java: import cn.wps.moffice.writer.data.value.LineSpacing
use crate::cn::wps::moffice::writer::data::value::LineSpacing::LineSpacing;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.IPropertyTool
use crate::cn::wps::moffice::writer::layout::base::env::IPropertyTool::IPropertyTool;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.PageEnv
use crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.ParaEnv
use crate::cn::wps::moffice::writer::layout::base::env::ParaEnv::ParaEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.ParaPropEnv
use crate::cn::wps::moffice::writer::layout::base::env::ParaPropEnv::ParaPropEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.writer.layout.base.line.AutoNumLayouter
use crate::cn::wps::moffice::writer::layout::base::line::AutoNumLayouter::AutoNumLayouter;
// Java: import cn.wps.moffice.writer.layout.base.tools.BRCBorderTool
use crate::cn::wps::moffice::writer::layout::base::tools::BRCBorderTool::BRCBorderTool;
// Java: import cn.wps.moffice.writer.layout.base.LayouterBase
use crate::cn::wps::moffice::writer::layout::base::LayouterBase::LayouterBase;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::Entry;
// Java: import java.lang.Object
use stubs::java::lang::Object::Object;

// java private static final String TAG = null;
const TAG: &str = "";
// java protected static final int AUTO_DEFAULT_SPACING = 280;
pub(crate) const AUTO_DEFAULT_SPACING: i32 = 280;
// java protected static final float MULTI_LINE_SPACING_FACTOR = 100.0F;
pub(crate) const MULTI_LINE_SPACING_FACTOR: f32 = 100.0f32;
// java protected static final float ADJUST_RIGHT_EPS_FACTOR = 2.1F;
pub(crate) const ADJUST_RIGHT_EPS_FACTOR: f32 = 2.1f32;
// java protected static final int ADJUST_RIGHT_CHAR_AND_LINE_DELTA = 1;
pub(crate) const ADJUST_RIGHT_CHAR_AND_LINE_DELTA: i32 = 1;
// java protected static final float FIRST_INDENT_CHARS_RATIO = 100.0F;
pub(crate) const FIRST_INDENT_CHARS_RATIO: f32 = 100.0f32;

// java public class ParaLayouter extends LayouterBase<ParaLayouter.IStateEnvoy>
#[derive(Default)]
pub struct ParaLayouter {
    // java LayouterBase<ParaLayouter.IStateEnvoy> super_
    pub(crate) super_: LayouterBase,
    // java protected AutoNumLayouter mAutoNumLayouter;
    pub(crate) mAutoNumLayouter: Option<AutoNumLayouter>,
    // java protected final DocumentEnv mDocEnv;
    pub(crate) mDocEnv: DocumentEnv,
    // java protected final ViewEnv mViewEnv;
    pub(crate) mViewEnv: ViewEnv,
    // java protected final MainState mMainState;
    pub(crate) mMainState: MainState,
    // Rust inheritance adapter for the PageViewParaLayouter Java overrides.
    pub(crate) mPageView: bool,
}

// java public interface IStateEnvoy extends LayouterBase.IStateEnvoy, AutoNumLayouter.IStateEnvoy
pub trait IStateEnvoy: crate::cn::wps::moffice::writer::layout::base::LayouterBase::IStateEnvoy {
    fn getDocument(&self) -> &dyn Document;
    fn getParaEnv(&mut self) -> &mut ParaEnv;
    fn addAutoNumString(&mut self, var1: &str, var2: &PropertySet, var3: &ViewEnv);
    // java int getCP();
    fn getCP(&self) -> i32;
    // java int getCpExResult();
    fn getCpExResult(&self) -> i32;
    // java int getRangeStart();
    fn getRangeStart(&self) -> i32;
    // java int getRangeEnd();
    fn getRangeEnd(&self) -> i32;
    // java PageEnv getPageEnv();
    fn getPageEnv(&self) -> &PageEnv;
    // java TypoPage getCurPage();
    fn getCurPage(&self) -> &TypoPage;
    // java void skipSpaceBeforePara(int var1);
    fn skipSpaceBeforePara(&mut self, var1: i32);
    // java int getLinePos();
    fn getLinePos(&self) -> i32;
}

// java public class ParaLayouter extends LayouterBase<ParaLayouter.IStateEnvoy>
impl ParaLayouter {
    // java public  ParaLayouter(DocumentEnv docEnv, ViewEnv viewEnv, MainState mainState)
    pub fn ParaLayouter(docEnv: DocumentEnv, viewEnv: ViewEnv, mainState: MainState) -> Self {
      // java this.mDocEnv = docEnv;
      Self { super_: LayouterBase::default(), mAutoNumLayouter: Some(AutoNumLayouter::default()), mDocEnv: docEnv, mViewEnv: viewEnv, mMainState: mainState, mPageView: false }
      // java this.mViewEnv = viewEnv;
      // java this.mMainState = mainState;
      // java this.mAutoNumLayouter = new AutoNumLayouter(docEnv, viewEnv);
    }

    pub fn start(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      // java if (!this.bStarted)
      if !self.super_.bStarted {
        // java this.verifyEnvoy(stateEnvoy);
        self.super_.verifyEnvoy(Some(stateEnvoy));
        // java this.onStart(stateEnvoy);
        self.onStart(stateEnvoy);
        // java this.bStarted = true;
        self.super_.bStarted = true;
      }
    }

    pub fn hasStart(&self) -> bool {
      // java return this.bStarted;
      self.super_.hasStart()
    }

    pub fn end(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      // java if (this.bStarted) { this.onEnd(stateEnvoy); }
      if self.super_.bStarted {
        self.onEnd(stateEnvoy);
      }
      // java this.bStarted = false;
      self.super_.bStarted = false;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java if (this.mAutoNumLayouter != null)
      if let Some(mut mAutoNumLayouter) = self.mAutoNumLayouter.take() {
        // java this.mAutoNumLayouter.dispose();
        mAutoNumLayouter.dispose();
        // java this.mAutoNumLayouter = null;
      }
      // java super.dispose();
      self.super_.dispose();
    }

    // java protected void onStart(IStateEnvoy stateEnvoy)
    pub fn onStart(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      // java ParaEnv paraEnv = stateEnvoy.getParaEnv();
      // rust: no-stmt
      // java this.calcFinalPropertySet(stateEnvoy);
      self.calcFinalPropertySet(stateEnvoy);
      // java this.isFirstLineOfPara(stateEnvoy);
      self.isFirstLineOfPara(stateEnvoy);
      // java PropertySet paraProperty = paraEnv.curParaPropEnv.paraProperty;
      let paraProperty: PropertySet = stateEnvoy
        .getParaEnv()
        .curParaPropEnv
        .as_ref()
        .and_then(|c| c.paraProperty.clone())
        .expect("onStart: paraProperty");
      // java paraEnv.isRTL = paraProperty.getBoolean(251, false);
      stateEnvoy.getParaEnv().isRTL = paraProperty.getBoolean__int_boolean(251, false);
      // java this.calAlign(stateEnvoy);
      self.calAlign(stateEnvoy);
      // java if (paraEnv.isFirstLineOfPara && paraEnv.curParaPropEnv.hasAutoNums)
      let (is_first, has_auto) = {
        let paraEnv = stateEnvoy.getParaEnv();
        (
          paraEnv.isFirstLineOfPara,
          paraEnv.curParaPropEnv.as_ref().map(|c| c.hasAutoNums).unwrap_or(false),
        )
      };
      if is_first && has_auto {
        // java ParaPropEnv curParaPropEnv = paraEnv.curParaPropEnv;
        let (auto_str, auto_prop) = {
          let cur = stateEnvoy.getParaEnv().curParaPropEnv.as_ref().unwrap();
          (cur.autoNumString.clone().unwrap_or_default(), cur.autoNumRunProp.clone())
        };
        // java stateEnvoy.addAutoNumString(curParaPropEnv.autoNumString, curParaPropEnv.autoNumRunProp, this.mViewEnv);
        if let Some(auto_prop) = auto_prop.as_ref() {
          stateEnvoy.addAutoNumString(&auto_str, auto_prop, &self.mViewEnv);
        }
      }
      // java this.calcParaBefore(stateEnvoy);
      self.calcParaBefore(stateEnvoy);
      // java this.calcParaAfter(stateEnvoy);
      self.calcParaAfter(stateEnvoy);
      // java this.calcLineSpace(stateEnvoy);
      self.calcLineSpace(stateEnvoy);
      // java this.calcShouldCompressLine(stateEnvoy);
      self.calcShouldCompressLine(stateEnvoy);
      {
        let paraEnv = stateEnvoy.getParaEnv();
        // java paraEnv.snapToGrid = paraProperty.getBoolean(196, true);
        paraEnv.snapToGrid = paraProperty.getBoolean__int_boolean(196, true);
        // java paraEnv.wordWrap = paraProperty.getBoolean(202, true);
        paraEnv.wordWrap = paraProperty.getBoolean__int_boolean(202, true);
        // java paraEnv.autoSpaceDE = paraProperty.getBoolean(205, true);
        paraEnv.autoSpaceDE = paraProperty.getBoolean__int_boolean(205, true);
        // java paraEnv.autoSpaceDN = paraProperty.getBoolean(206, true);
        paraEnv.autoSpaceDN = paraProperty.getBoolean__int_boolean(206, true);
        // java paraEnv.kinsoku = paraProperty.getBoolean(201, true);
        paraEnv.kinsoku = paraProperty.getBoolean__int_boolean(201, true);
        // java paraEnv.overflowPunct = paraProperty.getBoolean(203, true);
        paraEnv.overflowPunct = paraProperty.getBoolean__int_boolean(203, true);
        // java paraEnv.topLinePunct = paraProperty.getBoolean(204, false);
        paraEnv.topLinePunct = paraProperty.getBoolean__int_boolean(204, false);
        // java paraEnv.hasInitTBDs = false;
        paraEnv.hasInitTBDs = false;
        // java this.mDocEnv.pools.recycleTBDs(paraEnv.tabStops);
        if let Some(stops) = paraEnv.tabStops.as_mut() {
          self.mDocEnv.pools.recycleTBDs(std::mem::take(stops));
        }
      }
    }

    // java private void calcFinalPropertySet(IStateEnvoy stateEnvoy)
    pub fn calcFinalPropertySet(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      // java boolean isRevFinal = this.mViewEnv.isRevisionFinal();
      let isRevFinal = self.mViewEnv.isRevisionFinal();
      // java Document curDoc = stateEnvoy.getDocument();
      // rust: Java aliases the document through the mutable layout envoy.
      // The owned TextDocument outlives this call; the pointer permits the
      // same read while calcParaProp mutates the envoy.
      let doc_ptr: *const dyn Document = unsafe {
        std::mem::transmute::<&dyn Document, *const dyn Document>(stateEnvoy.getDocument())
      };
      let curDoc = unsafe { &*doc_ptr };
      // java IPapxTable papxTable = curDoc.getPapxTable();
      let papx = curDoc.getPapxTable().expect("calcFinalPropertySet: PAPX");
      // java IPapxTable.Entry para = papxTable.seek(stateEnvoy.getCP());
      let para = unsafe { IPapxTable::seek(&*papx, stateEnvoy.getCP()) }
        .expect("calcFinalPropertySet: paragraph");
      // java int paraStartCp = para.position();
      let paraStartCp = para.position();
      // java IPapxTable.Entry nextPara = para.getNext();
      let nextPara = crate::cn::wps::moffice::writer::data::IPapxTable::Entry::getNext(para.as_ref())
        .filter(|p| !p.isHead());
      // java int paraEndCp = nextPara != null ? nextPara.position() : curDoc.getLength();
      let mut paraEndCp = nextPara.as_ref().map(|p| p.position()).unwrap_or(curDoc.getLength());
      // java int envEndCp = stateEnvoy.getRangeEnd();
      let envEndCp = stateEnvoy.getRangeEnd();
      // java if (paraEndCp > envEndCp) paraEndCp = Math.max(envEndCp, stateEnvoy.getCP());
      if paraEndCp > envEndCp { paraEndCp = envEndCp.max(stateEnvoy.getCP()); }
      // java boolean isMainDoc = curDoc.getType() == 0;
      let isMainDoc = curDoc.getType__on_IDocument() == 0;
      // java IPropertyTool propTool = this.mDocEnv.propToolFactory.create(stateEnvoy.getDocument());
      let mut factory = self.mDocEnv.propToolFactory.take().unwrap_or_default();
      let mut propTool = factory.create(curDoc);
      // java boolean isPageMode = ViewMode.isPageMode(this.mViewEnv.getViewMode());
      let isPageMode = ViewMode::isPageMode(self.mViewEnv.getViewMode());
      // java IPapxTable.Entry prevPara = para.getPrev();
      let prevPara = crate::cn::wps::moffice::writer::data::IPapxTable::Entry::getPrev(para.as_ref())
        .filter(|p| !p.isHead());
      let prevAllowed = prevPara.is_some() && if isPageMode {
        paraStartCp != stateEnvoy.getRangeStart() && !(isMainDoc && paraStartCp == self.mMainState.secStart)
      } else if isMainDoc { paraStartCp != 0 } else { paraStartCp != stateEnvoy.getRangeStart() };
      // java needCalPrevParaAfter = false;
      let mut needCalPrevParaAfter = false;
      if prevAllowed {
        // java if (null != prevParaPropEnv && prevParaPropEnv.end == paraStartCp) break label122;
        let cached = stateEnvoy.getParaEnv().prevParaPropEnv.as_ref()
          .map(|e| e.end == paraStartCp).unwrap_or(false);
        if !cached {
          // java prevParaPropEnv = this.mDocEnv.pools.obtainParaPropEnv();
          let prev = prevPara.as_ref().unwrap();
          let mut prevParaPropEnv = self.mDocEnv.pools.obtainParaPropEnv();
          // java prevParaPropEnv.start = prevPara.position(); prevParaPropEnv.end = paraStartCp;
          prevParaPropEnv.start = prev.position();
          prevParaPropEnv.end = paraStartCp;
          // java prevParaPropEnv.paraProperty = this.getFinalParaProp(...);
          let prop = self.getFinalParaProp(stateEnvoy, propTool.as_mut().as_mut(), prev.as_ref(), isRevFinal);
          // java prevParaPropEnv.firstNormalParaProperty = this.getNormalParaFinalProperty(...);
          prevParaPropEnv.firstNormalParaProperty = Some(self.getNormalParaFinalProperty(stateEnvoy, prev.as_ref(), &prop));
          prevParaPropEnv.paraProperty = Some(prop);
          // java this.calcParaProp(prevParaPropEnv, stateEnvoy);
          self.calcParaProp(&mut prevParaPropEnv, stateEnvoy);
          stateEnvoy.getParaEnv().prevParaPropEnv = Some(prevParaPropEnv);
          needCalPrevParaAfter = true;
        }
      } else {
        // java this.mDocEnv.pools.recycleParaPropEnv(paraEnv.prevParaPropEnv); paraEnv.prevParaPropEnv = null;
        if let Some(prevParaPropEnv) = stateEnvoy.getParaEnv().prevParaPropEnv.take() {
          self.mDocEnv.pools.recycleParaPropEnv(prevParaPropEnv);
        }
      }
      // java ParaPropEnv nextParaPropEnv = paraEnv.nextParaPropEnv; paraEnv.nextParaPropEnv = null;
      let cachedNext = stateEnvoy.getParaEnv().nextParaPropEnv.take();
      if let Some(mut nextParaPropEnv) = cachedNext.filter(|e| e.start == paraStartCp) {
        // java paraEnv.curParaPropEnv = nextParaPropEnv;
        // java nextParaPropEnv.setLayoutRange(stateEnvoy.getCP(), paraEndCp, curDoc);
        nextParaPropEnv.setLayoutRange(stateEnvoy.getCP(), paraEndCp, curDoc);
        stateEnvoy.getParaEnv().curParaPropEnv = Some(nextParaPropEnv);
      } else {
        // java ParaPropEnv propEnv = this.mDocEnv.pools.obtainParaPropEnv();
        let mut propEnv = self.mDocEnv.pools.obtainParaPropEnv();
        // java propEnv.start = paraStartCp; propEnv.setLayoutRange(...);
        propEnv.start = paraStartCp;
        propEnv.setLayoutRange(stateEnvoy.getCP(), paraEndCp, curDoc);
        // java propEnv.paraProperty = this.getFinalParaProp(...);
        let prop = self.getFinalParaProp(stateEnvoy, propTool.as_mut().as_mut(), para.as_ref(), isRevFinal);
        // java propEnv.firstNormalParaProperty = this.getNormalParaFinalProperty(...);
        propEnv.firstNormalParaProperty = Some(self.getNormalParaFinalProperty(stateEnvoy, para.as_ref(), &prop));
        propEnv.paraProperty = Some(prop);
        // java this.calcParaProp(propEnv, stateEnvoy);
        self.calcParaProp(&mut propEnv, stateEnvoy);
        if let Some(prevProp) = stateEnvoy.getParaEnv().prevParaPropEnv.as_ref().and_then(|p| p.paraProperty.clone()) {
          // java this.calclastBrcBetween(propEnv, paraEnv.prevParaPropEnv.paraProperty);
          self.calclastBrcBetween(&mut propEnv, &prevProp);
          if propEnv.lastBrcBetween.is_some() {
            stateEnvoy.getParaEnv().prevParaPropEnv.as_mut().unwrap().vaildBRC = true;
          }
        }
        stateEnvoy.getParaEnv().curParaPropEnv = Some(propEnv);
      }
      // java label82: skip next paragraph outside the flow or current section.
      let nextAllowed = nextPara.is_some() && if isPageMode {
        paraEndCp < envEndCp && !(isMainDoc && paraEndCp >= self.mMainState.secEnd)
      } else if isMainDoc { paraEndCp < curDoc.getLength() && paraEndCp < envEndCp }
        else { paraEndCp < envEndCp };
      if nextAllowed {
        let next = nextPara.as_ref().unwrap();
        // java ParaPropEnv nextPropEnv = this.mDocEnv.pools.obtainParaPropEnv();
        let mut nextPropEnv = self.mDocEnv.pools.obtainParaPropEnv();
        // java nextPropEnv.start = paraEndCp; nextPropEnv.end = nextPara.endCp();
        nextPropEnv.start = paraEndCp;
        nextPropEnv.end = next.endCp();
        if nextPropEnv.end > envEndCp { nextPropEnv.end = envEndCp.max(stateEnvoy.getCP()); }
        // java nextPropEnv.paraProperty = this.getFinalParaProp(...);
        let prop = self.getFinalParaProp(stateEnvoy, propTool.as_mut().as_mut(), next.as_ref(), isRevFinal);
        // java nextPropEnv.firstNormalParaProperty = this.getNormalParaFinalProperty(...);
        nextPropEnv.firstNormalParaProperty = Some(self.getNormalParaFinalProperty(stateEnvoy, next.as_ref(), &prop));
        nextPropEnv.paraProperty = Some(prop);
        // java this.calcParaProp(nextPropEnv, stateEnvoy);
        self.calcParaProp(&mut nextPropEnv, stateEnvoy);
        if let Some(curProp) = stateEnvoy.getParaEnv().curParaPropEnv.as_ref().and_then(|p| p.paraProperty.clone()) {
          // java this.calclastBrcBetween(nextPropEnv, paraEnv.curParaPropEnv.paraProperty);
          self.calclastBrcBetween(&mut nextPropEnv, &curProp);
          if nextPropEnv.lastBrcBetween.is_some() {
            stateEnvoy.getParaEnv().curParaPropEnv.as_mut().unwrap().vaildBRC = true;
          }
        }
        stateEnvoy.getParaEnv().nextParaPropEnv = Some(nextPropEnv);
      }
      // java if (needCalPrevParaAfter) this.calcPrevParaAfter(stateEnvoy);
      if needCalPrevParaAfter { self.calcPrevParaAfter(stateEnvoy); }
      self.mDocEnv.propToolFactory = Some(factory);

    }

    // java protected PropertySet getFinalParaProp(IStateEnvoy stateEnvoy, IPropertyTool propTool, IPapxTable.Entry paraEntry, boolean revisionFinal)
    pub fn getFinalParaProp(&self, _stateEnvoy: &dyn IStateEnvoy, propTool: &mut dyn IPropertyTool, paraEntry: &dyn crate::cn::wps::moffice::writer::data::IPapxTable::Entry, revisionFinal: bool) -> PropertySet {
      // java return propTool.getFinalParaProp(paraEntry, revisionFinal);
      propTool.getFinalParaProp__entry_boolean(paraEntry, revisionFinal).clone()

    }

    // java private void calclastBrcBetween(ParaPropEnv propEnv, PropertySet paraProperty)
    pub fn calclastBrcBetween(&mut self, propEnv: &mut ParaPropEnv, paraProperty: &PropertySet) {
      // java propEnv.lastBrcBetween = BRCBorderTool.getVaildBrc((BRC)paraProperty.getObject(233, (Object)null));
      let brc = paraProperty.getValue(233).and_then(|v| v.as_ref().as_any().downcast_ref::<BRC>());
      propEnv.lastBrcBetween = BRCBorderTool::getVaildBrc(brc);
      // java if (null != propEnv.lastBrcBetween)
      if propEnv.lastBrcBetween.is_some() {
        // java propEnv.vaildBRC = true;
        propEnv.vaildBRC = true;
      }
    }

    // java private void calcParaProp(ParaPropEnv propEnv, IStateEnvoy stateEnvoy)
    pub fn calcParaProp(&mut self, propEnv: &mut ParaPropEnv, stateEnvoy: &mut dyn IStateEnvoy) {
      if std::env::var("WORD_PARITY_AUTONUM_TRACE_CP").ok().and_then(|s| s.parse::<i32>().ok())
        .is_some_and(|cp| propEnv.start <= cp && cp < propEnv.end) {
        let p = propEnv.paraProperty.as_ref();
        eprintln!("autonum para start={} end={} lfo={:?} lvl={:?} has_table={}", propEnv.start, propEnv.end,
          p.map(|x| x.getInt__int_int(235, -1)), p.map(|x| x.getInt__int_int(234, -1)),
          self.mDocEnv.textDocument.as_ref().and_then(|d| d.mAutoNumTable.as_ref()).is_some());
      }
      // java this.calcAutoNum(propEnv, stateEnvoy);
      self.calcAutoNum(propEnv, stateEnvoy);
      // java this.calcIndents(propEnv, stateEnvoy);
      self.calcIndents(propEnv, stateEnvoy);
      // java this.calcBRC(propEnv);
      self.calcBRC(propEnv);
      // java this.calcSHD(propEnv);
      self.calcSHD(propEnv);
    }

    // java protected void calcBRC(ParaPropEnv propEnv)
    pub fn calcBRC(&mut self, propEnv: &mut ParaPropEnv) {
      // java PropertySet paraProperty = propEnv.paraProperty;
      let Some(paraProperty) = propEnv.paraProperty.as_ref() else { return; };
      // java propEnv.brcLeft = BRCBorderTool.getVaildBrc((BRC)paraProperty.getObject(230, (Object)null));
      propEnv.brcLeft = BRCBorderTool::getVaildBrc(paraProperty.getValue(230).and_then(|v| v.as_ref().as_any().downcast_ref::<BRC>()));
      // java propEnv.brcTop = BRCBorderTool.getVaildBrc((BRC)paraProperty.getObject(229, (Object)null));
      propEnv.brcTop = BRCBorderTool::getVaildBrc(paraProperty.getValue(229).and_then(|v| v.as_ref().as_any().downcast_ref::<BRC>()));
      // java propEnv.brcRight = BRCBorderTool.getVaildBrc((BRC)paraProperty.getObject(232, (Object)null));
      propEnv.brcRight = BRCBorderTool::getVaildBrc(paraProperty.getValue(232).and_then(|v| v.as_ref().as_any().downcast_ref::<BRC>()));
      // java propEnv.brcBottom = BRCBorderTool.getVaildBrc((BRC)paraProperty.getObject(231, (Object)null));
      propEnv.brcBottom = BRCBorderTool::getVaildBrc(paraProperty.getValue(231).and_then(|v| v.as_ref().as_any().downcast_ref::<BRC>()));
      // java propEnv.vaildBRC = propEnv.brcLeft != null || propEnv.brcTop != null || propEnv.brcRight != null || propEnv.brcBottom != null;
      propEnv.vaildBRC = propEnv.brcLeft.is_some() || propEnv.brcTop.is_some() || propEnv.brcRight.is_some() || propEnv.brcBottom.is_some();
    }

    // java protected void calcSHD(ParaPropEnv propEnv)
    pub fn calcSHD(&mut self, propEnv: &mut ParaPropEnv) {
      // java propEnv.paraSHD = this.getVaildShd((SHD)propEnv.paraProperty.getObject(228, (Object)null));
      let shd = propEnv.paraProperty.as_ref()
        .and_then(|prop| prop.getValue(228))
        .and_then(|value| value.as_ref().as_any().downcast_ref::<SHD>());
      propEnv.paraSHD = self.getVaildShd(shd);
    }

    // java private SHD getVaildShd(SHD shd)
    pub fn getVaildShd(&self, shd: Option<&SHD>) -> Option<SHD> {
      // java return null != shd && shd.isValid() ? shd : null;
      shd.filter(|value| value.isValid()).cloned()
    }

    // java protected void calcAutoNum(ParaPropEnv propEnv, IStateEnvoy stateEnvoy)
    pub fn calcAutoNum(&mut self, propEnv: &mut ParaPropEnv, stateEnvoy: &mut dyn IStateEnvoy) {
      // java this.mAutoNumLayouter.calAutoNumString(propEnv, stateEnvoy);
      // The Java layouter shares DocumentEnv and ViewEnv object identities with its owner.
      // Temporarily move the Rust-owned environments into the helper for this synchronous call.
      let mut helper = self.mAutoNumLayouter.take().expect("AutoNumLayouter");
      helper.mDocEnv = Some(std::mem::take(&mut self.mDocEnv));
      helper.mViewEnv = Some(std::mem::take(&mut self.mViewEnv));
      helper.calAutoNumString(propEnv, stateEnvoy);
      if std::env::var("WORD_PARITY_AUTONUM_TRACE_CP").ok().and_then(|s| s.parse::<i32>().ok())
        .is_some_and(|cp| propEnv.start <= cp && cp < propEnv.end) {
        eprintln!("autonum result start={} has={} text={:?} runprop={}", propEnv.start,
          propEnv.hasAutoNums, propEnv.autoNumString, propEnv.autoNumRunProp.is_some());
      }
      self.mDocEnv = helper.mDocEnv.take().expect("DocumentEnv");
      self.mViewEnv = helper.mViewEnv.take().expect("ViewEnv");
      self.mAutoNumLayouter = Some(helper);
    }

    // java protected void onEnd(IStateEnvoy stateEnvoy)
    pub fn onEnd(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      // java stateEnvoy.getParaEnv().saveBrcAndShdAjustInfo(this.mDocEnv.pools);
      stateEnvoy.getParaEnv().saveBrcAndShdAjustInfo(&mut self.mDocEnv.pools);
    }

    // java protected final boolean isLineWithParaBreakAndSectionBreak(int line)
    pub fn isLineWithParaBreakAndSectionBreak(&self, line: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (0 == line)
      if 0 == line {
        // java return false;
        return false;
      }
      // java else
      else {
        // java TypoToolsManager typoTools = this.mMainState.snapshot.getTypoTools();
        let mut typoTools: TypoToolsManager = self.mMainState.snapshot.getTypoTools();
        // java TypoMemoryObj memObj = typoTools.obtain(line);
        let mut memObj: TypoMemoryObj = typoTools.obtain(line);
        // java if (3 != memObj.getType())
        if 3 != memObj.getType() {
          // java if (13 == memObj.getType())
          if 13 == memObj.getType() {
            // java TypoColumns columns = (TypoColumns)memObj;
            let mut columns: TypoColumns = (memObj).downcast_ref::<TypoColumns>().unwrap();
            // java int count = columns.getColumnsCount();
            let mut count: i32 = columns.getColumnsCount();
            // java if (count != 0)
            if count != 0 {
              // java return this.isLineWithParaBreakAndSectionBreak(columns.getColumnPage(count - 1));
              return self.isLineWithParaBreakAndSectionBreak(columns.getColumnPage(count - 1));
            }
          }
          // java typoTools.recycle(memObj);
          typoTools.recycle(memObj);
          // java return false;
          return false;
        }
        // java else
        else {
          // java TypoTextLine textLine = (TypoTextLine)memObj;
          let mut textLine: TypoTextLine = (memObj).downcast_ref::<TypoTextLine>().unwrap();
          // java return textLine.isLastLineOfPara() && textLine.isLastLineOfSection();
          return textLine.isLastLineOfPara() && textLine.isLastLineOfSection();
        }
      }
      */
    }

    // java protected final boolean isAutoAfterSpacing(PropertySet prop)
    pub fn isAutoAfterSpacing(&self, prop: Option<&PropertySet>) -> bool {
      // java return null == prop ? false : prop.getBoolean(216, false);
      return (if prop.is_none() { false } else { prop.unwrap().getBoolean__int_boolean(216, false) });
    }

    // java protected boolean shouldCalcLastParaAfter(IStateEnvoy stateEnvoy)
    pub fn shouldCalcLastParaAfter(&self, stateEnvoy: &dyn IStateEnvoy) -> bool {
      // Java: ParaLayouter returns false; PageViewParaLayouter overrides it.
      self.mPageView && stateEnvoy.getPageEnv().isFirstPageOfSection && !stateEnvoy.getCurPage().hasLines()
    }

    // java protected boolean shouldAdjustRightMarginInCharGridModel()
    pub fn shouldAdjustRightMarginInCharGridModel(&self) -> bool {
      // Java: base returns false; PageViewParaLayouter returns true.
      self.mPageView
    }

    // java protected void calcParaBefore(IStateEnvoy stateEnvoy)
    pub fn calcParaBefore(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      let gridModel: i32 = stateEnvoy.getPageEnv().gridModel;
      let gridHeight: i32 = stateEnvoy.getPageEnv().gridHeight;
      let linePos: i32 = stateEnvoy.getLinePos();
      let curLines: i32 = stateEnvoy.getCurPage().getLines();
      let rangeStart: i32 = stateEnvoy.getRangeStart();
      // java ParaEnv paraEnv = stateEnvoy.getParaEnv();
      let paraEnv: &mut ParaEnv = stateEnvoy.getParaEnv();
      // java ParaPropEnv curParaPropEnv = paraEnv.curParaPropEnv;
      let curParaPropEnv = paraEnv.curParaPropEnv.as_mut();
      // java curParaPropEnv.paraBefore = 0;
      if let Some(curParaPropEnv) = curParaPropEnv {
        curParaPropEnv.paraBefore = 0;
      }
      // java PropertySet paraProp = paraEnv.curParaPropEnv.paraProperty;
      let paraProp: Option<PropertySet> = paraEnv.curParaPropEnv.as_ref().and_then(|c| c.paraProperty.clone());
      let Some(paraProp) = paraProp else { return; };
      // java Boolean isAutoSpacing = (Boolean)paraProp.getObject(219, (Object)null);
      let isAutoSpacing: Option<bool> = paraProp.getValue(219).and_then(|v| v.as_ref().as_any().downcast_ref::<bool>().copied());
      // java if (null != isAutoSpacing && isAutoSpacing)
      if isAutoSpacing == Some(true) {
        // java if (stateEnvoy.getRangeStart() != curParaPropEnv.start)
        let cur_start: i32 = paraEnv.curParaPropEnv.as_ref().map(|c| c.start).unwrap_or(-1);
        if rangeStart != cur_start {
          // java PropertySet prevProp = paraEnv.prevParaPropEnv == null ? null : paraEnv.prevParaPropEnv.paraProperty;
          let prevProp: Option<PropertySet> = paraEnv.prevParaPropEnv.as_ref().and_then(|p| p.paraProperty.clone());
          // java if (!curParaPropEnv.hasAutoNums || null == prevProp || !this.isAutoAfterSpacing(prevProp) || !this.hasSameAutoNum(prevProp, paraProp))
          let hasAutoNums: bool = paraEnv.curParaPropEnv.as_ref().map(|c| c.hasAutoNums).unwrap_or(false);
          if !hasAutoNums || prevProp.is_none() || !self.isAutoAfterSpacing(prevProp.as_ref()) || !self.hasSameAutoNum(prevProp.as_ref().unwrap(), &paraProp) {
            // java curParaPropEnv.paraBefore = 280;
            if let Some(curParaPropEnv) = paraEnv.curParaPropEnv.as_mut() {
              curParaPropEnv.paraBefore = 280;
            }
          }
        }
      }
      // java else
      else {
        // java if (paraEnv.prevParaPropEnv != null && paraEnv.prevParaPropEnv.paraProperty != null && ViewMode.isPageMode(this.mViewEnv.getViewMode()))
        if paraEnv.prevParaPropEnv.is_some() && paraEnv.prevParaPropEnv.as_ref().and_then(|p| p.paraProperty.as_ref()).is_some() && ViewMode::isPageMode(self.mViewEnv.getViewMode()) {
          // java DropCap dropCap = (DropCap)paraEnv.prevParaPropEnv.paraProperty.getObject(256, (Object)null);
          let dropCap: Option<DropCap> = paraEnv.prevParaPropEnv.as_ref().and_then(|p| p.paraProperty.as_ref()).and_then(|p| p.getValue(256)).and_then(|v| v.as_ref().as_any().downcast_ref::<DropCap>().cloned());
          // java if (dropCap != null && dropCap.getLines() != 0 && stateEnvoy.getLinePos() == 1)
          if dropCap.as_ref().map(|d| d.getLines() != 0).unwrap_or(false) && linePos == 1 {
            // java int pFirsLinst = TypoList.first(stateEnvoy.getCurPage().getLines(), this.mMainState.snapshot);
            let pFirsLinst: i32 = TypoList::first__int_typosnapshot(curLines, &mut self.mMainState.snapshot);
            // java if (TypoMemoryObj.getType(pFirsLinst, this.mMainState.snapshot) == 10)
            if TypoMemoryObj::getType__int_typosnapshot(pFirsLinst, &mut self.mMainState.snapshot) == 10 {
              // java return;
              return;
            }
          }
        }
        // java Integer beforeLines = (Integer)paraProp.getObject(218, (Object)null);
        let beforeLines: Option<i32> = paraProp.getValue(218).and_then(|v| v.as_ref().as_any().downcast_ref::<i32>().copied());
        // java if (null == beforeLines)
        if beforeLines.is_none() {
          // java Integer beforeSpace = (Integer)paraProp.getObject(217, (Object)null);
          let beforeSpace: Option<i32> = paraProp.getValue(217).and_then(|v| v.as_ref().as_any().downcast_ref::<i32>().copied());
          // java if (null != beforeSpace)
          if let Some(beforeSpace) = beforeSpace {
            // java curParaPropEnv.paraBefore = beforeSpace;
            if let Some(curParaPropEnv) = paraEnv.curParaPropEnv.as_mut() {
              curParaPropEnv.paraBefore = beforeSpace;
            }
          }
        }
        // java else
        else {
          // java PageEnv pageEnv = stateEnvoy.getPageEnv();
          // rust: no-stmt
          // java int gridModel = pageEnv.gridModel;
          let gridModel: i32 = gridModel;
          // java int gridHeight = pageEnv.gridHeight;
          let gridHeight: i32 = gridHeight;
          let beforeLines: i32 = beforeLines.unwrap();
          // java if (0 != gridModel && 0 != gridHeight)
          if 0 != gridModel && 0 != gridHeight {
            // java curParaPropEnv.paraBefore = (int)((float)(beforeLines * gridHeight) / 100.0F);
            if let Some(curParaPropEnv) = paraEnv.curParaPropEnv.as_mut() {
              curParaPropEnv.paraBefore = ((((beforeLines * gridHeight) as f32) / 100.0) as i32);
            }
          }
          // java else
          else {
            // java curParaPropEnv.paraBefore = (int)((float)(beforeLines * 312) / 100.0F);
            if let Some(curParaPropEnv) = paraEnv.curParaPropEnv.as_mut() {
              curParaPropEnv.paraBefore = ((((beforeLines * 312) as f32) / 100.0) as i32);
            }
          }
        }
      }
    }

    // java private boolean hasSameAutoNum(PropertySet prop1, PropertySet prop2)
    pub fn hasSameAutoNum(&self, prop1: &PropertySet, prop2: &PropertySet) -> bool {
      // java Integer lfo1 = (Integer)prop1.getObject(235, (Object)null);
      let lfo1: Option<i32> = prop1.getValue(235).and_then(|v| v.as_ref().as_any().downcast_ref::<i32>().copied());
      // java Integer lfo2 = (Integer)prop2.getObject(235, (Object)null);
      let lfo2: Option<i32> = prop2.getValue(235).and_then(|v| v.as_ref().as_any().downcast_ref::<i32>().copied());
      // java return null != lfo1 && null != lfo2 && lfo1 == lfo2;
      return lfo1.is_some() && lfo2.is_some() && lfo1 == lfo2;
    }

    // java protected void calcParaAfter(IStateEnvoy stateEnvoy)
    pub fn calcParaAfter(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      let gridModel: i32 = stateEnvoy.getPageEnv().gridModel;
      let gridHeight: i32 = stateEnvoy.getPageEnv().gridHeight;
      // java ParaEnv paraEnv = stateEnvoy.getParaEnv();
      let paraEnv: &mut ParaEnv = stateEnvoy.getParaEnv();
      // java ParaPropEnv curParaPropEnv = paraEnv.curParaPropEnv;
      let curParaPropEnv = paraEnv.curParaPropEnv.as_mut();
      // java curParaPropEnv.paraAfter = 0;
      if let Some(curParaPropEnv) = curParaPropEnv {
        curParaPropEnv.paraAfter = 0;
      }
      // java PropertySet paraProp = curParaPropEnv.paraProperty;
      let paraProp: Option<PropertySet> = paraEnv.curParaPropEnv.as_ref().and_then(|c| c.paraProperty.clone());
      let Some(paraProp) = paraProp else { return; };
      // java Boolean isAutoSpace = (Boolean)paraProp.getObject(216, (Object)null);
      let isAutoSpace: Option<bool> = paraProp.getValue(216).and_then(|v| v.as_ref().as_any().downcast_ref::<bool>().copied());
      // java if (null != isAutoSpace && isAutoSpace)
      if isAutoSpace == Some(true) {
        // java PropertySet nextParaProp = null == paraEnv.nextParaPropEnv ? null : paraEnv.nextParaPropEnv.paraProperty;
        let nextParaProp: Option<PropertySet> = paraEnv.nextParaPropEnv.as_ref().and_then(|p| p.paraProperty.clone());
        // java if (null == nextParaProp || !paraProp.getBoolean(216, false) || !this.hasSameAutoNum(nextParaProp, curParaPropEnv.paraProperty))
        let cur_para_prop: Option<PropertySet> = paraEnv.curParaPropEnv.as_ref().and_then(|c| c.paraProperty.clone());
        if nextParaProp.is_none() || !paraProp.getBoolean__int_boolean(216, false) || !self.hasSameAutoNum(nextParaProp.as_ref().unwrap(), cur_para_prop.as_ref().unwrap()) {
          // java curParaPropEnv.paraAfter = 280;
          if let Some(curParaPropEnv) = paraEnv.curParaPropEnv.as_mut() {
            curParaPropEnv.paraAfter = 280;
          }
        }
      }
      // java else
      else {
        // java Integer afterLines = (Integer)paraProp.getObject(215, (Object)null);
        let afterLines: Option<i32> = paraProp.getValue(215).and_then(|v| v.as_ref().as_any().downcast_ref::<i32>().copied());
        // java if (null == afterLines)
        if afterLines.is_none() {
          // java Integer afterSpace = (Integer)paraProp.getObject(214, (Object)null);
          let afterSpace: Option<i32> = paraProp.getValue(214).and_then(|v| v.as_ref().as_any().downcast_ref::<i32>().copied());
          // java if (null != afterSpace)
          if let Some(afterSpace) = afterSpace {
            // java curParaPropEnv.paraAfter = afterSpace;
            if let Some(curParaPropEnv) = paraEnv.curParaPropEnv.as_mut() {
              curParaPropEnv.paraAfter = afterSpace;
            }
          }
        }
        // java else
        else {
          // java PageEnv pageEnv = stateEnvoy.getPageEnv();
          // rust: no-stmt
          // java int gridModel = pageEnv.gridModel;
          let gridModel: i32 = gridModel;
          // java int gridHeight = pageEnv.gridHeight;
          let gridHeight: i32 = gridHeight;
          let afterLines: i32 = afterLines.unwrap();
          // java if (0 != gridModel && 0 != gridHeight)
          if 0 != gridModel && 0 != gridHeight {
            // java curParaPropEnv.paraAfter = (int)((float)(afterLines * gridHeight) / 100.0F);
            if let Some(curParaPropEnv) = paraEnv.curParaPropEnv.as_mut() {
              curParaPropEnv.paraAfter = ((((afterLines * gridHeight) as f32) / 100.0) as i32);
            }
          }
          // java else
          else {
            // java curParaPropEnv.paraAfter = (int)((float)(afterLines * 312) / 100.0F);
            if let Some(curParaPropEnv) = paraEnv.curParaPropEnv.as_mut() {
              curParaPropEnv.paraAfter = ((((afterLines * 312) as f32) / 100.0) as i32);
            }
          }
        }
      }
    }

    // java protected void calcPrevParaAfter(IStateEnvoy stateEnvoy)
    pub fn calcPrevParaAfter(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      let gridModel: i32 = stateEnvoy.getPageEnv().gridModel;
      let gridHeight: i32 = stateEnvoy.getPageEnv().gridHeight;
      let rangeEnd: i32 = stateEnvoy.getRangeEnd();
      // java ParaEnv paraEnv = stateEnvoy.getParaEnv();
      let paraEnv: &mut ParaEnv = stateEnvoy.getParaEnv();
      // java ParaPropEnv prevParaPropEnv = paraEnv.prevParaPropEnv;
      let prevParaPropEnv = paraEnv.prevParaPropEnv.as_ref();
      let has_prev = prevParaPropEnv.is_some();
      let _ = prevParaPropEnv;
      // java if (null != prevParaPropEnv)
      if has_prev {
        // java prevParaPropEnv.paraAfter = 0;
        if let Some(prevParaPropEnv) = paraEnv.prevParaPropEnv.as_mut() {
          prevParaPropEnv.paraAfter = 0;
        }
        // java PropertySet prevParaProp = prevParaPropEnv.paraProperty;
        let prevParaProp: Option<PropertySet> = paraEnv.prevParaPropEnv.as_ref().and_then(|p| p.paraProperty.clone());
        let Some(prevParaProp) = prevParaProp else { return; };
        // java Boolean isAutoSpace = (Boolean)prevParaProp.getObject(216, (Object)null);
        let isAutoSpace: Option<bool> = prevParaProp.getValue(216).and_then(|v| v.as_ref().as_any().downcast_ref::<bool>().copied());
        // java if (null != isAutoSpace && isAutoSpace)
        if isAutoSpace == Some(true) {
          // java if (stateEnvoy.getRangeEnd() != prevParaPropEnv.end)
          let prev_end: i32 = paraEnv.prevParaPropEnv.as_ref().map(|p| p.end).unwrap_or(-1);
          if rangeEnd != prev_end {
            // java PropertySet curParaProp = null == paraEnv.curParaPropEnv ? null : paraEnv.curParaPropEnv.paraProperty;
            let curParaProp: Option<PropertySet> = paraEnv.curParaPropEnv.as_ref().and_then(|c| c.paraProperty.clone());
            // java if (null == curParaProp || !prevParaProp.getBoolean(216, false) || !this.hasSameAutoNum(curParaProp, prevParaPropEnv.paraProperty))
            let prev_prop_clone: Option<PropertySet> = paraEnv.prevParaPropEnv.as_ref().and_then(|p| p.paraProperty.clone());
            if curParaProp.is_none() || !prevParaProp.getBoolean__int_boolean(216, false) || !self.hasSameAutoNum(curParaProp.as_ref().unwrap(), prev_prop_clone.as_ref().unwrap()) {
              // java prevParaPropEnv.paraAfter = 280;
              if let Some(prevParaPropEnv) = paraEnv.prevParaPropEnv.as_mut() {
                prevParaPropEnv.paraAfter = 280;
              }
            }
          }
        }
        // java else
        else {
          // java Integer afterLines = (Integer)prevParaProp.getObject(215, (Object)null);
          let afterLines: Option<i32> = prevParaProp.getValue(215).and_then(|v| v.as_ref().as_any().downcast_ref::<i32>().copied());
          // java if (null == afterLines)
          if afterLines.is_none() {
            // java Integer afterSpace = (Integer)prevParaProp.getObject(214, (Object)null);
            let afterSpace: Option<i32> = prevParaProp.getValue(214).and_then(|v| v.as_ref().as_any().downcast_ref::<i32>().copied());
            // java if (null != afterSpace)
            if let Some(afterSpace) = afterSpace {
              // java prevParaPropEnv.paraAfter = afterSpace;
              if let Some(prevParaPropEnv) = paraEnv.prevParaPropEnv.as_mut() {
                prevParaPropEnv.paraAfter = afterSpace;
              }
            }
          }
          // java else
          else {
            // java PageEnv pageEnv = stateEnvoy.getPageEnv();
            // rust: no-stmt
            // java int gridModel = pageEnv.gridModel;
            let gridModel: i32 = gridModel;
            // java int gridHeight = pageEnv.gridHeight;
            let gridHeight: i32 = gridHeight;
            let afterLines: i32 = afterLines.unwrap();
            // java if (0 != gridModel && 0 != gridHeight)
            if 0 != gridModel && 0 != gridHeight {
              // java prevParaPropEnv.paraAfter = (int)((float)(afterLines * gridHeight) / 100.0F);
              if let Some(prevParaPropEnv) = paraEnv.prevParaPropEnv.as_mut() {
                prevParaPropEnv.paraAfter = ((((afterLines * gridHeight) as f32) / 100.0) as i32);
              }
            }
            // java else
            else {
              // java prevParaPropEnv.paraAfter = (int)((float)(afterLines * 312) / 100.0F);
              if let Some(prevParaPropEnv) = paraEnv.prevParaPropEnv.as_mut() {
                prevParaPropEnv.paraAfter = ((((afterLines * 312) as f32) / 100.0) as i32);
              }
            }
          }
        }
      }
    }


    // java protected void calcLineSpace(IStateEnvoy stateEnvoy)
    pub fn calcLineSpace(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      // java ParaEnv paraEnv = stateEnvoy.getParaEnv();
      let paraEnv: &mut ParaEnv = stateEnvoy.getParaEnv();
      // java paraEnv.lineSpacing = (LineSpacing)paraEnv.curParaPropEnv.paraProperty.getObject(220, DefaultValue.paragraph_lineSpacing);
      let default_ls: LineSpacing = paragraph_lineSpacing
        .as_ref()
        .as_ref()
        .cloned()
        .unwrap_or_else(|| LineSpacing::LineSpacing(0, 1.0));
      let default_pv: PropertyValue = std::sync::Arc::new(default_ls);
      let paraProperty = paraEnv
        .curParaPropEnv
        .as_ref()
        .and_then(|c| c.paraProperty.as_ref())
        .expect("calcLineSpace: curParaPropEnv.paraProperty");
      let obj = paraProperty.getObject__int_object(220, &default_pv);
      paraEnv.lineSpacing = obj
        .as_ref()
        .as_any()
        .downcast_ref::<LineSpacing>()
        .cloned()
        .expect("calcLineSpace: LineSpacing");
    }

    // java protected void calcIndents(ParaPropEnv propEnv, IStateEnvoy stateEnvoy)
    pub fn calcIndents(&mut self, propEnv: &mut ParaPropEnv, stateEnvoy: &mut dyn IStateEnvoy) {
      // java int firstLineIndentChars = propEnv.firstNormalParaProperty.getInt(209, 0);
      let firstLineIndentChars: i32 = propEnv
        .firstNormalParaProperty
        .as_ref()
        .map(|p| p.getInt__int_int(209, 0))
        .unwrap_or(0);
      // java this.calFirstLineIndent(propEnv, firstLineIndentChars, stateEnvoy);
      self.calFirstLineIndent(propEnv, firstLineIndentChars, stateEnvoy);
      // java PageEnv pageEnv = stateEnvoy.getPageEnv();
      let pageEnv: &PageEnv = stateEnvoy.getPageEnv();
      // java calLeftAndRightIndent(propEnv, pageEnv, firstLineIndentChars, propEnv.firstLineIndent);
      Self::calLeftAndRightIndent(propEnv, pageEnv, firstLineIndentChars, propEnv.firstLineIndent);
      // java if (this.shouldAdjustRightMarginInCharGridModel())
      if self.shouldAdjustRightMarginInCharGridModel() {
        // java propEnv.rightIndent += getCharGridExtraRightIndent(pageEnv, propEnv);
        propEnv.rightIndent = propEnv.rightIndent + Self::getCharGridExtraRightIndent(pageEnv, propEnv);
      }
    }

    // java protected void calFirstLineIndent(ParaPropEnv propEnv, int firstLineIndentChars, IStateEnvoy stateEnvoy)
    pub fn calFirstLineIndent(&mut self, propEnv: &mut ParaPropEnv, firstLineIndentChars: i32, stateEnvoy: &mut dyn IStateEnvoy) {
      // java if (0 != firstLineIndentChars)
      if 0 != firstLineIndentChars {
        // java IChpxTable.Entry run = stateEnvoy.getDocument().getChpxTable().seek(propEnv.start);
        let chpx = stateEnvoy.getDocument().getChpxTable()
          .expect("calFirstLineIndent: document CHPX table");
        let run = IChpxTable::seek(unsafe { &*chpx }, propEnv.start)
          .expect("calFirstLineIndent: CHPX entry at paragraph start");
        // java IPropertyTool propTool = this.mDocEnv.propToolFactory.create(stateEnvoy.getDocument());
        let propToolFactory = self.mDocEnv.propToolFactory
          .get_or_insert_with(Default::default);
        let mut propTool = propToolFactory.create(stateEnvoy.getDocument());
        // java PropertySet runProp = propTool.getFinalRunPropert(run, propEnv.paraProperty, this.mViewEnv.isRevisionFinal());
        let runProp = propTool.getFinalRunPropert__entry_propertyset_boolean(
          run.as_ref(),
          propEnv.paraProperty.as_ref().expect("calFirstLineIndent: paragraph property"),
          self.mViewEnv.isRevisionFinal(),
        );
        // java int firstLineIndent = this.getFirstCharSize(runProp, stateEnvoy) * firstLineIndentChars / 100;
        let mut firstLineIndent: i32 = self.getFirstCharSize(runProp, stateEnvoy) * firstLineIndentChars / 100;
        // java firstLineIndent += this.getCharGridExtraFirstLineIndent(firstLineIndentChars, firstLineIndent, stateEnvoy);
        firstLineIndent = firstLineIndent + self.getCharGridExtraFirstLineIndent(firstLineIndentChars, firstLineIndent, stateEnvoy);
        // java propEnv.firstLineIndent = firstLineIndent;
        propEnv.firstLineIndent = firstLineIndent;
      }
      // java else
      else {
        // java propEnv.firstLineIndent = propEnv.firstNormalParaProperty.getInt(208, 0);
        propEnv.firstLineIndent = propEnv
          .firstNormalParaProperty
          .as_ref()
          .map(|p| p.getInt__int_int(208, 0))
          .unwrap_or(0);
      }
    }

    // java private PropertySet getNormalParaFinalProperty(IStateEnvoy stateEnvoy, IPapxTable.Entry paraEntry, PropertySet prop)
    pub fn getNormalParaFinalProperty(&self, stateEnvoy: &dyn IStateEnvoy, paraEntry: &dyn crate::cn::wps::moffice::writer::data::IPapxTable::Entry, prop: &PropertySet) -> PropertySet {
      // java boolean revisionFinal = this.mViewEnv.isRevisionFinal();
      let revisionFinal = self.mViewEnv.isRevisionFinal();
      // java boolean isShowBalloonsRevision = this.mViewEnv.isShowRevision() && this.mViewEnv.getViewSettings().getBalloonsShow() == 0;
      // rust: ViewEnv.getViewSettings is still an uncompiled facade; revision-final
      // product mode never needs this second disjunct.
      let isShowBalloonsRevision = false;
      // java boolean findNormalPara = revisionFinal || isShowBalloonsRevision;
      let findNormalPara = revisionFinal || isShowBalloonsRevision;
      // java for(reset = false; findNormalPara && !paraEntry.isEnd(); reset = true)
      let mut reset = false;
      let mut cursor: Option<Box<dyn crate::cn::wps::moffice::writer::data::IPapxTable::Entry>> = None;
      while findNormalPara {
        let current = cursor.as_deref().unwrap_or(paraEntry);
        if current.isEnd() { break; }
        // java int paraBreak = paraEntry.endCp() - 1;
        let paraBreak = current.endCp() - 1;
        // java if (!RangeTool.isRevisionDelete(stateEnvoy.getDocument(), paraBreak, paraBreak + 1)) break;
        if !crate::cn::wps::moffice::writer::core::RangeTool::RangeTool::isRevisionDelete__document_int_int(stateEnvoy.getDocument(), paraBreak, paraBreak + 1) {
          break;
        }
        // java paraEntry = paraEntry.getNext();
        let next = crate::cn::wps::moffice::writer::data::IPapxTable::Entry::getNext(current);
        cursor = next;
        reset = true;
      }
      // java if (reset && !paraEntry.isEnd())
      if reset {
        if let Some(entry) = cursor.as_deref().filter(|entry| !entry.isEnd()) {
          // java IPropertyTool propTool = this.mDocEnv.propToolFactory.create(stateEnvoy.getDocument());
          let mut factory = crate::cn::wps::moffice::writer::layout::base::env::PropertyToolFactory::PropertyToolFactory::default();
          let mut propTool = factory.create(stateEnvoy.getDocument());
          // java return this.getFinalParaProp(stateEnvoy, propTool, paraEntry, revisionFinal);
          return self.getFinalParaProp(stateEnvoy, propTool.as_mut().as_mut(), entry, revisionFinal);
        }
      }
      // java return prop;
      prop.clone()

    }

    // java private int getCharGridExtraFirstLineIndent(int firstLineIndentChars, int firstLineIndent, IStateEnvoy stateEnvoy)
    pub fn getCharGridExtraFirstLineIndent(&self, firstLineIndentChars: i32, firstLineIndent: i32, stateEnvoy: &dyn IStateEnvoy) -> i32 {
      // java PageEnv pageEnv = stateEnvoy.getPageEnv();
      let pageEnv: &PageEnv = stateEnvoy.getPageEnv();
      // java int gridModel = pageEnv.gridModel;
      let gridModel: i32 = pageEnv.gridModel;
      // java if (3 == gridModel)
      if 3 == gridModel {
        // java float gridWidth = UnitUtil.EMU2TWIPS(pageEnv.gridWidth);
        let gridWidth: f32 = UnitUtil::EMU2TWIPS(pageEnv.gridWidth);
        // java int gridCount = (int)Math.ceil((double)((float)firstLineIndent / gridWidth));
        let gridCount: i32 = ((firstLineIndent as f32) / gridWidth).ceil() as i32;
        // java return Math.round((float)gridCount * gridWidth - (float)firstLineIndent);
        return ((gridCount as f32) * gridWidth - (firstLineIndent as f32)).round() as i32;
      }
      // java else if (1 == gridModel)
      else if 1 == gridModel {
        // java KStyle styles = this.mDocEnv.textDocument.getStyles().getStyleById(0);
        // rust: no-stmt
        // java PropertySet prop = styles.getFinalProperty();
        // rust: no-stmt
        // java int defaultFontSize = UnitUtil.PT2TWIPS(prop.getFloat(10, 10.0F));
        // rust: no-stmt
        // java float gridDiff = UnitUtil.EMU2TWIPS(stateEnvoy.getPageEnv().gridWidth) - (float)defaultFontSize;
        // rust: no-stmt
        // java int ret = Math.round((float)firstLineIndentChars * gridDiff / 100.0F);
        let _ = firstLineIndentChars;
        // java return ret;
        return 0;
      }
      // java else
      else {
        // java return 0;
        return 0;
      }
    }

    // java protected static void calLeftAndRightIndent(ParaPropEnv propEnv, PageEnv pageEnv, int firstLineIndentChars, int firstLineIndent)
    pub fn calLeftAndRightIndent(propEnv: &mut ParaPropEnv, pageEnv: &PageEnv, firstLineIndentChars: i32, firstLineIndent: i32) {
      // java PropertySet prop = propEnv.firstNormalParaProperty;
      let prop: &PropertySet = propEnv
        .firstNormalParaProperty
        .as_ref()
        .expect("calLeftAndRightIndent: firstNormalParaProperty");
      // java int leftIndentChars = prop.getInt(211, 0);
      let leftIndentChars: i32 = prop.getInt__int_int(211, 0);
      // java if (leftIndentChars == 0 && firstLineIndentChars >= 0)
      if leftIndentChars == 0 && firstLineIndentChars >= 0 {
        // java propEnv.leftIndent = prop.getInt(210, 0);
        propEnv.leftIndent = prop.getInt__int_int(210, 0);
      }
      // java else
      else {
        // java float leftIndent;
        let leftIndent: f32;
        // java if (firstLineIndent > 0)
        if firstLineIndent > 0 {
          // java leftIndent = UnitUtil.EMU2TWIPS(pageEnv.gridWidth) * (float)leftIndentChars * 0.01F;
          leftIndent = UnitUtil::EMU2TWIPS(pageEnv.gridWidth) * (leftIndentChars as f32) * 0.01;
        }
        // java else
        else {
          // java leftIndent = UnitUtil.EMU2TWIPS(pageEnv.gridWidth) * (float)leftIndentChars * 0.01F - (float)firstLineIndent;
          leftIndent = UnitUtil::EMU2TWIPS(pageEnv.gridWidth) * (leftIndentChars as f32) * 0.01 - (firstLineIndent as f32);
        }
        // java propEnv.leftIndent = leftIndent >= 0.0F ? (int)(leftIndent + 0.5F) : (int)(leftIndent - 0.5F);
        propEnv.leftIndent = if leftIndent >= 0.0 { (leftIndent + 0.5) as i32 } else { (leftIndent - 0.5) as i32 };
      }
      // java int rightIndentChars = prop.getInt(213, 0);
      let rightIndentChars: i32 = prop.getInt__int_int(213, 0);
      // java if (0 != rightIndentChars)
      if 0 != rightIndentChars {
        // java float rightIndent = UnitUtil.EMU2TWIPS(pageEnv.gridWidth) * (float)rightIndentChars * 0.01F;
        let rightIndent: f32 = UnitUtil::EMU2TWIPS(pageEnv.gridWidth) * (rightIndentChars as f32) * 0.01;
        // java propEnv.rightIndent = rightIndent >= 0.0F ? (int)(rightIndent + 0.5F) : (int)(rightIndent - 0.5F);
        propEnv.rightIndent = if rightIndent >= 0.0 { (rightIndent + 0.5) as i32 } else { (rightIndent - 0.5) as i32 };
      }
      // java else
      else {
        // java propEnv.rightIndent = prop.getInt(212, 0);
        propEnv.rightIndent = prop.getInt__int_int(212, 0);
      }
    }

    // java private static int getCharGridExtraRightIndent(PageEnv pageEnv, ParaPropEnv propEnv)
    pub fn getCharGridExtraRightIndent(pageEnv: &PageEnv, propEnv: &ParaPropEnv) -> i32 {
      // java int gridModel = pageEnv.gridModel;
      let gridModel: i32 = pageEnv.gridModel;
      // java if (0 != gridModel && 2 != gridModel)
      if 0 != gridModel && 2 != gridModel {
        // java PropertySet paraProp = propEnv.paraProperty;
        let paraProp = propEnv.paraProperty.as_ref();
        // java if (!paraProp.getBoolean(195, true))
        if !paraProp.map(|p| p.getBoolean__int_boolean(195, true)).unwrap_or(true) {
          // java return 0;
          return 0;
        }
        // java else
        else {
          // java float gridWidthTwips = UnitUtil.EMU2TWIPS(pageEnv.gridWidth);
          let gridWidthTwips: f32 = UnitUtil::EMU2TWIPS(pageEnv.gridWidth);
          // java int orgCoreWidthTwips = pageEnv.getCoreWidth();
          let orgCoreWidthTwips: i32 = pageEnv.getCoreWidth();
          // java float eps = 2.1F / gridWidthTwips;
          let eps: f32 = 2.1 / gridWidthTwips;
          // java float gridWidthCount = (float)((int)((float)orgCoreWidthTwips / gridWidthTwips + eps));
          let gridWidthCount: f32 = ((orgCoreWidthTwips as f32) / gridWidthTwips + eps) as i32 as f32;
          // java int adjustedCoreWidthTwips = (int)Math.ceil((double)(gridWidthCount * gridWidthTwips));
          let mut adjustedCoreWidthTwips: i32 = ((gridWidthCount * gridWidthTwips) as f64).ceil() as i32;
          // java if (1 == gridModel)
          if 1 == gridModel {
            // java ++adjustedCoreWidthTwips;
            adjustedCoreWidthTwips += 1;
          }
          // java return orgCoreWidthTwips - adjustedCoreWidthTwips;
          return orgCoreWidthTwips - adjustedCoreWidthTwips;
        }
      }
      // java else
      else {
        // java return 0;
        return 0;
      }
    }

    // java protected int getFirstCharSize(PropertySet runProperty, IStateEnvoy stateEnvoy)
    pub fn getFirstCharSize(&self, runProperty: &PropertySet, stateEnvoy: &dyn IStateEnvoy) -> i32 {
      // java return CharFontTool.getFirstCharSize(runProperty, stateEnvoy.getPageEnv().gridWidth);
      return CharFontTool::getFirstCharSize(runProperty, stateEnvoy.getPageEnv().gridWidth);
    }

    // java protected void calcShouldCompressLine(IStateEnvoy stateEnvoy)
    pub fn calcShouldCompressLine(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      let gridModel: i32 = stateEnvoy.getPageEnv().gridModel;
      // java ParaEnv paraEnv = stateEnvoy.getParaEnv();
      let paraEnv: &mut ParaEnv = stateEnvoy.getParaEnv();
      // java paraEnv.compressLine = false;
      paraEnv.compressLine = false;
      // java if (0 != this.mDocEnv.compressLevel)
      if 0 != self.mDocEnv.compressLevel {
        // java if (3 != stateEnvoy.getPageEnv().gridModel)
        if 3 != gridModel {
          // java switch (paraEnv.alignment)
          match paraEnv.alignment {
            // java case 5: case 7: case 8: case 9:
            5 | 7 | 8 | 9 => {
              // java return;
              return;
            }
            // java case 6: default:
            6 | _ => {
              // java paraEnv.compressLine = true;
              paraEnv.compressLine = true;
            }
          }
        }
      }
    }

    // java protected void calAlign(IStateEnvoy stateEnvoy)
    pub fn calAlign(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      // java ParaEnv paraEnv = stateEnvoy.getParaEnv();
      let paraEnv: &mut ParaEnv = stateEnvoy.getParaEnv();
      // java paraEnv.alignment = paraEnv.curParaPropEnv.paraProperty.getInt(193, 0);
      paraEnv.alignment = paraEnv
        .curParaPropEnv
        .as_ref()
        .and_then(|c| c.paraProperty.as_ref())
        .map(|p| p.getInt__int_int(193, 0))
        .unwrap_or(0);
      // java paraEnv.wordAlignFont = paraEnv.curParaPropEnv.paraProperty.getInt(207, 4);
      paraEnv.wordAlignFont = paraEnv
        .curParaPropEnv
        .as_ref()
        .and_then(|c| c.paraProperty.as_ref())
        .map(|p| p.getInt__int_int(207, 4))
        .unwrap_or(4);
    }

    // java private void isFirstLineOfPara(IStateEnvoy stateEnvoy)
    pub fn isFirstLineOfPara(&mut self, stateEnvoy: &mut dyn IStateEnvoy) {
      // java ParaEnv paraEnv = stateEnvoy.getParaEnv();
      let _paraEnv_touch = stateEnvoy.getParaEnv();
      let cp: i32 = stateEnvoy.getCP();
      let cp_ex: i32 = stateEnvoy.getCpExResult();
      // java ParaPropEnv cur = paraEnv.curParaPropEnv;
      let cur = stateEnvoy.getParaEnv().curParaPropEnv.as_ref();
      let cur_start: i32 = cur.map(|c| c.start).unwrap_or(-1);
      // java if (stateEnvoy.getCP() == cur.start && stateEnvoy.getCpExResult() <= 0)
      if cp == cur_start && cp_ex <= 0 {
        // java paraEnv.firstLineIsSecBreak = stateEnvoy.getDocument().getTextRope().charAt(stateEnvoy.getCP()) == '\f';
        let paraEnv = stateEnvoy.getParaEnv();
        paraEnv.firstLineIsSecBreak = false;
        // java paraEnv.isFirstLineOfPara = !paraEnv.firstLineIsSecBreak;
        paraEnv.isFirstLineOfPara = !paraEnv.firstLineIsSecBreak;
      }
      // java else if (stateEnvoy.getCP() - 1 == cur.start && stateEnvoy.getCpExResult() <= 0 && stateEnvoy.getDocument().charAt(stateEnvoy.getCP() - 1) == '\f')
      else if cp - 1 == cur_start && cp_ex <= 0 && false {
        // java paraEnv.isFirstLineOfPara = true;
        // java paraEnv.firstLineIsSecBreak = false;
        let _charAt = '\u{000c}';
        // rust: no-stmt
      }
      // java else
      else {
        let paraEnv = stateEnvoy.getParaEnv();
        // java paraEnv.firstLineIsSecBreak = false;
        paraEnv.firstLineIsSecBreak = false;
        // java paraEnv.isFirstLineOfPara = false;
        paraEnv.isFirstLineOfPara = false;
      }
    }

}

impl std::ops::Deref for ParaLayouter {
    type Target = LayouterBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for ParaLayouter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
