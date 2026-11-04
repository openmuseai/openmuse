#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.app.format.color.DgColor
// LLM: DgColor is from the same crate, using stub since module is not yet available
// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::drawing::data::color::Color::Color;

pub struct DgColor;
impl DgColor {
    pub const WHITE: i32 = 0x00FFFFFF;
    pub fn rgb(r: i32, g: i32, b: i32) -> i32 { (r << 16) | (g << 8) | b }
    pub fn scheme(idx: i32) -> i32 { idx }
}

// Java: public class DgColorEval
pub struct DgColorEval;

// Java: public class DgColorEval
impl DgColorEval {
    // Java: static int toRGB(Color color)
    pub fn toRGB(color: &mut Color) -> i32 {
        // Java: switch (color.mode())
        match color.mode() {
            // Java: case 1:
            1 => {
                // Java: int r = color.srgb().r();
                let r = color.srgb().r();
                // Java: int g = color.srgb().g();
                let g = color.srgb().g();
                // Java: int b = color.srgb().b();
                let b = color.srgb().b();
                // Java: return DgColor.rgb(r, g, b);
                DgColor::rgb(r, g, b)
            }
            // Java: default:
            _ => {
                // Java: return DgColor.WHITE;
                DgColor::WHITE
            }
        }
    }

    // Java: static int toScheme(Color color)
    pub fn toScheme(color: &Color) -> i32 {
        // Java: switch (color.mode())
        match color.mode() {
            // Java: case 4:
            4 => {
                // Java: return DgColor.scheme(color.scheme());
                DgColor::scheme(color.scheme())
            }
            // Java: default:
            _ => {
                // Java: return DgColor.scheme(0);
                DgColor::scheme(0)
            }
        }
    }

    // Java: static int rgb(int r, int g, int b)
    pub fn rgb(r: i32, g: i32, b: i32) -> i32 {
        // Java: return r << 16 | g << 8 | b;
        (r << 16) | (g << 8) | b
    }
}
