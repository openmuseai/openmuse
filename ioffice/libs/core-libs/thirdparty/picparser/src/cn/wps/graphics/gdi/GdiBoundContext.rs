#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import android.graphics.Matrix
// Java: import cn.wps.graphics.PointF

use crate::cn::wps::graphics::gdi::Size::Size;

#[derive(Clone)]
pub struct Matrix {
    values: [f32; 9],
}

impl Matrix {
    pub fn new() -> Self { Matrix { values: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0] } }
    pub fn from_matrix(m: &Matrix) -> Self { m.clone() }
    pub fn set(&mut self, other: &Matrix) { self.values = other.values; }
    pub fn reset(&mut self) { self.values = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]; }
    pub fn getValues(&self, out: &mut [f32; 9]) { *out = self.values; }
    pub fn preConcat(&mut self, _other: &Matrix) {}
    pub fn postConcat(&mut self, _other: &Matrix) {}
    pub fn preTranslate(&mut self, dx: f32, dy: f32) { self.values[2] += dx; self.values[5] += dy; }
    pub fn preScale(&mut self, sx: f32, sy: f32) { self.values[0] *= sx; self.values[4] *= sy; }
    pub fn preRotate(&mut self, _degrees: f32) {}
}

#[derive(Clone)]
pub struct PointF { pub x: f32, pub y: f32 }
impl PointF {
    pub fn new(x: f32, y: f32) -> Self { PointF { x, y } }
    pub fn set(&mut self, x: f32, y: f32) { self.x = x; self.y = y; }
}

fn gdi_tool_get_scale_x(m: &Matrix) -> f32 { m.values[0] }
fn gdi_tool_get_scale_y(m: &Matrix) -> f32 { m.values[4] }

// Java: public class GdiBoundContext implements Cloneable
#[derive(Clone)]
pub struct GdiBoundContext {
    // Java field: Matrix srcTransform
    pub(crate) srcTransform: Matrix,
    // Java field: Size mWindowSize
    mWindowSize: Size,
    // Java field: Size mViewportSize
    mViewportSize: Size,
    // Java field: PointF mWindowOffset
    mWindowOffset: PointF,
    // Java field: PointF mViewportOffset
    mViewportOffset: PointF,
    // Java field: int mMapMode
    pub(crate) mMapMode: i32,
    // Java field: boolean mIsViewportSizeInit
    mIsViewportSizeInit: bool,
    // Java field: boolean mIsUnitPixelMode
    mIsUnitPixelMode: bool,
    // Java field: Matrix transform
    pub(crate) transform: Matrix,
    // Java field: Matrix worldTransform
    pub(crate) worldTransform: Matrix,
}

// Java: public class GdiBoundContext implements Cloneable
impl GdiBoundContext {
    // Java: GdiBoundContext()
    pub fn new() -> Self {
        // Java: super();
        GdiBoundContext {
            srcTransform: Matrix::new(),
            mWindowSize: Size::new(1.0, 1.0),
            mViewportSize: Size::new(1.0, 1.0),
            mWindowOffset: PointF::new(0.0, 0.0),
            mViewportOffset: PointF::new(0.0, 0.0),
            mMapMode: 1,
            mIsViewportSizeInit: false,
            mIsUnitPixelMode: false,
            transform: Matrix::new(),
            worldTransform: Matrix::new(),
        }
    }

    // Java: void initSrcTransform(Matrix transform)
    pub fn initSrcTransform(&mut self, transform: &Matrix) {
        // Java: this.srcTransform.set(transform);
        self.srcTransform.set(transform);
    }

    // Java: void reset(boolean isEmf)
    pub fn reset(&mut self, isEmf: bool) {
        // Java: this.mWindowSize.setSize(1.0F, 1.0F);
        self.mWindowSize.setSize(1.0, 1.0);
        // Java: this.mViewportSize.setSize(1.0F, 1.0F);
        self.mViewportSize.setSize(1.0, 1.0);
        // Java: this.mWindowOffset.set(1.0F, 1.0F);
        self.mWindowOffset.set(1.0, 1.0);
        // Java: this.mWindowOffset.set(1.0F, 1.0F);
        self.mWindowOffset.set(1.0, 1.0);
        // Java: this.mMapMode = 1;
        self.mMapMode = 1;
        // Java: this.mIsViewportSizeInit = false;
        self.mIsViewportSizeInit = false;
        // Java: this.mIsUnitPixelMode = isEmf;
        self.mIsUnitPixelMode = isEmf;
        // Java: this.transform = new Matrix(this.srcTransform);
        self.transform = Matrix::from_matrix(&self.srcTransform);
        // Java: this.worldTransform = new Matrix();
        self.worldTransform = Matrix::new();
    }

    // Java: GdiBoundContext clone()
    pub fn clone_ctx(&self) -> GdiBoundContext {
        // Java: GdiBoundContext clone = new GdiBoundContext();
        let mut c = GdiBoundContext::new();
        // Java: clone.mWindowSize = this.mWindowSize.clone();
        c.mWindowSize = self.mWindowSize.clone();
        // Java: clone.mViewportSize = this.mViewportSize.clone();
        c.mViewportSize = self.mViewportSize.clone();
        // Java: clone.mWindowOffset = this.mWindowOffset.clone();
        c.mWindowOffset = self.mWindowOffset.clone();
        // Java: clone.mViewportOffset = this.mViewportOffset.clone();
        c.mViewportOffset = self.mViewportOffset.clone();
        // Java: clone.mMapMode = this.mMapMode;
        c.mMapMode = self.mMapMode;
        // Java: clone.mIsViewportSizeInit = this.mIsViewportSizeInit;
        c.mIsViewportSizeInit = self.mIsViewportSizeInit;
        // Java: clone.mIsUnitPixelMode = this.mIsUnitPixelMode;
        c.mIsUnitPixelMode = self.mIsUnitPixelMode;
        // Java: clone.transform = new Matrix(this.transform);
        c.transform = Matrix::from_matrix(&self.transform);
        // Java: clone.worldTransform = new Matrix(this.worldTransform);
        c.worldTransform = Matrix::from_matrix(&self.worldTransform);
        // Java: return clone;
        c
    }

    // Java: boolean isUnitToArbitrary()
    fn isUnitToArbitrary(&self) -> bool {
        // Java: return this.mMapMode == 8 || this.mMapMode == 7;
        self.mMapMode == 8 || self.mMapMode == 7
    }

    // Java: void setMapMode(int mode)
    pub fn setMapMode(&mut self, mode: i32) {
        // Java: if (mode == 1)
        if mode == 1 {
            // Java: this.mIsUnitPixelMode = true;
            self.mIsUnitPixelMode = true;
        }
        // Java: this.mMapMode = mode;
        self.mMapMode = mode;
        // Java: this.resetTransformation();
        self.resetTransformation();
    }

    // Java: void resetTransformation()
    pub fn resetTransformation(&mut self) {
        // Java: float scaleX = this.mViewportSize.width / this.mWindowSize.width;
        let mut scaleX = self.mViewportSize.width / self.mWindowSize.width;
        // Java: float scaleY = this.mViewportSize.height / this.mWindowSize.height;
        let mut scaleY = self.mViewportSize.height / self.mWindowSize.height;
        // Java: switch (this.mMapMode)
        match self.mMapMode {
            // Java: case 1:
            1 => {
                // Java: if (this.mIsUnitPixelMode)
                if self.mIsUnitPixelMode {
                    // Java: scaleY = 1.0F;
                    scaleY = 1.0;
                    // Java: scaleX = 1.0F;
                    scaleX = 1.0;
                }
                // Java: break;
            }
            // Java: case 2:
            2 => {
                // Java: scaleX = 0.3385827F;
                scaleX = 0.3385827;
                // Java: scaleY = -0.3385827F;
                scaleY = -0.3385827;
                // Java: break;
            }
            // Java: case 3:
            3 => {
                // Java: scaleX = 0.03385827F;
                scaleX = 0.03385827;
                // Java: scaleY = -0.03385827F;
                scaleY = -0.03385827;
                // Java: break;
            }
            // Java: case 4:
            4 => {
                // Java: scaleX = 0.86F;
                scaleX = 0.86;
                // Java: scaleY = -0.86F;
                scaleY = -0.86;
                // Java: break;
            }
            // Java: case 5:
            5 => {
                // Java: scaleX = 0.086F;
                scaleX = 0.086;
                // Java: scaleY = -0.086F;
                scaleY = -0.086;
                // Java: break;
            }
            // Java: case 6:
            6 => {
                // Java: scaleX = 0.059722222F;
                scaleX = 0.059722222;
                // Java: scaleY = -0.059722222F;
                scaleY = -0.059722222;
                // Java: break;
            }
            // Java: case 7:
            7 => {
                // Java: float scaleXAbs = Math.abs(scaleX);
                let scaleXAbs = scaleX.abs();
                // Java: float scaleYAbs = Math.abs(scaleY);
                let scaleYAbs = scaleY.abs();
                // Java: if (scaleXAbs < scaleYAbs)
                if scaleXAbs < scaleYAbs {
                    // Java: scaleY = (float)(scaleY < 0.0F ? -1 : 1) * scaleXAbs;
                    scaleY = if scaleY < 0.0 { -1.0f32 } else { 1.0f32 } * scaleXAbs;
                }
                // Java: else
                else {
                    // Java: scaleX = (float)(scaleX < 0.0F ? -1 : 1) * scaleYAbs;
                    scaleX = if scaleX < 0.0 { -1.0f32 } else { 1.0f32 } * scaleYAbs;
                }
            }
            // Java: case 8:
            8 => {}
            _ => {}
        }
        // Java: this.resetTransform();
        self.resetTransform();
        // Java: this.translate(this.mViewportOffset.x - this.mWindowOffset.x * scaleX, this.mViewportOffset.y - this.mWindowOffset.y * scaleY);
        let tx = self.mViewportOffset.x - self.mWindowOffset.x * scaleX;
        let ty = self.mViewportOffset.y - self.mWindowOffset.y * scaleY;
        self.translate(tx, ty);
        // Java: this.scale(scaleX, scaleY);
        self.scale(scaleX, scaleY);
    }

    // Java: void setViewportOffset(float dx, float dy)
    pub fn setViewportOffset(&mut self, dx: f32, dy: f32) {
        // Java: this.mViewportOffset.set(dx, dy);
        self.mViewportOffset.set(dx, dy);
        // Java: if (dx != 0.0F || dy != 0.0F)
        if dx != 0.0 || dy != 0.0 {
            // Java: this.resetTransformation();
            self.resetTransformation();
        }
    }

    // Java: void setViewportSize(int width, int height)
    pub fn setViewportSize(&mut self, width: i32, height: i32) {
        // Java: if (Math.abs(width) > 1 && Math.abs(height) > 1)
        if width.abs() > 1 && height.abs() > 1 {
            // Java: this.mIsViewportSizeInit = true;
            self.mIsViewportSizeInit = true;
            // Java: if (!this.mIsUnitPixelMode || this.isUnitToArbitrary())
            if !self.mIsUnitPixelMode || self.isUnitToArbitrary() {
                // Java: this.mViewportSize.setSize((float)width, (float)height);
                self.mViewportSize.setSize(width as f32, height as f32);
                // Java: if (this.isUnitToArbitrary())
                if self.isUnitToArbitrary() {
                    // Java: this.resetTransformation();
                    self.resetTransformation();
                }
            }
        }
    }

    // Java: void scaleViewportSize(int xNum, int xDenom, int yNum, int yDenom)
    pub fn scaleViewportSize(&mut self, xNum: i32, xDenom: i32, yNum: i32, yDenom: i32) {
        // Java: if (this.mViewportSize != null)
        {
            // Java: if (!this.mIsUnitPixelMode || this.isUnitToArbitrary())
            if !self.mIsUnitPixelMode || self.isUnitToArbitrary() {
                // Java: Size var10000 = this.mViewportSize;
                // Java: var10000.width *= (float)xNum / (float)xDenom;
                self.mViewportSize.width *= xNum as f32 / xDenom as f32;
                // Java: var10000 = this.mViewportSize;
                // Java: var10000.height *= (float)yNum / (float)yDenom;
                self.mViewportSize.height *= yNum as f32 / yDenom as f32;
                // Java: if (this.isUnitToArbitrary())
                if self.isUnitToArbitrary() {
                    // Java: this.resetTransformation();
                    self.resetTransformation();
                }
            }
        }
    }

    // Java: void setWindowOffset(float dx, float dy)
    pub fn setWindowOffset(&mut self, dx: f32, dy: f32) {
        // Java: this.mWindowOffset.set(dx, dy);
        self.mWindowOffset.set(dx, dy);
        // Java: this.resetTransformation();
        self.resetTransformation();
    }

    // Java: void setWindowSize(int width, int height)
    pub fn setWindowSize(&mut self, width: i32, height: i32) {
        // Java: if (width != 0 && height != 0)
        if width != 0 && height != 0 {
            // Java: this.mWindowSize.setSize((float)width, (float)height);
            self.mWindowSize.setSize(width as f32, height as f32);
            // Java: if (!this.mIsViewportSizeInit)
            if !self.mIsViewportSizeInit {
                // Java: this.setViewportSize(Math.abs(width), Math.abs(height));
                self.setViewportSize(width.abs(), height.abs());
            }
            // Java: this.resetTransformation();
            self.resetTransformation();
        }
    }

    // Java: void scaleWindowSize(int xNum, int xDenom, int yNum, int yDenom)
    pub fn scaleWindowSize(&mut self, xNum: i32, xDenom: i32, yNum: i32, yDenom: i32) {
        // Java: if (this.mWindowSize != null)
        {
            // Java: if (!this.mIsUnitPixelMode || this.isUnitToArbitrary())
            if !self.mIsUnitPixelMode || self.isUnitToArbitrary() {
                // Java: Size var10000 = this.mWindowSize;
                // Java: var10000.width *= (float)xNum / (float)xDenom;
                self.mWindowSize.width *= xNum as f32 / xDenom as f32;
                // Java: var10000 = this.mWindowSize;
                // Java: var10000.height *= (float)yNum / (float)yDenom;
                self.mWindowSize.height *= yNum as f32 / yDenom as f32;
                // Java: this.resetTransformation();
                self.resetTransformation();
            }
        }
    }

    // Java: float getScaleX()
    pub fn getScaleX(&self) -> f32 {
        // Java: return GdiTool.getScaleX(this.transform);
        gdi_tool_get_scale_x(&self.transform)
    }

    // Java: float getScaleY()
    pub fn getScaleY(&self) -> f32 {
        // Java: return GdiTool.getScaleY(this.transform);
        gdi_tool_get_scale_y(&self.transform)
    }

    // Java: void resetTransform()
    pub fn resetTransform(&mut self) {
        // Java: if (null == this.transform)
        {
            // Java: this.transform = new Matrix(this.srcTransform);
            let src = self.srcTransform.clone();
            self.transform = Matrix::from_matrix(&src);
        }
        // Java: else
        {
            // Java: this.transform.set(this.srcTransform);
            let src = self.srcTransform.clone();
            self.transform.set(&src);
        }
        // Java: if (null != this.worldTransform)
        {
            // Java: this.transform.preConcat(this.worldTransform);
            let wt = self.worldTransform.clone();
            self.transform.preConcat(&wt);
        }
    }

    // Java: void transform(Matrix matrix)
    pub fn do_transform(&mut self, matrix: &Matrix) {
        // Java: this.transform.preConcat(matrix);
        self.transform.preConcat(matrix);
    }

    // Java: void translate(float dx, float dy)
    pub fn translate(&mut self, dx: f32, dy: f32) {
        // Java: if (dx != 0.0F || dy != 0.0F)
        if dx != 0.0 || dy != 0.0 {
            // Java: this.transform.preTranslate(dx, dy);
            self.transform.preTranslate(dx, dy);
        }
    }

    // Java: void scale(float sx, float sy)
    pub fn scale(&mut self, sx: f32, sy: f32) {
        // Java: this.transform.preScale(sx, sy);
        self.transform.preScale(sx, sy);
    }

    // Java: void setWorldTransform(Matrix transform)
    pub fn setWorldTransform(&mut self, transform: &Matrix) {
        // Java: this.worldTransform.set(transform);
        self.worldTransform.set(transform);
    }

    // Java: void resetWorldTransform()
    pub fn resetWorldTransform(&mut self) {
        // Java: if (this.worldTransform == null)
        {
            // Java: this.worldTransform = new Matrix();
            self.worldTransform = Matrix::new();
        }
        // Java: else
        {
            // Java: this.worldTransform.reset();
            self.worldTransform.reset();
        }
    }

    // Java: void worldTransformPostConcat(Matrix other)
    pub fn worldTransformPostConcat(&mut self, other: &Matrix) {
        // Java: this.worldTransform.postConcat(other);
        self.worldTransform.postConcat(other);
    }

    // Java: void worldTransformPreConcat(Matrix other)
    pub fn worldTransformPreConcat(&mut self, other: &Matrix) {
        // Java: this.worldTransform.preConcat(other);
        self.worldTransform.preConcat(other);
    }

    // Java: int getMapMode()
    pub fn getMapMode(&self) -> i32 {
        // Java: return this.mMapMode;
        self.mMapMode
    }

    // Java: Matrix getSrcTransform()
    pub fn getSrcTransform(&self) -> &Matrix {
        // Java: return this.srcTransform;
        &self.srcTransform
    }
}
