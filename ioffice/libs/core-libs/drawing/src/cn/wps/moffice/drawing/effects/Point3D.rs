#![allow(non_snake_case, dead_code)]

// Java: public class Point3D
pub struct Point3D {
    // Java field: float x
    pub(crate) x: f32,
    // Java field: float y
    pub(crate) y: f32,
    // Java field: float z
    pub(crate) z: f32,
}

// Java: public class Point3D
impl Point3D {
    // Java: Point3D()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.x = this.y = this.z = 0.0F;
      Point3D { x: 0.0_f32, y: 0.0_f32, z: 0.0_f32 }
    }

    // Java: Point3D(float x, float y, float z)
    pub fn new_xyz(x: f32, y: f32, z: f32) -> Self {
      // Java: super();
      // Java: this.x = x;
      // Java: this.y = y;
      // Java: this.z = z;
      Point3D { x, y, z }
    }

    // Java: Point3D(Vector3D v)
    pub fn new_from_vector(vx: f32, vy: f32, vz: f32) -> Self {
      // Java: super();
      // Java: this.x = v.vx;
      // Java: this.y = v.vy;
      // Java: this.z = v.vz;
      Point3D { x: vx, y: vy, z: vz }
    }

    // Java: void set(float x, float y, float z)
    pub fn set(&mut self, x: f32, y: f32, z: f32) {
      // Java: this.x = x;
      self.x = x;
      // Java: this.y = y;
      self.y = y;
      // Java: this.z = z;
      self.z = z;
    }

    // Java: void set(Vector3D v)
    pub fn set_from_vector(&mut self, vx: f32, vy: f32, vz: f32) {
      // Java: this.x = v.vx;
      self.x = vx;
      // Java: this.y = v.vy;
      self.y = vy;
      // Java: this.z = v.vz;
      self.z = vz;
    }

    // Java: void normalize()
    pub fn normalize(&mut self) {
      // Java: double dist = Math.sqrt((double)(this.x * this.x + this.y * this.y + this.z * this.z));
      let dist = ((self.x * self.x + self.y * self.y + self.z * self.z) as f64).sqrt();
      // Java: if (dist != (double)0.0F)
      if dist != 0.0_f64
      {
         // Java: this.x = (float)((double)this.x / dist);
         self.x = (self.x as f64 / dist) as f32;
         // Java: this.y = (float)((double)this.y / dist);
         self.y = (self.y as f64 / dist) as f32;
         // Java: this.z = (float)((double)this.z / dist);
         self.z = (self.z as f64 / dist) as f32;
      }
    }

    // Java: float length()
    pub fn length(&self) -> f32 {
      // Java: return (float)Math.sqrt((double)(this.x * this.x + this.y * this.y + this.z * this.z));
      ((self.x * self.x + self.y * self.y + self.z * self.z) as f64).sqrt() as f32
    }

}
