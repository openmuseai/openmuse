// Java: cn.wps.moffice.writer.layout.base.line.align.IHAlignHandler
// rust: ★wr-29 05 —— 真实移植（Java 28 行）；ITypoRect → TypoRectCoreImpl（真实矩形）
#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;

// java public interface IHAlignHandler
pub trait IHAlignHandler {
    // java void setSubLineEnv(SubLineEnv var1, LayoutState var2);
    fn setSubLineEnv(&mut self, var1: &mut SubLineEnv, var2: &mut LayoutState);

    // java void onAddRun(Graphs var1);
    fn onAddRun(&mut self, var1: &mut Graphs);

    // java void onRemoveRun(Graphs var1);
    fn onRemoveRun(&mut self, var1: &mut Graphs);

    // java void adjustRun(Graphs var1);
    fn adjustRun(&mut self, var1: &mut Graphs);

    // java void adjustLine(ITypoRect var1);
    fn adjustLine(&mut self, var1: &mut TypoRectCoreImpl);

    // java boolean calcLineLeftAndRight(ITypoRect var1);
    fn calcLineLeftAndRight(&mut self, var1: &mut TypoRectCoreImpl) -> bool;

    // java void prepareRecalcForBidi();
    fn prepareRecalcForBidi(&mut self);

    // java void onAddBidiRun(Graphs var1, boolean var2);
    fn onAddBidiRun(&mut self, var1: &mut Graphs, var2: bool);

    // java void adjustBidiLine(ITypoRect var1);
    fn adjustBidiLine(&mut self, var1: &mut TypoRectCoreImpl);

    // java void dispose();
    fn dispose(&mut self);
}
