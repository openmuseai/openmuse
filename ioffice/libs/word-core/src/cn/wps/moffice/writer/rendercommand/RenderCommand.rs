//! Platform-neutral render commands (Stage3 JSON tagged union).

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrawText {
    pub page: i32,
    pub x: i32,
    /// Baseline, page-absolute twip (P3). Older goldens without `xpos` still parse.
    pub y: i32,
    pub text: String,
    pub font_id: String,
    pub size: i32,
    pub color: u32,
    pub style: i32,
    /// Per visible codepoint, page-absolute twip. Same origin as `x` for the first glyph.
    #[serde(default)]
    pub xpos: Vec<i32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrawRect {
    pub page: i32,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub color: u32,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrawLine {
    pub page: i32,
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub kind: String,
    pub color: u32,
    pub width: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeginPage {
    pub page: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndPage {
    pub page: i32,
}

/// Java `InstractionBuilder.drawPicture` / `drawShape`: page-absolute twip box,
/// shape id, media id and path, rotation and crop.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrawImage {
    pub page: i32,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub shape_id: i32,
    pub media_id: i32,
    pub media_path: String,
    pub rotation: f32,
    pub crop_left: f32,
    pub crop_top: f32,
    pub crop_right: f32,
    pub crop_bottom: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t")]
pub enum RenderCommand {
    #[serde(rename = "begin_page")]
    BeginPage(BeginPage),
    #[serde(rename = "end_page")]
    EndPage(EndPage),
    #[serde(rename = "text")]
    Text(DrawText),
    #[serde(rename = "rect")]
    Rect(DrawRect),
    #[serde(rename = "line")]
    Line(DrawLine),
    #[serde(rename = "image")]
    Image(DrawImage),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandDocument {
    pub version: i32,
    pub commands: Vec<RenderCommand>,
}

pub const STYLE_BOLD: i32 = 1;
pub const STYLE_ITALIC: i32 = 2;
