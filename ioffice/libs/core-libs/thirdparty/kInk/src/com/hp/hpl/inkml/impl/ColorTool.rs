// Java field: float LIGHT_COLOR_SCALE
pub(crate) const LIGHT_COLOR_SCALE: f32 = 0.6;
// Java field: int CHAR_NIGHT_COLOR
pub(crate) const CHAR_NIGHT_COLOR: i32 = -6514020;
// Java field: int BACK_NIGHT_COLOR
pub(crate) const BACK_NIGHT_COLOR: i32 = -13553359;

// Java: public class ColorTool
pub struct ColorTool {
}

// Java: public class ColorTool
impl ColorTool {
    // Java: ColorTool()
    pub fn ColorTool() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static int changeColorAtNightMode(int color)
    pub fn changeColorAtNightMode(color: i32) -> i32 {
      // Java: int alpha = color >>> 24;
      let alpha = ((color as u32) >> 24) as i32;
      // Java: int red = color >> 16 & 255;
      let mut red = (color >> 16) & 255;
      // Java: int green = color >> 8 & 255;
      let mut green = (color >> 8) & 255;
      // Java: int blue = color & 255;
      let mut blue = color & 255;
      // Java: red = (int)((float)red * 0.6F);
      red = (red as f32 * 0.6) as i32;
      // Java: green = (int)((float)green * 0.6F);
      green = (green as f32 * 0.6) as i32;
      // Java: blue = (int)((float)blue * 0.6F);
      blue = (blue as f32 * 0.6) as i32;
      // Java: if (isSimilarColor(-13553359, rgb(red, green, blue), 63))
      if Self::isSimilarColor(-13553359, Self::rgb(red, green, blue), 63)
      {
         // Java: red = 255 - red;
         red = 255 - red;
         // Java: green = 255 - green;
         green = 255 - green;
         // Java: blue = 255 - blue;
         blue = 255 - blue;
         // Java: red = (int)((float)red * 0.6F);
         red = (red as f32 * 0.6) as i32;
         // Java: green = (int)((float)green * 0.6F);
         green = (green as f32 * 0.6) as i32;
         // Java: blue = (int)((float)blue * 0.6F);
         blue = (blue as f32 * 0.6) as i32;
         // Java: if (isSimilarColor(-13553359, rgb(red, green, blue), 63))
         if Self::isSimilarColor(-13553359, Self::rgb(red, green, blue), 63)
         {
            // Java: return -6514020;
            return -6514020;
         }
      }
      // Java: return alpha << 24 | red << 16 | green << 8 | blue;
      (alpha << 24) | (red << 16) | (green << 8) | blue
    }

    // Java: static boolean isSimilarColor(int color1, int color2, int factor)
    pub fn isSimilarColor(color1: i32, color2: i32, factor: i32) -> bool {
      // Java: int red1 = red(color1);
      let red1 = Self::red(color1);
      // Java: int red2 = red(color2);
      let red2 = Self::red(color2);
      // Java: int green1 = green(color1);
      let green1 = Self::green(color1);
      // Java: int green2 = green(color2);
      let green2 = Self::green(color2);
      // Java: int blue1 = blue(color1);
      let blue1 = Self::blue(color1);
      // Java: int blue2 = blue(color2);
      let blue2 = Self::blue(color2);
      // Java: float diff = (float)square(red1 - red2) * 0.2126F + (float)square(green1 - green2) * 0.7152F + (float)square(blue1 - blue2) * 0.0722F;
      let diff = Self::square(red1 - red2) as f32 * 0.2126 + Self::square(green1 - green2) as f32 * 0.7152 + Self::square(blue1 - blue2) as f32 * 0.0722;
      // Java: return diff < (float)square(factor);
      diff < Self::square(factor) as f32
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

    // Java: static int rgb(int red, int green, int blue)
    pub fn rgb(red: i32, green: i32, blue: i32) -> i32 {
      // Java: return -16777216 | red << 16 | green << 8 | blue;
      -16777216i32 | (red << 16) | (green << 8) | blue
    }

    // Java: static int argb(int alpha, int red, int green, int blue)
    pub fn argb(alpha: i32, red: i32, green: i32, blue: i32) -> i32 {
      // Java: return alpha << 24 | red << 16 | green << 8 | blue;
      (alpha << 24) | (red << 16) | (green << 8) | blue
    }

    // Java: static int square(int value)
    pub fn square(value: i32) -> i32 {
      // Java: return value * value;
      value * value
    }

    // Java: static int brightness(int color)
    pub fn brightness(color: i32) -> i32 {
      // Java: return Math.max(blue(color), Math.max(red(color), green(color)));
      Self::blue(color).max(Self::red(color).max(Self::green(color)))
    }

}
