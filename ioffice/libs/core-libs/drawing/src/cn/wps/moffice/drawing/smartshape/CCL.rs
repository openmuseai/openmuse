#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables, unused_mut, unused_assignments)]

use std::sync::Arc;
use std::any::Any;
use std::collections::HashMap;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: import cn.wps.graphics.PointF
use core_libs_util::graphics::PointF::PointF;
// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.graphics.GraphicsPath
use crate::cn::wps::moffice::drawing::graphics::GraphicsPath::GraphicsPath;
// Java: import cn.wps.moffice.drawing.graphics.PathElem
use crate::cn::wps::moffice::drawing::graphics::PathElem::PathElem;
use super::PresetData::PresetData;

// Java field: int C_MOVETO
pub(crate) const C_MOVETO: i32 = 0;
// Java field: int C_LINETO
pub(crate) const C_LINETO: i32 = 1;
// Java field: int C_CLOSE
pub(crate) const C_CLOSE: i32 = 2;
// Java field: int C_ARCTO
pub(crate) const C_ARCTO: i32 = 3;
// Java field: int C_QUADBEZTO
pub(crate) const C_QUADBEZTO: i32 = 4;
// Java field: int C_CUBICBEZTO
pub(crate) const C_CUBICBEZTO: i32 = 5;
// Java field: int C_QAUDRANTX
pub(crate) const C_QAUDRANTX: i32 = 6;
// Java field: int C_QAUDRANTY
pub(crate) const C_QAUDRANTY: i32 = 7;
// Java field: int C_ESCAPEARC
pub(crate) const C_ESCAPEARC: i32 = 8;
// Java field: int C_ESCAPEARCTO
pub(crate) const C_ESCAPEARCTO: i32 = 9;
// Java field: int C_CLOCKWISEARC
pub(crate) const C_CLOCKWISEARC: i32 = 10;
// Java field: int C_CLOCKWISEARCTO
pub(crate) const C_CLOCKWISEARCTO: i32 = 11;
// Java field: int C_ANGLEELLIPSE
pub(crate) const C_ANGLEELLIPSE: i32 = 12;
// Java field: int C_UNKNOWN
pub(crate) const C_UNKNOWN: i32 = -1;
// Java field: int C_STOP
pub(crate) const C_STOP: i32 = -2;
// Java field: int A_Fill_Darken
pub(crate) const A_Fill_Darken: i32 = 6;
// Java field: int A_Fill_DarkenLess
pub(crate) const A_Fill_DarkenLess: i32 = 7;
// Java field: int A_Fill_Lighten
pub(crate) const A_Fill_Lighten: i32 = 8;
// Java field: int A_Fill_Lightenless
pub(crate) const A_Fill_Lightenless: i32 = 9;
// Java field: int A_Fill_None
pub(crate) const A_Fill_None: i32 = 10;
// Java field: int A_Fill_Norm
pub(crate) const A_Fill_Norm: i32 = 11;
// Java field: int A_Stroke_None
pub(crate) const A_Stroke_None: i32 = 12;
// Java field: int A_Stroke_Norm
pub(crate) const A_Stroke_Norm: i32 = 13;
// Java field: int AH_X
pub(crate) const AH_X: i32 = 14;
// Java field: int AH_Y
pub(crate) const AH_Y: i32 = 16;
// Java field: int AH_Pos
pub(crate) const AH_Pos: i32 = 17;
// Java field: int AH_Ang
pub(crate) const AH_Ang: i32 = 18;
// Java field: int AH_Rad
pub(crate) const AH_Rad: i32 = 19;

// Java: public class CCL
pub struct CCL {
}

// Java: public class CCL
impl CCL {
    // Java: CCL()
    pub fn new() -> Self {
      // Java: super();
      CCL {}
    }

    // Java: static GraphicsPath[] generatePath(PathData pathData, Map<Integer, Float> gdValueBuf, int[] varList)
    pub fn generatePath(pathData: &PresetData, gdValueBuf: &mut HashMap<i32, f32>, varList: &[i32]) -> Option<Vec<GraphicsPath>> {
      // Java: return generatePath(pathData, true, gdValueBuf, varList);
      Self::generatePath_with_render(pathData, true, gdValueBuf, varList)
    }

    // Java: static GraphicsPath[] generatePath(PathData pathData, boolean isRenderPath, Map<Integer, Float> gdValueBuf, int[] varList)
    pub fn generatePath_with_render(pathData: &PresetData, isRenderPath: bool, gdValueBuf: &mut HashMap<i32, f32>, varList: &[i32]) -> Option<Vec<GraphicsPath>> {
      // Java: int[][] presetShapePathList = pathData.getPathList();
      let presetShapePathList = pathData.getPathList();
      // Java: if (presetShapePathList == null)
      if presetShapePathList.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         let presetShapePathList = presetShapePathList.unwrap();
         // Java: int pathCommand = 0;
         let mut pathCommand: i32 = 0;
         // Java: float currX = 0.0F;
         let mut currX: f32 = 0.0_f32;
         // Java: float currY = 0.0F;
         let mut currY: f32 = 0.0_f32;
         // Java: float dstX = 0.0F;
         let mut dstX: f32 = 0.0_f32;
         // Java: float dstY = 0.0F;
         let mut dstY: f32 = 0.0_f32;
         // Java: int shapeType = pathData.getType();
         let shapeType: i32 = pathData.base.getType();
         // Java: RectF rectF = shapeType != 106 && shapeType != 225 ? null : pathData.getRect();
         let rectF: Option<&RectF> = if shapeType != 106 && shapeType != 225 { None } else { pathData.base.getRect() };
         // Java: int pathListSize = presetShapePathList.length;
         let pathListSize = presetShapePathList.len();
         // Java: GraphicsPath[] gpList = new GraphicsPath[pathListSize];
         let mut gpList: Vec<GraphicsPath> = Vec::with_capacity(pathListSize);
         // Java: for(int j = 0; j < pathListSize; ++j)
         for j in 0..pathListSize
         {
            // Java: int[] presetShapePath = presetShapePathList[j];
            let presetShapePath = &presetShapePathList[j];
            // Java: GraphicsPath graphicsPath = new GraphicsPath();
            let mut graphicsPath = GraphicsPath::new();
            // Java: graphicsPath.setShapeType(shapeType);
            graphicsPath.setShapeType(shapeType);
            // Java: if (shapeType == 99)
            if shapeType == 99
            {
               // Java: graphicsPath.setFillType2(1);
               graphicsPath.setFillType2(1);
            }
            // Java: PathElem pathElem = null;
            let mut pathElem: Option<PathElem> = None;
            // Java: if (presetShapePath[0] != 10)
            if presetShapePath[0] != 10
            {
               // Java: if (presetShapePath[0] == 6)
               if presetShapePath[0] == 6
               {
                  // Java: graphicsPath.setFillType(0);
                  graphicsPath.setFillType(0);
               }
               // Java: else if (presetShapePath[0] == 7)
               else if presetShapePath[0] == 7
               {
                  // Java: graphicsPath.setFillType(1);
                  graphicsPath.setFillType(1);
               }
               // Java: else if (presetShapePath[0] == 8)
               else if presetShapePath[0] == 8
               {
                  // Java: graphicsPath.setFillType(2);
                  graphicsPath.setFillType(2);
               }
               // Java: else if (presetShapePath[0] == 9)
               else if presetShapePath[0] == 9
               {
                  // Java: graphicsPath.setFillType(3);
                  graphicsPath.setFillType(3);
               }
               // Java: else if (presetShapePath[0] == 11)
               else if presetShapePath[0] == 11
               {
                  // Java: graphicsPath.setFillType(5);
                  graphicsPath.setFillType(5);
               }
            }
            // Java: if (presetShapePath[1] != 12)
            if presetShapePath[1] != 12
            {
               // Java: graphicsPath.setNeedStroke(true);
               graphicsPath.setNeedStroke(true);
            }
            // Java: int len = presetShapePath.length;
            let len = presetShapePath.len();
            // Java: int i = 2;
            let mut i: usize = 2;
            // Java: label166:
            // Java: while(i < len)
            'label166: while i < len
            {
               // Java: pathCommand = presetShapePath[i];
               pathCommand = presetShapePath[i];
               // Java: if (pathCommand == -2)
               if pathCommand == -2
               {
                  // Java: break;
                  break;
               }
               // Java: switch (pathCommand)
               match pathCommand
               {
                  // Java: case -1:
                  -1 => {
                     // Java: ++i;
                     i += 1;
                     // Java: break;
                  }
                  // Java: case 0:
                  0 => {
                     // Java: currX = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                     currX = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                     // Java: currY = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                     currY = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                     // Java: pathElem = PathElem.createMoveElem(currX, currY);
                     let pe = PathElem::createMoveElem(currX, currY);
                     // Java: graphicsPath.addElem(pathElem);
                     graphicsPath.addElem(pe);
                     // Java: i += 3;
                     i += 3;
                     // Java: break;
                  }
                  // Java: case 1:
                  1 => {
                     // Java: currX = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                     currX = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                     // Java: currY = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                     currY = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                     // Java: pathElem = PathElem.createLineElem(currX, currY);
                     let pe = PathElem::createLineElem(currX, currY);
                     // Java: graphicsPath.addElem(pathElem);
                     graphicsPath.addElem(pe);
                     // Java: i += 3;
                     i += 3;
                     // Java: break;
                  }
                  // Java: case 2:
                  2 => {
                     // Java: pathElem = PathElem.createCloseElem();
                     let pe = PathElem::createCloseElem();
                     // Java: graphicsPath.addElem(pathElem);
                     graphicsPath.addElem(pe);
                     // Java: ++i;
                     i += 1;
                     // Java: break;
                  }
                  // Java: case 3:
                  3 => {
                     // Java: float width = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                     let width: f32 = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                     // Java: float height = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                     let height: f32 = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                     // Java: float startAngle = pathData.getValue(presetShapePath[i + 3], gdValueBuf, varList) / 60000.0F;
                     let startAngle: f32 = pathData.getValue(presetShapePath[i + 3], gdValueBuf, varList) / 60000.0_f32;
                     // Java: float sweepAngle = pathData.getValue(presetShapePath[i + 4], gdValueBuf, varList) / 60000.0F;
                     let sweepAngle: f32 = pathData.getValue(presetShapePath[i + 4], gdValueBuf, varList) / 60000.0_f32;
                     // Java: PointF point = arcToCubic(rectF, currX, currY, width, height, startAngle, sweepAngle, graphicsPath);
                     let point = Self::arcToCubic(rectF, currX, currY, width, height, startAngle, sweepAngle, &mut graphicsPath);
                     // Java: if (point != null)
                     if let Some(pt) = point
                     {
                        // Java: currX = point.x;
                        currX = pt.x;
                        // Java: currY = point.y;
                        currY = pt.y;
                     }
                     // Java: i += 5;
                     i += 5;
                     // Java: break;
                  }
                  // Java: case 4:
                  4 => {
                     // Java: float ctrlX = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                     let ctrlX: f32 = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                     // Java: float ctrlY = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                     let ctrlY: f32 = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                     // Java: dstX = pathData.getValue(presetShapePath[i + 3], gdValueBuf, varList);
                     dstX = pathData.getValue(presetShapePath[i + 3], gdValueBuf, varList);
                     // Java: dstY = pathData.getValue(presetShapePath[i + 4], gdValueBuf, varList);
                     dstY = pathData.getValue(presetShapePath[i + 4], gdValueBuf, varList);
                     // Java: addQuad(currX, currY, ctrlX, ctrlY, dstX, dstY, graphicsPath, false);
                     Self::addQuad(currX, currY, ctrlX, ctrlY, dstX, dstY, &mut graphicsPath, false);
                     // Java: currX = dstX;
                     currX = dstX;
                     // Java: currY = dstY;
                     currY = dstY;
                     // Java: i += 5;
                     i += 5;
                     // Java: break;
                  }
                  // Java: case 5:
                  5 => {
                     // Java: float ctrlX1 = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                     let ctrlX1: f32 = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                     // Java: float ctrlY1 = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                     let ctrlY1: f32 = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                     // Java: float ctrlX2 = pathData.getValue(presetShapePath[i + 3], gdValueBuf, varList);
                     let ctrlX2: f32 = pathData.getValue(presetShapePath[i + 3], gdValueBuf, varList);
                     // Java: float ctrlY2 = pathData.getValue(presetShapePath[i + 4], gdValueBuf, varList);
                     let ctrlY2: f32 = pathData.getValue(presetShapePath[i + 4], gdValueBuf, varList);
                     // Java: currX = pathData.getValue(presetShapePath[i + 5], gdValueBuf, varList);
                     currX = pathData.getValue(presetShapePath[i + 5], gdValueBuf, varList);
                     // Java: currY = pathData.getValue(presetShapePath[i + 6], gdValueBuf, varList);
                     currY = pathData.getValue(presetShapePath[i + 6], gdValueBuf, varList);
                     // Java: pathElem = PathElem.createCubicElem(ctrlX1, ctrlY1, ctrlX2, ctrlY2, currX, currY);
                     let pe = PathElem::createCubicElem(ctrlX1, ctrlY1, ctrlX2, ctrlY2, currX, currY);
                     // Java: graphicsPath.addElem(pathElem);
                     graphicsPath.addElem(pe);
                     // Java: i += 7;
                     i += 7;
                     // Java: break;
                  }
                  // Java: case 6:
                  6 => {
                     // Java: boolean isClockWise = false;
                     let mut isClockWise = false;
                     // Java: dstX = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                     dstX = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                     // Java: dstY = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                     dstY = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                     // Java: if (currX > dstX && currY > dstY)
                     if currX > dstX && currY > dstY
                     {
                        // Java: isClockWise = true;
                        isClockWise = true;
                     }
                     // Java: if (currX < dstX && currY < dstY)
                     if currX < dstX && currY < dstY
                     {
                        // Java: isClockWise = true;
                        isClockWise = true;
                     }
                     // Java: while(true)
                     loop
                     {
                        // Java: dstX = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                        dstX = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                        // Java: dstY = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                        dstY = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                        // Java: if (isClockWise)
                        if isClockWise
                        {
                           // Java: qaudrantClockWise(currX, currY, dstX, dstY, graphicsPath, false);
                           Self::qaudrantClockWise(currX, currY, dstX, dstY, &mut graphicsPath, false);
                        }
                        // Java: else
                        else
                        {
                           // Java: qaudrantCounterClockWise(currX, currY, dstX, dstY, graphicsPath, false);
                           Self::qaudrantCounterClockWise(currX, currY, dstX, dstY, &mut graphicsPath, false);
                        }
                        // Java: currX = dstX;
                        currX = dstX;
                        // Java: currY = dstY;
                        currY = dstY;
                        // Java: i += 3;
                        i += 3;
                        // Java: if (i >= len || presetShapePath[i] != 6)
                        if i >= len || presetShapePath[i] != 6
                        {
                           // Java: continue label166;
                           continue 'label166;
                        }
                     }
                  }
                  // Java: case 7:
                  7 => {
                     // Java: isClockWise = false;
                     let mut isClockWise = false;
                     // Java: dstX = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                     dstX = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                     // Java: dstY = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                     dstY = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                     // Java: if (currX > dstX && currY < dstY)
                     if currX > dstX && currY < dstY
                     {
                        // Java: isClockWise = true;
                        isClockWise = true;
                     }
                     // Java: if (currX < dstX && currY > dstY)
                     if currX < dstX && currY > dstY
                     {
                        // Java: isClockWise = true;
                        isClockWise = true;
                     }
                     // Java: while(true)
                     loop
                     {
                        // Java: dstX = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                        dstX = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                        // Java: dstY = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                        dstY = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                        // Java: if (isClockWise)
                        if isClockWise
                        {
                           // Java: qaudrantClockWise(currX, currY, dstX, dstY, graphicsPath, false);
                           Self::qaudrantClockWise(currX, currY, dstX, dstY, &mut graphicsPath, false);
                        }
                        // Java: else
                        else
                        {
                           // Java: qaudrantCounterClockWise(currX, currY, dstX, dstY, graphicsPath, false);
                           Self::qaudrantCounterClockWise(currX, currY, dstX, dstY, &mut graphicsPath, false);
                        }
                        // Java: currX = dstX;
                        currX = dstX;
                        // Java: currY = dstY;
                        currY = dstY;
                        // Java: i += 3;
                        i += 3;
                        // Java: if (i >= len || presetShapePath[i] != 7)
                        if i >= len || presetShapePath[i] != 7
                        {
                           // Java: continue label166;
                           continue 'label166;
                        }
                     }
                  }
                  // Java: case 8:
                  // Java: case 9:
                  // Java: case 10:
                  // Java: case 11:
                  8 | 9 | 10 | 11 => {
                     // Java: float[] values = new float[8];
                     let mut values = vec![0.0_f32; 8];
                     // Java: for(int index = 0; index < values.length; ++index)
                     for index in 0..values.len()
                     {
                        // Java: values[index] = pathData.getValue(presetShapePath[i + 1 + index], gdValueBuf, varList);
                        values[index] = pathData.getValue(presetShapePath[i + 1 + index], gdValueBuf, varList);
                     }
                     // Java: float startX = values[4];
                     let startX = values[4];
                     // Java: float startY = values[5];
                     let startY = values[5];
                     // Java: float endX = values[6];
                     let endX = values[6];
                     // Java: float endY = values[7];
                     let endY = values[7];
                     // Java: float centerX = (values[0] + values[2]) / 2.0F;
                     let centerX = (values[0] + values[2]) / 2.0_f32;
                     // Java: float centerY = (values[1] + values[3]) / 2.0F;
                     let centerY = (values[1] + values[3]) / 2.0_f32;
                     // Java: width = Math.abs(values[0] - values[2]) / 2.0F;
                     let width = (values[0] - values[2]).abs() / 2.0_f32;
                     // Java: height = Math.abs(values[1] - values[3]) / 2.0F;
                     let height = (values[1] - values[3]).abs() / 2.0_f32;
                     // Java: isClockWise = pathCommand == 10 || pathCommand == 11;
                     let isClockWise = pathCommand == 10 || pathCommand == 11;
                     // Java: boolean isArcTo = pathCommand == 9 || pathCommand == 11;
                     let isArcTo = pathCommand == 9 || pathCommand == 11;
                     // Java: escapeArc(startX, startY, endX, endY, centerX, centerY, width, height, isClockWise, isArcTo, graphicsPath, rectF, true, i == 2);
                     Self::escapeArc(startX, startY, endX, endY, centerX, centerY, width, height, isClockWise, isArcTo, &mut graphicsPath, rectF, true, i == 2);
                     // Java: currX = endX;
                     currX = endX;
                     // Java: currY = endY;
                     currY = endY;
                     // Java: i += 9;
                     i += 9;
                     // Java: break;
                  }
                  // Java: case 12:
                  12 => {
                     // Java: float wR = pathData.getValue(presetShapePath[i + 3], gdValueBuf, varList);
                     let wR: f32 = pathData.getValue(presetShapePath[i + 3], gdValueBuf, varList);
                     // Java: float hR = pathData.getValue(presetShapePath[i + 4], gdValueBuf, varList);
                     let hR: f32 = pathData.getValue(presetShapePath[i + 4], gdValueBuf, varList);
                     // Java: if ((double)Math.abs(wR) > 1.0E-6 || (double)Math.abs(hR) > 1.0E-6)
                     if wR.abs() as f64 > 1.0e-6_f64 || hR.abs() as f64 > 1.0e-6_f64
                     {
                        // Java: float cx = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                        let cx: f32 = pathData.getValue(presetShapePath[i + 1], gdValueBuf, varList);
                        // Java: float cy = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                        let cy: f32 = pathData.getValue(presetShapePath[i + 2], gdValueBuf, varList);
                        // Java: startAngle = pathData.getValue(presetShapePath[i + 5], gdValueBuf, varList) / 65536.0F;
                        let startAngle: f32 = pathData.getValue(presetShapePath[i + 5], gdValueBuf, varList) / 65536.0_f32;
                        // Java: sweepAngle = pathData.getValue(presetShapePath[i + 6], gdValueBuf, varList) / 65536.0F;
                        let sweepAngle: f32 = pathData.getValue(presetShapePath[i + 6], gdValueBuf, varList) / 65536.0_f32;
                        // Java: currX = (float)((double)wR * Math.cos((double)(startAngle / 180.0F) * Math.PI) + (double)cx);
                        currX = (wR as f64 * ((startAngle / 180.0_f32) as f64 * std::f64::consts::PI).cos() + cx as f64) as f32;
                        // Java: currY = (float)((double)hR * Math.sin((double)(startAngle / 180.0F) * Math.PI) + (double)cy);
                        currY = (hR as f64 * ((startAngle / 180.0_f32) as f64 * std::f64::consts::PI).sin() + cy as f64) as f32;
                        // Java: pathElem = PathElem.createMoveElem(currX, currY);
                        let pe = PathElem::createMoveElem(currX, currY);
                        // Java: graphicsPath.addElem(pathElem);
                        graphicsPath.addElem(pe);
                        // Java: point = arcToCubic(rectF, currX, currY, wR, hR, startAngle, sweepAngle, graphicsPath);
                        let point = Self::arcToCubic(rectF, currX, currY, wR, hR, startAngle, sweepAngle, &mut graphicsPath);
                        // Java: if (point != null)
                        if let Some(pt) = point
                        {
                           // Java: currX = point.x;
                           currX = pt.x;
                           // Java: currY = point.y;
                           currY = pt.y;
                        }
                     }
                     // Java: i += 7;
                     i += 7;
                  }
                  _ => { i += 1; }
               }
            }
            // Java: gpList[j] = graphicsPath;
            gpList.push(graphicsPath);
         }
         // Java: return gpList;
         return Some(gpList);
      }
    }

    // Java: static void addQuad(float currX, float currY, float ctrlX, float ctrlY, float dstX, float dstY, GraphicsPath path, boolean bCubic)
    pub fn addQuad(currX: f32, currY: f32, ctrlX: f32, ctrlY: f32, dstX: f32, dstY: f32, path: &mut GraphicsPath, bCubic: bool) {
      // Java: if (bCubic)
      if bCubic
      {
         // Java: float t1 = 0.33333334F;
         let t1: f32 = 0.33333334_f32;
         // Java: float subT1 = 1.0F - t1;
         let subT1 = 1.0_f32 - t1;
         // Java: float t12 = t1 * t1;
         let t12 = t1 * t1;
         // Java: float subT12 = subT1 * subT1;
         let subT12 = subT1 * subT1;
         // Java: float t13 = t1 * t1 * t1;
         let t13 = t1 * t1 * t1;
         // Java: float subT13 = subT1 * subT1 * subT1;
         let subT13 = subT1 * subT1 * subT1;
         // Java: float p2 = 2.0F * t1 * (1.0F - t1);
         let mut p2 = 2.0_f32 * t1 * (1.0_f32 - t1);
         // Java: float x1 = subT12 * currX + p2 * ctrlX + t12 * dstX;
         let x1 = subT12 * currX + p2 * ctrlX + t12 * dstX;
         // Java: float y1 = subT12 * currY + p2 * ctrlY + t12 * dstY;
         let y1 = subT12 * currY + p2 * ctrlY + t12 * dstY;
         // Java: float t2 = 0.6666667F;
         let t2: f32 = 0.6666667_f32;
         // Java: float subT2 = 1.0F - t2;
         let subT2 = 1.0_f32 - t2;
         // Java: float t22 = t2 * t2;
         let t22 = t2 * t2;
         // Java: float subT22 = subT2 * subT2;
         let subT22 = subT2 * subT2;
         // Java: float t23 = t2 * t2 * t2;
         let t23 = t2 * t2 * t2;
         // Java: float subT23 = subT2 * subT2 * subT2;
         let subT23 = subT2 * subT2 * subT2;
         // Java: p2 = 2.0F * t2 * (1.0F - t2);
         p2 = 2.0_f32 * t2 * (1.0_f32 - t2);
         // Java: float x2 = subT22 * currX + p2 * ctrlX + t22 * dstX;
         let x2 = subT22 * currX + p2 * ctrlX + t22 * dstX;
         // Java: float y2 = subT22 * currY + p2 * ctrlY + t22 * dstY;
         let y2 = subT22 * currY + p2 * ctrlY + t22 * dstY;
         // Java: float a1 = 3.0F * t1 * subT12;
         let a1 = 3.0_f32 * t1 * subT12;
         // Java: float b1 = 3.0F * t12 * subT1;
         let b1 = 3.0_f32 * t12 * subT1;
         // Java: float c1 = currX * subT13 + dstX * t13 - x1;
         let mut c1 = currX * subT13 + dstX * t13 - x1;
         // Java: float a2 = 3.0F * t2 * subT22;
         let a2 = 3.0_f32 * t2 * subT22;
         // Java: float b2 = 3.0F * t22 * subT2;
         let b2 = 3.0_f32 * t22 * subT2;
         // Java: float c2 = currX * subT23 + dstX * t23 - x2;
         let mut c2 = currX * subT23 + dstX * t23 - x2;
         // Java: float c1X = (b1 * c2 - b2 * c1) / (a1 * b2 - b1 * a2);
         let c1X = (b1 * c2 - b2 * c1) / (a1 * b2 - b1 * a2);
         // Java: float c2X = (a1 * c2 - a2 * c1) / (b1 * a2 - a1 * b2);
         let c2X = (a1 * c2 - a2 * c1) / (b1 * a2 - a1 * b2);
         // Java: c1 = currY * subT13 + dstY * t13 - y1;
         c1 = currY * subT13 + dstY * t13 - y1;
         // Java: c2 = currY * subT23 + dstY * t23 - y2;
         c2 = currY * subT23 + dstY * t23 - y2;
         // Java: float c1Y = (b1 * c2 - b2 * c1) / (a1 * b2 - b1 * a2);
         let c1Y = (b1 * c2 - b2 * c1) / (a1 * b2 - b1 * a2);
         // Java: float c2Y = (a1 * c2 - a2 * c1) / (b1 * a2 - a1 * b2);
         let c2Y = (a1 * c2 - a2 * c1) / (b1 * a2 - a1 * b2);
         // Java: PathElem pathElem = PathElem.createCubicElem(c1X, c1Y, c2X, c2Y, dstX, dstY);
         let pathElem = PathElem::createCubicElem(c1X, c1Y, c2X, c2Y, dstX, dstY);
         // Java: path.addElem(pathElem);
         path.addElem(pathElem);
      }
      // Java: else
      else
      {
         // Java: PathElem pathElem = PathElem.createQuadElem(ctrlX, ctrlY, dstX, dstY);
         let pathElem = PathElem::createQuadElem(ctrlX, ctrlY, dstX, dstY);
         // Java: path.addElem(pathElem);
         path.addElem(pathElem);
      }
    }

    // Java: static void addArcToPath(RectF rectF, float x, float y, float wR, float hR, float stAng, float swAng, GraphicsPath path, boolean bCubic)
    pub fn addArcToPath(rectF: Option<&RectF>, x: f32, y: f32, wR: f32, hR: f32, stAng: f32, swAng: f32, path: &mut GraphicsPath, bCubic: bool) {
      // Java: if (bCubic)
      if bCubic
      {
         // Java: arcToCubic(rectF, x, y, wR, hR, stAng, swAng, path);
         Self::arcToCubic(rectF, x, y, wR, hR, stAng, swAng, path);
      }
      // Java: else
      else
      {
         // Java: path.addElem(PathElem.createArcElem(wR, hR, stAng, swAng));
         path.addElem(PathElem::createArcElem(wR, hR, stAng, swAng));
      }
    }

    // Java: static PointF arcToCubic(RectF rectF, float x, float y, float wR, float hR, float stAng, float swAng, GraphicsPath path)
    pub fn arcToCubic(rectF: Option<&RectF>, x: f32, y: f32, wR: f32, hR: f32, stAng: f32, swAng: f32, path: &mut GraphicsPath) -> Option<PointF> {
      // Java: stAng = (360.0F - stAng) % 360.0F;
      let mut stAng = (360.0_f32 - stAng) % 360.0_f32;
      // Java: swAng = -swAng;
      let swAng = -swAng;
      // Java: float endAngleDeg = stAng + swAng;
      let endAngleDeg = stAng + swAng;
      // Java: double stAngleRad = (double)stAng * Math.PI / (double)180.0F;
      let stAngleRad = stAng as f64 * std::f64::consts::PI / 180.0_f64;
      // Java: double endAngleRad = (double)endAngleDeg * Math.PI / (double)180.0F;
      let endAngleRad = endAngleDeg as f64 * std::f64::consts::PI / 180.0_f64;
      // Java: float wR0 = wR;
      let mut wR0 = wR;
      // Java: float hR0 = hR;
      let mut hR0 = hR;
      // Java: if (rectF != null)
      if let Some(rect) = rectF
      {
         // Java: wR0 = wR / rectF.width();
         wR0 = wR / rect.width();
         // Java: hR0 = hR / rectF.height();
         hR0 = hR / rect.height();
      }
      // Java: double chgStAngleRad = Math.atan2(Math.sin(stAngleRad) * (double)wR0, Math.cos(stAngleRad) * (double)hR0);
      let mut chgStAngleRad = f64::atan2(stAngleRad.sin() * wR0 as f64, stAngleRad.cos() * hR0 as f64);
      // Java: double chgEndAngleRad = Math.atan2(Math.sin(endAngleRad) * (double)wR0, Math.cos(endAngleRad) * (double)hR0);
      let mut chgEndAngleRad = f64::atan2(endAngleRad.sin() * wR0 as f64, endAngleRad.cos() * hR0 as f64);
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
      // Java: PointF point = null;
      let mut point: Option<PointF> = None;
      // Java: int segs = (int)Math.ceil(Math.abs(swingAngleRad) * (double)2.0F / Math.PI);
      let segs = (swingAngleRad.abs() * 2.0_f64 / std::f64::consts::PI).ceil() as i32;
      // Java: if (segs > 0)
      if segs > 0
      {
         // Java: double theta = -swingAngleRad / (double)segs;
         let theta = -swingAngleRad / segs as f64;
         // Java: double x0 = Math.cos(theta / (double)2.0F);
         let x0 = (theta / 2.0_f64).cos();
         // Java: double y0 = Math.sin(theta / (double)2.0F);
         let y0 = (theta / 2.0_f64).sin();
         // Java: double angle = -chgStAngleRad;
         let mut angle = -chgStAngleRad;
         // Java: double cx = (double)x - Math.cos(angle) * (double)wR;
         let cx = x as f64 - angle.cos() * wR as f64;
         // Java: double cy = (double)y - Math.sin(angle) * (double)hR;
         let cy = y as f64 - angle.sin() * hR as f64;
         // Java: for(int i = 0; i < segs; ++i)
         for i in 0..segs
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
            let ax = (cx + wR as f64 * (x0 * cs - y0 * sn)) as f32;
            // Java: float ay = (float)(cy + (double)hR * (x0 * sn + y0 * cs));
            let ay = (cy + hR as f64 * (x0 * sn + y0 * cs)) as f32;
            // Java: float cx1 = (float)(cx + (double)wR * ((x0 + tx) * cs + ty * sn));
            let cx1 = (cx + wR as f64 * ((x0 + tx) * cs + ty * sn)) as f32;
            // Java: float cy1 = (float)(cy + (double)hR * ((x0 + tx) * sn - ty * cs));
            let cy1 = (cy + hR as f64 * ((x0 + tx) * sn - ty * cs)) as f32;
            // Java: float cx2 = (float)(cx + (double)wR * ((x0 + tx) * cs - ty * sn));
            let cx2 = (cx + wR as f64 * ((x0 + tx) * cs - ty * sn)) as f32;
            // Java: float cy2 = (float)(cy + (double)hR * ((x0 + tx) * sn + ty * cs));
            let cy2 = (cy + hR as f64 * ((x0 + tx) * sn + ty * cs)) as f32;
            // Java: PathElem elem = PathElem.createCubicElem(cx1, cy1, cx2, cy2, ax, ay);
            let elem = PathElem::createCubicElem(cx1, cy1, cx2, cy2, ax, ay);
            // Java: if (path != null)
            {
               // Java: path.addElem(elem);
               path.addElem(elem);
            }
            // Java: if (i == segs - 1)
            if i == segs - 1
            {
               // Java: point = new PointF(ax, ay);
               point = Some(PointF::new_xy(ax, ay));
            }
         }
      }
      // Java: return point;
      point
    }

    // Java: static void escapeArc(float startX, float startY, float endX, float endY, float centerX, float centerY, float width, float height, boolean isCloseWise, boolean isArcTo, GraphicsPath path, RectF rectF, boolean bCubic, boolean isFirst)
    pub fn escapeArc(startX: f32, startY: f32, endX: f32, endY: f32, centerX: f32, centerY: f32, width: f32, height: f32, isCloseWise: bool, isArcTo: bool, path: &mut GraphicsPath, rectF: Option<&RectF>, bCubic: bool, isFirst: bool) {
      // Java: float startAngle = (float)(Math.atan2((double)((startY - centerY) / height), (double)((startX - centerX) / width)) * (double)180.0F / Math.PI);
      let startAngle = (((startY - centerY) / height) as f64).atan2(((startX - centerX) / width) as f64) as f32 * 180.0_f32 / std::f32::consts::PI;
      // Java: float sweepAngle = (float)(Math.atan2((double)((endY - centerY) / height), (double)((endX - centerX) / width)) * (double)180.0F / Math.PI - (double)startAngle);
      let mut sweepAngle = (((endY - centerY) / height) as f64).atan2(((endX - centerX) / width) as f64) as f32 * 180.0_f32 / std::f32::consts::PI - startAngle;
      // Java: if (Math.abs(sweepAngle) > 180.0F)
      if sweepAngle.abs() > 180.0_f32
      {
         // Java: if (sweepAngle < 0.0F)
         if sweepAngle < 0.0_f32
         {
            // Java: sweepAngle += 360.0F;
            sweepAngle += 360.0_f32;
         }
         // Java: else
         else
         {
            // Java: sweepAngle -= 360.0F;
            sweepAngle -= 360.0_f32;
         }
      }
      // Java: if (isCloseWise)
      if isCloseWise
      {
         // Java: if (sweepAngle < 0.0F)
         if sweepAngle < 0.0_f32
         {
            // Java: sweepAngle += 360.0F;
            sweepAngle += 360.0_f32;
         }
      }
      // Java: else if (sweepAngle > 0.0F)
      else if sweepAngle > 0.0_f32
      {
         // Java: sweepAngle -= 360.0F;
         sweepAngle -= 360.0_f32;
      }
      // Java: if (isFirst)
      if isFirst
      {
         // Java: PathElem pathElem = PathElem.createMoveElem(startX, startY);
         let pathElem = PathElem::createMoveElem(startX, startY);
         // Java: path.addElem(pathElem);
         path.addElem(pathElem);
      }
      // Java: else
      else
      {
         // Java: PathElem pathElem = PathElem.createLineElem(startX, startY);
         let pathElem = PathElem::createLineElem(startX, startY);
         // Java: path.addElem(pathElem);
         path.addElem(pathElem);
      }
      // Java: addArcToPath(rectF, startX, startY, width, height, startAngle, sweepAngle, path, bCubic);
      Self::addArcToPath(rectF, startX, startY, width, height, startAngle, sweepAngle, path, bCubic);
    }

    // Java: static void qaudrantClockWise(float beginX, float beginY, float endX, float endY, GraphicsPath path, boolean bCubic)
    pub fn qaudrantClockWise(beginX: f32, beginY: f32, endX: f32, endY: f32, path: &mut GraphicsPath, bCubic: bool) {
      // Java: float width = Math.abs(beginX - endX);
      let width = (beginX - endX).abs();
      // Java: float height = Math.abs(beginY - endY);
      let height = (beginY - endY).abs();
      // Java: float startAngle = 0.0F;
      let mut startAngle: f32 = 0.0_f32;
      // Java: float sweepAngle = 90.0F;
      let sweepAngle: f32 = 90.0_f32;
      // Java: if (beginX < endX)
      if beginX < endX
      {
         // Java: if (beginY < endY)
         if beginY < endY
         {
            // Java: startAngle = 270.0F;
            startAngle = 270.0_f32;
         }
         // Java: else
         else
         {
            // Java: startAngle = 180.0F;
            startAngle = 180.0_f32;
         }
      }
      // Java: else if (beginX > endX)
      else if beginX > endX
      {
         // Java: if (beginY < endY)
         if beginY < endY
         {
            // Java: startAngle = 0.0F;
            startAngle = 0.0_f32;
         }
         // Java: else
         else
         {
            // Java: startAngle = 90.0F;
            startAngle = 90.0_f32;
         }
      }
      // Java: if (beginX != endX)
      if beginX != endX
      {
         // Java: addArcToPath((RectF)null, beginX, beginY, width, height, startAngle, sweepAngle, path, bCubic);
         Self::addArcToPath(None, beginX, beginY, width, height, startAngle, sweepAngle, path, bCubic);
      }
    }

    // Java: static void qaudrantCounterClockWise(float beginX, float beginY, float endX, float endY, GraphicsPath path, boolean bCubic)
    pub fn qaudrantCounterClockWise(beginX: f32, beginY: f32, endX: f32, endY: f32, path: &mut GraphicsPath, bCubic: bool) {
      // Java: float width = Math.abs(beginX - endX);
      let width = (beginX - endX).abs();
      // Java: float height = Math.abs(beginY - endY);
      let height = (beginY - endY).abs();
      // Java: float startAngle = 0.0F;
      let mut startAngle: f32 = 0.0_f32;
      // Java: float sweepAngle = -90.0F;
      let sweepAngle: f32 = -90.0_f32;
      // Java: if (beginX < endX)
      if beginX < endX
      {
         // Java: if (beginY < endY)
         if beginY < endY
         {
            // Java: startAngle = 180.0F;
            startAngle = 180.0_f32;
         }
         // Java: else
         else
         {
            // Java: startAngle = 90.0F;
            startAngle = 90.0_f32;
         }
      }
      // Java: else if (beginX > endX)
      else if beginX > endX
      {
         // Java: if (beginY < endY)
         if beginY < endY
         {
            // Java: startAngle = 270.0F;
            startAngle = 270.0_f32;
         }
         // Java: else
         else
         {
            // Java: startAngle = 0.0F;
            startAngle = 0.0_f32;
         }
      }
      // Java: if (beginX != endX)
      if beginX != endX
      {
         // Java: addArcToPath((RectF)null, beginX, beginY, width, height, startAngle, sweepAngle, path, bCubic);
         Self::addArcToPath(None, beginX, beginY, width, height, startAngle, sweepAngle, path, bCubic);
      }
    }

}
