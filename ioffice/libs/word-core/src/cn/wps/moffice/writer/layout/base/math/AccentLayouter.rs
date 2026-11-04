#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.AccentLayouter
use stubs::java::lang::Object::Object;
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::math::AccProp::AccProp;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::MathLayouterBase::MathLayouterBase;

// java public class AccentLayouter extends MathLayouterBase
pub struct AccentLayouter {
    base: MathLayouterBase,
}

impl AccentLayouter {
    // java public AccentLayouter(MainState mainState, DocumentEnv documentEnv, ViewEnv viewEnv)
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: MathLayouterBase::new(main, doc, view) }
    }

    // java public TypoDrawing layout(Document document, PLCMathFunction.MathStartNode mathNode, boolean isInMathPara)
    pub fn layout(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> Option<TypoDrawing> {
        self.base.init(unsafe { &*document }, math_node, is_in_math_para);
        let mut drawing = self.base.obtainTypoDrawing();
        let property = math_node.getPropertySet();
        let accent_char = property.and_then(acc_char);
        if let Some(ch) = accent_char {
            self.base.set_sep_char(ch, true);
        }
        let acc = self.base.getTypoDrawing(document, self.base.start_cp, self.base.start_cp + 1, property);
        self.base.set_sep_char('\0', false);
        let element = self.base.layout_range(document, self.base.start_cp + 1, self.base.end_cp, true);
        if let Some(acc) = acc.as_ref() {
            drawing.addChildDrawing(acc.getPointer());
        }
        if let Some(element) = element.as_ref() {
            drawing.addChildDrawing(element.getPointer());
        }
        MathLayouterBase::adjustDrawingVert(&mut drawing, self.base.snapshot(), 0, 2);
        if let Some(element) = element.as_ref() {
            let top = TypoObjBase::getTop__int_typosnapshot(element.getPointer(), self.base.snapshot());
            let align = TypoDrawing::getAlignLine__int_typosnapshot(element.getPointer(), self.base.snapshot());
            drawing.setAlignLine(top + align);
            drawing.setAlignType(1);
        }
        if let Some(mut acc) = acc {
            self.base.recycle_drawing(&mut acc);
        }
        if let Some(mut element) = element {
            self.base.recycle_drawing(&mut element);
        }
        Some(drawing)
    }
}

fn acc_char(property: &util::moffice::property::PropertySet::PropertySet) -> Option<char> {
    let value = property.getValue(737)?;
    Some(value.as_ref().as_any().downcast_ref::<AccProp>()?.chr)
}
