// Java: import android.graphics.Bitmap
// Java: import android.graphics.BitmapShader
// Java: import android.graphics.Canvas
// Java: import android.graphics.LinearGradient
// Java: import android.graphics.Matrix
// Java: import android.graphics.Paint
// Java: import android.graphics.Path
// Java: import android.graphics.PointF
// Java: import android.graphics.RadialGradient
// Java: import android.graphics.RectF
// Java: import android.graphics.Shader
// Java: import android.graphics.Bitmap.Config
// Java: import android.graphics.Path.Direction
// Java: import android.graphics.Shader.TileMode
// Java: import android.text.TextPaint
// Java: import cn.wps.text.layout.data.GradientAttr
use crate::cn::wps::text::layout::data::GradientAttr::{GradientAttr, GradType};
use crate::cn::wps::text::layout::typo::ArtTransform::ArtTransform;

#[derive(Clone)]
pub struct PointF { pub x: f32, pub y: f32 }
impl PointF {
    pub fn new(x: f32, y: f32) -> Self { PointF { x, y } }
}

#[derive(Clone)]
pub struct RectF { pub left: f32, pub top: f32, pub right: f32, pub bottom: f32 }
impl RectF {
    pub fn new(l: f32, t: f32, r: f32, b: f32) -> Self { RectF { left: l, top: t, right: r, bottom: b } }
    pub fn default() -> Self { RectF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 } }
    pub fn width(&self) -> f32 { self.right - self.left }
    pub fn height(&self) -> f32 { self.bottom - self.top }
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.left && x <= self.right && y >= self.top && y <= self.bottom
    }
}

#[allow(dead_code)]
pub struct Path { points: Vec<(f32, f32)> }
impl Path {
    pub fn new() -> Self { Path { points: Vec::new() } }
    pub fn addRect(&mut self, _rect: &RectF, _dir: Direction) {}
    pub fn moveTo(&mut self, x: f32, y: f32) { self.points.push((x, y)); }
    pub fn lineTo(&mut self, x: f32, y: f32) { self.points.push((x, y)); }
    pub fn close(&mut self) {}
}

#[derive(Clone, Copy)]
pub enum Direction { CW }

pub struct LinearGradient { pub x0: f32, pub y0: f32, pub x1: f32, pub y1: f32 }
impl LinearGradient {
    pub fn new(_x0: f32, _y0: f32, _x1: f32, _y1: f32, _colors: &[i32], _positions: &[f32], _tile: TileMode) -> Self {
        LinearGradient { x0: _x0, y0: _y0, x1: _x1, y1: _y1 }
    }
    pub fn setLocalMatrix(&mut self, _m: &Matrix) {}
}

pub struct RadialGradient;
impl RadialGradient {
    pub fn new(_cx: f32, _cy: f32, _radius: f32, _colors: &[i32], _positions: &[f32], _tile: TileMode) -> Self { RadialGradient }
    pub fn setLocalMatrix(&mut self, _m: &Matrix) {}
}

pub struct BitmapShader;
impl BitmapShader {
    pub fn new(_bmp: &Bitmap, _tx: TileMode, _ty: TileMode) -> Self { BitmapShader }
    pub fn setLocalMatrix(&mut self, _m: &Matrix) {}
}

pub struct Matrix;
impl Matrix {
    pub fn new() -> Self { Matrix }
    pub fn preTranslate(&mut self, _dx: f32, _dy: f32) {}
    pub fn postTranslate(&mut self, _dx: f32, _dy: f32) {}
}

pub struct Bitmap;
impl Bitmap {
    pub fn createBitmap(_w: i32, _h: i32, _config: Config) -> Self { Bitmap }
}

pub struct Canvas;
impl Canvas {
    pub fn new() -> Self { Canvas }
    pub fn setBitmap(&mut self, _bmp: &Bitmap) {}
    pub fn save(&self) {}
    pub fn restore(&self) {}
    pub fn clipPath(&mut self, _path: &Path) {}
    pub fn drawPath(&self, _path: &Path, _paint: &Paint) {}
    pub fn drawRect(&self, _l: f32, _t: f32, _r: f32, _b: f32, _paint: &Paint) {}
}

pub struct Paint { color: i32, anti_alias: bool }
impl Paint {
    pub fn new() -> Self { Paint { color: 0, anti_alias: false } }
    pub fn setAntiAlias(&mut self, aa: bool) { self.anti_alias = aa; }
    pub fn setColor(&mut self, c: i32) { self.color = c; }
    pub fn getColor(&self) -> i32 { self.color }
    pub fn setShader(&mut self, _s: Option<&LinearGradient>) {}
}

pub struct TextPaint;
impl TextPaint {
    pub fn setShader_linear(&mut self, _s: &LinearGradient) {}
    pub fn setShader_radial(&mut self, _s: &RadialGradient) {}
    pub fn setShader_bitmap(&mut self, _s: &BitmapShader) {}
}

#[derive(Clone, Copy)]
pub enum TileMode { CLAMP, MIRROR }

#[derive(Clone, Copy)]
pub enum Config { ARGB_8888 }

// Java: public class GradPaintSetter
pub struct GradPaintSetter {
    // Java field: float mLnHei
    mLnHei: f32,
    // Java field: float mLnWid
    mLnWid: f32,
    // Java field: float mLnLeft
    mLnLeft: f32,
    // Java field: Path mPath
    mPath: Path,
    // Java field: ArtTransform mArtTransform
    mArtTransform: Option<ArtTransform>,
    // Java field: float mMinLeft
    mMinLeft: f32,
    // Java field: float mMinTop
    mMinTop: f32,
    // Java field: PointF mOffset
    mOffset: Option<PointF>,
}

// Java: public class GradPaintSetter
impl GradPaintSetter {
    // Java: GradPaintSetter(float lnWid, float lnHei, float lnLeft)
    pub fn new(lnWid: f32, lnHei: f32, lnLeft: f32) -> Self {
        // Java: super();
        // Java: this.mLnWid = lnWid;
        // Java: this.mLnHei = lnHei;
        // Java: this.mLnLeft = lnLeft;
        // Java: this.mPath = new Path();
        let mut mPath = Path::new();
        // Java: this.mPath.addRect(new RectF(this.mLnLeft, 0.0F, this.mLnLeft + lnWid, lnHei), Direction.CW);
        let rect = RectF::new(lnLeft, 0.0, lnLeft + lnWid, lnHei);
        mPath.addRect(&rect, Direction::CW);
        GradPaintSetter { mLnWid: lnWid, mLnHei: lnHei, mLnLeft: lnLeft, mPath, mArtTransform: None, mMinLeft: 0.0, mMinTop: 0.0, mOffset: None }
    }

    // Java: GradPaintSetter(float lnWid, float lnHei, float lnLeft, ArtTransform artTransform, float minLeft, float minTop, PointF offset)
    pub fn new_with_art(lnWid: f32, lnHei: f32, lnLeft: f32, mut artTransform: Option<ArtTransform>, minLeft: f32, minTop: f32, offset: PointF) -> Self {
        // Java: super();
        // Java: this.mLnWid = lnWid;
        // Java: this.mLnHei = lnHei;
        // Java: this.mLnLeft = lnLeft;
        // Java: this.mPath = new Path();
        let mut mPath = Path::new();
        // Java: this.mArtTransform = artTransform;
        // Java: this.mMinLeft = minLeft;
        // Java: this.mMinTop = minTop;
        // Java: this.mOffset = offset;
        // Java: if (this.mArtTransform == null)
        if artTransform.is_none()
        {
            // Java: this.mPath.addRect(new RectF(this.mLnLeft, 0.0F, this.mLnLeft + lnWid, lnHei), Direction.CW);
            let rect = RectF::new(lnLeft, 0.0, lnLeft + lnWid, lnHei);
            mPath.addRect(&rect, Direction::CW);
        }
        // Java: else
        else
        {
            // Java: this.mPath.addRect(new RectF(0.0F, 0.0F, this.mArtTransform.getTextWidth(), this.mArtTransform.getTextHeight()), Direction.CW);
            let at = artTransform.as_mut().unwrap();
            let rect = RectF::new(0.0, 0.0, at.getTextWidth(), at.getTextHeight());
            mPath.addRect(&rect, Direction::CW);
        }
        GradPaintSetter { mLnWid: lnWid, mLnHei: lnHei, mLnLeft: lnLeft, mPath, mArtTransform: artTransform, mMinLeft: minLeft, mMinTop: minTop, mOffset: Some(offset) }
    }

    // Java: GradPaintSetter(float lnWid, float lnHei, float lnLeft, ArtTransform artTransform, float minLeft, PointF offset)
    pub fn new_with_art_no_top(lnWid: f32, lnHei: f32, lnLeft: f32, mut artTransform: Option<ArtTransform>, minLeft: f32, offset: PointF) -> Self {
        // Java: super();
        // Java: this.mLnWid = lnWid;
        // Java: this.mLnHei = lnHei;
        // Java: this.mLnLeft = lnLeft;
        // Java: this.mPath = new Path();
        let mut mPath = Path::new();
        // Java: this.mArtTransform = artTransform;
        // Java: this.mMinLeft = minLeft;
        // Java: this.mOffset = offset;
        // Java: if (this.mArtTransform == null)
        if artTransform.is_none()
        {
            // Java: this.mPath.addRect(new RectF(this.mLnLeft, 0.0F, this.mLnLeft + lnWid, lnHei), Direction.CW);
            let rect = RectF::new(lnLeft, 0.0, lnLeft + lnWid, lnHei);
            mPath.addRect(&rect, Direction::CW);
        }
        // Java: else
        else
        {
            // Java: this.mPath.addRect(new RectF(this.mMinLeft, 0.0F, this.mMinLeft + this.mArtTransform.getTextWidth(), this.mArtTransform.getTextHeight()), Direction.CW);
            let at = artTransform.as_mut().unwrap();
            let rect = RectF::new(minLeft, 0.0, minLeft + at.getTextWidth(), at.getTextHeight());
            mPath.addRect(&rect, Direction::CW);
        }
        GradPaintSetter { mLnWid: lnWid, mLnHei: lnHei, mLnLeft: lnLeft, mPath, mArtTransform: artTransform, mMinLeft: minLeft, mMinTop: 0.0, mOffset: Some(offset) }
    }

    // Java: void applyGradFill(TextPaint paint, GradientAttr gradAttr, float lnTop)
    pub fn applyGradFill(&mut self, paint: &mut TextPaint, gradAttr: &GradientAttr, lnTop: f32) {
        // Java: int[] colors = gradAttr.colors();
        let colors = gradAttr.colors();
        // Java: float[] positions = gradAttr.positions();
        let positions = gradAttr.positions();
        // Java: switch (gradAttr.gradType())
        match gradAttr.gradType()
        {
            // Java: case LINEAR:
            GradType::LINEAR => {
                // Java: RectF rect = this.getGradFillRect();
                let rect = self.getGradFillRect();
                // Java: if (this.mLnHei > 0.0F && this.mLnWid > 0.0F)
                if self.mLnHei > 0.0 && self.mLnWid > 0.0
                {
                    // Java: LinearGradient linearGrad = this.getLinearGrad(gradAttr.angle(), rect, colors, positions, lnTop);
                    let linearGrad = self.getLinearGrad(gradAttr.angle(), &rect, colors, positions, lnTop);
                    // Java: paint.setShader(linearGrad);
                    paint.setShader_linear(&linearGrad);
                }
                // Java: break;
            }
            // Java: case CIRCLE:
            GradType::CIRCLE => {
                // Java: RadialGradient radialGrad = this.getRadialGrad(colors, positions, gradAttr);
                let radialGrad = self.getRadialGrad(colors, positions, gradAttr);
                // Java: paint.setShader(radialGrad);
                paint.setShader_radial(&radialGrad);
                // Java: break;
            }
            // Java: case RECT:
            GradType::RECT => {
                // Java: Bitmap rectBmp = this.getGradBmp(gradAttr.fillRect(), gradAttr.titleRect(), colors, positions);
                let rectBmp = self.getGradBmp(gradAttr.fillRect(), gradAttr.titleRect(), colors, positions);
                // Java: BitmapShader rectShader = new BitmapShader(rectBmp, TileMode.CLAMP, TileMode.CLAMP);
                let mut rectShader = BitmapShader::new(&rectBmp, TileMode::CLAMP, TileMode::CLAMP);
                // Java: if (this.mArtTransform != null)
                if self.mArtTransform.is_some()
                {
                    // Java: Matrix matrix = new Matrix();
                    let mut matrix = Matrix::new();
                    // Java: RectF textRect = this.mArtTransform.getTextRect();
                    let tr = self.mArtTransform.as_ref().unwrap().mTextRect.as_ref().unwrap();
                    let textRect_left = tr.left;
                    let textRect_top = tr.top;
                    // Java: matrix.postTranslate(textRect.left - this.mMinLeft, textRect.top - this.mMinTop);
                    matrix.postTranslate(textRect_left - self.mMinLeft, textRect_top - self.mMinTop);
                    // Java: matrix.postTranslate(-this.mOffset.x, -this.mOffset.y);
                    let off = self.mOffset.as_ref().unwrap();
                    matrix.postTranslate(-off.x, -off.y);
                    // Java: rectShader.setLocalMatrix(matrix);
                    rectShader.setLocalMatrix(&matrix);
                }
                // Java: paint.setShader(rectShader);
                paint.setShader_bitmap(&rectShader);
                // Java: break;
            }
            // Java: case SHAPE:
            GradType::SHAPE => {
                // Java: Bitmap pathBmp = this.getGradBmp(gradAttr.fillRect(), gradAttr.titleRect(), colors, positions);
                let pathBmp = self.getGradBmp(gradAttr.fillRect(), gradAttr.titleRect(), colors, positions);
                // Java: BitmapShader pathShader = new BitmapShader(pathBmp, TileMode.CLAMP, TileMode.CLAMP);
                let mut pathShader = BitmapShader::new(&pathBmp, TileMode::CLAMP, TileMode::CLAMP);
                // Java: if (this.mArtTransform != null)
                if self.mArtTransform.is_some()
                {
                    // Java: Matrix matrix2 = new Matrix();
                    let mut matrix2 = Matrix::new();
                    // Java: RectF textRect2 = this.mArtTransform.getTextRect();
                    let tr2 = self.mArtTransform.as_ref().unwrap().mTextRect.as_ref().unwrap();
                    let textRect2_left = tr2.left;
                    let textRect2_top = tr2.top;
                    // Java: matrix2.postTranslate(textRect2.left - this.mMinLeft, textRect2.top - this.mMinTop);
                    matrix2.postTranslate(textRect2_left - self.mMinLeft, textRect2_top - self.mMinTop);
                    // Java: matrix2.postTranslate(-this.mOffset.x, -this.mOffset.y);
                    let off = self.mOffset.as_ref().unwrap();
                    matrix2.postTranslate(-off.x, -off.y);
                    // Java: pathShader.setLocalMatrix(matrix2);
                    pathShader.setLocalMatrix(&matrix2);
                }
                // Java: paint.setShader(pathShader);
                paint.setShader_bitmap(&pathShader);
            }
        }
    }

    // Java: RectF getGradFillRect()
    fn getGradFillRect(&mut self) -> RectF {
        // Java: new RectF();
        // Java: RectF rect;
        let rect;
        // Java: if (this.mArtTransform == null)
        if self.mArtTransform.is_none()
        {
            // Java: rect = new RectF(this.mLnLeft, 0.0F, this.mLnLeft + this.mLnWid, this.mLnHei);
            rect = RectF::new(self.mLnLeft, 0.0, self.mLnLeft + self.mLnWid, self.mLnHei);
        }
        // Java: else
        else
        {
            // Java: float textWid = this.mArtTransform.getTextWidth();
            let textWid = self.mArtTransform.as_mut().unwrap().getTextWidth();
            // Java: float textHei = this.mArtTransform.getTextHeight();
            let textHei = self.mArtTransform.as_mut().unwrap().getTextHeight();
            // Java: float left = -this.mOffset.x + this.mMinLeft;
            let left = -self.mOffset.as_ref().unwrap().x + self.mMinLeft;
            // Java: float top = -this.mOffset.y;
            let top = -self.mOffset.as_ref().unwrap().y;
            // Java: rect = new RectF(left, top, left + textWid, top + textHei);
            rect = RectF::new(left, top, left + textWid, top + textHei);
        }
        // Java: return rect;
        rect
    }

    // Java: LinearGradient getLinearGrad(int angle, RectF rect, int[] colors, float[] positions, float lnTop)
    fn getLinearGrad(&mut self, angle: i32, rect: &RectF, colors: &[i32], positions: &[f32], lnTop: f32) -> LinearGradient {
        // Java: PointF start = new PointF(rect.left, rect.top);
        let start = PointF::new(rect.left, rect.top);
        // Java: PointF end = null;
        let mut end: Option<PointF> = None;
        // Java: int sx = 0;
        let mut sx: i32 = 0;
        // Java: int sy = 0;
        let mut sy: i32 = 0;
        // Java: if (angle == 0)
        if angle == 0
        {
            // Java: sx = 1;
            sx = 1;
        }
        // Java: else if (angle < 90)
        else if angle < 90
        {
            // Java: sx = 1;
            sx = 1;
            // Java: sy = 1;
            sy = 1;
        }
        // Java: else if (angle == 90)
        else if angle == 90
        {
            // Java: sy = 1;
            sy = 1;
        }
        // Java: else if (angle < 180)
        else if angle < 180
        {
            // Java: sx = -1;
            sx = -1;
            // Java: sy = 1;
            sy = 1;
        }
        // Java: else if (angle == 180)
        else if angle == 180
        {
            // Java: sx = -1;
            sx = -1;
        }
        // Java: else if (angle < 270)
        else if angle < 270
        {
            // Java: sx = -1;
            sx = -1;
            // Java: sy = -1;
            sy = -1;
        }
        // Java: else if (angle == 270)
        else if angle == 270
        {
            // Java: sy = -1;
            sy = -1;
        }
        // Java: else
        else
        {
            // Java: sx = 1;
            sx = 1;
            // Java: sy = -1;
            sy = -1;
        }
        // Java: end = new PointF(rect.left + rect.width() * (float)sx, rect.top + rect.height() * (float)sy);
        end = Some(PointF::new(rect.left + rect.width() * sx as f32, rect.top + rect.height() * sy as f32));
        // Java: if (sx != 0 && sy != 0)
        if sx != 0 && sy != 0
        {
            // Java: end = this.getFootPoint((float)Math.tan(Math.toRadians((double)angle)), start, end);
            let tan_val = (angle as f64).to_radians().tan() as f32;
            end = self.getFootPoint_with_k(tan_val, &start, &end.unwrap());
        }
        // Java: float dx = 0.0F;
        let mut dx: f32 = 0.0;
        // Java: float dy = 0.0F;
        let mut dy: f32 = 0.0;
        // Java: if (sx < 0)
        if sx < 0
        {
            // Java: dx = rect.width();
            dx = rect.width();
        }
        // Java: if (sy < 0)
        if sy < 0
        {
            // Java: dy = rect.height();
            dy = rect.height();
        }
        // Java: LinearGradient linearGrad = this.lineGradFill(start, end, dx, dy, colors, positions, lnTop);
        let linearGrad = self.lineGradFill(&start, &end.unwrap_or(PointF::new(0.0, 0.0)), dx, dy, colors, positions, lnTop);
        // Java: return linearGrad;
        linearGrad
    }

    // Java: LinearGradient lineGradFill(PointF start, PointF end, float dx, float dy, int[] colors, float[] positions, float lnTop)
    fn lineGradFill(&mut self, start: &PointF, end: &PointF, dx: f32, dy: f32, colors: &[i32], positions: &[f32], lnTop: f32) -> LinearGradient {
        // Java: float startY = start.y + lnTop;
        let startY = start.y + lnTop;
        // Java: float endY = end.y + lnTop;
        let endY = end.y + lnTop;
        // Java: LinearGradient linearGrad = new LinearGradient(start.x, startY, end.x, endY, colors, positions, TileMode.MIRROR);
        let mut linearGrad = LinearGradient::new(start.x, startY, end.x, endY, colors, positions, TileMode::MIRROR);
        // Java: if (dx != 0.0F || dy != 0.0F)
        if dx != 0.0 || dy != 0.0
        {
            // Java: Matrix matrix = new Matrix();
            let mut matrix = Matrix::new();
            // Java: matrix.preTranslate(dx, dy);
            matrix.preTranslate(dx, dy);
            // Java: linearGrad.setLocalMatrix(matrix);
            linearGrad.setLocalMatrix(&matrix);
        }
        // Java: return linearGrad;
        linearGrad
    }

    // Java: RadialGradient getRadialGrad(int[] colors, float[] positions, GradientAttr gradAttr)
    fn getRadialGrad(&mut self, colors: &[i32], positions: &[f32], gradAttr: &GradientAttr) -> RadialGradient {
        // Java: float[] dis = new float[4];
        let mut dis: [f32; 4] = [0.0; 4];
        // Java: float left = 0.0F;
        let mut left: f32 = 0.0;
        // Java: float top = 0.0F;
        let mut top: f32 = 0.0;
        // Java: if (gradAttr.fillRect() != null)
        if let Some(fill_rect) = gradAttr.fillRect()
        {
            // Java: left = gradAttr.fillRect().left;
            left = fill_rect.left;
            // Java: top = gradAttr.fillRect().top;
            top = fill_rect.top;
        }
        // Java: float x;
        let x;
        // Java: float y;
        let y;
        // Java: if (this.mArtTransform == null)
        if self.mArtTransform.is_none()
        {
            // Java: x = this.mLnLeft + this.mLnWid * left;
            x = self.mLnLeft + self.mLnWid * left;
            // Java: y = this.mLnHei * top;
            y = self.mLnHei * top;
            // Java: PointF center = new PointF(x, y);
            let center = PointF::new(x, y);
            // Java: dis[0] = this.twoPointsDis(center, new PointF(this.mLnLeft, 0.0F));
            dis[0] = self.twoPointsDis(&center, &PointF::new(self.mLnLeft, 0.0));
            // Java: dis[1] = this.twoPointsDis(center, new PointF(this.mLnLeft + this.mLnWid, 0.0F));
            dis[1] = self.twoPointsDis(&center, &PointF::new(self.mLnLeft + self.mLnWid, 0.0));
            // Java: dis[2] = this.twoPointsDis(center, new PointF(this.mLnLeft + this.mLnWid, this.mLnHei));
            dis[2] = self.twoPointsDis(&center, &PointF::new(self.mLnLeft + self.mLnWid, self.mLnHei));
            // Java: dis[3] = this.twoPointsDis(center, new PointF(this.mLnLeft, this.mLnHei));
            dis[3] = self.twoPointsDis(&center, &PointF::new(self.mLnLeft, self.mLnHei));
        }
        // Java: else
        else
        {
            // Java: float textWid = this.mArtTransform.getTextWidth();
            let textWid = self.mArtTransform.as_mut().unwrap().getTextWidth();
            // Java: float textHei = this.mArtTransform.getTextHeight();
            let textHei = self.mArtTransform.as_mut().unwrap().getTextHeight();
            // Java: x = this.mMinLeft + textWid * left;
            x = self.mMinLeft + textWid * left;
            // Java: y = textHei * top;
            y = textHei * top;
            // Java: PointF center = new PointF(x, y);
            let center = PointF::new(x, y);
            // Java: dis[0] = this.twoPointsDis(center, new PointF(this.mMinLeft, 0.0F));
            dis[0] = self.twoPointsDis(&center, &PointF::new(self.mMinLeft, 0.0));
            // Java: dis[1] = this.twoPointsDis(center, new PointF(this.mMinLeft + textWid, 0.0F));
            dis[1] = self.twoPointsDis(&center, &PointF::new(self.mMinLeft + textWid, 0.0));
            // Java: dis[2] = this.twoPointsDis(center, new PointF(this.mMinLeft + textWid, textHei));
            dis[2] = self.twoPointsDis(&center, &PointF::new(self.mMinLeft + textWid, textHei));
            // Java: dis[3] = this.twoPointsDis(center, new PointF(this.mMinLeft, textHei));
            dis[3] = self.twoPointsDis(&center, &PointF::new(self.mMinLeft, textHei));
        }
        // Java: float radius = 0.0F;
        let mut radius: f32 = 0.0;
        // Java: for(int i = 0; i < dis.length; ++i)
        for i in 0..dis.len()
        {
            // Java: if (dis[i] > radius)
            if dis[i] > radius
            {
                // Java: radius = dis[i];
                radius = dis[i];
            }
        }
        // Java: RadialGradient radialGrad = new RadialGradient(x, y, radius, colors, positions, TileMode.CLAMP);
        let mut radialGrad = RadialGradient::new(x, y, radius, colors, positions, TileMode::CLAMP);
        // Java: if (this.mArtTransform != null)
        if self.mArtTransform.is_some()
        {
            // Java: Matrix matrix = new Matrix();
            let mut matrix = Matrix::new();
            // Java: matrix.preTranslate(-this.mOffset.x, -this.mOffset.y);
            let off = self.mOffset.as_ref().unwrap();
            matrix.preTranslate(-off.x, -off.y);
            // Java: radialGrad.setLocalMatrix(matrix);
            radialGrad.setLocalMatrix(&matrix);
        }
        // Java: return radialGrad;
        radialGrad
    }

    // Java: float twoPointsDis(PointF point1, PointF point2)
    fn twoPointsDis(&self, point1: &PointF, point2: &PointF) -> f32 {
        // Java: float distance = (float)Math.sqrt(Math.pow((double)Math.abs(point1.x - point2.x), (double)2.0F) + Math.pow((double)Math.abs(point1.y - point2.y), (double)2.0F));
        let distance = ((point1.x - point2.x).abs().powi(2) + (point1.y - point2.y).abs().powi(2)).sqrt();
        // Java: return distance;
        distance
    }

    // Java: Bitmap getGradBmp(RectF fillToRect, RectF tileRect, int[] colors, float[] positions)
    fn getGradBmp(&mut self, fillToRect: Option<&crate::cn::wps::text::layout::data::GradientAttr::RectF>, tileRect: Option<&crate::cn::wps::text::layout::data::GradientAttr::RectF>, colors: &[i32], positions: &[f32]) -> Bitmap {
        // Java: float textWid = 0.0F;
        let mut textWid: f32 = 0.0;
        // Java: float textHei = 0.0F;
        let mut textHei: f32 = 0.0;
        // Java: RectF rect;
        let rect;
        // Java: if (this.mArtTransform == null)
        if self.mArtTransform.is_none()
        {
            // Java: rect = new RectF(this.mLnLeft, 0.0F, this.mLnLeft + this.mLnWid, this.mLnHei);
            rect = RectF::new(self.mLnLeft, 0.0, self.mLnLeft + self.mLnWid, self.mLnHei);
        }
        // Java: else
        else
        {
            // Java: textWid = this.mArtTransform.getTextWidth();
            textWid = self.mArtTransform.as_mut().unwrap().getTextWidth();
            // Java: textHei = this.mArtTransform.getTextHeight();
            textHei = self.mArtTransform.as_mut().unwrap().getTextHeight();
            // Java: rect = new RectF(0.0F, 0.0F, textWid, textHei);
            rect = RectF::new(0.0, 0.0, textWid, textHei);
        }
        // Java: if (fillToRect == null)
        let fillToRect = match fillToRect {
            Some(r) => RectF::new(r.left, r.top, r.right, r.bottom),
            // Java: fillToRect = new RectF(0.0F, 0.0F, 1.0F, 1.0F);
            None => RectF::new(0.0, 0.0, 1.0, 1.0),
        };
        // Java: if (tileRect == null)
        let tileRect = match tileRect {
            Some(r) => RectF::new(r.left, r.top, r.right, r.bottom),
            // Java: tileRect = new RectF();
            None => RectF::default(),
        };
        // Java: fillToRect = this.transPercentageRect(fillToRect, rect);
        let fillToRect = self.transPercentageRect(&fillToRect, &rect);
        // Java: tileRect = this.transPercentageRect(tileRect, rect);
        let tileRect = self.transPercentageRect(&tileRect, &rect);
        // Java: Canvas canvas = new Canvas();
        let mut canvas = Canvas::new();
        // Java: Bitmap gradBmp;
        let gradBmp;
        // Java: if (this.mArtTransform == null)
        if self.mArtTransform.is_none()
        {
            // Java: gradBmp = Bitmap.createBitmap((int)Math.ceil((double)(this.mLnLeft + this.mLnWid)), (int)Math.ceil((double)this.mLnHei), Config.ARGB_8888);
            gradBmp = Bitmap::createBitmap((self.mLnLeft + self.mLnWid).ceil() as i32, self.mLnHei.ceil() as i32, Config::ARGB_8888);
        }
        // Java: else
        else
        {
            // Java: gradBmp = Bitmap.createBitmap((int)Math.ceil((double)textWid), (int)Math.ceil((double)textHei), Config.ARGB_8888);
            gradBmp = Bitmap::createBitmap(textWid.ceil() as i32, textHei.ceil() as i32, Config::ARGB_8888);
        }
        // Java: canvas.setBitmap(gradBmp);
        canvas.setBitmap(&gradBmp);
        // Java: canvas.save();
        canvas.save();
        // Java: Paint paint = new Paint();
        let mut paint = Paint::new();
        // Java: paint.setAntiAlias(true);
        paint.setAntiAlias(true);
        // Java: PointF[] tilePoints = new PointF[4];
        // Java: tilePoints[0] = new PointF(tileRect.left, tileRect.top);
        // Java: tilePoints[1] = new PointF(tileRect.right, tileRect.top);
        // Java: tilePoints[2] = new PointF(tileRect.right, tileRect.bottom);
        // Java: tilePoints[3] = new PointF(tileRect.left, tileRect.bottom);
        let tilePoints = [
            PointF::new(tileRect.left, tileRect.top),
            PointF::new(tileRect.right, tileRect.top),
            PointF::new(tileRect.right, tileRect.bottom),
            PointF::new(tileRect.left, tileRect.bottom),
        ];
        // Java: PointF[] fillPoints = new PointF[4];
        // Java: fillPoints[0] = new PointF(fillToRect.left, fillToRect.top);
        // Java: fillPoints[1] = new PointF(fillToRect.right, fillToRect.top);
        // Java: fillPoints[2] = new PointF(fillToRect.right, fillToRect.bottom);
        // Java: fillPoints[3] = new PointF(fillToRect.left, fillToRect.bottom);
        let fillPoints = [
            PointF::new(fillToRect.left, fillToRect.top),
            PointF::new(fillToRect.right, fillToRect.top),
            PointF::new(fillToRect.right, fillToRect.bottom),
            PointF::new(fillToRect.left, fillToRect.bottom),
        ];
        // Java: if (fillToRect.width() > 0.0F && fillToRect.height() > 0.0F)
        if fillToRect.width() > 0.0 && fillToRect.height() > 0.0
        {
            // Java: int oriColor = paint.getColor();
            let oriColor = paint.getColor();
            // Java: paint.setColor(colors[0]);
            paint.setColor(colors[0]);
            // Java: canvas.drawRect(fillToRect.left, fillToRect.top, fillToRect.right, fillToRect.bottom, paint);
            canvas.drawRect(fillToRect.left, fillToRect.top, fillToRect.right, fillToRect.bottom, &paint);
            // Java: paint.setColor(oriColor);
            paint.setColor(oriColor);
        }
        // Java: this.gradFillForParallelTwoLine(canvas, paint, colors, positions, fillPoints[0], fillPoints[1], tilePoints[1], tilePoints[0], tileRect);
        self.gradFillForParallelTwoLine(&mut canvas, &mut paint, colors, positions, &fillPoints[0], &fillPoints[1], &tilePoints[1], &tilePoints[0], &tileRect);
        // Java: this.gradFillForParallelTwoLine(canvas, paint, colors, positions, fillPoints[1], fillPoints[2], tilePoints[2], tilePoints[1], tileRect);
        self.gradFillForParallelTwoLine(&mut canvas, &mut paint, colors, positions, &fillPoints[1], &fillPoints[2], &tilePoints[2], &tilePoints[1], &tileRect);
        // Java: this.gradFillForParallelTwoLine(canvas, paint, colors, positions, fillPoints[2], fillPoints[3], tilePoints[3], tilePoints[2], tileRect);
        self.gradFillForParallelTwoLine(&mut canvas, &mut paint, colors, positions, &fillPoints[2], &fillPoints[3], &tilePoints[3], &tilePoints[2], &tileRect);
        // Java: this.gradFillForParallelTwoLine(canvas, paint, colors, positions, fillPoints[0], fillPoints[3], tilePoints[3], tilePoints[0], tileRect);
        self.gradFillForParallelTwoLine(&mut canvas, &mut paint, colors, positions, &fillPoints[0], &fillPoints[3], &tilePoints[3], &tilePoints[0], &tileRect);
        // Java: canvas.restore();
        canvas.restore();
        // Java: return gradBmp;
        gradBmp
    }

    // Java: RectF transPercentageRect(RectF percentRect, RectF dstRect)
    fn transPercentageRect(&self, percentRect: &RectF, dstRect: &RectF) -> RectF {
        // Java: float left = dstRect.left + dstRect.width() * percentRect.left;
        let left = dstRect.left + dstRect.width() * percentRect.left;
        // Java: float top = dstRect.top + dstRect.height() * percentRect.top;
        let top = dstRect.top + dstRect.height() * percentRect.top;
        // Java: float right = dstRect.right - dstRect.width() * percentRect.right;
        let right = dstRect.right - dstRect.width() * percentRect.right;
        // Java: float bottom = dstRect.bottom - dstRect.height() * percentRect.bottom;
        let bottom = dstRect.bottom - dstRect.height() * percentRect.bottom;
        // Java: RectF newRectF = new RectF(left, top, right, bottom);
        let newRectF = RectF::new(left, top, right, bottom);
        // Java: return newRectF;
        newRectF
    }

    // Java: void gradFillForParallelTwoLine(Canvas canvas, Paint paint, int[] colors, float[] positions, PointF line1Start, PointF line1End, PointF line2Start, PointF line2End, RectF tileRect)
    fn gradFillForParallelTwoLine(&self, canvas: &mut Canvas, paint: &mut Paint, colors: &[i32], positions: &[f32], line1Start: &PointF, line1End: &PointF, line2Start: &PointF, line2End: &PointF, tileRect: &RectF) {
        // Java: if (Math.abs(line1Start.x - line1End.x) < 0.001F && Math.abs(line1Start.y - line1End.y) < 0.001F)
        if (line1Start.x - line1End.x).abs() < 0.001 && (line1Start.y - line1End.y).abs() < 0.001
        {
            // Java: this.drawTriangle(canvas, paint, line1Start, line2Start, line2End, colors, positions);
            self.drawTriangle(canvas, paint, line1Start, line2Start, line2End, colors, positions);
        }
        // Java: else
        else
        {
            // Java: if (!tileRect.contains(line1Start.x, line1Start.y) && !tileRect.contains(line1End.x, line1End.y))
            if !tileRect.contains(line1Start.x, line1Start.y) && !tileRect.contains(line1End.x, line1End.y)
            {
                // Java: return;
                return;
            }
            // Java: PointF footPointF = this.getFootPoint(line1Start, line2Start, line2End);
            let footPointF = self.getFootPoint_3pt(line1Start, line2Start, line2End);
            // Java: if (footPointF == null || this.isSamePoint(line1Start.x, line1Start.y, footPointF.x, footPointF.y))
            if footPointF.is_none() || self.isSamePoint(line1Start.x, line1Start.y, footPointF.as_ref().unwrap().x, footPointF.as_ref().unwrap().y)
            {
                // Java: return;
                return;
            }
            let footPointF = footPointF.unwrap();
            // Java: Path path = new Path();
            let mut path = Path::new();
            // Java: path.moveTo(line1Start.x, line1Start.y);
            path.moveTo(line1Start.x, line1Start.y);
            // Java: path.lineTo(line1End.x, line1End.y);
            path.lineTo(line1End.x, line1End.y);
            // Java: path.lineTo(line2Start.x, line2Start.y);
            path.lineTo(line2Start.x, line2Start.y);
            // Java: path.lineTo(line2End.x, line2End.y);
            path.lineTo(line2End.x, line2End.y);
            // Java: path.close();
            path.close();
            // Java: LinearGradient linearGrad = new LinearGradient(line1Start.x, line1Start.y, footPointF.x, footPointF.y, colors, positions, TileMode.MIRROR);
            let _linearGrad = LinearGradient::new(line1Start.x, line1Start.y, footPointF.x, footPointF.y, colors, positions, TileMode::MIRROR);
            // Java: paint.setShader(linearGrad);
            paint.setShader(None);
            // Java: canvas.save();
            canvas.save();
            // Java: canvas.clipPath(path);
            canvas.clipPath(&path);
            // Java: canvas.drawPath(this.mPath, paint);
            canvas.drawPath(&self.mPath, paint);
            // Java: canvas.restore();
            canvas.restore();
            // Java: paint.setShader((Shader)null);
            paint.setShader(None);
        }
    }

    // Java: boolean isSamePoint(float x1, float y1, float x2, float y2)
    fn isSamePoint(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> bool {
        // Java: float precision = 1.0E-4F;
        let precision: f32 = 1.0e-4;
        // Java: return Math.abs(x1 - x2) < precision && Math.abs(y1 - y2) < precision;
        (x1 - x2).abs() < precision && (y1 - y2).abs() < precision
    }

    // Java: void drawTriangle(Canvas canvas, Paint paint, PointF point1, PointF point2, PointF point3, int[] colors, float[] positions)
    fn drawTriangle(&self, canvas: &mut Canvas, paint: &mut Paint, point1: &PointF, point2: &PointF, point3: &PointF, colors: &[i32], positions: &[f32]) {
        // Java: canvas.save();
        canvas.save();
        // Java: PointF footPoint = this.getFootPoint(point1, point2, point3);
        let footPoint = self.getFootPoint_3pt(point1, point2, point3).unwrap_or(PointF::new(0.0, 0.0));
        // Java: LinearGradient linearGrad = new LinearGradient(point1.x, point1.y, footPoint.x, footPoint.y, colors, positions, TileMode.MIRROR);
        let _linearGrad = LinearGradient::new(point1.x, point1.y, footPoint.x, footPoint.y, colors, positions, TileMode::MIRROR);
        // Java: paint.setShader(linearGrad);
        paint.setShader(None);
        // Java: Path path = new Path();
        let mut path = Path::new();
        // Java: path.moveTo(point1.x, point1.y);
        path.moveTo(point1.x, point1.y);
        // Java: path.lineTo(point2.x, point2.y);
        path.lineTo(point2.x, point2.y);
        // Java: path.lineTo(point3.x, point3.y);
        path.lineTo(point3.x, point3.y);
        // Java: path.close();
        path.close();
        // Java: canvas.clipPath(path);
        canvas.clipPath(&path);
        // Java: canvas.drawPath(this.mPath, paint);
        canvas.drawPath(&self.mPath, paint);
        // Java: canvas.restore();
        canvas.restore();
        // Java: paint.setShader((Shader)null);
        paint.setShader(None);
    }

    // Java: PointF getFootPoint(PointF C, PointF A, PointF B)
    fn getFootPoint_3pt(&self, c: &PointF, a: &PointF, b: &PointF) -> Option<PointF> {
        // Java: PointF destPoint = null;
        let destPoint;
        // Java: if (A.x == B.x)
        if a.x == b.x
        {
            // Java: destPoint = new PointF(A.x, C.y);
            destPoint = Some(PointF::new(a.x, c.y));
        }
        // Java: else if (A.y == B.y)
        else if a.y == b.y
        {
            // Java: destPoint = new PointF(C.x, A.y);
            destPoint = Some(PointF::new(c.x, a.y));
        }
        // Java: else
        else
        {
            // Java: float k = (B.y - A.y) / (B.x - A.x);
            let k = (b.y - a.y) / (b.x - a.x);
            // Java: destPoint = this.getFootPoint(k, A, C);
            destPoint = self.getFootPoint_with_k(k, a, c);
        }
        // Java: return destPoint;
        destPoint
    }

    // Java: PointF getFootPoint(float k, PointF A, PointF C)
    fn getFootPoint_with_k(&self, k: f32, a: &PointF, c: &PointF) -> Option<PointF> {
        // Java: if (C.x - A.x != 0.0F)
        if c.x - a.x != 0.0
        {
            // Java: float k1 = (C.y - A.y) / (C.x - A.x);
            let k1 = (c.y - a.y) / (c.x - a.x);
            // Java: if (Math.abs(k1 - k) < 1.0E-5F)
            if (k1 - k).abs() < 1.0e-5
            {
                // Java: return C;
                return Some(c.clone());
            }
            // Java: else
            {
                // Java: k1 = (k * A.x + C.x / k + C.y - A.y) / (1.0F / k + k);
                let k1 = (k * a.x + c.x / k + c.y - a.y) / (1.0 / k + k);
                // Java: float y = k * (k1 - A.x) + A.y;
                let y = k * (k1 - a.x) + a.y;
                // Java: return new PointF(k1, y);
                return Some(PointF::new(k1, y));
            }
        }
        // Java: else
        else
        {
            // Java: return C;
            Some(c.clone())
        }
    }
}
