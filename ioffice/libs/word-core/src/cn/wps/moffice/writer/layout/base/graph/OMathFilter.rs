#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.graph.OMathFilter
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::IChpxTable::IChpxTable;
use crate::cn::wps::moffice::writer::data::PLC::nodes_same;
use crate::cn::wps::moffice::writer::data::PLCMathEnd::OMathEndNode;
use crate::cn::wps::moffice::writer::data::PLCMathStart::{load_end, load_start, OMathStartNode};
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::FilterArgs::FilterArgs;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphReader::GraphReader;
use crate::cn::wps::moffice::writer::layout::base::graph::IFilter::IFilter;

pub enum OMathEdge {
    Start(OMathStartNode),
    End(OMathEndNode),
}

impl OMathEdge {
    fn cp(&self) -> i32 {
        match self {
            OMathEdge::Start(node) => node.getCP(),
            OMathEdge::End(node) => node.getCP(),
        }
    }

    fn is_start(&self) -> bool {
        matches!(self, OMathEdge::Start(_))
    }

    fn is_end(&self) -> bool {
        matches!(self, OMathEdge::End(_))
    }
}

// java public class OMathFilter implements IFilter
pub struct OMathFilter {
    reader: *mut GraphReader,
    max: i32,
    next: Option<OMathEdge>,
    next_cp: i32,
}

impl OMathFilter {
    pub fn new(edge: OMathEdge, end: i32, reader: *mut GraphReader) -> Self {
        let next_cp = edge.cp();
        Self { reader, max: end, next: Some(edge), next_cp }
    }

    // java public static PLC.Node getNextOMathNode(Document document, int start, int end)
    pub fn getNextOMathNode(document: &dyn Document, start: i32, end: i32) -> Option<OMathEdge> {
        let starts = document.getOMathStarts()?;
        if starts.is_null() {
            return None;
        }
        let ends = document.getOMathEnds()?;
        if ends.is_null() {
            return None;
        }
        let starts = unsafe { &*starts };
        let ends = unsafe { &*ends };
        let start_node = starts.super_.findUpper(start);
        let end_node = ends.super_.super_.findUpper(start);
        let start_head = starts.super_.getHead().as_node();
        let end_head = ends.super_.super_.getHead().as_node();
        let start_live = !start_node.ptr.is_null() && !nodes_same(start_node, start_head);
        let end_live = !end_node.ptr.is_null() && !nodes_same(end_node, end_head);
        if start_live {
            if end_live {
                if start_node.getCP() >= end_node.getCP() {
                    if end_node.getCP() < end {
                        return Some(OMathEdge::End(load_end(end_node)));
                    }
                } else if start_node.getCP() < end {
                    return Some(OMathEdge::Start(load_start(start_node, true)));
                }
            }
        } else if end_live && end_node.getCP() < end {
            return Some(OMathEdge::End(load_end(end_node)));
        }
        None
    }

    fn leave_math(&self, layout: &mut LayoutState) {
        let reader = unsafe { &mut *self.reader };
        reader.setIsInMath(false, layout);
        reader.setIgnoreHpsAndVAlignRun(false);
        reader.setIgnoreCapsAndSmallCaps(false);
        reader.setCalcGraphHeight(false);
    }

    fn enter_math(&self, layout: &mut LayoutState) {
        let reader = unsafe { &mut *self.reader };
        reader.setIsInMath(true, layout);
        reader.setIgnoreHpsAndVAlignRun(true);
        reader.setIgnoreCapsAndSmallCaps(true);
        reader.setCalcGraphHeight(true);
    }

    fn pull_next(&mut self, start: i32, layout: &LayoutState) {
        let Some(doc) = layout.document.filter(|p| !p.is_null()) else {
            self.next = None;
            return;
        };
        self.next = Self::getNextOMathNode(unsafe { &*doc }, start, self.max);
        self.next_cp = self.next.as_ref().map(|n| n.cp()).unwrap_or(0);
    }
}

impl IFilter for OMathFilter {
    // java public boolean doFilter(FilterArgs args, LayoutState layoutState)
    fn doFilter(&mut self, args: &mut FilterArgs, layout: &mut LayoutState) -> bool {
        if self.next.is_none() {
            return false;
        }
        if args.start > self.next.as_ref().map(|n| n.cp()).unwrap_or(0) {
            self.pull_next(args.start, layout);
            if self.next.is_none() {
                self.leave_math(layout);
                return false;
            }
        }
        if args.start < self.next_cp {
            args.next = args.next.min(self.next_cp);
            args.end = args.end.min(self.next_cp);
            if self.next.as_ref().is_some_and(|n| n.is_end()) {
                self.enter_math(layout);
                if let Some(OMathEdge::End(end)) = self.next.as_ref() {
                    let start_cp = end.getStart().map(|n| n.getCP()).unwrap_or(end.getCP());
                    calc_math_underline(self.reader, start_cp, layout);
                    if let Some(start) = end.getStart() {
                        if start.isInOMathPara() {
                            layout.addMathParaRange(start.getCP(), end.getCP(), start);
                        }
                    }
                }
            } else {
                self.leave_math(layout);
            }
        } else if self.next.as_ref().is_some_and(|n| n.is_start()) {
            if let Some(OMathEdge::Start(start)) = self.next.as_ref() {
                let end_cp = start.getEnd().map(|n| n.getCP()).unwrap_or(start.getCP());
                args.next = args.next.min(end_cp);
                args.end = args.end.min(args.next);
                let in_para = start.isInOMathPara();
                let start_cp = start.getCP();
                let start_node = start.clone();
                self.enter_math(layout);
                calc_math_underline(self.reader, start_cp, layout);
                if in_para {
                    layout.addMathParaRange(start_cp, end_cp, &start_node);
                }
            }
        } else {
            self.pull_next(args.start + 1, layout);
            if let Some(edge) = self.next.as_ref() {
                self.next_cp = edge.cp();
                args.next = args.next.min(self.next_cp);
                args.end = args.end.min(self.next_cp);
            }
            if self.next.as_ref().map(|n| n.is_start()).unwrap_or(true) {
                self.leave_math(layout);
            }
        }
        false
    }
}

fn calc_math_underline(reader: *mut GraphReader, mut cp: i32, layout: &mut LayoutState) {
    let Some(document) = layout.document.filter(|p| !p.is_null()) else { return };
    let document = unsafe { &*document };
    if let Some(plc) = document.getPLCField().filter(|p| !p.is_null()) {
        let plc = unsafe { &*plc };
        let node = plc.super_.super_.findUpper(cp);
        let head = plc.super_.super_.getHead().as_node();
        if !node.ptr.is_null() && !nodes_same(node, head) && node.getCP() == cp {
            // Field character lives on the field node. A plain PLC header has no field char.
            let _ = node;
        }
    }
    let Some(run) = document.getChpxTable().and_then(|t| unsafe { IChpxTable::seek(&*t, cp) }) else { return };
    let reader = unsafe { &mut *reader };
    let revision = reader.super_.aliased_view().map(|v| v.isRevisionFinal()).unwrap_or(false);
    let Some(para) = layout.paraEnv.curParaPropEnv.as_ref().and_then(|env| env.paraProperty.as_ref()) else { return };
    let Some(prop) = reader.mPropertyTool.as_mut().map(|tool| tool.getFinalRunPropert__entry_propertyset_boolean(&*run, para, revision)) else { return };
    let underline = prop.getValue(8).and_then(|v| v.as_ref().as_any().downcast_ref::<i32>().copied());
    if let Some(underline) = underline.filter(|n| *n != 0) {
        let mut color = prop.getInt__int_int(9, -16777216);
        if color == -16777216 {
            color = prop.getInt__int_int(11, -16777216);
        }
        reader.setMathUnderline(underline, color);
    } else {
        reader.setMathUnderline(0, -16777216);
    }
}
