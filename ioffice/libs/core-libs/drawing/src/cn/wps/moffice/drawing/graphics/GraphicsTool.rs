#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use std::any::Any;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class GraphicsTool
pub struct GraphicsTool {
}

// Java: public class GraphicsTool
impl GraphicsTool {
    // Java: GraphicsTool()
    pub fn new() -> Self {
      // Java: super();
      GraphicsTool {}
    }

    // Java: static void tranLineProperty(Paint linePaint, LineProperty strokeStyle, float strokeWidth)
    pub fn tranLineProperty(linePaint: &mut Element, strokeStyle: &Element, strokeWidth: f32) {
      // Java: int clr = strokeStyle.getColor();
      // TODO: Android Paint API - stub
      // Java: if (strokeStyle.getType() == 1)
      // TODO: Android Paint API - stub
      {
         // Java: FillBase fill = strokeStyle.getFill();
         // TODO: Android Paint API - stub
         // Java: if (fill != null)
         // TODO: Android Paint API - stub
         {
            // Java: clr = fill.getColor();
            // TODO: Android Paint API - stub
         }
      }
      // Java: int color = ColorUtil.rgb2Argb(clr);
      // TODO: Android Paint API - stub
      // Java: linePaint.setColor(color);
      // TODO: Android Paint API - stub
      // Java: linePaint.setAlpha((int)((1.0F - strokeStyle.getTransparency()) * 255.0F));
      // TODO: Android Paint API - stub
      // Java: linePaint.setStrokeWidth(strokeWidth);
      // TODO: Android Paint API - stub
      // Java: linePaint.setStyle(Style.STROKE);
      // TODO: Android Paint API - stub
      // Java: linePaint.setStrokeJoin(getJoin(strokeStyle.getJoinStyle()));
      // TODO: Android Paint API - stub
      // Java: linePaint.setStrokeCap(getCap(strokeStyle.getEndCapStyle()));
      // TODO: Android Paint API - stub
      // Java: PathEffect effect = getPathEffect(strokeStyle, strokeWidth, linePaint.getStrokeCap() == Cap.ROUND);
      // TODO: Android Paint API - stub
      // Java: linePaint.setPathEffect(effect);
      // TODO: Android Paint API - stub
    }

    // Java: static PathEffect getPathEffect(LineProperty strokeStyle, float strokeWidth, boolean isRoundCap)
    pub fn getPathEffect(strokeStyle: &Element, strokeWidth: f32, isRoundCap: bool) -> Element {
      // Java: int dashing = strokeStyle.getDashing();
      // TODO: Android Path/PathEffect API - stub
      // Java: Integer endCap = strokeStyle.getEndCapStyleObj();
      // TODO: Android Path/PathEffect API - stub
      // Java: if ((dashing == 2 || dashing == 5) && endCap != null && endCap == 0)
      // TODO: Android Path/PathEffect API - stub
      {
         // Java: Path path = new Path();
         // TODO: Android Path/PathEffect API - stub
         // Java: path.addCircle(0.0F, 0.0F, strokeWidth / 2.0F, Direction.CW);
         // TODO: Android Path/PathEffect API - stub
         // Java: return new PathDashPathEffect(path, strokeWidth * 2.0F, 0.0F, android.graphics.PathDashPathEffect.Style.ROTATE);
         // TODO: Android Path/PathEffect API - stub
      }
      // Java: else
      // TODO: Android Path/PathEffect API - stub
      {
         // Java: PathEffect effect = null;
         // TODO: Android Path/PathEffect API - stub
         // Java: int style = strokeStyle.getLineStyle();
         // TODO: Android Path/PathEffect API - stub
         // Java: float[] dash = getDash(dashing, strokeStyle, isRoundCap);
         // TODO: Android Path/PathEffect API - stub
         // Java: mulWidth(dash, strokeWidth);
         // TODO: Android Path/PathEffect API - stub
         // Java: if (style == 0)
         // TODO: Android Path/PathEffect API - stub
         {
            // Java: effect = dash != null ? new DashPathEffect(dash, 0.0F) : null;
            // TODO: Android Path/PathEffect API - stub
         }
         // Java: else
         // TODO: Android Path/PathEffect API - stub
         {
            // Java: float[] vertDash = getVertDash(style);
            // TODO: Android Path/PathEffect API - stub
            // Java: if (vertDash != null && vertDash.length > 2)
            // TODO: Android Path/PathEffect API - stub
            {
               // Java: mulWidth(vertDash, strokeWidth);
               // TODO: Android Path/PathEffect API - stub
               // Java: effect = getDashEffect(dash, vertDash, strokeWidth);
               // TODO: Android Path/PathEffect API - stub
            }
         }
         // Java: return effect;
         None
      }
    }

    // Java: static void mulWidth(float[] dash, float width)
    pub fn mulWidth(dash: &mut Option<Vec<f32>>, width: f32) {
      // Java: if (dash != null)
      if let Some(ref mut d) = dash
      {
         // Java: for(int i = 0; i < dash.length; ++i)
         for i in 0..d.len()
         {
            // Java: dash[i] *= width;
            d[i] *= width;
         }
      }
    }

    // Java: static PathEffect getDashEffect(float[] dash, float[] vertDash, float width)
    pub fn getDashEffect(dash: &Option<Vec<f32>>, vertDash: &Vec<f32>, width: f32) -> Element {
      // Java: Path path = new Path();
      // TODO: Android Path/PathEffect API - stub
      // Java: int row = vertDash.length / 2;
      let row = vertDash.len() / 2;
      // Java: RectF rectF = new RectF(0.0F, -(width / 2.0F), width, 0.0F);
      // TODO: Android Path/PathEffect API - stub
      // Java: for(int i = 0; i < row; ++i)
      // TODO: Android Path/PathEffect API - stub
      {
         // Java: rectF.bottom = rectF.top + vertDash[2 * i];
         // TODO: Android Path/PathEffect API - stub
         // Java: path.addRect(rectF.left, rectF.top, rectF.right, rectF.bottom, Direction.CW);
         // TODO: Android Path/PathEffect API - stub
         // Java: rectF.top = rectF.bottom + vertDash[2 * i + 1];
         // TODO: Android Path/PathEffect API - stub
      }
      // Java: PathEffect effect = new PathDashPathEffect(path, width, 0.0F, android.graphics.PathDashPathEffect.Style.MORPH);
      // TODO: Android Path/PathEffect API - stub
      // Java: if (dash != null && dash.length > 1)
      // TODO: Android Path/PathEffect API - stub
      {
         // Java: PathEffect dashEffect = new DashPathEffect(dash, 0.0F);
         // TODO: Android Path/PathEffect API - stub
         // Java: effect = new ComposePathEffect(effect, dashEffect);
         // TODO: Android Path/PathEffect API - stub
      }
      // Java: return effect;
      None
    }

    // Java: static float getDashLength(float[] dash)
    pub fn getDashLength(dash: &Option<Vec<f32>>) -> f32 {
      // Java: if (dash == null)
      if dash.is_none()
      {
         // Java: return 1.0F;
         return 1.0_f32;
      }
      // Java: else
      else
      {
         // Java: float len = 0.0F;
         let mut len = 0.0_f32;
         // Java: for(int i = 0; i < dash.length; ++i)
         let d = dash.as_ref().unwrap();
         for i in 0..d.len()
         {
            // Java: len += dash[i];
            len += d[i];
         }
         // Java: return len;
         return len;
      }
    }

    // Java: static float[] getVertDash(int style)
    pub fn getVertDash(style: i32) -> Option<Vec<f32>> {
      // Java: float[] dash = null;
      let mut dash: Option<Vec<f32>> = None;
      // Java: switch (style)
      match style
      {
         // Java: case 0:
         // Java: default:
         0 | _ if style != 1 && style != 2 && style != 3 && style != 4 => {
            // Java: break;
         }
         // Java: case 1:
         1 => {
            // Java: dash = new float[]{0.33F, 0.34F, 0.33F, 0.0F};
            dash = Some(vec![0.33_f32, 0.34_f32, 0.33_f32, 0.0_f32]);
            // Java: break;
         }
         // Java: case 2:
         2 => {
            // Java: dash = new float[]{0.6F, 0.2F, 0.2F, 0.0F};
            dash = Some(vec![0.6_f32, 0.2_f32, 0.2_f32, 0.0_f32]);
            // Java: break;
         }
         // Java: case 3:
         3 => {
            // Java: dash = new float[]{0.2F, 0.2F, 0.6F, 0.0F};
            dash = Some(vec![0.2_f32, 0.2_f32, 0.6_f32, 0.0_f32]);
            // Java: break;
         }
         // Java: case 4:
         4 => {
            // Java: dash = new float[]{0.165F, 0.165F, 0.34F, 0.165F, 0.165F, 0.0F};
            dash = Some(vec![0.165_f32, 0.165_f32, 0.34_f32, 0.165_f32, 0.165_f32, 0.0_f32]);
         }
         _ => {}
      }
      // Java: return dash;
      dash
    }

    // Java: static float[] getDash(int dashing, LineProperty strokeStyle)
    pub fn getDash_two(dashing: i32, strokeStyle: &Element) -> Option<Vec<f32>> {
      // Java: return getDash(dashing, strokeStyle, false);
      Self::getDash(dashing, strokeStyle, false)
    }

    // Java: static float[] getDash(int dashing, LineProperty strokeStyle, boolean isRoundCap)
    pub fn getDash(dashing: i32, strokeStyle: &Element, isRoundCap: bool) -> Option<Vec<f32>> {
      // Java: float[] dash = null;
      let dash: Option<Vec<f32>>;
      // Java: switch (dashing)
      match dashing
      {
         // Java: case 0:
         0 => {
            // Java: float[] dashStyle = strokeStyle.getDashStyle();
            // TODO: requires LineProperty.getDashStyle()
            let dashStyle: Option<Vec<f32>> = None;
            // Java: dash = getDashAdjust(dashStyle);
            dash = Self::getDashAdjust(&dashStyle);
            // Java: break;
         }
         // Java: case 1:
         1 => {
            // Java: dash = isRoundCap ? new float[]{2.0F, 2.0F} : new float[]{3.0F, 1.0F};
            dash = if isRoundCap { Some(vec![2.0_f32, 2.0_f32]) } else { Some(vec![3.0_f32, 1.0_f32]) };
            // Java: break;
         }
         // Java: case 2:
         // Java: case 5:
         2 | 5 => {
            // Java: dash = new float[]{1.0F, 1.0F};
            dash = Some(vec![1.0_f32, 1.0_f32]);
            // Java: break;
         }
         // Java: case 3:
         // Java: case 8:
         3 | 8 => {
            // Java: dash = isRoundCap ? new float[]{4.0F, 3.55F, 0.1F, 3.55F} : new float[]{5.0F, 2.5F, 1.0F, 2.5F};
            dash = if isRoundCap { Some(vec![4.0_f32, 3.55_f32, 0.1_f32, 3.55_f32]) } else { Some(vec![5.0_f32, 2.5_f32, 1.0_f32, 2.5_f32]) };
            // Java: break;
         }
         // Java: case 4:
         4 => {
            // Java: dash = isRoundCap ? new float[]{4.0F, 3.55F, 0.075F, 3.55F, 0.075F, 3.55F} : new float[]{5.0F, 2.5F, 1.0F, 2.5F, 1.0F, 2.5F};
            dash = if isRoundCap { Some(vec![4.0_f32, 3.55_f32, 0.075_f32, 3.55_f32, 0.075_f32, 3.55_f32]) } else { Some(vec![5.0_f32, 2.5_f32, 1.0_f32, 2.5_f32, 1.0_f32, 2.5_f32]) };
            // Java: break;
         }
         // Java: case 6:
         6 => {
            // Java: dash = isRoundCap ? new float[]{3.8F, 3.2F} : new float[]{4.5F, 2.5F};
            dash = if isRoundCap { Some(vec![3.8_f32, 3.2_f32]) } else { Some(vec![4.5_f32, 2.5_f32]) };
            // Java: break;
         }
         // Java: case 7:
         7 => {
            // Java: dash = isRoundCap ? new float[]{7.0F, 4.0F} : new float[]{8.0F, 3.0F};
            dash = if isRoundCap { Some(vec![7.0_f32, 4.0_f32]) } else { Some(vec![8.0_f32, 3.0_f32]) };
            // Java: break;
         }
         // Java: case 9:
         9 => {
            // Java: dash = isRoundCap ? new float[]{7.0F, 4.05F, 0.1F, 4.05F} : new float[]{8.0F, 3.0F, 1.0F, 3.0F};
            dash = if isRoundCap { Some(vec![7.0_f32, 4.05_f32, 0.1_f32, 4.05_f32]) } else { Some(vec![8.0_f32, 3.0_f32, 1.0_f32, 3.0_f32]) };
            // Java: break;
         }
         // Java: case 10:
         10 => {
            // Java: dash = isRoundCap ? new float[]{7.0F, 4.05F, 0.075F, 4.05F, 0.075F, 4.05F} : new float[]{8.0F, 3.0F, 1.0F, 3.0F, 1.0F, 3.0F};
            dash = if isRoundCap { Some(vec![7.0_f32, 4.05_f32, 0.075_f32, 4.05_f32, 0.075_f32, 4.05_f32]) } else { Some(vec![8.0_f32, 3.0_f32, 1.0_f32, 3.0_f32, 1.0_f32, 3.0_f32]) };
            // Java: break;
         }
         // Java: case 11:
         11 => {
            // Java: dash = strokeStyle.getDashStyle();
            // TODO: requires LineProperty.getDashStyle()
            dash = None;
            // Java: break;
         }
         // Java: default:
         _ => {
            // Java: dash = null;
            dash = None;
         }
      }
      // Java: return dash;
      dash
    }

    // Java: static float[] getDashAdjust(float[] dashStyle)
    pub fn getDashAdjust(dashStyle: &Option<Vec<f32>>) -> Option<Vec<f32>> {
      // Java: float[] dash = null;
      let mut dash: Option<Vec<f32>> = None;
      // Java: if (dashStyle != null && dashStyle.length > 0)
      if let Some(ref ds) = dashStyle { if !ds.is_empty()
      {
         // Java: int size = dashStyle.length;
         let size = ds.len();
         // Java: dash = new float[size];
         let mut d = vec![0.0_f32; size];
         // Java: float maxValue = 0.0F;
         let mut maxValue = 0.0_f32;
         // Java: for(int i = 0; i < size; ++i)
         for i in 0..size
         {
            // Java: dash[i] = dashStyle[i];
            d[i] = ds[i];
            // Java: if (dashStyle[i] > 20.0F)
            if ds[i] > 20.0_f32
            {
               // Java: maxValue = Math.max(maxValue, dashStyle[i]);
               maxValue = maxValue.max(ds[i]);
            }
         }
         // Java: if (maxValue > 0.0F)
         if maxValue > 0.0_f32
         {
            // Java: for(int i = 0; i < size; ++i)
            for i in 0..size
            {
               // Java: dash[i] *= 20.0F / maxValue;
               d[i] *= 20.0_f32 / maxValue;
               // Java: dash[i] = Math.max(1.0F, dash[i]);
               d[i] = d[i].max(1.0_f32);
            }
         }
         dash = Some(d);
      }}
      // Java: return dash;
      dash
    }

    // Java: static void setGlow(Paint paint, Glow glow)
    pub fn setGlow(paint: &mut Element, glow: &Element) {
      // Java: if (glow != null && paint != null)
      // TODO: Android Paint/BlurMaskFilter API - stub
      {
         // Java: float size = (float)(glow.getGlowSize() / 2);
         // TODO: Android Paint/BlurMaskFilter API - stub
         // Java: if (!((double)size < 0.01))
         // TODO: Android Paint/BlurMaskFilter API - stub
         {
            // Java: int alpha = (int)(glow.getGlowOpacity() * 255.0F);
            // TODO: Android Paint/BlurMaskFilter API - stub
            // Java: int clr = 16777215 & glow.getGlowColor() | alpha << 24;
            // TODO: Android Paint/BlurMaskFilter API - stub
            // Java: BlurMaskFilter blur = new BlurMaskFilter(size, Blur.NORMAL);
            // TODO: Android Paint/BlurMaskFilter API - stub
            // Java: paint.setColor(clr);
            // TODO: Android Paint/BlurMaskFilter API - stub
            // Java: paint.setStrokeWidth(size);
            // TODO: Android Paint/BlurMaskFilter API - stub
            // Java: paint.setMaskFilter(blur);
            // TODO: Android Paint/BlurMaskFilter API - stub
         }
      }
    }

    // Java: static void clearGlow(Paint paint)
    pub fn clearGlow(paint: &mut Element) {
      // Java: if (paint != null)
      // TODO: Android Paint API - stub
      {
         // Java: paint.setMaskFilter((MaskFilter)null);
         // TODO: Android Paint API - stub
      }
    }

    // Java: static void setShadow(Paint paint, Shadow shadow)
    pub fn setShadow(paint: &mut Element, shadow: &Element) {
      // Java: if (shadow != null && paint != null)
      // TODO: Android Paint API - stub
      {
         // Java: float radius = shadow.getBlurRadius();
         // TODO: Android Paint API - stub
         // Java: float dx = shadow.getShadowOffsetX();
         // TODO: Android Paint API - stub
         // Java: float dy = shadow.getShadowOffsetY();
         // TODO: Android Paint API - stub
         // Java: int color = shadow.getShadowColor();
         // TODO: Android Paint API - stub
         // Java: paint.setShadowLayer(radius, dx, dy, color);
         // TODO: Android Paint API - stub
      }
    }

    // Java: static void clearShadow(Paint paint)
    pub fn clearShadow(paint: &mut Element) {
      // Java: if (paint != null)
      // TODO: Android Paint API - stub
      {
         // Java: paint.clearShadowLayer();
         // TODO: Android Paint API - stub
      }
    }

    // Java: static void drawArrow(Canvas canvas, Paint paint, Arrow arrow, float strokeWidth, Path arrowPath, Matrix matrix, float rot)
    pub fn drawArrow(canvas: &mut Element, paint: &mut Element, arrow: &Element, strokeWidth: f32, arrowPath: &mut Element, matrix: &mut Element, rot: f32) {
      // Java: int arrowType = arrow.getHead();
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: float arrowUnit = arrow.getLineLength(Math.max(3.0F, strokeWidth)) / 2.0F;
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: float arrowUnitY = arrow.getLineWidth(Math.max(3.0F, strokeWidth)) / 2.0F;
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: if (arrowUnit != arrowUnitY)
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      {
         // Java: matrix.preScale(1.0F, arrowUnitY / arrowUnit);
         // TODO: Android Canvas/Paint/Path/Matrix API - stub
      }
      // Java: float[] pts = null;
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: arrowPath.reset();
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: switch (arrowType)
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      {
         // Java: case 1:
         // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: pts = new float[]{1.0F, 0.0F, -arrowUnit * 2.0F, -arrowUnit, -arrowUnit * 2.0F, arrowUnit};
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: matrix.mapPoints(pts);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.moveTo(pts[0], pts[1]);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.lineTo(pts[2], pts[3]);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.lineTo(pts[4], pts[5]);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.close();
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: break;
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
         // Java: case 2:
         // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: pts = new float[]{1.0F, 0.0F, -arrowUnit * 2.0F, -arrowUnit, -arrowUnit, 0.0F, -arrowUnit * 2.0F, arrowUnit};
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: matrix.mapPoints(pts);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.moveTo(pts[0], pts[1]);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.lineTo(pts[2], pts[3]);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.lineTo(pts[4], pts[5]);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.lineTo(pts[6], pts[7]);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.close();
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: break;
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
         // Java: case 3:
         // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: pts = new float[]{arrowUnit, 0.0F, 0.0F, arrowUnit, -arrowUnit, 0.0F, 0.0F, -arrowUnit};
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: matrix.mapPoints(pts);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.moveTo(pts[0], pts[1]);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.lineTo(pts[2], pts[3]);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.lineTo(pts[4], pts[5]);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.lineTo(pts[6], pts[7]);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.close();
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: break;
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
         // Java: case 4:
         // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: if ((rot + 360.0F) % 90.0F < 1.0F)
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            {
               // Java: pts = new float[]{-arrowUnit, -arrowUnit, arrowUnit, arrowUnit};
               // TODO: Android Canvas/Paint/Path/Matrix API - stub
               // Java: matrix.mapPoints(pts);
               // TODO: Android Canvas/Paint/Path/Matrix API - stub
               // Java: arrowPath.addOval(new android.graphics.RectF(pts[0], pts[1], pts[2], pts[3]), Direction.CW);
               // TODO: Android Canvas/Paint/Path/Matrix API - stub
               // Java: arrowPath.close();
               // TODO: Android Canvas/Paint/Path/Matrix API - stub
            }
            // Java: else
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            {
               // Java: pts = new float[]{arrowUnit, 0.0F};
               // TODO: Android Canvas/Paint/Path/Matrix API - stub
               // Java: matrix.mapPoints(pts);
               // TODO: Android Canvas/Paint/Path/Matrix API - stub
               // Java: arrowPath.moveTo(pts[0], pts[1]);
               // TODO: Android Canvas/Paint/Path/Matrix API - stub
               // Java: addCircle(arrowUnit, 0.0F, arrowUnit, arrowUnit, 0.0F, 360.0F, arrowPath, matrix);
               // TODO: Android Canvas/Paint/Path/Matrix API - stub
               // Java: arrowPath.close();
               // TODO: Android Canvas/Paint/Path/Matrix API - stub
            }
            // Java: break;
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
         // Java: case 5:
         // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: float offset = 0.0F;
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: if (!arrow.needSpecialHanding())
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            {
               // Java: double radian = Math.atan2((double)arrowUnitY, (double)(2.0F * arrowUnit));
               // TODO: Android Canvas/Paint/Path/Matrix API - stub
               // Java: if (radian != (double)0.0F)
                // TODO: Android Canvas/Paint/Path/Matrix API - stub
               {
                  // Java: offset = (float)((double)(-strokeWidth) / ((double)2.0F * Math.sin(radian)));
                  // TODO: Android Canvas/Paint/Path/Matrix API - stub
                  // Java: matrix.preTranslate(offset, 0.0F);
                  // TODO: Android Canvas/Paint/Path/Matrix API - stub
               }
            }
            // Java: pts = new float[]{-arrowUnit * 2.0F, -arrowUnit, 1.0F, 0.0F, -arrowUnit * 2.0F, arrowUnit};
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: matrix.mapPoints(pts);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.moveTo(pts[0], pts[1]);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.lineTo(pts[2], pts[3]);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
            // Java: arrowPath.lineTo(pts[4], pts[5]);
            // TODO: Android Canvas/Paint/Path/Matrix API - stub
      }
      // Java: Paint.Style oldStyle = paint.getStyle();
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: Paint.Cap oldCap = paint.getStrokeCap();
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: Paint.Join oldJoin = paint.getStrokeJoin();
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: float oldWidth = paint.getStrokeWidth();
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: PathEffect pathEffect = paint.getPathEffect();
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: paint.setPathEffect((PathEffect)null);
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: if (arrowType == 5)
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      {
         // Java: paint.setStyle(Style.STROKE);
         // TODO: Android Canvas/Paint/Path/Matrix API - stub
         // Java: paint.setStrokeCap(Cap.ROUND);
         // TODO: Android Canvas/Paint/Path/Matrix API - stub
         // Java: paint.setStrokeJoin(Join.MITER);
         // TODO: Android Canvas/Paint/Path/Matrix API - stub
         // Java: paint.setStrokeWidth(strokeWidth);
         // TODO: Android Canvas/Paint/Path/Matrix API - stub
      }
      // Java: else
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      {
         // Java: paint.setStyle(Style.FILL);
         // TODO: Android Canvas/Paint/Path/Matrix API - stub
         // Java: paint.setStrokeCap(Cap.BUTT);
         // TODO: Android Canvas/Paint/Path/Matrix API - stub
         // Java: paint.setStrokeJoin(Join.BEVEL);
         // TODO: Android Canvas/Paint/Path/Matrix API - stub
      }
      // Java: canvas.drawPath(arrowPath, paint);
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: paint.setStyle(oldStyle);
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: paint.setStrokeCap(oldCap);
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: paint.setStrokeJoin(oldJoin);
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: paint.setStrokeWidth(oldWidth);
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
      // Java: paint.setPathEffect(pathEffect);
      // TODO: Android Canvas/Paint/Path/Matrix API - stub
    }

    // Java: static void addCircle(float x, float y, float wR, float hR, float stAng, float swAng, Path path, Matrix matrix)
    pub fn addCircle(x: f32, y: f32, wR: f32, hR: f32, stAng: f32, swAng: f32, path: &mut Element, matrix: &Element) {
      // Java: stAng = (360.0F - stAng) % 360.0F;
      let mut stAng = (360.0_f32 - stAng) % 360.0_f32;
      // Java: swAng = -swAng;
      let swAng = -swAng;
      // Java: float endAngleDeg = stAng + swAng;
      let endAngleDeg = stAng + swAng;
      // Java: double stAngleRad = (double)stAng * Math.PI / (double)180.0F;
      let stAngleRad = (stAng as f64) * std::f64::consts::PI / 180.0_f64;
      // Java: double endAngleRad = (double)endAngleDeg * Math.PI / (double)180.0F;
      let endAngleRad = (endAngleDeg as f64) * std::f64::consts::PI / 180.0_f64;
      // Java: double chgStAngleRad = Math.atan2(Math.sin(stAngleRad) * (double)wR, Math.cos(stAngleRad) * (double)hR);
      let mut chgStAngleRad = (stAngleRad.sin() * (wR as f64)).atan2(stAngleRad.cos() * (hR as f64));
      // Java: double chgEndAngleRad = Math.atan2(Math.sin(endAngleRad) * (double)wR, Math.cos(endAngleRad) * (double)hR);
      let mut chgEndAngleRad = (endAngleRad.sin() * (wR as f64)).atan2(endAngleRad.cos() * (hR as f64));
      // Java: if (stAng > 180.0F)
      if stAng > 180.0_f32
      {
         // Java: chgStAngleRad += (Math.PI * 2D);
         chgStAngleRad += std::f64::consts::PI * 2.0_f64;
      }
      // Java: else if (stAng < -180.0F)
      else if stAng < -180.0_f32
      {
         // Java: chgStAngleRad -= (Math.PI * 2D);
         chgStAngleRad -= std::f64::consts::PI * 2.0_f64;
      }
      // Java: if (0.0F == hR)
      if 0.0_f32 == hR
      {
         // Java: chgStAngleRad = stAngleRad;
         chgStAngleRad = stAngleRad;
      }
      // Java: if (endAngleDeg > 180.0F)
      if endAngleDeg > 180.0_f32
      {
         // Java: chgEndAngleRad += (Math.PI * 2D);
         chgEndAngleRad += std::f64::consts::PI * 2.0_f64;
      }
      // Java: else if (endAngleDeg < -180.0F)
      else if endAngleDeg < -180.0_f32
      {
         // Java: chgEndAngleRad -= (Math.PI * 2D);
         chgEndAngleRad -= std::f64::consts::PI * 2.0_f64;
      }
      // Java: if (0.0F == hR)
      if 0.0_f32 == hR
      {
         // Java: chgEndAngleRad = endAngleRad;
         chgEndAngleRad = endAngleRad;
      }
      // Java: double swingAngleRad = chgEndAngleRad - chgStAngleRad;
      let mut swingAngleRad = chgEndAngleRad - chgStAngleRad;
      // Java: if (Math.abs(swingAngleRad) > (Math.PI * 2D))
      if swingAngleRad.abs() > std::f64::consts::PI * 2.0_f64
      {
         // Java: swingAngleRad = (Math.PI * 2D);
         swingAngleRad = std::f64::consts::PI * 2.0_f64;
      }
      // Java: if (swAng < 0.0F && swingAngleRad > (double)0.0F)
      if swAng < 0.0_f32 && swingAngleRad > 0.0_f64
      {
         // Java: swingAngleRad -= (Math.PI * 2D);
         swingAngleRad -= std::f64::consts::PI * 2.0_f64;
      }
      // Java: if (swAng > 0.0F && swingAngleRad < (double)0.0F)
      if swAng > 0.0_f32 && swingAngleRad < 0.0_f64
      {
         // Java: swingAngleRad += (Math.PI * 2D);
         swingAngleRad += std::f64::consts::PI * 2.0_f64;
      }
      // Java: int segs = (int)Math.ceil(Math.abs(swingAngleRad) * (double)2.0F / Math.PI);
      let segs = (swingAngleRad.abs() * 2.0_f64 / std::f64::consts::PI).ceil() as i32;
      // Java: if (segs > 0)
      if segs > 0
      {
         // Java: double theta = -swingAngleRad / (double)segs;
         let theta = -swingAngleRad / (segs as f64);
         // Java: double x0 = Math.cos(theta / (double)2.0F);
         let x0 = (theta / 2.0_f64).cos();
         // Java: double y0 = Math.sin(theta / (double)2.0F);
         let y0 = (theta / 2.0_f64).sin();
         // Java: double angle = -chgStAngleRad;
         let mut angle = -chgStAngleRad;
         // Java: double cx = (double)x - Math.cos(angle) * (double)wR;
         let cx = (x as f64) - angle.cos() * (wR as f64);
         // Java: double cy = (double)y - Math.sin(angle) * (double)hR;
         let cy = (y as f64) - angle.sin() * (hR as f64);
         // Java: for(int i = 0; i < segs; ++i)
         for _i in 0..segs
         {
            // Java: angle += theta;
            angle += theta;
            // Java: double tx = ((double)1.0F - x0) * (double)4.0F / (double)3.0F;
            let tx = (1.0_f64 - x0) * 4.0_f64 / 3.0_f64;
            // Java: double ty = y0 - tx * x0 / y0;
            let ty = y0 - tx * x0 / y0;
            // Java: double angleMid = angle - theta / (double)2.0F;
            let angleMid = angle - theta / 2.0_f64;
            // Java: double sn = Math.sin(angleMid);
            let sn = angleMid.sin();
            // Java: double cs = Math.cos(angleMid);
            let cs = angleMid.cos();
            // Java: float ax = (float)(cx + (double)wR * (x0 * cs - y0 * sn));
            let _ax = (cx + (wR as f64) * (x0 * cs - y0 * sn)) as f32;
            // Java: float ay = (float)(cy + (double)hR * (x0 * sn + y0 * cs));
            let _ay = (cy + (hR as f64) * (x0 * sn + y0 * cs)) as f32;
            // Java: float cx1 = (float)(cx + (double)wR * ((x0 + tx) * cs + ty * sn));
            let _cx1 = (cx + (wR as f64) * ((x0 + tx) * cs + ty * sn)) as f32;
            // Java: float cy1 = (float)(cy + (double)hR * ((x0 + tx) * sn - ty * cs));
            let _cy1 = (cy + (hR as f64) * ((x0 + tx) * sn - ty * cs)) as f32;
            // Java: float cx2 = (float)(cx + (double)wR * ((x0 + tx) * cs - ty * sn));
            let _cx2 = (cx + (wR as f64) * ((x0 + tx) * cs - ty * sn)) as f32;
            // Java: float cy2 = (float)(cy + (double)hR * ((x0 + tx) * sn + ty * cs));
            let _cy2 = (cy + (hR as f64) * ((x0 + tx) * sn + ty * cs)) as f32;
            // Java: float[] pts = new float[]{cx1, cy1, cx2, cy2, ax, ay};
            // TODO: Android Path API - path.cubicTo requires Path object
            // Java: matrix.mapPoints(pts);
            // TODO: Android Matrix API - stub
            // Java: path.cubicTo(pts[0], pts[1], pts[2], pts[3], pts[4], pts[5]);
            // TODO: Android Path API - stub
         }
      }
    }

    // Java: static FillBase createFillForPath(FillBase sourceFill, GraphicsPath path)
    pub fn createFillForPath(sourceFill: Element, path: &Element) -> Element {
      // Java: int fillType = path.getFillType();
      // TODO: requires GraphicsPath and FillBase/SolidFill types
      // Java: if (sourceFill != null && fillType != 4 && sourceFill.getFillOn())
      // TODO: requires GraphicsPath and FillBase/SolidFill types
      {
         // Java: int mainColor = FillBase.getMainColor(sourceFill);
         // TODO: requires FillBase type
         // Java: if (fillType == 0)
         // TODO: requires FillBase type
         {
            // Java: return new SolidFill(getDarkenColor(mainColor));
            // TODO: requires SolidFill type
         }
         // Java: else if (fillType == 1)
         // TODO: requires FillBase type
         {
            // Java: return new SolidFill(getDarkenlessColor(mainColor));
            // TODO: requires SolidFill type
         }
         // Java: else if (fillType == 2)
         // TODO: requires FillBase type
         {
            // Java: return new SolidFill(getLightenColor(mainColor));
            // TODO: requires SolidFill type
         }
         // Java: else if (fillType == 3)
         // TODO: requires FillBase type
         {
            // Java: return new SolidFill(getLightenlessColor(mainColor));
            // TODO: requires SolidFill type
         }
         // Java: else
         // TODO: requires FillBase type
         {
            // Java: return fillType == 5 ? sourceFill : null;
            // TODO: requires FillBase type
         }
      }
      // Java: else
      // TODO: stub
      {
         // Java: return null;
         None
      }
    }

    // Java: static int getLightenlessColor(int color)
    pub fn getLightenlessColor(color: i32) -> i32 {
      // Java: return getLightenColor(color, 210);
      Self::getLightenColor_with_dd(color, 210)
    }

    // Java: static int getLightenColor(int color)
    pub fn getLightenColor(color: i32) -> i32 {
      // Java: return getLightenColor(color, 180);
      Self::getLightenColor_with_dd(color, 180)
    }

    // Java: static int getDarkenlessColor(int color)
    pub fn getDarkenlessColor(color: i32) -> i32 {
      // Java: return getDarkenColor(color, 210);
      Self::getDarkenColor_with_dd(color, 210)
    }

    // Java: static int getDarkenColor(int color)
    pub fn getDarkenColor(color: i32) -> i32 {
      // Java: return getDarkenColor(color, 180);
      Self::getDarkenColor_with_dd(color, 180)
    }

    // Java: static int getLightenColor(int color, int dd)
    pub fn getLightenColor_with_dd(color: i32, dd: i32) -> i32 {
      // Java: int red = color >> 16 & 255;
      let mut red = (color >> 16) & 255;
      // Java: int green = color >> 8 & 255;
      let mut green = (color >> 8) & 255;
      // Java: int blue = color & 255;
      let mut blue = color & 255;
      // Java: float scale = (float)dd / 255.0F;
      let scale = (dd as f32) / 255.0_f32;
      // Java: red = (int)(255.0F - (float)(255 - red) * scale);
      red = (255.0_f32 - ((255 - red) as f32) * scale) as i32;
      // Java: green = (int)(255.0F - (float)(255 - green) * scale);
      green = (255.0_f32 - ((255 - green) as f32) * scale) as i32;
      // Java: blue = (int)(255.0F - (float)(255 - blue) * scale);
      blue = (255.0_f32 - ((255 - blue) as f32) * scale) as i32;
      // Java: return color & -16777216 | red << 16 | green << 8 | blue;
      (color & -16777216_i32) | (red << 16) | (green << 8) | blue
    }

    // Java: static int getDarkenColor(int color, int dd)
    pub fn getDarkenColor_with_dd(color: i32, dd: i32) -> i32 {
      // Java: int red = color >> 16 & 255;
      let mut red = (color >> 16) & 255;
      // Java: int green = color >> 8 & 255;
      let mut green = (color >> 8) & 255;
      // Java: int blue = color & 255;
      let mut blue = color & 255;
      // Java: float scale = (float)dd / 255.0F;
      let scale = (dd as f32) / 255.0_f32;
      // Java: red = (int)((float)red * scale);
      red = ((red as f32) * scale) as i32;
      // Java: green = (int)((float)green * scale);
      green = ((green as f32) * scale) as i32;
      // Java: blue = (int)((float)blue * scale);
      blue = ((blue as f32) * scale) as i32;
      // Java: return color & -16777216 | red << 16 | green << 8 | blue;
      (color & -16777216_i32) | (red << 16) | (green << 8) | blue
    }

}
