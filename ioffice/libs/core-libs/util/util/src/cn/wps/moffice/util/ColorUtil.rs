#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java field: int BLACK
pub const BLACK: i32 = -16777216;
// Java field: int DKGRAY
pub const DKGRAY: i32 = -12303292;
// Java field: int GRAY
pub const GRAY: i32 = -7829368;
// Java field: int LTGRAY
pub const LTGRAY: i32 = -3355444;
// Java field: int WHITE
pub const WHITE: i32 = -1;
// Java field: int RED
pub const RED: i32 = -65536;
// Java field: int GREEN
pub const GREEN: i32 = -16711936;
// Java field: int BLUE
pub const BLUE: i32 = -16776961;
// Java field: int YELLOW
pub const YELLOW: i32 = -256;
// Java field: int CYAN
pub const CYAN: i32 = -16711681;
// Java field: int MAGENTA
pub const MAGENTA: i32 = -65281;
// Java field: int TRANSPARENT
pub const TRANSPARENT: i32 = 0;
// Java field: int READ_BG_COLOR_EYE_PROTECTION_LIGHT_GREEN
pub const READ_BG_COLOR_EYE_PROTECTION_LIGHT_GREEN: i32 = -919568;
// Java field: int READ_BG_COLOR_COWHIDE_LIGHT_YELLOW
pub const READ_BG_COLOR_COWHIDE_LIGHT_YELLOW: i32 = -67345;
// Java field: int READ_BG_COLOR_LIGHT_BROWN
pub const READ_BG_COLOR_LIGHT_BROWN: i32 = -987418;
// Java field: int READ_BG_COLOR_LIGHTER_BLUE
pub const READ_BG_COLOR_LIGHTER_BLUE: i32 = -1116166;
// Java field: int READ_BG_COLOR_LIGHT_GREY
pub const READ_BG_COLOR_LIGHT_GREY: i32 = -1315861;
// Java field: int FONT_COLOR_LIGHT_BROWN
pub const FONT_COLOR_LIGHT_BROWN: i32 = -10075633;
// Java field: int LIGHT
pub const LIGHT: i32 = -2162689;
// Java field: int GRAY_LIGHT
pub const GRAY_LIGHT: i32 = -3355444;
// Java field: int GRAY_DARK
pub const GRAY_DARK: i32 = -13421773;
// Java field: int BLUE_DARK
pub const BLUE_DARK: i32 = -16218128;
// Java field: int REVISION_ACCEPT
pub const REVISION_ACCEPT: i32 = 1486678;
// Java field: int REVISION_REJECT
pub const REVISION_REJECT: i32 = 15405884;
// Java field: int SPELLCHECKWAVE
pub const SPELLCHECKWAVE: i32 = -1426128896;
// Java field: int COLOR_NONE
pub const COLOR_NONE: i32 = -2;
// Java field: int COLOR_AUTO
pub const COLOR_AUTO: i32 = 0;

// Java: public class ColorUtil
pub struct ColorUtil {
}

// Java: public enum ColorLayer
pub enum ColorLayer {
    lighten,
    darken,
}

// Java: public class ColorUtil
impl ColorUtil {
    // Java: ColorUtil()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }

    // Java: static int rgb2Argb(int color, float alpha)
    pub fn rgb2Argb_alpha(color: i32, alpha: f32) -> i32 {
        // Java: return (int)(255.0F * alpha) << 24 | color & 16777215;
        ((255.0f32 * alpha) as i32) << 24 | color & 16777215
    }

    // Java: static int rgb2Argb(int color)
    pub fn rgb2Argb(color: i32) -> i32 {
        // Java: return -16777216 | color & 16777215;
        -16777216i32 | color & 16777215
    }

    // Java: static int rgb2Argb(int red, int green, int blue)
    pub fn rgb2Argb_components(red: i32, green: i32, blue: i32) -> i32 {
        // Java: return -16777216 | red << 16 | green << 8 | blue;
        -16777216i32 | red << 16 | green << 8 | blue
    }

    // Java: static int argb2rgb(int color)
    pub fn argb2rgb(color: i32) -> i32 {
        // Java: return color & 16777215;
        color & 16777215
    }

    // Java: static int getMixtureColor(int color1, int color2, float alpha)
    pub fn getMixtureColor_alpha(color1: i32, color2: i32, alpha: f32) -> i32 {
        // Java: int r1 = color1 >> 16 & 255;
        let mut r1 = color1 >> 16 & 255;
        // Java: int g1 = color1 >> 8 & 255;
        let mut g1 = color1 >> 8 & 255;
        // Java: int b1 = color1 & 255;
        let mut b1 = color1 & 255;
        // Java: int r2 = color2 >> 16 & 255;
        let r2 = color2 >> 16 & 255;
        // Java: int g2 = color2 >> 8 & 255;
        let g2 = color2 >> 8 & 255;
        // Java: int b2 = color2 & 255;
        let b2 = color2 & 255;
        // Java: r1 = (int)((float)r1 * alpha + (float)(r2 * 1) * (1.0F - alpha));
        r1 = (r1 as f32 * alpha + (r2 * 1) as f32 * (1.0f32 - alpha)) as i32;
        // Java: g1 = (int)((float)g1 * alpha + (float)(g2 * 1) * (1.0F - alpha));
        g1 = (g1 as f32 * alpha + (g2 * 1) as f32 * (1.0f32 - alpha)) as i32;
        // Java: b1 = (int)((float)b1 * alpha + (float)(b2 * 1) * (1.0F - alpha));
        b1 = (b1 as f32 * alpha + (b2 * 1) as f32 * (1.0f32 - alpha)) as i32;
        // Java: return rgb2Argb(r1, g1, b1);
        Self::rgb2Argb_components(r1, g1, b1)
    }

    // Java: static int getLightenColor(int color, float alpha)
    pub fn getLightenColor_alpha(color: i32, alpha: f32) -> i32 {
        // Java: return getMixtureColor(color, -1, alpha);
        Self::getMixtureColor_alpha(color, -1, alpha)
    }

    // Java: static int getDarkenColor(int color, float alpha)
    pub fn getDarkenColor_alpha(color: i32, alpha: f32) -> i32 {
        // Java: return getMixtureColor(color, -16777216, alpha);
        Self::getMixtureColor_alpha(color, -16777216, alpha)
    }

    // Java: static int getLightenColor(int color)
    pub fn getLightenColor(color: i32) -> i32 {
        // Java: return getMixtureColor(color, 85);
        Self::getMixtureColor(color, 85)
    }

    // Java: static int getDarkenColor(int color)
    pub fn getDarkenColor(color: i32) -> i32 {
        // Java: return getMixtureColor(color, -85);
        Self::getMixtureColor(color, -85)
    }

    // Java: static int getMixtureColor(int color, int delta)
    pub fn getMixtureColor(color: i32, delta: i32) -> i32 {
        // Java: int red = color >> 16 & 255;
        let mut red = color >> 16 & 255;
        // Java: int green = color >> 8 & 255;
        let mut green = color >> 8 & 255;
        // Java: int blue = color & 255;
        let mut blue = color & 255;
        // Java: red = Math.max(0, Math.min(255, red + delta));
        red = 0.max(255.min(red + delta));
        // Java: green = Math.max(0, Math.min(255, green + delta));
        green = 0.max(255.min(green + delta));
        // Java: blue = Math.max(0, Math.min(255, blue + delta));
        blue = 0.max(255.min(blue + delta));
        // Java: return rgb2Argb(red, green, blue);
        Self::rgb2Argb_components(red, green, blue)
    }

    // Java: static float getMixtureAlpha(int color)
    pub fn getMixtureAlpha(color: i32) -> f32 {
        // Java: int red = color >> 16 & 255;
        let red = color >> 16 & 255;
        // Java: int green = color >> 8 & 255;
        let green = color >> 8 & 255;
        // Java: int blue = color & 255;
        let blue = color & 255;
        // Java: float alpha = Math.max(1.0F * (float)red / 255.0F, 1.0F * (float)green / 255.0F);
        let mut alpha = (1.0f32 * red as f32 / 255.0f32).max(1.0f32 * green as f32 / 255.0f32);
        // Java: alpha = Math.max(alpha, 1.0F * (float)blue / 255.0F);
        alpha = alpha.max(1.0f32 * blue as f32 / 255.0f32);
        // Java: return alpha;
        alpha
    }

    // Java: static int getMultiplyColor(int color, float alpha)
    pub fn getMultiplyColor(color: i32, alpha: f32) -> i32 {
        // Java: int red = color >> 16 & 255;
        let mut red = color >> 16 & 255;
        // Java: int green = color >> 8 & 255;
        let mut green = color >> 8 & 255;
        // Java: int blue = color & 255;
        let mut blue = color & 255;
        // Java: red = (int)((float)red * alpha);
        red = (red as f32 * alpha) as i32;
        // Java: green = (int)((float)green * alpha);
        green = (green as f32 * alpha) as i32;
        // Java: blue = (int)((float)blue * alpha);
        blue = (blue as f32 * alpha) as i32;
        // Java: return rgb2Argb(red, green, blue);
        Self::rgb2Argb_components(red, green, blue)
    }

    // Java: static int convertColor(int origin, float brightness)
    pub fn convertColor(origin: i32, brightness: f32) -> i32 {
        // Java: int red = origin >> 16 & 255;
        let mut red = origin >> 16 & 255;
        // Java: int green = origin >> 8 & 255;
        let mut green = origin >> 8 & 255;
        // Java: int blue = origin & 255;
        let mut blue = origin & 255;
        // Java: red = (int)((float)red * brightness + 0.5F);
        red = (red as f32 * brightness + 0.5f32) as i32;
        // Java: green = (int)((float)green * brightness + 0.5F);
        green = (green as f32 * brightness + 0.5f32) as i32;
        // Java: blue = (int)((float)blue * brightness + 0.5F);
        blue = (blue as f32 * brightness + 0.5f32) as i32;
        // Java: if (red > 255)
        if red > 255
        {
            // Java: red = 255;
            red = 255;
        }
        // Java: if (green > 255)
        if green > 255
        {
            // Java: green = 255;
            green = 255;
        }
        // Java: if (blue > 255)
        if blue > 255
        {
            // Java: blue = 255;
            blue = 255;
        }
        // Java: return rgb2Argb(red, green, blue);
        Self::rgb2Argb_components(red, green, blue)
    }

    // Java: static int rgb2bgr(int origin)
    pub fn rgb2bgr(origin: i32) -> i32 {
        // Java: if (origin == -2)
        if origin == -2
        {
            // Java: return -16777216;
            return -16777216;
        }
        // Java: else
        {
            // Java: int red = origin >> 16 & 255;
            let red = origin >> 16 & 255;
            // Java: int green = origin >> 8 & 255;
            let green = origin >> 8 & 255;
            // Java: int blue = origin & 255;
            let blue = origin & 255;
            // Java: return blue << 16 | green << 8 | red;
            blue << 16 | green << 8 | red
        }
    }
}

// Java: public enum ColorLayer
// Java: ColorLayer()
