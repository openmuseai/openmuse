//! Viewport zoom/scroll: replay the same commands (no relayout).

use crate::cn::wps::moffice::writer::api::RenderPainter::{paint, PaintOp};
use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::CommandDocument;

#[derive(Clone, Debug)]
pub struct Viewport {
    pub zoom: f64,
    pub offset_x: i32,
    pub offset_y: i32,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport {
            zoom: 1.0,
            offset_x: 0,
            offset_y: 0,
        }
    }
}

impl Viewport {
    pub fn on_scale(&mut self, factor: f64) {
        self.zoom = (self.zoom * factor).max(0.1);
    }

    pub fn on_scroll(&mut self, dx: i32, dy: i32) {
        self.offset_x += dx;
        self.offset_y += dy;
    }

    pub fn paint_ops(&self, doc: &CommandDocument) -> Vec<PaintOp> {
        let mut ops = paint(doc, self.zoom);
        for op in &mut ops {
            match op {
                PaintOp::Text { x_px, y_px, .. } => {
                    *x_px += self.offset_x as f64;
                    *y_px += self.offset_y as f64;
                }
                PaintOp::Rect { x_px, y_px, .. } => {
                    *x_px += self.offset_x as f64;
                    *y_px += self.offset_y as f64;
                }
                PaintOp::Line {
                    x1_px,
                    y1_px,
                    x2_px,
                    y2_px,
                    ..
                } => {
                    *x1_px += self.offset_x as f64;
                    *y1_px += self.offset_y as f64;
                    *x2_px += self.offset_x as f64;
                    *y2_px += self.offset_y as f64;
                }
                PaintOp::Image { x_px, y_px, .. } => {
                    *x_px += self.offset_x as f64;
                    *y_px += self.offset_y as f64;
                }
            }
        }
        ops
    }
}
