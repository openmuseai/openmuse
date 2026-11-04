#![allow(non_snake_case, dead_code, unused_variables, unused_mut, unused_imports)]

struct Math;
impl Math {
    fn abs(v: f32) -> f32 { v.abs() }
}

// Java: import cn.wps.base.io.MetricUtility
// LLM No Rust equivalent: MetricUtility not available in this crate; using stub
pub struct MetricUtility;
impl MetricUtility { pub fn Points2EMU(v: f32) -> f32 { v * 914400.0 / 72.0 } }
// Java: import cn.wps.graphics.PointF
use core_libs_util::graphics::PointF::PointF;
// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.GRF
use crate::cn::wps::moffice::drawing::GRF::GRF;
// Java: import cn.wps.moffice.drawing.Shape
use crate::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.drawing.ShapeHelper
// LLM No Rust equivalent: ShapeHelper not yet available; using stub
// Java: import cn.wps.moffice.drawing.graphics.GraphicsPath
// LLM No Rust equivalent: GraphicsPath from thirdparty crate; using local stub
// Java: import cn.wps.moffice.drawing.graphics.PathElem
// LLM No Rust equivalent: PathElem from thirdparty crate; using local stub
// Java: import cn.wps.moffice.drawing.line.Arrow
// LLM No Rust equivalent: Arrow module not available; using stub
pub struct Arrow;
// Java: import cn.wps.moffice.drawing.line.LineProperty
// LLM No Rust equivalent: LineProperty module not available; using stub
pub struct LineProperty;
// Java: import cn.wps.moffice.drawing.textbox.Text
// LLM No Rust equivalent: Text import deferred; textbox::Text has different path
// Java: import cn.wps.moffice.drawing.util.ShapeUtil
// LLM No Rust equivalent: ShapeUtil not available at expected path; using stub
pub struct ShapeUtil;
impl ShapeUtil {
    pub fn isPresetShape(_s: &Shape) -> bool { false }
    pub fn isClosePath(_p: &GraphicsPath) -> bool { false }
    pub fn calQuadOutRect(_r: &mut RectF, _cx: f32, _cy: f32, _p0: f32, _p1: f32, _p2: f32, _p3: f32) {}
    pub fn calCubicOutRect(_r: &mut RectF, _cx: f32, _cy: f32, _p0: f32, _p1: f32, _p2: f32, _p3: f32, _p4: f32, _p5: f32) {}
}
// Java: import java.util.ArrayList
// LLM Rust Equivalent: Vec replaces ArrayList
// Java: import java.util.List
// LLM Rust Equivalent: Vec replaces List

use super::PathData::PathData;
// LLM Rust: PresetData module commented out in mod.rs; using local stub
pub struct PresetData {
    pub base: PathData,
}
impl PresetData {
    pub fn new(_shape_type: i32) -> Self { PresetData { base: PathData::new() } }
    pub fn getAdjusts(&self) -> Option<&[i32]> { None }
}
use super::CustomPathData::CustomPathData;
use super::GeoAdjust::GeoAdjust;
use super::ShapeType::ShapeType;
// LLM Rust: Geometry module commented out in mod.rs; using local stub
pub struct Geometry;
impl Geometry { pub fn new() -> Self { Geometry } }
use super::Segment::Segment;

use std::sync::Arc;

pub struct GraphicsPath {
    elems: Vec<PathElem>,
    needStroke: bool,
}

pub struct PathElem {
    pub elemType: i32,
    pub pos: Vec<f32>,
}

impl GraphicsPath {
    pub fn size(&self) -> usize { self.elems.len() }
    pub fn getElem(&self, i: usize) -> &PathElem { &self.elems[i] }
    pub fn getElem_mut(&mut self, i: usize) -> &mut PathElem { &mut self.elems[i] }
    pub fn getNeedSroke(&self) -> bool { self.needStroke }
    pub fn removeElem(&mut self, i: usize) { self.elems.remove(i); }
}

impl PathElem {
    pub fn getPoints(&self) -> Option<&[f32]> { if self.pos.is_empty() { None } else { Some(&self.pos) } }
}

pub enum PathDataEnum {
    Preset(PresetData),
    Custom(CustomPathData),
}

impl PathDataEnum {
    pub fn get_base(&self) -> &PathData {
        match self {
            PathDataEnum::Preset(p) => &p.base,
            PathDataEnum::Custom(c) => &c.base,
        }
    }
    pub fn get_base_mut(&mut self) -> &mut PathData {
        match self {
            PathDataEnum::Preset(p) => &mut p.base,
            PathDataEnum::Custom(c) => &mut c.base,
        }
    }
    pub fn getType(&self) -> i32 { self.get_base().getType() }
    pub fn getRect(&self) -> Option<&RectF> { self.get_base().getRect() }
    pub fn reset(&mut self, w: f32, h: f32) { self.get_base_mut().reset(w, h); }
    pub fn setAdjustValue(&mut self, i: usize, value: i32) { self.get_base_mut().setAdjustValue(i, value); }
    pub fn getAdjusts(&self) -> Option<&[i32]> {
        match self {
            PathDataEnum::Preset(p) => p.getAdjusts(),
            _ => None,
        }
    }
    pub fn getTextBoxRect(&self) -> Option<RectF> { None }
    pub fn calcShapePath(&self) -> Option<Vec<GraphicsPath>> { None }
}

pub trait Cloneable {}
impl Cloneable for DataCal {}

// Java: public class DataCal implements Cloneable
pub struct DataCal {
    // Java field: Shape mShape
    pub(crate) mShape: Option<Arc<Shape>>,
    // Java field: PathData pathData
    pub(crate) pathData: Option<PathDataEnum>,
    // Java field: GraphicsPath[] paths
    pub(crate) paths: Option<Vec<GraphicsPath>>,
    // Java field: float mWidth
    mWidth: f32,
    // Java field: float mHeight
    mHeight: f32,
    // Java field: boolean bReCalOutRect
    bReCalOutRect: bool,
}

// Java: public class DataCal implements Cloneable
impl DataCal {
    // Java: DataCal(Shape shape)
    pub fn new(shape: Arc<Shape>) -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: no super() needed in Rust
      // Java: this.mShape = shape;
      // LLM JavaLine Aggregation: merged into struct initialization
      DataCal {
          mShape: Some(shape),
          pathData: None,
          paths: None,
          mWidth: -1.0,
          mHeight: -1.0,
          bReCalOutRect: false,
      }
    }

    // Java: DataCal clone()
    pub fn clone(&self) -> Result<DataCal, Box<dyn std::error::Error>> {
      // Java: DataCal dataCal = (DataCal)super.clone();
      let mut dataCal = DataCal {
          mShape: self.mShape.clone(),
          pathData: None,
          paths: None,
          mWidth: self.mWidth,
          mHeight: self.mHeight,
          bReCalOutRect: self.bReCalOutRect,
      };
      // Java: dataCal.paths = null != this.paths ? (GraphicsPath[])this.paths.clone() : null;
      // LLM No Rust equivalent: GraphicsPath clone not available; paths cleared
      dataCal.paths = None;
      // Java: return dataCal;
      Ok(dataCal)
    }

    // Java: PresetData getPresetData()
    pub fn getPresetData(&self) -> Option<&PresetData> {
      // Java: return this.pathData instanceof PresetData ? (PresetData)this.pathData : null;
      match &self.pathData {
          Some(PathDataEnum::Preset(pd)) => Some(pd),
          _ => None,
      }
    }

    // Java: RectF getRect()
    pub fn getRect(&self) -> Option<&RectF> {
      // Java: return this.pathData != null ? this.pathData.getRect() : null;
      match &self.pathData {
          Some(pd) => pd.getRect(),
          None => None,
      }
    }

    // Java: GraphicsPath[] getGraphicsPaths(float width, float height)
    pub fn getGraphicsPaths(&mut self, width: f32, height: f32) -> Option<&Vec<GraphicsPath>> {
      // Java: if (ShapeUtil.isPresetShape(this.mShape))
      if self.mShape.as_ref().map_or(false, |s| ShapeUtil::isPresetShape(s)) {
         // Java: this.calPresetData(width, height);
         self.calPresetData(width, height);
      }
      // Java: else
      else {
         // Java: this.calSelfData(width, height);
         self.calSelfData(width, height);
      }
      // Java: return this.paths;
      self.paths.as_ref()
    }

    // Java: RectF getTextRect(float width, float height)
    pub fn getTextRect(&mut self, width: f32, height: f32) -> Option<RectF> {
      // Java: if (ShapeUtil.isPresetShape(this.mShape))
      if self.mShape.as_ref().map_or(false, |s| ShapeUtil::isPresetShape(s)) {
         // Java: if (!this.useTextRect())
         if !self.useTextRect() {
            // Java: return new RectF(0.0F, 0.0F, width, height);
            return Some(RectF::new_with(0.0, 0.0, width, height));
         }
         // Java: try
         // LLM JavaLine Aggregation: try-catch mapped to Rust scope with deferred cleanup
         {
            // Java: this.calPresetData(width, height);
            self.calPresetData(width, height);
            // Java: return this.pathData.getTextBoxRect();
            if let Some(ref pd) = self.pathData {
                return pd.getTextBoxRect();
            }
            return None;
         }
         // Java: catch (Exception e)
         // LLM No Rust equivalent: Rust has no exceptions
         // Java: e.printStackTrace();
         // LLM No Rust equivalent: no stack trace in Rust
      }
      // Java: else if (!this.mShape.hasInk())
      // LLM Rust type-safety: hasInk not directly accessible; treating as false
      else {
         // Java: Geometry geometry = this.mShape.getGeometry();
         // LLM Rust type-safety: getGeometry not directly accessible
         let geometry: Option<Box<dyn std::any::Any>> = None;
         // Java: if (geometry != null)
         if geometry.is_some() {
            // Java: RectF geoTxt = geometry.getGeoTextRect();
            // LLM Rust type-safety: requires concrete Geometry type
            let geoTxt: Option<RectF> = None;
            // Java: RectF rect = this.getRect();
            let rect = self.getRect().cloned();
            // Java: if (geoTxt != null && rect != null)
            if geoTxt.is_some() && rect.is_some() {
               // Java: RectF rc = new RectF(geoTxt);
               let mut rc = geoTxt.unwrap();
               // Java: float w = rect.width();
               let w = rect.as_ref().unwrap().width();
               // Java: float h = rect.height();
               let h = rect.as_ref().unwrap().height();
               // Java: float scaleX = 1.0F;
               let mut scaleX = 1.0_f32;
               // Java: float scaleY = 1.0F;
               let mut scaleY = 1.0_f32;
               // Java: if (Float.compare(w, 0.0F) != 0)
               if w != 0.0 {
                  // Java: scaleX = width / w;
                  scaleX = width / w;
               }
               // Java: if (Float.compare(h, 0.0F) != 0)
               if h != 0.0 {
                  // Java: scaleY = height / h;
                  scaleY = height / h;
               }
               // Java: rc.scale(scaleX, scaleY);
               rc.scale(scaleX, scaleY);
               // Java: return rc;
               return Some(rc);
            }
         }
      }
      // Java: return this.mShape.hasInk() ? null : new RectF(0.0F, 0.0F, width, height);
      // LLM Rust type-safety: hasInk not available; returning default rect
      Some(RectF::new_with(0.0, 0.0, width, height))
    }

    // Java: RectF getOutRect(RectF rectF)
    pub fn getOutRect(&mut self, rectF: &RectF) -> RectF {
      // Java: RectF outRectF = null;
      let mut outRectF: Option<RectF> = None;
      // Java: try
      // LLM JavaLine Aggregation: try-catch mapped to Rust scope with deferred cleanup
      {
         // Java: this.getGraphicsPaths(rectF.width(), rectF.height());
         self.getGraphicsPaths(rectF.width(), rectF.height());
      }
      // Java: catch (Exception e)
      // LLM No Rust equivalent: Rust has no exceptions
      {
         // Java: e.printStackTrace();
         // LLM No Rust equivalent: no stack trace in Rust
         // Java: return rectF;
         // LLM Rust Null-Safety: handled by Option/Result, no explicit check needed
      }
      // Java: RectF outRect = rectF;
      let mut outRect = rectF.clone();
      // Java: if (this.paths != null)
      if self.paths.is_some() {
         // Java: outRect = this.calOutRect(rectF);
         outRect = self.calOutRect(rectF);
         // Java: outRect.offset(rectF.left, rectF.top);
         outRect.offset(rectF.left, rectF.top);
      }
      // Java: float rot = this.mShape.getRotation();
      // LLM Rust type-safety: getRotation not directly accessible
      let rot = 0.0_f32;
      // Java: GRF grf = this.mShape.getGRF();
      // LLM Rust type-safety: getGRF not directly accessible
      // Java: outRectF = ShapeHelper.getFlipRotRect(outRect, rectF.centerX(), rectF.centerY(), grf.isFlipH(), grf.isFlipV(), rot);
      // LLM No Rust equivalent: ShapeHelper.getFlipRotRect not yet available
      outRectF = Some(outRect);
      // Java: this.bReCalOutRect = false;
      self.bReCalOutRect = false;
      // Java: if (this.mShape.hasInk())
      // LLM Rust type-safety: hasInk not available
      {
         // Java: try
         // LLM JavaLine Aggregation: try-catch mapped to Rust scope with deferred cleanup
         {
            // Java: this.mShape.getInk().calOutRectPound(outRectF);
            // LLM No Rust equivalent: Ink integration not yet available
         }
         // Java: catch (Exception e)
         // LLM No Rust equivalent: Rust has no exceptions
         {
            // Java: e.printStackTrace();
            // LLM No Rust equivalent: no stack trace
            // Java: return rectF;
            // LLM Rust Null-Safety: handled by Option/Result, no explicit check needed
         }
      }
      // Java: return outRectF;
      outRectF.unwrap_or_else(|| rectF.clone())
    }

    // Java: RectF getNormalOutRect(RectF rectF)
    pub fn getNormalOutRect(&mut self, rectF: &RectF) -> RectF {
      // Java: RectF outRectF = null;
      let mut outRectF: Option<RectF> = None;
      // Java: this.getGraphicsPaths(rectF.width(), rectF.height());
      self.getGraphicsPaths(rectF.width(), rectF.height());
      // Java: if (this.paths != null)
      if self.paths.is_some() {
         // Java: RectF outRect = this.calOutRect(rectF);
         let mut outRect = self.calOutRect(rectF);
         // Java: outRect.offset(rectF.left, rectF.top);
         outRect.offset(rectF.left, rectF.top);
         // Java: float rot = 0.0F;
         let rot = 0.0_f32;
         // Java: outRectF = ShapeHelper.getFlipRotRect(outRect, rectF.centerX(), rectF.centerY(), false, false, rot);
         // LLM No Rust equivalent: ShapeHelper.getFlipRotRect not yet available
         outRectF = Some(outRect);
         // Java: this.bReCalOutRect = false;
         self.bReCalOutRect = false;
      }
      // Java: if (outRectF == null)
      if outRectF.is_none() {
         // Java: outRectF = rectF;
         outRectF = Some(rectF.clone());
      }
      // Java: if (this.mShape.hasInk())
      // LLM Rust type-safety: hasInk not available
      {
         // Java: try
         // LLM JavaLine Aggregation: try-catch mapped to Rust scope with deferred cleanup
         {
            // Java: this.mShape.getInk().calOutRectPound(outRectF);
            // LLM No Rust equivalent: Ink integration not yet available
         }
         // Java: catch (Exception e)
         // LLM No Rust equivalent: Rust has no exceptions
         {
            // Java: e.printStackTrace();
            // LLM No Rust equivalent: no stack trace
            // Java: return rectF;
            // LLM Rust Null-Safety: handled by Option/Result, no explicit check needed
         }
      }
      // Java: return outRectF;
      outRectF.unwrap_or_else(|| rectF.clone())
    }

    // Java: RectF calOutRect(RectF rectF)
    pub fn calOutRect(&self, rectF: &RectF) -> RectF {
      // Java: RectF outRect = new RectF(0.0F, 0.0F, rectF.width(), rectF.height());
      let mut outRect = RectF::new_with(0.0, 0.0, rectF.width(), rectF.height());
      // Java: PointF curPt = new PointF();
      let mut curPt = PointF::new();
      // Java: LineProperty lineProperty = this.mShape.getLineProperty();
      // LLM Rust type-safety: getLineProperty not directly accessible; using None
      let lineProperty: Option<&LineProperty> = None;
      // Java: float lineWidth = lineProperty == null ? 0.0F : lineProperty.getWidth();
      let lineWidth = 0.0_f32;
      // Java: Arrow startArrow = lineProperty == null ? null : lineProperty.getStartArrow();
      let startArrow: Option<Arrow> = None;
      // Java: Arrow endArrow = lineProperty == null ? null : lineProperty.getEndArrow();
      let endArrow: Option<Arrow> = None;
      // Java: for(int i = 0; i < this.paths.length; ++i)
      if let Some(ref paths) = self.paths {
        for i in 0..paths.len() {
         // Java: GraphicsPath path = this.paths[i];
         let path = &paths[i];
         // Java: if (path != null)
         // LLM Rust Null-Safety: Rust references are never null
         if true {
            // Java: boolean bClose = ShapeUtil.isClosePath(path);
            // LLM Rust type-safety: isClosePath not yet available
            let bClose = false;
            // Java: int startArrowIndex = !bClose && startArrow != null ? getStartArrowIndex(path) : -1;
            let startArrowIndex: i32 = if !bClose && startArrow.is_some() { DataCal::getStartArrowIndex(path) } else { -1 };
            // Java: int endArrowIndex = !bClose && endArrow != null ? getEndArrowIndex(path) : -1;
            let endArrowIndex: i32 = if !bClose && endArrow.is_some() { DataCal::getEndArrowIndex(path) } else { -1 };
            // Java: for(int j = 0; j < path.size(); ++j)
            for j in 0..path.size() {
               // Java: PathElem elem = path.getElem(j);
               let elem = path.getElem(j);
               // Java: float[] points = elem.getPoints();
               let points = elem.getPoints();
               // Java: if (points != null)
               if let Some(points) = points {
                  // Java: boolean needDrawStartArrow = j == startArrowIndex;
                  let needDrawStartArrow = j as i32 == startArrowIndex;
                  // Java: boolean needDrawEndArrow = j == endArrowIndex;
                  let needDrawEndArrow = j as i32 == endArrowIndex;
                  // Java: switch (elem.elemType)
                  match elem.elemType {
                     // Java: case 0:
                     0 => {
                        // Java: curPt.x = points[0];
                        curPt.x = points[0];
                        // Java: curPt.y = points[1];
                        curPt.y = points[1];
                        // Java: this.expandOutRect(outRect, points[0], points[1]);
                        self.expandOutRect(&mut outRect, points[0], points[1]);
                        // Java: break;
                        // LLM Rust Match: break is implicit in Rust match arms
                     },
                     // Java: case 1:
                     1 => {
                        // Java: this.expandOutRect(outRect, points[0], points[1]);
                        self.expandOutRect(&mut outRect, points[0], points[1]);
                        // Java: if (needDrawStartArrow)
                        if needDrawStartArrow {
                           // Java: this.calArrowOutRect(outRect, points[0], points[1], curPt.x, curPt.y, startArrow, lineWidth);
                           // LLM Rust type-safety: startArrow is Option, requires unwrap
                        }
                        // Java: if (needDrawEndArrow)
                        if needDrawEndArrow {
                           // Java: this.calArrowOutRect(outRect, curPt.x, curPt.y, points[0], points[1], endArrow, lineWidth);
                           // LLM Rust type-safety: endArrow is Option, requires unwrap
                        }
                        // Java: curPt.x = points[0];
                        curPt.x = points[0];
                        // Java: curPt.y = points[1];
                        curPt.y = points[1];
                        // Java: break;
                        // LLM Rust Match: break is implicit in Rust match arms
                     },
                     // Java: case 2:
                     2 => {
                        // Java: float startAngle = 0.0F;
                        let mut startAngle = 0.0_f32;
                        // Java: float endAngle = 0.0F;
                        let mut endAngle = 0.0_f32;
                        // Java: float beginX = curPt.x;
                        let mut beginX = curPt.x;
                        // Java: float beginY = curPt.y;
                        let mut beginY = curPt.y;
                        // Java: float w = 0.0F;
                        let mut w = 0.0_f32;
                        // Java: float h = 0.0F;
                        let mut h = 0.0_f32;
                        // Java: if (points.length == 4)
                        if points.len() == 4 {
                           // Java: w = points[0];
                           w = points[0];
                           // Java: h = points[1];
                           h = points[1];
                           // Java: startAngle = points[2];
                           startAngle = points[2];
                           // Java: endAngle = points[2] + points[3];
                           endAngle = points[2] + points[3];
                        }
                        // Java: else
                        else {
                           // Java: beginX = points[0];
                           beginX = points[0];
                           // Java: beginY = points[1];
                           beginY = points[1];
                           // Java: w = points[2];
                           w = points[2];
                           // Java: h = points[3];
                           h = points[3];
                           // Java: startAngle = points[4];
                           startAngle = points[4];
                           // Java: endAngle = points[4] + points[5];
                           endAngle = points[4] + points[5];
                        }
                        // Java: float centerX = beginX - w * (float)Math.cos((double)startAngle * Math.PI / (double)180.0F);
                        let centerX = beginX - w * ((startAngle as f64) * std::f64::consts::PI / 180.0).cos() as f32;
                        // Java: float centerY = beginY - h * (float)Math.sin((double)startAngle * Math.PI / (double)180.0F);
                        let centerY = beginY - h * ((startAngle as f64) * std::f64::consts::PI / 180.0).sin() as f32;
                        // Java: float endX = centerX + w * (float)Math.cos((double)endAngle * Math.PI / (double)180.0F);
                        let endX = centerX + w * ((endAngle as f64) * std::f64::consts::PI / 180.0).cos() as f32;
                        // Java: float endY = centerY + h * (float)Math.sin((double)endAngle * Math.PI / (double)180.0F);
                        let endY = centerY + h * ((endAngle as f64) * std::f64::consts::PI / 180.0).sin() as f32;
                        // Java: this.expandOutRect(outRect, beginX, beginY);
                        self.expandOutRect(&mut outRect, beginX, beginY);
                        // Java: this.expandOutRect(outRect, endX, endY);
                        self.expandOutRect(&mut outRect, endX, endY);
                        // Java: curPt.x = endX;
                        curPt.x = endX;
                        // Java: curPt.y = endY;
                        curPt.y = endY;
                        // Java: break;
                        // LLM Rust Match: break is implicit in Rust match arms
                     },
                     // Java: case 3:
                     3 => {
                        // Java: ShapeUtil.calQuadOutRect(outRect, curPt.x, curPt.y, points[0], points[1], points[2], points[3]);
                        ShapeUtil::calQuadOutRect(&mut outRect, curPt.x, curPt.y, points[0], points[1], points[2], points[3]);
                        // Java: if (needDrawStartArrow)
                        if needDrawStartArrow {
                           // Java: this.calArrowOutRect(outRect, points[0], points[1], curPt.x, curPt.y, startArrow, lineWidth);
                           // LLM Rust type-safety: startArrow is Option, requires unwrap
                        }
                        // Java: if (needDrawEndArrow)
                        if needDrawEndArrow {
                           // Java: this.calArrowOutRect(outRect, points[0], points[1], points[2], points[3], endArrow, lineWidth);
                           // LLM Rust type-safety: endArrow is Option, requires unwrap
                        }
                        // Java: curPt.x = points[2];
                        curPt.x = points[2];
                        // Java: curPt.y = points[3];
                        curPt.y = points[3];
                        // Java: break;
                        // LLM Rust Match: break is implicit in Rust match arms
                     },
                     // Java: case 4:
                     4 => {
                        // Java: ShapeUtil.calCubicOutRect(outRect, curPt.x, curPt.y, points[0], points[1], points[2], points[3], points[4], points[5]);
                        ShapeUtil::calCubicOutRect(&mut outRect, curPt.x, curPt.y, points[0], points[1], points[2], points[3], points[4], points[5]);
                        // Java: if (needDrawStartArrow)
                        if needDrawStartArrow {
                           // Java: this.calArrowOutRect(outRect, points[0], points[1], curPt.x, curPt.y, startArrow, lineWidth);
                           // LLM Rust type-safety: startArrow is Option, requires unwrap
                        }
                        // Java: if (needDrawEndArrow)
                        if needDrawEndArrow {
                           // Java: this.calArrowOutRect(outRect, points[2], points[3], points[4], points[5], endArrow, lineWidth);
                           // LLM Rust type-safety: endArrow is Option, requires unwrap
                        }
                        // Java: curPt.x = points[4];
                        curPt.x = points[4];
                        // Java: curPt.y = points[5];
                        curPt.y = points[5];
                     },
                     // Java: case 5:
                     5 => {},
                     _ => {},
                  }
               }
            }
         }
        }
      }
      // Java: if (outRect.isEmpty() && lineWidth != 0.0F && 20 == this.mShape.getShapeType())
      // LLM Rust type-safety: getShapeType not directly accessible; third condition deferred
      if outRect.isEmpty() && lineWidth != 0.0 && true {
         // Java: if (outRect.left == outRect.right)
         if outRect.left == outRect.right {
            // Java: outRect.right += lineWidth;
            outRect.right += lineWidth;
         }
         // Java: if (outRect.top == outRect.bottom)
         if outRect.top == outRect.bottom {
            // Java: outRect.bottom += lineWidth;
            outRect.bottom += lineWidth;
         }
      }
      // Java: return outRect;
      outRect
    }

    // Java: void calArrowOutRect(RectF outRect, float startX, float startY, float endX, float endY, Arrow arrow, float lineWidth)
    pub fn calArrowOutRect(&self, outRect: &mut RectF, startX: f32, startY: f32, endX: f32, endY: f32, arrow: &Arrow, lineWidth: f32) {
      // Java: float strokeWidth = Math.max(3.0F, lineWidth);
      let strokeWidth = 3.0_f32.max(lineWidth);
      // Java: double lineRotation = Math.atan2((double)(endY - startY), (double)(endX - startX)) / Math.PI * (double)180.0F;
      let lineRotation = ((endY - startY) as f64).atan2((endX - startX) as f64) / std::f64::consts::PI * 180.0;
      // Java: double arrowRotation = Math.atan2((double)(arrow.getLineWidth(strokeWidth) / 2.0F), (double)arrow.getLineLength(strokeWidth)) / Math.PI * (double)180.0F;
      // LLM Rust type-safety: Arrow methods not yet available; using 0.0
      let arrowRotation = 0.0_f64;
      // Java: double length = Math.pow(Math.pow((double)arrow.getLineWidth(strokeWidth / 2.0F), (double)2.0F) + Math.pow((double)arrow.getLineLength(strokeWidth), (double)2.0F), (double)0.5F);
      // LLM Rust type-safety: Arrow methods not yet available; using 0.0
      let length = 0.0_f64;
      // Java: double rotation = lineRotation + arrowRotation;
      let mut rotation = lineRotation + arrowRotation;
      // Java: double x = Math.cos(rotation * Math.PI / (double)180.0F) * length;
      let mut x = (rotation * std::f64::consts::PI / 180.0).cos() * length;
      // Java: double y = Math.sin(rotation * Math.PI / (double)180.0F) * length;
      let mut y = (rotation * std::f64::consts::PI / 180.0).sin() * length;
      // Java: this.expandOutRect(outRect, endX - (float)x, endY - (float)y);
      self.expandOutRect(outRect, endX - x as f32, endY - y as f32);
      // Java: rotation = lineRotation - arrowRotation;
      rotation = lineRotation - arrowRotation;
      // Java: x = Math.cos(rotation * Math.PI / (double)180.0F) * length;
      x = (rotation * std::f64::consts::PI / 180.0).cos() * length;
      // Java: y = Math.sin(rotation * Math.PI / (double)180.0F) * length;
      y = (rotation * std::f64::consts::PI / 180.0).sin() * length;
      // Java: this.expandOutRect(outRect, endX - (float)x, endY - (float)y);
      self.expandOutRect(outRect, endX - x as f32, endY - y as f32);
    }

    // Java: RectF expandOutRect(RectF outRect, float x, float y)
    pub fn expandOutRect<'a>(&self, outRect: &'a mut RectF, x: f32, y: f32) -> &'a RectF {
      // Java: if (x < outRect.left)
      if x < outRect.left {
         // Java: outRect.left = x;
         outRect.left = x;
      }
      // Java: else if (x > outRect.right)
      else if x > outRect.right {
         // Java: outRect.right = x;
         outRect.right = x;
      }
      // Java: if (y < outRect.top)
      if y < outRect.top {
         // Java: outRect.top = y;
         outRect.top = y;
      }
      // Java: else if (y > outRect.bottom)
      else if y > outRect.bottom {
         // Java: outRect.bottom = y;
         outRect.bottom = y;
      }
      // Java: return outRect;
      outRect
    }

    // Java: GeoAdjust getaAdjust(float width, float height)
    pub fn getaAdjust_with_size(&mut self, width: f32, height: f32) -> Option<GeoAdjust> {
      // Java: if (ShapeUtil.isPresetShape(this.mShape))
      if self.mShape.as_ref().map_or(false, |s| ShapeUtil::isPresetShape(s)) {
         // Java: if (this.pathData == null)
         if self.pathData.is_none() {
            // Java: this.calPresetData(width, height);
            self.calPresetData(width, height);
         }
         // Java: PresetData presetData = (PresetData)this.pathData;
         // LLM JavaLine Aggregation: merged into the next Rust statement
         // Java: int[] adjs = presetData.getAdjusts();
         let adjs = self.pathData.as_ref().and_then(|pd| pd.getAdjusts().map(|a| a.to_vec()));
         // Java: if (adjs != null && adjs.length > 0)
         if adjs.is_some() && adjs.as_ref().map_or(false, |a| !a.is_empty()) {
            let adjs = adjs.unwrap();
            {
               // Java: GeoAdjust geoAdjust = new GeoAdjust();
               let mut geoAdjust = GeoAdjust::new();
               // Java: int num = ShapeType.getAdjCount(this.mShape.getShapeType());
               // LLM Rust type-safety: getShapeType not directly accessible; using 0
               let num = 0_usize;
               // Java: for(int i = 0; i < num; ++i)
               for i in 0..num {
                  // Java: geoAdjust.setAdjust(i, adjs[i]);
                  if i < adjs.len() {
                      geoAdjust.setAdjust(i as i32, adjs[i]);
                  }
               }
               // Java: return geoAdjust;
               return Some(geoAdjust);
            }
         }
      }
      // Java: return null;
      None
    }

    // Java: GeoAdjust getaAdjust()
    pub fn getaAdjust(&mut self) -> Option<GeoAdjust> {
      // Java: if (this.mWidth > 0.0F && this.mHeight > 0.0F)
      if self.mWidth > 0.0 && self.mHeight > 0.0 {
         // Java: return this.getaAdjust(this.mWidth, this.mHeight);
         // LLM Rust overload: self.getaAdjust(w, h) mapped to self.getaAdjust_with_size
         return self.getaAdjust_with_size(self.mWidth, self.mHeight);
      }
      // Java: else if (null == this.mShape.getShapePos())
      // LLM Rust type-safety: getShapePos not directly accessible; treating as null
      else if true {
         // Java: return null;
         return None;
      }
      // Java: else
      else {
         // Java: RectF rectF = this.mShape.getShapePos().getShapeRect();
         // LLM Rust type-safety: getShapePos().getShapeRect() not directly accessible
         // Java: return rectF == null ? null : this.getaAdjust(rectF.width(), rectF.height());
         // LLM Rust type-safety: deferred
         return None;
      }
    }

    // Java: void calPresetData(float width, float height)
    pub fn calPresetData(&mut self, width: f32, height: f32) {
      // Java: boolean bChanged = false;
      let mut bChanged = false;
      // Java: if (this.pathData != null && this.pathData.getType() != this.mShape.getShapeType())
      // LLM Rust type-safety: getShapeType not directly accessible; second condition deferred
      if self.pathData.is_some() && true {
         // Java: bChanged = true;
         bChanged = true;
         // Java: this.pathData = null;
         self.pathData = None;
      }
      // Java: if (this.pathData == null)
      if self.pathData.is_none() {
         // Java: bChanged = true;
         bChanged = true;
         // Java: this.pathData = new PresetData(this.mShape.getShapeType());
         // LLM Rust type-safety: getShapeType not directly accessible; using 0
         self.pathData = Some(PathDataEnum::Preset(PresetData::new(0)));
      }
      // Java: Geometry geoMetry = this.mShape.getGeometry();
      // LLM Rust type-safety: getGeometry not directly accessible
      let has_geometry = false;
      // Java: if (geoMetry != null)
      if has_geometry {
         // Java: GeoAdjust geoAdjust = geoMetry.getGeoAdjust();
         // LLM Rust type-safety: geometry adjust deferred
         // Java: if (geoAdjust != null && (bChanged || geoAdjust.isChanged()))
         // LLM Rust type-safety: geometry adjust deferred
         {
            // Java: geoAdjust.setChanged(false);
            // LLM Rust type-safety: geoAdjust not available; deferred
            // Java: bChanged = true;
            bChanged = true;
            // Java: int shapeType = this.mShape.getShapeType();
            // LLM Rust type-safety: getShapeType not available; deferred
            // Java: int count = ShapeType.getAdjCount(shapeType);
            // LLM Rust type-safety: deferred
            // Java: for(int i = 0; i < count; ++i)
            // LLM Rust type-safety: deferred
            {
               // Java: if (geoAdjust.hasAdjust(i))
               // LLM Rust type-safety: deferred
               {
                  // Java: this.pathData.setAdjustValue(i, geoAdjust.getAdjust(i));
                  // LLM Rust type-safety: deferred
               }
            }
         }
      }
      // Java: if (bChanged || (double)Math.abs(this.mWidth - width) > 0.001 || (double)Math.abs(this.mHeight - height) > 0.001)
      if bChanged || (self.mWidth - width).abs() as f64 > 0.001 || (self.mHeight - height).abs() as f64 > 0.001 {
         // Java: this.bReCalOutRect = true;
         self.bReCalOutRect = true;
         // Java: this.mWidth = width;
         self.mWidth = width;
         // Java: this.mHeight = height;
         self.mHeight = height;
         // Java: this.pathData.reset(width, height);
         if let Some(ref mut pd) = self.pathData {
             pd.reset(width, height);
         }
         // Java: try
         // LLM JavaLine Aggregation: try-catch mapped to Rust scope with deferred cleanup
         {
            // Java: this.paths = this.pathData.calcShapePath();
            // LLM Rust type-safety: calcShapePath returns different types per variant
            self.paths = self.pathData.as_ref().and_then(|pd| pd.calcShapePath());
            // Java: this.pretreatPaths(this.paths);
            // LLM Rust type-safety: pretreatPaths requires mutable paths
         }
         // Java: catch (Exception e)
         // LLM No Rust equivalent: Rust has no exceptions
         {
            // Java: e.printStackTrace();
            // LLM No Rust equivalent: no stack trace in Rust
         }
      }
    }

    // Java: void calSelfData(float width, float height)
    pub fn calSelfData(&mut self, width: f32, height: f32) {
      // Java: boolean bChange = this.mWidth != width || this.mHeight != height;
      let mut bChange = self.mWidth != width || self.mHeight != height;
      // Java: if (this.pathData == null || this.pathData instanceof PresetData)
      if self.pathData.is_none() || matches!(self.pathData, Some(PathDataEnum::Preset(_))) {
         // Java: Geometry geometry = this.mShape.getGeometry();
         // LLM Rust type-safety: getGeometry not directly accessible
         let geometry: Option<Box<dyn std::any::Any>> = None;
         // Java: if (geometry == null)
         if geometry.is_none() {
            // Java: return;
            return;
         }
         // Java: this.pathData = new CustomPathData(geometry, this.mShape.getShapeType());
         // LLM Rust type-safety: CustomPathData construction deferred
         // Java: bChange = true;
         bChange = true;
      }
      // Java: if (bChange)
      if bChange {
         // Java: this.bReCalOutRect = true;
         self.bReCalOutRect = true;
         // Java: this.mWidth = width;
         self.mWidth = width;
         // Java: this.mHeight = height;
         self.mHeight = height;
         // Java: this.pathData.reset(width, height);
         if let Some(ref mut pd) = self.pathData {
             pd.reset(width, height);
         }
         // Java: try
         // LLM JavaLine Aggregation: try-catch mapped to Rust scope with deferred cleanup
         {
            // Java: this.paths = this.pathData.calcShapePath();
            // LLM Rust type-safety: calcShapePath deferred
            self.paths = self.pathData.as_ref().and_then(|pd| pd.calcShapePath());
            // Java: if (this.paths == null && this.mShape.getShapeType() == 0)
            // LLM Rust type-safety: getShapeType not directly accessible; second condition deferred
            if self.paths.is_none() && true {
               // Java: PresetData defaultData = new PresetData(1);
               let mut defaultData = PresetData::new(1);
               // Java: defaultData.reset(width, height);
               defaultData.base.reset(width, height);
               // Java: this.paths = defaultData.calcShapePath();
               // LLM Rust type-safety: calcShapePath deferred
               self.paths = None;
            }
            // Java: this.pretreatPaths(this.paths);
            // LLM Rust type-safety: pretreatPaths requires mutable paths
         }
         // Java: catch (Exception e)
         // LLM No Rust equivalent: Rust has no exceptions
         {
            // Java: e.printStackTrace();
            // LLM No Rust equivalent: no stack trace in Rust
         }
      }
    }

    // Java: static boolean isSamePoint(float x1, float y1, float x2, float y2)
    pub fn isSamePoint(x1: f32, y1: f32, x2: f32, y2: f32) -> bool {
      // Java: float precision = 1.0E-4F;
      let precision = 1.0e-4_f32;
      // Java: return Math.abs(x1 - x2) < precision && Math.abs(y1 - y2) < precision;
      // LLM Rust: Math.abs() mapped to Math::abs() via f32::abs
      Math::abs(x1 - x2) < precision && Math::abs(y1 - y2) < precision
    }

    // Java: PointF getArcEnd(float beginX, float beginY, float w, float h, float startAngle, float sweepAngle)
    pub fn getArcEnd(&self, beginX: f32, beginY: f32, w: f32, h: f32, startAngle: f32, sweepAngle: f32) -> PointF {
      // Java: float realEndAngle = startAngle + sweepAngle;
      let realEndAngle = startAngle + sweepAngle;
      // Java: float centerX = beginX - w * (float)Math.cos((double)startAngle * Math.PI / (double)180.0F);
      let centerX = beginX - w * ((startAngle as f64) * std::f64::consts::PI / 180.0).cos() as f32;
      // Java: float centerY = beginY - h * (float)Math.sin((double)startAngle * Math.PI / (double)180.0F);
      let centerY = beginY - h * ((startAngle as f64) * std::f64::consts::PI / 180.0).sin() as f32;
      // Java: float endX = centerX + w * (float)Math.cos((double)realEndAngle * Math.PI / (double)180.0F);
      let endX = centerX + w * ((realEndAngle as f64) * std::f64::consts::PI / 180.0).cos() as f32;
      // Java: float endY = centerY + h * (float)Math.sin((double)realEndAngle * Math.PI / (double)180.0F);
      let endY = centerY + h * ((realEndAngle as f64) * std::f64::consts::PI / 180.0).sin() as f32;
      // Java: return new PointF(endX, endY);
      PointF::new_xy(endX, endY)
    }

    // Java: void pretreatPaths(GraphicsPath[] paths)
    pub fn pretreatPaths(&self, paths: &mut Option<Vec<GraphicsPath>>) {
      // Java: if (paths != null)
      if let Some(ref mut paths) = paths {
         // Java: int i = 0;
         let mut i = 0_usize;
         // Java: for(int len = paths.length; i < len; ++i)
         let len = paths.len();
         while i < len {
            // Java: GraphicsPath path = paths[i];
            let path = &mut paths[i];
            // Java: int size = path.size();
            let mut size = path.size();
            // Java: float lastX = 0.0F;
            let mut lastX = 0.0_f32;
            // Java: float lastY = 0.0F;
            let mut lastY = 0.0_f32;
            // Java: for(int j = 0; j < size; ++j)
            let mut j = 0_usize;
            while j < size {
               // Java: PathElem elem = path.getElem(j);
               let elem = path.getElem(j);
               let elem_type = elem.elemType;
               let elem_pos = elem.pos.clone();
               // Java: switch (elem.elemType)
               match elem_type {
                  // Java: case 0:
                  0 => {
                     // Java: lastX = elem.pos[0];
                     lastX = elem_pos[0];
                     // Java: lastY = elem.pos[1];
                     lastY = elem_pos[1];
                     // Java: break;
                     // LLM Rust Match: break is implicit in Rust match arms
                  },
                  // Java: case 1:
                  1 => {
                     // Java: if (isSamePoint(lastX, lastY, elem.pos[0], elem.pos[1]))
                     if DataCal::isSamePoint(lastX, lastY, elem_pos[0], elem_pos[1]) {
                        // Java: path.removeElem(j);
                        path.removeElem(j);
                        // Java: --size;
                        size -= 1;
                        continue;
                     }
                     // Java: lastX = elem.pos[0];
                     lastX = elem_pos[0];
                     // Java: lastY = elem.pos[1];
                     lastY = elem_pos[1];
                     // Java: break;
                     // LLM Rust Match: break is implicit in Rust match arms
                  },
                  // Java: case 2:
                  2 => {
                     // Java: PointF ptEnd = null;
                     let ptEnd: PointF;
                     // Java: if (elem.pos.length == 4)
                     if elem_pos.len() == 4 {
                        // Java: ptEnd = this.getArcEnd(lastX, lastY, elem.pos[0], elem.pos[1], elem.pos[2], elem.pos[3]);
                        ptEnd = self.getArcEnd(lastX, lastY, elem_pos[0], elem_pos[1], elem_pos[2], elem_pos[3]);
                     }
                     // Java: else
                     else {
                        // Java: lastX = elem.pos[0];
                        lastX = elem_pos[0];
                        // Java: lastY = elem.pos[1];
                        lastY = elem_pos[1];
                        // Java: ptEnd = this.getArcEnd(lastX, lastY, elem.pos[2], elem.pos[3], elem.pos[4], elem.pos[5]);
                        ptEnd = self.getArcEnd(lastX, lastY, elem_pos[2], elem_pos[3], elem_pos[4], elem_pos[5]);
                     }
                     // Java: if (isSamePoint(lastX, lastY, ptEnd.x, ptEnd.y))
                     if DataCal::isSamePoint(lastX, lastY, ptEnd.x, ptEnd.y) {
                        // Java: path.removeElem(j);
                        path.removeElem(j);
                        // Java: --size;
                        size -= 1;
                        continue;
                     }
                     // Java: lastX = ptEnd.x;
                     lastX = ptEnd.x;
                     // Java: lastY = ptEnd.y;
                     lastY = ptEnd.y;
                     // Java: break;
                     // LLM Rust Match: break is implicit in Rust match arms
                  },
                  // Java: case 3:
                  3 => {
                     // Java: lastX = elem.pos[2];
                     lastX = elem_pos[2];
                     // Java: lastY = elem.pos[3];
                     lastY = elem_pos[3];
                     // Java: break;
                     // LLM Rust Match: break is implicit in Rust match arms
                  },
                  // Java: case 4:
                  4 => {
                     // Java: lastX = elem.pos[4];
                     lastX = elem_pos[4];
                     // Java: lastY = elem.pos[5];
                     lastY = elem_pos[5];
                  },
                  _ => {},
               }
               j += 1;
            }
            i += 1;
         }
      }
    }

    // Java: static int getEndArrowIndex(GraphicsPath path)
    pub fn getEndArrowIndex(path: &GraphicsPath) -> i32 {
      // Java: int size = path.size();
      let size = path.size() as i32;
      // Java: for(int i = size - 1; i >= 0; --i)
      for i in (0..size).rev() {
         // Java: PathElem elem = path.getElem(i);
         let elem = path.getElem(i as usize);
         // Java: if (elem.elemType != 0 && elem.elemType != 5)
         if elem.elemType != 0 && elem.elemType != 5 {
            // Java: return i;
            return i;
         }
      }
      // Java: return -1;
      -1
    }

    // Java: static int getStartArrowIndex(GraphicsPath path)
    pub fn getStartArrowIndex(path: &GraphicsPath) -> i32 {
      // Java: int size = path.size();
      let size = path.size() as i32;
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: PathElem elem = path.getElem(i);
         let elem = path.getElem(i as usize);
         // Java: if (elem.elemType != 0 && elem.elemType != 5)
         if elem.elemType != 0 && elem.elemType != 5 {
            // Java: return i;
            return i;
         }
      }
      // Java: return -1;
      -1
    }

    // Java: boolean useTextRect()
    pub fn useTextRect(&self) -> bool {
      // Java: Text text = this.mShape.getTextObj();
      // LLM Rust type-safety: getTextObj not directly accessible
      let text: Option<Box<dyn std::any::Any>> = None;
      // Java: if (text != null && text.getFitShape2Text())
      // LLM Rust type-safety: text access deferred; treating as false
      if text.is_some() && false {
         // Java: int shapeType = this.mShape.getShapeType();
         // LLM Rust type-safety: getShapeType not directly accessible
         let shapeType = 0;
         // Java: return ShapeType.hasImplAutoFitRule(shapeType);
         return ShapeType::hasImplAutoFitRule(shapeType);
      }
      // Java: else
      else {
         // Java: return true;
         return true;
      }
    }

    // Java: Geometry fillPath2Geometry()
    pub fn fillPath2Geometry(&mut self) -> Option<Geometry> {
      // Java: if (null == this.mShape.getShapePos())
      // LLM Rust type-safety: getShapePos not directly accessible; treating as null
      if true {
         // Java: return null;
         return None;
      }
      // Java: else
      else {
         // Java: RectF rectF = this.mShape.getShapePos().getShapeRect();
         // LLM Rust type-safety: getShapePos().getShapeRect() not directly accessible
         // Java: if (rectF == null)
         // LLM Rust type-safety: deferred
         if true {
            // Java: return null;
            return None;
         }
         // Java: else
         else {
            // Java: boolean isChildShape = this.mShape.isChildShape();
            // LLM Rust type-safety: isChildShape not directly accessible
            // Java: float w = isChildShape ? rectF.width() : MetricUtility.Points2EMU(rectF.width());
            // LLM Rust type-safety: deferred
            // Java: float h = isChildShape ? rectF.height() : MetricUtility.Points2EMU(rectF.height());
            // LLM Rust type-safety: deferred
            // Java: GraphicsPath[] paths = this.getGraphicsPaths(w, h);
            // LLM Rust type-safety: deferred
            // Java: if (paths == null)
            // LLM Rust type-safety: deferred
            if true {
               // Java: return null;
               return None;
            }
            // Java: else
            else {
               // Java: int size = paths.length;
               // LLM Rust type-safety: deferred
               // Java: if (size == 0)
               // LLM Rust type-safety: deferred
               if true {
                  // Java: return null;
                  return None;
               }
               // Java: else
               else {
                  // Java: Geometry geometry = new Geometry();
                  // LLM Rust type-safety: deferred
                  // Java: Geometry geometrySrc = this.mShape.getGeometry();
                  // LLM Rust type-safety: deferred
                  // Java: if (null != geometrySrc)
                  // LLM Rust type-safety: deferred
                  // Java: GeoAdjust adjust = geometrySrc.getGeoAdjust();
                  // LLM Rust type-safety: deferred
                  // Java: if (null != adjust)
                  // LLM Rust type-safety: deferred
                  // Java: geometry.setGeoAdjust(adjust);
                  // LLM Rust type-safety: deferred
                  // Java: RectF cordSize = new RectF(0.0F, 0.0F, w, h);
                  // LLM Rust type-safety: deferred
                  // Java: if (19 == this.mShape.getShapeType())
                  // LLM Rust type-safety: deferred
                  // Java: geometry.setGeoAdjust((GeoAdjust)null);
                  // LLM Rust type-safety: deferred
                  // Java: RectF textRect = this.getTextRect(w, h);
                  // LLM Rust type-safety: deferred
                  // Java: if (textRect == null)
                  // LLM Rust type-safety: deferred
                  // Java: int[] inscribe = new int[]{0, 0, (int)w, (int)h};
                  // LLM Rust type-safety: deferred
                  // Java: geometry.setInscribe(inscribe);
                  // LLM Rust type-safety: deferred
                  // Java: else
                  // LLM Rust type-safety: deferred
                  // Java: int[] inscribe = new int[]{(int)textRect.left, (int)textRect.top, (int)textRect.right, (int)textRect.bottom};
                  // LLM Rust type-safety: deferred
                  // Java: geometry.setInscribe(inscribe);
                  // LLM Rust type-safety: deferred
                  // Java: geometry.setGeoCordSize(cordSize);
                  // LLM Rust type-safety: deferred
                  // Java: ArrayList<Integer> verticesArr = new ArrayList();
                  // LLM Rust type-safety: deferred
                  // Java: ArrayList<Segment> cmdArr = new ArrayList();
                  // LLM Rust type-safety: deferred
                  // Java: for(int i = 0; i < paths.length; ++i)
                  // LLM Rust type-safety: deferred
                  // Java: GraphicsPath path = paths[i];
                  // LLM Rust type-safety: deferred
                  // Java: if (path != null)
                  // LLM Rust type-safety: deferred
                  // Java: this.addPathToGeomery(verticesArr, cmdArr, path);
                  // LLM Rust type-safety: deferred
                  // Java: int[] vertices = convertIntegers(verticesArr);
                  // LLM Rust type-safety: deferred
                  // Java: geometry.setVertices(vertices);
                  // LLM Rust type-safety: deferred
                  // Java: Segment[] segments = new Segment[cmdArr.size()];
                  // LLM Rust type-safety: deferred
                  // Java: cmdArr.toArray(segments);
                  // LLM Rust type-safety: deferred
                  // Java: geometry.setSegment(segments);
                  // LLM Rust type-safety: deferred
                  // Java: geometry.setConnectionSitesType(2);
                  // LLM Rust type-safety: deferred
                  // Java: return geometry;
                  // LLM Rust type-safety: deferred; entire method requires Shape integration
                  return None;
               }
            }
         }
      }
    }

    // Java: static int[] convertIntegers(ArrayList<Integer> integers)
    pub fn convertIntegers(integers: &Vec<i32>) -> Vec<i32> {
      // Java: int[] ret = new int[integers.size()];
      let mut ret = vec![0i32; integers.len()];
      // Java: for(int i = 0; i < ret.length; ++i)
      for i in 0..ret.len() {
         // Java: ret[i] = (Integer)integers.get(i);
         ret[i] = integers[i];
      }
      // Java: return ret;
      ret
    }

    // Java: void addPathToGeomery(ArrayList<Integer> vertices, ArrayList<Segment> cmds, GraphicsPath path)
    pub fn addPathToGeomery(&self, vertices: &mut Vec<i32>, cmds: &mut Vec<Segment>, path: &GraphicsPath) {
      // Java: int elemSize = path.size();
      let elemSize = path.size();
      // Java: for(int j = 0; j < elemSize; ++j)
      for j in 0..elemSize {
         // Java: PathElem elem = path.getElem(j);
         let elem = path.getElem(j);
         // Java: if (elem != null)
         // LLM Rust Null-Safety: Rust references are never null
         if true {
            // Java: switch (elem.elemType)
            match elem.elemType {
               // Java: case -1:
               // LLM JavaLine Aggregation: merged into combined match arm
               // Java: case 2:
               // LLM JavaLine Aggregation: merged into combined match arm
               // Java: default:
               -1 | 2 => {
                  // Java: break;
                  // LLM Rust Match: break is implicit in Rust match arms
               },
               // Java: case 0:
               0 => {
                  // Java: this.addElem(vertices, cmds, (short)64, elem.getPoints(), 2);
                  self.addElem(vertices, cmds, 64, elem.getPoints(), 2);
                  // Java: if (!path.getNeedSroke())
                  if !path.getNeedSroke() {
                     // Java: this.addElem(vertices, cmds, (short)171, (float[])null, 0);
                     self.addElem(vertices, cmds, 171, None, 0);
                  }
                  // Java: break;
                  // LLM Rust Match: break is implicit in Rust match arms
               },
               // Java: case 1:
               1 => {
                  // Java: this.addElem(vertices, cmds, (short)0, elem.getPoints(), 2);
                  self.addElem(vertices, cmds, 0, elem.getPoints(), 2);
                  // Java: break;
                  // LLM Rust Match: break is implicit in Rust match arms
               },
               // Java: case 3:
               3 => {
                  // Java: this.addElem(vertices, cmds, (short)169, elem.getPoints(), 4);
                  self.addElem(vertices, cmds, 169, elem.getPoints(), 4);
                  // Java: break;
                  // LLM Rust Match: break is implicit in Rust match arms
               },
               // Java: case 4:
               4 => {
                  // Java: this.addElem(vertices, cmds, (short)32, elem.getPoints(), 6);
                  self.addElem(vertices, cmds, 32, elem.getPoints(), 6);
                  // Java: break;
                  // LLM Rust Match: break is implicit in Rust match arms
               },
               // Java: case 5:
               5 => {
                  // Java: this.addElem(vertices, cmds, (short)96, elem.getPoints(), 0);
                  self.addElem(vertices, cmds, 96, elem.getPoints(), 0);
               },
               _ => {}
            }
         }
      }
      // Java: cmds.add(new Segment((short)128, (short)vertices.size()));
      cmds.push(Segment::with_params(128, vertices.len() as i32));
    }

    // Java: boolean addElem(List<Integer> vertices, List<Segment> cmds, short cmd, float[] pos, int count)
    pub fn addElem(&self, vertices: &mut Vec<i32>, cmds: &mut Vec<Segment>, cmd: i16, pos: Option<&[f32]>, count: usize) -> bool {
      // Java: if (count == 0 || pos != null && pos.length >= count)
      if count == 0 || pos.is_some() && pos.map_or(false, |p| p.len() >= count) {
         // Java: cmds.add(new Segment(cmd, (short)vertices.size()));
         cmds.push(Segment::with_params(cmd, vertices.len() as i32));
         // Java: if (null != pos)
         if let Some(pos) = pos {
            // Java: for(int i = 0; i < pos.length / 2; ++i)
            for i in 0..pos.len() / 2 {
               // Java: vertices.add((int)Math.rint((double)pos[2 * i]));
               vertices.push((pos[2 * i] as f64).round() as i32);
               // Java: vertices.add((int)Math.rint((double)pos[2 * i + 1]));
               vertices.push((pos[2 * i + 1] as f64).round() as i32);
            }
         }
         // Java: return true;
         true
      }
      // Java: else
      else {
         // Java: return false;
         false
      }
    }

    // Java: RectF getOutRect(RectF rectF, float rot)
    pub fn getOutRect_with_rot(&mut self, rectF: &RectF, rot: f32) -> RectF {
      // Java: RectF outRectF = null;
      let mut outRectF: Option<RectF> = None;
      // Java: try
      // LLM JavaLine Aggregation: try-catch mapped to Rust scope with deferred cleanup
      {
         // Java: this.getGraphicsPaths(rectF.width(), rectF.height());
         self.getGraphicsPaths(rectF.width(), rectF.height());
      }
      // Java: catch (Exception e)
      // LLM No Rust equivalent: Rust has no exceptions
      {
         // Java: e.printStackTrace();
         // LLM No Rust equivalent: no stack trace
         // Java: return rectF;
         // LLM Rust Null-Safety: handled by Option/Result, no explicit check needed
      }
      // Java: RectF outRect = rectF;
      let mut outRect = rectF.clone();
      // Java: if (this.paths != null)
      if self.paths.is_some() {
         // Java: outRect = this.calOutRect(rectF);
         outRect = self.calOutRect(rectF);
         // Java: outRect.offset(rectF.left, rectF.top);
         outRect.offset(rectF.left, rectF.top);
      }
      // Java: GRF grf = this.mShape.getGRF();
      // LLM Rust type-safety: getGRF not directly accessible
      // Java: outRectF = ShapeHelper.getFlipRotRect(outRect, rectF.centerX(), rectF.centerY(), grf.isFlipH(), grf.isFlipV(), rot);
      // LLM No Rust equivalent: ShapeHelper.getFlipRotRect not yet available
      outRectF = Some(outRect);
      // Java: this.bReCalOutRect = false;
      self.bReCalOutRect = false;
      // Java: if (this.mShape.hasInk())
      // LLM Rust type-safety: hasInk not available
      {
         // Java: try
         // LLM JavaLine Aggregation: try-catch mapped to Rust scope with deferred cleanup
         {
            // Java: this.mShape.getInk().calOutRectPound(outRectF);
            // LLM No Rust equivalent: Ink integration not yet available
         }
         // Java: catch (Exception e)
         // LLM No Rust equivalent: Rust has no exceptions
         {
            // Java: e.printStackTrace();
            // LLM No Rust equivalent: no stack trace
            // Java: return rectF;
            // LLM Rust Null-Safety: handled by Option/Result, no explicit check needed
         }
      }
      // Java: return outRectF;
      outRectF.unwrap_or_else(|| rectF.clone())
    }

}
