#![allow(dead_code, non_snake_case, unused_variables)]
// Java: cn.wps.moffice.writer.layout.base.math.MathLayouter
use stubs::java::lang::Object::Object;
use util::moffice::property::PropertySet::PropertySet;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::cache::TypoTextBox::TypoTextBox;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::DefaultSectionProperties::DefaultSectionProperties;
use crate::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
use crate::cn::wps::moffice::writer::data::PLC::nodes_same;
use crate::cn::wps::moffice::writer::data::PLC::Node;
use crate::cn::wps::moffice::writer::data::PLCSection::{as_section, PLCSection};
use crate::cn::wps::moffice::writer::data::table::TableInfos::TableInfos;
use crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup;
use crate::cn::wps::moffice::writer::data::value::Revision::Revision;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
use crate::cn::wps::moffice::writer::layout::base::env::PropertyTool::PropertyTool;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphBuff::MathEmbed;
use crate::cn::wps::moffice::writer::layout::base::math::AccentLayouter::AccentLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::ArrayLayouter::ArrayLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::BarLayouter::BarLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::BorderBoxLayouter::BorderBoxLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::BoxLayouter::BoxLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::DelimiterLayouter::DelimiterLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::FractionLayouter::FractionLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::FunctionLayouter::FunctionLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::GroupCharacterLayouter::GroupCharacterLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::LimLowerLayouter::LimLowerLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::LimUpperLayouter::LimUpperLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::MatrixLayouter::MatrixLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::NaryLayouter::NaryLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::RadLayouter::RadLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::PhantLayouter::PhantLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::SPreLayouter::SPreLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::SSubLayouter::SSubLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::SSubSupLayouter::SSubSupLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::SSupLayouter::SSupLayouter;
use crate::cn::wps::moffice::writer::layout::base::math::MathContentLayouter::MathContentLayouter;

// java public class MathLayouter
pub struct MathLayouter;

impl MathLayouter {
    // java public static int getMath(MainState mainState, DocumentEnv documentEnv, ViewEnv viewEnv, LayoutState layoutState, PLC.Node mathNode)
    pub fn getMath(
        main_state: &mut MainState,
        document_env: &mut DocumentEnv,
        view_env: &mut ViewEnv,
        layout_state: &LayoutState,
        math_node: &MathEmbed,
    ) -> i32 {
        let is_in_math_para = if layout_state.isMathContentState() {
            layout_state.isInMathPara
        } else {
            layout_state.isInMathPara()
        };
        let drawing = Self::getDrawing(
            main_state,
            document_env,
            view_env,
            layout_state.document,
            is_in_math_para,
            math_node,
        );
        if drawing == 0 {
            0
        } else {
            let snapshot = &mut main_state.snapshot;
            let mut drawing_obj = snapshot.getTypoTools().obtainTypoDrawing__int(drawing);
            drawing_obj.setIsRotateWithPage(true);
            let pointer = drawing_obj.getPointer();
            snapshot.getTypoTools().recycle_drawing(&mut drawing_obj);
            pointer
        }
    }

    // java static TypoDrawing getDrawing(...)
    fn getDrawing(
        main_state: &mut MainState,
        document_env: &mut DocumentEnv,
        view_env: &mut ViewEnv,
        document: Option<*mut dyn Document>,
        is_in_math_para: bool,
        math_node: &MathEmbed,
    ) -> i32 {
        match math_node {
            MathEmbed::Function { math_type, start_cp, end_cp, .. } => {
                if MathLayouterFactory::kind(*math_type).is_none() || *end_cp <= *start_cp {
                    return 0;
                }
                let Some(doc) = document else { return 0 };
                let Some(plc) = (unsafe { (*doc).getPLCMathFunction() }).filter(|p| !p.is_null()) else { return 0 };
                let Some(node) = (unsafe { (*plc).findMathStartNodeByCP(*start_cp) }) else { return 0 };
                let main_ptr = main_state as *mut MainState;
                let doc_ptr = document_env as *mut DocumentEnv;
                let view_ptr = view_env as *mut ViewEnv;
                let laid_out = match *math_type {
                    1 => {
                        let mut layouter = DelimiterLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    2 => {
                        let mut layouter = AccentLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    3 => {
                        let mut layouter = BarLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    4 => {
                        let mut layouter = BoxLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    5 => {
                        let mut layouter = BorderBoxLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    7 => {
                        let mut layouter = ArrayLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    8 => {
                        let mut layouter = FractionLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    9 => {
                        let mut layouter = FunctionLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    10 => {
                        let mut layouter = GroupCharacterLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    11 => {
                        let mut layouter = LimLowerLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    12 => {
                        let mut layouter = LimUpperLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    13 => {
                        let mut layouter = MatrixLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    14 => {
                        let mut layouter = NaryLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    15 => {
                        let mut layouter = PhantLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    16 => {
                        let mut layouter = RadLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    17 => {
                        let mut layouter = SPreLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    18 => {
                        let mut layouter = SSubLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    19 => {
                        let mut layouter = SSubSupLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    20 => {
                        let mut layouter = SSupLayouter::new(main_ptr, doc_ptr, view_ptr);
                        layouter.layout(doc, &node, is_in_math_para)
                    }
                    _ => return 0,
                };
                let Some(mut drawing) = laid_out else { return 0 };
                drawing.setEmbed(true);
                drawing.setIsRotateWithPage(true);
                let pointer = drawing.getPointer();
                main_state.snapshot.getTypoTools().recycle_drawing(&mut drawing);
                pointer
            }
            MathEmbed::OMath { start_cp, end_cp, in_para } => {
                Self::layout_omath(main_state, document_env, view_env, document, *start_cp, *end_cp, *in_para)
            }
            MathEmbed::Other => 0,
        }
    }

    // java PLCMathStart.OMathStartNode branch of getDrawing
    fn layout_omath(
        main_state: &mut MainState,
        document_env: &mut DocumentEnv,
        view_env: &mut ViewEnv,
        document: Option<*mut dyn Document>,
        start_cp: i32,
        end_cp: i32,
        in_para: bool,
    ) -> i32 {
        let main_ptr = main_state as *mut MainState;
        let doc_ptr = document_env as *mut DocumentEnv;
        let view_ptr = view_env as *mut ViewEnv;
        let mut layouter = MathContentLayouter::MathContentLayouter(main_ptr, doc_ptr, view_ptr);
        let mut env = layouter.createLayoutState();
        layouter.setUseAlignOMathParaHandler(true);
        env.isInMathPara = in_para;
        env.document = document;
        calc_page_env(document_env, view_env, &mut env.pageEnv, env.document, start_cp, end_cp);
        env.setRange__int_int(start_cp, end_cp);
        env.cp = start_cp;
        let revision = view_env.isRevisionFinal();
        env.initCoreContent(end_cp, revision);
        layouter.initFilter(env.document, env.cp, env.getRangeEnd(), &mut env);
        layouter.setNeedCollectHighlight(true);
        let text_box = layouter.flow(&mut env);
        let align_type = layouter.getAlignType();
        let align_line = layouter.getBaseLine();
        if text_box == 0 {
            return 0;
        }
        let core_width = env.pageEnv.getCoreWidth();
        let snapshot = &mut main_state.snapshot;
        let lines = TypoTextBox::getLines__int_typosnapshot(text_box, snapshot);
        let line_size = TypoList::size__int_typosnapshot(lines, snapshot);
        if line_size > 0 {
            if line_size == 1 {
                let first = TypoList::first__int_typosnapshot(lines, snapshot);
                TypoTextBox::setWidth__int_int_typosnapshot(
                    TypoObjBase::width__int_typosnapshot(first, snapshot),
                    text_box,
                    snapshot,
                );
                TypoTextBox::setHeight__int_int_typosnapshot(
                    TypoObjBase::height__int_typosnapshot(first, snapshot),
                    text_box,
                    snapshot,
                );
            } else {
                let last = TypoList::get__int_int_typosnapshot(line_size - 1, lines, snapshot);
                TypoTextBox::setWidth__int_int_typosnapshot(core_width, text_box, snapshot);
                TypoTextBox::setHeight__int_int_typosnapshot(
                    TypoObjBase::getBottom__int_typosnapshot(last, snapshot),
                    text_box,
                    snapshot,
                );
            }
        }
        let mut drawing = snapshot.getTypoTools().obtainTypoDrawing();
        drawing.alloc(snapshot);
        drawing.setDrawingType(14);
        drawing.setTextbox(text_box);
        drawing.setWidth(TypoTextBox::width__int_typosnapshot(text_box, snapshot));
        drawing.setHeight(TypoTextBox::height__int_typosnapshot(text_box, snapshot));
        drawing.setEmbed(true);
        drawing.setIsRotateWithPage(true);
        drawing.setAlignType(align_type);
        drawing.setAlignLine(align_line);
        let pointer = drawing.getPointer();
        snapshot.getTypoTools().recycle_drawing(&mut drawing);
        pointer
    }
}

// java private static void calcPageEnv(...)
fn calc_page_env(
    doc_env: &mut DocumentEnv,
    view_env: &ViewEnv,
    page_env: &mut PageEnv,
    doc: Option<*mut dyn Document>,
    start_cp: i32,
    _end_cp: i32,
) {
    let Some(doc) = doc.filter(|p| !p.is_null()) else { return; };
    let in_table = {
        let mut tool = PropertyTool::PropertyTool(unsafe { &*doc }, 4, 4);
        let table = unsafe { (*doc).getPapxTable() };
        if let Some(table) = table.filter(|p| !p.is_null()) {
            if let Some(entry) = unsafe { IPapxTable::seek(&*table, start_cp) } {
                let prop = tool.getFinalParaProp(entry.as_ref(), view_env.isRevisionFinal());
                prop.getBoolean__int_boolean(223, false)
            } else {
                false
            }
        } else {
            false
        }
    };
    if in_table {
        // Java leaves the page size unchanged when the cell lookup misses.
        let _ = apply_cell_page(page_env, unsafe { &*doc }, start_cp);
        return;
    }
    let main_cp = main_cp_for(doc_env, unsafe { &*doc }, start_cp);
    let sec = section_property(doc_env, unsafe { &*doc }, main_cp);
    let sec = merge_revision(sec, view_env.isRevisionFinal());
    let setup = page_setup(&sec);
    page_env.width = setup.getWidth() - setup.getMarginLeft() - setup.getMarginRight();
    page_env.height = setup.getHeight() - setup.getMarginTop() - setup.getMarginBottom();
}

fn apply_cell_page(page_env: &mut PageEnv, doc: &dyn Document, start_cp: i32) -> bool {
    let Some(infos) = doc.getTableInfos().filter(|p| !p.is_null()) else { return false; };
    let Some(table) = (unsafe { (*infos).getTableInfoByRange(start_cp, start_cp + 1) }) else { return false; };
    let Some(row) = table.getRowByCP(start_cp) else { return false; };
    let Some(index) = row.getCellIndexByCP(start_cp) else { return false; };
    let Some(cell) = row.getCellByCP(start_cp) else { return false; };
    page_env.width = row.getRgdxa(index + 1) - row.getRgdxa(index) - cell.margin_left() - cell.margin_right();
    page_env.height = row.getRowHeight().abs() - cell.margin_top() - cell.margin_bottom();
    true
}

fn main_cp_for(doc_env: &mut DocumentEnv, doc: &dyn Document, start_cp: i32) -> i32 {
    match doc.getType__on_IDocument() {
        0 => start_cp,
        1 => footnote_main_cp(doc, start_cp),
        2 => header_main_cp(doc_env, start_cp),
        4 => main_length(doc_env).saturating_sub(1),
        5 => textbox_main_cp(doc_env, doc, start_cp, false),
        6 => textbox_main_cp(doc_env, doc, start_cp, true),
        _ => 0,
    }
}

fn footnote_main_cp(doc: &dyn Document, start_cp: i32) -> i32 {
    let Some(txt) = doc.getFootnoteTxt().filter(|p| !p.is_null()) else { return 0; };
    let Some(node) = (unsafe { (*txt).findFootnoteEndnoteTxtContainsCP(start_cp) }) else { return 0; };
    node.getFootnoteEndnoteRef().map(|r| r.node.position()).unwrap_or(0)
}

fn header_main_cp(doc_env: &mut DocumentEnv, header_cp: i32) -> i32 {
    let Some(sections) = main_sections(doc_env) else { return 0; };
    get_main_cp(header_cp, unsafe { &*sections })
}

fn textbox_main_cp(doc_env: &mut DocumentEnv, doc: &dyn Document, start_cp: i32, header_shape: bool) -> i32 {
    let Some(textbox) = doc.getTextboxText().filter(|p| !p.is_null()) else { return 0; };
    let Some(node) = (unsafe { (*textbox).findTextboxByCP(start_cp) }) else { return 0; };
    let shape_id = node.getShapeId();
    if header_shape {
        let cp = {
            let Some(text) = doc_env.textDocument_mut() else { return 0; };
            let Some(sub) = text.getSubdocument(2) else { return 0; };
            let Some(anchors) = Document::getShapeAnchors(sub).filter(|p| !p.is_null()) else { return 0; };
            let Some(anchor) = (unsafe { (*anchors).findById(shape_id) }) else { return 0; };
            anchor.getCP()
        };
        return header_main_cp(doc_env, cp);
    }
    let Some(main) = main_document(doc_env) else { return 0; };
    let Some(anchors) = (unsafe { (*main).getShapeAnchors() }).filter(|p| !p.is_null()) else { return 0; };
    unsafe { (*anchors).findById(shape_id) }.map(|anchor| anchor.getCP()).unwrap_or(0)
}

fn main_document(doc_env: &mut DocumentEnv) -> Option<*const dyn Document> {
    let text = doc_env.textDocument_mut()?;
    let main = text.getMainDocumentMut()?;
    Some(main as *const dyn Document)
}

fn main_sections(doc_env: &mut DocumentEnv) -> Option<*mut PLCSection> {
    let main = main_document(doc_env)?;
    unsafe { (*main).getPLCSection() }
}

fn main_length(doc_env: &mut DocumentEnv) -> i32 {
    main_document(doc_env).map(|main| unsafe { (*main).getLength() }).unwrap_or(0)
}

fn section_property(doc_env: &mut DocumentEnv, doc: &dyn Document, main_cp: i32) -> PropertySet {
    let sections = main_sections(doc_env).or_else(|| doc.getPLCSection());
    if let Some(sections) = sections.filter(|p| !p.is_null()) {
        if let Some(node) = unsafe { (*sections).getSection(main_cp) } {
            return node.getPropertySet();
        }
    }
    DefaultSectionProperties::getProperties()
}

fn merge_revision(sec: PropertySet, revision_final: bool) -> PropertySet {
    if !revision_final {
        return sec;
    }
    let Some(value) = sec.getValue(727) else { return sec; };
    let Some(revision) = value.as_ref().as_any().downcast_ref::<Revision>() else { return sec; };
    let Some(extra) = revision.getProperty() else { return sec; };
    PropertySet::merge(&sec, &extra)
}

fn page_setup(prop: &PropertySet) -> PageSetup {
    prop.getValue(676)
        .and_then(|value| value.as_ref().as_any().downcast_ref::<PageSetup>().copied())
        .unwrap_or_else(|| PageSetup::PageSetup(11906, 16838, 1800, 1800, 1440, 1440))
}

fn get_main_cp(header_cp: i32, sections: &PLCSection) -> i32 {
    let head = sections.getHead();
    let first = head.getNext();
    if nodes_same(first, head.as_node()) || contain_header_cp(first, header_cp) {
        return 0;
    }
    let mut node = first.getNext();
    while !nodes_same(node, head.as_node()) && !node.as_node_ptr().is_null() {
        if contain_header_cp(node, header_cp) {
            return node.getCP() + 1;
        }
        node = node.getNext();
    }
    0
}

fn contain_header_cp(node: Node, cp: i32) -> bool {
    let section = as_section(node);
    cp >= section.getOddHeader().getCP()
}

// java public class MathLayouterFactory
pub struct MathLayouterFactory;

impl MathLayouterFactory {
    // java switch (type): 6 and default return null; 5 falls through after constructing BorderBoxLayouter.
    pub fn kind(math_type: i32) -> Option<i32> {
        match math_type {
            1 | 2 | 3 | 4 | 5 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => Some(math_type),
            _ => None,
        }
    }
}
