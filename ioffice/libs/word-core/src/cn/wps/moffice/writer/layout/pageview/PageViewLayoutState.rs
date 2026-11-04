#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
use crate::cn::wps::moffice::writer::data::value::PageBorderProp::PageBorderProp;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;

// java public class PageViewLayoutState extends LayoutState
#[derive(Default)]
pub struct PageViewLayoutState {
    // java LayoutState super_
    pub super_: LayoutState,
    // java public int pagePos;
    pub pagePos: i32,
    // java public BRC brcTop;
    pub brcTop: Option<BRC>,
    // java public BRC brcLeft;
    pub brcLeft: Option<BRC>,
    // java public BRC brcRight;
    pub brcRight: Option<BRC>,
    // java public BRC brcBottom;
    pub brcBottom: Option<BRC>,
    // java public PageBorderProp pageBorderProp;
    pub pageBorderProp: Option<PageBorderProp>,
    // java public FootEndNoteEnv footEndNoteEnv
    pub curFootNoteRefCP: i32,
    pub curEndNoteRefCP: i32,
}
