#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unreachable_code, unused_assignments)]
// Java: import cn.wps.base.converter.UnitsConverter
use crate::cn::wps::base::converter::UnitsConverter::UnitsConverter;
// Java: import cn.wps.drawing.data.geometry.AdjAngle
use crate::cn::wps::show::convert::j2r_hosts::Point2D as AdjAngle;
// Java: import cn.wps.drawing.data.geometry.AdjCoordinate
use crate::cn::wps::show::convert::j2r_hosts::Point2D as AdjCoordinate;
// Java: import cn.wps.drawing.data.geometry.AdjPoint2D
use crate::cn::wps::show::util::layout_shape::AdjPoint2DObj as AdjPoint2D;
// Java: import cn.wps.drawing.data.geometry.CustomGeometry
use crate::cn::wps::show::util::layout_shape::GeomObj as CustomGeometry;
// Java: import cn.wps.drawing.data.geometry.GeomGuide
use crate::cn::wps::show::convert::j2r_hosts::Point2D as GeomGuide;
// Java: import cn.wps.drawing.data.geometry.PresetGeometry
use crate::cn::wps::show::util::layout_shape::GeomObj as PresetGeometry;
// Java: import cn.wps.graphics.RectF
use util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.drawing.smartshape.GeoFormula
use crate::cn::wps::show::convert::j2r_hosts::ImagePool as GeoFormula;
// Java: import cn.wps.moffice.drawing.smartshape.Geometry
use drawing::cn::wps::moffice::drawing::smartshape::Geometry::Geometry;
// Java: import cn.wps.moffice.drawing.smartshape.Segment
use drawing::cn::wps::moffice::drawing::smartshape::Segment::Segment;
// Java: import cn.wps.moffice.drawing.smartshape.ShapeType
use drawing::cn::wps::moffice::drawing::smartshape::ShapeType::ShapeType;
// Java: import cn.wps.show.render.cache.GeometryCache
use crate::cn::wps::show::convert::j2r_hosts::ImagePool as GeometryCache;
// Java: import cn.wps.show.util.FormulaParser
use crate::cn::wps::show::convert::j2r_hosts::ImagePool as FormulaParser;
// Java: import cn.wps.show.util.ShapeUtil
use crate::cn::wps::show::util::ShapeUtil::ShapeUtil;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import cn.wps.show.convert.PresetShapeTypeMap
use crate::cn::wps::show::convert::PresetShapeTypeMap::PresetShapeTypeMap;
// Java: import android.graphics.Path
use stubs::android::graphics::Path::Path;
// Java: import cn.wps.smartart.data.gemo.ConnectionSite
use crate::cn::wps::show::convert::j2r_hosts::Point2D;

use crate::cn::wps::show::convert::Vertices::Vertices;
use crate::cn::wps::show::util::layout_shape::{PathObj, PathSegmentObj};
use drawing::cn::wps::moffice::drawing::GRF::GRF;
use std::any::Any;
use std::sync::Arc;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// java public static final PresetShapeTypeMap shapeTypeMap = new PresetShapeTypeMap();
pub(crate) static shapeTypeMap: std::sync::LazyLock<PresetShapeTypeMap> =
                    std::sync::LazyLock::new(|| PresetShapeTypeMap::new());

// java public class GeomtryConvert
#[derive(Default)]
pub struct GeomtryConvert {
}

// java public class GeomtryConvert
impl GeomtryConvert {
    // java public static void convertDataToPublicShape(PresetGeometry presetGeometry, Shape publicShape, float w, float h)
    pub fn convertDataToPublicShape__presetgeometry_shape_float_float(presetGeometry: PresetGeometry, publicShape: &mut Shape, w: f32, h: f32) {
      // java Geometry geometry = publicShape.getGeometry();
      let mut geometry: Geometry = publicShape
        .getGeometry()
        .and_then(|a| a.downcast_ref::<Geometry>().map(|g| Geometry { base: Clone::clone(&g.base) }))
        .unwrap_or_else(Geometry::new);
      // java if (geometry == null)
      if false {
        // java geometry = new Geometry();
        geometry = Geometry::new();
      }
      // java boolean isConnector = false;
      let mut isConnector: bool = false;
      // java int presetType = presetGeometry.prst();
      let mut presetType: i32 = presetGeometry.prst();
      // java String shapeName = shapeTypeMap.get(presetType);
      let mut shapeName: String = shapeTypeMap.get(presetType);
      // java int shapeType = ShapeType.getShapeType(shapeName);
      let mut shapeType: i32 = ShapeType::getShapeType(&shapeName);
      // java if (shapeType == 0)
      if shapeType == 0 {
        // java Vertices.writeOOXMLGeometryData(shapeName, geometry);
        Vertices::writeOOXMLGeometryData(&shapeName, &mut geometry);
      }
      // java if (shapeName != null && shapeName.indexOf("Connector") != -1 && !shapeName.startsWith("flowChart"))
      if shapeName.contains("Connector") && !shapeName.starts_with("flowChart") {
        // java isConnector = true;
        isConnector = true;
      }
      // java publicShape.setShapeName(shapeName);
      publicShape.setShapeName(shapeName.clone());
      // java publicShape.setShapeType(shapeType);
      publicShape.setShapeType(shapeType);
      // java if (shapeType == 0)
      if shapeType == 0 {
        // java publicShape.setGeometry(geometry);
        publicShape.setGeometry(Some(Arc::new(Geometry { base: Clone::clone(&geometry.base) })));
      }
      // java if (presetGeometry.hasAvLst())
      if presetGeometry.hasAvLst() {
        // java PresetGeometry.AdjustValueList adjustValueList = presetGeometry.avLst();
        // rust: avLst formula apply is deferred this wave
        let _ = presetGeometry.av_lst.len();
        // java List<GeomGuide> adjustValues = new ArrayList();
        let mut adjustValues = presetGeometry.av_lst.len();
        let _ = adjustValues;
        // java adjustValueList.read(adjustValues);
        // rust: no-stmt
        // java setGemoryAdjust(geometry, adjustValues, w, h);
        // rust: no-stmt
        // java publicShape.setGeometry(geometry);
        publicShape.setGeometry(Some(Arc::new(Geometry { base: Clone::clone(&geometry.base) })));
      }
      // java publicShape.getGRF().setConnector(isConnector);
      let mut grf = publicShape
        .getGRF()
        .and_then(|a| a.downcast_ref::<GRF>().map(Clone::clone))
        .unwrap_or_else(GRF::new);
      grf.setConnector(isConnector);
      publicShape.setGRF(Some(Arc::new(grf)));
      // java if (shapeType == 0)
      if shapeType == 0 {
        // java revisePath(publicShape);
        // rust: no-stmt — revisePath parked; Java only mutates corner/halfFrame
      }
    }

    // java public static void convertPlaceHolderToPublicShape(Shape publicShape)
    pub fn convertPlaceHolderToPublicShape(publicShape: &mut Shape) {
      // java covertToPublicRectShape(publicShape);
      Self::covertToPublicRectShape(publicShape);
    }

    // java public static void convertBackgroundToPublicShape(Shape publicShape)
    pub fn convertBackgroundToPublicShape(publicShape: &mut Shape) {
      // java covertToPublicRectShape(publicShape);
      Self::covertToPublicRectShape(publicShape);
    }

    // java public static void covertToPublicRectShape(Shape publicShape)
    pub fn covertToPublicRectShape(publicShape: &mut Shape) {
      // java String shapeName = shapeTypeMap.get(1);
      let mut shapeName: String = shapeTypeMap.get(1);
      // java int shapeType = ShapeType.getShapeType(shapeName);
      let mut shapeType: i32 = ShapeType::getShapeType(&shapeName);
      // java publicShape.setShapeName(shapeName);
      publicShape.setShapeName(shapeName);
      // java publicShape.setShapeType(shapeType);
      publicShape.setShapeType(shapeType);
    }

    // java public static void revisePath(Shape publicShape)
    pub fn revisePath(publicShape: &mut Shape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (publicShape.getShapeType() == 0 && "corner".equals(publicShape.getShapeName()))
      if publicShape.getShapeType() == 0 && "corner".equals(publicShape.getShapeName()) {
        // java RectF rectf = (RectF)publicShape.getProperty(733);
        let mut rectf: RectF = (publicShape.getProperty(733)).downcast_ref::<RectF>().unwrap();
        // java if (rectf == null || rectf.right == 0.0F || rectf.bottom == 0.0F)
        if false || rectf.right == 0 || rectf.bottom == 0 {
          // java return;
          return;
        }
        // java float w = rectf.right - rectf.left;
        let mut w: f32 = rectf.right - rectf.left;
        // java float h = rectf.bottom - rectf.top;
        let mut h: f32 = rectf.bottom - rectf.top;
        // java int[] vector = publicShape.getGeometry().getVertices();
        let mut vector: Vec<i32> = publicShape.getGeometry().getVertices();
        // java vector[6] = vector[8] = (int)w;
        vector[6usize] = { vector[8usize] = ((w) as i32); vector[8usize] };
        // java vector[9] = vector[11] = (int)h;
        vector[9usize] = { vector[11usize] = ((h) as i32); vector[11usize] };
        // java int len = (int)(w < h ? w : h);
        let mut len: i32 = (((if w < h { w } else { h })) as i32);
        // java len /= 2;
        len = len / 2;
        // java vector[2] = vector[4] = len;
        vector[2usize] = { vector[4usize] = len; vector[4usize] };
        // java vector[5] = vector[7] = vector[9] - len;
        vector[5usize] = { vector[7usize] = vector[9usize] - len; vector[7usize] };
        // java publicShape.getGeometry().setGeoCordSize(new RectF(0.0F, 0.0F, w, h));
        publicShape.getGeometry().setGeoCordSize(RectF::new(0, 0, w, h));
      }
      // java else if (publicShape.getShapeType() == 0 && "halfFrame".equals(publicShape.getShapeName()))
      else if publicShape.getShapeType() == 0 && "halfFrame".equals(publicShape.getShapeName()) {
        // java RectF rectf = (RectF)publicShape.getProperty(733);
        let mut rectf: RectF = (publicShape.getProperty(733)).downcast_ref::<RectF>().unwrap();
        // java if (rectf == null || rectf.right == 0.0F || rectf.bottom == 0.0F)
        if false || rectf.right == 0 || rectf.bottom == 0 {
          // java return;
          return;
        }
        // java float w = rectf.right - rectf.left;
        let mut w: f32 = rectf.right - rectf.left;
        // java float h = rectf.bottom - rectf.top;
        let mut h: f32 = rectf.bottom - rectf.top;
        // java int[] vector = publicShape.getGeometry().getVertices();
        let mut vector: Vec<i32> = publicShape.getGeometry().getVertices();
        // java vector[2] = (int)w;
        vector[2usize] = ((w) as i32);
        // java vector[11] = (int)h;
        vector[11usize] = ((h) as i32);
        // java int len = (int)(w < h ? w : h);
        let mut len: i32 = (((if w < h { w } else { h })) as i32);
        // java len /= 3;
        len = len / 3;
        // java vector[5] = vector[6] = vector[7] = vector[8] = len;
        vector[5usize] = { vector[6usize] = { vector[7usize] = { vector[8usize] = len; vector[8usize] }; vector[7usize] }; vector[6usize] };
        // java vector[4] = (int)(-w / h * (float)len + w);
        vector[4usize] = ((-w / h * ((len) as f32) + w) as i32);
        // java vector[9] = (int)((w - (float)len) * (h / w));
        vector[9usize] = ((w - ((len) as f32) * h / w) as i32);
        // java publicShape.getGeometry().setGeoCordSize(new RectF(0.0F, 0.0F, w, h));
        publicShape.getGeometry().setGeoCordSize(RectF::new(0, 0, w, h));
      }
      */
    }

    // java static void convertPath(FormulaParser parser, ArrayList<CustomGeometry.Path> pathList, List<Segment> segments, List<Integer> vectices, float w, float h)
    pub fn convertPath(_parser: Option<()>, pathList: &Vec<PathObj>, segments: &mut Vec<Segment>, vectices: &mut Vec<i32>, _w: f32, _h: f32) {
      // java int pathSize = pathList.size();
      let mut pathSize: i32 = (pathList.len() as i32);
      // java int count = 0;
      let mut count: i32 = 0;
      let mut i: i32 = 0;
      // java for (int i = 0; i < pathSize ++i)
      while i < pathSize {
        // java CustomGeometry.Path path = (CustomGeometry.Path)pathList.get(i);
        let path: &PathObj = &pathList[i as usize];
        // java if (i != 0)
        if i != 0 {
          // java Segment end = new Segment((short)128, count);
          let mut end: Segment = Segment::with_params((128) as i16, count);
          // java segments.add(end);
          segments.push(end);
        }
        // java if (path.hasFill() && path.fill() == 4)
        if path.hasFill() && path.fill() == 4 {
          // java Segment noFill = new Segment((short)170, count);
          let mut noFill: Segment = Segment::with_params((170) as i16, count);
          // java segments.add(noFill);
          segments.push(noFill);
        }
        // java if (path.hasStroke() && !path.isStroke())
        if path.hasStroke() && !path.isStroke() {
          // java Segment noStroke = new Segment((short)171, count);
          let mut noStroke: Segment = Segment::with_params((171) as i16, count);
          // java segments.add(noStroke);
          segments.push(noStroke);
        }
        // java CustomGeometry.Path.PathSegmentList segmentList = path.pathSegmentLst();
        let segmentList = path.pathSegmentLst();
        // java ArrayList<CustomGeometry.Path.PathSegment> pathSegments = new ArrayList();
        let pathSegments: &Vec<PathSegmentObj> = segmentList;
        // java segmentList.read(pathSegments);
        // rust: no-stmt
        // java int j = 0;
        let mut j: i32 = 0;
        let mut segSize: i32 = (pathSegments.len() as i32);
        // java for (int segSize = pathSegments.size(); j < segSize ++j)
        while j < segSize {
          // java CustomGeometry.Path.PathSegment pathSegment = (CustomGeometry.Path.PathSegment)pathSegments.get(j);
          let pathSegment: &PathSegmentObj = &pathSegments[j as usize];
          // java switch (pathSegment.type())
          match pathSegment.r#type() {
            // java case 0:
            0 => {
              // java Segment close = new Segment((short)96, count);
              let mut close: Segment = Segment::with_params((96) as i16, count);
              // java segments.add(close);
              segments.push(close);
            }
            // java case 1:
            1 => {
              // java Segment segment = new Segment((short)64, count);
              let mut segment: Segment = Segment::with_params((64) as i16, count);
              // java segments.add(segment);
              segments.push(segment);
              // java AdjPoint2D point = pathSegment.moveTo();
              let point = pathSegment.moveTo();
              // java vectices.add((int)ShapeUtil.getGdValue(point.x(), parser, w, h));
              vectices.push(point.x().coordinate());
              // java vectices.add((int)ShapeUtil.getGdValue(point.y(), parser, w, h));
              vectices.push(point.y().coordinate());
              // java count += 2;
              count = count + 2;
            }
            // java case 2:
            2 => {
              // java Segment lineTo = new Segment((short)0, count);
              let mut lineTo: Segment = Segment::with_params((0) as i16, count);
              // java segments.add(lineTo);
              segments.push(lineTo);
              // java AdjPoint2D lnTo = pathSegment.lnTo();
              let lnTo = pathSegment.lnTo();
              // java vectices.add((int)ShapeUtil.getGdValue(lnTo.x(), parser, w, h));
              vectices.push(lnTo.x().coordinate());
              // java vectices.add((int)ShapeUtil.getGdValue(lnTo.y(), parser, w, h));
              vectices.push(lnTo.y().coordinate());
              // java count += 2;
              count = count + 2;
            }
            // java case 3:
            3 => {
              // java Segment arcSegment = new Segment((short)163, count);
              let mut arcSegment: Segment = Segment::with_params((163) as i16, count);
              // java segments.add(arcSegment);
              segments.push(arcSegment);
              // java CustomGeometry.Path.ArcTo arcTo = pathSegment.arcTo();
              let arcTo = pathSegment.arcTo();
              // java AdjAngle startAngle = arcTo.startAngle();
              // rust: PathSegmentObj carries one AdjPoint this wave
              // java AdjAngle swingAngle = arcTo.swingAngle();
              // rust: no-stmt
              // java AdjCoordinate widthRadius = arcTo.widthRadius();
              // rust: no-stmt
              // java AdjCoordinate heightRadius = arcTo.heightRadius();
              // rust: no-stmt
              // java vectices.add((int)ShapeUtil.getGdValue(widthRadius, parser, w, h));
              vectices.push(arcTo.x().coordinate());
              // java vectices.add((int)ShapeUtil.getGdValue(heightRadius, parser, w, h));
              vectices.push(arcTo.y().coordinate());
              // java vectices.add((int)ShapeUtil.getGdValue(startAngle, parser, w, h));
              vectices.push(0);
              // java vectices.add((int)ShapeUtil.getGdValue(swingAngle, parser, w, h));
              vectices.push(0);
              // java count += 4;
              count = count + 4;
            }
            // java case 4:
            4 => {
              // java Segment curveTo = new Segment((short)32, count);
              let mut curveTo: Segment = Segment::with_params((32) as i16, count);
              // java segments.add(curveTo);
              segments.push(curveTo);
              // java CustomGeometry.Path.CubicBezierCurveTo cubicBezTo = pathSegment.cubicBezTo();
              let cubicBezTo = pathSegment.cubicBezTo();
              // java AdjPoint2D first = cubicBezTo.first();
              let first = cubicBezTo;
              // java AdjPoint2D second = cubicBezTo.second();
              let second = cubicBezTo;
              // java AdjPoint2D third = cubicBezTo.third();
              let third = cubicBezTo;
              // java vectices.add((int)ShapeUtil.getGdValue(first.x(), parser, w, h));
              vectices.push(first.x().coordinate());
              // java vectices.add((int)ShapeUtil.getGdValue(first.y(), parser, w, h));
              vectices.push(first.y().coordinate());
              // java vectices.add((int)ShapeUtil.getGdValue(second.x(), parser, w, h));
              vectices.push(second.x().coordinate());
              // java vectices.add((int)ShapeUtil.getGdValue(second.y(), parser, w, h));
              vectices.push(second.y().coordinate());
              // java vectices.add((int)ShapeUtil.getGdValue(third.x(), parser, w, h));
              vectices.push(third.x().coordinate());
              // java vectices.add((int)ShapeUtil.getGdValue(third.y(), parser, w, h));
              vectices.push(third.y().coordinate());
              // java count += 6;
              count = count + 6;
            }
            // java case 5:
            5 => {
              // java Segment quadCurveTo = new Segment((short)169, count);
              let mut quadCurveTo: Segment = Segment::with_params((169) as i16, count);
              // java segments.add(quadCurveTo);
              segments.push(quadCurveTo);
              // java CustomGeometry.Path.QuadraticBezierCurveTo quadBezTo = pathSegment.quadBezTo();
              let quadBezTo = pathSegment.quadBezTo();
              // java AdjPoint2D first = quadBezTo.first();
              let first = quadBezTo;
              // java AdjPoint2D second = quadBezTo.second();
              let second = quadBezTo;
              // java vectices.add((int)ShapeUtil.getGdValue(first.x(), parser, w, h));
              vectices.push(first.x().coordinate());
              // java vectices.add((int)ShapeUtil.getGdValue(first.y(), parser, w, h));
              vectices.push(first.y().coordinate());
              // java vectices.add((int)ShapeUtil.getGdValue(second.x(), parser, w, h));
              vectices.push(second.x().coordinate());
              // java vectices.add((int)ShapeUtil.getGdValue(second.y(), parser, w, h));
              vectices.push(second.y().coordinate());
              // java count += 4;
              count = count + 4;
            }
            // java default:
            _ => {
            }
          }
          j += 1;
        }
        i += 1;
      }
    }

    // java public static void convertDataToPublicShape(CustomGeometry customGeometry, Shape publicShape, float w, float h)
    pub fn convertDataToPublicShape__customgeometry_shape_float_float(customGeometry: CustomGeometry, publicShape: &mut Shape, w: f32, h: f32) {
      // java Geometry geometry = GeometryCache.get(publicShape.getShapeID(), w, h);
      let mut geometry: Option<Geometry> = None;
      // java if (geometry != null)
      if let Some(geometry) = geometry {
        // java publicShape.setGeometry(geometry);
        publicShape.setGeometry(Some(Arc::new(geometry)));
      }
      // java else
      else {
        // java CustomGeometry.PathList dataPathList = customGeometry.pathLst();
        let dataPathList = customGeometry.pathLst();
        // java ArrayList<CustomGeometry.Path> pathList = new ArrayList();
        let pathList: Vec<PathObj> = dataPathList;
        // java dataPathList.read(pathList);
        // rust: no-stmt
        // java int pathCount = pathList.size();
        let mut pathCount: i32 = (pathList.len() as i32);
        // java if (pathCount != 0)
        if pathCount != 0 {
          // java geometry = publicShape.getGeometry();
          let mut geometry: Geometry = publicShape
            .getGeometry()
            .and_then(|a| a.downcast_ref::<Geometry>().map(|g| Geometry { base: Clone::clone(&g.base) }))
            .unwrap_or_else(Geometry::new);
          // java if (geometry == null)
          if false {
            // java geometry = new Geometry();
            geometry = Geometry::new();
          }
          // java FormulaParser parser = null;
          let mut parser: Option<()> = None;
          // java if (customGeometry.hasAvLst())
          if customGeometry.hasAvLst() {
            // java CustomGeometry.AdjustValueList adjustValueList = customGeometry.avLst();
            // rust: no-stmt
            // java List<GeomGuide> adjustValues = new ArrayList();
            let mut adjustValues = customGeometry.av_lst.len();
            let _ = adjustValues;
            // java adjustValueList.read(adjustValues);
            // rust: no-stmt
            // java parser = setGemoryAdjust(geometry, adjustValues, w, h);
            parser = parser;
          }
          // java int forcount = 0;
          let mut forcount: i32 = 0;
          // java if (customGeometry.hasGdLst())
          if !customGeometry.gd_lst.is_empty() {
            // java CustomGeometry.GeomGuideList guideList = customGeometry.gdLst();
            // rust: no-stmt
            // java List<GeomGuide> adjustValues = new ArrayList();
            let mut adjustValues = customGeometry.gd_lst.len();
            // java guideList.read(adjustValues);
            // rust: no-stmt
            // java forcount = adjustValues.size();
            forcount = adjustValues as i32;
            // java parser = setGemoryFormula(parser, geometry, adjustValues, w, h);
            parser = parser;
          }
          // java setCustGemoRect(parser, customGeometry, geometry, w, h);
          // rust: no-stmt
          // java if (customGeometry.hasCxnList())
          if false {
          }
          // java CustomGeometry.Path path = (CustomGeometry.Path)pathList.get(0);
          let path: &PathObj = &pathList[0];
          // java float cordWidth = (float)path.w();
          let mut cordWidth: f32 = (path.w() as f32);
          // java float cordHeight = (float)path.h();
          let mut cordHeight: f32 = (path.h() as f32);
          // java RectF geoCordSize = new RectF(0.0F, 0.0F, cordWidth, cordHeight);
          let mut geoCordSize: RectF = RectF::new_with(0.0, 0.0, cordWidth, cordHeight);
          // java if (cordWidth == 0.0F && cordHeight == 0.0F)
          if cordWidth == 0.0 && cordHeight == 0.0 {
            // java geoCordSize.set(0.0F, 0.0F, w, h);
            geoCordSize.set(0.0, 0.0, w, h);
          }
          // java geometry.setGeoCordSize(geoCordSize);
          geometry.setGeoCordSize(Some(Arc::new(geoCordSize)));
          // java List<Segment> segments = new ArrayList();
          let mut segments: Vec<Segment> = Vec::new();
          // java List<Integer> vectices = new ArrayList();
          let mut vectices: Vec<i32> = Vec::new();
          // java convertPath(parser, pathList, segments, vectices, w, h);
          Self::convertPath(parser, &pathList, &mut segments, &mut vectices, w, h);
          // java int[] pvectices = new int[vectices.size()];
          let mut pvectices: Vec<i32> = vectices.clone();
          // java int i = 0;
          let mut i: i32 = 0;
          let mut size: i32 = (vectices.len() as i32);
          // java for (int size = vectices.size(); i < size ++i)
          while i < size {
            // java pvectices[i] = (Integer)vectices.get(i);
            pvectices[i as usize] = vectices[i as usize];
            i += 1;
          }
          // java geometry.setVertices(pvectices);
          geometry.setVertices(Some(Arc::new(pvectices)));
          // java geometry.setSegment((Segment[])segments.toArray(new Segment[0]));
          geometry.setSegment(Some(Arc::new(segments)));
          // java geometry.setVerticesCbElem(8);
          geometry.setVerticesCbElem(8);
          // java publicShape.setGeometry(geometry);
          publicShape.setGeometry(Some(Arc::new(geometry)));
          // java if (segments.size() > 50)
          // rust: GeometryCache skipped (convert↔render cycle)
        }
      }
    }

    // java private static void setCustGemoRect(FormulaParser parser, CustomGeometry customGeometry, Geometry geometry, float w, float h)
    pub fn setCustGemoRect(parser: &mut FormulaParser, customGeometry: &CustomGeometry, geometry: &mut Geometry, w: f32, h: f32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (customGeometry.hasRect())
      if customGeometry.hasRect() {
        // java CustomGeometry.GeomRect rcTxt = customGeometry.rect();
        let mut rcTxt: CustomGeometry::GeomRect = customGeometry.rect();
        // java RectF rc = geometry.getGeoTextRect();
        let mut rc: RectF = geometry.getGeoTextRect();
        // java if (rc == null)
        if false {
          // java rc = new RectF();
          rc = RectF::new();
        }
        // java float scaleX = UnitsConverter.docRenderScaleX();
        let mut scaleX: f32 = UnitsConverter::docRenderScaleX();
        // java float scaleY = UnitsConverter.docRenderScaleY();
        let mut scaleY: f32 = UnitsConverter::docRenderScaleY();
        // java rc.left = (float)ShapeUtil.getGdValue(rcTxt.l(), parser, w, h) * scaleX;
        rc.left = ((ShapeUtil::getGdValue__adjcoordinate_formulaparser_float_float(rcTxt.l(), parser, w, h)) as f32) * scaleX;
        // java rc.top = (float)ShapeUtil.getGdValue(rcTxt.t(), parser, w, h) * scaleY;
        rc.top = ((ShapeUtil::getGdValue__adjcoordinate_formulaparser_float_float(rcTxt.t(), parser, w, h)) as f32) * scaleY;
        // java rc.right = (float)ShapeUtil.getGdValue(rcTxt.r(), parser, w, h) * scaleX;
        rc.right = ((ShapeUtil::getGdValue__adjcoordinate_formulaparser_float_float(rcTxt.r(), parser, w, h)) as f32) * scaleX;
        // java rc.bottom = (float)ShapeUtil.getGdValue(rcTxt.b(), parser, w, h) * scaleY;
        rc.bottom = ((ShapeUtil::getGdValue__adjcoordinate_formulaparser_float_float(rcTxt.b(), parser, w, h)) as f32) * scaleY;
        // java geometry.setGeoTextRect(rc);
        geometry.setGeoTextRect(rc);
      }
      */
    }

    // java private static FormulaParser setGemoryFormula(FormulaParser parser, Geometry geometry, List<GeomGuide> adjustValues, float w, float h)
    pub fn setGemoryFormula(parser: Option<FormulaParser>, geometry: &mut Geometry, adjustValues: (), w: f32, h: f32) -> FormulaParser {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (parser == null)
      if parser.is_none() {
        // java parser = new FormulaParser();
        parser = FormulaParser::new();
      }
      // java FormulaParser.FormulaParseResult result = parser.parseFormula(adjustValues, w, h);
      let mut result: FormulaParser::FormulaParseResult = parser.parseFormula(adjustValues, w, h);
      // java GeoFormula[] geoFormulas = result.mGeoFormula;
      let mut geoFormulas: Vec<GeoFormula> = result.mGeoFormula;
      // java if (geoFormulas != null)
      if true {
        // java geometry.setFormulas(geoFormulas);
        geometry.setFormulas(geoFormulas);
      }
      // java return parser;
      return parser.unwrap();
      */
    }

    // java private static FormulaParser setGemoryAdjust(Geometry geometry, List<GeomGuide> adjustValues, float w, float h)
    pub fn setGemoryAdjust(geometry: &mut Geometry, adjustValues: (), w: f32, h: f32) -> FormulaParser {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java FormulaParser parser = new FormulaParser();
      let mut parser: FormulaParser = FormulaParser::new();
      // java FormulaParser.FormulaParseResult result = parser.parseAdjust(adjustValues, w, h);
      let mut result: FormulaParser::FormulaParseResult = parser.parseAdjust(adjustValues, w, h);
      // java if (result.mGeoAdjust != null)
      if result.mGeoAdjust.is_some() {
        // java geometry.setGeoAdjust(result.mGeoAdjust);
        geometry.setGeoAdjust(result.mGeoAdjust);
      }
      // java return parser;
      return parser;
      */
    }

    // java private static void setConnectionSite(FormulaParser parser, Geometry geometry, List<CustomGeometry.ConnectionSite> adjustValues)
    pub fn setConnectionSite(parser: &mut FormulaParser, geometry: &mut Geometry, adjustValues: ()) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int[] v = new int[adjustValues.size()];
      let mut v: Vec<i32> = vec![Default::default(); (adjustValues.len() as i32) as usize];
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (adjustValues.len() as i32);
      // java for (int size = adjustValues.size(); i < size ++i)
      while i < size {
        // java CustomGeometry.ConnectionSite site = (CustomGeometry.ConnectionSite)adjustValues.get(i);
        let mut site: CustomGeometry::ConnectionSite = { let __x: CustomGeometry::ConnectionSite = todo!("no align strategy matched"); __x };
        // java site.ang();
        site.ang();
        // java site.pos();
        site.pos();
      }
      // java geometry.setConnectionSites(v);
      geometry.setConnectionSites(v);
      */
    }

}
