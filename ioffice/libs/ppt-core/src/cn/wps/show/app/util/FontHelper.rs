// Rust stub for FontHelper (auto-generated)
// Original: cn/wps/show/app/util/FontHelper.java
use crate::*;

pub struct FontHelper {
    // TODO: fields
}

impl FontHelper {
    pub fn new() -> Self { todo!() }
    pub fn getFontSize(runprop: TextCharacterProperties) -> f32 { todo!() }
    pub fn getFontColor(runprop: TextCharacterProperties, colorCtx: ColorInterpretCtx) -> i32 { todo!() }
    pub fn getFontColor_1(clr: Color, colorCtx: ColorInterpretCtx) -> i32 { todo!() }
    pub fn getShowFontName(prop: TextCharacterProperties, phType: i32, ctx: KmoContext) -> String { todo!() }
    pub fn getCoreFontName(ch: u16, prop: TextCharacterProperties, phType: i32, ctx: KmoContext) -> String { todo!() }
    pub fn getFontName(langID: i32, font: TextFont, phType: i32, fontCtx: KmoContext) -> String { todo!() }
    pub fn getFontName_1(langID: i32, phType: i32, fontname: String, theme: KmoTheme) -> String { todo!() }
    pub fn getTheme(ctx: KmoContext) -> KmoTheme { todo!() }
    pub fn isSplitCharacter(ch: u16) -> bool { todo!() }
    pub fn increaseFontSize(srcSize: f32) -> f32 { todo!() }
    pub fn decreaseFontSize(srcSize: f32) -> f32 { todo!() }
}
