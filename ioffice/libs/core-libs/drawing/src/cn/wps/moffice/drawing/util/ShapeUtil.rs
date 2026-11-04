#![allow(non_snake_case, dead_code, unused_variables, unused_mut, unused_imports)]

// Java: import cn.wps.graphics.PointF
use core_libs_util::graphics::PointF::PointF;
// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.GRF
use crate::cn::wps::moffice::drawing::GRF::GRF;
// Java: import cn.wps.moffice.drawing.Shape
use crate::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.drawing.ShapeHelper
use crate::cn::wps::moffice::drawing::ShapeHelper::ShapeHelper;
// Java: import cn.wps.moffice.drawing.effects.Object3D
use crate::cn::wps::moffice::drawing::effects::Object3D::Object3D;
// Java: import cn.wps.moffice.drawing.graphics.GradFill
use crate::cn::wps::moffice::drawing::graphics::GradFill::GradFill;
// Java: import cn.wps.moffice.drawing.graphics.GradStop
use crate::cn::wps::moffice::drawing::graphics::GradStop::GradStop;
// Java: import cn.wps.moffice.drawing.graphics.GradStopList
use crate::cn::wps::moffice::drawing::graphics::GradStopList::GradStopList;
// Java: import cn.wps.moffice.drawing.graphics.GraphicsPath
use crate::cn::wps::moffice::drawing::graphics::GraphicsPath::GraphicsPath;
// Java: import cn.wps.moffice.drawing.graphics.PathElem
use crate::cn::wps::moffice::drawing::graphics::PathElem::PathElem;
// Java: import cn.wps.moffice.drawing.graphics.Picture
use crate::cn::wps::moffice::drawing::graphics::Picture::Picture;
// Java: import cn.wps.moffice.drawing.graphics.RenderModel
// Java: import cn.wps.moffice.drawing.line.Arrow
use crate::cn::wps::moffice::drawing::line::Arrow::Arrow;
// Java: import cn.wps.moffice.drawing.line.LineProperty
use crate::cn::wps::moffice::drawing::line::LineProperty::LineProperty;
// Java: import cn.wps.moffice.kfs.File

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java field: int MAXCOMPLEXCOUNT
pub const MAXCOMPLEXCOUNT: i32 = 100;
// Java field: int MAX_CHART_FILE
const MAX_CHART_FILE: i64 = 512000;
// Java field: int ARROW_UNIT
pub static ARROW_UNIT: i32 = 25;
// Java field: int FOCUS_TYPE1
pub const FOCUS_TYPE1: i32 = 100;
// Java field: int FOCUS_TYPE2
pub const FOCUS_TYPE2: i32 = 0;
// Java field: int FOCUS_TYPE3
pub const FOCUS_TYPE3: i32 = 50;
// Java field: int FOCUS_TYPE4
pub const FOCUS_TYPE4: i32 = -50;

// Java: public class ShapeUtil
pub struct ShapeUtil {
    _placeholder: (),
}

// Java: public class ShapeUtil
impl ShapeUtil {
    // Java: ShapeUtil()
    pub fn new() -> Self {
        // Java: super();
        ShapeUtil { _placeholder: () }
    }

    // Java: static PointF getOrigin(int algn)
    pub fn getOrigin(algn: i32) -> PointF {
        // Java: switch (algn)
        match algn {
            // Java: case 0: return new PointF(0.0F, 0.5F);
            0 => PointF::new_xy(0.0, 0.5),
            // Java: case 1:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: return new PointF(-0.5F, 0.5F);
            1 => PointF::new_xy(-0.5, 0.5),
            // Java: case 2:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: return new PointF(0.5F, 0.5F);
            2 => PointF::new_xy(0.5, 0.5),
            // Java: case 3:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: return new PointF(0.0F, 0.0F);
            3 => PointF::new_xy(0.0, 0.0),
            // Java: case 4:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: return new PointF(-0.5F, 0.0F);
            4 => PointF::new_xy(-0.5, 0.0),
            // Java: case 5:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: return new PointF(0.5F, 0.0F);
            5 => PointF::new_xy(0.5, 0.0),
            // Java: case 6:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: return new PointF(0.0F, -0.5F);
            6 => PointF::new_xy(0.0, -0.5),
            // Java: case 7:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: return new PointF(-0.5F, -0.5F);
            7 => PointF::new_xy(-0.5, -0.5),
            // Java: case 8:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: return new PointF(0.5F, -0.5F);
            8 => PointF::new_xy(0.5, -0.5),
            // Java: default: return new PointF(0.0F, 0.5F);
            _ => PointF::new_xy(0.0, 0.5),
        }
    }

    // Java: static boolean isPresetShape(Shape shape)
    pub fn isPresetShape(shape: &mut Shape) -> bool {
        // Java: int shapeType = shape.getShapeType();
        let shapeType = shape.getShapeType();
        // Java: if (shapeType != 100 && shapeType != 99 && shapeType != 204)
        if shapeType != 100 && shapeType != 99 && shapeType != 204 {
            // Java: return shapeType > 0 && shapeType < 264;
            shapeType > 0 && shapeType < 264
        // Java: else
        } else {
            // Java: return shape.getGeometry() == null || shape.getGeometry().getVertices() == null || shape.getGeometry().getVertices().length == 0;
            // LLM No Rust equivalent: getGeometry returns Element, vertices access not available via Any
            shape.getGeometry().is_none()
        }
    }

    // Java: static boolean isDmlShape(Shape shape)
    pub fn isDmlShape(shape: &Shape) -> bool {
        // Java: int shapeType = shape.getShapeType();
        let shapeType = shape.getShapeType();
        // Java: return shapeType > 220 && shapeType < 264 || shapeType == 75;
        (shapeType > 220 && shapeType < 264) || shapeType == 75
    }

    // Java: RectF getEffectRect(Shape shape, RectF rectF, float rot)
    pub fn getEffectRect(&self, shape: &mut Shape, rectF: RectF, rot: f32) -> RectF {
        // Java: return rectF;
        rectF
    }

    // Java: static RectF getPicRectNoClip(RenderModel shape, RectF rectF)
    pub fn getPicRectNoClip(shape: &mut Shape, rectF: &RectF) -> RectF {
        // Java: return getPicRectNoClip(shape, rectF, false);
        Self::getPicRectNoClip_flip(shape, rectF, false)
    }

    // Java: static RectF getPicRectNoClip(RenderModel shape, RectF rectF, boolean isFlip)
    pub fn getPicRectNoClip_flip(shape: &mut Shape, rectF: &RectF, isFlip: bool) -> RectF {
        // Java: Picture picture = shape.getPicture();
        let picture_elem = shape.getPicture();
        // LLM No Rust equivalent: getPicture returns Element, downcast to Picture
        let picture = picture_elem.as_ref().and_then(|p| p.downcast_ref::<Picture>());
        // Java: RectF _rect = getPicRectNoClip(picture, rectF);
        let mut _rect = if let Some(pic) = picture {
            Self::getPicRectNoClip_picture(pic, rectF)
        } else {
            rectF.clone()
        };
        // Java: if (isFlip)
        if isFlip {
            // Java: if (shape.isFlipH())
            if shape.isFlipH() {
                // Java: float cx = rectF.centerX();
                let cx = (rectF.left + rectF.right) / 2.0;
                // Java: float left = _rect.left;
                let left = _rect.left;
                // Java: _rect.left = cx + (cx - _rect.right);
                _rect.left = cx + (cx - _rect.right);
                // Java: _rect.right = cx + (cx - left);
                _rect.right = cx + (cx - left);
            }
            // Java: if (shape.isFlipV())
            if shape.isFlipV() {
                // Java: float cy = rectF.centerY();
                let cy = (rectF.top + rectF.bottom) / 2.0;
                // Java: float top = _rect.top;
                let top = _rect.top;
                // Java: _rect.top = cy + (cy - _rect.bottom);
                _rect.top = cy + (cy - _rect.bottom);
                // Java: _rect.bottom = cy + (cy - top);
                _rect.bottom = cy + (cy - top);
            }
        }
        // Java: return _rect;
        _rect
    }

    // Java: static RectF getPicRectNoClip(Picture picture, RectF rectF)
    pub fn getPicRectNoClip_picture(picture: &Picture, rectF: &RectF) -> RectF {
        // Java: float clipLeft = picture.getCropLeft();
        let clipLeft = picture.getCropLeft();
        // Java: float clipRight = picture.getCropRight();
        let clipRight = picture.getCropRight();
        // Java: float clipTop = picture.getCropTop();
        let clipTop = picture.getCropTop();
        // Java: float clipBottom = picture.getCropBottom();
        let clipBottom = picture.getCropBottom();
        // Java: if ((double)clipLeft == (double)0.0F && (double)clipRight == (double)0.0F && (double)clipTop == (double)0.0F && (double)clipBottom == (double)0.0F)
        if clipLeft as f64 == 0.0 && clipRight as f64 == 0.0 && clipTop as f64 == 0.0 && clipBottom as f64 == 0.0 {
            // Java: return new RectF(rectF);
            rectF.clone()
        // Java: else
        } else {
            // Java: float w = rectF.width() / (1.0F - clipLeft - clipRight);
            let w = rectF.width() / (1.0 - clipLeft - clipRight);
            // Java: float h = rectF.height() / (1.0F - clipTop - clipBottom);
            let h = rectF.height() / (1.0 - clipTop - clipBottom);
            // Java: RectF rectPic = new RectF(rectF.left - w * clipLeft, rectF.top - h * clipTop, rectF.right + w * clipRight, rectF.bottom + h * clipBottom);
            let rectPic = RectF::new_with(rectF.left - w * clipLeft, rectF.top - h * clipTop, rectF.right + w * clipRight, rectF.bottom + h * clipBottom);
            // Java: return rectPic;
            rectPic
        }
    }

    // Java: static RectF getPicRectClip(Shape shape, RectF rectF)
    pub fn getPicRectClip(shape: &mut Shape, rectF: &RectF) -> RectF {
        // Java: Picture picture = shape.getPicture();
        let picture_elem = shape.getPicture();
        // LLM No Rust equivalent: getPicture returns Element, downcast to Picture
        let picture = picture_elem.as_ref().and_then(|p| p.downcast_ref::<Picture>());
        if picture.is_none() {
            return rectF.clone();
        }
        let picture = picture.unwrap();
        // Java: float clipLeft = picture.getCropLeft();
        let clipLeft = picture.getCropLeft();
        // Java: float clipRight = picture.getCropRight();
        let clipRight = picture.getCropRight();
        // Java: float clipTop = picture.getCropTop();
        let clipTop = picture.getCropTop();
        // Java: float clipBottom = picture.getCropBottom();
        let clipBottom = picture.getCropBottom();
        // Java: if ((double)clipLeft >= (double)0.0F && (double)clipRight >= (double)0.0F && (double)clipTop >= (double)0.0F && (double)clipBottom >= (double)0.0F)
        if clipLeft as f64 >= 0.0 && clipRight as f64 >= 0.0 && clipTop as f64 >= 0.0 && clipBottom as f64 >= 0.0 {
            // Java: return rectF;
            return rectF.clone();
        // Java: else
        } else {
            // Java: float w = rectF.width() / (1.0F - clipLeft - clipRight);
            let w = rectF.width() / (1.0 - clipLeft - clipRight);
            // Java: float h = rectF.height() / (1.0F - clipTop - clipBottom);
            let h = rectF.height() / (1.0 - clipTop - clipBottom);
            // Java: RectF rectPic = new RectF(rectF);
            let mut rectPic = rectF.clone();
            // Java: if (clipLeft < 0.0F)
            if clipLeft < 0.0 {
                // Java: rectPic.left -= w * clipLeft;
                rectPic.left -= w * clipLeft;
            }
            // Java: if (clipRight < 0.0F)
            if clipRight < 0.0 {
                // Java: rectPic.right += w * clipRight;
                rectPic.right += w * clipRight;
            }
            // Java: if (clipTop < 0.0F)
            if clipTop < 0.0 {
                // Java: rectPic.top -= h * clipTop;
                rectPic.top -= h * clipTop;
            }
            // Java: if (clipBottom < 0.0F)
            if clipBottom < 0.0 {
                // Java: rectPic.bottom += h * clipBottom;
                rectPic.bottom += h * clipBottom;
            }
            // Java: return rectPic;
            rectPic
        }
    }

    // Java: static boolean isGroupShape(Shape shape)
    pub fn isGroupShape(shape: &mut Shape) -> bool {
        // Java: int shapeType = shape.getShapeType();
        let shapeType = shape.getShapeType();
        // Java: if (shapeType == 203)
        if shapeType == 203 {
            // Java: return true;
            true
        // Java: else
        } else {
            // Java: GRF grf = shape.getGRF();
            let grf = shape.getGRF();
            // Java: return null != grf ? grf.isGroup() : false;
            grf.as_ref()
                .and_then(|g| g.downcast_ref::<GRF>())
                .map(|g| g.isGroup())
                .unwrap_or(false)
        }
    }

    // Java: static boolean isPicture(Shape shape)
    pub fn isPicture(shape: &Shape) -> bool {
        // Java: int shapeType = shape.getShapeType();
        let shapeType = shape.getShapeType();
        // Java: return shapeType == 204;
        let shapeType = shapeType; return shapeType == 204;
    }

    // Java: static boolean isTextBox(Shape shape)
    pub fn isTextBox(shape: &Shape) -> bool {
        // Java: int shapeType = shape.getShapeType();
        let shapeType = shape.getShapeType();
        // Java: return shapeType == 202;
        let shapeType = shapeType; return shapeType == 202;
    }

    // Java: static boolean is3D(Shape shape)
    pub fn is3D(shape: &mut Shape) -> bool {
        // Java: Object3D object3d = shape.getObject3d();
        let object3d = shape.getObject3d();
        // Java: return object3d != null && object3d.get3D();
        object3d.as_ref()
            .and_then(|o| o.downcast_ref::<Object3D>())
            .map(|o3d| o3d.get3D())
            .unwrap_or(false)
    }

    // Java: boolean isInShape(Shape shape, RectF rectLayout, PointF pt, Shape[] result)
    pub fn isInShape_simple(&self, shape: &mut Shape, rectLayout: Option<RectF>, pt: &PointF, result: &mut Vec<Element>) -> bool {
        // Java: return this.isInShape(shape, rectLayout, pt, true, false, result);
        return self.isInShape(shape, rectLayout, pt, true, false, result);
    }

    // Java: boolean isInShape(Shape shape, RectF rectLayout, PointF pt, boolean isNeedRot, boolean isTextBox, Shape[] result)
    pub fn isInShape(&self, shape: &mut Shape, rectLayout: Option<RectF>, pt: &PointF, isNeedRot: bool, isTextBox: bool, result: &mut Vec<Element>) -> bool {
        let mut rectLayout = rectLayout;
        // Java: if (rectLayout == null)
        if rectLayout.is_none() {
            // Java: rectLayout = shape.getShapePos().getShapeRect();
            // LLM No Rust equivalent: getShapePos returns Element, using default rect
            rectLayout = Some(RectF::new());
        }
        let mut rectLayout = rectLayout.unwrap();
        // Java: PointF ptRot = new PointF(pt.x, pt.y);
        let mut ptRot = PointF::new_xy(pt.x, pt.y);
        // Java: if (isNeedRot)
        if isNeedRot {
            // Java: int rot = (int)shape.getRotation();
            let rot = shape.getRotation() as i32;
            // Java: if (rot != 0)
            if rot != 0 {
                // Java: PointF ptCenter = new PointF(rectLayout.centerX(), rectLayout.centerY());
                let ptCenter = PointF::new_xy((rectLayout.left + rectLayout.right) / 2.0, (rectLayout.top + rectLayout.bottom) / 2.0);
                // Java: ptRot = ShapeHelper.getRotPoint(ptRot, ptCenter, (float)(-rot));
                ptRot = ShapeHelper::getRotPoint_from_points(&ptRot, &ptCenter, (-rot) as f32);
            }
            // Java: GRF grf = shape.getGRF();
            let grf_elem = shape.getGRF();
            let grf = grf_elem.as_ref().and_then(|g| g.downcast_ref::<GRF>());
            if let Some(grf) = grf {
                // Java: if (grf.isFlipH())
                if grf.isFlipH() {
                    // Java: ptRot.x = rectLayout.centerX() * 2.0F - ptRot.x;
                    ptRot.x = (rectLayout.left + rectLayout.right) - ptRot.x;
                }
                // Java: if (grf.isFlipV())
                if grf.isFlipV() {
                    // Java: ptRot.y = rectLayout.centerY() * 2.0F - ptRot.y;
                    ptRot.y = (rectLayout.top + rectLayout.bottom) - ptRot.y;
                }
            }
        }
        // Java: return rectLayout.contains(ptRot.x, ptRot.y);
        rectLayout.contains(ptRot.x, ptRot.y)
    }

    // Java: static boolean isComplexShape(Shape shape)
    pub fn isComplexShape(shape: &mut Shape) -> bool {
        // Java: if (shape == null)
        // LLM Rust Null-Safety: shape is a reference, cannot be null in Rust
        // Java: return false;
        // LLM Rust Null-Safety: handled by Option/Result, no explicit check needed
        // Java: else if (shape.isGroupShape())
        if shape.isGroupShape() {
            // Java: return shape.count() > 100;
            return shape.count() > 100;
        // Java: else
        } else {
            // Java: Object3D object3d = shape.getObject3d();
            let object3d = shape.getObject3d();
            let object3d_ref = object3d.as_ref().and_then(|o| o.downcast_ref::<Object3D>());
            // Java: if (object3d != null && object3d.get3D())
            if object3d_ref.is_some() && object3d_ref.map(|o| o.get3D()).unwrap_or(false) {
                // Java: int type = shape.getShapeType();
                // LLM Rust Keyword: 'type' is reserved in Rust, using 'r#type'
                let r#type = shape.getShapeType();
                // Java: if (!shape.isArtText() && type != 1)
                if !shape.isArtText() && r#type != 1 {
                    // Java: return true;
                    return true;
                }
            }
            // Java: Shape chartShape = shape.getDrawingContainer() != null ? shape.getDrawingContainer().getChartShape(shape) : null;
            // LLM No Rust equivalent: getDrawingContainer returns Element, getChartShape not accessible; using shape directly
            // Java: if (null == chartShape)
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: chartShape = shape;
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: String path = chartShape.getProperty().getString(883, (String)null);
            let path = shape.base.mProperty.as_ref()
                .and_then(|mp| {
                    let val = mp.base.getValue(883);
                    val.and_then(|v| v.downcast_ref::<String>().cloned())
                });
            // Java: if (path != null)
            if let Some(ref path) = path {
                // Java: File file = new File(path);
                let file = std::path::Path::new(path.as_str());
                // Java: if (file != null && file.exists() && file.length() > 512000L)
                if file.is_file() && file.exists() && std::fs::metadata(file).map(|m| m.len()).unwrap_or(0) > MAX_CHART_FILE as u64 {
                    // Java: return true;
                    return true;
                }
            }
            // Java: return false;
            false
        }
    }

    // Java: static void expandRect(RectF rect, float x, float y)
    pub fn expandRect(rect: &mut RectF, x: f32, y: f32) {
        // Java: if (Float.isNaN(rect.left))
        if rect.left.is_nan() {
            // Java: rect.set(x, y, x, y);
            rect.set(x, y, x, y);
        // Java: else
        } else {
            // Java: if (x < rect.left)
            if x < rect.left {
                // Java: rect.left = x;
                rect.left = x;
            // Java: else if (x > rect.right)
            } else if x > rect.right {
                // Java: rect.right = x;
                rect.right = x;
            }
            // Java: if (y < rect.top)
            if y < rect.top {
                // Java: rect.top = y;
                rect.top = y;
            // Java: else if (y > rect.bottom)
            } else if y > rect.bottom {
                // Java: rect.bottom = y;
                rect.bottom = y;
            }
        }
    }

    // Java: static void calQuadOutRect(RectF outRect, float x1, float y1, float x2, float y2, float x3, float y3)
    pub fn calQuadOutRect(outRect: &mut RectF, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        // Java: int NUMSTEPS = 10;
        let NUMSTEPS: i32 = 10;
        // Java: float subdiv_step = 1.0F / (float)NUMSTEPS;
        let subdiv_step: f32 = 1.0 / NUMSTEPS as f32;
        // Java: float pre1x = x1 - 2.0F * x2 + x3;
        let pre1x = x1 - 2.0 * x2 + x3;
        // Java: float pre1y = y1 - 2.0F * y2 + y3;
        let pre1y = y1 - 2.0 * y2 + y3;
        // Java: float pre2x = x2 - x1;
        let pre2x = x2 - x1;
        // Java: float pre2y = y2 - y1;
        let pre2y = y2 - y1;
        // Java: float t = 0.0F;
        let mut t: f32 = 0.0;
        // Java: for(int i = 1; i < NUMSTEPS; ++i)
        for i in 1..NUMSTEPS {
            // Java: t += subdiv_step;
            t += subdiv_step;
            // Java: float dt = t * t;
            let dt = t * t;
            // Java: float x = dt * pre1x + 2.0F * t * pre2x + x1;
            let x = dt * pre1x + 2.0 * t * pre2x + x1;
            // Java: float y = dt * pre1y + 2.0F * t * pre2y + y1;
            let y = dt * pre1y + 2.0 * t * pre2y + y1;
            // Java: expandRect(outRect, x, y);
            Self::expandRect(outRect, x, y);
        }
        // Java: expandRect(outRect, x3, y3);
        Self::expandRect(outRect, x3, y3);
    }

    // Java: static void calCubicOutRect(RectF outRect, float x1, float y1, float x2, float y2, float x3, float y3, float x4, float y4)
    pub fn calCubicOutRect(outRect: &mut RectF, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32) {
        // Java: int NUMSTEPS = 10;
        let NUMSTEPS: i32 = 10;
        // Java: float subdiv_step = 1.0F / (float)(NUMSTEPS + 1);
        let subdiv_step: f32 = 1.0 / (NUMSTEPS + 1) as f32;
        // Java: float subdiv_step2 = subdiv_step * subdiv_step;
        let subdiv_step2 = subdiv_step * subdiv_step;
        // Java: float subdiv_step3 = subdiv_step * subdiv_step * subdiv_step;
        let subdiv_step3 = subdiv_step * subdiv_step * subdiv_step;
        // Java: float pre1 = 3.0F * subdiv_step;
        let pre1 = 3.0_f32 * subdiv_step;
        // Java: float pre2 = 3.0F * subdiv_step2;
        let pre2 = 3.0_f32 * subdiv_step2;
        // Java: float pre4 = 6.0F * subdiv_step2;
        let pre4 = 6.0_f32 * subdiv_step2;
        // Java: float pre5 = 6.0F * subdiv_step3;
        let pre5 = 6.0_f32 * subdiv_step3;
        // Java: float tmp1x = x1 - x2 * 2.0F + x3;
        let tmp1x = x1 - x2 * 2.0 + x3;
        // Java: float tmp1y = y1 - y2 * 2.0F + y3;
        let tmp1y = y1 - y2 * 2.0 + y3;
        // Java: float tmp2x = (x2 - x3) * 3.0F - x1 + x4;
        let tmp2x = (x2 - x3) * 3.0 - x1 + x4;
        // Java: float tmp2y = (y2 - y3) * 3.0F - y1 + y4;
        let tmp2y = (y2 - y3) * 3.0 - y1 + y4;
        // Java: float fx = x1;
        let mut fx = x1;
        // Java: float fy = y1;
        let mut fy = y1;
        // Java: float dfx = (x2 - x1) * pre1 + tmp1x * pre2 + tmp2x * subdiv_step3;
        let mut dfx = (x2 - x1) * pre1 + tmp1x * pre2 + tmp2x * subdiv_step3;
        // Java: float dfy = (y2 - y1) * pre1 + tmp1y * pre2 + tmp2y * subdiv_step3;
        let mut dfy = (y2 - y1) * pre1 + tmp1y * pre2 + tmp2y * subdiv_step3;
        // Java: float ddfx = tmp1x * pre4 + tmp2x * pre5;
        let mut ddfx = tmp1x * pre4 + tmp2x * pre5;
        // Java: float ddfy = tmp1y * pre4 + tmp2y * pre5;
        let mut ddfy = tmp1y * pre4 + tmp2y * pre5;
        // Java: float dddfx = tmp2x * pre5;
        let dddfx = tmp2x * pre5;
        // Java: float dddfy = tmp2y * pre5;
        let dddfy = tmp2y * pre5;
        // Java: int step = NUMSTEPS;
        let mut step = NUMSTEPS;
        // Java: while(step-- > 0)
        while { let old = step; step -= 1; old } > 0 {
            // Java: fx += dfx;
            fx += dfx;
            // Java: fy += dfy;
            fy += dfy;
            // Java: dfx += ddfx;
            dfx += ddfx;
            // Java: dfy += ddfy;
            dfy += ddfy;
            // Java: ddfx += dddfx;
            ddfx += dddfx;
            // Java: ddfy += dddfy;
            ddfy += dddfy;
            // Java: expandRect(outRect, fx, fy);
            Self::expandRect(outRect, fx, fy);
        }
        // Java: expandRect(outRect, x4, y4);
        Self::expandRect(outRect, x4, y4);
    }

    // Java: static RectF computeBounds(GraphicsPath path)
    pub fn computeBounds(path: Option<&GraphicsPath>) -> Option<RectF> {
        // Java: if (path == null)
        if path.is_none() {
            // Java: return null;
            return None;
        // Java: else
        } else {
            let path = path.unwrap();
            // Java: RectF outRect = new RectF(Float.NaN, Float.NaN, Float.NaN, Float.NaN);
            let mut outRect = RectF::new_with(f32::NAN, f32::NAN, f32::NAN, f32::NAN);
            // Java: PointF curPt = new PointF();
            let mut curPt = PointF::new();
            // Java: for(int j = 0; j < path.size(); ++j)
            for j in 0..path.size() {
                // Java: PathElem elem = path.getElem(j);
                let elem = path.getElem(j);
                // Java: float[] points = elem.getPoints();
                let points = elem.getPoints();
                // Java: if (points != null)
                if !points.is_empty() {
                    // Java: switch (elem.elemType)
                    match elem.elemType {
                        // Java: case 0:
                        // LLM JavaLine Aggregation: merged into the next Rust statement
                        0 => {
                            // Java: curPt.x = points[0];
                            curPt.x = points[0];
                            // Java: curPt.y = points[1];
                            curPt.y = points[1];
                            // Java: expandRect(outRect, points[0], points[1]);
                            Self::expandRect(&mut outRect, points[0], points[1]);
                            // Java: break;
                            // LLM Rust match arm implicitly breaks
                        }
                        // Java: case 1:
                        // LLM JavaLine Aggregation: merged into the next Rust statement
                        1 => {
                            // Java: expandRect(outRect, points[0], points[1]);
                            Self::expandRect(&mut outRect, points[0], points[1]);
                            // Java: curPt.x = points[0];
                            curPt.x = points[0];
                            // Java: curPt.y = points[1];
                            curPt.y = points[1];
                            // Java: break;
                            // LLM Rust match arm implicitly breaks
                        }
                        // Java: case 2:
                        // LLM JavaLine Aggregation: merged into the next Rust statement
                        2 => {
                            // Java: float startAngle = 0.0F;
                            let mut startAngle: f32 = 0.0;
                            // Java: float endAngle = 0.0F;
                            let mut endAngle: f32 = 0.0;
                            // Java: float beginX = curPt.x;
                            let mut beginX = curPt.x;
                            // Java: float beginY = curPt.y;
                            let mut beginY = curPt.y;
                            // Java: float w = 0.0F;
                            let mut w: f32 = 0.0;
                            // Java: float h = 0.0F;
                            let mut h: f32 = 0.0;
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
                            // Java: else
                            } else {
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
                            let centerX = beginX - w * ((startAngle as f64 * std::f64::consts::PI / 180.0).cos() as f32);
                            // Java: float centerY = beginY - h * (float)Math.sin((double)startAngle * Math.PI / (double)180.0F);
                            let centerY = beginY - h * ((startAngle as f64 * std::f64::consts::PI / 180.0).sin() as f32);
                            // Java: float endX = centerX + w * (float)Math.cos((double)endAngle * Math.PI / (double)180.0F);
                            let endX = centerX + w * ((endAngle as f64 * std::f64::consts::PI / 180.0).cos() as f32);
                            // Java: float endY = centerY + h * (float)Math.sin((double)endAngle * Math.PI / (double)180.0F);
                            let endY = centerY + h * ((endAngle as f64 * std::f64::consts::PI / 180.0).sin() as f32);
                            // Java: expandRect(outRect, beginX, beginY);
                            Self::expandRect(&mut outRect, beginX, beginY);
                            // Java: expandRect(outRect, endX, endY);
                            Self::expandRect(&mut outRect, endX, endY);
                            // Java: curPt.x = endX;
                            curPt.x = endX;
                            // Java: curPt.y = endY;
                            curPt.y = endY;
                            // Java: break;
                            // LLM Rust match arm implicitly breaks
                        }
                        // Java: case 3:
                        // LLM JavaLine Aggregation: merged into the next Rust statement
                        3 => {
                            // Java: calQuadOutRect(outRect, curPt.x, curPt.y, points[0], points[1], points[2], points[3]);
                            Self::calQuadOutRect(&mut outRect, curPt.x, curPt.y, points[0], points[1], points[2], points[3]);
                            // Java: curPt.x = points[2];
                            curPt.x = points[2];
                            // Java: curPt.y = points[3];
                            curPt.y = points[3];
                            // Java: break;
                            // LLM Rust match arm implicitly breaks
                        }
                        // Java: case 4:
                        // LLM JavaLine Aggregation: merged into the next Rust statement
                        4 => {
                            // Java: calCubicOutRect(outRect, curPt.x, curPt.y, points[0], points[1], points[2], points[3], points[4], points[5]);
                            Self::calCubicOutRect(&mut outRect, curPt.x, curPt.y, points[0], points[1], points[2], points[3], points[4], points[5]);
                            // Java: curPt.x = points[4];
                            curPt.x = points[4];
                            // Java: curPt.y = points[5];
                            curPt.y = points[5];
                            // Java: break;
                            // LLM Rust match arm implicitly breaks
                        }
                        // Java: case 5:
                        // LLM JavaLine Aggregation: merged into the next Rust statement
                        5 => {}
                        _ => {}
                    }
                }
            }
            // Java: return outRect;
            Some(outRect)
        }
    }

    // Java: static boolean isClosePath(GraphicsPath path)
    pub fn isClosePath(path: Option<&GraphicsPath>) -> bool {
        // Java: if (path == null)
        if path.is_none() {
            // Java: return false;
            return false;
        // Java: else
        } else {
            let path = path.unwrap();
            // Java: int size = path.size();
            let size = path.size();
            // Java: for(int i = size - 1; i >= 0; --i)
            let mut i = size - 1;
            while i >= 0 {
                // Java: PathElem elem = path.getElem(i);
                let elem = path.getElem(i);
                // Java: if (elem.elemType == 5)
                if elem.elemType == 5 {
                    // Java: return true;
                    return true;
                }
                i -= 1;
            }
            // Java: return false;
            return false;
        }
    }

    // Java: static float getArrowSpace(Arrow arrow, float strokeWidth)
    pub fn getArrowSpace(arrow: &Arrow, strokeWidth: f32) -> f32 {
        // Java: if (arrow.getHead() == 5 && arrow.needSpecialHanding())
        if arrow.getHead() == 5 && arrow.needSpecialHanding() {
            // Java: return 0.0F;
            return 0.0;
        // Java: else
        } else {
            // Java: float scaleX = arrow.getLineLength(strokeWidth) / (float)ARROW_UNIT / 2.0F;
            let scaleX = arrow.getLineLength(strokeWidth) / ARROW_UNIT as f32 / 2.0;
            // Java: float scaleY = arrow.getLineWidth(strokeWidth) / (float)ARROW_UNIT / 2.0F;
            let scaleY = arrow.getLineWidth(strokeWidth) / ARROW_UNIT as f32 / 2.0;
            // Java: double radian = Math.atan2((double)scaleY, (double)(2.0F * scaleX));
            let radian = (scaleY as f64).atan2((2.0 * scaleX) as f64);
            // Java: return (float)((double)strokeWidth / ((double)2.0F * Math.tan(radian)));
            (strokeWidth as f64 / (2.0 * radian.tan())) as f32
        }
    }

    // Java: static double getRotation(float xStart, float yStart, float xEnd, float yEnd)
    pub fn getRotation(xStart: f32, yStart: f32, xEnd: f32, yEnd: f32) -> f64 {
        // Java: float width = xEnd - xStart;
        let width = xEnd - xStart;
        // Java: float height = yEnd - yStart;
        let height = yEnd - yStart;
        // Java: double rotation = Math.atan2((double)height, (double)width) / Math.PI * (double)180.0F;
        let rotation = (height as f64).atan2(width as f64) / std::f64::consts::PI * 180.0;
        // Java: return rotation;
        rotation
    }

    // Java: static Arrow getBeginArrow(LineProperty stroke)
    pub fn getBeginArrow(stroke: Option<&LineProperty>) -> Element {
        // Java: return hasBeginArrow(stroke) ? stroke.getStartArrow() : null;
        if Self::hasBeginArrow(stroke) {
            stroke.unwrap().getStartArrow()
        } else {
            None
        }
    }

    // Java: static Arrow getEndArrow(LineProperty stroke)
    pub fn getEndArrow(stroke: Option<&LineProperty>) -> Element {
        // Java: return hasEndArrow(stroke) ? stroke.getEndArrow() : null;
        if Self::hasEndArrow(stroke) {
            stroke.unwrap().getEndArrow()
        } else {
            None
        }
    }

    // Java: static boolean hasBeginArrow(LineProperty stroke)
    pub fn hasBeginArrow(stroke: Option<&LineProperty>) -> bool {
        // Java: return stroke != null && stroke.getStartArrow() != null && stroke.getStartArrow().getHead() != 0;
        if let Some(stroke) = stroke {
            let arrow_elem = stroke.getStartArrow();
            if let Some(ref arrow_arc) = arrow_elem {
                if let Some(arrow) = arrow_arc.downcast_ref::<Arrow>() {
                    return arrow.getHead() != 0;
                }
            }
        }
        false
    }

    // Java: static boolean hasEndArrow(LineProperty stroke)
    pub fn hasEndArrow(stroke: Option<&LineProperty>) -> bool {
        // Java: return stroke != null && stroke.getEndArrow() != null && stroke.getEndArrow().getHead() != 0;
        if let Some(stroke) = stroke {
            let arrow_elem = stroke.getEndArrow();
            if let Some(ref arrow_arc) = arrow_elem {
                if let Some(arrow) = arrow_arc.downcast_ref::<Arrow>() {
                    return arrow.getHead() != 0;
                }
            }
        }
        false
    }

    // Java: static int getEndArrowIndex(GraphicsPath path)
    pub fn getEndArrowIndex(path: &GraphicsPath) -> i32 {
        // Java: int size = path.size();
        let size = path.size();
        // Java: for(int i = size - 1; i >= 0; --i)
        let mut i = size - 1;
        while i >= 0 {
            // Java: PathElem elem = path.getElem(i);
            let elem = path.getElem(i);
            // Java: if (elem.elemType != 0 && elem.elemType != 5)
            if elem.elemType != 0 && elem.elemType != 5 {
                // Java: return i;
                return i;
            }
            i -= 1;
        }
        // Java: return -1;
        -1
    }

    // Java: static int getStartArrowIndex(GraphicsPath path)
    pub fn getStartArrowIndex(path: &GraphicsPath) -> i32 {
        // Java: int size = path.size();
        let size = path.size();
        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size {
            // Java: PathElem elem = path.getElem(i);
            let elem = path.getElem(i);
            // Java: if (elem.elemType != 0 && elem.elemType != 5)
            if elem.elemType != 0 && elem.elemType != 5 {
                // Java: return i;
                return i;
            }
        }
        // Java: return -1;
        -1
    }

    // Java: static float rotateX(float Ox, float Oy, float x, float y, float angle, boolean CW)
    pub fn rotateX(Ox: f32, Oy: f32, x: f32, y: f32, angle: f32, CW: bool) -> f32 {
        // Java: float z = (float)Math.sqrt((double)((x - Ox) * (x - Ox) + (y - Oy) * (y - Oy)));
        let z = (((x - Ox) * (x - Ox) + (y - Oy) * (y - Oy)) as f64).sqrt() as f32;
        // Java: float P_angle = (float)((double)((float)Math.acos((double)((x - Ox) / z)) * 180.0F) / Math.PI);
        let P_angle = (((((x - Ox) / z) as f64).acos() * 180.0) / std::f64::consts::PI) as f32;
        // Java: if (CW)
        if CW {
            // Java: return y < Oy ? (float)((double)Ox + (double)z * Math.cos((double)(360.0F - P_angle + angle) * Math.PI / (double)180.0F)) : (float)((double)Ox + (double)z * Math.cos((double)(P_angle + angle) * Math.PI / (double)180.0F));
            if y < Oy {
                (Ox as f64 + z as f64 * ((360.0 - P_angle as f64 + angle as f64) * std::f64::consts::PI / 180.0).cos()) as f32
            } else {
                (Ox as f64 + z as f64 * ((P_angle as f64 + angle as f64) * std::f64::consts::PI / 180.0).cos()) as f32
            }
        // Java: else
        } else {
            // Java: return y < Oy ? (float)((double)Ox + (double)z * Math.cos((double)(360.0F - P_angle - angle) * Math.PI / (double)180.0F)) : (float)((double)Ox + (double)z * Math.cos((double)(P_angle - angle) * Math.PI / (double)180.0F));
            if y < Oy {
                (Ox as f64 + z as f64 * ((360.0 - P_angle as f64 - angle as f64) * std::f64::consts::PI / 180.0).cos()) as f32
            } else {
                (Ox as f64 + z as f64 * ((P_angle as f64 - angle as f64) * std::f64::consts::PI / 180.0).cos()) as f32
            }
        }
    }

    // Java: static float rotateY(float Ox, float Oy, float x, float y, float angle, boolean CW)
    pub fn rotateY(Ox: f32, Oy: f32, x: f32, y: f32, angle: f32, CW: bool) -> f32 {
        // Java: float z = (float)Math.sqrt((double)((x - Ox) * (x - Ox) + (y - Oy) * (y - Oy)));
        let z = (((x - Ox) * (x - Ox) + (y - Oy) * (y - Oy)) as f64).sqrt() as f32;
        // Java: float P_angle = (float)((double)((float)Math.asin((double)((y - Oy) / z)) * 180.0F) / Math.PI);
        let P_angle = (((((y - Oy) / z) as f64).asin() * 180.0) / std::f64::consts::PI) as f32;
        // Java: if (CW)
        if CW {
            // Java: return x < Ox ? (float)((double)Oy + (double)z * Math.sin((double)(180.0F - P_angle + angle) * Math.PI / (double)180.0F)) : (float)((double)Oy + (double)z * Math.sin((double)(P_angle + angle) * Math.PI / (double)180.0F));
            if x < Ox {
                (Oy as f64 + z as f64 * ((180.0 - P_angle as f64 + angle as f64) * std::f64::consts::PI / 180.0).sin()) as f32
            } else {
                (Oy as f64 + z as f64 * ((P_angle as f64 + angle as f64) * std::f64::consts::PI / 180.0).sin()) as f32
            }
        // Java: else
        } else {
            // Java: return x < Ox ? (float)((double)Oy + (double)z * Math.sin((double)(180.0F - P_angle - angle) * Math.PI / (double)180.0F)) : (float)((double)Oy + (double)z * Math.sin((double)(P_angle - angle) * Math.PI / (double)180.0F));
            if x < Ox {
                (Oy as f64 + z as f64 * ((180.0 - P_angle as f64 - angle as f64) * std::f64::consts::PI / 180.0).sin()) as f32
            } else {
                (Oy as f64 + z as f64 * ((P_angle as f64 - angle as f64) * std::f64::consts::PI / 180.0).sin()) as f32
            }
        }
    }

    // Java: static GradStopList getNoFocusGrad(GradFill gradFill)
    pub fn getNoFocusGrad(gradFill: Option<&GradFill>) -> Option<GradStopList> {
        // Java: if (gradFill == null)
        if gradFill.is_none() {
            // Java: return null;
            return None;
        // Java: else
        } else {
            let gradFill = gradFill.unwrap();
            // Java: GradStopList gradStopList = gradFill.getGradStopList();
            let gradStopList = gradFill.getGradStopList();
            // Java: if (gradStopList == null)
            if gradStopList.is_none() {
                // Java: return null;
                return None;
            // Java: else
            } else {
                // LLM Rust Type-Cast: downcast Element to GradStopList
                let gradStopList = gradStopList.as_ref().unwrap().downcast_ref::<GradStopList>().unwrap();
                // Java: int angle = ((int)gradFill.getAngle() + 360) % 360;
                let angle = ((gradFill.getAngle() as i32) + 360) % 360;
                // Java: boolean bRevert = angle % 180 != 0 && gradFill.getFillType() != 6;
                let bRevert = angle % 180 != 0 && gradFill.base.getFillType() != 6;
                // Java: float focus = gradFill.getFocus();
                let focus = gradFill.getFocus();
                // Java: if (focus != -50.0F && focus != 50.0F)
                if focus != -50.0 && focus != 50.0 {
                    // Java: if (focus != 100.0F && focus != 0.0F)
                    if focus != 100.0 && focus != 0.0 {
                        // Java: return gradStopList;
                        return Some(gradStopList.clone());
                    // Java: else
                    } else {
                        // Java: GradStopList newGS = new GradStopList();
                        let mut newGS = GradStopList::new();
                        // Java: int count = gradStopList.size();
                        let count = gradStopList.size();
                        // Java: if (focus == 100.0F && !bRevert || focus == 0.0F && bRevert)
                        if focus == 100.0 && !bRevert || focus == 0.0 && bRevert {
                            // Java: for(int i = 0; i < count; ++i)
                            for i in 0..count {
                                // Java: GradStop gradStop = gradStopList.at(i);
                                let gradStop = gradStopList.at(i).unwrap();
                                // Java: newGS.add(new GradStop(gradStop.getPostion(), gradStop.getColor()));
                                // LLM No Rust equivalent: GradStopList uses simplified GradStop without color
                                newGS.add(crate::cn::wps::moffice::drawing::graphics::GradStop::GradStop::new_with(gradStop.getPostion(), gradStop.getColor()));
                            }
                        // Java: else
                        } else {
                            // Java: for(int i = 0; i < count; ++i)
                            for i in 0..count {
                                // Java: GradStop gradStop = gradStopList.at(count - i - 1);
                                let gradStop = gradStopList.at(count - i - 1).unwrap();
                                // Java: newGS.add(new GradStop(1.0F - gradStop.getPostion(), gradStop.getColor()));
                                // LLM No Rust equivalent: GradStopList uses simplified GradStop without color
                                newGS.add(crate::cn::wps::moffice::drawing::graphics::GradStop::GradStop::new_with(1.0 - gradStop.getPostion(), gradStop.getColor()));
                            }
                        }
                        // Java: return newGS;
                        return Some(newGS);
                    }
                // Java: else
                } else {
                    // Java: GradStopList newGS = new GradStopList();
                    let mut newGS = GradStopList::new();
                    // Java: int size = gradStopList.size();
                    let size = gradStopList.size();
                    // Java: if (focus == 50.0F && !bRevert || focus == -50.0F && bRevert)
                    if focus == 50.0 && !bRevert || focus == -50.0 && bRevert {
                        // Java: for(int i = 0; i < size; ++i)
                        for i in 0..size {
                            // Java: GradStop gradStop = gradStopList.at(i);
                            let gradStop = gradStopList.at(i).unwrap();
                            // Java: newGS.add(new GradStop(gradStop.getPostion() / 2.0F, gradStop.getColor()));
                            newGS.add(crate::cn::wps::moffice::drawing::graphics::GradStop::GradStop::new_with(gradStop.getPostion() / 2.0, gradStop.getColor()));
                        }
                        // Java: for(int i = 1; i < size; ++i)
                        for i in 1..size {
                            // Java: GradStop gradStop = gradStopList.at(size - i - 1);
                            let gradStop = gradStopList.at(size - i - 1).unwrap();
                            // Java: newGS.add(new GradStop(0.5F + (1.0F - gradStop.getPostion()) / 2.0F, gradStop.getColor()));
                            newGS.add(crate::cn::wps::moffice::drawing::graphics::GradStop::GradStop::new_with(0.5 + (1.0 - gradStop.getPostion()) / 2.0, gradStop.getColor()));
                        }
                    // Java: else
                    } else {
                        // Java: for(int i = 0; i < size; ++i)
                        for i in 0..size {
                            // Java: GradStop gradStop = gradStopList.at(size - i - 1);
                            let gradStop = gradStopList.at(size - i - 1).unwrap();
                            // Java: newGS.add(new GradStop((1.0F - gradStop.getPostion()) / 2.0F, gradStop.getColor()));
                            newGS.add(crate::cn::wps::moffice::drawing::graphics::GradStop::GradStop::new_with((1.0 - gradStop.getPostion()) / 2.0, gradStop.getColor()));
                        }
                        // Java: for(int i = 1; i < size; ++i)
                        for i in 1..size {
                            // Java: GradStop gradStop = gradStopList.at(i);
                            let gradStop = gradStopList.at(i).unwrap();
                            // Java: newGS.add(new GradStop(gradStop.getPostion() / 2.0F + 0.5F, gradStop.getColor()));
                            newGS.add(crate::cn::wps::moffice::drawing::graphics::GradStop::GradStop::new_with(gradStop.getPostion() / 2.0 + 0.5, gradStop.getColor()));
                        }
                    }
                    // Java: return newGS;
                    return Some(newGS);
                }
            }
        }
    }

    // Java: static GradFill getFocusGrad(GradFill gradFill)
    pub fn getFocusGrad(gradFill: Option<&GradFill>) -> Option<GradFill> {
        // Java: if (gradFill == null)
        if gradFill.is_none() {
            // Java: return null;
            return None;
        // Java: else
        } else {
            let gradFill = gradFill.unwrap();
            // Java: float focus = gradFill.getFocus();
            let focus = gradFill.getFocus();
            // Java: if (focus != 100.0F)
            if focus != 100.0 {
                // Java: return null;
                return None;
            // Java: else
            } else {
                // Java: GradStopList gradStopList = gradFill.getGradStopList();
                let gradStopList = gradFill.getGradStopList();
                // Java: if (gradStopList == null)
                if gradStopList.is_none() {
                    // Java: return null;
                    return None;
                // Java: else
                } else {
                    // LLM Rust Type-Cast: downcast Element to GradStopList
                    let gradStopList = gradStopList.as_ref().unwrap().downcast_ref::<GradStopList>().unwrap();
                    // Java: int size = gradStopList.size();
                    let size = gradStopList.size();
                    // Java: if (size >= 3 && size % 2 != 0)
                    if size >= 3 && size % 2 != 0 {
                        // Java: for(int i = 0; i < size / 2; ++i)
                        for i in 0..size / 2 {
                            // Java: GradStop gs1 = gradStopList.at(i);
                            let gs1 = gradStopList.at(i).unwrap();
                            // Java: GradStop gs2 = gradStopList.at(size - i - 1);
                            let gs2 = gradStopList.at(size - i - 1).unwrap();
                            // Java: if (gs1.getColor() != gs2.getColor() || Math.abs(gs1.getPostion() + gs2.getPostion() - 1.0F) > 1.0E-4F)
                            // LLM No Rust equivalent: GradStopList.GradStop has no color field; first condition stubbed
                            if false || (gs1.getPostion() + gs2.getPostion() - 1.0).abs() > 1.0e-4 {
                                // Java: return null;
                                return None;
                            }
                        }
                        // Java: GradFill newGradFill = null;
                        // LLM JavaLine Aggregation: merged into the next Rust statement
                        // Java: try
                        // LLM Rust Null-Safety: try/catch not needed, clone always succeeds in Rust
                        // Java: newGradFill = (GradFill)gradFill.clone();
                        let mut newGradFill: Option<GradFill> = Some(GradFill {
                            base: gradFill.base.clone_fill(),
                        });
                        // Java: catch (CloneNotSupportedException e)
                        // LLM Rust Null-Safety: catch block unreachable in Rust, clone always succeeds
                        // Java: e.printStackTrace();
                        // LLM Rust Null-Safety: handled by Option/Result, no explicit check needed
                        // Java: if (newGradFill == null)
                        if newGradFill.is_none() {
                            // Java: return null;
                            return None;
                        // Java: else
                        } else {
                        let mut newGradFill = newGradFill.unwrap();
                        // Java: GradStopList newGS = new GradStopList();
                        let mut newGS = GradStopList::new();
                        // Java: size = size / 2 + 1;
                        let half_size = size / 2 + 1;
                        // Java: for(int i = 0; i < size; ++i)
                        for i in 0..half_size {
                            // Java: GradStop gradStop = gradStopList.at(i);
                            let gradStop = gradStopList.at(i).unwrap();
                            // Java: newGS.add(new GradStop(gradStop.getPostion() * 2.0F, gradStop.getColor()));
                            newGS.add(crate::cn::wps::moffice::drawing::graphics::GradStop::GradStop::new_with(gradStop.getPostion() * 2.0, gradStop.getColor()));
                        }
                        // Java: newGradFill.setGradStopList(newGS);
                        newGradFill.setGradStopList(Some(Arc::new(newGS)));
                        // Java: newGradFill.setFocus(50.0F);
                        newGradFill.setFocus(50.0);
                        // Java: return newGradFill;
                        Some(newGradFill)
                        }
                    // Java: else
                    } else {
                        // Java: return null;
                        None
                    }
                }
            }
        }
    }

    // Java: static boolean needFillGeometry(int shapeType)
    pub fn needFillGeometry(shapeType: i32) -> bool {
        // Java: return isDml2DocSpecial(shapeType) || shapeType == 224 || shapeType == 226 || shapeType == 229 || shapeType == 234 || 240 <= shapeType && shapeType <= 245 || 247 == shapeType || 250 <= shapeType && shapeType <= 256 || 258 <= shapeType && shapeType <= 261 || shapeType == 263 || shapeType == 76 || shapeType == 83;
        let shapeType = shapeType; Self::isDml2DocSpecial(shapeType)
            || shapeType == 224 || shapeType == 226 || shapeType == 229 || shapeType == 234
            || (240 <= shapeType && shapeType <= 245)
            || 247 == shapeType
            || (250 <= shapeType && shapeType <= 256)
            || (258 <= shapeType && shapeType <= 261)
            || shapeType == 263 || shapeType == 76 || shapeType == 83
    }

    // Java: static boolean needCompatibility(int shapeType)
    pub fn needCompatibility(shapeType: i32) -> bool {
        // Java: return shapeType == 76;
        let shapeType = shapeType; shapeType == 76
    }

    // Java: static boolean needFillGeometry4Vml(int shapeType)
    pub fn needFillGeometry4Vml(shapeType: i32) -> bool {
        // Java: return isDml2DocSpecial4Vml(shapeType) || shapeType == 224 || shapeType == 226 || shapeType == 229 || shapeType == 234 || 240 <= shapeType && shapeType <= 245 || 247 == shapeType || 250 <= shapeType && shapeType <= 256 || 258 <= shapeType && shapeType <= 261 || shapeType == 263 || shapeType == 76 || shapeType == 83;
        let shapeType = shapeType; Self::isDml2DocSpecial4Vml(shapeType)
            || shapeType == 224 || shapeType == 226 || shapeType == 229 || shapeType == 234
            || (240 <= shapeType && shapeType <= 245)
            || 247 == shapeType
            || (250 <= shapeType && shapeType <= 256)
            || (258 <= shapeType && shapeType <= 261)
            || shapeType == 263 || shapeType == 76 || shapeType == 83
    }

    // Java: static boolean isDml2DocSpecial(int shapeType)
    pub fn isDml2DocSpecial(shapeType: i32) -> bool {
        // Java: return shapeType == 8 || shapeType == 9 || shapeType == 12 || shapeType == 182 || shapeType == 91 || shapeType == 90 || shapeType == 101 || shapeType == 93 || shapeType == 95 || shapeType == 19 || shapeType == 99 || shapeType == 75 || shapeType == 74;
        let shapeType = shapeType; shapeType == 8 || shapeType == 9 || shapeType == 12 || shapeType == 182
            || shapeType == 91 || shapeType == 90 || shapeType == 101
            || shapeType == 93 || shapeType == 95 || shapeType == 19
            || shapeType == 99 || shapeType == 75 || shapeType == 74
    }

    // Java: static boolean isDml2DocSpecial4Vml(int shapeType)
    fn isDml2DocSpecial4Vml(shapeType: i32) -> bool {
        // Java: return shapeType == 182 || shapeType == 91 || shapeType == 90 || shapeType == 101 || shapeType == 93 || shapeType == 75;
        let shapeType = shapeType; shapeType == 182 || shapeType == 91 || shapeType == 90
            || shapeType == 101 || shapeType == 93 || shapeType == 75
    }
}
