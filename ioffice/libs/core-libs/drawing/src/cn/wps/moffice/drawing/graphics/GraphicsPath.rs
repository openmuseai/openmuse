#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.moffice.drawing.line.LineProperty
// use crate::cn::wps::moffice::drawing::line::LineProperty::LineProperty; // line module disabled

#[derive(Clone)]
pub struct LineProperty;
// Java: import java.util.ArrayList

use super::PathElem::PathElem;
use super::FillBase::FillBase;

// Java field: int FILLTYPE_Darken
pub const FILLTYPE_Darken: i32 = 0;
// Java field: int FILLTYPE_DarkenLess
pub const FILLTYPE_DarkenLess: i32 = 1;
// Java field: int FILLTYPE_Lighten
pub const FILLTYPE_Lighten: i32 = 2;
// Java field: int FILLTYPE_Lightenless
pub const FILLTYPE_Lightenless: i32 = 3;
// Java field: int FILLTYPE_None
pub const FILLTYPE_None: i32 = 4;
// Java field: int FILLTYPE_Norm
pub const FILLTYPE_Norm: i32 = 5;
// Java field: int WINDING
pub const WINDING: i32 = 0;
// Java field: int EVEN_ODD
pub const EVEN_ODD: i32 = 1;
// Java field: int INVERSE_WINDING
pub const INVERSE_WINDING: i32 = 2;
// Java field: int INVERSE_EVEN_ODD
pub const INVERSE_EVEN_ODD: i32 = 3;

// Java: public class GraphicsPath
#[derive(Clone)]
pub struct GraphicsPath {
    // Java field: int fillType
    pub(crate) fillType: i32,
    // Java field: int fillType2
    pub(crate) fillType2: i32,
    // Java field: boolean needStroke
    pub(crate) needStroke: bool,
    // Java field: int closedPaths
    pub(crate) closedPaths: i32,
    // Java field: ArrayList<PathElem> elems
    pub(crate) elems: Vec<PathElem>,
    // Java field: LineProperty lineProperty
    pub(crate) lineProperty: Option<LineProperty>,
    // Java field: FillBase fill
    pub(crate) fill: Option<FillBase>,
    // Java field: int shapeType
    pub(crate) shapeType: i32,
}

// Java: public class GraphicsPath
impl GraphicsPath {
    // Java: GraphicsPath()
    pub fn new() -> Self {
      // Java: super();
      GraphicsPath {
          fillType: 4,
          fillType2: 0,
          needStroke: false,
          closedPaths: 0,
          elems: Vec::new(),
          lineProperty: None,
          fill: None,
          shapeType: 0,
      }
    }

    // Java: void setShapeType(int shapeType)
    pub fn setShapeType(&mut self, shapeType: i32) {
      // Java: this.shapeType = shapeType;
      self.shapeType = shapeType;
    }

    // Java: int getShapeType()
    pub fn getShapeType(&self) -> i32 {
      // Java: return this.shapeType;
      self.shapeType
    }

    // Java: void setNeedStroke(boolean value)
    pub fn setNeedStroke(&mut self, value: bool) {
      // Java: this.needStroke = value;
      self.needStroke = value;
    }

    // Java: boolean getNeedSroke()
    pub fn getNeedSroke(&self) -> bool {
      // Java: return this.needStroke;
      self.needStroke
    }

    // Java: boolean getNeedFill()
    pub fn getNeedFill(&self) -> bool {
      // Java: return this.fillType != 4;
      self.fillType != 4
    }

    // Java: void setFillType(int fillType)
    pub fn setFillType(&mut self, fillType: i32) {
      // Java: this.fillType = fillType;
      self.fillType = fillType;
    }

    // Java: int getFillType()
    pub fn getFillType(&self) -> i32 {
      // Java: return this.fillType;
      self.fillType
    }

    // Java: void setFillType2(int fillType)
    pub fn setFillType2(&mut self, fillType: i32) {
      // Java: this.fillType2 = fillType;
      self.fillType2 = fillType;
    }

    // Java: int getFillType2()
    pub fn getFillType2(&self) -> i32 {
      // Java: return this.fillType2;
      self.fillType2
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.elems.size();
      self.elems.len() as i32
    }

    // Java: PathElem getElem(int index)
    pub fn getElem(&self, index: i32) -> &PathElem {
      // Java: return (PathElem)this.elems.get(index);
      &self.elems[index as usize]
    }

    fn getElem_mut(&mut self, index: i32) -> &mut PathElem {
      &mut self.elems[index as usize]
    }

    // Java: void addElem(PathElem pathElem)
    pub fn addElem(&mut self, pathElem: PathElem) {
      // Java: this.elems.add(pathElem);
      // Java: if (pathElem.elemType == 5)
      if pathElem.elemType == 5 {
         // Java: ++this.closedPaths;
         self.closedPaths += 1;
      }
      self.elems.push(pathElem);
    }

    // Java: PathElem getLastElem()
    pub fn getLastElem(&self) -> &PathElem {
      // Java: return (PathElem)this.elems.get(this.elems.size() - 1);
      self.elems.last().unwrap()
    }

    // Java: void scale(float scaleX, float scaleY)
    pub fn scale(&mut self, scaleX: f32, scaleY: f32) {
      // Java: int size = this.elems.size();
      let size = self.elems.len();
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: PathElem elem = this.getElem(i);
         let elem = &mut self.elems[i];
         // Java: if (elem.pos != null)
         if !elem.pos.is_empty() {
            // Java: int posCount = elem.pos.length;
            let mut posCount = elem.pos.len();
            // Java: if (elem.elemType == 2)
            if elem.elemType == 2 {
               // Java: posCount -= 2;
               posCount -= 2;
            }
            // Java: for(int posIndex = 0; posIndex < posCount; ++posIndex)
            for posIndex in 0..posCount {
               // Java: if (posIndex % 2 == 0)
               if posIndex % 2 == 0 {
                  // Java: float[] var10000 = elem.pos;
                  // Java: var10000[posIndex] *= scaleX;
                  elem.pos[posIndex] *= scaleX;
               }
               // Java: else
               else {
                  // Java: float[] var8 = elem.pos;
                  // Java: var8[posIndex] *= scaleY;
                  elem.pos[posIndex] *= scaleY;
               }
            }
         }
      }
    }

    // Java: void adjust(float offsetX, float offsetY, float scaleX, float scaleY)
    pub fn adjust(&mut self, offsetX: f32, offsetY: f32, scaleX: f32, scaleY: f32) {
      // Java: int size = this.elems.size();
      let size = self.elems.len();
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: PathElem elem = this.getElem(i);
         let elem = &mut self.elems[i];
         // Java: if (elem.pos != null)
         if !elem.pos.is_empty() {
            // Java: int posCount = elem.pos.length;
            let mut posCount = elem.pos.len();
            // Java: if (elem.elemType == 2)
            if elem.elemType == 2 {
               // Java: posCount -= 2;
               posCount -= 2;
            }
            // Java: for(int posIndex = 0; posIndex < posCount; ++posIndex)
            for posIndex in 0..posCount {
               // Java: if (posIndex % 2 == 0)
               if posIndex % 2 == 0 {
                  // Java: elem.pos[posIndex] = (elem.pos[posIndex] + offsetX) * scaleX;
                  elem.pos[posIndex] = (elem.pos[posIndex] + offsetX) * scaleX;
               }
               // Java: else
               else {
                  // Java: elem.pos[posIndex] = (elem.pos[posIndex] + offsetY) * scaleY;
                  elem.pos[posIndex] = (elem.pos[posIndex] + offsetY) * scaleY;
               }
            }
         }
      }
    }

    // Java: void offset(float offsetX, float offsetY)
    pub fn offset(&mut self, offsetX: f32, offsetY: f32) {
      // Java: int size = this.elems.size();
      let size = self.elems.len();
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: PathElem elem = this.getElem(i);
         let elem = &mut self.elems[i];
         // Java: if (elem.pos != null && elem.elemType != 2)
         if !elem.pos.is_empty() && elem.elemType != 2 {
            // Java: int posCount = elem.pos.length;
            let posCount = elem.pos.len();
            // Java: for(int posIndex = 0; posIndex < posCount; ++posIndex)
            for posIndex in 0..posCount {
               // Java: if (posIndex % 2 == 0)
               if posIndex % 2 == 0 {
                  // Java: float[] var10000 = elem.pos;
                  // Java: var10000[posIndex] += offsetX;
                  elem.pos[posIndex] += offsetX;
               }
               // Java: else
               else {
                  // Java: float[] var8 = elem.pos;
                  // Java: var8[posIndex] += offsetY;
                  elem.pos[posIndex] += offsetY;
               }
            }
         }
      }
    }

    // Java: void removeElem(int index)
    pub fn removeElem(&mut self, index: i32) {
      // Java: this.elems.remove(index);
      self.elems.remove(index as usize);
    }

    // Java: void setStrokeAndFill(LineProperty lineProperty, FillBase fill)
    pub fn setStrokeAndFill(&mut self, lineProperty: Option<LineProperty>, fill: Option<FillBase>) {
      // Java: this.lineProperty = lineProperty;
      self.lineProperty = lineProperty;
      // Java: this.fill = fill;
      self.fill = fill;
    }

    // Java: LineProperty getLineProperty()
    pub fn getLineProperty(&self) -> &Option<LineProperty> {
      // Java: return this.lineProperty;
      &self.lineProperty
    }

    // Java: FillBase getFill()
    pub fn getFill(&self) -> &Option<FillBase> {
      // Java: return this.fill;
      &self.fill
    }

    // Java: String getLog()
    pub fn getLog(&self) -> String {
      // Java: String value = "";
      let mut value = String::new();
      // Java: int size = this.size();
      let size = self.size();
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: PathElem elem = this.getElem(i);
         let elem = self.getElem(i);
         // Java: switch (elem.elemType)
         match elem.elemType {
            // Java: case 0:
            0 => {
               // Java: value = value + "moveTo  " + elem.pos[0] + "  " + elem.pos[1] + ";";
               value = format!("{}moveTo  {}  {};", value, elem.pos[0], elem.pos[1]);
               // Java: break;
            }
            // Java: case 1:
            1 => {
               // Java: value = value + "lineTo  " + elem.pos[0] + "  " + elem.pos[1] + ";";
               value = format!("{}lineTo  {}  {};", value, elem.pos[0], elem.pos[1]);
               // Java: break;
            }
            // Java: case 2:
            2 => {
               // Java: if (elem.pos.length == 4)
               if elem.pos.len() == 4 {
                  // Java: value = value + "arcTo  " + elem.pos[0] + "  " + elem.pos[1] + "  " + elem.pos[2] + "  " + elem.pos[3] + ";";
                  value = format!("{}arcTo  {}  {}  {}  {};", value, elem.pos[0], elem.pos[1], elem.pos[2], elem.pos[3]);
               }
               // Java: else
               else {
                  // Java: value = value + "arcTo  " + elem.pos[2] + "  " + elem.pos[3] + "  " + elem.pos[4] + "  " + elem.pos[5] + ";";
                  value = format!("{}arcTo  {}  {}  {}  {};", value, elem.pos[2], elem.pos[3], elem.pos[4], elem.pos[5]);
               }
               // Java: break;
            }
            // Java: case 3:
            3 => {
               // Java: value = value + "quadTo  " + elem.pos[0] + "  " + elem.pos[1] + "  " + elem.pos[2] + "  " + elem.pos[3] + ";";
               value = format!("{}quadTo  {}  {}  {}  {};", value, elem.pos[0], elem.pos[1], elem.pos[2], elem.pos[3]);
               // Java: break;
            }
            // Java: case 4:
            4 => {
               // Java: value = value + "cubicTo" + elem.pos[0] + "  " + elem.pos[1] + "  " + elem.pos[2] + "  " + elem.pos[3] + "  " + elem.pos[4] + "  " + elem.pos[5] + ";";
               value = format!("{}cubicTo{}  {}  {}  {}  {}  {};", value, elem.pos[0], elem.pos[1], elem.pos[2], elem.pos[3], elem.pos[4], elem.pos[5]);
               // Java: break;
            }
            // Java: case 5:
            5 => {
               // Java: value = value + "close";
               value = format!("{}close", value);
            }
            _ => {}
         }
      }
      // Java: return value;
      value
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: this.elems.clear();
      self.elems.clear();
      // Java: this.needStroke = false;
      self.needStroke = false;
      // Java: this.lineProperty = null;
      self.lineProperty = None;
      // Java: this.fill = null;
      self.fill = None;
      // Java: this.closedPaths = 0;
      self.closedPaths = 0;
    }

    // Java: boolean hasClosedPath()
    pub fn hasClosedPath(&self) -> bool {
      // Java: return this.closedPaths > 0;
      self.closedPaths > 0
    }

    // Java: int getClosedPathNum()
    pub fn getClosedPathNum(&self) -> i32 {
      // Java: return this.closedPaths;
      self.closedPaths
    }

}
