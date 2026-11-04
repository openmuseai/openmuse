// Java: public class Geometry
pub struct Geometry {
}

// Java: public class Geometry
impl Geometry {
    // Java: Geometry()
    pub fn Geometry() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static boolean isIntersectLine(float Xa, float Ya, float Xb, float Yb, float Xc, float Yc, float Xd, float Yd)
    pub fn isIntersectLine(xa: f32, ya: f32, xb: f32, yb: f32, xc: f32, yc: f32, xd: f32, yd: f32) -> bool {
      // Java: float S_ABC = getParalArea(Xc, Yc, Xa, Ya, Xb, Yb);
      let s_abc = Self::getParalArea(xc, yc, xa, ya, xb, yb);
      // Java: float S_ABD = getParalArea(Xd, Yd, Xa, Ya, Xb, Yb);
      let s_abd = Self::getParalArea(xd, yd, xa, ya, xb, yb);
      // Java: if (S_ABC * S_ABD < 0.0F)
      if s_abc * s_abd < 0.0
      {
         // Java: float S_ACD = getParalArea(Xc, Yc, Xd, Yd, Xa, Ya);
         let s_acd = Self::getParalArea(xc, yc, xd, yd, xa, ya);
         // Java: float S_BCD = getParalArea(Xc, Yc, Xd, Yd, Xb, Yb);
         let s_bcd = Self::getParalArea(xc, yc, xd, yd, xb, yb);
         // Java: if (S_ACD * S_BCD < 0.0F)
         if s_acd * s_bcd < 0.0
         {
            // Java: return true;
            return true;
         }
      }
      // Java: return false;
      false
    }

    // Java: static float getParalArea(float x1, float y1, float x2, float y2, float x3, float y3)
    pub fn getParalArea(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> f32 {
      // Java: return (x1 - x3) * (y2 - y3) - (y1 - y3) * (x2 - x3);
      (x1 - x3) * (y2 - y3) - (y1 - y3) * (x2 - x3)
    }

    // Java: static boolean isInCircle(float x, float y, float x0, float y0, float radius)
    pub fn isInCircle(x: f32, y: f32, x0: f32, y0: f32, radius: f32) -> bool {
      // Java: float aixsX = x0 - x;
      let aixs_x = x0 - x;
      // Java: float aixsY = y0 - y;
      let aixs_y = y0 - y;
      // Java: float leftEquation = aixsX * aixsX + aixsY * aixsY;
      let left_equation = aixs_x * aixs_x + aixs_y * aixs_y;
      // Java: float rightEquation = radius * radius;
      let right_equation = radius * radius;
      // Java: return leftEquation <= rightEquation;
      left_equation <= right_equation
    }

    // Java: static boolean isIntersectCircle(float x1, float y1, float x2, float y2, float x0, float y0, float radius)
    pub fn isIntersectCircle(x1: f32, y1: f32, x2: f32, y2: f32, x0: f32, y0: f32, radius: f32) -> bool {
      // Java: float A = y1 - y2;
      let a = y1 - y2;
      // Java: float B = x1 - x2;
      let b = x1 - x2;
      // Java: float denominator = Math.abs(A * x0 - B * y0 + x1 * y2 - y1 + x2);
      let denominator = (a * x0 - b * y0 + x1 * y2 - y1 + x2).abs();
      // Java: float numerator = (float)Math.sqrt((double)(A * A + B * B));
      let numerator = ((a * a + b * b) as f64).sqrt() as f32;
      // Java: return denominator / numerator < radius;
      denominator / numerator < radius
    }

    // Java: static boolean isInLineAB(float x1, float y1, float x2, float y2, float x0, float y0)
    pub fn isInLineAB(x1: f32, y1: f32, x2: f32, y2: f32, x0: f32, y0: f32) -> bool {
      // Java: boolean inLine = false;
      let mut in_line = false;
      // Java: if (x1 != x2)
      if x1 != x2
      {
         // Java: if (x1 < x2 && x0 >= x1 && x0 <= x2 || x2 < x1 && x0 >= x2 && x0 <= x1)
         if (x1 < x2 && x0 >= x1 && x0 <= x2) || (x2 < x1 && x0 >= x2 && x0 <= x1)
         {
            // Java: inLine = true;
            in_line = true;
         }
      }
      // Java: else if (y1 < y2 && y0 >= y1 && y0 <= y2 || y2 < y1 && y0 >= y2 && y0 <= y1)
      else if (y1 < y2 && y0 >= y1 && y0 <= y2) || (y2 < y1 && y0 >= y2 && y0 <= y1)
      {
         // Java: inLine = true;
         in_line = true;
      }
      // Java: return inLine;
      in_line
    }

    // Java: static float getPointDistance(float x1, float y1, float x2, float y2)
    pub fn getPointDistance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
      // Java: float a = x1 - x2;
      let a = x1 - x2;
      // Java: float b = y1 - y2;
      let b = y1 - y2;
      // Java: return (float)Math.sqrt((double)(a * a + b * b));
      ((a * a + b * b) as f64).sqrt() as f32
    }

    // Java: static float[] getDot(float x1, float y1, float x2, float y2, float x0, float y0)
    pub fn getDot(x1: f32, y1: f32, x2: f32, y2: f32, x0: f32, y0: f32) -> Vec<f32> {
      // Java: float a = x1 - x2;
      let a = x1 - x2;
      // Java: float b = y1 - y2;
      let b = y1 - y2;
      // Java: if (0.0F == a)
      if a == 0.0
      {
         // Java: return new float[]{x1, y0};
         return vec![x1, y0];
      }
      // Java: else if (0.0F == b)
      else if b == 0.0
      {
         // Java: return new float[]{x0, y1};
         return vec![x0, y1];
      }
      // Java: else
      else
      {
         // Java: float c = y0 - y2;
         let c = y0 - y2;
         // Java: float a_squ = a * a;
         let a_squ = a * a;
         // Java: float b_squ = b * b;
         let b_squ = b * b;
         // Java: float x = (a * b * c + a_squ * x0 + b_squ * x2) / (a_squ + b_squ);
         let x = (a * b * c + a_squ * x0 + b_squ * x2) / (a_squ + b_squ);
         // Java: float y = b / a * (x - x2) + y2;
         let y = b / a * (x - x2) + y2;
         // Java: return new float[]{x, y};
         vec![x, y]
      }
    }

}
