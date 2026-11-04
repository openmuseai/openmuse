#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import android.graphics.Canvas
// Java: import android.graphics.Matrix
// Java: import android.graphics.Paint
// Java: import android.graphics.Path
// Java: import android.graphics.RectF
// Java: import android.graphics.Path.Direction
// Java: import android.graphics.Path.FillType

use std::sync::Mutex;
use crate::cn::wps::graphics::gdi::Rectangle::Rectangle;
use crate::cn::wps::graphics::gdi::GraphicsPath::GraphicsPath;

#[derive(Clone)]
pub struct Matrix {
    values: [f32; 9],
}
impl Matrix {
    pub fn new() -> Self { Matrix { values: [1.0,0.0,0.0, 0.0,1.0,0.0, 0.0,0.0,1.0] } }
    pub fn from_matrix(m: &Matrix) -> Self { m.clone() }
    pub fn getValues(&self, out: &mut [f32; 9]) { *out = self.values; }
    pub fn preConcat(&mut self, _other: &Matrix) {}
}

pub struct Canvas;
impl Canvas {
    pub fn drawPath(&self, _path: &Path, _paint: &Paint) {}
}

pub struct Paint;

#[derive(Clone)]
pub struct RectF { pub left: f32, pub top: f32, pub right: f32, pub bottom: f32 }
impl RectF {
    pub fn new() -> Self { RectF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 } }
    pub fn set_from_rectf(&mut self, other: &RectF) { *self = other.clone(); }
    pub fn width(&self) -> f32 { self.right - self.left }
    pub fn height(&self) -> f32 { self.bottom - self.top }
}

#[derive(Clone, Copy, PartialEq)]
pub enum FillType { WINDING, EVEN_ODD }

#[derive(Clone, Copy)]
pub enum Direction { CW, CCW }

#[derive(Clone)]
pub struct Path {
    _data: Vec<u8>,
    fill_type: FillType,
}
impl Path {
    pub fn new() -> Self { Path { _data: Vec::new(), fill_type: FillType::WINDING } }
    pub fn from_path(p: &Path) -> Self { p.clone() }
    pub fn reset(&mut self) { self._data.clear(); }
    pub fn setFillType(&mut self, ft: FillType) { self.fill_type = ft; }
    pub fn moveTo(&mut self, _x: f32, _y: f32) {}
    pub fn lineTo(&mut self, _x: f32, _y: f32) {}
    pub fn cubicTo(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32, _x3: f32, _y3: f32) {}
    pub fn arcTo(&mut self, _rect: &RectF, _start: f32, _sweep: f32) {}
    pub fn addOval(&mut self, _rect: &RectF, _dir: Direction) {}
    pub fn addRect(&mut self, _rect: &RectF, _dir: Direction) {}
    pub fn addRoundRect(&mut self, _rect: &RectF, _rx: f32, _ry: f32, _dir: Direction) {}
    pub fn close(&mut self) {}
    pub fn computeBounds(&self, rect: &mut RectF, _exact: bool) { *rect = RectF::new(); }
}

pub struct PathElem {
    pub elemType: i32,
    pub pos: Vec<f32>,
    pub rect: Option<Rectangle>,
}
impl PathElem {
    pub fn createMoveElem(x: f32, y: f32) -> Self { PathElem { elemType: 0, pos: vec![x, y], rect: None } }
    pub fn createLineElem(x: f32, y: f32) -> Self { PathElem { elemType: 1, pos: vec![x, y], rect: None } }
    pub fn createArcElem(r: Rectangle, start: f32, sweep: f32) -> Self { PathElem { elemType: 2, pos: vec![start, sweep], rect: Some(r) } }
    pub fn createOvalElem(r: Rectangle) -> Self { PathElem { elemType: 3, pos: Vec::new(), rect: Some(r) } }
    pub fn createCubicElem(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> Self { PathElem { elemType: 4, pos: vec![x1,y1,x2,y2,x3,y3], rect: None } }
    pub fn createCloseElem() -> Self { PathElem { elemType: 5, pos: Vec::new(), rect: None } }
    pub fn createAddRectElem(r: Rectangle) -> Self { PathElem { elemType: 6, pos: Vec::new(), rect: Some(r) } }
    pub fn createAddRoundRectElem(r: Rectangle, rx: f32, ry: f32) -> Self { PathElem { elemType: 7, pos: vec![rx, ry], rect: Some(r) } }
}

// Java field: RectF tempRect
static TEMP_RECT: Mutex<RectF> = Mutex::new(RectF { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 });

// Java: public class GdiPath implements Cloneable
#[derive(Clone)]
pub struct GdiPath {
    // Java field: Path path
    pub(crate) path: Path,
    // Java field: GraphicsPath graphicsPath
    graphicsPath: GraphicsPath,
    // Java field: Matrix transform
    transform: Option<Matrix>,
    // Java field: float currentX
    pub(crate) currentX: f32,
    // Java field: float currentY
    pub(crate) currentY: f32,
}

// Java: public class GdiPath implements Cloneable
impl GdiPath {
    // Java: GdiPath(Path.FillType mode)
    pub fn new_with_mode(mode: FillType) -> Self {
        // Java: super();
        // Java: this.transform = null;
        // Java: this.currentX = 0.0F;
        // Java: this.currentY = 0.0F;
        // Java: this.path = new Path();
        let mut path = Path::new();
        // Java: this.graphicsPath = new GraphicsPath();
        let graphicsPath = GraphicsPath::new();
        // Java: this.transform = null;
        // Java: this.path.setFillType(mode);
        path.setFillType(mode);
        GdiPath { path, graphicsPath, transform: None, currentX: 0.0, currentY: 0.0 }
    }

    // Java: GdiPath(Path path)
    pub fn new_with_path(path: Path) -> Self {
        // Java: super();
        // Java: this.transform = null;
        // Java: this.currentX = 0.0F;
        // Java: this.currentY = 0.0F;
        // Java: this.path = path;
        GdiPath { path, graphicsPath: GraphicsPath::new(), transform: None, currentX: 0.0, currentY: 0.0 }
    }

    // Java: GdiPath()
    pub fn new() -> Self {
        // Java: this(FillType.WINDING);
        GdiPath::new_with_mode(FillType::WINDING)
    }

    // Java: void reset(Path.FillType fillType)
    pub fn reset_with_fill(&mut self, fillType: FillType) {
        // Java: this.reset();
        self.reset();
        // Java: this.path.setFillType(fillType);
        self.path.setFillType(fillType);
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.transform = null;
        self.transform = None;
        // Java: this.path.reset();
        self.path.reset();
        // Java: this.graphicsPath.reset();
        self.graphicsPath.reset();
        // Java: this.currentX = 0.0F;
        self.currentX = 0.0;
        // Java: this.currentY = 0.0F;
        self.currentY = 0.0;
    }

    // Java: void moveTo(float x, float y)
    pub fn moveTo(&mut self, x: f32, y: f32) {
        // Java: this.currentX = x;
        self.currentX = x;
        // Java: this.currentY = y;
        self.currentY = y;
        // Java: this.graphicsPath.addElem(PathElem.createMoveElem(x, y));
        self.graphicsPath.addElem(crate::cn::wps::graphics::gdi::GraphicsPath::PathElem { kind: 0, points: vec![x, y] });
    }

    // Java: void curveTo(float x1, float y1, float x2, float y2, float x3, float y3)
    pub fn curveTo(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        // Java: this.graphicsPath.addElem(PathElem.createCubicElem(x1, y1, x2, y2, x3, y3));
        self.graphicsPath.addElem(crate::cn::wps::graphics::gdi::GraphicsPath::PathElem { kind: 4, points: vec![x1,y1,x2,y2,x3,y3] });
    }

    // Java: void lineTo(float x, float y)
    pub fn lineTo(&mut self, x: f32, y: f32) {
        // Java: this.graphicsPath.addElem(PathElem.createLineElem(x, y));
        self.graphicsPath.addElem(crate::cn::wps::graphics::gdi::GraphicsPath::PathElem { kind: 1, points: vec![x, y] });
    }

    // Java: void arcTo(Rectangle oval, float startAngle, float sweepAngle)
    pub fn arcTo_rect(&mut self, oval: Rectangle, startAngle: f32, sweepAngle: f32) {
        // Java: this.graphicsPath.addElem(PathElem.createArcElem(oval, startAngle, sweepAngle));
        self.graphicsPath.addElem(crate::cn::wps::graphics::gdi::GraphicsPath::PathElem { kind: 2, points: vec![startAngle, sweepAngle] });
    }

    // Java: void arcTo(Rectangle source)
    pub fn arcTo_oval(&mut self, source: Rectangle) {
        // Java: this.graphicsPath.addElem(PathElem.createOvalElem(source));
        self.graphicsPath.addElem(crate::cn::wps::graphics::gdi::GraphicsPath::PathElem { kind: 3, points: Vec::new() });
    }

    // Java: void addRect(Rectangle source)
    pub fn addRect(&mut self, source: Rectangle) {
        // Java: this.graphicsPath.addElem(PathElem.createAddRectElem(source));
        self.graphicsPath.addElem(crate::cn::wps::graphics::gdi::GraphicsPath::PathElem { kind: 6, points: Vec::new() });
    }

    // Java: void addRoundRect(Rectangle source, float rx, float ry)
    pub fn addRoundRect(&mut self, source: Rectangle, rx: f32, ry: f32) {
        // Java: this.graphicsPath.addElem(PathElem.createAddRoundRectElem(source, rx, ry));
        self.graphicsPath.addElem(crate::cn::wps::graphics::gdi::GraphicsPath::PathElem { kind: 7, points: vec![rx, ry] });
    }

    // Java: void closePath()
    pub fn closePath(&mut self) {
        // Java: this.graphicsPath.addElem(PathElem.createCloseElem());
        self.graphicsPath.addElem(crate::cn::wps::graphics::gdi::GraphicsPath::PathElem { kind: 5, points: Vec::new() });
    }

    // Java: Path getPath()
    pub fn getPath_noarg(&mut self) -> &Path {
        // Java: if (this.transform == null)
        if self.transform.is_none() {
            // Java: this.createPath();
            self.createPath();
        }
        // Java: else
        else {
            // Java: this.createPathByScale(this.transform);
            let t = self.transform.clone().unwrap();
            self.createPathByScale(&t);
        }
        // Java: return this.path;
        &self.path
    }

    // Java: Path getPath(Matrix transform)
    pub fn getPath_with_transform(&mut self, transform: Option<&Matrix>) -> &Path {
        // Java: if (transform == null)
        if transform.is_none() {
            // Java: this.createPath();
            self.createPath();
        }
        // Java: else
        else {
            // Java: this.createPathByScale(transform);
            self.createPathByScale(transform.unwrap());
        }
        // Java: return this.path;
        &self.path
    }

    // Java: void draw(Canvas canvas, Paint paint)
    pub fn draw(&mut self, canvas: &Canvas, paint: &Paint) {
        // Java: canvas.drawPath(this.path, paint);
        canvas.drawPath(&self.path, paint);
    }

    // Java: Rectangle getBounds()
    pub fn getBounds(&self) -> Rectangle {
        // Java: this.path.computeBounds(tempRect, false);
        let mut temp = TEMP_RECT.lock().unwrap();
        self.path.computeBounds(&mut temp, false);
        // Java: int x = (int)tempRect.left;
        let x = temp.left as i32;
        // Java: int y = (int)tempRect.top;
        let y = temp.top as i32;
        // Java: int width = (int)tempRect.width();
        let width = temp.width() as i32;
        // Java: int height = (int)tempRect.height();
        let height = temp.height() as i32;
        // Java: return new Rectangle(x, y, width, height);
        Rectangle::new(x, y, width, height)
    }

    // Java: GdiPath clone()
    pub fn clone_path(&self) -> GdiPath {
        // Java: GdiPath newGp = new GdiPath();
        let mut newGp = GdiPath::new();
        // Java: newGp.path = new Path(this.path);
        newGp.path = Path::from_path(&self.path);
        // Java: return newGp;
        newGp
    }

    // Java: void append(GdiPath srcPath, boolean connect)
    pub fn append_connect(&mut self, srcPath: GdiPath, connect: bool) {
        // Java: this.graphicsPath.addPath(srcPath.graphicsPath);
        self.graphicsPath.addPath(srcPath.graphicsPath);
    }

    // Java: void append(GdiPath srcPath, Matrix transform)
    pub fn append_transform(&mut self, srcPath: GdiPath, transform: Matrix) {
        // Java: if (null != this.transform)
        if let Some(ref mut t) = self.transform {
            // Java: this.transform.preConcat(transform);
            t.preConcat(&transform);
        }
        // Java: else
        else {
            // Java: this.transform = new Matrix(transform);
            self.transform = Some(Matrix::from_matrix(&transform));
        }
        // Java: this.graphicsPath.addPath(srcPath.graphicsPath);
        self.graphicsPath.addPath(srcPath.graphicsPath);
    }

    // Java: Path createPathByScale(Matrix matrix)
    pub fn createPathByScale(&mut self, matrix: &Matrix) -> &Path {
        // Java: this.path.reset();
        self.path.reset();
        // Java: float[] matrixFloatValues = new float[9];
        let mut matrixFloatValues = [0.0f32; 9];
        // Java: matrix.getValues(matrixFloatValues);
        matrix.getValues(&mut matrixFloatValues);
        // Java: float scaleX = matrixFloatValues[0];
        let scaleX = matrixFloatValues[0];
        // Java: float skewX = matrixFloatValues[1];
        let skewX = matrixFloatValues[1];
        // Java: float transX = matrixFloatValues[2];
        let transX = matrixFloatValues[2];
        // Java: float skewY = matrixFloatValues[3];
        let skewY = matrixFloatValues[3];
        // Java: float scaleY = matrixFloatValues[4];
        let scaleY_val = matrixFloatValues[4];
        // Java: float transY = matrixFloatValues[5];
        let transY = matrixFloatValues[5];
        // Java: Rectangle rect = null;
        let mut rect: Option<Rectangle> = None;
        // Java: for(PathElem elem : this.graphicsPath.elems)
        let elems_snapshot: Vec<(i32, Vec<f32>)> = self.graphicsPath.elems.iter().map(|e| (e.kind, e.points.clone())).collect();
        for (elemType, pos) in &elems_snapshot {
            // Java: switch (elem.elemType)
            match elemType {
                // Java: case -1:
                // Java: default:
                -1 | _ if *elemType < 0 => {
                    // Java: break;
                }
                // Java: case 0:
                0 => {
                    // Java: this.currentX = (float)Math.round(elem.pos[0] * scaleX + elem.pos[1] * skewX + transX);
                    self.currentX = (pos[0] * scaleX + pos[1] * skewX + transX).round();
                    // Java: this.currentY = (float)Math.round(elem.pos[0] * skewY + elem.pos[1] * scaleY + transY);
                    self.currentY = (pos[0] * skewY + pos[1] * scaleY_val + transY).round();
                    // Java: this.path.moveTo(this.currentX, this.currentY);
                    self.path.moveTo(self.currentX, self.currentY);
                    // Java: break;
                }
                // Java: case 1:
                1 => {
                    // Java: this.path.lineTo((float)Math.round(elem.pos[0] * scaleX + elem.pos[1] * skewX + transX), (float)Math.round(elem.pos[0] * skewY + elem.pos[1] * scaleY + transY));
                    self.path.lineTo(
                        (pos[0] * scaleX + pos[1] * skewX + transX).round(),
                        (pos[0] * skewY + pos[1] * scaleY_val + transY).round(),
                    );
                    // Java: break;
                }
                // Java: case 2:
                2 => {
                    // Java: rect = elem.rect.preConcat(matrix);
                    // Java: tempRect.set(rect.getRect());
                    let mut temp = TEMP_RECT.lock().unwrap();
                    *temp = RectF::new();
                    // Java: this.path.arcTo(tempRect, elem.pos[0], elem.pos[1]);
                    self.path.arcTo(&temp, pos[0], pos[1]);
                    // Java: break;
                }
                // Java: case 3:
                3 => {
                    // Java: rect = elem.rect.preConcat(matrix);
                    // Java: tempRect.set(rect.getRect());
                    let temp = TEMP_RECT.lock().unwrap();
                    // Java: this.path.addOval(tempRect, Direction.CW);
                    self.path.addOval(&temp, Direction::CW);
                    // Java: break;
                }
                // Java: case 4:
                4 => {
                    // Java: this.path.cubicTo((float)Math.round(elem.pos[0] * scaleX + elem.pos[1] * skewX + transX), (float)Math.round(elem.pos[0] * skewY + elem.pos[1] * scaleY + transY), (float)Math.round(elem.pos[2] * scaleX + elem.pos[3] * skewX + transX), (float)Math.round(elem.pos[2] * skewY + elem.pos[3] * scaleY + transY), (float)Math.round(elem.pos[4] * scaleX + elem.pos[5] * skewX + transX), (float)Math.round(elem.pos[4] * skewY + elem.pos[5] * scaleY + transY));
                    self.path.cubicTo(
                        (pos[0] * scaleX + pos[1] * skewX + transX).round(),
                        (pos[0] * skewY + pos[1] * scaleY_val + transY).round(),
                        (pos[2] * scaleX + pos[3] * skewX + transX).round(),
                        (pos[2] * skewY + pos[3] * scaleY_val + transY).round(),
                        (pos[4] * scaleX + pos[5] * skewX + transX).round(),
                        (pos[4] * skewY + pos[5] * scaleY_val + transY).round(),
                    );
                    // Java: break;
                }
                // Java: case 5:
                5 => {
                    // Java: this.path.close();
                    self.path.close();
                    // Java: break;
                }
                // Java: case 6:
                6 => {
                    // Java: rect = elem.rect.preConcat(matrix);
                    // Java: tempRect.set(rect.getRect());
                    let temp = TEMP_RECT.lock().unwrap();
                    // Java: this.path.addRect(tempRect, Direction.CW);
                    self.path.addRect(&temp, Direction::CW);
                    // Java: break;
                }
                // Java: case 7:
                7 => {
                    // Java: rect = elem.rect.preConcat(matrix);
                    // Java: tempRect.set(rect.getRect());
                    let temp = TEMP_RECT.lock().unwrap();
                    // Java: this.path.addRoundRect(tempRect, elem.pos[0] * scaleX, elem.pos[1] * scaleY, Direction.CW);
                    self.path.addRoundRect(&temp, pos[0] * scaleX, pos[1] * scaleY_val, Direction::CW);
                }
                _ => {}
            }
        }
        // Java: return this.path;
        &self.path
    }

    // Java: void createPath()
    pub fn createPath(&mut self) {
        // Java: this.path.reset();
        self.path.reset();
        // Java: for(PathElem elem : this.graphicsPath.elems)
        let elems_snapshot: Vec<(i32, Vec<f32>)> = self.graphicsPath.elems.iter().map(|e| (e.kind, e.points.clone())).collect();
        for (elemType, pos) in &elems_snapshot {
            // Java: switch (elem.elemType)
            match elemType {
                // Java: case -1:
                // Java: default:
                -1 | _ if *elemType < 0 => {
                    // Java: break;
                }
                // Java: case 0:
                0 => {
                    // Java: this.currentX = elem.pos[0];
                    self.currentX = pos[0];
                    // Java: this.currentY = elem.pos[1];
                    self.currentY = pos[1];
                    // Java: this.path.moveTo(elem.pos[0], elem.pos[1]);
                    self.path.moveTo(pos[0], pos[1]);
                    // Java: break;
                }
                // Java: case 1:
                1 => {
                    // Java: this.path.lineTo(elem.pos[0], elem.pos[1]);
                    self.path.lineTo(pos[0], pos[1]);
                    // Java: break;
                }
                // Java: case 2:
                2 => {
                    // Java: tempRect.set(elem.rect.getRect());
                    let temp = TEMP_RECT.lock().unwrap();
                    // Java: this.path.arcTo(tempRect, elem.pos[0], elem.pos[1]);
                    self.path.arcTo(&temp, pos[0], pos[1]);
                    // Java: break;
                }
                // Java: case 3:
                3 => {
                    // Java: tempRect.set(elem.rect.getRect());
                    let temp = TEMP_RECT.lock().unwrap();
                    // Java: this.path.addOval(tempRect, Direction.CW);
                    self.path.addOval(&temp, Direction::CW);
                    // Java: break;
                }
                // Java: case 4:
                4 => {
                    // Java: this.path.cubicTo(elem.pos[0], elem.pos[1], elem.pos[2], elem.pos[3], elem.pos[4], elem.pos[5]);
                    self.path.cubicTo(pos[0], pos[1], pos[2], pos[3], pos[4], pos[5]);
                    // Java: break;
                }
                // Java: case 5:
                5 => {
                    // Java: this.path.close();
                    self.path.close();
                    // Java: break;
                }
                // Java: case 6:
                6 => {
                    // Java: tempRect.set(elem.rect.getRect());
                    let temp = TEMP_RECT.lock().unwrap();
                    // Java: this.path.addRect(tempRect, Direction.CW);
                    self.path.addRect(&temp, Direction::CW);
                    // Java: break;
                }
                // Java: case 7:
                7 => {
                    // Java: tempRect.set(elem.rect.getRect());
                    let temp = TEMP_RECT.lock().unwrap();
                    // Java: this.path.addRoundRect(tempRect, elem.pos[0], elem.pos[1], Direction.CW);
                    self.path.addRoundRect(&temp, pos[0], pos[1], Direction::CW);
                }
                _ => {}
            }
        }
    }

    // Java: float getCurrentX()
    pub fn getCurrentX(&self) -> f32 {
        // Java: return this.currentX;
        self.currentX
    }

    // Java: float getCurrentY()
    pub fn getCurrentY(&self) -> f32 {
        // Java: return this.currentY;
        self.currentY
    }

    // Java: void setCurrentX(float currentX)
    pub fn setCurrentX(&mut self, currentX: f32) {
        // Java: this.currentX = currentX;
        self.currentX = currentX;
    }

    // Java: void setCurrentY(float currentY)
    pub fn setCurrentY(&mut self, currentY: f32) {
        // Java: this.currentY = currentY;
        self.currentY = currentY;
    }
}
