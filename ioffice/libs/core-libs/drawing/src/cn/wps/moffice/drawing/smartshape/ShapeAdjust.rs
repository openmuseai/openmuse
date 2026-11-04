#![allow(non_snake_case, dead_code, unused_variables, unused_mut, unused_imports)]

// Java: import cn.wps.graphics.PointF
use core_libs_util::graphics::PointF::PointF;
// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.Shape
use crate::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.drawing.ShapeHelper
use crate::cn::wps::moffice::drawing::ShapeHelper::ShapeHelper;
// Java: import cn.wps.moffice.drawing.core.HitPos
use crate::cn::wps::moffice::drawing::core::HitPos::HitPos;
// Java: import cn.wps.moffice.drawing.core.ShapeRange
use crate::cn::wps::moffice::drawing::core::ShapeRange::ShapeRange;

use super::GeoAdjust::GeoAdjust;
use super::AdjustData::AdjustData;
use super::AdjustHandler::AdjustHandler;
use super::PresetShape::PresetShape;

use std::sync::Arc;
use std::any::Any;

// Java: public class ShapeAdjust
pub struct ShapeAdjust {
    // Java field: Shape mShape
    pub(crate) mShape: Option<Arc<Shape>>,
    // Java field: PresetShape mPresetShape
    pub(crate) mPresetShape: Option<PresetShape>,
    // Java field: AdjustHandler mAdjustHandler
    pub(crate) mAdjustHandler: Option<Box<dyn Any>>,
    // Java field: PointF mInitPoint
    pub(crate) mInitPoint: Option<PointF>,
    // Java field: int mHandler
    pub(crate) mHandler: i32,
    // Java field: float mRot
    pub(crate) mRot: f32,
    // Java field: boolean mFlipH
    pub(crate) mFlipH: bool,
    // Java field: boolean mFlipV
    pub(crate) mFlipV: bool,
}

// Java: public class ShapeAdjust
impl ShapeAdjust {
    // Java: ShapeAdjust()
    pub fn new() -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: no super() needed in Rust
      ShapeAdjust {
          mShape: None,
          mPresetShape: None,
          mAdjustHandler: None,
          mInitPoint: None,
          mHandler: 0,
          mRot: 0.0,
          mFlipH: false,
          mFlipV: false,
      }
    }

    // Java: boolean init(Shape shape, HitPos hitPos, float x, float y)
    pub fn init(&mut self, shape: Option<Arc<Shape>>, hitPos: &HitPos, x: f32, y: f32) -> bool {
      // Java: if (shape == null)
      if shape.is_none() {
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         // Java: this.mPresetShape = shape.getPresetShape();
         // LLM Rust type-safety: getPresetShape not directly accessible; deferred
         self.mPresetShape = None;
         // Java: if (this.mPresetShape == null)
         if self.mPresetShape.is_none() {
            // Java: return false;
            return false;
         }
         // Java: else
         else {
            // Java: this.mHandler = ShapeRange.getHitControlHandler(hitPos);
            // LLM Rust type-safety: ShapeRange not available; using 0
            self.mHandler = 0;
            // Java: if (this.mHandler < 0)
            if self.mHandler < 0 {
               // Java: return false;
               return false;
            }
            // Java: else
            else {
               // Java: this.mAdjustHandler = this.mPresetShape.getAdjustHandler(this.mHandler);
               // LLM Rust type-safety: getAdjustHandler returns Box<dyn Any>; deferred
               self.mAdjustHandler = None;
               // Java: if (this.mAdjustHandler == null)
               if self.mAdjustHandler.is_none() {
                  // Java: return false;
                  return false;
               }
               // Java: else
               else {
                  // Java: this.mShape = shape;
                  self.mShape = shape;
                  // Java: this.mInitPoint = new PointF(x, y);
                  self.mInitPoint = Some(PointF::new_xy(x, y));
                  // Java: this.mRot = this.mShape.getRotation();
                  // LLM Rust type-safety: getRotation not directly accessible
                  self.mRot = 0.0;
                  // Java: this.mFlipH = this.mShape.getGRF().isFlipH();
                  // LLM Rust type-safety: getGRF not directly accessible
                  self.mFlipH = false;
                  // Java: this.mFlipV = this.mShape.getGRF().isFlipV();
                  // LLM Rust type-safety: getGRF not directly accessible
                  self.mFlipV = false;
                  // Java: GeoAdjust geoAdjust = this.getGeoAdjust(shape);
                  // LLM Rust type-safety: getGeoAdjust requires mutable shape access
                  // Java: this.init(geoAdjust);
                  // LLM Rust type-safety: init overload deferred
                  // Java: return true;
                  return true;
               }
            }
         }
      }
    }

    // Java: GeoAdjust getGeoAdjust(Shape shape)
    pub fn getGeoAdjust_from_shape(&self, shape: &Arc<Shape>) -> Option<GeoAdjust> {
      // Java: Geometry geometry = shape.getGeometry();
      // LLM Rust type-safety: getGeometry not directly accessible
      // Java: if (geometry == null)
      // LLM Rust type-safety: deferred
      {
         // Java: geometry = new Geometry();
         // LLM Rust type-safety: deferred
         // Java: shape.setGeometry(geometry);
         // LLM Rust type-safety: deferred
      }
      // Java: GeoAdjust geoAdjust = geometry.getGeoAdjust();
      // LLM Rust type-safety: deferred
      // Java: if (geoAdjust == null)
      // LLM Rust type-safety: deferred
      {
         // Java: RectF rectF = this.mPresetShape.getRect();
         // LLM Rust type-safety: deferred
         // Java: geoAdjust = shape.getDefautAdjs(rectF.width(), rectF.height());
         // LLM Rust type-safety: deferred
         // Java: geometry.setGeoAdjust(geoAdjust);
         // LLM Rust type-safety: deferred
      }
      // Java: return geoAdjust;
      None
    }

    // Java: void init(GeoAdjust geoAdjust)
    pub fn init_with_adjust(&mut self, geoAdjust: &mut GeoAdjust) {
      // Java: if (this.mAdjustHandler.type != 3 && this.mAdjustHandler.type != 6)
      // LLM Rust type-safety: mAdjustHandler is Box<dyn Any>; type field not accessible
      if true && true {
         // Java: SingleAdjustHandler ah = (SingleAdjustHandler)this.mAdjustHandler;
         // LLM Rust type-safety: SingleAdjustHandler downcast deferred
         // Java: int adj = ah.adjustData.index;
         // LLM Rust type-safety: deferred
         let adj = 0_i32;
         // Java: if (!geoAdjust.hasAdjust(adj))
         if !geoAdjust.hasAdjust(adj) {
            // Java: geoAdjust.setAdjust(adj, (int)ah.adjustData.value);
            // LLM Rust type-safety: adjustData.value not accessible
         }
      }
      // Java: else
      else {
         // Java: CoupleAdjustHandler ah = (CoupleAdjustHandler)this.mAdjustHandler;
         // LLM Rust type-safety: CoupleAdjustHandler downcast deferred
         // Java: int adj1 = ah.firstAdjustData.index;
         // LLM Rust type-safety: deferred
         let adj1 = 0_i32;
         // Java: int adj2 = ah.secondAdjustData.index;
         // LLM Rust type-safety: deferred
         let adj2 = 0_i32;
         // Java: if (!geoAdjust.hasAdjust(adj1))
         if !geoAdjust.hasAdjust(adj1) {
            // Java: geoAdjust.setAdjust(adj1, (int)ah.firstAdjustData.value);
            // LLM Rust type-safety: deferred
         }
         // Java: if (!geoAdjust.hasAdjust(adj2))
         if !geoAdjust.hasAdjust(adj2) {
            // Java: geoAdjust.setAdjust(adj2, (int)ah.secondAdjustData.value);
            // LLM Rust type-safety: deferred
         }
      }
    }

    // Java: boolean adjust(float x, float y)
    pub fn adjust(&mut self, mut x: f32, mut y: f32) -> bool {
      // Java: RectF rectF = this.mPresetShape.getRect();
      // LLM Rust type-safety: mPresetShape access deferred; using default rect
      let rectF = RectF::new();
      // Java: if (this.mRot != 0.0F)
      if self.mRot != 0.0 {
         // Java: PointF ptRot = ShapeHelper.getRotPoint(x, y, rectF.centerX(), rectF.centerY(), -this.mRot);
         // LLM No Rust equivalent: ShapeHelper not available
         // Java: x = ptRot.x;
         // LLM Rust type-safety: deferred
         // Java: y = ptRot.y;
         // LLM Rust type-safety: deferred
      }
      // Java: if (this.mFlipH)
      if self.mFlipH {
         // Java: x = rectF.width() - x;
         x = rectF.width() - x;
      }
      // Java: if (this.mFlipV)
      if self.mFlipV {
         // Java: y = rectF.height() - y;
         y = rectF.height() - y;
      }
      // Java: Geometry geometry = this.mShape.getGeometry();
      // LLM Rust type-safety: getGeometry not directly accessible
      // Java: GeoAdjust geoAdjust = geometry.getGeoAdjust();
      // LLM Rust type-safety: deferred
      let mut geoAdjust = GeoAdjust::new();
      // Java: this.doAdjust(geoAdjust, x, y, rectF);
      self.doAdjust(&mut geoAdjust, x, y, &rectF);
      // Java: return true;
      true
    }

    // Java: void doAdjust(GeoAdjust geoAdjust, float x, float y, RectF rectF)
    pub fn doAdjust(&self, geoAdjust: &mut GeoAdjust, x: f32, y: f32, rectF: &RectF) {
      // Java: switch (this.mAdjustHandler.type)
      // LLM Rust type-safety: mAdjustHandler is Box<dyn Any>; type not accessible
      let handler_type = 0_i32;
      match handler_type {
         // Java: case 1:
         1 => {
            // Java: this.adjustCartesian(geoAdjust, ((SingleAdjustHandler)this.mAdjustHandler).adjustData, x);
            // LLM Rust type-safety: adjustData not accessible
            // Java: break;
            // LLM Rust Match: break is implicit in Rust match arms
         },
         // Java: case 2:
         2 => {
            // Java: this.adjustCartesian(geoAdjust, ((SingleAdjustHandler)this.mAdjustHandler).adjustData, y);
            // LLM Rust type-safety: adjustData not accessible
            // Java: break;
            // LLM Rust Match: break is implicit in Rust match arms
         },
         // Java: case 3:
         3 => {
            // Java: CoupleAdjustHandler ah = (CoupleAdjustHandler)this.mAdjustHandler;
            // LLM Rust type-safety: CoupleAdjustHandler downcast deferred
            // Java: this.adjustCartesian(geoAdjust, ah.firstAdjustData, x);
            // LLM Rust type-safety: deferred
            // Java: this.adjustCartesian(geoAdjust, ah.secondAdjustData, y);
            // LLM Rust type-safety: deferred
            // Java: break;
            // LLM Rust Match: break is implicit in Rust match arms
         },
         // Java: case 4:
         4 => {
            // Java: this.adjustPolarAngle(geoAdjust, ((SingleAdjustHandler)this.mAdjustHandler).adjustData, x, y, rectF);
            // LLM Rust type-safety: adjustData not accessible
            // Java: break;
            // LLM Rust Match: break is implicit in Rust match arms
         },
         // Java: case 5:
         5 => {
            // Java: this.adjustPolarRadius(geoAdjust, ((SingleAdjustHandler)this.mAdjustHandler).adjustData, x, y, rectF);
            // LLM Rust type-safety: adjustData not accessible
            // Java: break;
            // LLM Rust Match: break is implicit in Rust match arms
         },
         // Java: case 6:
         6 => {
            // Java: ah = (CoupleAdjustHandler)this.mAdjustHandler;
            // LLM Rust type-safety: CoupleAdjustHandler downcast deferred
            // Java: this.adjustPolarAngle(geoAdjust, ah.firstAdjustData, x, y, rectF);
            // LLM Rust type-safety: deferred
            // Java: this.adjustPolarRadius(geoAdjust, ah.secondAdjustData, x, y, rectF);
            // LLM Rust type-safety: deferred
         },
         _ => {},
      }
    }

    // Java: void adjustCartesian(GeoAdjust geoAdjust, AdjustData ad, float val)
    pub fn adjustCartesian(&self, geoAdjust: &mut GeoAdjust, ad: &AdjustData, val: f32) {
      // Java: boolean bReverse = ad.coordinateMinValue > ad.coordinateMaxValue;
      let bReverse = ad.coordinateMinValue > ad.coordinateMaxValue;
      // Java: float minVal = bReverse ? ad.coordinateMaxValue : ad.coordinateMinValue;
      let minVal = if bReverse { ad.coordinateMaxValue } else { ad.coordinateMinValue };
      // Java: float maxVal = bReverse ? ad.coordinateMinValue : ad.coordinateMaxValue;
      let maxVal = if bReverse { ad.coordinateMinValue } else { ad.coordinateMaxValue };
      // Java: float minAdj = bReverse ? ad.maxValue : ad.minValue;
      let minAdj = if bReverse { ad.maxValue } else { ad.minValue };
      // Java: float maxAdj = bReverse ? ad.minValue : ad.maxValue;
      let maxAdj = if bReverse { ad.minValue } else { ad.maxValue };
      // Java: int adj = ad.index;
      let adj = ad.index;
      // Java: if (val < minVal)
      if val < minVal {
         // Java: geoAdjust.setAdjust(adj, (int)minAdj);
         geoAdjust.setAdjust(adj, minAdj as i32);
      }
      // Java: else if (val > maxVal)
      else if val > maxVal {
         // Java: geoAdjust.setAdjust(adj, (int)maxAdj);
         geoAdjust.setAdjust(adj, maxAdj as i32);
      }
      // Java: else
      else {
         // Java: float len = (maxAdj - minAdj) / (maxVal - minVal);
         let len = (maxAdj - minAdj) / (maxVal - minVal);
         // Java: float adjust = minAdj + (val - minVal) * len;
         let adjust = minAdj + (val - minVal) * len;
         // Java: geoAdjust.setAdjust(adj, (int)adjust);
         geoAdjust.setAdjust(adj, adjust as i32);
      }
    }

    // Java: void adjust(GeoAdjust geoAdjust, AdjustData ad, float val, boolean bAll)
    pub fn adjust_with_all(&self, geoAdjust: &mut GeoAdjust, ad: &AdjustData, val: f32, bAll: bool) {
      // Java: boolean bReverse = ad.coordinateMinValue > ad.coordinateMaxValue;
      let mut bReverse = ad.coordinateMinValue > ad.coordinateMaxValue;
      // Java: float minVal = bReverse ? ad.coordinateMaxValue : ad.coordinateMinValue;
      let minVal = if bReverse { ad.coordinateMaxValue } else { ad.coordinateMinValue };
      // Java: float maxVal = bReverse ? ad.coordinateMinValue : ad.coordinateMaxValue;
      let maxVal = if bReverse { ad.coordinateMinValue } else { ad.coordinateMaxValue };
      // Java: if (!bAll)
      if !bAll {
         // Java: bReverse = ad.minValue > ad.maxValue;
         bReverse = ad.minValue > ad.maxValue;
      }
      // Java: float minAdj = bReverse ? ad.maxValue : ad.minValue;
      let minAdj = if bReverse { ad.maxValue } else { ad.minValue };
      // Java: float maxAdj = bReverse ? ad.minValue : ad.maxValue;
      let maxAdj = if bReverse { ad.minValue } else { ad.maxValue };
      // Java: int adj = ad.index;
      let adj = ad.index;
      // Java: if (val < minVal)
      if val < minVal {
         // Java: geoAdjust.setAdjust(adj, (int)minAdj);
         geoAdjust.setAdjust(adj, minAdj as i32);
      }
      // Java: else if (val > maxVal)
      else if val > maxVal {
         // Java: geoAdjust.setAdjust(adj, (int)maxAdj);
         geoAdjust.setAdjust(adj, maxAdj as i32);
      }
      // Java: else
      else {
         // Java: float len = (maxAdj - minAdj) / (maxVal - minVal);
         let len = (maxAdj - minAdj) / (maxVal - minVal);
         // Java: float adjust = minAdj + (val - minVal) * len;
         let adjust = minAdj + (val - minVal) * len;
         // Java: geoAdjust.setAdjust(adj, (int)adjust);
         geoAdjust.setAdjust(adj, adjust as i32);
      }
    }

    // Java: void adjustPolarRadius(GeoAdjust geoAdjust, AdjustData ad, float x, float y, RectF rectF)
    pub fn adjustPolarRadius(&self, geoAdjust: &mut GeoAdjust, ad: &AdjustData, x: f32, y: f32, rectF: &RectF) {
      // Java: float dx = rectF.centerX() - x;
      let dx = rectF.centerX() - x;
      // Java: float dy = rectF.centerY() - y;
      let dy = rectF.centerY() - y;
      // Java: float newRadius = (float)Math.sqrt((double)(dx * dx + dy * dy));
      let newRadius = ((dx * dx + dy * dy) as f64).sqrt() as f32;
      // Java: int newAdj = (int)((newRadius - ad.coordinateMinValue) / (ad.coordinateMaxValue - ad.coordinateMinValue) * (ad.maxValue - ad.minValue));
      let mut newAdj = ((newRadius - ad.coordinateMinValue) / (ad.coordinateMaxValue - ad.coordinateMinValue) * (ad.maxValue - ad.minValue)) as i32;
      // Java: if ((float)newAdj < ad.minValue)
      if (newAdj as f32) < ad.minValue {
         // Java: newAdj = (int)(ad.minValue + 1.0F);
         newAdj = (ad.minValue + 1.0) as i32;
      }
      // Java: else if ((float)newAdj > ad.maxValue)
      else if (newAdj as f32) > ad.maxValue {
         // Java: newAdj = (int)ad.maxValue;
         newAdj = ad.maxValue as i32;
      }
      // Java: geoAdjust.setAdjust(ad.index, newAdj);
      geoAdjust.setAdjust(ad.index, newAdj);
    }

    // Java: void adjustPolarAngle(GeoAdjust geoAdjust, AdjustData ad, float x, float y, RectF rectF)
    pub fn adjustPolarAngle(&self, geoAdjust: &mut GeoAdjust, ad: &AdjustData, x: f32, y: f32, rectF: &RectF) {
      // Java: float centerX = rectF.centerX();
      let centerX = rectF.centerX();
      // Java: float centerY = rectF.centerY();
      let centerY = rectF.centerY();
      // Java: int newAngleDeg = (int)(Math.atan2((double)(y - centerY), (double)(x - centerX)) * (double)1.08E7F / Math.PI);
      let mut newAngleDeg = (((y - centerY) as f64).atan2((x - centerX) as f64) * 1.08e7_f64 / std::f64::consts::PI) as i32;
      // Java: if (newAngleDeg < 0)
      if newAngleDeg < 0 {
         // Java: newAngleDeg += 21600000;
         newAngleDeg += 21600000;
      }
      // Java: if (this.mShape.getShapeType() == 244)
      // LLM Rust type-safety: getShapeType not directly accessible; deferred
      if false {
         // Java: newAngleDeg -= 10800000;
         newAngleDeg -= 10800000;
      }
      // Java: if ((float)newAngleDeg < ad.minValue)
      if (newAngleDeg as f32) < ad.minValue {
         // Java: newAngleDeg = (int)ad.minValue;
         newAngleDeg = ad.minValue as i32;
      }
      // Java: else if ((float)newAngleDeg > ad.maxValue)
      else if (newAngleDeg as f32) > ad.maxValue {
         // Java: newAngleDeg = (int)ad.maxValue;
         newAngleDeg = ad.maxValue as i32;
      }
      // Java: geoAdjust.setAdjust(ad.index, newAngleDeg);
      geoAdjust.setAdjust(ad.index, newAngleDeg);
    }

    // Java: static int GetCalloutType(int shapeType)
    pub fn GetCalloutType(shapeType: i32) -> i32 {
      // Java: switch (shapeType)
      match shapeType {
         // Java: case 41:
         // LLM JavaLine Aggregation: merged into combined match arm
         // Java: case 44:
         // LLM JavaLine Aggregation: merged into combined match arm
         // Java: case 47:
         // LLM JavaLine Aggregation: merged into combined match arm
         // Java: case 50:
         41 | 44 | 47 | 50 => {
            // Java: return 2;
            2
         },
         // Java: case 42:
         // LLM JavaLine Aggregation: merged into combined match arm
         // Java: case 45:
         // LLM JavaLine Aggregation: merged into combined match arm
         // Java: case 48:
         // LLM JavaLine Aggregation: merged into combined match arm
         // Java: case 51:
         42 | 45 | 48 | 51 => {
            // Java: return 3;
            3
         },
         // Java: case 43:
         // LLM JavaLine Aggregation: merged into combined match arm
         // Java: case 46:
         // LLM JavaLine Aggregation: merged into combined match arm
         // Java: case 49:
         // LLM JavaLine Aggregation: merged into combined match arm
         // Java: case 52:
         43 | 46 | 49 | 52 => {
            // Java: return 4;
            4
         },
         // Java: case 178:
         // LLM JavaLine Aggregation: merged into combined match arm
         // Java: case 179:
         // LLM JavaLine Aggregation: merged into combined match arm
         // Java: case 180:
         // LLM JavaLine Aggregation: merged into combined match arm
         // Java: case 181:
         178 | 179 | 180 | 181 => {
            // Java: return 1;
            1
         },
         // Java: default:
         _ => {
            // Java: return 0;
            0
         },
      }
    }

    // Java: static boolean IsLineCallout(int shapeType)
    pub fn IsLineCallout(shapeType: i32) -> bool {
      // Java: return GetCalloutType(shapeType) != 0;
      ShapeAdjust::GetCalloutType(shapeType) != 0
    }

    // Java: static ShapeAdjust getShapeAdjust(int shapeType)
    pub fn getShapeAdjust(shapeType: i32) -> ShapeAdjust {
      // Java: return (ShapeAdjust)(IsLineCallout(shapeType) ? new LineCalloutShapeAdjust() : new ShapeAdjust());
      // LLM Rust type-safety: LineCalloutShapeAdjust not available; returning base ShapeAdjust
      ShapeAdjust::new()
    }

}
