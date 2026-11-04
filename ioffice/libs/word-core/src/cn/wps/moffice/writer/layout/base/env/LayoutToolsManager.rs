// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.base.ViewMode
use crate::cn::wps::moffice::writer::base::ViewMode::ViewMode;
// Java: import cn.wps.moffice.writer.core.pool.Reusable
use crate::cn::wps::moffice::writer::core::pool::Reusable::Reusable;
// Java: import cn.wps.moffice.writer.core.pool.ReusableDefault
use crate::cn::wps::moffice::writer::core::pool::ReusableDefault::ReusableDefault;
// Java: import cn.wps.moffice.writer.layout.base.ParaBrcLayouter
use crate::cn::wps::moffice::writer::layout::base::ParaBrcLayouter::ParaBrcLayouter;
// Java: import cn.wps.moffice.writer.layout.base.graph.caps.AllCapsHandler
use crate::cn::wps::moffice::writer::layout::base::graph::caps::AllCapsHandler::AllCapsHandler;
// Java: import cn.wps.moffice.writer.layout.base.graph.caps.ICapsCallback
use crate::cn::wps::moffice::writer::layout::base::graph::caps::ICapsCallback::ICapsCallback;
// Java: import cn.wps.moffice.writer.layout.base.graph.caps.ICapsHandler
use crate::cn::wps::moffice::writer::layout::base::graph::caps::ICapsHandler::ICapsHandler;
// Java: import cn.wps.moffice.writer.layout.base.graph.caps.SmallCapsHandler
use crate::cn::wps::moffice::writer::layout::base::graph::caps::SmallCapsHandler::SmallCapsHandler;
// Java: import cn.wps.moffice.writer.layout.base.graph.symbolconvert.ExtraConvert
use crate::cn::wps::moffice::writer::layout::base::graph::symbolconvert::ExtraConvert::ExtraConvert;
// Java: import cn.wps.moffice.writer.layout.base.graph.symbolconvert.ISymbolConvert
use crate::cn::wps::moffice::writer::layout::base::graph::symbolconvert::ISymbolConvert::ISymbolConvert;
// Java: import cn.wps.moffice.writer.layout.base.graph.symbolconvert.KingsoftConfettiConverter
use crate::cn::wps::moffice::writer::layout::base::graph::symbolconvert::KingsoftConfettiConverter::KingsoftConfettiConverter;
// Java: import cn.wps.moffice.writer.layout.base.graph.symbolconvert.KingsoftExtraConverter
use crate::cn::wps::moffice::writer::layout::base::graph::symbolconvert::KingsoftExtraConverter::KingsoftExtraConverter;
// Java: import cn.wps.moffice.writer.layout.base.graph.symbolconvert.KingsoftSignConverter
use crate::cn::wps::moffice::writer::layout::base::graph::symbolconvert::KingsoftSignConverter::KingsoftSignConverter;
// Java: import cn.wps.moffice.writer.layout.base.graph.symbolconvert.SymbolConvert
use crate::cn::wps::moffice::writer::layout::base::graph::symbolconvert::SymbolConvert::SymbolConvert;
// Java: import cn.wps.moffice.writer.layout.base.graph.symbolconvert.WebdingsConvert
use crate::cn::wps::moffice::writer::layout::base::graph::symbolconvert::WebdingsConvert::WebdingsConvert;
// Java: import cn.wps.moffice.writer.layout.base.graph.symbolconvert.WingdingsConvert
use crate::cn::wps::moffice::writer::layout::base::graph::symbolconvert::WingdingsConvert::WingdingsConvert;
// Java: import cn.wps.moffice.writer.layout.base.line.TabHandler
use crate::cn::wps::moffice::writer::layout::base::line::TabHandler::TabHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.TextOpCollector
use crate::cn::wps::moffice::writer::layout::base::line::TextOpCollector::TextOpCollector;
// Java: import cn.wps.moffice.writer.layout.base.line.align.AlignBaselineHandler
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignBaselineHandler::AlignBaselineHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.align.AlignBothHandler
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignBothHandler::AlignBothHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.align.AlignBottomHandler
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignBottomHandler::AlignBottomHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.align.AlignCenterHandler
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignCenterHandler::AlignCenterHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.align.AlignDistributeHandler
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignDistributeHandler::AlignDistributeHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.align.AlignLeftHandler
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignLeftHandler::AlignLeftHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.align.AlignMiddleHandler
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignMiddleHandler::AlignMiddleHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.align.AlignRightHandler
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignRightHandler::AlignRightHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.align.AlignThaiDistributeHandler
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignThaiDistributeHandler::AlignThaiDistributeHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.align.AlignTopHandler
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignTopHandler::AlignTopHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.align.FitTextHandler
use crate::cn::wps::moffice::writer::layout::base::line::align::FitTextHandler::FitTextHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.align.IHAlignHandler
use crate::cn::wps::moffice::writer::layout::base::line::align::IHAlignHandler::IHAlignHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.align.IVAlignHandler
use crate::cn::wps::moffice::writer::layout::base::line::align::IVAlignHandler::IVAlignHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.IWhiteSpaceHandler
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::IWhiteSpaceHandler::IWhiteSpaceHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.KerningHandler
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::KerningHandler::KerningHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.WhiteSpaceForPhoneHandler
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::WhiteSpaceForPhoneHandler::WhiteSpaceForPhoneHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.WhiteSpaceHandler
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::WhiteSpaceHandler::WhiteSpaceHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.autoSpace.AutoSpaceDEHandler
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::autoSpace::AutoSpaceDEHandler::AutoSpaceDEHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.autoSpace.AutoSpaceDEWithMathOpHandler
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::autoSpace::AutoSpaceDEWithMathOpHandler::AutoSpaceDEWithMathOpHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.autoSpace.AutoSpaceDNHandler
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::autoSpace::AutoSpaceDNHandler::AutoSpaceDNHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.autoSpace.AutoSpaceDNWithMathOpHandler
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::autoSpace::AutoSpaceDNWithMathOpHandler::AutoSpaceDNWithMathOpHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.autoSpace.AutoSpaceDNandDEHandler
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::autoSpace::AutoSpaceDNandDEHandler::AutoSpaceDNandDEHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.autoSpace.AutoSpaceDNandDEWithMathOpHandler
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::autoSpace::AutoSpaceDNandDEWithMathOpHandler::AutoSpaceDNandDEWithMathOpHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.autoSpace.IAutoSpaceHandler
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::autoSpace::IAutoSpaceHandler::IAutoSpaceHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.autoSpace.MahtOpSpaceHandler
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::autoSpace::MahtOpSpaceHandler::MahtOpSpaceHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.punctSpace.CompressPunctSpaceHandler
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::punctSpace::CompressPunctSpaceHandler::CompressPunctSpaceHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charSpace.punctSpace.IPunctSpaceHandler
use crate::cn::wps::moffice::writer::layout::base::line::charSpace::punctSpace::IPunctSpaceHandler::IPunctSpaceHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.charborder.CharBorderHandler
use crate::cn::wps::moffice::writer::layout::base::line::charborder::CharBorderHandler::CharBorderHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.docfield.DocFieldCollector
use crate::cn::wps::moffice::writer::layout::base::line::docfield::DocFieldCollector::DocFieldCollector;
// Java: import cn.wps.moffice.writer.layout.base.line.drawPos.FastDrawPosHandler
use crate::cn::wps::moffice::writer::layout::base::line::drawPos::FastDrawPosHandler::FastDrawPosHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.drawPos.IDrawPosHandler
use crate::cn::wps::moffice::writer::layout::base::line::drawPos::IDrawPosHandler::IDrawPosHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.drawPos.NormalDrawPosHandler
use crate::cn::wps::moffice::writer::layout::base::line::drawPos::NormalDrawPosHandler::NormalDrawPosHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.drawPos.RotationFarEastDrawPosHandler
use crate::cn::wps::moffice::writer::layout::base::line::drawPos::RotationFarEastDrawPosHandler::RotationFarEastDrawPosHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.grid.CharGridHandler
use crate::cn::wps::moffice::writer::layout::base::line::grid::CharGridHandler::CharGridHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.grid.EnforceGridHandler
use crate::cn::wps::moffice::writer::layout::base::line::grid::EnforceGridHandler::EnforceGridHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.grid.IGridHandler
use crate::cn::wps::moffice::writer::layout::base::line::grid::IGridHandler::IGridHandler;
// Java: import cn.wps.moffice.writer.layout.base.line.style.BordersCollector
use crate::cn::wps::moffice::writer::layout::base::line::style::BordersCollector::BordersCollector;
// Java: import cn.wps.moffice.writer.layout.base.line.style.HighlightCollector
use crate::cn::wps::moffice::writer::layout::base::line::style::HighlightCollector::HighlightCollector;
// Java: import cn.wps.moffice.writer.layout.base.line.style.SHDCollector
use crate::cn::wps::moffice::writer::layout::base::line::style::SHDCollector::SHDCollector;
// Java: import cn.wps.moffice.writer.layout.base.line.style.StrikeThroughCollector
use crate::cn::wps::moffice::writer::layout::base::line::style::StrikeThroughCollector::StrikeThroughCollector;
// Java: import cn.wps.moffice.writer.layout.base.line.style.UnderlineCollector
use crate::cn::wps::moffice::writer::layout::base::line::style::UnderlineCollector::UnderlineCollector;
// Java: import cn.wps.moffice.writer.layout.base.math.AlignMathParaHandler
use crate::cn::wps::moffice::writer::layout::base::math::AlignMathParaHandler::AlignMathParaHandler;
// Java: import cn.wps.moffice.writer.layout.base.math.AlignOMathParaHandler
use crate::cn::wps::moffice::writer::layout::base::math::AlignOMathParaHandler::AlignOMathParaHandler;
// Java: import cn.wps.moffice.writer.layout.base.table.width.FastGraphDefaut
use crate::cn::wps::moffice::writer::layout::base::table::width::FastGraphDefaut::FastGraphDefaut;
// Java: import cn.wps.moffice.writer.layout.base.table.width.IFastGraph
use crate::cn::wps::moffice::writer::layout::base::table::width::IFastGraph::IFastGraph;
// Java: import cn.wps.moffice.writer.layout.base.table.width.WidthAnalyzer
use crate::cn::wps::moffice::writer::layout::base::table::width::WidthAnalyzer::WidthAnalyzer;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;

// java public class LayoutToolsManager implements Reusable
#[derive(Default)]
pub struct LayoutToolsManager {
    // java private IDrawPosHandler mFastDrawPosHandler;
    mFastDrawPosHandler: Option<IDrawPosHandlerHolder>,
    // java private IDrawPosHandler mNormalDrawPosHandler;
    mNormalDrawPosHandler: Option<IDrawPosHandlerHolder>,
    // java private IDrawPosHandler mRotationFarEastDrawPosHandler;
    mRotationFarEastDrawPosHandler: Option<IDrawPosHandlerHolder>,
    // java private UnderlineCollector mUnderlienCollector;
    mUnderlienCollector: Option<UnderlineCollectorHolder>,
    // java private StrikeThroughCollector mStrikeThroughCollector;
    mStrikeThroughCollector: Option<StrikeThroughCollectorHolder>,
    // java private SHDCollector mSHDCollector;
    mSHDCollector: Option<SHDCollectorHolder>,
    // java private BordersCollector mBorderCollector;
    mBorderCollector: Option<BordersCollectorHolder>,
    // java private HighlightCollector mHighlightCollector;
    mHighlightCollector: Option<HighlightCollectorHolder>,
    // java private TextOpCollector mTextOpCollector;
    mTextOpCollector: Option<TextOpCollectorHolder>,
    // java private SmallCapsHandler mSmallCapsHandler;
    mSmallCapsHandler: Option<SmallCapsHandlerHolder>,
    // java private AllCapsHandler mAllCapsHandler;
    mAllCapsHandler: Option<AllCapsHandlerHolder>,
    // java private AlignLeftHandler mAlignLeftHandler;
    mAlignLeftHandler: Option<AlignLeftHandlerHolder>,
    // java private AlignCenterHandler mAlignCenterHandler;
    mAlignCenterHandler: Option<AlignCenterHandlerHolder>,
    // java private AlignRightHandler mAlignRightHandler;
    mAlignRightHandler: Option<AlignRightHandlerHolder>,
    // java private AlignBothHandler mAlignBothHandler;
    mAlignBothHandler: Option<AlignBothHandlerHolder>,
    // java private AlignDistributeHandler mAlignDistributeHandler;
    mAlignDistributeHandler: Option<AlignDistributeHandlerHolder>,
    // java private AlignThaiDistributeHandler mAlignThaiDistributeHandler;
    mAlignThaiDistributeHandler: Option<AlignThaiDistributeHandlerHolder>,
    // java private AlignTopHandler mAlignTopHandler;
    mAlignTopHandler: Option<AlignTopHandlerHolder>,
    // java private AlignMiddleHandler mAlignMiddleHandler;
    mAlignMiddleHandler: Option<AlignMiddleHandlerHolder>,
    // java private AlignBottomHandler mAlignBottomHandler;
    mAlignBottomHandler: Option<AlignBottomHandlerHolder>,
    // java private AlignBaselineHandler mAlignBaselineHandler;
    mAlignBaselineHandler: Option<AlignBaselineHandlerHolder>,
    // java private WhiteSpaceHandler mWhiteSpaceHandler;
    mWhiteSpaceHandler: Option<WhiteSpaceHandlerHolder>,
    // java private WhiteSpaceForPhoneHandler mWhiteSpaceForPhoneHandler;
    mWhiteSpaceForPhoneHandler: Option<WhiteSpaceForPhoneHandlerHolder>,
    // java private AutoSpaceDEHandler mAutoSpaceDEHandler;
    mAutoSpaceDEHandler: Option<AutoSpaceDEHandlerHolder>,
    // java private AutoSpaceDEWithMathOpHandler mAutoSpaceDEWithMathOpHandler;
    mAutoSpaceDEWithMathOpHandler: Option<AutoSpaceDEWithMathOpHandlerHolder>,
    // java private AutoSpaceDNandDEHandler mAutoSpaceDNandDEHandler;
    mAutoSpaceDNandDEHandler: Option<AutoSpaceDNandDEHandlerHolder>,
    // java private AutoSpaceDNandDEWithMathOpHandler mAutoSpaceDNandDEWithMathOpHandler;
    mAutoSpaceDNandDEWithMathOpHandler: Option<AutoSpaceDNandDEWithMathOpHandlerHolder>,
    // java private AutoSpaceDNHandler mAutoSpaceDNHandler;
    mAutoSpaceDNHandler: Option<AutoSpaceDNHandlerHolder>,
    // java private AutoSpaceDNWithMathOpHandler mAutoSpaceDNWithMathOpHandler;
    mAutoSpaceDNWithMathOpHandler: Option<AutoSpaceDNWithMathOpHandlerHolder>,
    // java private MahtOpSpaceHandler mMahtOpSpaceHandler;
    mMahtOpSpaceHandler: Option<MahtOpSpaceHandlerHolder>,
    // java private CompressPunctSpaceHandler mCompressPunctSpaceHandler;
    mCompressPunctSpaceHandler: Option<CompressPunctSpaceHandlerHolder>,
    // java private EnforceGridHandler mEnforceGridHandler;
    mEnforceGridHandler: Option<EnforceGridHandlerHolder>,
    // java private CharGridHandler mCharGridHandler;
    mCharGridHandler: Option<CharGridHandlerHolder>,
    // java private TabHandler mTabHandler;
    mTabHandler: Option<TabHandlerHolder>,
    // java private AlignMathParaHandler mAlignMathParaHandler;
    mAlignMathParaHandler: Option<AlignMathParaHandlerHolder>,
    // java private IHAlignHandler mAlignOMathParaHandler;
    mAlignOMathParaHandler: Option<IHAlignHandlerHolder>,
    // java private ISymbolConvert mWingdingsConvert;
    mWingdingsConvert: Option<ISymbolConvertHolder>,
    // java private ISymbolConvert mWebdingsConvert;
    mWebdingsConvert: Option<ISymbolConvertHolder>,
    // java private ISymbolConvert mExtraConvert;
    mExtraConvert: Option<ISymbolConvertHolder>,
    // java private ISymbolConvert mSymbolConvert;
    mSymbolConvert: Option<ISymbolConvertHolder>,
    // java private ISymbolConvert mKinsoftConfettiConvert;
    mKinsoftConfettiConvert: Option<ISymbolConvertHolder>,
    // java private ISymbolConvert mKinsoftExtraConvert;
    mKinsoftExtraConvert: Option<ISymbolConvertHolder>,
    // java private ISymbolConvert mKinsoftSignConvert;
    mKinsoftSignConvert: Option<ISymbolConvertHolder>,
    // java private ParaBrcLayouter mParaBrcLayouter;
    mParaBrcLayouter: Option<ParaBrcLayouterHolder>,
    // java private KerningHandler mKerningHandler;
    mKerningHandler: Option<KerningHandlerHolder>,
    // java private FitTextHandler mFitTextHandler;
    mFitTextHandler: Option<FitTextHandlerHolder>,
    // java private CharBorderHandler mCharBorderHandler;
    mCharBorderHandler: Option<CharBorderHandlerHolder>,
    // java private WidthAnalyzer mWidthAnalyzer;
    mWidthAnalyzer: Option<WidthAnalyzerHolder>,
    // java private IFastGraph mDefaultFastGraph;
    mDefaultFastGraph: Option<IFastGraphHolder>,
    // java private DocFieldCollector mDocFieldCollect;
    mDocFieldCollect: Option<DocFieldCollectorHolder>,
    // java private DocumentEnv mDocEnv;
    mDocEnv: Option<DocumentEnvHolder>,
    // java private ViewEnv mViewEnv;
    mViewEnv: Option<ViewEnvHolder>,
}

// java public interface Reusable
impl Reusable for LayoutToolsManager {
    // java public boolean reuseClean()
    fn reuseClean(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ReusableDefault.reuseCleanChilds(this.mFastDrawPosHandler, this.mNormalDrawPosHandler, this.mRotationFarEastDrawPosHandler, this.mUnderlienCollector, this.mStrikeThroughCollector, this.mSHDCollector, this.mBorderCollector, this.mHighlightCollector, this.mTextOpCollector, this.mSmallCapsHandler, this.mAllCapsHandler, this.mAlignCenterHandler, this.mAlignRightHandler, this.mAlignBothHandler, this.mAlignDistributeHandler, this.mAlignLeftHandler, this.mWhiteSpaceHandler, this.mMahtOpSpaceHandler, this.mAutoSpaceDNandDEWithMathOpHandler, this.mAutoSpaceDNandDEHandler, this.mAutoSpaceDNWithMathOpHandler, this.mAutoSpaceDNHandler, this.mAutoSpaceDEWithMathOpHandler, this.mAutoSpaceDEHandler, this.mCompressPunctSpaceHandler, this.mEnforceGridHandler, this.mCharGridHandler, this.mTabHandler, this.mAlignMathParaHandler, (Reusable)this.mAlignOMathParaHandler, this.mParaBrcLayouter, this.mKerningHandler, this.mFitTextHandler, this.mWidthAnalyzer, (Reusable)this.mDefaultFastGraph, this.mDocFieldCollect, this.mCharBorderHandler, this.mWhiteSpaceForPhoneHandler);
      return ReusableDefault::reuseCleanChilds(self.mFastDrawPosHandler, self.mNormalDrawPosHandler, self.mRotationFarEastDrawPosHandler, self.mUnderlienCollector, self.mStrikeThroughCollector, self.mSHDCollector, self.mBorderCollector, self.mHighlightCollector, self.mTextOpCollector, self.mSmallCapsHandler, self.mAllCapsHandler, self.mAlignCenterHandler, self.mAlignRightHandler, self.mAlignBothHandler, self.mAlignDistributeHandler, self.mAlignLeftHandler, self.mWhiteSpaceHandler, self.mMahtOpSpaceHandler, self.mAutoSpaceDNandDEWithMathOpHandler, self.mAutoSpaceDNandDEHandler, self.mAutoSpaceDNWithMathOpHandler, self.mAutoSpaceDNHandler, self.mAutoSpaceDEWithMathOpHandler, self.mAutoSpaceDEHandler, self.mCompressPunctSpaceHandler, self.mEnforceGridHandler, self.mCharGridHandler, self.mTabHandler, self.mAlignMathParaHandler, todo!("cast → Box<dyn Reusable>"), self.mParaBrcLayouter, self.mKerningHandler, self.mFitTextHandler, self.mWidthAnalyzer, todo!("cast → Box<dyn Reusable>"), self.mDocFieldCollect, self.mCharBorderHandler, self.mWhiteSpaceForPhoneHandler);
      */
    }

    // java public void reuseInit()
    fn reuseInit(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ReusableDefault.reuseInitChilds(this.mFastDrawPosHandler, this.mNormalDrawPosHandler, this.mRotationFarEastDrawPosHandler, this.mUnderlienCollector, this.mStrikeThroughCollector, this.mSHDCollector, this.mBorderCollector, this.mHighlightCollector, this.mTextOpCollector, this.mSmallCapsHandler, this.mAllCapsHandler, this.mAlignCenterHandler, this.mAlignRightHandler, this.mAlignBothHandler, this.mAlignDistributeHandler, this.mAlignLeftHandler, this.mWhiteSpaceHandler, this.mMahtOpSpaceHandler, this.mAutoSpaceDNandDEWithMathOpHandler, this.mAutoSpaceDNandDEHandler, this.mAutoSpaceDNWithMathOpHandler, this.mAutoSpaceDNHandler, this.mAutoSpaceDEWithMathOpHandler, this.mAutoSpaceDEHandler, this.mCompressPunctSpaceHandler, this.mEnforceGridHandler, this.mCharGridHandler, this.mTabHandler, this.mAlignMathParaHandler, (Reusable)this.mAlignOMathParaHandler, this.mParaBrcLayouter, this.mKerningHandler, this.mFitTextHandler, this.mWidthAnalyzer, (Reusable)this.mDefaultFastGraph, this.mDocFieldCollect, this.mCharBorderHandler, this.mWhiteSpaceForPhoneHandler);
      ReusableDefault::reuseInitChilds(self.mFastDrawPosHandler, self.mNormalDrawPosHandler, self.mRotationFarEastDrawPosHandler, self.mUnderlienCollector, self.mStrikeThroughCollector, self.mSHDCollector, self.mBorderCollector, self.mHighlightCollector, self.mTextOpCollector, self.mSmallCapsHandler, self.mAllCapsHandler, self.mAlignCenterHandler, self.mAlignRightHandler, self.mAlignBothHandler, self.mAlignDistributeHandler, self.mAlignLeftHandler, self.mWhiteSpaceHandler, self.mMahtOpSpaceHandler, self.mAutoSpaceDNandDEWithMathOpHandler, self.mAutoSpaceDNandDEHandler, self.mAutoSpaceDNWithMathOpHandler, self.mAutoSpaceDNHandler, self.mAutoSpaceDEWithMathOpHandler, self.mAutoSpaceDEHandler, self.mCompressPunctSpaceHandler, self.mEnforceGridHandler, self.mCharGridHandler, self.mTabHandler, self.mAlignMathParaHandler, todo!("cast → Box<dyn Reusable>"), self.mParaBrcLayouter, self.mKerningHandler, self.mFitTextHandler, self.mWidthAnalyzer, todo!("cast → Box<dyn Reusable>"), self.mDocFieldCollect, self.mCharBorderHandler, self.mWhiteSpaceForPhoneHandler);
      */
    }

}

// java public class LayoutToolsManager implements Reusable
impl LayoutToolsManager {
    // java public  LayoutToolsManager(DocumentEnv documentEnv, ViewEnv viewEnv)
    pub fn LayoutToolsManager(documentEnv: &DocumentEnv, viewEnv: &ViewEnv) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mDocEnv = documentEnv;
      Self { mFastDrawPosHandler: Default::default(), mNormalDrawPosHandler: Default::default(), mRotationFarEastDrawPosHandler: Default::default(), mUnderlienCollector: Default::default(), mStrikeThroughCollector: Default::default(), mSHDCollector: Default::default(), mBorderCollector: Default::default(), mHighlightCollector: Default::default(), mTextOpCollector: Default::default(), mSmallCapsHandler: Default::default(), mAllCapsHandler: Default::default(), mAlignLeftHandler: Default::default(), mAlignCenterHandler: Default::default(), mAlignRightHandler: Default::default(), mAlignBothHandler: Default::default(), mAlignDistributeHandler: Default::default(), mAlignThaiDistributeHandler: Default::default(), mAlignTopHandler: Default::default(), mAlignMiddleHandler: Default::default(), mAlignBottomHandler: Default::default(), mAlignBaselineHandler: Default::default(), mWhiteSpaceHandler: Default::default(), mWhiteSpaceForPhoneHandler: Default::default(), mAutoSpaceDEHandler: Default::default(), mAutoSpaceDEWithMathOpHandler: Default::default(), mAutoSpaceDNandDEHandler: Default::default(), mAutoSpaceDNandDEWithMathOpHandler: Default::default(), mAutoSpaceDNHandler: Default::default(), mAutoSpaceDNWithMathOpHandler: Default::default(), mMahtOpSpaceHandler: Default::default(), mCompressPunctSpaceHandler: Default::default(), mEnforceGridHandler: Default::default(), mCharGridHandler: Default::default(), mTabHandler: Default::default(), mAlignMathParaHandler: Default::default(), mAlignOMathParaHandler: Default::default(), mWingdingsConvert: Default::default(), mWebdingsConvert: Default::default(), mExtraConvert: Default::default(), mSymbolConvert: Default::default(), mKinsoftConfettiConvert: Default::default(), mKinsoftExtraConvert: Default::default(), mKinsoftSignConvert: Default::default(), mParaBrcLayouter: Default::default(), mKerningHandler: Default::default(), mFitTextHandler: Default::default(), mCharBorderHandler: Default::default(), mWidthAnalyzer: Default::default(), mDefaultFastGraph: Default::default(), mDocFieldCollect: Default::default(), mDocEnv: documentEnv, mViewEnv: viewEnv }
      // java this.mViewEnv = viewEnv;
      */
    }

    // java public IDrawPosHandler getFastDrawPosHandler()
    pub fn getFastDrawPosHandler(&self) -> Option<IDrawPosHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mFastDrawPosHandler)
      if self.mFastDrawPosHandler.is_none() {
        // java this.mFastDrawPosHandler = new FastDrawPosHandler();
        self.mFastDrawPosHandler = FastDrawPosHandler::new();
      }
      // java return this.mFastDrawPosHandler;
      return self.mFastDrawPosHandler;
      */
    }

    // java public IDrawPosHandler getNormalDrawPosHandler()
    pub fn getNormalDrawPosHandler(&self) -> Option<IDrawPosHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mNormalDrawPosHandler)
      if self.mNormalDrawPosHandler.is_none() {
        // java this.mNormalDrawPosHandler = new NormalDrawPosHandler();
        self.mNormalDrawPosHandler = NormalDrawPosHandler::new();
      }
      // java return this.mNormalDrawPosHandler;
      return self.mNormalDrawPosHandler;
      */
    }

    // java public IDrawPosHandler getRotationFarEastDrawPosHandler()
    pub fn getRotationFarEastDrawPosHandler(&self) -> Option<IDrawPosHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mRotationFarEastDrawPosHandler)
      if self.mRotationFarEastDrawPosHandler.is_none() {
        // java this.mRotationFarEastDrawPosHandler = new RotationFarEastDrawPosHandler();
        self.mRotationFarEastDrawPosHandler = RotationFarEastDrawPosHandler::new();
      }
      // java return this.mRotationFarEastDrawPosHandler;
      return self.mRotationFarEastDrawPosHandler;
      */
    }

    // java public UnderlineCollector getUnderlineCollector()
    pub fn getUnderlineCollector(&self) -> Option<UnderlineCollectorHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mUnderlienCollector)
      if self.mUnderlienCollector.is_none() {
        // java this.mUnderlienCollector = new UnderlineCollector();
        self.mUnderlienCollector = UnderlineCollector::new();
      }
      // java return this.mUnderlienCollector;
      return self.mUnderlienCollector;
      */
    }

    // java public StrikeThroughCollector getStrikeThroughCollector()
    pub fn getStrikeThroughCollector(&self) -> Option<StrikeThroughCollectorHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mStrikeThroughCollector)
      if self.mStrikeThroughCollector.is_none() {
        // java this.mStrikeThroughCollector = new StrikeThroughCollector();
        self.mStrikeThroughCollector = StrikeThroughCollector::new();
      }
      // java return this.mStrikeThroughCollector;
      return self.mStrikeThroughCollector;
      */
    }

    // java public SHDCollector getSHDCollector()
    pub fn getSHDCollector(&self) -> Option<SHDCollectorHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mSHDCollector)
      if self.mSHDCollector.is_none() {
        // java this.mSHDCollector = new SHDCollector();
        self.mSHDCollector = SHDCollector::new();
      }
      // java return this.mSHDCollector;
      return self.mSHDCollector;
      */
    }

    // java public BordersCollector getBorderCollector()
    pub fn getBorderCollector(&self) -> Option<BordersCollectorHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mBorderCollector)
      if self.mBorderCollector.is_none() {
        // java this.mBorderCollector = new BordersCollector();
        self.mBorderCollector = BordersCollector::new();
      }
      // java return this.mBorderCollector;
      return self.mBorderCollector;
      */
    }

    // java public HighlightCollector getHighlightCollector()
    pub fn getHighlightCollector(&self) -> Option<HighlightCollectorHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mHighlightCollector)
      if self.mHighlightCollector.is_none() {
        // java this.mHighlightCollector = new HighlightCollector();
        self.mHighlightCollector = HighlightCollector::new();
      }
      // java return this.mHighlightCollector;
      return self.mHighlightCollector;
      */
    }

    // java public TextOpCollector getTextOpCollector()
    pub fn getTextOpCollector(&self) -> Option<TextOpCollectorHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mTextOpCollector)
      if self.mTextOpCollector.is_none() {
        // java this.mTextOpCollector = new TextOpCollector(this.mDocEnv, this.mViewEnv);
        self.mTextOpCollector = TextOpCollector::new(self.mDocEnv, self.mViewEnv);
      }
      // java return this.mTextOpCollector;
      return self.mTextOpCollector;
      */
    }

    // java public ICapsHandler getCapsHandler(PropertySet prop, ICapsCallback mCapsCallback)
    pub fn getCapsHandler(&self, prop: Option<PropertySetHolder>, mCapsCallback: Option<ICapsCallbackHolder>) -> Option<ICapsHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (prop.getBoolean(16, false))
      if prop.getBoolean(16, false) {
        // java if (null == this.mSmallCapsHandler)
        if self.mSmallCapsHandler.is_none() {
          // java this.mSmallCapsHandler = new SmallCapsHandler();
          self.mSmallCapsHandler = SmallCapsHandler::new();
        }
        // java this.mSmallCapsHandler.setCallback(mCapsCallback);
        self.mSmallCapsHandler.setCallback(mCapsCallback);
        // java return this.mSmallCapsHandler;
        return { let __x: Option<ICapsHandlerHolder> = todo!("no align strategy matched"); __x };
      }
      // java else if (prop.getBoolean(17, false))
      else if prop.getBoolean(17, false) {
        // java if (null == this.mAllCapsHandler)
        if self.mAllCapsHandler.is_none() {
          // java this.mAllCapsHandler = new AllCapsHandler();
          self.mAllCapsHandler = AllCapsHandler::new();
        }
        // java this.mAllCapsHandler.setCallback(mCapsCallback);
        self.mAllCapsHandler.setCallback(mCapsCallback);
        // java return this.mAllCapsHandler;
        return { let __x: Option<ICapsHandlerHolder> = todo!("no align strategy matched"); __x };
      }
      // java else
      else {
        // java return null;
        return None;
      }
      */
    }

    // java public IHAlignHandler getHAlignHandler(int alignment)
    pub fn getHAlignHandler(&self, alignment: i32) -> Option<IHAlignHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java switch (alignment)
      match alignment {
        // java case 1:
        1 => {
          // java if (null == this.mAlignCenterHandler)
          if self.mAlignCenterHandler.is_none() {
            // java this.mAlignCenterHandler = new AlignCenterHandler();
            self.mAlignCenterHandler = AlignCenterHandler::new();
          }
          // java return this.mAlignCenterHandler;
          return { let __x: Option<IHAlignHandlerHolder> = todo!("no align strategy matched"); __x };
        }
        // java case 2:
        2 => {
          // java if (null == this.mAlignRightHandler)
          if self.mAlignRightHandler.is_none() {
            // java this.mAlignRightHandler = new AlignRightHandler();
            self.mAlignRightHandler = AlignRightHandler::new();
          }
          // java return this.mAlignRightHandler;
          return { let __x: Option<IHAlignHandlerHolder> = todo!("no align strategy matched"); __x };
        }
        // java case 3: case 8:
        3 | 8 => {
          // java if (null == this.mAlignBothHandler)
          if self.mAlignBothHandler.is_none() {
            // java this.mAlignBothHandler = new AlignBothHandler();
            self.mAlignBothHandler = AlignBothHandler::new();
          }
          // java return this.mAlignBothHandler;
          return { let __x: Option<IHAlignHandlerHolder> = todo!("no align strategy matched"); __x };
        }
        // java case 4: case 5:
        4 | 5 => {
          // java if (null == this.mAlignDistributeHandler)
          if self.mAlignDistributeHandler.is_none() {
            // java this.mAlignDistributeHandler = new AlignDistributeHandler();
            self.mAlignDistributeHandler = AlignDistributeHandler::new();
          }
          // java return this.mAlignDistributeHandler;
          return { let __x: Option<IHAlignHandlerHolder> = todo!("no align strategy matched"); __x };
        }
        // java case 6: case 7: default:
        6 | 7 | _ => {
          // java if (null == this.mAlignLeftHandler)
          if self.mAlignLeftHandler.is_none() {
            // java this.mAlignLeftHandler = new AlignLeftHandler();
            self.mAlignLeftHandler = AlignLeftHandler::new();
          }
          // java return this.mAlignLeftHandler;
          return { let __x: Option<IHAlignHandlerHolder> = todo!("no align strategy matched"); __x };
        }
        // java case 9:
        9 => {
          // java if (null == this.mAlignThaiDistributeHandler)
          if self.mAlignThaiDistributeHandler.is_none() {
            // java this.mAlignThaiDistributeHandler = new AlignThaiDistributeHandler();
            self.mAlignThaiDistributeHandler = AlignThaiDistributeHandler::new();
          }
          // java return this.mAlignThaiDistributeHandler;
          return { let __x: Option<IHAlignHandlerHolder> = todo!("no align strategy matched"); __x };
        }
      }
      */
    }

    // java public IVAlignHandler getVAlignHandler(int wordAlingFont)
    pub fn getVAlignHandler(&self, wordAlingFont: i32) -> Option<IVAlignHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java switch (wordAlingFont)
      match wordAlingFont {
        // java case 0:
        0 => {
          // java if (null == this.mAlignTopHandler)
          if self.mAlignTopHandler.is_none() {
            // java this.mAlignTopHandler = new AlignTopHandler();
            self.mAlignTopHandler = AlignTopHandler::new();
          }
          // java return this.mAlignTopHandler;
          return { let __x: Option<IVAlignHandlerHolder> = todo!("no align strategy matched"); __x };
        }
        // java case 1:
        1 => {
          // java if (null == this.mAlignMiddleHandler)
          if self.mAlignMiddleHandler.is_none() {
            // java this.mAlignMiddleHandler = new AlignMiddleHandler();
            self.mAlignMiddleHandler = AlignMiddleHandler::new();
          }
          // java return this.mAlignMiddleHandler;
          return { let __x: Option<IVAlignHandlerHolder> = todo!("no align strategy matched"); __x };
        }
        // java case 2: default:
        2 | _ => {
          // java if (null == this.mAlignBaselineHandler)
          if self.mAlignBaselineHandler.is_none() {
            // java this.mAlignBaselineHandler = new AlignBaselineHandler();
            self.mAlignBaselineHandler = AlignBaselineHandler::new();
          }
          // java return this.mAlignBaselineHandler;
          return { let __x: Option<IVAlignHandlerHolder> = todo!("no align strategy matched"); __x };
        }
        // java case 3:
        3 => {
          // java if (null == this.mAlignBottomHandler)
          if self.mAlignBottomHandler.is_none() {
            // java this.mAlignBottomHandler = new AlignBottomHandler();
            self.mAlignBottomHandler = AlignBottomHandler::new();
          }
          // java return this.mAlignBottomHandler;
          return { let __x: Option<IVAlignHandlerHolder> = todo!("no align strategy matched"); __x };
        }
      }
      */
    }

    // java public IWhiteSpaceHandler getCharSpaceHandler(LayoutState env)
    pub fn getCharSpaceHandler(&self, env: Option<LayoutStateHolder>) -> Option<IWhiteSpaceHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mWhiteSpaceHandler)
      if self.mWhiteSpaceHandler.is_none() {
        // java this.mWhiteSpaceHandler = new WhiteSpaceHandler(this.mDocEnv, this.mViewEnv);
        self.mWhiteSpaceHandler = WhiteSpaceHandler::new(self.mDocEnv, self.mViewEnv);
      }
      // java this.mWhiteSpaceHandler.setEnv(env);
      self.mWhiteSpaceHandler.setEnv(env);
      // java return this.mWhiteSpaceHandler;
      return { let __x: Option<IWhiteSpaceHandlerHolder> = todo!("no align strategy matched"); __x };
      */
    }

    // java public IWhiteSpaceHandler getPhoneSpaceHandler(LayoutState env)
    pub fn getPhoneSpaceHandler(&self, env: Option<LayoutStateHolder>) -> Option<IWhiteSpaceHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mWhiteSpaceForPhoneHandler)
      if self.mWhiteSpaceForPhoneHandler.is_none() {
        // java this.mWhiteSpaceForPhoneHandler = new WhiteSpaceForPhoneHandler(this.mDocEnv, this.mViewEnv);
        self.mWhiteSpaceForPhoneHandler = WhiteSpaceForPhoneHandler::new(self.mDocEnv, self.mViewEnv);
      }
      // java this.mWhiteSpaceForPhoneHandler.setEnv(env);
      self.mWhiteSpaceForPhoneHandler.setEnv(env);
      // java return this.mWhiteSpaceForPhoneHandler;
      return { let __x: Option<IWhiteSpaceHandlerHolder> = todo!("no align strategy matched"); __x };
      */
    }

    // java public IAutoSpaceHandler getAutoSpaceHandler(LayoutState mEnv)
    pub fn getAutoSpaceHandler(&self, mEnv: Option<LayoutStateHolder>) -> Option<IAutoSpaceHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean hasMath = mEnv.hasMaths();
      let mut hasMath: bool = mEnv.hasMaths();
      // java if (mEnv.pageEnv.gridModel == 3 || !mEnv.paraEnv.autoSpaceDN && !mEnv.paraEnv.autoSpaceDE)
      if mEnv.pageEnv.gridModel == 3 || !mEnv.paraEnv.autoSpaceDN && !mEnv.paraEnv.autoSpaceDE {
        // java return hasMath ? this.getMahtOpSpaceHandler() : null;
        return Some((if hasMath { Some(self.getMahtOpSpaceHandler()) } else { None }));
      }
      // java else if (mEnv.paraEnv.autoSpaceDN && mEnv.paraEnv.autoSpaceDE)
      else if mEnv.paraEnv.autoSpaceDN && mEnv.paraEnv.autoSpaceDE {
        // java if (hasMath)
        if hasMath {
          // java if (null == this.mAutoSpaceDNandDEWithMathOpHandler)
          if self.mAutoSpaceDNandDEWithMathOpHandler.is_none() {
            // java this.mAutoSpaceDNandDEWithMathOpHandler = new AutoSpaceDNandDEWithMathOpHandler(this.mDocEnv);
            self.mAutoSpaceDNandDEWithMathOpHandler = AutoSpaceDNandDEWithMathOpHandler::new(self.mDocEnv);
          }
          // java return this.mAutoSpaceDNandDEWithMathOpHandler;
          return { let __x: Option<IAutoSpaceHandlerHolder> = todo!("no align strategy matched"); __x };
        }
        // java else
        else {
          // java if (null == this.mAutoSpaceDNandDEHandler)
          if self.mAutoSpaceDNandDEHandler.is_none() {
            // java this.mAutoSpaceDNandDEHandler = new AutoSpaceDNandDEHandler(this.mDocEnv);
            self.mAutoSpaceDNandDEHandler = AutoSpaceDNandDEHandler::new(self.mDocEnv);
          }
          // java return this.mAutoSpaceDNandDEHandler;
          return { let __x: Option<IAutoSpaceHandlerHolder> = todo!("no align strategy matched"); __x };
        }
      }
      // java else if (mEnv.paraEnv.autoSpaceDN)
      else if mEnv.paraEnv.autoSpaceDN {
        // java if (hasMath)
        if hasMath {
          // java if (null == this.mAutoSpaceDNWithMathOpHandler)
          if self.mAutoSpaceDNWithMathOpHandler.is_none() {
            // java this.mAutoSpaceDNWithMathOpHandler = new AutoSpaceDNWithMathOpHandler(this.mDocEnv);
            self.mAutoSpaceDNWithMathOpHandler = AutoSpaceDNWithMathOpHandler::new(self.mDocEnv);
          }
          // java return this.mAutoSpaceDNWithMathOpHandler;
          return { let __x: Option<IAutoSpaceHandlerHolder> = todo!("no align strategy matched"); __x };
        }
        // java else
        else {
          // java if (null == this.mAutoSpaceDNHandler)
          if self.mAutoSpaceDNHandler.is_none() {
            // java this.mAutoSpaceDNHandler = new AutoSpaceDNHandler(this.mDocEnv);
            self.mAutoSpaceDNHandler = AutoSpaceDNHandler::new(self.mDocEnv);
          }
          // java return this.mAutoSpaceDNHandler;
          return { let __x: Option<IAutoSpaceHandlerHolder> = todo!("no align strategy matched"); __x };
        }
      }
      // java else if (hasMath)
      else if hasMath {
        // java if (null == this.mAutoSpaceDEWithMathOpHandler)
        if self.mAutoSpaceDEWithMathOpHandler.is_none() {
          // java this.mAutoSpaceDEWithMathOpHandler = new AutoSpaceDEWithMathOpHandler(this.mDocEnv);
          self.mAutoSpaceDEWithMathOpHandler = AutoSpaceDEWithMathOpHandler::new(self.mDocEnv);
        }
        // java return this.mAutoSpaceDEWithMathOpHandler;
        return { let __x: Option<IAutoSpaceHandlerHolder> = todo!("no align strategy matched"); __x };
      }
      // java else
      else {
        // java if (null == this.mAutoSpaceDEHandler)
        if self.mAutoSpaceDEHandler.is_none() {
          // java this.mAutoSpaceDEHandler = new AutoSpaceDEHandler(this.mDocEnv);
          self.mAutoSpaceDEHandler = AutoSpaceDEHandler::new(self.mDocEnv);
        }
        // java return this.mAutoSpaceDEHandler;
        return { let __x: Option<IAutoSpaceHandlerHolder> = todo!("no align strategy matched"); __x };
      }
      */
    }

    // java public IAutoSpaceHandler getMahtOpSpaceHandler()
    pub fn getMahtOpSpaceHandler(&self) -> Option<IAutoSpaceHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mMahtOpSpaceHandler)
      if self.mMahtOpSpaceHandler.is_none() {
        // java this.mMahtOpSpaceHandler = new MahtOpSpaceHandler(this.mDocEnv);
        self.mMahtOpSpaceHandler = MahtOpSpaceHandler::new(self.mDocEnv);
      }
      // java return this.mMahtOpSpaceHandler;
      return { let __x: Option<IAutoSpaceHandlerHolder> = todo!("no align strategy matched"); __x };
      */
    }

    // java public IPunctSpaceHandler getPunctSpaceHandler(LayoutState layoutState)
    pub fn getPunctSpaceHandler(&self, layoutState: Option<LayoutStateHolder>) -> Option<IPunctSpaceHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java PropertySet docProp = this.mDocEnv.textDocument.getProperty();
      let mut docProp: PropertySet = self.mDocEnv.textDocument.getProperty();
      // java boolean compressPunct = 1 == docProp.getInt(382, 0) || ViewMode.isPhoneReadMode(this.mViewEnv.getViewMode()) && this.mViewEnv.getPhoneViewParam().isCompressLine();
      let mut compressPunct: bool = 1 == docProp.getInt(382, 0) || ViewMode::isPhoneReadMode(self.mViewEnv.getViewMode()) && self.mViewEnv.getPhoneViewParam().isCompressLine();
      // java if (compressPunct)
      if compressPunct {
        // java if (null == this.mCompressPunctSpaceHandler)
        if self.mCompressPunctSpaceHandler.is_none() {
          // java this.mCompressPunctSpaceHandler = new CompressPunctSpaceHandler(this.mDocEnv, this.mViewEnv);
          self.mCompressPunctSpaceHandler = CompressPunctSpaceHandler::new(self.mDocEnv, self.mViewEnv);
        }
        // java this.mCompressPunctSpaceHandler.setEnv(layoutState);
        self.mCompressPunctSpaceHandler.setEnv(layoutState);
        // java return this.mCompressPunctSpaceHandler;
        return { let __x: Option<IPunctSpaceHandlerHolder> = todo!("no align strategy matched"); __x };
      }
      // java else
      else {
        // java return null;
        return None;
      }
      */
    }

    // java public IGridHandler getGridHandler(int gridModel)
    pub fn getGridHandler(&self, gridModel: i32) -> Option<IGridHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java switch (gridModel)
      match gridModel {
        // java case 1:
        1 => {
          // java if (null == this.mCharGridHandler)
          if self.mCharGridHandler.is_none() {
            // java this.mCharGridHandler = new CharGridHandler(this.mDocEnv);
            self.mCharGridHandler = CharGridHandler::new(self.mDocEnv);
          }
          // java return this.mCharGridHandler;
          return { let __x: Option<IGridHandlerHolder> = todo!("no align strategy matched"); __x };
        }
        // java case 3:
        3 => {
          // java if (null == this.mEnforceGridHandler)
          if self.mEnforceGridHandler.is_none() {
            // java this.mEnforceGridHandler = new EnforceGridHandler();
            self.mEnforceGridHandler = EnforceGridHandler::new();
          }
          // java return this.mEnforceGridHandler;
          return { let __x: Option<IGridHandlerHolder> = todo!("no align strategy matched"); __x };
        }
        // java default:
        _ => {
          // java return null;
          return None;
        }
      }
      */
    }

    // java public TabHandler getTabHandler()
    pub fn getTabHandler(&self) -> Option<TabHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mTabHandler)
      if self.mTabHandler.is_none() {
        // java this.mTabHandler = new TabHandler(this.mDocEnv, this.mViewEnv);
        self.mTabHandler = TabHandler::new(self.mDocEnv, self.mViewEnv);
      }
      // java return this.mTabHandler;
      return self.mTabHandler;
      */
    }

    // java public IHAlignHandler getHAlignMathParaHandler()
    pub fn getHAlignMathParaHandler(&self) -> Option<IHAlignHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mAlignMathParaHandler)
      if self.mAlignMathParaHandler.is_none() {
        // java this.mAlignMathParaHandler = new AlignMathParaHandler(this.mDocEnv, this.mViewEnv);
        self.mAlignMathParaHandler = AlignMathParaHandler::new(self.mDocEnv, self.mViewEnv);
      }
      // java return this.mAlignMathParaHandler;
      return { let __x: Option<IHAlignHandlerHolder> = todo!("no align strategy matched"); __x };
      */
    }

    // java public IHAlignHandler getHAlignOMathParaHandler()
    pub fn getHAlignOMathParaHandler(&self) -> Option<IHAlignHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mAlignOMathParaHandler)
      if self.mAlignOMathParaHandler.is_none() {
        // java this.mAlignOMathParaHandler = new AlignOMathParaHandler(this.mDocEnv);
        self.mAlignOMathParaHandler = AlignOMathParaHandler::new(self.mDocEnv);
      }
      // java return this.mAlignOMathParaHandler;
      return self.mAlignOMathParaHandler;
      */
    }

    // java public ISymbolConvert getWingdingsConvert()
    pub fn getWingdingsConvert(&self) -> Option<ISymbolConvertHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mWingdingsConvert)
      if self.mWingdingsConvert.is_none() {
        // java this.mWingdingsConvert = new WingdingsConvert();
        self.mWingdingsConvert = WingdingsConvert::new();
      }
      // java return this.mWingdingsConvert;
      return self.mWingdingsConvert;
      */
    }

    // java public ISymbolConvert getWebdingsConvert()
    pub fn getWebdingsConvert(&self) -> Option<ISymbolConvertHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mWebdingsConvert)
      if self.mWebdingsConvert.is_none() {
        // java this.mWebdingsConvert = new WebdingsConvert();
        self.mWebdingsConvert = WebdingsConvert::new();
      }
      // java return this.mWebdingsConvert;
      return self.mWebdingsConvert;
      */
    }

    // java public ISymbolConvert getExtraConvert()
    pub fn getExtraConvert(&self) -> Option<ISymbolConvertHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mExtraConvert)
      if self.mExtraConvert.is_none() {
        // java this.mExtraConvert = new ExtraConvert();
        self.mExtraConvert = ExtraConvert::new();
      }
      // java return this.mExtraConvert;
      return self.mExtraConvert;
      */
    }

    // java public ISymbolConvert getSymbolConvert()
    pub fn getSymbolConvert(&self) -> Option<ISymbolConvertHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mSymbolConvert)
      if self.mSymbolConvert.is_none() {
        // java this.mSymbolConvert = new SymbolConvert();
        self.mSymbolConvert = SymbolConvert::new();
      }
      // java return this.mSymbolConvert;
      return self.mSymbolConvert;
      */
    }

    // java public ISymbolConvert getKingsoftConfettiConvert()
    pub fn getKingsoftConfettiConvert(&self) -> Option<ISymbolConvertHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mKinsoftConfettiConvert)
      if self.mKinsoftConfettiConvert.is_none() {
        // java this.mKinsoftConfettiConvert = new KingsoftConfettiConverter();
        self.mKinsoftConfettiConvert = KingsoftConfettiConverter::new();
      }
      // java return this.mKinsoftConfettiConvert;
      return self.mKinsoftConfettiConvert;
      */
    }

    // java public ISymbolConvert getKingsoftExtraConvert()
    pub fn getKingsoftExtraConvert(&self) -> Option<ISymbolConvertHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mKinsoftExtraConvert)
      if self.mKinsoftExtraConvert.is_none() {
        // java this.mKinsoftExtraConvert = new KingsoftExtraConverter();
        self.mKinsoftExtraConvert = KingsoftExtraConverter::new();
      }
      // java return this.mKinsoftExtraConvert;
      return self.mKinsoftExtraConvert;
      */
    }

    // java public ISymbolConvert getKingsoftSignConvert()
    pub fn getKingsoftSignConvert(&self) -> Option<ISymbolConvertHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mKinsoftSignConvert)
      if self.mKinsoftSignConvert.is_none() {
        // java this.mKinsoftSignConvert = new KingsoftSignConverter();
        self.mKinsoftSignConvert = KingsoftSignConverter::new();
      }
      // java return this.mKinsoftSignConvert;
      return self.mKinsoftSignConvert;
      */
    }

    // java public ParaBrcLayouter getParaBrcLayouter()
    pub fn getParaBrcLayouter(&self) -> Option<ParaBrcLayouterHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mParaBrcLayouter)
      if self.mParaBrcLayouter.is_none() {
        // java this.mParaBrcLayouter = new ParaBrcLayouter(this.mViewEnv);
        self.mParaBrcLayouter = ParaBrcLayouter::new(self.mViewEnv);
      }
      // java return this.mParaBrcLayouter;
      return self.mParaBrcLayouter;
      */
    }

    // java public KerningHandler getKerningHandler()
    pub fn getKerningHandler(&self) -> Option<KerningHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mKerningHandler)
      if self.mKerningHandler.is_none() {
        // java this.mKerningHandler = new KerningHandler();
        self.mKerningHandler = KerningHandler::new();
      }
      // java return this.mKerningHandler;
      return self.mKerningHandler;
      */
    }

    // java public CharBorderHandler getCharBorderHandler()
    pub fn getCharBorderHandler(&self) -> Option<CharBorderHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mCharBorderHandler)
      if self.mCharBorderHandler.is_none() {
        // java this.mCharBorderHandler = new CharBorderHandler();
        self.mCharBorderHandler = CharBorderHandler::new();
      }
      // java return this.mCharBorderHandler;
      return self.mCharBorderHandler;
      */
    }

    // java public IHAlignHandler getFitTextHandler()
    pub fn getFitTextHandler(&self) -> Option<IHAlignHandlerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mFitTextHandler)
      if self.mFitTextHandler.is_none() {
        // java this.mFitTextHandler = new FitTextHandler(this.mDocEnv);
        self.mFitTextHandler = FitTextHandler::new(self.mDocEnv);
      }
      // java return this.mFitTextHandler;
      return { let __x: Option<IHAlignHandlerHolder> = todo!("no align strategy matched"); __x };
      */
    }

    // java public WidthAnalyzer getWidthAnalyzer()
    pub fn getWidthAnalyzer(&self) -> Option<WidthAnalyzerHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mWidthAnalyzer)
      if self.mWidthAnalyzer.is_none() {
        // java this.mWidthAnalyzer = new WidthAnalyzer(this.mDocEnv, this.mViewEnv);
        self.mWidthAnalyzer = WidthAnalyzer::new(self.mDocEnv, self.mViewEnv);
      }
      // java return this.mWidthAnalyzer;
      return self.mWidthAnalyzer;
      */
    }

    // java public IFastGraph getDefaultFastGraph()
    pub fn getDefaultFastGraph(&self) -> Option<IFastGraphHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == this.mDefaultFastGraph)
      if self.mDefaultFastGraph.is_none() {
        // java this.mDefaultFastGraph = new FastGraphDefaut(this.mDocEnv, this.mViewEnv);
        self.mDefaultFastGraph = FastGraphDefaut::new(self.mDocEnv, self.mViewEnv);
      }
      // java return this.mDefaultFastGraph;
      return self.mDefaultFastGraph;
      */
    }

    // java public DocFieldCollector getDocFieldColect()
    pub fn getDocFieldColect(&self) -> Option<DocFieldCollectorHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mDocFieldCollect == null)
      if self.mDocFieldCollect.is_none() {
        // java this.mDocFieldCollect = new DocFieldCollector();
        self.mDocFieldCollect = DocFieldCollector::new();
      }
      // java return this.mDocFieldCollect;
      return self.mDocFieldCollect;
      */
    }

    // java public ViewEnv getViewEnv()
    pub fn getViewEnv(&self) -> Option<ViewEnvHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mViewEnv;
      return self.mViewEnv;
      */
    }

}
