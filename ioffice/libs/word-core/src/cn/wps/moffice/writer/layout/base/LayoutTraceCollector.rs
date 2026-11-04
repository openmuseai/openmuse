// Java: cn.wps.moffice.writer.layout.base.LayoutTraceCollector
// Read-only Stage2 observation at TextLayouter.doFlowTextLine, including
// lines committed inside TableCellLayouter.
#![allow(non_snake_case)]

use std::cell::RefCell;
use crate::cn::wps::moffice::writer::cache::Stage2Rows::Stage2Line;

struct Capture {
    active_page_index: i32,
    rows: Vec<Stage2Line>,
}

thread_local! {
    static CAPTURE: RefCell<Option<Capture>> = const { RefCell::new(None) };
}

pub struct LayoutTraceCollector;

impl LayoutTraceCollector {
    // java LayoutTraceCollector.reset() starts a new capture.
    pub fn beginStage2Capture() {
        CAPTURE.with(|slot| *slot.borrow_mut() = Some(Capture {
            active_page_index: -1,
            rows: Vec::new(),
        }));
    }

    // java LayoutTraceCollector.setActivePageIndex(int pageIndex)
    pub fn setActivePageIndex(page_index: i32) {
        CAPTURE.with(|slot| {
            if let Some(capture) = slot.borrow_mut().as_mut() {
                capture.active_page_index = page_index;
            }
        });
    }

    // java LayoutTraceCollector.onStage2Line(documentType, linePos, ...)
    pub fn onStage2Line(
        document_type: i32,
        line_pos: i32,
        start_cp: i32,
        end_cp: i32,
        left_twips: i32,
        top_twips: i32,
        width_twips: i32,
        height_twips: i32,
    ) {
        if document_type != 0 {
            return;
        }
        CAPTURE.with(|slot| {
            if let Some(capture) = slot.borrow_mut().as_mut() {
                capture.rows.push(Stage2Line {
                    page: capture.active_page_index.max(0),
                    line: line_pos,
                    start_cp,
                    end_cp,
                    x: left_twips,
                    y: top_twips,
                    width: width_twips,
                    height: height_twips,
                    xpos: Vec::new(),
                });
            }
        });
    }

    // java LayoutTraceCollector.getStage2Json(), Rust returns typed rows.
    pub fn takeStage2Rows() -> Vec<Stage2Line> {
        CAPTURE.with(|slot| slot.borrow_mut().take().map_or_else(Vec::new, |capture| capture.rows))
    }
}
