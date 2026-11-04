//! MVP 产品捷径：fixture `KmoSlide` → 层序 → 几何/文本指令（不跑 PptxReader）。

use std::sync::{Arc, Once};

use serde::Serialize;
use util::cn::wps::graphics::KDisplayMetrics::KDisplayMetrics;
use util_adapter::cn::wps::core::runtime::Platform::Platform;
use wpslayout::cn::wps::text::layout::data::TextBoxData::TextBoxData;

use crate::cn::wps::base::converter::UnitsConverter::UnitsConverter;
use crate::cn::wps::show::layout::dataaction::ShapeDataTransform::ShapeDataTransform;
use crate::cn::wps::show::layout::typo::GreedyWrap::{greedy_wrap, ShowTypoLine};
use crate::cn::wps::show::layout::typo::TextBoxGeometry::text_box_geometry;
use crate::cn::wps::show::render::rendercommand::{
    apply_fill_line, apply_run_style, bytes_to_b64, geometry_commands, text_line_commands, to_json,
    RenderCommand,
};
use crate::cn::wps::show::render::ShowTypoShape::ShowTypoShape;
use crate::cn::wps::show::render::SlideRender::{LayerItem, SlideRender};
use crate::cn::wps::show::util::layout_shape::{KmoShapeTree, KmoShowShape, KmoSlide};
use crate::cn::wps::show::util::layout_textbox::{
    KmoTextOperator, TextCharOptions, TextCharacterProperties, TextPara, TextParagraphProperties,
    TextParaOptions, TextRun,
};

static METRICS: Once = Once::new();

pub fn ensure_mvp_metrics() {
    METRICS.call_once(|| {
        Platform::setDisplayMetrics(Arc::new(KDisplayMetrics::new_with_params(
            0, 0, 1.0, 160, 1.0, 96.0, 96.0,
        )));
    });
}

#[derive(Clone, Serialize)]
pub struct MvpPageEnvelope {
    pub index: usize,
    pub width: f32,
    pub height: f32,
    pub commands: Vec<RenderCommand>,
}

#[derive(Serialize)]
pub struct MvpEnvelope {
    pub ok: bool,
    pub pipeline: &'static str,
    pub width: f32,
    pub height: f32,
    pub commands: Vec<RenderCommand>,
    pub page_count: usize,
    pub selected_page: usize,
    pub pages: Vec<MvpPageEnvelope>,
}

pub fn hello_slide() -> KmoSlide {
    let fill = 0xFF4472C4u32 as i32;
    let rect_shape = KmoShowShape {
        id: 1,
        is_text_box: false,
        transform2DX: 1_270_000,
        transform2DY: 1_270_000,
        transform2DW: 5_080_000,
        transform2DH: 2_540_000,
        sp_pr: {
            // rust: ★W6c-6 —— 填充的**类型与颜色**都要落在影子 `FillObj` 上：
            //   `FillObj.solid` 才是颜色载体（标量 `fill` 只承载类型，无法表达颜色）。
            //   `ConvertKmo::from_layout` 由 `FillObj` 合成真身填充，故此处写 `fill_obj`。
            //   owner: ppt.show.render.mvp_pipeline.hello_slide    wave: M-PPT-KMOSLIDE-W6c-6
            let mut p = crate::cn::wps::show::util::layout_shape::ShapePr {
                has_fill: true,
                fill,
                ..Default::default()
            };
            p.fill_obj.putType(0);
            p.fill_obj.solid = Some(fill);
            p
        },
        ..KmoShowShape::default()
    };
    let op = KmoTextOperator {
        paras: vec![TextPara {
            para_prop: TextParagraphProperties {
                options: TextParaOptions {
                    has_bu_none: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            runs: vec![TextRun {
                text: "Hello PPT".into(),
                start: 0,
                end: 9,
                ch_prop: TextCharacterProperties {
                    font_name: "Arial".into(),
                    font_size_pt: 24.0,
                    font_color: 0xFF000000u32 as i32,
                    options: TextCharOptions {
                        has_bold: true,
                        bold: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            }],
        }],
        has_paragraph_list: true,
        ph_type: 1,
        ..Default::default()
    };
    let text_shape = KmoShowShape {
        id: 2,
        is_text_box: true,
        has_text_body: true,
        text_op: Some(op),
        transform2DX: 1_270_000,
        transform2DY: 4_064_000,
        transform2DW: 8_890_000,
        transform2DH: 1_270_000,
        ..KmoShowShape::default()
    };
    KmoSlide {
        slide_width: 12_192_000,
        slide_height: 6_858_000,
        show_master_shape: false,
        has_background: false,
        sp_tree: KmoShapeTree {
            children: vec![rect_shape, text_shape],
            ..Default::default()
        },
        ..Default::default()
    }
}

fn advances_for(data: &TextBoxData) -> Vec<Vec<f32>> {
    let mut out = Vec::new();
    for td in data.getTextDatas() {
        let src = td.mSourceText.as_deref().unwrap_or("");
        let chars: Vec<char> = src.chars().filter(|c| *c != '\r').collect();
        let size = td.mFirstSpan.mFontSize.max(1.0);
        let mut row = Vec::with_capacity(chars.len());
        for ch in chars {
            let w = if ch == ' ' { size * 0.3 } else { size * 0.55 };
            row.push(w);
        }
        out.push(row);
    }
    out
}

fn emit_text(typo: &mut ShowTypoShape, draw_flags: i32) -> Vec<RenderCommand> {
    let Some(app) = typo.getShowShape() else {
        return Vec::new();
    };
    let attach = crate::cn::wps::show::app::shape::kmo_show_shape_hosts::convert_attach(app.id());
    if !attach.has_text_body && !typo.isTextBox() {
        return Vec::new();
    }
    let shape = crate::cn::wps::show::util::layout_shape::KmoShowShape {
        is_text_box: typo.isTextBox(),
        has_text_body: attach.has_text_body,
        text_op: attach.text_op.clone(),
        ..crate::cn::wps::show::util::layout_shape::KmoShowShape::default()
    };
    let rc = typo.getShapeRect();
    let (font, size, color, bold) = shape
        .text_op
        .as_ref()
        .and_then(|op| op.paras.first())
        .and_then(|p| p.runs.first())
        .map(|r| {
            (
                Some(r.ch_prop.font_name.clone()),
                Some(r.ch_prop.font_size_pt),
                Some(r.ch_prop.font_color as u32),
                Some(r.ch_prop.options.bold),
            )
        })
        .unwrap_or((None, None, None, None));
    // eaVert skeleton: stack chars in-box; skip RichTextLayout / DataTransform.
    if shape
        .text_op
        .as_ref()
        .map(|o| o.body_pr.has_vertical_text && o.body_pr.vertical_text == 4)
        .unwrap_or(false)
    {
        let raw: String = shape
            .text_op
            .as_ref()
            .map(|o| {
                o.paras
                    .iter()
                    .flat_map(|p| p.runs.iter().map(|r| r.text.as_str()))
                    .collect()
            })
            .unwrap_or_default();
        let step = size.unwrap_or(18.0);
        let mut y = rc.top;
        let mut stacked = Vec::new();
        for ch in raw.chars() {
            stacked.push(RenderCommand::DrawText {
                text: ch.to_string(),
                x: rc.left,
                y,
                font: font.clone(),
                size,
                color,
                bold,
                italic: None,
            });
            y += step;
        }
        return stacked;
    }
    let mut xf = ShapeDataTransform::default();
    let data: TextBoxData = xf.exToInternal(Some(&shape), None);
    if data.isEmpty() {
        return Vec::new();
    }
    let w = rc.right - rc.left;
    let h = rc.bottom - rc.top;
    let inner = text_box_geometry(&data, w, h);
    let lines: Vec<ShowTypoLine> = greedy_wrap(&data, inner, &advances_for(&data));
    let cmds = text_line_commands(typo, &lines, draw_flags, rc.left, rc.top);
    cmds.into_iter()
        .map(|c| apply_run_style(c, font.clone(), size, color, bold, None))
        .collect()
}

fn clip_commands_to_slide(cmds: Vec<RenderCommand>, sw: f32, sh: f32) -> Vec<RenderCommand> {
    if sw <= 0.0 || sh <= 0.0 {
        return cmds;
    }
    cmds.into_iter().filter_map(|c| clip_one(c, sw, sh)).collect()
}

fn clip_one(cmd: RenderCommand, sw: f32, sh: f32) -> Option<RenderCommand> {
    match cmd {
        RenderCommand::DrawRect { x, y, w, h, fill, stroke } => {
            let x2 = (x + w).min(sw).max(0.0);
            let y2 = (y + h).min(sh).max(0.0);
            let x1 = x.max(0.0).min(sw);
            let y1 = y.max(0.0).min(sh);
            if x2 <= x1 || y2 <= y1 {
                None
            } else {
                Some(RenderCommand::DrawRect {
                    x: x1,
                    y: y1,
                    w: x2 - x1,
                    h: y2 - y1,
                    fill,
                    stroke,
                })
            }
        }
        RenderCommand::DrawImage { x, y, w, h, media_id, r_id, bytes_b64 } => {
            let x2 = (x + w).min(sw).max(0.0);
            let y2 = (y + h).min(sh).max(0.0);
            let x1 = x.max(0.0).min(sw);
            let y1 = y.max(0.0).min(sh);
            if x2 <= x1 || y2 <= y1 {
                None
            } else {
                Some(RenderCommand::DrawImage {
                    x: x1,
                    y: y1,
                    w: x2 - x1,
                    h: y2 - y1,
                    media_id,
                    r_id,
                    bytes_b64,
                })
            }
        }
        RenderCommand::DrawPath { path, fill, stroke } => {
            if path.iter().all(|(x, y)| *x < 0.0 || *y < 0.0 || *x > sw || *y > sh) {
                return None;
            }
            let path = path
                .into_iter()
                .map(|(x, y)| (x.clamp(0.0, sw), y.clamp(0.0, sh)))
                .collect();
            Some(RenderCommand::DrawPath { path, fill, stroke })
        }
        RenderCommand::DrawText { text, x, y, font, size, color, bold, italic } => {
            if x >= sw || y >= sh || x + size.unwrap_or(1.0) < 0.0 || y + size.unwrap_or(1.0) < 0.0 {
                None
            } else {
                Some(RenderCommand::DrawText {
                    text,
                    x: x.max(0.0),
                    y: y.max(0.0),
                    font,
                    size,
                    color,
                    bold,
                    italic,
                })
            }
        }
    }
}

/// 编排：层序 + 几何 + 文本行 → 指令列表。
pub fn render_slide(slide: &KmoSlide, slide_draw_flags: i32) -> Vec<RenderCommand> {
    ensure_mvp_metrics();
    let mut cmds = Vec::new();
    let mut layers = SlideRender::layer_order(Some(slide), slide_draw_flags, 0);
    for item in &mut layers {
        let pic = match item {
            LayerItem::Shape(typo) => {
                let rc = typo.getShapeRect();
                typo.getShowShape().and_then(|s| {
                    let attach =
                        crate::cn::wps::show::app::shape::kmo_show_shape_hosts::convert_attach(s.id());
                    if s.r#type() == 2 || attach.media_bytes.is_some() || attach.media_r_id.is_some()
                    {
                        Some((
                            rc,
                            attach.media_id.unwrap_or(0),
                            attach.media_r_id.clone(),
                            attach.media_bytes.clone(),
                        ))
                    } else {
                        None
                    }
                })
            }
            _ => None,
        };
        if let Some((rc, media_id, r_id, bytes)) = pic {
            cmds.push(RenderCommand::DrawImage {
                x: rc.left,
                y: rc.top,
                w: rc.right - rc.left,
                h: rc.bottom - rc.top,
                media_id,
                r_id,
                bytes_b64: bytes.as_ref().map(|b| bytes_to_b64(b)),
            });
            continue;
        }
        let fill = match item {
            LayerItem::Shape(typo) => typo
                .getShape()

                .and_then(|mut s| {
                    s.getFill()
                        .and_then(|a| a.downcast_ref::<drawing::cn::wps::moffice::drawing::graphics::FillBase::FillBase>().cloned())
                        .map(|f| f.getColor() as u32)
                        .filter(|c| *c != 0)
                })
                .or_else(|| {
                    typo.getShowShape().and_then(|s| {
                        crate::cn::wps::show::app::shape::kmo_show_shape_hosts::convert_attach(
                            s.id(),
                        )
                        .fill
                    })
                }),
            LayerItem::Background { fill, .. } => *fill,
            _ => None,
        };
        let stroke = match item {
            LayerItem::Shape(typo) => typo.getShape().and_then(|mut s| {
                s.getLineProperty().and_then(|a| {
                    a.downcast_ref::<drawing::cn::wps::moffice::drawing::line::LineProperty::LineProperty>()
                        .map(|lp| lp.getColor() as u32)
                })
            }),
            _ => None,
        };
        let mut geo = geometry_commands(item);
        geo = geo
            .into_iter()
            .map(|c| {
                let (old_fill, old_stroke) = match &c {
                    RenderCommand::DrawRect { fill, stroke, .. }
                    | RenderCommand::DrawPath { fill, stroke, .. } => (*fill, *stroke),
                    _ => (None, None),
                };
                apply_fill_line(
                    c,
                    fill.or(old_fill),
                    stroke.filter(|c| *c != 0).or(old_stroke),
                )
            })
            .collect();
        cmds.extend(geo);
        if let LayerItem::Shape(typo) = item {
            cmds.extend(emit_text(typo, slide_draw_flags));
        }
    }
    let (sw, sh) = match layers.first() {
        Some(LayerItem::WhiteBackground { w, h }) => (*w, *h),
        _ => (
            UnitsConverter::render().fromEmuX(slide.slide_width as f32),
            UnitsConverter::render().fromEmuY(slide.slide_height as f32),
        ),
    };
    clip_commands_to_slide(cmds, sw, sh)
}

pub fn envelope_from_slide(slide: &KmoSlide, pipeline: &'static str) -> MvpEnvelope {
    ensure_mvp_metrics();
    let cmds = render_slide(slide, 0);
    let layers = SlideRender::layer_order(Some(slide), 0, 0);
    let (width, height) = match layers.first() {
        Some(LayerItem::WhiteBackground { w, h }) => (*w, *h),
        _ => (960.0, 540.0),
    };
    let page = MvpPageEnvelope {
        index: 0,
        width,
        height,
        commands: cmds.clone(),
    };
    MvpEnvelope {
        ok: true,
        pipeline,
        width,
        height,
        commands: cmds,
        page_count: 1,
        selected_page: 0,
        pages: vec![page],
    }
}

pub fn envelope_from_document(
    document: &crate::cn::wps::show::pptxr::PptxReader::PptxDocument,
    pipeline: &'static str,
) -> MvpEnvelope {
    ensure_mvp_metrics();
    let pages = document
        .slides
        .iter()
        .enumerate()
        .map(|(index, slide)| {
            let commands = render_slide(slide, 0);
            let layers = SlideRender::layer_order(Some(slide), 0, 0);
            let (width, height) = match layers.first() {
                Some(LayerItem::WhiteBackground { w, h }) => (*w, *h),
                _ => (960.0, 540.0),
            };
            MvpPageEnvelope {
                index,
                width,
                height,
                commands,
            }
        })
        .collect::<Vec<_>>();
    let first = pages.first().cloned().unwrap_or(MvpPageEnvelope {
        index: 0,
        width: 960.0,
        height: 540.0,
        commands: Vec::new(),
    });
    MvpEnvelope {
        ok: true,
        pipeline,
        width: first.width,
        height: first.height,
        commands: first.commands,
        page_count: pages.len(),
        selected_page: 0,
        pages,
    }
}

pub fn render_hello_slide() -> Vec<RenderCommand> {
    render_slide(&hello_slide(), 0)
}

pub fn render_pptx_bytes(bytes: &[u8]) -> Result<Vec<RenderCommand>, String> {
    let slide = crate::cn::wps::show::pptxr::PptxReader::open_pptx_bytes(bytes)?;
    Ok(render_slide(&slide, 0))
}

pub fn render_pptx_pages(bytes: &[u8]) -> Result<Vec<Vec<RenderCommand>>, String> {
    let document = crate::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes(bytes)?;
    Ok(document
        .slides
        .iter()
        .map(|slide| render_slide(slide, 0))
        .collect())
}

pub fn render_pptx_envelope_json(bytes: &[u8]) -> Result<String, String> {
    let document = crate::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes(bytes)?;
    serde_json::to_string_pretty(&envelope_from_document(
        &document,
        "ppt-core-PptxReader.open-document-v1",
    ))
        .map_err(|e| e.to_string())
}

pub fn render_pptx_page_json(bytes: &[u8], index: usize) -> Result<String, String> {
    let document = crate::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes(bytes)?;
    let envelope = envelope_from_document(&document, "ppt-core-PptxReader.open-document-v1");
    if envelope.pages.is_empty() {
        return Err("SuffixError: Not pptx file".to_string());
    }
    let i = index.min(envelope.pages.len() - 1);
    serde_json::to_string_pretty(&envelope.pages[i]).map_err(|e| e.to_string())
}

pub fn render_hello_envelope_json() -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&envelope_from_slide(
        &hello_slide(),
        "ppt-core-render_slide-v0-mvp",
    ))
}

/// 仅命令数组（pr-04-01 口径）。
pub fn render_hello_commands_json() -> Result<String, serde_json::Error> {
    to_json(&render_hello_slide())
}
