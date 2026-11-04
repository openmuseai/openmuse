// Rust stub for KmoFont (auto-generated)
// Original: cn/wps/show/app/format/KmoFont.java
use crate::*;

pub struct KmoFont {
    // TODO: fields
}

impl KmoFont {
    pub fn new() -> Self { todo!() }
    pub fn isBold(&self) -> bool { todo!() }
    pub fn putBold(&self, isBold: bool) { todo!() }
    pub fn isItalic(&self) -> bool { todo!() }
    pub fn putItalic(&self, isItalic: bool) { todo!() }
    pub fn name(&self) -> String { todo!() }
    pub fn isHasThaiChar(&self) -> bool { todo!() }
    pub fn isBreakSpanChar(&self, oneChar: u16) -> bool { todo!() }
    pub fn putNameAscii(&self, fontname: String) { todo!() }
    pub fn size(&self) -> f32 { todo!() }
    pub fn maxSize(&self) -> f32 { todo!() }
    pub fn firstSize(&self) -> f32 { todo!() }
    pub fn putSize(&self, size: f32) { todo!() }
    pub fn putSizeWithScale(&self, fontScale: f32) { todo!() }
    pub fn increaseSize(&self) { todo!() }
    pub fn decreaseSize(&self) { todo!() }
    pub fn getTable(drawable: KmoShowShape) -> KmoTable { todo!() }
    pub fn isSizeTriStateMixed(&self) -> bool { todo!() }
    pub fn color(&self) -> i32 { todo!() }
    pub fn putColor(&self, colorvalue: i32) { todo!() }
    pub fn putGradientColor(&self, fillBean: FillBean) { todo!() }
    pub fn strike(&self) -> i32 { todo!() }
    pub fn baseline(&self) -> f64 { todo!() }
    pub fn underlineType(&self) -> i32 { todo!() }
    pub fn putUnderlineType(&self, r#type: i32) { todo!() }
    pub fn underlineColor(&self) -> i32 { todo!() }
    pub fn putUnderlineColor(&self, colorvalue: i32) { todo!() }
    pub fn setShadow(&self, shadow: KmoShadow) { todo!() }
    pub fn mergeEffectExceptShadow(&self, effectProperties: EffectProperties, runProp: TextCharacterProperties) -> EffectProperties { todo!() }
    pub fn getShadow(&self) -> KmoShadow { todo!() }
    pub fn getShadowType(&self) -> i32 { todo!() }
    pub fn copyFormat(&self) -> CharBrush { todo!() }
    pub fn pasteFormat(&self, charBrush: CharBrush) { todo!() }
    pub fn getColorInterpretCtx(&self) -> ColorInterpretCtx { todo!() }
    pub fn save(&self, prop: TextCharacterProperties, tmpProp: TextCharacterProperties) { todo!() }
    pub fn save_1(&self, prop: TextCharacterProperties, tmpProp: TextCharacterProperties, isBrush: bool) { todo!() }
    pub fn newProperties(&self) -> TextCharacterProperties { todo!() }
    pub fn getFontname(&self, chProp: TextCharacterProperties, phType: i32) -> String { todo!() }
    pub fn colorIsEqulas(&self, color1: Color, color2: Color) -> bool { todo!() }
    pub fn temporaryProperties(&self) -> TextCharacterProperties { todo!() }
    pub fn setTemporaryProperties(&self, prop: TextCharacterProperties) { todo!() }
    pub fn isSymbolFont(&self, name: String) -> bool { todo!() }
    pub fn isCheckSymbolFont(&self, name: String, prop: TextCharacterProperties) -> bool { todo!() }
    pub fn hasColorTextRun(shape: KmoShowShape, targetColor: i32) -> bool { todo!() }
    pub fn getFontColor(showShape: KmoShowShape) -> List { todo!() }
}
