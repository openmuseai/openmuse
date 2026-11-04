// Java: import android.graphics.Bitmap
// Java: import android.graphics.Canvas
// Java: import android.graphics.Matrix
// Java: import android.graphics.Paint
// Java: import android.graphics.PointF
// Java: import android.graphics.RectF
// Java: import android.graphics.Bitmap.Config
// Java: import cn.wps.text.layout.data.TextData
// Java: import cn.wps.text.layout.data.TextSelectionParam
// Java: import cn.wps.text.layout.data.TextWarp
// Java: import cn.wps.text.layout.data.WpsPath
// Java: import cn.wps.text.layout.typo.layout.Layout
// Java: import java.util.List
use crate::cn::wps::text::layout::data::TextData::TextData;
use crate::cn::wps::text::layout::data::TextSelectionParam::TextSelectionParam;
use crate::cn::wps::text::layout::data::TextWarp::{TextWarp, WarpType};
use crate::cn::wps::text::layout::data::WpsPath::{WpsPath, Matrix};
use crate::cn::wps::text::layout::typo::layout::Layout::Layout;

#[derive(Clone)]
pub struct PointF { pub x: f32, pub y: f32 }

#[derive(Clone)]
pub struct RectF { pub left: f32, pub top: f32, pub right: f32, pub bottom: f32 }
impl RectF {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self { RectF { left, top, right, bottom } }
    pub fn width(&self) -> f32 { self.right - self.left }
    pub fn height(&self) -> f32 { self.bottom - self.top }
}

pub struct Bitmap;
impl Bitmap {
    pub fn createBitmap(_width: i32, _height: i32, _config: Config) -> Bitmap { Bitmap }
}

#[allow(non_camel_case_types)]
pub enum Config { ARGB_8888 }

pub struct Canvas;
impl Canvas {
    pub fn save(&self) {}
    pub fn restore(&self) {}
    pub fn scale(&self, _sx: f32, _sy: f32) {}
    pub fn translate(&self, _dx: f32, _dy: f32) {}
    pub fn drawBitmapMesh(&self, _bmp: &Bitmap, _mesh_w: i32, _mesh_h: i32, _verts: &[f32], _offset: i32, _colors: Option<&[i32]>, _color_offset: i32, _paint: &Paint) {}
    pub fn drawBitmap(&self, _bmp: &Bitmap, _x: f32, _y: f32, _paint: &Paint) {}
}

pub struct Paint;

// Java field: float TRANSFORM_MAGNIFY_FACTOR
pub(crate) const TRANSFORM_MAGNIFY_FACTOR: f32 = 2.0;
// Java field: int MESH_H_MAX
pub(crate) const MESH_H_MAX: i32 = 120;
// Java field: int MESH_V_MAX
pub(crate) const MESH_V_MAX: i32 = 60;
// Java field: int MESH_RAD
pub(crate) const MESH_RAD: i32 = 200;
// Java field: float PI
pub(crate) const PI: f32 = std::f32::consts::PI;

// Java: public class ArtTransform
pub struct ArtTransform {
    // Java field: Bitmap mBmp
    pub(crate) mBmp: Option<Bitmap>,
    // Java field: int mWidth
    mWidth: i32,
    // Java field: int mHeight
    mHeight: i32,
    // Java field: int mMeshWidth
    mMeshWidth: i32,
    // Java field: int mMeshHeight
    mMeshHeight: i32,
    // Java field: float mLastTextBottom
    mLastTextBottom: f32,
    // Java field: float[] mVerts
    mVerts: Option<Vec<f32>>,
    // Java field: float[] mSquareSins
    mSquareSins: Option<Vec<f32>>,
    // Java field: TextWarp mTextWarp
    mTextWarp: Option<TextWarp>,
    // Java field: int mRotDegrees
    mRotDegrees: i32,
    // Java field: List<Layout> mLayouts
    mLayouts: Option<Vec<Layout>>,
    // Java field: List<PointF> mLayoutOffsets
    mLayoutOffsets: Option<Vec<PointF>>,
    // Java field: List<TextData> mDatas
    mDatas: Option<Vec<TextData>>,
    // Java field: RectF mTypoRect
    mTypoRect: Option<RectF>,
    // Java field: RectF mTextRect
    pub(crate) mTextRect: Option<RectF>,
}

// Java: public class ArtTransform
impl ArtTransform {
    // Java: ArtTransform(int width, int height, float lastTextBottom, TextWarp textWarp, int rotDegrees)
    pub fn new(width: i32, height: i32, lastTextBottom: f32, textWarp: TextWarp, rotDegrees: i32) -> Self {
      // Java: super();
      // Java: this.mWidth = (int)((float)width * 2.0F);
      let mWidth = (width as f32 * 2.0) as i32;
      // Java: this.mHeight = (int)((float)height * 2.0F);
      let mHeight = (height as f32 * 2.0) as i32;
      // Java: this.mMeshWidth = width < 120 ? width : 120;
      let mMeshWidth = if width < 120 { width } else { 120 };
      // Java: this.mMeshHeight = height < 60 ? height : 60;
      let mMeshHeight = if height < 60 { height } else { 60 };
      // Java: this.mLastTextBottom = (float)((int)(lastTextBottom * 2.0F));
      let mLastTextBottom = (lastTextBottom * 2.0) as i32 as f32;
      // Java: this.mVerts = new float[(this.mMeshWidth + 1) * (this.mMeshHeight + 1) * 2];
      let mVerts = vec![0.0f32; ((mMeshWidth + 1) * (mMeshHeight + 1) * 2) as usize];
      // Java: this.mTextWarp = textWarp;
      // Java: this.mRotDegrees = rotDegrees;
      ArtTransform {
          mBmp: None,
          mWidth,
          mHeight,
          mMeshWidth,
          mMeshHeight,
          mLastTextBottom,
          mVerts: Some(mVerts),
          mSquareSins: None,
          mTextWarp: Some(textWarp),
          mRotDegrees: rotDegrees,
          mLayouts: None,
          mLayoutOffsets: None,
          mDatas: None,
          mTypoRect: None,
          mTextRect: None,
      }
    }

    // Java: ArtTransform(int width, int height, int rotDegrees, List<TextData> datas, List<Layout> layouts, List<PointF> offsets)
    pub fn new_for_sel(width: i32, height: i32, rotDegrees: i32, datas: Vec<TextData>, layouts: Vec<Layout>, offsets: Vec<PointF>) -> Self {
      // Java: super();
      // Java: this.mWidth = width;
      // Java: this.mHeight = height;
      // Java: this.mRotDegrees = rotDegrees;
      // Java: this.mDatas = datas;
      // Java: this.mLayouts = layouts;
      // Java: this.mLayoutOffsets = offsets;
      ArtTransform {
          mBmp: None,
          mWidth: width,
          mHeight: height,
          mMeshWidth: 0,
          mMeshHeight: 0,
          mLastTextBottom: 0.0,
          mVerts: None,
          mSquareSins: None,
          mTextWarp: None,
          mRotDegrees: rotDegrees,
          mLayouts: Some(layouts),
          mLayoutOffsets: Some(offsets),
          mDatas: Some(datas),
          mTypoRect: None,
          mTextRect: None,
      }
    }

    // Java: Bitmap createTextBitmap()
    pub fn createTextBitmap(&mut self) -> &Bitmap {
      // Java: this.mBmp = Bitmap.createBitmap(this.mWidth, this.mHeight, Config.ARGB_8888);
      self.mBmp = Some(Bitmap::createBitmap(self.mWidth, self.mHeight, Config::ARGB_8888));
      // Java: return this.mBmp;
      self.mBmp.as_ref().unwrap()
    }

    // Java: Bitmap createSelCurBitmap()
    pub fn createSelCurBitmap(&mut self) -> &Bitmap {
      // Java: int width;
      let width: i32;
      // Java: int height;
      let height: i32;
      // Java: switch (this.mRotDegrees)
      match self.mRotDegrees
      {
         // Java: case 90:
         // Java: case 270:
         90 | 270 => {
            // Java: width = this.mHeight;
            width = self.mHeight;
            // Java: height = this.mWidth;
            height = self.mWidth;
            // Java: break;
         }
         // Java: default:
         _ => {
            // Java: width = this.mWidth;
            width = self.mWidth;
            // Java: height = this.mHeight;
            height = self.mHeight;
         }
      }
      // Java: this.mBmp = Bitmap.createBitmap(width, height, Config.ARGB_8888);
      self.mBmp = Some(Bitmap::createBitmap(width, height, Config::ARGB_8888));
      // Java: return this.mBmp;
      self.mBmp.as_ref().unwrap()
    }

    // Java: void drawResult(Canvas canvas, Paint paint)
    pub fn drawResult(&self, canvas: &Canvas, paint: &Paint) {
      // Java: float scale = 0.5F;
      let _scale = 0.5f32;
      // Java: canvas.save();
      canvas.save();
      // Java: canvas.scale(scale, scale);
      canvas.scale(0.5, 0.5);
      // Java: switch (this.mTextWarp.mWarpType)
      match self.mTextWarp.as_ref().map(|tw| tw.mWarpType)
      {
         // Java: case ARCH_UP:
         // Java: case ARCH_DOWN:
         Some(WarpType::ARCH_UP) | Some(WarpType::ARCH_DOWN) => {
            // Java: canvas.drawBitmapMesh(this.mBmp, this.mMeshWidth, this.mMeshHeight, this.mVerts, 0, (int[])null, 0, paint);
            if let (Some(bmp), Some(verts)) = (&self.mBmp, &self.mVerts) {
                canvas.drawBitmapMesh(bmp, self.mMeshWidth, self.mMeshHeight, verts, 0, None, 0, paint);
            }
            // Java: break;
         }
         // Java: default:
         _ => {
            // Java: canvas.drawBitmap(this.mBmp, 0.0F, 0.0F, paint);
            if let Some(bmp) = &self.mBmp {
                canvas.drawBitmap(bmp, 0.0, 0.0, paint);
            }
         }
      }
      // Java: canvas.restore();
      canvas.restore();
    }

    // Java: void transformText()
    pub fn transformText(&mut self) {
      // Java: switch (this.mTextWarp.mWarpType)
      match self.mTextWarp.as_ref().map(|tw| tw.mWarpType)
      {
         // Java: case ARCH_UP:
         Some(WarpType::ARCH_UP) => {
            // Java: this.archUp();
            self.archUp();
            // Java: break;
         }
         // Java: case ARCH_DOWN:
         Some(WarpType::ARCH_DOWN) => {
            // Java: this.archDown();
            self.archDown();
         }
         _ => {}
      }
    }

    // Java: void transformSelCur()
    pub fn transformSelCur(&mut self) {
      // Java: switch (this.mTextWarp.mWarpType)
      match self.mTextWarp.as_ref().map(|tw| tw.mWarpType)
      {
         // Java: case ARCH_UP:
         Some(WarpType::ARCH_UP) => {
            // Java: this.archUp();
            self.archUp();
            // Java: this.mVerts = this.convertVertsFromRotate(this.mVerts);
            if let Some(verts) = self.mVerts.take() {
                self.mVerts = self.convertVertsFromRotate(verts);
            }
            // Java: break;
         }
         // Java: case ARCH_DOWN:
         Some(WarpType::ARCH_DOWN) => {
            // Java: this.archDown();
            self.archDown();
            // Java: this.mVerts = this.convertVertsFromRotate(this.mVerts);
            if let Some(verts) = self.mVerts.take() {
                self.mVerts = self.convertVertsFromRotate(verts);
            }
         }
         _ => {}
      }
    }

    // Java: void transformPoint(PointF point)
    pub fn transformPoint(&self, point: &mut PointF) {
      // Java: point.x *= 2.0F;
      point.x *= 2.0;
      // Java: point.y *= 2.0F;
      point.y *= 2.0;
      // Java: if (point.x < 0.0F)
      if point.x < 0.0
      {
         // Java: point.x = 0.0F;
         point.x = 0.0;
      }
      // Java: if (point.x > (float)this.mWidth)
      if point.x > self.mWidth as f32
      {
         // Java: point.x = (float)this.mWidth;
         point.x = self.mWidth as f32;
      }
      // Java: if (point.y < 0.0F)
      if point.y < 0.0
      {
         // Java: point.y = 0.0F;
         point.y = 0.0;
      }
      // Java: if (point.y > (float)this.mHeight)
      if point.y > self.mHeight as f32
      {
         // Java: point.y = (float)this.mHeight;
         point.y = self.mHeight as f32;
      }
      // Java: int col = (int)((float)this.mMeshWidth * point.x / (float)this.mWidth);
      let col = (self.mMeshWidth as f32 * point.x / self.mWidth as f32) as i32;
      // Java: int row = (int)((float)this.mMeshHeight * point.y / (float)this.mHeight);
      let row = (self.mMeshHeight as f32 * point.y / self.mHeight as f32) as i32;
      // Java: int inxx = ((this.mMeshWidth + 1) * row + col) * 2;
      let inxx = ((self.mMeshWidth + 1) * row + col) * 2;
      // Java: int inxy = inxx + 1;
      let inxy = inxx + 1;
      // Java: float tx = this.mVerts[inxx];
      let tx = self.mVerts.as_ref().map_or(0.0, |v| v[inxx as usize]);
      // Java: float ty = this.mVerts[inxy];
      let ty = self.mVerts.as_ref().map_or(0.0, |v| v[inxy as usize]);
      // Java: point.x = tx / 2.0F;
      point.x = tx / 2.0;
      // Java: point.y = ty / 2.0F;
      point.y = ty / 2.0;
    }

    // Java: void reverse(PointF point)
    pub fn reverse(&mut self, point: &mut PointF) {
      // Java: point.x *= 2.0F;
      point.x *= 2.0;
      // Java: point.y *= 2.0F;
      point.y *= 2.0;
      // Java: switch (this.mTextWarp.mWarpType)
      match self.mTextWarp.as_ref().map(|tw| tw.mWarpType)
      {
         // Java: case ARCH_UP:
         // Java: case ARCH_DOWN:
         Some(WarpType::ARCH_UP) | Some(WarpType::ARCH_DOWN) => {
            // Java: this.convertCoorToRotate(point);
            self.convertCoorToRotate(point);
            // Java: this.meshReverse(point);
            self.meshReverse(point);
            // Java: this.convertCoorFromRotate(point);
            self.convertCoorFromRotate(point);
         }
         // Java: default:
         _ => {}
      }
      // Java: point.x /= 2.0F;
      point.x /= 2.0;
      // Java: point.y /= 2.0F;
      point.y /= 2.0;
    }

    // Java: float getTransformMagnifyFactor()
    pub fn getTransformMagnifyFactor(&self) -> f32 {
      // Java: return 2.0F;
      2.0
    }

    // Java: PointF getScale()
    pub fn getScale(&mut self) -> PointF {
      // Java: if (null == this.mTextRect)
      if self.mTextRect.is_none()
      {
         // Java: this.calcTextRect();
         self.calcTextRect();
      }
      // Java: if (null == this.mTypoRect)
      if self.mTypoRect.is_none()
      {
         // Java: this.calcTypoRect();
         self.calcTypoRect();
      }
      // Java: float xscale;
      let xscale: f32;
      // Java: float yscale;
      let yscale: f32;
      let text_rect = self.mTextRect.as_ref().unwrap();
      let typo_rect = self.mTypoRect.as_ref().unwrap();
      // Java: if (0 == this.mRotDegrees)
      if 0 == self.mRotDegrees
      {
         // Java: xscale = this.mTypoRect.width() / this.mTextRect.width();
         xscale = typo_rect.width() / text_rect.width();
         // Java: yscale = this.mTypoRect.height() / this.mTextRect.height();
         yscale = typo_rect.height() / text_rect.height();
      }
      // Java: else
      else
      {
         // Java: xscale = this.mTypoRect.height() / this.mTextRect.width();
         xscale = typo_rect.height() / text_rect.width();
         // Java: yscale = this.mTypoRect.width() / this.mTextRect.height();
         yscale = typo_rect.width() / text_rect.height();
      }
      // Java: return new PointF(xscale, yscale);
      PointF { x: xscale, y: yscale }
    }

    // Java: float getTextHeight()
    pub fn getTextHeight(&mut self) -> f32 {
      // Java: if (null == this.mTextRect)
      if self.mTextRect.is_none()
      {
         // Java: this.calcTextRect();
         self.calcTextRect();
      }
      // Java: return this.mTextRect.height();
      self.mTextRect.as_ref().map_or(0.0, |r| r.height())
    }

    // Java: float getTextWidth()
    pub fn getTextWidth(&mut self) -> f32 {
      // Java: if (null == this.mTextRect)
      if self.mTextRect.is_none()
      {
         // Java: this.calcTextRect();
         self.calcTextRect();
      }
      // Java: return this.mTextRect.width();
      self.mTextRect.as_ref().map_or(0.0, |r| r.width())
    }

    // Java: void squareText(Canvas canvas)
    pub fn squareText(&mut self, canvas: &Canvas) {
      // Java: if (null == this.mTextRect)
      if self.mTextRect.is_none()
      {
         // Java: this.calcTextRect();
         self.calcTextRect();
      }
      // Java: if (null == this.mTypoRect)
      if self.mTypoRect.is_none()
      {
         // Java: this.calcTypoRect();
         self.calcTypoRect();
      }
      // Java: float xscale;
      let xscale: f32;
      // Java: float yscale;
      let yscale: f32;
      // Java: float xoffset;
      let mut xoffset: f32;
      // Java: float yoffset;
      let yoffset: f32;
      let text_rect = self.mTextRect.as_ref().unwrap();
      let typo_rect = self.mTypoRect.as_ref().unwrap();
      // Java: if (0 == this.mRotDegrees)
      if 0 == self.mRotDegrees
      {
         // Java: xscale = this.mTypoRect.width() / this.mTextRect.width();
         xscale = typo_rect.width() / text_rect.width();
         // Java: yscale = this.mTypoRect.height() / this.mTextRect.height();
         yscale = typo_rect.height() / text_rect.height();
         // Java: xoffset = -this.mTextRect.left * xscale;
         xoffset = -text_rect.left * xscale;
         // Java: yoffset = -this.mTextRect.top * yscale;
         yoffset = -text_rect.top * yscale;
      }
      // Java: else
      else
      {
         // Java: xscale = this.mTypoRect.height() / this.mTextRect.width();
         xscale = typo_rect.height() / text_rect.width();
         // Java: yscale = this.mTypoRect.width() / this.mTextRect.height();
         yscale = typo_rect.width() / text_rect.height();
         // Java: xoffset = -this.mTextRect.left * xscale;
         xoffset = -text_rect.left * xscale;
         // Java: yoffset = -this.mTextRect.top * yscale;
         yoffset = -text_rect.top * yscale;
         // Java: if (90 == this.mRotDegrees || 270 == this.mRotDegrees)
         if 90 == self.mRotDegrees || 270 == self.mRotDegrees
         {
            // Java: xoffset += (this.mTypoRect.width() - this.mTypoRect.height()) / 2.0F;
            xoffset += (typo_rect.width() - typo_rect.height()) / 2.0;
         }
      }
      // Java: canvas.translate(xoffset, yoffset);
      canvas.translate(xoffset, yoffset);
      // Java: canvas.scale(xscale, yscale);
      canvas.scale(xscale, yscale);
    }

    // Java: void squarePointF(PointF pointF)
    pub fn squarePointF(&mut self, pointF: &mut PointF) {
      // Java: if (null == this.mTextRect)
      if self.mTextRect.is_none()
      {
         // Java: this.calcTextRect();
         self.calcTextRect();
      }
      // Java: if (null == this.mTypoRect)
      if self.mTypoRect.is_none()
      {
         // Java: this.calcTypoRect();
         self.calcTypoRect();
      }
      // Java: float xscale;
      let xscale: f32;
      // Java: float yscale;
      let yscale: f32;
      // Java: float xTxtOffset;
      let xTxtOffset: f32;
      // Java: float yTxtOffset;
      let yTxtOffset: f32;
      let text_rect = self.mTextRect.as_ref().unwrap();
      let typo_rect = self.mTypoRect.as_ref().unwrap();
      // Java: if (0 == this.mRotDegrees)
      if 0 == self.mRotDegrees
      {
         // Java: xscale = this.mTypoRect.width() / this.mTextRect.width();
         xscale = typo_rect.width() / text_rect.width();
         // Java: yscale = this.mTypoRect.height() / this.mTextRect.height();
         yscale = typo_rect.height() / text_rect.height();
         // Java: xTxtOffset = -this.mTextRect.left * xscale;
         xTxtOffset = -text_rect.left * xscale;
         // Java: yTxtOffset = -this.mTextRect.top * yscale;
         yTxtOffset = -text_rect.top * yscale;
      }
      // Java: else
      else
      {
         // Java: xscale = this.mTypoRect.height() / this.mTextRect.width();
         xscale = typo_rect.height() / text_rect.width();
         // Java: yscale = this.mTypoRect.width() / this.mTextRect.height();
         yscale = typo_rect.width() / text_rect.height();
         // Java: xTxtOffset = -this.mTextRect.left * xscale;
         xTxtOffset = -text_rect.left * xscale;
         // Java: yTxtOffset = -this.mTextRect.top * yscale;
         yTxtOffset = -text_rect.top * yscale;
      }
      // Java: pointF.x *= xscale;
      pointF.x *= xscale;
      // Java: pointF.y *= yscale;
      pointF.y *= yscale;
      // Java: pointF.x += xTxtOffset;
      pointF.x += xTxtOffset;
      // Java: pointF.y += yTxtOffset;
      pointF.y += yTxtOffset;
    }

    // Java: void squareCurSel(TextSelectionParam selParam)
    pub fn squareCurSel(&mut self, selParam: &mut TextSelectionParam) {
      // Java: if (null == this.mTextRect)
      if self.mTextRect.is_none()
      {
         // Java: this.calcTextRect();
         self.calcTextRect();
      }
      // Java: if (null == this.mTypoRect)
      if self.mTypoRect.is_none()
      {
         // Java: this.calcTypoRect();
         self.calcTypoRect();
      }
      // Java: List<WpsPath> paths = selParam.getPaths();
      // Java: List<PointF> offsets = selParam.getMoffsets();
      // Java: float xscale;
      let xscale: f32;
      // Java: float yscale;
      let yscale: f32;
      // Java: float xTxtOffset;
      let xTxtOffset: f32;
      // Java: float yTxtOffset;
      let yTxtOffset: f32;
      let text_rect = self.mTextRect.as_ref().unwrap();
      let typo_rect = self.mTypoRect.as_ref().unwrap();
      // Java: if (0 == this.mRotDegrees)
      if 0 == self.mRotDegrees
      {
         // Java: xscale = this.mTypoRect.width() / this.mTextRect.width();
         xscale = typo_rect.width() / text_rect.width();
         // Java: yscale = this.mTypoRect.height() / this.mTextRect.height();
         yscale = typo_rect.height() / text_rect.height();
         // Java: xTxtOffset = -this.mTextRect.left * xscale;
         xTxtOffset = -text_rect.left * xscale;
         // Java: yTxtOffset = -this.mTextRect.top * yscale;
         yTxtOffset = -text_rect.top * yscale;
      }
      // Java: else
      else
      {
         // Java: xscale = this.mTypoRect.height() / this.mTextRect.width();
         xscale = typo_rect.height() / text_rect.width();
         // Java: yscale = this.mTypoRect.width() / this.mTextRect.height();
         yscale = typo_rect.width() / text_rect.height();
         // Java: xTxtOffset = -this.mTextRect.left * xscale;
         xTxtOffset = -text_rect.left * xscale;
         // Java: yTxtOffset = -this.mTextRect.top * yscale;
         yTxtOffset = -text_rect.top * yscale;
      }
      // Java: Matrix matrix = new Matrix();
      let matrix = Matrix;
      // Java: for(int i = 0; i < paths.size(); ++i)
      for i in 0..selParam.paths.len()
      {
         // Java: matrix.reset();
         // Java: WpsPath path = (WpsPath)paths.get(i);
         // Java: PointF offset = (PointF)offsets.get(i);
         let offset_x = selParam.moffsets[i].x;
         let offset_y = selParam.moffsets[i].y;
         // Java: float xoffset = xTxtOffset + offset.x * (xscale - 1.0F);
         let xoffset = xTxtOffset + offset_x * (xscale - 1.0);
         // Java: float yoffset = yTxtOffset + offset.y * (yscale - 1.0F);
         let yoffset = yTxtOffset + offset_y * (yscale - 1.0);
         // Java: matrix.preScale(xscale, yscale);
         // Java: path.transform(matrix);
         selParam.paths[i].transform(&matrix);
         // Java: path.offset(xoffset, yoffset);
         selParam.paths[i].offset(xoffset, yoffset);
      }
      // Java: PointF startDrag = selParam.getSelectDragStart();
      // Java: PointF endDrag = selParam.getSelectDragEnd();
      // Java: if (null != startDrag && null != endDrag)
      if selParam.selectDragStart.is_some() && selParam.selectDragEnd.is_some()
      {
         // Java: startDrag.x *= xscale;
         // Java: startDrag.y *= yscale;
         // Java: startDrag.x += xTxtOffset;
         // Java: startDrag.y += yTxtOffset;
         if let Some(ref mut start) = selParam.selectDragStart {
             start.x *= xscale;
             start.y *= yscale;
             start.x += xTxtOffset;
             start.y += yTxtOffset;
         }
         // Java: endDrag.x *= xscale;
         // Java: endDrag.y *= yscale;
         // Java: endDrag.x += xTxtOffset;
         // Java: endDrag.y += yTxtOffset;
         if let Some(ref mut end) = selParam.selectDragEnd {
             end.x *= xscale;
             end.y *= yscale;
             end.x += xTxtOffset;
             end.y += yTxtOffset;
         }
      }
      // Java: float slHeight = selParam.getStartLineHeight() * yscale;
      let slHeight = selParam.getStartLineHeight() * yscale;
      // Java: float elHeight = selParam.getEndLineHeight() * yscale;
      let elHeight = selParam.getEndLineHeight() * yscale;
      // Java: selParam.setStartLineHeight(slHeight);
      selParam.setStartLineHeight(slHeight);
      // Java: selParam.setEndLineHeight(elHeight);
      selParam.setEndLineHeight(elHeight);
      // Java: selParam.setRotateDegrees(0);
      selParam.setRotateDegrees(0);
    }

    // Java: void mapPoint(Matrix matrix, PointF point)
    pub fn mapPoint(&self, _matrix: &Matrix, _point: &mut PointF) {
      // Java: float[] start = new float[]{point.x, point.y};
      let mut start = [_point.x, _point.y];
      // Java: matrix.mapPoints(start);
      let _ = &mut start;
      // Java: point.x = start[0];
      _point.x = start[0];
      // Java: point.y = start[1];
      _point.y = start[1];
    }

    // Java: void squareHitCoor(float[] xy)
    pub fn squareHitCoor(&mut self, xy: &mut [f32]) {
      // Java: if (null == this.mTextRect)
      if self.mTextRect.is_none()
      {
         // Java: this.calcTextRect();
         self.calcTextRect();
      }
      // Java: if (null == this.mTypoRect)
      if self.mTypoRect.is_none()
      {
         // Java: this.calcTypoRect();
         self.calcTypoRect();
      }
      // Java: float xscale;
      let xscale: f32;
      // Java: float yscale;
      let yscale: f32;
      let text_rect = self.mTextRect.as_ref().unwrap();
      let typo_rect = self.mTypoRect.as_ref().unwrap();
      // Java: if (0 == this.mRotDegrees)
      if 0 == self.mRotDegrees
      {
         // Java: xscale = this.mTypoRect.width() / this.mTextRect.width();
         xscale = typo_rect.width() / text_rect.width();
         // Java: yscale = this.mTypoRect.height() / this.mTextRect.height();
         yscale = typo_rect.height() / text_rect.height();
      }
      // Java: else
      else
      {
         // Java: xscale = this.mTypoRect.height() / this.mTextRect.width();
         xscale = typo_rect.height() / text_rect.width();
         // Java: yscale = this.mTypoRect.width() / this.mTextRect.height();
         yscale = typo_rect.width() / text_rect.height();
      }
      // Java: xy[0] /= xscale;
      xy[0] /= xscale;
      // Java: xy[1] /= yscale;
      xy[1] /= yscale;
      // Java: xy[0] += this.mTextRect.left;
      xy[0] += text_rect.left;
      // Java: xy[1] += this.mTextRect.top;
      xy[1] += text_rect.top;
    }

    // Java: float[] convertVertsFromRotate(float[] rverts)
    pub fn convertVertsFromRotate(&self, rverts: Vec<f32>) -> Option<Vec<f32>> {
      // Java: if (this.mRotDegrees != 90 && this.mRotDegrees != 270)
      if self.mRotDegrees != 90 && self.mRotDegrees != 270
      {
         // Java: return rverts;
         return Some(rverts);
      }
      // Java: else
      {
         // Java: int rrows = this.mMeshHeight + 1;
         let rrows = self.mMeshHeight + 1;
         // Java: int rcols = this.mMeshWidth + 1;
         let rcols = self.mMeshWidth + 1;
         // Java: float[] nverts = new float[rrows * rcols * 2];
         let mut nverts = vec![0.0f32; (rrows * rcols * 2) as usize];
         // Java: PointF point = new PointF();
         let mut point = PointF { x: 0.0, y: 0.0 };
         // Java: int nvert = 0;
         let mut nvert: i32 = 0;
         // Java: switch (this.mRotDegrees)
         match self.mRotDegrees
         {
            // Java: case 90:
            90 => {
               // Java: for(int i = 0; i < rcols; ++i)
               for i in 0..rcols
               {
                  // Java: for(int j = rrows - 1; j >= 0; --j)
                  for j in (0..rrows).rev()
                  {
                     // Java: int rvert = j * rcols + i;
                     let rvert = j * rcols + i;
                     // Java: point.x = rverts[rvert * 2];
                     point.x = rverts[(rvert * 2) as usize];
                     // Java: point.y = rverts[rvert * 2 + 1];
                     point.y = rverts[(rvert * 2 + 1) as usize];
                     // Java: this.convertCoorFromRotate(point);
                     self.convertCoorFromRotate(&mut point);
                     // Java: nverts[nvert * 2] = point.x;
                     nverts[(nvert * 2) as usize] = point.x;
                     // Java: nverts[nvert * 2 + 1] = point.y;
                     nverts[(nvert * 2 + 1) as usize] = point.y;
                     // Java: ++nvert;
                     nvert += 1;
                  }
               }
               // Java: break;
            }
            // Java: case 270:
            270 => {
               // Java: for(int i = rcols - 1; i >= 0; --i)
               for i in (0..rcols).rev()
               {
                  // Java: for(int j = 0; j < rrows; ++j)
                  for j in 0..rrows
                  {
                     // Java: int rvert = j * rcols + i;
                     let rvert = j * rcols + i;
                     // Java: point.x = rverts[rvert * 2];
                     point.x = rverts[(rvert * 2) as usize];
                     // Java: point.y = rverts[rvert * 2 + 1];
                     point.y = rverts[(rvert * 2 + 1) as usize];
                     // Java: this.convertCoorFromRotate(point);
                     self.convertCoorFromRotate(&mut point);
                     // Java: nverts[nvert * 2] = point.x;
                     nverts[(nvert * 2) as usize] = point.x;
                     // Java: nverts[nvert * 2 + 1] = point.y;
                     nverts[(nvert * 2 + 1) as usize] = point.y;
                     // Java: ++nvert;
                     nvert += 1;
                  }
               }
               // Java: break;
            }
            // Java: default:
            _ => {
               // Java: nverts = rverts;
               return Some(rverts);
            }
         }
         // Java: return nverts;
         Some(nverts)
      }
    }

    // Java: void convertCoorToRotate(PointF point)
    pub fn convertCoorToRotate(&self, point: &mut PointF) {
      // Java: if (null != point)
      {
         // Java: switch (this.mRotDegrees)
         match self.mRotDegrees
         {
            // Java: case 90:
            90 => {
               // Java: float temp = (float)this.mHeight - point.x;
               let temp = self.mHeight as f32 - point.x;
               // Java: point.x = point.y;
               point.x = point.y;
               // Java: point.y = temp;
               point.y = temp;
               // Java: break;
            }
            // Java: case 270:
            270 => {
               // Java: temp = point.x;
               let temp = point.x;
               // Java: point.x = (float)this.mWidth - point.y;
               point.x = self.mWidth as f32 - point.y;
               // Java: point.y = temp;
               point.y = temp;
            }
            _ => {}
         }
      }
    }

    // Java: void convertCoorFromRotate(PointF point)
    pub fn convertCoorFromRotate(&self, point: &mut PointF) {
      // Java: if (null != point)
      {
         // Java: switch (this.mRotDegrees)
         match self.mRotDegrees
         {
            // Java: case 90:
            90 => {
               // Java: float temp = point.x;
               let temp = point.x;
               // Java: point.x = (float)this.mHeight - point.y;
               point.x = self.mHeight as f32 - point.y;
               // Java: point.y = temp;
               point.y = temp;
               // Java: break;
            }
            // Java: case 270:
            270 => {
               // Java: temp = (float)this.mWidth - point.x;
               let temp = self.mWidth as f32 - point.x;
               // Java: point.x = point.y;
               point.x = point.y;
               // Java: point.y = temp;
               point.y = temp;
            }
            _ => {}
         }
      }
    }

    // Java: void archUp()
    pub fn archUp(&mut self) {
      // Java: if (null == this.mSquareSins)
      if self.mSquareSins.is_none()
      {
         // Java: this.mSquareSins = this.getSquareSins((float)Math.PI, 0.0F);
         self.mSquareSins = self.getSquareSins(std::f32::consts::PI, 0.0);
      }
      // Java: PointF[] lineVerts = new PointF[this.mMeshWidth + 1];
      let mut lineVerts: Vec<PointF> = Vec::with_capacity((self.mMeshWidth + 1) as usize);
      // Java: for(int i = 0; i < lineVerts.length; ++i)
      for _i in 0..=(self.mMeshWidth)
      {
         // Java: lineVerts[i] = new PointF();
         lineVerts.push(PointF { x: 0.0, y: 0.0 });
      }
      // Java: float ao = (float)(this.mWidth / 2);
      let ao = (self.mWidth / 2) as f32;
      // Java: float bo = (float)(this.mHeight / 2);
      let bo = (self.mHeight / 2) as f32;
      // Java: for(int i = 0; i <= this.mMeshHeight; ++i)
      for i in 0..=self.mMeshHeight
      {
         // Java: float dec = (float)(i * this.mHeight / this.mMeshHeight);
         let mut dec = (i * self.mHeight / self.mMeshHeight) as f32;
         // Java: float undertext = dec > this.mLastTextBottom ? dec - this.mLastTextBottom : 0.0F;
         let undertext = if dec > self.mLastTextBottom { dec - self.mLastTextBottom } else { 0.0 };
         // Java: dec -= undertext;
         dec -= undertext;
         // Java: dec *= (float)this.mHeight / (4.0F * this.mLastTextBottom);
         dec *= self.mHeight as f32 / (4.0 * self.mLastTextBottom);
         // Java: float b = bo - dec;
         let b = bo - dec;
         // Java: float a = ao * b / bo;
         let a = ao * b / bo;
         // Java: this.getEllipseVerts(lineVerts, this.mSquareSins, a, b);
         if let Some(ref ssins) = self.mSquareSins.clone() {
             Self::getEllipseVerts_impl(&mut lineVerts, ssins, a, b);
         }
         // Java: for(int j = 0; j <= this.mMeshWidth; ++j)
         for j in 0..=self.mMeshWidth
         {
            // Java: int inxx = (i * (this.mMeshWidth + 1) + j) * 2;
            let inxx = (i * (self.mMeshWidth + 1) + j) * 2;
            // Java: int inxy = inxx + 1;
            let inxy = inxx + 1;
            // Java: float x = lineVerts[j].x + (float)(this.mWidth / 2);
            let x = lineVerts[j as usize].x + (self.mWidth / 2) as f32;
            // Java: float y = (float)(this.mHeight / 2) - lineVerts[j].y;
            let y = (self.mHeight / 2) as f32 - lineVerts[j as usize].y;
            // Java: this.mVerts[inxx] = x;
            if let Some(ref mut verts) = self.mVerts {
                verts[inxx as usize] = x;
            // Java: this.mVerts[inxy] = y + undertext;
                verts[inxy as usize] = y + undertext;
            }
         }
      }
    }

    // Java: void archDown()
    pub fn archDown(&mut self) {
      // Java: if (null == this.mSquareSins)
      if self.mSquareSins.is_none()
      {
         // Java: this.mSquareSins = this.getSquareSins((float)Math.PI, ((float)Math.PI * 2F));
         self.mSquareSins = self.getSquareSins(std::f32::consts::PI, std::f32::consts::PI * 2.0);
      }
      // Java: PointF[] lineVerts = new PointF[this.mMeshWidth + 1];
      let mut lineVerts: Vec<PointF> = Vec::with_capacity((self.mMeshWidth + 1) as usize);
      // Java: for(int i = 0; i < lineVerts.length; ++i)
      for _i in 0..=(self.mMeshWidth)
      {
         // Java: lineVerts[i] = new PointF();
         lineVerts.push(PointF { x: 0.0, y: 0.0 });
      }
      // Java: float ao = (float)(this.mWidth / 2);
      let ao = (self.mWidth / 2) as f32;
      // Java: float bo = (float)(this.mHeight / 2);
      let bo = (self.mHeight / 2) as f32;
      // Java: for(int i = 0; i <= this.mMeshHeight; ++i)
      for i in 0..=self.mMeshHeight
      {
         // Java: float dec = (float)(i * this.mHeight / this.mMeshHeight);
         let mut dec = (i * self.mHeight / self.mMeshHeight) as f32;
         // Java: float undertext = dec > this.mLastTextBottom ? dec - this.mLastTextBottom : 0.0F;
         let undertext = if dec > self.mLastTextBottom { dec - self.mLastTextBottom } else { 0.0 };
         // Java: dec -= undertext;
         dec -= undertext;
         // Java: dec = (this.mLastTextBottom - dec) * (float)this.mHeight / (4.0F * this.mLastTextBottom);
         dec = (self.mLastTextBottom - dec) * self.mHeight as f32 / (4.0 * self.mLastTextBottom);
         // Java: float b = bo - dec;
         let b = bo - dec;
         // Java: float a = ao * b / bo;
         let a = ao * b / bo;
         // Java: this.getEllipseVerts(lineVerts, this.mSquareSins, a, b);
         if let Some(ref ssins) = self.mSquareSins.clone() {
             Self::getEllipseVerts_impl(&mut lineVerts, ssins, a, b);
         }
         // Java: for(int j = 0; j <= this.mMeshWidth; ++j)
         for j in 0..=self.mMeshWidth
         {
            // Java: int inxx = (i * (this.mMeshWidth + 1) + j) * 2;
            let inxx = (i * (self.mMeshWidth + 1) + j) * 2;
            // Java: int inxy = inxx + 1;
            let inxy = inxx + 1;
            // Java: float x = lineVerts[j].x + (float)(this.mWidth / 2);
            let x = lineVerts[j as usize].x + (self.mWidth / 2) as f32;
            // Java: float y = (float)(this.mHeight / 2) - lineVerts[j].y;
            let y = (self.mHeight / 2) as f32 - lineVerts[j as usize].y;
            // Java: this.mVerts[inxx] = x;
            if let Some(ref mut verts) = self.mVerts {
                verts[inxx as usize] = x;
            // Java: this.mVerts[inxy] = y + undertext;
                verts[inxy as usize] = y + undertext;
            }
         }
      }
    }

    // Java: void meshReverse(PointF point)
    pub fn meshReverse(&self, point: &mut PointF) {
      // Java: int rr = 0;
      let mut rr: i32 = 0;
      // Java: int rc = 0;
      let mut rc: i32 = 0;
      // Java: int ix = ((this.mMeshWidth + 1) * rr + rc) * 2;
      let mut ix = ((self.mMeshWidth + 1) * rr + rc) * 2;
      // Java: int iy = ix + 1;
      let mut iy = ix + 1;
      // Java: float cdx = point.x - this.mVerts[ix];
      let verts = self.mVerts.as_ref().unwrap();
      let mut cdx = point.x - verts[ix as usize];
      // Java: float cdy = point.y - this.mVerts[iy];
      let mut cdy = point.y - verts[iy as usize];
      // Java: float sd = cdx * cdx + cdy * cdy;
      let mut sd = cdx * cdx + cdy * cdy;
      // Java: for(int i = 0; i <= this.mMeshHeight; ++i)
      for i in 0..=self.mMeshHeight
      {
         // Java: for(int j = 0; j <= this.mMeshWidth; ++j)
         for j in 0..=self.mMeshWidth
         {
            // Java: ix = ((this.mMeshWidth + 1) * i + j) * 2;
            ix = ((self.mMeshWidth + 1) * i + j) * 2;
            // Java: iy = ix + 1;
            iy = ix + 1;
            // Java: cdx = point.x - this.mVerts[ix];
            cdx = point.x - verts[ix as usize];
            // Java: cdy = point.y - this.mVerts[iy];
            cdy = point.y - verts[iy as usize];
            // Java: float csd = cdx * cdx + cdy * cdy;
            let csd = cdx * cdx + cdy * cdy;
            // Java: if (sd > csd)
            if sd > csd
            {
               // Java: sd = csd;
               sd = csd;
               // Java: rr = i;
               rr = i;
               // Java: rc = j;
               rc = j;
            }
         }
      }
      // Java: point.x = (float)(this.mWidth * rc / this.mMeshWidth);
      point.x = (self.mWidth * rc / self.mMeshWidth) as f32;
      // Java: point.y = (float)(this.mHeight * rr / this.mMeshHeight);
      point.y = (self.mHeight * rr / self.mMeshHeight) as f32;
    }

    // Java: float mapEllipseToX(float x, float[] squareSins, float a, float b)
    pub fn mapEllipseToX(&self, x: f32, squareSins: &[f32], a: f32, b: f32) -> f32 {
      // Java: if (null == squareSins)
      if squareSins.is_empty()
      {
         // Java: return 0.0F;
         return 0.0;
      }
      // Java: else
      {
         // Java: float arcl = 0.0F;
         let mut arcl = 0.0f32;
         // Java: int slen = squareSins.length - 2;
         let slen = squareSins.len() - 2;
         // Java: float[] arcs = new float[slen];
         let mut arcs = vec![0.0f32; slen];
         // Java: float stepRad = squareSins[slen + 1];
         let stepRad = squareSins[slen + 1];
         // Java: float absStepRad = Math.abs(stepRad);
         let absStepRad = stepRad.abs();
         // Java: for(int i = 0; i < slen; ++i)
         for i in 0..slen
         {
            // Java: double ssin = (double)squareSins[i];
            let ssin = squareSins[i] as f64;
            // Java: double scos = (double)1.0F - ssin;
            let scos = 1.0f64 - ssin;
            // Java: arcs[i] = (float)Math.sqrt((double)(a * a) * ssin + (double)(b * b) * scos) * absStepRad;
            arcs[i] = ((a * a) as f64 * ssin + (b * b) as f64 * scos).sqrt() as f32 * absStepRad;
            // Java: arcl += arcs[i];
            arcl += arcs[i];
         }
         // Java: float rad = squareSins[slen];
         let mut rad = squareSins[slen];
         // Java: float xrad = (float)Math.acos((double)(x / a));
         let xrad = ((x / a) as f64).acos() as f32;
         // Java: float xarc = 0.0F;
         let mut xarc = 0.0f32;
         // Java: for(int i = 0; i < slen; ++i)
         for i in 0..slen
         {
            // Java: if (!(Math.abs(rad - xrad) >= absStepRad))
            if !((rad - xrad).abs() >= absStepRad)
            {
               // Java: xarc += arcs[i] * Math.abs(rad - xrad) / absStepRad;
               xarc += arcs[i] * (rad - xrad).abs() / absStepRad;
               // Java: break;
               break;
            }
            // Java: xarc += arcs[i];
            xarc += arcs[i];
            // Java: rad += stepRad;
            rad += stepRad;
         }
         // Java: x = 2.0F * a * xarc / arcl - a;
         let x = 2.0 * a * xarc / arcl - a;
         // Java: return x;
         x
      }
    }

    // Java: void getEllipseVerts(PointF[] verts, float[] squareSins, float a, float b)
    pub fn getEllipseVerts(&self, verts: &mut Vec<PointF>, squareSins: &[f32], a: f32, b: f32) {
        Self::getEllipseVerts_impl(verts, squareSins, a, b);
    }

    fn getEllipseVerts_impl(verts: &mut Vec<PointF>, squareSins: &[f32], a: f32, b: f32) {
      // Java: if (null != verts && 0 != verts.length && null != squareSins && squareSins.length >= 4)
      if !verts.is_empty() && squareSins.len() >= 4
      {
         // Java: int vertnum = verts.length;
         let vertnum = verts.len();
         // Java: float arcl = 0.0F;
         let mut arcl = 0.0f32;
         // Java: int slen = squareSins.length - 2;
         let slen = squareSins.len() - 2;
         // Java: float[] arcs = new float[slen];
         let mut arcs = vec![0.0f32; slen];
         // Java: float rad = squareSins[slen];
         let mut rad = squareSins[slen];
         // Java: float stepRad = squareSins[slen + 1];
         let stepRad = squareSins[slen + 1];
         // Java: float absStepRad = Math.abs(stepRad);
         let absStepRad = stepRad.abs();
         // Java: for(int i = 0; i < slen; ++i)
         for i in 0..slen
         {
            // Java: double ssin = (double)squareSins[i];
            let ssin = squareSins[i] as f64;
            // Java: double scos = (double)1.0F - ssin;
            let scos = 1.0f64 - ssin;
            // Java: arcs[i] = (float)Math.sqrt((double)(a * a) * ssin + (double)(b * b) * scos) * absStepRad;
            arcs[i] = ((a * a) as f64 * ssin + (b * b) as f64 * scos).sqrt() as f32 * absStepRad;
            // Java: arcl += arcs[i];
            arcl += arcs[i];
         }
         // Java: float stepArc = arcl / (float)(vertnum - 1);
         let stepArc = arcl / (vertnum - 1) as f32;
         // Java: float arcd = 0.0F;
         let mut arcd = 0.0f32;
         // Java: int arcinx = 0;
         let mut arcinx: usize = 0;
         // Java: verts[0].x = (float)((double)a * Math.cos((double)rad));
         verts[0].x = (a as f64 * (rad as f64).cos()) as f32;
         // Java: verts[0].y = (float)((double)b * Math.sin((double)rad));
         verts[0].y = (b as f64 * (rad as f64).sin()) as f32;
         // Java: for(int i = 1; i < vertnum; ++i)
         for i in 1..vertnum
         {
            // Java: while(arcinx < slen)
            while arcinx < slen
            {
               // Java: if (!(arcd < stepArc))
               if !(arcd < stepArc)
               {
                  // Java: float vertrad = rad - stepRad * (arcd - stepArc) / arcs[arcinx];
                  let vertrad = rad - stepRad * (arcd - stepArc) / arcs[arcinx];
                  // Java: arcd -= stepArc;
                  arcd -= stepArc;
                  // Java: verts[i].x = (float)((double)a * Math.cos((double)vertrad));
                  verts[i].x = (a as f64 * (vertrad as f64).cos()) as f32;
                  // Java: verts[i].y = (float)((double)b * Math.sin((double)vertrad));
                  verts[i].y = (b as f64 * (vertrad as f64).sin()) as f32;
                  // Java: break;
                  break;
               }
               // Java: arcd += arcs[arcinx++];
               arcd += arcs[arcinx];
               arcinx += 1;
               // Java: rad += stepRad;
               rad += stepRad;
            }
         }
      }
    }

    // Java: float[] getSquareSins(float startRad, float endRad)
    pub fn getSquareSins(&self, startRad: f32, endRad: f32) -> Option<Vec<f32>> {
      // Java: if (startRad == endRad)
      if startRad == endRad
      {
         // Java: return null;
         return None;
      }
      // Java: else
      {
         // Java: int mesh = 200;
         let mesh: usize = 200;
         // Java: float[] ssins = new float[mesh + 2];
         let mut ssins = vec![0.0f32; mesh + 2];
         // Java: float step = (endRad - startRad) / (float)mesh;
         let step = (endRad - startRad) / mesh as f32;
         // Java: float rad = startRad;
         let mut rad = startRad;
         // Java: for(int i = 0; i < mesh; ++i)
         for i in 0..mesh
         {
            // Java: ssins[i] = (float)Math.sin((double)rad);
            ssins[i] = (rad as f64).sin() as f32;
            // Java: ssins[i] *= ssins[i];
            ssins[i] *= ssins[i];
            // Java: rad += step;
            rad += step;
         }
         // Java: ssins[mesh] = startRad;
         ssins[mesh] = startRad;
         // Java: ssins[mesh + 1] = step;
         ssins[mesh + 1] = step;
         // Java: return ssins;
         Some(ssins)
      }
    }

    // Java: void calcTextRect()
    pub fn calcTextRect(&mut self) {
      // Java: if (null != this.mLayouts && !this.mLayouts.isEmpty() && null != this.mLayoutOffsets && !this.mLayoutOffsets.isEmpty() && null != this.mDatas && !this.mDatas.isEmpty())
      if self.mLayouts.as_ref().map_or(false, |l| !l.is_empty())
          && self.mLayoutOffsets.as_ref().map_or(false, |o| !o.is_empty())
          && self.mDatas.as_ref().map_or(false, |d| !d.is_empty())
      {
         // Java: float left = Float.NaN;
         let mut left = f32::NAN;
         // Java: float top = Float.NaN;
         let mut top = f32::NAN;
         // Java: float right = Float.NaN;
         let mut right = f32::NAN;
         // Java: float bottom = Float.NaN;
         let mut bottom = f32::NAN;
         // Java: for(int i = 0; i < this.mDatas.size(); ++i)
         let datas_len = self.mDatas.as_ref().unwrap().len();
         let layouts_len = self.mLayouts.as_ref().unwrap().len();
         for i in 0..datas_len
         {
            // Java: TextData data = (TextData)this.mDatas.get(i);
            let data = &self.mDatas.as_ref().unwrap()[i];
            // Java: if (data.mPara.mbulletsInfo.getBulletType() != -1)
            if data.mPara.mbulletsInfo.getBulletType() != -1
            {
               // Java: float val = data.mPara.mbulletsInfo.getX();
               let val = data.mPara.mbulletsInfo.getX();
               // Java: if (Float.isNaN(left) || left > val)
               if left.is_nan() || left > val
               {
                  // Java: left = val;
                  left = val;
               }
            }
            // Java: Layout layout = (Layout)this.mLayouts.get(i);
            let layout = &self.mLayouts.as_ref().unwrap()[i];
            // Java: PointF offset = (PointF)this.mLayoutOffsets.get(i);
            let offset = &self.mLayoutOffsets.as_ref().unwrap()[i];
            // Java: for(int j = 0; j < layout.getLineCount(); ++j)
            for j in 0..layout.getLineCount()
            {
               // Java: float val = layout.getLineLeft(j) + offset.x;
               let val = layout.getLineLeft(j) + offset.x;
               // Java: if (Float.isNaN(left) || left > val)
               if left.is_nan() || left > val
               {
                  // Java: left = val;
                  left = val;
               }
               // Java: val = layout.getLineRight(j) + offset.x;
               let val = layout.getLineRight(j) + offset.x;
               // Java: if (Float.isNaN(right) || right < val)
               if right.is_nan() || right < val
               {
                  // Java: right = val;
                  right = val;
               }
            }
            // Java: if (0 == i)
            if 0 == i
            {
               // Java: top = layout.getLineTop(0) + offset.y;
               top = layout.getLineTop(0) + offset.y;
            }
            // Java: if (this.mLayouts.size() - 1 == i)
            if layouts_len - 1 == i
            {
               // Java: int lastline = layout.getLineCount() - 1;
               let lastline = layout.getLineCount() - 1;
               // Java: bottom = layout.getLineBaseline(lastline) - layout.getLineAscent(lastline) / 4.0F + offset.y;
               bottom = layout.getLineBaseline(lastline) - layout.getLineAscent(lastline) / 4.0 + offset.y;
            }
         }
         // Java: this.mTextRect = new RectF(left, top, right, bottom);
         self.mTextRect = Some(RectF::new(left, top, right, bottom));
      }
    }

    // Java: void calcTypoRect()
    pub fn calcTypoRect(&mut self) {
      // Java: switch (this.mRotDegrees)
      match self.mRotDegrees
      {
         // Java: case 90:
         // Java: case 270:
         90 | 270 => {
            // Java: this.mTypoRect = new RectF(0.0F, 0.0F, (float)this.mHeight, (float)this.mWidth);
            self.mTypoRect = Some(RectF::new(0.0, 0.0, self.mHeight as f32, self.mWidth as f32));
            // Java: break;
         }
         // Java: default:
         _ => {
            // Java: this.mTypoRect = new RectF(0.0F, 0.0F, (float)this.mWidth, (float)this.mHeight);
            self.mTypoRect = Some(RectF::new(0.0, 0.0, self.mWidth as f32, self.mHeight as f32));
         }
      }
    }

    // Java: RectF getTextRect()
    pub fn getTextRect(&mut self) -> &RectF {
      // Java: if (this.mTextRect == null)
      if self.mTextRect.is_none()
      {
         // Java: this.calcTextRect();
         self.calcTextRect();
      }
      // Java: return this.mTextRect;
      self.mTextRect.as_ref().unwrap()
    }

}
