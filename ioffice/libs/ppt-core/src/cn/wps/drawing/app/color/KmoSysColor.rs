// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::drawing::data::color::Color::Color;

// Java field: int WINDOW_COLOR
pub(crate) const WINDOW_COLOR: i32 = -1;
// Java field: int WINDOW_TEXT_COLOR
pub(crate) const WINDOW_TEXT_COLOR: i32 = -16777216;

// Java: public class KmoSysColor
pub struct KmoSysColor {
    // Java field: int mWindowColor
    mWindowColor: i32,
    // Java field: int mWindowTextColor
    mWindowTextColor: i32,
}

// Java: public class KmoSysColor
impl KmoSysColor {
    // Java: int getSysColor(Color sys)
    pub fn getSysColor(&self, sys: &mut Color) -> i32 {
      // Java: int color;
      let color: i32;
      // Java: switch (sys.sys())
      match sys.sys()
      {
         // Java: case 5:
         // LLM JavaLine Aggregation: fall-through
         // Java: case 27:
         5 | 27 =>
         {
            // Java: color = this.mWindowColor;
            color = self.mWindowColor;
            // Java: break;
            // LLM No Rust equivalent
         }
         // Java: case 29:
         29 =>
         {
            // Java: color = this.mWindowTextColor;
            color = self.mWindowTextColor;
            // Java: break;
            // LLM No Rust equivalent
         }
         // Java: default:
         _ =>
         {
            // Java: i32 rgb = sys.sysLastClr();
            let rgb = sys.sysLastClr();
            // Java: color = rgb(rgb.r(), rgb.g(), rgb.b());
            color = Self::rgb(rgb.r(), rgb.g(), rgb.b());
         }
      }
      // Java: return color;
      color
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
      -16777216 | red << 16 | green << 8 | blue
    }

    // Java: static int alpha(int color)
    pub fn alpha(color: i32) -> i32 {
      // Java: return color >>> 24;
      ((color as u32) >> 24) as i32
    }

    // Java: static int red(int color)
    pub fn red(color: i32) -> i32 {
      // Java: return color >> 16 & 255;
      color >> 16 & 255
    }

    // Java: static int green(int color)
    pub fn green(color: i32) -> i32 {
      // Java: return color >> 8 & 255;
      color >> 8 & 255
    }

    // Java: static int blue(int color)
    pub fn blue(color: i32) -> i32 {
      // Java: return color & 255;
      color & 255
    }

}
