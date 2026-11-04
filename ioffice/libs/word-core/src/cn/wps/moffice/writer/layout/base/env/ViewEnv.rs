#![allow(dead_code, unused_variables, unused_imports, non_snake_case, unused_mut, unreachable_code, unused_assignments)]
// Java: import cn.wps.graphics.Rect
// Java: import cn.wps.moffice.property.PropertySet
// Java: import cn.wps.moffice.writer.base.ViewMode
use crate::cn::wps::moffice::writer::base::ViewMode::*;
// Java: import cn.wps.moffice.writer.core.IDebugger
// Java: import cn.wps.moffice.writer.core.IInterruptState
// Java: import cn.wps.moffice.writer.core.env.IRevisionStringMap
// Java: import cn.wps.moffice.writer.core.env.RevisionStringMap
// Java: import cn.wps.moffice.writer.core.pool.Reusable
// Java: import cn.wps.moffice.writer.data.value.BRC
// Java: import cn.wps.moffice.writer.layout.base.IPageChangeListener
// Java: import cn.wps.moffice.writer.layout.base.IViewListener
// Java: import cn.wps.moffice.writer.layout.base.balloons.IBalloonCacheListener
// Java: import cn.wps.moffice.writer.service.IViewSettings
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.layout.base.env.PhoneViewParam
// Java: import java.lang.Object
use core::any::Any;

// java private static final String TAG = null;
const TAG: i32 = 0;
// java public static final int DIRECTION_NONE = 0;
pub(crate) const DIRECTION_NONE: i32 = 0;
// java public static final int DIRECTION_DOWN = 1;
pub(crate) const DIRECTION_DOWN: i32 = 1;
// java public static final int DIRECTION_UP = 2;
pub(crate) const DIRECTION_UP: i32 = 2;
// java public static final int MAX_FONT_SIZE_OF_PHONE_VIEW = 520;
pub(crate) const MAX_FONT_SIZE_OF_PHONE_VIEW: i32 = 520;
// java static String commentInfoFormat = "%s[%s%d]:";
pub(crate) static commentInfoFormat: i32 = 0;
// java static String revisionFormat = "[%s]%s";
pub(crate) static revisionFormat: i32 = 0;
// java static String revisionFormatWithContent = "[%s]%s:";
pub(crate) static revisionFormatWithContent: i32 = 0;
// java static String substituteForDrawingFormat = "<%s>";
pub(crate) static substituteForDrawingFormat: i32 = 0;

#[derive(Clone)]
struct DefaultEqxBrc(crate::cn::wps::moffice::writer::data::value::BRC::BRC);

impl Default for DefaultEqxBrc {
    fn default() -> Self {
        // java new BRC(0.5F, 1)
        Self(crate::cn::wps::moffice::writer::data::value::BRC::BRC::new__float_int(0.5, 1))
    }
}

// java public class ViewEnv implements IPageChangeListener, Reusable
#[derive(Default)]
pub struct ViewEnv {
    // java int mViewMode = 0;
    pub(crate) mViewMode: i32,
    // java int mViewHeight = 0;
    pub(crate) mViewHeight: i32,
    // java int mViewWidth = 0;
    pub(crate) mViewWidth: i32,
    // java int mViewTop = 0;
    pub(crate) mViewTop: i32,
    // java int mScrollDirection = 0;
    pub(crate) mScrollDirection: i32,
    // java int mWebViewMarginTop = 0;
    pub(crate) mWebViewMarginTop: i32,
    // java int mWebViewMarginLeft = 0;
    pub(crate) mWebViewMarginLeft: i32,
    // java int mWebViewMarginRight = 0;
    pub(crate) mWebViewMarginRight: i32,
    // java int mWebViewMarginBottom = 0;
    pub(crate) mWebViewMarginBottom: i32,
    // java float mPhoneViewLineSpacingLines = 1.0F;
    pub(crate) mPhoneViewLineSpacingLines: i32,
    // java int mPhoneViewIndent = 1;
    pub(crate) mPhoneViewIndent: i32,
    // java int mBalloonsWidth = 0;
    pub(crate) mBalloonsWidth: i32,
    // java int mBalloonsMarginLeft = 0;
    pub(crate) mBalloonsMarginLeft: i32,
    // java int mBalloonsMarginRight = 0;
    pub(crate) mBalloonsMarginRight: i32,
    // java PropertySet mRevisionInsertProperty;
    pub(crate) mRevisionInsertProperty: Option<util::moffice::property::PropertySet::PropertySet>,
    // java PropertySet mRevisionDeleteProperty;
    pub(crate) mRevisionDeleteProperty: Option<util::moffice::property::PropertySet::PropertySet>,
    // java boolean mRevisionFinal = true;
    pub(crate) mRevisionFinal: i32,
    // java boolean mShowRevisionInballoons = true;
    pub(crate) mShowRevisionInballoons: i32,
    // java boolean mSeparateAudioCommnet = false;
    pub(crate) mSeparateAudioCommnet: i32,
    // java boolean mShowRevisionMakeup = false;
    pub(crate) mShowRevisionMakeup: i32,
    // java boolean mShowComment = false;
    pub(crate) mShowComment: i32,
    // java boolean mShowRevision = false;
    pub(crate) mShowRevision: i32,
    // java boolean mShowRevisionChangeInballoons = true;
    pub(crate) mShowRevisionChangeInballoons: i32,
    // java boolean mShowRevisionInsOrDelInBalloons = true;
    pub(crate) mShowRevisionInsOrDelInBalloons: RevisionBalloonFlag,
    // java IRevisionStringMap mRevisionMap = null;
    pub(crate) mRevisionMap: i32,
    // java int mMaxLineOfComment = 4;
    pub(crate) mMaxLineOfComment: i32,
    // java int mSpaceBetweenBalloon = 80;
    pub(crate) mSpaceBetweenBalloon: i32,
    // java float mBalloonsZoom = 1.0F;
    pub(crate) mBalloonsZoom: i32,
    // java int mTabWidth = 400;
    pub(crate) mTabWidth: i32,
    // java PropertySet mShareViewProperty = null;
    pub(crate) mShareViewProperty: Option<util::moffice::property::PropertySet::PropertySet>,
    // java float mContentLengthToBallonsLengthRatio = 1.0F;
    pub(crate) mContentLengthToBallonsLengthRatio: i32,
    // java int mPageBreakWidth = 0;
    pub(crate) mPageBreakWidth: i32,
    // java int mDefSectionNfcPgn = 0;
    pub(crate) mDefSectionNfcPgn: i32,
    // java boolean mShowBalloons = false;
    pub(crate) mShowBalloons: i32,
    // java boolean mDrawSpace = false;
    pub(crate) mDrawSpace: i32,
    // java boolean mDrawParaBreak = false;
    pub(crate) mDrawParaBreak: i32,
    // java boolean mDrawNewLine = false;
    pub(crate) mDrawNewLine: i32,
    // java boolean mEnableFixSectionBreak = false;
    pub(crate) mEnableFixSectionBreak: i32,
    // java boolean mHasFlowToBegin = false;
    pub(crate) mHasFlowToBegin: i32,
    // java boolean mHasFlowToEnd = false;
    pub(crate) mHasFlowToEnd: i32,
    // java boolean mUlTrailSpace = true;
    pub(crate) mUlTrailSpace: i32,
    // java int mFootEndNoteSepLineWidth = 0;
    pub(crate) mFootEndNoteSepLineWidth: i32,
    // java int mFootEndNoteTagWidth = 0;
    pub(crate) mFootEndNoteTagWidth: i32,
    // java int mFootEndNoteTagHeight = 0;
    pub(crate) mFootEndNoteTagHeight: i32,
    // java int mDocmentLID = -1;
    pub(crate) mDocmentLID: i32,
    // java int mPhoneModeAdjustLine = 0;
    pub(crate) mPhoneModeAdjustLine: i32,
    // java float mPhoneModeLineSpaceLines = 0.0F;
    pub(crate) mPhoneModeLineSpaceLines: i32,
    // java float mPhoneModeSpaceBeforeLines = 0.0F;
    pub(crate) mPhoneModeSpaceBeforeLines: i32,
    // java PhoneViewParam mPhoneViewParam = new PhoneViewParam();
    pub(crate) mPhoneViewParam: i32,
    // java PhoneViewParam.mFitTableForView defaults false
    pub(crate) fit_table_for_view: bool,
    // java boolean mSmartFirstLineIndent = false;
    pub(crate) mSmartFirstLineIndent: i32,
    // java boolean mAutoHyphenation = true;
    pub(crate) mAutoHyphenation: i32,
    // java boolean mShowCardInfo = false;
    pub(crate) mShowCardInfo: i32,
    // java float mShareViewFontsize = 10.0F;
    pub(crate) mShareViewFontsize: f32,
    // java boolean mForceUseShareViewRunProperty = false;
    pub(crate) mForceUseShareViewRunProperty: i32,
    // java IViewListener mViewListener = null;
    pub(crate) mViewListener: i32,
    // java IBalloonCacheListener mBalloonCacheListener = null;
    pub(crate) mBalloonCacheListener: i32,
    // java int mMaxInkCommentHeightWithFold = 0;
    pub(crate) mMaxInkCommentHeightWithFold: i32,
    // java final ArrayList<IPageChangeListener> mPageChangeListeners = new ArrayList();
    pub(crate) mPageChangeListeners: i32,
    // java int mAudioCommentHeight = 0;
    pub(crate) mAudioCommentHeight: i32,
    // java int mEmptyCommentHeight = 0;
    pub(crate) mEmptyCommentHeight: i32,
    // java BRC mDefaultEQXBRC = new BRC(0.5F, 1);
    pub(crate) mDefaultEQXBRC: DefaultEqxBrc,
    // java IViewSettings mViewSettings;
    pub(crate) mViewSettings: i32,
    // java ViewSettings.mCommentPicDisplaySize = MetricsUtil.pixel2twips(80). 0 means the Java initializer.
    pub(crate) commentPicDisplaySize: f32,
    // java IDebugger mDebugger;
    pub(crate) mDebugger: i32,
    // java IInterruptState mInterruptState;
    pub(crate) mInterruptState: i32,
    pub(crate) pageSetup: Option<crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup>,
    pub(crate) zoom_x100: i32,
    pub(crate) needRelayout: bool,
}

trait IViewListener {}
trait IInterruptState {}
trait IRevisionStringMap {}
trait IBalloonCacheListener {}
trait IPageChangeListener {}
trait IViewSettings {}
trait IDebugger {}
#[derive(Default, Clone)]
struct PhoneViewParam;
#[derive(Clone, Copy)]
pub(crate) struct RevisionBalloonFlag(pub i32);

impl Default for RevisionBalloonFlag {
    fn default() -> Self {
        // java boolean mShowRevisionInsOrDelInBalloons = true;
        Self(1)
    }
}

#[derive(Default, Clone)]
struct PropertySet;

// java public class ViewEnv implements IPageChangeListener, Reusable
impl ViewEnv {
    // java public boolean reuseClean()
    pub fn reuseClean(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mViewMode = 0;
      self.mViewMode = 0;
      // java this.mViewHeight = 0;
      self.mViewHeight = 0;
      // java this.mViewWidth = 0;
      self.mViewWidth = 0;
      // java this.mViewTop = 0;
      self.mViewTop = 0;
      // java this.mScrollDirection = 0;
      self.mScrollDirection = 0;
      // java this.mWebViewMarginTop = 0;
      self.mWebViewMarginTop = 0;
      // java this.mWebViewMarginLeft = 0;
      self.mWebViewMarginLeft = 0;
      // java this.mWebViewMarginRight = 0;
      self.mWebViewMarginRight = 0;
      // java this.mWebViewMarginBottom = 0;
      self.mWebViewMarginBottom = 0;
      // java this.mPhoneViewLineSpacingLines = 1.0F;
      self.mPhoneViewLineSpacingLines = 1;
      // java this.mPhoneViewIndent = 1;
      self.mPhoneViewIndent = 1;
      // java this.mBalloonsWidth = 0;
      self.mBalloonsWidth = 0;
      // java this.mBalloonsMarginLeft = 0;
      self.mBalloonsMarginLeft = 0;
      // java this.mBalloonsMarginRight = 0;
      self.mBalloonsMarginRight = 0;
      // java this.mRevisionFinal = true;
      self.mRevisionFinal = true;
      // java this.mShowRevisionInballoons = true;
      self.mShowRevisionInballoons = true;
      // java this.mSeparateAudioCommnet = false;
      self.mSeparateAudioCommnet = false;
      // java this.mRevisionMap = null;
      self.mRevisionMap = None;
      // java this.mMaxLineOfComment = 4;
      self.mMaxLineOfComment = 4;
      // java this.mSpaceBetweenBalloon = 80;
      self.mSpaceBetweenBalloon = 80;
      // java this.mBalloonsZoom = 1.0F;
      self.mBalloonsZoom = 1;
      // java this.mTabWidth = 400;
      self.mTabWidth = 400;
      // java this.mShareViewProperty = null;
      self.mShareViewProperty = None;
      // java this.mContentLengthToBallonsLengthRatio = 1.0F;
      self.mContentLengthToBallonsLengthRatio = 1;
      // java this.mPageBreakWidth = 0;
      self.mPageBreakWidth = 0;
      // java this.mDefSectionNfcPgn = 0;
      self.mDefSectionNfcPgn = 0;
      // java this.mShowBalloons = false;
      self.mShowBalloons = false;
      // java this.mDrawSpace = false;
      self.mDrawSpace = false;
      // java this.mDrawParaBreak = false;
      self.mDrawParaBreak = false;
      // java this.mDrawNewLine = false;
      self.mDrawNewLine = false;
      // java this.mEnableFixSectionBreak = false;
      self.mEnableFixSectionBreak = false;
      // java this.mHasFlowToBegin = false;
      self.mHasFlowToBegin = false;
      // java this.mHasFlowToEnd = false;
      self.mHasFlowToEnd = false;
      // java this.mUlTrailSpace = true;
      self.mUlTrailSpace = true;
      // java this.mFootEndNoteSepLineWidth = 0;
      self.mFootEndNoteSepLineWidth = 0;
      // java this.mFootEndNoteTagWidth = 0;
      self.mFootEndNoteTagWidth = 0;
      // java this.mFootEndNoteTagHeight = 0;
      self.mFootEndNoteTagHeight = 0;
      // java this.mDocmentLID = -1;
      self.mDocmentLID = -1;
      // java this.mPhoneModeAdjustLine = 0;
      self.mPhoneModeAdjustLine = 0;
      // java this.mPhoneModeLineSpaceLines = 0.0F;
      self.mPhoneModeLineSpaceLines = 0;
      // java this.mPhoneModeSpaceBeforeLines = 0.0F;
      self.mPhoneModeSpaceBeforeLines = 0;
      // java this.mShareViewFontsize = 10.0F;
      self.mShareViewFontsize = 10;
      // java this.mViewListener = null;
      self.mViewListener = None;
      // java this.mBalloonCacheListener = null;
      self.mBalloonCacheListener = None;
      // java this.mMaxInkCommentHeightWithFold = 0;
      self.mMaxInkCommentHeightWithFold = 0;
      // java this.mPageChangeListeners.clear();
      self.mPageChangeListeners.clear();
      // java this.mViewSettings = null;
      self.mViewSettings = None;
      // java this.mDebugger = null;
      self.mDebugger = None;
      // java this.mInterruptState = null;
      self.mInterruptState = None;
      // java return true;
      return true;
      **/
    }

    // java public void reuseInit()
    pub fn reuseInit(&mut self) {
    todo!("DEFERRED_NON_FUNCTION");
    /* DEFERRED_NON_FUNCTION

    **/
    }

    // java public  ViewEnv(IViewListener listener)
    pub fn ViewEnv() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.setViewListener(listener);
      this.setViewListener(listener);
      **/
    }

    // java public void setViewListener(IViewListener listener)
    pub fn setViewListener(&mut self, listener: Option<&dyn IViewListener>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mViewListener = listener;
      self.mViewListener = listener;
      // java if (null != listener)
      if let Some(listener) = listener {
        // java this.mPageChangeListeners.add(listener);
        self.mPageChangeListeners.add(listener);
      }
      **/
    }

    // java public void setViewSettings(IViewSettings viewSettings)
    pub fn setViewSettings(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mViewSettings = viewSettings;
      self.mViewSettings = viewSettings;
      **/
    }

    // java public void setDebugger(IDebugger debugger)
    pub fn setDebugger(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mDebugger = debugger;
      self.mDebugger = debugger;
      **/
    }

    // java public boolean isDebugging()
    pub fn isDebugging(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mDebugger != null;
      return true;
      **/
    }

    // java public void setInterrupt(IInterruptState interrupt)
    pub fn setInterrupt(&mut self, interrupt: &dyn IInterruptState) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mInterruptState = interrupt;
      self.mInterruptState = interrupt;
      **/
    }

    // java public IInterruptState getInterrupt()
    pub fn getInterrupt(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mInterruptState;
      return self.mInterruptState;
      **/
    }

    // java public void onDebug(String id, Object... params)
    pub fn onDebug(&mut self, id: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mDebugger != null)
      if true {
        // java this.mDebugger.onDebug(id, params);
        self.mDebugger.onDebug(id, params);
      }
      **/
    }

    // java public IViewSettings getViewSettings()
    pub fn getViewSettings(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mViewSettings;
      return self.mViewSettings;
      **/
    }

    // java public int getViewMode()
    // java IViewSettings.getCommentPicDisplaySize()
    pub fn getCommentPicDisplaySize(&self) -> f32 {
      if self.commentPicDisplaySize > 0.0 {
        self.commentPicDisplaySize
      } else {
        util::moffice::util::MetricsUtil::MetricsUtil::pixel2twips(80.0)
      }
    }

    pub fn getViewMode(&self) -> i32 {
      // java return this.mViewMode;
      return self.mViewMode;
    }

    // java public void setViewMode(int viewMode)
    pub fn setViewMode(&mut self, viewMode: i32) {
      // java this.mViewMode = viewMode;
      self.mViewMode = viewMode;
    }

    // java public boolean isPhoneReadMode()
    pub fn isPhoneReadMode(&self) -> bool {
      // java return ViewMode.isPhoneReadMode(this.mViewMode);
      return ViewMode::isPhoneReadMode(self.mViewMode);
    }

    // java public boolean isWebLayoutMode()
    pub fn isWebLayoutMode(&self) -> bool {
      // java return ViewMode.isWebLayoutMode(this.mViewMode);
      return ViewMode::isWebLayoutMode(self.mViewMode);
    }

    // java public boolean isPageMode()
    pub fn isPageMode(&self) -> bool {
      // java return ViewMode.isPageMode(this.mViewMode);
      return ViewMode::isPageMode(self.mViewMode);
    }

    // java public int getViewHeight()
    pub fn getViewHeight(&self) -> i32 {
      // java return this.mViewHeight;
      return self.mViewHeight;
    }

    // java public void setViewHeight(int viewHeight)
    pub fn setViewHeight(&mut self, viewHeight: i32) {
      // java this.mViewHeight = viewHeight;
      self.mViewHeight = viewHeight;
    }

    // java public int getViewWidth()
    pub fn getViewWidth(&self) -> i32 {
      // java return this.mViewWidth;
      return self.mViewWidth;
    }

    // java public void setViewWidth(int viewWidth)
    pub fn setViewWidth(&mut self, viewWidth: i32) {
      // java this.mViewWidth = viewWidth;
      self.mViewWidth = viewWidth;
    }

    pub fn new() -> Self {
      let mut e = Self::default();
      e.zoom_x100 = 100;
      e.mViewMode = 0;
      e
    }

    pub fn setPageSetup(&mut self, pageSetup: crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup) {
      self.pageSetup = Some(pageSetup);
    }

    pub fn setZoom(&mut self, zoom: i32) {
      self.zoom_x100 = if zoom <= 0 { 100 } else { zoom * 100 };
    }

    pub fn contentWidth(&self) -> i32 {
      let w = self.pageSetup.as_ref().map(|p| p.contentWidth()).unwrap_or(self.mViewWidth);
      w * 100 / self.zoom_x100.max(1)
    }

    pub fn contentHeight(&self) -> i32 {
      let h = self.pageSetup.as_ref().map(|p| p.contentHeight()).unwrap_or(self.mViewHeight);
      h * 100 / self.zoom_x100.max(1)
    }

    pub fn changeSize(&mut self, _needClearFontStyleCache: bool) {
      self.needRelayout = true;
    }

    pub fn takeNeedRelayout(&mut self) -> bool {
      let v = self.needRelayout;
      self.needRelayout = false;
      v
    }


    // java public void setPhoneViewIndent(int ratio)
    pub fn setPhoneViewIndent(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mPhoneViewIndent = ratio;
      self.mPhoneViewIndent = ratio;
      **/
    }

    // java public int getPhoneViewIndent()
    pub fn getPhoneViewIndent(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mPhoneViewIndent;
      return self.mPhoneViewIndent;
      **/
    }

    // java public int getViewTop()
    pub fn getViewTop(&self) -> i32 {
      // java return this.mViewTop;
      return self.mViewTop;
    }

    // java public void onScroll(int pos)
    pub fn onScroll(&mut self, pos: i32) {
      // java if (pos > this.mViewTop)
      if pos > self.mViewTop {
        // java this.mScrollDirection = 1;
        self.mScrollDirection = 1;
      }
      // java else if (pos < this.mViewTop)
      else if pos < self.mViewTop {
        // java this.mScrollDirection = 2;
        self.mScrollDirection = 2;
      }
      // java else
      else {
        // java this.mScrollDirection = 0;
        self.mScrollDirection = 0;
      }
      // java this.mViewTop = pos;
      self.mViewTop = pos;
    }

    // java public void resetViewTop()
    pub fn resetViewTop(&mut self) {
      // java this.mViewTop = 0;
      self.mViewTop = 0;
      // java this.mScrollDirection = 0;
      self.mScrollDirection = 0;
    }

    // java public boolean isScrollUp()
    pub fn isScrollUp(&self) -> bool {
      // java return this.mScrollDirection == 2;
      return self.mScrollDirection == 2;
    }

    // java public boolean isScrollDown()
    pub fn isScrollDown(&self) -> bool {
      // java return this.mScrollDirection == 1;
      return self.mScrollDirection == 1;
    }

    // java public int getWebViewMarginTop()
    pub fn getWebViewMarginTop(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mWebViewMarginTop;
      return self.mWebViewMarginTop;
      **/
    }

    // java public void setWebViewMarginTop(int webViewMarginTop)
    pub fn setWebViewMarginTop(&mut self, webViewMarginTop: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mWebViewMarginTop = webViewMarginTop;
      self.mWebViewMarginTop = webViewMarginTop;
      **/
    }

    // java public int getWebViewMarginLeft()
    pub fn getWebViewMarginLeft(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mWebViewMarginLeft;
      return self.mWebViewMarginLeft;
      **/
    }

    // java public void setWebViewMarginLeft(int webViewMarginLeft)
    pub fn setWebViewMarginLeft(&mut self, webViewMarginLeft: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mWebViewMarginLeft = webViewMarginLeft;
      self.mWebViewMarginLeft = webViewMarginLeft;
      **/
    }

    // java public int getWebViewMarginRight()
    pub fn getWebViewMarginRight(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mWebViewMarginRight;
      return self.mWebViewMarginRight;
      **/
    }

    // java public void setWebViewMarginRight(int webViewMarginRight)
    pub fn setWebViewMarginRight(&mut self, webViewMarginRight: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mWebViewMarginRight = webViewMarginRight;
      self.mWebViewMarginRight = webViewMarginRight;
      **/
    }

    // java public int getWebViewMarginBottom()
    pub fn getWebViewMarginBottom(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mWebViewMarginBottom;
      return self.mWebViewMarginBottom;
      **/
    }

    // java public void setWebViewMarginBottom(int webViewMarginBottom)
    pub fn setWebViewMarginBottom(&mut self, webViewMarginBottom: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mWebViewMarginBottom = webViewMarginBottom;
      self.mWebViewMarginBottom = webViewMarginBottom;
      **/
    }

    // java public int getBalloonsWidth()
    pub fn getBalloonsWidth(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mBalloonsWidth;
      return self.mBalloonsWidth;
      **/
    }

    // java public void setBalloonsWidth(int balloonsWidth)
    pub fn setBalloonsWidth(&mut self, balloonsWidth: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mBalloonsWidth = balloonsWidth;
      self.mBalloonsWidth = balloonsWidth;
      **/
    }

    // java public int getBalloonsMarginLeft()
    pub fn getBalloonsMarginLeft(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mBalloonsMarginLeft;
      return self.mBalloonsMarginLeft;
      **/
    }

    // java public void setBalloonsMarginLeft(int balloonsMarginLeft)
    pub fn setBalloonsMarginLeft(&mut self, balloonsMarginLeft: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mBalloonsMarginLeft = balloonsMarginLeft;
      self.mBalloonsMarginLeft = balloonsMarginLeft;
      **/
    }

    // java public int getBalloonsMarginRight()
    pub fn getBalloonsMarginRight(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mBalloonsMarginRight;
      return self.mBalloonsMarginRight;
      **/
    }

    // java public void setBalloonsMarginRight(int balloonsMarginRight)
    pub fn setBalloonsMarginRight(&mut self, balloonsMarginRight: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mBalloonsMarginRight = balloonsMarginRight;
      self.mBalloonsMarginRight = balloonsMarginRight;
      **/
    }

    // java public boolean isRevisionFinal()
    pub fn isRevisionFinal(&self) -> bool {
      // java return this.mRevisionFinal || !this.isShowRevision();
      (self.mRevisionFinal != 0) || !self.isShowRevision()
    }

    // java public void setRevisionFinal(boolean revisionFinal)
    pub fn setRevisionFinal(&mut self, revisionFinal: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mRevisionFinal = revisionFinal;
      self.mRevisionFinal = revisionFinal;
      **/
    }

    // java public boolean isSeparateAudioCommnet()
    pub fn isSeparateAudioCommnet(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSeparateAudioCommnet;
      return (self.mSeparateAudioCommnet as bool);
      **/
    }

    // java public void setSeparateAudioComment(boolean separateAudioCommnet)
    pub fn setSeparateAudioComment(&mut self, separateAudioCommnet: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSeparateAudioCommnet = false;
      self.mSeparateAudioCommnet = false;
      **/
    }

    // java public IRevisionStringMap getRevisionMap()
    pub fn getRevisionMap(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mRevisionMap)
      if false {
        // java RevisionStringMap revisionMap = RevisionStringMap.getInstance();
        let mut revisionMap: RevisionStringMap = RevisionStringMap::getInstance();
        // java this.mRevisionMap = revisionMap;
        self.mRevisionMap = revisionMap;
      }
      // java return this.mRevisionMap;
      return self.mRevisionMap;
      **/
    }

    // java public void setRevisionMap(IRevisionStringMap revisionMap)
    pub fn setRevisionMap(&mut self, revisionMap: &dyn IRevisionStringMap) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mRevisionMap = revisionMap;
      self.mRevisionMap = revisionMap;
      **/
    }

    // java public int getMaxLineOfComment()
    pub fn getMaxLineOfComment(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mMaxLineOfComment;
      return self.mMaxLineOfComment;
      **/
    }

    // java public void setMaxLineOfComment(int maxLineOfComment)
    pub fn setMaxLineOfComment(&mut self, maxLineOfComment: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMaxLineOfComment = maxLineOfComment;
      self.mMaxLineOfComment = maxLineOfComment;
      **/
    }

    // java public int getSpaceBetweenBalloon()
    pub fn getSpaceBetweenBalloon(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSpaceBetweenBalloon;
      return self.mSpaceBetweenBalloon;
      **/
    }

    // java public void setSpaceBetweenBalloon(int spaceBetweenBalloon)
    pub fn setSpaceBetweenBalloon(&mut self, spaceBetweenBalloon: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSpaceBetweenBalloon = spaceBetweenBalloon;
      self.mSpaceBetweenBalloon = spaceBetweenBalloon;
      **/
    }

    // java public float getBalloonsZoom()
    pub fn getBalloonsZoom(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mBalloonsZoom;
      return self.mBalloonsZoom;
      **/
    }

    // java public void setBalloonsZoom(float zoom)
    pub fn setBalloonsZoom(&mut self, zoom: f32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mBalloonsZoom = zoom;
      self.mBalloonsZoom = zoom;
      **/
    }

    // java public int getTabWidth()
    pub fn getTabWidth(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mTabWidth;
      return self.mTabWidth;
      **/
    }

    // java public void setTabWidth(int tabWidth)
    pub fn setTabWidth(&mut self, tabWidth: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mTabWidth = tabWidth;
      self.mTabWidth = tabWidth;
      **/
    }

    // java public PropertySet getBalloonContentProperty()
    pub fn getBalloonContentProperty(&self) -> util::moffice::property::PropertySet::PropertySet {
      // java return null == this.mViewSettings ? PropertySet.empty : this.mViewSettings.getBalloonContentProperty();
      // The Java settings object is not on this env yet, so this is the null branch.
      util::moffice::property::PropertySet::PropertySet::empty()
    }

    // java public PropertySet getEquationExceptionProperty()
    pub fn getEquationExceptionProperty(&self) -> Option<PropertySet> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return null == this.mViewSettings ? PropertySet.empty : this.mViewSettings.getEquationExceptionProperty();
      return (if false { Some(PropertySet::empty()) } else { Some(self.mViewSettings.getEquationExceptionProperty()) });
      **/
    }

    // java public float getContentLengthToBallonsLengthRatio()
    pub fn getContentLengthToBallonsLengthRatio(&self) -> f32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mContentLengthToBallonsLengthRatio;
      return self.mContentLengthToBallonsLengthRatio;
      **/
    }

    // java public void setContentLengthToBallonsLengthRatio(float contentLengthToBallonsLengthRatio)
    pub fn setContentLengthToBallonsLengthRatio(&mut self, contentLengthToBallonsLengthRatio: f32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mContentLengthToBallonsLengthRatio = contentLengthToBallonsLengthRatio;
      self.mContentLengthToBallonsLengthRatio = contentLengthToBallonsLengthRatio;
      **/
    }

    // java public int getPageBreakWidth()
    pub fn getPageBreakWidth(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mPageBreakWidth;
      return self.mPageBreakWidth;
      **/
    }

    // java public void setPageBreakWidth(int pageBreakWidth)
    pub fn setPageBreakWidth(&mut self, pageBreakWidth: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mPageBreakWidth = pageBreakWidth;
      self.mPageBreakWidth = pageBreakWidth;
      **/
    }

    // java public int getDefSectionNfcPgn()
    pub fn getDefSectionNfcPgn(&self) -> i32 {
      // java return this.mDefSectionNfcPgn;
      self.mDefSectionNfcPgn
    }

    // java public void setDefSectionNfcPgn(int defSectionNfcPgn)
    pub fn setDefSectionNfcPgn(&mut self, defSectionNfcPgn: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mDefSectionNfcPgn = defSectionNfcPgn;
      self.mDefSectionNfcPgn = defSectionNfcPgn;
      **/
    }

    // java public int contentLengthToBalloonsLength(int length)
    pub fn contentLengthToBalloonsLength(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return (int)((float)length / this.mContentLengthToBallonsLengthRatio);
      return ((((length) as f32) / self.mContentLengthToBallonsLengthRatio) as i32);
      **/
    }

    // java public int balloonsLengthToContentLength(int length, float mainScale, float scale)
    pub fn balloonsLengthToContentLength(&mut self, length: i32, mainScale: f32, scale: f32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return (int)((float)length * scale / mainScale);
      return ((((length) as f32) * scale / mainScale) as i32);
      **/
    }

    // java public String getCommentInfoString(String abbreviation, int indexOfText)
    pub fn getCommentInfoString(&self, abbreviation: &str, indexOfText: i32) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java String result = String.format(commentInfoFormat, this.mRevisionMap.getString(6), abbreviation, indexOfText);
      let mut result: String = String::format(commentInfoFormat, self.mRevisionMap.getString(6), abbreviation, indexOfText);
      // java return result;
      return result;
      **/
    }

    // java public String getRevisionInfo(String author, int mark, boolean needFlowColon)
    pub fn getRevisionInfo(&self, author: &str, mark: i32, needFlowColon: bool) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java String typeString = null;
      let mut typeString: String = Default::default();
      // java switch (mark)
      match mark {
        // java case 1:
        1 => {
          // java typeString = this.mRevisionMap.getString(2);
          typeString = self.mRevisionMap.getString(2);
        }
        // java case 2:
        2 => {
          // java typeString = this.mRevisionMap.getString(1);
          typeString = self.mRevisionMap.getString(1);
        }
        // java case 3:
        3 => {
          // java typeString = this.mRevisionMap.getString(3);
          typeString = self.mRevisionMap.getString(3);
        }
        // java default:
        _ => {
        }
      }
      // java String format = needFlowColon ? revisionFormatWithContent : revisionFormat;
      let mut format: String = (if needFlowColon { revisionFormatWithContent } else { revisionFormat });
      // java return String.format(format, author, typeString);
      return String::format(format, author, typeString);
      **/
    }

    // java public String getSubstituteForDrawing()
    pub fn getSubstituteForDrawing(&self) -> String {
      // java String str = this.mRevisionMap.getString(4);
      // java return String.format("<%s>", str.isEmpty() ? "对象" : str);
      // Revision string 4 is not installed on this env, so the empty branch is live.
      format!("<{}>", "对象")
    }

    // java public void setShowRevisionMakeup(boolean showRevisionMakeup)
    pub fn setShowRevisionMakeup(&mut self, showRevisionMakeup: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mShowRevisionMakeup = showRevisionMakeup;
      self.mShowRevisionMakeup = showRevisionMakeup;
      **/
    }

    // java public boolean getShowBalloon()
    pub fn getShowBalloon(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mShowRevisionMakeup || this.mShowComment;
      return self.mShowRevisionMakeup || self.mShowComment;
      **/
    }

    // java public boolean getShowRevisionMakeup()
    pub fn getShowRevisionMakeup(&self) -> bool {
      // java return this.mShowRevisionMakeup && this.isShowRevision();
      self.mShowRevisionMakeup != 0 && self.isShowRevision()
    }

    // java public boolean isShowRevisionChangeInballoons()
    pub fn isShowRevisionChangeInballoons(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mShowRevisionChangeInballoons;
      return self.mShowRevisionChangeInballoons;
      **/
    }

    // java public void setShowRevisionChangeInballoons(boolean showRevisionChangeInballoons)
    pub fn setShowRevisionChangeInballoons(&mut self, showRevisionChangeInballoons: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mShowRevisionChangeInballoons = showRevisionChangeInballoons;
      self.mShowRevisionChangeInballoons = showRevisionChangeInballoons;
      **/
    }

    // java public boolean isShowRevisionInsOrDelInBalloons()
    pub fn isShowRevisionInsOrDelInBalloons(&self) -> bool {
      // java return this.mShowRevisionInsOrDelInBalloons;
      self.mShowRevisionInsOrDelInBalloons.0 != 0
    }

    // java public void setShowRevisionInsOrDelInBalloons(boolean showRevisionInsOrDelInBalloons)
    pub fn setShowRevisionInsOrDelInBalloons(&mut self, showRevisionInsOrDelInBalloons: bool) {
      // java this.mShowRevisionInsOrDelInBalloons = showRevisionInsOrDelInBalloons;
      self.mShowRevisionInsOrDelInBalloons = RevisionBalloonFlag(if showRevisionInsOrDelInBalloons { 1 } else { 0 });
    }

    // java public boolean isDrawSpace()
    pub fn isDrawSpace(&self) -> bool {
      // java return this.mDrawSpace;
      self.mDrawSpace != 0
    }

    // java public void setDrawSpace(boolean drawSpace)
    pub fn setDrawSpace(&mut self, drawSpace: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mDrawSpace = drawSpace;
      self.mDrawSpace = drawSpace;
      **/
    }

    // java public boolean isDrawParaBreak()
    pub fn isDrawParaBreak(&self) -> bool {
      // java return this.mDrawParaBreak;
      self.mDrawParaBreak != 0
    }

    // java public void setDrawParaBreak(boolean drawParaBreak)
    pub fn setDrawParaBreak(&mut self, drawParaBreak: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mDrawParaBreak = drawParaBreak;
      self.mDrawParaBreak = drawParaBreak;
      **/
    }

    // java public boolean isDrawNewLine()
    pub fn isDrawNewLine(&self) -> bool {
      // java return this.mDrawNewLine;
      self.mDrawNewLine != 0
    }

    // java public void setDrawNewLine(boolean drawNewLine)
    pub fn setDrawNewLine(&mut self, drawNewLine: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mDrawNewLine = drawNewLine;
      self.mDrawNewLine = drawNewLine;
      **/
    }

    // java public boolean isEnableFixSectionBreak()
    pub fn isEnableFixSectionBreak(&self) -> bool {
      // java return this.mEnableFixSectionBreak;
      self.mEnableFixSectionBreak != 0
    }

    // java public void setEnableFixSectionBreak(boolean enableFixSectionBreak)
    pub fn setEnableFixSectionBreak(&mut self, enableFixSectionBreak: bool) {
      // java this.mEnableFixSectionBreak = enableFixSectionBreak;
      self.mEnableFixSectionBreak = i32::from(enableFixSectionBreak);
    }

    // java public float getPhoneViewLineSpacingLines()
    pub fn getPhoneViewLineSpacingLines(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mPhoneViewLineSpacingLines;
      return self.mPhoneViewLineSpacingLines;
      **/
    }

    // java public void setPhoneViewLineSpacingLines(float phoneViewLineSpacingLines)
    pub fn setPhoneViewLineSpacingLines(&mut self, phoneViewLineSpacingLines: f32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mPhoneViewLineSpacingLines = phoneViewLineSpacingLines;
      self.mPhoneViewLineSpacingLines = phoneViewLineSpacingLines;
      **/
    }

    // java public boolean isUlTrailSpace()
    pub fn isUlTrailSpace(&self) -> bool {
      // java return this.mUlTrailSpace;
      self.mUlTrailSpace != 0
    }

    // java public void setUlTrailSpace(boolean mUlTrailSpace)
    pub fn setUlTrailSpace(&mut self, mUlTrailSpace: bool) {
      // java this.mUlTrailSpace = mUlTrailSpace;
      self.mUlTrailSpace = i32::from(mUlTrailSpace);
    }

    // java public IViewListener getViewListener()
    pub fn getViewListener(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mViewListener;
      return self.mViewListener;
      **/
    }

    // java public IBalloonCacheListener getBalloonCacheListener()
    pub fn getBalloonCacheListener(&self) -> Box<dyn IBalloonCacheListener> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mBalloonCacheListener;
      return self.mBalloonCacheListener;
      **/
    }

    // java public void setBalloonCacheListener(IBalloonCacheListener listener)
    pub fn setBalloonCacheListener(&mut self, listener: &dyn IBalloonCacheListener) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mBalloonCacheListener = listener;
      self.mBalloonCacheListener = listener;
      **/
    }

    // java public void setHasFlowToBegin(boolean hasFlowToBegin)
    pub fn setHasFlowToBegin(&mut self, hasFlowToBegin: bool) {
      // java this.mHasFlowToBegin = hasFlowToBegin;
      self.mHasFlowToBegin = if hasFlowToBegin { 1 } else { 0 };
      // java if (this.mHasFlowToBegin && this.mViewListener != null) notify
      // The view listener is absent on this compile surface, so a null listener
      // takes Java's no-op branch.
    }

    // java public void setHasFlowToEnd(boolean hasFlowToEnd)
    pub fn setHasFlowToEnd(&mut self, hasFlowToEnd: bool) {
      // java this.mHasFlowToEnd = hasFlowToEnd;
      self.mHasFlowToEnd = if hasFlowToEnd { 1 } else { 0 };
    }

    // java public boolean hasFlowToBegin()
    pub fn hasFlowToBegin(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mHasFlowToBegin;
      return (self.mHasFlowToBegin as bool);
      **/
    }

    // java public boolean hasFlowToEnd()
    pub fn hasFlowToEnd(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mHasFlowToEnd;
      return self.mHasFlowToEnd;
      **/
    }

    // java public int getFootEndNoteSepLineWidth()
    pub fn getFootEndNoteSepLineWidth(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mFootEndNoteSepLineWidth;
      return self.mFootEndNoteSepLineWidth;
      **/
    }

    // java public void setFootEndNoteSepLineWidth(int val)
    pub fn setFootEndNoteSepLineWidth(&mut self, val: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFootEndNoteSepLineWidth = val;
      self.mFootEndNoteSepLineWidth = val;
      **/
    }

    // java public int getFootEndNoteTagWidth()
    pub fn getFootEndNoteTagWidth(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mFootEndNoteTagWidth;
      return self.mFootEndNoteTagWidth;
      **/
    }

    // java public void setFootEndNoteTagWidth(int val)
    pub fn setFootEndNoteTagWidth(&mut self, val: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFootEndNoteTagWidth = val;
      self.mFootEndNoteTagWidth = val;
      **/
    }

    // java public int getFootEndNoteTagHeight()
    pub fn getFootEndNoteTagHeight(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mFootEndNoteTagHeight;
      return self.mFootEndNoteTagHeight;
      **/
    }

    // java public void setFootEndNoteTagHeight(int val)
    pub fn setFootEndNoteTagHeight(&mut self, val: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFootEndNoteTagHeight = val;
      self.mFootEndNoteTagHeight = val;
      **/
    }

    // java public void setMaxInkCommentHeightWithFold(int val)
    pub fn setMaxInkCommentHeightWithFold(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMaxInkCommentHeightWithFold = val;
      self.mMaxInkCommentHeightWithFold = val;
      **/
    }

    // java public int getMaxInkCommentHeightWithFold()
    pub fn getMaxInkCommentHeightWithFold(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mMaxInkCommentHeightWithFold;
      return self.mMaxInkCommentHeightWithFold;
      **/
    }

    // java public boolean isSmartFontSize()
    pub fn isSmartFontSize(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mPhoneViewParam.isSmartFontSize();
      return self.mPhoneViewParam.isSmartFontSize();
      **/
    }

    // java public void setSmartFontSize(boolean smartFontSize)
    pub fn setSmartFontSize(&mut self, smartFontSize: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mPhoneViewParam.setSmartFontSize(smartFontSize);
      self.mPhoneViewParam.setSmartFontSize(smartFontSize);
      **/
    }

    // java public boolean isSmartFirstLineIndent()
    pub fn isSmartFirstLineIndent(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSmartFirstLineIndent;
      return (self.mSmartFirstLineIndent as bool);
      **/
    }

    // java public void setSmartFirstLineIndent(boolean smartFirstLineIndent)
    pub fn setSmartFirstLineIndent(&mut self, smartFirstLineIndent: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSmartFirstLineIndent = smartFirstLineIndent;
      self.mSmartFirstLineIndent = smartFirstLineIndent;
      **/
    }

    // java public boolean isAutoHyphenation()
    pub fn isAutoHyphenation(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mAutoHyphenation;
      return (self.mAutoHyphenation as bool);
      **/
    }

    // java public void setAutoHyphenation(boolean autoHyphenation)
    pub fn setAutoHyphenation(&mut self, autoHyphenation: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mAutoHyphenation = autoHyphenation;
      self.mAutoHyphenation = autoHyphenation;
      **/
    }

    // java public boolean isShowCardInfo()
    pub fn isShowCardInfo(&self) -> bool {
      // java return ViewMode.isPhoneScrollMode(this.mViewMode) ? this.mShowCardInfo : false;
      return if ViewMode::isPhoneScrollMode(self.mViewMode) { self.mShowCardInfo != 0 } else { false };
    }

    // java public void setShowCardInfo(boolean showCardInfo)
    pub fn setShowCardInfo(&mut self, showCardInfo: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mShowCardInfo = showCardInfo;
      self.mShowCardInfo = showCardInfo;
      **/
    }

    // java PhoneViewParam.isFitTableForView()
    pub fn isFitTableForView(&self) -> bool {
      self.fit_table_for_view
    }

    // java public PhoneViewParam getPhoneViewParam()
    pub fn getPhoneViewParam(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mPhoneViewParam;
      return self.mPhoneViewParam;
      **/
    }

    // java public void setPhoneViewParam(PhoneViewParam phoneViewParam)
    pub fn setPhoneViewParam(&mut self, phoneViewParam: Option<&PhoneViewParam>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (phoneViewParam != null)
      if let Some(phoneViewParam) = phoneViewParam {
        // java this.mPhoneViewParam = phoneViewParam.clone();
        self.mPhoneViewParam = phoneViewParam.clone();
      }
      **/
    }

    // java public int getDocumentLID()
    pub fn getDocumentLID(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mDocmentLID;
      return self.mDocmentLID;
      **/
    }

    // java public void setDocumentLID(int documentLID)
    pub fn setDocumentLID(&mut self, documentLID: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mDocmentLID = documentLID;
      self.mDocmentLID = documentLID;
      **/
    }

    // java public int getPhoneModeAdjustLine()
    pub fn getPhoneModeAdjustLine(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mPhoneModeAdjustLine;
      return self.mPhoneModeAdjustLine;
      **/
    }

    // java public void setPhoneModeAdjustLine(int mPhoneModeAdjustLine)
    pub fn setPhoneModeAdjustLine(&mut self, mPhoneModeAdjustLine: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mPhoneModeAdjustLine = mPhoneModeAdjustLine;
      self.mPhoneModeAdjustLine = mPhoneModeAdjustLine;
      **/
    }

    // java public float getPhoneModeSpaceBeforeLines()
    pub fn getPhoneModeSpaceBeforeLines(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mPhoneModeSpaceBeforeLines;
      return self.mPhoneModeSpaceBeforeLines;
      **/
    }

    // java public void setPhoneModeSpaceBeforeLines(float mPhoneModeSpaceBeforeLines)
    pub fn setPhoneModeSpaceBeforeLines(&mut self, mPhoneModeSpaceBeforeLines: f32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mPhoneModeSpaceBeforeLines = mPhoneModeSpaceBeforeLines;
      self.mPhoneModeSpaceBeforeLines = mPhoneModeSpaceBeforeLines;
      **/
    }

    // java public BRC getDefaultEQXBRC()
    pub fn getDefaultEQXBRC(&self) -> crate::cn::wps::moffice::writer::data::value::BRC::BRC {
      self.mDefaultEQXBRC.0.clone()
    }

    // java public PropertySet getShareViewProperty()
    pub fn getShareViewProperty(&self) -> Option<util::moffice::property::PropertySet::PropertySet> {
      // java return this.mShareViewProperty;
      self.mShareViewProperty.clone()
    }

    // java public void setShareViewProperty(PropertySet mShareViewProperty)
    pub fn setShareViewProperty(&mut self, mShareViewProperty: &util::moffice::property::PropertySet::PropertySet) {
      // java this.mShareViewProperty = mShareViewProperty;
      self.mShareViewProperty = Some(mShareViewProperty.clone());
    }

    // java public float getShareViewFontsize()
    pub fn getShareViewFontsize(&self) -> f32 {
      // java return this.mShareViewFontsize;
      self.mShareViewFontsize
    }

    // java public void setShareViewFontsize(float fontSize)
    pub fn setShareViewFontsize(&mut self, fontSize: f32) {
      // java this.mShareViewFontsize = fontSize;
      self.mShareViewFontsize = fontSize;
    }

    // java public void setAudioCommentHeight(int val)
    pub fn setAudioCommentHeight(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mAudioCommentHeight = val;
      self.mAudioCommentHeight = val;
      **/
    }

    // java public float getAudioCommentHeight()
    pub fn getAudioCommentHeight(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return (float)this.mAudioCommentHeight;
      return ((self.mAudioCommentHeight) as f32);
      **/
    }

    // java public void setEmptyCommentHeight(int val)
    pub fn setEmptyCommentHeight(&mut self, val: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mEmptyCommentHeight = val;
      self.mEmptyCommentHeight = val;
      **/
    }

    // java public float getEmptyCommentHeight()
    pub fn getEmptyCommentHeight(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return (float)this.mEmptyCommentHeight;
      return ((self.mEmptyCommentHeight) as f32);
      **/
    }

    // java public void setForceUseShareViewRunProperty(boolean value)
    pub fn setForceUseShareViewRunProperty(&mut self, value: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mForceUseShareViewRunProperty = value;
      self.mForceUseShareViewRunProperty = value;
      **/
    }

    // java public boolean getForceUseShareViewRunProperty()
    pub fn getForceUseShareViewRunProperty(&self) -> bool {
      // java return this.mForceUseShareViewRunProperty;
      (self.mForceUseShareViewRunProperty != 0)
    }

    // java public void addPageListener(IPageChangeListener l)
    pub fn addPageListener(&mut self, l: Option<&dyn IPageChangeListener>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != l)
      if let Some(l) = l {
          // java (this.mPageChangeListeners)
          self.mPageChangeListeners;
            // java this.mPageChangeListeners.add(l);
            self.mPageChangeListeners.add(l);
      }
      **/
    }

    // java public void removePageListener(IPageChangeListener l)
    pub fn removePageListener(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != l)
      if true {
          // java (this.mPageChangeListeners)
          self.mPageChangeListeners;
            // java this.mPageChangeListeners.remove(l);
            self.mPageChangeListeners.remove(l);
      }
      **/
    }

    // java public void onPageModified(Rect changeRect)
    pub fn onPageModified(&mut self, _changeRect: Option<&()>) {
      // java synchronized(this.mPageChangeListeners) empty Default list
      let _ = _changeRect;
    }

    // java public void onOffsetPageAfter(int pos, int yOffset)
    pub fn onOffsetPageAfter(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java (this.mPageChangeListeners)
        self.mPageChangeListeners;
          // java int i = 0;
          let mut i: i32 = 0;
          let mut count: i32 = (self.mPageChangeListeners.len() as i32);
          // java for(int count = this.mPageChangeListeners.size(); i < count; ++i)
          while i < count {
            // java ((IPageChangeListener)this.mPageChangeListeners.get(i)).onOffsetPageAfter(pos, yOffset);
            todo!("cast → Box<dyn IPageChangeListener>").onOffsetPageAfter(pos, yOffset);
          }
        **/
    }

    // java public void beforeClearDocument()
    pub fn beforeClearDocument(&mut self) {
        // java synchronized(this.mPageChangeListeners) empty Default list
    }

    // java public void onPageInserted(int pLayoutPage, int pos)
    pub fn onPageInserted(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java (this.mPageChangeListeners)
        self.mPageChangeListeners;
          // java int i = 0;
          let mut i: i32 = 0;
          let mut count: i32 = (self.mPageChangeListeners.len() as i32);
          // java for(int count = this.mPageChangeListeners.size(); i < count; ++i)
          while i < count {
            // java ((IPageChangeListener)this.mPageChangeListeners.get(i)).onPageInserted(pLayoutPage, pos);
            todo!("cast → Box<dyn IPageChangeListener>").onPageInserted(pLayoutPage, pos);
          }
        **/
    }

    // java public void onPageDeleted(int pLayoutPage)
    pub fn onPageDeleted(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java (this.mPageChangeListeners)
        self.mPageChangeListeners;
          // java int i = 0;
          let mut i: i32 = 0;
          let mut count: i32 = (self.mPageChangeListeners.len() as i32);
          // java for(int count = this.mPageChangeListeners.size(); i < count; ++i)
          while i < count {
            // java ((IPageChangeListener)this.mPageChangeListeners.get(i)).onPageDeleted(pLayoutPage);
            todo!("cast → Box<dyn IPageChangeListener>").onPageDeleted(pLayoutPage);
          }
        **/
    }

    // java public void setShowComment(boolean show)
    pub fn setShowComment(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mShowComment = show;
      self.mShowComment = show;
      **/
    }

    // java public boolean isShowComment()
    pub fn isShowComment(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mShowComment;
      return (self.mShowComment as bool);
      **/
    }

    // java public void setShowRevision(boolean show)
    pub fn setShowRevision(&mut self, show: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mShowRevision = show;
      self.mShowRevision = show;
      **/
    }

    // java public boolean isShowRevision()
    pub fn isShowRevision(&self) -> bool {
      // java return this.mShowRevision;
      self.mShowRevision != 0
    }

    // java public void setRevisionInsertProperty(PropertySet revisionInsertProperty)
    pub fn setRevisionInsertProperty(&mut self, revisionInsertProperty: &util::moffice::property::PropertySet::PropertySet) {
      // java this.mRevisionInsertProperty = revisionInsertProperty;
      self.mRevisionInsertProperty = Some(revisionInsertProperty.clone());
    }

    // java public PropertySet getRevisionInsertProperty()
    pub fn getRevisionInsertProperty(&self) -> Option<&util::moffice::property::PropertySet::PropertySet> {
      // java return this.mRevisionInsertProperty;
      self.mRevisionInsertProperty.as_ref()
    }

    // java public void setRevisionDeleteProperty(PropertySet revisionDeleteProperty)
    pub fn setRevisionDeleteProperty(&mut self, revisionDeleteProperty: &util::moffice::property::PropertySet::PropertySet) {
      // java this.mRevisionDeleteProperty = revisionDeleteProperty;
      self.mRevisionDeleteProperty = Some(revisionDeleteProperty.clone());
    }

    // java public PropertySet getRevisionDeleteProperty()
    pub fn getRevisionDeleteProperty(&self) -> Option<&util::moffice::property::PropertySet::PropertySet> {
      // java return this.mRevisionDeleteProperty;
      self.mRevisionDeleteProperty.as_ref()
    }

    // java public boolean isCompressSpace()
    pub fn isCompressSpace(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return !this.mSmartFirstLineIndent && this.mPhoneViewParam.isCompressSpace();
      return !self.mSmartFirstLineIndent && self.mPhoneViewParam.isCompressSpace();
      **/
    }

    // java public int getCardMarginLeft()
    pub fn getCardMarginLeft(&self) -> i32 {
      // java return this.isShowCardInfo() ? 320 : 0;
      if self.isShowCardInfo() { 320 } else { 0 }
    }

    // java public int getCardMarginRight()
    pub fn getCardMarginRight(&self) -> i32 {
      // java return this.isShowCardInfo() ? 320 : 0;
      if self.isShowCardInfo() { 320 } else { 0 }
    }

    // java public int getCardMarginBottom()
    pub fn getCardMarginBottom(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.isShowCardInfo() ? 320 : 0;
      return (if self.isShowCardInfo() { 320 } else { 0 });
      **/
    }

    // java public int getCardMarginTop()
    pub fn getCardMarginTop(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.isShowCardInfo() ? 320 : 0;
      return (if self.isShowCardInfo() { 320 } else { 0 });
      **/
    }

}
