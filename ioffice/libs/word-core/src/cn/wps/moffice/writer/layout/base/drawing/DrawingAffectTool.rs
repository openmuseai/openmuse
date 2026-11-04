// Rust stub for DrawingAffectTool (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/drawing/DrawingAffectTool.java
use crate::*;

pub struct DrawingAffectTool {
    // TODO: fields
}

impl DrawingAffectTool {
    pub fn new() -> Self { todo!() }
    pub fn unionAffectRect(affectRect: TypoRectCoreImpl, layoutRect: Box<dyn ITypoRect>) { todo!() }
    pub fn getAffectedPage(curPage: TypoPage, layoutRect: TypoRectCoreImpl, snapshot: TypoSnapshot) -> i32 { todo!() }
    pub fn isIntersectedWithPageCore(linesPage: TypoPage, affectRect: TypoRectCoreImpl, snapshot: TypoSnapshot) -> bool { todo!() }
    pub fn getFirstRollbackLineInColumns(columns: i32, affectRectPage: TypoPage, affectRect: TypoRectCoreImpl, snapshot: TypoSnapshot) -> i32 { todo!() }
    pub fn getFirstRollbackLineInPage(linesPage: TypoPage, affectRectPage: TypoPage, affectRect: TypoRectCoreImpl, needFindLeftTopMostLine: bool, snapshot: TypoSnapshot) -> i32 { todo!() }
}
