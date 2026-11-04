// Java: import android.graphics.Bitmap
#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut, unreachable_code)]
use std::rc::Rc;
use stubs::android::graphics::Bitmap::Bitmap;
// Java: import android.graphics.Canvas
use stubs::android::graphics::Canvas::Canvas;
// Java: import android.graphics.Matrix
use stubs::android::graphics::Matrix::Matrix;
// Java: import android.graphics.Path
use stubs::android::graphics::Path::Path;
// Java: import android.graphics.Point
use stubs::android::graphics::Point::Point;
// Java: import android.graphics.PointF
use stubs::android::graphics::PointF::PointF;
// Java: import android.graphics.RectF
use util::cn::wps::graphics::RectF::RectF;
// Java: import android.graphics.Bitmap.Config
use super::layout_shape::Config;
// Java: import android.graphics.Path.Direction
use stubs::android::graphics::Path::Path as Direction;
// Java: import cn.wps.base.converter.UnitsConverter
use crate::cn::wps::base::converter::UnitsConverter::UnitsConverter;
// Java: import cn.wps.drawing.app.graphic.GraphicObject
use super::layout_shape::GraphicObject;
// Java: import cn.wps.drawing.app.table.KmoTable
use super::layout_shape::KmoTable;
// Java: import cn.wps.drawing.app.text.KmoTextBody
use super::layout_shape::KmoTextBody;
// Java: import cn.wps.drawing.data.basic.GroupTransform2D
use super::layout_shape::GroupTransform2D;
// Java: import cn.wps.drawing.data.basic.Point2D
use super::layout_shape::Point2D;
// Java: import cn.wps.drawing.data.basic.PositiveSize2D
use super::layout_shape::PositiveSize2D;
// Java: import cn.wps.drawing.data.basic.Transform2D
use super::layout_shape::Transform2D;
// Java: import cn.wps.drawing.data.geometry.AdjAngle
use super::layout_shape::AdjAngle;
// Java: import cn.wps.drawing.data.geometry.AdjCoordinate
use super::layout_shape::AdjCoordinate;
// Java: import cn.wps.drawing.data.geometry.CustomGeometry
use super::layout_shape::CustomGeometry;
// Java: import cn.wps.drawing.data.geometry.GeomGuide
use super::layout_shape::GeomGuide;
// Java: import cn.wps.drawing.data.geometry.Geometry2D
use super::layout_shape::Geometry2D;
// Java: import cn.wps.drawing.data.geometry.PresetGeometry
use super::layout_shape::PresetGeometry;
// Java: import cn.wps.drawing.data.shape.Shape
use super::layout_shape::Shape;
// Java: import cn.wps.drawing.data.shape.ShapePr
use super::layout_shape::ShapePr;
// Java: import cn.wps.drawing.data.text.TextBodyProperties
use super::layout_shape::TextBodyProperties;
// Java: import cn.wps.moffice.drawing.graphics.BlipFill
use super::layout_shape::BlipFill;
// Java: import cn.wps.moffice.drawing.media.IMediaLib
use super::layout_shape::IMediaLib;
// Java: import cn.wps.moffice.drawing.media.MediaTypeEnum
use super::layout_shape::MediaTypeEnum;
// Java: import cn.wps.moffice.drawing.smartshape.CDL
use super::layout_shape::CDL;
// Java: import cn.wps.moffice.drawing.smartshape.CFL
use super::layout_shape::CFL;
// Java: import cn.wps.moffice.drawing.smartshape.GeoAdjust
use super::layout_shape::GeoAdjust;
// Java: import cn.wps.moffice.drawing.smartshape.PreDefData
use super::layout_shape::PreDefData;
// Java: import cn.wps.moffice.drawing.smartshape.ShapeType
use super::layout_shape::ShapeType;
// Java: import cn.wps.show.app.KmoPresentation
use super::layout_shape::KmoPresentation;
// Java: import cn.wps.show.app.media.KmoMediaUtil
use super::layout_shape::KmoMediaUtil;
// Java: import cn.wps.show.app.shape.KmoShowShape
use super::layout_shape::KmoShowShape;
// Java: import cn.wps.show.app.slide.ISlide
use super::layout_shape::ISlide;
// Java: import cn.wps.show.app.slide.KmoSlide
use super::layout_shape::KmoSlide;
// Java: import cn.wps.show.convert.ConvertTool
use super::layout_shape::ConvertTool;
// Java: import cn.wps.show.convert.GeomtryConvert
use super::layout_shape::GeomtryConvert;
// Java: import cn.wps.show.graphics.shape3d.shaderOpengl.GlCache
use super::layout_shape::GlCache;
// Java: import cn.wps.show.moffice.drawing.render.effectlayer.EffectLayerRender
use super::layout_shape::EffectLayerRender;
// Java: import cn.wps.show.moffice.drawing.render.shape3d.HitShape3DHelper
use super::layout_shape::HitShape3DHelper;
// Java: import cn.wps.show.radapter.interpreter.InterpretCtx
use super::layout_shape::InterpretCtx;
// Java: import cn.wps.show.render.ISlideRender
use super::layout_shape::ISlideRender;
// Java: import cn.wps.show.render.ShowTypoShape
use super::layout_shape::ShowTypoShape;
// Java: import cn.wps.show.render.SlideRender
use super::layout_shape::SlideRender;
// Java: import cn.wps.show.render.cache.ShapeInfo
use super::layout_shape::ShapeInfo;
// Java: import cn.wps.show.render.cache.ShapeInfoCache
use super::layout_shape::ShapeInfoCache;
// Java: import cn.wps.show.render.cache.SlideCache
use super::layout_shape::SlideCache;
// Java: import cn.wps.show.render.util.ShowShapeEnv
use super::layout_shape::ShowShapeEnv;
// Java: import cn.wps.show.render.util.SlideRenderTool
use super::layout_shape::SlideRenderTool;
// Java: import com.hp.hpl.inkml.Ink
use super::layout_shape::Ink;
// Java: import com.hp.hpl.inkml.impl.Brush
use super::layout_shape::Brush;
// Java: import com.hp.hpl.inkml.impl.TraceAdapter
use super::layout_shape::TraceAdapter;
// Java: import com.hp.hpl.inkml.impl.Brush.RasterOp
use super::layout_shape::Brush as RasterOp;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import cn.wps.show.util.FormulaParser
use super::layout_shape::FormulaParser;

// java public static final int MIN_LINE_WIDTH = 12700;
pub(crate) const MIN_LINE_WIDTH: i32 = 12700;

// java public class ShapeUtil
#[derive(Default)]
pub struct ShapeUtil {
}

// java public static enum RoundCornerCLipSizeType
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RoundCornerCLipSizeType {
    CLIP_SIZE_NONE,
    CLIP_SIZE_THUMB,
    CLIP_SIZE_NORMAL,
}

// java public class ShapeUtil
impl ShapeUtil {
    // java public static RectF calcShapeText2TopMatrix(KmoShowShape shape, Matrix matrix)
    pub fn calcShapeText2TopMatrix(shape: &KmoShowShape, matrix: Option<&Matrix>) -> RectF {
      // java return calcShape2TopMatrix(shape, matrix, true, true);
      return Self::calcShape2TopMatrix__kmoshowshape_matrix_boolean_boolean(shape, matrix, true, true);
    }

    // java public static RectF calcShape2TopMatrix(KmoShowShape shape, Matrix matrix)
    pub fn calcShape2TopMatrix__kmoshowshape_matrix(shape: &KmoShowShape, matrix: Option<&Matrix>) -> RectF {
      // java return calcShape2TopMatrix(shape, matrix, true, false);
      return Self::calcShape2TopMatrix__kmoshowshape_matrix_boolean_boolean(shape, matrix, true, false);
    }

    // java public static RectF calcShape2TopMatrix(KmoShowShape shape, Matrix matrix, boolean rotate, boolean isText)
    pub fn calcShape2TopMatrix__kmoshowshape_matrix_boolean_boolean(shape: &KmoShowShape, matrix: Option<&Matrix>, rotate: bool, isText: bool) -> RectF {
      // java List<KmoShowShape> shapeLink = new ArrayList();
      let mut shapeLink: Vec<Rc<KmoShowShape>> = Vec::new();
      // java KmoShowShape tmpShape = shape;
      let mut tmpShape: Rc<KmoShowShape> = Rc::new(shape.clone());
      // java shapeLink.add(shape);
      shapeLink.push(tmpShape.clone());
      // java while (null != tmpShape.parentGroup())
      while tmpShape.parentGroup().is_some() {
        // java tmpShape = tmpShape.parentGroup();
        tmpShape = tmpShape.parentGroup().unwrap();
        // java shapeLink.add(tmpShape);
        shapeLink.push(tmpShape.clone());
      }
      // java RectF rcShape = new RectF();
      let mut rcShape: RectF = RectF::new();
      // java RectF rcGroupShape = new RectF();
      let mut rcGroupShape: RectF = RectF::new();
      // java int len = shapeLink.size();
      let len: i32 = shapeLink.len() as i32;
      // java boolean isParentGroup = false;
      let mut isParentGroup: bool = false;
      let mut i: i32 = len - 1;
      // java for (int i = len - 1; i >= 0 --i)
      while i >= 0 {
        // java tmpShape = (KmoShowShape)shapeLink.get(i);
        let tmpShape = &shapeLink[i as usize];
        // java rcShape.left = (float)shapeInfo.transform2DX;
        rcShape.left = tmpShape.transform2DX as f32;
        // java rcShape.top = (float)shapeInfo.transform2DY;
        rcShape.top = tmpShape.transform2DY as f32;
        // java rcShape.right = rcShape.left + (float)shapeInfo.transform2DW;
        rcShape.right = rcShape.left + tmpShape.transform2DW as f32;
        // java rcShape.bottom = rcShape.top + (float)shapeInfo.transform2DH;
        rcShape.bottom = rcShape.top + tmpShape.transform2DH as f32;
        // java float rotation = 0.0F;
        let mut rotation: f32 = 0.0;
        // java boolean flipV = false;
        let mut flipV: bool = false;
        // java boolean flipH = false;
        let mut flipH: bool = false;
        // java if (rotate && shapeInfo.canRotate)
        if rotate && tmpShape.can_rotate {
          // java rotation = toDegree(shapeInfo.rotation);
          rotation = Self::toDegree(tmpShape.rotation);
        }
        // java if (rotate)
        if rotate {
          // java flipH = shapeInfo.isFlipHorz;
          flipH = tmpShape.isFlipHorz;
          // java flipV = shapeInfo.isFlipVert;
          flipV = tmpShape.isFlipVert;
          // java if (isText && (flipH && !flipV || flipV && !flipH))
          if isText && ((flipH && !flipV) || (flipV && !flipH)) {
            // java flipH = !flipH;
            flipH = !flipH;
          }
        }
        // java if (isParentGroup)
        if isParentGroup {
          // java calcChildRelativeRect(tmpShape.parentGroup(), rcGroupShape, rcShape, rotation);
          if let Some(pg) = tmpShape.parentGroup() {
            Self::calcChildRelativeRect(&pg, &rcGroupShape, &mut rcShape, rotation);
          }
        }
        // java if (null != matrix)
        if matrix.is_some() {
          // java calcShapeMatrix(matrix, rcShape, rotation, flipH, flipV);
          let _ = (flipH, flipV);
          // rust: calcShapeMatrix is DEFERRED_NON_FUNCTION (MVP matrix=None)
        }
        // java isParentGroup = tmpShape.type() == 0;
        isParentGroup = tmpShape.r#type() == 0;
        // java rcGroupShape.set(rcShape);
        rcGroupShape.set(rcShape.left, rcShape.top, rcShape.right, rcShape.bottom);
        i -= 1;
      }
      // java return rcShape;
      return rcShape;
    }

    // java public static void calcShapeMatrix(Matrix matrix, RectF rcShape, float rotation, boolean flipH, boolean flipV)
    pub fn calcShapeMatrix(matrix: &mut Matrix, rcShape: &RectF, rotation: f32, flipH: bool, flipV: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java matrix.preTranslate(rcShape.left, rcShape.top);
      matrix.preTranslate(rcShape.left, rcShape.top);
      // java float halfW = (rcShape.right - rcShape.left) / 2.0F;
      let mut halfW: f32 = rcShape.right - rcShape.left / 2;
      // java float halfH = (rcShape.bottom - rcShape.top) / 2.0F;
      let mut halfH: f32 = rcShape.bottom - rcShape.top / 2;
      // java if (rotation != 0.0F)
      if rotation != 0 {
        // java matrix.preTranslate(halfW, halfH);
        matrix.preTranslate(halfW, halfH);
        // java rotation %= 360.0F;
        rotation = rotation % 360;
        // java if (rotation < 0.0F)
        if rotation < 0 {
          // java rotation += 360.0F;
          rotation = rotation + 360;
        }
        // java matrix.preRotate(rotation);
        matrix.preRotate(rotation);
        // java matrix.preTranslate(-halfW, -halfH);
        matrix.preTranslate(-halfW, -halfH);
      }
      // java if (flipH || flipV)
      if flipH || flipV {
        // java matrix.preTranslate(halfW, halfH);
        matrix.preTranslate(halfW, halfH);
        // java matrix.preScale(flipH ? -1.0F : 1.0F, flipV ? -1.0F : 1.0F);
        matrix.preScale((if flipH { -1 } else { 1 }), (if flipV { -1 } else { 1 }));
        // java matrix.preTranslate(-halfW, -halfH);
        matrix.preTranslate(-halfW, -halfH);
      }
      */
    }

    // java public static void calcChildRelativeRect(KmoShowShape group, RectF rcGroup, RectF rcChild, float childRotation)
    pub fn calcChildRelativeRect(group: &KmoShowShape, rcGroup: &RectF, rcChild: &mut RectF, mut childRotation: f32) {
      // java if (groupTransform2d.hasChildPoint() && groupTransform2d.hasChildExtents())
      if group.hasChildPoint() && group.hasChildExtents() {
        // java Point2D childOffset = groupTransform2d.childPoint();
        let childOffset = group.childPoint();
        // java PositiveSize2D childSize = groupTransform2d.childExtents();
        let childSize = group.childExtents();
        // java rcChild.offset(-((float)childOffset.x()), -((float)childOffset.y()));
        rcChild.offset(-(childOffset.0 as f32), -(childOffset.1 as f32));
        // java int cx = childSize.cx();
        let cx: i32 = childSize.0;
        // java int cy = childSize.cy();
        let cy: i32 = childSize.1;
        // java float scaleX = cx == 0 ? 1.0F : rcGroup.width() / (float)cx;
        let scaleX: f32 = if cx == 0 { 1.0 } else { rcGroup.width() / (cx as f32) };
        // java float scaleY = cy == 0 ? 1.0F : rcGroup.height() / (float)cy;
        let scaleY: f32 = if cy == 0 { 1.0 } else { rcGroup.height() / (cy as f32) };
        // java float width = rcChild.width();
        let mut width: f32 = rcChild.width();
        // java float height = rcChild.height();
        let mut height: f32 = rcChild.height();
        // java float centerX = rcChild.centerX() * scaleX;
        let centerX: f32 = rcChild.centerX() * scaleX;
        // java float centerY = rcChild.centerY() * scaleY;
        let centerY: f32 = rcChild.centerY() * scaleY;
        // java childRotation %= 360.0F;
        childRotation = childRotation % 360.0;
        // java if (childRotation < 0.0F)
        if childRotation < 0.0 {
          // java childRotation += 360.0F;
          childRotation = childRotation + 360.0;
        }
        // java if ((!(childRotation >= 45.0F) || !(childRotation < 135.0F)) && (!(childRotation >= 225.0F) || !(childRotation < 315.0F)))
        if !((childRotation >= 45.0 && childRotation < 135.0) || (childRotation >= 225.0 && childRotation < 315.0)) {
          // java width *= scaleX;
          width = width * scaleX;
          // java height *= scaleY;
          height = height * scaleY;
        }
        // java else
        else {
          // java width *= scaleY;
          width = width * scaleY;
          // java height *= scaleX;
          height = height * scaleX;
        }
        // java float left = centerX - width * 0.5F;
        let left: f32 = centerX - width * 0.5;
        // java float top = centerY - height * 0.5F;
        let top: f32 = centerY - height * 0.5;
        // java rcChild.set(left, top, left + width, top + height);
        rcChild.set(left, top, left + width, top + height);
      }
    }

    // java public static void calcRelativeRect(KmoShowShape shape, RectF rcShape)
    pub fn calcRelativeRect__kmoshowshape_rectf(shape: &KmoShowShape, rcShape: &mut RectF) {
      // java calcRelativeRect(shape, rcShape, true);
      Self::calcRelativeRect__kmoshowshape_rectf_boolean(shape, rcShape, true);
    }

    // java public static void calcRelativeRect(KmoShowShape shape, RectF rcShape, boolean calcSelf)
    pub fn calcRelativeRect__kmoshowshape_rectf_boolean(shape: &KmoShowShape, rcShape: &mut RectF, calcSelf: bool) {
      // java if (calcSelf || rcShape.isEmpty())
      if calcSelf || rcShape.isEmpty() {
        // java shapeInfoCache.transform2D(shape, rcShape);
        rcShape.left = shape.transform2DX as f32;
        rcShape.top = shape.transform2DY as f32;
        rcShape.right = rcShape.left + shape.transform2DW as f32;
        rcShape.bottom = rcShape.top + shape.transform2DH as f32;
      }
      // java KmoShowShape parent = shape.parentGroup();
      let parent = shape.parentGroup();
      // java if (parent != null)
      if let Some(parent) = parent {
        // java float rotation = 0.0F;
        let mut rotation: f32 = 0.0;
        // java if (shapeInfoCache.canRotate(shape))
        if shape.canRotate() {
          // java rotation = toDegree(shapeInfoCache.rotation(shape));
          rotation = Self::toDegree(shape.rotation());
        }
        // java RectF rcGroupShape = calcShape2TopMatrix(parent, (Matrix)null);
        let rcGroupShape: RectF = Self::calcShape2TopMatrix__kmoshowshape_matrix(&parent, None);
        // java calcChildRelativeRect(parent, rcGroupShape, rcShape, (float)((int)rotation));
        Self::calcChildRelativeRect(&parent, &rcGroupShape, rcShape, (rotation as i32) as f32);
      }
    }

    // java public static RectF calcRelativeTextRect(KmoShowShape shape)
    pub fn calcRelativeTextRect(shape: &KmoShowShape) -> Option<RectF> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == shape.getTextRect())
      if shape.getTextRect().is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java RectF txtRect = new RectF(shape.getTextRect());
        let mut txtRect: RectF = RectF::new(shape.getTextRect());
        // java RectF rcShape = new RectF();
        let mut rcShape: RectF = RectF::new();
        // java calcRelativeRect(shape, rcShape);
        Self::calcRelativeRect__kmoshowshape_rectf(shape, &mut rcShape);
        // java txtRect.offset(rcShape.left, rcShape.top);
        txtRect.offset(rcShape.left, rcShape.top);
        // java return txtRect;
        return Some(txtRect);
      }
      */
    }

    // java public static void calcShapeRect(KmoShowShape shape, RectF rcShape)
    pub fn calcShapeRect(shape: &KmoShowShape, rcShape: &mut RectF) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Matrix matrix = new Matrix();
      let mut matrix: Matrix = Matrix::new();
      // java RectF rectF = calcShape2TopMatrix(shape, matrix, false, false);
      let mut rectF: RectF = Self::calcShape2TopMatrix__kmoshowshape_matrix_boolean_boolean(shape, Some(&matrix), false, false);
      // java float[] pts = new float[2];
      let mut pts: Vec<f32> = vec![Default::default(); 2usize];
      // java pts[0] = 0.0F;
      pts[0usize] = 0;
      // java pts[1] = 0.0F;
      pts[1usize] = 0;
      // java matrix.mapPoints(pts);
      matrix.mapPoints(pts);
      // java float x = pts[0];
      let mut x: f32 = pts[0usize];
      // java float y = pts[1];
      let mut y: f32 = pts[1usize];
      // java float w = matrix.mapRadius(rectF.width());
      let mut w: f32 = matrix.mapRadius(rectF.width());
      // java float h = matrix.mapRadius(rectF.height());
      let mut h: f32 = matrix.mapRadius(rectF.height());
      // java rcShape.set(x, y, x + w, y + h);
      rcShape.set(x, y, x + w, y + h);
      */
    }

    // java public static RectF calcShapeTextRect(KmoShowShape shape)
    pub fn calcShapeTextRect__kmoshowshape(shape: &KmoShowShape) -> RectF {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java RectF shapeRect = new RectF();
      let mut shapeRect: RectF = RectF::new();
      // java calcShapeRect(shape, shapeRect);
      Self::calcShapeRect(&shape, &mut shapeRect);
      // java return calcShapeTextRect(shape, shapeRect.width(), shapeRect.height());
      return Self::calcShapeTextRect__kmoshowshape_float_float(&shape, shapeRect.width(), shapeRect.height());
      */
    }

    // java public static RectF calcShapeTextRect(KmoShowShape shape, float width, float height)
    pub fn calcShapeTextRect__kmoshowshape_float_float(shape: &mut KmoShowShape, width: f32, height: f32) -> Option<RectF> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java RectF rc = new RectF(0.0F, 0.0F, width, height);
      let mut rc: RectF = RectF::new(0, 0, width, height);
      // java int shapeType = shape.type();
      let mut shapeType: i32 = shape.r#type();
      // java if (shapeType == 1 && !shape.hasInk())
      if shapeType == 1 && !shape.hasInk() {
        // java ShapePr shapePr = shape.shapePr();
        let mut shapePr: ShapePr = shape.shapePr();
        // java if (shapePr == null)
        if false {
          // java return rc;
          return rc;
        }
        // java else
        else {
          // java Geometry2D geometry2D = shapePr.geom();
          let mut geometry2D: Geometry2D = shapePr.geom();
          // java if (geometry2D.hasPrstGeom())
          if geometry2D.hasPrstGeom() {
            // java PresetGeometry presetGeometry = geometry2D.prstGeom();
            let mut presetGeometry: PresetGeometry = geometry2D.prstGeom();
            // java int type = presetGeometry.prst();
            let mut r#type: i32 = presetGeometry.prst();
            // java String shapeName = GeomtryConvert.shapeTypeMap.get(type);
            let mut shapeName: String = GeomtryConvert::shapeTypeMap.get(r#type);
            // java if (shapeName.isEmpty())
            if shapeName.is_empty() {
              // java return rc;
              return rc;
            }
            // java else
            else {
              // java int drawingType = ShapeType.getShapeType(shapeName);
              let mut drawingType: i32 = ShapeType::getShapeType(shapeName);
              // java boolean isValidType = drawingType > 0 && drawingType < 264;
              let mut isValidType: bool = drawingType > 0 && drawingType < 264;
              // java if (!isValidType)
              if !isValidType {
                // java return rc;
                return rc;
              }
              // java else
              else {
                // java boolean calcTextRect = ShapeType.hasImplAutoFitRule(drawingType);
                let mut calcTextRect: bool = ShapeType::hasImplAutoFitRule(drawingType);
                // java if (!calcTextRect)
                if !calcTextRect {
                  // java KmoTextBody textBody = shape.getTextOperator().getTextBody();
                  let mut textBody: KmoTextBody = shape.getTextOperator().getTextBody();
                  // java if (textBody != null && textBody.hasTextBodyProperties())
                  if true && textBody.hasTextBodyProperties() {
                    // java TextBodyProperties textBodyProperties = textBody.textBodyProperties();
                    let mut textBodyProperties: TextBodyProperties = textBody.textBodyProperties();
                    // java if (textBodyProperties.hasTextAutofit())
                    if textBodyProperties.hasTextAutofit() {
                      // java int autoFitType = textBodyProperties.textAutofit().type();
                      let mut autoFitType: i32 = textBodyProperties.textAutofit().r#type();
                      // java if (autoFitType == 2)
                      if autoFitType == 2 {
                        // java return rc;
                        return rc;
                      }
                    }
                  }
                }
                // java PreDefData defData = CDL.getDefData(drawingType);
                let mut defData: PreDefData = CDL::getDefData(drawingType);
                // java if (defData.rect.length != 4)
                if (defData.rect.len() as i32) != 4 {
                  // java return rc;
                  return rc;
                }
                // java else
                else {
                  // java GeoAdjust geoAdjust = null;
                  let mut geoAdjust: GeoAdjust = Default::default();
                  // java if (presetGeometry.hasAvLst())
                  if presetGeometry.hasAvLst() {
                    // java PresetGeometry.AdjustValueList adjustValueList = presetGeometry.avLst();
                    let mut adjustValueList: PresetGeometry::AdjustValueList = presetGeometry.avLst();
                    // java List<GeomGuide> adjustValues = new ArrayList();
                    let mut adjustValues: Vec<GeomGuide> = { let __x: Vec<GeomGuide> = todo!("no align strategy matched"); __x };
                    // java adjustValueList.read(adjustValues);
                    adjustValueList.read(adjustValues);
                    // java FormulaParser parser = new FormulaParser();
                    let mut parser: FormulaParser = FormulaParser::new();
                    // java FormulaParser.FormulaParseResult result = parser.parseAdjust(adjustValues, width, height);
                    let mut result: FormulaParser::FormulaParseResult = parser.parseAdjust(adjustValues, width, height);
                    // java geoAdjust = result.mGeoAdjust;
                    geoAdjust = result.mGeoAdjust;
                  }
                  // java int adjCount = ShapeType.getAdjCount(drawingType);
                  let mut adjCount: i32 = ShapeType::getAdjCount(drawingType);
                  // java rc.left = getPresetValue(geoAdjust, adjCount, defData, defData.rect[0], width, height);
                  rc.left = Self::getPresetValue(&geoAdjust, adjCount, &defData, defData.rect[0usize], width, height);
                  // java rc.top = getPresetValue(geoAdjust, adjCount, defData, defData.rect[1], width, height);
                  rc.top = Self::getPresetValue(&geoAdjust, adjCount, &defData, defData.rect[1usize], width, height);
                  // java rc.right = getPresetValue(geoAdjust, adjCount, defData, defData.rect[2], width, height);
                  rc.right = Self::getPresetValue(&geoAdjust, adjCount, &defData, defData.rect[2usize], width, height);
                  // java rc.bottom = getPresetValue(geoAdjust, adjCount, defData, defData.rect[3], width, height);
                  rc.bottom = Self::getPresetValue(&geoAdjust, adjCount, &defData, defData.rect[3usize], width, height);
                  // java return rc;
                  return rc;
                }
              }
            }
          }
          // java else
          else {
            // java if (geometry2D.hasCustGeom())
            if geometry2D.hasCustGeom() {
              // java CustomGeometry customGeometry = geometry2D.custGeom();
              let mut customGeometry: CustomGeometry = geometry2D.custGeom();
              // java if (customGeometry.hasRect())
              if customGeometry.hasRect() {
                // java CustomGeometry.GeomRect rcTxt = customGeometry.rect();
                let mut rcTxt: CustomGeometry::GeomRect = customGeometry.rect();
                // java FormulaParser parser = null;
                let mut parser: FormulaParser = Default::default();
                // java if (customGeometry.hasAvLst())
                if customGeometry.hasAvLst() {
                  // java CustomGeometry.AdjustValueList adjustValueList = customGeometry.avLst();
                  let mut adjustValueList: CustomGeometry::AdjustValueList = customGeometry.avLst();
                  // java List<GeomGuide> adjustValues = new ArrayList();
                  let mut adjustValues: Vec<GeomGuide> = { let __x: Vec<GeomGuide> = todo!("no align strategy matched"); __x };
                  // java adjustValueList.read(adjustValues);
                  adjustValueList.read(adjustValues);
                  // java parser = new FormulaParser();
                  parser = FormulaParser::new();
                  // java parser.parseAdjustNonReturn(adjustValues, width, height);
                  parser.parseAdjustNonReturn(adjustValues, width, height);
                }
                // java if (customGeometry.hasGdLst())
                if customGeometry.hasGdLst() {
                  // java CustomGeometry.GeomGuideList guideList = customGeometry.gdLst();
                  let mut guideList: CustomGeometry::GeomGuideList = customGeometry.gdLst();
                  // java List<GeomGuide> gds = new ArrayList();
                  let mut gds: Vec<GeomGuide> = { let __x: Vec<GeomGuide> = todo!("no align strategy matched"); __x };
                  // java guideList.read(gds);
                  guideList.read(gds);
                  // java parser = parser == null ? new FormulaParser() : parser;
                  parser = (if false { Some(FormulaParser::new()) } else { Some(parser) });
                  // java parser.parseFormulaNonReturn(gds, width, height);
                  parser.parseFormulaNonReturn(gds, width, height);
                }
                // java rc.left = (float)getGdValue(rcTxt.l(), parser, width, height);
                rc.left = ((Self::getGdValue__adjcoordinate_formulaparser_float_float(&rcTxt.l(), &parser, width, height)) as f32);
                // java rc.top = (float)getGdValue(rcTxt.t(), parser, width, height);
                rc.top = ((Self::getGdValue__adjcoordinate_formulaparser_float_float(&rcTxt.t(), &parser, width, height)) as f32);
                // java rc.right = (float)getGdValue(rcTxt.r(), parser, width, height);
                rc.right = ((Self::getGdValue__adjcoordinate_formulaparser_float_float(&rcTxt.r(), &parser, width, height)) as f32);
                // java rc.bottom = (float)getGdValue(rcTxt.b(), parser, width, height);
                rc.bottom = ((Self::getGdValue__adjcoordinate_formulaparser_float_float(&rcTxt.b(), &parser, width, height)) as f32);
                // java return rc;
                return rc;
              }
            }
            // java return rc;
            return rc;
          }
        }
      }
      // java else
      else {
        // java return rc;
        return rc;
      }
      */
    }

    // java public static double getGdValue(AdjCoordinate coordinate, FormulaParser parser, float w, float h)
    pub fn getGdValue__adjcoordinate_formulaparser_float_float(coordinate: &mut AdjCoordinate, parser: Option<&FormulaParser>, w: f32, h: f32) -> Option<f64> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (coordinate.hasGeomGuideName())
      if coordinate.hasGeomGuideName() {
        // java String guide = coordinate.geomGuideName();
        let mut guide: String = coordinate.geomGuideName();
        // java return parser == null ? (double)FormulaParser.getBuildInVal(guide, w, h) : parser.getValueByName(guide);
        return (if parser.is_none() { Some(((FormulaParser::getBuildInVal(guide, w, h)) as f64)) } else { Some(parser.getValueByName(guide)) });
      }
      // java else
      else {
        // java return (double)coordinate.coordinate();
        return ((coordinate.coordinate()) as f64);
      }
      */
    }

    // java public static double getGdValue(AdjAngle adjAngle, FormulaParser parser, float w, float h)
    pub fn getGdValue__adjangle_formulaparser_float_float(adjAngle: &mut AdjAngle, parser: Option<&FormulaParser>, w: f32, h: f32) -> Option<f64> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (adjAngle.hasGeomGuideName())
      if adjAngle.hasGeomGuideName() {
        // java String guide = adjAngle.geomGuideName();
        let mut guide: String = adjAngle.geomGuideName();
        // java return parser == null ? (double)FormulaParser.getBuildInVal(guide, w, h) : parser.getValueByName(guide);
        return (if parser.is_none() { Some(((FormulaParser::getBuildInVal(guide, w, h)) as f64)) } else { Some(parser.getValueByName(guide)) });
      }
      // java else
      else {
        // java return (double)adjAngle.angle();
        return ((adjAngle.angle()) as f64);
      }
      */
    }

    // java private static float getPresetValue(GeoAdjust geoAdjust, int AdjCount, PreDefData preDefData, int pcArg, float w, float h)
    pub fn getPresetValue(geoAdjust: &GeoAdjust, AdjCount: i32, preDefData: &PreDefData, pcArg: i32, w: f32, h: f32) -> f32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (CFL.isFormulaIndex(pcArg))
      if CFL::isFormulaIndex(pcArg) {
        // java return FormulaParser.getGuildInValue(pcArg, w, h);
        return FormulaParser::getGuildInValue(pcArg, w, h);
      }
      // java else
      else {
        // java int avcGdIndex = CFL.getAvcGdIndex(pcArg);
        let mut avcGdIndex: i32 = CFL::getAvcGdIndex(pcArg);
        // java if (avcGdIndex < preDefData.avcLst.length)
        if avcGdIndex < (preDefData.avcLst.len() as i32) {
          // java return geoAdjust != null && avcGdIndex < AdjCount && geoAdjust.hasAdjust(avcGdIndex) ? geoAdjust.getAdjust(avcGdIndex).floatValue() : (float)preDefData.avcLst[avcGdIndex];
          return (if true && avcGdIndex < AdjCount && geoAdjust.hasAdjust(avcGdIndex) { geoAdjust.getAdjust__int(avcGdIndex).floatValue() } else { ((preDefData.avcLst[(avcGdIndex) as usize]) as f32) });
        }
        // java else
        else {
          // java avcGdIndex -= preDefData.avcLst.length;
          avcGdIndex = avcGdIndex - (preDefData.avcLst.len() as i32);
          // java int avcGdValue = preDefData.gdLst[avcGdIndex];
          let mut avcGdValue: i32 = preDefData.gdLst[(avcGdIndex) as usize];
          // java int formulaIndex = CFL.getFormulaIndex(avcGdValue);
          let mut formulaIndex: i32 = CFL::getFormulaIndex(avcGdValue);
          // java int argCnt = CFL.getArgCnt(avcGdValue);
          let mut argCnt: i32 = CFL::getArgCnt(avcGdValue);
          // java float[] args = new float[argCnt];
          let mut args: Vec<f32> = vec![Default::default(); (argCnt) as usize];
          let mut i: i32 = 0;
          // java for (int i = 0; i < argCnt ++i)
          while i < argCnt {
            // java args[i] = getPresetValue(geoAdjust, AdjCount, preDefData, preDefData.gdLst[avcGdIndex + i + 1], w, h);
            args[(i) as usize] = Self::getPresetValue(geoAdjust, AdjCount, preDefData, preDefData.gdLst[(avcGdIndex + i + 1) as usize], w, h);
          }
          // java Float value = CFL.getFormulasValue(formulaIndex, args);
          let mut value: f32 = CFL::getFormulasValue(formulaIndex, args);
          // java return value;
          return value;
        }
      }
      */
    }

    // java public static void calcShapeFlipEx(KmoShowShape shape, Point flip, boolean containSelf)
    pub fn calcShapeFlipEx(shape: &KmoShowShape, flip: &mut Point, containSelf: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean flipH = false;
      let mut flipH: bool = false;
      // java boolean flipV = false;
      let mut flipV: bool = false;
      // java ShapeInfoCache shapeInfoCache = (ShapeInfoCache)shape.getSlideCache().shapeInfoCache();
      let mut shapeInfoCache: ShapeInfoCache = (shape.getSlideCache().shapeInfoCache()).downcast_ref::<ShapeInfoCache>().unwrap();
      // java if (containSelf)
      if containSelf {
        // java flipH = shapeInfoCache.isFlipHorz(shape);
        flipH = shapeInfoCache.isFlipHorz(shape);
        // java flipV = shapeInfoCache.isFlipVert(shape);
        flipV = shapeInfoCache.isFlipVert(shape);
      }
      // java boolean v;
      let mut v: bool;
      let mut tmpShape: KmoShowShape = shape.clone();
      // java for (KmoShowShape tmpShape = shape; null != tmpShape.parentGroup() flipV = v ? !flipV : flipV)
      while tmpShape.parentGroup().is_some() {
        // java tmpShape = tmpShape.parentGroup();
        tmpShape = tmpShape.parentGroup();
        // java boolean h = shapeInfoCache.isFlipHorz(tmpShape);
        let mut h: bool = shapeInfoCache.isFlipHorz(tmpShape);
        // java v = shapeInfoCache.isFlipVert(tmpShape);
        v = shapeInfoCache.isFlipVert(tmpShape);
        // java flipH = h ? !flipH : flipH;
        flipH = (if h { !flipH } else { flipH });
      }
      // java flip.set(flipH ? -1 : 0, flipV ? -1 : 0);
      flip.set((if flipH { -1 } else { 0 }), (if flipV { -1 } else { 0 }));
      */
    }

    // java public static void calcShapeFlip(KmoShowShape shape, Point flip, boolean containSelf)
    pub fn calcShapeFlip(shape: &KmoShowShape, flip: &mut Point, containSelf: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java List<KmoShowShape> shapeLink = new ArrayList();
      let mut shapeLink: Vec<KmoShowShape> = { let __x: Vec<KmoShowShape> = todo!("no align strategy matched"); __x };
      // java if (containSelf)
      if containSelf {
        // java shapeLink.add(shape);
        shapeLink.add(shape);
      }
      // java KmoShowShape tmpShape = shape;
      let mut tmpShape: KmoShowShape = shape.clone();
      // java while (null != tmpShape.parentGroup())
      while tmpShape.parentGroup().is_some() {
        // java tmpShape = tmpShape.parentGroup();
        tmpShape = tmpShape.parentGroup();
        // java shapeLink.add(tmpShape);
        shapeLink.add(tmpShape);
      }
      // java int len = shapeLink.size();
      let mut len: i32 = (shapeLink.len() as i32);
      // java boolean flipH = false;
      let mut flipH: bool = false;
      // java boolean flipV = false;
      let mut flipV: bool = false;
      let mut i: i32 = len - 1;
      // java for (int i = len - 1; i >= 0 --i)
      while i >= 0 {
        // java tmpShape = (KmoShowShape)shapeLink.get(i);
        tmpShape = (shapeLink.get(i)).downcast_ref::<KmoShowShape>().unwrap();
        // java Transform2D transform2D = tmpShape.transform2D();
        let mut transform2D: Transform2D = tmpShape.transform2D();
        // java boolean h = transform2D.hasFlipHorizontal() ? transform2D.isFlipHorizontal() : false;
        let mut h: bool = (if transform2D.hasFlipHorizontal() { transform2D.isFlipHorizontal() } else { false });
        // java boolean v = transform2D.hasFlipVertical() ? transform2D.isFlipVertical() : false;
        let mut v: bool = (if transform2D.hasFlipVertical() { transform2D.isFlipVertical() } else { false });
        // java flipH = h ? !flipH : flipH;
        flipH = (if h { !flipH } else { flipH });
        // java flipV = v ? !flipV : flipV;
        flipV = (if v { !flipV } else { flipV });
      }
      // java flip.set(flipH ? -1 : 0, flipV ? -1 : 0);
      flip.set((if flipH { -1 } else { 0 }), (if flipV { -1 } else { 0 }));
      */
    }

    // java public static int calcShapeRotation(KmoShowShape shape, boolean containSelf)
    pub fn calcShapeRotation(shape: &KmoShowShape, containSelf: bool) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java List<KmoShowShape> shapeLink = new ArrayList();
      let mut shapeLink: Vec<KmoShowShape> = { let __x: Vec<KmoShowShape> = todo!("no align strategy matched"); __x };
      // java if (containSelf)
      if containSelf {
        // java shapeLink.add(shape);
        shapeLink.add(shape);
      }
      // java KmoShowShape tmpShape = shape;
      let mut tmpShape: KmoShowShape = shape.clone();
      // java while (null != tmpShape.parentGroup())
      while tmpShape.parentGroup().is_some() {
        // java tmpShape = tmpShape.parentGroup();
        tmpShape = tmpShape.parentGroup();
        // java shapeLink.add(tmpShape);
        shapeLink.add(tmpShape);
      }
      // java int len = shapeLink.size();
      let mut len: i32 = (shapeLink.len() as i32);
      // java int rotation = 0;
      let mut rotation: i32 = 0;
      let mut i: i32 = len - 1;
      // java for (int i = len - 1; i >= 0 --i)
      while i >= 0 {
        // java tmpShape = (KmoShowShape)shapeLink.get(i);
        tmpShape = (shapeLink.get(i)).downcast_ref::<KmoShowShape>().unwrap();
        // java if (tmpShape.canRotate())
        if tmpShape.canRotate() {
          // java Transform2D transform2D = tmpShape.transform2D();
          let mut transform2D: Transform2D = tmpShape.transform2D();
          // java rotation += (int)toDegree(transform2D.rotation());
          rotation = rotation + ((Self::toDegree(transform2D.rotation())) as i32);
        }
      }
      // java return rotation;
      return rotation;
      */
    }

    // java public static float toDegree(int rotation)
    pub fn toDegree(rotation: i32) -> f32 {
      // java return (float)rotation / 60000.0F;
      return ((rotation) as f32) / 60000.0;
    }

    // java public static int toRotation(float degree)
    pub fn toRotation(degree: f32) -> i32 {
      // java return (int)(degree * 60000.0F);
      return ((degree * 60000.0) as i32);
    }

    // java public static synchronized void extendShapeRect(KmoShowShape shape, RectF rcShape)
    pub fn extendShapeRect(shape: &KmoShowShape, rcShape: &mut RectF) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int borderWidth = (int)UnitsConverter.pointToEmu((float)shape.borderWidth());
      let mut borderWidth: i32 = ((UnitsConverter::pointToEmu(((shape.borderWidth()) as f32))) as i32);
      // java borderWidth = borderWidth < 101600 ? 101600 : borderWidth;
      borderWidth = (if borderWidth < 101600 { 101600 } else { borderWidth });
      // java int dbBorderWidth = borderWidth << 1;
      let mut dbBorderWidth: i32 = borderWidth << 1;
      // java int diff = (int)((float)dbBorderWidth - rcShape.width());
      let mut diff: i32 = ((((dbBorderWidth) as f32) - rcShape.width()) as i32);
      // java if (diff > 0)
      if diff > 0 {
        // java int half = diff >> 1;
        let mut half: i32 = diff >> 1;
        // java rcShape.left -= (float)half;
        rcShape.left = rcShape.left - ((half) as f32);
        // java rcShape.right += (float)half;
        rcShape.right = rcShape.right + ((half) as f32);
      }
      // java diff = (int)((float)dbBorderWidth - rcShape.height());
      diff = ((((dbBorderWidth) as f32) - rcShape.height()) as i32);
      // java if (diff > 0)
      if diff > 0 {
        // java int half = diff >> 1;
        let mut half: i32 = diff >> 1;
        // java rcShape.top -= (float)half;
        rcShape.top = rcShape.top - ((half) as f32);
        // java rcShape.bottom += (float)half;
        rcShape.bottom = rcShape.bottom + ((half) as f32);
      }
      */
    }

    // java public static KmoTable getTable(KmoShowShape drawable)
    pub fn getTable(drawable: &KmoShowShape) -> Option<KmoTable> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (drawable != null && drawable.type() == 4)
      if true && drawable.r#type() == 4 {
        // java GraphicObject graphic = drawable.graphic();
        let mut graphic: Box<dyn GraphicObject> = drawable.graphic();
        // java if (graphic != null && graphic.type() == 1)
        if true && graphic.r#type() == 1 {
          // java return (KmoTable)drawable.graphic();
          return Some((drawable.graphic()).downcast_ref::<KmoTable>().unwrap());
        }
      }
      // java return null;
      return None;
      */
    }

    // java public static Bitmap shapeSnapshot(KmoShowShape shape, Bitmap snap)
    pub fn shapeSnapshot__kmoshowshape_bitmap(shape: &mut KmoShowShape, snap: Option<&Bitmap>) -> Bitmap {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (snap == null)
      if snap.is_none() {
        // java return snap;
        return snap;
      }
      // java else
      else {
        // java InterpretCtx ctx = new InterpretCtx();
        let mut ctx: InterpretCtx = InterpretCtx::new();
        // java ctx.setSlide(shape.parent());
        ctx.setSlide(shape.parent());
        // java ShowTypoShape showShape = new ShowTypoShape(shape, ctx);
        let mut showShape: ShowTypoShape = ShowTypoShape::new(shape, ctx);
        // java ShowShapeEnv showShapeEnv = new ShowShapeEnv(shape.parent(), 15474692);
        let mut showShapeEnv: ShowShapeEnv = ShowShapeEnv::new(shape.parent(), 15474692);
        // java Canvas canvas = new Canvas();
        let mut canvas: Canvas = Canvas::new();
        // java canvas.setBitmap(snap);
        canvas.setBitmap(snap);
        // java cn.wps.graphics.RectF shapeRect = showShape.getShapeRect();
        let mut shapeRect: RectF = showShape.getShapeRect();
        // java float shapeWidth = shapeRect.width();
        let mut shapeWidth: f32 = shapeRect.width();
        // java float shapeHeight = shapeRect.height();
        let mut shapeHeight: f32 = shapeRect.height();
        // java int width = snap.getWidth();
        let mut width: i32 = snap.getWidth();
        // java int height = snap.getHeight();
        let mut height: i32 = snap.getHeight();
        // java float sx = (float)width / shapeWidth;
        let mut sx: f32 = ((width) as f32) / shapeWidth;
        // java float sy = (float)height / shapeHeight;
        let mut sy: f32 = ((height) as f32) / shapeHeight;
        // java canvas.save();
        canvas.save();
        // java canvas.scale(sx, sy);
        canvas.scale(sx, sy);
        // java canvas.translate(-shapeRect.left, -shapeRect.top);
        canvas.translate(-shapeRect.left, -shapeRect.top);
        // java showShapeEnv.attach(canvas);
        showShapeEnv.attach(canvas);
        // java SlideRenderTool renderTool = new SlideRenderTool();
        let mut renderTool: SlideRenderTool = SlideRenderTool::new();
        // java renderTool.drawShape(showShape, showShapeEnv);
        renderTool.drawShape(showShape, showShapeEnv);
        // java showShapeEnv.detach();
        showShapeEnv.detach();
        // java canvas.restore();
        canvas.restore();
        // java return snap;
        return snap;
      }
      */
    }

    // java public static Bitmap shapeSnapshot(KmoShowShape shape, RectF realRect, RectF rect, float scale)
    pub fn shapeSnapshot__kmoshowshape_rectf_rectf_float(shape: &mut KmoShowShape, realRect: &mut RectF, rect: &mut RectF, scale: f32) -> Option<Bitmap> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java InterpretCtx ctx = new InterpretCtx();
      let mut ctx: InterpretCtx = InterpretCtx::new();
      // java ctx.setSlide(shape.parent());
      ctx.setSlide(shape.parent());
      // java ShowTypoShape showShape = new ShowTypoShape(shape, ctx);
      let mut showShape: ShowTypoShape = ShowTypoShape::new(shape, ctx);
      // java ShowShapeEnv showShapeEnv = new ShowShapeEnv(shape.parent(), 15474692);
      let mut showShapeEnv: ShowShapeEnv = ShowShapeEnv::new(shape.parent(), 15474692);
      // java cn.wps.graphics.RectF shapeRect = showShape.getShapeRect();
      let mut shapeRect: RectF = showShape.getShapeRect();
      // java rect.set(shapeRect.left, shapeRect.top, shapeRect.right, shapeRect.bottom);
      rect.set(shapeRect.left, shapeRect.top, shapeRect.right, shapeRect.bottom);
      // java float shapeWidth = shapeRect.width();
      let mut shapeWidth: f32 = shapeRect.width();
      // java float shapeHeight = shapeRect.height();
      let mut shapeHeight: f32 = shapeRect.height();
      // java int width = (int)shapeWidth;
      let mut width: i32 = ((shapeWidth) as i32);
      // java int height = (int)shapeHeight;
      let mut height: i32 = ((shapeHeight) as i32);
      // java float sx = (float)width / shapeWidth;
      let mut sx: f32 = ((width) as f32) / shapeWidth;
      // java float sy = (float)height / shapeHeight;
      let mut sy: f32 = ((height) as f32) / shapeHeight;
      // java if (shape.hasInk())
      if shape.hasInk() {
        // java cn.wps.graphics.RectF effectRect = new cn.wps.graphics.RectF(shapeRect);
        let mut effectRect: RectF = RectF::new(shapeRect);
        // java effectRect = showShape.getShape().getNormalOutRect(effectRect);
        effectRect = showShape.getShape().getNormalOutRect(effectRect);
        // java realRect.set(effectRect.left, effectRect.top, effectRect.right, effectRect.bottom);
        realRect.set(effectRect.left, effectRect.top, effectRect.right, effectRect.bottom);
      }
      // java else
      else {
        // java RectF effectRect = EffectLayerRender.getBound(showShapeEnv, showShape);
        let mut effectRect: RectF = EffectLayerRender::getBound(showShapeEnv, showShape);
        // java realRect.set(effectRect);
        realRect.set(effectRect);
      }
      // java width = (int)(Math.ceil((double)realRect.width()) * (double)scale);
      width = ((Math::ceil(((realRect.width()) as f64)) * ((scale) as f64)) as i32);
      // java height = (int)(Math.ceil((double)realRect.height()) * (double)scale);
      height = ((Math::ceil(((realRect.height()) as f64)) * ((scale) as f64)) as i32);
      // java Bitmap bitmap;
      let mut bitmap: Bitmap = Box::new(()) as Bitmap;
      bitmap = None;
      // java for (bitmap = null bitmap == null height = (int)(Math.ceil((double)realRect.height()) * (double)scale))
      while false {
            // java bitmap = Bitmap.createBitmap(width, height, Config.ARGB_8888);
            bitmap = Bitmap::createBitmap(width, height, Config::ARGB_8888);
            // java OutOfMemoryError e
            todo!();
              // java e.printStackTrace();
              e.printStackTrace();
            // java Exception e
            todo!();
              // java e.printStackTrace();
              e.printStackTrace();
        // java if (bitmap != null || width < 100 && height < 100)
        if true || width < 100 && height < 100 {
          // java break;
          break;
        }
        // java scale *= 0.8F;
        scale = scale * 0.8;
        // java width = (int)(Math.ceil((double)realRect.width()) * (double)scale);
        width = ((Math::ceil(((realRect.width()) as f64)) * ((scale) as f64)) as i32);
      }
      // java if (bitmap == null)
      if false {
        // java return null;
        return None;
      }
      // java else
      else {
        // java Canvas canvas = new Canvas();
        let mut canvas: Canvas = Canvas::new();
        // java canvas.setBitmap(bitmap);
        canvas.setBitmap(bitmap);
        // java canvas.save();
        canvas.save();
        // java canvas.scale(sx * scale, sy * scale);
        canvas.scale(sx * scale, sy * scale);
        // java canvas.translate(-realRect.left, -realRect.top);
        canvas.translate(-realRect.left, -realRect.top);
        // java List<Integer> tranList = null;
        let mut tranList: Vec<i32> = Default::default();
        // java if (shape.hasInk())
        if shape.hasInk() {
          // java int rot = shape.rotation() / '\uea60';
          let mut rot: i32 = shape.rotation() / '\u{ea60}';
          // java if (rot != 0)
          if rot != 0 {
            // java canvas.rotate((float)(-rot), realRect.centerX(), realRect.centerY());
            canvas.rotate(((-rot) as f32), realRect.centerX(), realRect.centerY());
          }
          // java tranList = setInkTransparent(showShape.getRenderModel().getInk(), 160);
          tranList = Self::setInkTransparent__ink_int(&showShape.getRenderModel().getInk(), 160);
        }
        // java showShapeEnv.attach(canvas);
        showShapeEnv.attach(canvas);
        // java SlideRenderTool renderTool = new SlideRenderTool();
        let mut renderTool: SlideRenderTool = SlideRenderTool::new();
        // java renderTool.drawShape(showShape, showShapeEnv);
        renderTool.drawShape(showShape, showShapeEnv);
        // java showShapeEnv.detach();
        showShapeEnv.detach();
        // java canvas.restore();
        canvas.restore();
        // java if (tranList != null)
        if true {
          // java setInkTransparent(showShape.getRenderModel().getInk(), tranList);
          Self::setInkTransparent__ink_int(&showShape.getRenderModel().getInk(), { let __x: i32 = todo!("no align strategy matched"); __x });
        }
        // java return bitmap;
        return bitmap;
      }
      */
    }

    // java private static Bitmap textShapeSnapshotWithRelativeInfo(KmoShowShape shape, Bitmap snap, int drawFlags)
    pub fn textShapeSnapshotWithRelativeInfo__kmoshowshape_bitmap_int(shape: &mut KmoShowShape, snap: Option<&Bitmap>, drawFlags: i32) -> Bitmap {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (snap == null)
      if snap.is_none() {
        // java return snap;
        return snap;
      }
      // java else
      else {
        // java ISlide slide = shape.parent();
        let mut slide: Box<dyn ISlide> = shape.parent();
        // java KmoPresentation presentation = slide.getPresentation();
        let mut presentation: KmoPresentation = slide.getPresentation();
        // java int slideWidth = (int)UnitsConverter.render().fromEmuX((float)presentation.slideWidth());
        let mut slideWidth: i32 = ((UnitsConverter::render().fromEmuX(((presentation.slideWidth()) as f32))) as i32);
        // java int slideHeight = (int)UnitsConverter.render().fromEmuY((float)presentation.slideHeight());
        let mut slideHeight: i32 = ((UnitsConverter::render().fromEmuY(((presentation.slideHeight()) as f32))) as i32);
        // java int requestW = snap.getWidth();
        let mut requestW: i32 = snap.getWidth();
        // java int requestH = snap.getHeight();
        let mut requestH: i32 = snap.getHeight();
        // java InterpretCtx ctx = new InterpretCtx();
        let mut ctx: InterpretCtx = InterpretCtx::new();
        // java ctx.setSlide(slide);
        ctx.setSlide(slide);
        // java ShowTypoShape showShape = new ShowTypoShape(shape, ctx);
        let mut showShape: ShowTypoShape = ShowTypoShape::new(shape, ctx);
        // java ShowShapeEnv showShapeEnv = new ShowShapeEnv(shape.parent(), 15474692 | drawFlags);
        let mut showShapeEnv: ShowShapeEnv = ShowShapeEnv::new(shape.parent(), 15474692 | drawFlags);
        // java cn.wps.graphics.RectF shapeRect = showShape.getShapeRect();
        let mut shapeRect: RectF = showShape.getShapeRect();
        // java float shapeWidth = shapeRect.width();
        let mut shapeWidth: f32 = shapeRect.width();
        // java float shapeHeight = shapeRect.height();
        let mut shapeHeight: f32 = shapeRect.height();
        // java Canvas canvas = new Canvas();
        let mut canvas: Canvas = Canvas::new();
        // java canvas.setBitmap(snap);
        canvas.setBitmap(snap);
        // java canvas.save();
        canvas.save();
        // java adjustCanvas(canvas, (KmoSlide)null, slideWidth, slideHeight, requestW, requestH);
        Self::adjustCanvas(&mut canvas, Default::default(), slideWidth, slideHeight, requestW, requestH);
        // java canvas.translate(((float)slideWidth - shapeWidth) / 2.0F - shapeRect.left, ((float)slideHeight - shapeHeight) / 2.0F - shapeRect.top);
        canvas.translate(((slideWidth) as f32) - shapeWidth / 2 - shapeRect.left, ((slideHeight) as f32) - shapeHeight / 2 - shapeRect.top);
        // java showShapeEnv.attach(canvas);
        showShapeEnv.attach(canvas);
        // java SlideRenderTool renderTool = new SlideRenderTool();
        let mut renderTool: SlideRenderTool = SlideRenderTool::new();
        // java renderTool.drawShape(showShape, showShapeEnv);
        renderTool.drawShape(showShape, showShapeEnv);
        // java showShapeEnv.detach();
        showShapeEnv.detach();
        // java canvas.restore();
        canvas.restore();
        // java return snap;
        return snap;
      }
      */
    }

    // java private static Bitmap textShapeSnapshot(KmoShowShape shape, Bitmap snap, int drawFlags)
    pub fn textShapeSnapshot__kmoshowshape_bitmap_int(shape: &mut KmoShowShape, snap: Option<&Bitmap>, drawFlags: i32) -> Bitmap {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (snap == null)
      if snap.is_none() {
        // java return snap;
        return snap;
      }
      // java else
      else {
        // java RectF shapeRect = calcMinFullDrawShapeRect(shape);
        let mut shapeRect: RectF = Self::calcMinFullDrawShapeRect(&shape);
        // java RectF textRectF = calcRelativeTextRect(shape);
        let mut textRectF: RectF = Self::calcRelativeTextRect(&shape).unwrap();
        // java if (textRectF == null)
        if false {
          // java return snap;
          return snap;
        }
        // java else
        else {
          // java int originsW = (int)UnitsConverter.render().fromEmuX(Math.max(shapeRect.width(), textRectF.width()));
          let mut originsW: i32 = ((UnitsConverter::render().fromEmuX((shapeRect.width()).max(textRectF.width()))) as i32);
          // java int originsH = (int)UnitsConverter.render().fromEmuY(Math.max(shapeRect.height(), textRectF.height()));
          let mut originsH: i32 = ((UnitsConverter::render().fromEmuY((shapeRect.height()).max(textRectF.height()))) as i32);
          // java if (textRectF.height() > shapeRect.height())
          if textRectF.height() > shapeRect.height() {
            // java originsH += 10;
            originsH = originsH + 10;
          }
          // java else if (textRectF.width() > shapeRect.width())
          else if textRectF.width() > shapeRect.width() {
            // java originsW += 10;
            originsW = originsW + 10;
          }
          // java RectF shapeRectReal = new RectF();
          let mut shapeRectReal: RectF = RectF::new();
          // java calcRelativeRect(shape, shapeRectReal);
          Self::calcRelativeRect__kmoshowshape_rectf(&shape, &mut shapeRectReal);
          // java int shapeWidth = (int)UnitsConverter.render().fromEmuX(shapeRectReal.width());
          let mut shapeWidth: i32 = ((UnitsConverter::render().fromEmuX(shapeRectReal.width())) as i32);
          // java int shapeHeight = (int)UnitsConverter.render().fromEmuY(shapeRectReal.height());
          let mut shapeHeight: i32 = ((UnitsConverter::render().fromEmuY(shapeRectReal.height())) as i32);
          // java Canvas canvas = new Canvas();
          let mut canvas: Canvas = Canvas::new();
          // java canvas.setBitmap(snap);
          canvas.setBitmap(snap);
          // java float tx;
          let mut tx: f32;
          // java float dx;
          let mut dx: f32;
          // java if (textRectF.width() >= shapeRect.width())
          if textRectF.width() >= shapeRect.width() {
            // java tx = UnitsConverter.render().fromEmuX(textRectF.width());
            tx = UnitsConverter::render().fromEmuX(textRectF.width());
            // java dx = UnitsConverter.render().fromEmuX(textRectF.left);
            dx = UnitsConverter::render().fromEmuX(textRectF.left);
          }
          // java else
          else {
            // java tx = (float)shapeWidth;
            tx = ((shapeWidth) as f32);
            // java dx = UnitsConverter.render().fromEmuX(shapeRectReal.left);
            dx = UnitsConverter::render().fromEmuX(shapeRectReal.left);
          }
          // java float ty;
          let mut ty: f32;
          // java float dy;
          let mut dy: f32;
          // java if (textRectF.height() >= shapeRect.height())
          if textRectF.height() >= shapeRect.height() {
            // java ty = UnitsConverter.render().fromEmuY(textRectF.height());
            ty = UnitsConverter::render().fromEmuY(textRectF.height());
            // java dy = UnitsConverter.render().fromEmuY(textRectF.top);
            dy = UnitsConverter::render().fromEmuY(textRectF.top);
          }
          // java else
          else {
            // java ty = (float)shapeHeight;
            ty = ((shapeHeight) as f32);
            // java dy = UnitsConverter.render().fromEmuY(shapeRectReal.top);
            dy = UnitsConverter::render().fromEmuY(shapeRectReal.top);
          }
          // java ISlide slide = shape.parent();
          let mut slide: Box<dyn ISlide> = shape.parent();
          // java InterpretCtx ctx = new InterpretCtx();
          let mut ctx: InterpretCtx = InterpretCtx::new();
          // java ctx.setSlide(slide);
          ctx.setSlide(slide);
          // java ShowTypoShape showShape = new ShowTypoShape(shape, ctx);
          let mut showShape: ShowTypoShape = ShowTypoShape::new(shape, ctx);
          // java canvas.save();
          canvas.save();
          // java adjustCanvas(canvas, (KmoSlide)null, originsW, originsH, snap.getWidth(), snap.getHeight());
          Self::adjustCanvas(&mut canvas, Default::default(), originsW, originsH, snap.getWidth(), snap.getHeight());
          // java canvas.translate(((float)originsW - tx) / 2.0F - dx, ((float)originsH - ty) / 2.0F - dy);
          canvas.translate(((originsW) as f32) - tx / 2 - dx, ((originsH) as f32) - ty / 2 - dy);
          // java ShowShapeEnv showShapeEnv = new ShowShapeEnv(shape.parent(), 15474692 | drawFlags);
          let mut showShapeEnv: ShowShapeEnv = ShowShapeEnv::new(shape.parent(), 15474692 | drawFlags);
          // java showShapeEnv.attach(canvas);
          showShapeEnv.attach(canvas);
          // java SlideRenderTool renderTool = new SlideRenderTool();
          let mut renderTool: SlideRenderTool = SlideRenderTool::new();
          // java renderTool.drawShape(showShape, showShapeEnv);
          renderTool.drawShape(showShape, showShapeEnv);
          // java showShapeEnv.detach();
          showShapeEnv.detach();
          // java canvas.restore();
          canvas.restore();
          // java return snap;
          return snap;
        }
      }
      */
    }

    // java public static Bitmap textShapeSnapshotWithRelativeInfo(KmoShowShape shape)
    pub fn textShapeSnapshotWithRelativeInfo__kmoshowshape(shape: &mut KmoShowShape) -> Option<Bitmap> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ISlide slide = shape.parent();
      let mut slide: Box<dyn ISlide> = shape.parent();
      // java KmoPresentation presentation = slide.getPresentation();
      let mut presentation: KmoPresentation = slide.getPresentation();
      // java int slideWidth = (int)UnitsConverter.render().fromEmuX((float)presentation.slideWidth());
      let mut slideWidth: i32 = ((UnitsConverter::render().fromEmuX(((presentation.slideWidth()) as f32))) as i32);
      // java int slideHeight = (int)UnitsConverter.render().fromEmuY((float)presentation.slideHeight());
      let mut slideHeight: i32 = ((UnitsConverter::render().fromEmuY(((presentation.slideHeight()) as f32))) as i32);
      // java int originsW = slideWidth;
      let mut originsW: i32 = slideWidth;
      // java int originsH = slideHeight;
      let mut originsH: i32 = slideHeight;
      // java InterpretCtx ctx = new InterpretCtx();
      let mut ctx: InterpretCtx = InterpretCtx::new();
      // java ctx.setSlide(slide);
      ctx.setSlide(slide);
      // java ShowTypoShape showShape = new ShowTypoShape(shape, ctx);
      let mut showShape: ShowTypoShape = ShowTypoShape::new(shape, ctx);
      // java cn.wps.graphics.RectF shapeRect = showShape.getShapeRect();
      let mut shapeRect: RectF = showShape.getShapeRect();
      // java float shapeWidth = shapeRect.width();
      let mut shapeWidth: f32 = shapeRect.width();
      // java float shapeHeight = shapeRect.height();
      let mut shapeHeight: f32 = shapeRect.height();
      // java Bitmap bitmap = null;
      let mut bitmap: Bitmap = Default::default();
      // java float scale;
      let mut scale: f32;
      scale = 1;
      // java for (scale = 1.0F bitmap == null slideHeight = (int)((float)slideHeight * scale))
      while false {
            // java bitmap = Bitmap.createBitmap(slideWidth, slideHeight, Config.ARGB_8888);
            bitmap = Bitmap::createBitmap(slideWidth, slideHeight, Config::ARGB_8888);
            // java OutOfMemoryError e
            todo!();
              // java e.printStackTrace();
              e.printStackTrace();
            // java Exception e
            todo!();
              // java e.printStackTrace();
              e.printStackTrace();
        // java if (bitmap != null || slideWidth < 100 && slideHeight < 100)
        if true || slideWidth < 100 && slideHeight < 100 {
          // java break;
          break;
        }
        // java scale *= 0.8F;
        scale = scale * 0.8;
        // java slideWidth = (int)((float)slideWidth * scale);
        slideWidth = ((((slideWidth) as f32) * scale) as i32);
      }
      // java if (bitmap == null)
      if false {
        // java return null;
        return None;
      }
      // java else
      else {
        // java Canvas canvas = new Canvas();
        let mut canvas: Canvas = Canvas::new();
        // java canvas.setBitmap(bitmap);
        canvas.setBitmap(bitmap);
        // java canvas.save();
        canvas.save();
        // java canvas.scale(scale, scale);
        canvas.scale(scale, scale);
        // java canvas.translate(((float)originsW - shapeWidth) / 2.0F - shapeRect.left, ((float)originsH - shapeHeight) / 2.0F - shapeRect.top);
        canvas.translate(((originsW) as f32) - shapeWidth / 2 - shapeRect.left, ((originsH) as f32) - shapeHeight / 2 - shapeRect.top);
        // java ShowShapeEnv showShapeEnv = new ShowShapeEnv(shape.parent(), 15474820);
        let mut showShapeEnv: ShowShapeEnv = ShowShapeEnv::new(shape.parent(), 15474820);
        // java showShapeEnv.attach(canvas);
        showShapeEnv.attach(canvas);
        // java SlideRenderTool renderTool = new SlideRenderTool();
        let mut renderTool: SlideRenderTool = SlideRenderTool::new();
        // java renderTool.drawShape(showShape, showShapeEnv);
        renderTool.drawShape(showShape, showShapeEnv);
        // java showShapeEnv.detach();
        showShapeEnv.detach();
        // java canvas.restore();
        canvas.restore();
        // java return bitmap;
        return bitmap;
      }
      */
    }

    // java public static Bitmap textShapeSnapshot(KmoShowShape shape)
    pub fn textShapeSnapshot__kmoshowshape(shape: Option<&mut KmoShowShape>) -> Option<Bitmap> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (shape == null)
      if shape.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java RectF shapeRect = calcMinFullDrawShapeRect(shape);
        let mut shapeRect: RectF = Self::calcMinFullDrawShapeRect(&shape);
        // java if (shapeRect.width() != 0.0F && shapeRect.height() != 0.0F)
        if shapeRect.width() != 0 && shapeRect.height() != 0 {
          // java RectF textRectF = calcRelativeTextRect(shape);
          let mut textRectF: RectF = Self::calcRelativeTextRect(&shape).unwrap();
          // java if (textRectF == null)
          if false {
            // java return null;
            return None;
          }
          // java else
          else {
            // java RectF shapeRectReal = new RectF();
            let mut shapeRectReal: RectF = RectF::new();
            // java calcRelativeRect(shape, shapeRectReal);
            Self::calcRelativeRect__kmoshowshape_rectf(&shape, &mut shapeRectReal);
            // java int shapeWidth = (int)UnitsConverter.render().fromEmuX(shapeRectReal.width());
            let mut shapeWidth: i32 = ((UnitsConverter::render().fromEmuX(shapeRectReal.width())) as i32);
            // java int shapeHeight = (int)UnitsConverter.render().fromEmuY(shapeRectReal.height());
            let mut shapeHeight: i32 = ((UnitsConverter::render().fromEmuY(shapeRectReal.height())) as i32);
            // java int originsW = (int)UnitsConverter.render().fromEmuX(Math.max(shapeRect.width(), textRectF.width()));
            let mut originsW: i32 = ((UnitsConverter::render().fromEmuX((shapeRect.width()).max(textRectF.width()))) as i32);
            // java int originsH = (int)UnitsConverter.render().fromEmuY(Math.max(shapeRect.height(), textRectF.height()));
            let mut originsH: i32 = ((UnitsConverter::render().fromEmuY((shapeRect.height()).max(textRectF.height()))) as i32);
            // java if (textRectF.height() >= shapeRect.height())
            if textRectF.height() >= shapeRect.height() {
              // java originsH += 10;
              originsH = originsH + 10;
            }
            // java else if (textRectF.width() >= shapeRect.width())
            else if textRectF.width() >= shapeRect.width() {
              // java originsW += 10;
              originsW = originsW + 10;
            }
            // java Bitmap bitmap = null;
            let mut bitmap: Bitmap = Default::default();
                // java bitmap = Bitmap.createBitmap(originsW, originsH, Config.ARGB_8888);
                bitmap = Bitmap::createBitmap(originsW, originsH, Config::ARGB_8888);
                // java OutOfMemoryError e
                todo!();
                  // java e.printStackTrace();
                  e.printStackTrace();
                // java Exception e
                todo!();
                  // java e.printStackTrace();
                  e.printStackTrace();
            // java if (bitmap == null)
            if false {
              // java return null;
              return None;
            }
            // java else
            else {
              // java Canvas canvas = new Canvas();
              let mut canvas: Canvas = Canvas::new();
              // java canvas.setBitmap(bitmap);
              canvas.setBitmap(bitmap);
              // java float tx;
              let mut tx: f32;
              // java float dx;
              let mut dx: f32;
              // java if (textRectF.width() >= shapeRect.width())
              if textRectF.width() >= shapeRect.width() {
                // java tx = UnitsConverter.render().fromEmuX(textRectF.width());
                tx = UnitsConverter::render().fromEmuX(textRectF.width());
                // java dx = UnitsConverter.render().fromEmuX(textRectF.left);
                dx = UnitsConverter::render().fromEmuX(textRectF.left);
              }
              // java else
              else {
                // java tx = (float)shapeWidth;
                tx = ((shapeWidth) as f32);
                // java dx = UnitsConverter.render().fromEmuX(shapeRectReal.left);
                dx = UnitsConverter::render().fromEmuX(shapeRectReal.left);
              }
              // java float ty;
              let mut ty: f32;
              // java float dy;
              let mut dy: f32;
              // java if (textRectF.height() >= shapeRect.height())
              if textRectF.height() >= shapeRect.height() {
                // java ty = UnitsConverter.render().fromEmuY(textRectF.height());
                ty = UnitsConverter::render().fromEmuY(textRectF.height());
                // java dy = UnitsConverter.render().fromEmuY(textRectF.top);
                dy = UnitsConverter::render().fromEmuY(textRectF.top);
              }
              // java else
              else {
                // java ty = (float)shapeHeight;
                ty = ((shapeHeight) as f32);
                // java dy = UnitsConverter.render().fromEmuY(shapeRectReal.top);
                dy = UnitsConverter::render().fromEmuY(shapeRectReal.top);
              }
              // java ISlide slide = shape.parent();
              let mut slide: Box<dyn ISlide> = shape.parent();
              // java InterpretCtx ctx = new InterpretCtx();
              let mut ctx: InterpretCtx = InterpretCtx::new();
              // java ctx.setSlide(slide);
              ctx.setSlide(slide);
              // java ShowTypoShape showShape = new ShowTypoShape(shape, ctx);
              let mut showShape: ShowTypoShape = ShowTypoShape::new(shape, ctx);
              // java canvas.save();
              canvas.save();
              // java canvas.translate(((float)originsW - tx) / 2.0F - dx, ((float)originsH - ty) / 2.0F - dy);
              canvas.translate(((originsW) as f32) - tx / 2 - dx, ((originsH) as f32) - ty / 2 - dy);
              // java ShowShapeEnv showShapeEnv = new ShowShapeEnv(shape.parent(), 15474820);
              let mut showShapeEnv: ShowShapeEnv = ShowShapeEnv::new(shape.parent(), 15474820);
              // java showShapeEnv.attach(canvas);
              showShapeEnv.attach(canvas);
              // java SlideRenderTool renderTool = new SlideRenderTool();
              let mut renderTool: SlideRenderTool = SlideRenderTool::new();
              // java renderTool.drawShape(showShape, showShapeEnv);
              renderTool.drawShape(showShape, showShapeEnv);
              // java showShapeEnv.detach();
              showShapeEnv.detach();
              // java canvas.restore();
              canvas.restore();
              // java return bitmap;
              return bitmap;
            }
          }
        }
        // java else
        else {
          // java return null;
          return None;
        }
      }
      */
    }

    // java public static Bitmap textShapeSnapshotWithRelativeInfo(KmoShowShape sourceShape, Bitmap outBitmap)
    pub fn textShapeSnapshotWithRelativeInfo__kmoshowshape_bitmap(sourceShape: &KmoShowShape, outBitmap: &Bitmap) -> Bitmap {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return textShapeSnapshotWithRelativeInfo(sourceShape, outBitmap, 128);
      return Self::textShapeSnapshotWithRelativeInfo__kmoshowshape_bitmap_int(&sourceShape, Some(outBitmap), 128);
      */
    }

    // java public static Bitmap textShapeSnapshot(KmoShowShape sourceShape, Bitmap outBitmap)
    pub fn textShapeSnapshot__kmoshowshape_bitmap(sourceShape: &KmoShowShape, outBitmap: &Bitmap) -> Bitmap {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return textShapeSnapshot(sourceShape, outBitmap, 128);
      return Self::textShapeSnapshot__kmoshowshape_bitmap_int(&sourceShape, Some(outBitmap), 128);
      */
    }

    // java private static RectF calcMinFullDrawShapeRect(KmoShowShape shape)
    pub fn calcMinFullDrawShapeRect(shape: &KmoShowShape) -> RectF {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Matrix matrix = new Matrix();
      let mut matrix: Matrix = Matrix::new();
      // java RectF rectF = calcShape2TopMatrix(shape, matrix, true, true);
      let mut rectF: RectF = Self::calcShape2TopMatrix__kmoshowshape_matrix_boolean_boolean(shape, Some(&matrix), true, true);
      // java matrix.mapRect(rectF);
      matrix.mapRect(rectF);
      // java return rectF;
      return rectF;
      */
    }

    // java public static List<Integer> setInkTransparent(Ink ink, int tran)
    pub fn setInkTransparent__ink_int(ink: &Ink, tran: i32) -> Vec<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ArrayList<TraceAdapter> traces = ink.getTraces();
      let mut traces: Vec<TraceAdapter> = ink.getTraces();
      // java List<Integer> tranList = new ArrayList();
      let mut tranList: Vec<i32> = { let __x: Vec<i32> = todo!("no align strategy matched"); __x };
      // java int traceSize = traces.size();
      let mut traceSize: i32 = (traces.len() as i32);
      let mut traceIndex: i32 = 0;
      // java for (int traceIndex = 0; traceIndex < traceSize ++traceIndex)
      while traceIndex < traceSize {
        // java TraceAdapter trace = (TraceAdapter)traces.get(traceIndex);
        let mut trace: TraceAdapter = (traces.get(traceIndex)).downcast_ref::<TraceAdapter>().unwrap();
        // java Brush brush = trace.getBrush();
        let mut brush: Brush = trace.getBrush();
        // java int old = 255;
        let mut old: i32 = 255;
        // java if (brush != null)
        if true {
          // java int color = brush.getColor();
          let mut color: i32 = brush.getColor();
          // java old = color >> 24 & 255;
          old = color >> 24 & 255;
          // java Brush.RasterOp rasterOp = brush.getRasterOp();
          let mut rasterOp: Brush::RasterOp = brush.getRasterOp();
          // java if (rasterOp != RasterOp.copyPen)
          if rasterOp != RasterOp::copyPen {
            // java brush.setColor(tran << 24 | color & 16777215);
            brush.setColor(tran << 24 | color & 16777215);
          }
        }
        // java tranList.add(old);
        tranList.add(old);
      }
      // java return tranList;
      return tranList;
      */
    }

    // java public static void setInkTransparent(Ink ink, List<Integer> trans)
    pub fn setInkTransparent__ink_listinteger(ink: &Ink, trans: &Vec<i32>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ArrayList<TraceAdapter> traces = ink.getTraces();
      let mut traces: Vec<TraceAdapter> = ink.getTraces();
      // java int traceSize = traces.size();
      let mut traceSize: i32 = (traces.len() as i32);
      let mut traceIndex: i32 = 0;
      // java for (int traceIndex = 0; traceIndex < traceSize ++traceIndex)
      while traceIndex < traceSize {
        // java TraceAdapter trace = (TraceAdapter)traces.get(traceIndex);
        let mut trace: TraceAdapter = (traces.get(traceIndex)).downcast_ref::<TraceAdapter>().unwrap();
        // java Brush brush = trace.getBrush();
        let mut brush: Brush = trace.getBrush();
        // java if (brush != null)
        if true {
          // java int color = brush.getColor();
          let mut color: i32 = brush.getColor();
          // java brush.setColor((Integer)trans.get(traceIndex) << 24 | color & 16777215);
          brush.setColor(((trans.get(traceIndex)) as i32) << 24 | color & 16777215);
        }
      }
      */
    }

    // java public static Bitmap slideSnapshot(KmoSlide slide, int slideW, int slideH, int requestW, int requestH)
    pub fn slideSnapshot__kmoslide_int_int_int_int(slide: &KmoSlide, slideW: i32, slideH: i32, requestW: i32, requestH: i32) -> Bitmap {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return slideSnapshot(slide, slideW, slideH, requestW, requestH, ShapeUtil.RoundCornerCLipSizeType.CLIP_SIZE_NONE);
      return Self::slideSnapshot__kmoslide_int_int_int_int_roundcornerclipsizetype(&slide, slideW, slideH, requestW, requestH, ShapeUtil::RoundCornerCLipSizeType.CLIP_SIZE_NONE);
      */
    }

    // java public static Bitmap slideSnapshot(KmoSlide slide, int slideW, int slideH, int requestW, int requestH, RoundCornerCLipSizeType type)
    pub fn slideSnapshot__kmoslide_int_int_int_int_roundcornerclipsizetype(slide: Option<&KmoSlide>, slideW: i32, slideH: i32, requestW: i32, requestH: i32, r#type: &RoundCornerCLipSizeType) -> Option<Bitmap> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (slide != null && requestW > 0 && requestH > 0)
      if let Some(slide) = slide {
        if requestW > 0 && requestH > 0 {
          // java Bitmap bitmap;
          let mut bitmap: Bitmap = Box::new(()) as Bitmap;
              // java bitmap = Bitmap.createBitmap(requestW, requestH, Config.RGB_565);
              bitmap = Bitmap::createBitmap(requestW, requestH, Config::RGB_565);
              // java OutOfMemoryError var8
              todo!();
                // java return null;
                return None;
          // java if (bitmap == null)
          if false {
            // java return null;
            return None;
          }
          // java else
          else {
            // java Canvas canvas = new Canvas(bitmap);
            let mut canvas: Canvas = Canvas::new(bitmap);
            // java canvas.drawColor(-1);
            canvas.drawColor(-1);
            // java canvas.save();
            canvas.save();
            // java slideSnapshot(canvas, slide, slideW, slideH, requestW, requestH, type);
            Self::slideSnapshot__canvas_kmoslide_int_int_int_int_roundcornerclipsizetype({ let __x: &KmoSlide = todo!("no align strategy matched"); __x }, slide, slideW, slideH, requestW, requestH, r#type);
            // java canvas.restore();
            canvas.restore();
            // java return bitmap;
            return bitmap;
          }
        }
      }
      // java else
      else {
        // java return null;
        return None;
      }
      */
    }

    // java public static Bitmap slideSnapshot(KmoSlide slide, int slideW, int slideH, int requestW, int requestH, boolean drawPh)
    pub fn slideSnapshot__kmoslide_int_int_int_int_boolean(slide: Option<&KmoSlide>, slideW: i32, slideH: i32, requestW: i32, requestH: i32, drawPh: bool) -> Option<Bitmap> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (slide != null && requestW > 0 && requestH > 0)
      if let Some(slide) = slide {
        if requestW > 0 && requestH > 0 {
          // java Bitmap bitmap;
          let mut bitmap: Bitmap = Box::new(()) as Bitmap;
              // java bitmap = Bitmap.createBitmap(requestW, requestH, Config.RGB_565);
              bitmap = Bitmap::createBitmap(requestW, requestH, Config::RGB_565);
              // java OutOfMemoryError var9
              todo!();
                // java return null;
                return None;
          // java if (bitmap == null)
          if false {
            // java return null;
            return None;
          }
          // java else
          else {
            // java Canvas canvas = new Canvas(bitmap);
            let mut canvas: Canvas = Canvas::new(bitmap);
            // java canvas.drawColor(-1);
            canvas.drawColor(-1);
            // java canvas.save();
            canvas.save();
            // java adjustCanvas(canvas, slide, slideW, slideH, requestW, requestH);
            Self::adjustCanvas(&mut canvas, &slide, slideW, slideH, requestW, requestH);
            // java if (drawPh)
            if drawPh {
              // java SlideRender slideRender = SlideRender.getOrAllocEditSlideRender();
              let mut slideRender: SlideRender = SlideRender::getOrAllocEditSlideRender();
              // java slideRender.draw(canvas, slide);
              slideRender.draw(canvas, slide);
              // java SlideRender.reuseEditSlideRender(slideRender);
              SlideRender::reuseEditSlideRender(slideRender);
            }
            // java else
            else {
              // java SlideRender slideRender = SlideRender.getOrAllocReadSlideRender();
              let mut slideRender: SlideRender = SlideRender::getOrAllocReadSlideRender();
              // java slideRender.draw(canvas, slide);
              slideRender.draw(canvas, slide);
              // java SlideRender.reuseReadSlideRender(slideRender);
              SlideRender::reuseReadSlideRender(slideRender);
            }
            // java canvas.restore();
            canvas.restore();
            // java return bitmap;
            return bitmap;
          }
        }
      }
      // java else
      else {
        // java return null;
        return None;
      }
      */
    }

    // java private static void adjustCanvas(Canvas canvas, KmoSlide slide, int slideW, int slideH, int requestW, int requestH)
    pub fn adjustCanvas(canvas: &mut Canvas, slide: &KmoSlide, slideW: i32, slideH: i32, requestW: i32, requestH: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java float slideScale = 1.0F * (float)slideW / (float)slideH;
      let mut slideScale: f32 = 1 * ((slideW) as f32) / ((slideH) as f32);
      // java float requestScale = 1.0F * (float)requestW / (float)requestH;
      let mut requestScale: f32 = 1 * ((requestW) as f32) / ((requestH) as f32);
      // java int suitWidth;
      let mut suitWidth: i32;
      // java int suitHeight;
      let mut suitHeight: i32;
      // java if (requestScale >= 1.0F)
      if requestScale >= 1 {
        // java if (requestScale < slideScale)
        if requestScale < slideScale {
          // java suitWidth = requestW;
          suitWidth = requestW;
          // java suitHeight = (int)((float)requestW / slideScale);
          suitHeight = ((((requestW) as f32) / slideScale) as i32);
        }
        // java else
        else {
          // java suitHeight = requestH;
          suitHeight = requestH;
          // java suitWidth = (int)((float)requestH * slideScale);
          suitWidth = ((((requestH) as f32) * slideScale) as i32);
        }
      }
      // java else if (requestScale > slideScale)
      else if requestScale > slideScale {
        // java suitHeight = requestH;
        suitHeight = requestH;
        // java suitWidth = (int)((float)requestH * slideScale);
        suitWidth = ((((requestH) as f32) * slideScale) as i32);
      }
      // java else
      else {
        // java suitWidth = requestW;
        suitWidth = requestW;
        // java suitHeight = (int)((float)requestW / slideScale);
        suitHeight = ((((requestW) as f32) / slideScale) as i32);
      }
      // java float scale = 1.0F * (float)suitWidth / (float)slideW;
      let mut scale: f32 = 1 * ((suitWidth) as f32) / ((slideW) as f32);
      // java float offsetX = (float)(requestW - suitWidth) / 2.0F;
      let mut offsetX: f32 = ((requestW - suitWidth) as f32) / 2;
      // java float offsetY = (float)(requestH - suitHeight) / 2.0F;
      let mut offsetY: f32 = ((requestH - suitHeight) as f32) / 2;
      // java canvas.translate(offsetX, offsetY);
      canvas.translate(offsetX, offsetY);
      // java canvas.scale(scale, scale);
      canvas.scale(scale, scale);
      // java canvas.clipRect(0, 0, slideW, slideH);
      canvas.clipRect(0, 0, slideW, slideH);
      */
    }

    // java public static void slideSnapshot(Canvas canvas, KmoSlide slide, int slideW, int slideH, int requestW, int requestH, RoundCornerCLipSizeType type)
    pub fn slideSnapshot__canvas_kmoslide_int_int_int_int_roundcornerclipsizetype(canvas: Option<&mut Canvas>, slide: Option<&KmoSlide>, slideW: i32, slideH: i32, requestW: i32, requestH: i32, r#type: &RoundCornerCLipSizeType) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (canvas != null && slide != null && requestW > 0 && requestH > 0)
      if let (Some(canvas), Some(slide)) = (canvas, slide) {
        if requestW > 0 && requestH > 0 {
          // java adjustCanvas(canvas, slide, slideW, slideH, requestW, requestH);
          Self::adjustCanvas(&mut canvas, &slide, slideW, slideH, requestW, requestH);
          // java ISlideRender slideRender = SlideRender.createPrintSlideRender();
          let mut slideRender: Box<dyn ISlideRender> = SlideRender::createPrintSlideRender();
          // java int drawFlag = slideRender.drawFlags() | '耀';
          let mut drawFlag: i32 = slideRender.drawFlags() | '耀';
          // java int followDrawFlag = slideRender.followerDrawFlags() | '耀';
          let mut followDrawFlag: i32 = slideRender.followerDrawFlags() | '耀';
          // java slideRender.setDrawFlags(drawFlag, followDrawFlag);
          slideRender.setDrawFlags(drawFlag, followDrawFlag);
          // java if (type != ShapeUtil.RoundCornerCLipSizeType.CLIP_SIZE_NONE)
          if r#type != ShapeUtil::RoundCornerCLipSizeType.CLIP_SIZE_NONE {
            // java Path path = new Path();
            let mut path: std::path::PathBuf = { let __x: std::path::PathBuf = todo!("no align strategy matched"); __x };
            // java RectF rectF = new RectF(0.0F, 0.0F, (float)slideW, (float)slideH);
            let mut rectF: RectF = RectF::new(0, 0, ((slideW) as f32), ((slideH) as f32));
            // java float radius = UnitsConverter.render().dipToPixel((float)type.mSize);
            let mut radius: f32 = UnitsConverter::render().dipToPixel(((r#type.mSize) as f32));
            // java path.addRoundRect(rectF, radius, radius, Direction.CCW);
            path.addRoundRect(rectF, radius, radius, Direction::CCW);
            // java canvas.clipPath(path);
            canvas.clipPath(path);
          }
          // java slideRender.draw(canvas, slide);
          slideRender.draw(canvas, slide);
        }
      }
      */
    }

    // java public static void setGlCachEnable(boolean enable)
    pub fn setGlCachEnable(enable: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java GlCache.setEnable(enable);
      GlCache::setEnable(enable);
      */
    }

    // java public static boolean invertCoordFor3dShape(KmoShowShape kmoShape, RectF rcShape, PointF pt, boolean forTextCoord)
    pub fn invertCoordFor3dShape(kmoShape: &KmoShowShape, rcShape: &RectF, pt: &mut PointF, forTextCoord: bool) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java cn.wps.moffice.drawing.Shape shape = kmoShape.getSlideCache().getShape(kmoShape);
      let mut shape: Shape = kmoShape.getSlideCache().getShape(kmoShape);
      // java if (shape == null)
      if false {
        // java InterpretCtx ctx = new InterpretCtx();
        let mut ctx: InterpretCtx = InterpretCtx::new();
        // java ctx.setSlide(kmoShape.parent());
        ctx.setSlide(kmoShape.parent());
        // java shape = ConvertTool.convertShapeToPublicShape(kmoShape, ctx);
        shape = ConvertTool::convertShapeToPublicShape(kmoShape, ctx);
        // java if (shape == null || shape.getObject3d() == null || !shape.getObject3d().get3D())
        if false || shape.getObject3d().is_none() || !shape.getObject3d().get3D() {
          // java return false;
          return false;
        }
        // java shape.setShapeID(kmoShape.id());
        shape.setShapeID(kmoShape.id());
        // java kmoShape.getSlideCache().addShape(kmoShape, shape);
        kmoShape.getSlideCache().addShape(kmoShape, shape);
      }
      // java else if (shape.getObject3d() == null || !shape.getObject3d().get3D())
      else if shape.getObject3d().is_none() || !shape.getObject3d().get3D() {
        // java return false;
        return false;
      }
      // java cn.wps.graphics.RectF typeRect = cn.wps.graphics.RectF.obtain();
      let mut typeRect: RectF = cn.wps.graphics.RectF.obtain();
      // java UnitsConverter units = UnitsConverter.render();
      let mut units: UnitsConverter = UnitsConverter::render();
      // java typeRect.set(units.fromEmuX(rcShape.left), units.fromEmuY(rcShape.top), units.fromEmuX(rcShape.right), units.fromEmuY(rcShape.bottom));
      typeRect.set(units.fromEmuX(rcShape.left), units.fromEmuY(rcShape.top), units.fromEmuX(rcShape.right), units.fromEmuY(rcShape.bottom));
      // java pt.offset(-rcShape.left, -rcShape.top);
      pt.offset(-rcShape.left, -rcShape.top);
      // java pt.set(units.fromEmuX(pt.x), units.fromEmuY(pt.y));
      pt.set(units.fromEmuX(pt.x), units.fromEmuY(pt.y));
      // java boolean ret = HitShape3DHelper.invertCoord(shape, typeRect, pt, forTextCoord);
      let mut ret: bool = HitShape3DHelper::invertCoord(shape, typeRect, pt, forTextCoord);
      // java if (ret)
      if ret {
        // java pt.set(units.toEmuX(pt.x), units.toEmuY(pt.y));
        pt.set(units.toEmuX(pt.x), units.toEmuY(pt.y));
        // java pt.offset(rcShape.left, rcShape.top);
        pt.offset(rcShape.left, rcShape.top);
      }
      // java typeRect.recycle();
      typeRect.recycle();
      // java return ret;
      return ret;
      */
    }

    // java public static String isGifShape(cn.wps.moffice.drawing.Shape showShape, IMediaLib mediaLib)
    pub fn isGifShape(showShape: &Shape, mediaLib: &IMediaLib) -> Option<String> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java String path = null;
      let mut path: String = Default::default();
      // java BlipFill blipFill = showShape.getBlipFill();
      let mut blipFill: BlipFill = showShape.getBlipFill();
      // java if (blipFill != null)
      if true {
        // java int mediaId = blipFill.getMediaID();
        let mut mediaId: i32 = blipFill.getMediaID();
        // java path = mediaLib.getMediaByID(mediaId, MediaTypeEnum.PICTURE);
        path = mediaLib.getMediaByID(mediaId, MediaTypeEnum::PICTURE);
        // java if (path != null)
        if true {
          // java String fileExtension = path.substring(path.lastIndexOf(".") + 1);
          let mut fileExtension: String = path.substring(path.lastIndexOf(".") + 1);
          // java boolean isGif = KmoMediaUtil.getMediaType(fileExtension) == 8;
          let mut isGif: bool = KmoMediaUtil::getMediaType(fileExtension) == 8;
          // java if (!isGif)
          if !isGif {
            // java path = null;
            path = None;
          }
        }
      }
      // java return path;
      return Some(path);
      */
    }

    // java public static boolean isTable(KmoShowShape drawable)
    pub fn isTable(drawable: &KmoShowShape) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (drawable != null && drawable.type() == 4)
      if true && drawable.r#type() == 4 {
        // java GraphicObject graphic = drawable.graphic();
        let mut graphic: Box<dyn GraphicObject> = drawable.graphic();
        // java if (graphic != null && graphic.type() == 1)
        if true && graphic.r#type() == 1 {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
      */
    }

    // java public static void calVirtualGroupRect(KmoShowShape[] shapes, RectF rect)
    pub fn calVirtualGroupRect(shapes: (), rect: &mut RectF) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (shapes != null && shapes.length != 0)
      if true && (shapes.len() as i32) != 0 {
        // java if (rect == null)
        if false {
          // java rect = new RectF();
          rect = RectF::new();
        }
        // java ShapeInfoCache shapeInfoCache = (ShapeInfoCache)shapes[0].getSlideCache().shapeInfoCache();
        let mut shapeInfoCache: ShapeInfoCache = (shapes[0usize].getSlideCache().shapeInfoCache()).downcast_ref::<ShapeInfoCache>().unwrap();
        // java ShapeInfo shapeInfo = shapeInfoCache.getLocal();
        let mut shapeInfo: ShapeInfo = shapeInfoCache.getLocal();
        // java for (KmoShowShape child : shapes)
        for child in shapes.iter() {
          // java float[] bound = calVirtualGroupBound(child, shapeInfoCache, shapeInfo, 0.0F, 0.0F);
          let mut bound: Vec<f32> = Self::calVirtualGroupBound(&child, &shapeInfoCache, &mut shapeInfo, 0, 0);
          // java if (rect.isEmpty())
          if rect.isEmpty() {
            // java rect.left = bound[0];
            rect.left = bound[0usize];
            // java rect.top = bound[1];
            rect.top = bound[1usize];
            // java rect.right = bound[2];
            rect.right = bound[2usize];
            // java rect.bottom = bound[3];
            rect.bottom = bound[3usize];
          }
          // java else
          else {
            // java rect.left = Math.min(rect.left, bound[0]);
            rect.left = (rect.left).min(bound[0usize]);
            // java rect.top = Math.min(rect.top, bound[1]);
            rect.top = (rect.top).min(bound[1usize]);
            // java rect.right = Math.max(rect.right, bound[2]);
            rect.right = (rect.right).max(bound[2usize]);
            // java rect.bottom = Math.max(rect.bottom, bound[3]);
            rect.bottom = (rect.bottom).max(bound[3usize]);
          }
        }
      }
      */
    }

    // java private static float[] calVirtualGroupBound(KmoShowShape shape, ShapeInfoCache cache, ShapeInfo info, float offX, float offY)
    pub fn calVirtualGroupBound(shape: &KmoShowShape, cache: &ShapeInfoCache, info: &mut ShapeInfo, offX: f32, offY: f32) -> Vec<f32> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java cache.shapeInfo(shape, info);
      cache.shapeInfo(shape, info);
      // java float left = (float)info.transform2DX;
      let mut left: f32 = ((info.transform2DX) as f32);
      // java float top = (float)info.transform2DY;
      let mut top: f32 = ((info.transform2DY) as f32);
      // java float right = left + (float)info.transform2DW;
      let mut right: f32 = left + ((info.transform2DW) as f32);
      // java float bottom = top + (float)info.transform2DH;
      let mut bottom: f32 = top + ((info.transform2DH) as f32);
      // java float rotation = toDegree(info.rotation);
      let mut rotation: f32 = Self::toDegree(info.rotation);
      // java rotation %= 360.0F;
      rotation = rotation % 360;
      // java if (shape.parentGroup() != null)
      if shape.parentGroup().is_some() {
        // java RectF rectF = new RectF(left, top, right, bottom);
        let mut rectF: RectF = RectF::new(left, top, right, bottom);
        // java calcRelativeRect(shape, rectF, false);
        Self::calcRelativeRect__kmoshowshape_rectf_boolean(shape, &mut rectF, false);
        // java left = offX + rectF.left;
        left = offX + rectF.left;
        // java top = offY + rectF.top;
        top = offY + rectF.top;
        // java right = offX + rectF.right;
        right = offX + rectF.right;
        // java bottom = offY + rectF.bottom;
        bottom = offY + rectF.bottom;
      }
      // java float absoluteLeft = left;
      let mut absoluteLeft: f32 = left;
      // java float absoluteTop = top;
      let mut absoluteTop: f32 = top;
      // java if (rotation != 0.0F)
      if rotation != 0 {
        // java float halfW = (right - left) / 2.0F;
        let mut halfW: f32 = right - left / 2;
        // java float halfH = (bottom - top) / 2.0F;
        let mut halfH: f32 = bottom - top / 2;
        // java double radians = degree2Radian(rotation);
        let mut radians: f64 = Self::degree2Radian(rotation);
        // java float deltaH = (float)((double)halfW * Math.sin(radians) + (double)halfH * Math.cos(radians) - (double)halfH);
        let mut deltaH: f32 = ((((halfW) as f64) * Math::sin(radians) + ((halfH) as f64) * Math::cos(radians) - ((halfH) as f64)) as f32);
        // java float deltaW = (float)((double)halfH * Math.sin(radians) + (double)halfW * Math.cos(radians) - (double)halfW);
        let mut deltaW: f32 = ((((halfH) as f64) * Math::sin(radians) + ((halfW) as f64) * Math::cos(radians) - ((halfW) as f64)) as f32);
        // java left -= deltaW;
        left = left - deltaW;
        // java top -= deltaH;
        top = top - deltaH;
        // java right += deltaW;
        right = right + deltaW;
        // java bottom += deltaH;
        bottom = bottom + deltaH;
      }
      // java if (shape.type() == 0)
      if shape.r#type() == 0 {
        // java int i = 0;
        let mut i: i32 = 0;
        let mut len: i32 = shape.childCount();
        // java for (int len = shape.childCount(); i < len ++i)
        while i < len {
          // java KmoShowShape child = shape.childAt(i);
          let mut child: KmoShowShape = shape.childAt(i);
          // java if (child.shapePr() != null && child.shapePr().hasXfrm() && child.shapePr().xfrm().hasRotation())
          if child.shapePr().is_some() && child.shapePr().hasXfrm() && child.shapePr().xfrm().hasRotation() {
            // java float[] bound = calVirtualGroupBound(child, cache, info, absoluteLeft, absoluteTop);
            let mut bound: Vec<f32> = Self::calVirtualGroupBound(&child, cache, info, absoluteLeft, absoluteTop);
            // java left = Math.min(left, bound[0]);
            left = (left).min(bound[0usize]);
            // java top = Math.min(top, bound[1]);
            top = (top).min(bound[1usize]);
            // java right = Math.max(right, bound[2]);
            right = (right).max(bound[2usize]);
            // java bottom = Math.max(bottom, bound[3]);
            bottom = (bottom).max(bound[3usize]);
          }
        }
      }
      // java return new float[]{left, top, right, bottom};
      return vec![left, top, right, bottom];
      */
    }

    // java private static double degree2Radian(float rotation)
    pub fn degree2Radian(rotation: f32) -> f64 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java rotation = rotation < 0.0F ? 360.0F + rotation : rotation;
      rotation = (if rotation < 0 { 360 + rotation } else { rotation });
      // java if (rotation > 90.0F && rotation <= 180.0F)
      if rotation > 90 && rotation <= 180 {
        // java rotation = 180.0F - rotation;
        rotation = 180 - rotation;
      }
      // java if (rotation > 180.0F && rotation <= 270.0F)
      if rotation > 180 && rotation <= 270 {
        // java rotation -= 180.0F;
        rotation = rotation - 180;
      }
      // java if (rotation > 270.0F && rotation <= 360.0F)
      if rotation > 270 && rotation <= 360 {
        // java rotation = 360.0F - rotation;
        rotation = 360 - rotation;
      }
      // java return Math.toRadians((double)rotation);
      return Math::toRadians(((rotation) as f64));
      */
    }

    // java public static boolean isSameArrayShape(KmoShowShape[] src, List<KmoShowShape> target)
    pub fn isSameArrayShape(src: (), target: ()) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (src != null && target != null && src.length == target.size())
      if true && true && (src.len() as i32) == (target.len() as i32) {
        // java int len = src.length;
        let mut len: i32 = (src.len() as i32);
        // java boolean isSame = true;
        let mut isSame: bool = true;
        let mut i: i32 = 0;
        // java for (int i = 0; i < len ++i)
        while i < len {
          // java if (src[i].id() != ((KmoShowShape)target.get(i)).id())
          if src[(i) as usize].id() != (target.get(i)).downcast_ref::<KmoShowShape>().unwrap().id() {
            // java isSame = false;
            isSame = false;
            // java break;
            break;
          }
        }
        // java return isSame;
        return isSame;
      }
      // java else
      else {
        // java return false;
        return false;
      }
      */
    }

}

// java public static enum RoundCornerCLipSizeType
impl RoundCornerCLipSizeType {
    // java public int mSize
    pub const mSize: i32 = 0;
    // java private RoundCornerCLipSizeType(int size)
    pub fn RoundCornerCLipSizeType(_size: i32) -> Self {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java this.mSize = size;
        */
    }
}
