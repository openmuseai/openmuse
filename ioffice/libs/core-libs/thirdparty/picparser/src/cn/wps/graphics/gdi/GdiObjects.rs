#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut)]

use std::sync::Mutex;

use crate::cn::wps::graphics::gdi::IGdiObject::{IGdiObject, GdiPainter};

pub struct ColorPaint { color: i32 }
impl ColorPaint {
    pub const WHITE: ColorPaint = ColorPaint { color: -1 };
    pub const BLACK: ColorPaint = ColorPaint { color: -16777216 };
    pub const NULL: ColorPaint = ColorPaint { color: 0 };
    pub fn getColor(&self) -> i32 { self.color }
}

pub struct GDILogBrush { style: i32, color: Option<ColorPaint> }
impl GDILogBrush {
    pub fn new(style: i32, color: Option<ColorPaint>) -> Self { GDILogBrush { style, color } }
}
impl IGdiObject for GDILogBrush { fn render(&self, _: &GdiPainter) {} }

pub struct GdiPen { style: i32, width: i32, color: ColorPaint }
impl GdiPen {
    pub fn new(style: i32, width: i32, color: ColorPaint) -> Self { GdiPen { style, width, color } }
}
impl IGdiObject for GdiPen { fn render(&self, _: &GdiPainter) {} }

pub struct GDIFont { name: String }
impl GDIFont {
    pub fn new(name: &str) -> Self { GDIFont { name: name.to_string() } }
}
impl IGdiObject for GDIFont { fn render(&self, _: &GdiPainter) {} }

fn create_brush_color(color: &ColorPaint) -> Box<dyn IGdiObject> { Box::new(GDILogBrush::new(0, Some(ColorPaint { color: color.color }))) }
fn create_brush_int(color: i32) -> Box<dyn IGdiObject> { Box::new(GDILogBrush::new(0, Some(ColorPaint { color }))) }
fn create_pen(color: &ColorPaint) -> Box<dyn IGdiObject> { Box::new(GdiPen::new(0, 0, ColorPaint { color: color.color })) }
fn create_font(name: &str) -> Box<dyn IGdiObject> { Box::new(GDIFont::new(name)) }

// Java: public class GdiObjects
pub struct GdiObjects {
    // Java field: String TAG
    // Java field: IGdiObject defaultFont
    // Java field: IGdiObject[] gdiObjects
    pub(crate) gdiObjects: Vec<Option<Box<dyn IGdiObject>>>,
    // Java field: int WHITE_BRUSH
    // Java field: int LTGRAY_BRUSH
    // Java field: int GRAY_BRUSH
    // Java field: int DKGRAY_BRUSH
    // Java field: int BLACK_BRUSH
    // Java field: int NULL_BRUSH
    // Java field: int WHITE_PEN
    // Java field: int BLACK_PEN
    // Java field: int NULL_PEN
    // Java field: int OEM_FIXED_FONT
    // Java field: int ANSI_FIXED_FONT
    // Java field: int ANSI_VAR_FONT
    // Java field: int SYSTEM_FONT
    // Java field: int DEVICE_DEFAULT_FONT
    // Java field: int DEFAULT_PALETTE
    // Java field: int SYSTEM_FIXED_FONT
    // Java field: int DEFAULT_GUI_FONT
    // Java field: int DC_BRUSH
    // Java field: int DC_PEN
}

pub(crate) const TAG: &str = "";
static DEFAULT_FONT: Mutex<Option<Box<dyn IGdiObject + Send + Sync>>> = Mutex::new(None);
pub(crate) const WHITE_BRUSH: i32 = i32::MIN;
pub(crate) const LTGRAY_BRUSH: i32 = -2147483647;
pub(crate) const GRAY_BRUSH: i32 = -2147483646;
pub(crate) const DKGRAY_BRUSH: i32 = -2147483645;
pub(crate) const BLACK_BRUSH: i32 = -2147483644;
pub(crate) const NULL_BRUSH: i32 = -2147483643;
pub(crate) const WHITE_PEN: i32 = -2147483642;
pub(crate) const BLACK_PEN: i32 = -2147483641;
pub(crate) const NULL_PEN: i32 = -2147483640;
pub(crate) const OEM_FIXED_FONT: i32 = -2147483638;
pub(crate) const ANSI_FIXED_FONT: i32 = -2147483637;
pub(crate) const ANSI_VAR_FONT: i32 = -2147483636;
pub(crate) const SYSTEM_FONT: i32 = -2147483635;
pub(crate) const DEVICE_DEFAULT_FONT: i32 = -2147483634;
pub(crate) const DEFAULT_PALETTE: i32 = -2147483633;
pub(crate) const SYSTEM_FIXED_FONT: i32 = -2147483632;
pub(crate) const DEFAULT_GUI_FONT: i32 = -2147483631;
pub(crate) const DC_BRUSH: i32 = -2147483630;
pub(crate) const DC_PEN: i32 = -2147483629;

// Java: public class GdiObjects
impl GdiObjects {
    // Java: GdiObjects(int handles)
    pub fn new(handles: i32) -> Self {
        // Java: super();
        // Java: this.gdiObjects = new IGdiObject[handles];
        let mut gdiObjects = Vec::with_capacity(handles as usize);
        for _ in 0..handles { gdiObjects.push(None); }
        GdiObjects { gdiObjects }
    }

    // Java: void setObject(int index, IGdiObject object)
    pub fn setObject(&mut self, mut index: i32, object: Option<Box<dyn IGdiObject>>) {
        // Java: if (index == -1)
        if index == -1 {
            // Java: for(int i = 0; i < this.gdiObjects.length; ++i)
            for i in 0..self.gdiObjects.len() {
                // Java: if (this.gdiObjects[i] == null)
                if self.gdiObjects[i].is_none() {
                    // Java: index = i;
                    index = i as i32;
                    // Java: break;
                    break;
                }
            }
        }
        // Java: if (index != -1)
        if index != -1 {
            // Java: this.gdiObjects[index] = object;
            self.gdiObjects[index as usize] = object;
        }
    }

    // Java: void deleteObject(int index)
    pub fn deleteObject(&mut self, index: i32) {
        // Java: this.setObject(index, (IGdiObject)null);
        self.setObject(index, None);
    }

    // Java: IGdiObject getObject(int index)
    pub fn getObject(&self, index: i32) -> Option<&dyn IGdiObject> {
        // Java: IGdiObject gdiObject = null;
        let mut gdiObject: Option<&dyn IGdiObject> = None;
        // Java: if (index < 0)
        if index < 0 {
            // Java: gdiObject = getStockObject(index);
            return None;
        }
        // Java: else if (index < this.gdiObjects.length)
        else if (index as usize) < self.gdiObjects.len() {
            // Java: gdiObject = this.gdiObjects[index];
            gdiObject = self.gdiObjects[index as usize].as_deref();
        }
        // Java: return gdiObject;
        gdiObject
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: int size = this.gdiObjects.length;
        let size = self.gdiObjects.len();
        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size {
            // Java: this.gdiObjects[i] = null;
            self.gdiObjects[i] = None;
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.gdiObjects = null;
        self.gdiObjects.clear();
    }

    // Java: static IGdiObject getStockObject(int stockId)
    pub fn getStockObject(stockId: i32) -> Option<Box<dyn IGdiObject>> {
        // Java: if (stockId >= Integer.MIN_VALUE && stockId <= -2147483629)
        if stockId >= i32::MIN && stockId <= -2147483629 {
            // Java: IGdiObject gdiObject = null;
            let gdiObject: Option<Box<dyn IGdiObject>>;
            // Java: switch (stockId)
            match stockId {
                // Java: case Integer.MIN_VALUE:
                i32::MIN => {
                    // Java: gdiObject = GdiTool.createBrush(ColorPaint.WHITE);
                    gdiObject = Some(create_brush_color(&ColorPaint::WHITE));
                    // Java: break;
                }
                // Java: case -2147483647:
                -2147483647 => {
                    // Java: gdiObject = GdiTool.createBrush(-3355444);
                    gdiObject = Some(create_brush_int(-3355444));
                    // Java: break;
                }
                // Java: case -2147483646:
                -2147483646 => {
                    // Java: gdiObject = GdiTool.createBrush(-7829368);
                    gdiObject = Some(create_brush_int(-7829368));
                    // Java: break;
                }
                // Java: case -2147483645:
                -2147483645 => {
                    // Java: gdiObject = GdiTool.createBrush(-12303292);
                    gdiObject = Some(create_brush_int(-12303292));
                    // Java: break;
                }
                // Java: case -2147483644:
                -2147483644 => {
                    // Java: gdiObject = GdiTool.createBrush(ColorPaint.BLACK);
                    gdiObject = Some(create_brush_color(&ColorPaint::BLACK));
                    // Java: break;
                }
                // Java: case -2147483643:
                -2147483643 => {
                    // Java: gdiObject = new GDILogBrush(1, (ColorPaint)null);
                    gdiObject = Some(Box::new(GDILogBrush::new(1, None)));
                    // Java: break;
                }
                // Java: case -2147483642:
                // Java: case -2147483630:
                // Java: case -2147483629:
                -2147483642 | -2147483630 | -2147483629 => {
                    // Java: gdiObject = GdiTool.createPen(ColorPaint.WHITE);
                    gdiObject = Some(create_pen(&ColorPaint::WHITE));
                    // Java: break;
                }
                // Java: case -2147483641:
                -2147483641 => {
                    // Java: gdiObject = GdiTool.createPen(ColorPaint.BLACK);
                    gdiObject = Some(create_pen(&ColorPaint::BLACK));
                    // Java: break;
                }
                // Java: case -2147483640:
                -2147483640 => {
                    // Java: gdiObject = new GdiPen(5, 0, ColorPaint.NULL);
                    gdiObject = Some(Box::new(GdiPen::new(5, 0, ColorPaint::NULL)));
                    // Java: break;
                }
                // Java: case -2147483639:
                // Java: case -2147483633:
                // Java: default:
                -2147483639 | -2147483633 => {
                    // Java: gdiObject = null;
                    gdiObject = None;
                    // Java: break;
                }
                // Java: case -2147483638:
                // Java: case -2147483637:
                // Java: case -2147483636:
                // Java: case -2147483635:
                // Java: case -2147483634:
                // Java: case -2147483632:
                // Java: case -2147483631:
                -2147483638 | -2147483637 | -2147483636 | -2147483635 | -2147483634 | -2147483632 | -2147483631 => {
                    // Java: if (defaultFont == null)
                    // Java: defaultFont = GdiTool.createFont("serif");
                    // Java: gdiObject = defaultFont;
                    gdiObject = Some(create_font("serif"));
                }
                _ => {
                    gdiObject = None;
                }
            }
            // Java: return gdiObject;
            gdiObject
        }
        // Java: else
        else {
            // Java: return null;
            None
        }
    }
}
