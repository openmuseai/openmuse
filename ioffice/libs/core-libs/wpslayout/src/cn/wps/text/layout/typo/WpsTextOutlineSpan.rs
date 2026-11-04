// Java: import android.graphics.BlurMaskFilter
// Java: import android.graphics.ComposePathEffect
// Java: import android.graphics.DashPathEffect
// Java: import android.graphics.MaskFilter
// Java: import android.graphics.Matrix
// Java: import android.graphics.Path
// Java: import android.graphics.PathDashPathEffect
// Java: import android.graphics.PathEffect
// Java: import android.graphics.RectF
// Java: import android.graphics.Shader
// Java: import android.graphics.BlurMaskFilter.Blur
// Java: import android.graphics.Paint.Join
// Java: import android.graphics.Paint.Style
// Java: import android.graphics.Path.Direction
// Java: import android.text.TextPaint
use crate::cn::wps::text::layout::typo::layout::style::CharacterStyle::{TextPaint, CharacterStyleTrait};
// Java: import cn.wps.text.layout.data.TextOutline
use crate::cn::wps::text::layout::data::TextOutline::{TextOutline, TxtOutlnType};
// Java: import cn.wps.text.layout.typo.layout.style.CharacterStyle

pub struct Matrix;

// Java: public class WpsTextOutlineSpan extends CharacterStyle
pub struct WpsTextOutlineSpan {
    // Java field: TextOutline mTextOutline
    mTextOutline: TextOutline,
    // Java field: float mLineWid
    mLineWid: f32,
    // Java field: float mLineHei
    mLineHei: f32,
    // Java field: float mLineTop
    mLineTop: f32,
    // Java field: float mActualWid_pt
    pub(crate) mActualWid_pt: f32,
    // Java field: float mLineLeft
    mLineLeft: f32,
    // Java field: Matrix canvasMatrix
    canvasMatrix: Option<Matrix>,
    // Java field: float mBlurRad
    mBlurRad: f32,
    // Java field: int mShdwClr
    mShdwClr: i32,
    // Java field: boolean mIsYCompress
    mIsYCompress: bool,
}

// Java: public class WpsTextOutlineSpan extends CharacterStyle
impl WpsTextOutlineSpan {
    // Java: WpsTextOutlineSpan(TextOutline textOutline, float lineWid, float lineHei, float lineTop, float lineLeft)
    pub fn new(textOutline: TextOutline, lineWid: f32, lineHei: f32, lineTop: f32, lineLeft: f32) -> Self {
      // Java: super();
      // Java: this.mTextOutline = textOutline;
      // Java: this.mLineWid = lineWid;
      // Java: this.mLineHei = lineHei;
      // Java: this.mLineTop = lineTop;
      let mut s = WpsTextOutlineSpan {
          mTextOutline: textOutline,
          mLineWid: lineWid,
          mLineHei: lineHei,
          mLineTop: lineTop,
          mActualWid_pt: 0.0,
          mLineLeft: lineLeft,
          canvasMatrix: None,
          mBlurRad: 0.0,
          mShdwClr: 0,
          mIsYCompress: false,
      };
      // Java: this.mActualWid_pt = this.getOutlnWid();
      s.mActualWid_pt = s.getOutlnWid();
      // Java: this.mLineLeft = lineLeft;
      s
    }

    // Java: WpsTextOutlineSpan(float blurRad, int shdwClr, TextOutline textOutline, boolean isYCompress)
    pub fn new_blur(blurRad: f32, shdwClr: i32, textOutline: TextOutline, isYCompress: bool) -> Self {
      // Java: super();
      // Java: this.mTextOutline = textOutline;
      // Java: this.mBlurRad = blurRad;
      // Java: this.mShdwClr = shdwClr;
      // Java: this.mIsYCompress = isYCompress;
      let mut s = WpsTextOutlineSpan {
          mTextOutline: textOutline,
          mLineWid: 0.0,
          mLineHei: 0.0,
          mLineTop: 0.0,
          mActualWid_pt: 0.0,
          mLineLeft: 0.0,
          canvasMatrix: None,
          mBlurRad: blurRad,
          mShdwClr: shdwClr,
          mIsYCompress: isYCompress,
      };
      // Java: this.mActualWid_pt = this.getOutlnWid();
      s.mActualWid_pt = s.getOutlnWid();
      s
    }

    // Java: float getOutlnWid()
    pub fn getOutlnWid(&self) -> f32 {
      // Java: float wid = this.mTextOutline.getLineWid();
      let mut wid = self.mTextOutline.getLineWid();
      // Java: wid = wid < 0.25F ? 0.25F : wid;
      wid = if wid < 0.25 { 0.25 } else { wid };
      // Java: return wid;
      wid
    }

    // Java: void updateDrawState(TextPaint paint)
    pub fn updateDrawState(&self, paint: &mut TextPaint) {
      // Java: this.applyTextOutline(paint);
      self.applyTextOutline(paint);
      // Java: if (this.mBlurRad > 0.0F)
      if self.mBlurRad > 0.0
      {
         // Java: this.applyOutlnBlur(paint);
         self.applyOutlnBlur(paint);
      }
    }

    // Java: void applyTextOutline(TextPaint paint)
    pub fn applyTextOutline(&self, _paint: &mut TextPaint) {
      // Java: paint.setShader((Shader)null);
      // Java: paint.setMaskFilter((MaskFilter)null);
      // Java: paint.setStyle(Style.STROKE);
      // Java: paint.setAntiAlias(true);
      // Java: this.applyLineClr(paint);
      // Java: this.applyStrokeWid(paint);
      // Java: this.applyStrokeJoin(paint);
      // Java: this.applyPathEffect(paint, this.mActualWid_pt);
    }

    // Java: void applyLineClr(TextPaint paint)
    pub fn applyLineClr(&self, _paint: &mut TextPaint) {
      // Java: if (this.mShdwClr == 0)
      if self.mShdwClr == 0
      {
         // Java: if (this.mTextOutline.getTxtOutlnType() == TextOutline.TxtOutlnType.GRAD_LINE && this.mLineWid > 0.0F && this.mLineHei > 0.0F)
         if self.mTextOutline.getTxtOutlnType() == TxtOutlnType::GRAD_LINE && self.mLineWid > 0.0 && self.mLineHei > 0.0
         {
            // Java: GradPaintSetter gradSetter = new GradPaintSetter(this.mLineWid, this.mLineHei, this.mLineLeft);
            // Java: gradSetter.applyGradFill(paint, this.mTextOutline.getGradAttr(), this.mLineTop);
         }
         // Java: else if (this.mTextOutline.getTxtOutlnType() == TextOutline.TxtOutlnType.SOLID_LINE)
         else if self.mTextOutline.getTxtOutlnType() == TxtOutlnType::SOLID_LINE
         {
            // Java: paint.setColor(this.mTextOutline.getLineClr());
         }
         // Java: else if (this.mTextOutline.getTxtOutlnType() == TextOutline.TxtOutlnType.PATTERN_LINE)
         else if self.mTextOutline.getTxtOutlnType() == TxtOutlnType::PATTERN_LINE
         {
            // Java: PatternPaintSetter patternPaintSetter = new PatternPaintSetter(this.mTextOutline.getPatternAttr());
            // Java: patternPaintSetter.applyPatternFill(paint, this.canvasMatrix);
         }
      }
      // Java: else
      else
      {
         // Java: paint.setColor(this.mShdwClr);
      }
    }

    // Java: void applyStrokeWid(TextPaint paint)
    pub fn applyStrokeWid(&self, _paint: &mut TextPaint) {
      // Java: paint.setStrokeWidth(this.mActualWid_pt);
    }

    // Java: void applyStrokeJoin(TextPaint paint)
    pub fn applyStrokeJoin(&self, _paint: &mut TextPaint) {
      // Java: switch (this.mTextOutline.getJoinType())
      match self.mTextOutline.getJoinType()
      {
         // Java: case 0:
         0 => {
            // Java: paint.setStrokeJoin(Join.BEVEL);
            // Java: break;
         }
         // Java: case 1:
         1 => {
            // Java: paint.setStrokeJoin(Join.MITER);
            // Java: break;
         }
         // Java: case 2:
         2 => {
            // Java: paint.setStrokeJoin(Join.ROUND);
         }
         _ => {}
      }
    }

    // Java: void applyPathEffect(TextPaint paint, float wid)
    pub fn applyPathEffect(&self, _paint: &mut TextPaint, wid: f32) {
      // Java: PathEffect dashEffect = this.getDashPathEffect(wid);
      let dashEffect = self.getDashPathEffect(wid);
      // Java: if (this.mTextOutline.getComplexType() != 0)
      if self.mTextOutline.getComplexType() != 0
      {
         // Java: PathEffect complexEffect = this.getComplexEffect(dashEffect, wid);
         let _complexEffect = self.getComplexEffect(dashEffect, wid);
         // Java: paint.setPathEffect(complexEffect);
      }
      // Java: else
      else
      {
         // Java: paint.setPathEffect(dashEffect);
         let _ = dashEffect;
      }
    }

    // Java: PathEffect getDashPathEffect(float wid)
    pub fn getDashPathEffect(&self, wid: f32) -> Option<Vec<f32>> {
      // Java: PathEffect effect = null;
      let effect: Option<Vec<f32>>;
      // Java: switch (this.mTextOutline.getLineType())
      match self.mTextOutline.getLineType()
      {
         // Java: case 0:
         // Java: case 3:
         // Java: case 4:
         // Java: case 5:
         // Java: default:
         0 | 3 | 4 | 5 => {
            // Java: break;
            effect = None;
         }
         // Java: case 1:
         1 => {
            // Java: effect = new DashPathEffect(new float[]{wid * 2.0F, wid, wid * 2.0F, wid}, 1.0F);
            effect = Some(vec![wid * 2.0, wid, wid * 2.0, wid]);
            // Java: break;
         }
         // Java: case 2:
         2 => {
            // Java: effect = new DashPathEffect(new float[]{wid, wid, wid, wid}, 1.0F);
            effect = Some(vec![wid, wid, wid, wid]);
            // Java: break;
         }
         // Java: case 6:
         6 => {
            // Java: effect = new DashPathEffect(new float[]{wid * 4.0F, wid * 2.0F, wid * 4.0F, wid * 2.0F}, 1.0F);
            effect = Some(vec![wid * 4.0, wid * 2.0, wid * 4.0, wid * 2.0]);
            // Java: break;
         }
         // Java: case 7:
         7 => {
            // Java: effect = new DashPathEffect(new float[]{wid * 8.0F, wid * 2.0F, wid * 8.0F, wid * 2.0F}, 1.0F);
            effect = Some(vec![wid * 8.0, wid * 2.0, wid * 8.0, wid * 2.0]);
            // Java: break;
         }
         // Java: case 8:
         8 => {
            // Java: effect = new DashPathEffect(new float[]{wid * 4.0F, wid * 2.0F, wid, wid * 2.0F}, 1.0F);
            effect = Some(vec![wid * 4.0, wid * 2.0, wid, wid * 2.0]);
            // Java: break;
         }
         // Java: case 9:
         9 => {
            // Java: effect = new DashPathEffect(new float[]{wid * 8.0F, wid * 2.0F, wid, wid * 2.0F}, 1.0F);
            effect = Some(vec![wid * 8.0, wid * 2.0, wid, wid * 2.0]);
            // Java: break;
         }
         // Java: case 10:
         10 => {
            // Java: effect = new DashPathEffect(new float[]{wid * 8.0F, wid * 2.0F, wid, wid * 2.0F, wid, wid * 2.0F}, 1.0F);
            effect = Some(vec![wid * 8.0, wid * 2.0, wid, wid * 2.0, wid, wid * 2.0]);
         }
         _ => {
            effect = None;
         }
      }
      // Java: return effect;
      effect
    }

    // Java: PathEffect getComplexEffect(PathEffect dashEffect, float width)
    pub fn getComplexEffect(&self, dashEffect: Option<Vec<f32>>, width: f32) -> Option<Vec<f32>> {
      // Java: PathEffect effect = null;
      let mut effect: Option<Vec<f32>> = None;
      // Java: float[] vertDash = getVertDash(this.mTextOutline.getComplexType());
      let vertDash = Self::getVertDash(self.mTextOutline.getComplexType());
      // Java: if (vertDash != null && vertDash.length > 2)
      if let Some(mut vd) = vertDash {
        if vd.len() > 2
        {
         // Java: mulWidth(vertDash, width);
         Self::mulWidth(&mut vd, width);
         // Java: Path path = new Path();
         // Java: int row = vertDash.length / 2;
         let row = vd.len() / 2;
         // Java: RectF rectF = new RectF(0.0F, -(width / 2.0F), width, 0.0F);
         let mut rectF_top = -(width / 2.0);
         let mut rectF_bottom;
         // Java: for(int i = 0; i < row; ++i)
         for i in 0..row
         {
            // Java: rectF.bottom = rectF.top + vertDash[2 * i];
            rectF_bottom = rectF_top + vd[2 * i];
            // Java: path.addRect(rectF.left, rectF.top, rectF.right, rectF.bottom, Direction.CW);
            // Java: rectF.top = rectF.bottom + vertDash[2 * i + 1];
            rectF_top = rectF_bottom + vd[2 * i + 1];
         }
         // Java: effect = new PathDashPathEffect(path, width, 0.0F, android.graphics.PathDashPathEffect.Style.MORPH);
         effect = Some(vec![width]);
         // Java: if (dashEffect != null)
         if dashEffect.is_some()
         {
            // Java: effect = new ComposePathEffect(effect, dashEffect);
         }
        }
      }
      // Java: return effect;
      effect
    }

    // Java: static float[] getVertDash(int style)
    pub fn getVertDash(style: i32) -> Option<Vec<f32>> {
      // Java: float[] dash = null;
      let dash: Option<Vec<f32>>;
      // Java: switch (style)
      match style
      {
         // Java: case 0:
         // Java: default:
         0 => {
            // Java: break;
            dash = None;
         }
         // Java: case 1:
         1 => {
            // Java: dash = new float[]{0.33F, 0.34F, 0.33F, 0.0F};
            dash = Some(vec![0.33, 0.34, 0.33, 0.0]);
            // Java: break;
         }
         // Java: case 2:
         2 => {
            // Java: dash = new float[]{0.6F, 0.2F, 0.2F, 0.0F};
            dash = Some(vec![0.6, 0.2, 0.2, 0.0]);
            // Java: break;
         }
         // Java: case 3:
         3 => {
            // Java: dash = new float[]{0.2F, 0.2F, 0.6F, 0.0F};
            dash = Some(vec![0.2, 0.2, 0.6, 0.0]);
            // Java: break;
         }
         // Java: case 4:
         4 => {
            // Java: dash = new float[]{0.165F, 0.165F, 0.34F, 0.165F, 0.165F, 0.0F};
            dash = Some(vec![0.165, 0.165, 0.34, 0.165, 0.165, 0.0]);
         }
         _ => {
            dash = None;
         }
      }
      // Java: return dash;
      dash
    }

    // Java: static void mulWidth(float[] dash, float width)
    pub fn mulWidth(dash: &mut Vec<f32>, width: f32) {
      // Java: if (dash != null)
      {
         // Java: for(int i = 0; i < dash.length; ++i)
         for i in 0..dash.len()
         {
            // Java: dash[i] *= width;
            dash[i] *= width;
         }
      }
    }

    // Java: void applyOutlnBlur(TextPaint paint)
    pub fn applyOutlnBlur(&self, _paint: &mut TextPaint) {
      // Java: float shdwRad = this.mBlurRad;
      let mut shdwRad = self.mBlurRad;
      // Java: if (!this.mIsYCompress)
      if !self.mIsYCompress
      {
         // Java: shdwRad /= 2.0F;
         shdwRad /= 2.0;
      }
      // Java: BlurMaskFilter blurFilter = new BlurMaskFilter(shdwRad, Blur.NORMAL);
      let _blurFilter = shdwRad;
      // Java: paint.setMaskFilter(blurFilter);
    }

    // Java: float getActualWid_pt()
    pub fn getActualWid_pt(&self) -> f32 {
      // Java: return this.mActualWid_pt;
      self.mActualWid_pt
    }

    // Java: void setCanvasMatrix(Matrix canvasMatrix)
    pub fn setCanvasMatrix(&mut self, canvasMatrix: Matrix) {
      // Java: this.canvasMatrix = canvasMatrix;
      self.canvasMatrix = Some(canvasMatrix);
    }

}

impl CharacterStyleTrait for WpsTextOutlineSpan {
    fn updateDrawState(&self, paint: &mut TextPaint) {
      self.updateDrawState(paint);
    }

    fn getUnderlying(&self) -> &dyn CharacterStyleTrait {
        self
    }
}
