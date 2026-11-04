//! P1/P3/P7 orchestration I/O: docx/runs + fonts + view → typed `LayoutFrame`.
//!
//! Product hot path (wr-29 06):
//! `docx → XWPF parse → DocumentImporter → TextDocument → GraphBuffFactory
//! → GraphReader.prepareGraphs → GraphFactory.createNodes → TextLayouter.doFlowTextLine
//! → LineLayouter.flow → PaginatedPage → emit`.
//! Stage2 `layout_lines` remains on `layout_text_document_stage2` / `layout_layout_runs_with_styles`.
//! `xpos[i]` is the page-absolute origin of visible codepoint `i`.
//! JSON is a derived Stage3 dump, not the product type.

use crate::cn::wps::moffice::writer::cache::Stage2Rows::{rows_from_pages, Stage2Line};
use crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup;
use crate::cn::wps::moffice::writer::layout::base::env::ContentWidth::ContentWidth;
use crate::cn::wps::moffice::writer::layout::base::env::FaceSelect::{is_east_asia, FaceSelect};
use crate::cn::wps::moffice::writer::layout::base::env::FontStyleFactory::FontStyleFactory;
use crate::cn::wps::moffice::writer::layout::base::env::LineBox::LineSpacing;
use crate::cn::wps::moffice::writer::layout::base::env::LineLayout::{
    layout_lines, LayoutRun as LineLayoutRun, ParaPapx,
};
use crate::cn::wps::moffice::writer::layout::base::env::PageLayout::paginate;
use crate::cn::wps::moffice::writer::layout::base::env::TypoFontStyle::TypoFontStyle;
use crate::cn::wps::moffice::writer::layout::base::env::WidowControl::{CellFrame, PageLine};
use crate::cn::wps::moffice::writer::layout::base::line::RunMetrics::RunMetrics;
use crate::cn::wps::moffice::writer::layout::base::text::Alignment::JC_LEFT;
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::layout::base::text::RunStream::{LayoutSourceRun, RunStream};
use crate::cn::wps::moffice::writer::layout::base::TextLayouter::TextLayouter;
use crate::cn::wps::moffice::writer::layout::base::ParaLayouter::ParaLayouter;
use crate::cn::wps::moffice::writer::layout::base::SectionTool::SectionTool;
use crate::cn::wps::moffice::writer::layout::pageview::PageViewSectionLayouter::PageViewSectionLayouter;
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use crate::cn::wps::moffice::writer::layout::base::env::PageLayout::PaginatedPage;
use crate::cn::wps::moffice::writer::render::renderer::PageRender::PageRender;
use crate::cn::wps::moffice::writer::render::renderer::TableRender::TableRender;
use crate::cn::wps::moffice::writer::layout::base::env::ParaPropEnv::ParaPropEnv;
use crate::cn::wps::moffice::writer::layout::base::env::PropertyToolFactory::PropertyToolFactory;
use crate::cn::wps::moffice::writer::layout::base::env::IPropertyTool::IPropertyTool;
use crate::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::graph::GraphBuffFactory::GraphBuffFactory;
use crate::cn::wps::moffice::writer::layout::base::graph::SrcNode::{SrcNode, TYPE_SRC_CORECHARS};
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::layout::text::FontHost::{FontHost, InjectedFace};
use crate::cn::wps::moffice::writer::layout::text::FontStack::{
    pt_to_twip, FontError, FontErrorKind, FontStack,
};
use util::moffice::property::PropertySet::{empty, PropertySet};
use crate::cn::wps::moffice::writer::rendercommand::DecoCommand::{
    emit_deco, DecoStyle, UnderlineKind,
};
use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::{
    CommandDocument, DrawImage, DrawLine, DrawText, RenderCommand,
};
use crate::cn::wps::moffice::writer::rendercommand::Serialize::{from_json, to_json};
use crate::cn::wps::moffice::writer::rendercommand::TextCommand::{emit_text, TextRunEmit};
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};

/// Embedded Latin MVP face (space + A–Z a–z). Same file as test fixture `MvpLatin.ttf`.
pub const DEFAULT_MVP_FONT: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/MvpLatin.ttf"));

pub const PIPELINE_LAYOUT_V1: &str = "word-core-layout-v1";
/// ★wr-29 06 —— Java 管线产物标签（可行命令同 v1 契约，引擎为 java_pipeline_pages）
pub const PIPELINE_JAVA_V1: &str = "word-core-java-pipeline-v1";

/// IR version for `LayoutFrame` (not TypoSnapshot). 2 = baseline y + xpos.
pub const LAYOUT_FRAME_VERSION: i32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutErrorKind {
    Zip,
    Font,
    Measure,
    Serialize,
}

#[derive(Debug, Clone)]
pub struct LayoutError {
    pub kind: LayoutErrorKind,
    pub message: String,
    /// ★`54` 不变量 4：`Zip` 一档的细分触发点（其余 kind 为 `None`）。
    /// 这是「同一个 `LayoutErrorKind` 覆盖多类错」时唯一能把它们分开的依据。
    pub at: Option<String>,
}

impl LayoutError {
    pub(crate) fn zip(msg: impl Into<String>) -> Self {
        LayoutError {
            kind: LayoutErrorKind::Zip,
            message: msg.into(),
            at: None,
        }
    }
    fn measure(msg: impl Into<String>) -> Self {
        LayoutError {
            kind: LayoutErrorKind::Measure,
            message: msg.into(),
            at: None,
        }
    }
}

impl From<FontError> for LayoutError {
    fn from(e: FontError) -> Self {
        let kind = match e.kind {
            FontErrorKind::MissingGlyph => LayoutErrorKind::Measure,
            _ => LayoutErrorKind::Font,
        };
        LayoutError {
            kind,
            message: e.message,
            at: None,
        }
    }
}

impl std::fmt::Display for LayoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for LayoutError {}

/// Injected face. `id` is copied onto `DrawText.font_id`.
#[derive(Clone, Debug)]
pub struct FontBlob {
    pub id: String,
    pub bytes: Vec<u8>,
}

/// Page box and wrap/page overrides. Zoom is **not** an input (MVP: draw-only scale).
#[derive(Clone, Debug, PartialEq)]
pub struct ViewParams {
    pub page_width_twip: i32,
    pub page_height_twip: i32,
    pub margin_left_twip: i32,
    pub margin_right_twip: i32,
    pub margin_top_twip: i32,
    pub margin_bottom_twip: i32,
    /// If set, wrap uses this instead of `page − left − right`.
    pub content_width_twip: Option<i32>,
    /// If set, pagination uses this instead of `page − top − bottom`.
    pub content_height_twip: Option<i32>,
    pub font_size_pt: f64,
    pub jc: i32,
    /// Force single underline via `emit_deco` (also set when XML has `<w:u`).
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
            jc: JC_LEFT,
            underline: false,
        }
    }

    pub fn content_width(&self) -> i32 {
        match self.content_width_twip {
            Some(w) if w > 0 => w,
            _ => self.page_setup().contentWidth(),
        }
    }

    pub fn content_height(&self) -> i32 {
        match self.content_height_twip {
            Some(h) if h > 0 => h,
            _ => self.page_setup().contentHeight(),
        }
    }

    fn page_setup(&self) -> PageSetup {
        PageSetup::PageSetup(
            self.page_width_twip,
            self.page_height_twip,
            self.margin_left_twip,
            self.margin_right_twip,
            self.margin_top_twip,
            self.margin_bottom_twip,
        )
    }
}

impl Default for ViewParams {
    fn default() -> Self {
        Self::default_a4()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

impl PageMetrics {
    fn from_view(view: &ViewParams, page_count: i32) -> Self {
        PageMetrics {
            page_count: page_count.max(1),
            page_width_twip: view.page_width_twip,
            page_height_twip: view.page_height_twip,
            margin_left_twip: view.margin_left_twip,
            margin_right_twip: view.margin_right_twip,
            margin_top_twip: view.margin_top_twip,
            margin_bottom_twip: view.margin_bottom_twip,
            paper_size: "A4".into(),
        }
    }
}

/// Product IR. `commands` are page-absolute twip; `DrawText.y` = baseline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutFrame {
    pub version: i32,
    pub pipeline: String,
    pub page: PageMetrics,
    pub commands: Vec<RenderCommand>,
}

impl LayoutFrame {
    pub fn to_stage3_json(&self) -> Result<String, LayoutError> {
        to_json(&CommandDocument {
            version: self.version,
            commands: self.commands.clone(),
        })
        .map_err(|e| LayoutError {
            kind: LayoutErrorKind::Serialize,
            message: format!("json: {e}"),
            at: None,
        })
    }

    pub fn to_envelope_json(&self) -> Result<String, LayoutError> {
        #[derive(Serialize)]
        struct Envelope<'a> {
            ok: bool,
            pipeline: &'a str,
            page: &'a PageMetrics,
            document: CommandDocument,
        }
        serde_json::to_string_pretty(&Envelope {
            ok: true,
            pipeline: &self.pipeline,
            page: &self.page,
            document: CommandDocument {
                version: self.version,
                commands: self.commands.clone(),
            },
        })
        .map_err(|e| LayoutError {
            kind: LayoutErrorKind::Serialize,
            message: format!("envelope: {e}"),
            at: None,
        })
    }

    pub fn text_commands(&self) -> Vec<&DrawText> {
        self.commands
            .iter()
            .filter_map(|c| match c {
                RenderCommand::Text(t) => Some(t),
                _ => None,
            })
            .collect()
    }
}

fn empty_frame(view: &ViewParams) -> LayoutFrame {
    LayoutFrame {
        version: LAYOUT_FRAME_VERSION,
        pipeline: PIPELINE_LAYOUT_V1.into(),
        page: PageMetrics::from_view(view, 1),
        commands: Vec::new(),
    }
}

fn load_stack(fonts: &[FontBlob]) -> Result<(FontStack, String), LayoutError> {
    // Host supplies bytes (Java Platform font dirs). Empty → bundled MVP, never FontStack::default().
    let faces: Vec<InjectedFace> = if fonts.is_empty() {
        vec![InjectedFace {
            id: "default".into(),
            bytes: DEFAULT_MVP_FONT.to_vec(),
            collection_index: 0,
        }]
    } else {
        fonts
            .iter()
            .map(|f| InjectedFace {
                id: f.id.clone(),
                bytes: f.bytes.clone(),
                collection_index: 0,
            })
            .collect()
    };
    let host = FontHost::from_faces(faces)?;
    let id = host.default_font_id().to_string();
    Ok((host.into_font_stack()?, id))
}

fn join_paragraphs(paragraphs: &[String]) -> String {
    paragraphs
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join("\r")
}

fn visible_for_emit(s: &str) -> String {
    s.chars()
        .filter(|c| *c != '\r' && *c != '\n' && *c != '\u{000c}')
        .collect()
}

fn is_control_for_xpos(ch: char) -> bool {
    // Java FieldFilter consumes the field delimiters before TextOpCollector.
    // They remain in the source rope consumed by this command bridge, but do
    // not have a visible glyph or a text advance.
    matches!(ch, '\r' | '\n' | '\u{000c}' | '\u{0013}' | '\u{0014}' | '\u{0015}')
}

// Java: TableRowLayouter.adjustRowHeight / adjustMarginsForBRC.
// Non-negative row height is an at-least box. Negative row height is exact,
// and this host only grows a short row up to that box.
fn apply_java_row_box(lines: &mut [PageLine]) {
    let mut index = 0;
    while index < lines.len() {
        if lines[index].cell.is_none() {
            index += 1;
            continue;
        }
        let row_start = lines[index].cell.unwrap().row_start;
        let mut end = index;
        let mut content = 0;
        let mut margin_top = 0;
        let mut margin_bottom = 0;
        let mut top_w = 0;
        let mut bottom_w = 0;
        let mut row_height = 0;
        let mut first_row = false;
        while end < lines.len() && lines[end].cell.is_some_and(|cell| cell.row_start == row_start) {
            let cell = lines[end].cell.unwrap();
            content += lines[end].line.height.max(0);
            margin_top = margin_top.max(cell.margin_top);
            margin_bottom = margin_bottom.max(cell.margin_bottom);
            top_w = top_w.max(cell.top_w);
            bottom_w = bottom_w.max(cell.bottom_w);
            row_height = cell.row_height;
            first_row |= cell.first_row;
            end += 1;
        }
        let top_pad = if first_row { top_w } else { 0 };
        let occupied = content + margin_top + margin_bottom + bottom_w + top_pad;
        let minimum = if row_height >= 0 {
            row_height + bottom_w + top_pad
        } else {
            -row_height + top_pad
        };
        let target = occupied.max(minimum);
        if target > content {
            let last = &mut lines[end - 1].line;
            last.setHeight(last.height.saturating_add(target - content));
        }
        index = end;
    }
}

fn emit_table_borders(commands: &mut Vec<RenderCommand>, page: &PaginatedPage, doc: &TextDocument, paper: &ViewParams) {
    struct BoxAcc {
        top: i32,
        bottom: i32,
        frame: CellFrame,
    }
    let mut boxes: Vec<BoxAcc> = Vec::new();
    for pl in &page.lines {
        let Some(frame) = pl.cell else { continue };
        if frame.top_w == 0 && frame.left_w == 0 && frame.bottom_w == 0 && frame.right_w == 0 {
            continue;
        }
        let line = &pl.line;
        let setup = doc.getPageSetup(line.getStartCP());
        let line_top = setup.as_ref().map_or(paper.margin_top_twip, PageSetup::getMarginTop) + line.top;
        let bottom = line_top + line.height.max(0);
        if let Some(acc) = boxes.iter_mut().find(|acc| acc.frame.row_start == frame.row_start && acc.frame.border_left == frame.border_left) {
            acc.top = acc.top.min(line_top);
            acc.bottom = acc.bottom.max(bottom);
        } else {
            boxes.push(BoxAcc { top: line_top, bottom, frame });
        }
    }
    for acc in boxes {
        let margin_left = doc.getPageSetup(acc.frame.row_start).as_ref().map_or(paper.margin_left_twip, PageSetup::getMarginLeft);
        let x1 = margin_left + acc.frame.border_left;
        let x2 = margin_left + acc.frame.border_right;
        let color = if acc.frame.color == 0 { 0xFF00_0000 } else { acc.frame.color };
        let page_index = page.page_index;
        let push = |commands: &mut Vec<RenderCommand>, x1, y1, x2, y2, width| {
            if width <= 0 { return; }
            commands.push(RenderCommand::Line(DrawLine {
                page: page_index,
                x1,
                y1,
                x2,
                y2,
                kind: "border".into(),
                color,
                width,
            }));
        };
        push(commands, x1, acc.top, x2, acc.top, acc.frame.top_w);
        push(commands, x1, acc.bottom, x2, acc.bottom, acc.frame.bottom_w);
        push(commands, x1, acc.top, x1, acc.bottom, acc.frame.left_w);
        push(commands, x2, acc.top, x2, acc.bottom, acc.frame.right_w);
    }
}

fn is_embed_char(ch: char) -> bool {
    // Java inline picture is '\u0001'; anchored / in-dgg picture is '\u0008'.
    ch == '\u{0001}' || ch == '\u{0008}'
}

fn shape_metrics_round(value: f32) -> i32 {
    if value >= 0.0 { (value + 0.5) as i32 } else { (value - 0.5) as i32 }
}

// Java: DrawingShapeRectTool embed rect → twip, then InstractionBuilder.drawPicture/drawShape.
fn embed_draw_image(
    main: *mut crate::cn::wps::moffice::writer::core::SubDocument::SubDocument,
    page: i32,
    x: i32,
    baseline: i32,
    layout_width: i32,
    cp: i32,
    place: Option<(i32, i32, i32, i32, i32, i32, i32, i32, i32)>,
) -> Option<DrawImage> {
    if main.is_null() {
        return None;
    }
    let sub = unsafe { &mut *main };
    let shape_id = sub.mShapeAnchors.as_mut()?.findByCP(cp)?.getShapeId();
    let doc_ptr = sub.getTextDocument();
    let element = sub.drawingContainer()?.findShapeByID(shape_id)?;
    let locked = element.downcast_ref::<std::sync::Mutex<drawing::cn::wps::moffice::drawing::Shape::Shape>>()?;
    let mut shape = locked.lock().ok()?;
    let (rect_w, rect_h, rotation) = shape_layout_box(&mut shape);
    let (outer_w, outer_h) = crate::cn::wps::moffice::writer::layout::base::drawing::DrawingShapeRectTool::DrawingShapeRectTool::embed_outer_size(&mut shape);
    // Java: InstractionBuilder.drawPicture is entered only for a Picture.
    // A shape with textbox content follows TypoDrawingRender.drawShape; it
    // cannot be exported as an image with media id -1.
    let picture = shape.getPicture()?;
    // Java BlipFill.getMediaID may follow Picture.mShape back to its base Shape.
    // Release the Rust Shape mutex before that Java call sequence to preserve
    // the same object graph without self-deadlocking on the back-reference.
    drop(shape);
    let (media_id, media_path, crop_left, crop_top, crop_right, crop_bottom) = picture_media(Some(picture), doc_ptr);
    let (w, h, x, y) = if let Some((_, content_w, content_h, pad_l, pad_t, pad_r, pad_b, max_ascent, border_width)) = place {
        if std::env::var_os("WORD_PARITY_EMBED_TRACE").is_some() {
            eprintln!("EMBED_RECT shape={shape_id} page={page} cp={cp} text_left={x} baseline={baseline} content={content_w},{content_h} padding={pad_l},{pad_t},{pad_r},{pad_b} max_ascent={max_ascent} border={border_width}");
        }
        // Java: DrawingShapeRectTool.doGetEmbedRectForLine sets right to
        // textLeft + layoutPadding.left + drawing.width() + layoutPadding.right,
        // and bottom to top + drawing.height() + layoutPadding.bottom.
        // The Shape branch has already populated these padding values in
        // DrawingLayouter.setDrawingLayoutRect; keep that same rectangle at S14.
        let w = if content_w > 0 { content_w.saturating_add(pad_r) } else if layout_width > 0 { layout_width } else { rect_w };
        let h = if content_h > 0 { content_h.saturating_add(pad_b) } else if outer_h > 0 { outer_h } else { rect_h.max(0) };
        (w, h, x.saturating_add(pad_l), baseline - max_ascent + pad_t + border_width)
    } else {
        let w = if layout_width > 0 { layout_width } else if outer_w > 0 { outer_w } else { rect_w };
        let h = if outer_h > 0 { outer_h } else { rect_h.max(0) };
        (w, h, x, baseline.saturating_sub(h))
    };
    Some(DrawImage {
        page,
        x,
        y,
        w,
        h,
        shape_id,
        media_id,
        media_path,
        rotation,
        crop_left,
        crop_top,
        crop_right,
        crop_bottom,
    })
}

fn shape_layout_box(shape: &mut drawing::cn::wps::moffice::drawing::Shape::Shape) -> (i32, i32, f32) {
    let Some(pos_el) = shape.getShapePos() else { return (0, 0, 0.0); };
    let Some(pos) = pos_el.downcast_ref::<drawing::cn::wps::moffice::drawing::ShapePos::ShapePos>() else { return (0, 0, 0.0); };
    let rotation = pos.getRotation();
    let Some(rect_el) = pos.getShapeRect() else { return (0, 0, rotation); };
    let Some(rect) = rect_el.downcast_ref::<util::cn::wps::graphics::RectF::RectF>() else { return (0, 0, rotation); };
    let mut width = shape_metrics_round(rect.right * 20.0) - shape_metrics_round(rect.left * 20.0);
    let mut height = shape_metrics_round(rect.bottom * 20.0) - shape_metrics_round(rect.top * 20.0);
    let scale_h = pos.getScaleH();
    if scale_h != 1.0 { width = (width as f32 * scale_h) as i32; }
    let scale_v = pos.getScaleV();
    if scale_v != 1.0 { height = (height as f32 * scale_v) as i32; }
    (width, height, rotation)
}

fn picture_media(
    pic_el: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    doc: *mut crate::cn::wps::moffice::writer::core::TextDocument::TextDocument,
) -> (i32, String, f32, f32, f32, f32) {
    let Some(pic_el) = pic_el else { return (-1, String::new(), 0.0, 0.0, 0.0, 0.0); };
    let Some(pic) = pic_el.downcast_ref::<std::sync::Mutex<drawing::cn::wps::moffice::drawing::graphics::Picture::Picture>>() else {
        return (-1, String::new(), 0.0, 0.0, 0.0, 0.0);
    };
    let Ok(mut pic) = pic.lock() else { return (-1, String::new(), 0.0, 0.0, 0.0, 0.0); };
    let media_id = pic.getMediaID();
    let crop = (pic.getCropLeft(), pic.getCropTop(), pic.getCropRight(), pic.getCropBottom());
    drop(pic);
    let path = if media_id < 0 || doc.is_null() {
        String::new()
    } else {
        unsafe { &*doc }.mMediaLib.as_ref().map(|lib| {
            lib.getMediaByID_impl(media_id, drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::MediaTypeEnum::PICTURE)
        }).unwrap_or_default()
    };
    (media_id, path, crop.0, crop.1, crop.2, crop.3)
}

#[derive(Clone)]
struct MeasuredSrc {
    start_cp: i32,
    end_cp: i32,
    text: String,
    advances: Vec<i32>,
    ascent: i32,
    descent: i32,
    style: TypoFontStyle,
    papx: PropertySet,
}

fn slice_measured(src: &MeasuredSrc, char_start: usize, char_end: usize) -> MeasuredSrc {
    let chs: Vec<char> = src.text.chars().collect();
    let text: String = chs[char_start..char_end].iter().collect();
    let advances = src.advances[char_start..char_end].to_vec();
    MeasuredSrc {
        start_cp: src.start_cp + char_start as i32,
        end_cp: src.start_cp + char_end as i32,
        text,
        advances,
        ascent: src.ascent,
        descent: src.descent,
        style: src.style.clone(),
        papx: src.papx.clone(),
    }
}

fn split_into_paras(runs: Vec<MeasuredSrc>) -> Vec<Vec<MeasuredSrc>> {
    let mut paras: Vec<Vec<MeasuredSrc>> = Vec::new();
    let mut cur: Vec<MeasuredSrc> = Vec::new();
    for run in runs {
        let chs: Vec<char> = run.text.chars().collect();
        let mut piece_start = 0usize;
        for i in 0..chs.len() {
            if chs[i] == '\r' {
                cur.push(slice_measured(&run, piece_start, i + 1));
                paras.push(std::mem::take(&mut cur));
                piece_start = i + 1;
            }
        }
        if piece_start < chs.len() {
            cur.push(slice_measured(&run, piece_start, chs.len()));
        }
    }
    if !cur.is_empty() {
        paras.push(cur);
    }
    paras
}

fn lang_type_for(text: &str) -> i32 {
    if text.chars().any(is_east_asia) {
        2
    } else {
        0
    }
}

fn bind_style_faces(style: &mut TypoFontStyle, stack: &FontStack, default_id: &str) {
    if !stack.name_hits_file(&style.ascii) {
        style.ascii = default_id.to_string();
    }
    if !stack.name_hits_file(&style.eastAsia) {
        style.eastAsia = if stack.name_hits_file("宋体") {
            "宋体".to_string()
        } else {
            default_id.to_string()
        };
    }
    if !stack.name_hits_file(&style.fontName) {
        style.fontName = style.ascii.clone();
    }
}

fn measure_for_layout(
    stack: &FontStack,
    style: &TypoFontStyle,
    text: &str,
) -> Result<RunMetrics, FontError> {
    let size_pt = (style.fontSize as f64) / 20.0;
    let first_face = if text.chars().any(|c| !c.is_control()) {
        FaceSelect::selectFace(style, text.chars().find(|c| !c.is_control()).unwrap())
    } else if !style.ascii.is_empty() {
        style.ascii.as_str()
    } else {
        style.fontName.as_str()
    };
    let (ascent, descent, line_gap) = stack.line_box_twip(Some(first_face), size_pt)?;
    let descent = descent.abs();
    if text.is_empty() {
        return Ok(RunMetrics {
            advances: Vec::new(),
            width: 0,
            ascent,
            descent,
            line_gap,
        });
    }
    let mut advances = Vec::new();
    let mut width = 0i32;
    for ch in text.chars() {
        if ch.is_control() {
            advances.push(0);
            continue;
        }
        let face = FaceSelect::selectFace(style, ch);
        let adv = stack.glyph_advance_twip(Some(face), ch, size_pt)?;
        advances.push(adv);
        width = width.saturating_add(adv);
    }
    Ok(RunMetrics {
        advances,
        width,
        ascent,
        descent,
        line_gap,
    })
}

fn style_from_source(
    run: &LayoutSourceRun,
    view: &ViewParams,
    styles: Option<&crate::cn::wps::moffice::writer::core::KStyles::KStyles>,
) -> TypoFontStyle {
    let merged = crate::cn::wps::moffice::writer::layout::base::env::StyleMerge::StyleMerge::merge(
        &run.chpx,
        &run.papx,
        styles,
    );
    let mut style = FontStyleFactory::createFromRun(&run.chpx, &run.papx, styles, lang_type_for(&run.text));
    // Direct `w:sz` / named pStyle (sti>0) win. sti 0 is Normal filler from
    // missing w:pStyle and must not block ViewParams (Hello 12pt goldens).
    let has_direct_sz = run.chpx.contains(10);
    let pstyle_id = run.papx.getInt__int_int(191, -1);
    let has_named_pstyle = pstyle_id > 0;
    let merged_sz = merged.getFloat__int_float(10, -1.0);
    if !has_direct_sz && (!has_named_pstyle || merged_sz <= 0.0) {
        let pt = if view.font_size_pt > 0.0 {
            view.font_size_pt
        } else {
            12.0
        };
        style.fontSize = pt_to_twip(pt);
    }
    style
}

fn para_papx_from(ps: &PropertySet, view: &ViewParams, prev_after: i32) -> ParaPapx {
    ParaPapx {
        jc: ps.getInt__int_int(193, view.jc),
        first_line_indent: ps.getInt__int_int(208, 0),
        spacing: LineSpacing::auto(1.0),
        para_before: ps.getInt__int_int(217, 0),
        para_after: ps.getInt__int_int(214, 0),
        prev_para_after: prev_after,
    }
}

/// Java: ParaLayouter.calcFinalPropertySet uses `papxTable.seek(cp)`, then
/// `para.position()` and `nextPara.position()` for the paragraph read window.
/// Section breaks and field/control characters need not be a `\r` in the run
/// stream; searching rendered text can merge two adjacent PAPX paragraphs.
fn para_cp_bounds(main: &dyn Document, cp: i32, doc_len: i32) -> Result<(i32, i32), LayoutError> {
    let table = main.getPapxTable().ok_or_else(|| LayoutError::measure("missing PAPX table"))?;
    let para = unsafe { IPapxTable::seek(&*table, cp) }
        .ok_or_else(|| LayoutError::measure(format!("missing PAPX entry at CP {cp}")))?;
    let start = para.position();
    let next = crate::cn::wps::moffice::writer::data::IPapxTable::Entry::getNext(para.as_ref())
        .filter(|e| !e.isHead());
    let end = next.map(|e| e.position()).unwrap_or(doc_len);
    Ok((start, end.min(doc_len).max(cp + 1)))
}

// Java: GraphReaderTool.isInTable then TableInfos.getTableInfoByLevel and the cell rgdxa box.
fn table_cell_frame(main: &crate::cn::wps::moffice::writer::core::SubDocument::SubDocument, cp: i32) -> Option<CellFrame> {
    use crate::cn::wps::moffice::writer::data::IPapxTable::Entry;
    use crate::cn::wps::moffice::writer::data::table::TableInfos::TableInfos;
    use crate::cn::wps::moffice::writer::layout::base::graph::GraphReaderTool::GraphReaderTool;
    let papx = main.getPapxTable()?;
    let para = unsafe { IPapxTable::seek(&*papx, cp) }?;
    let prop = Entry::getProperty(para.as_ref())?;
    if !GraphReaderTool::isInTable(prop, 1) {
        return None;
    }
    let level = prop.getInt__int_int(224, 1).max(1);
    let mut table = main.getTableInfos()?.getTableInfoByLevel(cp, level)?;
    let cell = table.cell_layout_box(cp)?;
    if cell.border_right <= cell.border_left {
        return None;
    }
    Some(CellFrame {
        text_dx: cell.text_dx,
        border_left: cell.border_left,
        border_right: cell.border_right,
        content_width: cell.content_width,
        row_start: cell.row_start,
        row_height: cell.row_height,
        first_row: cell.first_row,
        margin_top: cell.margin_top,
        margin_bottom: cell.margin_bottom,
        top_w: cell.top_w,
        left_w: cell.left_w,
        bottom_w: cell.bottom_w,
        right_w: cell.right_w,
        color: cell.color,
    })
}

fn source_from_paragraphs(paragraphs: &[String]) -> Vec<LayoutSourceRun> {
    let mut cp = 0i32;
    let mut out = Vec::new();
    for p in paragraphs {
        let mut text = p.clone();
        text.push('\r');
        let n = text.chars().count() as i32;
        out.push(LayoutSourceRun {
            start_cp: cp,
            end_cp: cp + n,
            text,
            chpx: empty.clone(),
            papx: empty.clone(),
        });
        cp += n;
    }
    out
}

/// Java sticky: once a slice needs a fallback face, the whole slice uses that `font_id` for paint.
fn font_id_for_text(stack: &FontStack, text: &str, default_id: &str) -> String {
    for ch in text.chars() {
        if is_control_for_xpos(ch) {
            continue;
        }
        if stack.has_glyph(None, ch) {
            continue;
        }
        if let Some(id) = stack.face_id_for_char(None, ch) {
            return id;
        }
    }
    default_id.to_string()
}

fn xml_has_underline(xml: &str) -> bool {
    xml.contains("<w:u")
}

/// Inner I/O: paragraph strings already extracted. Does not parse zip.
/// Builds wr-09-shaped runs (empty CHPX/PAPX) then [layout_layout_runs].
pub fn layout_source_runs(
    paragraphs: &[String],
    fonts: &[FontBlob],
    view: &ViewParams,
) -> Result<LayoutFrame, LayoutError> {
    if join_paragraphs(paragraphs).is_empty() {
        return Ok(empty_frame(view));
    }
    layout_layout_runs_with_styles(&source_from_paragraphs(paragraphs), fonts, view, None)
        .map(|(frame, _)| frame)
}

/// ★wr-29 06 —— Java 管线宿主：doc → GraphBuffFactory.createGraphBuff → GraphReader.prepareGraphs
/// 读 run → GraphFactory.createNodes 量宽 → TextLayouter.doFlowTextLine → LineLayouter.flow
/// → 镜像行 → PageLine → paginate → PaginatedPage（产品信封装帧）。
/// rust: Java GraphBuffFactory 返回 GraphFactory（IS-A GraphBuff）；createNodes 在 Java 由
///       GraphReader 虚分派，Rust 无虚分派 → 宿主显式遍历 charsList 触发（GR-D2）。
pub fn java_pipeline_pages(
    doc: &TextDocument,
    fonts: &[FontBlob],
    view: &ViewParams,
) -> Result<Vec<PaginatedPage>, LayoutError> {
    // ① 字体表（Java FontHost.getFontTable 对等）
    let (stack, _id) = load_stack(fonts)?;
    crate::cn::wps::moffice::writer::layout::text::FontHost::set_font_table(std::sync::Arc::new(stack));
    // ② 文档主故事（+ doc 保活，03 悬垂修复同款）
    let Some(main) = doc.getMainDocument() else {
        return Ok(Vec::new());
    };
    let doc_len: i32 = main.getLength();
    if doc_len <= 0 {
        return Ok(Vec::new());
    }
    // Java page view sizes the page from the section page setup, not the host view envelope.
    let _ = view;
    let runs = RunStream::collectMainStoryRuns(main);
    let concat: String = runs.iter().map(|r| r.text.as_str()).collect();
    // Java FlowLayout.createDocLayouter builds PageViewLayouter. Its constructor
    // runs TextLayouter.init, which calls initLayouters. flowDocument then calls flowPage.
    // java FlowLayout.initEnv: mFontStyleCacheFactory = new FontStyleCacheFactory();
    // java FlowLayout.initEnv: mDocEnv = new DocumentEnv(..., mFontStyleCacheFactory, ...);
    let mut doc_env = DocumentEnv::default();
    doc_env.fontStyleCacheFactory = Some(
        crate::cn::wps::moffice::writer::layout::base::env::FontStyleCacheFactory::FontStyleCacheFactory::default(),
    );
    let mut layouter = crate::cn::wps::moffice::writer::layout::pageview::PageViewLayouter::PageViewLayouter::PageViewLayouter(
        MainState::default(),
        doc_env,
        ViewEnv::default(),
    );
    layouter.bind_open_document(doc);
    layouter.prepareFlowFromDocumentBegin();
    let mut out_pages: Vec<PaginatedPage> = Vec::new();
    let mut page_index: i32 = 0;
    let mut safety: i32 = 0;
    while safety < 10000 {
        safety += 1;
        if layouter.document_flow_complete() {
            break;
        }
        let before_cp = layouter.layout_cp();
        crate::cn::wps::moffice::writer::layout::base::LayoutTraceCollector::LayoutTraceCollector::setActivePageIndex(page_index);
        layouter.flowPage();
        let (flowed, advances, embeds, table_cells) = layouter.flowed_lines();
        let snapshot_table_cells = layouter.snapshot_table_cells();
        let had_lines = !flowed.is_empty();
        if had_lines {
            let mut page_lines: Vec<PageLine> = Vec::new();
            for (idx, line) in flowed.into_iter().enumerate() {
                let a = line.getStartCP();
                let b = line.getEndCP();
                let slice: String = concat
                    .chars()
                    .skip(a.max(0) as usize)
                    .take((b - a).max(0) as usize)
                    .collect();
                let force_page_after = slice.contains('\u{000c}');
                let layout_advances = advances.get(idx).cloned().unwrap_or_default();
                let line_embeds = embeds.iter().copied().filter(|place| place.0 >= a && place.0 < b).collect();
                page_lines.push(PageLine {
                    line,
                    layout_advances,
                    para_id: 0,
                    widow_control: !force_page_after,
                    force_page_after,
                    cell: table_cells.get(idx).copied().flatten(),
                    embeds: line_embeds,
                });
            }
            out_pages.push(PaginatedPage { page_index, lines: page_lines, table_cells: snapshot_table_cells });
            page_index += 1;
        }
        if layouter.layout_cp() == before_cp && !had_lines {
            break;
        }
    }
    Ok(out_pages)
}

/// ★wr-29 06 —— Java 栈 Frame：java_pipeline_pages（行）+ 度量 runs → RenderCommand
/// （渲染契约与 Stage2 对齐：margin/基线同公式；pipeline 标签区分引擎）
pub fn java_layout_text_document(
    doc: &TextDocument,
    fonts: &[FontBlob],
    view: &ViewParams,
) -> Result<LayoutFrame, LayoutError> {
    let pages = java_pipeline_pages(doc, fonts, view)?;
    if pages.is_empty() {
        return Ok(empty_frame(view));
    }
    let Some(main) = doc.getMainDocument() else {
        return Ok(empty_frame(view));
    };
    // Java FieldFilter consumes EMBED field instructions before TypoRun/TextOpCollector.
    // This command bridge still reads the source rope; carry that same hidden
    // code interval into emission until PageRender consumes TypoRun directly.
    let hidden_embed_codes: Vec<(i32, i32)> = main.getPLCField().map(|plc| {
        plc.getFieldTriples().into_iter()
            .filter(|field| field.getFieldType() == crate::cn::wps::moffice::writer::data::field::KFieldType::KFieldType::FieldEmbed.getFlt())
            .filter(|field| field.hasSep())
            .map(|field| (field.getStartCP() + 1, field.getSepCP() + 1))
            .collect()
    }).unwrap_or_default();
    let source = RunStream::collectMainStoryRuns(main);
    let main_ptr = main as *const crate::cn::wps::moffice::writer::core::SubDocument::SubDocument
        as *mut crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
    if source.is_empty() {
        return Ok(empty_frame(view));
    }
    let (stack, font_id) = load_stack(fonts)?;
    // 度量（与 Stage2 同栈同公式 —— 渲染面独立于布局引擎）
    let mut measured: Vec<MeasuredSrc> = Vec::new();
    for run in source.iter().cloned() {
        // Java GraphReader/PropertyTool resolves paragraph style before
        // TextOpCollector emits glyphs; pass the parsed style table here too.
        let mut style = style_from_source(&run, view, doc.getStyles());
        bind_style_faces(&mut style, &stack, &font_id);
        let metrics = measure_for_layout(&stack, &style, &run.text)
            .map_err(|e| LayoutError::measure(format!("measure_run_metrics: {e}")))?;
        measured.push(MeasuredSrc {
            start_cp: run.start_cp,
            end_cp: run.end_cp,
            text: run.text,
            advances: metrics.advances,
            ascent: metrics.ascent,
            descent: metrics.descent,
            style,
            papx: run.papx,
        });
    }
    let page_count = pages.len().max(1) as i32;
    let mut paper_view = view.clone();
    if let Some(setup) = doc.getPageSetup(0) {
        paper_view.page_width_twip = setup.getWidth();
        paper_view.page_height_twip = setup.getHeight();
        paper_view.margin_left_twip = setup.getMarginLeft();
        paper_view.margin_right_twip = setup.getMarginRight();
        paper_view.margin_top_twip = setup.getMarginTop();
        paper_view.margin_bottom_twip = setup.getMarginBottom();
    }
    let mut commands = Vec::new();
    for page in &pages {
        for pl in &page.lines {
            let line = &pl.line;
            let ls = line.getStartCP();
            let le = line.getEndCP();
            // Java page/section margin comes from the SEPX at this line CP.
            // The command bridge uses the same model source as the page layouter.
            let line_setup = doc.getPageSetup(ls);
            let cell_dx = pl.cell.as_ref().map(|cell| cell.text_dx).unwrap_or(0);
            let x_line = line_setup.as_ref().map_or(paper_view.margin_left_twip, PageSetup::getMarginLeft) + line.getLeft() + cell_dx;
            let line_top = line_setup.as_ref().map_or(paper_view.margin_top_twip, PageSetup::getMarginTop) + line.top;
            // Java: LineHeightLayouter.calcBaseLinePos computes
            // mAscentPos = mBaselinePos - mMaxAscent. The TypoTextLine field
            // exposed by getAscentPos() is that offset, not the text baseline.
            // Java: AlignBaselineHandler.adjustRun sets run.baseLine to
            // mAlignLinePos + fontStyle.hpsPos; TextOpCollector emits it.
            let text_max_ascent = measured.iter()
                .filter(|src| src.start_cp < le && src.end_cp > ls)
                .map(|src| src.ascent)
                .max()
                .unwrap_or(0);
            // Java LineHeightLayouter includes embed Graph fontStyle.maxAscent
            // when it computes the common line baseline. DrawingShapeRectTool
            // then subtracts that same ascent from TypoText.baseline. The
            // command bridge must include the embed's laid-out outer height;
            // using only the measured text runs places a tall picture above
            // its line even when the Stage2 line box itself is correct.
            let embed_max_ascent = pl.embeds.iter()
                .filter(|place| place.0 >= ls && place.0 < le)
                .map(|place| place.7)
                .max()
                .unwrap_or(0);
            let max_ascent = text_max_ascent.max(embed_max_ascent);
            let baseline = line_top + line.getAscentPos() + max_ascent;
            let mut line_advances = std::collections::BTreeMap::<i32, i32>::new();
            for &(cp, advance) in &pl.layout_advances {
                *line_advances.entry(cp).or_default() += advance;
            }
            let mut run_x = 0i32;
            for src in &measured {
                let a = src.start_cp.max(ls);
                let b = src.end_cp.min(le);
                if b <= a {
                    continue;
                }
                let local_s = (a - src.start_cp) as usize;
                let local_e = (b - src.start_cp) as usize;
                let raw = slice_measured(src, local_s, local_e);
                let origin = x_line + run_x;
                let mut xpos = Vec::new();
                let mut gx = origin;
                let mut vis = String::new();
                for (offset, (ch, adv)) in raw.text.chars().zip(raw.advances.iter()).enumerate() {
                    if is_control_for_xpos(ch) {
                        continue;
                    }
                    let cp = a + offset as i32;
                    if hidden_embed_codes.iter().any(|&(start, end)| start <= cp && cp < end) {
                        continue;
                    }
                    let layout_advance = line_advances.get(&cp).copied().unwrap_or(*adv);
                    // Java table cell terminator is consumed by GraphFactory
                    // for line width but has no visible TypoText glyph in
                    // PageRender/TypoTextRender. Keep its advance for later CPs.
                    if ch == '\u{0007}' {
                        gx = gx.saturating_add(layout_advance);
                        continue;
                    }
                    if is_embed_char(ch) {
                        if let Some(image) = embed_draw_image(main_ptr, page.page_index, gx, baseline, layout_advance, cp, pl.embeds.iter().find(|place| place.0 == cp).copied()) {
                            commands.push(RenderCommand::Image(image));
                        }
                        gx = gx.saturating_add(layout_advance);
                        continue;
                    }
                    xpos.push(gx);
                    vis.push(ch);
                    gx = gx.saturating_add(layout_advance);
                }
                if !vis.is_empty() {
                    commands.push(emit_text(&TextRunEmit {
                        page: page.page_index,
                        line_top: baseline,
                        align_x: x_line,
                        run_x,
                        text: vis.clone(),
                        font_id: font_id_for_text(&stack, &vis, &font_id),
                        size: src.style.fontSize,
                        color: 0,
                        bold: src.style.bold,
                        xpos,
                    }));
                }
                run_x = gx.saturating_sub(x_line);
            }
        }
        if !page.table_cells.is_empty() && TableRender::supports(&page.table_cells) {
            // Java PageRender draws the committed cell snapshot in the page's
            // current section coordinate system. The document's first page
            // margin can differ from the section containing this table.
            let table_setup = doc.getPageSetup(page.table_cells[0].row_start_cp);
            let page_x = table_setup.as_ref().map_or(paper_view.margin_left_twip, PageSetup::getMarginLeft);
            let page_y = table_setup.as_ref().map_or(paper_view.margin_top_twip, PageSetup::getMarginTop);
            PageRender::drawLines(page, page_x, page_y, &mut commands);
        } else {
            emit_table_borders(&mut commands, page, doc, &paper_view);
        }
    }
    Ok(LayoutFrame {
        version: LAYOUT_FRAME_VERSION,
        // rust: 06 默认栈 —— 命令格式契约同 V1（引擎为 Java 管线；格式未变不换标签）
        pipeline: PIPELINE_LAYOUT_V1.into(),
        page: PageMetrics::from_view(&paper_view, page_count),
        commands,
    })
}

/// Relayout a mutable document model. Editing must mutate [TextDocument] then call this.
pub fn layout_text_document(
    doc: &TextDocument,
    fonts: &[FontBlob],
    view: &ViewParams,
) -> Result<LayoutFrame, LayoutError> {
    // rust: ★wr-29 06 —— 默认 Java 管线栈（Stage2 路径保留在 layout_layout_runs_with_styles/layout_text_document_stage2 供比对）
    java_layout_text_document(doc, fonts, view)
}

/// Bytes → owned [TextDocument] (rope / CHPX / PAPX). Heap-stable for edits.
pub fn open_text_document(docx: &[u8]) -> Result<Box<TextDocument>, LayoutError> {
    let xml = read_document_xml(docx)?;
    crate::cn::wps::moffice::writer::api::DocumentOpen::open_boxed_text_document_from_docx(
        docx, &xml,
    )
}

/// wr-10..15 on an already-collected [LayoutSourceRun] stream (not wr-04 wrap).
/// Run `text` is rope `char` (Unicode scalar) slices; `start_cp`/`end_cp` use the
/// same index space as `ITextRope::length` (not UTF-16 code units).
pub fn layout_layout_runs(
    source: &[LayoutSourceRun],
    fonts: &[FontBlob],
    view: &ViewParams,
) -> Result<LayoutFrame, LayoutError> {
    Ok(layout_layout_runs_with_styles(source, fonts, view, None)?.0)
}

/// Live Stage2 rows from the same paginate() used by [layout_document]. Not a frozen JSON file.
pub fn layout_document_stage2(
    docx: &[u8],
    fonts: &[FontBlob],
    view: &ViewParams,
) -> Result<Vec<Stage2Line>, LayoutError> {
    let xml = read_document_xml(docx)?;
    let mut view = view.clone();
    if xml_has_underline(&xml) {
        view.underline = true;
    }
    let mut doc = Box::new(TextDocument::TextDocument__boolean(true));
    doc._create();
    crate::cn::wps::moffice::writer::api::DocumentOpen::fill_text_document_from_docx(
        &mut doc, docx, &xml,
    )?;
    layout_text_document_stage2(&doc, fonts, &view)
}

pub fn layout_text_document_stage2(
    doc: &TextDocument,
    fonts: &[FontBlob],
    view: &ViewParams,
) -> Result<Vec<Stage2Line>, LayoutError> {
    let Some(main) = doc.getMainDocument() else {
        return Ok(Vec::new());
    };
    let runs = RunStream::collectMainStoryRuns(main);
    if runs.is_empty() {
        return Ok(Vec::new());
    }
    Ok(layout_layout_runs_with_styles(&runs, fonts, view, doc.getStyles())?.1)
}

fn layout_layout_runs_with_styles(
    source: &[LayoutSourceRun],
    fonts: &[FontBlob],
    view: &ViewParams,
    styles: Option<&crate::cn::wps::moffice::writer::core::KStyles::KStyles>,
) -> Result<(LayoutFrame, Vec<Stage2Line>), LayoutError> {
    let (stack, font_id) = load_stack(fonts)?;
    if source.is_empty() {
        return Ok((empty_frame(view), Vec::new()));
    }

    let mut measured: Vec<MeasuredSrc> = Vec::new();
    for run in source.iter().cloned() {
        let mut style = style_from_source(&run, view, styles);
        bind_style_faces(&mut style, &stack, &font_id);
        let metrics = measure_for_layout(&stack, &style, &run.text)
            .map_err(|e| LayoutError::measure(format!("measure_run_metrics: {e}")))?;
        measured.push(MeasuredSrc {
            start_cp: run.start_cp,
            end_cp: run.end_cp,
            text: run.text,
            advances: metrics.advances,
            ascent: metrics.ascent,
            descent: metrics.descent,
            style,
            papx: run.papx,
        });
    }

    let content_w = view.content_width();
    let content_h = view.content_height();
    let paras = split_into_paras(measured.clone());
    let mut page_lines: Vec<PageLine> = Vec::new();
    let mut prev_after = 0i32;
    for (para_id, para_runs) in paras.iter().enumerate() {
        if para_runs.is_empty() {
            continue;
        }
        let papx = para_papx_from(&para_runs[0].papx, view, prev_after);
        prev_after = papx.para_after;
        let left_indent = para_runs[0].papx.getInt__int_int(210, 0);
        let right_indent = para_runs[0].papx.getInt__int_int(212, 0);
        let para_w = ContentWidth::paraMaxWidth(content_w, left_indent, right_indent);
        let layout_runs: Vec<LineLayoutRun> = para_runs
            .iter()
            .map(|r| LineLayoutRun {
                text: r.text.clone(),
                advances: r.advances.clone(),
                ascent: r.ascent,
                descent: r.descent,
            })
            .collect();
        let origin = para_runs[0].start_cp;
        let mut lines = layout_lines(&layout_runs, para_w, &papx);
        for line in &mut lines {
            line.setStartCP(line.getStartCP() + origin);
            line.setEndCP(line.getEndCP() + origin);
        }
        let concat: String = para_runs.iter().map(|r| r.text.as_str()).collect();
        for line in lines {
            let a = line.getStartCP() - origin;
            let b = line.getEndCP() - origin;
            let slice: String = concat
                .chars()
                .skip(a.max(0) as usize)
                .take((b - a).max(0) as usize)
                .collect();
            page_lines.push(PageLine {
                line,
                layout_advances: Vec::new(),
                para_id: para_id as i32,
                widow_control: para_runs[0].papx.getBoolean__int_boolean(197, true),
                force_page_after: slice.contains('\u{000c}'),
                cell: None,
                embeds: Vec::new(),
            });
        }
    }

    let pages = paginate(page_lines, content_h);
    let page_count = pages.len().max(1) as i32;
    let mut commands = Vec::new();
    for page in &pages {
        for pl in &page.lines {
            let line = &pl.line;
            let ls = line.getStartCP();
            let le = line.getEndCP();
            let x_line = view.margin_left_twip + line.getLeft() + pl.cell.as_ref().map(|cell| cell.text_dx).unwrap_or(0);
            let line_top = view.margin_top_twip + line.getYPos();
            // Java: LineHeightLayouter.calcBaseLinePos stores
            // mAscentPos = mBaselinePos - mMaxAscent, while
            // AlignBaselineHandler.adjustRun writes the actual run baseline.
            let max_ascent = measured.iter()
                .filter(|src| src.start_cp < le && src.end_cp > ls)
                .map(|src| src.ascent)
                .max()
                .unwrap_or(0);
            let baseline = line_top + line.getAscentPos() + max_ascent;
            let mut run_x = 0i32;
            for src in &measured {
                let a = src.start_cp.max(ls);
                let b = src.end_cp.min(le);
                if b <= a {
                    continue;
                }
                let local_s = (a - src.start_cp) as usize;
                let local_e = (b - src.start_cp) as usize;
                let raw = slice_measured(src, local_s, local_e);
                let vis = visible_for_emit(&raw.text);
                if vis.is_empty() {
                    continue;
                }
                let origin = x_line + run_x;
                let mut xpos = Vec::new();
                let mut gx = origin;
                for (ch, adv) in raw.text.chars().zip(raw.advances.iter()) {
                    if is_control_for_xpos(ch) {
                        continue;
                    }
                    xpos.push(gx);
                    gx = gx.saturating_add(*adv);
                }
                let slice_w: i32 = raw
                    .text
                    .chars()
                    .zip(raw.advances.iter())
                    .filter(|(ch, _)| !is_control_for_xpos(*ch))
                    .map(|(_, adv)| *adv)
                    .sum();
                commands.push(emit_text(&TextRunEmit {
                    page: page.page_index,
                    line_top: baseline,
                    align_x: x_line,
                    run_x,
                    text: vis.clone(),
                    font_id: font_id_for_text(&stack, &vis, &font_id),
                    size: src.style.fontSize,
                    color: 0,
                    bold: src.style.bold,
                    xpos,
                }));
                if view.underline || src.style.underline != 0 {
                    commands.extend(emit_deco(&DecoStyle {
                        underline: UnderlineKind::Single,
                        strike: false,
                        bg_color: None,
                        page: page.page_index,
                        x: origin,
                        y: line_top,
                        w: slice_w,
                        h: line.getTextRectHeight(),
                        baseline,
                        line_color: 0,
                    }));
                }
                run_x = run_x.saturating_add(slice_w);
            }
        }
    }
    Ok((
        LayoutFrame {
            version: LAYOUT_FRAME_VERSION,
            pipeline: PIPELINE_LAYOUT_V1.into(),
            page: PageMetrics::from_view(view, page_count),
            commands,
        },
        rows_from_pages(&pages),
    ))
}

/// Product I/O: docx → TextDocument → RunStream → `LayoutFrame`.
pub fn layout_document(
    docx: &[u8],
    fonts: &[FontBlob],
    view: &ViewParams,
) -> Result<LayoutFrame, LayoutError> {
    let xml = read_document_xml(docx)?;
    let mut view = view.clone();
    if xml_has_underline(&xml) {
        view.underline = true;
    }
    let mut doc = Box::new(TextDocument::TextDocument__boolean(true));
    doc._create();
    // Heap-stable: SAX DocumentImporter stores *mut TextDocument (same as open_text_document).
    crate::cn::wps::moffice::writer::api::DocumentOpen::fill_text_document_from_docx(
        &mut doc, docx, &xml,
    )?;
    layout_text_document(&doc, fonts, &view)
}

pub fn extract_paragraphs_from_docx(bytes: &[u8]) -> Result<Vec<String>, LayoutError> {
    let xml = read_document_xml(bytes)?;
    Ok(extract_paragraphs(&xml))
}

/// ★`54` 不变量 4：`LayoutErrorKind::Zip` 只有一个档，却覆盖 3 类错
/// ⇒ 用 `at` **四分**（本函数是这 3 个 `at` 的**唯一真源**，供 `54` 分类复用）：
///
/// | `at` | 触发条件 | 期望 `kind` |
/// |---|---|---|
/// | `…read_document_xml#PK_header` | 前 2 字节不是 `PK` | `not_zip` |
/// | `…read_document_xml#ZipArchive_new` | `ZipArchive::new` 失败 | `zip_corrupt` |
/// | `…read_document_xml#by_name` | 找不到 `word/document.xml` | `missing_main_part` |
/// | `…read_document_xml#read` | 读出/解码失败 | `zip_corrupt` |
pub const AT_READ_DOCUMENT_XML_PK_HEADER: &str =
    "word::moffice::writer::api::LayoutDocument::read_document_xml#PK_header";
pub const AT_READ_DOCUMENT_XML_ZIP_NEW: &str =
    "word::moffice::writer::api::LayoutDocument::read_document_xml#ZipArchive_new";
pub const AT_READ_DOCUMENT_XML_BY_NAME: &str =
    "word::moffice::writer::api::LayoutDocument::read_document_xml#by_name";
pub const AT_READ_DOCUMENT_XML_READ: &str =
    "word::moffice::writer::api::LayoutDocument::read_document_xml#read";

/// 带 `at` 的 `LayoutError::Zip`（不变量 4：一档靠 `at` 细分）。
pub(crate) fn zip_at(at: &str, msg: impl Into<String>) -> LayoutError {
    LayoutError {
        kind: LayoutErrorKind::Zip,
        message: msg.into(),
        at: Some(at.to_string()),
    }
}

fn read_document_xml(bytes: &[u8]) -> Result<String, LayoutError> {
    if bytes.len() < 4 || &bytes[0..2] != b"PK" {
        return Err(zip_at(AT_READ_DOCUMENT_XML_PK_HEADER, "not a zip/docx"));
    }
    let cursor = Cursor::new(bytes);
    let mut zip = zip::ZipArchive::new(cursor)
        .map_err(|e| zip_at(AT_READ_DOCUMENT_XML_ZIP_NEW, format!("zip: {e}")))?;
    let mut xml = String::new();
    {
        let mut f = zip
            .by_name("word/document.xml")
            .map_err(|e| zip_at(AT_READ_DOCUMENT_XML_BY_NAME, format!("document.xml: {e}")))?;
        f.read_to_string(&mut xml)
            .map_err(|e| zip_at(AT_READ_DOCUMENT_XML_READ, format!("read: {e}")))?;
    }
    Ok(xml)
}

pub(crate) fn extract_paragraphs(xml: &str) -> Vec<String> {
    let mut paras = Vec::new();
    let mut rest = xml;
    loop {
        let Some(idx) = rest.find("<w:p") else {
            break;
        };
        let tail = &rest[idx + 4..];
        if !(tail.starts_with('>')
            || tail.starts_with(' ')
            || tail.starts_with('\n')
            || tail.starts_with('\r')
            || tail.starts_with('/'))
        {
            rest = tail;
            continue;
        }
        let gt = match tail.find('>') {
            Some(i) => i,
            None => break,
        };
        let after_open = &tail[gt + 1..];
        if tail.starts_with('/') {
            paras.push(String::new());
            rest = after_open;
            continue;
        }
        let Some(end) = after_open.find("</w:p>") else {
            break;
        };
        let inner = &after_open[..end];
        paras.push(extract_wt(inner).concat());
        rest = &after_open[end + 6..];
    }
    paras
}

fn extract_wt(xml: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(s) = rest.find("<w:t") {
        rest = &rest[s..];
        let gt = match rest.find('>') {
            Some(i) => i,
            None => break,
        };
        rest = &rest[gt + 1..];
        if let Some(end) = rest.find("</w:t>") {
            let t = rest[..end].to_string();
            if !t.is_empty() {
                out.push(unescape_xml(&t));
            }
            rest = &rest[end + 6..];
        } else {
            break;
        }
    }
    out
}

fn unescape_xml(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

/// Derived Stage3 document for old tests / goldens.
pub fn command_document_from_frame(frame: &LayoutFrame) -> CommandDocument {
    CommandDocument {
        version: frame.version,
        commands: frame.commands.clone(),
    }
}

pub fn stage3_roundtrip(frame: &LayoutFrame) -> Result<CommandDocument, LayoutError> {
    let json = frame.to_stage3_json()?;
    from_json(&json).map_err(|e| LayoutError {
        kind: LayoutErrorKind::Serialize,
        message: format!("from_json: {e}"),
        at: None,
    })
}

#[cfg(test)]
mod papx_boundary_tests {
    use super::*;

    #[test]
    fn section_break_uses_papx_entries_for_two_adjacent_paragraphs() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../samples/docx-mvp-render/flutter/assets/sample2/机械制造与自动化毕业论文.docx");
        let bytes = std::fs::read(path).expect("complex sample");
        let mut doc = TextDocument::TextDocument__boolean(true);
        doc._create();
        crate::cn::wps::moffice::writer::api::DocumentOpen::fill_text_document_from_docx(&mut doc, &bytes, "")
            .expect("import");
        let main = doc.getMainDocument().expect("main story");
        assert_eq!(para_cp_bounds(main, 997, main.getLength()).unwrap(), (997, 1035));
        assert_eq!(para_cp_bounds(main, 1035, main.getLength()).unwrap(), (1035, 1038));
    }
}
