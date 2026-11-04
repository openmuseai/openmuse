use super::super::control::grid::render::graphics::{Canvas, Paint, Path, Rect};
use serde_json::{json, Value};
// java public final class CellDrawerUtil
pub struct CellDrawerUtil;
// rust: Native bitmap value carries the original off-screen Canvas commands to the platform allocator.
pub struct PatternBitmap {
    pub width: i32,
    pub height: i32,
    pub commands: Vec<Value>,
}
impl CellDrawerUtil {
    // java public static void drawBorderLine(short style, Canvas canvas, Paint paint, int color, float[] points) {
    // java if (style != 0) {
    // java if (KmoColor.isAutomaticColor(color)) {
    // java color = -16777216;
    // java }
    // java
    // java paint.reset();
    // java paint.setAntiAlias(true);
    // java paint.setColor(color);
    // java paint.setStyle(Style.STROKE);
    // java float[] intervals = null;
    // java float strokeWidth = 1.0F;
    // java switch (style) {
    // java case 1:
    // java strokeWidth = 1.0F;
    // java break;
    // java case 2:
    // java strokeWidth = 2.0F;
    // java break;
    // java case 3:
    // java case 8:
    // java strokeWidth = style == 3 ? 1.0F : 2.0F;
    // java intervals = getDataStorage(2);
    // java intervals[0] = 7.0F;
    // java intervals[1] = 3.0F;
    // java drawDashDotLine(canvas, paint, points, intervals, strokeWidth);
    // java return;
    // java case 4:
    // java drawDotLine(canvas, paint, points, 1);
    // java return;
    // java case 5:
    // java strokeWidth = 3.0F;
    // java break;
    // java case 6:
    // java strokeWidth = 1.0F;
    // java drawSolidLine(canvas, paint, points, 2, strokeWidth);
    // java return;
    // java case 7:
    // java drawDotLine(canvas, paint, points, 2);
    // java return;
    // java case 9:
    // java case 10:
    // java strokeWidth = style == 9 ? 1.0F : 2.0F;
    // java intervals = getDashDotData();
    // java drawDashDotLine(canvas, paint, points, intervals, strokeWidth);
    // java return;
    // java case 11:
    // java case 12:
    // java strokeWidth = style == 11 ? 1.0F : 2.0F;
    // java intervals = getDashDotDotData();
    // java drawDashDotLine(canvas, paint, points, intervals, strokeWidth);
    // java return;
    // java case 13:
    // java drawSlantedDashDotLine(canvas, paint, points, 2.0F);
    // java return;
    // java default:
    // java strokeWidth = 1.0F;
    // java }
    // java
    // java drawSolidLine(canvas, paint, points, 1, strokeWidth);
    // java }
    // java }
    pub fn drawBorderLine(
        style: i16,
        c: &mut Canvas,
        p: &mut Paint,
        mut color: i32,
        points: &mut [f32; 4],
    ) {
        if style == 0 {
            return;
        }
        if crate::cn::wps::moss::util::KmoColor::KmoColor::isAutomaticColor(color) {
            color = -16777216;
        }
        *p = Paint::default();
        p.flags |= 1;
        p.color = color;
        p.style = 1;
        let stroke = match style {
            1 => 1.0,
            2 => 2.0,
            3 | 8 => {
                Self::drawDashDotLine(
                    c,
                    p,
                    points,
                    &[7.0, 3.0],
                    if style == 3 { 1.0 } else { 2.0 },
                );
                return;
            }
            4 => {
                Self::drawDotLine(c, p, points, 1);
                return;
            }
            5 => 3.0,
            6 => {
                Self::drawSolidLine(c, p, points, 2, 1.0);
                return;
            }
            7 => {
                Self::drawDotLine(c, p, points, 2);
                return;
            }
            9 | 10 => {
                Self::drawDashDotLine(
                    c,
                    p,
                    points,
                    &[7.0, 3.0, 2.0, 2.0],
                    if style == 9 { 1.0 } else { 2.0 },
                );
                return;
            }
            11 | 12 => {
                Self::drawDashDotLine(
                    c,
                    p,
                    points,
                    &[7.0, 3.0, 2.0, 3.0, 2.0, 3.0],
                    if style == 11 { 1.0 } else { 2.0 },
                );
                return;
            }
            13 => {
                Self::drawSlantedDashDotLine(c, p, points, 2.0);
                return;
            }
            _ => 1.0,
        };
        Self::drawSolidLine(c, p, points, 1, stroke);
    }
    // java private static void setPath(float[] points, Path path) {
    // java path.reset();
    // java path.moveTo(points[0], points[1]);
    // java path.lineTo(points[2], points[3]);
    // java }
    fn setPath(v: &[f32; 4], path: &mut Path) {
        path.reset();
        path.moveTo(v[0], v[1]);
        path.lineTo(v[2], v[3]);
    }
    // java private static void drawDotLine(Canvas canvas, Paint paint, float[] points, int dotSize) {
    // java float[] intervals = getDataStorage(2);
    // java intervals[0] = (float)dotSize;
    // java intervals[1] = 1.0F;
    // java PathEffect effect = new DashPathEffect(intervals, 1.0F);
    // java paint.setPathEffect(effect);
    // java Path path = (Path)sLocalPath.get();
    // java setPath(points, path);
    // java canvas.drawPath(path, paint);
    // java }
    fn drawDotLine(c: &mut Canvas, p: &mut Paint, v: &[f32; 4], size: i32) {
        p.pathEffect = Some(json!({"intervals":[size as f32,1.0],"phase":1.0}));
        let mut path = Path::default();
        Self::setPath(v, &mut path);
        c.drawPath(&path, p);
    }
    // java private static void drawDashDotLine(Canvas canvas, Paint paint, float[] points, float[] intervals, float strokeWidth) {
    // java paint.setStrokeWidth(strokeWidth);
    // java PathEffect effect = new DashPathEffect(intervals, 1.0F);
    // java paint.setPathEffect(effect);
    // java Path path = (Path)sLocalPath.get();
    // java setPath(points, path);
    // java canvas.drawPath(path, paint);
    // java }
    fn drawDashDotLine(
        c: &mut Canvas,
        p: &mut Paint,
        v: &[f32; 4],
        intervals: &[f32],
        stroke: f32,
    ) {
        p.strokeWidth = stroke;
        p.pathEffect = Some(json!({"intervals":intervals,"phase":1.0}));
        let mut path = Path::default();
        Self::setPath(v, &mut path);
        c.drawPath(&path, p);
    }
    // java private static void drawSolidLine(Canvas canvas, Paint paint, float[] points, int count, float strokeWidth) {
    // java paint.setStrokeCap(Cap.SQUARE);
    // java paint.setStrokeJoin(Join.BEVEL);
    // java paint.setStrokeWidth(strokeWidth);
    // java
    // java for(int i = 0; i < count; ++i) {
    // java canvas.drawLines(points, 0, points.length, paint);
    // java if (points[1] == points[3]) {
    // java points[1] += 2.0F;
    // java points[3] += 2.0F;
    // java points[2] += 2.0F;
    // java } else if (points[0] == points[2]) {
    // java points[0] += 2.0F;
    // java points[2] += 2.0F;
    // java points[3] += 2.0F;
    // java } else {
    // java points[1] += 2.0F;
    // java points[3] += 2.0F;
    // java }
    // java }
    // java
    // java }
    fn drawSolidLine(c: &mut Canvas, p: &mut Paint, v: &mut [f32; 4], count: i32, stroke: f32) {
        p.strokeCap = 2;
        p.strokeJoin = 2;
        p.strokeWidth = stroke;
        for _ in 0..count {
            c.drawLines(v, p);
            if v[1] == v[3] {
                v[1] += 2.0;
                v[3] += 2.0;
                v[2] += 2.0;
            } else if v[0] == v[2] {
                v[0] += 2.0;
                v[2] += 2.0;
                v[3] += 2.0;
            } else {
                v[1] += 2.0;
                v[3] += 2.0;
            }
        }
    }
    // java private static void drawSlantedDashDotLine(Canvas canvas, Paint paint, float[] pts, float strokeWidth) {
    // java canvas.save();
    // java if (pts[0] == pts[2]) {
    // java float minY = Math.min(pts[1], pts[3]);
    // java float maxY = Math.max(pts[1], pts[3]);
    // java canvas.translate(pts[0], minY);
    // java canvas.rotate(90.0F);
    // java drawSlantedDashDotLineH(canvas, maxY - minY, paint, 2.0F);
    // java } else if (pts[1] == pts[3]) {
    // java float minX = Math.min(pts[0], pts[2]);
    // java float maxX = Math.max(pts[0], pts[2]);
    // java canvas.translate(minX, pts[1]);
    // java drawSlantedDashDotLineH(canvas, maxX - minX, paint, 2.0F);
    // java } else {
    // java if (pts[0] < pts[2]) {
    // java canvas.translate(pts[0], pts[1]);
    // java } else {
    // java canvas.translate(pts[2], pts[3]);
    // java }
    // java
    // java float length = (float)Math.sqrt(Math.pow((double)(pts[0] - pts[2]), (double)2.0F) + Math.pow((double)(pts[3] - pts[1]), (double)2.0F));
    // java double atan = Math.atan((double)((pts[3] - pts[1]) / (pts[2] - pts[0])));
    // java double d = atan / Math.PI * (double)180.0F;
    // java canvas.rotate((float)d);
    // java drawSlantedDashDotLineH(canvas, length, paint, 1.0F);
    // java }
    // java
    // java canvas.restore();
    // java }
    fn drawSlantedDashDotLine(c: &mut Canvas, p: &mut Paint, v: &[f32; 4], _stroke: f32) {
        c.save();
        if v[0] == v[2] {
            let min = v[1].min(v[3]);
            let max = v[1].max(v[3]);
            c.translate(v[0], min);
            c.rotate(90.0);
            Self::drawSlantedDashDotLineH(c, max - min, p, 2.0);
        } else if v[1] == v[3] {
            let min = v[0].min(v[2]);
            let max = v[0].max(v[2]);
            c.translate(min, v[1]);
            Self::drawSlantedDashDotLineH(c, max - min, p, 2.0);
        } else {
            if v[0] < v[2] {
                c.translate(v[0], v[1]);
            } else {
                c.translate(v[2], v[3]);
            }
            let length =
                (((v[0] - v[2]) as f64).powf(2.0) + ((v[3] - v[1]) as f64).powf(2.0)).sqrt() as f32;
            let atan = (((v[3] - v[1]) / (v[2] - v[0])) as f64).atan();
            let d = atan / std::f64::consts::PI * 180.0;
            c.rotate(d as f32);
            Self::drawSlantedDashDotLineH(c, length, p, 1.0);
        }
        c.restore();
    }
    // java private static void drawSlantedDashDotLineH(Canvas canvas, float stopLength, Paint paint, float offset) {
    // java canvas.save();
    // java canvas.clipRect(0.0F, -1.0F, stopLength, 1.0F);
    // java Path path = (Path)sLocalPath.get();
    // java path.reset();
    // java path.moveTo(offset, -offset);
    // java path.lineTo(-offset, offset);
    // java float offset1 = 6.0F;
    // java float offset2 = 7.0F;
    // java float strokeWidth1 = 2.0F;
    // java float strokeWidth2 = 5.0F;
    // java
    // java for(double go = (double)0.0F; go < (double)stopLength; go += (double)(offset1 + offset2)) {
    // java paint.setStrokeWidth(strokeWidth1);
    // java canvas.drawPath(path, paint);
    // java canvas.translate(offset1, 0.0F);
    // java paint.setStrokeWidth(strokeWidth2);
    // java canvas.drawPath(path, paint);
    // java canvas.translate(offset2, 0.0F);
    // java }
    // java
    // java canvas.restore();
    // java }
    fn drawSlantedDashDotLineH(c: &mut Canvas, length: f32, p: &mut Paint, offset: f32) {
        c.save();
        c.clipRect__float(0.0, -1.0, length, 1.0);
        let mut path = Path::default();
        path.moveTo(offset, -offset);
        path.lineTo(-offset, offset);
        let mut go = 0.0f64;
        while go < length as f64 {
            p.strokeWidth = 2.0;
            c.drawPath(&path, p);
            c.translate(6.0, 0.0);
            p.strokeWidth = 5.0;
            c.drawPath(&path, p);
            c.translate(7.0, 0.0);
            go += 13.0;
        }
        c.restore();
    }

    // java public static void drawPatternStyle(int color, short style, Canvas canvas, Paint paint, Rect rect) {
    // java drawPatternStyle(color, style, canvas, paint, rect, sContentScale);
    // java }
    pub fn drawPatternStyle(color: i32, style: i16, c: &mut Canvas, p: &mut Paint, r: &Rect) {
        Self::drawPatternStyle__scale(color, style, c, p, r, 1.0);
    }
    // java public static void drawPatternStyle(int color, short style, Canvas canvas, Paint paint, Rect rect, float scale) {
    // java paint.reset();
    // java paint.setColor(color);
    // java if (style == 1) {
    // java fillRect(canvas, paint, rect);
    // java } else {
    // java Bitmap bitmap = getPatternBitmapMold(style, paint, 6, scale);
    // java if (bitmap != null) {
    // java BitmapShader shader = new BitmapShader(bitmap, TileMode.REPEAT, TileMode.REPEAT);
    // java int width = bitmap.getWidth();
    // java int height = bitmap.getHeight();
    // java int timesX = rect.left / width;
    // java int timesY = rect.top / height;
    // java int offsetX = width * timesX;
    // java int offsetY = height * timesY;
    // java canvas.translate((float)offsetX, (float)offsetY);
    // java paint.setStyle(Style.FILL);
    // java paint.setShader(shader);
    // java canvas.drawRect((float)(rect.left - offsetX), (float)(rect.top - offsetY), (float)(rect.right - offsetX), (float)(rect.bottom - offsetY), paint);
    // java paint.reset();
    // java canvas.translate((float)(-offsetX), (float)(-offsetY));
    // java }
    // java
    // java Path path = (Path)sLocalPath.get();
    // java path.close();
    // java }
    // java }
    pub fn drawPatternStyle__scale(
        color: i32,
        style: i16,
        c: &mut Canvas,
        p: &mut Paint,
        r: &Rect,
        scale: f32,
    ) {
        *p = Paint::default();
        p.color = color;
        if style == 1 {
            Self::fillRect(c, p, r);
        } else {
            if let Some(bm) = Self::getPatternBitmapMold(style, p, 6, scale) {
                let x = bm.width * (r.left / bm.width);
                let y = bm.height * (r.top / bm.height);
                c.translate(x as f32, y as f32);
                p.style = 0;
                p.shader = Some(
                    json!({"kind":"bitmap","width":bm.width,"height":bm.height,"config":"ARGB_4444","commands":bm.commands,"tileMode":"repeat"}),
                );
                c.drawRect__float(
                    (r.left - x) as f32,
                    (r.top - y) as f32,
                    (r.right - x) as f32,
                    (r.bottom - y) as f32,
                    p,
                );
                *p = Paint::default();
                c.translate(-x as f32, -y as f32);
            }
        }
    }
    // java private static Bitmap getPatternBitmapMold(short style, Paint paint, int moldWidth, float contentScale) {
    // java int scale = Math.round(contentScale);
    // java Bitmap bitmap;
    // java switch (style) {
    // java case 2:
    // java bitmap = fillMediumGray(paint, scale);
    // java break;
    // java case 3:
    // java bitmap = fillDarkGray(paint, scale);
    // java break;
    // java case 4:
    // java bitmap = fillLightGray(paint, scale);
    // java break;
    // java case 5:
    // java bitmap = fillDarkHorizontal(paint, scale);
    // java break;
    // java case 6:
    // java bitmap = fillDarkVertical(paint, scale);
    // java break;
    // java case 7:
    // java bitmap = fillDarkDown(paint, scale);
    // java break;
    // java case 8:
    // java bitmap = fillDarkUp(paint, scale);
    // java break;
    // java case 9:
    // java bitmap = fillDarkGrid(paint, scale);
    // java break;
    // java case 10:
    // java bitmap = fillDarkTrellis(paint, scale);
    // java break;
    // java case 11:
    // java bitmap = fillLightHorizontal(paint, scale);
    // java break;
    // java case 12:
    // java bitmap = fillLightVertical(paint, scale);
    // java break;
    // java case 13:
    // java bitmap = fillLightDown(paint, scale);
    // java break;
    // java case 14:
    // java bitmap = fillLightUp(paint, scale);
    // java break;
    // java case 15:
    // java bitmap = fillLightGrid(paint, scale);
    // java break;
    // java case 16:
    // java bitmap = fillLightTrellis(paint, scale);
    // java break;
    // java case 17:
    // java bitmap = fillGray125(paint, scale);
    // java break;
    // java case 18:
    // java bitmap = fillGray0625(paint, scale);
    // java break;
    // java default:
    // java return null;
    // java }
    // java
    // java return bitmap;
    // java }
    pub fn getPatternBitmapMold(
        style: i16,
        p: &mut Paint,
        _moldWidth: i32,
        contentScale: f32,
    ) -> Option<PatternBitmap> {
        let scale = crate::java_runtime::javaMathRoundFloat(contentScale);
        Some(match style {
            2 => Self::fillMediumGray(p, scale),
            3 => Self::fillDarkGray(p, scale),
            4 => Self::fillLightGray(p, scale),
            5 => Self::fillDarkHorizontal(p, scale),
            6 => Self::fillDarkVertical(p, scale),
            7 => Self::fillDarkDown(p, scale),
            8 => Self::fillDarkUp(p, scale),
            9 => Self::fillDarkGrid(p, scale),
            10 => Self::fillDarkTrellis(p, scale),
            11 => Self::fillLightHorizontal(p, scale),
            12 => Self::fillLightVertical(p, scale),
            13 => Self::fillLightDown(p, scale),
            14 => Self::fillLightUp(p, scale),
            15 => Self::fillLightGrid(p, scale),
            16 => Self::fillLightTrellis(p, scale),
            17 => Self::fillGray125(p, scale),
            18 => Self::fillGray0625(p, scale),
            _ => return None,
        })
    }
    // java private static void fillRect(Canvas canvas, Paint paint, Rect rect) {
    // java paint.setStyle(Style.FILL);
    // java canvas.drawRect(rect, paint);
    // java }
    fn fillRect(c: &mut Canvas, p: &mut Paint, r: &Rect) {
        p.style = 0;
        c.drawRect(r, p);
    }
    // java private static Bitmap fillDarkGray(Paint paint, int scale) {
    // java Bitmap bitmap = Bitmap.createBitmap(4 * scale, 2 * scale, Config.ARGB_4444);
    // java Canvas canvas = new Canvas(bitmap);
    // java canvas.scale((float)scale, (float)scale);
    // java canvas.translate(-0.5F, -0.5F);
    // java canvas.drawRect(0.0F, 1.0F, 1.0F, 2.0F, paint);
    // java canvas.drawRect(4.0F, 1.0F, 5.0F, 2.0F, paint);
    // java canvas.drawRect(1.0F, 0.0F, 4.0F, 1.0F, paint);
    // java canvas.drawRect(1.0F, 2.0F, 4.0F, 3.0F, paint);
    // java canvas.drawRect(1.0F, 0.0F, 2.0F, 3.0F, paint);
    // java canvas.drawRect(3.0F, 0.0F, 4.0F, 3.0F, paint);
    // java return bitmap;
    // java }
    fn fillDarkGray(p: &mut Paint, scale: i32) -> PatternBitmap {
        let mut c = Canvas::new(4 * scale, 2 * scale);
        c.scale(scale as f32, scale as f32, 0.0, 0.0);
        c.translate(-0.5, -0.5);
        c.drawRect__float(0.0, 1.0, 1.0, 2.0, p);
        c.drawRect__float(4.0, 1.0, 5.0, 2.0, p);
        c.drawRect__float(1.0, 0.0, 4.0, 1.0, p);
        c.drawRect__float(1.0, 2.0, 4.0, 3.0, p);
        c.drawRect__float(1.0, 0.0, 2.0, 3.0, p);
        c.drawRect__float(3.0, 0.0, 4.0, 3.0, p);
        PatternBitmap {
            width: c.width,
            height: c.height,
            commands: c.commands,
        }
    }
    // java private static Bitmap fillLightGray(Paint paint, int scale) {
    // java Bitmap bitmap = Bitmap.createBitmap(4 * scale, 2 * scale, Config.ARGB_4444);
    // java Canvas canvas = new Canvas(bitmap);
    // java canvas.scale((float)scale, (float)scale);
    // java canvas.translate(-0.5F, -0.5F);
    // java canvas.drawRect(0.0F, 0.0F, 1.0F, 1.0F, paint);
    // java canvas.drawRect(0.0F, 2.0F, 1.0F, 3.0F, paint);
    // java canvas.drawRect(4.0F, 0.0F, 5.0F, 1.0F, paint);
    // java canvas.drawRect(4.0F, 2.0F, 5.0F, 3.0F, paint);
    // java canvas.drawRect(2.0F, 1.0F, 3.0F, 2.0F, paint);
    // java return bitmap;
    // java }
    fn fillLightGray(p: &mut Paint, scale: i32) -> PatternBitmap {
        let mut c = Canvas::new(4 * scale, 2 * scale);
        c.scale(scale as f32, scale as f32, 0.0, 0.0);
        c.translate(-0.5, -0.5);
        c.drawRect__float(0.0, 0.0, 1.0, 1.0, p);
        c.drawRect__float(0.0, 2.0, 1.0, 3.0, p);
        c.drawRect__float(4.0, 0.0, 5.0, 1.0, p);
        c.drawRect__float(4.0, 2.0, 5.0, 3.0, p);
        c.drawRect__float(2.0, 1.0, 3.0, 2.0, p);
        PatternBitmap {
            width: c.width,
            height: c.height,
            commands: c.commands,
        }
    }
    // java private static Bitmap fillMediumGray(Paint paint, int scale) {
    // java Bitmap bitmap = Bitmap.createBitmap(2 * scale, 2 * scale, Config.ARGB_4444);
    // java Canvas canvas = new Canvas(bitmap);
    // java canvas.scale((float)scale, (float)scale);
    // java canvas.translate(-0.5F, -0.5F);
    // java canvas.drawRect(0.0F, 0.0F, 1.0F, 1.0F, paint);
    // java canvas.drawRect(0.0F, 2.0F, 1.0F, 3.0F, paint);
    // java canvas.drawRect(2.0F, 0.0F, 3.0F, 1.0F, paint);
    // java canvas.drawRect(2.0F, 2.0F, 3.0F, 3.0F, paint);
    // java canvas.drawRect(1.0F, 1.0F, 2.0F, 2.0F, paint);
    // java return bitmap;
    // java }
    fn fillMediumGray(p: &mut Paint, scale: i32) -> PatternBitmap {
        let mut c = Canvas::new(2 * scale, 2 * scale);
        c.scale(scale as f32, scale as f32, 0.0, 0.0);
        c.translate(-0.5, -0.5);
        c.drawRect__float(0.0, 0.0, 1.0, 1.0, p);
        c.drawRect__float(0.0, 2.0, 1.0, 3.0, p);
        c.drawRect__float(2.0, 0.0, 3.0, 1.0, p);
        c.drawRect__float(2.0, 2.0, 3.0, 3.0, p);
        c.drawRect__float(1.0, 1.0, 2.0, 2.0, p);
        PatternBitmap {
            width: c.width,
            height: c.height,
            commands: c.commands,
        }
    }
    // java private static Bitmap fillDarkGrid(Paint paint, int scale) {
    // java Bitmap bitmap = Bitmap.createBitmap(4 * scale, 4 * scale, Config.ARGB_4444);
    // java Canvas canvas = new Canvas(bitmap);
    // java canvas.scale((float)scale, (float)scale);
    // java canvas.translate(-1.0F, -1.0F);
    // java canvas.drawRect(0.0F, 0.0F, 2.0F, 2.0F, paint);
    // java canvas.drawRect(0.0F, 4.0F, 2.0F, 6.0F, paint);
    // java canvas.drawRect(4.0F, 0.0F, 6.0F, 2.0F, paint);
    // java canvas.drawRect(4.0F, 4.0F, 6.0F, 6.0F, paint);
    // java canvas.drawRect(2.0F, 2.0F, 4.0F, 4.0F, paint);
    // java return bitmap;
    // java }
    fn fillDarkGrid(p: &mut Paint, scale: i32) -> PatternBitmap {
        let mut c = Canvas::new(4 * scale, 4 * scale);
        c.scale(scale as f32, scale as f32, 0.0, 0.0);
        c.translate(-1.0, -1.0);
        c.drawRect__float(0.0, 0.0, 2.0, 2.0, p);
        c.drawRect__float(0.0, 4.0, 2.0, 6.0, p);
        c.drawRect__float(4.0, 0.0, 6.0, 2.0, p);
        c.drawRect__float(4.0, 4.0, 6.0, 6.0, p);
        c.drawRect__float(2.0, 2.0, 4.0, 4.0, p);
        PatternBitmap {
            width: c.width,
            height: c.height,
            commands: c.commands,
        }
    }
    // java private static Bitmap fillLightGrid(Paint paint, int scale) {
    // java Bitmap bitmap = Bitmap.createBitmap(4 * scale, 4 * scale, Config.ARGB_4444);
    // java Canvas canvas = new Canvas(bitmap);
    // java canvas.scale((float)scale, (float)scale);
    // java canvas.translate(-0.5F, -0.5F);
    // java canvas.drawRect(0.0F, 0.0F, 5.0F, 1.0F, paint);
    // java canvas.drawRect(0.0F, 4.0F, 5.0F, 5.0F, paint);
    // java canvas.drawRect(0.0F, 0.0F, 1.0F, 5.0F, paint);
    // java canvas.drawRect(4.0F, 0.0F, 5.0F, 5.0F, paint);
    // java return bitmap;
    // java }
    fn fillLightGrid(p: &mut Paint, scale: i32) -> PatternBitmap {
        let mut c = Canvas::new(4 * scale, 4 * scale);
        c.scale(scale as f32, scale as f32, 0.0, 0.0);
        c.translate(-0.5, -0.5);
        c.drawRect__float(0.0, 0.0, 5.0, 1.0, p);
        c.drawRect__float(0.0, 4.0, 5.0, 5.0, p);
        c.drawRect__float(0.0, 0.0, 1.0, 5.0, p);
        c.drawRect__float(4.0, 0.0, 5.0, 5.0, p);
        PatternBitmap {
            width: c.width,
            height: c.height,
            commands: c.commands,
        }
    }
    // java private static Bitmap fillDarkTrellis(Paint paint, int scale) {
    // java Bitmap bitmap = Bitmap.createBitmap(4 * scale, 6 * scale, Config.ARGB_4444);
    // java Canvas canvas = new Canvas(bitmap);
    // java canvas.scale((float)scale, (float)scale);
    // java canvas.translate(-1.0F, -2.0F);
    // java canvas.drawRect(0.0F, 0.0F, 2.0F, 4.0F, paint);
    // java canvas.drawRect(0.0F, 6.0F, 2.0F, 10.0F, paint);
    // java canvas.drawRect(4.0F, 0.0F, 6.0F, 4.0F, paint);
    // java canvas.drawRect(4.0F, 6.0F, 6.0F, 10.0F, paint);
    // java canvas.drawRect(2.0F, 3.0F, 4.0F, 7.0F, paint);
    // java return bitmap;
    // java }
    fn fillDarkTrellis(p: &mut Paint, scale: i32) -> PatternBitmap {
        let mut c = Canvas::new(4 * scale, 6 * scale);
        c.scale(scale as f32, scale as f32, 0.0, 0.0);
        c.translate(-1.0, -2.0);
        c.drawRect__float(0.0, 0.0, 2.0, 4.0, p);
        c.drawRect__float(0.0, 6.0, 2.0, 10.0, p);
        c.drawRect__float(4.0, 0.0, 6.0, 4.0, p);
        c.drawRect__float(4.0, 6.0, 6.0, 10.0, p);
        c.drawRect__float(2.0, 3.0, 4.0, 7.0, p);
        PatternBitmap {
            width: c.width,
            height: c.height,
            commands: c.commands,
        }
    }
    // java private static Bitmap fillLightTrellis(Paint paint, int scale) {
    // java Bitmap bitmap = Bitmap.createBitmap(4 * scale, 4 * scale, Config.ARGB_4444);
    // java Canvas canvas = new Canvas(bitmap);
    // java canvas.scale((float)scale, (float)scale);
    // java canvas.translate(-0.5F, -0.5F);
    // java canvas.drawRect(0.0F, 0.0F, 1.0F, 1.0F, paint);
    // java canvas.drawRect(0.0F, 4.0F, 1.0F, 5.0F, paint);
    // java canvas.drawRect(4.0F, 0.0F, 5.0F, 1.0F, paint);
    // java canvas.drawRect(4.0F, 4.0F, 5.0F, 5.0F, paint);
    // java canvas.drawRect(1.0F, 1.0F, 2.0F, 2.0F, paint);
    // java canvas.drawRect(3.0F, 1.0F, 4.0F, 2.0F, paint);
    // java canvas.drawRect(1.0F, 3.0F, 2.0F, 4.0F, paint);
    // java canvas.drawRect(3.0F, 3.0F, 4.0F, 4.0F, paint);
    // java canvas.drawRect(2.0F, 2.0F, 3.0F, 3.0F, paint);
    // java return bitmap;
    // java }
    fn fillLightTrellis(p: &mut Paint, scale: i32) -> PatternBitmap {
        let mut c = Canvas::new(4 * scale, 4 * scale);
        c.scale(scale as f32, scale as f32, 0.0, 0.0);
        c.translate(-0.5, -0.5);
        c.drawRect__float(0.0, 0.0, 1.0, 1.0, p);
        c.drawRect__float(0.0, 4.0, 1.0, 5.0, p);
        c.drawRect__float(4.0, 0.0, 5.0, 1.0, p);
        c.drawRect__float(4.0, 4.0, 5.0, 5.0, p);
        c.drawRect__float(1.0, 1.0, 2.0, 2.0, p);
        c.drawRect__float(3.0, 1.0, 4.0, 2.0, p);
        c.drawRect__float(1.0, 3.0, 2.0, 4.0, p);
        c.drawRect__float(3.0, 3.0, 4.0, 4.0, p);
        c.drawRect__float(2.0, 2.0, 3.0, 3.0, p);
        PatternBitmap {
            width: c.width,
            height: c.height,
            commands: c.commands,
        }
    }
    // java private static Bitmap fillGray0625(Paint paint, int scale) {
    // java Bitmap bitmap = Bitmap.createBitmap(8 * scale, 4 * scale, Config.ARGB_4444);
    // java Canvas canvas = new Canvas(bitmap);
    // java canvas.scale((float)scale, (float)scale);
    // java canvas.translate(-0.5F, -0.5F);
    // java canvas.drawRect(0.0F, 0.0F, 1.0F, 1.0F, paint);
    // java canvas.drawRect(0.0F, 4.0F, 1.0F, 5.0F, paint);
    // java canvas.drawRect(8.0F, 0.0F, 9.0F, 1.0F, paint);
    // java canvas.drawRect(8.0F, 4.0F, 9.0F, 5.0F, paint);
    // java canvas.drawRect(4.0F, 2.0F, 5.0F, 3.0F, paint);
    // java return bitmap;
    // java }
    fn fillGray0625(p: &mut Paint, scale: i32) -> PatternBitmap {
        let mut c = Canvas::new(8 * scale, 4 * scale);
        c.scale(scale as f32, scale as f32, 0.0, 0.0);
        c.translate(-0.5, -0.5);
        c.drawRect__float(0.0, 0.0, 1.0, 1.0, p);
        c.drawRect__float(0.0, 4.0, 1.0, 5.0, p);
        c.drawRect__float(8.0, 0.0, 9.0, 1.0, p);
        c.drawRect__float(8.0, 4.0, 9.0, 5.0, p);
        c.drawRect__float(4.0, 2.0, 5.0, 3.0, p);
        PatternBitmap {
            width: c.width,
            height: c.height,
            commands: c.commands,
        }
    }
    // java private static Bitmap fillGray125(Paint paint, int scale) {
    // java Bitmap bitmap = Bitmap.createBitmap(4 * scale, 4 * scale, Config.ARGB_4444);
    // java Canvas canvas = new Canvas(bitmap);
    // java canvas.scale((float)scale, (float)scale);
    // java canvas.translate(-0.5F, -0.5F);
    // java canvas.drawRect(0.0F, 0.0F, 1.0F, 1.0F, paint);
    // java canvas.drawRect(0.0F, 4.0F, 1.0F, 5.0F, paint);
    // java canvas.drawRect(4.0F, 0.0F, 5.0F, 1.0F, paint);
    // java canvas.drawRect(4.0F, 4.0F, 5.0F, 5.0F, paint);
    // java canvas.drawRect(2.0F, 2.0F, 3.0F, 3.0F, paint);
    // java return bitmap;
    // java }
    fn fillGray125(p: &mut Paint, scale: i32) -> PatternBitmap {
        let mut c = Canvas::new(4 * scale, 4 * scale);
        c.scale(scale as f32, scale as f32, 0.0, 0.0);
        c.translate(-0.5, -0.5);
        c.drawRect__float(0.0, 0.0, 1.0, 1.0, p);
        c.drawRect__float(0.0, 4.0, 1.0, 5.0, p);
        c.drawRect__float(4.0, 0.0, 5.0, 1.0, p);
        c.drawRect__float(4.0, 4.0, 5.0, 5.0, p);
        c.drawRect__float(2.0, 2.0, 3.0, 3.0, p);
        PatternBitmap {
            width: c.width,
            height: c.height,
            commands: c.commands,
        }
    }
    // java private static Bitmap fillDialImpl(Paint paint, int scale, boolean up, boolean light) {
    // java int width = 6;
    // java int height = 6;
    // java Bitmap bitmap = Bitmap.createBitmap(width * scale, height * scale, Config.ARGB_4444);
    // java Canvas canvas = new Canvas(bitmap);
    // java canvas.scale((float)scale, (float)scale);
    // java if (light) {
    // java paint.setStrokeWidth(1.0F);
    // java } else {
    // java paint.setStrokeWidth(2.0F);
    // java }
    // java
    // java paint.setDither(true);
    // java paint.setStyle(Style.STROKE);
    // java paint.setStrokeCap(Cap.SQUARE);
    // java if (!up) {
    // java canvas.drawLine(0.0F, 0.0F, (float)width, (float)height, paint);
    // java canvas.drawLine(-2.0F, (float)(height - 2), 2.0F, (float)(height + 2), paint);
    // java canvas.drawLine((float)(width - 2), -2.0F, (float)(width + 2), 2.0F, paint);
    // java } else {
    // java canvas.drawLine(0.0F, (float)height, (float)width, 0.0F, paint);
    // java canvas.drawLine(-2.0F, 2.0F, 2.0F, -2.0F, paint);
    // java canvas.drawLine((float)(width - 2), (float)(height + 2), (float)(width + 2), (float)(height - 2), paint);
    // java }
    // java
    // java return bitmap;
    // java }
    fn fillDialImpl(p: &mut Paint, scale: i32, up: bool, light: bool) -> PatternBitmap {
        let (w, h) = (6, 6);
        let mut c = Canvas::new(w * scale, h * scale);
        c.scale(scale as f32, scale as f32, 0.0, 0.0);
        p.strokeWidth = if light { 1.0 } else { 2.0 };
        p.flags |= 4;
        p.style = 1;
        p.strokeCap = 2;
        if !up {
            c.drawLine(0.0, 0.0, w as f32, h as f32, p);
            c.drawLine(-2.0, (h - 2) as f32, 2.0, (h + 2) as f32, p);
            c.drawLine((w - 2) as f32, -2.0, (w + 2) as f32, 2.0, p);
        } else {
            c.drawLine(0.0, h as f32, w as f32, 0.0, p);
            c.drawLine(-2.0, 2.0, 2.0, -2.0, p);
            c.drawLine(
                (w - 2) as f32,
                (h + 2) as f32,
                (w + 2) as f32,
                (h - 2) as f32,
                p,
            );
        }
        PatternBitmap {
            width: c.width,
            height: c.height,
            commands: c.commands,
        }
    }
    // java private static Bitmap fillDarkUp(Paint paint, int scale) {
    // java return fillDialImpl(paint, scale, true, false);
    // java }
    fn fillDarkUp(p: &mut Paint, s: i32) -> PatternBitmap {
        Self::fillDialImpl(p, s, true, false)
    }
    // java private static Bitmap fillDarkDown(Paint paint, int scale) {
    // java return fillDialImpl(paint, scale, false, false);
    // java }
    fn fillDarkDown(p: &mut Paint, s: i32) -> PatternBitmap {
        Self::fillDialImpl(p, s, false, false)
    }
    // java private static Bitmap fillLightUp(Paint paint, int scale) {
    // java return fillDialImpl(paint, scale, true, true);
    // java }
    fn fillLightUp(p: &mut Paint, s: i32) -> PatternBitmap {
        Self::fillDialImpl(p, s, true, true)
    }
    // java private static Bitmap fillLightDown(Paint paint, int scale) {
    // java return fillDialImpl(paint, scale, false, true);
    // java }
    fn fillLightDown(p: &mut Paint, s: i32) -> PatternBitmap {
        Self::fillDialImpl(p, s, false, true)
    }
    // java private static Bitmap fillVerticalImpl(Paint paint, int scale, boolean light) {
    // java Bitmap bitmap = Bitmap.createBitmap(4 * scale, 4 * scale, Config.ARGB_4444);
    // java Canvas canvas = new Canvas(bitmap);
    // java canvas.scale((float)scale, (float)scale);
    // java if (!light) {
    // java canvas.drawRect(1.0F, 0.0F, 3.0F, 4.0F, paint);
    // java } else {
    // java canvas.drawRect(1.5F, 0.0F, 2.5F, 4.0F, paint);
    // java }
    // java
    // java return bitmap;
    // java }
    fn fillVerticalImpl(p: &mut Paint, scale: i32, light: bool) -> PatternBitmap {
        let mut c = Canvas::new(4 * scale, 4 * scale);
        c.scale(scale as f32, scale as f32, 0.0, 0.0);
        if !light {
            c.drawRect__float(1.0, 0.0, 3.0, 4.0, p);
        } else {
            c.drawRect__float(1.5, 0.0, 2.5, 4.0, p);
        }
        PatternBitmap {
            width: c.width,
            height: c.height,
            commands: c.commands,
        }
    }
    // java private static Bitmap fillHorizontalImpl(Paint paint, int scale, boolean light) {
    // java Bitmap bitmap = Bitmap.createBitmap(4 * scale, 4 * scale, Config.ARGB_4444);
    // java Canvas canvas = new Canvas(bitmap);
    // java canvas.scale((float)scale, (float)scale);
    // java if (!light) {
    // java canvas.drawRect(0.0F, 1.0F, 4.0F, 3.0F, paint);
    // java } else {
    // java canvas.drawRect(0.0F, 1.5F, 4.0F, 2.5F, paint);
    // java }
    // java
    // java return bitmap;
    // java }
    fn fillHorizontalImpl(p: &mut Paint, scale: i32, light: bool) -> PatternBitmap {
        let mut c = Canvas::new(4 * scale, 4 * scale);
        c.scale(scale as f32, scale as f32, 0.0, 0.0);
        if !light {
            c.drawRect__float(0.0, 1.0, 4.0, 3.0, p);
        } else {
            c.drawRect__float(0.0, 1.5, 4.0, 2.5, p);
        }
        PatternBitmap {
            width: c.width,
            height: c.height,
            commands: c.commands,
        }
    }
    // java private static Bitmap fillDarkVertical(Paint paint, int scale) {
    // java return fillVerticalImpl(paint, scale, false);
    // java }
    fn fillDarkVertical(p: &mut Paint, s: i32) -> PatternBitmap {
        Self::fillVerticalImpl(p, s, false)
    }
    // java private static Bitmap fillLightVertical(Paint paint, int scale) {
    // java return fillVerticalImpl(paint, scale, true);
    // java }
    fn fillLightVertical(p: &mut Paint, s: i32) -> PatternBitmap {
        Self::fillVerticalImpl(p, s, true)
    }
    // java private static Bitmap fillDarkHorizontal(Paint paint, int scale) {
    // java return fillHorizontalImpl(paint, scale, false);
    // java }
    fn fillDarkHorizontal(p: &mut Paint, s: i32) -> PatternBitmap {
        Self::fillHorizontalImpl(p, s, false)
    }
    // java private static Bitmap fillLightHorizontal(Paint paint, int scale) {
    // java return fillHorizontalImpl(paint, scale, true);
    // java }
    fn fillLightHorizontal(p: &mut Paint, s: i32) -> PatternBitmap {
        Self::fillHorizontalImpl(p, s, true)
    }
}
