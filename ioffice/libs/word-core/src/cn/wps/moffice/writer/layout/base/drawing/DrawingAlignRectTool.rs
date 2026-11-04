#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool::{self, CoordRect};

pub struct DrawingAlignRectTool;

impl DrawingAlignRectTool {
    // java public static void calcAlignRectLeftAndRightWithPage(int relX, TypoPage alignPage, TypoPage floatsPage, ITypoRect outRect)
    pub fn calcAlignRectLeftAndRightWithPage(rel_x: i32, align_page: i32, floats_page: i32, rect: &mut CoordRect, snapshot: &TypoSnapshot) {
        DrawingCoordTool::calcAlignRectLeftAndRightWithPage(rel_x, align_page, floats_page, rect, snapshot);
    }
}
