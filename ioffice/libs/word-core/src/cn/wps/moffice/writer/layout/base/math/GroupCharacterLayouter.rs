#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.GroupCharacterLayouter
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::math::GroupChrProp::GroupChrProp;
use crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::math::MathLayouterBase::MathLayouterBase;

pub const GROUPCHRSCALSE: f32 = 0.65;

// java public class GroupCharacterLayouter extends MathLayouterBase
pub struct GroupCharacterLayouter {
    base: MathLayouterBase,
}

impl GroupCharacterLayouter {
    pub fn new(main: *mut MainState, doc: *mut DocumentEnv, view: *mut ViewEnv) -> Self {
        Self { base: MathLayouterBase::new(main, doc, view) }
    }

    // java public TypoDrawing layout(...)
    pub fn layout(&mut self, document: *mut dyn Document, math_node: &MathStartNode, is_in_math_para: bool) -> Option<TypoDrawing> {
        let doc = unsafe { &*document };
        self.base.init(doc, math_node, is_in_math_para);
        let gpr = math_node.getPropertySet().map(GroupChrProp::from_property).unwrap_or_default();
        let align_by_e = gpr.pos != gpr.vertJc;
        self.base.group_chr = true;
        let mut group = self.base.getTypoDrawing(document, self.base.start_cp, self.base.start_cp + 1, math_node.getPropertySet())?;
        let mut element = self.base.layout_range(document, self.base.start_cp + 1, self.base.end_cp, true)?;
        if !align_by_e {
            self.base.scaleMath(&mut element, GROUPCHRSCALSE);
        }
        let zoom = {
            let snap = self.base.snapshot();
            TypoDrawing::layoutWidth__int_typosnapshot(element.getPointer(), snap) as f32
                / TypoDrawing::layoutWidth__int_typosnapshot(group.getPointer(), snap) as f32
        };
        if zoom > 1.0 {
            self.base.setMathScale(&mut group, zoom, 1);
        }
        let mut drawing = self.base.obtainTypoDrawing();
        if gpr.pos == 0 {
            drawing.addChildDrawing(group.getPointer());
            drawing.addChildDrawing(element.getPointer());
        } else {
            drawing.addChildDrawing(element.getPointer());
            drawing.addChildDrawing(group.getPointer());
        }
        MathLayouterBase::adjustDrawingVert(&mut drawing, self.base.snapshot(), 0, 2);
        let height = {
            let snap = self.base.snapshot();
            TypoDrawing::layoutHeight__int_typosnapshot(group.getPointer(), snap)
                + TypoDrawing::layoutHeight__int_typosnapshot(element.getPointer(), snap)
        };
        drawing.setHeight(height);
        let anchor = if align_by_e { element.getPointer() } else { group.getPointer() };
        let (align_type, top, align) = {
            let snap = self.base.snapshot();
            (
                TypoDrawing::getAlignType__int_typosnapshot(anchor, snap),
                TypoObjBase::getTop__int_typosnapshot(anchor, snap),
                TypoDrawing::getAlignLine__int_typosnapshot(anchor, snap),
            )
        };
        drawing.setAlignType(align_type);
        drawing.setAlignLine(top + align);
        self.base.recycle_drawing(&mut group);
        self.base.recycle_drawing(&mut element);
        Some(drawing)
    }
}
