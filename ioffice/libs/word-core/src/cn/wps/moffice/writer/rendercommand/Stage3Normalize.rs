//! P4 Stage3 normalize: Java `InstractionBuilder` / `ins_draw_x_pos_text` projection.
//!
//! Product IR keeps one `DrawText` + `xpos[]` per run. Goldens compare the
//! **expanded per-glyph** sequence (same as Java's xpos drawText loop).
//! `pipeline` is not part of the document. Color 0 and 0xFF000000 are both ink-black.

use crate::cn::wps::moffice::writer::api::LayoutDocument::LayoutFrame;
use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::{
    CommandDocument, DrawLine, DrawRect, RenderCommand,
};
use serde::{Deserialize, Serialize};

/// Canonical Stage3 after Java-shaped expansion. `version` matches `LayoutFrame`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NormalizedStage3 {
    pub version: i32,
    pub commands: Vec<NormalizedCmd>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t")]
pub enum NormalizedCmd {
    #[serde(rename = "glyph")]
    Glyph {
        page: i32,
        x: i32,
        y: i32,
        ch: String,
        size: i32,
        color: u32,
        style: i32,
    },
    #[serde(rename = "rect")]
    Rect {
        page: i32,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        color: u32,
        kind: String,
    },
    #[serde(rename = "line")]
    Line {
        page: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        kind: String,
        color: u32,
        width: i32,
    },
}

/// 0 and 0xFF000000 are the same black in Java Canvas / Flutter.
pub fn canonical_ink(color: u32) -> u32 {
    if color == 0 || color == 0xFF000000 {
        0
    } else {
        color
    }
}

fn visible_chars(text: &str) -> Vec<char> {
    text.chars()
        .filter(|c| *c != '\r' && *c != '\n' && *c != '\u{000c}')
        .collect()
}

pub fn normalize_commands(version: i32, commands: &[RenderCommand]) -> NormalizedStage3 {
    let mut out = Vec::new();
    for cmd in commands {
        match cmd {
            RenderCommand::BeginPage(_) | RenderCommand::EndPage(_) => {}
            RenderCommand::Text(t) => {
                let color = canonical_ink(t.color);
                let glyphs = visible_chars(&t.text);
                if !t.xpos.is_empty() && t.xpos.len() == glyphs.len() {
                    for (i, ch) in glyphs.into_iter().enumerate() {
                        out.push(NormalizedCmd::Glyph {
                            page: t.page,
                            x: t.xpos[i],
                            y: t.y,
                            ch: ch.to_string(),
                            size: t.size,
                            color,
                            style: t.style,
                        });
                    }
                } else {
                    // Java drawText(String,x,y) / missing xpos: one run, origin = x.
                    out.push(NormalizedCmd::Glyph {
                        page: t.page,
                        x: t.x,
                        y: t.y,
                        ch: glyphs.into_iter().collect(),
                        size: t.size,
                        color,
                        style: t.style,
                    });
                }
            }
            RenderCommand::Rect(DrawRect {
                page,
                x,
                y,
                w,
                h,
                color,
                kind,
            }) => out.push(NormalizedCmd::Rect {
                page: *page,
                x: *x,
                y: *y,
                w: *w,
                h: *h,
                color: canonical_ink(*color),
                kind: kind.clone(),
            }),
            RenderCommand::Line(DrawLine {
                page,
                x1,
                y1,
                x2,
                y2,
                kind,
                color,
                width,
            }) => out.push(NormalizedCmd::Line {
                page: *page,
                x1: *x1,
                y1: *y1,
                x2: *x2,
                y2: *y2,
                kind: kind.clone(),
                color: canonical_ink(*color),
                width: *width,
            }),
            RenderCommand::Image(_) => {}
        }
    }
    NormalizedStage3 {
        version,
        commands: out,
    }
}

pub fn normalize_document(doc: &CommandDocument) -> NormalizedStage3 {
    normalize_commands(doc.version, &doc.commands)
}

pub fn normalize_frame(frame: &LayoutFrame) -> NormalizedStage3 {
    normalize_commands(frame.version, &frame.commands)
}

/// Compact JSON; field order is serde declaration order (stable).
pub fn canonical_json(n: &NormalizedStage3) -> Result<String, serde_json::Error> {
    serde_json::to_string(n)
}

/// FNV-1a 64 of canonical JSON (rustc-independent; not a crypto hash).
pub fn canonical_hash(n: &NormalizedStage3) -> Result<String, serde_json::Error> {
    let json = canonical_json(n)?;
    Ok(format!("{:016x}", fnv1a64(json.as_bytes())))
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// First mismatched command index, for CI output (`page/x/y/text`).
pub fn first_divergence(rust: &NormalizedStage3, expected: &NormalizedStage3) -> Option<String> {
    if rust.version != expected.version {
        return Some(format!(
            "version rust={} expected={}",
            rust.version, expected.version
        ));
    }
    let n = rust.commands.len().max(expected.commands.len());
    for i in 0..n {
        let a = rust.commands.get(i);
        let b = expected.commands.get(i);
        if a != b {
            return Some(format!(
                "cmd[{i}] rust={} expected={}",
                fmt_cmd(a),
                fmt_cmd(b)
            ));
        }
    }
    None
}

fn fmt_cmd(c: Option<&NormalizedCmd>) -> String {
    match c {
        None => "<missing>".into(),
        Some(NormalizedCmd::Glyph {
            page, x, y, ch, ..
        }) => format!("glyph page={page} x={x} y={y} ch={ch:?}"),
        Some(NormalizedCmd::Rect { page, x, y, w, h, kind, .. }) => {
            format!("rect page={page} x={x} y={y} {w}x{h} kind={kind}")
        }
        Some(NormalizedCmd::Line {
            page, x1, y1, x2, y2, kind, ..
        }) => format!("line page={page} ({x1},{y1})-({x2},{y2}) kind={kind}"),
    }
}
