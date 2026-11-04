//! FRB I/O: Dart `layoutDocument` (one-shot) and opaque `WordSession` (edit).
//! Delegates to word-core; does not reimplement wrap/measure.

pub mod session;
pub use session::{CaretHit, WordSession};

use flutter_rust_bridge::frb;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    layout_document as core_layout_document, FontBlob as CoreFontBlob, LayoutError,
    LayoutFrame as CoreLayoutFrame, ViewParams as CoreViewParams,
};
use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::RenderCommand;

#[frb(init, sync)]
pub fn init_app() {
    #[cfg(not(target_family = "wasm"))]
    flutter_rust_bridge::setup_default_user_utils();
}

/// Product host equivalent of Java writer-demo PlatformUtil.platformInit:
/// set the writable application cache root before constructing TextDocument.
#[frb(sync)]
pub fn configure_temp_directory(path: String) -> Result<(), String> {
    let root = std::path::Path::new(&path);
    if !root.is_absolute() {
        return Err("Word temp directory must be absolute".into());
    }
    std::fs::create_dir_all(root).map_err(|error| error.to_string())?;
    util_adapter::cn::wps::core::runtime::Platform::Platform::setTempDirectory(&path);
    Ok(())
}

#[derive(Clone, Debug)]
pub struct FontBlob {
    pub id: String,
    pub bytes: Vec<u8>,
}

/// Twip page box. Zoom is not a field (draw-only in the shell).
#[derive(Clone, Debug)]
pub struct ViewParams {
    pub page_width_twip: i32,
    pub page_height_twip: i32,
    pub margin_left_twip: i32,
    pub margin_right_twip: i32,
    pub margin_top_twip: i32,
    pub margin_bottom_twip: i32,
    pub content_width_twip: Option<i32>,
    pub content_height_twip: Option<i32>,
    pub font_size_pt: f64,
    pub jc: i32,
    /// When true, empty `fonts` is an error (Web). Desktop may leave false and use crate default TTF.
    pub require_injected_fonts: bool,
    /// Force `emit_deco` single underline (XML `<w:u` also sets this in the kernel).
    pub underline: bool,
}

impl ViewParams {
    pub fn default_a4() -> Self {
        ViewParams {
            page_width_twip: 11906,
            page_height_twip: 16838,
            margin_left_twip: 1800,
            margin_right_twip: 1800,
            margin_top_twip: 1440,
            margin_bottom_twip: 1440,
            content_width_twip: None,
            content_height_twip: None,
            font_size_pt: 12.0,
            jc: 0,
            require_injected_fonts: false,
            underline: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PageMetrics {
    pub page_count: i32,
    pub page_width_twip: i32,
    pub page_height_twip: i32,
    pub margin_left_twip: i32,
    pub margin_right_twip: i32,
    pub margin_top_twip: i32,
    pub margin_bottom_twip: i32,
    pub paper_size: String,
}

#[derive(Clone, Debug)]
pub enum RenderCommandWire {
    Text {
        page: i32,
        x: i32,
        y: i32,
        text: String,
        font_id: String,
        size: i32,
        color: u32,
        style: i32,
        xpos: Vec<i32>,
    },
    Rect {
        page: i32,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        color: u32,
        kind: String,
    },
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
    Image {
        page: i32,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        shape_id: i32,
        media_id: i32,
        media_path: String,
        bytes: Vec<u8>,
        rotation: f32,
        crop_left: f32,
        crop_top: f32,
        crop_right: f32,
        crop_bottom: f32,
    },
}

#[derive(Clone, Debug)]
pub struct LayoutFrame {
    pub version: i32,
    pub pipeline: String,
    pub page: PageMetrics,
    pub commands: Vec<RenderCommandWire>,
}

pub(crate) fn map_err(e: LayoutError) -> String {
    format!("{:?}: {}", e.kind, e.message)
}

pub(crate) fn to_core_view(view: &ViewParams) -> CoreViewParams {
    CoreViewParams {
        page_width_twip: view.page_width_twip,
        page_height_twip: view.page_height_twip,
        margin_left_twip: view.margin_left_twip,
        margin_right_twip: view.margin_right_twip,
        margin_top_twip: view.margin_top_twip,
        margin_bottom_twip: view.margin_bottom_twip,
        content_width_twip: view.content_width_twip,
        content_height_twip: view.content_height_twip,
        font_size_pt: view.font_size_pt,
        jc: view.jc,
        underline: view.underline,
    }
}

fn wire_command(cmd: &RenderCommand) -> Result<Option<RenderCommandWire>, String> {
    Ok(match cmd {
        RenderCommand::Text(t) => Some(RenderCommandWire::Text {
            page: t.page,
            x: t.x,
            y: t.y,
            text: t.text.clone(),
            font_id: t.font_id.clone(),
            size: t.size,
            color: t.color,
            style: t.style,
            xpos: t.xpos.clone(),
        }),
        RenderCommand::Rect(r) => Some(RenderCommandWire::Rect {
            page: r.page,
            x: r.x,
            y: r.y,
            w: r.w,
            h: r.h,
            color: r.color,
            kind: r.kind.clone(),
        }),
        RenderCommand::Line(l) => Some(RenderCommandWire::Line {
            page: l.page,
            x1: l.x1,
            y1: l.y1,
            x2: l.x2,
            y2: l.y2,
            kind: l.kind.clone(),
            color: l.color,
            width: l.width,
        }),
        RenderCommand::Image(i) => Some(RenderCommandWire::Image {
            page: i.page,
            x: i.x,
            y: i.y,
            w: i.w,
            h: i.h,
            shape_id: i.shape_id,
            media_id: i.media_id,
            media_path: i.media_path.clone(),
            // FFI owns the image bytes: a media path in the Rust process is
            // not a resource URI that Flutter can open on Android.
            bytes: std::fs::read(&i.media_path).map_err(|e| format!(
                "Image media {} for shape {} (id {}): {}",
                i.media_path, i.shape_id, i.media_id, e
            ))?,
            rotation: i.rotation,
            crop_left: i.crop_left,
            crop_top: i.crop_top,
            crop_right: i.crop_right,
            crop_bottom: i.crop_bottom,
        }),
        RenderCommand::BeginPage(_) | RenderCommand::EndPage(_) => None,
    })
}

pub(crate) fn frame_from_core(core: CoreLayoutFrame) -> Result<LayoutFrame, String> {
    Ok(LayoutFrame {
        version: core.version,
        pipeline: core.pipeline,
        page: PageMetrics {
            page_count: core.page.page_count,
            page_width_twip: core.page.page_width_twip,
            page_height_twip: core.page.page_height_twip,
            margin_left_twip: core.page.margin_left_twip,
            margin_right_twip: core.page.margin_right_twip,
            margin_top_twip: core.page.margin_top_twip,
            margin_bottom_twip: core.page.margin_bottom_twip,
            paper_size: core.page.paper_size,
        },
        commands: core.commands.iter().map(wire_command)
            .collect::<Result<Vec<_>, _>>()?.into_iter().flatten().collect(),
    })
}

/// Product FFI. Coordinates remain twip.
#[frb(sync)]
pub fn layout_document(
    docx: Vec<u8>,
    fonts: Vec<FontBlob>,
    view: ViewParams,
) -> Result<LayoutFrame, String> {
    if view.require_injected_fonts && fonts.is_empty() {
        return Err("Font: require_injected_fonts but fonts is empty".into());
    }
    let core_fonts: Vec<CoreFontBlob> = fonts
        .into_iter()
        .map(|f| CoreFontBlob {
            id: f.id,
            bytes: f.bytes,
        })
        .collect();
    let core = core_layout_document(&docx, &core_fonts, &to_core_view(&view)).map_err(map_err)?;
    frame_from_core(core)
}

/// Derived dump for goldens / debug (not the product hot path).
#[frb(sync)]
pub fn dump_json(frame: LayoutFrame) -> Result<String, String> {
    let cmds: Vec<serde_json::Value> = frame
        .commands
        .iter()
        .map(|c| match c {
            RenderCommandWire::Text {
                page,
                x,
                y,
                text,
                font_id,
                size,
                color,
                style,
                xpos,
            } => serde_json::json!({
                "t": "text", "page": page, "x": x, "y": y, "text": text,
                "font_id": font_id, "size": size, "color": color, "style": style,
                "xpos": xpos
            }),
            RenderCommandWire::Rect {
                page,
                x,
                y,
                w,
                h,
                color,
                kind,
            } => serde_json::json!({
                "t": "rect", "page": page, "x": x, "y": y, "w": w, "h": h, "color": color, "kind": kind
            }),
            RenderCommandWire::Line {
                page,
                x1,
                y1,
                x2,
                y2,
                kind,
                color,
                width,
            } => serde_json::json!({
                "t": "line", "page": page, "x1": x1, "y1": y1, "x2": x2, "y2": y2,
                "kind": kind, "color": color, "width": width
            }),
            RenderCommandWire::Image {
                page, x, y, w, h, shape_id, media_id, media_path, bytes,
                rotation, crop_left, crop_top, crop_right, crop_bottom,
            } => serde_json::json!({
                "t": "image", "page": page, "x": x, "y": y, "w": w, "h": h,
                "shape_id": shape_id, "media_id": media_id,
                "media_path": media_path, "bytes_len": bytes.len(),
                "rotation": rotation,
                "crop_left": crop_left, "crop_top": crop_top,
                "crop_right": crop_right, "crop_bottom": crop_bottom
            }),
        })
        .collect();
    serde_json::to_string_pretty(&serde_json::json!({
        "ok": true,
        "pipeline": frame.pipeline,
        "page": {
            "page_count": frame.page.page_count,
            "page_width_twip": frame.page.page_width_twip,
            "page_height_twip": frame.page.page_height_twip,
            "margin_left_twip": frame.page.margin_left_twip,
            "margin_right_twip": frame.page.margin_right_twip,
            "margin_top_twip": frame.page.margin_top_twip,
            "margin_bottom_twip": frame.page.margin_bottom_twip,
            "paper_size": frame.page.paper_size,
        },
        "document": {
            "version": frame.version,
            "commands": cmds,
        }
    }))
    .map_err(|e| e.to_string())
}
