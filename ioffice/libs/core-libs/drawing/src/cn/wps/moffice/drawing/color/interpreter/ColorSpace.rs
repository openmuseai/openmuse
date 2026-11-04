#![allow(non_snake_case, dead_code, unused_variables)]

// Java: public class ColorSpace
pub struct ColorSpace {
}

// Java: public class RGBA
pub struct RGBA {
    // Java field: double r
    pub(crate) r: f64,
    // Java field: double g
    pub(crate) g: f64,
    // Java field: double b
    pub(crate) b: f64,
    // Java field: double a
    pub(crate) a: f64,
    // Java field: boolean gammaCorrected
    pub(crate) gammaCorrected: bool,
}

// Java: public class RGB
pub struct RGB {
    // Java field: double r
    pub(crate) r: f64,
    // Java field: double g
    pub(crate) g: f64,
    // Java field: double b
    pub(crate) b: f64,
}

// Java: public class CRGB
pub struct CRGB {
    // Java field: double r
    pub(crate) r: f64,
    // Java field: double g
    pub(crate) g: f64,
    // Java field: double b
    pub(crate) b: f64,
}

// Java: public class HSL
pub struct HSL {
    // Java field: double h
    pub(crate) h: f64,
    // Java field: double s
    pub(crate) s: f64,
    // Java field: double l
    pub(crate) l: f64,
}

// Java: public class ColorSpace
impl ColorSpace {
    // Java: ColorSpace()
    pub fn new() -> Self {
      // Java: super();
      ColorSpace {}
    }

    // Java: static double revertGamma(double v)
    pub fn revertGamma(v: f64) -> f64 {
      // Java: return v <= 0.04045 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4);
      if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055_f64).powf(2.4) }
    }

    // Java: static double gammaCorrected(double v)
    pub fn gammaCorrected(v: f64) -> f64 {
      // Java: double EXP = 0.4166666666666667;
      let EXP: f64 = 0.4166666666666667;
      // Java: return v <= 0.0031308 ? v * 12.92 : 1.055 * Math.pow(v, 0.4166666666666667) - 0.055;
      if v <= 0.0031308 { v * 12.92 } else { 1.055 * v.powf(0.4166666666666667) - 0.055 }
    }

    // Java: static void RGBToHsl(double r, double g, double b, HSL hsl)
    pub fn RGBToHsl(r: f64, g: f64, b: f64, hsl: &mut HSL) {
      // Java: double max = Math.max(Math.max(r, g), b);
      let max = r.max(g).max(b);
      // Java: double min = Math.min(Math.min(r, g), b);
      let min = r.min(g).min(b);
      // Java: double delta = max - min;
      let delta = max - min;
      // Java: double delta2 = max + min;
      let delta2 = max + min;
      // Java: hsl.h = (double)0.0F;
      hsl.h = 0.0;
      // Java: hsl.l = (double)0.5F * delta2;
      hsl.l = 0.5 * delta2;
      // Java: if (Double.compare(delta, (double)0.0F) == 0)
      if delta == 0.0 {
         // Java: hsl.s = (double)0.0F;
         hsl.s = 0.0;
      // Java: else
      } else {
         // Java: hsl.s = hsl.l < (double)0.5F ? delta / delta2 : delta / ((double)2.0F - delta2);
         hsl.s = if hsl.l < 0.5 { delta / delta2 } else { delta / (2.0 - delta2) };
         // Java: if (Double.compare(r, max) == 0)
         if r == max {
            // Java: hsl.h = (g - b) / delta + (g < b ? (double)6.0F : (double)0.0F);
            hsl.h = (g - b) / delta + if g < b { 6.0 } else { 0.0 };
         // Java: else if (Double.compare(g, max) == 0)
         } else if g == max {
            // Java: hsl.h = (double)2.0F + (b - r) / delta;
            hsl.h = 2.0 + (b - r) / delta;
         // Java: else if (Double.compare(b, max) == 0)
         } else if b == max {
            // Java: hsl.h = (double)4.0F + (r - g) / delta;
            hsl.h = 4.0 + (r - g) / delta;
         // Java: else
         } else {
            // Java: assert false;
            debug_assert!(false);
         }
         // Java: hsl.h *= 0.16666666666666666;
         hsl.h *= 0.16666666666666666;
      }
    }

    // Java: static void HslToRGB(double h, double s, double l, RGB rgb)
    pub fn HslToRGB(h: f64, s: f64, l: f64, rgb: &mut RGB) {
      // Java: if (Double.compare(h, (double)1.0F) == 0)
      if h == 1.0 {
         // Java: rgb.r = rgb.g = rgb.b = l;
         rgb.r = l;
         rgb.g = l;
         rgb.b = l;
      // Java: else if (s < (double)0.0F)
      } else if s < 0.0 {
         // Java: double q = l < (double)0.5F ? l * ((double)1.0F - s) : l - s + l * s;
         let q = if l < 0.5 { l * (1.0 - s) } else { l - s + l * s };
         // Java: double p = (double)2.0F * l - q;
         let p = 2.0 * l - q;
         // Java: rgb.r = (double)2.0F * l - HueToRGB(p, q, h + 0.3333333333333333);
         rgb.r = 2.0 * l - Self::HueToRGB(p, q, h + 0.3333333333333333);
         // Java: rgb.g = (double)2.0F * l - HueToRGB(p, q, h);
         rgb.g = 2.0 * l - Self::HueToRGB(p, q, h);
         // Java: rgb.b = (double)2.0F * l - HueToRGB(p, q, h - 0.3333333333333333);
         rgb.b = 2.0 * l - Self::HueToRGB(p, q, h - 0.3333333333333333);
         // Java: rgb.r = Math.max((double)0.0F, Math.min(rgb.r, (double)1.0F));
         rgb.r = (0.0_f64).max(rgb.r.min(1.0));
         // Java: rgb.g = Math.max((double)0.0F, Math.min(rgb.g, (double)1.0F));
         rgb.g = (0.0_f64).max(rgb.g.min(1.0));
         // Java: rgb.b = Math.max((double)0.0F, Math.min(rgb.b, (double)1.0F));
         rgb.b = (0.0_f64).max(rgb.b.min(1.0));
      // Java: else if (Double.compare(l, (double)0.0F) == 0)
      } else if l == 0.0 {
         // Java: rgb.r = rgb.g = rgb.b = (double)0.0F;
         rgb.r = 0.0;
         rgb.g = 0.0;
         rgb.b = 0.0;
      // Java: else
      } else {
         // Java: double q = l < (double)0.5F ? l * ((double)1.0F + s) : l + s - l * s;
         let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
         // Java: double p = (double)2.0F * l - q;
         let p = 2.0 * l - q;
         // Java: rgb.r = HueToRGB(p, q, h + 0.3333333333333333);
         rgb.r = Self::HueToRGB(p, q, h + 0.3333333333333333);
         // Java: rgb.g = HueToRGB(p, q, h);
         rgb.g = Self::HueToRGB(p, q, h);
         // Java: rgb.b = HueToRGB(p, q, h - 0.3333333333333333);
         rgb.b = Self::HueToRGB(p, q, h - 0.3333333333333333);
         // Java: rgb.r = Math.max((double)0.0F, Math.min(rgb.r, (double)1.0F));
         rgb.r = (0.0_f64).max(rgb.r.min(1.0));
         // Java: rgb.g = Math.max((double)0.0F, Math.min(rgb.g, (double)1.0F));
         rgb.g = (0.0_f64).max(rgb.g.min(1.0));
         // Java: rgb.b = Math.max((double)0.0F, Math.min(rgb.b, (double)1.0F));
         rgb.b = (0.0_f64).max(rgb.b.min(1.0));
      }
    }

    // Java: static double HueToRGB(double p, double q, double t)
    pub fn HueToRGB(p: f64, q: f64, mut t: f64) -> f64 {
      // Java: if (t < (double)0.0F)
      if t < 0.0 {
         // Java: ++t;
         t += 1.0;
      }
      // Java: if (t > (double)1.0F)
      if t > 1.0 {
         // Java: --t;
         t -= 1.0;
      }
      // Java: if (t < 0.16666666666666666)
      if t < 0.16666666666666666 {
         // Java: return p + (q - p) * (double)6.0F * t;
         return p + (q - p) * 6.0 * t;
      // Java: else if (t < (double)0.5F)
      } else if t < 0.5 {
         // Java: return q;
         return q;
      // Java: else
      } else {
         // Java: return t < 0.6666666666666666 ? p + (q - p) * (0.6666666666666666 - t) * (double)6.0F : p;
         return if t < 0.6666666666666666 { p + (q - p) * (0.6666666666666666 - t) * 6.0 } else { p };
      }
    }

}

// Java: public class RGBA
impl RGBA {
    // Java: RGBA()
    pub fn new() -> Self {
         // Java: super();
         RGBA {
             r: 0.0,
             g: 0.0,
             b: 0.0,
             a: 1.0,
             gammaCorrected: true,
         }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
         // Java: this.r = (double)0.0F;
         self.r = 0.0;
         // Java: this.g = (double)0.0F;
         self.g = 0.0;
         // Java: this.b = (double)0.0F;
         self.b = 0.0;
         // Java: this.a = (double)1.0F;
         self.a = 1.0;
         // Java: this.gammaCorrected = true;
         self.gammaCorrected = true;
    }

    // Java: void setRGB(double r, double g, double b)
    pub fn setRGB(&mut self, r: f64, g: f64, b: f64) {
         // Java: this.r = r;
         self.r = r;
         // Java: this.g = g;
         self.g = g;
         // Java: this.b = b;
         self.b = b;
         // Java: this.gammaCorrected = true;
         self.gammaCorrected = true;
    }

    // Java: void gammaCorrect()
    pub fn gammaCorrect(&mut self) {
         // Java: if (!this.gammaCorrected)
         if !self.gammaCorrected {
            // Java: this.r = ColorSpace.gammaCorrected(this.r);
            self.r = ColorSpace::gammaCorrected(self.r);
            // Java: this.g = ColorSpace.gammaCorrected(this.g);
            self.g = ColorSpace::gammaCorrected(self.g);
            // Java: this.b = ColorSpace.gammaCorrected(this.b);
            self.b = ColorSpace::gammaCorrected(self.b);
            // Java: this.gammaCorrected = true;
            self.gammaCorrected = true;
         }
    }

    // Java: void revertGamma()
    pub fn revertGamma(&mut self) {
         // Java: if (this.gammaCorrected)
         if self.gammaCorrected {
            // Java: this.r = ColorSpace.revertGamma(this.r);
            self.r = ColorSpace::revertGamma(self.r);
            // Java: this.g = ColorSpace.revertGamma(this.g);
            self.g = ColorSpace::revertGamma(self.g);
            // Java: this.b = ColorSpace.revertGamma(this.b);
            self.b = ColorSpace::revertGamma(self.b);
            // Java: this.gammaCorrected = false;
            self.gammaCorrected = false;
         }
    }

    // Java: RGB rgb()
    pub fn rgb(&self) -> RGB {
         // Java: return new RGB(this.r, this.g, this.b);
         RGB::new_with_rgb(self.r, self.g, self.b)
    }

}

// Java: public class RGB
impl RGB {
    // Java: RGB()
    pub fn new() -> Self {
         // Java: super();
         RGB {
             r: 0.0,
             g: 0.0,
             b: 0.0,
         }
    }

    // Java: RGB(double r, double g, double b)
    pub fn new_with_rgb(r: f64, g: f64, b: f64) -> Self {
         // Java: super();
         let mut instance = RGB { r: 0.0, g: 0.0, b: 0.0 };
         instance.init_rgb(r, g, b);
         instance
    }

    fn init_rgb(&mut self, r: f64, g: f64, b: f64) {
         // Java: this.r = r;
         self.r = r;
         // Java: this.g = g;
         self.g = g;
         // Java: this.b = b;
         self.b = b;
    }

    // Java: void subtract(RGB o)
    pub fn subtract(&mut self, o: &RGB) {
         // Java: this.r -= o.r;
         self.r -= o.r;
         // Java: this.g -= o.g;
         self.g -= o.g;
         // Java: this.b -= o.b;
         self.b -= o.b;
    }

}

// Java: public class CRGB
impl CRGB {
    // Java: CRGB(double r, double g, double b)
    pub fn new(r: f64, g: f64, b: f64) -> Self {
         // Java: super();
         let mut instance = CRGB { r: 0.0, g: 0.0, b: 0.0 };
         instance.init_rgb(r, g, b);
         instance
    }

    fn init_rgb(&mut self, r: f64, g: f64, b: f64) {
         // Java: this.r = r;
         self.r = r;
         // Java: this.g = g;
         self.g = g;
         // Java: this.b = b;
         self.b = b;
    }

}

// Java: public class HSL
impl HSL {
    // Java: HSL()
    pub fn new() -> Self {
         // Java: super();
         HSL {
             h: 0.0,
             s: 0.0,
             l: 0.0,
         }
    }

    // Java: HSL(double h, double s, double l)
    pub fn new_with_hsl(h: f64, s: f64, l: f64) -> Self {
         // Java: super();
         let mut instance = HSL { h: 0.0, s: 0.0, l: 0.0 };
         instance.init_hsl(h, s, l);
         instance
    }

    fn init_hsl(&mut self, h: f64, s: f64, l: f64) {
         // Java: this.h = h;
         self.h = h;
         // Java: this.s = s;
         self.s = s;
         // Java: this.l = l;
         self.l = l;
    }

    // Java: void subtract(HSL o)
    pub fn subtract(&mut self, o: &HSL) {
         // Java: this.h -= o.h;
         self.h -= o.h;
         // Java: this.s -= o.s;
         self.s -= o.s;
         // Java: this.l -= o.l;
         self.l -= o.l;
    }

}
