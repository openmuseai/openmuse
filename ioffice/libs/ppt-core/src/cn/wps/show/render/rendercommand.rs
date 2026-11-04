use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RenderCommand {
    DrawRect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        fill: Option<u32>,
        stroke: Option<u32>,
    },
    DrawPath {
        path: Vec<(f32, f32)>,
        fill: Option<u32>,
        stroke: Option<u32>,
    },
    DrawText {
        text: String,
        x: f32,
        y: f32,
        font: Option<String>,
        size: Option<f32>,
        color: Option<u32>,
        bold: Option<bool>,
        italic: Option<bool>,
    },
    DrawImage {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        media_id: i32,
        r_id: Option<String>,
        #[serde(default)]
        bytes_b64: Option<String>,
    },
}

pub fn to_json(cmds: &[RenderCommand]) -> Result<String, serde_json::Error> {
    serde_json::to_string(cmds)
}

pub fn from_json(s: &str) -> Result<Vec<RenderCommand>, serde_json::Error> {
    serde_json::from_str(s)
}

use crate::cn::wps::show::layout::typo::GreedyWrap::ShowTypoLine;
use crate::cn::wps::show::render::ShowTypoShape::ShowTypoShape;
use crate::cn::wps::show::render::SlideRender::LayerItem;
use crate::cn::wps::show::util::DrawFlags::DrawFlags;
use crate::cn::wps::show::app::shape::kmo_show_shape_hosts;
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use util::cn::wps::graphics::RectF::RectF;
use drawing::cn::wps::moffice::drawing::graphics::GraphicsPath::GraphicsPath;
use drawing::cn::wps::moffice::drawing::smartshape::ShapeType::ShapeType;
use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

fn graphics_paths_to_points(gps: &[Element], rc: &RectF) -> Option<Vec<(f32, f32)>> {
    let mut out: Vec<(f32, f32)> = Vec::new();
    let mut start: Option<(f32, f32)> = None;
    for el in gps {
        let Some(arc) = el else { continue };
        let Some(gp) = arc.downcast_ref::<GraphicsPath>() else { continue };
        let n = gp.size();
        for i in 0..n {
            let elem = gp.getElem(i);
            match elem.elemType {
                0 | 1 => {
                    if elem.pos.len() >= 2 {
                        let pt = (elem.pos[0] + rc.left, elem.pos[1] + rc.top);
                        if elem.elemType == 0 {
                            start = Some(pt);
                        }
                        out.push(pt);
                    }
                }
                3 => {
                    if elem.pos.len() >= 4 {
                        out.push((elem.pos[2] + rc.left, elem.pos[3] + rc.top));
                    }
                }
                4 => {
                    if elem.pos.len() >= 6 {
                        out.push((elem.pos[4] + rc.left, elem.pos[5] + rc.top));
                    }
                }
                5 => {
                    if let Some(s) = start {
                        out.push(s);
                    }
                }
                _ => {}
            }
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

/// OOXML preset inscribed in `rc` (already render px). Returns None → caller uses DrawRect.
fn preset_path_in_rect(prst: i32, rc: &RectF) -> Option<Vec<(f32, f32)>> {
    let l = rc.left;
    let t = rc.top;
    let r = rc.right;
    let b = rc.bottom;
    let cx = (l + r) * 0.5;
    let cy = (t + b) * 0.5;
    let rx = (r - l) * 0.5;
    let ry = (b - t) * 0.5;
    match prst {
        20 => Some(vec![(l, cy), (r, cy)]),
        3 => {
            let n = 16i32;
            let mut path = Vec::with_capacity((n + 1) as usize);
            for i in 0..n {
                let a = (i as f32) * std::f32::consts::TAU / (n as f32);
                path.push((cx + rx * a.cos(), cy + ry * a.sin()));
            }
            if let Some(first) = path.first().copied() {
                path.push(first);
            }
            Some(path)
        }
        56 => {
            let mut path = Vec::with_capacity(6);
            for i in 0..5 {
                let a = -std::f32::consts::FRAC_PI_2 + (i as f32) * std::f32::consts::TAU / 5.0;
                path.push((cx + rx * a.cos(), cy + ry * a.sin()));
            }
            if let Some(first) = path.first().copied() {
                path.push(first);
            }
            Some(path)
        }
        _ => None,
    }
}

pub fn geometry_commands(item: &mut LayerItem) -> Vec<RenderCommand> {
    match item {
        LayerItem::WhiteBackground { w, h } => vec![RenderCommand::DrawRect {
            x: 0.0,
            y: 0.0,
            w: *w,
            h: *h,
            fill: Some(0xFFFFFFFF),
            stroke: None,
        }],
        LayerItem::Background { w, h, fill } => vec![RenderCommand::DrawRect {
            x: 0.0,
            y: 0.0,
            w: *w,
            h: *h,
            fill: *fill,
            stroke: None,
        }],
        LayerItem::Shape(typo) => {
            let Some(base) = typo.getShowShape() else {
                return Vec::new();
            };
            let attach = kmo_show_shape_hosts::convert_attach(base.id());
            let graphics_path = attach.graphics_path.clone();
            let prst = attach.prst;
            let skip_text_wireframe = (typo.isTextBox() || attach.has_text_body)
                && attach.fill.is_none()
                && attach.media_bytes.is_none()
                && attach.media_r_id.is_none();
            if skip_text_wireframe {
                return Vec::new();
            }
            if let Some(pts) = graphics_path {
                if pts.is_empty() {
                    return Vec::new();
                }
                return vec![RenderCommand::DrawPath {
                    path: pts,
                    fill: None,
                    stroke: None,
                }];
            }
            let mut public_shape = typo.getShape();
            let prop_rc = if let Some(ref mut ps) = public_shape {
                ps.getPropertyValue(733)
                    .and_then(|a| a.downcast_ref::<RectF>().cloned())
                    .filter(|r| r.right - r.left > 0.0 && r.bottom - r.top > 0.0)
            } else {
                None
            };
            let rc = prop_rc.clone().unwrap_or_else(|| typo.getShapeRect());
            if let (Some(ref mut ps), Some(ref prop_rc)) = (public_shape.as_mut(), prop_rc.as_ref())
            {
                let w = prop_rc.right - prop_rc.left;
                let h = prop_rc.bottom - prop_rc.top;
                if let Some(gps) = ps.getGraphicsPaths(w, h) {
                    if let Some(path) = graphics_paths_to_points(&gps, &rc) {
                        let shape_type = ps.getShapeType();
                        let prst_is_line = prst.map(ShapeType::isLine).unwrap_or(false);
                        // Drawable prst=line but CDL ran as another preset (often placeholder rect).
                        if !(prst_is_line && path.len() != 2) {
                            let stroke = if ShapeType::isLine(shape_type) || prst_is_line || path.len() == 2 {
                                Some(0xFF000000)
                            } else {
                                None
                            };
                            return vec![RenderCommand::DrawPath {
                                path,
                                fill: None,
                                stroke,
                            }];
                        }
                    }
                }
            }
            let shape_type = public_shape.as_ref().map(|s| s.getShapeType());
            let prst = prst.or(shape_type);
            if let Some(prst) = prst {
                if let Some(path) = preset_path_in_rect(prst, &rc) {
                    // OOXML line (prst=20) is stroke-only; empty fill+stroke is skipped by the host painter.
                    let stroke = if ShapeType::isLine(prst) { Some(0xFF000000) } else { None };
                    return vec![RenderCommand::DrawPath {
                        path,
                        fill: None,
                        stroke,
                    }];
                }
            }
            vec![RenderCommand::DrawRect {
                x: rc.left,
                y: rc.top,
                w: rc.right - rc.left,
                h: rc.bottom - rc.top,
                fill: None,
                stroke: None,
            }]
        }
    }
}

pub fn bytes_to_b64(data: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((data.len() + 2) / 3 * 4);
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i] as u32;
        let b1 = if i + 1 < data.len() { data[i + 1] as u32 } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(T[((triple >> 18) & 63) as usize] as char);
        out.push(T[((triple >> 12) & 63) as usize] as char);
        out.push(if i + 1 < data.len() {
            T[((triple >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if i + 2 < data.len() {
            T[(triple & 63) as usize] as char
        } else {
            '='
        });
        i += 3;
    }
    out
}

pub fn apply_fill_line(cmd: RenderCommand, fill: Option<u32>, stroke: Option<u32>) -> RenderCommand {
    match cmd {
        RenderCommand::DrawRect { x, y, w, h, .. } => RenderCommand::DrawRect {
            x,
            y,
            w,
            h,
            fill,
            stroke,
        },
        RenderCommand::DrawPath { path, .. } => RenderCommand::DrawPath {
            path,
            fill,
            stroke,
        },
        RenderCommand::DrawImage { .. } => cmd,
        other => other,
    }
}

pub fn text_line_commands(
    typo: &ShowTypoShape,
    lines: &[ShowTypoLine],
    drawFlags: i32,
    origin_x: f32,
    origin_y: f32,
) -> Vec<RenderCommand> {
    if 0 != (drawFlags & DrawFlags::NOT_DRAW_PLACEHOLDER_TEXT) && typo.isPlaceholder() {
        return Vec::new();
    }
    if lines.is_empty() {
        return Vec::new();
    }
    lines
        .iter()
        .map(|ln| RenderCommand::DrawText {
            text: ln.text.clone(),
            x: origin_x + ln.x,
            y: origin_y + ln.y,
            font: None,
            size: Some(ln.h),
            color: None,
            bold: None,
            italic: None,
        })
        .collect()
}

pub fn apply_run_style(
    cmd: RenderCommand,
    font: Option<String>,
    size: Option<f32>,
    color: Option<u32>,
    bold: Option<bool>,
    italic: Option<bool>,
) -> RenderCommand {
    match cmd {
        RenderCommand::DrawText { text, x, y, .. } => RenderCommand::DrawText {
            text,
            x,
            y,
            font,
            size,
            color,
            bold,
            italic,
        },
        other => other,
    }
}

#[allow(dead_code)]
fn _touch_shape(_s: &KmoShowShape) {}
