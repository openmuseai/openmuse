#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// Java: public class ColorMatrix
#[derive(Clone)]
pub struct ColorMatrix {
    // Java field: float[] mArray
    pub(crate) mArray: Vec<f32>,
}

struct System;
impl System {
    fn arraycopy(src: &[f32], src_pos: usize, dst: &mut [f32], dst_pos: usize, length: usize) {
        dst[dst_pos..dst_pos + length].copy_from_slice(&src[src_pos..src_pos + length]);
    }
}

// Java: public class ColorMatrix
impl ColorMatrix {
    // Java: ColorMatrix()
    pub fn new() -> Self {
        // Java: super();
        let mut instance = Self { mArray: vec![0.0; 20] };
        instance._init();
        instance
    }

    fn _init(&mut self) {
        // Java: this.reset();
        self.reset();
    }

    // Java: ColorMatrix(float[] src)
    pub fn from_array(src: &[f32]) -> Self {
        // Java: super();
        let mut instance = Self { mArray: vec![0.0f32; 20] };
        // Java: System.arraycopy(src, 0, this.mArray, 0, 20);
        System::arraycopy(src, 0, &mut instance.mArray, 0, 20);
        instance
    }

    // Java: ColorMatrix(ColorMatrix src)
    pub fn from_matrix(src: &ColorMatrix) -> Self {
        // Java: super();
        let mut instance = Self { mArray: vec![0.0f32; 20] };
        // Java: System.arraycopy(src.mArray, 0, this.mArray, 0, 20);
        System::arraycopy(&src.mArray, 0, &mut instance.mArray, 0, 20);
        instance
    }

    // Java: float[] getArray()
    pub fn getArray(&self) -> &[f32] {
        // Java: return this.mArray;
        &self.mArray
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: float[] a = this.mArray;
        let a = &mut self.mArray;
        // Java: for(int i = 19; i > 0; --i)
        for i in (1..20).rev() {
            // Java: a[i] = 0.0F;
            a[i] = 0.0;
        }
        // Java: a[0] = a[6] = a[12] = a[18] = 1.0F;
        a[0] = 1.0; a[6] = 1.0; a[12] = 1.0; a[18] = 1.0;
    }

    // Java: void set(ColorMatrix src)
    pub fn set_from_matrix(&mut self, src: &ColorMatrix) {
        // Java: System.arraycopy(src.mArray, 0, this.mArray, 0, 20);
        System::arraycopy(&src.mArray, 0, &mut self.mArray, 0, 20);
    }

    // Java: void set(float[] src)
    pub fn set_from_array(&mut self, src: &[f32]) {
        // Java: System.arraycopy(src, 0, this.mArray, 0, 20);
        System::arraycopy(src, 0, &mut self.mArray, 0, 20);
    }

    // Java: void setScale(float rScale, float gScale, float bScale, float aScale)
    pub fn setScale(&mut self, rScale: f32, gScale: f32, bScale: f32, aScale: f32) {
        // Java: float[] a = this.mArray;
        let a = &mut self.mArray;
        // Java: for(int i = 19; i > 0; --i)
        for i in (1..20).rev() {
            // Java: a[i] = 0.0F;
            a[i] = 0.0;
        }
        // Java: a[0] = rScale;
        a[0] = rScale;
        // Java: a[6] = gScale;
        a[6] = gScale;
        // Java: a[12] = bScale;
        a[12] = bScale;
        // Java: a[18] = aScale;
        a[18] = aScale;
    }

    // Java: void setRotate(int axis, float degrees)
    pub fn setRotate(&mut self, axis: i32, degrees: f32) {
        // Java: this.reset();
        self.reset();
        // Java: float radians = degrees * (float)Math.PI / 180.0F;
        let radians = degrees * std::f32::consts::PI / 180.0;
        // Java: float cosine = (float)Math.cos((double)radians);
        let cosine = (radians as f64).cos() as f32;
        // Java: float sine = (float)Math.sin((double)radians);
        let sine = (radians as f64).sin() as f32;
        // Java: switch (axis)
        'switch_axis: {
            match axis {
                // Java: case 0:
                0 => {
                    // Java: this.mArray[6] = this.mArray[12] = cosine;
                    self.mArray[6] = cosine; self.mArray[12] = cosine;
                    // Java: this.mArray[7] = sine;
                    self.mArray[7] = sine;
                    // Java: this.mArray[11] = -sine;
                    self.mArray[11] = -sine;
                    // Java: break;
                    break 'switch_axis;
                }
                // Java: case 1:
                1 => {
                    // Java: this.mArray[0] = this.mArray[12] = cosine;
                    self.mArray[0] = cosine; self.mArray[12] = cosine;
                    // Java: this.mArray[2] = -sine;
                    self.mArray[2] = -sine;
                    // Java: this.mArray[10] = sine;
                    self.mArray[10] = sine;
                    // Java: break;
                    break 'switch_axis;
                }
                // Java: case 2:
                2 => {
                    // Java: this.mArray[0] = this.mArray[6] = cosine;
                    self.mArray[0] = cosine; self.mArray[6] = cosine;
                    // Java: this.mArray[1] = sine;
                    self.mArray[1] = sine;
                    // Java: this.mArray[5] = -sine;
                    self.mArray[5] = -sine;
                    // Java: break;
                    break 'switch_axis;
                }
                // Java: default:
                _ => {
                    // Java: throw new RuntimeException();
                    panic!("Invalid axis");
                }
            }
        }
    }

    // Java: void setConcat(ColorMatrix matA, ColorMatrix matB)
    pub fn setConcat(&mut self, matA: &ColorMatrix, matB: &ColorMatrix) {
        // Java: float[] tmp = null;
        let mut tmp;
        // Java: if (matA != this && matB != this)
        if !std::ptr::eq(matA as *const _, self as *const ColorMatrix) && !std::ptr::eq(matB as *const _, self as *const ColorMatrix) {
            // Java: tmp = this.mArray;
            tmp = vec![0.0f32; 20];
        // Java: else
        } else {
            // Java: tmp = new float[20];
            tmp = vec![0.0f32; 20];
        }
        // Java: float[] a = matA.mArray;
        let a = &matA.mArray;
        // Java: float[] b = matB.mArray;
        let b = &matB.mArray;
        // Java: int index = 0;
        let mut index = 0usize;
        // Java: for(int j = 0; j < 20; j += 5)
        let mut j = 0;
        while j < 20 {
            // Java: for(int i = 0; i < 4 && index < tmp.length; ++i)
            let mut i = 0;
            while i < 4 && index < tmp.len() {
                // Java: tmp[index++] = a[j + 0] * b[i + 0] + a[j + 1] * b[i + 5] + a[j + 2] * b[i + 10] + a[j + 3] * b[i + 15];
                tmp[index] = a[j + 0] * b[i + 0] + a[j + 1] * b[i + 5] + a[j + 2] * b[i + 10] + a[j + 3] * b[i + 15];
                index += 1;
                i += 1;
            }
            // Java: if (index >= tmp.length)
            if index >= tmp.len() {
                // Java: break;
                break;
            }
            // Java: tmp[index++] = a[j + 0] * b[4] + a[j + 1] * b[9] + a[j + 2] * b[14] + a[j + 3] * b[19] + a[j + 4];
            tmp[index] = a[j + 0] * b[4] + a[j + 1] * b[9] + a[j + 2] * b[14] + a[j + 3] * b[19] + a[j + 4];
            index += 1;
            j += 5;
        }
        // Java: if (tmp != this.mArray)
        if tmp.as_ptr() != self.mArray.as_ptr() {
            // Java: System.arraycopy(tmp, 0, this.mArray, 0, 20);
            System::arraycopy(&tmp, 0, &mut self.mArray, 0, 20);
        }
    }

    // Java: void preConcat(ColorMatrix prematrix)
    pub fn preConcat(&mut self, prematrix: &ColorMatrix) {
        // Java: this.setConcat(this, prematrix);
        let self_copy = self.clone();
        self.setConcat(&self_copy, prematrix);
    }

    // Java: void postConcat(ColorMatrix postmatrix)
    pub fn postConcat(&mut self, postmatrix: &ColorMatrix) {
        // Java: this.setConcat(postmatrix, this);
        let self_copy = self.clone();
        self.setConcat(postmatrix, &self_copy);
    }

    // Java: void setSaturation(float sat)
    pub fn setSaturation(&mut self, sat: f32) {
        // Java: this.reset();
        self.reset();
        // Java: float[] m = this.mArray;
        let m = &mut self.mArray;
        // Java: float invSat = 1.0F - sat;
        let invSat = 1.0f32 - sat;
        // Java: float R = 0.213F * invSat;
        let R = 0.213f32 * invSat;
        // Java: float G = 0.715F * invSat;
        let G = 0.715f32 * invSat;
        // Java: float B = 0.072F * invSat;
        let B = 0.072f32 * invSat;
        // Java: m[0] = R + sat;
        m[0] = R + sat;
        // Java: m[1] = G;
        m[1] = G;
        // Java: m[2] = B;
        m[2] = B;
        // Java: m[5] = R;
        m[5] = R;
        // Java: m[6] = G + sat;
        m[6] = G + sat;
        // Java: m[7] = B;
        m[7] = B;
        // Java: m[10] = R;
        m[10] = R;
        // Java: m[11] = G;
        m[11] = G;
        // Java: m[12] = B + sat;
        m[12] = B + sat;
    }

    // Java: void setRGB2YUV()
    pub fn setRGB2YUV(&mut self) {
        // Java: this.reset();
        self.reset();
        // Java: float[] m = this.mArray;
        let m = &mut self.mArray;
        // Java: m[0] = 0.299F;
        m[0] = 0.299;
        // Java: m[1] = 0.587F;
        m[1] = 0.587;
        // Java: m[2] = 0.114F;
        m[2] = 0.114;
        // Java: m[5] = -0.16874F;
        m[5] = -0.16874;
        // Java: m[6] = -0.33126F;
        m[6] = -0.33126;
        // Java: m[7] = 0.5F;
        m[7] = 0.5;
        // Java: m[10] = 0.5F;
        m[10] = 0.5;
        // Java: m[11] = -0.41869F;
        m[11] = -0.41869;
        // Java: m[12] = -0.08131F;
        m[12] = -0.08131;
    }

    // Java: void setYUV2RGB()
    pub fn setYUV2RGB(&mut self) {
        // Java: this.reset();
        self.reset();
        // Java: float[] m = this.mArray;
        let m = &mut self.mArray;
        // Java: m[2] = 1.402F;
        m[2] = 1.402;
        // Java: m[5] = 1.0F;
        m[5] = 1.0;
        // Java: m[6] = -0.34414F;
        m[6] = -0.34414;
        // Java: m[7] = -0.71414F;
        m[7] = -0.71414;
        // Java: m[10] = 1.0F;
        m[10] = 1.0;
        // Java: m[11] = 1.772F;
        m[11] = 1.772;
        // Java: m[12] = 0.0F;
        m[12] = 0.0;
    }
}
