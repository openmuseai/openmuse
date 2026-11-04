#![allow(non_snake_case, dead_code)]

use crate::cn::wps::moffice::drawing::effects::Point3D::Point3D;

// Java: public class Vector3D
pub struct Vector3D {
    // Java field: float vx
    pub(crate) vx: f32,
    // Java field: float vy
    pub(crate) vy: f32,
    // Java field: float vz
    pub(crate) vz: f32,
}

// Java: public class Vector3D
impl Vector3D {
    // Java: Vector3D()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.vx = this.vy = this.vz = 0.0F;
      Vector3D { vx: 0.0_f32, vy: 0.0_f32, vz: 0.0_f32 }
    }

    // Java: Vector3D(float x, float y, float z)
    pub fn new_xyz(x: f32, y: f32, z: f32) -> Self {
      // Java: super();
      // Java: this.vx = x;
      // Java: this.vy = y;
      // Java: this.vz = z;
      Vector3D { vx: x, vy: y, vz: z }
    }

    // Java: Vector3D(Point3D point)
    pub fn new_from_point(point: &Point3D) -> Self {
      // Java: super();
      // Java: this.vx = point.x;
      // Java: this.vy = point.y;
      // Java: this.vz = point.z;
      Vector3D { vx: point.x, vy: point.y, vz: point.z }
    }

    // Java: void set(float x, float y, float z)
    pub fn set(&mut self, x: f32, y: f32, z: f32) {
      // Java: this.vx = x;
      self.vx = x;
      // Java: this.vy = y;
      self.vy = y;
      // Java: this.vz = z;
      self.vz = z;
    }

    // Java: void set(Point3D point)
    pub fn set_from_point(&mut self, point: &Point3D) {
      // Java: this.vx = point.x;
      self.vx = point.x;
      // Java: this.vy = point.y;
      self.vy = point.y;
      // Java: this.vz = point.z;
      self.vz = point.z;
    }

    // Java: void normalize()
    pub fn normalize(&mut self) {
      // Java: double dist = Math.sqrt((double)(this.vx * this.vx + this.vy * this.vy + this.vz * this.vz));
      let dist = ((self.vx * self.vx + self.vy * self.vy + self.vz * self.vz) as f64).sqrt();
      // Java: if (dist != (double)0.0F)
      if dist != 0.0_f64
      {
         // Java: this.vx = (float)((double)this.vx / dist);
         self.vx = (self.vx as f64 / dist) as f32;
         // Java: this.vy = (float)((double)this.vy / dist);
         self.vy = (self.vy as f64 / dist) as f32;
         // Java: this.vz = (float)((double)this.vz / dist);
         self.vz = (self.vz as f64 / dist) as f32;
      }
    }

    // Java: float dot(Vector3D v)
    pub fn dot(&self, v: &Vector3D) -> f32 {
      // Java: return this.vx * v.vx + this.vy * v.vy + this.vz * v.vz;
      self.vx * v.vx + self.vy * v.vy + self.vz * v.vz
    }

    // Java: void inverse()
    pub fn inverse(&mut self) {
      // Java: this.vx = -this.vx;
      self.vx = -self.vx;
      // Java: this.vy = -this.vy;
      self.vy = -self.vy;
      // Java: this.vz = -this.vz;
      self.vz = -self.vz;
    }

    // Java: float length()
    pub fn length(&self) -> f32 {
      // Java: return (float)Math.sqrt((double)(this.vx * this.vx + this.vy * this.vy + this.vz * this.vz));
      ((self.vx * self.vx + self.vy * self.vy + self.vz * self.vz) as f64).sqrt() as f32
    }

    // Java: void scale(float scale)
    pub fn scale(&mut self, scale: f32) {
      // Java: this.vx *= scale;
      self.vx *= scale;
      // Java: this.vy *= scale;
      self.vy *= scale;
      // Java: this.vz *= scale;
      self.vz *= scale;
    }

}
