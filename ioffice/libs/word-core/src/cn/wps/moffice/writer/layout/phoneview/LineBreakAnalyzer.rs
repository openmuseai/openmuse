// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::ITextRope;
// Java: import cn.wps.moffice.util.UnitUtil
use util::moffice::util::UnitUtil::UnitUtil;
// Java: import cn.wps.moffice.writer.cache.TypoDocument
use crate::cn::wps::moffice::writer::cache::TypoDocument::TypoDocument;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
// Java: import cn.wps.moffice.writer.layout.base.ILayouterWhileLoading
use crate::cn::wps::moffice::writer::layout::base::ILayouterWhileLoading::ILayouterWhileLoading;
// Java: import cn.wps.moffice.writer.layout.base.ParaLayouter
use crate::cn::wps::moffice::writer::layout::base::ParaLayouter::ParaLayouter;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.PageEnv
use crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.PhoneViewFitInfo
use crate::cn::wps::moffice::writer::layout::base::env::PhoneViewFitInfo::PhoneViewFitInfo;
// Java: import cn.wps.moffice.writer.layout.base.env.SignableRange
use crate::cn::wps::moffice::writer::layout::base::env::SignableRange::SignableRange;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphBuff
use crate::cn::wps::moffice::writer::layout::base::graph::GraphBuff::GraphBuff;
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphBuffFactory
use crate::cn::wps::moffice::writer::layout::base::graph::GraphBuffFactory::GraphBuffFactory;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.FilterForSpaces
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::FilterForSpaces::FilterForSpaces;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.PredictSpacesInfo
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::PredictSpacesInfo::PredictSpacesInfo;
// Java: import cn.wps.moffice.writer.layout.pageview.PageViewLayoutStateEnvoy
use crate::cn::wps::moffice::writer::layout::pageview::PageViewLayoutStateEnvoy::PageViewLayoutStateEnvoy;
// Java: import cn.wps.moffice.writer.layout.phoneview.FastGraphForAnalyzer
use crate::cn::wps::moffice::writer::layout::phoneview::FastGraphForAnalyzer::FastGraphForAnalyzer;
// Java: import cn.wps.moffice.writer.layout.phoneview.PhoneViewParaLayouter
use crate::cn::wps::moffice::writer::layout::phoneview::PhoneViewParaLayouter::PhoneViewParaLayouter;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::Entry;
// Java: import cn.wps.moffice.writer.layout.phoneview.PhoneViewLayoutState
use crate::cn::wps::moffice::writer::layout::phoneview::PhoneViewLayoutState::PhoneViewLayoutState;

// java public class LineBreakAnalyzer implements PredictSpacesInfo.IContextHelper
#[derive(Default)]
pub struct LineBreakAnalyzer {
    // java private MainState mMainState = new MainState();
    mMainState: Option<MainStateHolder>,
    // java private DocumentEnv mDocEnv;
    mDocEnv: Option<DocumentEnvHolder>,
    // java private ViewEnv mViewEnv;
    mViewEnv: Option<ViewEnvHolder>,
    // java private LayoutState mLayoutState;
    mLayoutState: Option<LayoutStateHolder>,
    // java private ParaLayouter mParaLayouter;
    mParaLayouter: Option<ParaLayouterHolder>,
    // java private GraphBuff mCacheGraphBuff;
    mCacheGraphBuff: Option<GraphBuffHolder>,
    // java private FastGraphForAnalyzer mFastGraph;
    mFastGraph: Option<FastGraphForAnalyzerHolder>,
    // java private PredictSpacesInfo mPredictSpacesInfo;
    mPredictSpacesInfo: Option<PredictSpacesInfoHolder>,
    // java private FilterForSpaces mFilterForSpace;
    mFilterForSpace: Option<FilterForSpacesHolder>,
    // java private ILayouterWhileLoading mLoading;
    mLoading: Option<ILayouterWhileLoadingHolder>,
    // java private char[] mCharBuff = new char[64];
    mCharBuff: Vec<char>,
}

// java public class LineBreakAnalyzer implements PredictSpacesInfo.IContextHelper
impl LineBreakAnalyzer {
    // java public void setLayouterWhileLoading(ILayouterWhileLoading loading)
    pub fn setLayouterWhileLoading(&mut self, loading: Option<ILayouterWhileLoadingHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mLoading = loading;
      self.mLoading = loading;
      */
    }

    // java public  LineBreakAnalyzer(DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn LineBreakAnalyzer(docEnv: &DocumentEnv, viewEnv: &ViewEnv) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mDocEnv = docEnv;
      let mut this = Self { mMainState: Default::default(), mDocEnv: docEnv, mViewEnv: viewEnv, mLayoutState: self.createLayoutState(), mParaLayouter: PhoneViewParaLayouter::new(self.mDocEnv, self.mViewEnv, self.mMainState), mCacheGraphBuff: Default::default(), mFastGraph: Default::default(), mPredictSpacesInfo: PredictSpacesInfo::new(self.mViewEnv, self.mLayoutState, self), mFilterForSpace: Default::default(), mLoading: Default::default(), mCharBuff: Default::default() };
      // java this.mViewEnv = viewEnv;
      // java this.mLayoutState = this.createLayoutState();
      // java this.mParaLayouter = new PhoneViewParaLayouter(this.mDocEnv, this.mViewEnv, this.mMainState);
      // java this.mPredictSpacesInfo = new PredictSpacesInfo(this.mViewEnv, this.mLayoutState, this);
      // java this.mFilterForSpace = new FilterForSpaces(this.mLayoutState, this.mDocEnv);
      this.mFilterForSpace = FilterForSpaces::new(this.mLayoutState, this.mDocEnv);
      this
      */
    }

    // java public PhoneViewFitInfo getFitInfo()
    pub fn getFitInfo(&self) -> Option<PhoneViewFitInfoHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mDocEnv.getPhoneViewFitInfo();
      return Some(self.mDocEnv.getPhoneViewFitInfo());
      */
    }

    // java public LineBreakAnalyzer getLineBreakAnalyzer()
    pub fn getLineBreakAnalyzer(&self) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mDocEnv.mLineBreakAnalyzer == null)
      if self.mDocEnv.mLineBreakAnalyzer.is_none() {
        // java this.mDocEnv.mLineBreakAnalyzer = new LineBreakAnalyzer(this.mDocEnv, this.mViewEnv);
        self.mDocEnv.mLineBreakAnalyzer = LineBreakAnalyzer::new(self.mDocEnv, self.mViewEnv);
      }
      // java return this.mDocEnv.mLineBreakAnalyzer;
      return self.mDocEnv.mLineBreakAnalyzer;
      */
    }

    // java public boolean isInAnalyzer()
    pub fn isInAnalyzer(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return true;
      return true;
      */
    }

    // java public boolean analyzeOption(LayoutState hostState, int layoutWidth)
    pub fn analyzeOption(&self, hostState: Option<LayoutStateHolder>, layoutWidth: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (hostState.document.getType() != this.mLayoutState.document.getType())
      if hostState.document.getType() != self.mLayoutState.document.getType() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java this.initEnv();
        self.initEnv();
        // java boolean var8;
        let mut var8: bool;
            // java Document doc = this.mLayoutState.document;
            let mut doc: Box<dyn Document> = self.mLayoutState.document;
            // java int analyStart = hostState.paraEnv.curParaPropEnv.end;
            let mut analyStart: i32 = hostState.paraEnv.curParaPropEnv.end;
            // java int maxCp = this.waitToCp(analyStart + 256);
            let mut maxCp: i32 = self.waitToCp(analyStart + 256);
            // java this.mLayoutState.setRange(analyStart, maxCp);
            self.mLayoutState.setRange__int_int(analyStart, maxCp);
            // java IPapxTable papxTable = doc.getPapxTable();
            let mut papxTable: Box<dyn IPapxTable> = doc.getPapxTable();
            // java IPapxTable.Entry entry = papxTable.seek(analyStart);
            let mut entry: Box<dyn Entry> = papxTable.seek(analyStart);
            // java if (entry.isEnd() || entry.position() >= maxCp)
            if entry.isEnd() || entry.position() >= maxCp {
              // java return false;
              return false;
            }
            // java var8 = this.doAnalyze(entry.position(), entry.endCp(), layoutWidth);
            var8 = self.doAnalyze(entry.position(), entry.endCp(), layoutWidth);
              // java this.closeEnv();
              self.closeEnv();
        // java return var8;
        return var8;
      }
      */
    }

    // java  int waitToCp(int cp)
    pub fn waitToCp(&self, cp: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mLoading != null ? this.mLoading.waitToCp(cp) : this.mLayoutState.document.getLength();
      return (if self.mLoading.is_some() { self.mLoading.waitToCp(cp) } else { self.mLayoutState.document.getLength() });
      */
    }

    // java public boolean analyzeSignable(LayoutState hostState, int layoutWidth)
    pub fn analyzeSignable(&self, hostState: Option<LayoutStateHolder>, layoutWidth: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (hostState.document.getType() != this.mLayoutState.document.getType())
      if hostState.document.getType() != self.mLayoutState.document.getType() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java this.initEnv();
        self.initEnv();
        // java SignableRange range = (SignableRange)this.mDocEnv.getPhoneViewFitInfo().getItemByStartCp(hostState.cp, SignableRange.class);
        let mut range: SignableRange = (self.mDocEnv.getPhoneViewFitInfo().getItemByStartCp(hostState.cp, todo!())).downcast_ref::<SignableRange>().unwrap();
        // java if (range == null)
        if false {
          // java return false;
          return false;
        }
        // java else
        else {
          // java boolean meetOnce = false;
          let mut meetOnce: bool = false;
              // java Document doc = this.mLayoutState.document;
              let mut doc: Box<dyn Document> = self.mLayoutState.document;
              // java IPapxTable papxTable = doc.getPapxTable();
              let mut papxTable: Box<dyn IPapxTable> = doc.getPapxTable();
              // java if (range.hasMeetContractTitle())
              if range.hasMeetContractTitle() {
                // java int maxCp = this.waitToCp(range.mEndCp + 512);
                let mut maxCp: i32 = self.waitToCp(range.mEndCp + 512);
                // java this.mLayoutState.setRange(range.mEndCp, maxCp);
                self.mLayoutState.setRange__int_int(range.mEndCp, maxCp);
                let mut entry: Box<dyn Entry> = papxTable.seek(range.mEndCp);
                // java for(IPapxTable.Entry entry = papxTable.seek(range.mEndCp); entry != null && !entry.isEnd() && entry.position() < maxCp entry = entry.getNext())
                while true && !entry.isEnd() && entry.position() < maxCp {
                  // java int starCp = entry.position();
                  let mut starCp: i32 = entry.position();
                  // java int endCp = entry.endCp();
                  let mut endCp: i32 = entry.endCp();
                  // java if (endCp - starCp < 8)
                  if endCp - starCp < 8 {
                    // java if (this.hasNormalCP(starCp, endCp))
                    if self.hasNormalCP(starCp, endCp) {
                      // java break;
                      break;
                    }
                    // java range.insertBlank(starCp, endCp);
                    range.insertBlank(starCp, endCp);
                  }
                  // java if (!this.hasNormalCP(starCp, endCp))
                  if !self.hasNormalCP(starCp, endCp) {
                    // java range.insertBlank(starCp, endCp);
                    range.insertBlank(starCp, endCp);
                  }
                  // java else
                  else {
                    // java if (!this.doAnalyzeSignable(entry.position(), entry.endCp(), layoutWidth))
                    if !self.doAnalyzeSignable(entry.position(), entry.endCp(), layoutWidth) {
                      // java break;
                      break;
                    }
                    // java meetOnce = true;
                    meetOnce = true;
                  }
                }
              }
              // java else if (range.mStartCp > 0)
              else if range.mStartCp > 0 {
                // java int beforeEnd = range.mStartCp;
                let mut beforeEnd: i32 = range.mStartCp;
                // java int afterStart = range.mEndCp;
                let mut afterStart: i32 = range.mEndCp;
                // java IPapxTable.Entry entry = papxTable.seek(beforeEnd);
                let mut entry: Box<dyn Entry> = papxTable.seek(beforeEnd);
                let mut var19: Box<dyn Entry> = entry.getPrev();
                // java for(IPapxTable.Entry var19 = entry.getPrev(); var19 != null && !var19.isEnd() var19 = var19.getPrev())
                while true && !var19.isEnd() {
                  // java int starCp = var19.position();
                  let mut starCp: i32 = var19.position();
                  // java int endCp = var19.endCp();
                  let mut endCp: i32 = var19.endCp();
                  // java if (endCp - starCp < 8)
                  if endCp - starCp < 8 {
                    // java if (this.hasNormalCP(starCp, endCp))
                    if self.hasNormalCP(starCp, endCp) {
                      // java break;
                      break;
                    }
                    // java range.insertBlank(starCp, endCp);
                    range.insertBlank(starCp, endCp);
                  }
                  // java else
                  else {
                    // java if (!this.doAnalyzeSignable(var19.position(), var19.endCp(), layoutWidth))
                    if !self.doAnalyzeSignable(var19.position(), var19.endCp(), layoutWidth) {
                      // java break;
                      break;
                    }
                    // java meetOnce = true;
                    meetOnce = true;
                  }
                }
                // java int maxCp = this.waitToCp(afterStart + 512);
                let mut maxCp: i32 = self.waitToCp(afterStart + 512);
                let mut var20: Box<dyn Entry> = papxTable.seek(afterStart);
                // java for(IPapxTable.Entry var20 = papxTable.seek(afterStart); !var20.isEnd() && var20.position() < maxCp var20 = var20.getNext())
                while !var20.isEnd() && var20.position() < maxCp {
                  // java int starCp = var20.position();
                  let mut starCp: i32 = var20.position();
                  // java int endCp = var20.endCp();
                  let mut endCp: i32 = var20.endCp();
                  // java if (endCp - starCp < 8)
                  if endCp - starCp < 8 {
                    // java if (this.hasNormalCP(starCp, endCp))
                    if self.hasNormalCP(starCp, endCp) {
                      // java break;
                      break;
                    }
                    // java range.insertBlank(starCp, endCp);
                    range.insertBlank(starCp, endCp);
                  }
                  // java else
                  else {
                    // java if (!this.doAnalyzeSignable(var20.position(), var20.endCp(), layoutWidth))
                    if !self.doAnalyzeSignable(var20.position(), var20.endCp(), layoutWidth) {
                      // java break;
                      break;
                    }
                    // java meetOnce = true;
                    meetOnce = true;
                  }
                }
              }
                // java this.closeEnv();
                self.closeEnv();
          // java range.trim();
          range.trim();
          // java return meetOnce;
          return meetOnce;
        }
      }
      */
    }

    // java private boolean hasNormalCP(int curCP, int maxCP)
    pub fn hasNormalCP(&self, curCP: i32, maxCP: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java char[] buf = this.mCharBuff;
      let mut buf: Vec<char> = self.mCharBuff;
      // java int readThisTime;
      let mut readThisTime: i32;
      let mut textRope: Box<dyn ITextRope> = self.mLayoutState.document.getTextRope();
      // java for(ITextRope textRope = this.mLayoutState.document.getTextRope(); curCP < maxCP curCP += readThisTime)
      while curCP < maxCP {
        // java readThisTime = Math.min(buf.length, maxCP - curCP);
        readThisTime = ((buf.len() as i32)).min(maxCP - curCP);
        // java textRope.getText(curCP, curCP + readThisTime, buf, 0);
        textRope.getText(curCP, curCP + readThisTime, buf, 0);
        let mut i: i32 = 0;
        // java for(int i = 0; i < readThisTime; ++i)
        while i < readThisTime {
          // java char c = buf[i];
          let mut c: char = buf[(i) as usize];
          // java if (!this.isCanSkipChar(c))
          if !self.isCanSkipChar(c) {
            // java return true;
            return true;
          }
        }
      }
      // java return false;
      return false;
      */
    }

    // java private boolean isCanSkipChar(char c)
    pub fn isCanSkipChar(&self, c: char) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return c == ' ' || c == 12288 || c == 11 || c == '\r' || c == '\t' || c == 160;
      return c == ' ' || c == 12288 || c == 11 || c == '\r' || c == '\t' || c == 160;
      */
    }

    // java  boolean doAnalyzeSignable(int starCp, int endCp, int layoutWidth)
    pub fn doAnalyzeSignable(&self, starCp: i32, endCp: i32, layoutWidth: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mLayoutState.setRange(starCp, endCp);
      self.mLayoutState.setRange__int_int(starCp, endCp);
      // java this.mLayoutState.cp = starCp;
      self.mLayoutState.cp = starCp;
      // java this.mParaLayouter.start(this.mLayoutState.getEnvoy());
      self.mParaLayouter.start(self.mLayoutState.getEnvoy());
      // java this.initGraphBuff(this.mLayoutState);
      self.initGraphBuff(self.mLayoutState);
      // java this.mFastGraph.prepareGraphs(this.mLayoutState, this.mCacheGraphBuff);
      self.mFastGraph.prepareGraphs(self.mLayoutState, self.mCacheGraphBuff);
      // java if (!this.mLayoutState.hasEmbed() && !this.mLayoutState.hasMaths() && !this.mLayoutState.hasEquation() && !this.mLayoutState.hasDrawing() && this.mLayoutState.revisions.isEmpty())
      if !self.mLayoutState.hasEmbed() && !self.mLayoutState.hasMaths() && !self.mLayoutState.hasEquation() && !self.mLayoutState.hasDrawing() && self.mLayoutState.revisions.is_empty() {
        // java this.mFilterForSpace.clear();
        self.mFilterForSpace.clear();
        // java this.mPredictSpacesInfo.clear();
        self.mPredictSpacesInfo.clear();
        // java this.mPredictSpacesInfo.predictSpacePattern(layoutWidth, this.mFilterForSpace);
        self.mPredictSpacesInfo.predictSpacePattern(layoutWidth, self.mFilterForSpace);
        // java this.mParaLayouter.end(this.mLayoutState.getEnvoy());
        self.mParaLayouter.end(self.mLayoutState.getEnvoy());
        // java this.mCacheGraphBuff.clear(this.mLayoutState);
        self.mCacheGraphBuff.clear(self.mLayoutState);
        // java return this.mPredictSpacesInfo.mBreakType.isSignable();
        return self.mPredictSpacesInfo.mBreakType.isSignable();
      }
      // java else
      else {
        // java this.mParaLayouter.end(this.mLayoutState.getEnvoy());
        self.mParaLayouter.end(self.mLayoutState.getEnvoy());
        // java this.mCacheGraphBuff.clear(this.mLayoutState);
        self.mCacheGraphBuff.clear(self.mLayoutState);
        // java return false;
        return false;
      }
      */
    }

    // java  boolean doAnalyze(int starCp, int endCp, int layoutWidth)
    pub fn doAnalyze(&self, starCp: i32, endCp: i32, layoutWidth: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mLayoutState.cp = starCp;
      self.mLayoutState.cp = starCp;
      // java this.mParaLayouter.start(this.mLayoutState.getEnvoy());
      self.mParaLayouter.start(self.mLayoutState.getEnvoy());
      // java this.initGraphBuff(this.mLayoutState);
      self.initGraphBuff(self.mLayoutState);
      // java this.mFastGraph.prepareGraphs(this.mLayoutState, this.mCacheGraphBuff);
      self.mFastGraph.prepareGraphs(self.mLayoutState, self.mCacheGraphBuff);
      // java this.mFilterForSpace.clear();
      self.mFilterForSpace.clear();
      // java this.mPredictSpacesInfo.clear();
      self.mPredictSpacesInfo.clear();
      // java this.mPredictSpacesInfo.predictSpacePattern(layoutWidth, this.mFilterForSpace);
      self.mPredictSpacesInfo.predictSpacePattern(layoutWidth, self.mFilterForSpace);
      // java this.mParaLayouter.end(this.mLayoutState.getEnvoy());
      self.mParaLayouter.end(self.mLayoutState.getEnvoy());
      // java this.mCacheGraphBuff.clear(this.mLayoutState);
      self.mCacheGraphBuff.clear(self.mLayoutState);
      // java return true;
      return true;
      */
    }

    // java protected void initGraphBuff(LayoutState layoutState)
    pub fn initGraphBuff(&mut self, layoutState: Option<LayoutStateHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mCacheGraphBuff == null)
      if self.mCacheGraphBuff.is_none() {
        // java this.mCacheGraphBuff = GraphBuffFactory.createGraphBuff(this.mMainState, this.mDocEnv, this.mViewEnv);
        self.mCacheGraphBuff = GraphBuffFactory::createGraphBuff(self.mMainState, self.mDocEnv, self.mViewEnv);
      }
      // java layoutState.initCoreContent(layoutState.getRangeEnd(), this.mViewEnv.isRevisionFinal());
      layoutState.initCoreContent(layoutState.getRangeEnd(), self.mViewEnv.isRevisionFinal());
      // java this.mCacheGraphBuff.initFilter(layoutState.document, layoutState.cp, layoutState.getRangeEnd(), layoutState);
      self.mCacheGraphBuff.initFilter(layoutState.document, layoutState.cp, layoutState.getRangeEnd(), layoutState);
      */
    }

    // java private LayoutState createLayoutState()
    pub fn createLayoutState(&self) -> Option<LayoutStateHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java PhoneViewLayoutState newEnv = new PhoneViewLayoutState((PageViewLayoutStateEnvoy)null);
      let mut newEnv: PhoneViewLayoutState = PhoneViewLayoutState::new(None);
      // java PageEnv pageEnv = newEnv.pageEnv;
      let mut pageEnv: PageEnv = newEnv.pageEnv;
      // java newEnv.document = this.mDocEnv.textDocument.getMainDocument();
      newEnv.document = self.mDocEnv.textDocument.getMainDocument();
      // java pageEnv.gridModel = 0;
      pageEnv.gridModel = 0;
      // java pageEnv.gridHeight = 312;
      pageEnv.gridHeight = 312;
      // java pageEnv.gridWidth = UnitUtil.PT2EMU(10.5F);
      pageEnv.gridWidth = UnitUtil::PT2EMU(10.5);
      // java newEnv.setAllowTransForSpace();
      newEnv.setAllowTransForSpace();
      // java return newEnv;
      return { let __x: Option<LayoutStateHolder> = todo!("no align strategy matched"); __x };
      */
    }

    // java private void initEnv()
    pub fn initEnv(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mLayoutState.document = this.mDocEnv.textDocument.getMainDocument();
      self.mLayoutState.document = self.mDocEnv.textDocument.getMainDocument();
      // java if (this.mMainState.snapshot == null)
      if self.mMainState.snapshot.is_none() {
        // java this.mMainState.snapshot = this.cleanForkSnapshot(this.mDocEnv.typoDocument);
        self.mMainState.snapshot = self.cleanForkSnapshot(Some(self.mDocEnv.typoDocument));
      }
      // java this.mMainState.snapshot.switchUsingThread();
      self.mMainState.snapshot.switchUsingThread();
      // java this.mMainState.layoutPage.alloc(this.mMainState.snapshot);
      self.mMainState.layoutPage.alloc(self.mMainState.snapshot);
      // java if (this.mLayoutState.curPage == null)
      if self.mLayoutState.curPage.is_none() {
        // java this.mLayoutState.curPage = this.mMainState.layoutPage;
        self.mLayoutState.curPage = self.mMainState.layoutPage;
      }
      // java PageEnv pageEnv = this.mLayoutState.pageEnv;
      let mut pageEnv: PageEnv = self.mLayoutState.pageEnv;
      // java pageEnv.init();
      pageEnv.init();
      // java pageEnv.width = 11906;
      pageEnv.width = 11906;
      // java pageEnv.height = 16838;
      pageEnv.height = 16838;
      // java pageEnv.maxLayoutHeight = 16838;
      pageEnv.maxLayoutHeight = 16838;
      // java if (this.mFastGraph == null)
      if self.mFastGraph.is_none() {
        // java this.mFastGraph = new FastGraphForAnalyzer(this.mMainState, this.mDocEnv, this.mViewEnv);
        self.mFastGraph = FastGraphForAnalyzer::new(self.mMainState, self.mDocEnv, self.mViewEnv);
      }
      */
    }

    // java private void closeEnv()
    pub fn closeEnv(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMainState.layoutPage.clear();
      self.mMainState.layoutPage.clear();
      // java this.mLayoutState.curPage = null;
      self.mLayoutState.curPage = None;
      // java this.mMainState.snapshot.reset();
      self.mMainState.snapshot.reset();
      */
    }

    // java private TypoSnapshot cleanForkSnapshot(TypoDocument typoDocument)
    pub fn cleanForkSnapshot(&self, typoDocument: Option<TypoDocumentHolder>) -> Option<TypoSnapshotHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot cloneSnapshot = typoDocument.getSnapshot();
      let mut cloneSnapshot: TypoSnapshot = typoDocument.getSnapshot();
      // java TypoSnapshot cleanForkSnapshot = cloneSnapshot.cleanFork();
      let mut cleanForkSnapshot: TypoSnapshot = cloneSnapshot.cleanFork();
      // java cloneSnapshot.release();
      cloneSnapshot.release();
      // java return cleanForkSnapshot;
      return { let __x: Option<TypoSnapshotHolder> = todo!("no align strategy matched"); __x };
      */
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mCacheGraphBuff != null)
      if let Some(mCacheGraphBuff) = self.mCacheGraphBuff.as_mut() {
        // java this.mCacheGraphBuff.dispose();
        mCacheGraphBuff.dispose();
        // java this.mCacheGraphBuff = null;
        mCacheGraphBuff = None;
      }
      // java if (this.mFastGraph != null)
      if let Some(mFastGraph) = self.mFastGraph.as_mut() {
        // java this.mFastGraph.dispose();
        mFastGraph.dispose();
        // java this.mFastGraph = null;
        mFastGraph = None;
      }
      // java if (this.mParaLayouter != null)
      if let Some(mParaLayouter) = self.mParaLayouter.as_mut() {
        // java this.mParaLayouter.dispose();
        mParaLayouter.dispose();
        // java this.mParaLayouter = null;
        mParaLayouter = None;
      }
      // java if (this.mLayoutState != null)
      if let Some(mLayoutState) = self.mLayoutState.as_mut() {
        // java this.mLayoutState.dispose();
        mLayoutState.dispose();
        // java this.mLayoutState = null;
        mLayoutState = None;
      }
      // java this.mFilterForSpace = null;
      self.mFilterForSpace = None;
      // java if (this.mMainState != null)
      if let Some(mMainState) = self.mMainState.as_mut() {
        // java if (this.mMainState.snapshot != null)
        if mMainState.snapshot.is_some() {
          // java this.mMainState.snapshot.release();
          mMainState.snapshot.release();
        }
        // java this.mMainState = null;
        mMainState = None;
      }
      */
    }

}
