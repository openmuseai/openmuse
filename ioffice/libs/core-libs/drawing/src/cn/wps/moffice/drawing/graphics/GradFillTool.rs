#![allow(non_snake_case, dead_code, unused_variables, unused_mut, unused_imports)]

// Java: import android.graphics.Canvas
// LLM No Rust equivalent: android.graphics.Canvas
// Java: import android.graphics.LinearGradient
// LLM No Rust equivalent: android.graphics.LinearGradient
// Java: import android.graphics.Matrix
// LLM No Rust equivalent: android.graphics.Matrix
// Java: import android.graphics.Path
// LLM No Rust equivalent: android.graphics.Path
// Java: import android.graphics.RadialGradient
// LLM No Rust equivalent: android.graphics.RadialGradient
// Java: import android.graphics.RectF
// Java: import android.graphics.Shader
// LLM No Rust equivalent: android.graphics.Shader
// Java: import android.graphics.Shader.TileMode
// LLM No Rust equivalent: android.graphics.Shader.TileMode
// Java: import cn.wps.moffice.drawing.ShapeHelper
// LLM: getRotationRect implemented locally as ShapeHelper uses incompatible local RectF

use core_libs_util::graphics::RectF::RectF;
use super::GradFill::GradFill;

use std::sync::Arc;
use std::any::Any;

struct GradStopListAdapter {
    stops: Vec<(i32, f32)>,
}

impl GradStopListAdapter {
    fn from_element(element: &Element) -> Option<Self> {
        element.as_ref().and_then(|arc| {
            arc.downcast_ref::<super::GradStopList::GradStopList>().map(|gsl| {
                let mut stops = Vec::new();
                for i in 0..gsl.size() {
                    if let Some(s) = gsl.at(i) {
                        stops.push((0i32, s.getPostion()));
                    }
                }
                GradStopListAdapter { stops }
            }).or_else(|| {
                arc.downcast_ref::<Vec<(i32, f32)>>().map(|v| {
                    GradStopListAdapter { stops: v.clone() }
                })
            })
        })
    }

    fn size(&self) -> i32 { self.stops.len() as i32 }
    fn get_color(&self, i: usize) -> i32 { self.stops.get(i).map(|s| s.0).unwrap_or(0) }
    fn get_position(&self, i: usize) -> f32 { self.stops.get(i).map(|s| s.1).unwrap_or(0.0) }
}

type Element = Option<Arc<dyn Any + Send + Sync>>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileMode { CLAMP, MIRROR, REPEAT }

pub struct Shader {
    pub local_matrix: Option<Matrix>,
}

impl Shader {
    pub fn new() -> Self { Shader { local_matrix: None } }
    pub fn setLocalMatrix(&mut self, matrix: Matrix) { self.local_matrix = Some(matrix); }
}

pub struct LinearGradient {
    pub base: Shader,
    pub x0: f32, pub y0: f32, pub x1: f32, pub y1: f32,
    pub colors: Vec<i32>, pub positions: Vec<f32>,
    pub tile_mode: TileMode,
}

impl LinearGradient {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32, colors: &[i32], positions: &[f32], tile_mode: TileMode) -> Self {
        LinearGradient {
            base: Shader::new(),
            x0, y0, x1, y1,
            colors: colors.to_vec(),
            positions: positions.to_vec(),
            tile_mode,
        }
    }
    pub fn as_shader(self) -> Shader { self.base }
}

pub struct RadialGradient {
    pub base: Shader,
    pub cx: f32, pub cy: f32, pub radius: f32,
    pub colors: Vec<i32>, pub positions: Vec<f32>,
    pub tile_mode: TileMode,
}

impl RadialGradient {
    pub fn new(cx: f32, cy: f32, radius: f32, colors: &[i32], positions: &[f32], tile_mode: TileMode) -> Self {
        RadialGradient {
            base: Shader::new(),
            cx, cy, radius,
            colors: colors.to_vec(),
            positions: positions.to_vec(),
            tile_mode,
        }
    }
    pub fn as_shader(self) -> Shader { self.base }
}

#[derive(Clone)]
pub struct Matrix { values: [f32; 9] }
impl Matrix {
    pub fn new() -> Self { Matrix { values: [1.0,0.0,0.0, 0.0,1.0,0.0, 0.0,0.0,1.0] } }
    pub fn setRotate(&mut self, degrees: f32, px: f32, py: f32) {
        let rad = degrees.to_radians();
        let cos = rad.cos();
        let sin = rad.sin();
        self.values[0] = cos; self.values[1] = -sin; self.values[2] = px - px*cos + py*sin;
        self.values[3] = sin; self.values[4] = cos;  self.values[5] = py - px*sin - py*cos;
    }
}

pub struct Path { points: Vec<(f32, f32)>, closed: bool }
impl Path {
    pub fn new() -> Self { Path { points: Vec::new(), closed: false } }
    pub fn moveTo(&mut self, x: f32, y: f32) { self.points.clear(); self.points.push((x, y)); self.closed = false; }
    pub fn lineTo(&mut self, x: f32, y: f32) { self.points.push((x, y)); }
    pub fn close(&mut self) { self.closed = true; }
    pub fn reset(&mut self) { self.points.clear(); self.closed = false; }
}

pub struct Paint {
    shader: Option<Shader>,
    color: i32,
}
impl Paint {
    pub fn new() -> Self { Paint { shader: None, color: 0 } }
    pub fn setShader(&mut self, shader: Option<Shader>) { self.shader = shader; }
    pub fn setColor(&mut self, color: i32) { self.color = color; }
}

pub struct Canvas;
impl Canvas {
    pub fn save(&self) {}
    pub fn restore(&self) {}
    pub fn clipRect(&self, _rect: &RectF) {}
    pub fn clipPath(&self, _path: &Path) {}
    pub fn drawRect_with_paint(&self, _left: f32, _top: f32, _right: f32, _bottom: f32, _paint: &Paint) {}
    pub fn drawRect(&self, _rect: &RectF, _paint: &Paint) {}
    pub fn drawPath(&self, _path: &Path, _paint: &Paint) {}
    pub fn scale(&self, _sx: f32, _sy: f32, _px: f32, _py: f32) {}
}

pub struct PathDrawer {
    pub canvas: Canvas,
    pub paint: Paint,
    pub rectF: Option<RectF>,
    pub path: Option<Path>,
}

impl PathDrawer {
    pub fn draw(&mut self, shader: Shader, rect: Option<RectF>) {
        let rect = rect.or_else(|| self.rectF.clone());
        self.paint.setShader(Some(shader));
        if let Some(ref r) = rect {
            self.canvas.drawRect(r, &self.paint);
        }
    }
}

// Java field: int FOCUS_TYPE1
pub(crate) const FOCUS_TYPE1: i32 = 100;
// Java field: int FOCUS_TYPE2
pub(crate) const FOCUS_TYPE2: i32 = 0;
// Java field: int FOCUS_TYPE3
pub(crate) const FOCUS_TYPE3: i32 = 50;
// Java field: int FOCUS_TYPE4
pub(crate) const FOCUS_TYPE4: i32 = -50;
// Java field: int SHAPE_TYPE_0
pub(crate) const SHAPE_TYPE_0: i32 = 0;
// Java field: int SHAPE_TYPE_1
pub(crate) const SHAPE_TYPE_1: i32 = 1;
// Java field: int SHAPE_TYPE_2
pub(crate) const SHAPE_TYPE_2: i32 = 2;
// Java field: int SHAPE_TYPE_3
pub(crate) const SHAPE_TYPE_3: i32 = 3;
// Java field: int SHAPE_TYPE_4
pub(crate) const SHAPE_TYPE_4: i32 = 4;

fn getRotationRect(sourceRect: &RectF, rotation: f32) -> RectF {
    if rotation == 0.0 {
        return sourceRect.clone();
    }
    let center_x = sourceRect.centerX();
    let center_y = sourceRect.centerY();
    let rad = (rotation as f64) * std::f64::consts::PI / 180.0;
    let cos_value = rad.cos().abs() as f32;
    let sin_value = rad.sin().abs() as f32;
    let height = sourceRect.height() * cos_value + sourceRect.width() * sin_value;
    let width = sourceRect.width() * cos_value + sourceRect.height() * sin_value;
    RectF { left: center_x - width / 2.0, top: center_y - height / 2.0, right: center_x + width / 2.0, bottom: center_y + height / 2.0 }
}

// Java: public class GradFillTool
pub struct GradFillTool {
}

// Java: public class GradFillTool
impl GradFillTool {
    // Java: GradFillTool()
    pub fn new() -> Self {
      // Java: super();
      GradFillTool {}
    }

    // Java: static void fillShape(GradFill gradFill, PathDrawer drawer, RectF bounds, boolean bCircle)
    pub fn fillShape(gradFill: &GradFill, drawer: &mut PathDrawer, bounds: &RectF, bCircle: bool) {
      // Java: GradStopList gsList = gradFill.getGradStopList();
      let gsList = GradStopListAdapter::from_element(&gradFill.getGradStopList());
      // Java: if (gsList != null)
      if let Some(gsList) = gsList {
             // Java: int listSize = gsList.size();
             let listSize = gsList.size();
             // Java: if (listSize >= 2)
             if listSize >= 2 {
                // Java: int[] colors = new int[listSize];
                let mut colors: Vec<i32> = vec![0; listSize as usize];
                // Java: float[] positions = new float[listSize];
                let mut positions: Vec<f32> = vec![0.0; listSize as usize];
                // Java: for(int i = 0; i < listSize; ++i)
                for i in 0..listSize as usize {
                   // Java: GradStop stop = gsList.at(i);
                   let stop = (gsList.get_color(i), gsList.get_position(i));
                   // Java: colors[i] = stop.getColor();
                   colors[i] = stop.0;
                   // Java: positions[i] = stop.getPostion();
                   positions[i] = stop.1;
                }
                // Java: int fillType = gradFill.getFillType();
                let fillType = gradFill.base.getFillType();
                // Java: if (fillType != 4 && fillType != 7)
                if fillType != 4 && fillType != 7 {
                   // Java: fillShapeShader(drawer, bounds, gradFill, colors, positions, bCircle);
                   GradFillTool::fillShapeShader(drawer, bounds, gradFill, &mut colors, &mut positions, bCircle);
                }
                // Java: else
                else {
                   // Java: int angle = (int)gradFill.getAngle();
                   let mut angle = gradFill.getAngle() as i32;
                   // Java: if (angle < 0)
                   if angle < 0 {
                      // Java: angle += 180;
                      angle += 180;
                   }
                   // Java: angle = (360 - angle) % 360;
                   angle = (360 - angle) % 360;
                   // Java: RectF dstRect = ShapeHelper.getRotationRect(bounds, (float)angle);
                   let dstRect = getRotationRect(bounds, angle as f32);
                   // Java: Shader shader = createLinearShader(gradFill, colors, positions, dstRect, true);
                   let mut shader = GradFillTool::createLinearShader_with_gradfill(gradFill, &colors, &positions, &dstRect, true);
                   // Java: float rot = gradFill.getFillRot();
                   let mut rot = gradFill.base.getFillRot();
                   // Java: if (gradFill.getAngle() != 0.0F)
                   if gradFill.getAngle() != 0.0_f32 {
                      // Java: rot += (float)angle;
                      rot += angle as f32;
                   }
                   // Java: if (rot != 0.0F)
                   if rot != 0.0_f32 {
                      // Java: Matrix matrix = new Matrix();
                      let mut matrix = Matrix::new();
                      // Java: matrix.setRotate(rot, dstRect.centerX(), dstRect.centerY());
                      matrix.setRotate(rot, dstRect.centerX(), dstRect.centerY());
                      // Java: shader.setLocalMatrix(matrix);
                      shader.setLocalMatrix(matrix);
                   }
                   // Java: drawer.draw(shader, (RectF)null);
                   drawer.draw(shader, None);
                }
             }
      }
    }

    // Java: static void fillShapeShader(PathDrawer drawer, RectF dstRect, GradFill gradFill, int[] colors, float[] positions, boolean bCircle)
    pub fn fillShapeShader(drawer: &mut PathDrawer, dstRect: &RectF, gradFill: &GradFill, colors: &mut Vec<i32>, positions: &mut Vec<f32>, bCircle: bool) {
      // Java: RectF rectF = drawer.rectF;
      let rectF = drawer.rectF.clone();
      // Java: Canvas canvas = drawer.canvas;
      // canvas accessed via drawer.canvas directly
      // Java: int fillType = gradFill.getFillType();
      let fillType = gradFill.base.getFillType();
      // Java: if (fillType != 5 && fillType != 6 && fillType != 11)
      if fillType != 5 && fillType != 6 && fillType != 11 {
         // Java: if (fillType == 10)
         if fillType == 10 {
            // Java: if (gradFill.getFocus() != 100.0F)
            if gradFill.getFocus() != 100.0_f32 {
               // Java: int size = colors.length;
               let size = colors.len();
               // Java: int[] newColors = new int[size];
               let mut newColors: Vec<i32> = vec![0; size];
               // Java: for(int i = 0; i < size; ++i)
               for i in 0..size {
                  // Java: newColors[size - i - 1] = colors[i];
                  newColors[size - i - 1] = colors[i];
               }
               // Java: colors = newColors;
               *colors = newColors;
            }
            // Java: canvas.save();
            drawer.canvas.save();
            // Java: canvas.clipRect(dstRect);
            drawer.canvas.clipRect(dstRect);
            let mut rectF_opt = rectF;
            // Java: if (rectF == null)
            if rectF_opt.is_none() {
               // Java: rectF = dstRect;
               rectF_opt = Some(dstRect.clone());
            }
            let rectF = rectF_opt.unwrap();
            // Java: cn.wps.graphics.RectF toRect = gradFill.getFillToRect();
            let toRect = gradFill.base.getFillToRect();
            // Java: cn.wps.graphics.RectF tileRect = gradFill.getTileRect();
            let tileRect_element = gradFill.getTileRect();
            let tileRect = tileRect_element.and_then(|v| v.downcast_ref::<RectF>().cloned());
            // Java: RectF rcBound = clipped(rectF, tileRect);
            let rcBound = GradFillTool::clipped(&rectF, tileRect.as_ref());
            // Java: RectF rcFocus = toRect != null ? new RectF(toRect.left, toRect.top, toRect.left + Math.max(0.0F, toRect.right - toRect.left), toRect.top + Math.max(0.0F, toRect.bottom - toRect.top)) : new RectF(0.0F, 0.0F, 1.0F, 1.0F);
            let rcFocus = if let Some(ref tr) = toRect {
                RectF { left: tr.left, top: tr.top, right: tr.left + f32::max(0.0, tr.right - tr.left), bottom: tr.top + f32::max(0.0, tr.bottom - tr.top) }
            } else {
                RectF { left: 0.0, top: 0.0, right: 1.0, bottom: 1.0 }
            };
            // Java: float sx = rcFocus.width();
            let sx = rcFocus.width();
            // Java: float sy = rcFocus.height();
            let sy = rcFocus.height();
            // Java: float focusX = (double)Math.abs(1.0F - sx) < 1.0E-6 ? rcFocus.left + sx * 0.5F : rcFocus.left / (1.0F - sx);
            let focusX = if ((1.0_f32 - sx).abs() as f64) < 1.0e-6 { rcFocus.left + sx * 0.5 } else { rcFocus.left / (1.0 - sx) };
            // Java: float focusY = (double)Math.abs(1.0F - sy) < 1.0E-6 ? rcFocus.top + sy * 0.5F : rcFocus.top / (1.0F - sy);
            let focusY = if ((1.0_f32 - sy).abs() as f64) < 1.0e-6 { rcFocus.top + sy * 0.5 } else { rcFocus.top / (1.0 - sy) };
            // Java: float cx = rectF.left + rectF.width() * focusX;
            let mut cx = rectF.left + rectF.width() * focusX;
            // Java: float cy = rectF.top + rectF.height() * focusY;
            let mut cy = rectF.top + rectF.height() * focusY;
            // Java: float r1 = (float)Math.sqrt((double)((rcBound.left - cx) * (rcBound.left - cx) + (rcBound.top - cy) * (rcBound.top - cy)));
            let r1 = ((rcBound.left - cx) * (rcBound.left - cx) + (rcBound.top - cy) * (rcBound.top - cy)).sqrt();
            // Java: float r2 = (float)Math.sqrt((double)((rcBound.left - cx) * (rcBound.left - cx) + (rcBound.bottom - cy) * (rcBound.bottom - cy)));
            let r2 = ((rcBound.left - cx) * (rcBound.left - cx) + (rcBound.bottom - cy) * (rcBound.bottom - cy)).sqrt();
            // Java: float r3 = (float)Math.sqrt((double)((rcBound.right - cx) * (rcBound.right - cx) + (rcBound.top - cy) * (rcBound.top - cy)));
            let r3 = ((rcBound.right - cx) * (rcBound.right - cx) + (rcBound.top - cy) * (rcBound.top - cy)).sqrt();
            // Java: float r4 = (float)Math.sqrt((double)((rcBound.right - cx) * (rcBound.right - cx) + (rcBound.bottom - cy) * (rcBound.bottom - cy)));
            let r4 = ((rcBound.right - cx) * (rcBound.right - cx) + (rcBound.bottom - cy) * (rcBound.bottom - cy)).sqrt();
            // Java: float rf = Math.max(Math.max(r1, r2), Math.max(r3, r4));
            let rf = f32::max(f32::max(r1, r2), f32::max(r3, r4));
            // Java: float r = (float)Math.sqrt((double)(rcBound.width() * rcBound.width() + rcBound.height() * rcBound.height())) / 2.0F;
            let mut r = (rcBound.width() * rcBound.width() + rcBound.height() * rcBound.height()).sqrt() / 2.0;
            // Java: if (r < rf)
            if r < rf {
               // Java: r = rf;
               r = rf;
            }
            // Java: if ((cx - rectF.centerX()) * (cx - rectF.centerX()) + (cy - rectF.centerY()) * (cy - rectF.centerY()) < (rectF.centerX() - rcBound.centerX()) * (rectF.centerX() - rcBound.centerX()) + (rectF.centerY() - rcBound.centerY()) * (rectF.centerY() - rcBound.centerY()))
            if (cx - rectF.centerX()) * (cx - rectF.centerX()) + (cy - rectF.centerY()) * (cy - rectF.centerY()) < (rectF.centerX() - rcBound.centerX()) * (rectF.centerX() - rcBound.centerX()) + (rectF.centerY() - rcBound.centerY()) * (rectF.centerY() - rcBound.centerY()) {
               // Java: cx = rcBound.centerX();
               cx = rcBound.centerX();
               // Java: cy = rcBound.centerY();
               cy = rcBound.centerY();
            }
            // Java: Shader shader = new RadialGradient(cx, cy, r, colors, positions, TileMode.CLAMP);
            let shader = RadialGradient::new(cx, cy, r, colors, positions, TileMode::CLAMP).as_shader();
            // Java: drawer.draw(shader, new RectF(rectF.left, rectF.top, rectF.right, rectF.bottom));
            drawer.draw(shader, Some(RectF { left: rectF.left, top: rectF.top, right: rectF.right, bottom: rectF.bottom }));
            // Java: canvas.restore();
            drawer.canvas.restore();
         }
      }
      // Java: else
      else {
         // Java: int shapeType = getShapeFillType(gradFill);
         let shapeType = GradFillTool::getShapeFillType(gradFill);
         // Java: if (shapeType != 0)
         if shapeType != 0 {
            // Java: fillShapeShader(drawer, dstRect, colors, positions, shapeType);
            GradFillTool::fillShapeShader_with_type(drawer, dstRect, colors, positions, shapeType);
         }
         // Java: else
         else {
            // Java: if (gradFill.getFocus() != 100.0F)
            if gradFill.getFocus() != 100.0_f32 {
               // Java: int size = colors.length;
               let size = colors.len();
               // Java: int[] newColors = new int[size];
               let mut newColors: Vec<i32> = vec![0; size];
               // Java: for(int i = 0; i < size; ++i)
               for i in 0..size {
                  // Java: newColors[size - i - 1] = colors[i];
                  newColors[size - i - 1] = colors[i];
               }
               // Java: colors = newColors;
               *colors = newColors;
            }
            // Java: if (drawer.path == null && bCircle)
            if drawer.path.is_none() && bCircle {
               let rectF = drawer.rectF.clone().unwrap_or(dstRect.clone());
               // Java: canvas.save();
               drawer.canvas.save();
               // Java: canvas.clipRect(dstRect);
               drawer.canvas.clipRect(dstRect);
               // Java: float cx = rectF.centerX();
               let cx = rectF.centerX();
               // Java: float cy = rectF.centerY();
               let cy = rectF.centerY();
               // Java: float w = rectF.width();
               let w = rectF.width();
               // Java: float h = rectF.height();
               let h = rectF.height();
               // Java: float hw = w / 2.0F;
               let hw = w / 2.0;
               // Java: float hh = h / 2.0F;
               let hh = h / 2.0;
               // Java: float r = Math.max(Math.max(hw, hh), 0.01F);
               let r = f32::max(f32::max(hw, hh), 0.01);
               // Java: Shader shader = new RadialGradient(cx, cy, r, colors, positions, TileMode.CLAMP);
               let shader = RadialGradient::new(cx, cy, r, colors, positions, TileMode::CLAMP).as_shader();
               // Java: drawer.paint.setShader(shader);
               drawer.paint.setShader(Some(shader));
               // Java: canvas.scale(hw / r, hh / r, cx, cy);
               drawer.canvas.scale(hw / r, hh / r, cx, cy);
               // Java: canvas.drawRect(cx - r, cy - r, cx + r, cy + r, drawer.paint);
               drawer.canvas.drawRect_with_paint(cx - r, cy - r, cx + r, cy + r, &drawer.paint);
               // Java: canvas.restore();
               drawer.canvas.restore();
               // Java: return;
               return;
            }
            // Java: RectF childRect = null;
            let mut childRect: Option<RectF>;
            // Java: float w = dstRect.width();
            let w = dstRect.width();
            // Java: float h = dstRect.height();
            let h = dstRect.height();
            // Java: cn.wps.graphics.RectF toRect = gradFill.getFillToRect();
            let toRect = gradFill.base.getFillToRect().unwrap_or(RectF { left: 0.0, top: 0.0, right: 1.0, bottom: 1.0 });
            // Java: boolean bRect = toRect.right - toRect.left > 0.01F && toRect.bottom - toRect.top > 0.01F;
            let bRect = toRect.right - toRect.left > 0.01 && toRect.bottom - toRect.top > 0.01;
            // Java: if (bRect)
            if bRect {
               // Java: childRect = new RectF(dstRect.left + w * toRect.left - 0.5F, dstRect.top + h * toRect.top - 0.5F, dstRect.left + w * toRect.right + 0.5F, dstRect.top + h * toRect.bottom + 0.5F);
               childRect = Some(RectF {
                   left: dstRect.left + w * toRect.left - 0.5,
                   top: dstRect.top + h * toRect.top - 0.5,
                   right: dstRect.left + w * toRect.right + 0.5,
                   bottom: dstRect.top + h * toRect.bottom + 0.5,
               });
               // Java: drawer.paint.setShader((Shader)null);
               drawer.paint.setShader(None);
               // Java: drawer.paint.setColor(colors[0]);
               drawer.paint.setColor(colors[0]);
               // Java: canvas.drawRect(childRect, drawer.paint);
               if let Some(ref cr) = childRect { drawer.canvas.drawRect(cr, &drawer.paint); }
            }
            // Java: if (toRect.left > 0.0F)
            if toRect.left > 0.0 {
               // Java: canvas.save();
               drawer.canvas.save();
               // Java: childRect = new RectF(dstRect.left, dstRect.top, dstRect.left + w * toRect.left, dstRect.bottom);
               childRect = Some(RectF { left: dstRect.left, top: dstRect.top, right: dstRect.left + w * toRect.left, bottom: dstRect.bottom });
               // Java: Shader shader = createLinearShader(colors, positions, 0.0F, childRect, false);
               let shader = GradFillTool::createLinearShader(colors, positions, 0.0, childRect.as_ref().unwrap(), false);
               // Java: Path shapePath = new Path();
               let mut shapePath = Path::new();
               let cr = childRect.as_ref().unwrap();
               // Java: shapePath.moveTo(childRect.left, childRect.top);
               shapePath.moveTo(cr.left, cr.top);
               // Java: shapePath.lineTo(childRect.right, childRect.top + h * toRect.top);
               shapePath.lineTo(cr.right, cr.top + h * toRect.top);
               // Java: if (toRect.bottom - toRect.top > 0.01F)
               if toRect.bottom - toRect.top > 0.01 {
                  // Java: shapePath.lineTo(childRect.right, childRect.top + h * toRect.bottom);
                  shapePath.lineTo(cr.right, cr.top + h * toRect.bottom);
               }
               // Java: shapePath.lineTo(childRect.left, childRect.bottom);
               shapePath.lineTo(cr.left, cr.bottom);
               // Java: shapePath.close();
               shapePath.close();
               // Java: canvas.clipPath(shapePath);
               drawer.canvas.clipPath(&shapePath);
               // Java: drawer.draw(shader, (RectF)null);
               drawer.draw(shader, None);
               // Java: canvas.restore();
               drawer.canvas.restore();
            }
            // Java: if (toRect.right < 1.0F)
            if toRect.right < 1.0 {
               // Java: canvas.save();
               drawer.canvas.save();
               // Java: childRect = new RectF(dstRect.left + w * toRect.right, dstRect.top, dstRect.right, dstRect.bottom);
               childRect = Some(RectF { left: dstRect.left + w * toRect.right, top: dstRect.top, right: dstRect.right, bottom: dstRect.bottom });
               // Java: Shader shader = createLinearShader(colors, positions, 100.0F, childRect, false);
               let shader = GradFillTool::createLinearShader(colors, positions, 100.0, childRect.as_ref().unwrap(), false);
               // Java: Path shapePath = new Path();
               let mut shapePath = Path::new();
               let cr = childRect.as_ref().unwrap();
               // Java: shapePath.moveTo(childRect.right, childRect.top);
               shapePath.moveTo(cr.right, cr.top);
               // Java: shapePath.lineTo(childRect.right, childRect.bottom);
               shapePath.lineTo(cr.right, cr.bottom);
               // Java: shapePath.lineTo(childRect.left, childRect.top + h * toRect.bottom);
               shapePath.lineTo(cr.left, cr.top + h * toRect.bottom);
               // Java: if (toRect.bottom - toRect.top > 0.01F)
               if toRect.bottom - toRect.top > 0.01 {
                  // Java: shapePath.lineTo(childRect.left, childRect.top + h * toRect.top);
                  shapePath.lineTo(cr.left, cr.top + h * toRect.top);
               }
               // Java: shapePath.close();
               shapePath.close();
               // Java: canvas.clipPath(shapePath);
               drawer.canvas.clipPath(&shapePath);
               // Java: drawer.draw(shader, (RectF)null);
               drawer.draw(shader, None);
               // Java: canvas.restore();
               drawer.canvas.restore();
            }
            // Java: if (toRect.top > 0.0F)
            if toRect.top > 0.0 {
               // Java: canvas.save();
               drawer.canvas.save();
               // Java: childRect = new RectF(dstRect.left, dstRect.top, dstRect.right, dstRect.top + h * toRect.top);
               childRect = Some(RectF { left: dstRect.left, top: dstRect.top, right: dstRect.right, bottom: dstRect.top + h * toRect.top });
               // Java: Shader shader = createLinearShader(colors, positions, 0.0F, childRect, true);
               let shader = GradFillTool::createLinearShader(colors, positions, 0.0, childRect.as_ref().unwrap(), true);
               // Java: Path shapePath = new Path();
               let mut shapePath = Path::new();
               let cr = childRect.as_ref().unwrap();
               // Java: shapePath.moveTo(childRect.left, childRect.top);
               shapePath.moveTo(cr.left, cr.top);
               // Java: shapePath.lineTo(childRect.right, childRect.top);
               shapePath.lineTo(cr.right, cr.top);
               // Java: shapePath.lineTo(childRect.left + w * toRect.right, childRect.bottom);
               shapePath.lineTo(cr.left + w * toRect.right, cr.bottom);
               // Java: if (toRect.right - toRect.left > 0.01F)
               if toRect.right - toRect.left > 0.01 {
                  // Java: shapePath.lineTo(childRect.left + w * toRect.left, childRect.bottom);
                  shapePath.lineTo(cr.left + w * toRect.left, cr.bottom);
               }
               // Java: shapePath.close();
               shapePath.close();
               // Java: canvas.clipPath(shapePath);
               drawer.canvas.clipPath(&shapePath);
               // Java: drawer.draw(shader, (RectF)null);
               drawer.draw(shader, None);
               // Java: canvas.restore();
               drawer.canvas.restore();
            }
            // Java: if (toRect.bottom < 1.0F)
            if toRect.bottom < 1.0 {
               // Java: canvas.save();
               drawer.canvas.save();
               // Java: childRect = new RectF(dstRect.left, dstRect.top + h * toRect.bottom, dstRect.right, dstRect.bottom);
               childRect = Some(RectF { left: dstRect.left, top: dstRect.top + h * toRect.bottom, right: dstRect.right, bottom: dstRect.bottom });
               // Java: Shader shader = createLinearShader(colors, positions, 100.0F, childRect, true);
               let shader = GradFillTool::createLinearShader(colors, positions, 100.0, childRect.as_ref().unwrap(), true);
               // Java: Path shapePath = new Path();
               let mut shapePath = Path::new();
               let cr = childRect.as_ref().unwrap();
               // Java: shapePath.moveTo(childRect.left, childRect.bottom);
               shapePath.moveTo(cr.left, cr.bottom);
               // Java: shapePath.lineTo(childRect.right, childRect.bottom);
               shapePath.lineTo(cr.right, cr.bottom);
               // Java: shapePath.lineTo(childRect.left + w * toRect.right, childRect.top);
               shapePath.lineTo(cr.left + w * toRect.right, cr.top);
               // Java: if (toRect.right - toRect.left > 0.01F)
               if toRect.right - toRect.left > 0.01 {
                  // Java: shapePath.lineTo(childRect.left + w * toRect.left, childRect.top);
                  shapePath.lineTo(cr.left + w * toRect.left, cr.top);
               }
               // Java: shapePath.close();
               shapePath.close();
               // Java: canvas.clipPath(shapePath);
               drawer.canvas.clipPath(&shapePath);
               // Java: drawer.draw(shader, (RectF)null);
               drawer.draw(shader, None);
               // Java: canvas.restore();
               drawer.canvas.restore();
            }
         }
      }
    }

    // Java: static RectF clipped(RectF rect, cn.wps.graphics.RectF tileRect)
    pub fn clipped(rect: &RectF, tileRect: Option<&RectF>) -> RectF {
      // Java: if (tileRect == null)
      if tileRect.is_none() {
         // Java: return rect;
         return rect.clone();
      }
      // Java: else if ((double)Math.abs(tileRect.left) < 1.0E-6 && (double)Math.abs(tileRect.top) < 1.0E-6 && (double)Math.abs(tileRect.right) < 1.0E-6 && (double)Math.abs(tileRect.bottom) < 1.0E-6)
      else if (tileRect.unwrap().left.abs() as f64) < 1.0e-6 && (tileRect.unwrap().top.abs() as f64) < 1.0e-6 && (tileRect.unwrap().right.abs() as f64) < 1.0e-6 && (tileRect.unwrap().bottom.abs() as f64) < 1.0e-6 {
         // Java: return rect;
         return rect.clone();
      }
      // Java: else
      else {
      let tileRect = tileRect.unwrap();
      // Java: float w = rect.width();
      let w = rect.width();
      // Java: float h = rect.height();
      let h = rect.height();
      // Java: RectF result = new RectF(rect.left + (float)Math.round(w * tileRect.left), rect.top + (float)Math.round(h * tileRect.top), rect.right - (float)Math.round(w * tileRect.right), rect.bottom - (float)Math.round(h * tileRect.bottom));
      let result = RectF {
          left: rect.left + (w * tileRect.left).round(),
          top: rect.top + (h * tileRect.top).round(),
          right: rect.right - (w * tileRect.right).round(),
          bottom: rect.bottom - (h * tileRect.bottom).round(),
      };
      // Java: return result;
      return result;
      }
    }

    // Java: static Shader createLinearShader(GradFill gradFill, int[] colors, float[] positions, RectF dstRect, boolean bVertical)
    pub fn createLinearShader_with_gradfill(gradFill: &GradFill, colors: &[i32], positions: &[f32], dstRect: &RectF, bVertical: bool) -> Shader {
      // Java: float focus = gradFill.getFocus();
      let mut focus = gradFill.getFocus();
      // Java: if (bVertical && gradFill.getLinearType() != 1 && gradFill.getAngle() < 0.0F)
      if bVertical && gradFill.getLinearType() != 1 && gradFill.getAngle() < 0.0 {
         // Java: if (focus == 50.0F)
         if focus == 50.0 {
            // Java: focus = -50.0F;
            focus = -50.0;
         }
         // Java: else if (focus == -50.0F)
         else if focus == -50.0 {
            // Java: focus = 50.0F;
            focus = 50.0;
         }
      }
      // Java: return createLinearShader(colors, positions, focus, dstRect, true);
      GradFillTool::createLinearShader(colors, positions, focus, dstRect, true)
    }

    // Java: static Shader createLinearShader(int[] colors, float[] positions, float focus, RectF dstRect, boolean bVertical)
    pub fn createLinearShader(colors: &[i32], positions: &[f32], focus: f32, dstRect: &RectF, bVertical: bool) -> Shader {
      // Java: if (focus != 50.0F && focus != -50.0F)
      if focus != 50.0 && focus != -50.0 {
         // Java: float x0 = bVertical ? dstRect.centerX() : (focus == 0.0F ? dstRect.right : dstRect.left);
         let x0 = if bVertical { dstRect.centerX() } else { if focus == 0.0 { dstRect.right } else { dstRect.left } };
         // Java: float x1 = bVertical ? dstRect.centerX() : (focus == 0.0F ? dstRect.left : dstRect.right);
         let x1 = if bVertical { dstRect.centerX() } else { if focus == 0.0 { dstRect.left } else { dstRect.right } };
         // Java: float y0 = bVertical ? (focus == 0.0F ? dstRect.bottom : dstRect.top) : dstRect.centerY();
         let y0 = if bVertical { if focus == 0.0 { dstRect.bottom } else { dstRect.top } } else { dstRect.centerY() };
         // Java: float y1 = bVertical ? (focus == 0.0F ? dstRect.top : dstRect.bottom) : dstRect.centerY();
         let y1 = if bVertical { if focus == 0.0 { dstRect.top } else { dstRect.bottom } } else { dstRect.centerY() };
         // Java: return new LinearGradient(x0, y0, x1, y1, colors, positions, TileMode.MIRROR);
         LinearGradient::new(x0, y0, x1, y1, colors, positions, TileMode::MIRROR).as_shader()
      }
      // Java: else
      else {
         // Java: int size = colors.length;
         let size = colors.len();
         // Java: int len = size * 2 - 1;
         let len = size * 2 - 1;
         // Java: int[] colorsNew = new int[len];
         let mut colorsNew: Vec<i32> = vec![0; len];
         // Java: float[] posNew = new float[len];
         let mut posNew: Vec<f32> = vec![0.0; len];
         // Java: if (focus == 50.0F)
         if focus == 50.0 {
            // Java: for(int i = 0; i < size; ++i)
            for i in 0..size {
               // Java: colorsNew[i] = colors[i];
               colorsNew[i] = colors[i];
               // Java: posNew[i] = positions[i] / 2.0F;
               posNew[i] = positions[i] / 2.0;
            }
            // Java: for(int i = 1; i < size; ++i)
            for i in 1..size {
               // Java: colorsNew[i + size - 1] = colors[size - i - 1];
               colorsNew[i + size - 1] = colors[size - i - 1];
               // Java: posNew[i + size - 1] = 0.5F + (1.0F - positions[size - i - 1]) / 2.0F;
               posNew[i + size - 1] = 0.5 + (1.0 - positions[size - i - 1]) / 2.0;
            }
         }
         // Java: else
         else {
            // Java: for(int i = 0; i < size; ++i)
            for i in 0..size {
               // Java: colorsNew[i] = colors[size - i - 1];
               colorsNew[i] = colors[size - i - 1];
               // Java: posNew[i] = (1.0F - positions[size - i - 1]) / 2.0F;
               posNew[i] = (1.0 - positions[size - i - 1]) / 2.0;
            }
            // Java: for(int i = 1; i < size; ++i)
            for i in 1..size {
               // Java: colorsNew[i + size - 1] = colors[i];
               colorsNew[i + size - 1] = colors[i];
               // Java: posNew[i + size - 1] = positions[i] / 2.0F + 0.5F;
               posNew[i + size - 1] = positions[i] / 2.0 + 0.5;
            }
         }
         // Java: return createLinearShader(colorsNew, posNew, 100.0F, dstRect, bVertical);
         GradFillTool::createLinearShader(&colorsNew, &posNew, 100.0, dstRect, bVertical)
      }
    }

    // Java: static void fillShapeShader(PathDrawer drawer, RectF rectF, int[] colors, float[] positions, int shapeType)
    pub fn fillShapeShader_with_type(drawer: &mut PathDrawer, rectF: &RectF, colors: &[i32], positions: &[f32], shapeType: i32) {
      // Java: Canvas canvas = drawer.canvas;
      // canvas accessed via drawer.canvas directly
      // Java: if (shapeType == 1)
      if shapeType == 1 {
         // Java: canvas.save();
         drawer.canvas.save();
         // Java: canvas.clipRect(rectF);
         drawer.canvas.clipRect(rectF);
         // Java: Path shapePath = new Path();
         let mut shapePath = Path::new();
         // Java: shapePath.moveTo(rectF.left, rectF.top);
         shapePath.moveTo(rectF.left, rectF.top);
         // Java: shapePath.lineTo(rectF.right, rectF.bottom);
         shapePath.lineTo(rectF.right, rectF.bottom);
         // Java: shapePath.lineTo(rectF.left, rectF.bottom);
         shapePath.lineTo(rectF.left, rectF.bottom);
         // Java: shapePath.close();
         shapePath.close();
         // Java: canvas.clipPath(shapePath);
         drawer.canvas.clipPath(&shapePath);
         // Java: Shader shader = createLinearShader(colors, positions, 100.0F, rectF, true);
         let shader = GradFillTool::createLinearShader(colors, positions, 100.0, rectF, true);
         // Java: drawer.draw(shader, (RectF)null);
         drawer.draw(shader, None);
         // Java: canvas.restore();
         drawer.canvas.restore();
         // Java: canvas.save();
         drawer.canvas.save();
         // Java: canvas.clipRect(rectF);
         drawer.canvas.clipRect(rectF);
         // Java: shapePath.reset();
         shapePath.reset();
         // Java: shapePath.moveTo(rectF.left, rectF.top);
         shapePath.moveTo(rectF.left, rectF.top);
         // Java: shapePath.lineTo(rectF.right, rectF.bottom);
         shapePath.lineTo(rectF.right, rectF.bottom);
         // Java: shapePath.lineTo(rectF.right, rectF.top);
         shapePath.lineTo(rectF.right, rectF.top);
         // Java: shapePath.close();
         shapePath.close();
         // Java: canvas.clipPath(shapePath);
         drawer.canvas.clipPath(&shapePath);
         // Java: shader = createLinearShader(colors, positions, 100.0F, rectF, false);
         let shader = GradFillTool::createLinearShader(colors, positions, 100.0, rectF, false);
         // Java: drawer.draw(shader, (RectF)null);
         drawer.draw(shader, None);
         // Java: canvas.restore();
         drawer.canvas.restore();
      }
      // Java: else if (shapeType == 4)
      else if shapeType == 4 {
         // Java: canvas.save();
         drawer.canvas.save();
         // Java: canvas.clipRect(rectF);
         drawer.canvas.clipRect(rectF);
         // Java: Path shapePath = new Path();
         let mut shapePath = Path::new();
         // Java: shapePath.moveTo(rectF.right, rectF.bottom);
         shapePath.moveTo(rectF.right, rectF.bottom);
         // Java: shapePath.lineTo(rectF.left, rectF.top);
         shapePath.lineTo(rectF.left, rectF.top);
         // Java: shapePath.lineTo(rectF.right, rectF.top);
         shapePath.lineTo(rectF.right, rectF.top);
         // Java: shapePath.close();
         shapePath.close();
         // Java: canvas.clipPath(shapePath);
         drawer.canvas.clipPath(&shapePath);
         // Java: Shader shader = createLinearShader(colors, positions, 0.0F, rectF, true);
         let shader = GradFillTool::createLinearShader(colors, positions, 0.0, rectF, true);
         // Java: drawer.draw(shader, (RectF)null);
         drawer.draw(shader, None);
         // Java: canvas.restore();
         drawer.canvas.restore();
         // Java: canvas.save();
         drawer.canvas.save();
         // Java: canvas.clipRect(rectF);
         drawer.canvas.clipRect(rectF);
         // Java: shapePath.reset();
         shapePath.reset();
         // Java: shapePath.moveTo(rectF.right, rectF.bottom);
         shapePath.moveTo(rectF.right, rectF.bottom);
         // Java: shapePath.lineTo(rectF.left, rectF.top);
         shapePath.lineTo(rectF.left, rectF.top);
         // Java: shapePath.lineTo(rectF.left, rectF.bottom);
         shapePath.lineTo(rectF.left, rectF.bottom);
         // Java: shapePath.close();
         shapePath.close();
         // Java: canvas.clipPath(shapePath);
         drawer.canvas.clipPath(&shapePath);
         // Java: shader = createLinearShader(colors, positions, 0.0F, rectF, false);
         let shader = GradFillTool::createLinearShader(colors, positions, 0.0, rectF, false);
         // Java: drawer.draw(shader, (RectF)null);
         drawer.draw(shader, None);
         // Java: canvas.restore();
         drawer.canvas.restore();
      }
      // Java: else if (shapeType == 2)
      else if shapeType == 2 {
         // Java: canvas.save();
         drawer.canvas.save();
         // Java: canvas.clipRect(rectF);
         drawer.canvas.clipRect(rectF);
         // Java: Path shapePath = new Path();
         let mut shapePath = Path::new();
         // Java: shapePath.moveTo(rectF.right, rectF.top);
         shapePath.moveTo(rectF.right, rectF.top);
         // Java: shapePath.lineTo(rectF.left, rectF.bottom);
         shapePath.lineTo(rectF.left, rectF.bottom);
         // Java: shapePath.lineTo(rectF.right, rectF.bottom);
         shapePath.lineTo(rectF.right, rectF.bottom);
         // Java: shapePath.close();
         shapePath.close();
         // Java: canvas.clipPath(shapePath);
         drawer.canvas.clipPath(&shapePath);
         // Java: Shader shader = createLinearShader(colors, positions, 100.0F, rectF, true);
         let shader = GradFillTool::createLinearShader(colors, positions, 100.0, rectF, true);
         // Java: drawer.draw(shader, (RectF)null);
         drawer.draw(shader, None);
         // Java: canvas.restore();
         drawer.canvas.restore();
         // Java: canvas.save();
         drawer.canvas.save();
         // Java: canvas.clipRect(rectF);
         drawer.canvas.clipRect(rectF);
         // Java: shapePath.reset();
         shapePath.reset();
         // Java: shapePath.moveTo(rectF.right, rectF.top);
         shapePath.moveTo(rectF.right, rectF.top);
         // Java: shapePath.lineTo(rectF.left, rectF.bottom);
         shapePath.lineTo(rectF.left, rectF.bottom);
         // Java: shapePath.lineTo(rectF.left, rectF.top);
         shapePath.lineTo(rectF.left, rectF.top);
         // Java: shapePath.close();
         shapePath.close();
         // Java: canvas.clipPath(shapePath);
         drawer.canvas.clipPath(&shapePath);
         // Java: shader = createLinearShader(colors, positions, 0.0F, rectF, false);
         let shader = GradFillTool::createLinearShader(colors, positions, 0.0, rectF, false);
         // Java: drawer.draw(shader, (RectF)null);
         drawer.draw(shader, None);
         // Java: canvas.restore();
         drawer.canvas.restore();
      }
      // Java: else if (shapeType == 3)
      else if shapeType == 3 {
         // Java: canvas.save();
         drawer.canvas.save();
         // Java: canvas.clipRect(rectF);
         drawer.canvas.clipRect(rectF);
         // Java: Path shapePath = new Path();
         let mut shapePath = Path::new();
         // Java: shapePath.moveTo(rectF.left, rectF.bottom);
         shapePath.moveTo(rectF.left, rectF.bottom);
         // Java: shapePath.lineTo(rectF.right, rectF.top);
         shapePath.lineTo(rectF.right, rectF.top);
         // Java: shapePath.lineTo(rectF.left, rectF.top);
         shapePath.lineTo(rectF.left, rectF.top);
         // Java: shapePath.close();
         shapePath.close();
         // Java: canvas.clipPath(shapePath);
         drawer.canvas.clipPath(&shapePath);
         // Java: Shader shader = createLinearShader(colors, positions, 0.0F, rectF, true);
         let shader = GradFillTool::createLinearShader(colors, positions, 0.0, rectF, true);
         // Java: drawer.draw(shader, (RectF)null);
         drawer.draw(shader, None);
         // Java: canvas.restore();
         drawer.canvas.restore();
         // Java: canvas.save();
         drawer.canvas.save();
         // Java: canvas.clipRect(rectF);
         drawer.canvas.clipRect(rectF);
         // Java: shapePath.reset();
         shapePath.reset();
         // Java: shapePath.moveTo(rectF.left, rectF.bottom);
         shapePath.moveTo(rectF.left, rectF.bottom);
         // Java: shapePath.lineTo(rectF.right, rectF.top);
         shapePath.lineTo(rectF.right, rectF.top);
         // Java: shapePath.lineTo(rectF.right, rectF.bottom);
         shapePath.lineTo(rectF.right, rectF.bottom);
         // Java: shapePath.close();
         shapePath.close();
         // Java: canvas.clipPath(shapePath);
         drawer.canvas.clipPath(&shapePath);
         // Java: shader = createLinearShader(colors, positions, 100.0F, rectF, false);
         let shader = GradFillTool::createLinearShader(colors, positions, 100.0, rectF, false);
         // Java: drawer.draw(shader, (RectF)null);
         drawer.draw(shader, None);
         // Java: canvas.restore();
         drawer.canvas.restore();
      }
    }

    // Java: static int getShapeFillType(GradFill gradFill)
    pub fn getShapeFillType(gradFill: &GradFill) -> i32 {
      // Java: cn.wps.graphics.RectF fillToRect = gradFill.getFillToRect();
      let fillToRect = gradFill.base.getFillToRect();
      // Java: int type = 1;
      // LLM Rust Keyword: 'type' is reserved in Rust, using 'r#type'
      let mut r#type = 1;
      // Java: if (fillToRect != null)
      if let Some(ref fillToRect) = fillToRect {
         // Java: if (fillToRect.left != 0.0F && fillToRect.left != 1.0F || fillToRect.top != 0.0F && fillToRect.top != 1.0F || fillToRect.right != 0.0F && fillToRect.right != 1.0F || fillToRect.bottom != 0.0F && fillToRect.bottom != 1.0F)
         if fillToRect.left != 0.0 && fillToRect.left != 1.0 || fillToRect.top != 0.0 && fillToRect.top != 1.0 || fillToRect.right != 0.0 && fillToRect.right != 1.0 || fillToRect.bottom != 0.0 && fillToRect.bottom != 1.0 {
            // Java: return 0;
            return 0;
         }
         // Java: if (fillToRect.left != 0.0F && fillToRect.top != 0.0F)
         if fillToRect.left != 0.0 && fillToRect.top != 0.0 {
            // Java: type = 4;
            r#type = 4;
         }
         // Java: else if (fillToRect.left != 0.0F)
         else if fillToRect.left != 0.0 {
            // Java: type = 2;
            r#type = 2;
         }
         // Java: else if (fillToRect.top != 0.0F)
         else if fillToRect.top != 0.0 {
            // Java: type = 3;
            r#type = 3;
         }
      }
      // Java: return type;
      r#type
    }
}
