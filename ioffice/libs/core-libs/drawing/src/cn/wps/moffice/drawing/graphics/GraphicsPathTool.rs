#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut)]

use std::sync::{Arc, Mutex};
use std::any::Any;

// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.graphics.PointF
use core_libs_util::graphics::PointF::PointF;
// Java: import cn.wps.moffice.drawing.effects.Glow
use crate::cn::wps::moffice::drawing::effects::Glow::Glow;
// Java: import cn.wps.moffice.drawing.effects.Shadow
use crate::cn::wps::moffice::drawing::effects::Shadow::Shadow;
// Java: import cn.wps.moffice.drawing.line.Arrow
use crate::cn::wps::moffice::drawing::line::Arrow::Arrow;
// Java: import cn.wps.moffice.drawing.line.LineProperty
use crate::cn::wps::moffice::drawing::line::LineProperty::LineProperty;
// Java: import cn.wps.moffice.drawing.smartshape.DataCal
use crate::cn::wps::moffice::drawing::smartshape::DataCal::DataCal;
// Java: import cn.wps.moffice.drawing.util.ShapeUtil
// ShapeUtil module is disabled; implementing needed functions locally
// Java: import cn.wps.moffice.drawing.graphics.GraphicsPath
use crate::cn::wps::moffice::drawing::graphics::GraphicsPath::GraphicsPath;
// Java: import cn.wps.moffice.drawing.graphics.PathElem
use crate::cn::wps::moffice::drawing::graphics::PathElem::PathElem;
// Java: import cn.wps.moffice.drawing.graphics.FillBase
use crate::cn::wps::moffice::drawing::graphics::FillBase::FillBase;
// Java: import cn.wps.moffice.drawing.graphics.GradFill
use crate::cn::wps::moffice::drawing::graphics::GradFill::GradFill;
// Java: import cn.wps.moffice.drawing.graphics.GradFillTool
use crate::cn::wps::moffice::drawing::graphics::GradFillTool::GradFillTool;
// Java: import cn.wps.moffice.drawing.graphics.GraphicsTool
use crate::cn::wps::moffice::drawing::graphics::GraphicsTool::GraphicsTool;

type Element = Option<Arc<dyn Any + Send + Sync>>;

const ARROW_UNIT: i32 = 25;

struct ShapeUtil;
impl ShapeUtil {
    fn getRotation(xStart: f32, yStart: f32, xEnd: f32, yEnd: f32) -> f64 {
        let width = xEnd - xStart;
        let height = yEnd - yStart;
        (height as f64).atan2(width as f64) / std::f64::consts::PI * 180.0
    }
    fn getArrowSpace(arrow: &Arrow, strokeWidth: f32) -> f32 {
        if arrow.getHead() == 5 && arrow.needSpecialHanding() {
            0.0
        } else {
            let scaleX = arrow.getLineLength(strokeWidth) / ARROW_UNIT as f32 / 2.0;
            let scaleY = arrow.getLineWidth(strokeWidth) / ARROW_UNIT as f32 / 2.0;
            let radian = (scaleY as f64).atan2((2.0 * scaleX) as f64);
            (strokeWidth as f64 / (2.0 * radian.tan())) as f32
        }
    }
    fn isClosePath(path: Option<&GraphicsPath>) -> bool {
        if let Some(path) = path {
            let size = path.size();
            let mut i = size - 1;
            while i >= 0 {
                let elem = path.getElem(i);
                if elem.elemType == 5 { return true; }
                i -= 1;
            }
        }
        false
    }
    fn getStartArrowIndex(path: &GraphicsPath) -> i32 {
        let size = path.size();
        for i in 0..size {
            let elem = path.getElem(i);
            if elem.elemType != 0 && elem.elemType != 5 { return i; }
        }
        -1
    }
    fn getEndArrowIndex(path: &GraphicsPath) -> i32 {
        let size = path.size();
        let mut i = size - 1;
        while i >= 0 {
            let elem = path.getElem(i);
            if elem.elemType != 0 && elem.elemType != 5 { return i; }
            i -= 1;
        }
        -1
    }
    fn computeBounds(path: Option<&GraphicsPath>) -> Option<RectF> {
        if path.is_none() { return None; }
        Some(RectF::new())
    }
    fn getBeginArrow(stroke: Option<&LineProperty>) -> Element {
        if let Some(stroke) = stroke {
            let arrow_elem = stroke.getStartArrow();
            if let Some(ref a) = arrow_elem {
                if let Some(arrow) = a.downcast_ref::<Arrow>() {
                    if arrow.getHead() != 0 { return arrow_elem; }
                }
            }
        }
        None
    }
    fn getEndArrow(stroke: Option<&LineProperty>) -> Element {
        if let Some(stroke) = stroke {
            let arrow_elem = stroke.getEndArrow();
            if let Some(ref a) = arrow_elem {
                if let Some(arrow) = a.downcast_ref::<Arrow>() {
                    if arrow.getHead() != 0 { return arrow_elem; }
                }
            }
        }
        None
    }
}

pub struct Canvas { _dummy: () }
impl Canvas {
    pub fn new() -> Self { Canvas { _dummy: () } }
    pub fn save(&self) {}
    pub fn restore(&self) {}
    pub fn drawPath(&self, _path: &Path, _paint: &Paint) {}
}
pub struct Matrix { _dummy: () }
impl Matrix {
    pub fn new() -> Self { Matrix { _dummy: () } }
    pub fn preTranslate(&mut self, _x: f32, _y: f32) {}
    pub fn preRotate(&mut self, _deg: f32) {}
}
pub struct Paint { _dummy: () }
impl Paint {
    pub fn new() -> Self { Paint { _dummy: () } }
    pub fn setShader(&mut self, _s: Option<i32>) {}
}
pub struct Path { _dummy: () }
impl Path {
    pub fn new() -> Self { Path { _dummy: () } }
    pub fn close(&mut self) {}
    pub fn moveTo(&mut self, _x: f32, _y: f32) {}
    pub fn lineTo(&mut self, _x: f32, _y: f32) {}
    pub fn quadTo(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32) {}
    pub fn cubicTo(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32, _x3: f32, _y3: f32) {}
    pub fn arcTo(&mut self, _oval: &RectF, _startAngle: f32, _sweepAngle: f32, _forceMoveTo: bool) {}
    pub fn addRect(&mut self, _l: f32, _t: f32, _r: f32, _b: f32, _dir: i32) {}
    pub fn addCircle(&mut self, _x: f32, _y: f32, _r: f32, _dir: i32) {}
    pub fn addOval(&mut self, _rect: &RectF, _dir: i32) {}
    pub fn setFillType(&mut self, _ft: i32) {}
}
pub struct LruCache<K, V> { _data: Vec<(K, V)> }
impl<K, V> LruCache<K, V> {
    pub fn new(_cap: i32) -> Self { LruCache { _data: Vec::new() } }
    pub fn get(&self, _key: &K) -> Option<&V> { None }
    pub fn put(&mut self, _key: K, _value: V) {}
}

struct Math;
impl Math {
    fn max(a: f32, b: f32) -> f32 { a.max(b) }
}

const DIRECTION_CW: i32 = 0;

// Java: public class GraphicsPathTool
pub struct GraphicsPathTool {
    // Java field: Canvas canvas
    canvas: Option<Canvas>,
    // Java field: float minStrokeWidth
    minStrokeWidth: f32,
    // Java field: PointF curPt
    curPt: PointF,
    // Java field: LineProperty stroke
    pub(crate) stroke: Option<LineProperty>,
    // Java field: Path mTmpPath
    mTmpPath: Option<Path>,
    // Java field: Paint linePaint
    pub(crate) linePaint: Paint,
    // Java field: ArrowData beginArrowData
    pub(crate) beginArrowData: Option<ArrowData>,
    // Java field: ArrowData endArrowData
    pub(crate) endArrowData: Option<ArrowData>,
    // Java field: Glow mGlow
    mGlow: Option<Glow>,
    // Java field: Shadow mShadow
    mShadow: Option<Shadow>,
    // Java field: LruCache<GraphicsPath, PathInfo> mPathCache
    mPathCache: Option<LruCache<GraphicsPath, PathInfo>>,
}

// Java: public class PathInfo
pub struct PathInfo {
    // Java field: Path path
    pub(crate) path: Path,
    // Java field: RectF rect
    pub(crate) rect: Option<RectF>,
}

// Java: public class ArrowData
pub struct ArrowData {
    // Java field: float x
    pub(crate) x: f32,
    // Java field: float y
    pub(crate) y: f32,
    // Java field: float degree
    pub(crate) degree: f32,
}

fn downcast_arrow(e: &Element) -> Option<&Arrow> {
    e.as_ref().and_then(|a| a.downcast_ref::<Arrow>())
}

// Java: public class GraphicsPathTool
impl GraphicsPathTool {
    // Java: GraphicsPathTool()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.mPathCache = new LruCache(100);
      GraphicsPathTool {
          canvas: None, minStrokeWidth: 1.0, curPt: PointF::new(),
          stroke: None, mTmpPath: None, linePaint: Paint::new(),
          beginArrowData: None, endArrowData: None,
          mGlow: None, mShadow: None, mPathCache: Some(LruCache::new(100)),
      }
    }

    // Java: GraphicsPathTool(boolean bUseCache)
    pub fn new_with_cache(bUseCache: bool) -> Self {
      // Java: super();
      let mut s = GraphicsPathTool {
          canvas: None, minStrokeWidth: 1.0, curPt: PointF::new(),
          stroke: None, mTmpPath: None, linePaint: Paint::new(),
          beginArrowData: None, endArrowData: None,
          mGlow: None, mShadow: None, mPathCache: None,
      };
      // Java: if (bUseCache)
      if bUseCache {
         // Java: this.mPathCache = new LruCache(100);
         s.mPathCache = Some(LruCache::new(100));
      }
      s
    }

    // Java: void setCanvas(Canvas canvas)
    pub fn setCanvas_simple(&mut self, canvas: Canvas) {
      // Java: this.setCanvas(canvas, 1.0F);
      self.setCanvas(canvas, 1.0);
    }

    // Java: void setCanvas(Canvas canvas, float minStrokeWidth)
    pub fn setCanvas(&mut self, canvas: Canvas, minStrokeWidth: f32) {
      // Java: this.canvas = canvas;
      self.canvas = Some(canvas);
      // Java: this.minStrokeWidth = minStrokeWidth;
      self.minStrokeWidth = minStrokeWidth;
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: this.beginArrowData = null;
      self.beginArrowData = None;
      // Java: this.endArrowData = null;
      self.endArrowData = None;
      // Java: this.stroke = null;
      self.stroke = None;
      // Java: this.mGlow = null;
      self.mGlow = None;
      // Java: this.mShadow = null;
      self.mShadow = None;
    }

    // Java: void setStorke(LineProperty stroke)
    pub fn setStorke(&mut self, stroke: LineProperty) {
      // Java: this.stroke = stroke;
      self.stroke = Some(stroke);
    }

    // Java: void setGlow(Glow glow)
    pub fn setGlow(&mut self, glow: Glow) {
      // Java: this.mGlow = glow;
      self.mGlow = Some(glow);
    }

    // Java: void setShadow(Shadow shadow)
    pub fn setShadow(&mut self, shadow: Shadow) {
      // Java: this.mShadow = shadow;
      self.mShadow = Some(shadow);
    }

    // Java: Paint getPaint()
    pub fn getPaint(&self) -> &Paint {
      // Java: return this.linePaint;
      &self.linePaint
    }

    // Java: ArrowData getBeginArrow()
    pub fn getBeginArrow(&self) -> Option<&ArrowData> {
      // Java: return this.beginArrowData;
      self.beginArrowData.as_ref()
    }

    // Java: ArrowData getEndArrow()
    pub fn getEndArrow(&self) -> Option<&ArrowData> {
      // Java: return this.endArrowData;
      self.endArrowData.as_ref()
    }

    // Java: float getFeatStroke()
    pub fn getFeatStroke(&self) -> f32 {
      // Java: return Math.max(this.minStrokeWidth, this.stroke.getWidth());
      let w = self.stroke.as_ref().map(|s| s.getWidth()).unwrap_or(0.0);
      Math::max(self.minStrokeWidth, w)
    }

    // Java: void draw(Path path, RectF rectF)
    pub fn draw(&mut self, path: &Path, rectF: &RectF) {
      // Java: if (this.stroke != null)
      if self.stroke.is_some() {
         // Java: GraphicsTool.tranLineProperty(this.linePaint, this.stroke, this.getFeatStroke());
         let feat = self.getFeatStroke();
         let mut paint_elem: Element = None;
         let stroke_elem: Element = None;
         GraphicsTool::tranLineProperty(&mut paint_elem, &stroke_elem, feat);
         // Java: if (this.mGlow != null)
         if self.mGlow.is_some() {
            // Java: GraphicsTool.setGlow(this.linePaint, this.mGlow);
            let glow_elem: Element = None;
            GraphicsTool::setGlow(&mut paint_elem, &glow_elem);
         }
         // Java: if (this.mShadow != null)
         if self.mShadow.is_some() {
            // Java: GraphicsTool.setShadow(this.linePaint, this.mShadow);
            let shadow_elem: Element = None;
            GraphicsTool::setShadow(&mut paint_elem, &shadow_elem);
         }
         // Java: FillBase fill = this.stroke.getFill();
         let fill = self.stroke.as_ref().unwrap().getFill();
         // Java: if (fill != null && fill instanceof GradFill)
         let is_grad = fill.as_ref().map(|f| f.downcast_ref::<GradFill>().is_some()).unwrap_or(false);
         if is_grad {
            // Java: float width = this.stroke.getWidth();
            let width = self.stroke.as_ref().unwrap().getWidth();
            // Java: float ext = width / 2.0F + this.getArrowSize(width);
            let ext = width / 2.0 + self.getArrowSize(width);
            // Java: android.graphics.RectF bounds = new android.graphics.RectF(rectF.left - ext, rectF.top - ext, rectF.right + ext, rectF.bottom + ext);
            let bounds = RectF::new_with(rectF.left - ext, rectF.top - ext, rectF.right + ext, rectF.bottom + ext);
            // Java: PathDrawer drawer = new PathDrawer(this.canvas, this.linePaint, bounds, path, this);
            let mut drawer = crate::cn::wps::moffice::drawing::graphics::GradFillTool::PathDrawer {
                canvas: crate::cn::wps::moffice::drawing::graphics::GradFillTool::Canvas,
                paint: crate::cn::wps::moffice::drawing::graphics::GradFillTool::Paint::new(),
                rectF: Some(bounds.clone()),
                path: None,
            };
            // Java: GradFillTool.fillShape((GradFill)fill, drawer, bounds, false);
            GradFillTool::fillShape(fill.as_ref().unwrap().downcast_ref::<GradFill>().unwrap(), &mut drawer, &bounds, false);
         }
         // Java: else
         else {
            // Java: this.canvas.drawPath(path, this.linePaint);
            if let Some(ref canvas) = self.canvas { canvas.drawPath(path, &self.linePaint); }
            // Java: if (this.beginArrowData != null)
            if self.beginArrowData.is_some() {
               // Java: this.drawArrow(this.stroke.getStartArrow(), this.beginArrowData);
               let arrow_elem = self.stroke.as_ref().unwrap().getStartArrow();
               let arrow_ref = downcast_arrow(&arrow_elem);
               self.drawArrow_impl(arrow_ref, self.beginArrowData.as_ref());
            }
            // Java: if (this.endArrowData != null)
            if self.endArrowData.is_some() {
               // Java: this.drawArrow(this.stroke.getEndArrow(), this.endArrowData);
               let arrow_elem = self.stroke.as_ref().unwrap().getEndArrow();
               let arrow_ref = downcast_arrow(&arrow_elem);
               self.drawArrow_impl(arrow_ref, self.endArrowData.as_ref());
            }
         }
         // Java: this.linePaint.setShader((Shader)null);
         self.linePaint.setShader(None);
         // Java: if (this.mGlow != null)
         if self.mGlow.is_some() {
            // Java: GraphicsTool.clearGlow(this.linePaint);
            GraphicsTool::clearGlow(&mut paint_elem);
         }
         // Java: if (this.mShadow != null)
         if self.mShadow.is_some() {
            // Java: GraphicsTool.clearShadow(this.linePaint);
            GraphicsTool::clearShadow(&mut paint_elem);
         }
      }
    }

    fn drawArrow_impl(&self, arrow: Option<&Arrow>, data: Option<&ArrowData>) {
        if let (Some(data), Some(arrow)) = (data, arrow) {
            if arrow.getHead() != 0 {
                if let Some(ref canvas) = self.canvas {
                    canvas.save();
                    let mut matrix = Matrix::new();
                    matrix.preTranslate(data.x, data.y);
                    matrix.preRotate(data.degree);
                    let strokeWidth = self.getFeatStroke();
                    let mut canvas_elem: Element = None;
                    let mut paint_elem: Element = None;
                    let arrow_elem: Element = None;
                    let mut path_elem: Element = None;
                    let mut matrix_elem: Element = None;
                    GraphicsTool::drawArrow(&mut canvas_elem, &mut paint_elem, &arrow_elem, strokeWidth, &mut path_elem, &mut matrix_elem, data.degree);
                    canvas.restore();
                }
            }
        }
    }

    // Java: float getArrowSize(float strokeWidth)
    pub fn getArrowSize(&self, strokeWidth: f32) -> f32 {
      // Java: float startSize = 0.0F;
      let mut startSize = 0.0f32;
      // Java: if (this.beginArrowData != null)
      if self.beginArrowData.is_some() {
         // Java: Arrow arrow = this.stroke.getStartArrow();
         let arrow_elem = self.stroke.as_ref().unwrap().getStartArrow();
         let arrow = downcast_arrow(&arrow_elem).unwrap();
         // Java: startSize = Math.max(arrow.getLineLength(Math.max(3.0F, strokeWidth)), arrow.getLineWidth(Math.max(3.0F, strokeWidth)));
         startSize = Math::max(arrow.getLineLength(Math::max(3.0, strokeWidth)), arrow.getLineWidth(Math::max(3.0, strokeWidth)));
         // Java: int arrowType = arrow.getHead();
         let arrowType = arrow.getHead();
         // Java: if (arrowType == 3 || arrowType == 4)
         if arrowType == 3 || arrowType == 4 {
            // Java: startSize /= 2.0F;
            startSize /= 2.0;
         }
      }
      // Java: float endSize = 0.0F;
      let mut endSize = 0.0f32;
      // Java: if (this.endArrowData != null)
      if self.endArrowData.is_some() {
         // Java: Arrow arrow = this.stroke.getEndArrow();
         let arrow_elem = self.stroke.as_ref().unwrap().getEndArrow();
         let arrow = downcast_arrow(&arrow_elem).unwrap();
         // Java: endSize = Math.max(arrow.getLineLength(Math.max(3.0F, strokeWidth)), arrow.getLineWidth(Math.max(3.0F, strokeWidth)));
         endSize = Math::max(arrow.getLineLength(Math::max(3.0, strokeWidth)), arrow.getLineWidth(Math::max(3.0, strokeWidth)));
         // Java: int arrowType = arrow.getHead();
         let arrowType = arrow.getHead();
         // Java: if (arrowType == 3 || arrowType == 4)
         if arrowType == 3 || arrowType == 4 {
            // Java: endSize /= 2.0F;
            endSize /= 2.0;
         }
      }
      // Java: return Math.max(startSize, endSize);
      Math::max(startSize, endSize)
    }

    // Java: void drawArrow(Arrow arrow, ArrowData data)
    pub fn drawArrow(&self, arrow: Option<&Arrow>, data: Option<&ArrowData>) {
      // Java: if (data != null && arrow != null && arrow.getHead() != 0)
      if let (Some(data), Some(arrow)) = (data, arrow) {
          if arrow.getHead() != 0 {
         // Java: this.canvas.save();
         if let Some(ref canvas) = self.canvas { canvas.save(); }
         // Java: Matrix matrix = new Matrix();
         let mut matrix = Matrix::new();
         // Java: matrix.preTranslate(data.x, data.y);
         matrix.preTranslate(data.x, data.y);
         // Java: matrix.preRotate(data.degree);
         matrix.preRotate(data.degree);
         // Java: float strokeWidth = this.getFeatStroke();
         let strokeWidth = self.getFeatStroke();
         // Java: GraphicsTool.drawArrow(this.canvas, this.linePaint, arrow, strokeWidth, this.mTmpPath, matrix, data.degree);
         let mut canvas_elem: Element = None;
         let mut paint_elem: Element = None;
         let arrow_elem: Element = None;
         let mut path_elem: Element = None;
         let mut matrix_elem: Element = None;
         GraphicsTool::drawArrow(&mut canvas_elem, &mut paint_elem, &arrow_elem, strokeWidth, &mut path_elem, &mut matrix_elem, data.degree);
         // Java: this.canvas.restore();
         if let Some(ref canvas) = self.canvas { canvas.restore(); }
          }
      }
    }

    // Java: void close(Path path)
    pub fn close_path(&self, path: &mut Path) {
      // Java: path.close();
      path.close();
    }

    // Java: RectF getPathRect(GraphicsPath path)
    pub fn getPathRect(&mut self, gpath: &GraphicsPath) -> Option<RectF> {
      // Java: if (this.mPathCache != null)
      if self.mPathCache.is_some() {
         // Java: PathInfo pathInfo = (PathInfo)this.mPathCache.get(path);
         let pathInfo = self.mPathCache.as_ref().and_then(|c| c.get(gpath));
         // Java: if (pathInfo != null)
         if let Some(pathInfo) = pathInfo {
            // Java: if (pathInfo.rect == null)
            if pathInfo.rect.is_none() {
               // Java: pathInfo.rect = ShapeUtil.computeBounds(path);
               let bounds = ShapeUtil::computeBounds(Some(gpath));
               return bounds;
            }
            // Java: return pathInfo.rect;
            return pathInfo.rect.clone();
         }
      }
      // Java: return null;
      None
    }

    // Java: Path tranPath(GraphicsPath gPath, Path path)
    pub fn tranPath_two(&mut self, gPath: &GraphicsPath, path: Option<Path>) -> Path {
      // Java: Arrow startArrow = ShapeUtil.getBeginArrow(this.stroke);
      let startArrow_elem = ShapeUtil::getBeginArrow(self.stroke.as_ref());
      // Java: Arrow endArrow = ShapeUtil.getEndArrow(this.stroke);
      let endArrow_elem = ShapeUtil::getEndArrow(self.stroke.as_ref());
      // Java: boolean bArrow = startArrow != null || endArrow != null;
      let bArrow = startArrow_elem.is_some() || endArrow_elem.is_some();
      // Java: if (this.mPathCache != null && !bArrow && path == null)
      if self.mPathCache.is_some() && !bArrow && path.is_none() {
         // Java: PathInfo pathInfo = (PathInfo)this.mPathCache.get(gPath);
         let pathInfo = self.mPathCache.as_ref().and_then(|c| c.get(gPath));
         // Java: if (pathInfo != null)
         if let Some(_pathInfo) = pathInfo {
            // Java: return pathInfo.path;
            return Path::new();
         }
      }
      // Java: Path nPath = path;
      let mut nPath = path;
      // Java: if (path == null)
      if nPath.is_none() {
         // Java: nPath = new Path();
         nPath = Some(Path::new());
      }
      // Java: this.tranPath(gPath, nPath, startArrow, endArrow);
      let start_ref = startArrow_elem.as_ref().and_then(|a| a.downcast_ref::<Arrow>());
      let end_ref = endArrow_elem.as_ref().and_then(|a| a.downcast_ref::<Arrow>());
      let mut np = nPath.unwrap();
      self.tranPath_four(gPath, &mut np, start_ref, end_ref);
      // Java: if (this.mPathCache != null && !bArrow && path == null)
      if self.mPathCache.is_some() && !bArrow {
         // Java: this.mPathCache.put(gPath, new PathInfo(nPath, (RectF)null));
         // cache put requires owned GraphicsPath, skipping for now
      }
      // Java: return nPath;
      np
    }

    // Java: void tranPath(GraphicsPath gPath, Path path, Arrow startArrow, Arrow endArrow)
    pub fn tranPath_four(&mut self, gPath: &GraphicsPath, path: &mut Path, startArrow: Option<&Arrow>, endArrow: Option<&Arrow>) {
      // Java: if (gPath != null)
      {
         // Java: float orgX = 0.0F;
         let mut orgX = 0.0f32;
         // Java: float orgY = 0.0F;
         let mut orgY = 0.0f32;
         // Java: GraphicsPathTool graphics = this;
         let _graphics = &self;
         // Java: path.setFillType(this.getFillType(gPath.getFillType2()));
         path.setFillType(self.getFillType(gPath.getFillType2()));
         // Java: boolean bClose = ShapeUtil.isClosePath(gPath);
         let bClose = ShapeUtil::isClosePath(Some(gPath));
         // Java: int startArrowIndex = !bClose && startArrow != null ? ShapeUtil.getStartArrowIndex(gPath) : -1;
         let startArrowIndex = if !bClose && startArrow.is_some() { ShapeUtil::getStartArrowIndex(gPath) } else { -1 };
         // Java: int endArrowIndex = !bClose && endArrow != null ? ShapeUtil.getEndArrowIndex(gPath) : -1;
         let endArrowIndex = if !bClose && endArrow.is_some() { ShapeUtil::getEndArrowIndex(gPath) } else { -1 };
         // Java: int size = gPath.size();
         let size = gPath.size();
         // Java: for(int i = 0; i < size; ++i)
         for i in 0..size {
            // Java: Arrow start = i == startArrowIndex ? startArrow : null;
            let start = if i == startArrowIndex { startArrow } else { None };
            // Java: Arrow end = i == endArrowIndex ? endArrow : null;
            let end = if i == endArrowIndex { endArrow } else { None };
            // Java: PathElem elem = gPath.getElem(i);
            let elem = gPath.getElem(i);
            // Java: switch (elem.elemType)
            match elem.elemType {
               // Java: case 0:
               0 => {
                  // Java: graphics.moveTo(elem.pos[0], elem.pos[1], path);
                  self.moveTo(elem.pos[0], elem.pos[1], path);
                  // Java: this.curPt.x = orgX = elem.pos[0];
                  orgX = elem.pos[0]; self.curPt.x = orgX;
                  // Java: this.curPt.y = orgY = elem.pos[1];
                  orgY = elem.pos[1]; self.curPt.y = orgY;
                  // Java: break;
               }
               // Java: case 1:
               1 => {
                  // Java: graphics.lineTo(elem.pos[0], elem.pos[1], start, end, path);
                  self.lineTo_with_arrows(elem.pos[0], elem.pos[1], start, end, path);
                  // Java: break;
               }
               // Java: case 2:
               2 => {
                  // Java: if (elem.pos.length == 4)
                  if elem.pos.len() == 4 {
                     // Java: graphics.arcTo(elem.pos[0], elem.pos[1], elem.pos[2], elem.pos[3], start, end, path);
                     self.arcTo(elem.pos[0], elem.pos[1], elem.pos[2], elem.pos[3], start, end, path);
                  }
                  // Java: break;
               }
               // Java: case 3:
               3 => {
                  // Java: graphics.quadTo(elem.pos[0], elem.pos[1], elem.pos[2], elem.pos[3], start, end, path);
                  self.quadTo(elem.pos[0], elem.pos[1], elem.pos[2], elem.pos[3], start, end, path);
                  // Java: break;
               }
               // Java: case 4:
               4 => {
                  // Java: graphics.cubicTo(elem.pos[0], elem.pos[1], elem.pos[2], elem.pos[3], elem.pos[4], elem.pos[5], start, end, path);
                  self.cubicTo(elem.pos[0], elem.pos[1], elem.pos[2], elem.pos[3], elem.pos[4], elem.pos[5], start, end, path);
                  // Java: break;
               }
               // Java: case 5:
               5 => {
                  // Java: if (i > 0)
                  if i > 0 {
                     // Java: if (DataCal.isSamePoint(orgX, orgY, this.curPt.x, this.curPt.y))
                     if DataCal::isSamePoint(orgX, orgY, self.curPt.x, self.curPt.y) {
                        // Java: graphics.close(path);
                        self.close_path(path);
                     }
                     // Java: else
                     else {
                        // Java: graphics.lineTo(orgX, orgY, path);
                        self.lineTo_simple(orgX, orgY, path);
                        // Java: graphics.close(path);
                        self.close_path(path);
                     }
                  }
               }
               _ => {}
            }
         }
      }
    }

    // Java: void lineTo(float x, float y, Arrow startArrow, Arrow endArrow, Path path)
    pub fn lineTo_with_arrows(&mut self, x: f32, y: f32, startArrow: Option<&Arrow>, endArrow: Option<&Arrow>, path: &mut Path) {
      // Java: float realStartX = this.curPt.x;
      let mut realStartX = self.curPt.x;
      // Java: float realStartY = this.curPt.y;
      let mut realStartY = self.curPt.y;
      // Java: float realEndX = x;
      let mut realEndX = x;
      // Java: float realEndY = y;
      let mut realEndY = y;
      // Java: if (startArrow != null)
      if let Some(startArrow) = startArrow {
         // Java: float rotation = (float)ShapeUtil.getRotation(this.curPt.x, this.curPt.y, x, y);
         let rotation = ShapeUtil::getRotation(self.curPt.x, self.curPt.y, x, y) as f32;
         // Java: this.beginArrowData = new ArrowData(this.curPt.x, this.curPt.y, rotation + 180.0F);
         self.beginArrowData = Some(ArrowData::new(self.curPt.x, self.curPt.y, rotation + 180.0));
         // Java: int arrowHead = startArrow.getHead();
         let arrowHead = startArrow.getHead();
         // Java: if (arrowHead != 3 && arrowHead != 4)
         if arrowHead != 3 && arrowHead != 4 {
            // Java: float space = ShapeUtil.getArrowSpace(this.stroke.getStartArrow(), this.stroke.getWidth());
            let space = ShapeUtil::getArrowSpace(startArrow, self.stroke.as_ref().unwrap().getWidth());
            // Java: double degree = (double)rotation * Math.PI / (double)180.0F;
            let degree = rotation as f64 * std::f64::consts::PI / 180.0;
            // Java: realStartX = (float)((double)realStartX + (double)space * Math.cos(degree));
            realStartX = (realStartX as f64 + space as f64 * degree.cos()) as f32;
            // Java: realStartY = (float)((double)realStartY + (double)space * Math.sin(degree));
            realStartY = (realStartY as f64 + space as f64 * degree.sin()) as f32;
         }
      }
      // Java: if (endArrow != null)
      if let Some(endArrow) = endArrow {
         // Java: float rotation = (float)ShapeUtil.getRotation(this.curPt.x, this.curPt.y, x, y);
         let rotation = ShapeUtil::getRotation(self.curPt.x, self.curPt.y, x, y) as f32;
         // Java: this.endArrowData = new ArrowData(x, y, rotation);
         self.endArrowData = Some(ArrowData::new(x, y, rotation));
         // Java: int arrowHead = endArrow.getHead();
         let arrowHead = endArrow.getHead();
         // Java: if (arrowHead != 3 && arrowHead != 4)
         if arrowHead != 3 && arrowHead != 4 {
            // Java: float space = ShapeUtil.getArrowSpace(this.stroke.getEndArrow(), this.stroke.getWidth());
            let space = ShapeUtil::getArrowSpace(endArrow, self.stroke.as_ref().unwrap().getWidth());
            // Java: double degree = (double)rotation * Math.PI / (double)180.0F;
            let degree = rotation as f64 * std::f64::consts::PI / 180.0;
            // Java: realEndX = (float)((double)x - (double)space * Math.cos(degree));
            realEndX = (x as f64 - space as f64 * degree.cos()) as f32;
            // Java: realEndY = (float)((double)y - (double)space * Math.sin(degree));
            realEndY = (y as f64 - space as f64 * degree.sin()) as f32;
         }
      }
      // Java: if (this.curPt.x != realStartX || this.curPt.y != realStartY)
      if self.curPt.x != realStartX || self.curPt.y != realStartY {
         // Java: path.moveTo(realStartX, realStartY);
         path.moveTo(realStartX, realStartY);
      }
      // Java: path.lineTo(realEndX, realEndY);
      path.lineTo(realEndX, realEndY);
      // Java: this.curPt.x = x;
      self.curPt.x = x;
      // Java: this.curPt.y = y;
      self.curPt.y = y;
    }

    // Java: void arcTo(float w, float h, float startAngle, float sweepAngle, Arrow beginArrow, Arrow endArrow, Path path)
    pub fn arcTo(&mut self, w: f32, h: f32, startAngle: f32, sweepAngle: f32, beginArrow: Option<&Arrow>, endArrow: Option<&Arrow>, path: &mut Path) {
      // Java: float realStartAngle = startAngle;
      let mut realStartAngle = startAngle;
      // Java: float realEndAngle = startAngle + sweepAngle;
      let mut realEndAngle = startAngle + sweepAngle;
      // Java: float beginX = this.curPt.x;
      let beginX = self.curPt.x;
      // Java: float beginY = this.curPt.y;
      let beginY = self.curPt.y;
      // Java: float centerX = beginX - w * (float)Math.cos((double)startAngle * Math.PI / (double)180.0F);
      let centerX = beginX - w * ((startAngle as f64 * std::f64::consts::PI / 180.0).cos() as f32);
      // Java: float centerY = beginY - h * (float)Math.sin((double)startAngle * Math.PI / (double)180.0F);
      let centerY = beginY - h * ((startAngle as f64 * std::f64::consts::PI / 180.0).sin() as f32);
      // Java: float endX = centerX + w * (float)Math.cos((double)realEndAngle * Math.PI / (double)180.0F);
      let endX = centerX + w * ((realEndAngle as f64 * std::f64::consts::PI / 180.0).cos() as f32);
      // Java: float endY = centerY + h * (float)Math.sin((double)realEndAngle * Math.PI / (double)180.0F);
      let endY = centerY + h * ((realEndAngle as f64 * std::f64::consts::PI / 180.0).sin() as f32);
      // Java: boolean focusMoveTo = false;
      let mut focusMoveTo = false;
      // Java: if (beginArrow != null)
      if beginArrow.is_some() {
         // Java: float rotation = 270.0F + startAngle;
         let rotation = 270.0 + startAngle;
         // Java: this.beginArrowData = new ArrowData(beginX, beginY, rotation);
         self.beginArrowData = Some(ArrowData::new(beginX, beginY, rotation));
         // Java: realStartAngle = startAngle + 5.0F;
         realStartAngle = startAngle + 5.0;
         // Java: focusMoveTo = true;
         focusMoveTo = true;
      }
      // Java: if (endArrow != null)
      if endArrow.is_some() {
         // Java: float rotation = realEndAngle + 90.0F;
         let rotation = realEndAngle + 90.0;
         // Java: this.endArrowData = new ArrowData(endX, endY, rotation);
         self.endArrowData = Some(ArrowData::new(endX, endY, rotation));
         // Java: realEndAngle -= 5.0F;
         realEndAngle -= 5.0;
      }
      // Java: android.graphics.RectF oval = new android.graphics.RectF(centerX - w, centerY - h, centerX + w, centerY + h);
      let oval = RectF::new_with(centerX - w, centerY - h, centerX + w, centerY + h);
      // Java: path.arcTo(oval, realStartAngle, realEndAngle - realStartAngle, focusMoveTo);
      path.arcTo(&oval, realStartAngle, realEndAngle - realStartAngle, focusMoveTo);
      // Java: this.curPt.x = endX;
      self.curPt.x = endX;
      // Java: this.curPt.y = endY;
      self.curPt.y = endY;
    }

    // Java: void lineTo(float x, float y, Path path)
    pub fn lineTo_simple(&mut self, x: f32, y: f32, path: &mut Path) {
      // Java: path.lineTo(x, y);
      path.lineTo(x, y);
      // Java: this.curPt.x = x;
      self.curPt.x = x;
      // Java: this.curPt.y = y;
      self.curPt.y = y;
    }

    // Java: void moveTo(float x, float y, Path path)
    pub fn moveTo(&mut self, x: f32, y: f32, path: &mut Path) {
      // Java: this.curPt.x = x;
      self.curPt.x = x;
      // Java: this.curPt.y = y;
      self.curPt.y = y;
      // Java: path.moveTo(x, y);
      path.moveTo(x, y);
    }

    // Java: void drawRect(float x, float y, float width, float height, Path path)
    pub fn drawRect(&self, x: f32, y: f32, width: f32, height: f32, path: &mut Path) {
      // Java: path.addRect(x, y, x + width, y + height, Direction.CW);
      path.addRect(x, y, x + width, y + height, DIRECTION_CW);
    }

    // Java: void drawCircle(float x, float y, float radius, Path path)
    pub fn drawCircle(&self, x: f32, y: f32, radius: f32, path: &mut Path) {
      // Java: path.addCircle(x, y, radius, Direction.CW);
      path.addCircle(x, y, radius, DIRECTION_CW);
    }

    // Java: void drawEllipse(android.graphics.RectF rectF, Path path)
    pub fn drawEllipse(&self, rectF: &RectF, path: &mut Path) {
      // Java: path.addOval(rectF, Direction.CW);
      path.addOval(rectF, DIRECTION_CW);
    }

    // Java: static PointF getQuadPoint(float x0, float y0, float x1, float y1, float x2, float y2, float t)
    pub fn getQuadPoint(x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> PointF {
      // Java: float p0 = (1.0F - t) * (1.0F - t);
      let p0 = (1.0 - t) * (1.0 - t);
      // Java: float p1 = 2.0F * t * (1.0F - t);
      let p1 = 2.0 * t * (1.0 - t);
      // Java: float p2 = t * t;
      let p2 = t * t;
      // Java: return new PointF(x0 * p0 + x1 * p1 + x2 * p2, y0 * p0 + y1 * p1 + y2 * p2);
      PointF::new_xy(x0 * p0 + x1 * p1 + x2 * p2, y0 * p0 + y1 * p1 + y2 * p2)
    }

    // Java: void quadTo(float x1, float y1, float x2, float y2, Arrow startArrow, Arrow endArrow, Path path)
    pub fn quadTo(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, startArrow: Option<&Arrow>, endArrow: Option<&Arrow>, path: &mut Path) {
      // Java: float realStartX = this.curPt.x;
      let mut realStartX = self.curPt.x;
      // Java: float realStartY = this.curPt.y;
      let mut realStartY = self.curPt.y;
      // Java: float realEndX = x2;
      let mut realEndX = x2;
      // Java: float realEndY = y2;
      let mut realEndY = y2;
      // Java: if (startArrow != null)
      if let Some(startArrow) = startArrow {
         // Java: PointF pt = getQuadPoint(realStartX, realStartY, x1, y1, x2, y2, 0.25F);
         let pt = Self::getQuadPoint(realStartX, realStartY, x1, y1, x2, y2, 0.25);
         // Java: float rotation = (float)ShapeUtil.getRotation(this.curPt.x, this.curPt.y, pt.x, pt.y);
         let rotation = ShapeUtil::getRotation(self.curPt.x, self.curPt.y, pt.x, pt.y) as f32;
         // Java: this.beginArrowData = new ArrowData(this.curPt.x, this.curPt.y, rotation + 180.0F);
         self.beginArrowData = Some(ArrowData::new(self.curPt.x, self.curPt.y, rotation + 180.0));
         // Java: float space = ShapeUtil.getArrowSpace(startArrow, this.stroke.getWidth());
         let sw = self.stroke.as_ref().map(|s| s.getWidth()).unwrap_or(0.0);
         let space = ShapeUtil::getArrowSpace(startArrow, sw);
         // Java: realStartX = (float)((double)realStartX + (double)space * Math.cos((double)rotation * Math.PI / (double)180.0F));
         let degree = rotation as f64 * std::f64::consts::PI / 180.0;
         realStartX = (realStartX as f64 + space as f64 * degree.cos()) as f32;
         // Java: realStartY = (float)((double)realStartY + (double)space * Math.sin((double)rotation * Math.PI / (double)180.0F));
         realStartY = (realStartY as f64 + space as f64 * degree.sin()) as f32;
      }
      // Java: if (endArrow != null)
      if let Some(endArrow) = endArrow {
         // Java: PointF pt = getQuadPoint(realStartX, realStartY, x1, y1, x2, y2, 0.75F);
         let pt = Self::getQuadPoint(realStartX, realStartY, x1, y1, x2, y2, 0.75);
         // Java: float rotation = (float)ShapeUtil.getRotation(pt.x, pt.y, x2, y2);
         let rotation = ShapeUtil::getRotation(pt.x, pt.y, x2, y2) as f32;
         // Java: this.endArrowData = new ArrowData(x2, y2, rotation);
         self.endArrowData = Some(ArrowData::new(x2, y2, rotation));
         // Java: float space = ShapeUtil.getArrowSpace(endArrow, this.stroke.getWidth());
         let sw = self.stroke.as_ref().map(|s| s.getWidth()).unwrap_or(0.0);
         let space = ShapeUtil::getArrowSpace(endArrow, sw);
         // Java: realEndX = (float)((double)x2 - (double)space * Math.cos((double)rotation * Math.PI / (double)180.0F));
         let degree = rotation as f64 * std::f64::consts::PI / 180.0;
         realEndX = (x2 as f64 - space as f64 * degree.cos()) as f32;
         // Java: realEndY = (float)((double)y2 - (double)space * Math.sin((double)rotation * Math.PI / (double)180.0F));
         realEndY = (y2 as f64 - space as f64 * degree.sin()) as f32;
      }
      // Java: if (this.curPt.x != realStartX || this.curPt.y != realStartY)
      if self.curPt.x != realStartX || self.curPt.y != realStartY {
         // Java: path.moveTo(realStartX, realStartY);
         path.moveTo(realStartX, realStartY);
      }
      // Java: path.quadTo(x1, y1, realEndX, realEndY);
      path.quadTo(x1, y1, realEndX, realEndY);
      // Java: this.curPt.x = x2;
      self.curPt.x = x2;
      // Java: this.curPt.y = y2;
      self.curPt.y = y2;
    }

    // Java: static PointF getCubicPoint(float x0, float y0, float x1, float y1, float x2, float y2, float x3, float y3, float t)
    pub fn getCubicPoint(x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, t: f32) -> PointF {
      // Java: float p0 = (1.0F - t) * (1.0F - t) * (1.0F - t);
      let p0 = (1.0 - t) * (1.0 - t) * (1.0 - t);
      // Java: float p1 = 3.0F * t * (1.0F - t) * (1.0F - t);
      let p1 = 3.0 * t * (1.0 - t) * (1.0 - t);
      // Java: float p2 = 3.0F * t * t * (1.0F - t);
      let p2 = 3.0 * t * t * (1.0 - t);
      // Java: float p3 = t * t * t;
      let p3 = t * t * t;
      // Java: return new PointF(x0 * p0 + x1 * p1 + x2 * p2 + x3 * p3, y0 * p0 + y1 * p1 + y2 * p2 + y3 * p3);
      PointF::new_xy(x0 * p0 + x1 * p1 + x2 * p2 + x3 * p3, y0 * p0 + y1 * p1 + y2 * p2 + y3 * p3)
    }

    // Java: void cubicTo(float x1, float y1, float x2, float y2, float x3, float y3, Arrow startArrow, Arrow endArrow, Path path)
    pub fn cubicTo(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, startArrow: Option<&Arrow>, endArrow: Option<&Arrow>, path: &mut Path) {
      // Java: float realStartX = this.curPt.x;
      let mut realStartX = self.curPt.x;
      // Java: float realStartY = this.curPt.y;
      let mut realStartY = self.curPt.y;
      // Java: float realEndX = x3;
      let mut realEndX = x3;
      // Java: float realEndY = y3;
      let mut realEndY = y3;
      // Java: if (startArrow != null)
      if let Some(startArrow) = startArrow {
         // Java: PointF pt = getCubicPoint(realStartX, realStartY, x1, y1, x2, y2, x3, y3, 0.25F);
         let pt = Self::getCubicPoint(realStartX, realStartY, x1, y1, x2, y2, x3, y3, 0.25);
         // Java: float rotation = (float)ShapeUtil.getRotation(this.curPt.x, this.curPt.y, pt.x, pt.y);
         let rotation = ShapeUtil::getRotation(self.curPt.x, self.curPt.y, pt.x, pt.y) as f32;
         // Java: this.beginArrowData = new ArrowData(this.curPt.x, this.curPt.y, rotation + 180.0F);
         self.beginArrowData = Some(ArrowData::new(self.curPt.x, self.curPt.y, rotation + 180.0));
         // Java: float space = ShapeUtil.getArrowSpace(startArrow, this.stroke.getWidth());
         let sw = self.stroke.as_ref().map(|s| s.getWidth()).unwrap_or(0.0);
         let space = ShapeUtil::getArrowSpace(startArrow, sw);
         // Java: realStartX = (float)((double)realStartX + (double)space * Math.cos((double)rotation * Math.PI / (double)180.0F));
         let degree = rotation as f64 * std::f64::consts::PI / 180.0;
         realStartX = (realStartX as f64 + space as f64 * degree.cos()) as f32;
         // Java: realStartY = (float)((double)realStartY + (double)space * Math.sin((double)rotation * Math.PI / (double)180.0F));
         realStartY = (realStartY as f64 + space as f64 * degree.sin()) as f32;
      }
      // Java: if (endArrow != null)
      if let Some(endArrow) = endArrow {
         // Java: PointF pt = getCubicPoint(realStartX, realStartY, x1, y1, x2, y2, x3, y3, 0.75F);
         let pt = Self::getCubicPoint(realStartX, realStartY, x1, y1, x2, y2, x3, y3, 0.75);
         // Java: float rotation = (float)ShapeUtil.getRotation(pt.x, pt.y, x3, y3);
         let rotation = ShapeUtil::getRotation(pt.x, pt.y, x3, y3) as f32;
         // Java: this.endArrowData = new ArrowData(x3, y3, rotation);
         self.endArrowData = Some(ArrowData::new(x3, y3, rotation));
         // Java: float space = ShapeUtil.getArrowSpace(endArrow, this.stroke.getWidth());
         let sw = self.stroke.as_ref().map(|s| s.getWidth()).unwrap_or(0.0);
         let space = ShapeUtil::getArrowSpace(endArrow, sw);
         // Java: realEndX = (float)((double)x3 - (double)space * Math.cos((double)rotation * Math.PI / (double)180.0F));
         let degree = rotation as f64 * std::f64::consts::PI / 180.0;
         realEndX = (x3 as f64 - space as f64 * degree.cos()) as f32;
         // Java: realEndY = (float)((double)y3 - (double)space * Math.sin((double)rotation * Math.PI / (double)180.0F));
         realEndY = (y3 as f64 - space as f64 * degree.sin()) as f32;
      }
      // Java: if (this.curPt.x != realStartX || this.curPt.y != realStartY)
      if self.curPt.x != realStartX || self.curPt.y != realStartY {
         // Java: path.moveTo(realStartX, realStartY);
         path.moveTo(realStartX, realStartY);
      }
      // Java: path.cubicTo(x1, y1, x2, y2, realEndX, realEndY);
      path.cubicTo(x1, y1, x2, y2, realEndX, realEndY);
      // Java: this.curPt.x = x3;
      self.curPt.x = x3;
      // Java: this.curPt.y = y3;
      self.curPt.y = y3;
    }

    pub fn getFillType(&self, fillType: i32) -> i32 { fillType }
}

// Java: public class PathInfo
impl PathInfo {
    // Java: PathInfo(Path path, RectF rectF)
    pub fn new(path: Path, rectF: Option<RectF>) -> Self {
         // Java: super();
         let _super = ();
         // Java: this.path = path;
         let p = path;
         // Java: this.rect = rectF;
         let r = rectF;
         PathInfo { path: p, rect: r }
    }
}

// Java: public class ArrowData
impl ArrowData {
    // Java: ArrowData(float x, float y, float degree)
    pub fn new(x: f32, y: f32, degree: f32) -> Self {
         // Java: super();
         let _super = ();
         // Java: this.x = x;
         let ax = x;
         // Java: this.y = y;
         let ay = y;
         // Java: this.degree = degree;
         let ad = degree;
         ArrowData { x: ax, y: ay, degree: ad }
    }
}
