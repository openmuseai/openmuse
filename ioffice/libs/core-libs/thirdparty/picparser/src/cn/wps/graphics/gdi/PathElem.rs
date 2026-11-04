#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports)]

use crate::cn::wps::graphics::gdi::Rectangle::Rectangle;

// Java field: int PATH_NONE
pub(crate) const PATH_NONE: i32 = -1;
// Java field: int PATH_MOVE
pub(crate) const PATH_MOVE: i32 = 0;
// Java field: int PATH_LINE
pub(crate) const PATH_LINE: i32 = 1;
// Java field: int PATH_ARC
pub(crate) const PATH_ARC: i32 = 2;
// Java field: int PATH_OVAL
pub(crate) const PATH_OVAL: i32 = 3;
// Java field: int PATH_CUBIC
pub(crate) const PATH_CUBIC: i32 = 4;
// Java field: int PATH_CLOSE
pub(crate) const PATH_CLOSE: i32 = 5;
// Java field: int PATH_ADDRECT
pub(crate) const PATH_ADDRECT: i32 = 6;
// Java field: int PATH_ADDRoundRECT
pub(crate) const PATH_ADDRoundRECT: i32 = 7;

// Java: public class PathElem
pub struct PathElem {
    // Java field: int elemType
    pub(crate) elemType: i32,
    // Java field: float[] pos
    pub(crate) pos: Option<Vec<f32>>,
    // Java field: Rectangle rect
    pub(crate) rect: Option<Rectangle>,
}

// Java: public class PathElem
impl PathElem {
    // Java: PathElem()
    pub fn new() -> Self {
      // Java: super();
      PathElem { elemType: -1, pos: None, rect: None }
    }

    // Java: float[] getPoints()
    pub fn getPoints(&self) -> Option<&Vec<f32>> {
      // Java: return this.pos;
      self.pos.as_ref()
    }

    // Java: static PathElem createMoveElem(float x, float y)
    pub fn createMoveElem(x: f32, y: f32) -> PathElem {
      // Java: PathElem elem = new PathElem();
      let mut elem = PathElem::new();
      // Java: elem.elemType = 0;
      elem.elemType = 0;
      // Java: elem.pos = new float[]{x, y};
      elem.pos = Some(vec![x, y]);
      // Java: return elem;
      elem
    }

    // Java: static PathElem createLineElem(float x, float y)
    pub fn createLineElem(x: f32, y: f32) -> PathElem {
      // Java: PathElem elem = new PathElem();
      let mut elem = PathElem::new();
      // Java: elem.elemType = 1;
      elem.elemType = 1;
      // Java: elem.pos = new float[]{x, y};
      elem.pos = Some(vec![x, y]);
      // Java: return elem;
      elem
    }

    // Java: static PathElem createArcElem(Rectangle rect, float startAngle, float swingAngle)
    pub fn createArcElem(rect: &Rectangle, startAngle: f32, swingAngle: f32) -> PathElem {
      // Java: PathElem elem = new PathElem();
      let mut elem = PathElem::new();
      // Java: elem.elemType = 2;
      elem.elemType = 2;
      // Java: elem.rect = new Rectangle(rect);
      elem.rect = Some(rect.clone());
      // Java: elem.pos = new float[]{startAngle, swingAngle};
      elem.pos = Some(vec![startAngle, swingAngle]);
      // Java: return elem;
      elem
    }

    // Java: static PathElem createOvalElem(Rectangle rect)
    pub fn createOvalElem(rect: &Rectangle) -> PathElem {
      // Java: PathElem elem = new PathElem();
      let mut elem = PathElem::new();
      // Java: elem.elemType = 3;
      elem.elemType = 3;
      // Java: elem.rect = new Rectangle(rect);
      elem.rect = Some(rect.clone());
      // Java: return elem;
      elem
    }

    // Java: static PathElem createCubicElem(float ctrlX1, float ctrlY1, float ctrlX2, float ctrlY2, float anchorX, float anchorY)
    pub fn createCubicElem(ctrlX1: f32, ctrlY1: f32, ctrlX2: f32, ctrlY2: f32, anchorX: f32, anchorY: f32) -> PathElem {
      // Java: PathElem elem = new PathElem();
      let mut elem = PathElem::new();
      // Java: elem.elemType = 4;
      elem.elemType = 4;
      // Java: elem.pos = new float[]{ctrlX1, ctrlY1, ctrlX2, ctrlY2, anchorX, anchorY};
      elem.pos = Some(vec![ctrlX1, ctrlY1, ctrlX2, ctrlY2, anchorX, anchorY]);
      // Java: return elem;
      elem
    }

    // Java: static PathElem createCloseElem()
    pub fn createCloseElem() -> PathElem {
      // Java: PathElem elem = new PathElem();
      let mut elem = PathElem::new();
      // Java: elem.elemType = 5;
      elem.elemType = 5;
      // Java: return elem;
      elem
    }

    // Java: static PathElem createAddRectElem(Rectangle rect)
    pub fn createAddRectElem(rect: &Rectangle) -> PathElem {
      // Java: PathElem elem = new PathElem();
      let mut elem = PathElem::new();
      // Java: elem.elemType = 6;
      elem.elemType = 6;
      // Java: elem.rect = new Rectangle(rect);
      elem.rect = Some(rect.clone());
      // Java: return elem;
      elem
    }

    // Java: static PathElem createAddRoundRectElem(Rectangle rect, float x, float y)
    pub fn createAddRoundRectElem(rect: &Rectangle, x: f32, y: f32) -> PathElem {
      // Java: PathElem elem = new PathElem();
      let mut elem = PathElem::new();
      // Java: elem.elemType = 7;
      elem.elemType = 7;
      // Java: elem.rect = new Rectangle(rect);
      elem.rect = Some(rect.clone());
      // Java: elem.pos = new float[]{x, y};
      elem.pos = Some(vec![x, y]);
      // Java: return elem;
      elem
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: String str = "";
      let mut str_val = String::new();
      // Java: switch (this.elemType)
      match self.elemType
      {
         // Java: case 0:
         0 =>
         {
            // Java: str = str + "moveTo";
            str_val += "moveTo";
            // Java: break;
         }
         // Java: case 1:
         1 =>
         {
            // Java: str = str + "lineTo";
            str_val += "lineTo";
            // Java: break;
         }
         // Java: case 2:
         2 =>
         {
            // Java: str = str + "arcTo";
            str_val += "arcTo";
            // Java: break;
         }
         // Java: case 3:
         3 =>
         {
            // Java: str = str + "ovalTo";
            str_val += "ovalTo";
            // Java: break;
         }
         // Java: case 4:
         4 =>
         {
            // Java: str = str + "cubicTo";
            str_val += "cubicTo";
            // Java: break;
         }
         // Java: case 5:
         5 =>
         {
            // Java: str = str + "close";
            str_val += "close";
            // Java: break;
         }
         // Java: case 6:
         6 =>
         {
            // Java: str = str + "addRect";
            str_val += "addRect";
         }
         _ => {}
      }
      // Java: if (this.pos != null)
      if self.pos.is_some()
      {
         // Java: for(int i = 0; i < this.pos.length; ++i)
         if let Some(ref pos) = self.pos {
             for i in 0..pos.len()
             {
                // Java: str = str + " " + this.pos[i];
                str_val += &format!(" {}", pos[i]);
             }
         }
      }
      // Java: return str;
      str_val
    }

}
