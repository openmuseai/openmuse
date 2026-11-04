#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, non_camel_case_types)]

// Java: import android.graphics.Matrix
// Java: import android.graphics.Region
// Java: import android.graphics.Region.Op
// Java: import cn.wps.graphics.PointF

use std::sync::Mutex;

#[derive(Clone)]
pub struct Matrix {
    values: [f32; 9],
}

impl Matrix {
    pub fn new() -> Self { Matrix { values: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0] } }
    pub fn from_matrix(m: &Matrix) -> Self { m.clone() }
    pub fn setValues(&mut self, v: &[f32; 9]) { self.values = *v; }
    pub fn getValues(&self, v: &mut [f32; 9]) { *v = self.values; }
    pub fn mapPoints(&self, pts: &mut [f32]) {
        if pts.len() >= 4 {
            let x0 = pts[0]; let y0 = pts[1];
            let x1 = pts[2]; let y1 = pts[3];
            pts[0] = self.values[0] * x0 + self.values[1] * y0 + self.values[2];
            pts[1] = self.values[3] * x0 + self.values[4] * y0 + self.values[5];
            pts[2] = self.values[0] * x1 + self.values[1] * y1 + self.values[2];
            pts[3] = self.values[3] * x1 + self.values[4] * y1 + self.values[5];
        }
    }
    pub fn preRotate(&mut self, _degrees: f32) {}
}

pub struct PointF { pub x: f32, pub y: f32 }
pub struct Rectangle { pub x: i32, pub y: i32, pub width: i32, pub height: i32 }
pub struct GdiPath;
impl GdiPath {
    pub fn new() -> Self { GdiPath }
    pub fn moveTo(&mut self, _x: f32, _y: f32) {}
    pub fn lineTo(&mut self, _x: f32, _y: f32) {}
    pub fn arcTo(&mut self, _bounds: &Rectangle, _start: f32, _sweep: f32) {}
    pub fn closePath(&mut self) {}
}
pub struct ColorPaint { pub color: i32 }
impl ColorPaint { pub fn create(color: i32) -> Self { ColorPaint { color } } }
pub struct GDILogBrush { style: i32, color: ColorPaint }
impl GDILogBrush { pub fn new(style: i32, color: ColorPaint) -> Self { GDILogBrush { style, color } } }
pub struct GdiPen { style: i32, width: i32, color: ColorPaint }
impl GdiPen { pub fn new(style: i32, width: i32, color: ColorPaint) -> Self { GdiPen { style, width, color } } }
pub struct GDIFont { name: String, style: i32 }
impl GDIFont { pub fn new(name: String, style: i32) -> Self { GDIFont { name, style } } }
pub struct LogFont { font: GDIFont }
impl LogFont { pub fn new(font: GDIFont) -> Self { LogFont { font } } }

#[derive(Clone, Copy, PartialEq)]
pub enum Op { INTERSECT, UNION, XOR, DIFFERENCE }

// Java field: float[] matrixFloatValues
static MATRIX_FLOAT_VALUES: Mutex<[f32; 9]> = Mutex::new([0.0; 9]);
// Java field: int RGN_AND
pub(crate) const RGN_AND: i32 = 1;
// Java field: int RGN_OR
pub(crate) const RGN_OR: i32 = 2;
// Java field: int RGN_XOR
pub(crate) const RGN_XOR: i32 = 3;
// Java field: int RGN_DIFF
pub(crate) const RGN_DIFF: i32 = 4;
// Java field: int RGN_COPY
pub(crate) const RGN_COPY: i32 = 5;

// Java: public class GdiTool
pub struct GdiTool {
}

// Java: public class GdiTool
impl GdiTool {
    // Java: GdiTool()
    pub fn new() -> Self {
        // Java: super();
        GdiTool {}
    }

    // Java: static Matrix createMatrix(float m00, float m10, float m01, float m11, float m02, float m12)
    pub fn createMatrix(m00: f32, m10: f32, m01: f32, m11: f32, m02: f32, m12: f32) -> Matrix {
        // Java: Matrix matrix = new Matrix();
        let mut matrix = Matrix::new();
        let mut mfv = MATRIX_FLOAT_VALUES.lock().unwrap();
        // Java: matrixFloatValues[0] = m00;
        mfv[0] = m00;
        // Java: matrixFloatValues[1] = m01;
        mfv[1] = m01;
        // Java: matrixFloatValues[2] = m02;
        mfv[2] = m02;
        // Java: matrixFloatValues[3] = m10;
        mfv[3] = m10;
        // Java: matrixFloatValues[4] = m11;
        mfv[4] = m11;
        // Java: matrixFloatValues[5] = m12;
        mfv[5] = m12;
        // Java: matrixFloatValues[6] = 0.0F;
        mfv[6] = 0.0;
        // Java: matrixFloatValues[7] = 0.0F;
        mfv[7] = 0.0;
        // Java: matrixFloatValues[8] = 1.0F;
        mfv[8] = 1.0;
        // Java: matrix.setValues(matrixFloatValues);
        matrix.setValues(&*mfv);
        // Java: return matrix;
        matrix
    }

    // Java: static float getRotation(Matrix matrix)
    pub fn getRotation(matrix: &Matrix) -> f32 {
        // Java: float[] pts = new float[]{0.0F, 0.0F, 0.0F, 1.0F};
        let mut pts = [0.0f32, 0.0, 0.0, 1.0];
        // Java: matrix.mapPoints(pts);
        matrix.mapPoints(&mut pts);
        // Java: float dx = pts[2] - pts[0];
        let dx = pts[2] - pts[0];
        // Java: float dy = pts[3] - pts[1];
        let dy = pts[3] - pts[1];
        // Java: return Math.abs(dx) < 1.0E-6F ? 0.0F : (float)(Math.toDegrees(Math.atan((double)(dy / dx))) - (double)90.0F);
        if dx.abs() < 1.0e-6 { 0.0 } else { ((dy as f64 / dx as f64).atan().to_degrees() - 90.0) as f32 }
    }

    // Java: static float getScale(Matrix matrix)
    pub fn getScale(matrix: &Matrix) -> f32 {
        // Java: float rot = getRotation(matrix);
        let rot = GdiTool::getRotation(matrix);
        let mut mfv = MATRIX_FLOAT_VALUES.lock().unwrap();
        // Java: if ((int)rot == 0)
        if rot as i32 == 0 {
            // Java: matrix.getValues(matrixFloatValues);
            matrix.getValues(&mut *mfv);
        }
        // Java: else
        else {
            // Java: Matrix rotMatrix = new Matrix(matrix);
            let mut rotMatrix = Matrix::from_matrix(matrix);
            // Java: rotMatrix.preRotate(-rot);
            rotMatrix.preRotate(-rot);
            // Java: rotMatrix.getValues(matrixFloatValues);
            rotMatrix.getValues(&mut *mfv);
        }
        // Java: return Math.max(Math.abs(matrixFloatValues[0]), Math.abs(matrixFloatValues[4]));
        mfv[0].abs().max(mfv[4].abs())
    }

    // Java: static float getScaleX(Matrix matrix)
    pub fn getScaleX(matrix: &Matrix) -> f32 {
        let mut mfv = MATRIX_FLOAT_VALUES.lock().unwrap();
        // Java: matrix.getValues(matrixFloatValues);
        matrix.getValues(&mut *mfv);
        // Java: return matrixFloatValues[0];
        mfv[0]
    }

    // Java: static float getSkewX(Matrix matrix)
    pub fn getSkewX(matrix: &Matrix) -> f32 {
        let mut mfv = MATRIX_FLOAT_VALUES.lock().unwrap();
        // Java: matrix.getValues(matrixFloatValues);
        matrix.getValues(&mut *mfv);
        // Java: return matrixFloatValues[1];
        mfv[1]
    }

    // Java: static float getScaleY(Matrix matrix)
    pub fn getScaleY(matrix: &Matrix) -> f32 {
        let mut mfv = MATRIX_FLOAT_VALUES.lock().unwrap();
        // Java: matrix.getValues(matrixFloatValues);
        matrix.getValues(&mut *mfv);
        // Java: return matrixFloatValues[4];
        mfv[4]
    }

    // Java: static float getSkewY(Matrix matrix)
    pub fn getSkewY(matrix: &Matrix) -> f32 {
        let mut mfv = MATRIX_FLOAT_VALUES.lock().unwrap();
        // Java: matrix.getValues(matrixFloatValues);
        matrix.getValues(&mut *mfv);
        // Java: return matrixFloatValues[3];
        mfv[3]
    }

    // Java: static void initPath(GdiPath path, int[] points, int size)
    pub fn initPath_no_offset(path: &mut GdiPath, points: &[i32], size: i32) {
        // Java: initPath(path, points, 0, size);
        GdiTool::initPath(path, points, 0, size);
    }

    // Java: static void initPath(GdiPath path, int[] points, int offset, int size)
    pub fn initPath(path: &mut GdiPath, points: &[i32], offset: i32, size: i32) {
        let off = offset as usize;
        let sz = size as usize;
        // Java: if (points != null && points.length != 0 && offset + size <= points.length)
        if !points.is_empty() && off + sz <= points.len() {
            // Java: path.moveTo((float)points[offset], (float)points[offset + 1]);
            path.moveTo(points[off] as f32, points[off + 1] as f32);
            // Java: for(int i = 2; i < size; i += 2)
            let mut i = 2usize;
            while i < sz {
                // Java: path.lineTo((float)points[offset + i], (float)points[offset + i + 1]);
                path.lineTo(points[off + i] as f32, points[off + i + 1] as f32);
                i += 2;
            }
        }
    }

    // Java: static GdiPath createPath(Rectangle bounds, PointF start, PointF end)
    pub fn createPath_arc(bounds: &Rectangle, start: &PointF, end: &PointF) -> GdiPath {
        // Java: return createPath(bounds, start, end, false);
        GdiTool::createPath_arc_pie(bounds, start, end, false)
    }

    // Java: static GdiPath createPath(Rectangle bounds, PointF start, PointF end, boolean isPie)
    pub fn createPath_arc_pie(bounds: &Rectangle, start: &PointF, end: &PointF, isPie: bool) -> GdiPath {
        // Java: int centerX = bounds.x + bounds.width / 2;
        let centerX = bounds.x + bounds.width / 2;
        // Java: int centerY = bounds.y + bounds.height / 2;
        let centerY = bounds.y + bounds.height / 2;
        // Java: int startAngle = (int)(Math.atan2((double)(start.y - (float)centerY), (double)(start.x - (float)centerX)) * (double)180.0F / Math.PI);
        let startAngle = ((start.y - centerY as f32) as f64).atan2((start.x - centerX as f32) as f64).to_degrees() as i32;
        // Java: int sweepAngle = (int)(Math.atan2((double)(end.y - (float)centerY), (double)(end.x - (float)centerX)) * (double)180.0F / Math.PI - (double)startAngle);
        let mut sweepAngle = (((end.y - centerY as f32) as f64).atan2((end.x - centerX as f32) as f64).to_degrees() - startAngle as f64) as i32;
        // Java: if (isPie)
        if isPie {
            // Java: if (sweepAngle > 0)
            if sweepAngle > 0 {
                // Java: sweepAngle -= 360;
                sweepAngle -= 360;
            }
        }
        // Java: else
        else {
            // Java: if (sweepAngle > 360)
            if sweepAngle > 360 {
                // Java: sweepAngle -= 360;
                sweepAngle -= 360;
            }
            // Java: if (sweepAngle < 0)
            if sweepAngle < 0 {
                // Java: sweepAngle += 360;
                sweepAngle += 360;
            }
        }
        // Java: GdiPath path = new GdiPath();
        let mut path = GdiPath::new();
        // Java: if (isPie)
        if isPie {
            // Java: path.moveTo((float)centerX, (float)centerY);
            path.moveTo(centerX as f32, centerY as f32);
            // Java: path.arcTo(bounds, (float)startAngle, (float)sweepAngle);
            path.arcTo(bounds, startAngle as f32, sweepAngle as f32);
            // Java: path.closePath();
            path.closePath();
        }
        // Java: else
        else {
            // Java: path.arcTo(bounds, (float)startAngle, (float)sweepAngle);
            path.arcTo(bounds, startAngle as f32, sweepAngle as f32);
        }
        // Java: return path;
        path
    }

    // Java: static GDILogBrush createBrush(int color)
    pub fn createBrush_int(color: i32) -> GDILogBrush {
        // Java: return new GDILogBrush(0, ColorPaint.create(color));
        GDILogBrush::new(0, ColorPaint::create(color))
    }

    // Java: static GDILogBrush createBrush(ColorPaint color)
    pub fn createBrush(color: ColorPaint) -> GDILogBrush {
        // Java: return new GDILogBrush(0, color);
        GDILogBrush::new(0, color)
    }

    // Java: static GdiPen createPen(int color)
    pub fn createPen_int(color: i32) -> GdiPen {
        // Java: return new GdiPen(0, 1, ColorPaint.create(color));
        GdiPen::new(0, 1, ColorPaint::create(color))
    }

    // Java: static GdiPen createPen(ColorPaint color)
    pub fn createPen(color: ColorPaint) -> GdiPen {
        // Java: return new GdiPen(0, 1, color);
        GdiPen::new(0, 1, color)
    }

    // Java: static LogFont createFont(String name)
    pub fn createFont(name: String) -> LogFont {
        // Java: return new LogFont(new GDIFont(name, 0));
        LogFont::new(GDIFont::new(name, 0))
    }

    // Java: static Region.Op getOpByMode(int mode)
    pub fn getOpByMode(mode: i32) -> Op {
        match mode {
            1 => Op::INTERSECT,
            2 => Op::UNION,
            3 => Op::XOR,
            4 => Op::DIFFERENCE,
            5 => Op::UNION,
            _ => Op::UNION,
        }
    }

    // Java: static float[] getDash(int dashStyle, float[] dash)
    pub fn getDash(dashStyle: i32, dash: Option<Vec<f32>>) -> Option<Vec<f32>> {
        // Java: switch (dashStyle)
        match dashStyle {
            // Java: case 1:
            1 => {
                // Java: return new float[]{5.0F, 5.0F};
                Some(vec![5.0, 5.0])
            }
            // Java: case 2:
            2 => {
                // Java: return new float[]{1.0F, 2.0F};
                Some(vec![1.0, 2.0])
            }
            // Java: case 3:
            3 => {
                // Java: return new float[]{5.0F, 2.0F, 1.0F, 2.0F};
                Some(vec![5.0, 2.0, 1.0, 2.0])
            }
            // Java: case 4:
            4 => {
                // Java: return new float[]{5.0F, 2.0F, 1.0F, 2.0F, 1.0F, 2.0F};
                Some(vec![5.0, 2.0, 1.0, 2.0, 1.0, 2.0])
            }
            // Java: case 5:
            // Java: case 6:
            // Java: default:
            5 | 6 => {
                // Java: return null;
                None
            }
            // Java: case 7:
            7 => {
                // Java: return dash;
                dash
            }
            _ => None,
        }
    }
}
