#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.MathLayouterBase
use thirdparty_font::cn::wps::font::ITypeface::ITypeface;
use util::graphics::RectF::RectF;
use util::moffice::property::PropertySet::PropertySet;
use util::moffice::util::UnitUtil::UnitUtil;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
use crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl;
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
use crate::cn::wps::moffice::writer::cache::TypoTextBox::TypoTextBox;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::IChpxTable::IChpxTable;
use crate::cn::wps::moffice::writer::data::IPapxTable::{Entry as PapxEntry, IPapxTable};
use crate::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
use crate::cn::wps::moffice::writer::data::MathTool::MathTool;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::FontStyleFactory::FontStyleFactory;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::PropertyTool::PropertyTool;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::equation::IEquationLayouter::IEquationLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::MathContentLayouter::MathContentLayouter;

pub const TOP: i32 = 1;
pub const LEFT: i32 = 1;
pub const CENTER: i32 = 2;
pub const BASELINE: i32 = 3;
pub const BOTTOM: i32 = 1;
pub const RIGHT: i32 = 1;

// java public class MathLayouterBase
pub struct MathLayouterBase {
    content: MathContentLayouter,
    scale: f32,
    pub font_size: i32,
    pub start_cp: i32,
    pub end_cp: i32,
    pub ignore_leading: bool,
    pub math_content: bool,
    main: *mut MainState,
    view: *mut ViewEnv,
    pub is_in_math_para: bool,
    cur_char: char,
    sep_graph: bool,
    // Java FunctionLayouter overrides readGraphBuff. Set only by that layouter.
    pub function_sep: bool,
    // Java GroupCharacterLayouter overrides readGraphBuff when the property set is present.
    pub group_chr: bool,
    // Java NaryLayouter overrides readGraphBuff when the property set is present.
    pub nary: bool,
}

impl MathLayouterBase {
    // java public MathLayouterBase(MainState mainState, DocumentEnv documentEnv, ViewEnv viewEnv)
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self {
            content: MathContentLayouter::MathContentLayouter(main, doc, view),
            scale: 1.0,
            font_size: UnitUtil::PT2TWIPS(10.0),
            start_cp: 0,
            end_cp: 0,
            ignore_leading: false,
            math_content: false,
            main,
            view,
            is_in_math_para: false,
            cur_char: '\0',
            sep_graph: false,
            function_sep: false,
            group_chr: false,
            nary: false,
        }
    }

    // java protected boolean init(Document document, PLCMathFunction.MathStartNode mathNode, boolean isInMathPara)
    pub fn init(&mut self, document: &dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> bool {
        self.start_cp = math_node.getStartNode().map(|n| n.getCP()).unwrap_or_else(|| math_node.getCP());
        self.end_cp = math_node.getEndNode().map(|n| n.getCP()).unwrap_or(self.start_cp);
        self.font_size = self.getFontSize(document, self.start_cp);
        self.is_in_math_para = is_in_math_para;
        true
    }

    // java public TypoDrawing obtainTypoDrawing()
    pub fn obtainTypoDrawing(&mut self) -> TypoDrawing {
        let snapshot = self.snapshot();
        let mut drawing = snapshot.getTypoTools().obtainTypoDrawing();
        drawing.alloc(snapshot);
        drawing.setDrawingType(13);
        drawing
    }

    pub fn set_sep_char(&mut self, ch: char, enabled: bool) {
        self.cur_char = ch;
        self.sep_graph = enabled;
    }

    // java protected void setMathScale(TypoDrawing drawing, float scale, int type)
    pub fn setMathScale(&mut self, drawing: &mut TypoDrawing, scale: f32, zoom_type: i32) {
        let snapshot = self.snapshot();
        IEquationLayouter::setMathScale(drawing, snapshot, scale, zoom_type);
    }

    // java protected void scaleMath(TypoDrawing drawing, float scale)
    pub fn scaleMath(&mut self, drawing: &mut TypoDrawing, scale: f32) {
        let snapshot = self.snapshot();
        let zoom = TypoDrawing::getZoom__int_typosnapshot(drawing.getPointer(), snapshot) * scale;
        drawing.setZoom(zoom);
        scale_layout_rect(drawing, snapshot, scale);
        let align = TypoDrawing::getAlignLine__int_typosnapshot(drawing.getPointer(), snapshot);
        drawing.setAlignLine((align as f32 * scale + 0.5) as i32);
    }

    // java protected TypoDrawing getTypoDrawing(Document doc, int start, int end, PropertySet propertySet)
    pub fn getTypoDrawing(&mut self, doc: *mut dyn Document, start: i32, end: i32, property_set: Option<&PropertySet>) -> Option<TypoDrawing> {
        let text_box = self.get_text_box(doc, start, end, property_set);
        if text_box == 0 { None } else { Some(self.drawing_from_textbox(text_box)) }
    }

    // java protected float getScale()
    pub fn getScale(&self) -> f32 {
        self.scale
    }

    // java mContentLayouter.getBaseLine()
    pub fn content_base_line(&self) -> i32 {
        self.content.getBaseLine()
    }

    // java protected TypoDrawing layout(Document document, int startCP, int endCP)
    pub fn layout_span(&mut self, document: *mut dyn Document, start_cp: i32, end_cp: i32) -> Option<TypoDrawing> {
        if end_cp <= start_cp { None } else { self.getTypoDrawing(document, start_cp, end_cp, None) }
    }

    // java protected TypoDrawing layout(Document document, int startCP, int endCP, boolean createNullDrawing)
    pub fn layout_range(&mut self, document: *mut dyn Document, start_cp: i32, end_cp: i32, create_null: bool) -> Option<TypoDrawing> {
        let found = if end_cp <= start_cp { None } else { self.getTypoDrawing(document, start_cp, end_cp, None) };
        if found.is_none() && create_null {
            let doc = unsafe { &*document };
            if self.font_size <= 0 {
                self.font_size = self.getFontSize(doc, start_cp);
            }
            return Some(self.getEmptyDrawing(doc, self.font_size));
        }
        found
    }

    // java protected TypoDrawing getEmptyDrawing(Document document, int size)
    pub fn getEmptyDrawing(&mut self, document: &dyn Document, size: i32) -> TypoDrawing {
        let mut drawing = self.obtainTypoDrawing();
        drawing.set(0, 0, size, size);
        self.calcBaseline(&mut drawing, document, size / 2, self.start_cp);
        drawing
    }

    // java protected void calcBaseline(TypoDrawing drawing, Document document, int alignCenter, int mainOpCP)
    pub fn calcBaseline(&mut self, drawing: &mut TypoDrawing, document: &dyn Document, align_center: i32, main_op_cp: i32) {
        let base_line = align_center + self.getEqualOpCenter(document, main_op_cp);
        drawing.setAlignType(1);
        drawing.setAlignLine(base_line);
    }

    // java protected int getEqualOpCenter(Document document, int mainOpCP)
    pub fn getEqualOpCenter(&mut self, document: &dyn Document, main_op_cp: i32) -> i32 {
        let Some(papx) = document.getPapxTable().filter(|p| !p.is_null()) else { return 0 };
        let Some(chpx) = document.getChpxTable().filter(|p| !p.is_null()) else { return 0 };
        let Some(para) = (unsafe { IPapxTable::seek(&*papx, main_op_cp) }) else { return 0 };
        let Some(run) = (unsafe { IChpxTable::seek(&*chpx, main_op_cp) }) else { return 0 };
        let doc_type = document.getType__on_IDocument();
        let (para_cache, run_cache) = if doc_type == 0 { (64, 128) } else { (4, 4) };
        let mut tool = PropertyTool::PropertyTool(document, para_cache, run_cache);
        let revision_final = unsafe { (*self.view).isRevisionFinal() };
        let para_prop = tool.getFinalParaProp(para.as_ref() as &dyn PapxEntry, revision_final).clone();
        let run_prop = tool.getFinalRunPropert(run.as_ref() as &dyn ChpxEntry, &para_prop, revision_final).clone();
        let Some(face) = FontStyleFactory::createMathTypeface(&run_prop, 0) else { return 0 };
        let size = UnitUtil::PT2TWIPS(run_prop.getFloat__int_float(10, 10.0));
        let rect = ITypeface::getTextRect_char(face.as_ref(), size as f32, '=' as u16);
        let center_y = (rect.top + rect.bottom) * 0.5;
        (center_y.abs() + 0.5) as i32
    }

    // java protected int getFontSize(Document document, int cp)
    pub fn getFontSize(&self, document: &dyn Document, cp: i32) -> i32 {
        let prop = run_property(document, cp);
        let pt = prop.as_ref().map(|p| p.getFloat__int_float(10, 10.0)).unwrap_or(10.0);
        UnitUtil::PT2TWIPS(pt)
    }

    // java protected int getColor(Document document, int cp)
    pub fn getColor(&self, document: &dyn Document, cp: i32) -> i32 {
        run_property(document, cp).map(|p| p.getInt__int_int(11, -16777216)).unwrap_or(-16777216)
    }

    // java protected char getChar(Document document, int cp)
    pub fn getChar(&self, document: &dyn Document, cp: i32) -> char {
        let mut buffer = ['\0'];
        document.getText(cp, cp + 1, &mut buffer, 0);
        buffer[0]
    }

    // java protected int getAlign(int baseJc)
    pub fn getAlign(base_jc: i32) -> i32 {
        match base_jc {
            -8 => 2,
            _ => 1,
        }
    }

    // java protected void adjustDrawingHori(TypoDrawing typoDrawing, int minDist, int algn)
    pub fn adjustDrawingHori(&mut self, drawing: &mut TypoDrawing, min_dist: i32, algn: i32) {
        if algn == 2 {
            self.adjustDrawingHoriByBaseLine(drawing, min_dist);
            return;
        }
        let height = drawing.getMaxHeight();
        let pointer = drawing.getPointer();
        let left = {
            let snapshot = self.snapshot();
            let children = TypoDrawing::getChildDrawings__int_typosnapshot(pointer, snapshot);
            let size = TypoList::size__int_typosnapshot(children, snapshot);
            let mut left = -min_dist;
            for i in 0..size {
                left += min_dist;
                let child = TypoList::quickGet__int_int_typosnapshot(i, children, snapshot);
                let h = TypoDrawing::layoutHeight__int_typosnapshot(child, snapshot);
                let top = if algn == 1 { height - h } else { 0 };
                TypoObjBase::offsetTo__int_int_int_typosnapshot(left, top, child, snapshot);
                left += TypoDrawing::layoutWidth__int_typosnapshot(child, snapshot);
            }
            left
        };
        drawing.set(0, 0, left, height);
    }

    // java protected void adjustDrawingHoriByBaseLine(TypoDrawing typoDrawing, int minDist)
    pub fn adjustDrawingHoriByBaseLine(&mut self, drawing: &mut TypoDrawing, min_dist: i32) {
        let pointer = drawing.getPointer();
        let (max_ascent, max_descent) = {
            let snapshot = self.snapshot();
            let children = TypoDrawing::getChildDrawings__int_typosnapshot(pointer, snapshot);
            if TypoList::size__int_typosnapshot(children, snapshot) <= 0 {
                return;
            }
            let size = TypoList::size__int_typosnapshot(children, snapshot);
            let mut left = -min_dist;
            let mut max_ascent = 0;
            let mut max_descent = 0;
            for i in 0..size {
                left += min_dist;
                let child = TypoList::quickGet__int_int_typosnapshot(i, children, snapshot);
                let ascent = TypoDrawing::getAlignLine__int_typosnapshot(child, snapshot);
                let descent = TypoDrawing::height__int(child, snapshot) - ascent;
                max_ascent = max_ascent.max(ascent);
                max_descent = max_descent.max(descent);
                TypoObjBase::offsetTo__int_int_int_typosnapshot(left, -ascent, child, snapshot);
                left += TypoDrawing::layoutWidth__int_typosnapshot(child, snapshot);
            }
            (max_ascent, max_descent)
        };
        offset_child(drawing, self.snapshot(), 0, max_ascent, true);
        drawing.setHeight(max_ascent + max_descent);
        drawing.setAlignLine(max_ascent);
        drawing.setAlignType(1);
    }

    // java static void adjustDrawingVert(TypoDrawing typoDrawing, int minDist, int algn)
    pub fn adjustDrawingVert(drawing: &mut TypoDrawing, snapshot: &mut TypoSnapshot, min_dist: i32, algn: i32) {
        let width = drawing.getMaxWidth();
        let pointer = drawing.getPointer();
        let top = {
            let children = TypoDrawing::getChildDrawings__int_typosnapshot(pointer, snapshot);
            let size = TypoList::size__int_typosnapshot(children, snapshot);
            let mut top = -min_dist;
            for i in 0..size {
                top += min_dist;
                let child = TypoList::quickGet__int_int_typosnapshot(i, children, snapshot);
                let w = TypoDrawing::layoutWidth__int_typosnapshot(child, snapshot);
                let left = if algn == 2 { (width - w) / 2 } else if algn == 1 { width - w } else { 0 };
                TypoObjBase::offsetTo__int_int_int_typosnapshot(left, top, child, snapshot);
                top += TypoDrawing::layoutHeight__int_typosnapshot(child, snapshot);
            }
            top
        };
        drawing.set(0, 0, width, top);
    }

    // java static void adjustDrawingSize(TypoDrawing typoDrawing)
    pub fn adjustDrawingSize(drawing: &mut TypoDrawing, snapshot: &mut TypoSnapshot) {
        let mut rect = RectF::new_with(0.0, 0.0, 0.0, 0.0);
        let children = TypoDrawing::getChildDrawings__int_typosnapshot(drawing.getPointer(), snapshot);
        let size = TypoList::size__int_typosnapshot(children, snapshot);
        for i in 0..size {
            let child = TypoList::quickGet__int_int_typosnapshot(i, children, snapshot);
            let zoom = TypoDrawing::getZoom__int_typosnapshot(child, snapshot);
            let left = TypoObjBase::getLeft__int_typosnapshot(child, snapshot) as f32;
            let top = TypoObjBase::getTop__int_typosnapshot(child, snapshot) as f32;
            let right = left + TypoDrawing::width__int(child, snapshot) as f32 * zoom;
            let bottom = top + TypoDrawing::height__int(child, snapshot) as f32 * zoom;
            rect.union(left, top, right, bottom);
        }
        drawing.set(0, 0, rect.width() as i32, rect.height() as i32);
    }

    // java protected void removeStrikeThroughs(TypoDrawing sepDrawing)
    pub fn removeStrikeThroughs(&mut self, drawing: &TypoDrawing) {
        let snapshot = self.snapshot();
        let text_box = TypoDrawing::getTextbox__int_typosnapshot(drawing.getPointer(), snapshot);
        if text_box == 0 {
            return;
        }
        let lines = TypoTextBox::getLines__int_typosnapshot(text_box, snapshot);
        let line = TypoList::first__int_typosnapshot(lines, snapshot);
        crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::setLines__int_int_typosnapshot(0, line, snapshot);
    }

    fn get_text_box(&mut self, doc: *mut dyn Document, start: i32, end: i32, property_set: Option<&PropertySet>) -> i32 {
        let mut env = self.content.createLayoutState();
        env.document = Some(doc);
        env.setRange__int_int(start, end);
        env.cp = start;
        env.styleResult.ignoreBdr = true;
        env.isInMathPara = self.is_in_math_para;
        self.read_graph_buff(&mut env, property_set);
        env.pageEnv.width = i32::MAX;
        self.content.setIgnoreLeading(self.ignore_leading);
        self.content.setMathContent(self.math_content);
        let text_box = self.content.flow(&mut env);
        if text_box == 0 {
            return 0;
        }
        let font_size = self.font_size;
        let snapshot = self.snapshot();
        let mut max_right = -1;
        let mut min_left = TypoTextBox::getCoreWidth__int_typosnapshot(text_box, snapshot);
        let lines = TypoTextBox::getLines__int_typosnapshot(text_box, snapshot);
        let size = TypoList::size__int_typosnapshot(lines, snapshot);
        for i in 0..size {
            let line = TypoList::quickGet__int_int_typosnapshot(i, lines, snapshot);
            if TypoMemoryObj::getType__int_typosnapshot(line, snapshot) == 3 {
                min_left = min_left.min(TypoObjBase::getLeft__int_typosnapshot(line, snapshot));
                max_right = max_right.max(TypoObjBase::getRight__int_typosnapshot(line, snapshot));
            }
        }
        let width = if min_left < max_right && max_right >= 0 { max_right - min_left } else { font_size };
        TypoTextBox::setWidth__int_int_typosnapshot(width, text_box, snapshot);
        text_box
    }

    fn read_graph_buff(&mut self, env: &mut LayoutState, property_set: Option<&PropertySet>) {
        if self.sep_graph && property_set.is_some() {
            let text = self.cur_char.to_string();
            let cp = env.getRangeStart();
            if let Some(doc) = env.document {
                env.addMathString(Some(&text), unsafe { &*doc }, cp);
            }
            return;
        }
        // java FunctionLayouter.readGraphBuff
        if self.group_chr {
            if let Some(prop) = property_set {
                let gpr = crate::cn::wps::moffice::writer::data::math::GroupChrProp::GroupChrProp::from_property(prop);
                let text = gpr.chr.to_string();
                if let Some(doc) = env.document.filter(|p| !p.is_null()) {
                    env.addMathString(Some(&text), unsafe { &*doc }, env.getRangeStart());
                }
                return;
            }
        }
        // java NaryLayouter.readGraphBuff
        if self.nary {
            if let Some(prop) = property_set {
                let npr = crate::cn::wps::moffice::writer::data::math::NaryProp::NaryProp::from_property(prop);
                let text = npr.chr.to_string();
                if let Some(doc) = env.document.filter(|p| !p.is_null()) {
                    env.addMathString(Some(&text), unsafe { &*doc }, env.getRangeStart());
                }
                return;
            }
        }
        if self.function_sep {
            if let Some(doc) = env.document.filter(|p| !p.is_null()) {
                let sep_cp = MathTool::getMathSepaCP(unsafe { &*doc }, self.start_cp);
                if sep_cp < self.end_cp && sep_cp > self.start_cp + 1 {
                    let revision = unsafe { (*self.view).isRevisionFinal() };
                    env.initCoreContent(env.getRangeEnd(), revision);
                    self.content.initFilter(env.document, env.cp, env.getRangeEnd(), env);
                    self.content.addFunctionSepField(sep_cp);
                    return;
                }
            }
        }
        let revision = unsafe { (*self.view).isRevisionFinal() };
        env.initCoreContent(env.getRangeEnd(), revision);
        self.content.initFilter(env.document, env.cp, env.getRangeEnd(), env);
    }

    fn drawing_from_textbox(&mut self, text_box: i32) -> TypoDrawing {
        let mut drawing = self.obtainTypoDrawing();
        let align_line = self.content.getBaseLine();
        let align_type = self.content.getAlignType();
        let snapshot = self.snapshot();
        TypoTextBox::setParent__int_int_typosnapshot(drawing.getPointer(), text_box, snapshot);
        drawing.setTextbox(text_box);
        drawing.setLeft(0);
        drawing.setTop(0);
        drawing.setHeight(TypoTextBox::height__int_typosnapshot(text_box, snapshot));
        drawing.setWidth(TypoTextBox::width__int_typosnapshot(text_box, snapshot));
        drawing.setAlignLine(align_line);
        drawing.setAlignType(align_type);
        drawing
    }

    pub fn snapshot(&mut self) -> &mut TypoSnapshot {
        unsafe { &mut (*self.main).snapshot }
    }

    pub fn recycle_drawing(&mut self, drawing: &mut TypoDrawing) {
        self.snapshot().getTypoTools().recycle_drawing(drawing);
    }
}

fn run_property(document: &dyn Document, cp: i32) -> Option<PropertySet> {
    let table = document.getChpxTable()?;
    if table.is_null() {
        return None;
    }
    let entry = unsafe { IChpxTable::seek(&*table, cp) }?;
    ChpxEntry::getProperty(entry.as_ref()).cloned()
}

fn scale_layout_rect(drawing: &mut TypoDrawing, snapshot: &mut TypoSnapshot, scale: f32) {
    let pad = drawing.getLayoutPadding();
    let left = drawing.getLeft();
    let top = drawing.getTop();
    let right = drawing.getRight();
    let bottom = drawing.getBottom();
    let pad_left = TypoRectMemoryImpl::getLeft__int_typosnapshot(pad, snapshot);
    let pad_top = TypoRectMemoryImpl::getTop__int_typosnapshot(pad, snapshot);
    let pad_right = TypoRectMemoryImpl::getRight__int_typosnapshot(pad, snapshot);
    let pad_bottom = TypoRectMemoryImpl::getBottom__int_typosnapshot(pad, snapshot);
    let rect_left = ((left - pad_left) as f32 * scale) as i32;
    let rect_top = ((top - pad_top) as f32 * scale) as i32;
    let rect_right = ((right + pad_right) as f32 * scale) as i32;
    let rect_bottom = ((bottom + pad_bottom) as f32 * scale) as i32;
    TypoRectMemoryImpl::setLeft__int_int_typosnapshot(left - rect_left, pad, snapshot);
    TypoRectMemoryImpl::setTop__int_int_typosnapshot(top - rect_top, pad, snapshot);
    TypoRectMemoryImpl::setRight__int_int_typosnapshot(rect_right - right, pad, snapshot);
    TypoRectMemoryImpl::setBottom__int_int_typosnapshot(rect_bottom - bottom, pad, snapshot);
}

pub(crate) fn offset_drawing_to(pointer: i32, x: i32, y: i32, snapshot: &mut TypoSnapshot) {
    // java static void offsetDrawingTo(int typoDrawing, int x, int y, TypoSnapshot snapshot)
    if pointer != 0 {
        TypoObjBase::offsetTo__int_int_int_typosnapshot(x, y, pointer, snapshot);
    }
}

pub(crate) fn offset_child(drawing: &mut TypoDrawing, snapshot: &mut TypoSnapshot, dx: i32, dy: i32, adjust_parent: bool) {
    let children = TypoDrawing::getChildDrawings__int_typosnapshot(drawing.getPointer(), snapshot);
    let size = TypoList::size__int_typosnapshot(children, snapshot);
    let mut width = 0;
    let mut height = 0;
    for i in 0..size {
        let child = TypoList::quickGet__int_int_typosnapshot(i, children, snapshot);
        TypoObjBase::offset__int_int_int_typosnapshot(dx, dy, child, snapshot);
        if adjust_parent {
            let child_width = TypoObjBase::getLeft__int_typosnapshot(child, snapshot) + TypoDrawing::layoutWidth__int_typosnapshot(child, snapshot);
            let child_height = TypoObjBase::getTop__int_typosnapshot(child, snapshot) + TypoDrawing::layoutHeight__int_typosnapshot(child, snapshot);
            width = width.max(child_width);
            height = height.max(child_height);
        }
    }
    if adjust_parent {
        drawing.setWidth(width);
        drawing.setHeight(height);
    }
}
