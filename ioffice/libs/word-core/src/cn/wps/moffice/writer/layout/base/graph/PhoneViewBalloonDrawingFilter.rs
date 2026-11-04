#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.graph.PhoneViewBalloonDrawingFilter
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::IChpxTable::{IChpxTable, Entry as ChpxEntry};
use crate::cn::wps::moffice::writer::data::PLC::nodes_same;
use crate::cn::wps::moffice::writer::data::PLCField::FieldTriple;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::{is_math_start, math_start_from, MathStartNode};
use crate::cn::wps::moffice::writer::data::PLCMathStart::{load_start, OMathStartNode};
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::graph::FilterArgs::FilterArgs;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphReader::GraphReader;
use crate::cn::wps::moffice::writer::layout::base::graph::IFilter::IFilter;
use crate::cn::wps::moffice::writer::VersionState::VersionState;
use stubs::java::lang::Object::Object;

enum NextMath {
    Function(MathStartNode),
    Other(i32),
    OMath(OMathStartNode),
}

impl NextMath {
    fn cp(&self) -> i32 {
        match self {
            NextMath::Function(node) => node.getCP(),
            NextMath::Other(cp) => *cp,
            NextMath::OMath(node) => node.getCP(),
        }
    }

    fn end_cp(&self) -> i32 {
        match self {
            // java MathStartNode: end = getEndNode().getCP() + 1
            NextMath::Function(node) => node.getEndNode().map(|end| end.getCP() + 1).unwrap_or(node.getCP() + 1),
            // java MathNode: end = getCP() + 1
            NextMath::Other(cp) => cp + 1,
            // java OMathStartNode: end = getEnd().getCP()
            NextMath::OMath(node) => node.getEnd().map(|end| end.getCP()).unwrap_or(node.getCP() + 1),
        }
    }
}

// java public class PhoneViewBalloonDrawingFilter implements IFilter
pub struct PhoneViewBalloonDrawingFilter {
    reader: *mut GraphReader,
    view: *mut ViewEnv,
    max_cp: i32,
    is_revision: bool,
    is_in_embed_balloon: bool,
    m_start: i32,
    m_end: i32,
}

impl PhoneViewBalloonDrawingFilter {
    // java PhoneViewBalloonDrawingFilter(GraphReader, LayoutState, int start, int end, ViewEnv)
    pub fn new(
        reader: *mut GraphReader,
        is_revision: bool,
        is_in_embed_balloon: bool,
        document: Option<*mut dyn Document>,
        start: i32,
        end: i32,
        view: *mut ViewEnv,
    ) -> Self {
        let mut this = Self {
            reader,
            view,
            max_cp: end,
            is_revision,
            is_in_embed_balloon,
            m_start: -1,
            m_end: -1,
        };
        this.pull(document, start);
        this
    }

    fn pull(&mut self, document: Option<*mut dyn Document>, start_cp: i32) {
        self.m_start = -1;
        self.m_end = -1;
        let Some(document) = document.filter(|p| !p.is_null()) else { return };
        if let Some(node) = next_math(unsafe { &*document }, start_cp, self.max_cp) {
            self.m_start = node.cp();
            self.m_end = node.end_cp();
            if self.m_start > start_cp {
                self.scan_drawing(document, start_cp, self.m_start);
            }
        } else {
            self.scan_drawing(document, start_cp, self.max_cp);
        }
    }

    // java private void getDrawingStartCP(Document document, int start, int maxCP)
    fn scan_drawing(&mut self, document: *mut dyn Document, start: i32, max_cp: i32) {
        let doc = unsafe { &mut *document };
        let Some(mut run) = doc.getChpxTable().and_then(|table| IChpxTable::seek(unsafe { &*table }, start)) else { return };
        let mut cur = start;
        while cur < max_cp && !run.isEnd() {
            let is_drawing = ChpxEntry::getProperty(run.as_ref()).map(|prop| prop.getBoolean__int_boolean(28, false)).unwrap_or(false);
            if is_drawing {
                let ch = doc.charAt(cur);
                if ch == '\u{8}' || ch == '\u{1}' {
                    // java if (KShape.isPenkit(ole, document)) { ++curCP; continue; }
                    if is_penkit(doc, cur) {
                        cur += 1;
                        continue;
                    }
                    let field = doc.getPLCField().and_then(|plc| unsafe { (*plc).getContainingField(cur) });
                    if !self.is_revision && !VersionState::isInRomReadMode() {
                        if let Some(id) = find_shape_id(doc, cur) {
                            // View settings are absent, so isDisplayPicInComment is not true.
                            let _ = (id, shape_is_picture(doc, id));
                        }
                    }
                    let at = field.as_ref().map(|field| field.getStartCP()).unwrap_or(cur);
                    if !self.is_revision && !VersionState::isInRomReadMode() {
                        if let Some(id) = find_shape_id(doc, at) {
                            if shape_has_ink(doc, id) {
                                cur += 1;
                                continue;
                            }
                        }
                    }
                    if let Some(field) = field {
                        self.m_start = at;
                        self.m_end = field.getEnd();
                    } else {
                        self.m_start = at;
                        self.m_end = at + 1;
                    }
                    break;
                }
            }
            let Some(next) = ChpxEntry::getNext(run.as_ref()) else { break };
            cur = if next.isEnd() { doc.getLength() } else { next.position() };
            run = next;
        }
    }
}

impl IFilter for PhoneViewBalloonDrawingFilter {
    // java public boolean doFilter(FilterArgs args, LayoutState layoutState)
    fn doFilter(&mut self, args: &mut FilterArgs, layout: &mut LayoutState) -> bool {
        if self.m_start < 0 {
            return false;
        }
        if args.start > self.m_start {
            self.pull(layout.document, args.start);
            if self.m_start < 0 {
                return false;
            }
        }
        if args.start == self.m_start {
            let (text, prop) = if self.view.is_null() {
                (String::new(), util::moffice::property::PropertySet::PropertySet::empty())
            } else {
                let view = unsafe { &*self.view };
                (view.getSubstituteForDrawing(), view.getBalloonContentProperty())
            };
            unsafe { (*self.reader).readDrawingFlag(self.m_start, self.m_end, &text, &prop, layout); }
            args.next = self.m_end;
            true
        } else {
            args.end = args.end.min(self.m_start);
            args.next = args.next.min(self.m_start);
            false
        }
    }
}

fn next_math(document: &dyn Document, start: i32, max_cp: i32) -> Option<NextMath> {
    let function = document.getPLCMathFunction().filter(|p| !p.is_null()).and_then(|plc| {
        let plc = unsafe { &*plc };
        let head = plc.super_.super_.getHead().as_node();
        let node = plc.super_.super_.findUpper(start);
        if node.ptr.is_null() || nodes_same(node, head) || node.getCP() >= max_cp {
            None
        } else if is_math_start(node) {
            math_start_from(node).map(NextMath::Function)
        } else {
            Some(NextMath::Other(node.getCP()))
        }
    });
    let omath = document.getOMathStarts().filter(|p| !p.is_null()).and_then(|starts| {
        let starts = unsafe { &*starts };
        let node = starts.super_.findUpper(start);
        let head = starts.super_.getHead().as_node();
        if node.ptr.is_null() || nodes_same(node, head) || node.getCP() >= max_cp {
            None
        } else {
            Some(NextMath::OMath(load_start(node, true)))
        }
    });
    match (function, omath) {
        (None, None) => None,
        (Some(node), None) | (None, Some(node)) => Some(node),
        (Some(function), Some(omath)) => {
            if omath.cp() <= function.cp() { Some(omath) } else { Some(function) }
        }
    }
}

// java KShape.isPenkit(OLE, Document) with a null brand.
fn is_penkit(doc: &mut dyn Document, cp: i32) -> bool {
    let Some(prog) = ole_prog_at(doc, cp) else { return false };
    if prog.to_lowercase().ends_with("penkit") {
        return true;
    }
    // java prog.equalsIgnoreCase("Package") looks up the ole file name.
    // A missing name returns false.
    if prog.eq_ignore_ascii_case("Package") {
        return false;
    }
    false
}

fn ole_prog_at(doc: &mut dyn Document, cp: i32) -> Option<String> {
    let plc = doc.getPLCField()?;
    let field: FieldTriple = unsafe { (*plc).getMatchCpField(cp) }?;
    let run = doc.getChpxTable().and_then(|table| IChpxTable::seek(unsafe { &*table }, field.getSepCP()))?;
    if run.isEnd() {
        return None;
    }
    let prop = ChpxEntry::getProperty(run.as_ref())?;
    let value = prop.getValue(56)?;
    let shape = value.as_ref().as_any().downcast_ref::<drawing::cn::wps::moffice::drawing::Shape::Shape>()?;
    // java shape.getOLE() reads property 863.
    let ole = shape.base.mProperty.as_ref()?.base.getValue(863)?;
    if let Ok(ole) = ole.clone().downcast::<drawing::cn::wps::moffice::drawing::OLE::OLE>() {
        return Some(ole.getProgId());
    }
    if let Ok(ole) = ole.clone().downcast::<std::sync::Mutex<drawing::cn::wps::moffice::drawing::OLE::OLE>>() {
        return Some(ole.lock().expect("ole").getProgId());
    }
    None
}

fn find_shape_id(doc: &mut dyn Document, cp: i32) -> Option<i32> {
    let sub = doc.asAny_mut().downcast_mut::<crate::cn::wps::moffice::writer::core::SubDocument::SubDocument>()?;
    let anchor = sub.mShapeAnchors.as_mut()?.findByCP(cp)?;
    let id = anchor.getShapeId();
    sub.getDrawingContainer()?.findShapeByID(id).map(|_| id)
}

fn with_shape<T>(doc: &mut dyn Document, shape_id: i32, f: impl FnOnce(&mut drawing::cn::wps::moffice::drawing::Shape::Shape) -> T) -> Option<T> {
    let sub = doc.asAny_mut().downcast_mut::<crate::cn::wps::moffice::writer::core::SubDocument::SubDocument>()?;
    let element = sub.getDrawingContainer()?.findShapeByID(shape_id)?;
    let shape = element.downcast_ref::<std::sync::Mutex<drawing::cn::wps::moffice::drawing::Shape::Shape>>()?;
    let mut shape = shape.lock().expect("shape lock");
    Some(f(&mut shape))
}

fn shape_is_picture(doc: &mut dyn Document, shape_id: i32) -> bool {
    with_shape(doc, shape_id, |shape| shape.isPicture()).unwrap_or(false)
}

fn shape_has_ink(doc: &mut dyn Document, shape_id: i32) -> bool {
    with_shape(doc, shape_id, |shape| shape.getInkModel().is_some()).unwrap_or(false)
}
