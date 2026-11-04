#![allow(non_snake_case, unused, non_camel_case_types, dead_code)]
// Java: import cn.wps.graphics.PointF
// use util::cn::wps::graphics::PointF;
// Java: import cn.wps.graphics.RectF
// use util::cn::wps::graphics::RectF;
// Java: import cn.wps.moffice.drawing.GRF
// use drawing::cn::wps::moffice::drawing::GRF;
// Java: import cn.wps.moffice.drawing.HitShapeStatus
// use drawing::cn::wps::moffice::drawing::HitShapeStatus;
// Java: import cn.wps.moffice.drawing.Shape
// use drawing::cn::wps::moffice::drawing::Shape;
// Java: import cn.wps.moffice.drawing.ShapeHelper
// use drawing::cn::wps::moffice::drawing::ShapeHelper;
// Java: import cn.wps.moffice.drawing.smartshape.PresetShape
// use drawing::cn::wps::moffice::drawing::smartshape::PresetShape;
// Java: import cn.wps.moffice.drawing.smartshape.ShapeType
// use drawing::cn::wps::moffice::drawing::smartshape::ShapeType;
// Java: import cn.wps.moffice.drawing.util.ShapeUtil
// use drawing::cn::wps::moffice::drawing::util::ShapeUtil;

#[derive(Default)]
pub struct PointF {
    pub x: f32,
    pub y: f32,
}

#[derive(Default)]
pub struct RectF {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl RectF {
    pub fn width(&self) -> f32 { self.right - self.left }
    pub fn height(&self) -> f32 { self.bottom - self.top }
    pub fn centerX(&self) -> f32 { (self.left + self.right) / 2.0 }
    pub fn centerY(&self) -> f32 { (self.top + self.bottom) / 2.0 }
    pub fn contains(&self, _x: f32, _y: f32) -> bool { false }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct GRF;

#[derive(Default, Clone, Copy, PartialEq)]
pub enum HitPos {
    #[default]
    None,
    ControlHandle1,
    ControlHandle2,
    ControlHandle3,
    ControlHandle4,
    ControlHandle5,
    ControlHandle6,
    ControlHandle7,
    ControlHandle8,
    Rotate,
    Body,
    Edge,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum HitShapeStatus {
    #[default]
    None,
}

#[derive(Default)]
pub struct Shape;

impl Shape {
    pub fn getShapeType(&self) -> ShapeType { Default::default() }
    pub fn isLine(&self) -> bool { false }
    pub fn getRotation(&self) -> f32 { 0.0 }
    pub fn isFlipH(&self) -> bool { false }
    pub fn isFlipV(&self) -> bool { false }
    pub fn getAdjustValues(&self) -> Vec<f32> { Vec::new() }
}

pub struct ShapeHelper;
impl ShapeHelper {
    pub fn isPresetShape(_shape: &Shape) -> bool { false }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct ShapeType;

pub struct PresetShape;
impl PresetShape {
    pub fn isLineOrConnector(_t: ShapeType) -> bool { false }
}

pub struct ShapeUtil;
impl ShapeUtil {
    pub fn getHitCtrlPointRadiusNew(_grf: GRF) -> f32 { 0.0 }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum EditType {
    #[default]
    None,
}

// Java: public class ShapeRange
pub struct ShapeRange {
}

// Java: public class ShapeRange
impl ShapeRange {
    // Java: ShapeRange()
    pub fn ShapeRange(&self) {
      // Java: super();
      // LLM No Rust equivalent
    }

    // Java: HitPos hitTestShapeHandles(Shape shape, RectF rectShape, float x, float y, float rot, float zoom)
    pub fn hitTestShapeHandles(&self, shape: Shape, rectShape: RectF, x: f32, y: f32, rot: f32, zoom: f32) -> HitPos {
      // Java: PointF pt = new PointF(x, y);
      // LLM No Rust equivalent
      // Java: int distX = ShapeHelper.getCtrlRadius(rectShape.width(), zoom);
      // LLM No Rust equivalent
      // Java: int distY = ShapeHelper.getCtrlRadius(rectShape.height(), zoom);
      // LLM No Rust equivalent
      // Java: PointF ptCenter = new PointF(rectShape.centerX(), rectShape.centerY());
      // LLM No Rust equivalent
      // Java: boolean bLine = ShapeType.isLine(shape.getShapeType());
      // LLM No Rust equivalent
      // Java: boolean bFlip = false;
      // LLM No Rust equivalent
      // Java: if (rot != 0.0F || bLine)
        // LLM No Rust equivalent
      {
         // Java: bFlip = this.isFlip(shape);
         // LLM No Rust equivalent
      }
      // Java: if (rot != 0.0F)
        // LLM No Rust equivalent
      {
         // Java: pt = ShapeHelper.getRotPoint(pt, ptCenter, -rot);
         // LLM No Rust equivalent
      }
      // Java: HitPos hitPos = this.hitTestShapeAdjust(shape, rectShape, pt);
      // LLM No Rust equivalent
      // Java: if (hitPos != null)
        // LLM No Rust equivalent
      {
         // Java: return hitPos;
         // LLM No Rust equivalent
      }
      // Java: else
        // LLM No Rust equivalent
      {
         // Java: return bLine ? this.hitLine(rectShape, pt, (float)distX, (float)distY, bFlip) : this.hitOther(shape, rectShape, pt, (float)distX, (float)distY, bFlip, zoom);
         // LLM No Rust equivalent
             Default::default()
      }
    }

    // Java: HitPos hitLine(RectF rectF, PointF pt, float distX, float distY, boolean bFlip)
    pub fn hitLine(&self, rectF: RectF, pt: PointF, distX: f32, distY: f32, bFlip: bool) -> HitPos {
      // Java: if (bFlip)
        // LLM No Rust equivalent
      {
         // Java: if (Math.abs(pt.x - rectF.right) < distX && Math.abs(pt.y - rectF.top) < distY)
            // LLM No Rust equivalent
         {
            // Java: return HitPos.RightTop;
            // LLM No Rust equivalent
                Default::default()
         }
         // Java: if (Math.abs(pt.x - rectF.left) < distX && Math.abs(pt.y - rectF.bottom) < distY)
            // LLM No Rust equivalent
         {
            // Java: return HitPos.LeftBottom;
            // LLM No Rust equivalent
                Default::default()
         }
      }
      // Java: else
        // LLM No Rust equivalent
      {
         // Java: if (Math.abs(pt.x - rectF.left) < distX && Math.abs(pt.y - rectF.top) < distY)
            // LLM No Rust equivalent
         {
            // Java: return HitPos.LeftTop;
            // LLM No Rust equivalent
                Default::default()
         }
         // Java: if (Math.abs(pt.x - rectF.right) < distX && Math.abs(pt.y - rectF.bottom) < distY)
            // LLM No Rust equivalent
         {
            // Java: return HitPos.RightBottom;
            // LLM No Rust equivalent
                Default::default()
         }
      }
      // Java: return rectF.contains(pt.x, pt.y) ? HitPos.Region : HitPos.None;
      // LLM No Rust equivalent
          Default::default()
    }

    // Java: boolean isInXLineRange(PointF touchPoint, float baseY, float xMin, float xMax, float dist)
    pub fn isInXLineRange(&self, touchPoint: PointF, baseY: f32, xMin: f32, xMax: f32, dist: f32) -> bool {
      // Java: float yOffset = Math.abs(touchPoint.y - baseY);
      // LLM No Rust equivalent
      // Java: return yOffset < dist && touchPoint.x > xMin && touchPoint.x < xMax;
      // LLM No Rust equivalent
          false
    }

    // Java: boolean isInYLineRange(PointF touchPoint, float baseX, float yMin, float yMax, float dist)
    pub fn isInYLineRange(&self, touchPoint: PointF, baseX: f32, yMin: f32, yMax: f32, dist: f32) -> bool {
      // Java: float xOffset = Math.abs(touchPoint.x - baseX);
      // LLM No Rust equivalent
      // Java: return xOffset < dist && touchPoint.y > yMin && touchPoint.y < yMax;
      // LLM No Rust equivalent
          false
    }

    // Java: boolean isHoverHitCtrlPoint(RectF hoverRect, float centerX, float centerY, float ctrlPointRadius)
    pub fn isHoverHitCtrlPoint(&self, hoverRect: RectF, centerX: f32, centerY: f32, ctrlPointRadius: f32) -> bool {
      // Java: RectF rectF = RectF.obtain();
      // LLM No Rust equivalent
      // Java: rectF.set(centerX - ctrlPointRadius, centerY - ctrlPointRadius, centerX + ctrlPointRadius, centerY + ctrlPointRadius);
      // LLM No Rust equivalent
      // Java: boolean isHit = rectF.intersect(hoverRect);
      // LLM No Rust equivalent
      // Java: rectF.recycle();
      // LLM No Rust equivalent
      // Java: return isHit;
      // LLM No Rust equivalent
          false
    }

    // Java: HitPos hitOther(Shape shape, RectF rectShape, PointF pt, float distX, float distY, boolean bFlip, float zoom)
    pub fn hitOther(&self, shape: Shape, rectShape: RectF, pt: PointF, distX: f32, distY: f32, bFlip: bool, zoom: f32) -> HitPos {
      // Java: boolean isInClip = HitShapeStatus.isInClip();
      // LLM No Rust equivalent
      // Java: if (isInClip)
        // LLM No Rust equivalent
      {
         // Java: float stroke = ShapeHelper.getMultiple() / zoom;
         // LLM No Rust equivalent
         // Java: RectF adjustRect = new RectF(rectShape);
         // LLM No Rust equivalent
         // Java: adjustRect.expand(stroke, stroke);
         // LLM No Rust equivalent
         // Java: rectShape = adjustRect;
         // LLM No Rust equivalent
             Default::default()
      }
      // Java: RectF rectF = ShapeHelper.getCtrlRect(rectShape, zoom);
      // LLM No Rust equivalent
      // Java: boolean fromMouse = HitShapeStatus.isFromMouse();
      // LLM No Rust equivalent
      // Java: float clipCtrlLength = fromMouse ? ShapeHelper.getClipCtrlLengh(zoom) : 0.0F;
      // LLM No Rust equivalent
      // Java: float clipCtrlDist = fromMouse ? ShapeHelper.getClipCtrlStroke(zoom) : 0.0F;
      // LLM No Rust equivalent
      // Java: float radius = ShapeHelper.getPointRadius(zoom);
      // LLM No Rust equivalent
      // Java: float hoverHeight = ShapeHelper.getHoverHeight(zoom);
      // LLM No Rust equivalent
      // Java: if (!fromMouse)
        // LLM No Rust equivalent
      {
         // Java: if (Math.abs(pt.x - rectF.left) < distX && Math.abs(pt.y - rectF.top) < distY)
            // LLM No Rust equivalent
         {
            // Java: return HitPos.LeftTop;
            // LLM No Rust equivalent
         }
         // Java: if (Math.abs(pt.x - rectF.right) < distX && Math.abs(pt.y - rectF.bottom) < distY)
            // LLM No Rust equivalent
         {
            // Java: return HitPos.RightBottom;
            // LLM No Rust equivalent
         }
         // Java: if (Math.abs(pt.x - rectF.right) < distX && Math.abs(pt.y - rectF.top) < distY)
            // LLM No Rust equivalent
         {
            // Java: return HitPos.RightTop;
            // LLM No Rust equivalent
         }
         // Java: if (Math.abs(pt.x - rectF.left) < distX && Math.abs(pt.y - rectF.bottom) < distY)
            // LLM No Rust equivalent
         {
            // Java: return HitPos.LeftBottom;
            // LLM No Rust equivalent
         }
      }
      // Java: else
        // LLM No Rust equivalent
      {
         // Java: RectF hoverRect = RectF.obtain();
         // LLM No Rust equivalent
         // Java: float hoverWidth = ShapeHelper.getHoverWidth(zoom);
         // LLM No Rust equivalent
         // Java: hoverRect.set(pt.x, pt.y, pt.x + hoverWidth, pt.y + hoverWidth);
         // LLM No Rust equivalent
         // Java: boolean isClipHit = isInClip && (this.isInXLineRange(pt, rectF.top, rectF.left, rectF.left + clipCtrlLength, clipCtrlDist) || this.isInYLineRange(pt, rectF.left, rectF.top, rectF.top + clipCtrlLength, clipCtrlDist));
         // LLM No Rust equivalent
         // Java: boolean isSelHit = !isInClip && this.isHoverHitCtrlPoint(hoverRect, rectF.left, rectF.top, radius);
         // LLM No Rust equivalent
         // Java: if (isClipHit || isSelHit)
            // LLM No Rust equivalent
         {
            // Java: hoverRect.recycle();
            // LLM No Rust equivalent
            // Java: return HitPos.LeftTop;
            // LLM No Rust equivalent
         }
         // Java: hoverRect.set(pt.x, pt.y - hoverWidth, pt.x + hoverWidth, pt.y);
         // LLM No Rust equivalent
         // Java: isClipHit = isInClip && (this.isInXLineRange(pt, rectF.bottom, rectF.left, rectF.left + clipCtrlLength, clipCtrlDist) || this.isInYLineRange(pt, rectF.left, rectF.bottom - clipCtrlLength, rectF.bottom, clipCtrlDist));
         // LLM No Rust equivalent
         // Java: isSelHit = !isInClip && this.isHoverHitCtrlPoint(hoverRect, rectF.left, rectF.bottom, radius);
         // LLM No Rust equivalent
         // Java: if (isClipHit || isSelHit)
            // LLM No Rust equivalent
         {
            // Java: hoverRect.recycle();
            // LLM No Rust equivalent
            // Java: return HitPos.LeftBottom;
            // LLM No Rust equivalent
         }
         // Java: hoverRect.set(pt.x - hoverWidth, pt.y - hoverWidth, pt.x, pt.y);
         // LLM No Rust equivalent
         // Java: isClipHit = isInClip && (this.isInXLineRange(pt, rectF.bottom, rectF.right - clipCtrlLength, rectF.right, clipCtrlDist) || this.isInYLineRange(pt, rectF.right, rectF.bottom - clipCtrlLength, rectF.bottom, clipCtrlDist));
         // LLM No Rust equivalent
         // Java: isSelHit = !isInClip && this.isHoverHitCtrlPoint(hoverRect, rectF.right, rectF.bottom, radius);
         // LLM No Rust equivalent
         // Java: if (isClipHit || isSelHit)
            // LLM No Rust equivalent
         {
            // Java: hoverRect.recycle();
            // LLM No Rust equivalent
            // Java: return HitPos.RightBottom;
            // LLM No Rust equivalent
         }
         // Java: hoverRect.set(pt.x - hoverWidth, pt.y, pt.x, pt.y + hoverWidth);
         // LLM No Rust equivalent
         // Java: isClipHit = isInClip && (this.isInXLineRange(pt, rectF.top, rectF.right - clipCtrlLength, rectF.right, clipCtrlDist) || this.isInYLineRange(pt, rectF.right, rectF.top, rectF.top + clipCtrlLength, clipCtrlDist));
         // LLM No Rust equivalent
         // Java: isSelHit = !isInClip && this.isHoverHitCtrlPoint(hoverRect, rectF.right, rectF.top, radius);
         // LLM No Rust equivalent
         // Java: if (isClipHit || isSelHit)
            // LLM No Rust equivalent
         {
            // Java: hoverRect.recycle();
            // LLM No Rust equivalent
            // Java: return HitPos.RightTop;
            // LLM No Rust equivalent
         }
         // Java: hoverRect.recycle();
         // LLM No Rust equivalent
      }
      // Java: float centerX = rectF.centerX();
      // LLM No Rust equivalent
      // Java: float centerY = rectF.centerY();
      // LLM No Rust equivalent
      // Java: if (fromMouse)
        // LLM No Rust equivalent
      {
         // Java: RectF hoverRect = RectF.obtain();
         // LLM No Rust equivalent
         // Java: float hoverHalfWidth = ShapeHelper.getHoverWidth(zoom) / 2.0F;
         // LLM No Rust equivalent
         // Java: hoverRect.set(pt.x - hoverHalfWidth, pt.y - hoverHeight, pt.x + hoverHalfWidth, pt.y);
         // LLM No Rust equivalent
         // Java: if (isInClip && this.isInXLineRange(pt, rectF.top, centerX - clipCtrlLength, centerX + clipCtrlLength, clipCtrlDist) || !isInClip && this.isHoverHitCtrlPoint(hoverRect, centerX, rectF.top, radius))
            // LLM No Rust equivalent
         {
            // Java: hoverRect.recycle();
            // LLM No Rust equivalent
            // Java: return HitPos.Top;
            // LLM No Rust equivalent
         }
         // Java: hoverRect.set(pt.x, pt.y - hoverHalfWidth, pt.x + hoverHeight, pt.y + hoverHalfWidth);
         // LLM No Rust equivalent
         // Java: if (isInClip && this.isInYLineRange(pt, rectF.right, centerY - clipCtrlLength, centerY + clipCtrlLength, clipCtrlDist) || !isInClip && this.isHoverHitCtrlPoint(hoverRect, rectF.right, centerY, radius))
            // LLM No Rust equivalent
         {
            // Java: hoverRect.recycle();
            // LLM No Rust equivalent
            // Java: return HitPos.Right;
            // LLM No Rust equivalent
         }
         // Java: hoverRect.set(pt.x - hoverHalfWidth, pt.y, pt.x + hoverHalfWidth, pt.y + hoverHeight);
         // LLM No Rust equivalent
         // Java: if (isInClip && this.isInXLineRange(pt, rectF.bottom, centerX - clipCtrlLength, centerX + clipCtrlLength, clipCtrlDist) || !isInClip && this.isHoverHitCtrlPoint(hoverRect, centerX, rectF.bottom, radius))
            // LLM No Rust equivalent
         {
            // Java: hoverRect.recycle();
            // LLM No Rust equivalent
            // Java: return HitPos.Bottom;
            // LLM No Rust equivalent
         }
         // Java: hoverRect.set(pt.x - hoverHeight, pt.y - hoverHalfWidth, pt.x, pt.y + hoverHalfWidth);
         // LLM No Rust equivalent
         // Java: if (isInClip && this.isInYLineRange(pt, rectF.left, centerY - clipCtrlLength, centerY + clipCtrlLength, clipCtrlDist) || !isInClip && this.isHoverHitCtrlPoint(hoverRect, rectF.left, centerY, radius))
            // LLM No Rust equivalent
         {
            // Java: hoverRect.recycle();
            // LLM No Rust equivalent
            // Java: return HitPos.Left;
            // LLM No Rust equivalent
         }
         // Java: hoverRect.recycle();
         // LLM No Rust equivalent
      }
      // Java: else
        // LLM No Rust equivalent
      {
         // Java: if (Math.abs(pt.x - centerX) < distX && Math.abs(pt.y - rectF.top) < distY)
            // LLM No Rust equivalent
         {
            // Java: return HitPos.Top;
            // LLM No Rust equivalent
         }
         // Java: if (Math.abs(pt.x - rectF.right) < distX && Math.abs(pt.y - centerY) < distY)
            // LLM No Rust equivalent
         {
            // Java: return HitPos.Right;
            // LLM No Rust equivalent
         }
         // Java: if (Math.abs(pt.x - centerX) < distX && Math.abs(pt.y - rectF.bottom) < distY)
            // LLM No Rust equivalent
         {
            // Java: return HitPos.Bottom;
            // LLM No Rust equivalent
         }
         // Java: if (Math.abs(pt.x - rectF.left) < distX && Math.abs(pt.y - centerY) < distY)
            // LLM No Rust equivalent
         {
            // Java: return HitPos.Left;
            // LLM No Rust equivalent
         }
      }
      // Java: boolean isChild = shape.getParent() != null;
      // LLM No Rust equivalent
      // Java: if (!isInClip && !ShapeType.isLine(shape.getShapeType()) && !isChild && !shape.isMutlSel && shape.canRotate() && !this.canOpenOle(shape))
        // LLM No Rust equivalent
      {
         // Java: float hotDistX = (float)ShapeHelper.getCtrlRadiusForRot(zoom);
         // LLM No Rust equivalent
         // Java: float hotDistY = (float)ShapeHelper.getCtrlRadiusForRot(zoom);
         // LLM No Rust equivalent
         // Java: float rotPointDes = ShapeHelper.getRotPointDes(zoom);
         // LLM No Rust equivalent
         // Java: boolean isFlipV = shape.getGRF().isFlipV();
         // LLM No Rust equivalent
         // Java: boolean isFlipH = shape.getGRF().isFlipH();
         // LLM No Rust equivalent
         // Java: boolean isHitCenterX = fromMouse ? Math.abs(pt.x - centerX) < radius : Math.abs(pt.x - centerX) < distX;
         // LLM No Rust equivalent
         // Java: boolean isHitCenterY = fromMouse ? Math.abs(pt.y - centerY) < radius : Math.abs(pt.y - centerY) < distY;
         // LLM No Rust equivalent
         // Java: boolean isHitRotPoint = false;
         // LLM No Rust equivalent
         // Java: switch (shape.rotPos)
            // LLM No Rust equivalent
         {
            // Java: case TOP:
            // LLM No Rust equivalent
               // Java: float base = isFlipV ? rectF.bottom : rectF.top;
               // LLM No Rust equivalent
               // Java: float distance = isFlipV ? pt.y - base : base - pt.y;
               // LLM No Rust equivalent
               // Java: if (isHitCenterX && this.isHitRotPoint(fromMouse, distance, rotPointDes, radius, hotDistY))
                // LLM No Rust equivalent
               {
                  // Java: isHitRotPoint = true;
                  // LLM No Rust equivalent
               }
               // Java: break;
               // LLM No Rust equivalent
            // Java: case BOTTOM:
            // LLM No Rust equivalent
               // Java: base = isFlipV ? rectF.top : rectF.bottom;
               // LLM No Rust equivalent
               // Java: distance = isFlipV ? base - pt.y : pt.y - base;
               // LLM No Rust equivalent
               // Java: if (isHitCenterX && this.isHitRotPoint(fromMouse, distance, rotPointDes, radius, hotDistY))
                // LLM No Rust equivalent
               {
                  // Java: isHitRotPoint = true;
                  // LLM No Rust equivalent
               }
               // Java: break;
               // LLM No Rust equivalent
            // Java: case RIGHT:
            // LLM No Rust equivalent
               // Java: base = isFlipH ? rectF.left : rectF.right;
               // LLM No Rust equivalent
               // Java: distance = isFlipH ? base - pt.x : pt.x - base;
               // LLM No Rust equivalent
               // Java: if (isHitCenterY && this.isHitRotPoint(fromMouse, distance, rotPointDes, radius, hotDistX))
                // LLM No Rust equivalent
               {
                  // Java: isHitRotPoint = true;
                  // LLM No Rust equivalent
               }
               // Java: break;
               // LLM No Rust equivalent
            // Java: case LEFT:
            // LLM No Rust equivalent
               // Java: base = isFlipH ? rectF.right : rectF.left;
               // LLM No Rust equivalent
               // Java: distance = isFlipH ? pt.x - base : base - pt.x;
               // LLM No Rust equivalent
               // Java: if (isHitCenterY && this.isHitRotPoint(fromMouse, distance, rotPointDes, radius, hotDistX))
                // LLM No Rust equivalent
               {
                  // Java: isHitRotPoint = true;
                  // LLM No Rust equivalent
               }
         }
         // Java: if (isHitRotPoint)
            // LLM No Rust equivalent
         {
            // Java: return HitPos.Rotation;
            // LLM No Rust equivalent
         }
      }
      // Java: if (this.getEditType() == EditType.type_clip)
        // LLM No Rust equivalent
      {
         // Java: RectF rectPic = ShapeUtil.getPicRectNoClip(shape, rectF, true);
         // LLM No Rust equivalent
         // Java: if (rectPic.contains(pt.x, pt.y))
            // LLM No Rust equivalent
         {
            // Java: return HitPos.ClipPic;
            // LLM No Rust equivalent
                Default::default()
         }
      }
      // Java: if (rectShape.contains(pt.x, pt.y))
        // LLM No Rust equivalent
      {
         // Java: return HitPos.Region;
         // LLM No Rust equivalent
             Default::default()
      }
      // Java: else
        // LLM No Rust equivalent
      {
         // Java: return HitPos.None;
         // LLM No Rust equivalent
             Default::default()
      }
    }

    // Java: boolean isHitRotPoint(boolean fromMouse, float delta, float rotDes, float rotRadius, float hotDes)
    pub fn isHitRotPoint(&self, fromMouse: bool, delta: f32, rotDes: f32, rotRadius: f32, hotDes: f32) -> bool {
      // Java: if (!fromMouse)
        // LLM No Rust equivalent
      {
         // Java: return Math.abs(delta - rotDes) < hotDes;
         // LLM No Rust equivalent
      }
      // Java: else
        // LLM No Rust equivalent
      {
         // Java: float min = rotDes + 2.0F * rotRadius;
         // LLM No Rust equivalent
         // Java: float max = rotDes + 5.0F * rotRadius;
         // LLM No Rust equivalent
         // Java: return delta > min && delta < max;
         // LLM No Rust equivalent
      }
      false
    }

    // Java: boolean isFlip(Shape shape)
    pub fn isFlip(&self, shape: Shape) -> bool {
      // Java: GRF grf = shape.getGRF();
      // LLM No Rust equivalent
      // Java: boolean bFlipH = grf.isFlipH();
      // LLM No Rust equivalent
      // Java: boolean bFlipV = grf.isFlipV();
      // LLM No Rust equivalent
      // Java: return bFlipH && !bFlipV || !bFlipH && bFlipV;
      // LLM No Rust equivalent
          false
    }

    // Java: HitPos hitTestShapeAdjust(Shape shape, RectF rectShape, PointF pt)
    pub fn hitTestShapeAdjust(&self, shape: Shape, rectShape: RectF, pt: PointF) -> HitPos {
      // Java: if (this.count() > 1)
        // LLM No Rust equivalent
      {
         // Java: return null;
         // LLM No Rust equivalent
      }
      // Java: else
        // LLM No Rust equivalent
      {
         // Java: PresetShape presetShape = shape.getPresetShape();
         // LLM No Rust equivalent
         // Java: if (presetShape == null)
            // LLM No Rust equivalent
         {
            // Java: return null;
            // LLM No Rust equivalent
         }
         // Java: else
            // LLM No Rust equivalent
         {
            // Java: float radius = 15.0F;
            // LLM No Rust equivalent
            // Java: float x = pt.x - rectShape.left;
            // LLM No Rust equivalent
            // Java: float y = pt.y - rectShape.top;
            // LLM No Rust equivalent
            // Java: GRF grf = shape.getGRF();
            // LLM No Rust equivalent
            // Java: if (grf.isFlipH())
                // LLM No Rust equivalent
            {
               // Java: x = rectShape.width() - x;
               // LLM No Rust equivalent
            }
            // Java: if (grf.isFlipV())
                // LLM No Rust equivalent
            {
               // Java: y = rectShape.height() - y;
               // LLM No Rust equivalent
            }
            // Java: int count = presetShape.getAdjustCount();
            // LLM No Rust equivalent
            // Java: for(int i = 0; i < count; ++i)
                // LLM No Rust equivalent
            {
               // Java: PointF ptAdjust = presetShape.getAdjustPos(i);
               // LLM No Rust equivalent
               // Java: if (Math.abs(x - ptAdjust.x) < radius && Math.abs(y - ptAdjust.y) < radius)
                    // LLM No Rust equivalent
               {
                  // Java: return getHitControlHandler(i);
                  // LLM No Rust equivalent
               }
            }
            // Java: return null;
            // LLM No Rust equivalent
         }
      }
      Default::default()
    }

    // Java: static HitPos getHitControlHandler(int i)
    pub fn getHitControlHandler(&self, i: i32) -> HitPos {
      // Java: switch (i)
        // LLM No Rust equivalent
      {
         // Java: case 0:
         // LLM No Rust equivalent
            // Java: return HitPos.ControlHandle1;
            // LLM No Rust equivalent
         // Java: case 1:
         // LLM No Rust equivalent
            // Java: return HitPos.ControlHandle2;
            // LLM No Rust equivalent
         // Java: case 2:
         // LLM No Rust equivalent
            // Java: return HitPos.ControlHandle3;
            // LLM No Rust equivalent
         // Java: case 3:
         // LLM No Rust equivalent
            // Java: return HitPos.ControlHandle4;
            // LLM No Rust equivalent
         // Java: case 4:
         // LLM No Rust equivalent
            // Java: return HitPos.ControlHandle5;
            // LLM No Rust equivalent
         // Java: case 5:
         // LLM No Rust equivalent
            // Java: return HitPos.ControlHandle6;
            // LLM No Rust equivalent
         // Java: case 6:
         // LLM No Rust equivalent
            // Java: return HitPos.ControlHandle7;
            // LLM No Rust equivalent
         // Java: case 7:
         // LLM No Rust equivalent
            // Java: return HitPos.ControlHandle8;
            // LLM No Rust equivalent
         // Java: default:
         // LLM No Rust equivalent
            // Java: return null;
            // LLM No Rust equivalent
      }
      Default::default()
    }

    // Java: static int getHitControlHandler(HitPos hitPos)
    pub fn getHitControlHandler_from_hitPos(&self, hitPos: HitPos) -> i32 {
      // Java: switch (hitPos)
        // LLM No Rust equivalent
      {
         // Java: case ControlHandle1:
         // LLM No Rust equivalent
            // Java: return 0;
            // LLM No Rust equivalent
         // Java: case ControlHandle2:
         // LLM No Rust equivalent
            // Java: return 1;
            // LLM No Rust equivalent
         // Java: case ControlHandle3:
         // LLM No Rust equivalent
            // Java: return 2;
            // LLM No Rust equivalent
         // Java: case ControlHandle4:
         // LLM No Rust equivalent
            // Java: return 3;
            // LLM No Rust equivalent
         // Java: case ControlHandle5:
         // LLM No Rust equivalent
            // Java: return 4;
            // LLM No Rust equivalent
         // Java: case ControlHandle6:
         // LLM No Rust equivalent
            // Java: return 5;
            // LLM No Rust equivalent
         // Java: case ControlHandle7:
         // LLM No Rust equivalent
            // Java: return 6;
            // LLM No Rust equivalent
         // Java: case ControlHandle8:
         // LLM No Rust equivalent
            // Java: return 7;
            // LLM No Rust equivalent
         // Java: default:
         // LLM No Rust equivalent
            // Java: return -1;
            // LLM No Rust equivalent
      }
      0
    }

    // Java: abstract int count()
    pub fn count(&self) -> i32 {
    // LLM No Rust equivalent
        0
    }

    // Java: abstract Shape getShape(int var1)
    pub fn getShape(&self, var1: i32) -> Shape {
    // LLM No Rust equivalent
        Default::default()
    }

    // Java: abstract EditType getEditType()
    pub fn getEditType(&self) -> EditType {
    // LLM No Rust equivalent
        Default::default()
    }

    // Java: abstract boolean canOpenOle(Shape var1)
    pub fn canOpenOle(&self, var1: Shape) -> bool {
    // LLM No Rust equivalent
        false
    }

    // Java: static boolean isHitAdjust(HitPos hitPos)
    pub fn isHitAdjust(&self, hitPos: HitPos) -> bool {
      // Java: return hitPos == HitPos.ControlHandle1 || hitPos == HitPos.ControlHandle2 || hitPos == HitPos.ControlHandle3 || hitPos == HitPos.ControlHandle4 || hitPos == HitPos.ControlHandle5 || hitPos == HitPos.ControlHandle6 || hitPos == HitPos.ControlHandle7 || hitPos == HitPos.ControlHandle8;
      // LLM No Rust equivalent
          false
    }

}
