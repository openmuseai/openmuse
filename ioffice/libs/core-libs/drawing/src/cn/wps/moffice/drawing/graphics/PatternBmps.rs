#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use std::any::Any;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: import cn.wps.moffice.util.ColorUtil
use core_libs_util::cn::wps::moffice::util::ColorUtil::ColorUtil;

struct Bitmap;
impl Bitmap {
    pub fn createBitmap(_colors: &[i32], _width: i32, _height: i32) -> Element { None }
    pub fn getWidth(_bitmap: &Element) -> i32 { 0 }
    pub fn getHeight(_bitmap: &Element) -> i32 { 0 }
    pub fn getPixels(_bitmap: &Element, _colors: &mut [i32], _offset: i32, _stride: i32, _x: i32, _y: i32, _width: i32, _height: i32) {}
}

// Java field: byte[] darkdownwarddiagonal
static darkdownwarddiagonal: [u8; 64] = [1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1];
// Java field: byte[] darkhorizontal
static darkhorizontal: [u8; 64] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
// Java field: byte[] darkupwarddiagonal
static darkupwarddiagonal: [u8; 64] = [0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1];
// Java field: byte[] darkvertical
static darkvertical: [u8; 64] = [1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0];
// Java field: byte[] thinhorizontal
static thinhorizontal: [u8; 64] = [1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
// Java field: byte[] thinvertical
static thinvertical: [u8; 64] = [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0];
// Java field: byte[] downwarddiagonal
static downwarddiagonal: [u8; 64] = [1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1];
// Java field: byte[] upwarddiagonal
static upwarddiagonal: [u8; 64] = [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0];
// Java field: byte[] dottedgrid
static dottedgrid: [u8; 64] = [1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1];
// Java field: byte[] eightycentesimal
static eightycentesimal: [u8; 64] = [1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1];
// Java field: byte[] twopointfivetesimal
static twopointfivetesimal: [u8; 64] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
// Java field: byte[] fivecentesimal
static fivecentesimal: [u8; 64] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
// Java field: byte[] fiftycentesimal
static fiftycentesimal: [u8; 64] = [1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1];
// Java field: byte[] fortysentesimal
static fortysentesimal: [u8; 64] = [1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1];
// Java field: byte[] ninetycentesimal
static ninetycentesimal: [u8; 64] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1];
// Java field: byte[] solid
static solid: [u8; 64] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
// Java field: byte[] seventyfivecentesimal
static seventyfivecentesimal: [u8; 64] = [0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1];
// Java field: byte[] seventycentesimal
static seventycentesimal: [u8; 64] = [0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1];
// Java field: byte[] sixtycentesimal
static sixtycentesimal: [u8; 64] = [1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1];
// Java field: byte[] smallcheckerboard
static smallcheckerboard: [u8; 64] = [1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1];
// Java field: byte[] smallgrid
static smallgrid: [u8; 64] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0];
// Java field: byte[] tencentesimal
static tencentesimal: [u8; 64] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
// Java field: byte[] thirtycentesimal
static thirtycentesimal: [u8; 64] = [1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1];
// Java field: byte[] twentycentesimal
static twentycentesimal: [u8; 64] = [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];
// Java field: byte[] twentyfivecentesimal
static twentyfivecentesimal: [u8; 64] = [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0];

// Java: public class PatternBmps
pub struct PatternBmps {
}

// Java: public class PatternBmps
impl PatternBmps {
    // Java: PatternBmps()
    pub fn new() -> Self {
      // Java: super();
      PatternBmps {}
    }

    // Java: static Integer getColorByIndex(int index, int patternColor, int backColor)
    pub fn getColorByIndex(index: i32, patternColor: i32, backColor: i32) -> Option<i32> {
      // Java: if (index != 65535 && index != 0)
      if index != 65535 && index != 0 {
         // Java: patternColor = ColorUtil.rgb2Argb(patternColor);
         let patternColor = ColorUtil::rgb2Argb(patternColor);
         // Java: backColor = ColorUtil.rgb2Argb(backColor);
         let backColor = ColorUtil::rgb2Argb(backColor);
         // Java: return patternColor == -16777216 && backColor == -1 ? getSpecialColorByIndex(index) : null;
         let patternColor = patternColor;
         if patternColor == -16777216_i32 && backColor == -1_i32 { return Self::getSpecialColorByIndex(index); } else { return None; }
      // Java: else
      } else {
         // Java: return null;
         return None;
      }
    }

    // Java: static Bitmap getBitmapByIndex(int index, int patternColor, int backColor)
    pub fn getBitmapByIndex(index: i32, patternColor: i32, backColor: i32) -> Element {
      // Java: if (index != 65535 && index != 0)
      if index != 65535 && index != 0 {
         // Java: patternColor = ColorUtil.rgb2Argb(patternColor);
         let patternColor = ColorUtil::rgb2Argb(patternColor);
         // Java: backColor = ColorUtil.rgb2Argb(backColor);
         let backColor = ColorUtil::rgb2Argb(backColor);
         // Java: if (patternColor == -16777216 && backColor == -1)
         if patternColor == -16777216_i32 && backColor == -1_i32 {
            // Java: int[] colors = getSpecialColorsByIndex(index);
            let colors = Self::getSpecialColorsByIndex(index);
            // Java: if (colors != null)
            if colors.is_some() {
               // Java: return Bitmap.createBitmap(colors, 8, 8, Config.RGB_565);
               return Bitmap::createBitmap(&colors.unwrap(), 8, 8);
            }
         }
         // Java: byte[] colors = getColorsByIndex(index);
         let colors = Self::getColorsByIndex(index);
         // Java: if (colors == null)
         if colors.is_none() {
            // Java: return null;
            return None;
         // Java: else
         } else {
            // Java: int[] newColors = getBitmap(colors, patternColor, backColor);
            let newColors = Self::getBitmap(&colors.unwrap(), patternColor, backColor);
            // Java: return Bitmap.createBitmap(newColors, 8, 8, Config.ARGB_4444);
            return Bitmap::createBitmap(&newColors, 8, 8);
         }
      // Java: else
      } else {
         // Java: return null;
         return None;
      }
    }

    // Java: static Bitmap createColorBitmap(Bitmap bitmap, int patternColor, int backColor)
    pub fn createColorBitmap(bitmap: &Element, patternColor: i32, backColor: i32) -> Element {
      // Java: int width = bitmap.getWidth();
      let width: i32 = Bitmap::getWidth(bitmap);
      // Java: int height = bitmap.getHeight();
      let height: i32 = Bitmap::getHeight(bitmap);
      // Java: int[] colors = new int[width * height];
      let mut colors = vec![0_i32; (width * height) as usize];
      // Java: bitmap.getPixels(colors, 0, width, 0, 0, width, height);
      Bitmap::getPixels(bitmap, &mut colors, 0, width, 0, 0, width, height);
      // Java: patternColor = ColorUtil.rgb2Argb(patternColor);
      let patternColor = ColorUtil::rgb2Argb(patternColor);
      // Java: backColor = ColorUtil.rgb2Argb(backColor);
      let backColor = ColorUtil::rgb2Argb(backColor);
      // Java: int size = width * height;
      let size = width * height;
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size as usize {
         // Java: if ((colors[i] & 16777215) == 0)
         if (colors[i] & 16777215) == 0 {
            // Java: colors[i] = backColor;
            colors[i] = backColor;
         // Java: else
         } else {
            // Java: colors[i] = patternColor;
            colors[i] = patternColor;
         }
      }
      // Java: return Bitmap.createBitmap(colors, width, height, Config.ARGB_4444);
      Bitmap::createBitmap(&colors, width, height)
    }

    // Java: static int[] getBitmap(byte[] colors, int patternColor, int backColor)
    pub fn getBitmap(colors: &[u8], patternColor: i32, backColor: i32) -> Vec<i32> {
      // Java: int[] newColors = new int[64];
      let mut newColors = vec![0_i32; 64];
      // Java: for(int i = 0; i < 64; ++i)
      for i in 0..64_usize {
         // Java: if (colors[i] == 1)
         if colors[i] == 1 {
            // Java: newColors[i] = patternColor;
            newColors[i] = patternColor;
         // Java: else
         } else {
            // Java: newColors[i] = backColor;
            newColors[i] = backColor;
         }
      }
      // Java: return newColors;
      newColors
    }

    // Java: static byte[] getColorsByIndex(int index)
    pub fn getColorsByIndex(index: i32) -> Option<Vec<u8>> {
      // Java: switch (index)
      match index
      {
         // Java: case 1:
         1 =>
            // Java: return solid;
            { return Some(solid.to_vec()); }
         // Java: case 2:
         2 =>
            // Java: return fivecentesimal;
            { return Some(fivecentesimal.to_vec()); }
         // Java: case 3:
         3 =>
            // Java: return tencentesimal;
            { return Some(tencentesimal.to_vec()); }
         // Java: case 4:
         4 =>
            // Java: return twentycentesimal;
            { return Some(twentycentesimal.to_vec()); }
         // Java: case 5:
         5 =>
            // Java: return twentyfivecentesimal;
            { return Some(twentyfivecentesimal.to_vec()); }
         // Java: case 6:
         6 =>
            // Java: return thirtycentesimal;
            { return Some(thirtycentesimal.to_vec()); }
         // Java: case 7:
         7 =>
            // Java: return fortysentesimal;
            { return Some(fortysentesimal.to_vec()); }
         // Java: case 8:
         8 =>
            // Java: return fiftycentesimal;
            { return Some(fiftycentesimal.to_vec()); }
         // Java: case 9:
         9 =>
            // Java: return sixtycentesimal;
            { return Some(sixtycentesimal.to_vec()); }
         // Java: case 10:
         10 =>
            // Java: return seventycentesimal;
            { return Some(seventycentesimal.to_vec()); }
         // Java: case 11:
         11 =>
            // Java: return seventyfivecentesimal;
            { return Some(seventyfivecentesimal.to_vec()); }
         // Java: case 12:
         12 =>
            // Java: return eightycentesimal;
            { return Some(eightycentesimal.to_vec()); }
         // Java: case 13:
         13 =>
            // Java: return ninetycentesimal;
            { return Some(ninetycentesimal.to_vec()); }
         // Java: case 14:
         14 =>
            // Java: return darkhorizontal;
            { return Some(darkhorizontal.to_vec()); }
         // Java: case 15:
         15 =>
            // Java: return darkvertical;
            { return Some(darkvertical.to_vec()); }
         // Java: case 16:
         16 =>
            // Java: return darkdownwarddiagonal;
            { return Some(darkdownwarddiagonal.to_vec()); }
         // Java: case 17:
         17 =>
            // Java: return darkupwarddiagonal;
            { return Some(darkupwarddiagonal.to_vec()); }
         // Java: case 18:
         18 =>
            // Java: return smallcheckerboard;
            { return Some(smallcheckerboard.to_vec()); }
         // Java: case 19:
         19 =>
            // Java: return smallcheckerboard;
            { return Some(smallcheckerboard.to_vec()); }
         // Java: case 20:
         20 =>
            // Java: return thinhorizontal;
            { return Some(thinhorizontal.to_vec()); }
         // Java: case 21:
         21 =>
            // Java: return thinvertical;
            { return Some(thinvertical.to_vec()); }
         // Java: case 22:
         22 =>
            // Java: return downwarddiagonal;
            { return Some(downwarddiagonal.to_vec()); }
         // Java: case 23:
         23 =>
            // Java: return upwarddiagonal;
            { return Some(upwarddiagonal.to_vec()); }
         // Java: case 24:
         24 =>
            // Java: return smallgrid;
            { return Some(smallgrid.to_vec()); }
         // Java: case 25:
         25 =>
            // Java: return dottedgrid;
            { return Some(dottedgrid.to_vec()); }
         // Java: case 26:
         // Java: case 27:
         // Java: case 28:
         // Java: case 29:
         // Java: case 30:
         // Java: case 31:
         // Java: case 32:
         // Java: case 33:
         // Java: case 34:
         // Java: default:
         26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 =>
            // Java: return null;
            { return None; }
         // Java: case 35:
         35 =>
            // Java: return twopointfivetesimal;
            { return Some(twopointfivetesimal.to_vec()); }
         // Java: case 36:
         36 =>
            // Java: return getColorByPlus(twopointfivetesimal, fivecentesimal);
            { return Some(Self::getColorByPlus(&twopointfivetesimal, &fivecentesimal)); }
         // Java: case 37:
         37 =>
            // Java: return getColorByPlus(twopointfivetesimal, tencentesimal);
            { return Some(Self::getColorByPlus(&twopointfivetesimal, &tencentesimal)); }
         // Java: case 38:
         38 =>
            // Java: return getColorByPlus(fivecentesimal, tencentesimal);
            { return Some(Self::getColorByPlus(&fivecentesimal, &tencentesimal)); }
         // Java: case 39:
         39 =>
            // Java: return getColorByPlus(twopointfivetesimal, fivecentesimal, tencentesimal);
            { return Some(Self::getColorByPlus_three(&twopointfivetesimal, &fivecentesimal, &tencentesimal)); }
         // Java: case 40:
         40 =>
            // Java: return getColorByPlus(twopointfivetesimal, twentycentesimal);
            { return Some(Self::getColorByPlus(&twopointfivetesimal, &twentycentesimal)); }
         // Java: case 41:
         41 =>
            // Java: return getColorByPlus(twopointfivetesimal, twentyfivecentesimal);
            { return Some(Self::getColorByPlus(&twopointfivetesimal, &twentyfivecentesimal)); }
         // Java: case 42:
         42 =>
            // Java: return getColorByPlus(twopointfivetesimal, thirtycentesimal);
            { return Some(Self::getColorByPlus(&twopointfivetesimal, &thirtycentesimal)); }
         // Java: case 43:
         43 =>
            // Java: return getColorByPlus(fivecentesimal, thirtycentesimal);
            { return Some(Self::getColorByPlus(&fivecentesimal, &thirtycentesimal)); }
         // Java: case 44:
         44 =>
            // Java: return getColorByPlus(twopointfivetesimal, fivecentesimal, thirtycentesimal);
            { return Some(Self::getColorByPlus_three(&twopointfivetesimal, &fivecentesimal, &thirtycentesimal)); }
         // Java: case 45:
         45 =>
            // Java: return getColorByPlus(twopointfivetesimal, fortysentesimal);
            { return Some(Self::getColorByPlus(&twopointfivetesimal, &fortysentesimal)); }
         // Java: case 46:
         46 =>
            // Java: return getColorByPlus(fivecentesimal, fortysentesimal);
            { return Some(Self::getColorByPlus(&fivecentesimal, &fortysentesimal)); }
         // Java: case 47:
         47 =>
            // Java: return getColorByPlus(twopointfivetesimal, fivecentesimal, fortysentesimal);
            { return Some(Self::getColorByPlus_three(&twopointfivetesimal, &fivecentesimal, &fortysentesimal)); }
         // Java: case 48:
         48 =>
            // Java: return getColorByPlus(twopointfivetesimal, fiftycentesimal);
            { return Some(Self::getColorByPlus(&twopointfivetesimal, &fiftycentesimal)); }
         // Java: case 49:
         49 =>
            // Java: return getColorByPlus(fivecentesimal, fiftycentesimal);
            { return Some(Self::getColorByPlus(&fivecentesimal, &fiftycentesimal)); }
         // Java: case 50:
         50 =>
            // Java: return getColorByPlus(twopointfivetesimal, fivecentesimal, fiftycentesimal);
            { return Some(Self::getColorByPlus_three(&twopointfivetesimal, &fivecentesimal, &fiftycentesimal)); }
         // Java: case 51:
         51 =>
            // Java: return getColorByPlus(twopointfivetesimal, sixtycentesimal);
            { return Some(Self::getColorByPlus(&twopointfivetesimal, &sixtycentesimal)); }
         // Java: case 52:
         52 =>
            // Java: return getColorByPlus(fivecentesimal, sixtycentesimal);
            { return Some(Self::getColorByPlus(&fivecentesimal, &sixtycentesimal)); }
         // Java: case 53:
         53 =>
            // Java: return getColorByPlus(twopointfivetesimal, fivecentesimal, sixtycentesimal);
            { return Some(Self::getColorByPlus_three(&twopointfivetesimal, &fivecentesimal, &sixtycentesimal)); }
         // Java: case 54:
         54 =>
            // Java: return getColorByPlus(twopointfivetesimal, seventycentesimal);
            { return Some(Self::getColorByPlus(&twopointfivetesimal, &seventycentesimal)); }
         // Java: case 55:
         55 =>
            // Java: return getColorByPlus(twopointfivetesimal, seventyfivecentesimal);
            { return Some(Self::getColorByPlus(&twopointfivetesimal, &seventyfivecentesimal)); }
         // Java: case 56:
         56 =>
            // Java: return getColorByPlus(twopointfivetesimal, eightycentesimal);
            { return Some(Self::getColorByPlus(&twopointfivetesimal, &eightycentesimal)); }
         // Java: case 57:
         57 =>
            // Java: return getColorByPlus(fivecentesimal, eightycentesimal);
            { return Some(Self::getColorByPlus(&fivecentesimal, &eightycentesimal)); }
         // Java: case 58:
         58 =>
            // Java: return getColorByPlus(twopointfivetesimal, fivecentesimal, eightycentesimal);
            { return Some(Self::getColorByPlus_three(&twopointfivetesimal, &fivecentesimal, &eightycentesimal)); }
         // Java: case 59:
         59 =>
            // Java: return getColorByPlus(twopointfivetesimal, ninetycentesimal);
            { return Some(Self::getColorByPlus(&twopointfivetesimal, &ninetycentesimal)); }
         // Java: case 60:
         60 =>
            // Java: return getColorByPlus(fivecentesimal, ninetycentesimal);
            { return Some(Self::getColorByPlus(&fivecentesimal, &ninetycentesimal)); }
         // Java: case 61:
         // Java: case 62:
         61 | 62 =>
            // Java: return getColorByPlus(twopointfivetesimal, fivecentesimal, ninetycentesimal);
            { return Some(Self::getColorByPlus_three(&twopointfivetesimal, &fivecentesimal, &ninetycentesimal)); }
         _ =>
            // Java: return null;
            { return None; }
      }
    }

    // Java: static Integer getSpecialColorByIndex(int index)
    pub fn getSpecialColorByIndex(index: i32) -> Option<i32> {
      // Java: float percent = -1.0F;
      let mut percent: f32 = -1.0_f32;
      // Java: switch (index)
      match index
      {
         // Java: case 2:
         2 => {
            // Java: percent = 0.05F;
            percent = 0.05_f32;
            // Java: break;
         }
         // Java: case 3:
         3 => {
            // Java: percent = 0.1F;
            percent = 0.1_f32;
            // Java: break;
         }
         // Java: case 4:
         4 => {
            // Java: percent = 0.2F;
            percent = 0.2_f32;
            // Java: break;
         }
         // Java: case 5:
         5 => {
            // Java: percent = 0.25F;
            percent = 0.25_f32;
            // Java: break;
         }
         // Java: case 6:
         6 => {
            // Java: percent = 0.3F;
            percent = 0.3_f32;
            // Java: break;
         }
         // Java: case 7:
         7 => {
            // Java: percent = 0.4F;
            percent = 0.4_f32;
            // Java: break;
         }
         // Java: case 8:
         8 => {
            // Java: percent = 0.5F;
            percent = 0.5_f32;
            // Java: break;
         }
         // Java: case 9:
         9 => {
            // Java: percent = 0.6F;
            percent = 0.6_f32;
            // Java: break;
         }
         // Java: case 10:
         10 => {
            // Java: percent = 0.7F;
            percent = 0.7_f32;
            // Java: break;
         }
         // Java: case 11:
         11 => {
            // Java: percent = 0.75F;
            percent = 0.75_f32;
            // Java: break;
         }
         // Java: case 12:
         12 => {
            // Java: percent = 0.8F;
            percent = 0.8_f32;
            // Java: break;
         }
         // Java: case 13:
         13 => {
            // Java: percent = 0.9F;
            percent = 0.9_f32;
         }
         // Java: case 14:
         // Java: case 15:
         // Java: case 16:
         // Java: case 17:
         // Java: case 18:
         // Java: case 19:
         // Java: case 20:
         // Java: case 21:
         // Java: case 22:
         // Java: case 23:
         // Java: case 24:
         // Java: case 25:
         // Java: case 26:
         // Java: case 27:
         // Java: case 28:
         // Java: case 29:
         // Java: case 30:
         // Java: case 31:
         // Java: case 32:
         // Java: case 33:
         // Java: case 34:
         // Java: default:
         14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 => {
            // Java: break;
         }
         // Java: case 35:
         35 => {
            // Java: percent = 0.02F;
            percent = 0.02_f32;
            // Java: break;
         }
         // Java: case 36:
         36 => {
            // Java: percent = 0.07F;
            percent = 0.07_f32;
            // Java: break;
         }
         // Java: case 37:
         37 => {
            // Java: percent = 0.12F;
            percent = 0.12_f32;
            // Java: break;
         }
         // Java: case 38:
         38 => {
            // Java: percent = 0.15F;
            percent = 0.15_f32;
            // Java: break;
         }
         // Java: case 39:
         39 => {
            // Java: percent = 0.17F;
            percent = 0.17_f32;
            // Java: break;
         }
         // Java: case 40:
         40 => {
            // Java: percent = 0.22F;
            percent = 0.22_f32;
            // Java: break;
         }
         // Java: case 41:
         41 => {
            // Java: percent = 0.27F;
            percent = 0.27_f32;
            // Java: break;
         }
         // Java: case 42:
         42 => {
            // Java: percent = 0.32F;
            percent = 0.32_f32;
            // Java: break;
         }
         // Java: case 43:
         43 => {
            // Java: percent = 0.35F;
            percent = 0.35_f32;
            // Java: break;
         }
         // Java: case 44:
         44 => {
            // Java: percent = 0.37F;
            percent = 0.37_f32;
            // Java: break;
         }
         // Java: case 45:
         45 => {
            // Java: percent = 0.42F;
            percent = 0.42_f32;
            // Java: break;
         }
         // Java: case 46:
         46 => {
            // Java: percent = 0.45F;
            percent = 0.45_f32;
            // Java: break;
         }
         // Java: case 47:
         47 => {
            // Java: percent = 0.47F;
            percent = 0.47_f32;
            // Java: break;
         }
         // Java: case 48:
         48 => {
            // Java: percent = 0.52F;
            percent = 0.52_f32;
            // Java: break;
         }
         // Java: case 49:
         49 => {
            // Java: percent = 0.55F;
            percent = 0.55_f32;
            // Java: break;
         }
         // Java: case 50:
         50 => {
            // Java: percent = 0.57F;
            percent = 0.57_f32;
            // Java: break;
         }
         // Java: case 51:
         51 => {
            // Java: percent = 0.62F;
            percent = 0.62_f32;
            // Java: break;
         }
         // Java: case 52:
         52 => {
            // Java: percent = 0.65F;
            percent = 0.65_f32;
            // Java: break;
         }
         // Java: case 53:
         53 => {
            // Java: percent = 0.67F;
            percent = 0.67_f32;
            // Java: break;
         }
         // Java: case 54:
         54 => {
            // Java: percent = 0.72F;
            percent = 0.72_f32;
            // Java: break;
         }
         // Java: case 55:
         55 => {
            // Java: percent = 0.77F;
            percent = 0.77_f32;
            // Java: break;
         }
         // Java: case 56:
         56 => {
            // Java: percent = 0.82F;
            percent = 0.82_f32;
            // Java: break;
         }
         // Java: case 57:
         57 => {
            // Java: percent = 0.85F;
            percent = 0.85_f32;
            // Java: break;
         }
         // Java: case 58:
         58 => {
            // Java: percent = 0.87F;
            percent = 0.87_f32;
            // Java: break;
         }
         // Java: case 59:
         59 => {
            // Java: percent = 0.92F;
            percent = 0.92_f32;
            // Java: break;
         }
         // Java: case 60:
         60 => {
            // Java: percent = 0.95F;
            percent = 0.95_f32;
            // Java: break;
         }
         // Java: case 61:
         // Java: case 62:
         61 | 62 => {
            // Java: percent = 0.97F;
            percent = 0.97_f32;
         }
         _ => {}
      }
      // Java: if (percent == -1.0F)
      if percent == -1.0_f32 {
         // Java: return null;
         return None;
      // Java: else
      } else {
         // Java: int red = (int)(255.0F * (1.0F - percent));
         let red = (255.0_f32 * (1.0_f32 - percent)) as i32;
         // Java: int color = red << 16 | red << 8 | red;
         let color = red << 16 | red << 8 | red;
         // Java: return color;
         return Some(color);
      }
    }

    // Java: static int[] getSpecialColorsByIndex(int index)
    pub fn getSpecialColorsByIndex(index: i32) -> Option<Vec<i32>> {
      // Java: Integer _color = getSpecialColorByIndex(index);
      let _color = Self::getSpecialColorByIndex(index);
      // Java: if (null == _color)
      if _color.is_none() {
         // Java: return null;
         return None;
      // Java: else
      } else {
         // Java: int color = _color;
         let color = _color.unwrap();
         // Java: int[] colors = new int[64];
         let mut colors = vec![0_i32; 64];
         // Java: for(int i = 0; i < 64; ++i)
         for i in 0..64_usize {
            // Java: colors[i] = color;
            colors[i] = color;
         }
         // Java: return colors;
         return Some(colors);
      }
    }

    // Java: static byte[] getColorByPlus(byte[] colors1, byte[] colors2)
    pub fn getColorByPlus(colors1: &[u8], colors2: &[u8]) -> Vec<u8> {
      // Java: byte[] plusColors = new byte[64];
      let mut plusColors = vec![0_u8; 64];
      // Java: for(int i = 0; i < 64; ++i)
      for i in 0..64_usize
      {
         // Java: plusColors[i] = (byte)(colors1[i] + colors2[i]);
         plusColors[i] = colors1[i].wrapping_add(colors2[i]);
      }
      // Java: return plusColors;
      plusColors
    }

    // Java: static byte[] getColorByPlus(byte[] colors1, byte[] colors2, byte[] colors3)
    pub fn getColorByPlus_three(colors1: &[u8], colors2: &[u8], colors3: &[u8]) -> Vec<u8> {
      // Java: byte[] plusColors = new byte[64];
      let mut plusColors = vec![0_u8; 64];
      // Java: for(int i = 0; i < 64; ++i)
      for i in 0..64_usize
      {
         // Java: plusColors[i] = (byte)(colors1[i] + colors2[i] + colors3[i]);
         plusColors[i] = colors1[i].wrapping_add(colors2[i]).wrapping_add(colors3[i]);
      }
      // Java: return plusColors;
      plusColors
    }

}
