#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.base.assertion.KSAssert
use core_libs_util::base::assertion::KSAssert::KSAssert;

use crate::cn::wps::moffice::drawing::effects::Point3D::Point3D;

// Java field: int row
pub(crate) const row: i32 = 4;
// Java field: int col
pub(crate) const col: i32 = 4;

// Java: public class Matrix3D
pub struct Matrix3D {
    // Java field: float[] _data
    pub(crate) _data: Vec<f32>,
}

// Java: public class Matrix3D
impl Matrix3D {
    // Java: Matrix3D()
    pub fn new() -> Self {
      // Java: super();
      let mut instance = Matrix3D { _data: vec![0.0_f32; 16] };
      instance.init_new();
      instance
    }

    fn init_new(&mut self) {
      // Java: this._data = new float[16];
      self._data = vec![0.0_f32; 16];
      // Java: this.reset();
      self.reset();
    }

    // Java: Matrix3D(Matrix3D matrix)
    pub fn new_from_matrix(matrix: &Matrix3D) -> Self {
      // Java: super();
      let mut instance = Matrix3D { _data: vec![0.0_f32; 16] };
      instance.init_from_matrix(matrix);
      instance
    }

    fn init_from_matrix(&mut self, matrix: &Matrix3D) {
      // Java: this._data = new float[16];
      self._data = vec![0.0_f32; 16];
      // Java: int size = 16;
      let size = 16;
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: this._data[i] = matrix._data[i];
         self._data[i] = matrix._data[i];
      }
    }

    // Java: Matrix3D(float[] m)
    pub fn new_from_array(m: Vec<f32>) -> Self {
      // Java: super();
      let mut instance = Matrix3D { _data: vec![0.0_f32; 16] };
      instance.init_from_array(m);
      instance
    }

    fn init_from_array(&mut self, m: Vec<f32>) {
      // Java: this._data = new float[16];
      self._data = vec![0.0_f32; 16];
      // Java: this.setMatrix(m);
      self.setMatrix(m);
    }

    // Java: Matrix3D(float m11, float m12, float m13, float m14, float m21, float m22, float m23, float m24, float m31, float m32, float m33, float m34, float m41, float m42, float m43, float m44)
    pub fn new_from_values(m11: f32, m12: f32, m13: f32, m14: f32, m21: f32, m22: f32, m23: f32, m24: f32, m31: f32, m32: f32, m33: f32, m34: f32, m41: f32, m42: f32, m43: f32, m44: f32) -> Self {
      // Java: super();
      let mut instance = Matrix3D { _data: vec![0.0_f32; 16] };
      instance.init_from_values(m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, m41, m42, m43, m44);
      instance
    }

    fn init_from_values(&mut self, m11: f32, m12: f32, m13: f32, m14: f32, m21: f32, m22: f32, m23: f32, m24: f32, m31: f32, m32: f32, m33: f32, m34: f32, m41: f32, m42: f32, m43: f32, m44: f32) {
      // Java: this._data = new float[16];
      self._data = vec![0.0_f32; 16];
      // Java: int i = 0;
      let mut i = 0usize;
      // Java: this._data[i++] = m11;
      self._data[i] = m11; i += 1;
      // Java: this._data[i++] = m12;
      self._data[i] = m12; i += 1;
      // Java: this._data[i++] = m13;
      self._data[i] = m13; i += 1;
      // Java: this._data[i++] = m14;
      self._data[i] = m14; i += 1;
      // Java: this._data[i++] = m21;
      self._data[i] = m21; i += 1;
      // Java: this._data[i++] = m22;
      self._data[i] = m22; i += 1;
      // Java: this._data[i++] = m23;
      self._data[i] = m23; i += 1;
      // Java: this._data[i++] = m24;
      self._data[i] = m24; i += 1;
      // Java: this._data[i++] = m31;
      self._data[i] = m31; i += 1;
      // Java: this._data[i++] = m32;
      self._data[i] = m32; i += 1;
      // Java: this._data[i++] = m33;
      self._data[i] = m33; i += 1;
      // Java: this._data[i++] = m34;
      self._data[i] = m34; i += 1;
      // Java: this._data[i++] = m41;
      self._data[i] = m41; i += 1;
      // Java: this._data[i++] = m42;
      self._data[i] = m42; i += 1;
      // Java: this._data[i++] = m43;
      self._data[i] = m43; i += 1;
      // Java: this._data[i++] = m44;
      self._data[i] = m44;
    }

    // Java: void reset()
    pub fn reset(&mut self) {
      // Java: for(int i = 0; i < 4; ++i)
      for i in 0..4
      {
         // Java: for(int j = 0; j < 4; ++j)
         for j in 0..4
         {
            // Java: this._data[4 * i + j] = i == j ? 1.0F : 0.0F;
            self._data[4 * i + j] = if i == j { 1.0_f32 } else { 0.0_f32 };
         }
      }
    }

    // Java: float[] getMatrix()
    pub fn getMatrix(&self) -> &Vec<f32> {
      // Java: return this._data;
      &self._data
    }

    // Java: void setMatrix(float[] m)
    pub fn setMatrix(&mut self, m: Vec<f32>) {
      // Java: KSAssert.assertEquals(16, m.length);
      KSAssert::assertEquals(16, m.len() as i32);
      // Java: this._data = m;
      self._data = m;
    }

    // Java: void translate(float x, float y, float z)
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
      // Java: for(int i = 0; i < 4; ++i)
      for i in 0..4 {
         // Java: float[] var10000 = this._data;
         let var10000 = &mut self._data;
         // Java: var10000[12 + i] += this._data[i] * x + this._data[4 + i] * y + this._data[8 + i] * z;
         let val = var10000[i] * x + var10000[4 + i] * y + var10000[8 + i] * z;
         var10000[12 + i] += val;
      }
    }

    // Java: void scale(float sx, float sy, float sz)
    pub fn scale(&mut self, sx: f32, sy: f32, sz: f32) {
      // Java: Matrix3D matrix = new Matrix3D();
      let mut matrix = Matrix3D::new();
      // Java: matrix._data[0] = sx;
      matrix._data[0] = sx;
      // Java: matrix._data[5] = sy;
      matrix._data[5] = sy;
      // Java: matrix._data[10] = sz;
      matrix._data[10] = sz;
      // Java: this.multiply(matrix);
      self.multiply_matrix(&matrix);
    }

    // Java: void rotateX(float rot)
    pub fn rotateX(&mut self, rot: f32) {
      // Java: double r = Math.toRadians((double)rot);
      let r = (rot as f64).to_radians();
      // Java: float cosR = (float)Math.cos(r);
      let cosR = r.cos() as f32;
      // Java: float sinR = (float)Math.sin(r);
      let sinR = r.sin() as f32;
      // Java: Matrix3D matrix = new Matrix3D();
      let mut matrix = Matrix3D::new();
      // Java: matrix._data[5] = cosR;
      matrix._data[5] = cosR;
      // Java: matrix._data[6] = sinR;
      matrix._data[6] = sinR;
      // Java: matrix._data[9] = -sinR;
      matrix._data[9] = -sinR;
      // Java: matrix._data[10] = cosR;
      matrix._data[10] = cosR;
      // Java: this.multiply(matrix);
      self.multiply_matrix(&matrix);
    }

    // Java: void rotateY(float rot)
    pub fn rotateY(&mut self, rot: f32) {
      // Java: double r = Math.toRadians((double)rot);
      let r = (rot as f64).to_radians();
      // Java: float cosR = (float)Math.cos(r);
      let cosR = r.cos() as f32;
      // Java: float sinR = (float)Math.sin(r);
      let sinR = r.sin() as f32;
      // Java: Matrix3D matrix = new Matrix3D();
      let mut matrix = Matrix3D::new();
      // Java: matrix._data[0] = cosR;
      matrix._data[0] = cosR;
      // Java: matrix._data[2] = -sinR;
      matrix._data[2] = -sinR;
      // Java: matrix._data[8] = sinR;
      matrix._data[8] = sinR;
      // Java: matrix._data[10] = cosR;
      matrix._data[10] = cosR;
      // Java: this.multiply(matrix);
      self.multiply_matrix(&matrix);
    }

    // Java: void rotateZ(float rot)
    pub fn rotateZ(&mut self, rot: f32) {
      // Java: double r = Math.toRadians((double)rot);
      let r = (rot as f64).to_radians();
      // Java: float cosR = (float)Math.cos(r);
      let cosR = r.cos() as f32;
      // Java: float sinR = (float)Math.sin(r);
      let sinR = r.sin() as f32;
      // Java: Matrix3D matrix = new Matrix3D();
      let mut matrix = Matrix3D::new();
      // Java: matrix._data[0] = cosR;
      matrix._data[0] = cosR;
      // Java: matrix._data[1] = sinR;
      matrix._data[1] = sinR;
      // Java: matrix._data[4] = -sinR;
      matrix._data[4] = -sinR;
      // Java: matrix._data[5] = cosR;
      matrix._data[5] = cosR;
      // Java: this.multiply(matrix);
      self.multiply_matrix(&matrix);
    }

    // Java: void multiply(Matrix3D matrix)
    pub fn multiply_matrix(&mut self, matrix: &Matrix3D) {
      // Java: this.multiply(matrix._data);
      self.multiply_array(&matrix._data.clone());
    }

    // Java: void multiply(float[] m)
    pub fn multiply_array(&mut self, m: &[f32]) {
      // Java: float[] datas = new float[16];
      let mut datas = vec![0.0_f32; 16];
      // Java: for(int i = 0; i < 4; ++i)
      for i in 0..4
      {
         // Java: for(int j = 0; j < 4; ++j)
         for j in 0..4
         {
            // Java: float data = 0.0F;
            let mut data = 0.0_f32;
            // Java: for(int k = 0; k < 4; ++k)
            for k in 0..4
            {
               // Java: data += m[i * 4 + k] * this._data[k * 4 + j];
               data += m[i * 4 + k] * self._data[k * 4 + j];
            }
            // Java: datas[i * 4 + j] = data;
            datas[i * 4 + j] = data;
         }
      }
      // Java: this._data = datas;
      self._data = datas;
    }

    // Java: static Matrix3D multiply(Matrix3D matrix1, Matrix3D matrix2)
    pub fn multiply_static(matrix1: &Matrix3D, matrix2: &Matrix3D) -> Matrix3D {
      // Java: Matrix3D result = new Matrix3D();
      let mut result = Matrix3D::new();
      // Java: for(int i = 0; i < 4; ++i)
      for i in 0..4
      {
         // Java: for(int j = 0; j < 4; ++j)
         for j in 0..4
         {
            // Java: float data = 0.0F;
            let mut data = 0.0_f32;
            // Java: for(int k = 0; k < 4; ++k)
            for k in 0..4
            {
               // Java: data += matrix2._data[i * 4 + k] * matrix1._data[k * 4 + j];
               data += matrix2._data[i * 4 + k] * matrix1._data[k * 4 + j];
            }
            // Java: result._data[i * 4 + j] = data;
            result._data[i * 4 + j] = data;
         }
      }
      // Java: return result;
      result
    }

    // Java: void mapPoints(float[] pts)
    pub fn mapPoints(&self, pts: &mut [f32]) {
      // Java: int size = pts.length / 3;
      let size = pts.len() / 3;
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size
      {
         // Java: int index = 3 * i;
         let index = 3 * i;
         // Java: float x = pts[index];
         let x = pts[index];
         // Java: float y = pts[index + 1];
         let y = pts[index + 1];
         // Java: float z = pts[index + 2];
         let z = pts[index + 2];
         // Java: pts[index] = x * this._data[0] + y * this._data[4] + z * this._data[8] + 1.0F * this._data[12];
         pts[index] = x * self._data[0] + y * self._data[4] + z * self._data[8] + 1.0_f32 * self._data[12];
         // Java: pts[index + 1] = x * this._data[1] + y * this._data[5] + z * this._data[9] + 1.0F * this._data[13];
         pts[index + 1] = x * self._data[1] + y * self._data[5] + z * self._data[9] + 1.0_f32 * self._data[13];
         // Java: pts[index + 2] = x * this._data[2] + y * this._data[6] + z * this._data[10] + 1.0F * this._data[14];
         pts[index + 2] = x * self._data[2] + y * self._data[6] + z * self._data[10] + 1.0_f32 * self._data[14];
      }
    }

    // Java: void mapPoints(float[] dst, float[] src)
    pub fn mapPoints_dst_src(&self, dst: &mut [f32], src: &[f32]) {
      // Java: if (dst.length == src.length)
      if dst.len() == src.len() {
         // Java: int size = src.length / 3;
         let size = src.len() / 3;
         // Java: for(int i = 0; i < size; ++i)
         for i in 0..size
         {
            // Java: int index = 3 * i;
            let index = 3 * i;
            // Java: float x = src[index];
            let x = src[index];
            // Java: float y = src[index + 1];
            let y = src[index + 1];
            // Java: float z = src[index + 2];
            let z = src[index + 2];
            // Java: dst[index] = x * this._data[0] + y * this._data[4] + z * this._data[8] + 1.0F * this._data[12];
            dst[index] = x * self._data[0] + y * self._data[4] + z * self._data[8] + 1.0_f32 * self._data[12];
            // Java: dst[index + 1] = x * this._data[1] + y * this._data[5] + z * this._data[9] + 1.0F * this._data[13];
            dst[index + 1] = x * self._data[1] + y * self._data[5] + z * self._data[9] + 1.0_f32 * self._data[13];
            // Java: dst[index + 2] = x * this._data[2] + y * this._data[6] + z * this._data[10] + 1.0F * this._data[14];
            dst[index + 2] = x * self._data[2] + y * self._data[6] + z * self._data[10] + 1.0_f32 * self._data[14];
         }
      }
    }

    // Java: float[] mapPoints(float[] pts, float z)
    pub fn mapPoints_with_z(&self, pts: &[f32], z: f32) -> Vec<f32> {
      // Java: int size = pts.length / 2;
      let size = pts.len() / 2;
      // Java: float[] result = new float[size * 3];
      let mut result = vec![0.0_f32; size * 3];
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size
      {
         // Java: float x = pts[2 * i];
         let x = pts[2 * i];
         // Java: float y = pts[2 * i + 1];
         let y = pts[2 * i + 1];
         // Java: result[3 * i] = x * this._data[0] + y * this._data[4] + z * this._data[8] + 1.0F * this._data[12];
         result[3 * i] = x * self._data[0] + y * self._data[4] + z * self._data[8] + 1.0_f32 * self._data[12];
         // Java: result[3 * i + 1] = x * this._data[1] + y * this._data[5] + z * this._data[9] + 1.0F * this._data[13];
         result[3 * i + 1] = x * self._data[1] + y * self._data[5] + z * self._data[9] + 1.0_f32 * self._data[13];
         // Java: result[3 * i + 2] = x * this._data[2] + y * this._data[6] + z * this._data[10] + 1.0F * this._data[14];
         result[3 * i + 2] = x * self._data[2] + y * self._data[6] + z * self._data[10] + 1.0_f32 * self._data[14];
      }
      // Java: return result;
      result
    }

    // Java: Point3D mapPoint(Point3D point)
    pub fn mapPoint_from_point(&self, point: &Point3D) -> Point3D {
      // Java: return this.mapPoint(point.x, point.y, point.z);
      self.mapPoint_xyz(point.x, point.y, point.z)
    }

    // Java: Point3D mapPoint(float x, float y, float z)
    pub fn mapPoint_xyz(&self, x: f32, y: f32, z: f32) -> Point3D {
      // Java: Point3D result = new Point3D();
      let mut result = Point3D::new();
      // Java: result.x = x * this._data[0] + y * this._data[4] + z * this._data[8] + 1.0F * this._data[12];
      result.x = x * self._data[0] + y * self._data[4] + z * self._data[8] + 1.0_f32 * self._data[12];
      // Java: result.y = x * this._data[1] + y * this._data[5] + z * this._data[9] + 1.0F * this._data[13];
      result.y = x * self._data[1] + y * self._data[5] + z * self._data[9] + 1.0_f32 * self._data[13];
      // Java: result.z = x * this._data[2] + y * this._data[6] + z * this._data[10] + 1.0F * this._data[14];
      result.z = x * self._data[2] + y * self._data[6] + z * self._data[10] + 1.0_f32 * self._data[14];
      // Java: return result;
      result
    }

    // Java: void mapPoint(Point3D src, Point3D dst)
    pub fn mapPoint_src_dst(&self, src: &Point3D, dst: &mut Point3D) {
      // Java: dst.x = src.x * this._data[0] + src.y * this._data[4] + src.z * this._data[8] + 1.0F * this._data[12];
      dst.x = src.x * self._data[0] + src.y * self._data[4] + src.z * self._data[8] + 1.0_f32 * self._data[12];
      // Java: dst.y = src.x * this._data[1] + src.y * this._data[5] + src.z * this._data[9] + 1.0F * this._data[13];
      dst.y = src.x * self._data[1] + src.y * self._data[5] + src.z * self._data[9] + 1.0_f32 * self._data[13];
      // Java: dst.z = src.x * this._data[2] + src.y * this._data[6] + src.z * this._data[10] + 1.0F * this._data[14];
      dst.z = src.x * self._data[2] + src.y * self._data[6] + src.z * self._data[10] + 1.0_f32 * self._data[14];
    }

}
