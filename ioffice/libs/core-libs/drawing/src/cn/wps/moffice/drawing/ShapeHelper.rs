#![allow(unused_imports, dead_code, unused_variables, non_snake_case, unused_mut, non_upper_case_globals)]

// Java: import cn.wps.core.runtime.IResourceManager
// LLM No Rust equivalent: using stub
// Java: import cn.wps.core.runtime.Platform
// LLM No Rust equivalent: using stub
// Java: import cn.wps.graphics.PointF
// LLM No Rust equivalent: using local stub
// Java: import cn.wps.graphics.Rect
// LLM No Rust equivalent: using local stub
// Java: import cn.wps.graphics.RectF
// LLM No Rust equivalent: using local stub
// Java: import cn.wps.moffice.drawing.graphics.BlipFill
// LLM No Rust equivalent: using local stub
// Java: import cn.wps.moffice.drawing.graphics.Picture
// LLM No Rust equivalent: using local stub
// Java: import java.util.ArrayList
// LLM No Rust equivalent: using Vec

use std::sync::Mutex;

pub struct PointF { pub x: f32, pub y: f32 }
impl PointF {
    pub fn new() -> Self { PointF { x: 0.0, y: 0.0 } }
}
pub struct Rect { pub left: i32, pub top: i32, pub right: i32, pub bottom: i32 }
impl Rect {
    pub fn width(&self) -> i32 { self.right - self.left }
    pub fn height(&self) -> i32 { self.bottom - self.top }
    pub fn centerX(&self) -> f32 { (self.left + self.right) as f32 / 2.0 }
    pub fn centerY(&self) -> f32 { (self.top + self.bottom) as f32 / 2.0 }
}
pub struct RectF { pub left: f32, pub top: f32, pub right: f32, pub bottom: f32 }
impl RectF {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        RectF { left, top, right, bottom }
    }
    pub fn from_rectf(other: &RectF) -> Self {
        RectF { left: other.left, top: other.top, right: other.right, bottom: other.bottom }
    }
    pub fn width(&self) -> f32 { self.right - self.left }
    pub fn height(&self) -> f32 { self.bottom - self.top }
    pub fn centerX(&self) -> f32 { (self.left + self.right) / 2.0 }
    pub fn centerY(&self) -> f32 { (self.top + self.bottom) / 2.0 }
}
pub struct BlipFill { _private: () }
pub struct Picture { _private: () }
impl Picture {
    pub fn getCropLeft(&self) -> f32 { 0.0 }
    pub fn getCropTop(&self) -> f32 { 0.0 }
    pub fn getCropRight(&self) -> f32 { 0.0 }
    pub fn getCropBottom(&self) -> f32 { 0.0 }
}
pub struct HitShapeStatus;
impl HitShapeStatus {
    pub fn isMultiGridState() -> bool { false }
}
pub struct IResourceManager;
impl IResourceManager {
    pub fn getDimenId(&self, _name: &str) -> i32 { 0 }
    pub fn getDimen(&self, _id: i32) -> i32 { 0 }
}
pub struct Platform;
impl Platform {
    pub fn getResourceManager() -> IResourceManager { IResourceManager }
}

// Java field: float radius
static radius: Mutex<f32> = Mutex::new(5.0);
// Java field: float Multiple
static Multiple: Mutex<f32> = Mutex::new(1.0);
// Java field: float MinSize
static MinSize: Mutex<f32> = Mutex::new(0.0);
// Java field: float HalfMinSize
static HalfMinSize: Mutex<f32> = Mutex::new(0.0);
// Java field: float SmallTurnPoint
static SmallTurnPoint: Mutex<f32> = Mutex::new(0.0);
// Java field: float BigTurnPoint
static BigTurnPoint: Mutex<f32> = Mutex::new(0.0);
// Java field: float SmallCtrlSize
static SmallCtrlSize: Mutex<f32> = Mutex::new(0.0);
// Java field: float MidCtrlSize
static MidCtrlSize: Mutex<f32> = Mutex::new(0.0);
// Java field: float BigCtrlSize
static BigCtrlSize: Mutex<f32> = Mutex::new(0.0);
// Java field: float RotCtrlSize
static RotCtrlSize: Mutex<f32> = Mutex::new(0.0);
// Java field: Integer BoundLengthClip
static BoundLengthClip: Mutex<Option<i32>> = Mutex::new(None);
// Java field: float ExpandSize
static ExpandSize: Mutex<f32> = Mutex::new(0.0);
// Java field: float rotationDistance
static rotationDistance: Mutex<f32> = Mutex::new(0.0);
// Java field: float hoverWidth
static hoverWidth: Mutex<f32> = Mutex::new(0.0);
// Java field: float hoverHeight
static hoverHeight: Mutex<f32> = Mutex::new(0.0);

// Java: public class ShapeHelper
pub struct ShapeHelper {
}

// Java: public class ShapeHelper
impl ShapeHelper {
    // Java: ShapeHelper()
    pub fn new() -> Self {
      // Java: super();
      // LLM No Rust equivalent: no superclass
      ShapeHelper {}
    }

    // Java: static void setMultiple(float multiple)
    pub fn setMultiple(multiple: f32) {
      // Java: Multiple = multiple;
      *Multiple.lock().unwrap() = multiple;
      // Java: radius = 5.0F * multiple;
      *radius.lock().unwrap() = 5.0 * multiple;
      // Java: rotationDistance = 25.0F * multiple;
      *rotationDistance.lock().unwrap() = 25.0 * multiple;
      // Java: MinSize = radius * 16.0F;
      let r = *radius.lock().unwrap();
      *MinSize.lock().unwrap() = r * 16.0;
      // Java: HalfMinSize = MinSize / 2.0F;
      let ms = *MinSize.lock().unwrap();
      *HalfMinSize.lock().unwrap() = ms / 2.0;
      // Java: SmallTurnPoint = MinSize * 1.5F;
      *SmallTurnPoint.lock().unwrap() = ms * 1.5;
      // Java: BigTurnPoint = MinSize * 2.0F;
      *BigTurnPoint.lock().unwrap() = ms * 2.0;
      // Java: SmallCtrlSize = 16.0F * multiple;
      *SmallCtrlSize.lock().unwrap() = 16.0 * multiple;
      // Java: MidCtrlSize = 24.0F * multiple;
      *MidCtrlSize.lock().unwrap() = 24.0 * multiple;
      // Java: BigCtrlSize = 32.0F * multiple;
      *BigCtrlSize.lock().unwrap() = 32.0 * multiple;
      // Java: ExpandSize = radius * 2.0F;
      *ExpandSize.lock().unwrap() = r * 2.0;
      // Java: hoverWidth *= multiple;
      *hoverWidth.lock().unwrap() *= multiple;
      // Java: hoverHeight *= multiple;
      *hoverHeight.lock().unwrap() *= multiple;
    }

    // Java: static float getMultiple()
    pub fn getMultiple() -> f32 {
      // Java: return Multiple;
      *Multiple.lock().unwrap()
    }

    // Java: static float getRotPointDes(float zoom)
    pub fn getRotPointDes(zoom: f32) -> f32 {
      let mut zoom = zoom;
      // Java: if (!HitShapeStatus.isMultiGridState() && zoom < 1.0F)
      if !HitShapeStatus::isMultiGridState() && zoom < 1.0 {
         // Java: zoom = 1.0F;
         zoom = 1.0;
      }
      // Java: return rotationDistance / zoom;
      *rotationDistance.lock().unwrap() / zoom
    }

    // Java: static float getPointRadius(float zoom)
    pub fn getPointRadius(zoom: f32) -> f32 {
      let mut zoom = zoom;
      // Java: if (!HitShapeStatus.isMultiGridState() && zoom < 1.0F)
      if !HitShapeStatus::isMultiGridState() && zoom < 1.0 {
         // Java: zoom = 1.0F;
         zoom = 1.0;
      }
      // Java: return radius / zoom;
      *radius.lock().unwrap() / zoom
    }

    // Java: static float getHoverWidth(float zoom)
    pub fn getHoverWidth(zoom: f32) -> f32 {
      let mut zoom = zoom;
      // Java: if (!HitShapeStatus.isMultiGridState() && zoom < 1.0F)
      if !HitShapeStatus::isMultiGridState() && zoom < 1.0 {
         // Java: zoom = 1.0F;
         zoom = 1.0;
      }
      // Java: return hoverWidth / zoom;
      *hoverWidth.lock().unwrap() / zoom
    }

    // Java: static float getHoverHeight(float zoom)
    pub fn getHoverHeight(zoom: f32) -> f32 {
      let mut zoom = zoom;
      // Java: if (!HitShapeStatus.isMultiGridState() && zoom < 1.0F)
      if !HitShapeStatus::isMultiGridState() && zoom < 1.0 {
         // Java: zoom = 1.0F;
         zoom = 1.0;
      }
      // Java: return hoverHeight / zoom;
      *hoverHeight.lock().unwrap() / zoom
    }

    // Java: static RectF getCtrlRect(RectF rectShape, float zoom)
    pub fn getCtrlRect(rectShape: &RectF, zoom: f32) -> RectF {
      let mut zoom = zoom;
      // Java: if (!HitShapeStatus.isMultiGridState() && zoom < 1.0F)
      if !HitShapeStatus::isMultiGridState() && zoom < 1.0 {
         // Java: zoom = 1.0F;
         zoom = 1.0;
      }
      // Java: RectF rectF = new RectF(rectShape);
      let mut rectF = RectF::from_rectf(rectShape);
      let min_size = *MinSize.lock().unwrap();
      let half_min = *HalfMinSize.lock().unwrap();
      // Java: if (rectShape.width() < MinSize / zoom)
      if rectShape.width() < min_size / zoom {
         // Java: float centerX = rectShape.centerX();
         let centerX = rectShape.centerX();
         // Java: rectF.left = centerX - HalfMinSize / zoom;
         rectF.left = centerX - half_min / zoom;
         // Java: rectF.right = centerX + HalfMinSize / zoom;
         rectF.right = centerX + half_min / zoom;
      }
      // Java: if (rectShape.height() < MinSize / zoom)
      if rectShape.height() < min_size / zoom {
         // Java: float centerY = rectShape.centerY();
         let centerY = rectShape.centerY();
         // Java: rectF.top = centerY - HalfMinSize / zoom;
         rectF.top = centerY - half_min / zoom;
         // Java: rectF.bottom = centerY + HalfMinSize / zoom;
         rectF.bottom = centerY + half_min / zoom;
      }
      // Java: return rectF;
      rectF
    }

    // Java: static RectF getCtrlRect(Rect rectShape, float zoom)
    pub fn getCtrlRect_from_rect(rectShape: &Rect, zoom: f32) -> RectF {
      let mut zoom = zoom;
      // Java: if (!HitShapeStatus.isMultiGridState() && zoom < 1.0F)
      if !HitShapeStatus::isMultiGridState() && zoom < 1.0 {
         // Java: zoom = 1.0F;
         zoom = 1.0;
      }
      // Java: RectF rectF = new RectF((float)rectShape.left, (float)rectShape.top, (float)rectShape.right, (float)rectShape.bottom);
      let mut rectF = RectF::new(rectShape.left as f32, rectShape.top as f32, rectShape.right as f32, rectShape.bottom as f32);
      let min_size = *MinSize.lock().unwrap();
      let half_min = *HalfMinSize.lock().unwrap();
      // Java: if ((float)rectShape.width() < MinSize / zoom)
      if (rectShape.width() as f32) < min_size / zoom {
         // Java: float centerX = (float)rectShape.centerX();
         let centerX = rectShape.centerX();
         // Java: rectF.left = centerX - HalfMinSize / zoom;
         rectF.left = centerX - half_min / zoom;
         // Java: rectF.right = centerX + HalfMinSize / zoom;
         rectF.right = centerX + half_min / zoom;
      }
      // Java: if ((float)rectShape.height() < MinSize / zoom)
      if (rectShape.height() as f32) < min_size / zoom {
         // Java: float centerY = (float)rectShape.centerY();
         let centerY = rectShape.centerY();
         // Java: rectF.top = centerY - HalfMinSize / zoom;
         rectF.top = centerY - half_min / zoom;
         // Java: rectF.bottom = centerY + HalfMinSize / zoom;
         rectF.bottom = centerY + half_min / zoom;
      }
      // Java: return rectF;
      rectF
    }

    // Java: static int getBoundLengthClip()
    pub fn getBoundLengthClip() -> i32 {
      let mut blc = BoundLengthClip.lock().unwrap();
      // Java: if (BoundLengthClip == null)
      if blc.is_none() {
         // Java: IResourceManager rm = Platform.getResourceManager();
         let rm = Platform::getResourceManager();
         // Java: BoundLengthClip = rm.getDimen(rm.getDimenId("writer_render_picture_clip_bound_length"));
         let val = rm.getDimen(rm.getDimenId("writer_render_picture_clip_bound_length"));
         *blc = Some(val);
         // Java: if (BoundLengthClip == 0)
         if val == 0 {
            // Java: BoundLengthClip = 52;
            *blc = Some(52);
         }
      }
      // Java: return BoundLengthClip;
      blc.unwrap()
    }

    // Java: static float getClipCtrlLengh(float zoom)
    pub fn getClipCtrlLengh(zoom: f32) -> f32 {
      let mut zoom = zoom;
      // Java: if (!HitShapeStatus.isMultiGridState() && zoom < 1.0F)
      if !HitShapeStatus::isMultiGridState() && zoom < 1.0 {
         // Java: zoom = 1.0F;
         zoom = 1.0;
      }
      // Java: return (float)(getBoundLengthClip() / 2) / zoom;
      (Self::getBoundLengthClip() / 2) as f32 / zoom
    }

    // Java: static float getClipCtrlStroke(float zoom)
    pub fn getClipCtrlStroke(zoom: f32) -> f32 {
      let mut zoom = zoom;
      // Java: if (!HitShapeStatus.isMultiGridState() && zoom < 1.0F)
      if !HitShapeStatus::isMultiGridState() && zoom < 1.0 {
         // Java: zoom = 1.0F;
         zoom = 1.0;
      }
      // Java: return getMultiple() * 6.0F / zoom;
      Self::getMultiple() * 6.0 / zoom
    }

    // Java: static int getCtrlRadius(float size, float zoom)
    pub fn getCtrlRadius(size: f32, zoom: f32) -> i32 {
      let mut zoom = zoom;
      // Java: if (!HitShapeStatus.isMultiGridState() && zoom < 1.0F)
      if !HitShapeStatus::isMultiGridState() && zoom < 1.0 {
         // Java: zoom = 1.0F;
         zoom = 1.0;
      }
      // Java: int radius = (int)(BigCtrlSize / zoom);
      // LLM No Rust equivalent: 'radius' shadows module-level static, renamed to ctrl_radius
      let mut ctrl_radius = (*BigCtrlSize.lock().unwrap() / zoom) as i32;
      let small_tp = *SmallTurnPoint.lock().unwrap();
      let big_tp = *BigTurnPoint.lock().unwrap();
      // Java: if (size < SmallTurnPoint / zoom)
      if size < small_tp / zoom {
         // Java: radius = (int)(SmallCtrlSize / zoom);
         ctrl_radius = (*SmallCtrlSize.lock().unwrap() / zoom) as i32;
      }
      // Java: else if (size < BigTurnPoint / zoom)
      else if size < big_tp / zoom {
         // Java: radius = (int)(MidCtrlSize / zoom);
         ctrl_radius = (*MidCtrlSize.lock().unwrap() / zoom) as i32;
      }
      // Java: return radius;
      ctrl_radius
    }

    // Java: static int getCtrlRadiusForRot(float zoom)
    pub fn getCtrlRadiusForRot(zoom: f32) -> i32 {
      let mut zoom = zoom;
      // Java: if (!HitShapeStatus.isMultiGridState() && zoom < 1.0F)
      if !HitShapeStatus::isMultiGridState() && zoom < 1.0 {
         // Java: zoom = 1.0F;
         zoom = 1.0;
      }
      // Java: return (int)(RotCtrlSize / zoom);
      (*RotCtrlSize.lock().unwrap() / zoom) as i32
    }

    // Java: static int getCtrlRadiusForTextBox(float zoom)
    pub fn getCtrlRadiusForTextBox(zoom: f32) -> i32 {
      let mut zoom = zoom;
      // Java: if (!HitShapeStatus.isMultiGridState() && zoom < 1.0F)
      if !HitShapeStatus::isMultiGridState() && zoom < 1.0 {
         // Java: zoom = 1.0F;
         zoom = 1.0;
      }
      // Java: return (int)(SmallCtrlSize / zoom);
      (*SmallCtrlSize.lock().unwrap() / zoom) as i32
    }

    // Java: static RectF getRefreshRectF(RectF rectF)
    pub fn getRefreshRectF(rectF: &RectF) -> RectF {
      // Java: float expandX = getExpand(rectF.width());
      let expandX = Self::getExpand(rectF.width());
      // Java: float expandY = getExpand(rectF.height());
      let expandY = Self::getExpand(rectF.height());
      // Java: return new RectF(rectF.left - expandX, rectF.top - expandY, rectF.right + expandX, rectF.bottom + expandY);
      RectF::new(rectF.left - expandX, rectF.top - expandY, rectF.right + expandX, rectF.bottom + expandY)
    }

    // Java: static RectF getRefreshRectF(float left, float top, float right, float bottom)
    pub fn getRefreshRectF_from_coords(left: f32, top: f32, right: f32, bottom: f32) -> RectF {
      // Java: float expandX = getExpand(right - left);
      let expandX = Self::getExpand(right - left);
      // Java: float expandY = getExpand(bottom - top);
      let expandY = Self::getExpand(bottom - top);
      // Java: return new RectF(left - expandX, top - expandY, right + expandX, bottom + expandY);
      RectF::new(left - expandX, top - expandY, right + expandX, bottom + expandY)
    }

    // Java: static Rect getRefreshRect(Rect rect)
    pub fn getRefreshRect(rect: &Rect) -> Rect {
      // Java: int expandX = (int)getExpand((float)rect.width());
      let expandX = Self::getExpand(rect.width() as f32) as i32;
      // Java: int expandY = (int)getExpand((float)rect.height());
      let expandY = Self::getExpand(rect.height() as f32) as i32;
      // Java: return new Rect(rect.left - expandX, rect.top - expandY, rect.right + expandX, rect.bottom + expandY);
      Rect { left: rect.left - expandX, top: rect.top - expandY, right: rect.right + expandX, bottom: rect.bottom + expandY }
    }

    // Java: static RectF getRefreshRectF(RectF rectF, float zoom)
    pub fn getRefreshRectF_with_zoom(rectF: &RectF, zoom: f32) -> RectF {
      // Java: float expandX = getExpand(rectF.width(), zoom);
      let expandX = Self::getExpand_with_zoom(rectF.width(), zoom);
      // Java: float expandY = getExpand(rectF.height(), zoom);
      let expandY = Self::getExpand_with_zoom(rectF.height(), zoom);
      // Java: return new RectF(rectF.left - expandX, rectF.top - expandY, rectF.right + expandX, rectF.bottom + expandY);
      RectF::new(rectF.left - expandX, rectF.top - expandY, rectF.right + expandX, rectF.bottom + expandY)
    }

    // Java: static float getExpand(float size)
    pub fn getExpand(size: f32) -> f32 {
      // Java: float expand = ExpandSize;
      let mut expand = *ExpandSize.lock().unwrap();
      let min_size = *MinSize.lock().unwrap();
      // Java: if (size <= MinSize)
      if size <= min_size {
         // Java: expand += (MinSize - size) / 2.0F;
         expand += (min_size - size) / 2.0;
      }
      // Java: return expand;
      expand
    }

    // Java: static float getExpand(float size, float zoom)
    pub fn getExpand_with_zoom(size: f32, zoom: f32) -> f32 {
      // Java: float expand = ExpandSize * zoom;
      let expand_size = *ExpandSize.lock().unwrap();
      let mut expand = expand_size * zoom;
      let min_size = *MinSize.lock().unwrap();
      // Java: if (size <= MinSize * zoom)
      if size <= min_size * zoom {
         // Java: expand += (MinSize * zoom - size) / 2.0F;
         expand += (min_size * zoom - size) / 2.0;
      }
      // Java: return expand;
      expand
    }

    // Java: static PointF getRotPoint(PointF pt, PointF ptCenter, float rot)
    pub fn getRotPoint_from_points(pt: &PointF, ptCenter: &PointF, rot: f32) -> PointF {
      // Java: PointF ptRot = new PointF();
      let mut ptRot = PointF::new();
      // Java: getRotPoint(pt.x, pt.y, ptCenter.x, ptCenter.y, rot, ptRot);
      Self::getRotPoint_mut(pt.x, pt.y, ptCenter.x, ptCenter.y, rot, &mut ptRot);
      // Java: return ptRot;
      ptRot
    }

    // Java: static PointF getRotPoint(float x, float y, float centerX, float centerY, float rot)
    pub fn getRotPoint(x: f32, y: f32, centerX: f32, centerY: f32, rot: f32) -> PointF {
      // Java: PointF ptRot = new PointF();
      let mut ptRot = PointF::new();
      // Java: getRotPoint(x, y, centerX, centerY, rot, ptRot);
      Self::getRotPoint_mut(x, y, centerX, centerY, rot, &mut ptRot);
      // Java: return ptRot;
      ptRot
    }

    // Java: static void getRotPoint(float x, float y, float centerX, float centerY, float rot, PointF outPoint)
    pub fn getRotPoint_mut(x: f32, y: f32, centerX: f32, centerY: f32, rot: f32, outPoint: &mut PointF) {
      // Java: float xTranslate = x - centerX;
      let xTranslate = x - centerX;
      // Java: float yTranslate = y - centerY;
      let yTranslate = y - centerY;
      // Java: double angle = (double)rot * Math.PI / (double)180.0F;
      let angle = (rot as f64) * std::f64::consts::PI / 180.0;
      // Java: float cosValue = (float)Math.cos(angle);
      let cosValue = angle.cos() as f32;
      // Java: float sinValue = (float)Math.sin(angle);
      let sinValue = angle.sin() as f32;
      // Java: outPoint.x = xTranslate * cosValue - yTranslate * sinValue;
      outPoint.x = xTranslate * cosValue - yTranslate * sinValue;
      // Java: outPoint.y = xTranslate * sinValue + yTranslate * cosValue;
      outPoint.y = xTranslate * sinValue + yTranslate * cosValue;
      // Java: outPoint.x += centerX;
      outPoint.x += centerX;
      // Java: outPoint.y += centerY;
      outPoint.y += centerY;
    }

    // Java: static void rotPoints(ArrayList<Float> pts, int start, int len, float rot)
    pub fn rotPoints_list(pts: &mut Vec<f32>, start: i32, len: i32, rot: f32) {
      let mut len = len;
      // Java: if (len >= 1 && pts.size() >= start + 2)
      if len >= 1 && pts.len() as i32 >= start + 2 {
         // Java: if (pts.size() > (start + len) * 2)
         if pts.len() as i32 > (start + len) * 2 {
            // Java: len = (pts.size() - start * 2) / 2;
            len = (pts.len() as i32 - start * 2) / 2;
         }
         // Java: double angle = (double)rot * Math.PI / (double)180.0F;
         let angle = (rot as f64) * std::f64::consts::PI / 180.0;
         // Java: float cosValue = (float)Math.cos(angle);
         let cosValue = angle.cos() as f32;
         // Java: float sinValue = (float)Math.sin(angle);
         let sinValue = angle.sin() as f32;
         // Java: int end = start + len;
         let end = start + len;
         // Java: for(int i = start; i < end; ++i)
         for i in start..end {
            // Java: int index = 2 * i;
            let index = (2 * i) as usize;
            // Java: float x = (Float)pts.get(index) * cosValue - (Float)pts.get(index + 1) * sinValue;
            let x = pts[index] * cosValue - pts[index + 1] * sinValue;
            // Java: float y = (Float)pts.get(index) * sinValue + (Float)pts.get(index + 1) * cosValue;
            let y = pts[index] * sinValue + pts[index + 1] * cosValue;
            // Java: pts.set(index, x);
            pts[index] = x;
            // Java: pts.set(index + 1, y);
            pts[index + 1] = y;
         }
      }
    }

    // Java: static void rotPoints(PointF pt, float rot)
    pub fn rotPoints_point(pt: &mut PointF, rot: f32) {
      // Java: if (pt != null)
      // LLM No Rust equivalent: pt is a reference, always non-null
      {
         // Java: double angle = (double)rot * Math.PI / (double)180.0F;
         let angle = (rot as f64) * std::f64::consts::PI / 180.0;
         // Java: float cosValue = (float)Math.cos(angle);
         let cosValue = angle.cos() as f32;
         // Java: float sinValue = (float)Math.sin(angle);
         let sinValue = angle.sin() as f32;
         // Java: float x = pt.x * cosValue - pt.y * sinValue;
         let x = pt.x * cosValue - pt.y * sinValue;
         // Java: float y = pt.x * sinValue + pt.y * cosValue;
         let y = pt.x * sinValue + pt.y * cosValue;
         // Java: pt.x = x;
         pt.x = x;
         // Java: pt.y = y;
         pt.y = y;
      }
    }

    // Java: static void rotPoints(float[] pts, float rot)
    pub fn rotPoints_array(pts: &mut [f32], rot: f32) {
      // Java: int len = pts.length / 2;
      let len = pts.len() as i32 / 2;
      // Java: if (len >= 1)
      if len >= 1 {
         // Java: double angle = (double)rot * Math.PI / (double)180.0F;
         let angle = (rot as f64) * std::f64::consts::PI / 180.0;
         // Java: float cosValue = (float)Math.cos(angle);
         let cosValue = angle.cos() as f32;
         // Java: float sinValue = (float)Math.sin(angle);
         let sinValue = angle.sin() as f32;
         // Java: for(int i = 0; i < len; ++i)
         for i in 0..len {
            // Java: int index = 2 * i;
            let index = (2 * i) as usize;
            // Java: float x = pts[index] * cosValue - pts[index + 1] * sinValue;
            let x = pts[index] * cosValue - pts[index + 1] * sinValue;
            // Java: float y = pts[index] * sinValue + pts[index + 1] * cosValue;
            let y = pts[index] * sinValue + pts[index + 1] * cosValue;
            // Java: pts[index] = x;
            pts[index] = x;
            // Java: pts[index + 1] = y;
            pts[index + 1] = y;
         }
      }
    }

    // Java: static float[] getRotPoints(float[] pts, float rot)
    pub fn getRotPoints(pts: &[f32], rot: f32) -> Option<Vec<f32>> {
      // Java: int len = pts.length / 2;
      let len = pts.len() / 2;
      // Java: if (len < 1)
      if len < 1 {
         // Java: return null;
         return None;
      }
      // Java: else
      else {
         // Java: double angle = (double)rot * Math.PI / (double)180.0F;
         let angle = (rot as f64) * std::f64::consts::PI / 180.0;
         // Java: float cosValue = (float)Math.cos(angle);
         let cosValue = angle.cos() as f32;
         // Java: float sinValue = (float)Math.sin(angle);
         let sinValue = angle.sin() as f32;
         // Java: float[] result = new float[len * 2];
         let mut result = vec![0.0f32; len * 2];
         // Java: for(int i = 0; i < len; ++i)
         for i in 0..len {
            // Java: int index = 2 * i;
            let index = 2 * i;
            // Java: result[index] = pts[index] * cosValue - pts[index + 1] * sinValue;
            result[index] = pts[index] * cosValue - pts[index + 1] * sinValue;
            // Java: result[index + 1] = pts[index] * sinValue + pts[index + 1] * cosValue;
            result[index + 1] = pts[index] * sinValue + pts[index + 1] * cosValue;
         }
         // Java: return result;
         Some(result)
      }
    }

    // Java: static boolean isChangeRect(int rot)
    pub fn isChangeRect(rot: i32) -> bool {
      let mut rot = rot;
      // Java: while(rot < 0)
      while rot < 0 {
         // Java: rot += 360;
         rot += 360;
      }
      // Java: rot %= 360;
      rot %= 360;
      // Java: return rot >= 45 && rot < 135 || rot >= 225 && rot < 315;
      (rot >= 45 && rot < 135) || (rot >= 225 && rot < 315)
    }

    // Java: static RectF getShapeBoundRect(RectF rectF, int rot)
    pub fn getShapeBoundRect(rectF: &RectF, rot: i32) -> RectF {
      // Java: RectF rectBound = new RectF(rectF);
      let mut rectBound = RectF::from_rectf(rectF);
      // Java: if (isChangeRect(rot))
      if Self::isChangeRect(rot) {
         // Java: double nXMid = (double)(rectF.left * 2.0F + rectF.width()) / (double)2.0F;
         let nXMid = (rectF.left as f64 * 2.0 + rectF.width() as f64) / 2.0;
         // Java: double nYMid = (double)(rectF.top * 2.0F + rectF.height()) / (double)2.0F;
         let nYMid = (rectF.top as f64 * 2.0 + rectF.height() as f64) / 2.0;
         // Java: double nWidth = (double)rectF.width();
         let nWidth = rectF.width() as f64;
         // Java: double nHeight = (double)rectF.height();
         let nHeight = rectF.height() as f64;
         // Java: rectBound.left = (float)(nXMid - nHeight / (double)2.0F);
         rectBound.left = (nXMid - nHeight / 2.0) as f32;
         // Java: rectBound.right = (float)((double)rectBound.left + nHeight);
         rectBound.right = (rectBound.left as f64 + nHeight) as f32;
         // Java: rectBound.top = (float)(nYMid - nWidth / (double)2.0F);
         rectBound.top = (nYMid - nWidth / 2.0) as f32;
         // Java: rectBound.bottom = (float)((double)rectBound.top + nWidth);
         rectBound.bottom = (rectBound.top as f64 + nWidth) as f32;
      }
      // Java: return rectBound;
      rectBound
    }

    // Java: static RectF getShapeBoundRectNoMove(RectF rectF, int rot)
    pub fn getShapeBoundRectNoMove(rectF: &RectF, rot: i32) -> RectF {
      // Java: RectF rectBound = new RectF(rectF);
      let mut rectBound = RectF::from_rectf(rectF);
      // Java: if (isChangeRect(rot))
      if Self::isChangeRect(rot) {
         // Java: rectBound.left = rectF.left;
         rectBound.left = rectF.left;
         // Java: rectBound.right = rectF.left + rectF.height();
         rectBound.right = rectF.left + rectF.height();
         // Java: rectBound.top = rectF.top;
         rectBound.top = rectF.top;
         // Java: rectBound.bottom = rectF.top + rectF.width();
         rectBound.bottom = rectF.top + rectF.width();
      }
      // Java: return rectBound;
      rectBound
    }

    // Java: static RectF getShapeBoundRect(RectF rectF)
    pub fn getShapeBoundRect_simple(rectF: &RectF) -> RectF {
      // Java: double nXMid = (double)(rectF.left * 2.0F + rectF.width()) / (double)2.0F;
      let nXMid = (rectF.left as f64 * 2.0 + rectF.width() as f64) / 2.0;
      // Java: double nYMid = (double)(rectF.top * 2.0F + rectF.height()) / (double)2.0F;
      let nYMid = (rectF.top as f64 * 2.0 + rectF.height() as f64) / 2.0;
      // Java: double nWidth = (double)rectF.width();
      let nWidth = rectF.width() as f64;
      // Java: double nHeight = (double)rectF.height();
      let nHeight = rectF.height() as f64;
      // Java: RectF rectBound = new RectF(rectF);
      let mut rectBound = RectF::from_rectf(rectF);
      // Java: rectBound.left = (float)(nXMid - nHeight / (double)2.0F);
      rectBound.left = (nXMid - nHeight / 2.0) as f32;
      // Java: rectBound.right = (float)((double)rectBound.left + nHeight);
      rectBound.right = (rectBound.left as f64 + nHeight) as f32;
      // Java: rectBound.top = (float)(nYMid - nWidth / (double)2.0F);
      rectBound.top = (nYMid - nWidth / 2.0) as f32;
      // Java: rectBound.bottom = (float)((double)rectBound.top + nWidth);
      rectBound.bottom = (rectBound.top as f64 + nWidth) as f32;
      // Java: return rectBound;
      rectBound
    }

    // Java: static RectF getRotRect(RectF rectF, float cx, float cy, float rot)
    pub fn getRotRect(rectF: &RectF, cx: f32, cy: f32, rot: f32) -> RectF {
      // Java: if (rot == 0.0F)
      if rot == 0.0 {
         // Java: return new RectF(rectF);
         return RectF::from_rectf(rectF);
      }
      // Java: else
      else {
         // Java: PointF pt1 = getRotPoint(rectF.left, rectF.top, cx, cy, rot);
         let pt1 = Self::getRotPoint(rectF.left, rectF.top, cx, cy, rot);
         // Java: PointF pt2 = getRotPoint(rectF.right, rectF.top, cx, cy, rot);
         let pt2 = Self::getRotPoint(rectF.right, rectF.top, cx, cy, rot);
         // Java: PointF pt3 = getRotPoint(rectF.right, rectF.bottom, cx, cy, rot);
         let pt3 = Self::getRotPoint(rectF.right, rectF.bottom, cx, cy, rot);
         // Java: PointF pt4 = getRotPoint(rectF.left, rectF.bottom, cx, cy, rot);
         let pt4 = Self::getRotPoint(rectF.left, rectF.bottom, cx, cy, rot);
         // Java: return new RectF(Math.min(Math.min(pt1.x, pt2.x), Math.min(pt3.x, pt4.x)), Math.min(Math.min(pt1.y, pt2.y), Math.min(pt3.y, pt4.y)), Math.max(Math.max(pt1.x, pt2.x), Math.max(pt3.x, pt4.x)), Math.max(Math.max(pt1.y, pt2.y), Math.max(pt3.y, pt4.y)));
         RectF::new(
            pt1.x.min(pt2.x).min(pt3.x).min(pt4.x),
            pt1.y.min(pt2.y).min(pt3.y).min(pt4.y),
            pt1.x.max(pt2.x).max(pt3.x).max(pt4.x),
            pt1.y.max(pt2.y).max(pt3.y).max(pt4.y),
         )
      }
    }

    // Java: static double getRotationDegree(PointF centerP, PointF originP, PointF moveP)
    pub fn getRotationDegree(centerP: &PointF, originP: &PointF, moveP: &PointF) -> f64 {
      // Java: float vAX = originP.x - centerP.x;
      let vAX = originP.x - centerP.x;
      // Java: float vAY = originP.y - centerP.y;
      let vAY = originP.y - centerP.y;
      // Java: float vBX = moveP.x - centerP.x;
      let vBX = moveP.x - centerP.x;
      // Java: float vBY = moveP.y - centerP.y;
      let vBY = moveP.y - centerP.y;
      // Java: double dotProduct = (double)(vAX * vBX + vBY * vAY);
      let dotProduct = (vAX * vBX + vBY * vAY) as f64;
      // Java: double mA = Math.sqrt((double)(vAX * vAX + vAY * vAY));
      let mA = ((vAX * vAX + vAY * vAY) as f64).sqrt();
      // Java: double mB = Math.sqrt((double)(vBX * vBX + vBY * vBY));
      let mB = ((vBX * vBX + vBY * vBY) as f64).sqrt();
      // Java: double cos = dotProduct / (mA * mB);
      let mut cos = dotProduct / (mA * mB);
      // Java: if (cos > (double)1.0F)
      if cos > 1.0 {
         // Java: cos = (double)1.0F;
         cos = 1.0;
      }
      // Java: if (cos < (double)-1.0F)
      if cos < -1.0 {
         // Java: cos = (double)-1.0F;
         cos = -1.0;
      }
      // Java: double angleRad = Math.acos(cos);
      let angleRad = cos.acos();
      // Java: return getDirection(vAX, vAY, vBX, vBY) ? -Math.toDegrees(angleRad) : Math.toDegrees(angleRad);
      if Self::getDirection(vAX, vAY, vBX, vBY) {
         -angleRad.to_degrees()
      } else {
         angleRad.to_degrees()
      }
    }

    // Java: static boolean getDirection(float vAX, float vAY, float vBX, float vBY)
    pub fn getDirection(vAX: f32, vAY: f32, vBX: f32, vBY: f32) -> bool {
      // Java: return vAX * vBY - vAY * vBX < 0.0F;
      vAX * vBY - vAY * vBX < 0.0
    }

    // Java: static RectF getFlipRect(RectF rectF, float cx, float cy, boolean bFlipH, boolean bFlipV)
    pub fn getFlipRect(rectF: &mut RectF, cx: f32, cy: f32, bFlipH: bool, bFlipV: bool) -> RectF {
      // Java: if (bFlipH)
      if bFlipH {
         // Java: float d1 = cx - rectF.left;
         let d1 = cx - rectF.left;
         // Java: float d2 = rectF.right - cx;
         let d2 = rectF.right - cx;
         // Java: if ((int)d1 != (int)d2)
         if d1 as i32 != d2 as i32 {
            // Java: rectF.left = cx - d2;
            rectF.left = cx - d2;
            // Java: rectF.right = cx + d1;
            rectF.right = cx + d1;
         }
      }
      // Java: if (bFlipV)
      if bFlipV {
         // Java: float d1 = cy - rectF.top;
         let d1 = cy - rectF.top;
         // Java: float d2 = rectF.bottom - cy;
         let d2 = rectF.bottom - cy;
         // Java: if ((int)d1 != (int)d2)
         if d1 as i32 != d2 as i32 {
            // Java: rectF.top = cy - d2;
            rectF.top = cy - d2;
            // Java: rectF.bottom = cy + d1;
            rectF.bottom = cy + d1;
         }
      }
      // Java: return rectF;
      RectF::from_rectf(rectF)
    }

    // Java: static RectF getFlipRotRect(RectF rectF, float cx, float cy, boolean bFlipH, boolean bFlipV, float rot)
    pub fn getFlipRotRect(rectF: &mut RectF, cx: f32, cy: f32, bFlipH: bool, bFlipV: bool, rot: f32) -> RectF {
      // Java: rectF = getFlipRect(rectF, cx, cy, bFlipH, bFlipV);
      let mut rectF_val = Self::getFlipRect(rectF, cx, cy, bFlipH, bFlipV);
      let mut rot = rot;
      // Java: if (rot != 0.0F)
      if rot != 0.0 {
         // Java: if (bFlipH && !bFlipV || !bFlipH && bFlipV)
         if bFlipH && !bFlipV || !bFlipH && bFlipV {
            // Java: rot = -rot;
            rot = -rot;
         }
         // Java: rectF = getRotRect(rectF, cx, cy, rot);
         rectF_val = Self::getRotRect(&rectF_val, cx, cy, rot);
      }
      // Java: return rectF;
      rectF_val
    }

    // Java: static boolean hasCropInfo(BlipFill fill)
    pub fn hasCropInfo(fill: &BlipFill) -> bool {
      // Java: if (!(fill instanceof Picture))
      // LLM No Rust equivalent: instanceof check - stub always returns false
      {
         // Java: return false;
         return false;
      }
      // Java: else
      // LLM No Rust equivalent: unreachable with stub
      {
         // Java: Picture pic = (Picture)fill;
         // LLM No Rust equivalent: downcast stub
         // Java: return pic.getCropLeft() != 0.0F || pic.getCropTop() != 0.0F || pic.getCropRight() != 0.0F || pic.getCropBottom() != 0.0F;
         // LLM No Rust equivalent: stub
         false
      }
    }

}
