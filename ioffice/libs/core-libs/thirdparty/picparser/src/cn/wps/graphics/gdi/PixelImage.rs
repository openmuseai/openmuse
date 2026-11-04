#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import android.graphics.Bitmap
// Java: import android.graphics.PorterDuffXfermode
// Java: import android.graphics.Xfermode
// Java: import android.graphics.Bitmap.Config
// Java: import android.graphics.PorterDuff.Mode

pub struct Bitmap;
impl Bitmap {
    pub fn createBitmap(_colors: &[i32], _width: i32, _height: i32, _config: BitmapConfig) -> Bitmap { Bitmap }
}

#[derive(Clone, Copy, PartialEq)]
pub enum BitmapConfig { ARGB_4444, ARGB_8888, RGB_565 }

#[derive(Clone, Copy, PartialEq)]
pub enum Mode { DARKEN, DST, SRC, LIGHTEN, SRC_OVER }

#[derive(Clone, Copy, PartialEq)]
pub struct Xfermode { mode: Mode }

#[derive(Clone, Copy, PartialEq)]
pub struct PorterDuffXfermode { mode: Mode }
impl PorterDuffXfermode {
    pub fn new(mode: Mode) -> Self { PorterDuffXfermode { mode } }
    pub fn as_xfermode(&self) -> Xfermode { Xfermode { mode: self.mode } }
}

pub struct ColorPaint { color: i32 }
impl ColorPaint {
    pub fn getColor(&self) -> i32 { self.color }
}

// Java field: Xfermode defaultXfermode
static DEFAULT_XFERMODE: std::sync::LazyLock<Xfermode> = std::sync::LazyLock::new(|| PorterDuffXfermode::new(Mode::SRC_OVER).as_xfermode());

// Java: public class PixelImage implements GdiImage
pub struct PixelImage {
    // Java field: int mRop
    pub(crate) mRop: i32,
    // Java field: int width
    pub(crate) width: i32,
    // Java field: int height
    pub(crate) height: i32,
    // Java field: Xfermode xfermode
    pub(crate) xfermode: Xfermode,
    // Java field: Bitmap.Config config
    pub(crate) config: BitmapConfig,
    // Java field: int[] colors
    pub(crate) colors: Vec<i32>,
    // Java field: int transparentColor
    transparentColor: i32,
    // Java field: int HS_HORIZONTAL
    // Java field: int HS_VERTICAL
    // Java field: int HS_FDIAGONAL
    // Java field: int HS_BDIAGONAL
    // Java field: int HS_CROSS
    // Java field: int HS_DIAGCROSS
}

pub(crate) const HS_HORIZONTAL: i32 = 0;
pub(crate) const HS_VERTICAL: i32 = 1;
pub(crate) const HS_FDIAGONAL: i32 = 2;
pub(crate) const HS_BDIAGONAL: i32 = 3;
pub(crate) const HS_CROSS: i32 = 4;
pub(crate) const HS_DIAGCROSS: i32 = 5;

// Java: public class PixelImage implements GdiImage
impl PixelImage {
    // Java: PixelImage(int width, int height, Bitmap.Config config)
    pub fn new(width: i32, height: i32, config: BitmapConfig) -> Self {
        // Java: super();
        // Java: this.width = width;
        // Java: this.height = height;
        // Java: this.config = config;
        PixelImage {
            mRop: 0,
            width,
            height,
            xfermode: *DEFAULT_XFERMODE,
            config,
            colors: Vec::new(),
            transparentColor: 0,
        }
    }

    // Java: void setRGB(int x, int y, int color)
    pub fn setRGB(&mut self, x: i32, y: i32, color: i32) {
        // Java: this.colors[y * this.width + x] = color;
        self.colors[(y * self.width + x) as usize] = color;
    }

    // Java: void initColors()
    pub fn initColors(&mut self) {
        // Java: this.colors = new int[this.width * this.height];
        self.colors = vec![0i32; (self.width * self.height) as usize];
    }

    // Java: void setColors(int[] colors)
    pub fn setColors_vec(&mut self, colors: Vec<i32>) {
        // Java: this.colors = colors;
        self.colors = colors;
    }

    // Java: Bitmap getBitmap()
    pub fn getBitmap_impl(&self) -> Bitmap {
        // Java: return Bitmap.createBitmap(this.colors, this.width, this.height, this.config);
        Bitmap::createBitmap(&self.colors, self.width, self.height, self.config)
    }

    // Java: int getWidth()
    pub fn getWidth_impl(&self) -> i32 {
        // Java: return this.width;
        self.width
    }

    // Java: int getHeight()
    pub fn getHeight_impl(&self) -> i32 {
        // Java: return this.height;
        self.height
    }

    // Java: int[] getColor()
    pub fn getColor(&self) -> &[i32] {
        // Java: return this.colors;
        &self.colors
    }

    // Java: boolean isAlaph()
    pub fn isAlaph(&self) -> bool {
        // Java: return this.config == Config.ARGB_4444 || this.config == Config.ARGB_8888;
        self.config == BitmapConfig::ARGB_4444 || self.config == BitmapConfig::ARGB_8888
    }

    // Java: Xfermode getXferMode()
    pub fn getXferMode_impl(&mut self) -> Xfermode {
        // Java: switch (this.mRop)
        match self.mRop {
            // Java: case 8913094:
            8913094 => {
                // Java: this.xfermode = new PorterDuffXfermode(Mode.DARKEN);
                self.xfermode = PorterDuffXfermode::new(Mode::DARKEN).as_xfermode();
                // Java: break;
            }
            // Java: case 11141161:
            11141161 => {
                // Java: this.xfermode = new PorterDuffXfermode(Mode.DST);
                self.xfermode = PorterDuffXfermode::new(Mode::DST).as_xfermode();
                // Java: break;
            }
            // Java: case 13369376:
            13369376 => {
                // Java: this.xfermode = new PorterDuffXfermode(Mode.SRC);
                self.xfermode = PorterDuffXfermode::new(Mode::SRC).as_xfermode();
                // Java: break;
            }
            // Java: case 15597702:
            15597702 => {
                // Java: this.xfermode = new PorterDuffXfermode(Mode.LIGHTEN);
                self.xfermode = PorterDuffXfermode::new(Mode::LIGHTEN).as_xfermode();
                // Java: break;
            }
            // Java: default:
            _ => {
                // Java: this.xfermode = defaultXfermode;
                self.xfermode = *DEFAULT_XFERMODE;
            }
        }
        // Java: return this.xfermode;
        self.xfermode
    }

    // Java: void setRop(int rop)
    pub fn setRop_impl(&mut self, rop: i32) {
        // Java: this.mRop = rop;
        self.mRop = rop;
    }

    // Java: static GdiImage createHatchedImage(ColorPaint color, int hatch, int bkColor)
    pub fn createHatchedImage(color: &ColorPaint, hatch: i32, bkColor: i32) -> PixelImage {
        // Java: PixelImage pixelImage = new PixelImage(8, 8, Config.ARGB_4444);
        let mut pixelImage = PixelImage::new(8, 8, BitmapConfig::ARGB_4444);
        // Java: int[] pixels = new int[64];
        let mut pixels = vec![0i32; 64];
        // Java: switch (hatch)
        match hatch {
            // Java: case 0:
            0 => {
                // Java: createHorizontal(pixels, color, bkColor);
                PixelImage::createHorizontal(&mut pixels, color, bkColor);
                // Java: break;
            }
            // Java: case 1:
            1 => {
                // Java: createVertical(pixels, color, bkColor);
                PixelImage::createVertical(&mut pixels, color, bkColor);
                // Java: break;
            }
            // Java: case 2:
            2 => {
                // Java: createFdiagonal(pixels, color, bkColor);
                PixelImage::createFdiagonal(&mut pixels, color, bkColor);
                // Java: break;
            }
            // Java: case 3:
            3 => {
                // Java: createBdiagonal(pixels, color, bkColor);
                PixelImage::createBdiagonal(&mut pixels, color, bkColor);
                // Java: break;
            }
            // Java: case 4:
            4 => {
                // Java: createCross(pixels, color, bkColor);
                PixelImage::createCross(&mut pixels, color, bkColor);
                // Java: break;
            }
            // Java: case 5:
            5 => {
                // Java: createDiagcross(pixels, color, bkColor);
                PixelImage::createDiagcross(&mut pixels, color, bkColor);
            }
            _ => {}
        }
        // Java: pixelImage.setColors(pixels);
        pixelImage.setColors_vec(pixels);
        // Java: return pixelImage;
        pixelImage
    }

    // Java: static void createHorizontal(int[] pixels, ColorPaint color, int bkColor)
    fn createHorizontal(pixels: &mut [i32], color: &ColorPaint, bkColor: i32) {
        // Java: for(int i = 0; i < 8; ++i)
        for i in 0..8 {
            // Java: for(int j = 0; j < 8; ++j)
            for j in 0..8 {
                // Java: if (i == 4)
                if i == 4 {
                    // Java: pixels[i * 8 + j] = color.getColor();
                    pixels[i * 8 + j] = color.getColor();
                }
                // Java: else
                else {
                    // Java: pixels[i * 8 + j] = bkColor;
                    pixels[i * 8 + j] = bkColor;
                }
            }
        }
    }

    // Java: static void createVertical(int[] pixels, ColorPaint color, int bkColor)
    fn createVertical(pixels: &mut [i32], color: &ColorPaint, bkColor: i32) {
        // Java: for(int i = 0; i < 8; ++i)
        for i in 0..8 {
            // Java: for(int j = 0; j < 8; ++j)
            for j in 0..8 {
                // Java: if (j == 4)
                if j == 4 {
                    // Java: pixels[i * 8 + j] = color.getColor();
                    pixels[i * 8 + j] = color.getColor();
                }
                // Java: else
                else {
                    // Java: pixels[i * 8 + j] = bkColor;
                    pixels[i * 8 + j] = bkColor;
                }
            }
        }
    }

    // Java: static void createFdiagonal(int[] pixels, ColorPaint color, int bkColor)
    fn createFdiagonal(pixels: &mut [i32], color: &ColorPaint, bkColor: i32) {
        // Java: for(int i = 0; i < 8; ++i)
        for i in 0..8 {
            // Java: for(int j = 0; j < 8; ++j)
            for j in 0..8 {
                // Java: if (i == j)
                if i == j {
                    // Java: pixels[i * 8 + j] = color.getColor();
                    pixels[i * 8 + j] = color.getColor();
                }
                // Java: else
                else {
                    // Java: pixels[i * 8 + j] = bkColor;
                    pixels[i * 8 + j] = bkColor;
                }
            }
        }
    }

    // Java: static void createBdiagonal(int[] pixels, ColorPaint color, int bkColor)
    fn createBdiagonal(pixels: &mut [i32], color: &ColorPaint, bkColor: i32) {
        // Java: for(int i = 0; i < 8; ++i)
        for i in 0..8 {
            // Java: for(int j = 0; j < 8; ++j)
            for j in 0..8 {
                // Java: if (i + j == 7)
                if i + j == 7 {
                    // Java: pixels[i * 8 + j] = color.getColor();
                    pixels[i * 8 + j] = color.getColor();
                }
                // Java: else
                else {
                    // Java: pixels[i * 8 + j] = bkColor;
                    pixels[i * 8 + j] = bkColor;
                }
            }
        }
    }

    // Java: static void createCross(int[] pixels, ColorPaint color, int bkColor)
    fn createCross(pixels: &mut [i32], color: &ColorPaint, bkColor: i32) {
        // Java: for(int i = 0; i < 8; ++i)
        for i in 0..8 {
            // Java: for(int j = 0; j < 8; ++j)
            for j in 0..8 {
                // Java: if (i != 4 && j != 4)
                if i != 4 && j != 4 {
                    // Java: pixels[i * 8 + j] = bkColor;
                    pixels[i * 8 + j] = bkColor;
                }
                // Java: else
                else {
                    // Java: pixels[i * 8 + j] = color.getColor();
                    pixels[i * 8 + j] = color.getColor();
                }
            }
        }
    }

    // Java: static void createDiagcross(int[] pixels, ColorPaint color, int bkColor)
    fn createDiagcross(pixels: &mut [i32], color: &ColorPaint, bkColor: i32) {
        // Java: for(int i = 0; i < 8; ++i)
        for i in 0..8 {
            // Java: for(int j = 0; j < 8; ++j)
            for j in 0..8 {
                // Java: if (i != j && i + j != 7)
                if i != j && i + j != 7 {
                    // Java: pixels[i * 8 + j] = bkColor;
                    pixels[i * 8 + j] = bkColor;
                }
                // Java: else
                else {
                    // Java: pixels[i * 8 + j] = color.getColor();
                    pixels[i * 8 + j] = color.getColor();
                }
            }
        }
    }

    // Java: void transparent(int bkColor)
    pub fn transparent_impl(&mut self, bkColor: i32) {
        // Java: if (bkColor != this.transparentColor)
        if bkColor != self.transparentColor {
            // Java: this.transparentColor = bkColor;
            self.transparentColor = bkColor;
            // Java: int len = this.colors.length;
            let len = self.colors.len();
            // Java: for(int i = 0; i < len; ++i)
            for i in 0..len {
                // Java: if (this.colors[i] == bkColor)
                if self.colors[i] == bkColor {
                    // Java: this.colors[i] = 0;
                    self.colors[i] = 0;
                }
            }
        }
    }

    // Java: void concat(PixelImage lastPiexlImage)
    pub fn concat(&mut self, lastPiexlImage: &PixelImage) {
        // Java: this.xfermode = defaultXfermode;
        self.xfermode = *DEFAULT_XFERMODE;
        // Java: switch (this.mRop)
        match self.mRop {
            // Java: case 6684742:
            6684742 => {
                // Java: for(int i = 0; i < this.height; ++i)
                for i in 0..self.height as usize {
                    // Java: for(int j = 0; j < this.width; ++j)
                    for j in 0..self.width as usize {
                        // Java: int index = i * this.width + j;
                        let index = i * self.width as usize + j;
                        // Java: this.colors[index] ^= lastPiexlImage.colors[index];
                        self.colors[index] ^= lastPiexlImage.colors[index];
                        // Java: if (this.colors[index] != -16777216 && this.colors[index] != -1)
                        if self.colors[index] != -16777216 && self.colors[index] != -1 {
                            // Java: this.colors[index] |= -16777216;
                            self.colors[index] |= -16777216i32;
                        }
                    }
                }
                // Java: break;
            }
            // Java: case 8913094:
            8913094 => {
                // Java: for(int i = 0; i < this.height; ++i)
                for i in 0..self.height as usize {
                    // Java: for(int j = 0; j < this.width; ++j)
                    for j in 0..self.width as usize {
                        // Java: int index = i * this.width + j;
                        let index = i * self.width as usize + j;
                        // Java: this.colors[index] &= lastPiexlImage.colors[index];
                        self.colors[index] &= lastPiexlImage.colors[index];
                        // Java: if (this.colors[index] != -16777216 && this.colors[index] != -1)
                        if self.colors[index] != -16777216 && self.colors[index] != -1 {
                            // Java: this.colors[index] |= -16777216;
                            self.colors[index] |= -16777216i32;
                        }
                    }
                }
                // Java: break;
            }
            // Java: case 15597702:
            15597702 => {
                // Java: for(int i = 0; i < this.height; ++i)
                for i in 0..self.height as usize {
                    // Java: for(int j = 0; j < this.width; ++j)
                    for j in 0..self.width as usize {
                        // Java: int index = i * this.width + j;
                        let index = i * self.width as usize + j;
                        // Java: this.colors[index] |= lastPiexlImage.colors[index];
                        self.colors[index] |= lastPiexlImage.colors[index];
                        // Java: if (this.colors[index] != -16777216 && this.colors[index] != -1)
                        if self.colors[index] != -16777216 && self.colors[index] != -1 {
                            // Java: this.colors[index] |= -16777216;
                            self.colors[index] |= -16777216i32;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Java: int getRop()
    pub fn getRop_impl(&self) -> i32 {
        // Java: return this.mRop;
        self.mRop
    }
}

use crate::cn::wps::graphics::gdi::GdiImage::GdiImage;
use crate::cn::wps::graphics::gdi::GdiImage::Bitmap as GdiBitmap;
use crate::cn::wps::graphics::gdi::GdiImage::Xfermode as GdiXfermode;

impl GdiImage for PixelImage {
    fn setColors(&mut self, var1: &[i32]) {
        self.colors = var1.to_vec();
    }
    fn getBitmap(&self) -> GdiBitmap { GdiBitmap }
    fn getWidth(&self) -> i32 { self.getWidth_impl() }
    fn getHeight(&self) -> i32 { self.getHeight_impl() }
    fn getXferMode(&self) -> GdiXfermode { GdiXfermode }
    fn setRop(&mut self, var1: i32) { self.setRop_impl(var1); }
    fn getRop(&self) -> i32 { self.getRop_impl() }
    fn transparent(&mut self, var1: i32) { self.transparent_impl(var1); }
}
