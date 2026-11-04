#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

// Java: import android.graphics.Matrix
// Java: import android.graphics.Path
// Java: import android.graphics.Typeface
// Java: import android.graphics.Path.FillType
// Java: import cn.wps.base.assertion.Assert
// Java: import java.util.HashSet
// Java: import java.util.Set
// Java: import java.util.Stack
// Java: import java.util.Vector

use std::collections::HashSet;
use crate::cn::wps::graphics::gdi::GdiDC::{GdiDC, FillType};
use crate::cn::wps::graphics::gdi::GdiPath::GdiPath;
use crate::cn::wps::graphics::gdi::GdiObjects::GdiObjects;
use crate::cn::wps::graphics::gdi::GraphicsDrawer::GraphicsDrawer;
use crate::cn::wps::graphics::gdi::GdiBoundContext::GdiBoundContext;
use crate::cn::wps::graphics::gdi::Rectangle::Rectangle;
use crate::cn::wps::graphics::gdi::ColorPaint::ColorPaint;

pub(crate) const ALTERNATE: i32 = 1;
pub(crate) const WINDING: i32 = 2;

// Java: public class GdiPainter
pub struct GdiPainter {
    // Java field: GraphicsDrawer drawer
    pub(crate) drawer: Option<GraphicsDrawer>,
    // Java field: Stack<GdiDC> stackDC
    pub(crate) stackDC: Vec<GdiDC>,
    // Java field: GdiDC dc
    pub(crate) dc: GdiDC,
    // Java field: GdiObjects gdiObjects
    pub(crate) gdiObjects: Option<GdiObjects>,
    // Java field: GdiPath drawingPath
    pub(crate) drawingPath: Option<GdiPath>,
    // Java field: PathCache pathCache
    pub(crate) pathCache: PathCache,
    // Java field: Set<String> affectCharsetSet
    pub(crate) affectCharsetSet: Option<HashSet<String>>,
    // Java field: int ALTERNATE
    // Java field: int WINDING
}

// Java: public class PathCache
pub struct PathCache {
    // Java field: int currentPos
    pub(crate) currentPos: i32,
    // Java field: int size
    pub(crate) size: i32,
    // Java field: GdiPath[] paths
    pub(crate) paths: Vec<Option<GdiPath>>,
}

// Java: public class GdiPainter
impl GdiPainter {
    // Java: GdiPainter(int handles)
    pub fn new(handles: i32) -> Self {
        // Java: super();
        // Java: this.gdiObjects = new GdiObjects(handles);
        GdiPainter {
            drawer: None,
            stackDC: Vec::new(),
            dc: GdiDC::new(),
            gdiObjects: Some(GdiObjects::new(handles)),
            drawingPath: Some(GdiPath::new()),
            pathCache: PathCache::new(),
            affectCharsetSet: None,
        }
    }

    // Java: void resetEnv(boolean isEmf)
    pub fn resetEnv(&mut self, isEmf: bool) {
        // Java: this.stackDC.clear();
        self.stackDC.clear();
        // Java: this.dc.reset(isEmf);
        self.dc.reset(isEmf);
        // Java: this.gdiObjects.reset();
        if let Some(ref mut g) = self.gdiObjects { g.reset(); }
        // Java: if (null == this.drawingPath)
        if self.drawingPath.is_none() {
            // Java: this.drawingPath = new GdiPath();
            self.drawingPath = Some(GdiPath::new());
        }
        // Java: else
        else {
            // Java: this.drawingPath.reset();
            if let Some(ref mut dp) = self.drawingPath { dp.reset(); }
        }
    }

    // Java: GraphicsDrawer getDrawer()
    pub fn getDrawer(&self) -> &Option<GraphicsDrawer> {
        // Java: return this.drawer;
        &self.drawer
    }

    // Java: GdiDC getDC()
    pub fn getDC(&self) -> &GdiDC {
        // Java: return this.dc;
        &self.dc
    }

    // Java: GdiBoundContext getContext()
    pub fn getContext(&self) -> &Option<GdiBoundContext> {
        // Java: return this.dc.getContext();
        self.dc.getContext()
    }

    // Java: GdiObjects getGdiObjects()
    pub fn getGdiObjects(&self) -> &Option<GdiObjects> {
        // Java: return this.gdiObjects;
        &self.gdiObjects
    }

    // Java: Set<String> getAffectCharsetSet()
    pub fn getAffectCharsetSet(&mut self) -> &mut HashSet<String> {
        // Java: if (this.affectCharsetSet == null)
        if self.affectCharsetSet.is_none() {
            // Java: this.affectCharsetSet = new HashSet();
            self.affectCharsetSet = Some(HashSet::new());
        }
        // Java: return this.affectCharsetSet;
        self.affectCharsetSet.as_mut().unwrap()
    }

    // Java: void saveDC()
    pub fn saveDC(&mut self) {
        // Java: this.dc.setCanvasRect(this.drawer.canvas.getClipBounds());
        // Java: this.dc.setTextAlign(this.drawer.getTextAlign());
        if let Some(ref d) = self.drawer { self.dc.setTextAlign(d.getTextAlign()); }
        // Java: GdiDC saveDc = this.dc.clone();
        let saveDc = self.dc.clone_dc();
        // Java: this.stackDC.push(saveDc);
        self.stackDC.push(saveDc);
        // Java: if (this.drawer.states != null)
        {
            // Java: this.drawer.states.save();
        }
        // Java: this.drawer.canvas.save();
        if let Some(ref d) = self.drawer { d.canvas.save(); }
    }

    // Java: void restoreDC()
    pub fn restoreDC(&mut self) {
        // Java: if (!this.stackDC.empty())
        if !self.stackDC.is_empty() {
            // Java: GdiDC saveDc = (GdiDC)this.stackDC.pop();
            let mut saveDc = self.stackDC.pop().unwrap();
            // Java: this.dc.copy(saveDc);
            self.dc.copy(&saveDc);
            // Java: saveDc.dispose();
            saveDc.dispose();
            // Java: GdiDC var2 = null;
            // Java: this.drawer.canvas.restore();
            if let Some(ref d) = self.drawer { d.canvas.restore(); }
            // Java: if (this.drawer.states != null)
            {
                // Java: this.drawer.states.restore();
            }
            // Java: this.drawer.setTextAlign(this.dc.getTextAlign());
            let ta = self.dc.getTextAlign();
            if let Some(ref mut d) = self.drawer { d.setTextAlign(ta); }
        }
        // Java: else
        else {
            // Java: Assert.logFail("restoreDC_Stack is empty");
            eprintln!("restoreDC_Stack is empty");
        }
    }

    // Java: void closeFigure()
    pub fn closeFigure(&mut self) {
        // Java: this.appendFigure(true);
        self.appendFigure(true);
    }

    // Java: void appendFigure(boolean needClosePath)
    pub fn appendFigure(&mut self, needClosePath: bool) {
        // Java: if (this.drawingPath != null)
        if self.drawingPath.is_some() {
            // Java: if (needClosePath)
            if needClosePath {
                // Java: this.drawingPath.closePath();
                if let Some(ref mut dp) = self.drawingPath { dp.closePath(); }
            }
            // Java: this.dc.appendPath(this.drawingPath);
            // Java: this.pathCache.recyclePath(this.drawingPath);
            if let Some(dp) = self.drawingPath.take() {
                self.pathCache.recyclePath(dp);
            }
            // Java: this.drawingPath = null;
        }
    }

    // Java: void beginPath()
    pub fn beginPath(&mut self) {
        // Java: this.dc.resetPath(this.dc.pathFillType);
        let ft = self.dc.pathFillType.unwrap_or(FillType::WINDING);
        self.dc.resetPath(ft);
        // Java: this.dc.getContext().resetWorldTransform();
    }

    // Java: void endPath()
    pub fn endPath(&mut self) {
        // Java: if (this.dc.path != null)
        if self.dc.path.is_some() {
            // Java: this.appendFigure(false);
            self.appendFigure(false);
        }
    }

    // Java: void drawPath(GdiPath path)
    pub fn drawPath_gdi(&mut self, path: &mut GdiPath) {
        // Java: if (path != null)
        {
            // Java: this.drawer.drawPath(path);
            if let Some(ref mut d) = self.drawer { d.drawPath_gdi(path); }
        }
    }

    // Java: void drawPath(Path path, float x, float y, float height)
    pub fn drawPath_native(&mut self, _path: i32, mut x: f32, mut y: f32, height: f32) {
        // Java: if (this.drawingPath != null)
        if let Some(ref dp) = self.drawingPath {
            // Java: x += this.drawingPath.getCurrentX();
            x += dp.currentX;
            // Java: y += this.drawingPath.getCurrentY();
            y += dp.currentY;
        }
        // Java: this.drawer.drawPath(path, x, y, height);
        if let Some(ref mut d) = self.drawer { d.drawPath_native(_path, x, y, height); }
    }

    // Java: void fillRect(Rectangle bounds)
    pub fn fillRect(&mut self, bounds: &Rectangle) {
        // Java: GdiPath dstPath = this.pathCache.getPath(this.dc.pathFillType);
        let mut dstPath = self.pathCache.getPath();
        // Java: dstPath.addRect(bounds);
        dstPath.addRect(bounds.clone());
        // Java: this.fillPath(dstPath);
        // Java: this.pathCache.recyclePath(dstPath);
        self.pathCache.recyclePath(dstPath);
    }

    // Java: void fillRects(Vector<Rectangle> bounds)
    pub fn fillRects(&mut self, bounds: &[Rectangle]) {
        // Java: GdiPath dstPath = this.pathCache.getPath(this.dc.pathFillType);
        let mut dstPath = self.pathCache.getPath();
        // Java: for(Rectangle rect : bounds)
        for rect in bounds {
            // Java: dstPath.addRect(rect);
            dstPath.addRect(rect.clone());
        }
        // Java: this.fillPath(dstPath);
        // Java: this.pathCache.recyclePath(dstPath);
        self.pathCache.recyclePath(dstPath);
    }

    // Java: void fillPath(GdiPath path)
    pub fn fillPath(&mut self, path: &mut GdiPath) {
        // Java: if (path != null)
        {
            // Java: this.drawer.fill(path);
            if let Some(ref mut d) = self.drawer { d.fill(path); }
        }
    }

    // Java: void addPath(GdiPath path, boolean needFill)
    pub fn addPath_with_fill(&mut self, path: &mut GdiPath, needFill: bool) {
        // Java: if (this.beginningPath())
        if self.beginningPath() {
            // Java: this.dc.appendPath(path);
        }
        // Java: else
        else {
            // Java: if (needFill)
            if needFill {
                // Java: this.drawer.fill(path);
                if let Some(ref mut d) = self.drawer { d.fill(path); }
            }
            // Java: this.drawer.drawPath(path);
            if let Some(ref mut d) = self.drawer { d.drawPath_gdi(path); }
        }
    }

    // Java: void addPath(GdiPath path)
    pub fn addPath(&mut self, path: &mut GdiPath) {
        // Java: this.addPath(path, true);
        self.addPath_with_fill(path, true);
    }

    // Java: void selectObject(int index)
    pub fn selectObject(&mut self, index: i32) {
        // Java: IGdiObject gdiObject = this.gdiObjects.getObject(index);
        // Java: if (gdiObject != null)
        {
            // Java: gdiObject.render(this);
        }
    }

    // Java: boolean beginningPath()
    pub fn beginningPath(&self) -> bool {
        // Java: return this.dc.path != null;
        self.dc.path.is_some()
    }

    // Java: void drawCurrentPath()
    pub fn drawCurrentPath(&mut self) {
        // Java: this.drawPath(this.dc.path);
    }

    // Java: void setPolyFillMode(int mode)
    pub fn setPolyFillMode(&mut self, mode: i32) {
        // Java: Path.FillType fillType = FillType.WINDING;
        let mut fillType = FillType::WINDING;
        // Java: if (mode == 2)
        if mode == 2 {
            // Java: fillType = FillType.WINDING;
            fillType = FillType::WINDING;
        }
        // Java: else if (mode == 1)
        else if mode == 1 {
            // Java: fillType = FillType.EVEN_ODD;
            fillType = FillType::EVEN_ODD;
        }
        // Java: this.dc.setFillType(fillType);
        self.dc.setFillType(fillType);
    }

    // Java: void polyLine(int[] points, int count)
    pub fn polyLine(&mut self, points: &[i32], count: i32) {
        // Java: if (points != null && points.length != 0)
        if !points.is_empty() {
            // Java: GdiPath srcPath = this.pathCache.getPath(this.dc.pathFillType);
            let mut srcPath = self.pathCache.getPath();
            // Java: GdiTool.initPath(srcPath, points, count * 2);
            // Java: this.addPath(srcPath, false);
            self.addPath_with_fill(&mut srcPath, false);
            // Java: this.pathCache.recyclePath(srcPath);
            self.pathCache.recyclePath(srcPath);
        }
    }

    // Java: void polyLineTo(int[] points, int count)
    pub fn polyLineTo(&mut self, points: &[i32], count: i32) {
        // Java: if (points != null && points.length != 0)
        if !points.is_empty() {
            // Java: if (this.drawingPath == null)
            if self.drawingPath.is_none() {
                // Java: this.drawingPath = this.pathCache.getPath(this.dc.pathFillType);
                self.drawingPath = Some(self.pathCache.getPath());
            }
            // Java: int size = count * 2;
            let size = count as usize * 2;
            // Java: for(int i = 0; i < size; i += 2)
            let mut i = 0;
            while i < size {
                // Java: this.drawingPath.lineTo((float)points[i], (float)points[i + 1]);
                if let Some(ref mut dp) = self.drawingPath {
                    dp.lineTo(points[i] as f32, points[i + 1] as f32);
                }
                i += 2;
            }
            // Java: if (!this.beginningPath())
            if !self.beginningPath() {
                // Java: this.drawPath(this.drawingPath);
                // Java: this.drawingPath.reset();
                if let Some(ref mut dp) = self.drawingPath { dp.reset(); }
                // Java: this.drawingPath.moveTo((float)points[size - 2], (float)points[size - 1]);
                if let Some(ref mut dp) = self.drawingPath {
                    dp.moveTo(points[size - 2] as f32, points[size - 1] as f32);
                }
            }
        }
    }

    // Java: void polyGon(int[] points, int count)
    pub fn polyGon(&mut self, points: &[i32], count: i32) {
        // Java: if (points != null && points.length != 0)
        if !points.is_empty() {
            // Java: GdiPath path = this.pathCache.getPath(this.dc.pathFillType);
            let mut path = self.pathCache.getPath();
            // Java: GdiTool.initPath(path, points, count * 2);
            // Java: path.closePath();
            path.closePath();
            // Java: this.addPath(path, true);
            self.addPath_with_fill(&mut path, true);
            // Java: this.pathCache.recyclePath(path);
            self.pathCache.recyclePath(path);
        }
    }

    // Java: void polyPolyGon(int[] points, int[] counts, boolean closePath)
    pub fn polyPolyGon(&mut self, points: &[i32], counts: &[i32], closePath: bool) {
        // Java: if (points != null && points.length != 0 && counts != null && counts.length != 0)
        if !points.is_empty() && !counts.is_empty() {
            // Java: GdiPath path = this.pathCache.getPath(this.dc.pathFillType);
            let mut path = self.pathCache.getPath();
            // Java: int offset = 0;
            let mut offset = 0usize;
            // Java: for(int i = 0; i < counts.length; ++i)
            for i in 0..counts.len() {
                // Java: int size = counts[i] * 2;
                let size = counts[i] as usize * 2;
                // Java: GdiTool.initPath(path, points, offset, size);
                // Java: offset += size;
                offset += size;
                // Java: if (closePath)
                if closePath {
                    // Java: path.closePath();
                    path.closePath();
                }
            }
            // Java: this.addPath(path, closePath);
            self.addPath_with_fill(&mut path, closePath);
            // Java: this.pathCache.recyclePath(path);
            self.pathCache.recyclePath(path);
        }
    }

    // Java: void polyBezier(int[] points, int count)
    pub fn polyBezier(&mut self, points: &[i32], count: i32) {
        // Java: if (points != null && points.length > 0 && points.length >= count * 2)
        if !points.is_empty() && points.len() >= (count * 2) as usize {
            // Java: GdiPath path = this.pathCache.getPath(this.dc.pathFillType);
            let mut path = self.pathCache.getPath();
            // Java: path.moveTo((float)points[0], (float)points[1]);
            path.moveTo(points[0] as f32, points[1] as f32);
            // Java: for(int i = 2; i < count * 2; i += 6)
            let mut i = 2usize;
            let limit = count as usize * 2;
            while i < limit {
                // Java: path.curveTo((float)points[i], (float)points[i + 1], (float)points[i + 2], (float)points[i + 3], (float)points[i + 4], (float)points[i + 5]);
                path.curveTo(points[i] as f32, points[i+1] as f32, points[i+2] as f32, points[i+3] as f32, points[i+4] as f32, points[i+5] as f32);
                i += 6;
            }
            // Java: this.addPath(path);
            self.addPath(&mut path);
            // Java: this.pathCache.recyclePath(path);
            self.pathCache.recyclePath(path);
        }
    }

    // Java: void polyBezierTo(int[] points, int count)
    pub fn polyBezierTo(&mut self, points: &[i32], count: i32) {
        // Java: if (points != null && points.length > 0 && points.length >= count * 2)
        if !points.is_empty() && points.len() >= (count * 2) as usize {
            // Java: if (this.drawingPath == null)
            if self.drawingPath.is_none() {
                // Java: this.drawingPath = this.pathCache.getPath(this.dc.pathFillType);
                self.drawingPath = Some(self.pathCache.getPath());
            }
            // Java: int size = count * 2;
            let size = count as usize * 2;
            // Java: for(int i = 0; i < size; i += 6)
            let mut i = 0usize;
            while i < size {
                // Java: this.drawingPath.curveTo((float)points[i], (float)points[i + 1], (float)points[i + 2], (float)points[i + 3], (float)points[i + 4], (float)points[i + 5]);
                if let Some(ref mut dp) = self.drawingPath {
                    dp.curveTo(points[i] as f32, points[i+1] as f32, points[i+2] as f32, points[i+3] as f32, points[i+4] as f32, points[i+5] as f32);
                }
                i += 6;
            }
            // Java: if (!this.beginningPath())
            if !self.beginningPath() {
                // Java: this.drawPath(this.drawingPath);
                // Java: this.drawingPath.reset();
                if let Some(ref mut dp) = self.drawingPath { dp.reset(); }
                // Java: this.drawingPath.moveTo((float)points[size - 2], (float)points[size - 1]);
                if let Some(ref mut dp) = self.drawingPath {
                    dp.moveTo(points[size - 2] as f32, points[size - 1] as f32);
                }
            }
        }
    }

    // Java: void drawTextW(String text, int x, int y)
    pub fn drawTextW_simple(&mut self, text: &str, x: i32, y: i32) {
        // Java: this.drawTextW(text, x, y, (float[])null);
        self.drawTextW(text, x, y, None);
    }

    // Java: void drawTextW(String text, int x, int y, float[] widths, Typeface typeface)
    pub fn drawTextW_typeface(&mut self, text: &str, x: i32, y: i32, widths: Option<&[f32]>, _typeface: bool) {
        // Java: this.drawer.drawText(text, x, y, widths, this.drawingPath, false, typeface);
    }

    // Java: void drawTextW(String text, int x, int y, float[] widths)
    pub fn drawTextW(&mut self, text: &str, x: i32, y: i32, widths: Option<&[f32]>) {
        // Java: this.drawer.drawText(text, x, y, widths, this.drawingPath, false);
    }

    // Java: void drawTextA(String text, int x, int y, float[] widths)
    pub fn drawTextA(&mut self, text: &str, x: i32, y: i32, widths: Option<&[f32]>) {
        // Java: this.drawer.drawText(text, x, y, widths, this.drawingPath, true);
    }

    // Java: void moveTo(int x, int y)
    pub fn moveTo(&mut self, x: i32, y: i32) {
        // Java: if (this.drawingPath == null)
        if self.drawingPath.is_none() {
            // Java: this.drawingPath = this.pathCache.getPath(this.dc.pathFillType);
            self.drawingPath = Some(self.pathCache.getPath());
        }
        // Java: this.drawingPath.moveTo((float)x, (float)y);
        if let Some(ref mut dp) = self.drawingPath { dp.moveTo(x as f32, y as f32); }
    }

    // Java: void lineTo(int x, int y)
    pub fn lineTo(&mut self, x: i32, y: i32) {
        // Java: if (this.drawingPath == null)
        if self.drawingPath.is_none() {
            // Java: this.drawingPath = this.pathCache.getPath(this.dc.pathFillType);
            self.drawingPath = Some(self.pathCache.getPath());
        }
        // Java: this.drawingPath.lineTo((float)x, (float)y);
        if let Some(ref mut dp) = self.drawingPath { dp.lineTo(x as f32, y as f32); }
        // Java: if (!this.beginningPath())
        if !self.beginningPath() {
            // Java: this.drawPath(this.drawingPath);
            // Java: this.drawingPath.reset();
            if let Some(ref mut dp) = self.drawingPath { dp.reset(); }
            // Java: this.drawingPath.moveTo((float)x, (float)y);
            if let Some(ref mut dp) = self.drawingPath { dp.moveTo(x as f32, y as f32); }
        }
    }

    // Java: void clipPath(Rectangle bounds, int mode)
    pub fn clipPath_rect(&mut self, bounds: &Rectangle, mode: i32) {
        // Java: if (bounds != null)
        {
            // Java: this.getDC().setPath((GdiPath)null);
            // Java: this.getDrawer().setClip(bounds, mode);
        }
    }

    // Java: void clipPath(GdiPath path, int mode)
    pub fn clipPath_gdi(&mut self, _path: &GdiPath, mode: i32) {
        // Java: if (path != null)
        {
            // Java: this.getDC().setPath((GdiPath)null);
            // Java: this.getDrawer().setClip(path.getPath(), mode);
        }
    }

    // Java: void bitBlt(int x, int y, int width, int height, int rop, GdiImage image, Matrix transform)
    pub fn bitBlt(&mut self, x: i32, y: i32, width: i32, height: i32, rop: i32, _image: i32, _transform: Option<&[f32; 9]>) {
        // Java: Rectangle bounds = new Rectangle(x, y, width, height);
        let bounds = Rectangle::new(x, y, width, height);
        // Java: if (rop == 66)
        if rop == 66 {
            // Java: ColorPaint brush = ColorPaint.BLACK;
            let brush = ColorPaint::new(0xFF000000u32 as i32);
            // Java: this.dc.setBrushPaint(brush);
            self.dc.setBrushPaint(brush);
            // Java: this.fillRect(bounds);
            self.fillRect(&bounds);
        }
        // Java: else if (rop == 16711778)
        else if rop == 16711778 {
            // Java: ColorPaint brush = ColorPaint.WHITE;
            let brush = ColorPaint::new(0xFFFFFFFFu32 as i32);
            // Java: this.dc.setBrushPaint(brush);
            self.dc.setBrushPaint(brush);
            // Java: this.fillRect(bounds);
            self.fillRect(&bounds);
        }
        // Java: else if (rop == 15728673)
        else if rop == 15728673 {
            // Java: this.fillRect(bounds);
            self.fillRect(&bounds);
        }
        // Java: else if (transform == null)
        else if _transform.is_none() {
            // Java: this.drawer.drawImage(image, x, y, width, height, true);
            if let Some(ref mut d) = self.drawer { d.drawImage_transparent(_image, x, y, width, height, true); }
        }
        // Java: else
        else {
            // Java: this.drawer.drawImage(image, x, y, width, height, transform);
            if let Some(ref mut d) = self.drawer { d.drawImage_matrix(_image, x, y, width, height, _transform.unwrap()); }
        }
    }

    // Java: void destroy()
    pub fn destroy(&mut self) {
        // Java: this.drawer = null;
        self.drawer = None;
        // Java: this.dc.dispose();
        self.dc.dispose();
        // Java: this.dc = null;
        // Java: this.gdiObjects.dispose();
        if let Some(ref mut g) = self.gdiObjects { g.dispose(); }
        // Java: this.gdiObjects = null;
        self.gdiObjects = None;
        // Java: this.stackDC.clear();
        self.stackDC.clear();
        // Java: this.stackDC = null;
        // Java: this.pathCache.destroy();
        self.pathCache.destroy();
        // Java: this.pathCache = null;
    }
}

// Java: public class PathCache
impl PathCache {
    pub fn new() -> Self {
        PathCache { currentPos: 0, size: 10, paths: vec![None; 10] }
    }

    // Java: GdiPath getPath(Path.FillType fillType)
    pub fn getPath(&mut self) -> GdiPath {
        // Java: if (this.currentPos == 0)
        if self.currentPos == 0 {
            // Java: this.paths[this.currentPos] = new GdiPath(fillType);
            let p = GdiPath::new();
            // Java: return this.paths[this.currentPos];
            return p;
        }
        // Java: else
        else {
            // Java: --this.currentPos;
            self.currentPos -= 1;
            // Java: GdiPath path = this.paths[this.currentPos];
            let path = self.paths[self.currentPos as usize].take().unwrap_or_else(GdiPath::new);
            // Java: path.reset(fillType);
            // Java: return path;
            return path;
        }
    }

    // Java: void recyclePath(GdiPath path)
    pub fn recyclePath(&mut self, path: GdiPath) {
        // Java: if (this.currentPos == this.size)
        if self.currentPos == self.size {
            // Java: GdiPath[] newPaths = new GdiPath[this.size + 10];
            // Java: System.arraycopy(this.paths, 0, newPaths, 0, this.size);
            self.paths.resize_with((self.size + 10) as usize, || None);
            // Java: this.paths = newPaths;
            // Java: this.size += 10;
            self.size += 10;
        }
        // Java: this.paths[this.currentPos] = path;
        self.paths[self.currentPos as usize] = Some(path);
        // Java: ++this.currentPos;
        self.currentPos += 1;
    }

    // Java: void destroy()
    pub fn destroy(&mut self) {
        // Java: this.currentPos = 0;
        self.currentPos = 0;
        // Java: this.paths = null;
        self.paths.clear();
    }
}
