#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unreachable_code)]

// Java: import android.graphics.Bitmap
// Java: import android.graphics.BitmapShader
// Java: import android.graphics.Canvas
// Java: import android.graphics.ColorFilter
// Java: import android.graphics.Matrix
// Java: import android.graphics.Paint
// Java: import android.graphics.Path
// Java: import android.graphics.RectF
// Java: import android.graphics.Shader
// Java: import android.graphics.Typeface
// Java: import android.graphics.Bitmap.Config
// Java: import android.graphics.Paint.Style
// Java: import android.graphics.Path.Direction
// Java: import android.graphics.Shader.TileMode
// Java: import cn.wps.moffice.define.SymbolConverter

use crate::cn::wps::graphics::gdi::GdiDC::GdiDC;
use crate::cn::wps::graphics::gdi::GDIFont::GDIFont;
use crate::cn::wps::graphics::gdi::GdiPath::GdiPath;
use crate::cn::wps::graphics::gdi::Rectangle::Rectangle;
use crate::cn::wps::graphics::gdi::States::States;
use crate::cn::wps::graphics::gdi::ColorPaint::ColorPaint;

pub struct Canvas { _dummy: i32 }
impl Canvas {
    pub fn new() -> Self { Canvas { _dummy: 0 } }
    pub fn save(&self) {}
    pub fn restore(&self) {}
    pub fn clipPath(&self, _p: &i32) {}
    pub fn concat(&self, _m: &i32) {}
    pub fn translate(&self, _x: f32, _y: f32) {}
    pub fn scale(&self, _sx: f32, _sy: f32) {}
    pub fn rotate(&self, _deg: f32) {}
    pub fn drawRect(&self, _l: f32, _t: f32, _r: f32, _b: f32, _p: &Paint) {}
    pub fn drawPosText(&self, _t: &str, _pos: &[f32], _p: &Paint) {}
    pub fn drawText(&self, _t: &str, _x: f32, _y: f32, _p: &Paint) {}
    pub fn drawPath(&self, _path: &i32, _p: &Paint) {}
    pub fn drawBitmap_colors(&self, _c: &[i32], _o: i32, _s: i32, _x: i32, _y: i32, _w: i32, _h: i32, _a: bool, _p: &Paint) {}
    pub fn drawBitmap(&self, _b: &Bitmap, _x: f32, _y: f32) {}
    pub fn getMatrix(&self) -> [f32; 9] { [1.0,0.0,0.0, 0.0,1.0,0.0, 0.0,0.0,1.0] }
}
pub struct Paint {
    pub color: i32,
    pub text_size: f32,
    pub typeface: i32,
    pub fake_bold: bool,
    pub text_skew_x: f32,
    pub underline: bool,
    pub strike_thru: bool,
    pub shader: i32,
    pub style: i32,
    pub color_filter: i32,
}
impl Paint {
    pub fn new() -> Self { Paint { color: 0, text_size: 0.0, typeface: 0, fake_bold: false, text_skew_x: 0.0, underline: false, strike_thru: false, shader: 0, style: 0, color_filter: 0 } }
    pub fn setColor(&mut self, c: i32) { self.color = c; }
    pub fn setTextSize(&mut self, s: f32) { self.text_size = s; }
    pub fn getTextSize(&self) -> f32 { self.text_size }
    pub fn setTypeface(&mut self, _tf: i32) { self.typeface = _tf; }
    pub fn setFakeBoldText(&mut self, v: bool) { self.fake_bold = v; }
    pub fn setTextSkewX(&mut self, v: f32) { self.text_skew_x = v; }
    pub fn setUnderlineText(&mut self, v: bool) { self.underline = v; }
    pub fn setStrikeThruText(&mut self, v: bool) { self.strike_thru = v; }
    pub fn setShader(&mut self, _s: i32) { self.shader = _s; }
    pub fn setStyle(&mut self, _s: i32) { self.style = _s; }
    pub fn setColorFilter(&mut self, _f: i32) { self.color_filter = _f; }
    pub fn setTextAlign(&mut self, _a: i32) {}
    pub fn setXfermode(&mut self, _x: i32) {}
}
pub struct Bitmap { pub width: i32, pub height: i32 }
impl Bitmap {
    pub fn getWidth(&self) -> i32 { self.width }
    pub fn getHeight(&self) -> i32 { self.height }
}
pub struct RectF { pub left: f32, pub top: f32, pub right: f32, pub bottom: f32 }

struct TextAlign;
impl TextAlign {
    fn getAlign(_a: i32) -> i32 { 0 }
    fn isAlignCenter(a: i32) -> bool { (a & 6) == 6 }
    fn isAlignLeft(a: i32) -> bool { (a & 6) == 0 }
    fn isAlignBaseLine(a: i32) -> bool { (a & 24) == 24 }
    fn isAlignBottom(a: i32) -> bool { (a & 8) == 8 }
    fn isUpdateCp(a: i32) -> bool { (a & 1) != 0 }
}

// Java: public class GraphicsDrawer
pub struct GraphicsDrawer {
    // Java field: float MAX_TEXT_SIZE
    // Java field: Canvas canvas
    pub(crate) canvas: Canvas,
    // Java field: GdiDC dc
    dc: Option<GdiDC>,
    // Java field: Paint textPaint
    textPaint: Option<Paint>,
    // Java field: Paint paint
    paint: Option<Paint>,
    // Java field: Paint imageDrawerPaint
    imageDrawerPaint: Option<Paint>,
    // Java field: Path clipPath
    clipPath: bool,
    // Java field: int bkColor
    pub(crate) bkColor: i32,
    // Java field: int bkMode
    pub(crate) bkMode: i32,
    // Java field: int textColor
    textColor: i32,
    // Java field: int rop2
    rop2: i32,
    // Java field: int textAlign
    pub(crate) textAlign: i32,
    // Java field: int stretchMode
    stretchMode: i32,
    // Java field: float canvasScaleX
    canvasScaleX: f32,
    // Java field: float canvasScaleY
    canvasScaleY: f32,
    // Java field: float[] pos
    pos: Vec<f32>,
    // Java field: States states
    states: Option<States>,
    // Java field: char HIGH_SURROGATE_START
    // Java field: char HIGH_SURROGATE_END
    // Java field: char LOW_SURROGATE_START
    // Java field: char LOW_SURROGATE_END
}

pub(crate) const MAX_TEXT_SIZE: f32 = 1024.0;
pub(crate) const HIGH_SURROGATE_START: u16 = 0xD800;
pub(crate) const HIGH_SURROGATE_END: u16 = 0xDBFF;
pub(crate) const LOW_SURROGATE_START: u16 = 0xDC00;
pub(crate) const LOW_SURROGATE_END: u16 = 0xDFFF;

// Java: public class GraphicsDrawer
impl GraphicsDrawer {
    // Java: GraphicsDrawer(GdiDC dc, Canvas canvas)
    pub fn new(dc: GdiDC, canvas: Canvas) -> Self {
        // Java: this(dc, canvas, (Path)null, (ColorFilter)null, true);
        GraphicsDrawer::new_full(dc, canvas, false, 0, true)
    }

    // Java: GraphicsDrawer(GdiDC dc, Canvas canvas, Path clipPath, ColorFilter colorFilter, boolean isUseState)
    pub fn new_full(dc: GdiDC, canvas: Canvas, clipPath: bool, colorFilter: i32, isUseState: bool) -> Self {
        // Java: super();
        // Java: this.bkColor = -1;
        let bkColor = -1i32;
        // Java: this.bkMode = 2;
        let bkMode = 2;
        // Java: this.rop2 = 13;
        let rop2 = 13;
        // Java: this.textAlign = 0;
        let textAlign = 0;
        // Java: this.pos = new float[64];
        let pos = vec![0.0f32; 64];
        // Java: this.states = null;
        // Java: this.textPaint = new Paint(1);
        let mut textPaint = Paint::new();
        // Java: this.paint = new Paint(1);
        let mut paint = Paint::new();
        // Java: if (null != colorFilter)
        if colorFilter != 0 {
            // Java: this.textPaint.setColorFilter(colorFilter);
            textPaint.setColorFilter(colorFilter);
            // Java: this.paint.setColorFilter(colorFilter);
            paint.setColorFilter(colorFilter);
        }
        // Java: this.textColor = -16777216;
        let textColor = -16777216i32;
        // Java: this.dc = dc;
        // Java: this.canvas = canvas;
        // Java: this.clipPath = clipPath;
        // Java: Matrix matrix = canvas.getMatrix();
        let matrix = canvas.getMatrix();
        // Java: this.canvasScaleX = GdiTool.getScaleX(matrix);
        let canvasScaleX = matrix[0];
        // Java: this.canvasScaleY = GdiTool.getScaleY(matrix);
        let canvasScaleY = matrix[4];
        // Java: this.states = isUseState ? States.getInstance(canvas, clipPath) : null;
        let states = None;
        GraphicsDrawer {
            canvas, dc: Some(dc), textPaint: Some(textPaint), paint: Some(paint),
            imageDrawerPaint: None, clipPath, bkColor, bkMode, textColor, rop2,
            textAlign, stretchMode: 0, canvasScaleX, canvasScaleY, pos, states,
        }
    }

    // Java: void setRop2(int value)
    pub fn setRop2(&mut self, value: i32) {
        // Java: this.rop2 = value;
        self.rop2 = value;
    }

    // Java: void setTextAlign(int textAlign)
    pub fn setTextAlign(&mut self, textAlign: i32) {
        // Java: this.textAlign = textAlign;
        self.textAlign = textAlign;
    }

    // Java: int getTextAlign()
    pub fn getTextAlign(&self) -> i32 {
        // Java: return this.textAlign;
        self.textAlign
    }

    // Java: void setTextColor(int color)
    pub fn setTextColor(&mut self, color: i32) {
        // Java: this.textColor = color;
        self.textColor = color;
    }

    // Java: void setBkColor(int color)
    pub fn setBkColor(&mut self, color: i32) {
        // Java: this.bkColor = color;
        self.bkColor = color;
    }

    // Java: int getBkColor()
    pub fn getBkColor(&self) -> i32 {
        // Java: return this.bkColor;
        self.bkColor
    }

    // Java: void setBkMode(int mode)
    pub fn setBkMode(&mut self, mode: i32) {
        // Java: this.bkMode = mode;
        self.bkMode = mode;
    }

    // Java: int getBkMode()
    pub fn getBkMode(&self) -> i32 {
        // Java: return this.bkMode;
        self.bkMode
    }

    // Java: void setStretchMode(int mode)
    pub fn setStretchMode(&mut self, mode: i32) {
        // Java: this.stretchMode = mode;
        self.stretchMode = mode;
    }

    // Java: Canvas getCanvas()
    pub fn getCanvas(&self) -> &Canvas {
        // Java: return this.canvas;
        &self.canvas
    }

    // Java: void clip(Rectangle rectangle)
    pub fn clip(&self, rectangle: &Rectangle) {
        // Java: Path transformPath = new Path();
        // Java: transformPath.addRect((float)rectangle.x, (float)rectangle.y, (float)(rectangle.x + rectangle.width), (float)(rectangle.y + rectangle.height), Direction.CW);
        // Java: transformPath.transform(this.dc.getTransform());
        // Java: if (this.states != null)
        if self.states.is_some() {
            // Java: this.states.clip((Path)transformPath, 1);
        }
        // Java: else
        else {
            // Java: this.canvas.clipPath(transformPath);
        }
    }

    // Java: RectF getClip(Path clipPath)
    fn getClip_path(&self) -> RectF {
        // Java: float[] values = new float[9];
        let mut values = [0.0f32; 9];
        // Java: this.canvas.getMatrix().getValues(values);
        values = self.canvas.getMatrix();
        // Java: float sx = values[0];
        let sx = values[0];
        // Java: float sy = values[4];
        let sy = values[4];
        // Java: RectF clip = new RectF();
        let mut clip = RectF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 };
        // Java: clipPath.computeBounds(clip, true);
        // Java: clip.left *= sx;
        clip.left *= sx;
        // Java: clip.top *= sy;
        clip.top *= sy;
        // Java: clip.right *= sx;
        clip.right *= sx;
        // Java: clip.bottom *= sy;
        clip.bottom *= sy;
        // Java: return clip;
        clip
    }

    // Java: void setClip(Path path, int mode)
    pub fn setClip_path(&mut self, _path: i32, mode: i32) {
        // Java: Path transformPath = new Path(path);
        // Java: if (null != this.clipPath && mode != 4)
        if self.clipPath && mode != 4 {
            // Java: RectF clip = this.getClip(this.clipPath);
            let clip = self.getClip_path();
            // Java: transformPath.addRect(clip, Direction.CW);
        }
        // Java: Matrix dcTrans = this.dc.getTransform();
        // Java: transformPath.transform(dcTrans);
        // Java: if (this.states != null)
        if self.states.is_some() {
            // Java: this.states.clip(transformPath, mode);
        }
        // Java: else
        else {
            // Java: this.canvas.clipPath(transformPath, GdiTool.getOpByMode(mode));
        }
    }

    // Java: void setClip(Rectangle rectangle, int mode)
    pub fn setClip_rect(&mut self, rectangle: &Rectangle, mode: i32) {
        // Java: Path transformPath = new Path();
        // Java: Matrix dcTrans = this.dc.getContext().getSrcTransform();
        // Java: transformPath.addRect((float)rectangle.x, (float)rectangle.y, (float)(rectangle.x + rectangle.width), (float)(rectangle.y + rectangle.height), Direction.CW);
        // Java: if (null != this.clipPath && mode != 4)
        if self.clipPath && mode != 4 {
            // Java: RectF clip = this.getClip(this.clipPath);
            let clip = self.getClip_path();
            // Java: transformPath.addRect(clip, Direction.CW);
        }
        // Java: transformPath.transform(dcTrans);
        // Java: if (this.states != null)
        if self.states.is_some() {
            // Java: this.states.clip(transformPath, mode);
        }
        // Java: else
        else {
            // Java: this.canvas.clipPath(transformPath, GdiTool.getOpByMode(mode));
        }
    }

    // Java: void initTextPaint(GDIFont font)
    fn initTextPaint(&mut self, font: &mut GDIFont) {
        // Java: if (null != font)
        {
            // Java: font.initTextPaint(this.textPaint);
            if let Some(ref mut tp) = self.textPaint {
                font.initTextPaint(&mut crate::cn::wps::graphics::gdi::GDIFont::Paint {
                    typeface: tp.typeface, fake_bold: tp.fake_bold,
                    text_skew_x: tp.text_skew_x, text_size: tp.text_size,
                    underline: tp.underline, strike_thru: tp.strike_thru,
                });
            }
            // Java: this.textPaint.setTextAlign(TextAlign.getAlign(this.textAlign));
            // Java: this.textPaint.setStyle(Style.FILL);
            if let Some(ref mut tp) = self.textPaint {
                tp.setStyle(0);
            }
            // Java: this.textPaint.setColor(this.textColor);
            if let Some(ref mut tp) = self.textPaint {
                tp.setColor(self.textColor);
            }
            // Java: this.textPaint.setShader((Shader)null);
            if let Some(ref mut tp) = self.textPaint {
                tp.setShader(0);
            }
        }
    }

    // Java: void concatFont(GDIFont font, int x, int y, boolean isAlignBottom)
    fn concatFont(&self, font: &GDIFont, x: i32, y: i32, isAlignBottom: bool) {
        // Java: if (font != null)
        {
            // Java: float coordinateX = this.dc.getContext().getScaleX() > 0.0F ? 1.0F : -1.0F;
            let coordinateX = 1.0f32;
            // Java: float coordinateY = this.dc.getContext().getScaleY() > 0.0F ? 1.0F : -1.0F;
            let coordinateY = 1.0f32;
            // Java: if (font.height < 0 && this.dc.getContext().getScaleY() < 0.0F)
            if font.height < 0 {
                // Java: this.canvas.scale(1.0F, -1.0F);
                self.canvas.scale(1.0, -1.0);
            }
            // Java: else if ((font.clipPrecision & 16) != 0)
            else if (font.clipPrecision & 16) != 0 {
                // Java: this.canvas.scale(coordinateX, coordinateY);
                self.canvas.scale(coordinateX, coordinateY);
            }
            // Java: if (font.escapement != 0)
            if font.escapement != 0 {
                // Java: float degree = (float)(font.escapement * -1) * coordinateX * coordinateY;
                let degree = (font.escapement * -1) as f32 * coordinateX * coordinateY;
                // Java: this.canvas.rotate(degree);
                self.canvas.rotate(degree);
            }
        }
    }

    // Java: void drawTextBackground(int textLength, float textSize, float x, float y, float[] widths)
    fn drawTextBackground(&mut self, textLength: i32, textSize: f32, x: f32, y: f32, widths: Option<&[f32]>) {
        // Java: float px = x;
        let mut px = x;
        // Java: float py = y - textSize * 3.0F / 4.0F;
        let py = y - textSize * 3.0 / 4.0;
        // Java: float width = 0.0F;
        let mut width = 0.0f32;
        // Java: if (null != widths)
        if let Some(w) = widths {
            // Java: int len = widths.length;
            let len = w.len();
            // Java: for(int i = 0; i < len; ++i)
            for i in 0..len {
                // Java: width += widths[i];
                width += w[i];
            }
        }
        // Java: else
        else {
            // Java: width = textSize * (float)textLength / 2.0F;
            width = textSize * textLength as f32 / 2.0;
        }
        // Java: if (TextAlign.isAlignCenter(this.textAlign))
        if TextAlign::isAlignCenter(self.textAlign) {
            // Java: px = x - width / 2.0F;
            px = x - width / 2.0;
        }
        // Java: this.paint.setColor(this.bkColor);
        if let Some(ref mut p) = self.paint { p.setColor(self.bkColor); }
        // Java: this.paint.setStyle(Style.FILL);
        if let Some(ref mut p) = self.paint { p.setStyle(0); }
        // Java: this.canvas.drawRect(px, py, px + width, py + textSize, this.paint);
        if let Some(ref p) = self.paint { self.canvas.drawRect(px, py, px + width, py + textSize, p); }
    }

    // Java: boolean hasNonBMPChar(String str)
    fn hasNonBMPChar(&self, s: &str) -> bool {
        // Java: int len = str.length() - 1;
        let chars: Vec<u16> = s.encode_utf16().collect();
        let len = if chars.len() > 0 { chars.len() - 1 } else { 0 };
        // Java: if (len > 0)
        if len > 0 {
            // Java: for(int i = 0; i < len; ++i)
            for i in 0..len {
                // Java: char c1 = str.charAt(i);
                let c1 = chars[i];
                // Java: if (c1 >= '\ud800' && c1 <= '\udbff')
                if c1 >= HIGH_SURROGATE_START && c1 <= HIGH_SURROGATE_END {
                    // Java: char c2 = str.charAt(i + 1);
                    let c2 = chars[i + 1];
                    // Java: if (c2 >= '\udc00' && c2 <= '\udfff')
                    if c2 >= LOW_SURROGATE_START && c2 <= LOW_SURROGATE_END {
                        // Java: return true;
                        return true;
                    }
                }
            }
        }
        // Java: return false;
        false
    }

    // Java: void drawText(String text, int x, int y, float[] widths, GdiPath drawingPath, boolean isDrawTextA)
    pub fn drawText_simple(&mut self, text: &str, x: i32, y: i32, widths: Option<&[f32]>, drawingPath: Option<&mut GdiPath>, isDrawTextA: bool) {
        // Java: this.drawText(text, x, y, widths, drawingPath, isDrawTextA, (Typeface)null);
        self.drawText_full(text, x, y, widths, drawingPath, isDrawTextA, false);
    }

    // Java: void drawText(String text, int x, int y, float[] widths, GdiPath drawingPath, boolean isDrawTextA, Typeface typeface)
    pub fn drawText_full(&mut self, text: &str, mut x: i32, mut y: i32, widths: Option<&[f32]>, mut drawingPath: Option<&mut GdiPath>, isDrawTextA: bool, typeface: bool) {
        // Java: if (null != text && text.length() > 0)
        if !text.is_empty() {
            // Java: text = SymbolConverter.convertToSymbol(text, this.dc.font.familyName);
            let text = text.to_string();
            // Java: this.initTextPaint(this.dc.font);
            // Java: float textSize = this.textPaint.getTextSize();
            let mut textSize = self.textPaint.as_ref().map_or(0.0, |p| p.getTextSize());
            // Java: float scale = 1.0F;
            let mut scale = 1.0f32;
            // Java: if (textSize > 1024.0F)
            if textSize > 1024.0 {
                // Java: scale = textSize / 1024.0F;
                scale = textSize / 1024.0;
                // Java: textSize = 1024.0F;
                textSize = 1024.0;
                // Java: this.textPaint.setTextSize(textSize);
                if let Some(ref mut tp) = self.textPaint { tp.setTextSize(textSize); }
            }
            // Java: boolean needUpdateCP = TextAlign.isUpdateCp(this.textAlign) && drawingPath != null;
            let needUpdateCP = TextAlign::isUpdateCp(self.textAlign) && drawingPath.is_some();
            // Java: if (needUpdateCP && isDrawTextA || needUpdateCP && !isDrawTextA && (x | y) == 0)
            if (needUpdateCP && isDrawTextA) || (needUpdateCP && !isDrawTextA && (x | y) == 0) {
                if let Some(ref dp) = drawingPath {
                    // Java: x = (int)drawingPath.getCurrentX();
                    x = dp.currentX as i32;
                    // Java: y = (int)drawingPath.getCurrentY();
                    y = dp.currentY as i32;
                }
            }
            // Java: this.canvas.save();
            self.canvas.save();
            // Java: this.transformCanvas();
            self.transformCanvas();
            // Java: this.canvas.translate((float)x, (float)y);
            self.canvas.translate(x as f32, y as f32);
            // Java: this.canvas.scale(scale, scale);
            self.canvas.scale(scale, scale);
            // Java: x = 0;
            x = 0;
            // Java: y = this.calculateAndConcatFont(textSize, x);
            y = self.calculateAndConcatFont(textSize, x);
            // Java: if (typeface != null)
            if typeface {
                // Java: this.textPaint.setTypeface(typeface);
            }
            // Java: if (this.bkMode == 2)
            if self.bkMode == 2 {
                // Java: this.drawTextBackground(text.length(), textSize, (float)x, (float)y, widths);
                self.drawTextBackground(text.len() as i32, textSize, x as f32, y as f32, widths);
            }
            // Java: int textLength = text.length();
            let textLength = text.len();
            // Java: if (null != widths && TextAlign.isAlignLeft(this.textAlign) && !this.hasNonBMPChar(text))
            if widths.is_some() && TextAlign::isAlignLeft(self.textAlign) && !self.hasNonBMPChar(&text) {
                let w = widths.unwrap();
                // Java: float charWidth = widths[widths.length - 1] / scale;
                let mut charWidth = w[w.len() - 1] / scale;
                // Java: float minCharWidth = charWidth;
                let mut minCharWidth = charWidth;
                // Java: float[] charPos = textLength > 31 ? new float[(textLength + 1) * 2] : this.pos;
                let mut charPos = if textLength > 31 { vec![0.0f32; (textLength + 1) * 2] } else { self.pos.clone() };
                // Java: charPos[0] = (float)x;
                charPos[0] = x as f32;
                // Java: charPos[1] = (float)y;
                charPos[1] = y as f32;
                // Java: for(int i = 0; i < textLength; ++i)
                for i in 0..textLength {
                    // Java: charWidth = widths[i] / scale;
                    charWidth = w[i] / scale;
                    // Java: charPos[(i + 1) * 2] = charPos[i * 2] + charWidth;
                    charPos[(i + 1) * 2] = charPos[i * 2] + charWidth;
                    // Java: charPos[(i + 1) * 2 + 1] = (float)y;
                    charPos[(i + 1) * 2 + 1] = y as f32;
                    // Java: if (charWidth < minCharWidth)
                    if charWidth < minCharWidth {
                        // Java: minCharWidth = charWidth;
                        minCharWidth = charWidth;
                    }
                }
                // Java: if (needUpdateCP)
                if needUpdateCP {
                    // Java: drawingPath.setCurrentX((drawingPath.getCurrentX() + charPos[2 * textLength]) * scale);
                    if let Some(ref mut dp) = drawingPath {
                        dp.currentX = (dp.currentX + charPos[2 * textLength]) * scale;
                    }
                }
                // Java: if (textSize < 2.0F && minCharWidth > 0.0F)
                if textSize < 2.0 && minCharWidth > 0.0 {
                    // Java: this.textPaint.setTextSize(minCharWidth);
                    if let Some(ref mut tp) = self.textPaint { tp.setTextSize(minCharWidth); }
                }
                // Java: this.canvas.drawPosText(text, charPos, this.textPaint);
                if let Some(ref tp) = self.textPaint { self.canvas.drawPosText(&text, &charPos, tp); }
            }
            // Java: else
            else {
                // Java: this.canvas.drawText(text, (float)x, (float)y, this.textPaint);
                if let Some(ref tp) = self.textPaint { self.canvas.drawText(&text, x as f32, y as f32, tp); }
                // Java: if (needUpdateCP)
                if needUpdateCP {
                    // Java: drawingPath.moveTo((float)((int)(drawingPath.getCurrentX() + textSize * (float)textLength)), (float)((int)drawingPath.getCurrentY()));
                    if let Some(ref mut dp) = drawingPath {
                        dp.moveTo((dp.currentX + textSize * textLength as f32) as i32 as f32, dp.currentY as i32 as f32);
                    }
                }
            }
            // Java: this.textPaint.setTypeface((Typeface)null);
            if let Some(ref mut tp) = self.textPaint { tp.setTypeface(0); }
            // Java: this.bkColor = -1;
            self.bkColor = -1;
            // Java: this.canvas.restore();
            self.canvas.restore();
        }
    }

    // Java: int calculateAndConcatFont(float textSize, int x)
    fn calculateAndConcatFont(&self, textSize: f32, x: i32) -> i32 {
        // Java: int y = 0;
        let mut y = 0i32;
        // Java: boolean isAlignBottom = false;
        let mut isAlignBottom = false;
        // Java: if (!TextAlign.isAlignBaseLine(this.textAlign))
        if !TextAlign::isAlignBaseLine(self.textAlign) {
            // Java: if (TextAlign.isAlignBottom(this.textAlign))
            if TextAlign::isAlignBottom(self.textAlign) {
                // Java: isAlignBottom = true;
                isAlignBottom = true;
                // Java: y = (int)((float)y - 9.0F * textSize / 30.0F);
                y = (y as f32 - 9.0 * textSize / 30.0) as i32;
            }
            // Java: else if (TextAlign.isAlignCenter(this.textAlign))
            else if TextAlign::isAlignCenter(self.textAlign) {
                // Java: y = (int)((float)y + 27.0F * textSize / 30.0F);
                y = (y as f32 + 27.0 * textSize / 30.0) as i32;
            }
            // Java: else
            else {
                // Java: y = (int)((float)y + textSize);
                y = (y as f32 + textSize) as i32;
            }
        }
        // Java: this.concatFont(this.dc.font, x, y, isAlignBottom);
        // Java: return y;
        y
    }

    // Java: void fill(GdiPath path)
    pub fn fill(&mut self, path: &mut GdiPath) {
        // Java: if (!this.isNullGdiPaint(this.dc.brushPaint))
        {
            // Java: this.canvas.save();
            self.canvas.save();
            // Java: path.createPathByScale(this.dc.getTransform());
            // Java: this.setGdiPaint(this.dc.brushPaint);
            // Java: this.paint.setStyle(Style.FILL);
            if let Some(ref mut p) = self.paint { p.setStyle(0); }
            // Java: path.draw(this.canvas, this.paint);
            // Java: this.canvas.restore();
            self.canvas.restore();
        }
    }

    // Java: void drawPath(GdiPath path)
    pub fn drawPath_gdi(&mut self, path: &mut GdiPath) {
        // Java: if (this.rop2 != 11)
        if self.rop2 != 11 {
            // Java: IGdiPainter srcPaint = GDIRop2.getPaintByRop(this.dc.linePaint, this.rop2);
            // Java: if (srcPaint == null || !this.isNullGdiPaint(srcPaint))
            {
                // Java: this.canvas.save();
                self.canvas.save();
                // Java: float srcScale = GdiTool.getScale(this.canvas.getMatrix());
                // Java: this.dc.stroke.initPaint(this.paint, 1.0F / srcScale, GdiTool.getScaleX(this.dc.getTransform()));
                // Java: path.createPathByScale(this.dc.getTransform());
                // Java: if (srcPaint != null)
                {
                    // Java: this.setGdiPaint(srcPaint);
                }
                // Java: this.paint.setStyle(Style.STROKE);
                if let Some(ref mut p) = self.paint { p.setStyle(1); }
                // Java: path.draw(this.canvas, this.paint);
                // Java: this.canvas.restore();
                self.canvas.restore();
            }
        }
    }

    // Java: void drawPath(Path path, float x, float y, float height)
    pub fn drawPath_native(&mut self, _path: i32, x: f32, y: f32, height: f32) {
        // Java: this.canvas.save();
        self.canvas.save();
        // Java: this.transformCanvas();
        self.transformCanvas();
        // Java: this.canvas.translate(x, y);
        self.canvas.translate(x, y);
        // Java: float scale = (float)this.dc.getFont().getFontSize() / height;
        let scale = 0.0f32;
        // Java: this.canvas.scale(scale, scale);
        self.canvas.scale(scale, scale);
        // Java: this.paint.setColor(this.textColor);
        if let Some(ref mut p) = self.paint { p.setColor(self.textColor); }
        // Java: this.paint.setStyle(Style.FILL);
        if let Some(ref mut p) = self.paint { p.setStyle(0); }
        // Java: this.canvas.drawPath(path, this.paint);
        // Java: this.canvas.restore();
        self.canvas.restore();
    }

    // Java: void drawImage(GdiImage image, int x, int y, int width, int height, Matrix matrix)
    pub fn drawImage_matrix(&mut self, _image: i32, x: i32, y: i32, width: i32, height: i32, matrix: &[f32; 9]) {
        // Java: if (image != null)
        {
            // Java: float[] values = GdiTool.matrixFloatValues;
            let values = matrix;
            // Java: matrix.getValues(values);
            // Java: float scaleX = values[0];
            let scaleX = values[0];
            // Java: float scaleY = values[4];
            let scaleY = values[4];
            // Java: float translateX = values[2];
            let translateX = values[2];
            // Java: float translateY = values[5];
            let translateY = values[5];
            // Java: this.drawImage(image, (int)translateX + x, (int)translateY + y, (int)((float)Math.abs(width) * scaleX), (int)((float)Math.abs(height) * scaleY));
            self.drawImage_simple(_image, translateX as i32 + x, translateY as i32 + y, (width.abs() as f32 * scaleX) as i32, (height.abs() as f32 * scaleY) as i32);
        }
    }

    // Java: void drawImage(GdiImage image, int x, int y, int width, int height)
    pub fn drawImage_simple(&mut self, _image: i32, x: i32, y: i32, width: i32, height: i32) {
        // Java: this.drawImage(image, x, y, width, height, false);
        self.drawImage_transparent(_image, x, y, width, height, false);
    }

    // Java: void drawImage(GdiImage image, int x, int y, int width, int height, boolean bTransparent)
    pub fn drawImage_transparent(&mut self, _image: i32, x: i32, y: i32, width: i32, height: i32, bTransparent: bool) {
        // Java: if (image != null)
        {
            // Java: PixelImage pixel = (PixelImage)image;
            // Java: this.canvas.save();
            self.canvas.save();
            // Java: this.transformCanvas();
            self.transformCanvas();
            // Java: this.canvas.translate((float)x, (float)y);
            self.canvas.translate(x as f32, y as f32);
            // Java: this.canvas.scale((float)Math.abs(width) / (float)pixel.getWidth(), (float)Math.abs(height) / (float)pixel.getHeight());
            // Java: if (width < 0)
            if width < 0 {
                // Java: this.canvas.scale(-1.0F, 1.0F);
                self.canvas.scale(-1.0, 1.0);
            }
            // Java: if (height < 0)
            if height < 0 {
                // Java: this.canvas.scale(1.0F, -1.0F);
                self.canvas.scale(1.0, -1.0);
            }
            // Java: if (this.imageDrawerPaint == null)
            if self.imageDrawerPaint.is_none() {
                // Java: this.imageDrawerPaint = new Paint();
                self.imageDrawerPaint = Some(Paint::new());
            }
            // Java: this.imageDrawerPaint.setXfermode(image.getXferMode());
            // Java: if (bTransparent && this.bkMode == 1)
            if bTransparent && self.bkMode == 1 {
                // Java: image.transparent(this.bkColor);
            }
            // Java: this.canvas.drawBitmap(pixel.getColor(), 0, pixel.getWidth(), 0, 0, pixel.getWidth(), pixel.getHeight(), pixel.isAlaph(), this.imageDrawerPaint);
            // Java: this.canvas.restore();
            self.canvas.restore();
        }
    }

    // Java: void drawImage(Bitmap image, int x, int y, int width, int height)
    pub fn drawImage_bitmap(&mut self, image: &Bitmap, x: i32, y: i32, width: i32, height: i32) {
        // Java: if (image != null)
        {
            // Java: this.canvas.save();
            self.canvas.save();
            // Java: this.transformCanvas();
            self.transformCanvas();
            // Java: this.canvas.translate((float)x, (float)y);
            self.canvas.translate(x as f32, y as f32);
            // Java: this.canvas.scale((float)Math.abs(width) / (float)image.getWidth(), (float)Math.abs(height) / (float)image.getHeight());
            let iw = image.getWidth();
            let ih = image.getHeight();
            if iw > 0 && ih > 0 {
                self.canvas.scale(width.abs() as f32 / iw as f32, height.abs() as f32 / ih as f32);
            }
            // Java: if (width < 0)
            if width < 0 {
                // Java: this.canvas.scale(-1.0F, 1.0F);
                self.canvas.scale(-1.0, 1.0);
            }
            // Java: if (height < 0)
            if height < 0 {
                // Java: this.canvas.scale(1.0F, -1.0F);
                self.canvas.scale(1.0, -1.0);
            }
            // Java: this.canvas.drawBitmap(image, (float)x, (float)y, (Paint)null);
            self.canvas.drawBitmap(image, x as f32, y as f32);
            // Java: this.canvas.restore();
            self.canvas.restore();
        }
    }

    // Java: void transformCanvas()
    fn transformCanvas(&self) {
        // Java: this.canvas.concat(this.dc.getTransform());
    }

    // Java: void setGdiPaint(IGdiPainter gdiPaint)
    fn setGdiPaint(&mut self, gdiPaint: &ColorPaint) {
        // Java: if (gdiPaint.getType() == 0)
        if gdiPaint.getType() == 0 {
            // Java: ColorPaint colorPaint = (ColorPaint)gdiPaint;
            // Java: this.paint.setColor(colorPaint.getColor());
            if let Some(ref mut p) = self.paint { p.setColor(gdiPaint.getColor()); }
            // Java: this.paint.setShader((Shader)null);
            if let Some(ref mut p) = self.paint { p.setShader(0); }
        }
        // Java: else if (gdiPaint.getType() == 1)
        else if gdiPaint.getType() == 1 {
            // Java: BitGdiPaint bitPaint = (BitGdiPaint)gdiPaint;
            // Java: GdiImage image = bitPaint.getImage();
            // Java: if (image == null)
            {
                // Java: return;
                return;
            }
            // Java: Bitmap bitmap = this.getGdiPaintImage(image);
            // Java: if (bitmap == null)
            {
                // Java: return;
                return;
            }
            // Java: float scaleX = this.canvasScaleX < 0.5F ? 0.5F : this.canvasScaleX;
            let scaleX = if self.canvasScaleX < 0.5 { 0.5f32 } else { self.canvasScaleX };
            // Java: float scaleY = this.canvasScaleY < 0.5F ? 0.5F : this.canvasScaleY;
            let scaleY = if self.canvasScaleY < 0.5 { 0.5f32 } else { self.canvasScaleY };
            // Java: if (Math.abs(scaleX - 1.0F) > 0.01F || Math.abs(scaleY - 1.0F) > 0.01F)
            if (scaleX - 1.0).abs() > 0.01 || (scaleY - 1.0).abs() > 0.01 {
                // Java: Bitmap bitmapScale = Bitmap.createScaledBitmap(bitmap, (int)((float)bitmap.getWidth() * scaleX), (int)((float)bitmap.getHeight() * scaleY), true);
                // Java: if (bitmapScale != null && bitmap != bitmapScale)
                {
                    // Java: bitmap.recycle();
                    // Java: bitmap = bitmapScale;
                }
            }
            // Java: BitmapShader shader = new BitmapShader(bitmap, TileMode.REPEAT, TileMode.REPEAT);
            // Java: this.paint.setShader(shader);
            // Java: Object var9 = null;
        }
    }

    // Java: boolean isNullGdiPaint(IGdiPainter gdiPaint)
    fn isNullGdiPaint(&self, gdiPaint: &ColorPaint) -> bool {
        // Java: if (gdiPaint.getType() == 0)
        if gdiPaint.getType() == 0 {
            // Java: ColorPaint colorPaint = (ColorPaint)gdiPaint;
            // Java: return colorPaint == ColorPaint.NULL;
            return gdiPaint.getColor() == 0;
        }
        // Java: else
        else {
            // Java: return false;
            return false;
        }
    }

    // Java: Bitmap getGdiPaintImage(GdiImage image)
    fn getGdiPaintImage(&self, _image: i32) -> Option<Bitmap> {
        // Java: if (this.rop2 == 9)
        if self.rop2 == 9 {
            // Java: PixelImage pixelImage = (PixelImage)image;
            // Java: int[] clr = pixelImage.getColor();
            // Java: int[] newClr = new int[clr.length];
            // Java: int len = newClr.length;
            // Java: for(int i = 0; i < len; ++i)
            {
                // Java: newClr[i] = clr[i] == -1 ? 0 : clr[i];
            }
            // Java: return Bitmap.createBitmap(newClr, image.getWidth(), image.getHeight(), Config.ARGB_8888);
            return None;
        }
        // Java: else
        else {
            // Java: return image.getBitmap();
            return None;
        }
    }
}
