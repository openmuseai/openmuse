//! Command list → draw ops (twip→px). Unknown variants skipped.

use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::{
    CommandDocument, RenderCommand,
};

pub const TEXT_SKEW_VALUE: f64 = -0.25;

#[derive(Clone, Debug, PartialEq)]
pub enum PaintOp {
    Text {
        x_px: f64,
        y_px: f64,
        text: String,
    },
    Rect {
        x_px: f64,
        y_px: f64,
        w_px: f64,
        h_px: f64,
    },
    Line {
        x1_px: f64,
        y1_px: f64,
        x2_px: f64,
        y2_px: f64,
        kind: String,
    },
    Image {
        x_px: f64,
        y_px: f64,
        w_px: f64,
        h_px: f64,
        media_path: String,
    },
}

/// Java MetricsUtil 96dpi: `px = twip * zoom * (1.3333536 / 20)`
pub fn twip_to_px(twip: i32, zoom: f64) -> f64 {
    twip as f64 * zoom * (1.3333536 / 20.0)
}

pub fn paint(doc: &CommandDocument, zoom: f64) -> Vec<PaintOp> {
    let mut ops = Vec::new();
    for cmd in &doc.commands {
        match cmd {
            RenderCommand::BeginPage(_) | RenderCommand::EndPage(_) => {}
            RenderCommand::Text(t) => ops.push(PaintOp::Text {
                x_px: twip_to_px(t.x, zoom),
                y_px: twip_to_px(t.y, zoom),
                text: t.text.clone(),
            }),
            RenderCommand::Rect(r) => ops.push(PaintOp::Rect {
                x_px: twip_to_px(r.x, zoom),
                y_px: twip_to_px(r.y, zoom),
                w_px: twip_to_px(r.w, zoom),
                h_px: twip_to_px(r.h, zoom),
            }),
            RenderCommand::Line(l) => ops.push(PaintOp::Line {
                x1_px: twip_to_px(l.x1, zoom),
                y1_px: twip_to_px(l.y1, zoom),
                x2_px: twip_to_px(l.x2, zoom),
                y2_px: twip_to_px(l.y2, zoom),
                kind: l.kind.clone(),
            }),
            RenderCommand::Image(img) => ops.push(PaintOp::Image {
                x_px: twip_to_px(img.x, zoom),
                y_px: twip_to_px(img.y, zoom),
                w_px: twip_to_px(img.w, zoom),
                h_px: twip_to_px(img.h, zoom),
                media_path: img.media_path.clone(),
            }),
        }
    }
    ops
}
