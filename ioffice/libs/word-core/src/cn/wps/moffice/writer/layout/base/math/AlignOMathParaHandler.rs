#![allow(dead_code, non_snake_case, unused_variables)]
// Java: cn.wps.moffice.writer.layout.base.math.AlignOMathParaHandler
use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
use crate::cn::wps::moffice::writer::data::value::MathProperty::MathProperty;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;
use crate::cn::wps::moffice::writer::layout::base::line::align::AlignBaseHandler::AlignBaseHandler;
use crate::cn::wps::moffice::writer::layout::base::line::align::IHAlignHandler::IHAlignHandler;

// java public class AlignOMathParaHandler extends AlignBaseHandler
pub struct AlignOMathParaHandler {
    pub super_: AlignBaseHandler,
    // java private int mWrapIndent;
    mWrapIndent: i32,
    // java private boolean mFirstSubLineOfPara;
    mFirstSubLineOfPara: bool,
    // java private DocumentEnv mDocEnv;
    mDocEnv: *mut DocumentEnv,
}

impl AlignOMathParaHandler {
    // java public AlignOMathParaHandler(DocumentEnv docEnv)
    pub fn AlignOMathParaHandler(docEnv: *mut DocumentEnv) -> Self {
        Self {
            super_: AlignBaseHandler {
                mSubLineEnv: std::ptr::null_mut(),
                mLayoutState: std::ptr::null_mut(),
                mCurOffset: 0,
                mMaxValidGraphIndex: 0,
                mAlignLeftOffset: 0,
                mAlignRightOffset: 0,
                mFindValidGraphs: false,
                mParaIsRTL: false,
                mNeedToCompressGraphs: false,
                mCompressedSpaceSum: 0,
            },
            mWrapIndent: 0,
            mFirstSubLineOfPara: false,
            mDocEnv: docEnv,
        }
    }

    // java public void setSubLineEnv(SubLineEnv subLineEnv, LayoutState layoutState)
    pub fn setSubLineEnv(&mut self, subLineEnv: &mut SubLineEnv, layoutState: &mut LayoutState) {
        self.super_.setSubLineEnv(subLineEnv, layoutState);
        if subLineEnv.hasGraphs() {
            return;
        }
        self.mWrapIndent = 0;
        if !self.mDocEnv.is_null() {
            if let Some(text) = unsafe { (*self.mDocEnv).textDocument_mut() } {
                let property = text.getProperty();
                if let Some(value) = property.getValue(642) {
                    if let Some(math) = value.as_ref().as_any().downcast_ref::<MathProperty>() {
                        if let Some(set) = math.getMathPropertySet() {
                            self.mWrapIndent = set.getInt__int_int(610, 0);
                            if self.mWrapIndent > layoutState.pageEnv.getCoreWidth() / 2 {
                                self.mWrapIndent = 0;
                            }
                        }
                    }
                }
            }
        }
        if subLineEnv.enableLeftIndent {
            let indent = layoutState.paraEnv.curParaPropEnv.as_ref().map(|p| p.leftIndent).unwrap_or(0);
            subLineEnv.left += indent;
            subLineEnv.enableLeftIndent = false;
        }
        if subLineEnv.enableRightIndent {
            let indent = layoutState.paraEnv.curParaPropEnv.as_ref().map(|p| p.rightIndent).unwrap_or(0);
            subLineEnv.right -= indent;
            subLineEnv.enableRightIndent = false;
        }
        let first = layoutState.getGraphList().firstElement();
        self.mFirstSubLineOfPara = if first.is_null() {
            false
        } else {
            let index = unsafe { (*first).graphIndex };
            layoutState.getRangeStart() == layoutState.getCP(index)
        };
        if !self.mFirstSubLineOfPara && self.mWrapIndent > 0 {
            subLineEnv.left += self.mWrapIndent;
        }
    }

    // java public void adjustLine(ITypoRect lineRect)
    pub fn adjustLine(&mut self, lineRect: &mut TypoRectCoreImpl) {
        self.doAlignLeft(lineRect);
    }

    // java private void doAlignLeft(ITypoRect lineRect)
    fn doAlignLeft(&mut self, lineRect: &mut TypoRectCoreImpl) {
        let sub = unsafe { &*self.super_.mSubLineEnv };
        lineRect.setLeft(sub.layoutLeft);
        let width = sub.lastGraphs.as_ref().map(|g| g.getRight()).unwrap_or(0);
        lineRect.setWidth(width);
    }

    // java public void adjustBidiLine(ITypoRect lineRect)
    pub fn adjustBidiLine(&mut self, lineRect: &mut TypoRectCoreImpl) {
        self.doAlignLeftBidi(lineRect);
    }

    // java private void doAlignLeftBidi(ITypoRect lineRect)
    fn doAlignLeftBidi(&mut self, lineRect: &mut TypoRectCoreImpl) {
        let sub = unsafe { &*self.super_.mSubLineEnv };
        lineRect.setLeft(sub.layoutLeft - self.super_.mAlignLeftOffset);
        let width = sub.lastGraphs.as_ref().map(|g| g.getRight()).unwrap_or(0);
        lineRect.setWidth(width);
    }

    // java public boolean calcLineLeftAndRight(ITypoRect lineRect)
    pub fn calcLineLeftAndRight(&mut self, lineRect: &mut TypoRectCoreImpl) -> bool {
        self.adjustLine(lineRect);
        true
    }
}

impl IHAlignHandler for AlignOMathParaHandler {
    fn setSubLineEnv(&mut self, var1: &mut SubLineEnv, var2: &mut LayoutState) {
        self.setSubLineEnv(var1, var2);
    }
    fn onAddRun(&mut self, var1: &mut Graphs) {
        self.super_.onAddRun(var1);
    }
    fn onRemoveRun(&mut self, var1: &mut Graphs) {
        self.super_.onRemoveRun(var1);
    }
    fn adjustRun(&mut self, var1: &mut Graphs) {
        self.super_.adjustRun(var1);
    }
    fn adjustLine(&mut self, var1: &mut TypoRectCoreImpl) {
        self.adjustLine(var1);
    }
    fn calcLineLeftAndRight(&mut self, var1: &mut TypoRectCoreImpl) -> bool {
        self.calcLineLeftAndRight(var1)
    }
    fn prepareRecalcForBidi(&mut self) {
        self.super_.prepareRecalcForBidi();
    }
    fn onAddBidiRun(&mut self, var1: &mut Graphs, var2: bool) {
        self.super_.onAddBidiRun(var1, var2);
    }
    fn adjustBidiLine(&mut self, var1: &mut TypoRectCoreImpl) {
        self.adjustBidiLine(var1);
    }
    fn dispose(&mut self) {
        self.super_.dispose();
    }
}
