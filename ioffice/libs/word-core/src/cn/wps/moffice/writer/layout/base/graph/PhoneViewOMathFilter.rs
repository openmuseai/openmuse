#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.graph.PhoneViewOMathFilter
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::PLC::nodes_same;
use crate::cn::wps::moffice::writer::data::PLCMathStart::{load_start, OMathStartNode};
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::FilterArgs::FilterArgs;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphReader::GraphReader;
use crate::cn::wps::moffice::writer::layout::base::graph::IFilter::IFilter;

// java class PhoneViewOMathFilter implements IFilter
pub struct PhoneViewOMathFilter {
    reader: *mut GraphReader,
    max: i32,
    math_cp: i32,
    start: Option<OMathStartNode>,
}

impl PhoneViewOMathFilter {
    // java public PhoneViewOMathFilter(PLC.Node mathNode, int start, int end, GraphReader graphReader)
    pub fn new(start: OMathStartNode, end: i32, reader: *mut GraphReader) -> Self {
        let math_cp = start.getCP();
        Self { reader, max: end, math_cp, start: Some(start) }
    }

    // java public static PLC.Node getNextOMathNode(Document document, int start, int end)
    pub fn getNextOMathNode(document: &dyn Document, start: i32, end: i32) -> Option<OMathStartNode> {
        let starts = document.getOMathStarts()?;
        if starts.is_null() {
            return None;
        }
        let starts = unsafe { &*starts };
        let node = starts.super_.findUpper(start);
        let head = starts.super_.getHead().as_node();
        if !node.ptr.is_null() && !nodes_same(node, head) && node.getCP() < end {
            Some(load_start(node, true))
        } else {
            None
        }
    }
}

impl IFilter for PhoneViewOMathFilter {
    // java public boolean doFilter(FilterArgs args, LayoutState layoutState)
    fn doFilter(&mut self, args: &mut FilterArgs, layout: &mut LayoutState) -> bool {
        if self.start.is_none() {
            return false;
        }
        if args.start > self.math_cp {
            let doc = layout.document.filter(|p| !p.is_null()).map(|p| unsafe { &*p });
            self.start = doc.and_then(|doc| Self::getNextOMathNode(doc, args.start, self.max));
            if self.start.is_none() {
                return false;
            }
            self.math_cp = self.start.as_ref().map(|n| n.getCP()).unwrap_or(0);
        }
        if args.start < self.math_cp {
            args.next = args.next.min(self.math_cp);
            args.end = args.end.min(self.math_cp);
            return false;
        }
        if args.start == self.math_cp {
            let Some(start) = self.start.clone() else { return false };
            // java int endCP = this.mStartNode.getEnd().getCP();
            let Some(end_cp) = start.getEnd().map(|end| end.getCP()) else { return false };
            unsafe { (*self.reader).readOMath(&start, end_cp, layout); }
            args.next = end_cp;
            let doc = layout.document.filter(|p| !p.is_null()).map(|p| unsafe { &*p });
            self.start = doc.and_then(|doc| Self::getNextOMathNode(doc, end_cp, self.max));
            self.math_cp = self.start.as_ref().map(|n| n.getCP()).unwrap_or(0);
            return true;
        }
        false
    }
}
