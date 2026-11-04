#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables, unused_mut, unused_assignments)]

use std::sync::Arc;
use std::any::Any;
use std::collections::HashMap;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;
// Java: import cn.wps.moffice.drawing.graphics.GraphicsPath
// TODO: GraphicsPath crate not yet available, using Element
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.Map

use super::PathData::PathData;
use super::GeoFormula::GeoFormula;
use super::Segment::Segment;
use super::Cmd::Cmd;
use super::CFL;

// Java: public class CustomPathData extends PathData
pub struct CustomPathData {
    pub(crate) base: PathData,
    // Java field: GeoFormula[] _formulas
    _formulas: Option<Vec<GeoFormula>>,
    // Java field: RectF _cordSize
    _cordSize: Option<RectF>,
    // Java field: int[][] _pathList
    pub(crate) _pathList: Option<Vec<Vec<i32>>>,
    // Java field: ShapePath[] _shapePaths
    _shapePaths: Option<Vec<Element>>,
}

// Java: public class CustomPathData extends PathData
impl CustomPathData {
    // Java: CustomPathData(Geometry geometry, int type)
    pub fn new(geometry: &Element, r#type: i32) -> Self {
      // Java: super();
      let mut this = CustomPathData {
          base: PathData::new(),
          _formulas: None,
          _cordSize: None,
          _pathList: None,
          _shapePaths: None,
      };
      // Java: this.setType(type);
      this.base.setType(r#type);
      // Java: try
      {
         // Java: this.parsePreDefData(geometry);
         this.parsePreDefData(geometry);
      }
      // Java: catch (Exception var4)
      // Rust doesn't have try-catch; parsePreDefData is stubbed
      {
         // Java: this._formulas = null;
         // Java: this._cordSize = null;
         // Java: this._pathList = null;
         // Java: this._shapePaths = null;
      }
      this
    }

    // Java: GraphicsPath[] calcShapePath()
    pub fn calcShapePath(&mut self) -> Option<Vec<Element>> {
      // Java: Map<Integer, Float> gdValueBuf = new HashMap();
      let mut gdValueBuf: HashMap<i32, f32> = HashMap::new();
      // Java: GraphicsPath[] paths = CCL.generatePath(this, gdValueBuf, this._varList);
      // TODO: CCL and GraphicsPath types not yet available
      let paths: Option<Vec<Element>> = None;
      // Java: if (paths != null)
      if let Some(ref paths_vec) = paths
      {
         // Java: int size = paths.length;
         let size = paths_vec.len();
         // Java: float scaleX = 1.0F;
         let mut scaleX: f32 = 1.0_f32;
         // Java: float scaleY = 1.0F;
         let mut scaleY: f32 = 1.0_f32;
         // Java: if (Float.compare(this._cordSize.width(), 0.0F) != 0)
         if let Some(ref cord_size) = self._cordSize {
            if cord_size.width() != 0.0_f32
            {
               // Java: scaleX = this._rect.width() / this._cordSize.width();
               if let Some(ref rect) = self.base._rect {
                  scaleX = rect.width() / cord_size.width();
               }
            }
            // Java: if (Float.compare(this._cordSize.height(), 0.0F) != 0)
            if cord_size.height() != 0.0_f32
            {
               // Java: scaleY = this._rect.height() / this._cordSize.height();
               if let Some(ref rect) = self.base._rect {
                  scaleY = rect.height() / cord_size.height();
               }
            }
            // Java: for(int i = 0; i < size; ++i)
            for i in 0..size
            {
               // Java: GraphicsPath path = paths[i];
               // TODO: GraphicsPath type not yet available
               // Java: if (path.hasClosedPath())
               // Java: path.setFillType2(1);
               // Java: path.adjust(-this._cordSize.left, -this._cordSize.top, scaleX, scaleY);
            }
         }
         // Java: if (this._shapePaths != null && this._shapePaths.length == size)
         if let Some(ref shape_paths) = self._shapePaths {
            if shape_paths.len() == size
            {
               // Java: for(int i = 0; i < size; ++i)
               for i in 0..size
               {
                  // Java: paths[i].setStrokeAndFill(this._shapePaths[i].lineProperty, this._shapePaths[i].fill);
                  // TODO: ShapePath type not yet available
               }
            }
         }
      }
      // Java: return paths;
      paths
    }

    // Java: void parsePreDefData(Geometry geometry)
    pub fn parsePreDefData(&mut self, geometry: &Element) {
      // Java: if (geometry != null)
      if geometry.is_some()
      {
         // Java: int[] initVertices = geometry.getVertices();
         // TODO: Geometry type not yet available, stub
         let initVertices: Option<Vec<i32>> = None;
         // Java: if (initVertices != null && initVertices.length >= 1)
         if let Some(ref init_v) = initVertices {
            if init_v.len() >= 1
            {
               // Java: Segment[] segments = geometry.getSegment();
               // TODO: Geometry type
               let segments: Option<Vec<Segment>> = None;
               // Java: GeoAdjust adjust = geometry.getGeoAdjust();
               // TODO: Geometry type
               let adjust: Element = None;
               // Java: this._formulas = geometry.getFormulas();
               // TODO: Geometry type
               self._formulas = None;
               // Java: this._cordSize = geometry.getGeoCordSize();
               // TODO: Geometry type
               self._cordSize = None;
               // Java: int[] vertices = initVertices;
               let mut vertices = init_v.clone();
               // Java: if (adjust != null && adjust.getSize() > 0)
               if adjust.is_some()
               {
                  // Java: int adjustSize = 10;
                  let adjustSize: usize = 10;
                  // Java: this._varList = new int[adjustSize];
                  self.base._varList = vec![0i32; adjustSize];
                  // Java: for(int i = 0; i < adjustSize; ++i)
                  for i in 0..adjustSize
                  {
                     // Java: Integer adjustValue = adjust.getAdjust(i);
                     // TODO: GeoAdjust type
                     let adjustValue: Option<i32> = None;
                     // Java: this._varList[i] = adjustValue == null ? 0 : adjustValue;
                     self.base._varList[i] = adjustValue.unwrap_or(0);
                  }
                  // Java: Map<Integer, Float> gdValueBuf = new HashMap();
                  let mut gdValueBuf: HashMap<i32, f32> = HashMap::new();
                  // Java: vertices = new int[initVertices.length];
                  vertices = vec![0i32; init_v.len()];
                  // Java: for(int i = 0; i < vertices.length; ++i)
                  for i in 0..vertices.len()
                  {
                     // Java: vertices[i] = (int)this.getValue(initVertices[i], gdValueBuf);
                     vertices[i] = self.getValue_two(init_v[i], &mut gdValueBuf) as i32;
                  }
               }
               // Java: ArrayList<int[]> pathsArray = new ArrayList();
               let mut pathsArray: Vec<Vec<i32>> = Vec::new();
               // Java: if (segments != null && segments.length > 0)
               if let Some(ref segs) = segments {
                  if segs.len() > 0
                  {
                     // Java: this.initPathsArray(pathsArray, segments, vertices);
                     self.initPathsArray(&mut pathsArray, segs, &vertices);
                  }
                  // Java: else
               } else
               {
                  // Java: this.initClosedPolyPathsArray(pathsArray, vertices);
                  self.initClosedPolyPathsArray(&mut pathsArray, &vertices);
               }
               // Java: int[][] pathsLst = new int[pathsArray.size()][];
               let mut pathsLst = pathsArray;
               // Java: for(int i = 0; i < pathsLst.length; ++i)
               // pathsArray items are already in pathsLst
               // Java: pathsLst[i] = (int[])pathsArray.get(i);
               // (no-op in Rust, pathsLst already contains the arrays)

               // Java: if (segments != null && segments.length > 0)
               if let Some(ref segs) = segments {
                  if segs.len() > 0
                  {
                     // Java: this.setPathsLstData(segments, vertices, pathsLst, 0);
                     self.setPathsLstData(segs, &vertices, &mut pathsLst, 0);
                  }
                  // Java: else
               } else
               {
                  // Java: this.setClosedPolyPathsLstData(vertices, pathsLst);
                  self.setClosedPolyPathsLstData(&vertices, &mut pathsLst);
               }
               // Java: this._pathList = pathsLst;
               self._pathList = Some(pathsLst);
            }
         }
      }
    }

    // Java: float getValue(int pcArg, Map<Integer, Float> gdValueBuf)
    pub fn getValue_two(&self, pcArg: i32, gdValueBuf: &mut HashMap<i32, f32>) -> f32 {
      // Java: return this.getValue(pcArg, gdValueBuf, this._varList);
      self.getValue(pcArg, gdValueBuf, &self.base._varList.clone())
    }

    // Java: float getValue(int pcArg, Map<Integer, Float> gdValueBuf, int[] varList)
    pub fn getValue(&self, pcArg: i32, gdValueBuf: &mut HashMap<i32, f32>, varList: &[i32]) -> f32 {
      // Java: if (pcArg >= -2147483521 && pcArg < -2147483511)
      if pcArg >= -2147483521 && pcArg < -2147483511
      {
         // Java: return this._varList != null ? (float)this._varList[pcArg - -2147483521] : 0.0F;
         let idx = (pcArg - (-2147483521)) as usize;
         if !self.base._varList.is_empty() && idx < self.base._varList.len() {
            return self.base._varList[idx] as f32;
         } else {
            return 0.0_f32;
         }
      }
      // Java: else if (pcArg >= Integer.MIN_VALUE && pcArg < -2147483521)
      else if pcArg >= i32::MIN && pcArg < -2147483521
      {
         // Java: int formulaIndex = pcArg - Integer.MIN_VALUE;
         let formulaIndex = (pcArg - i32::MIN) as usize;
         // Java: Float valueBuffer = (Float)gdValueBuf.get(formulaIndex);
         let valueBuffer = gdValueBuf.get(&(formulaIndex as i32)).copied();
         // Java: if (valueBuffer != null && !valueBuffer.isNaN())
         if let Some(vb) = valueBuffer {
            if !vb.is_nan()
            {
               // Java: return valueBuffer;
               return vb;
            }
         }
         // Java: else if (this._formulas != null && formulaIndex < this._formulas.length)
         if let Some(ref formulas) = self._formulas {
            if formulaIndex < formulas.len()
            {
               // Java: int formulaName = this._formulas[formulaIndex].name;
               let formulaName = formulas[formulaIndex].name;
               // Java: int[] originArguments = this._formulas[formulaIndex].arguments;
               let originArguments = &formulas[formulaIndex].arguments;
               // Java: int argCnt = originArguments != null ? originArguments.length : 0;
               let mut argCnt = match originArguments {
                  Some(args) => args.len(),
                  None => 0,
               };
               // Java: argCnt = Math.min(argCnt, CFL.getFormulaParamsCount(formulaName));
               argCnt = std::cmp::min(argCnt, CFL::CFL::getFormulaParamsCount(formulaName) as usize);
               // Java: float[] args = new float[argCnt];
               let mut args = vec![0.0_f32; argCnt];
               // Java: for(int i = 0; i < argCnt; ++i)
               for i in 0..argCnt
               {
                  // Java: args[i] = this.getValue(originArguments[i], gdValueBuf, varList);
                  if let Some(ref orig_args) = originArguments {
                     args[i] = self.getValue(orig_args[i], gdValueBuf, varList);
                  }
               }
               // Java: Integer key = formulaIndex;
               let key = formulaIndex as i32;
               // Java: Float value = CFL.getFormulasValue(formulaName, args);
               let value = CFL::CFL::getFormulasValue(formulaName, &args);
               // Java: gdValueBuf.put(key, value);
               gdValueBuf.insert(key, value);
               // Java: return (Float)gdValueBuf.get(key);
               return *gdValueBuf.get(&key).unwrap_or(&0.0_f32);
            }
            // Java: else
            else
            {
               // Java: return 1.0F;
               return 1.0_f32;
            }
         }
         // Java: else
         else
         {
            // Java: return 1.0F;
            return 1.0_f32;
         }
      }
      // Java: else
      else
      {
         // Java: return pcArg >= -2147483511 && pcArg < -2147483463 ? CFL.getSymbolicValue(pcArg - -2147483511, this._cordSize) : (float)pcArg;
         if pcArg >= -2147483511 && pcArg < -2147483463 {
            if let Some(ref cord_size) = self._cordSize {
               return CFL::CFL::getSymbolicValue(pcArg - (-2147483511), cord_size);
            }
            return 0.0_f32;
         } else {
            return pcArg as f32;
         }
      }
    }

    // Java: int[][] getPathList()
    pub fn getPathList(&self) -> Option<&Vec<Vec<i32>>> {
      // Java: return this._pathList;
      self._pathList.as_ref()
    }

    // Java: void initPathsArray(ArrayList<int[]> pathsArray, Segment[] segments, int[] vertices)
    pub fn initPathsArray(&self, pathsArray: &mut Vec<Vec<i32>>, segments: &[Segment], vertices: &[i32]) {
      // Java: int size = segments.length;
      let size = segments.len();
      // Java: int verLen = vertices.length;
      let verLen = vertices.len();
      // Java: int currentPathLen = 0;
      let mut currentPathLen: usize = 0;
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size
      {
         // Java: if (segments[i].cmd == 128)
         if segments[i].cmd == 128
         {
            // Java: currentPathLen += 2;
            currentPathLen += 2;
            // Java: int[] pathLst = new int[currentPathLen];
            let pathLst = vec![0i32; currentPathLen];
            // Java: pathsArray.add(pathLst);
            pathsArray.push(pathLst);
            // Java: currentPathLen = 0;
            currentPathLen = 0;
         }
         // Java: else
         else
         {
            // Java: short cmd = segments[i].cmd;
            let cmd = segments[i].cmd;
            // Java: int cmdParaCount = Cmd.GetCmdParameterCount(cmd);
            let cmdParaCount = Cmd::GetCmdParameterCount(cmd);
            // Java: if (cmdParaCount != 0)
            if cmdParaCount != 0
            {
               // Java: int startIndex = segments[i].start;
               let startIndex = segments[i].start as usize;
               // Java: int endIndex = i == size - 1 ? verLen : segments[i + 1].start;
               let endIndex = if i == size - 1 { verLen } else { segments[i + 1].start as usize };
               // Java: int len = endIndex - startIndex;
               let len = endIndex - startIndex;
               // Java: if (len >= cmdParaCount)
               if len >= cmdParaCount as usize
               {
                  // Java: int repeatCount = len / cmdParaCount;
                  let repeatCount = len / cmdParaCount as usize;
                  // Java: currentPathLen += repeatCount * (1 + cmdParaCount);
                  currentPathLen += repeatCount * (1 + cmdParaCount as usize);
               }
               // Java: else if (cmd == 163 && endIndex - startIndex == 4)
               else if cmd == 163 && endIndex - startIndex == 4
               {
                  // Java: currentPathLen += 5;
                  currentPathLen += 5;
               }
            }
            // Java: else if (170 != cmd && 171 != cmd)
            else if 170 != cmd && 171 != cmd
            {
               // Java: ++currentPathLen;
               currentPathLen += 1;
            }
         }
      }
      // Java: if (currentPathLen > 0)
      if currentPathLen > 0
      {
         // Java: currentPathLen += 2;
         currentPathLen += 2;
         // Java: int[] pathLst = new int[currentPathLen];
         let pathLst = vec![0i32; currentPathLen];
         // Java: pathsArray.add(pathLst);
         pathsArray.push(pathLst);
      }
    }

    // Java: void initClosedPolyPathsArray(ArrayList<int[]> pathsArray, int[] vertices)
    pub fn initClosedPolyPathsArray(&self, pathsArray: &mut Vec<Vec<i32>>, vertices: &[i32]) {
      // Java: int verLen = vertices.length;
      let verLen = vertices.len();
      // Java: int[] pathLst = new int[2 + verLen / 2 * 3 + 1];
      let pathLst = vec![0i32; 2 + verLen / 2 * 3 + 1];
      // Java: pathsArray.add(pathLst);
      pathsArray.push(pathLst);
    }

    // Java: void setPathsLstData(Segment[] segments, int[] vertices, int[][] pathsLst, int pathIndex)
    pub fn setPathsLstData(&self, segments: &[Segment], vertices: &[i32], pathsLst: &mut Vec<Vec<i32>>, mut pathIndex: usize) {
      // Java: int[] pathLst = pathsLst[pathIndex];
      // Java: pathLst[0] = 11;
      pathsLst[pathIndex][0] = 11;
      // Java: pathLst[1] = 13;
      pathsLst[pathIndex][1] = 13;
      // Java: int pathCmdIndex = 1;
      let mut pathCmdIndex: usize = 1;
      // Java: int size = segments.length;
      let size = segments.len();
      // Java: int verLen = vertices.length;
      let verLen = vertices.len();
      // Java: boolean isVerticeMatch = true;
      let mut isVerticeMatch = true;
      // Java: for(int i = 0; i < size; ++i)
      'outer: for i in 0..size
      {
         // Java: short cmdCode = segments[i].cmd;
         let cmdCode = segments[i].cmd;
         // Java: int verticeIndex = segments[i].start;
         let mut verticeIndex = segments[i].start as usize;
         // Java: int endIndex = i == size - 1 ? verLen : segments[i + 1].start;
         let endIndex = if i == size - 1 { verLen } else { segments[i + 1].start as usize };
         // Java: int len = endIndex - verticeIndex;
         let len = endIndex - verticeIndex;
         // Java: int parameterCount = Cmd.GetCmdParameterCount(cmdCode);
         let mut parameterCount = Cmd::GetCmdParameterCount(cmdCode) as usize;
         // Java: if (len < parameterCount)
         if len < parameterCount
         {
            // Java: if (cmdCode != 163 || len != 4)
            if cmdCode != 163 || len != 4
            {
               // Java: continue;
               continue;
            }
            // Java: parameterCount = 4;
            parameterCount = 4;
         }
         // Java: do
         loop
         {
            // Java: switch (cmdCode)
            match cmdCode
            {
               // Java: case 0:
               0 => {
                  // Java: ++pathCmdIndex;
                  pathCmdIndex += 1;
                  // Java: pathLst[pathCmdIndex] = 1;
                  pathsLst[pathIndex][pathCmdIndex] = 1;
                  // Java: break;
               }
               // Java: case 32:
               32 => {
                  // Java: ++pathCmdIndex;
                  pathCmdIndex += 1;
                  // Java: pathLst[pathCmdIndex] = 5;
                  pathsLst[pathIndex][pathCmdIndex] = 5;
                  // Java: break;
               }
               // Java: case 64:
               64 => {
                  // Java: ++pathCmdIndex;
                  pathCmdIndex += 1;
                  // Java: pathLst[pathCmdIndex] = 0;
                  pathsLst[pathIndex][pathCmdIndex] = 0;
                  // Java: break;
               }
               // Java: case 96:
               96 => {
                  // Java: ++pathCmdIndex;
                  pathCmdIndex += 1;
                  // Java: pathLst[pathCmdIndex] = 2;
                  pathsLst[pathIndex][pathCmdIndex] = 2;
                  // Java: break;
               }
               // Java: case 128:
               128 => {
                  // Java: ++pathIndex;
                  pathIndex += 1;
                  // Java: if (pathIndex < pathsLst.length)
                  if pathIndex < pathsLst.len()
                  {
                     // Java: pathLst = pathsLst[pathIndex];
                     // Java: pathLst[0] = 11;
                     pathsLst[pathIndex][0] = 11;
                     // Java: pathLst[1] = 13;
                     pathsLst[pathIndex][1] = 13;
                     // Java: pathCmdIndex = 1;
                     pathCmdIndex = 1;
                  }
                  // Java: break;
               }
               // Java: case 161:
               // Java: case 162:
               161 | 162 => {
                  // Java: ++pathCmdIndex;
                  pathCmdIndex += 1;
                  // Java: pathLst[pathCmdIndex] = 12;
                  pathsLst[pathIndex][pathCmdIndex] = 12;
                  // Java: break;
               }
               // Java: case 163:
               163 => {
                  // Java: ++pathCmdIndex;
                  pathCmdIndex += 1;
                  // Java: pathLst[pathCmdIndex] = parameterCount == 4 ? 3 : 9;
                  pathsLst[pathIndex][pathCmdIndex] = if parameterCount == 4 { 3 } else { 9 };
                  // Java: break;
               }
               // Java: case 164:
               164 => {
                  // Java: ++pathCmdIndex;
                  pathCmdIndex += 1;
                  // Java: pathLst[pathCmdIndex] = 8;
                  pathsLst[pathIndex][pathCmdIndex] = 8;
                  // Java: break;
               }
               // Java: case 165:
               165 => {
                  // Java: ++pathCmdIndex;
                  pathCmdIndex += 1;
                  // Java: pathLst[pathCmdIndex] = 11;
                  pathsLst[pathIndex][pathCmdIndex] = 11;
                  // Java: break;
               }
               // Java: case 166:
               166 => {
                  // Java: ++pathCmdIndex;
                  pathCmdIndex += 1;
                  // Java: pathLst[pathCmdIndex] = 10;
                  pathsLst[pathIndex][pathCmdIndex] = 10;
                  // Java: break;
               }
               // Java: case 167:
               167 => {
                  // Java: ++pathCmdIndex;
                  pathCmdIndex += 1;
                  // Java: pathLst[pathCmdIndex] = 6;
                  pathsLst[pathIndex][pathCmdIndex] = 6;
                  // Java: break;
               }
               // Java: case 168:
               168 => {
                  // Java: ++pathCmdIndex;
                  pathCmdIndex += 1;
                  // Java: pathLst[pathCmdIndex] = 7;
                  pathsLst[pathIndex][pathCmdIndex] = 7;
                  // Java: break;
               }
               // Java: case 169:
               169 => {
                  // Java: ++pathCmdIndex;
                  pathCmdIndex += 1;
                  // Java: pathLst[pathCmdIndex] = 4;
                  pathsLst[pathIndex][pathCmdIndex] = 4;
                  // Java: break;
               }
               // Java: case 170:
               170 => {
                  // Java: pathLst[0] = 10;
                  pathsLst[pathIndex][0] = 10;
                  // Java: break;
               }
               // Java: case 171:
               171 => {
                  // Java: pathLst[1] = 12;
                  pathsLst[pathIndex][1] = 12;
                  // Java: break;
               }
               // Java: default:
               _ => {
                  // Java: ++pathCmdIndex;
                  pathCmdIndex += 1;
                  // Java: pathLst[pathCmdIndex] = -1;
                  pathsLst[pathIndex][pathCmdIndex] = -1;
               }
            }
            // Java: for(int parameterIndex = 0; parameterIndex < parameterCount; ++parameterIndex)
            for parameterIndex in 0..parameterCount
            {
               // Java: int index = verticeIndex + parameterIndex;
               let index = verticeIndex + parameterIndex;
               // Java: if (index >= verLen || pathCmdIndex + 1 >= pathLst.length)
               if index >= verLen || pathCmdIndex + 1 >= pathsLst[pathIndex].len()
               {
                  // Java: isVerticeMatch = false;
                  isVerticeMatch = false;
                  // Java: pathCmdIndex -= parameterIndex + 1;
                  pathCmdIndex = pathCmdIndex.wrapping_sub(parameterIndex + 1);
                  // Java: break;
                  break;
               }
               // Java: ++pathCmdIndex;
               pathCmdIndex += 1;
               // Java: pathLst[pathCmdIndex] = vertices[index];
               pathsLst[pathIndex][pathCmdIndex] = vertices[index];
            }
            // Java: verticeIndex += parameterCount;
            verticeIndex += parameterCount;
            // do-while condition: isVerticeMatch && verticeIndex < endIndex && parameterCount != 0
            if !(isVerticeMatch && verticeIndex < endIndex && parameterCount != 0) {
               break;
            }
         }
         // Java: if (!isVerticeMatch)
         if !isVerticeMatch
         {
            // Java: if (pathLst.length > pathCmdIndex + 1)
            if pathsLst[pathIndex].len() > pathCmdIndex + 1
            {
               // Java: ++pathCmdIndex;
               pathCmdIndex += 1;
               // Java: pathLst[pathCmdIndex] = -2;
               pathsLst[pathIndex][pathCmdIndex] = -2;
            }
            // Java: ++pathIndex;
            pathIndex += 1;
            // Java: while(pathIndex < pathsLst.length)
            while pathIndex < pathsLst.len()
            {
               // Java: pathLst = pathsLst[pathIndex];
               // Java: pathLst[0] = 11;
               pathsLst[pathIndex][0] = 11;
               // Java: pathLst[1] = 13;
               pathsLst[pathIndex][1] = 13;
               // Java: pathLst[2] = -2;
               pathsLst[pathIndex][2] = -2;
               // Java: ++pathIndex;
               pathIndex += 1;
            }
            // Java: break;
            break;
         }
      }
    }

    // Java: void setClosedPolyPathsLstData(int[] vertices, int[][] pathsLst)
    pub fn setClosedPolyPathsLstData(&self, vertices: &[i32], pathsLst: &mut Vec<Vec<i32>>) {
      // Java: int[] pathLst = pathsLst[0];
      // Java: int pathCmdIndex = 0;
      let mut pathCmdIndex: usize = 0;
      // Java: pathLst[pathCmdIndex++] = 11;
      pathsLst[0][pathCmdIndex] = 11;
      pathCmdIndex += 1;
      // Java: pathLst[pathCmdIndex++] = 13;
      pathsLst[0][pathCmdIndex] = 13;
      pathCmdIndex += 1;
      // Java: int verLen = vertices.length;
      let verLen = vertices.len();
      // Java: if (verLen >= 6)
      if verLen >= 6
      {
         // Java: int verticeIndex = 0;
         let mut verticeIndex: usize = 0;
         // Java: int parameterCount = 2;
         let parameterCount: usize = 2;
         // Java: for(int i = 0; i < verLen / 2; ++i)
         for i in 0..verLen / 2
         {
            // Java: pathLst[pathCmdIndex++] = i == 0 ? 0 : 1;
            pathsLst[0][pathCmdIndex] = if i == 0 { 0 } else { 1 };
            pathCmdIndex += 1;
            // Java: for(int parameterIndex = 0; parameterIndex < parameterCount; ++parameterIndex)
            for parameterIndex in 0..parameterCount
            {
               // Java: pathLst[pathCmdIndex++] = vertices[verticeIndex + parameterIndex];
               pathsLst[0][pathCmdIndex] = vertices[verticeIndex + parameterIndex];
               pathCmdIndex += 1;
            }
            // Java: verticeIndex += parameterCount;
            verticeIndex += parameterCount;
         }
         // Java: pathLst[pathCmdIndex++] = 2;
         pathsLst[0][pathCmdIndex] = 2;
         pathCmdIndex += 1;
      }
    }

}
