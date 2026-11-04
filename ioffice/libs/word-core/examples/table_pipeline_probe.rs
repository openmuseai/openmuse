//! Read-only table model and final style probe from the real DOCX importer.
use std::{env, fs};
use serde_json::{json, Value};
use word_core::cn::wps::moffice::writer::api::DocumentOpen;
use word_core::cn::wps::moffice::writer::data::table::TableTool::TableTool;
use word_core::cn::wps::moffice::writer::data::value::BRC::BRC;
use word_core::cn::wps::moffice::writer::data::value::TableBorders::TableBorders;
use drawing::cn::wps::moffice::drawing::IPos::IPos;

fn border(brc: Option<&BRC>) -> Value {
    match brc {
        Some(b) => json!({"type": b.getBorderType(), "widthPt": b.getLineWidth(), "color": b.getColor()}),
        None => Value::Null,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut self_check = TableBorders::new();
    self_check.setBorder(0, &BRC::new__float_int(0.5, 1));
    if self_check.getBorder(0).is_none() {
        return Err("TableBorders::setBorder/getBorder round trip failed".into());
    }
    let path = env::args().nth(1).ok_or("usage: table_pipeline_probe DOCX")?;
    let bytes = fs::read(&path)?;
    let (doc, _) = DocumentOpen::open_boxed_text_document_from_docx_outcome(&bytes, "")
        .map_err(|e| format!("DOCX import failed: {e:?}"))?;
    let main = doc.getMainDocument().ok_or("no main story")?;
    let mut table = main.getTableInfos().and_then(|infos| infos.getTableByIndex(0)).ok_or("no root table")?;
    let jc = table.getJC();
    let pos = table.getPos().map(|p| {
        let rect = p.getShapeRect();
        json!({
            "alignmentH": p.getAlignmentH(), "alignmentV": p.getAlignmentV(),
            "relativeH": p.getRelativeH(), "relativeV": p.getRelativeV(),
            "rect": [rect.left, rect.top, rect.right, rect.bottom],
        })
    });
    let table_borders = table.getTableBorders().map(|b| (0..6).map(|i| border(b.getBorder(i).as_ref())).collect::<Vec<_>>());
    let mut rows = Vec::new();
    for row_index in 0..table.rows().len() {
        let (start, end, count) = {
            let row = &table.rows()[row_index];
            (row.getStartCP(), row.getEndCP(), row.cells().len())
        };
        let row_prop = TableTool::getFinalRowProperty(main, end - 1);
        let raw_borders = row_prop.getValue(310)
            .and_then(|v| v.as_any().downcast_ref::<TableBorders>())
            .map(|b| (0..6).map(|i| border(b.getBorder(i).as_ref())).collect::<Vec<_>>());
        let mut cells = Vec::new();
        for cell_index in 0..count {
            let (cp, cell_end) = {
                let cell = &table.rows()[row_index].cells()[cell_index];
                (cell.getStartCP(), cell.getEndCP())
            };
            let cell_box = table.cell_layout_box(cp);
            let layout_flags = table.cell_env_flags(row_index, cell_index);
            let cell = &table.rows()[row_index].cells()[cell_index];
            cells.push(json!({
                "startCp": cp, "endCp": cell_end,
                "horiMerge": cell.getHoriMerge(), "vertMerge": cell.getVertMerge(),
                "layoutFlags": [layout_flags.0, layout_flags.1, layout_flags.2],
                "width": table.rows()[row_index].cell_width(cell_index),
                "margins": [cell.margin_top(), cell.margin_left(), cell.margin_bottom(), cell.margin_right()],
                "brc": [border(cell.getBrcTop()), border(cell.getBrcLeft()), border(cell.getBrcBottom()), border(cell.getBrcRight())],
                "box": cell_box.map(|b| json!({"left": b.border_left, "right": b.border_right,
                    "textDx": b.text_dx, "contentWidth": b.content_width,
                    "lineWidths": [b.top_w, b.left_w, b.bottom_w, b.right_w]})),
            }));
        }
        let row = &table.rows()[row_index];
        rows.push(json!({"startCp": start, "endCp": end, "rowHeight": row.getRowHeight(),
            "rgdxa": (0..row.rgdxa_count()).map(|i| row.rgdxa(i)).collect::<Vec<_>>(),
            "rawBorders": raw_borders, "cells": cells}));
    }
    println!("{}", serde_json::to_string_pretty(&json!({
        "input": path, "table": {"startCp": table.getStartCP(), "endCp": table.getEndCP(),
            "pos": pos, "jc": [jc.jc, jc.jcLeft, jc.jcRight, jc.tableIndent],
            "borders": table_borders, "rows": rows}
    }))?);
    Ok(())
}
