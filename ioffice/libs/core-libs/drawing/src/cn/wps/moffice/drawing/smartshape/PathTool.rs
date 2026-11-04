#![allow(non_snake_case, dead_code, unused_variables, unused_mut, unused_assignments)]

// Java: import cn.wps.graphics.PointF
use core_libs_util::graphics::PointF::PointF;
// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.graphics.GraphicsPath
use crate::cn::wps::moffice::drawing::graphics::GraphicsPath::GraphicsPath;
// Java: import cn.wps.moffice.drawing.graphics.PathElem
use crate::cn::wps::moffice::drawing::graphics::PathElem::PathElem;
// Java: import java.util.ArrayList

use super::Segment::Segment;
use super::Geometry::Geometry;
use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class PathTool
pub struct PathTool {
}

// Java: public class PathTool
impl PathTool {
    // Java: PathTool()
    pub fn new() -> Self {
      // Java: super();
      PathTool {}
    }

    // Java: static Geometry convertToCubic(int shapeType, RectF shapeRect, int[] vertices, Segment[] segments)
    pub fn convertToCubic(shapeType: i32, shapeRect: &RectF, vertices: &[i32], segments: &[Segment]) -> Option<Geometry> {
      // Java: if (vertices != null && segments != null)
      {
         // Java: RectF rectF = shapeType != 106 && shapeType != 225 ? null : shapeRect;
         let rectF: Option<&RectF> = if shapeType != 106 && shapeType != 225 { None } else { Some(shapeRect) };
         // Java: GraphicsPath graphicsPath = new GraphicsPath();
         let mut graphicsPath = GraphicsPath::new();
         // Java: float currX = 0.0F;
         let mut currX: f32 = 0.0_f32;
         // Java: float currY = 0.0F;
         let mut currY: f32 = 0.0_f32;
         // Java: float dstX = 0.0F;
         let mut dstX: f32 = 0.0_f32;
         // Java: float dstY = 0.0F;
         let mut dstY: f32 = 0.0_f32;
         // Java: int index = 0;
         let mut index: usize = 0;
         // Java: boolean clockWise = false;
         let mut clockWise: bool = false;
         // Java: int size = segments.length;
         let size = segments.len();
         // Java: ArrayList<Integer> verArr = new ArrayList();
         let mut verArr: Vec<i32> = Vec::new();
         // Java: ArrayList<Segment> segArr = new ArrayList();
         let mut segArr: Vec<Segment> = Vec::new();
         // Java: try
         {
            // Java: int lastSegCmd = -1;
            let mut lastSegCmd: i32 = -1;
            // Java: for(int i = 0; i < segments.length; ++i)
            let mut i = 0;
            while i < segments.len()
            {
               // Java: Segment seg = segments[i];
               let seg = &segments[i];
               // Java: int verticeIndex = segments[i].start;
               let mut verticeIndex = segments[i].start as usize;
               // Java: int endIndex = i == size - 1 ? vertices.length : segments[i + 1].start;
               let endIndex: usize = if i == size - 1 { vertices.len() } else { segments[i + 1].start as usize };
               // Java: int parameterCount = 0;
               let mut parameterCount: usize = 0;
               // Java: while(true)
               loop
               {
                  // Java: switch (seg.cmd)
                  match seg.cmd as i32
                  {
                     // Java: case 0:
                     // Java: case 64:
                     0 | 64 => {
                        // Java: currX = (float)vertices[index++];
                        currX = vertices[index] as f32; index += 1;
                        // Java: currY = (float)vertices[index++];
                        currY = vertices[index] as f32; index += 1;
                        // Java: segArr.add(new Segment(seg.cmd, verArr.size()));
                        segArr.push(Segment::with_params(seg.cmd, verArr.len() as i32));
                        // Java: verArr.add((int)currX);
                        verArr.push(currX as i32);
                        // Java: verArr.add((int)currY);
                        verArr.push(currY as i32);
                        // Java: parameterCount = 2;
                        parameterCount = 2;
                        // Java: break;
                     }
                     // Java: case 32:
                     32 => {
                        // Java: segArr.add(new Segment(seg.cmd, verArr.size()));
                        segArr.push(Segment::with_params(seg.cmd, verArr.len() as i32));
                        // Java: verArr.add(vertices[index++]);
                        verArr.push(vertices[index]); index += 1;
                        // Java: verArr.add(vertices[index++]);
                        verArr.push(vertices[index]); index += 1;
                        // Java: verArr.add(vertices[index++]);
                        verArr.push(vertices[index]); index += 1;
                        // Java: verArr.add(vertices[index++]);
                        verArr.push(vertices[index]); index += 1;
                        // Java: currX = (float)vertices[index++];
                        currX = vertices[index] as f32; index += 1;
                        // Java: currY = (float)vertices[index++];
                        currY = vertices[index] as f32; index += 1;
                        // Java: verArr.add((int)currX);
                        verArr.push(currX as i32);
                        // Java: verArr.add((int)currY);
                        verArr.push(currY as i32);
                        // Java: parameterCount = 6;
                        parameterCount = 6;
                        // Java: break;
                     }
                     // Java: case 163:
                     // Java: case 164:
                     // Java: case 165:
                     // Java: case 166:
                     163 | 164 | 165 | 166 => {
                        // Java: if ((seg.cmd == 164 || seg.cmd == 163) && endIndex - verticeIndex == 4)
                        if (seg.cmd == 164 || seg.cmd == 163) && endIndex - verticeIndex == 4
                        {
                           // Java: float width = (float)vertices[index++];
                           let width = vertices[index] as f32; index += 1;
                           // Java: float height = (float)vertices[index++];
                           let height = vertices[index] as f32; index += 1;
                           // Java: float startAngle = (float)vertices[index++];
                           let mut startAngle = vertices[index] as f32; index += 1;
                           // Java: float sweepAngle = (float)vertices[index++];
                           let mut sweepAngle = vertices[index] as f32; index += 1;
                           // Java: if (seg.cmd == 163)
                           if seg.cmd == 163
                           {
                              // Java: startAngle /= 60000.0F;
                              startAngle /= 60000.0_f32;
                              // Java: sweepAngle /= 60000.0F;
                              sweepAngle /= 60000.0_f32;
                           }
                           // Java: graphicsPath.reset();
                           graphicsPath.reset();
                           // Java: PointF point = CCL.arcToCubic(rectF, currX, currY, width, height, startAngle, sweepAngle, graphicsPath);
                           // TODO: CCL.arcToCubic not yet available
                           let point: Option<PointF> = None;
                           // Java: if (point != null)
                           if let Some(ref pt) = point
                           {
                              // Java: currX = point.x;
                              currX = pt.x;
                              // Java: currY = point.y;
                              currY = pt.y;
                              // Java: addPath(graphicsPath, verArr, segArr);
                              PathTool::addPath(&graphicsPath, &mut verArr, &mut segArr);
                           }
                           // Java: parameterCount = 4;
                           parameterCount = 4;
                        }
                        // Java: else
                        else
                        {
                           // Java: float[] values = new float[8];
                           let mut values = [0.0_f32; 8];
                           // Java: for(int j = 0; j < values.length; ++j)
                           for j in 0..values.len()
                           {
                              // Java: values[j] = (float)vertices[index++];
                              values[j] = vertices[index] as f32; index += 1;
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
                           // Java: float width = Math.abs(values[0] - values[2]) / 2.0F;
                           let width = (values[0] - values[2]).abs() / 2.0_f32;
                           // Java: float height = Math.abs(values[1] - values[3]) / 2.0F;
                           let height = (values[1] - values[3]).abs() / 2.0_f32;
                           // Java: boolean isClockWise = seg.cmd == 166 || seg.cmd == 165;
                           let isClockWise = seg.cmd == 166 || seg.cmd == 165;
                           // Java: boolean isArcTo = seg.cmd == 165 || seg.cmd == 163;
                           let isArcTo = seg.cmd == 165 || seg.cmd == 163;
                           // Java: graphicsPath.reset();
                           graphicsPath.reset();
                           // Java: CCL.escapeArc(startX, startY, endX, endY, centerX, centerY, width, height, isClockWise, isArcTo, graphicsPath, rectF, true, lastSegCmd == -1 || lastSegCmd == 128);
                           // TODO: CCL.escapeArc not yet available
                           // Java: addPath(graphicsPath, verArr, segArr);
                           PathTool::addPath(&graphicsPath, &mut verArr, &mut segArr);
                           // Java: currX = endX;
                           currX = endX;
                           // Java: currY = endY;
                           currY = endY;
                           // Java: parameterCount = 8;
                           parameterCount = 8;
                        }
                        // Java: break;
                     }
                     // Java: case 167:
                     // Java: case 168:
                     167 | 168 => {
                        // Java: boolean bx = seg.cmd == 167;
                        let bx = seg.cmd == 167;
                        // Java: if (lastSegCmd != seg.cmd)
                        if lastSegCmd != seg.cmd as i32
                        {
                           // Java: clockWise = false;
                           clockWise = false;
                           // Java: dstX = (float)vertices[index];
                           dstX = vertices[index] as f32;
                           // Java: dstY = (float)vertices[index + 1];
                           dstY = vertices[index + 1] as f32;
                           // Java: if (bx)
                           if bx
                           {
                              // Java: if (currX > dstX && currY > dstY)
                              if currX > dstX && currY > dstY
                              {
                                 // Java: clockWise = true;
                                 clockWise = true;
                              }
                              // Java: if (currX < dstX && currY < dstY)
                              if currX < dstX && currY < dstY
                              {
                                 // Java: clockWise = true;
                                 clockWise = true;
                              }
                           }
                           // Java: else
                           else
                           {
                              // Java: if (currX > dstX && currY < dstY)
                              if currX > dstX && currY < dstY
                              {
                                 // Java: clockWise = true;
                                 clockWise = true;
                              }
                              // Java: if (currX < dstX && currY > dstY)
                              if currX < dstX && currY > dstY
                              {
                                 // Java: clockWise = true;
                                 clockWise = true;
                              }
                           }
                        }
                        // Java: graphicsPath.reset();
                        graphicsPath.reset();
                        // Java: do
                        loop
                        {
                           // Java: dstX = (float)vertices[index++];
                           dstX = vertices[index] as f32; index += 1;
                           // Java: dstY = (float)vertices[index++];
                           dstY = vertices[index] as f32; index += 1;
                           // Java: if (clockWise)
                           if clockWise
                           {
                              // Java: CCL.qaudrantClockWise(currX, currY, dstX, dstY, graphicsPath, true);
                              // TODO: CCL.qaudrantClockWise not yet available
                           }
                           // Java: else
                           else
                           {
                              // Java: CCL.qaudrantCounterClockWise(currX, currY, dstX, dstY, graphicsPath, true);
                              // TODO: CCL.qaudrantCounterClockWise not yet available
                           }
                           // Java: currX = dstX;
                           currX = dstX;
                           // Java: currY = dstY;
                           currY = dstY;
                           // Java: ++i;
                           i += 1;
                           if i >= segments.len() || segments[i].cmd != seg.cmd { break; }
                        }
                        // Java: addPath(graphicsPath, verArr, segArr);
                        PathTool::addPath(&graphicsPath, &mut verArr, &mut segArr);
                        // Java: --i;
                        i -= 1;
                        // Java: parameterCount = 2;
                        parameterCount = 2;
                        // Java: break;
                     }
                     // Java: case 169:
                     169 => {
                        // Java: float ctrlX = (float)vertices[index++];
                        let ctrlX = vertices[index] as f32; index += 1;
                        // Java: float ctrlY = (float)vertices[index++];
                        let ctrlY = vertices[index] as f32; index += 1;
                        // Java: dstX = (float)vertices[index++];
                        dstX = vertices[index] as f32; index += 1;
                        // Java: dstY = (float)vertices[index++];
                        dstY = vertices[index] as f32; index += 1;
                        // Java: graphicsPath.reset();
                        graphicsPath.reset();
                        // Java: CCL.addQuad(currX, currY, ctrlX, ctrlY, dstX, dstY, graphicsPath, true);
                        // TODO: CCL.addQuad not yet available
                        // Java: addPath(graphicsPath, verArr, segArr);
                        PathTool::addPath(&graphicsPath, &mut verArr, &mut segArr);
                        // Java: currX = dstX;
                        currX = dstX;
                        // Java: currY = dstY;
                        currY = dstY;
                        // Java: parameterCount = 4;
                        parameterCount = 4;
                        // Java: break;
                     }
                     // Java: default:
                     _ => {
                        // Java: segArr.add(new Segment(seg.cmd, verArr.size()));
                        segArr.push(Segment::with_params(seg.cmd, verArr.len() as i32));
                        // Java: parameterCount = endIndex - verticeIndex;
                        parameterCount = endIndex - verticeIndex;
                        // Java: for(int j = 0; j < parameterCount; ++j)
                        for j in 0..parameterCount
                        {
                           // Java: verArr.add(vertices[index++]);
                           verArr.push(vertices[index]); index += 1;
                        }
                     }
                  }
                  // Java: lastSegCmd = seg.cmd;
                  lastSegCmd = seg.cmd as i32;
                  // Java: verticeIndex += parameterCount;
                  verticeIndex += parameterCount;
                  // Java: if (verticeIndex >= endIndex || parameterCount == 0)
                  if verticeIndex >= endIndex || parameterCount == 0
                  {
                     // Java: break;
                     break;
                  }
               }
               i += 1;
            }
         }
         // Java: catch (Exception e)
         // Rust: error handling handled by bounds checks above
         {
            // Java: e.printStackTrace();
         }
         // Java: size = verArr.size();
         let size = verArr.len();
         // Java: int[] newVertics = new int[size];
         let mut newVertics: Vec<i32> = Vec::with_capacity(size);
         // Java: for(int i = 0; i < size; ++i)
         for i in 0..size
         {
            // Java: newVertics[i] = (Integer)verArr.get(i);
            newVertics.push(verArr[i]);
         }
         // Java: Segment[] newSegs = new Segment[segArr.size()];
         // Java: segArr.toArray(newSegs);
         let newSegs = segArr;
         // Java: Geometry geometry = new Geometry();
         let mut geometry = Geometry::new();
         // Java: geometry.setVertices(newVertics);
         geometry.setVertices(Some(Arc::new(newVertics)));
         // Java: geometry.setSegment(newSegs);
         geometry.setSegment(Some(Arc::new(newSegs)));
         // Java: return geometry;
         return Some(geometry);
      }
      // Java: else
      {
         // Java: return null;
         None
      }
    }

    // Java: static void addPath(GraphicsPath path, ArrayList<Integer> verArr, ArrayList<Segment> segArr)
    pub fn addPath(path: &GraphicsPath, verArr: &mut Vec<i32>, segArr: &mut Vec<Segment>) {
      // Java: if (path != null)
      {
         // Java: int size = path.size();
         let size = path.size() as usize;
         // Java: for(int i = 0; i < size; ++i)
         for i in 0..size
         {
            // Java: PathElem elem = path.getElem(i);
            let elem = path.getElem(i as i32);
            // Java: if (elem != null)
            {
               // Java: short cmd = 32;
               let mut cmd: i16 = 32;
               // Java: if (elem.elemType == 0)
               if elem.elemType == 0
               {
                  // Java: cmd = 64;
                  cmd = 64;
               }
               // Java: else if (elem.elemType == 1)
               else if elem.elemType == 1
               {
                  // Java: cmd = 0;
                  cmd = 0;
               }
               // Java: segArr.add(new Segment(cmd, verArr.size()));
               segArr.push(Segment::with_params(cmd, verArr.len() as i32));
               // Java: for(int j = 0; j < elem.pos.length; ++j)
               for j in 0..elem.pos.len()
               {
                  // Java: verArr.add((int)elem.pos[j]);
                  verArr.push(elem.pos[j] as i32);
               }
            }
         }
      }
    }

}
