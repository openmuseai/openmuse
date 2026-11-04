#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// Java: public abstract class AutoSpaceBaseHandler implements IAutoSpaceHandler, Reusable
pub struct AutoSpaceBaseHandler {
    // Java: protected DocumentEnv mDocEnv;
    mDocEnv: *const DocumentEnv,
    // Java: protected boolean mDiffSBMB = false;
    mDiffSBMB: bool,
    // Java: protected LayoutState mLayoutState = null;
    mLayoutState: *mut LayoutState,
    // Java: private SubLineEnv mSubLineEnv;
    mSubLineEnv: *mut SubLineEnv,
}

impl AutoSpaceBaseHandler {
    // Java: public AutoSpaceBaseHandler(DocumentEnv docEnv)
    pub fn AutoSpaceBaseHandler(docEnv: *const DocumentEnv) -> Self {
        Self { mDocEnv: docEnv, mDiffSBMB: false, mLayoutState: std::ptr::null_mut(), mSubLineEnv: std::ptr::null_mut() }
    }
    // Java: public boolean reuseClean()
    pub fn reuseClean(&mut self) -> bool { self.mLayoutState = std::ptr::null_mut(); self.mSubLineEnv = std::ptr::null_mut(); true }
    // Java: public void reuseInit()
    pub fn reuseInit(&mut self) {}
    // Java: public void setEnv(LayoutState env)
    pub fn setEnv(&mut self, env: *mut LayoutState) {
        self.mLayoutState = env;
        // Java: this.mDiffSBMB = this.mDocEnv.balanceSingleByteDoubleByteWidth;
        self.mDiffSBMB = unsafe { self.mDocEnv.as_ref() }.map(|d| d.balanceSingleByteDoubleByteWidth).unwrap_or(false);
    }
    // Java: public void setSubLineEnv(SubLineEnv subLineEnv)
    pub fn setSubLineEnv(&mut self, env: *mut SubLineEnv) { self.mSubLineEnv = env; }
    // Java: protected final void extendSpace(Graphs run)
    pub fn extendSpace(&mut self, run: &mut Graphs) {
        let Some(fontStyle) = run.fontStyle.as_ref() else { return; };
        // Java: float ratio = 4.0F; if (!mDiffSBMB) ratio = typeface.getWriterLayoutCharSpaceRatio();
        let ratio = if self.mDiffSBMB { 4.0 } else {
            fontStyle.typeface.as_ref().map(|t| t.getWriterLayoutCharSpaceRatio()).unwrap_or(4.0)
        };
        // Java: float extendSpace = fontSize * charScale / ratio + fontSpace / 2.0F;
        let extendSpace = ((fontStyle.fontSize as f32 * fontStyle.charScale / ratio) + fontStyle.fontSpace as f32 / 2.0).max(0.0);
        // Java: if (0.0F != extendSpace)
        if extendSpace != 0.0 {
            let env = unsafe { &mut *self.mLayoutState };
            // Java: mLayoutState.saveOrgWidth(run); mLayoutState.extendWidths(run, (int)extendSpace, run.getEndGraphIndex(), 1); run.extendsWidth((int)extendSpace);
            env.saveOrgWidth(run);
            let width = extendSpace as i32;
            env.extendWidths(run, width, run.getEndGraphIndex(), 1);
            run.extendsWidth(width);
            // Java: SubLineEnv.lastGraphs/firstGraphs reference the same Graphs instance as run.
            // Rust's current SubLineEnv holder is a value snapshot, so refresh that holder when
            // this Java mutation changes the graph observed by the following addRun.
            if let Some(subLine) = unsafe { self.mSubLineEnv.as_mut() } {
                if subLine.lastGraphs.as_ref().is_some_and(|g| g.graphIndex == run.graphIndex) {
                    subLine.lastGraphs = Some(Clone::clone(run));
                }
                if subLine.firstGraphs.as_ref().is_some_and(|g| g.graphIndex == run.graphIndex) {
                    subLine.firstGraphs = Some(Clone::clone(run));
                }
            }
            // Java: lineCompressEnv.addOtherCompressableSpace(run.getEndGraphIndex(), (int)(extendSpace / 2.0F));
            if let Some(subLine) = unsafe { self.mSubLineEnv.as_mut() } {
                if let Some(compressEnv) = subLine.lineCompressEnv.as_mut() {
                    compressEnv.addOtherCompressableSpace(run.getEndGraphIndex(), (extendSpace / 2.0) as i32);
                }
            }
        }
    }
}
