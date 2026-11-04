// Java: public class TextImage
#[derive(Clone)]
pub struct TextImage {
    // Java field: float blurRad
    pub(crate) blurRad: f32,
    // Java field: double stA
    pub(crate) stA: f64,
    // Java field: double endA
    pub(crate) endA: f64,
    // Java field: double stPos
    pub(crate) stPos: f64,
    // Java field: double endPos
    pub(crate) endPos: f64,
    // Java field: float dist
    pub(crate) dist: f32,
    // Java field: int dir
    pub(crate) dir: i32,
    // Java field: double sx
    pub(crate) sx: f64,
    // Java field: double sy
    pub(crate) sy: f64,
    // Java field: int algn
    pub(crate) algn: i32,
    // Java field: boolean isRotWithShape
    pub(crate) isRotWithShape: bool,
}

// Java: public class TextImage
impl TextImage {
    // Java: TextImage(float blurRad, double stA, double endA, double stPos, double endPos, float dist, int dir, double sx, double sy, int algn, boolean isRotWithShape)
    pub fn new(blurRad: f32, stA: f64, endA: f64, stPos: f64, endPos: f64, dist: f32, dir: i32, sx: f64, sy: f64, algn: i32, isRotWithShape: bool) -> Self {
      // Java: super();
      // Java: this.blurRad = blurRad;
      // Java: this.stA = stA;
      // Java: this.endA = endA;
      // Java: this.stPos = stPos;
      // Java: this.endPos = endPos;
      // Java: this.dist = dist;
      // Java: this.dir = dir;
      // Java: this.sx = sx;
      // Java: this.sy = sy;
      // Java: this.algn = algn;
      // Java: this.isRotWithShape = isRotWithShape;
      TextImage { blurRad, stA, endA, stPos, endPos, dist, dir, sx, sy, algn, isRotWithShape }
    }

    // Java: float getBlurRad()
    pub fn getBlurRad(&self) -> f32 {
      // Java: return this.blurRad;
      self.blurRad
    }

    // Java: double getStA()
    pub fn getStA(&self) -> f64 {
      // Java: return this.stA;
      self.stA
    }

    // Java: double getEndA()
    pub fn getEndA(&self) -> f64 {
      // Java: return this.endA;
      self.endA
    }

    // Java: double getStPos()
    pub fn getStPos(&self) -> f64 {
      // Java: return this.stPos;
      self.stPos
    }

    // Java: double getEndPos()
    pub fn getEndPos(&self) -> f64 {
      // Java: return this.endPos;
      self.endPos
    }

    // Java: float getDist()
    pub fn getDist(&self) -> f32 {
      // Java: return this.dist;
      self.dist
    }

    // Java: int getDir()
    pub fn getDir(&self) -> i32 {
      // Java: return this.dir;
      self.dir
    }

    // Java: double getSx()
    pub fn getSx(&self) -> f64 {
      // Java: return this.sx;
      self.sx
    }

    // Java: double getSy()
    pub fn getSy(&self) -> f64 {
      // Java: return this.sy;
      self.sy
    }

    // Java: int getAlgn()
    pub fn getAlgn(&self) -> i32 {
      // Java: return this.algn;
      self.algn
    }

    // Java: boolean isRotWithShape()
    pub fn isRotWithShape(&self) -> bool {
      // Java: return this.isRotWithShape;
      self.isRotWithShape
    }

    // Java: boolean equals(TextImage txtImage)
    pub fn equals(&self, txtImage: Option<&TextImage>) -> bool {
      // Java: if (txtImage == null)
      let txtImage = match txtImage {
          None =>
      {
         // Java: return false;
         return false;
      }
      // Java: else
          Some(t) => t,
      };
      {
         // Java: return this.blurRad == txtImage.blurRad && this.stA == txtImage.stA && this.endA == txtImage.endA && this.stPos == txtImage.stPos && this.endPos == txtImage.endPos && this.dist == txtImage.dist && this.dir == txtImage.dir && this.sx == txtImage.sx && this.sy == txtImage.sy && this.algn == txtImage.algn && this.isRotWithShape == txtImage.isRotWithShape;
         self.blurRad == txtImage.blurRad
             && self.stA == txtImage.stA
             && self.endA == txtImage.endA
             && self.stPos == txtImage.stPos
             && self.endPos == txtImage.endPos
             && self.dist == txtImage.dist
             && self.dir == txtImage.dir
             && self.sx == txtImage.sx
             && self.sy == txtImage.sy
             && self.algn == txtImage.algn
             && self.isRotWithShape == txtImage.isRotWithShape
      }
    }

}
