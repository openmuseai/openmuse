// Java: import cn.wps.font.emoji.EmojiServiceFactory
use thirdparty_font::cn::wps::font::emoji::EmojiServiceFactory::EmojiServiceFactory;
// Java: import cn.wps.font.emoji.IEmojiService
use thirdparty_font::cn::wps::font::emoji::IEmojiService::IEmojiService;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.util.UnitUtil
use util::moffice::util::UnitUtil::UnitUtil;
// Java: import cn.wps.moffice.writer.cache.TypoDocument
use crate::cn::wps::moffice::writer::cache::TypoDocument::TypoDocument;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
// Java: import cn.wps.moffice.writer.core.WdCompatibilityMode
use crate::cn::wps::moffice::writer::core::WdCompatibilityMode::WdCompatibilityMode;
// Java: import cn.wps.moffice.writer.core.pool.Reusable
use crate::cn::wps::moffice::writer::core::pool::Reusable::Reusable;
// Java: import cn.wps.moffice.writer.function.card.CardPaper
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::function::card::CardPaper::CardPaper;
// Java: import cn.wps.moffice.writer.layout.base.CacheCleaner
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::CacheCleaner::CacheCleaner;
// Java: import cn.wps.moffice.writer.layout.base.footendnote.FootEndNotePropCalc
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::footendnote::FootEndNotePropCalc::FootEndNotePropCalc;
// Java: import cn.wps.moffice.writer.layout.base.footendnote.IFootEndNoteNumLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::footendnote::IFootEndNoteNumLayouter::IFootEndNoteNumLayouter;
// Java: import cn.wps.moffice.writer.layout.phoneview.LineBreakAnalyzer
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::phoneview::LineBreakAnalyzer::LineBreakAnalyzer;
// Java: import cn.wps.moffice.writer.layout.base.env.Pools
use crate::cn::wps::moffice::writer::layout::base::env::Pools::Pools;
// Java: import cn.wps.moffice.writer.layout.base.env.PropertyToolFactory
// rust: ★wr-29 04 —— 真实模块（去影子）
use crate::cn::wps::moffice::writer::layout::base::env::PropertyToolFactory::PropertyToolFactory as RealPropertyToolFactory;
// Java: import cn.wps.moffice.writer.layout.base.env.FontStyleCacheFactory
// rust: ★wr-29 04 —— 真实模块（去影子）
use crate::cn::wps::moffice::writer::layout::base::env::FontStyleCacheFactory::FontStyleCacheFactory as RealFontStyleCacheFactory;
// Java: import cn.wps.moffice.writer.layout.base.env.FontStyleCacheFactory
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::FontStyleCacheFactory::FontStyleCacheFactory;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutToolsManager
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::LayoutToolsManager::LayoutToolsManager;
// Java: import cn.wps.moffice.writer.layout.base.env.PhoneViewFitInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::PhoneViewFitInfo::PhoneViewFitInfo;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;

// CARGO_OFF_SURFACE_TYPES
// java public final class CacheCleaner
#[derive(Default)]
pub struct CacheCleaner {
    mMainState: *mut crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState,
    mDocEnv: *mut DocumentEnv,
}
// rust: ★wr-29 L4-2/04 —— 去影子：FontStyleCacheFactory 用真实模块（env::FontStyleCacheFactory）
//       PropertyToolFactory 同理（env::PropertyToolFactory）
#[derive(Default)]
pub struct FootEndNotePropCalc;

// java public static final int DEFTABCHARS = 2;
pub(crate) const DEFTABCHARS: i32 = 2;

// java public class DocumentEnv implements Reusable
#[derive(Default)]
pub struct DocumentEnv {
    // java public TextDocument textDocument;
    pub textDocument: Option<TextDocument>,
    // rust: Java DocumentEnv holds the same TextDocument object as the importer.
    // The product host owns it for the whole synchronous layout call; this alias
    // keeps numbering tables and their mutable LstLfoTool attached to that object.
    pub(crate) textDocumentAlias: *mut TextDocument,
    // rust: Java `textDocument.getAsiaTypography()` 经同一文档对象取得；产品入口借用
    // TextDocument 时在 updateFromDocument 装配对应的文档级断行表，供行排版读取。
    pub(crate) textDocumentAsiaTypography: Option<crate::cn::wps::moffice::writer::core::AsiaTypography::AsiaTypography>,
    // java public TypoDocument typoDocument;
    pub(crate) typoDocument: TypoDocument,
    // java public int defTab = 400;
    pub(crate) defTab: i32,
    // java public boolean evenAndOddPagesDiff = false;
    pub(crate) evenAndOddPagesDiff: bool,
    // java public boolean noTabHangInd = false;
    pub(crate) noTabHangInd: bool,
    // java public boolean noNumTabHandInd = false;
    pub(crate) noNumTabHandInd: bool,
    // java public boolean notBreakWrappedTables = false;
    pub(crate) notBreakWrappedTables: bool,
    // java public boolean canContinueFlowNextSec;
    pub(crate) canContinueFlowNextSec: bool,
    // java public boolean isShapeLayoutLikeWW8;
    pub(crate) isShapeLayoutLikeWW8: bool,
    // java public int compressLevel;
    pub compressLevel: i32,
    // java public boolean noAdjustLineHeightInTable = false;
    pub(crate) noAdjustLineHeightInTable: bool,
    // java public boolean useAltKinsokuLineBreakRules = false;
    pub useAltKinsokuLineBreakRules: bool,
    // java public boolean balanceSingleByteDoubleByteWidth = false;
    // java public boolean balanceSingleByteDoubleByteWidth = false;
    pub balanceSingleByteDoubleByteWidth: bool,
    // java public boolean forbidPunctuationKerning = false;
    pub(crate) forbidPunctuationKerning: bool,
    // java public boolean forbidEastAsianBreakRules = false;
    pub(crate) forbidEastAsianBreakRules: bool,
    // java public boolean wrapTrailSpaces = false;
    pub wrapTrailSpaces: bool,
    // java public boolean useWord97LineBreakRules = false;
    pub(crate) useWord97LineBreakRules: bool,
    // java public boolean alignTablesRowByRow = false;
    pub(crate) alignTablesRowByRow: bool,
    // java public boolean growAutoFit = false;
    pub(crate) growAutoFit: bool,
    // java public boolean isTableIndentedByOldSTyle = false;
    pub(crate) isTableIndentedByOldSTyle: bool,
    // java public boolean showEndNotesAtTheEndOfSection = false;
    pub(crate) showEndNotesAtTheEndOfSection: bool,
    // java public boolean doNotUseHTMLParagraphAutoSpacing = false;
    pub(crate) doNotUseHTMLParagraphAutoSpacing: bool,
    // java public boolean notMergeTypoText;
    pub(crate) notMergeTypoText: bool,
    // java public boolean cachedColBalance;
    pub(crate) cachedColBalance: bool,
    // java public boolean mirrorMargins;
    pub(crate) mirrorMargins: bool,
    // java public Pools pools;
    pub(crate) pools: Pools,
    // java public CacheCleaner cacheCleaner;
    pub(crate) cacheCleaner: CacheCleaner,
    // java public IFootEndNoteNumLayouter footEndNoteNumLayouter;
    pub(crate) footEndNoteNumLayouter: Option<Box<dyn crate::cn::wps::moffice::writer::layout::base::footendnote::IFootEndNoteNumLayouter::IFootEndNoteNumLayouter>>,
    // java public PropertyToolFactory propToolFactory;
    pub propToolFactory: Option<crate::cn::wps::moffice::writer::layout::base::env::PropertyToolFactory::PropertyToolFactory>,
    // java public FontStyleCacheFactory fontStyleCacheFactory;
    // rust: ★wr-29 04 —— 真实 FontStyleCacheFactory（Holder 影子已去）
    // rust: ★wr-29 04 —— pub（测试/04 单测装配字体样式缓存工厂）
    pub fontStyleCacheFactory: Option<RealFontStyleCacheFactory>,
    // java public LayoutToolsManager layoutToolsManager;
    pub(crate) layoutToolsManager: Option<LayoutToolsManagerHolder>,
    // java private IEmojiService mEmojiService;
    mEmojiService: Option<EmojiHolder>,
    // java private PhoneViewFitInfo mPhoneViewFitInfo;
    mPhoneViewFitInfo: Option<PhoneViewFitInfoHolder>,
    // java public LineBreakAnalyzer mLineBreakAnalyzer;
    pub(crate) mLineBreakAnalyzer: Option<LineBreakAnalyzerHolder>,
    // java private CardPaper mCarPaper;
    mCarPaper: Option<CardPaperHolder>,
    // java public boolean mIsDirty;
    pub(crate) mIsDirty: bool,
}

// java public interface Reusable
impl Reusable for DocumentEnv {
    // java public boolean reuseClean()
    fn reuseClean(&mut self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.textDocument = null;
      self.textDocument = None;
      self.textDocumentAlias = std::ptr::null_mut();
      // java this.layoutToolsManager.reuseClean();
      self.layoutToolsManager.reuseClean();
      // java this.footEndNoteNumLayouter = null;
      self.footEndNoteNumLayouter = None;
      // java this.mPhoneViewFitInfo = null;
      self.mPhoneViewFitInfo = None;
      // java this.mCarPaper = null;
      self.mCarPaper = None;
      // java return true;
      return true;
      */
    }

    // java public void reuseInit()
    fn reuseInit(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.textDocument = this.typoDocument.getDocument();
      self.textDocument = self.typoDocument.getDocument();
      // java this.layoutToolsManager.reuseInit();
      self.layoutToolsManager.reuseInit();
      */
    }

}

// java public class DocumentEnv implements Reusable
impl DocumentEnv {
    // java public CardPaper getCardPaper()
    pub fn getCardPaper(&mut self) -> Option<CardPaperHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mIsDirty || this.mCarPaper == null)
      if self.mIsDirty || self.mCarPaper.is_none() {
        // java this.mCarPaper = this.collectCardInfo();
        self.mCarPaper = self.collectCardInfo();
        // java this.mIsDirty = false;
        self.mIsDirty = false;
      }
      // java return this.mCarPaper;
      return self.mCarPaper;
      */
    }

    // java private CardPaper collectCardInfo()
    pub fn collectCardInfo(&self) -> Option<CardPaperHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.textDocument != null && this.layoutToolsManager.getViewEnv().isShowCardInfo())
      if let Some(textDocument) = self.textDocument.as_ref() {
        if self.layoutToolsManager.getViewEnv().isShowCardInfo() {
          // java CardPaper cardPaper = new CardPaper(this.textDocument.getMainDocument());
          let mut cardPaper: CardPaper = CardPaper::new(textDocument.getMainDocument());
          // java if (cardPaper.collect())
          if cardPaper.collect() {
            // java return cardPaper;
            return { let __x: Option<CardPaperHolder> = todo!("no align strategy matched"); __x };
          }
        }
      }
      // java return null;
      return None;
      */
    }

    // java public void setCardPaperDirty(boolean isDirty)
    pub fn setCardPaperDirty(&mut self, isDirty: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mIsDirty = isDirty;
      self.mIsDirty = isDirty;
      */
    }

    // java public PhoneViewFitInfo getPhoneViewFitInfo()
    pub fn getPhoneViewFitInfo(&mut self) -> Option<PhoneViewFitInfoHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mPhoneViewFitInfo == null)
      if self.mPhoneViewFitInfo.is_none() {
        // java this.mPhoneViewFitInfo = new PhoneViewFitInfo();
        self.mPhoneViewFitInfo = PhoneViewFitInfo::new();
      }
      // java return this.mPhoneViewFitInfo;
      return self.mPhoneViewFitInfo;
      */
    }

    // java public void clearFitInfo()
    pub fn clearFitInfo(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mPhoneViewFitInfo = null;
      self.mPhoneViewFitInfo = None;
      */
    }

    // java public  DocumentEnv(TypoDocument typoDocument, Pools pools, CacheCleaner cleaner, PropertyToolFactory propTool, FontStyleCacheFactory fontStyleCache, ViewEnv viewEnv)
    pub fn DocumentEnv(typoDocument: &TypoDocument, pools: &Pools, cleaner: &CacheCleaner, propTool: &RealPropertyToolFactory, fontStyleCache: &RealFontStyleCacheFactory, viewEnv: &ViewEnv) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.typoDocument = typoDocument;
      Self { textDocument: typoDocument.getDocument(), textDocumentAsiaTypography: None, typoDocument: typoDocument, defTab: Default::default(), evenAndOddPagesDiff: Default::default(), noTabHangInd: Default::default(), noNumTabHandInd: Default::default(), notBreakWrappedTables: Default::default(), canContinueFlowNextSec: Default::default(), isShapeLayoutLikeWW8: Default::default(), compressLevel: Default::default(), noAdjustLineHeightInTable: Default::default(), useAltKinsokuLineBreakRules: Default::default(), balanceSingleByteDoubleByteWidth: Default::default(), forbidPunctuationKerning: Default::default(), forbidEastAsianBreakRules: Default::default(), wrapTrailSpaces: Default::default(), useWord97LineBreakRules: Default::default(), alignTablesRowByRow: Default::default(), growAutoFit: Default::default(), isTableIndentedByOldSTyle: Default::default(), showEndNotesAtTheEndOfSection: Default::default(), doNotUseHTMLParagraphAutoSpacing: Default::default(), notMergeTypoText: Default::default(), cachedColBalance: Default::default(), mirrorMargins: Default::default(), pools: pools, cacheCleaner: cleaner, footEndNoteNumLayouter: Default::default(), propToolFactory: propTool, fontStyleCacheFactory: fontStyleCache, layoutToolsManager: LayoutToolsManager::new(self, viewEnv), mEmojiService: Default::default(), mPhoneViewFitInfo: Default::default(), mLineBreakAnalyzer: Default::default(), mCarPaper: Default::default(), mIsDirty: Default::default() }
      // java this.textDocument = typoDocument.getDocument();
      // java this.pools = pools;
      // java this.cacheCleaner = cleaner;
      // java this.propToolFactory = propTool;
      // java this.fontStyleCacheFactory = fontStyleCache;
      // java this.layoutToolsManager = new LayoutToolsManager(this, viewEnv);
      */
    }

    // java public DocumentEnv getClone()
    pub fn getClone(&self) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DocumentEnv docEnv = new DocumentEnv(this.typoDocument, this.pools, this.cacheCleaner, this.propToolFactory, this.fontStyleCacheFactory, this.layoutToolsManager.getViewEnv());
      let mut docEnv: DocumentEnv = DocumentEnv::new(self.typoDocument, self.pools, self.cacheCleaner, self.propToolFactory, self.fontStyleCacheFactory, self.layoutToolsManager.getViewEnv());
      // java return docEnv;
      return { let __x: Self = todo!("no align strategy matched"); __x };
      */
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.textDocument = null;
      self.textDocument = None;
      // java this.typoDocument = null;
      self.typoDocument = None;
      // java this.pools = null;
      self.pools = None;
      // java this.cacheCleaner = null;
      self.cacheCleaner = None;
      // java this.propToolFactory = null;
      self.propToolFactory = None;
      // java this.fontStyleCacheFactory = null;
      self.fontStyleCacheFactory = None;
      // java if (null != this.mEmojiService)
      if let Some(mEmojiService) = self.mEmojiService.as_mut() {
        // java this.mEmojiService.dispose();
        mEmojiService.dispose();
        // java this.mEmojiService = null;
        mEmojiService = None;
      }
      // java this.mPhoneViewFitInfo = null;
      self.mPhoneViewFitInfo = None;
      // java if (null != this.mCarPaper)
      if let Some(mCarPaper) = self.mCarPaper.as_mut() {
        // java this.mCarPaper.dispose();
        mCarPaper.dispose();
      }
      // java this.mCarPaper = null;
      self.mCarPaper = None;
      */
    }

    // rust: Java DocumentEnv.update 读取 textDocument；产品入口借用同一文档，
    // 因此以引用传入，避免复制 TextDocument 的可变模型。
    pub fn updateFromDocument(&mut self, textDocument: &TextDocument) {
      self.textDocumentAlias = textDocument as *const TextDocument as *mut TextDocument;
      if std::env::var_os("WORD_PARITY_AUTONUM_TRACE_CP").is_some() {
        eprintln!("autonum document table={} lfo_count={} lst_count={} level_1_1={}", textDocument.mAutoNumTable.is_some(),
          textDocument.mAutoNumTable.as_ref().and_then(|table| table.mLfoTable.as_ref()).and_then(|table| table.mLfoMap.as_ref()).map(|m| m.len()).unwrap_or(0),
          textDocument.mAutoNumTable.as_ref().and_then(|table| table.mLstTable.as_ref()).and_then(|table| table.mLstMap.as_ref()).map(|m| m.len()).unwrap_or(0),
          textDocument.mAutoNumTable.as_ref().is_some_and(|table| table.level_exists(1, 1)));
      }
      // java this.textDocument.getAsiaTypography() 首次访问时 new AsiaTypography(this); init();
      let mut asia = crate::cn::wps::moffice::writer::core::AsiaTypography::AsiaTypography::AsiaTypography(textDocument);
      asia.init();
      self.textDocumentAsiaTypography = Some(asia);
      // java PropertySet property = this.textDocument.getProperty();
      let property = textDocument.getProperty();
      // java WdCompatibilityMode compatMode = this.textDocument.getCompatibilityMode();
      let compatMode = property.getValue(542)
        .and_then(|v| v.as_any().downcast_ref::<Vec<crate::cn::wps::moffice::writer::data::value::CompatSetting::CompatSetting>>())
        .and_then(|settings| settings.iter().find(|s| s.getName().as_deref().is_some_and(|n| n.eq_ignore_ascii_case("compatibilityMode"))))
        .and_then(|setting| setting.getVal())
        .map(|value| WdCompatibilityMode::convertValue(&value))
        .unwrap_or(WdCompatibilityMode::wdWord2010);
      self.updateFromProperty(&property, compatMode);
    }

    pub(crate) fn textDocument_mut(&mut self) -> Option<&mut TextDocument> {
      if let Some(doc) = self.textDocument.as_mut() {
        Some(doc)
      } else if !self.textDocumentAlias.is_null() {
        // Java aliases the same mutable document through DocumentEnv. The
        // pointer is valid while java_pipeline_pages holds its boxed document.
        Some(unsafe { &mut *self.textDocumentAlias })
      } else {
        None
      }
    }

    // rust: DocumentEnv.update 的属性赋值体；供借用文档的产品宿主复用。
    fn updateFromProperty(&mut self, property: &PropertySet, compatMode: WdCompatibilityMode) {
      // java this.evenAndOddPagesDiff = property.getBoolean(391, false);
      self.evenAndOddPagesDiff = property.getBoolean__int_boolean(391, false);
      // java this.defTab = property.getInt(377, UnitUtil.PT2TWIPS(10.0F) * 2);
      self.defTab = property.getInt__int_int(377, 400);
      // java this.noTabHangInd = property.getBoolean(482, false);
      self.noTabHangInd = property.getBoolean__int_boolean(482, false);
      // java this.noNumTabHandInd = property.getBoolean(527, false);
      self.noNumTabHandInd = property.getBoolean__int_boolean(527, false);
      // java this.notBreakWrappedTables = property.getBoolean(517, false);
      self.notBreakWrappedTables = property.getBoolean__int_boolean(517, false);
      // java this.isShapeLayoutLikeWW8 = property.getBoolean(541, false);
      self.isShapeLayoutLikeWW8 = property.getBoolean__int_boolean(541, false);
      // java this.noAdjustLineHeightInTable = property.getBoolean(390, false);
      self.noAdjustLineHeightInTable = property.getBoolean__int_boolean(390, false);
      // java this.compressLevel = property.getInt(382, 0);
      self.compressLevel = property.getInt__int_int(382, 0);
      // java this.useAltKinsokuLineBreakRules = property.getBoolean(528, false);
      self.useAltKinsokuLineBreakRules = property.getBoolean__int_boolean(528, false);
      // java this.balanceSingleByteDoubleByteWidth = property.getBoolean(388, false);
      self.balanceSingleByteDoubleByteWidth = property.getBoolean__int_boolean(388, false);
      // java this.forbidPunctuationKerning = property.getBoolean(383, false);
      self.forbidPunctuationKerning = property.getBoolean__int_boolean(383, false);
      // java this.forbidEastAsianBreakRules = property.getBoolean(522, false);
      self.forbidEastAsianBreakRules = property.getBoolean__int_boolean(522, false);
      // java this.wrapTrailSpaces = property.getBoolean(508, false);
      self.wrapTrailSpaces = property.getBoolean__int_boolean(508, false);
      // java this.useWord97LineBreakRules = property.getBoolean(516, false);
      self.useWord97LineBreakRules = property.getBoolean__int_boolean(516, false);
      // java this.cachedColBalance = true;
      self.cachedColBalance = true;
      // java this.doNotUseHTMLParagraphAutoSpacing = property.getBoolean(513, false);
      self.doNotUseHTMLParagraphAutoSpacing = property.getBoolean__int_boolean(513, false);
      // java this.isTableIndentedByOldSTyle = compatMode.isTableIndentedByOldStyle();
      self.isTableIndentedByOldSTyle = compatMode.isTableIndentedByOldStyle();
      // java this.alignTablesRowByRow = compatMode.isBefore2013Mode() && property.getBoolean(389, false);
      self.alignTablesRowByRow = compatMode.isBefore2013Mode() && property.getBoolean__int_boolean(389, false);
      // java this.growAutoFit = compatMode.isBefore2013Mode() && property.getBoolean(524, false);
      self.growAutoFit = compatMode.isBefore2013Mode() && property.getBoolean__int_boolean(524, false);
      // java this.showEndNotesAtTheEndOfSection = FootEndNotePropCalc.shouldShowEndNotesAtTheEndOfSection(property);
      self.showEndNotesAtTheEndOfSection = property.getInt__int_int(641, 3) == 0;
      // java this.mirrorMargins = property.getBoolean(408, false);
      self.mirrorMargins = property.getBoolean__int_boolean(408, false);
    }

    // java public void update()
    pub fn update(&mut self) {
      // java PropertySet property = this.textDocument.getProperty();
      // The product binds the open document through textDocumentAlias, which is
      // the same object Java stores in textDocument.
      let loaded = self.textDocument_mut().map(|doc| {
        let property = doc.getProperty();
        let compatMode = property.getValue(542)
          .and_then(|v| v.as_any().downcast_ref::<Vec<crate::cn::wps::moffice::writer::data::value::CompatSetting::CompatSetting>>())
          .and_then(|settings| settings.iter().find(|s| s.getName().as_deref().is_some_and(|n| n.eq_ignore_ascii_case("compatibilityMode"))))
          .and_then(|setting| setting.getVal())
          .map(|value| WdCompatibilityMode::convertValue(&value))
          .unwrap_or(WdCompatibilityMode::wdWord2010);
        (property, compatMode)
      });
      if let Some((property, compatMode)) = loaded {
        self.updateFromProperty(&property, compatMode);
      } else {
        // java else { this.defTab = 400; ... this.compressLevel = 0; ... }
        self.init();
        self.isTableIndentedByOldSTyle = true;
        self.cachedColBalance = true;
      }
    }

    // java public void init()
    pub fn init(&mut self) {
      // java this.defTab = 400;
      self.defTab = 400;
      // java this.evenAndOddPagesDiff = false;
      self.evenAndOddPagesDiff = false;
      // java this.noTabHangInd = false;
      self.noTabHangInd = false;
      // java this.noNumTabHandInd = false;
      self.noNumTabHandInd = false;
      // java this.notBreakWrappedTables = false;
      self.notBreakWrappedTables = false;
      // java this.noAdjustLineHeightInTable = false;
      self.noAdjustLineHeightInTable = false;
      // java this.isShapeLayoutLikeWW8 = false;
      self.isShapeLayoutLikeWW8 = false;
      // java this.compressLevel = 0;
      self.compressLevel = 0;
      // java this.useAltKinsokuLineBreakRules = false;
      self.useAltKinsokuLineBreakRules = false;
      // java this.balanceSingleByteDoubleByteWidth = false;
      self.balanceSingleByteDoubleByteWidth = false;
      // java this.forbidPunctuationKerning = false;
      self.forbidPunctuationKerning = false;
      // java this.forbidEastAsianBreakRules = false;
      self.forbidEastAsianBreakRules = false;
      // java this.wrapTrailSpaces = false;
      self.wrapTrailSpaces = false;
      // java this.useWord97LineBreakRules = false;
      self.useWord97LineBreakRules = false;
      // java this.showEndNotesAtTheEndOfSection = false;
      self.showEndNotesAtTheEndOfSection = false;
      // java this.doNotUseHTMLParagraphAutoSpacing = false;
      self.doNotUseHTMLParagraphAutoSpacing = false;
      // java this.notMergeTypoText = false;
      self.notMergeTypoText = false;
    }

    // java public IEmojiService getEmojiService()
    pub fn getEmojiService(&mut self) -> Option<EmojiHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mEmojiService)
      if self.mEmojiService.is_none() {
        // java this.mEmojiService = EmojiServiceFactory.create();
        self.mEmojiService = EmojiServiceFactory::create();
      }
      // java return this.mEmojiService;
      return self.mEmojiService;
      */
    }

    // java public void updateTypoDocument(TypoDocument document)
    pub fn updateTypoDocument(&mut self, document: TypoDocument) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.typoDocument = document;
      self.typoDocument = document;
      */
    }

}

impl CacheCleaner {
    // java public void init(MainState mainState, DocumentEnv docEnv)
    pub fn bind(&mut self, main_state: *mut crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState) {
        self.mMainState = main_state;
    }

    // java CacheCleaner.init keeps both MainState and DocumentEnv.
    pub fn bind_env(&mut self, main_state: *mut crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState, doc_env: *mut DocumentEnv) {
        self.mMainState = main_state;
        self.mDocEnv = doc_env;
    }

    fn doc(&mut self) -> Option<&mut DocumentEnv> {
        if self.mDocEnv.is_null() { None } else { Some(unsafe { &mut *self.mDocEnv }) }
    }

    fn main(&mut self) -> Option<&mut crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState> {
        if self.mMainState.is_null() { None } else { Some(unsafe { &mut *self.mMainState }) }
    }

    // java private int getFootNotesPage(int curPage)
    fn getFootNotesPage(&mut self, mut cur_page: i32) -> i32 {
        let Some(main) = self.main() else { return 0; };
        let snapshot = &main.snapshot;
        let mut guard = 0i32;
        while cur_page != 0 && guard < 32 {
            let page_type = snapshot.get__int(cur_page);
            if page_type == 2 || page_type == 14 { break; }
            let next = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getPage__int_typosnapshot(cur_page, snapshot);
            if next == cur_page { break; }
            cur_page = next;
            guard += 1;
        }
        if cur_page != 0 { snapshot.get__int(cur_page + 19) } else { 0 }
    }

    // java private void adjustFootNotesPage(int footNotes)
    fn adjustFootNotesPage(&mut self, foot_notes: i32) {
        if foot_notes == 0 { return; }
        let Some(main) = self.main() else { return; };
        let snapshot = &mut main.snapshot;
        let parent_page = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getParent__int_typosnapshot(foot_notes, snapshot);
        let foot_notes_size = snapshot.get__int(foot_notes + 7);
        if foot_notes_size > 1 {
            let mut cur_top = 0i32;
            for i in 0..foot_notes_size {
                let cur_line = if i >= 0 && i < foot_notes_size { snapshot.get__int(foot_notes + 8 + i) } else { 0 };
                let top = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getTop__int_typosnapshot(cur_line, snapshot);
                let dy = cur_top.wrapping_sub(top);
                snapshot.extend__int_int(cur_line + 4, dy);
                snapshot.extend__int_int(cur_line + 6, dy);
                cur_top = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getBottom__int_typosnapshot(cur_line, snapshot);
            }
            let top = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getTop__int_typosnapshot(foot_notes, snapshot);
            snapshot.set__int_int(foot_notes + 6, top.wrapping_add(cur_top));
            let text_flow = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getTextFlow__int_typosnapshot(parent_page, snapshot);
            let mut max_height = if text_flow == 0 {
                crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getCoreHeight__int_typosnapshot(parent_page, snapshot)
            } else {
                let width = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getRight__int_typosnapshot(parent_page, snapshot)
                    - crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getLeft__int_typosnapshot(parent_page, snapshot);
                width
                    - crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getMarginTop__int_typosnapshot(parent_page, snapshot)
                    - crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getMarginBottom__int_typosnapshot(parent_page, snapshot)
            };
            if snapshot.get__int(parent_page) == 2 && text_flow == 0 {
                let footer_page = crate::cn::wps::moffice::writer::cache::TypoLayoutPage::TypoLayoutPage::getFooterPage(parent_page, snapshot);
                if footer_page != 0 {
                    let footer_top = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getTop__int_typosnapshot(footer_page, snapshot);
                    let margin_top = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getMarginTop__int_typosnapshot(parent_page, snapshot);
                    max_height = max_height.min(footer_top - margin_top);
                }
            }
            let top = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getTop__int_typosnapshot(foot_notes, snapshot);
            let dy = max_height.wrapping_sub(cur_top).wrapping_sub(top);
            snapshot.extend__int_int(foot_notes + 4, dy);
            snapshot.extend__int_int(foot_notes + 6, dy);
        } else if parent_page != 0 {
            snapshot.set__int_int(parent_page + 19, 0);
        }
    }

    // java private void recycleFootNote(int startCP, int endCP, int footNotesPage)
    fn recycleFootNote(&mut self, start_cp: i32, end_cp: i32, foot_notes_page: i32) {
        if let Some(main) = self.main() {
            if let Some(cache) = main.footEndNoteCache.as_mut() {
                cache.removeFootEndNoteCache(start_cp, end_cp);
            }
        }
        if foot_notes_page == 0 { return; }
        let size = {
            let Some(main) = self.main() else { return; };
            main.snapshot.get__int(foot_notes_page + 7)
        };
        let mut i = size - 1;
        while i >= 0 {
            let (foot_note, ref_cp) = {
                let Some(main) = self.main() else { return; };
                let snapshot = &main.snapshot;
                let foot_note = snapshot.get__int(foot_notes_page + 8 + i);
                let ref_cp = if foot_note == 0 { 0 } else { snapshot.getCPRef(snapshot.get__int(foot_note + 19)) };
                (foot_note, ref_cp)
            };
            if foot_note != 0 && ref_cp >= start_cp && ref_cp < end_cp {
                self.recyclePageContent(foot_note);
                if let Some(main) = self.main() {
                    let snapshot = &mut main.snapshot;
                    let size = snapshot.get__int(foot_notes_page + 7);
                    if i < size - 1 {
                        snapshot.copy__int_int_int(foot_notes_page + 8 + i + 1, foot_notes_page + 8 + i, size - i - 1);
                    }
                    snapshot.set__int_int(foot_notes_page + 7, size - 1);
                }
            }
            i -= 1;
        }
    }

    // java public void recycleRow(int row)
    pub fn recycleRow(&mut self, row: i32) {
        self.recycle_row(row, std::ptr::null_mut());
    }

    // java public void recycleRow(int row, TypoDrawingLayers needRecycleDrawingsInLayers)
    pub fn recycle_row(&mut self, row: i32, layers: *mut crate::cn::wps::moffice::writer::cache::TypoDrawingLayers::TypoDrawingLayers) {
        if row == 0 { return; }
        // java DrawingCacheTool.saveQuasiFloats: the quasi-float loop body is empty, so the snapshot is unchanged.
        self.save_quasi_floats(row);
        let cells = {
            let Some(main) = self.main() else { return; };
            let snapshot = &main.snapshot;
            let count = crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow::getCellsCount__int_typosnapshot(row, snapshot);
            (0..count).map(|i| crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow::getCell__int_int_typosnapshot(i, row, snapshot)).collect::<Vec<_>>()
        };
        for cell in cells {
            if !layers.is_null() {
                unsafe { (*layers).removeDrawinghsForParent(cell); }
            }
            self.recyclePageContent(cell);
        }
    }

    fn save_quasi_floats(&mut self, row: i32) {
        let Some(main) = self.main() else { return; };
        if !crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow::hasQuasiFloats__int_typosnapshot(row, &main.snapshot) {
            return;
        }
    }

    // java private void recycleColumns(int typoColumns)
    fn recycleColumns(&mut self, typo_columns: i32) {
        let columns = {
            let Some(main) = self.main() else { return; };
            let snapshot = &main.snapshot;
            let count = crate::cn::wps::moffice::writer::cache::TypoColumns::TypoColumns::getColumnsCount__int_typosnapshot(typo_columns, snapshot);
            (0..count).map(|i| {
                let column_page = crate::cn::wps::moffice::writer::cache::TypoColumns::TypoColumns::getColumnPage__int_int_typosnapshot(i, typo_columns, snapshot);
                let notes = snapshot.get__int(column_page + 19);
                let note_count = if notes == 0 { 0 } else { snapshot.get__int(notes + 7) };
                let notes_items = (0..note_count).map(|j| snapshot.get__int(notes + 8 + j)).collect::<Vec<_>>();
                (column_page, notes, notes_items)
            }).collect::<Vec<_>>()
        };
        for (column_page, notes, notes_items) in columns {
            for foot_note in notes_items {
                self.recyclePageContent(foot_note);
            }
            if notes != 0 {
                if let Some(main) = self.main() {
                    main.snapshot.set__int_int(column_page + 19, 0);
                }
            }
            self.recyclePageContent(column_page);
        }
    }

    // java public void recyclePageContent(int page)
    pub fn recyclePageContent(&mut self, page: i32) {
        if page == 0 { return; }
        let (lines, doc_type, items) = {
            let Some(main) = self.main() else { return; };
            let snapshot = &main.snapshot;
            let lines = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getLines__int_typosnapshot(page, snapshot);
            let doc_type = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getDocumentType__int_typosnapshot(page, snapshot);
            let size = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(lines, snapshot);
            let items = (0..size).map(|i| {
                let line = snapshot.get__int(lines + 3 + i);
                (snapshot.get__int(line), line)
            }).collect::<Vec<_>>();
            (lines, doc_type, items)
        };
        let mut foot_notes_page = if doc_type == 0 { self.getFootNotesPage(page) } else { 0 };
        for (ty, line) in items.into_iter().rev() {
            match ty {
                3 => {
                    let (start_cp, end_cp) = {
                        let Some(main) = self.main() else { return; };
                        let snapshot = &main.snapshot;
                        (
                            crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::getStartCP__int_typosnapshot(line, snapshot),
                            crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::getEndCP__int_typosnapshot(line, snapshot),
                        )
                    };
                    self.recycleFootNote(start_cp, end_cp, foot_notes_page);
                }
                5 => self.recycleRow(line),
                11 => self.recyclePageContent(line),
                13 => self.recycleColumns(line),
                _ => {}
            }
            if foot_notes_page != 0 {
                foot_notes_page = self.getFootNotesPage(page);
            }
        }
        if doc_type == 0 {
            let notes = self.getFootNotesPage(page);
            self.adjustFootNotesPage(notes);
        }
        if lines != 0 {
            if let Some(main) = self.main() {
                crate::cn::wps::moffice::writer::cache::TypoList::TypoList::setSize__int_int_typosnapshot(0, lines, &mut main.snapshot);
            }
        }
    }

    // java public void recycleDrawing(int remove, TypoDrawingLayers drawingLayers)
    pub fn recycleDrawing(&mut self, remove: i32, layers: *mut crate::cn::wps::moffice::writer::cache::TypoDrawingLayers::TypoDrawingLayers) {
        if remove == 0 { return; }
        if !layers.is_null() {
            unsafe { (*layers).removeDrawing(remove); }
        }
        let (children, textbox, is_table) = {
            let Some(main) = self.main() else { return; };
            let snapshot = &main.snapshot;
            let child_list = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getChildDrawings__int_typosnapshot(remove, snapshot);
            let size = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(child_list, snapshot);
            let children = (0..size).rev().map(|index| snapshot.get__int(child_list + 3 + index)).filter(|child| *child != 0).collect::<Vec<_>>();
            let textbox = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getTextbox__int_typosnapshot(remove, snapshot);
            let is_table = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::isTable__int_typosnapshot(remove, snapshot);
            (children, textbox, is_table)
        };
        for child in children {
            self.recycleDrawing(child, std::ptr::null_mut());
        }
        self.recyclePageContent(textbox);
        if !layers.is_null() && is_table {
            unsafe { (*layers).removeDrawinghsForParent(textbox); }
        }
    }

    // java public void cleanAfter(LayoutState env)
    pub fn cleanAfter(&mut self, env: &mut crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState) {
        let doc_ptr = env.document.filter(|pointer| !pointer.is_null());
        let doc_type = doc_ptr.map(|pointer| unsafe { (*pointer).getType__on_IDocument() }).unwrap_or(-1);
        let page_pointer = env.curPage.as_ref().map(|page| page.getPointer()).unwrap_or(0);
        let lines = env.curPage.as_ref().map(|page| page.getLines()).unwrap_or(0);
        let page_type = env.curPage.as_ref().map(|page| page.getType()).unwrap_or(0);
        let mut foot_notes_page = if doc_type == 0 { self.getFootNotesPage(page_pointer) } else { 0 };
        let limit = env.linePos;
        let mut need_del_wrap = 0i32;
        let cache_size = self.main().and_then(|main| main.footEndNoteCache.as_ref().map(|cache| cache.size())).unwrap_or(0);
        if cache_size > 0 {
            let last_line = {
                let Some(main) = self.main() else { return; };
                crate::cn::wps::moffice::writer::cache::TypoList::TypoList::last__int_typosnapshot(lines, &main.snapshot)
            };
            let (ty, wrap) = {
                let Some(main) = self.main() else { return; };
                let snapshot = &main.snapshot;
                (
                    crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj::getType__int_typosnapshot(last_line, snapshot),
                    crate::cn::wps::moffice::writer::cache::TypoTableRow::TypoTableRow::isWrap__int_typosnapshot(last_line, snapshot),
                )
            };
            if ty == 5 && wrap {
                if let Some(document) = doc_ptr {
                    let layers_ptr = self.main().and_then(|main| main.getDrawingLayers(unsafe { &*document }).map(|layers| layers as *mut crate::cn::wps::moffice::writer::cache::TypoDrawingLayers::TypoDrawingLayers)).unwrap_or(std::ptr::null_mut());
                    if !layers_ptr.is_null() {
                        let (start_cp, layout_page) = {
                            let Some(main) = self.main() else { return; };
                            (
                                crate::cn::wps::moffice::writer::cache::TypoRangeBase::TypoRangeBase::getStartCP__int_typosnapshot(last_line, &main.snapshot),
                                main.layoutPage.getPointer(),
                            )
                        };
                        let wrap_drawing = unsafe { (*layers_ptr).getWrapTableByCP(start_cp, layout_page) };
                        let drop_it = {
                            let Some(main) = self.main() else { return; };
                            wrap_drawing != 0 && crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::isNoKeepCacheWhenRollBack(wrap_drawing, &main.snapshot)
                        };
                        if drop_it { need_del_wrap = wrap_drawing; }
                    }
                }
            }
        }
        let deleted = {
            let Some(main) = self.main() else { return; };
            let snapshot = &main.snapshot;
            let size = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(lines, snapshot);
            let mut items = Vec::new();
            let mut index = size - 1;
            while index >= limit {
                let line = snapshot.get__int(lines + 3 + index);
                items.push((snapshot.get__int(line), line));
                index -= 1;
            }
            items
        };
        let layers_ptr = doc_ptr.and_then(|document| {
            self.main().and_then(|main| main.getDrawingLayers(unsafe { &*document }).map(|layers| layers as *mut crate::cn::wps::moffice::writer::cache::TypoDrawingLayers::TypoDrawingLayers))
        }).unwrap_or(std::ptr::null_mut());
        for (ty, line) in deleted {
            match ty {
                3 => {
                    let (start_cp, end_cp) = {
                        let Some(main) = self.main() else { return; };
                        let snapshot = &main.snapshot;
                        (
                            crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::getStartCP__int_typosnapshot(line, snapshot),
                            crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::getEndCP__int_typosnapshot(line, snapshot),
                        )
                    };
                    self.recycleFootNote(start_cp, end_cp, foot_notes_page);
                }
                5 => self.recycle_row(line, layers_ptr),
                11 => self.recyclePageContent(line),
                13 => self.recycleColumns(line),
                _ => {}
            }
            if foot_notes_page != 0 {
                foot_notes_page = self.getFootNotesPage(page_pointer);
            }
        }
        if let Some(main) = self.main() {
            crate::cn::wps::moffice::writer::cache::TypoList::TypoList::setSize__int_int_typosnapshot(limit, lines, &mut main.snapshot);
        }
        if env.hasEmbed() && env.typoPara.getEmbedSize() > 0 {
            env.typoPara.clearRedundantEmbeds(env.cp);
        }
        if let Some(document) = doc_ptr {
            if !layers_ptr.is_null() {
                let floats_page = if page_type == 14 {
                    self.main().map(|main| main.layoutPage.getPointer()).unwrap_or(0)
                } else {
                    page_pointer
                };
                let mut drawing_size = {
                    let Some(main) = self.main() else { return; };
                    let drawings = unsafe { (*layers_ptr).getDrawins() };
                    crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(drawings, &main.snapshot)
                };
                let mut index = 0i32;
                while index < drawing_size {
                    let drawings = unsafe { (*layers_ptr).getDrawins() };
                    let (drawing, parent, drawing_cp, is_wrap) = {
                        let Some(main) = self.main() else { return; };
                        let snapshot = &main.snapshot;
                        let drawing = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::get__int_int_typosnapshot(index, drawings, snapshot);
                        (
                            drawing,
                            crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getParent__int_typosnapshot(drawing, snapshot),
                            crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getCP__int_typosnapshot(drawing, snapshot),
                            crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::isWrap__int_typosnapshot(drawing, snapshot),
                        )
                    };
                    if parent != floats_page || drawing_cp < env.cp || (is_wrap && need_del_wrap != drawing) {
                        index += 1;
                        continue;
                    }
                    unsafe { (*layers_ptr).removeDrawing(drawing); }
                    self.keep_or_recycle_drawing(drawing, layers_ptr);
                    drawing_size -= 1;
                }
            }
            if doc_type == 0 {
                let notes = self.getFootNotesPage(page_pointer);
                self.adjustFootNotesPage(notes);
            }
        }
    }

    fn keep_or_recycle_drawing(&mut self, drawing: i32, layers: *mut crate::cn::wps::moffice::writer::cache::TypoDrawingLayers::TypoDrawingLayers) {
        let wrap_table = {
            let Some(main) = self.main() else { return; };
            {
                let drawing_type = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getDrawingType__int_typosnapshot(drawing, &main.snapshot);
                drawing_type == 7 || drawing_type == 8
            }
        };
        if wrap_table {
            self.recycleDrawing(drawing, layers);
            return;
        }
        let (cp, shape) = {
            let Some(main) = self.main() else { return; };
            let snapshot = &mut main.snapshot;
            let cp = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getCP__int_typosnapshot(drawing, snapshot);
            let shape = crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getShape__int_typosnapshot(drawing, snapshot);
            crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::setIntersectedWithText__int_typosnapshot(false, drawing, snapshot);
            (cp, shape)
        };
        let mut item = {
            let Some(doc) = self.doc() else {
                self.recycleDrawing(drawing, layers);
                return;
            };
            doc.pools.obtainDrawingCacheItem()
        };
        item.anchor = 0;
        item.cp = cp;
        item.drawing = drawing;
        item.shape = shape;
        item.hasFlowed = true;
        let Some(main) = self.main() else { return; };
        let list = &mut main.floatsCache;
        if list.last().is_some_and(|last| last.cp >= item.cp) {
            let mut insert_at = 0usize;
            for (index, stored) in list.iter().enumerate() {
                if stored.cp < item.cp { insert_at = index + 1; } else { break; }
            }
            list.insert(insert_at, item);
        } else {
            list.push(item);
        }
    }
}

#[derive(Default, Clone)]
pub struct CacheCleanerHolder;
#[derive(Default, Clone)]
pub struct PropToolFactoryHolder;
// rust: 去影子 —— fontStyleCacheFactory 直存真实类型（旧 Holder 已删）
#[derive(Default, Clone)]
pub struct LayoutToolsManagerHolder;
#[derive(Default, Clone)]
pub struct EmojiHolder;
#[derive(Default, Clone)]
pub struct PhoneViewFitInfoHolder;
#[derive(Default, Clone)]
pub struct LineBreakAnalyzerHolder;
#[derive(Default, Clone)]
pub struct CardPaperHolder;
