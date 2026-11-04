#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables)]

// Java: import android.graphics.Color

use crate::cn::wps::graphics::gdi::IGdiPainter::IGdiPainter;

fn color_argb(alpha: i32, r: i32, g: i32, b: i32) -> i32 {
    ((alpha & 0xFF) << 24) | ((r & 0xFF) << 16) | ((g & 0xFF) << 8) | (b & 0xFF)
}

// Java: public class ColorPaint implements IGdiPainter
#[derive(Clone)]
pub struct ColorPaint {
    // Java field: ColorPaint WHITE
    // Java field: ColorPaint BLACK
    // Java field: ColorPaint NULL
    // Java field: int color
    pub(crate) color: i32,
    // Java field: int oriColor
    pub(crate) oriColor: i32,
}

pub static WHITE: std::sync::LazyLock<ColorPaint> = std::sync::LazyLock::new(|| ColorPaint::new(-1));
pub static BLACK: std::sync::LazyLock<ColorPaint> = std::sync::LazyLock::new(|| ColorPaint::new(-16777216));
pub static NULL_PAINT: std::sync::LazyLock<ColorPaint> = std::sync::LazyLock::new(|| ColorPaint::new(0));

// Java: public class ColorPaint implements IGdiPainter
impl ColorPaint {
    // Java: static ColorPaint create(int alpha, int r, int g, int b)
    pub fn create_argb(alpha: i32, r: i32, g: i32, b: i32) -> ColorPaint {
        // Java: return create(Color.argb(alpha, r, g, b));
        ColorPaint::create(color_argb(alpha, r, g, b))
    }

    // Java: static ColorPaint create(int r, int g, int b)
    pub fn create_rgb(r: i32, g: i32, b: i32) -> ColorPaint {
        // Java: return create(Color.argb(255, r, g, b));
        ColorPaint::create(color_argb(255, r, g, b))
    }

    // Java: static ColorPaint create(int color)
    pub fn create(color: i32) -> ColorPaint {
        // Java: switch (color)
        match color {
            // Java: case -16777216:
            -16777216 => {
                // Java: return BLACK;
                ColorPaint::new(-16777216)
            }
            // Java: case -1:
            -1 => {
                // Java: return WHITE;
                ColorPaint::new(-1)
            }
            // Java: case 0:
            0 => {
                // Java: return NULL;
                ColorPaint::new(0)
            }
            // Java: default:
            _ => {
                // Java: return new ColorPaint(color);
                ColorPaint::new(color)
            }
        }
    }

    // Java: ColorPaint(int color)
    pub fn new(color: i32) -> Self {
        // Java: super();
        // Java: this.oriColor = color;
        // Java: this.color = this.oriColor;
        ColorPaint { color, oriColor: color }
    }

    // Java: int getType()
    pub fn getType(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: int getColor()
    pub fn getColor(&self) -> i32 {
        // Java: return this.color;
        self.color
    }

    // Java: int getOriColor()
    pub fn getOriColor(&self) -> i32 {
        // Java: return this.oriColor;
        self.oriColor
    }

    // Java: void setColor(int color)
    pub fn setColor(&mut self, color: i32) {
        // Java: this.color = color;
        self.color = color;
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return Integer.toHexString(this.color);
        format!("{:x}", self.color)
    }
}

impl IGdiPainter for ColorPaint {
    fn getType(&self) -> i32 { self.getType() }
}
