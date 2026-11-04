// Java: cn.wps.moffice.writer.layout.base.env.FootEndNoteEnv
#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::writer::data::PLCSection::SectionNode;
use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::TextboxLayoutState;
use util::kfc::r#impl::arrayList::WIntArrayList::WIntArrayList;

// java public class FootEndNoteEnv
pub struct FootEndNoteEnv {
    // java public int curFootNoteRefCP = -1;
    pub curFootNoteRefCP: i32,
    // java public TextboxLayoutState curFootNoteLayoutState;
    pub curFootNoteLayoutState: Option<TextboxLayoutState>,
    // java public WIntArrayList footNoteRefCPs = new WIntArrayList();
    pub footNoteRefCPs: WIntArrayList,
    // java public int curEndNoteRefCP = -1;
    pub curEndNoteRefCP: i32,
    // java public TextboxLayoutState curEndNoteLayoutState;
    pub curEndNoteLayoutState: Option<TextboxLayoutState>,
    // java public boolean hasInitSecProp = false;
    pub hasInitSecProp: bool,
    // java public int footNoteRefMsoNfc;
    pub footNoteRefMsoNfc: i32,
    // java public int endNoteRefMsoNfc;
    pub endNoteRefMsoNfc: i32,
    // java public int footNoteStart;
    pub footNoteStart: i32,
    // java public int endNoteStart;
    pub endNoteStart: i32,
    // java public int footNoteRestart;
    pub footNoteRestart: i32,
    // java public int endNoteRestart;
    pub endNoteRestart: i32,
    // java public int footNotePosition;
    pub footNotePosition: i32,
    // java public boolean isEndNoteSuppressed;
    pub isEndNoteSuppressed: bool,
    // java public PLCSection.SectionNode curSectionNode;
    pub curSectionNode: Option<SectionNode>,
}

impl FootEndNoteEnv {
    // java field initializers
    pub fn new() -> Self {
        Self {
            curFootNoteRefCP: -1,
            curFootNoteLayoutState: None,
            footNoteRefCPs: WIntArrayList::new(),
            curEndNoteRefCP: -1,
            curEndNoteLayoutState: None,
            hasInitSecProp: false,
            footNoteRefMsoNfc: 0,
            endNoteRefMsoNfc: 0,
            footNoteStart: 0,
            endNoteStart: 0,
            footNoteRestart: 0,
            endNoteRestart: 0,
            footNotePosition: 0,
            isEndNoteSuppressed: false,
            curSectionNode: None,
        }
    }
}
