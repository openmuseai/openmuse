#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import android.graphics.Matrix
// Java: import android.graphics.Path
// Java: import android.graphics.Rect
// Java: import android.graphics.Path.FillType

use crate::cn::wps::graphics::gdi::GdiStroke::GdiStroke;
use crate::cn::wps::graphics::gdi::IGdiPainter::IGdiPainter;
use crate::cn::wps::graphics::gdi::GDIFont::GDIFont;
use crate::cn::wps::graphics::gdi::GdiPath::GdiPath;
use crate::cn::wps::graphics::gdi::GdiBoundContext::GdiBoundContext;
use crate::cn::wps::graphics::gdi::ColorPaint::ColorPaint;

#[derive(Clone, Copy, PartialEq)]
pub enum FillType { WINDING, EVEN_ODD }

#[derive(Clone)]
pub struct Rect { pub left: i32, pub top: i32, pub right: i32, pub bottom: i32 }

#[derive(Clone)]
pub struct Matrix { pub values: [f32; 9] }
impl Matrix {
    pub fn new() -> Self { Matrix { values: [1.0,0.0,0.0, 0.0,1.0,0.0, 0.0,0.0,1.0] } }
}

// Java: public class GdiDC implements Cloneable
pub struct GdiDC {
    // Java field: GdiStroke stroke
    pub(crate) stroke: Option<GdiStroke>,
    // Java field: IGdiPainter linePaint
    pub(crate) linePaint: Option<ColorPaint>,
    // Java field: IGdiPainter brushPaint
    pub(crate) brushPaint: Option<ColorPaint>,
    // Java field: Path.FillType pathFillType
    pub(crate) pathFillType: Option<FillType>,
    // Java field: GDIFont font
    pub(crate) font: Option<GDIFont>,
    // Java field: GdiPath path
    pub(crate) path: Option<GdiPath>,
    // Java field: Rect canvasRect
    pub(crate) canvasRect: Option<Rect>,
    // Java field: int meterLimit
    pub(crate) meterLimit: i32,
    // Java field: int textAlign
    pub(crate) textAlign: i32,
    // Java field: GdiBoundContext context
    pub(crate) context: Option<GdiBoundContext>,
}

// Java: public class GdiDC implements Cloneable
impl GdiDC {
    // Java: GdiDC()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.pathFillType = FillType.WINDING;
        // Java: this.meterLimit = 10;
        // Java: this.textAlign = 0;
        // Java: this.context = new GdiBoundContext();
        GdiDC {
            stroke: None,
            linePaint: None,
            brushPaint: None,
            pathFillType: Some(FillType::WINDING),
            font: None,
            path: None,
            canvasRect: None,
            meterLimit: 10,
            textAlign: 0,
            context: Some(GdiBoundContext::new()),
        }
    }

    // Java: void reset(boolean isEmf)
    pub fn reset(&mut self, isEmf: bool) {
        // Java: this.stroke = new GdiStroke();
        self.stroke = Some(GdiStroke::new(0.0, 0, 0, 0.0, None, 0.0));
        // Java: this.linePaint = ColorPaint.BLACK;
        self.linePaint = Some(ColorPaint::new(0xFF000000u32 as i32));
        // Java: this.brushPaint = ColorPaint.WHITE;
        self.brushPaint = Some(ColorPaint::new(0xFFFFFFFFu32 as i32));
        // Java: this.pathFillType = FillType.WINDING;
        self.pathFillType = Some(FillType::WINDING);
        // Java: this.font = null;
        self.font = None;
        // Java: this.path = null;
        self.path = None;
        // Java: this.canvasRect = null;
        self.canvasRect = None;
        // Java: this.meterLimit = 10;
        self.meterLimit = 10;
        // Java: this.textAlign = 0;
        self.textAlign = 0;
        // Java: this.context.reset(isEmf);
        if let Some(ref mut ctx) = self.context {
            ctx.reset(isEmf);
        }
    }

    // Java: void copy(GdiDC dc)
    pub fn copy(&mut self, dc: &GdiDC) {
        // Java: this.linePaint = dc.linePaint;
        self.linePaint = dc.linePaint.clone();
        // Java: this.stroke = dc.stroke;
        self.stroke = dc.stroke.clone();
        // Java: this.brushPaint = dc.brushPaint;
        self.brushPaint = dc.brushPaint.clone();
        // Java: this.canvasRect = dc.canvasRect;
        self.canvasRect = dc.canvasRect.clone();
        // Java: this.pathFillType = dc.pathFillType;
        self.pathFillType = dc.pathFillType;
        // Java: this.meterLimit = dc.meterLimit;
        self.meterLimit = dc.meterLimit;
        // Java: this.font = dc.font;
        self.font = dc.font.clone();
        // Java: this.context = dc.context;
        self.context = dc.context.clone();
        // Java: this.path = dc.path;
        self.path = dc.path.clone();
        // Java: this.textAlign = dc.textAlign;
        self.textAlign = dc.textAlign;
    }

    // Java: GdiDC clone()
    pub fn clone_dc(&self) -> GdiDC {
        // Java: GdiDC dc = new GdiDC();
        let mut dc = GdiDC::new();
        // Java: dc.linePaint = this.linePaint;
        dc.linePaint = self.linePaint.clone();
        // Java: dc.stroke = this.stroke;
        dc.stroke = self.stroke.clone();
        // Java: dc.brushPaint = this.brushPaint;
        dc.brushPaint = self.brushPaint.clone();
        // Java: dc.canvasRect = this.canvasRect;
        dc.canvasRect = self.canvasRect.clone();
        // Java: dc.pathFillType = this.pathFillType;
        dc.pathFillType = self.pathFillType;
        // Java: dc.meterLimit = this.meterLimit;
        dc.meterLimit = self.meterLimit;
        // Java: dc.textAlign = this.textAlign;
        dc.textAlign = self.textAlign;
        // Java: dc.font = this.font;
        dc.font = self.font.clone();
        // Java: dc.context = this.context.clone();
        dc.context = self.context.clone();
        // Java: if (this.path != null)
        if self.path.is_some() {
            // Java: dc.path = this.path.clone();
            dc.path = self.path.clone();
        }
        // Java: return dc;
        dc
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.context = null;
        self.context = None;
        // Java: this.linePaint = null;
        self.linePaint = None;
        // Java: this.stroke = null;
        self.stroke = None;
        // Java: this.brushPaint = null;
        self.brushPaint = None;
        // Java: this.canvasRect = null;
        self.canvasRect = None;
        // Java: this.pathFillType = null;
        self.pathFillType = None;
        // Java: this.path = null;
        self.path = None;
        // Java: this.font = null;
        self.font = None;
    }

    // Java: void setStroke(GdiStroke stroke)
    pub fn setStroke(&mut self, stroke: GdiStroke) {
        // Java: this.stroke = stroke;
        self.stroke = Some(stroke);
    }

    // Java: void setLinePaint(IGdiPainter linePaint)
    pub fn setLinePaint(&mut self, linePaint: ColorPaint) {
        // Java: this.linePaint = linePaint;
        self.linePaint = Some(linePaint);
    }

    // Java: void setBrushPaint(IGdiPainter brushPaint)
    pub fn setBrushPaint(&mut self, brushPaint: ColorPaint) {
        // Java: this.brushPaint = brushPaint;
        self.brushPaint = Some(brushPaint);
    }

    // Java: void setCanvasRect(Rect canvasRect)
    pub fn setCanvasRect(&mut self, canvasRect: Rect) {
        // Java: this.canvasRect = canvasRect;
        self.canvasRect = Some(canvasRect);
    }

    // Java: void setFillType(Path.FillType fillType)
    pub fn setFillType(&mut self, fillType: FillType) {
        // Java: this.pathFillType = fillType;
        self.pathFillType = Some(fillType);
    }

    pub fn getFillType(&self) -> Option<FillType> {
        self.pathFillType
    }

    // Java: void setMeterLimit(int meterLimit)
    pub fn setMeterLimit(&mut self, meterLimit: i32) {
        // Java: this.meterLimit = meterLimit;
        self.meterLimit = meterLimit;
    }

    // Java: int getMeterLimit()
    pub fn getMeterLimit(&self) -> i32 {
        // Java: return this.meterLimit;
        self.meterLimit
    }

    // Java: void setTextAlign(int textAlign)
    pub fn setTextAlign(&mut self, textAlign: i32) {
        // Java: this.textAlign = textAlign;
        self.textAlign = textAlign;
    }

    // Java: int getTextAlign()
    pub fn getTextAlign(&self) -> i32 {
        // Java: return this.textAlign;
        self.textAlign
    }

    // Java: void setPath(GdiPath path)
    pub fn setPath(&mut self, path: GdiPath) {
        // Java: this.path = path;
        self.path = Some(path);
    }

    // Java: GdiPath getPath()
    pub fn getPath(&self) -> &Option<GdiPath> {
        // Java: return this.path;
        &self.path
    }

    // Java: void resetPath(Path.FillType fillType)
    pub fn resetPath(&mut self, fillType: FillType) {
        // Java: if (this.path == null)
        if self.path.is_none() {
            // Java: this.path = new GdiPath(fillType);
            self.path = Some(GdiPath::new_with_mode(crate::cn::wps::graphics::gdi::GdiPath::FillType::WINDING));
        }
        // Java: else
        else {
            // Java: this.path.reset(fillType);
            if let Some(ref mut p) = self.path {
                p.reset();
            }
        }
    }

    // Java: void appendPath(GdiPath appendPath)
    pub fn appendPath(&mut self, appendPath: &GdiPath) {
        // Java: if (this.path != null)
        if self.path.is_some() {
            // Java: if (this.context.worldTransform != null)
            let has_world = self.context.is_some();
            if has_world {
                // Java: this.path.append(appendPath, this.context.worldTransform);
            }
            // Java: else
            else {
                // Java: this.path.append(appendPath, false);
            }
        }
    }

    // Java: void setFont(GDIFont gdiFont)
    pub fn setFont(&mut self, gdiFont: GDIFont) {
        // Java: this.font = gdiFont;
        self.font = Some(gdiFont);
    }

    // Java: void setFontGraphicsMode(int graphicsMode)
    pub fn setFontGraphicsMode(&mut self, graphicsMode: i32) {
        // Java: this.font.graphicsMode = graphicsMode;
        if let Some(ref mut f) = self.font {
            f.graphicsMode = graphicsMode;
        }
    }

    // Java: GDIFont getFont()
    pub fn getFont(&self) -> &Option<GDIFont> {
        // Java: return this.font;
        &self.font
    }

    // Java: GdiBoundContext getContext()
    pub fn getContext(&self) -> &Option<GdiBoundContext> {
        // Java: return this.context;
        &self.context
    }

    // Java: Matrix getTransform()
    pub fn getTransform(&self) -> Option<Matrix> {
        // Java: return this.context.transform;
        None
    }

    // Java: void abortPath()
    pub fn abortPath(&mut self) {
        // Java: if (null != this.path)
        if self.path.is_some() {
            // Java: this.path.reset();
            if let Some(ref mut p) = self.path {
                p.reset();
            }
        }
        // Java: this.path = null;
        self.path = None;
    }
}
