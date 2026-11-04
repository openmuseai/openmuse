// Java: import android.graphics.Bitmap
// Java: import android.graphics.BitmapShader
// Java: import android.graphics.MaskFilter
// Java: import android.graphics.Matrix
// Java: import android.graphics.PointF
// Java: import android.graphics.Shader
// Java: import android.graphics.Shader.TileMode
// Java: import android.text.TextPaint
// Java: import cn.wps.text.layout.data.FontFill
// Java: import cn.wps.text.layout.data.GradientAttr
// Java: import cn.wps.text.layout.data.PatternAttr
// Java: import cn.wps.text.layout.typo.layout.style.ForegroundColorSpan
use crate::cn::wps::text::layout::data::FontFill::{FontFill, FontFillType, GradType, GradientAttr, PatternAttr};
use super::ArtTransform::ArtTransform;

pub struct Bitmap;
impl Bitmap {
    pub fn getWidth(&self) -> i32 { 0 }
    pub fn getHeight(&self) -> i32 { 0 }
}

pub struct BitmapShader;
impl BitmapShader {
    pub fn new(_bmp: &Bitmap, _tx: TileMode, _ty: TileMode) -> Self { BitmapShader }
    pub fn setLocalMatrix(&self, _matrix: &Matrix) {}
}

pub struct Matrix;
impl Matrix {
    pub fn new() -> Self { Matrix }
    pub fn reset(&mut self) {}
    pub fn preTranslate(&mut self, _dx: f32, _dy: f32) {}
    pub fn preScale(&mut self, _sx: f32, _sy: f32) {}
}

#[derive(Clone)]
pub struct PointF { pub x: f32, pub y: f32 }

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileMode { CLAMP, REPEAT, MIRROR }

pub struct TextPaint {
    pub color: i32,
    pub alpha: i32,
}
impl TextPaint {
    pub fn setColor(&mut self, color: i32) { self.color = color; }
    pub fn setAlpha(&mut self, alpha: i32) { self.alpha = alpha; }
    pub fn setMaskFilter(&mut self, _mf: Option<()>) {}
    pub fn setShader(&mut self, _shader: Option<BitmapShader>) {}
}

struct GradPaintSetter;
impl GradPaintSetter {
    fn new(_wid: f32, _hei: f32, _left: f32) -> Self { GradPaintSetter }
    fn new_with_art(_wid: f32, _hei: f32, _left: f32, _art: &ArtTransform, _min_left: f32, _min_top: f32, _offset: &PointF) -> Self { GradPaintSetter }
    fn new_with_art_no_top(_wid: f32, _hei: f32, _left: f32, _art: &ArtTransform, _min_left: f32, _offset: &PointF) -> Self { GradPaintSetter }
    fn applyGradFill(&self, _paint: &mut TextPaint, _grad_attr: &GradientAttr, _ln_top: f32) {}
}

struct PatternPaintSetter;
impl PatternPaintSetter {
    fn new(_attr: &PatternAttr) -> Self { PatternPaintSetter }
    fn applyPatternFill(&self, _paint: &mut TextPaint, _matrix: &Option<Matrix>) {}
}


// Java: public class WpsForegroundColorSpan extends ForegroundColorSpan
pub struct WpsForegroundColorSpan {
    // Java field: FontFill mFontFill
    mFontFill: Option<FontFill>,
    // Java field: float mLnHei
    mLnHei: f32,
    // Java field: float mLnWid
    mLnWid: f32,
    // Java field: float mLnTop
    mLnTop: f32,
    // Java field: float mLnLeft
    mLnLeft: f32,
    // Java field: Matrix canvasMatrix
    canvasMatrix: Option<Matrix>,
    // Java field: Matrix matrix
    matrix: Option<Matrix>,
    // Java field: ArtTransform mArtTransform
    mArtTransform: Option<ArtTransform>,
    // Java field: PointF offset
    offset: Option<PointF>,
    // Java field: float mMinLeft
    mMinLeft: f32,
    // Java field: float mMinTop
    mMinTop: f32,
}

// Java: public class WpsForegroundColorSpan extends ForegroundColorSpan
impl WpsForegroundColorSpan {
    // Java: WpsForegroundColorSpan(FontFill fontFill, float lnHei, float lnWid, float lnTop, float lnLeft, ArtTransform artTransform, PointF offset, float minLeft, float minTop)
    pub fn new(fontFill: FontFill, lnHei: f32, lnWid: f32, lnTop: f32, lnLeft: f32, artTransform: Option<ArtTransform>, offset: Option<PointF>, minLeft: f32, minTop: f32) -> Self {
      // Java: super(0);
      // Java: this.mFontFill = fontFill;
      // Java: this.mLnHei = lnHei;
      // Java: this.mLnWid = lnWid;
      // Java: this.mLnTop = lnTop;
      // Java: this.mLnLeft = lnLeft;
      // Java: this.mArtTransform = artTransform;
      // Java: this.offset = offset;
      // Java: this.mMinLeft = minLeft;
      // Java: this.mMinTop = minTop;
      WpsForegroundColorSpan {
          mFontFill: Some(fontFill),
          mLnHei: lnHei,
          mLnWid: lnWid,
          mLnTop: lnTop,
          mLnLeft: lnLeft,
          canvasMatrix: None,
          matrix: Some(Matrix::new()),
          mArtTransform: artTransform,
          offset,
          mMinLeft: minLeft,
          mMinTop: minTop,
      }
    }

    // Java: void updateDrawState(TextPaint paint)
    pub fn updateDrawState(&mut self, paint: &mut TextPaint) {
      // Java: this.applyFontFill(paint);
      self.applyFontFill(paint);
    }

    // Java: void applyFontFill(TextPaint paint)
    pub fn applyFontFill(&mut self, paint: &mut TextPaint) {
      // Java: paint.setMaskFilter((MaskFilter)null);
      paint.setMaskFilter(None);
      // Java: paint.setShader((Shader)null);
      paint.setShader(None);
      let font_fill = match &self.mFontFill {
          Some(ff) => ff,
          None => return,
      };
      // Java: switch (this.mFontFill.mFillType)
      match font_fill.mFillType
      {
         // Java: case COLORFILL:
         FontFillType::COLORFILL => {
            // Java: paint.setColor(this.mFontFill.color);
            paint.setColor(font_fill.color);
            // Java: return;
            return;
         }
         // Java: case NONE:
         FontFillType::NONE => {
            // Java: paint.setAlpha(0);
            paint.setAlpha(0);
            // Java: return;
            return;
         }
         // Java: case BLIPFILL:
         FontFillType::BLIPFILL => {
            // Java: if (this.mFontFill.mBlipFill != null && this.mFontFill.mBlipFill.getBitmap() != null)
            if font_fill.mBlipFill.is_some() && font_fill.mBlipFill.as_ref().unwrap().getBitmap().is_some()
            {
               // Java: this.applyBlipFill(paint, this.mFontFill.mBlipFill.getBitmap());
               self.applyBlipFill(paint);
               // Java: return;
               return;
            }
            // Java: paint.setColor(-16777216);
            paint.setColor(-16777216);
            // Java: return;
            return;
         }
         // Java: case GRADFILL:
         FontFillType::GRADFILL => {
            // Java: if (this.mFontFill.mGradFill != null)
            if font_fill.mGradFill.is_some()
            {
               // Java: GradientAttr gradAttr = this.mFontFill.mGradFill.getGradAttr();
               let grad_attr = font_fill.mGradFill.as_ref().unwrap().getGradAttr();
               // Java: GradPaintSetter gradSetter;
               let gradSetter: GradPaintSetter;
               // Java: if (this.mArtTransform == null)
               if self.mArtTransform.is_none()
               {
                  // Java: gradSetter = new GradPaintSetter(this.mLnWid, this.mLnHei, this.mLnLeft);
                  gradSetter = GradPaintSetter::new(self.mLnWid, self.mLnHei, self.mLnLeft);
               }
               // Java: else if (gradAttr.gradType() == GradientAttr.GradType.RECT)
               else if grad_attr.gradType() == GradType::RECT
               {
                  // Java: gradSetter = new GradPaintSetter(this.mLnWid, this.mLnHei, this.mLnLeft, this.mArtTransform, this.mMinLeft, this.mMinTop, this.offset);
                  gradSetter = GradPaintSetter::new_with_art(self.mLnWid, self.mLnHei, self.mLnLeft, self.mArtTransform.as_ref().unwrap(), self.mMinLeft, self.mMinTop, self.offset.as_ref().unwrap());
               }
               // Java: else
               else
               {
                  // Java: gradSetter = new GradPaintSetter(this.mLnWid, this.mLnHei, this.mLnLeft, this.mArtTransform, this.mMinLeft, this.offset);
                  gradSetter = GradPaintSetter::new_with_art_no_top(self.mLnWid, self.mLnHei, self.mLnLeft, self.mArtTransform.as_ref().unwrap(), self.mMinLeft, self.offset.as_ref().unwrap());
               }
               // Java: gradSetter.applyGradFill(paint, gradAttr, this.mLnTop);
               gradSetter.applyGradFill(paint, grad_attr, self.mLnTop);
            }
            // Java: return;
            return;
         }
         // Java: case PATTERNFILL:
         FontFillType::PATTERNFILL => {
            // Java: if (this.mFontFill.mPattFill != null)
            if font_fill.mPattFill.is_some()
            {
               // Java: PatternAttr patternAttr = this.mFontFill.mPattFill.getPatternAttr();
               let pattern_attr = font_fill.mPattFill.as_ref().unwrap().getPatternAttr();
               // Java: PatternPaintSetter patternPaintSetter = new PatternPaintSetter(patternAttr);
               let patternPaintSetter = PatternPaintSetter::new(pattern_attr);
               // Java: patternPaintSetter.applyPatternFill(paint, this.canvasMatrix);
               patternPaintSetter.applyPatternFill(paint, &self.canvasMatrix);
            }
            // Java: break;
         }
         // Java: default:
         #[allow(unreachable_patterns)]
         _ => {
            // Java: paint.setColor(-16777216);
            paint.setColor(-16777216);
         }
      }
    }

    // Java: void applyBlipFill(TextPaint paint, Bitmap bmp)
    pub fn applyBlipFill(&mut self, _paint: &mut TextPaint) {
      // Java: this.matrix.reset();
      if let Some(ref mut matrix) = self.matrix { matrix.reset(); }
      let bmp = Bitmap;
      let bmp_width = bmp.getWidth() as f32;
      let bmp_height = bmp.getHeight() as f32;
      let fill_mode = self.mFontFill.as_ref().unwrap().mBlipFill.as_ref().unwrap().getFillMode();
      let alpha = self.mFontFill.as_ref().unwrap().mBlipFill.as_ref().unwrap().getAlpha();
      // Java: BitmapShader bmpShader;
      let bmpShader: BitmapShader;
      // Java: if (this.mFontFill.mBlipFill.getFillMode() == 1)
      if fill_mode == 1
      {
         // Java: if (this.mArtTransform != null)
         if self.mArtTransform.is_some()
         {
            // Java: this.matrix.preTranslate(-this.offset.x + this.mMinLeft, -this.offset.y);
            let (ox, oy) = self.offset.as_ref().map_or((0.0, 0.0), |o| (o.x, o.y));
            if let Some(ref mut matrix) = self.matrix {
                matrix.preTranslate(-ox + self.mMinLeft, -oy);
            }
            // Java: this.matrix.preScale(this.mArtTransform.getTextWidth() / (float)bmp.getWidth(), this.mArtTransform.getTextHeight() / (float)bmp.getHeight());
            if let Some(ref mut matrix) = self.matrix {
                matrix.preScale(0.0 / bmp_width, 0.0 / bmp_height);
            }
         }
         // Java: else
         else
         {
            // Java: this.matrix.preTranslate(this.mLnLeft, 0.0F);
            if let Some(ref mut matrix) = self.matrix { matrix.preTranslate(self.mLnLeft, 0.0); }
            // Java: this.matrix.preScale(this.mLnWid / (float)bmp.getWidth(), this.mLnHei / (float)bmp.getHeight());
            if let Some(ref mut matrix) = self.matrix { matrix.preScale(self.mLnWid / bmp_width, self.mLnHei / bmp_height); }
         }
         // Java: bmpShader = new BitmapShader(bmp, TileMode.CLAMP, TileMode.MIRROR);
         bmpShader = BitmapShader::new(&bmp, TileMode::CLAMP, TileMode::MIRROR);
      }
      // Java: else
      else
      {
         // Java: bmpShader = this.geneTileBmpShader(bmp);
         bmpShader = self.geneTileBmpShader(&bmp);
      }
      // Java: bmpShader.setLocalMatrix(this.matrix);
      if let Some(ref matrix) = self.matrix { bmpShader.setLocalMatrix(matrix); }
      // Java: paint.setShader(bmpShader);
      _paint.setShader(Some(bmpShader));
      // Java: paint.setAlpha((int)(this.mFontFill.mBlipFill.getAlpha() * 255.0F));
      _paint.setAlpha((alpha * 255.0) as i32);
    }

    // Java: void setPreTranslate(float bmpWid_pt, float bmpHei_pt)
    pub fn setPreTranslate(&mut self, bmpWid_pt: f32, bmpHei_pt: f32) {
      // Java: this.matrix.preTranslate(0.0F, this.mLnTop);
      if let Some(ref mut matrix) = self.matrix { matrix.preTranslate(0.0, self.mLnTop); }
      let offset_x = self.mFontFill.as_ref().unwrap().mBlipFill.as_ref().unwrap().getOffsetX();
      let offset_y = self.mFontFill.as_ref().unwrap().mBlipFill.as_ref().unwrap().getOffsetY();
      let algn = self.mFontFill.as_ref().unwrap().mBlipFill.as_ref().unwrap().getAlgn();
      // Java: this.matrix.preTranslate(this.mFontFill.mBlipFill.getOffsetX(), this.mFontFill.mBlipFill.getOffsetY() * 0.84F);
      if let Some(ref mut matrix) = self.matrix { matrix.preTranslate(offset_x, offset_y * 0.84); }
      // Java: float disX = 0.0F;
      let mut disX = 0.0f32;
      // Java: float disY = 0.0F;
      let mut disY = 0.0f32;
      // Java: switch (this.mFontFill.mBlipFill.getAlgn())
      match algn
      {
         // Java: case -1:
         // Java: case 7:
         // Java: default:
         -1 | 7 => {
            // Java: break;
         }
         // Java: case 0:
         0 => {
            // Java: disX = (this.mLnWid - bmpWid_pt) / 2.0F;
            disX = (self.mLnWid - bmpWid_pt) / 2.0;
            // Java: disY = this.mLnHei - bmpHei_pt;
            disY = self.mLnHei - bmpHei_pt;
            // Java: break;
         }
         // Java: case 1:
         1 => {
            // Java: disY = this.mLnHei - bmpHei_pt;
            disY = self.mLnHei - bmpHei_pt;
            // Java: break;
         }
         // Java: case 2:
         2 => {
            // Java: disX = this.mLnWid - bmpWid_pt;
            disX = self.mLnWid - bmpWid_pt;
            // Java: disY = this.mLnHei - bmpHei_pt;
            disY = self.mLnHei - bmpHei_pt;
            // Java: break;
         }
         // Java: case 3:
         3 => {
            // Java: disX = (this.mLnWid - bmpWid_pt) / 2.0F;
            disX = (self.mLnWid - bmpWid_pt) / 2.0;
            // Java: disY = (this.mLnHei - bmpHei_pt) / 2.0F;
            disY = (self.mLnHei - bmpHei_pt) / 2.0;
            // Java: break;
         }
         // Java: case 4:
         4 => {
            // Java: disY = (this.mLnHei - bmpHei_pt) / 2.0F;
            disY = (self.mLnHei - bmpHei_pt) / 2.0;
            // Java: break;
         }
         // Java: case 5:
         5 => {
            // Java: disX = this.mLnWid - bmpWid_pt;
            disX = self.mLnWid - bmpWid_pt;
            // Java: disY = (this.mLnHei - bmpHei_pt) / 2.0F;
            disY = (self.mLnHei - bmpHei_pt) / 2.0;
            // Java: break;
         }
         // Java: case 6:
         6 => {
            // Java: disX = (this.mLnWid - bmpWid_pt) / 2.0F;
            disX = (self.mLnWid - bmpWid_pt) / 2.0;
            // Java: break;
         }
         // Java: case 8:
         8 => {
            // Java: disX = this.mLnWid - bmpWid_pt;
            disX = self.mLnWid - bmpWid_pt;
         }
         _ => {}
      }
      // Java: this.matrix.preTranslate(disX, disY);
      if let Some(ref mut matrix) = self.matrix { matrix.preTranslate(disX, disY); }
      // Java: if (this.mArtTransform == null)
      if self.mArtTransform.is_none()
      {
         // Java: this.matrix.preTranslate(this.mLnLeft, 0.0F);
         if let Some(ref mut matrix) = self.matrix { matrix.preTranslate(self.mLnLeft, 0.0); }
      }
      // Java: else
      else
      {
         // Java: this.matrix.preTranslate(-this.offset.x + this.mMinLeft, -this.offset.y);
         if let (Some(ref mut matrix), Some(ref offset)) = (&mut self.matrix, &self.offset) {
             matrix.preTranslate(-offset.x + self.mMinLeft, -offset.y);
         }
      }
    }

    // Java: BitmapShader geneTileBmpShader(Bitmap bmp)
    pub fn geneTileBmpShader(&mut self, bmp: &Bitmap) -> BitmapShader {
      let scale_x = self.mFontFill.as_ref().unwrap().mBlipFill.as_ref().unwrap().getScaleX();
      let scale_y = self.mFontFill.as_ref().unwrap().mBlipFill.as_ref().unwrap().getScaleY();
      let flip_type = self.mFontFill.as_ref().unwrap().mBlipFill.as_ref().unwrap().getFlipType();
      // Java: float bmpWid_px = (float)bmp.getWidth();
      let bmpWid_px = bmp.getWidth() as f32;
      // Java: float bmpHei_px = (float)bmp.getHeight();
      let bmpHei_px = bmp.getHeight() as f32;
      // Java: float bmpWid_pt = bmpWid_px * 72.0F / 96.0F * this.mFontFill.mBlipFill.getScaleX();
      let bmpWid_pt = bmpWid_px * 72.0 / 96.0 * scale_x;
      // Java: if (this.mArtTransform == null)
      if self.mArtTransform.is_none()
      {
         // Java: float bmpHei_pt = bmpHei_px * 72.0F / 96.0F * this.mFontFill.mBlipFill.getScaleY() * 0.85F;
         let bmpHei_pt = bmpHei_px * 72.0 / 96.0 * scale_y * 0.85;
         // Java: this.setPreTranslate(bmpWid_pt, bmpHei_pt);
         self.setPreTranslate(bmpWid_pt, bmpHei_pt);
         // Java: this.matrix.preScale(bmpWid_pt / bmpWid_px, bmpHei_pt / bmpHei_px);
         if let Some(ref mut matrix) = self.matrix { matrix.preScale(bmpWid_pt / bmpWid_px, bmpHei_pt / bmpHei_px); }
      }
      // Java: else
      else
      {
         // Java: float bmpHei_pt = bmpHei_px * 72.0F / 96.0F * this.mFontFill.mBlipFill.getScaleY();
         let bmpHei_pt = bmpHei_px * 72.0 / 96.0 * scale_y;
         // Java: this.setPreTranslate(bmpWid_pt, bmpHei_pt);
         self.setPreTranslate(bmpWid_pt, bmpHei_pt);
         // Java: PointF artScale = this.mArtTransform.getScale();
         // Java: this.matrix.preScale(bmpWid_pt / bmpWid_px / artScale.x, bmpHei_pt / bmpHei_px / artScale.y);
         if let Some(ref mut matrix) = self.matrix { matrix.preScale(bmpWid_pt / bmpWid_px, bmpHei_pt / bmpHei_px); }
      }
      // Java: Shader.TileMode tileMode_x = TileMode.REPEAT;
      let mut tileMode_x = TileMode::REPEAT;
      // Java: Shader.TileMode tileMode_y = TileMode.REPEAT;
      let mut tileMode_y = TileMode::REPEAT;
      // Java: switch (this.mFontFill.mBlipFill.getFlipType())
      match flip_type
      {
         // Java: case 0:
         // Java: default:
         0 => {
            // Java: break;
         }
         // Java: case 1:
         1 => {
            // Java: tileMode_x = TileMode.MIRROR;
            tileMode_x = TileMode::MIRROR;
            // Java: break;
         }
         // Java: case 2:
         2 => {
            // Java: tileMode_y = TileMode.MIRROR;
            tileMode_y = TileMode::MIRROR;
            // Java: break;
         }
         // Java: case 3:
         3 => {
            // Java: tileMode_x = TileMode.MIRROR;
            tileMode_x = TileMode::MIRROR;
            // Java: tileMode_y = TileMode.MIRROR;
            tileMode_y = TileMode::MIRROR;
         }
         _ => {}
      }
      // Java: BitmapShader bmpShader = new BitmapShader(bmp, tileMode_x, tileMode_y);
      let bmpShader = BitmapShader::new(bmp, tileMode_x, tileMode_y);
      // Java: return bmpShader;
      bmpShader
    }

    // Java: void setCanvasMatrix(Matrix canvasMatrix)
    pub fn setCanvasMatrix(&mut self, canvasMatrix: Matrix) {
      // Java: this.canvasMatrix = canvasMatrix;
      self.canvasMatrix = Some(canvasMatrix);
    }

}
