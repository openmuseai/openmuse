#![allow(non_snake_case, non_upper_case_globals, dead_code)]

use std::sync::OnceLock;

// Java field: int WINDOW_COLOR
pub(crate) const WINDOW_COLOR: i32 = -1;
// Java field: int WINDOW_TEXT_COLOR
pub(crate) const WINDOW_TEXT_COLOR: i32 = -16777216;
// Java field: SystemColor DEFAULT_SYS_COLOR
static DEFAULT_SYS_COLOR: OnceLock<SystemColor> = OnceLock::new();
pub fn get_DEFAULT_SYS_COLOR() -> &'static SystemColor {
    DEFAULT_SYS_COLOR.get_or_init(|| SystemColor::new())
}

// Java: public class SystemColor
pub struct SystemColor {
    // Java field: int mWindowColor
    pub(crate) mWindowColor: i32,
    // Java field: int mWindowTextColor
    pub(crate) mWindowTextColor: i32,
}

// Java: public class SystemColor
impl SystemColor {
    // Java: SystemColor()
    pub fn new() -> Self {
      // Java: super();
      SystemColor { mWindowColor: -1, mWindowTextColor: -16777216 }
    }

    // Java: int getSysColor(int sysClr, int sysLastClr)
    pub fn getSysColor(&self, sysClr: i32, sysLastClr: i32) -> i32 {
      // Java: int argb;
      let argb: i32;
      // Java: switch (sysClr)
      argb = match sysClr {
         // Java: case 5:
         // Java: case 27:
         5 | 27 => {
            // Java: argb = this.mWindowColor;
            self.mWindowColor
            // Java: break;
         }
         // Java: case 29:
         29 => {
            // Java: argb = this.mWindowTextColor;
            self.mWindowTextColor
            // Java: break;
         }
         // Java: default:
         _ => {
            // Java: argb = sysLastClr;
            sysLastClr
         }
      };
      // Java: return argb;
      argb
    }

    // Java: void setWindowColor(int color)
    pub fn setWindowColor(&mut self, color: i32) {
      // Java: this.mWindowColor = color;
      self.mWindowColor = color;
    }

    // Java: void setWindowTextColor(int color)
    pub fn setWindowTextColor(&mut self, color: i32) {
      // Java: this.mWindowTextColor = color;
      self.mWindowTextColor = color;
    }

    // Java: static int rgb(int red, int green, int blue)
    pub fn rgb(red: i32, green: i32, blue: i32) -> i32 {
      // Java: return -16777216 | red << 16 | green << 8 | blue;
      -16777216_i32 | (red << 16) | (green << 8) | blue
    }

    // Java: static int alpha(int color)
    pub fn alpha(color: i32) -> i32 {
      // Java: return color >>> 24;
      ((color as u32) >> 24) as i32
    }

    // Java: static int red(int color)
    pub fn red(color: i32) -> i32 {
      // Java: return color >> 16 & 255;
      (color >> 16) & 255
    }

    // Java: static int green(int color)
    pub fn green(color: i32) -> i32 {
      // Java: return color >> 8 & 255;
      (color >> 8) & 255
    }

    // Java: static int blue(int color)
    pub fn blue(color: i32) -> i32 {
      // Java: return color & 255;
      color & 255
    }

}
