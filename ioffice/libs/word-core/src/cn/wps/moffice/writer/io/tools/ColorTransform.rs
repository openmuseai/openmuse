#![allow(non_snake_case)]

// Java: cn.wps.moffice.writer.io.tools.ColorTransform
// The image color pipeline uses these Java channel operations directly.
pub struct ColorTransform;
impl ColorTransform {
    // Java: public static int argb(int r, int g, int b, int alpha)
    pub fn argb(r: i32, g: i32, b: i32, alpha: i32) -> i32 {
        (alpha << 24) | (r << 16) | (g << 8) | b
    }
    // Java: public static short getAlpha(int color)
    pub fn getAlpha(color: i32) -> i16 { ((color >> 24) & 255) as i16 }
    // Java: public static short getRed(int color)
    pub fn getRed(color: i32) -> i16 { ((color >> 16) & 255) as i16 }
    // Java: public static short getGreen(int color)
    pub fn getGreen(color: i32) -> i16 { ((color >> 8) & 255) as i16 }
    // Java: public static short getBlue(int color)
    pub fn getBlue(color: i32) -> i16 { (color & 255) as i16 }
    // Java: public static int setRed(int color, int red)
    pub fn setRed(color: i32, red: i32) -> i32 { (color & !0x00ff_0000) | (red << 16) }
    // Java: public static int setGreen(int color, int green)
    pub fn setGreen(color: i32, green: i32) -> i32 { (color & !0x0000_ff00) | (green << 8) }
    // Java: public static int setBlue(int color, int blue)
    pub fn setBlue(color: i32, blue: i32) -> i32 { (color & !0x0000_00ff) | blue }
}
