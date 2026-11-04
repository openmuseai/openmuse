// Java: import android.graphics.Canvas
// Java: import android.graphics.Matrix
// Java: import android.graphics.Paint
// Java: import android.graphics.RectF
// Java: import android.graphics.Typeface
// Java: import cn.wps.text.layout.data.BulletsInfo
// Java: import cn.wps.text.layout.data.TextData
// Java: import cn.wps.text.layout.typo.ArtTransform
// Java: import cn.wps.text.layout.typo.PaintFontHelper

use crate::cn::wps::text::layout::data::BulletsInfo::BulletsInfo;
use crate::cn::wps::text::layout::data::TextData::TextData;
use crate::cn::wps::text::layout::typo::PaintFontHelper::{PaintFontHelper, Typeface};

pub struct Bitmap;

pub struct Canvas;
impl Canvas {
    pub fn new() -> Self { Canvas }
    pub fn save(&self) {}
    pub fn restore(&self) {}
    pub fn translate(&self, _x: f32, _y: f32) {}
    pub fn rotate(&self, _degrees: f32) {}
    pub fn rotate_at(&self, _degrees: f32, _px: f32, _py: f32) {}
    pub fn scale(&self, _sx: f32, _sy: f32) {}
    pub fn setBitmap(&self, _bmp: &Bitmap) {}
    pub fn drawText(&self, _text: &str, _x: f32, _y: f32, _paint: &Paint) {}
    pub fn drawBitmap(&self, _bmp: &Bitmap, _matrix: &Matrix, _paint: &Paint) {}
}

pub struct Paint {
    pub color: i32,
}
impl Paint {
    pub fn new() -> Self { Paint { color: 0 } }
    pub fn reset(&mut self) {}
    pub fn setAntiAlias(&mut self, _aa: bool) {}
    pub fn setSubpixelText(&mut self, _sp: bool) {}
    pub fn setFlags(&mut self, _flags: i32) {}
    pub fn setColor(&mut self, color: i32) { self.color = color; }
    pub fn setAlpha(&mut self, _alpha: i32) {}
    pub fn setTextSize(&mut self, _size: f32) {}
    pub fn setTypeface(&mut self, _tf: Typeface) {}
    pub fn setFilterBitmap(&mut self, _f: bool) {}
    pub fn setDither(&mut self, _d: bool) {}
}

pub struct RectF {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl RectF {
    pub fn new() -> Self { RectF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 } }
    pub fn width(&self) -> f32 { self.right - self.left }
    pub fn height(&self) -> f32 { self.bottom - self.top }
    pub fn set(&mut self, src: &RectF) { self.left = src.left; self.top = src.top; self.right = src.right; self.bottom = src.bottom; }
}

pub struct Matrix;
impl Matrix {
    pub fn new() -> Self { Matrix }
    pub fn postScale(&mut self, _sx: f32, _sy: f32) {}
}

// Java: public class BulletDrawer
pub struct BulletDrawer {
    // Java field: Paint paint
    paint: Paint,
    // Java field: RectF rect
    rect: RectF,
}

// Java: public class BulletDrawer
impl BulletDrawer {
    // Java: BulletDrawer()
    pub fn new() -> Self {
      // Java: super();
      BulletDrawer { paint: Paint::new(), rect: RectF::new() }
    }

    // Java: BulletDrawer(RectF typoRect)
    pub fn new_with_rect(typoRect: RectF) -> Self {
      // Java: super();
      // Java: this.rect = typoRect;
      BulletDrawer { paint: Paint::new(), rect: typoRect }
    }

    // Java: void setRenderRect(RectF src)
    pub fn setRenderRect(&mut self, src: &RectF) {
      // Java: this.rect.set(src);
      self.rect.set(src);
    }

    // Java: void drawText(Canvas canvas, BulletsInfo bullets, float fontsize, int visiblemode)
    pub fn drawText(&mut self, canvas: &Canvas, bullets: &mut BulletsInfo, fontsize: f32, visiblemode: i32) {
      // Java: if (bullets.getBulletStr().length() >= 1)
      if bullets.getBulletStr().len() >= 1
      {
         // Java: Paint textpaint = this.paint;
         let textpaint = &mut self.paint;
         // Java: textpaint.reset();
         textpaint.reset();
         // Java: textpaint.setAntiAlias(true);
         textpaint.setAntiAlias(true);
         // Java: textpaint.setSubpixelText(true);
         textpaint.setSubpixelText(true);
         // Java: textpaint.setFlags(129);
         textpaint.setFlags(129);
         // Java: textpaint.setColor(bullets.getColor());
         textpaint.setColor(bullets.getColor());
         // Java: if (2 == visiblemode)
         if 2 == visiblemode
         {
            // Java: textpaint.setAlpha(64);
            textpaint.setAlpha(64);
         }
         // Java: textpaint.setTextSize(bullets.getBulletSize() * bullets.getAnsizePercent());
         textpaint.setTextSize(bullets.getBulletSize() * bullets.getAnsizePercent());
         // Java: Typeface typeface = PaintFontHelper.getTypeface(bullets.getFontName(), false, false);
         let typeface = PaintFontHelper::getTypeface(bullets.getFontName(), false, false);
         // Java: textpaint.setTypeface(typeface);
         textpaint.setTypeface(typeface);
         // Java: canvas.save();
         canvas.save();
         // Java: switch (bullets.getRotateDegrees())
         match bullets.getRotateDegrees()
         {
            // Java: case 90:
            90 => {
               // Java: canvas.translate(this.rect.width(), 0.0F);
               canvas.translate(self.rect.width(), 0.0);
               // Java: canvas.rotate(90.0F, this.rect.left, this.rect.top);
               canvas.rotate_at(90.0, self.rect.left, self.rect.top);
               // Java: break;
            }
            // Java: case 270:
            270 => {
               // Java: canvas.translate(0.0F, this.rect.height());
               canvas.translate(0.0, self.rect.height());
               // Java: canvas.rotate(270.0F, this.rect.left, this.rect.top);
               canvas.rotate_at(270.0, self.rect.left, self.rect.top);
            }
            _ => {}
         }
         // Java: canvas.translate(this.rect.left, this.rect.top);
         canvas.translate(self.rect.left, self.rect.top);
         // Java: Canvas workCanvas = canvas;
         let workCanvas = canvas;
         // Java: ArtTransform artt = bullets.getArtTransform();
         // Java: if (null != artt)
         {
            // Java: workCanvas = new Canvas();
            // Java: workCanvas.scale(artt.getTransformMagnifyFactor(), artt.getTransformMagnifyFactor());
            // Java: workCanvas.setBitmap(artt.createTextBitmap());
         }
         // Java: workCanvas.translate(bullets.getX(), bullets.getY());
         workCanvas.translate(bullets.getX(), bullets.getY());
         // Java: if (bullets.isEastVertical() && TextData.isChinese(bullets.getBulletStr().charAt(0)))
         let first_char = bullets.getBulletStr().chars().next().unwrap_or('\0');
         if bullets.isEastVertical() && TextData::isChinese(first_char)
         {
            // Java: String text = bullets.getBulletStr();
            let text = bullets.getBulletStr().to_string();
            // Java: float txm = 0.0F;
            let mut txm = 0.0f32;
            // Java: float tym = 0.0F;
            let mut tym: f32;
            // Java: float xoff = 0.0F;
            let mut xoff = 0.0f32;
            // Java: float cwfst = 0.0F;
            let mut cwfst = 0.0f32;
            // Java: for(int i = 0; i < text.length(); ++i)
            for (i, c) in text.chars().enumerate()
            {
               // Java: char c = text.charAt(i);
               // Java: float cw = PaintFontHelper.getCharWidth(String.valueOf(c), bullets.getFontName(), bullets.getBulletSize() * bullets.getAnsizePercent(), false, false);
               let c_str = c.to_string();
               let cw = PaintFontHelper::getCharWidth_text(&c_str, bullets.getFontName(), bullets.getBulletSize() * bullets.getAnsizePercent(), false, false);
               // Java: float cm;
               let cm: f32;
               // Java: if (!TextData.isEnclosedAlphanum(text.charAt(0)) && text.length() <= 1)
               if !TextData::isEnclosedAlphanum(first_char) && text.len() <= 1
               {
                  // Java: cm = (float)((double)cw * 0.8);
                  cm = (cw as f64 * 0.8) as f32;
               }
               // Java: else
               else
               {
                  // Java: cm = (float)((double)cw * 1.1);
                  cm = (cw as f64 * 1.1) as f32;
               }
               // Java: if (c == '.')
               if c == '.'
               {
                  // Java: tym = cwfst * 0.7F;
                  tym = cwfst * 0.7;
               }
               // Java: else
               else
               {
                  // Java: tym = cwfst > 0.0F ? (cwfst - cw) / 2.0F : 0.0F;
                  tym = if cwfst > 0.0 { (cwfst - cw) / 2.0 } else { 0.0 };
               }
               // Java: workCanvas.save();
               workCanvas.save();
               // Java: workCanvas.translate(txm + xoff, tym);
               workCanvas.translate(txm + xoff, tym);
               // Java: workCanvas.rotate(-90.0F);
               workCanvas.rotate(-90.0);
               // Java: workCanvas.translate(-cm, 0.0F);
               workCanvas.translate(-cm, 0.0);
               // Java: workCanvas.drawText(String.valueOf(c), 0.0F, fontsize, textpaint);
               workCanvas.drawText(&c_str, 0.0, fontsize, &self.paint);
               // Java: workCanvas.restore();
               workCanvas.restore();
               // Java: txm += cw;
               txm += cw;
               // Java: xoff = -cw * 0.2F;
               xoff = -cw * 0.2;
               // Java: if (0 == i)
               if 0 == i
               {
                  // Java: cwfst = cw;
                  cwfst = cw;
               }
            }
         }
         // Java: else if (bullets.isRightToLeft)
         else if bullets.isRightToLeft()
         {
            // Java: String text = bullets.getBulletStr();
            let text = bullets.getBulletStr().to_string();
            // Java: workCanvas.translate(bullets.getWidth(), 0.0F);
            workCanvas.translate(bullets.getWidth(), 0.0);
            // Java: for(int i = 0; i < text.length(); ++i)
            for c in text.chars()
            {
               // Java: char c = text.charAt(i);
               // Java: float cw = PaintFontHelper.getCharWidth(String.valueOf(c), bullets.getFontName(), bullets.getBulletSize() * bullets.getAnsizePercent(), false, false);
               let c_str = c.to_string();
               let cw = PaintFontHelper::getCharWidth_text(&c_str, bullets.getFontName(), bullets.getBulletSize() * bullets.getAnsizePercent(), false, false);
               // Java: workCanvas.translate(-cw, 0.0F);
               workCanvas.translate(-cw, 0.0);
               // Java: workCanvas.drawText(String.valueOf(c), 0.0F, fontsize, textpaint);
               workCanvas.drawText(&c_str, 0.0, fontsize, &self.paint);
            }
         }
         // Java: else
         else
         {
            // Java: workCanvas.drawText(bullets.getBulletStr(), 0.0F, fontsize, textpaint);
            let bullet_str = bullets.getBulletStr().to_string();
            workCanvas.drawText(&bullet_str, 0.0, fontsize, &self.paint);
         }
         // Java: if (null != artt)
         {
            // Java: artt.transformText();
            // Java: artt.drawResult(canvas, textpaint);
         }
         // Java: canvas.restore();
         canvas.restore();
      }
    }

    // Java: void drawBitmap(Canvas canvas, BulletsInfo bullets, float fontsize)
    pub fn drawBitmap(&mut self, canvas: &Canvas, bullets: &mut BulletsInfo, fontsize: f32) {
      // Java: if (bullets.getBmp() != null)
      if bullets.getBmp().is_some()
      {
         // Java: Paint tempPaint = this.paint;
         let tempPaint = &mut self.paint;
         // Java: tempPaint.reset();
         tempPaint.reset();
         // Java: tempPaint.setAntiAlias(false);
         tempPaint.setAntiAlias(false);
         // Java: tempPaint.setFilterBitmap(true);
         tempPaint.setFilterBitmap(true);
         // Java: tempPaint.setDither(true);
         tempPaint.setDither(true);
         // Java: canvas.save();
         canvas.save();
         // Java: switch (bullets.getRotateDegrees())
         match bullets.getRotateDegrees()
         {
            // Java: case 90:
            90 => {
               // Java: canvas.translate(this.rect.width(), 0.0F);
               canvas.translate(self.rect.width(), 0.0);
               // Java: canvas.rotate(90.0F, this.rect.left, this.rect.top);
               canvas.rotate_at(90.0, self.rect.left, self.rect.top);
               // Java: break;
            }
            // Java: case 270:
            270 => {
               // Java: canvas.translate(0.0F, this.rect.height());
               canvas.translate(0.0, self.rect.height());
               // Java: canvas.rotate(270.0F, this.rect.left, this.rect.top);
               canvas.rotate_at(270.0, self.rect.left, self.rect.top);
            }
            _ => {}
         }
         // Java: canvas.translate(this.rect.left, this.rect.top);
         canvas.translate(self.rect.left, self.rect.top);
         // Java: Canvas workCanvas = canvas;
         let workCanvas = canvas;
         // Java: ArtTransform artt = bullets.getArtTransform();
         // Java: if (null != artt)
         {
            // Java: workCanvas = new Canvas();
            // Java: workCanvas.scale(artt.getTransformMagnifyFactor(), artt.getTransformMagnifyFactor());
            // Java: workCanvas.setBitmap(artt.createTextBitmap());
         }
         // Java: workCanvas.translate(bullets.getX(), bullets.getY());
         workCanvas.translate(bullets.getX(), bullets.getY());
         // Java: Matrix matrix = new Matrix();
         let mut matrix = Matrix::new();
         // Java: matrix.postScale(bullets.getBmpScale(), bullets.getBmpScale());
         matrix.postScale(bullets.getBmpScale(), bullets.getBmpScale());
         // Java: workCanvas.drawBitmap(bullets.getBmp(), matrix, tempPaint);
         // Java: if (null != artt)
         {
            // Java: artt.transformText();
            // Java: artt.drawResult(canvas, tempPaint);
         }
         // Java: canvas.restore();
         canvas.restore();
      }
      // Java: else
      else
      {
         // Java: if (bullets.getBulletStr() == null)
         if bullets.getBulletStr().is_empty()
         {
            // Java: if (bullets.getDefaultBullet() != null)
            if !bullets.getDefaultBullet().is_empty()
            {
               // Java: bullets.setBulletStr(bullets.getDefaultBullet());
               let default_bullet = bullets.getDefaultBullet().to_string();
               bullets.setBulletStr(default_bullet);
            }
            // Java: else
            else
            {
               // Java: bullets.setBulletStr(bullets.BULLET_DEFALT);
               bullets.setBulletStr(String::from("\u{2022}"));
            }
         }
         // Java: this.drawText(canvas, bullets, fontsize, 1);
         self.drawText(canvas, bullets, fontsize, 1);
      }
    }

}
