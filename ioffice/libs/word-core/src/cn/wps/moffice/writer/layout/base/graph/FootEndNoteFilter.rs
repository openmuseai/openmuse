// Java: cn.wps.moffice.writer.layout.base.graph.FootEndNoteFilter
#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::PLC::{nodes_same, Node};
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::PLCFootnoteEndnoteRef;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::FilterArgs::FilterArgs;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphReader::GraphReader;
use crate::cn::wps::moffice::writer::layout::base::graph::IFilter::IFilter;

// java public class FootEndNoteFilter implements IFilter
pub struct FootEndNoteFilter {
    // java private GraphReader mGraphReader;
    mGraphReader: *mut GraphReader,
    // java private int mMax;
    mMax: i32,
    // java private PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode mNextFootEndRefNode;
    mNextFootEndRefNode: Option<Node>,
    // java private int mNextFootEndRefNodeCP;
    mNextFootEndRefNodeCP: i32,
    // java private boolean mIsFootNote;
    mIsFootNote: bool,
}

impl FootEndNoteFilter {
    // java public FootEndNoteFilter(FootnoteEndnoteRefNode footEndRefNode, int start, int end, boolean isFootNote, GraphReader graphReader)
    pub fn new(node: Node, _start: i32, end: i32, is_foot_note: bool, reader: *mut GraphReader) -> Self {
        let cp = node.getCP();
        Self {
            mGraphReader: reader,
            mMax: end,
            mNextFootEndRefNode: Some(node),
            mNextFootEndRefNodeCP: cp,
            mIsFootNote: is_foot_note,
        }
    }

    fn get_next(&mut self, cp: i32, foot_end_ref: *mut PLCFootnoteEndnoteRef) {
        if foot_end_ref.is_null() {
            self.mNextFootEndRefNode = None;
            return;
        }
        let plc = unsafe { &*foot_end_ref };
        let head = plc.getHead().as_node();
        let node = plc.findUpper(cp);
        if !node.ptr.is_null() && !nodes_same(node, head) && node.getCP() < self.mMax {
            self.mNextFootEndRefNodeCP = node.getCP();
            self.mNextFootEndRefNode = Some(node);
        } else {
            self.mNextFootEndRefNode = None;
        }
    }
}

impl IFilter for FootEndNoteFilter {
    // java public boolean doFilter(FilterArgs args, LayoutState layoutState)
    fn doFilter(&mut self, args: &mut FilterArgs, layout_state: &mut LayoutState) -> bool {
        if self.mNextFootEndRefNode.is_none() {
            return false;
        }
        let Some(doc) = layout_state.document.filter(|p| !p.is_null()) else { return false; };
        let foot_end_ref = unsafe {
            if self.mIsFootNote { (*doc).getFootnoteRef() } else { (*doc).getEndnoteRef() }
        }.unwrap_or(std::ptr::null_mut());
        if args.start > self.mNextFootEndRefNodeCP {
            self.get_next(args.start, foot_end_ref);
        }
        if self.mNextFootEndRefNode.is_none() {
            return false;
        }
        if args.start < self.mNextFootEndRefNodeCP {
            args.end = args.end.min(self.mNextFootEndRefNodeCP);
            args.next = args.next.min(self.mNextFootEndRefNodeCP);
            return false;
        }
        if args.start == self.mNextFootEndRefNodeCP {
            let node = self.mNextFootEndRefNode.take().unwrap_or_default();
            if !self.mGraphReader.is_null() {
                let reader = unsafe { &mut *self.mGraphReader };
                if self.mIsFootNote {
                    reader.readFootNote(args.start, node, layout_state);
                } else {
                    reader.readEndNote(args.start, node, layout_state);
                }
            }
            args.next = args.start + 1;
            self.get_next(args.next, foot_end_ref);
            return true;
        }
        false
    }
}
