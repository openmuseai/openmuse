#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.graph.FieldFilter
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::IChpxTable::{IChpxTable, Entry as ChpxEntry};
use crate::cn::wps::moffice::writer::data::IPapxTable::{IPapxTable, Entry as PapxEntry};
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
use crate::cn::wps::moffice::writer::core::RangeTool::RangeTool;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphReaderTool::GraphReaderTool;
use crate::cn::wps::collections::textrope::ITextRope::ITextRope;
use crate::cn::wps::moffice::writer::data::PLC::{nodes_same, Node};
use crate::cn::wps::moffice::writer::data::PLCField::{FieldNode, FieldTriple, PLCField};
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;
use crate::cn::wps::moffice::writer::data::field::KFieldType::KFieldType;
use crate::cn::wps::moffice::writer::data::field::parser::IFieldParser::IFieldParser;
use crate::cn::wps::moffice::writer::data::field::parser::ParserFactory::ParserFactory;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::IPropertyTool::IPropertyTool;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::graph::FilterArgs::FilterArgs;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphReader::GraphReader;
use crate::cn::wps::moffice::writer::layout::base::graph::IFilter::IFilter;
use crate::cn::wps::moffice::writer::layout::base::graph::IRunPropFilter::IRunPropFilter;
use util::moffice::property::PropertySet::PropertySet;

// java public class FieldFilter implements IFilter
pub struct FieldFilter {
    reader: *mut GraphReader,
    next: Option<FieldNode>,
    next_cp: i32,
    max: i32,
    triple: FieldTriple,
    cur_level: i32,
    toc_level: i32,
    hyper_level: i32,
    hyper_id: i32,
    toc_filter: Option<*const dyn IRunPropFilter>,
}

impl FieldFilter {
    // java public static FieldFilter create(Document document, int start, int end, GraphReader graphReader)
    pub fn create(document: *mut dyn Document, start: i32, end: i32, reader: *mut GraphReader) -> Option<Self> {
        let plc = unsafe { (*document).getPLCField() }?;
        let plc = unsafe { &*plc };
        // java PLC.Node node = plcField.findUpper(start);
        let node = plc.super_.super_.findUpper(start);
        if !is_field(&node) {
            return None;
        }
        let field = field_of(&node);
        // java if (fieldNode.getCP() < end)
        if field.getCP() >= end {
            return None;
        }
        // java PLC.Node begin = plcField.begin();
        let mut begin = plc.super_.super_.begin();
        if field.getLevel() == 1 {
            if field.isFieldBegin() {
                return Some(Self::from_node(field, end, 0, -1, -1, -1, reader));
            }
            if let Some(start_node) = field.getStartNode() {
                begin = start_node.super_;
            }
        }
        Self::create_multi(plc, begin, start, end, reader)
    }

    // java private static FieldFilter createMultiLevelFieldFilter(...)
    fn create_multi(plc: &PLCField, mut node: Node, start: i32, end: i32, reader: *mut GraphReader) -> Option<Self> {
        let head = plc.super_.super_.getHead().as_node();
        let mut cur_level = 0;
        let mut toc_level = -1;
        let mut hyper_level = -1;
        let mut hyper_id = -1;
        while !node.as_node_ptr().is_null() && !nodes_same(node, head) {
            let field_cp = node.getCP();
            if start <= field_cp {
                if field_cp < end && is_field(&node) {
                    return Some(Self::from_node(field_of(&node), end, cur_level, toc_level, hyper_level, hyper_id, reader));
                }
                return None;
            }
            if !is_field(&node) {
                return None;
            }
            let field = field_of(&node);
            match field.getFieldChar() {
                19 => {
                    cur_level += 1;
                    let field_type = field.getFieldType();
                    if field_type == KFieldType::FieldTOC.getFlt() {
                        if toc_level == -1 {
                            toc_level = cur_level;
                        }
                    } else if field_type == KFieldType::FieldHyperlink.getFlt() && hyper_level == -1 {
                        hyper_level = cur_level;
                        // Java asks TypoDocument.getHyperlinkCache(); that cache is not a compiled type.
                        let _ = field.getEndNode();
                        hyper_id = -1;
                    }
                }
                21 => {
                    if toc_level == cur_level {
                        toc_level = -1;
                    } else if hyper_level == cur_level {
                        hyper_level = -1;
                        hyper_id = -1;
                    }
                    cur_level -= 1;
                }
                _ => {}
            }
            node = node.getNext();
        }
        None
    }

    fn from_node(node: FieldNode, end: i32, cur_level: i32, toc_level: i32, hyper_level: i32, hyper_id: i32, reader: *mut GraphReader) -> Self {
        let next_cp = node.getCP();
        let mut this = Self {
            reader,
            next: Some(node),
            next_cp,
            max: end,
            triple: FieldTriple::default(),
            cur_level,
            toc_level,
            hyper_level,
            hyper_id,
            toc_filter: None,
        };
        if this.toc_level > 0 && this.hyper_level > this.toc_level {
            this.install_toc_filter();
        }
        this
    }

    fn install_toc_filter(&mut self) {
        let doc = unsafe { (*self.reader).super_.doc_alias };
        if doc.is_null() {
            return;
        }
        unsafe { (*self.reader).addRunPropFilter(Box::new(TocRunPropFilter { doc })); }
        self.toc_filter = unsafe { (*self.reader).mRunPropFilters.last().map(|f| f.as_ref() as *const dyn IRunPropFilter) };
    }

    fn note_hyperlink(&mut self, end: Option<FieldNode>) {
        let _ = end;
        if self.hyper_level == -1 {
            self.hyper_level = self.cur_level;
        }
        self.hyper_id = -1;
        if self.toc_level > 0 && self.toc_filter.is_none() {
            self.install_toc_filter();
        }
    }

    fn clear_hyperlink(&mut self) {
        self.hyper_level = -1;
        self.hyper_id = -1;
        if self.toc_level > 0 {
            if let Some(filter) = self.toc_filter {
                unsafe { (*self.reader).removeRunPropFilter(&*filter); }
            }
        }
    }

    fn publish_field_state(&mut self) {
        let reader = self.reader;
        if self.cur_level == 0 {
            unsafe {
                (*reader).mIsInField = false;
                (*reader).mHappylinkId = -1;
            }
        } else {
            unsafe {
                (*reader).mIsInField = true;
                (*reader).mHappylinkId = self.hyper_id;
            }
        }
    }

    fn advance(&mut self, cur: &FieldNode) {
        // java getNextFieldNode: next is kept only while it is a field node before mMax.
        let next = cur.super_.getNext();
        if !next.as_node_ptr().is_null() && !next.isEnd() && next.getCP() < self.max && is_field(&next) {
            let field = field_of(&next);
            self.next_cp = field.getCP();
            self.next = Some(field);
        } else {
            self.next = None;
        }
    }

    // java public boolean doFilter(FilterArgs args, LayoutState layoutState)
    pub fn doFilter(&mut self, args: &mut FilterArgs, layoutState: &mut LayoutState) -> bool {
        let Some(next) = self.next.clone() else {
            unsafe {
                (*self.reader).mIsInField = false;
                (*self.reader).mHappylinkId = -1;
            }
            return false;
        };
        if args.start > self.next_cp {
            let mut node = next.super_.getNext();
            loop {
                if !node.as_node_ptr().is_null() && !node.isEnd() {
                    let cp = node.getCP();
                    if cp < args.start && is_field(&node) {
                        let field = field_of(&node);
                        match field.getFieldChar() {
                            19 => {
                                self.cur_level += 1;
                                let field_type = field.getFieldType();
                                if field_type == KFieldType::FieldTOC.getFlt() {
                                    if self.toc_level == -1 {
                                        self.toc_level = self.cur_level;
                                    }
                                } else if field_type == KFieldType::FieldHyperlink.getFlt() && self.hyper_level == -1 {
                                    self.note_hyperlink(field.getEndNode());
                                }
                            }
                            21 => {
                                if self.toc_level == self.cur_level {
                                    self.toc_level = -1;
                                } else if self.hyper_level == self.cur_level {
                                    self.clear_hyperlink();
                                }
                                self.cur_level -= 1;
                            }
                            _ => {}
                        }
                        node = node.getNext();
                        continue;
                    }
                }
                if node.as_node_ptr().is_null() || node.isEnd() || node.getCP() >= self.max {
                    self.next = None;
                    unsafe {
                        (*self.reader).mIsInField = false;
                        (*self.reader).mHappylinkId = -1;
                    }
                    return false;
                }
                if is_field(&node) {
                    let field = field_of(&node);
                    self.next_cp = field.getCP();
                    self.next = Some(field);
                } else {
                    self.next = None;
                    return false;
                }
                break;
            }
        }
        let mut hit = false;
        if let Some(next) = self.next.clone() {
            if args.start == self.next_cp {
                if next.getFieldChar() == 19 {
                    hit = self.read_field(args, layoutState);
                } else if next.getFieldChar() == 21 {
                    if self.toc_level == self.cur_level {
                        self.toc_level = -1;
                    } else if self.hyper_level == self.cur_level {
                        self.clear_hyperlink();
                    }
                    args.start += 1;
                    args.next = args.start;
                    self.cur_level -= 1;
                    self.advance(&next);
                    hit = true;
                }
            } else {
                args.end = self.next_cp.min(args.end);
                args.next = args.end.min(args.next);
            }
        }
        self.publish_field_state();
        hit
    }

    // java private boolean readField(FilterArgs args, LayoutState layoutState)
    fn read_field(&mut self, args: &mut FilterArgs, layoutState: &mut LayoutState) -> bool {
        let Some(start_node) = self.next.clone() else { return false };
        let field_type = start_node.getFieldType();
        self.triple.start = start_node.clone();
        self.triple.separator = start_node.getSeparatorNode();
        self.triple.end = if self.triple.separator.is_none() {
            start_node.getEndNode().unwrap_or_else(FieldNode::new)
        } else {
            self.triple.separator.as_ref().and_then(|sep| sep.getEndNode()).unwrap_or_else(FieldNode::new)
        };
        if field_type == KFieldType::FieldEQ.getFlt() {
            let end = self.triple.end.clone();
            unsafe { (*self.reader).readEquation(&self.triple, layoutState); }
            args.next = end.getCP() + 1;
            self.advance(&end);
            return true;
        }
        let result = flow_field(&self.triple, layoutState);
        let Some(result) = result else {
            if !self.triple.end.as_node_ptr().is_null() {
                args.next = self.triple.end.getCP() + 1;
                let end = self.triple.end.clone();
                self.advance(&end);
            } else {
                args.next = args.start + 1;
                let next = start_node.clone();
                self.advance(&next);
            }
            return true;
        };
        if field_type == KFieldType::FieldShape.getFlt() {
            self.read_shape_field(&result, args, layoutState);
            return true;
        }
        if result.has_result {
            if result.update_field {
                if let Some(text) = result.result_str.as_ref().filter(|s| !s.is_empty()) {
                    let reader = self.reader;
                    let start_cp = RANGEFactory::getStart(result.field_range);
                    let end_cp = RANGEFactory::getEnd(result.field_range);
                    let para = layoutState.paraEnv.curParaPropEnv.as_ref().and_then(|env| env.paraProperty.clone());
                    let run = layoutState.document.filter(|p| !p.is_null()).and_then(|doc| {
                        unsafe { (*doc).getChpxTable().and_then(|t| IChpxTable::seek(&*t, start_cp + 1)) }
                    });
                    if let (Some(para), Some(run)) = (para, run) {
                        let revision = unsafe { (*reader).mRevisionFinal };
                        if let Some(prop) = unsafe { (*reader).mPropertyTool.as_mut() }.map(|tool| {
                            tool.getFinalRunPropert__entry_propertyset_boolean(run.as_ref(), &para, revision).clone()
                        }) {
                            unsafe { (*reader).readFieldResult(text, result.field_range, start_cp, end_cp, &prop, layoutState); }
                        }
                    }
                }
                args.next = RANGEFactory::getEnd(result.field_range);
                let end = self.triple.end.clone();
                self.advance(&end);
            } else {
                self.cur_level += 1;
                if field_type == KFieldType::FieldTOC.getFlt() {
                    if self.toc_level == -1 {
                        self.toc_level = self.cur_level;
                    }
                } else if field_type == KFieldType::FieldHyperlink.getFlt() && self.hyper_level == -1 {
                    self.note_hyperlink(Some(self.triple.end.clone()));
                }
                args.next = RANGEFactory::getStart(result.result_range);
                if let Some(sep) = self.triple.separator.clone() {
                    self.advance(&sep);
                } else {
                    self.advance(&self.triple.start.clone());
                }
            }
        } else if !self.triple.end.as_node_ptr().is_null() {
            args.next = self.triple.end.getCP() + 1;
            let end = self.triple.end.clone();
            self.advance(&end);
        } else {
            args.next = args.start + 1;
            if args.next > self.next_cp {
                self.advance(&start_node);
            }
        }
        true
    }

    // java FieldShape branch of readField: scan result chars for embed (1) and line breaks.
    fn read_shape_field(&mut self, result: &FieldFlow, args: &mut FilterArgs, layout: &mut LayoutState) {
        let Some(document) = layout.document.filter(|p| !p.is_null()) else {
            args.next = RANGEFactory::getEnd(result.field_range);
            let end = self.triple.end.clone();
            self.advance(&end);
            return;
        };
        let para = layout.paraEnv.curParaPropEnv.as_ref().and_then(|env| env.paraProperty.clone());
        let Some(para) = para else {
            args.next = RANGEFactory::getEnd(result.field_range);
            let end = self.triple.end.clone();
            self.advance(&end);
            return;
        };
        let mut cp = RANGEFactory::getStart(result.result_range);
        let max = RANGEFactory::getEnd(result.result_range);
        let reader = self.reader;
        let mut embed_run = unsafe { (*document).getChpxTable().and_then(|table| IChpxTable::seek(&*table, cp)) };
        let mut embed_range = embed_run.as_ref().map(|run| run.range()).unwrap_or(0);
        let mut embed_prop = run_prop(reader, embed_run.as_deref(), &para);
        while cp < max {
            let ch = unsafe { (*document).getTextRope() }.map(|rope| unsafe { (*rope).charAt(cp) }).unwrap_or('\0');
            if ch == '\u{1}' {
                if !RANGEFactory::contains(embed_range, cp) {
                    embed_run = unsafe { (*document).getChpxTable().and_then(|table| IChpxTable::seek(&*table, cp)) };
                    embed_range = embed_run.as_ref().map(|run| run.range()).unwrap_or(0);
                    embed_prop = run_prop(reader, embed_run.as_deref(), &para);
                }
                let shape = find_shape_id(unsafe { &mut *document }, cp);
                let start_cp = self.triple.getStartCP();
                let end_cp = self.triple.getEnd();
                if let Some(prop) = embed_prop.as_ref() {
                    unsafe { (*reader).readEmbed(cp, shape, prop, start_cp, end_cp, layout); }
                }
            } else if TypoRun::isLineBreak(ch as i32) {
                let break_para = unsafe { (*document).getPapxTable().and_then(|table| IPapxTable::seek(&*table, cp)) };
                let break_run = unsafe { (*document).getChpxTable().and_then(|table| IChpxTable::seek(&*table, cp)) };
                let para_prop = break_para.as_ref().and_then(|entry| {
                    unsafe { (*reader).mPropertyTool.as_mut() }.map(|tool| {
                        tool.getFinalParaProp__entry_boolean(entry.as_ref(), unsafe { (*reader).mRevisionFinal }).clone()
                    })
                });
                let merged = match (break_run.as_ref(), para_prop.as_ref()) {
                    (Some(run), Some(para_prop)) => unsafe { (*reader).mPropertyTool.as_mut() }.map(|tool| {
                        tool.getFinalRunPropert__entry_propertyset_boolean(run.as_ref(), para_prop, unsafe { (*reader).mRevisionFinal }).clone()
                    }),
                    _ => None,
                };
                let vanished = para_prop.as_ref().map(|prop| GraphReaderTool::isVanishedText(prop)).unwrap_or(false);
                let deleted = merged.as_ref().map(|prop| RangeTool::isRevisionDelete__propertyset(prop)).unwrap_or(false);
                if !vanished && !deleted {
                    unsafe { (*reader).readBreak(cp, false, layout); }
                    args.next = cp + 1;
                    layout.hasReadEnd = true;
                    return;
                }
            }
            cp += 1;
        }
        args.next = RANGEFactory::getEnd(result.field_range);
        let end = self.triple.end.clone();
        self.advance(&end);
    }
}

fn run_prop(reader: *mut GraphReader, run: Option<&dyn ChpxEntry>, para: &PropertySet) -> Option<PropertySet> {
    let run = run?;
    let revision = unsafe { (*reader).mRevisionFinal };
    unsafe { (*reader).mPropertyTool.as_mut() }.map(|tool| tool.getFinalRunPropert__entry_propertyset_boolean(run, para, revision).clone())
}

fn find_shape_id(doc: &mut dyn Document, cp: i32) -> Option<i32> {
    let sub = doc.asAny_mut().downcast_mut::<crate::cn::wps::moffice::writer::core::SubDocument::SubDocument>()?;
    let anchor = sub.mShapeAnchors.as_mut()?.findByCP(cp)?;
    let id = anchor.getShapeId();
    let drawing = sub.getDrawingContainer()?;
    drawing.findShapeByID(id).map(|_| id)
}

impl IFilter for FieldFilter {
    fn doFilter(&mut self, args: &mut FilterArgs, layoutState: &mut LayoutState) -> bool {
        FieldFilter::doFilter(self, args, layoutState)
    }
}

struct FieldFlow {
    field_range: i64,
    result_range: i64,
    result_str: Option<String>,
    update_field: bool,
    has_result: bool,
}

// java FieldLayouter.flow for the traits that do not force a live update, plus the
// page/numpages/section-pages update that FieldtraitsFactory enables for stories 2 and 6.
fn flow_field(triple: &FieldTriple, layout: &mut LayoutState) -> Option<FieldFlow> {
    if triple.end.as_node_ptr().is_null() {
        return None;
    }
    let field_type = triple.getFieldType();
    let mut result = FieldFlow {
        field_range: RANGEFactory::getRANGE(triple.getStartCP(), triple.getEndCP() + 1),
        result_range: 0,
        result_str: None,
        update_field: false,
        has_result: false,
    };
    let ktype = KFieldType::getKFieldType(field_type);
    let doc_type = layout.document.filter(|p| !p.is_null()).map(|d| unsafe { (*d).getType__on_IDocument() }).unwrap_or(0);
    if ktype.is_some() && triple.hasSep() && !triple.isLocked() && needs_render_update(field_type, doc_type) {
        if let Some(kind) = ktype {
            let mut parser = ParserFactory::getFieldParser(kind);
            let arg = doc_type;
            if let Some(document) = layout.document.filter(|p| !p.is_null()) {
                parser.parse(triple, unsafe { &*document }, &arg);
            }
            result.result_str = parser.getResult();
            result.has_result = result.result_str.as_ref().map(|s| !s.is_empty()).unwrap_or(false);
            result.update_field = true;
            parser.dispose();
            return Some(result);
        }
    }
    if let Some(sep) = triple.separator.as_ref() {
        result.result_range = RANGEFactory::getRANGE(sep.getCP() + 1, triple.end.getCP());
        result.has_result = RANGEFactory::getLength(result.result_range) > 0;
    }
    Some(result)
}

fn needs_render_update(field_type: i32, doc_type: i32) -> bool {
    // Java FieldtraitsFactory: page, numpages and section pages update while rendering stories 2 and 6.
    let page = KFieldType::FieldPage.getFlt();
    let pages = KFieldType::FieldNumPages.getFlt();
    let section_pages = KFieldType::FieldSectionPages.getFlt();
    (field_type == page || field_type == pages || field_type == section_pages) && (doc_type == 2 || doc_type == 6)
}

fn is_field(node: &Node) -> bool {
    !node.as_node_ptr().is_null() && !node.isEnd() && !node.isHead()
}

fn field_of(node: &Node) -> FieldNode {
    FieldNode::from_node(*node)
}

// java FieldFilter.TocRunPropFilter
struct TocRunPropFilter {
    doc: *mut DocumentEnv,
}

impl IRunPropFilter for TocRunPropFilter {
    fn doFilter(&self, prop: &PropertySet, run: &dyn ChpxEntry, revision_final: bool, layout: &mut LayoutState) -> PropertySet {
        let Some(document) = layout.document.filter(|p| !p.is_null()) else { return prop.clone() };
        let Some(para) = layout.paraEnv.curParaPropEnv.as_ref().and_then(|env| env.paraProperty.as_ref()) else { return prop.clone() };
        let doc = unsafe { &mut *self.doc };
        let Some(factory) = doc.propToolFactory.as_mut() else { return prop.clone() };
        let mut tool = factory.create(unsafe { &*document });
        tool.getTocFinalRunPropert(run, para, revision_final).clone()
    }
}
