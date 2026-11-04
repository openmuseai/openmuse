// Rust stub for DgShape (auto-generated)
// Original: cn/wps/drawing/app/shape/DgShape.java
use crate::*;

pub struct DgShape {
    // TODO: fields
}

impl DgShape {
    pub fn new() -> Self { todo!() }
    pub fn find(&self, spId: i32) -> DgShape { todo!() }
    pub fn copyFrom(&self, srcSpId: i32) { todo!() }
    pub fn clearFormat(&self) { todo!() }
    pub fn loadStyleInfo(&self) { todo!() }
    pub fn hasDescr(&self) -> bool { todo!() }
    pub fn descr(&self) -> String { todo!() }
    pub fn setDescr(&self, descr: String) { todo!() }
    pub fn copyStyleFrom(&self, sp: Shape) { todo!() }
    pub fn copyTextBodyFrom(&self, sp: Shape) { todo!() }
    pub fn copyTextBodyPropertiesFrom(&self, sp: Shape) { todo!() }
    pub fn copyTextListStyleFrom(&self, sp: Shape) { todo!() }
    pub fn writeListStyle(&self, listStyle: TextListStyle) { todo!() }
    pub fn hasTextBody(&self) -> bool { todo!() }
    pub fn textBody(&self) -> TextBody { todo!() }
    pub fn copySpprWithoutTransfromFrom(&self, sp: Shape) { todo!() }
    pub fn copySpprWithoutTransfromFrom_1(&self, sp: Shape, changeGeom: bool) { todo!() }
    pub fn extractFill(sp: Shape) -> Fill { todo!() }
    pub fn extractOutline(sp: Shape) -> Outline { todo!() }
    pub fn copyShape(store: DgStore, srcSp: Box<dyn KoBin>, graphic: Box<dyn GraphicObject>, graphicChoice: Box<dyn GraphicObject>) -> i32 { todo!() }
    pub fn copyShape_1(store: DgStore, srcSp: Box<dyn KoBin>, graphic: Box<dyn GraphicObject>) -> i32 { todo!() }
    pub fn updateToStore(&self) { todo!() }
    pub fn hasStyle(&self) -> bool { todo!() }
    pub fn setFontColor(&self, fontColor: i32) { todo!() }
    pub fn setFillInTextParagraph(&self, textParagraph: TextParagraph, fontColor: i32) { todo!() }
    pub fn setFontColorInTextParagraph(&self, textParagraph: TextParagraph, fontColor: i32) { todo!() }
    pub fn putFontColor2TextRun(&self, textRun: TextRun, fontColor: i32) { todo!() }
    pub fn getFillColor(&self, fontColor: i32) -> Fill { todo!() }
    pub fn setFontSizeWithScale(&self, scale: f32) { todo!() }
    pub fn setSizeInTextParagraph(&self, textParagraph: TextParagraph, scale: f32) { todo!() }
    pub fn x(&self) -> i32 { todo!() }
    pub fn y(&self) -> i32 { todo!() }
    pub fn cx(&self) -> i32 { todo!() }
    pub fn cy(&self) -> i32 { todo!() }
    pub fn putCxAndCY(&self, cx: i32, cy: i32) { todo!() }
    pub fn rotation(&self) -> i32 { todo!() }
    pub fn isFlipHorizontal(&self) -> bool { todo!() }
    pub fn isFlipVertical(&self) -> bool { todo!() }
    pub fn hasTransform2D(&self) -> bool { todo!() }
    pub fn hasSpPr(&self) -> bool { todo!() }
    pub fn transform2D(&self) -> Transform2D { todo!() }
    pub fn delTransform2D(&self) { todo!() }
    pub fn realShapeTransformWithRotation(shape: Shape) -> Transform2D { todo!() }
    pub fn writeBodyPr(&self, bodyPr: TextBodyProperties) { todo!() }
    pub fn writeParaLst(&self, paragraphList: TextBody::ParagraphList) { todo!() }
    pub fn writeTextBody(&self, textBody: TextBody) { todo!() }
    pub fn writeTransform2D(&self, trans: Transform2D) { todo!() }
    pub fn writeShapeExtLst(&self, list: List) { todo!() }
    pub fn getShapeExtLst(&self) -> List { todo!() }
    pub fn r#move(&self, offsetX: i32, offsetY: i32) { todo!() }
    pub fn relocateModularGroup(&self, x: i32, y: i32) { todo!() }
    pub fn move_1(&self, offsetX: i32, offsetY: i32, parents: Box<dyn IIntList>) { todo!() }
    pub fn resize(&self, offsetTop: i32, offsetLeft: i32, offsetBottom: i32, offsetRight: i32) { todo!() }
    pub fn resize_1(&self, offsetTop: f32, offsetLeft: f32, offsetBottom: f32, offsetRight: f32, parentIds: Box<dyn IIntList>) { todo!() }
    pub fn ceilNormal(&self, src: f32) -> i32 { todo!() }
    pub fn ceilSpecial(&self, src: f32) -> i32 { todo!() }
    pub fn resizeWithoutFlip(&self, offsetTop: f32, offsetLeft: f32, offsetBottom: f32, offsetRight: f32, parentIds: Box<dyn IIntList>, minWidth: f32, minHeight: f32) { todo!() }
    pub fn resizeWithoutFlip_1(&self, offsetTop: i32, offsetLeft: i32, offsetBottom: i32, offsetRight: i32, minWidth: i32, minHeight: i32) { todo!() }
    pub fn rotate(&self, ang: i32) { todo!() }
    pub fn readFormat(&self, r#type: i32) { todo!() }
    pub fn readShowShape(&self, dgShape: DgShape, r#type: i32) { todo!() }
    pub fn readShapePr(&self, shapePr: ShapePr, r#type: i32) { todo!() }
    pub fn writeFormat(&self, formatType: i32) { todo!() }
    pub fn writeGroupPr(&self, groupPr: ShapePr, formatType: i32) { todo!() }
    pub fn writeShapePr(&self, shapePr: ShapePr, formatType: i32) { todo!() }
    pub fn childShapeAt(&self, groupShape: Shape, index: i32) -> Shape { todo!() }
    pub fn writeFormat2Child(&self, groupShape: Shape, dg3DFormat: Dg3DFormat) { todo!() }
    pub fn r#type(&self) -> i32 { todo!() }
    pub fn fill(&self) -> DgFillFormat { todo!() }
    pub fn loadFill(&self) { todo!() }
    pub fn updateFill(&self) { todo!() }
    pub fn updateChildFill(&self, showShape: Shape, shapeId: i32) { todo!() }
    pub fn threeDFormat(&self) -> Dg3DFormat { todo!() }
    pub fn load3D(&self) { todo!() }
    pub fn hasHlinkClick(&self) -> bool { todo!() }
    pub fn hlinkClickAction(&self) -> String { todo!() }
    pub fn hlinkClickIsInternal(&self) -> bool { todo!() }
    pub fn hlinkClickTarget(&self) -> String { todo!() }
    pub fn deleteHlinkClick(&self) { todo!() }
    pub fn setHlinkClick(&self, hyperlink: Hyperlink) { todo!() }
    pub fn hasHlinkOnHover(&self) -> bool { todo!() }
    pub fn hlinkHoverAction(&self) -> String { todo!() }
    pub fn hlinkHoverIsInternal(&self) -> bool { todo!() }
    pub fn hlinkHoverTarget(&self) -> String { todo!() }
    pub fn deleteHlinkHover(&self) { todo!() }
    pub fn setHlinkHover(&self, hyperlink: Hyperlink) { todo!() }
    pub fn getHyperlink(&self) -> Hyperlink { todo!() }
    pub fn childCount(&self) -> i32 { todo!() }
    pub fn deal3DByGroup(&self, parentDg3DFormat: Dg3DFormat) { todo!() }
    pub fn update3DFormat(&self) { todo!() }
    pub fn updateChild3DFormat(&self, dgShape: Shape, shapeId: i32, dg3DFormat: Dg3DFormat) { todo!() }
    pub fn hasFill(&self) -> bool { todo!() }
    pub fn hasOutline(&self) -> bool { todo!() }
    pub fn outline(&self) -> DgOutlineFormat { todo!() }
    pub fn loadOutline(&self) { todo!() }
    pub fn dealOutlineByGroup(&self, parentDgOutlineFormat: DgOutlineFormat) { todo!() }
    pub fn updateOutline(&self) { todo!() }
    pub fn updateChildOutline(&self, showShape: Shape, shapeId: i32, outline: DgOutlineFormat) { todo!() }
    pub fn shadow(&self) -> DgEffectShadow { todo!() }
    pub fn hasEffectShadow(&self) -> bool { todo!() }
    pub fn updateShadow(&self) { todo!() }
    pub fn reflection(&self) -> DgEffectReflection { todo!() }
    pub fn hasEffectReflection(&self) -> bool { todo!() }
    pub fn updateReflection(&self) { todo!() }
    pub fn softEdge(&self) -> DgEffectSoftEdge { todo!() }
    pub fn hasEffectSoftEdge(&self) -> bool { todo!() }
    pub fn updateSoftEdge(&self) { todo!() }
    pub fn glow(&self) -> DgEffectGlow { todo!() }
    pub fn loadEffect(&self) { todo!() }
    pub fn hasEffectGlow(&self) -> bool { todo!() }
    pub fn updateGlow(&self) { todo!() }
    pub fn geometry(&self) -> DgGeometry2D { todo!() }
    pub fn loadGeo(&self) { todo!() }
    pub fn updateGeometry(&self) { todo!() }
    pub fn setBorderSolidFillColor(&self, colorValue: i32) { todo!() }
    pub fn setBorderGradientFill(&self, fill: Fill) { todo!() }
    pub fn setColor(&self, color: Color, colorValue: i32) { todo!() }
    pub fn setSchemeColor(color: Color, theme: i32) { todo!() }
    pub fn setRgbColor(color: Color, rgb: i32) { todo!() }
    pub fn iniPro2ShapeStyle(&self) { todo!() }
    pub fn iniShapeStyle2Pro(&self) { todo!() }
    pub fn shapeStyle(&self) -> DgShapeStyle { todo!() }
    pub fn hasShapeStyle(&self) -> bool { todo!() }
    pub fn deleteShapeStyle(&self) { todo!() }
    pub fn setShapeStyle(&self, shapeStyle: DgShapeStyle) { todo!() }
    pub fn setNoGrp(&self, isNogrp: bool) { todo!() }
    pub fn setGroupChildShapeStyle(&self, shape: Shape, shapeId: i32, shapeQuickStyle: DgShapeStyle) { todo!() }
    pub fn dealGroupShapeStyle(&self, parentShapeStyle: DgShapeStyle) { todo!() }
    pub fn borderWidth(&self) -> f64 { todo!() }
    pub fn setBorderWidth(&self, width: f64) { todo!() }
    pub fn setBorderType(&self, r#type: i32) { todo!() }
    pub fn borderType(&self) -> i32 { todo!() }
    pub fn borderStyleType(&self) -> i32 { todo!() }
    pub fn setLn(&self, ln: Outline) { todo!() }
    pub fn setBorderStyleType(&self, borderStyleType: i32) { todo!() }
    pub fn setFillType(&self, r#type: i32) { todo!() }
    pub fn fillType(&self) -> i32 { todo!() }
    pub fn setFillColor(&self, colorValue: i32) { todo!() }
    pub fn setFill(&self, fill: Fill) { todo!() }
    pub fn clearAllShapeFormat(&self, shape: Shape) { todo!() }
    pub fn delShapePrFormat(&self, shapePr: ShapePr) { todo!() }
    pub fn isTextBox(&self) -> bool { todo!() }
    pub fn isNoTextEdit(&self) -> bool { todo!() }
    pub fn graphicId(&self) -> i32 { todo!() }
    pub fn graphicType(&self) -> i32 { todo!() }
    pub fn clearTextContents(&self) { todo!() }
    pub fn setName(&self, name: String) { todo!() }
    pub fn name(&self) -> String { todo!() }
    pub fn getTextContent(&self) -> String { todo!() }
    pub fn hasTextContext(&self) -> bool { todo!() }
    pub fn setTextContent(&self, content: String) { todo!() }
    pub fn replaceTextContentByPara(&self, content: String) { todo!() }
    pub fn connect(&self, isStart: bool, spId: i32, index: i32) { todo!() }
    pub fn noGroup(&self) -> bool { todo!() }
    pub fn setNoGroup(&self, noGroup: bool) { todo!() }
    pub fn setFontScale(&self, fontScale: f32, lnSpcReduction: f64) -> bool { todo!() }
    pub fn hasTextAutoFit(&self) -> bool { todo!() }
    pub fn textAutoFit(&self) -> i32 { todo!() }
    pub fn hasFontSizeFit(&self) -> bool { todo!() }
    pub fn clearFontSizeFit(&self) { todo!() }
    pub fn setAutoFit(&self, r#type: i32) { todo!() }
    pub fn isHidden(&self) -> bool { todo!() }
    pub fn setInset(&self, left: i32, top: i32, right: i32, bottom: i32) { todo!() }
    pub fn adjust(&self, adjustValueList: PresetGeometry::AdjustValueList) { todo!() }
}
