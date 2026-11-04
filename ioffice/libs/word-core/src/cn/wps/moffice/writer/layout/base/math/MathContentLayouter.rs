#![allow(dead_code, non_snake_case, unused_variables)]
// Java: cn.wps.moffice.writer.layout.base.math.MathContentLayouter
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::layout::base::balloons::content::BalloonContentLayouter::BalloonContentLayouter;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;

// java public class MathContentLayouter extends BalloonContentLayouter
pub struct MathContentLayouter {
    balloon: BalloonContentLayouter,
}

impl MathContentLayouter {
    // java public MathContentLayouter(MainState mainState, DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn MathContentLayouter(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        let mut balloon = BalloonContentLayouter::BalloonContentLayouter(main, doc, view);
        balloon.enable_math();
        Self { balloon }
    }

    // java public LayoutState createLayoutState()
    pub fn createLayoutState(&self) -> LayoutState {
        LayoutState::default()
    }

    // java public int flow(LayoutState layoutState)
    pub fn flow(&mut self, layout_state: &mut LayoutState) -> i32 {
        self.balloon.begin_math_flow(layout_state);
        let text_box = self.balloon.flow(layout_state);
        if !self.balloon.has_lines(text_box) { 0 } else { text_box }
    }

    // java public void initFilter(Document document, int start, int end, LayoutState layoutState)
    pub fn initFilter(&mut self, document: Option<*mut dyn Document>, start: i32, end: i32, layout_state: &mut LayoutState) {
        self.balloon.initFilter(document, start, end, layout_state);
    }

    // java public int getBaseLine()
    pub fn getBaseLine(&self) -> i32 {
        self.balloon.getBaseLine()
    }

    // java public int getAlignType()
    pub fn getAlignType(&self) -> i32 {
        self.balloon.getAlignType()
    }

    // java public void setUseAlignOMathParaHandler(boolean value)
    pub fn setUseAlignOMathParaHandler(&mut self, value: bool) {
        self.balloon.setUseAlignOMathParaHandler(value);
    }

    // java public void setNeedCollectHighlight(boolean val)
    pub fn setNeedCollectHighlight(&mut self, _val: bool) {
        // Highlight collectors are not on the balloon path yet. The flag is stored by Java
        // and only consulted by needCollectHighlight, which balloon does not call.
    }

    // java public void setIgnoreLeading(boolean ingoreLeading)
    pub fn setIgnoreLeading(&mut self, value: bool) {
        self.balloon.setIgnoreLeading(value);
    }

    // java public void setMathContent(boolean mathContent)
    pub fn setMathContent(&mut self, value: bool) {
        self.balloon.setMathContent(value);
    }

    // java public void addFunctionSepField(int sepCP)
    pub fn addFunctionSepField(&mut self, sep_cp: i32) {
        self.balloon.addFunctionSepField(sep_cp);
    }
}
