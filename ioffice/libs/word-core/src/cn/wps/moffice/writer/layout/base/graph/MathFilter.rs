#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.graph.MathFilter
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::PLC::{nodes_same, Node};
use crate::cn::wps::moffice::writer::data::PLCMathFunction::{is_math_start, math_start_from, MathStartNode, PLCMathFunction};
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::FilterArgs::FilterArgs;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphReader::GraphReader;
use crate::cn::wps::moffice::writer::layout::base::graph::IFilter::IFilter;

// java public class MathFilter implements IFilter
pub struct MathFilter {
    reader: *mut GraphReader,
    max: i32,
    math_node: Option<MathStartNode>,
    math_cp: i32,
}

impl MathFilter {
    // java public MathFilter(Document document, PLC.Node mathFunctionNode, int start, int end, GraphReader graphReader)
    pub fn new(_document: *mut dyn Document, node: Node, _start: i32, end: i32, reader: *mut GraphReader) -> Self {
        let math_cp = node.getCP();
        Self { reader, max: end, math_node: math_start_from(node), math_cp }
    }

    fn pull_next(&mut self, start: i32, layout_state: &LayoutState) {
        let Some(doc) = layout_state.document.filter(|p| !p.is_null()) else {
            self.math_node = None;
            return;
        };
        let Some(plc) = (unsafe { (*doc).getPLCMathFunction() }).filter(|p| !p.is_null()) else {
            self.math_node = None;
            return;
        };
        let plc = unsafe { &*plc };
        let found = next_math_start(plc, start, self.max);
        self.math_cp = found.as_ref().map(|n| n.getCP()).unwrap_or(0);
        self.math_node = found;
    }
}

fn next_math_start(plc: &PLCMathFunction, start: i32, max: i32) -> Option<MathStartNode> {
    let head = plc.super_.super_.getHead().as_node();
    let mut node = plc.super_.super_.findUpper(start);
    while !node.ptr.is_null() && !nodes_same(node, head) && !is_math_start(node) {
        node = node.getNext();
    }
    if !node.ptr.is_null() && !nodes_same(node, head) && node.getCP() >= start && node.getCP() < max {
        math_start_from(node)
    } else {
        None
    }
}

impl IFilter for MathFilter {
    // java public boolean doFilter(FilterArgs args, LayoutState layoutState)
    fn doFilter(&mut self, args: &mut FilterArgs, layout_state: &mut LayoutState) -> bool {
        if self.math_node.is_none() {
            return false;
        }
        if args.start > self.math_cp {
            self.pull_next(args.start, layout_state);
            if self.math_node.is_none() {
                return false;
            }
        }
        if args.start < self.math_cp {
            args.next = args.next.min(self.math_cp);
            args.end = args.end.min(self.math_cp);
            return false;
        }
        if args.start == self.math_cp {
            let Some(node) = self.math_node.as_ref() else { return false };
            let end_cp = node.getEndNode().map(|n| n.getCP() + 1).unwrap_or(node.getCP() + 1);
            let snap = MathStartNode {
                super_: node.super_.clone(),
                mPropSet: node.getPropertySet().cloned(),
                mType: node.getType(),
                mShape: None,
            };
            unsafe { (*self.reader).readMath(&snap, end_cp, layout_state); }
            args.next = end_cp;
            self.pull_next(end_cp, layout_state);
            return true;
        }
        false
    }
}
