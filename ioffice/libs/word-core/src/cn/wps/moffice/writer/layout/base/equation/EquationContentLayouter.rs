#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.equation.EquationContentLayouter
// Java: cn.wps.moffice.writer.layout.base.equation.IEquationLayouter
use std::sync::Arc;
use util::moffice::property::PropertySet::PropertySet;
use stubs::java::lang::Object::Object;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
use crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
use crate::cn::wps::moffice::writer::cache::TypoTextBox::TypoTextBox;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::layout::base::balloons::content::BalloonContentLayouter::BalloonContentLayouter;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;

// java public class EquationContentLayouter extends BalloonContentLayouter
pub struct EquationContentLayouter;

impl EquationContentLayouter {
    // java IEquationLayouter.getDrawing(Document, int start, int end)
    pub fn get_drawing(
        main: *mut MainState,
        doc: *mut DocumentEnv,
        view: *mut ViewEnv,
        document: *mut dyn Document,
        start: i32,
        end: i32,
        enable_revision: bool,
        need_calc_graph_height: bool,
    ) -> i32 {
        let mut layouter = BalloonContentLayouter::for_equation(main, doc, view);
        let mut state = LayoutState::LayoutState(None);
        state.document = Some(document);
        state.setRange__int_int(start, end);
        state.cp = start;
        state.styleResult.ignoreBdr = true;
        state.initCoreContent(end, enable_revision);
        layouter.initFilter(Some(document), start, end, &mut state);
        state.pageEnv.width = i32::MAX;
        layouter.setCalcGraphHeight(need_calc_graph_height);
        let text_box = layouter.flow(&mut state);
        if text_box == 0 {
            return 0;
        }
        let baseline = layouter.getBaseLine();
        let snapshot = unsafe { &mut (*main).snapshot };
        Self::adjustTextBox(text_box, snapshot);
        Self::obtain_drawing(snapshot, text_box, baseline)
    }

    // java private int getTextbox(Document, char) then obtainDrawing(textBox)
    pub fn get_char_drawing(
        main: *mut MainState,
        doc: *mut DocumentEnv,
        view: *mut ViewEnv,
        document: *mut dyn Document,
        ch: char,
        char_color: i32,
        adjust_star: bool,
        calc_graph_height: bool,
    ) -> i32 {
        let mut layouter = BalloonContentLayouter::for_equation(main, doc, view);
        layouter.setNeedAdjustDrawPosOffsetX(adjust_star);
        let mut state = LayoutState::LayoutState(None);
        state.document = Some(document);
        let prop = if char_color != 0 {
            PropertySet::valueOf(11, Arc::new(char_color) as Arc<dyn Object + Send + Sync>)
        } else {
            PropertySet::empty()
        };
        state.addBalloonInfoString(Some(&ch.to_string()), &prop);
        state.pageEnv.width = i32::MAX;
        layouter.setCalcGraphHeight(calc_graph_height);
        let text_box = layouter.flow(&mut state);
        if text_box == 0 {
            return 0;
        }
        let baseline = layouter.getBaseLine();
        let snapshot = unsafe { &mut (*main).snapshot };
        Self::adjustTextBox(text_box, snapshot);
        Self::obtain_drawing(snapshot, text_box, baseline)
    }

    // java public static void adjustTextBox(int textBox, TypoSnapshot snapshot)
    pub fn adjustTextBox(text_box: i32, snapshot: &mut TypoSnapshot) {
        let mut max_line_right = -1;
        let mut min_line_left = TypoTextBox::getCoreWidth__int_typosnapshot(text_box, snapshot);
        let lines = TypoTextBox::getLines__int_typosnapshot(text_box, snapshot);
        let size = TypoList::size__int_typosnapshot(lines, snapshot);
        for i in 0..size {
            let p_line = TypoList::quickGet__int_int_typosnapshot(i, lines, snapshot);
            if TypoMemoryObj::getType__int_typosnapshot(p_line, snapshot) == 3 {
                min_line_left = min_line_left.min(TypoObjBase::getLeft__int_typosnapshot(p_line, snapshot));
                max_line_right = max_line_right.max(TypoObjBase::getRight__int_typosnapshot(p_line, snapshot));
            }
        }
        TypoTextBox::setWidth__int_int_typosnapshot((max_line_right - min_line_left).max(0), text_box, snapshot);
    }

    // java obtainDrawing() then obtainDrawing(textBox)
    fn obtain_drawing(snapshot: &mut TypoSnapshot, text_box: i32, baseline: i32) -> i32 {
        let mut drawing = snapshot.getTypoTools().obtainTypoDrawing();
        drawing.alloc(snapshot);
        drawing.setDrawingType(15);
        drawing.setIsRotateWithPage(true);
        drawing.setEmbed(true);
        let pointer = drawing.getPointer();
        TypoTextBox::setParent__int_int_typosnapshot(pointer, text_box, snapshot);
        drawing.setTextbox(text_box);
        drawing.setLeft(0);
        drawing.setTop(0);
        drawing.setHeight(TypoTextBox::height__int_typosnapshot(text_box, snapshot));
        drawing.setWidth(TypoTextBox::width__int_typosnapshot(text_box, snapshot));
        drawing.setAlignLine(baseline);
        drawing.setAlignType(1);
        snapshot.getTypoTools().recycle_drawing(&mut drawing);
        pointer
    }
}
