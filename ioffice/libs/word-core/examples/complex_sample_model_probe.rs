//! Read-only Stage1 probe. It observes the same SAX importer as the product;
//! it does not alter the document or provide a layout fallback.
use std::{env, fs};

use drawing::cn::wps::moffice::drawing::IPos::IPos;
use word_core::cn::wps::moffice::writer::api::DocumentOpen;
use word_core::cn::wps::moffice::writer::data::IPapxTable::{Entry as PapxEntry, IPapxTable};
use word_core::cn::wps::moffice::writer::data::table::tree::TableInfoNode::TableInfoNode;
use word_core::cn::wps::moffice::writer::data::table::TableTool::TableTool;
use word_core::cn::wps::moffice::writer::data::value::PositionCode::PositionCode;
use word_core::cn::wps::moffice::writer::data::value::PageSetup::PageSetup;

fn table_json(table: &TableInfoNode) -> serde_json::Value {
    let mut position_view = table.clone();
    let pos_relative_x = position_view.getPos().map(|pos| IPos::getRelativeX(pos));
    let pos_relative_y = position_view.getPos().map(|pos| IPos::getRelativeY(pos));
    serde_json::json!({
        "startCp": table.getStartCP(),
        "endCp": table.getEndCP(),
        "level": table.getLevel(),
        "posRelativeX": pos_relative_x,
        "posRelativeY": pos_relative_y,
        "rowCount": table.row_count(),
        "rows": table.rows().iter().map(|row| serde_json::json!({
            "startCp": row.getStartCP(),
            "endCp": row.getEndCP(),
            "cellCount": row.cells().len(),
            "cells": row.cells().iter().map(|cell| serde_json::json!({
                "startCp": cell.getStartCP(),
                "endCp": cell.getEndCP(),
                "tables": cell.tables().iter().map(table_json).collect::<Vec<_>>()
            })).collect::<Vec<_>>()
        })).collect::<Vec<_>>()
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).ok_or("usage: complex_sample_model_probe DOCX")?;
    let bytes = fs::read(&path)?;
    let (mut doc, outcome) = DocumentOpen::open_boxed_text_document_from_docx_outcome(&bytes, "")
        .map_err(|e| format!("DOCX import failed: {e:?}"))?;
    let ole_shape_present = doc.getMainDocumentMut()
        .and_then(|main| main.getDrawingContainer())
        .map(|container| [1030, 1036, 1037, 1039].into_iter()
            .map(|id| (id.to_string(), container.findShapeByID(id).is_some()))
            .collect::<std::collections::BTreeMap<_, _>>());
    let ole_shape_line_widths = doc.getMainDocumentMut()
        .and_then(|main| main.getDrawingContainer())
        .map(|container| [1026, 1028, 1030, 1032, 1034, 1036, 1037].into_iter()
            .map(|id| {
                let width = container.findShapeByID(id)
                    .and_then(|element| element.downcast_ref::<std::sync::Mutex<drawing::cn::wps::moffice::drawing::Shape::Shape>>()
                        .and_then(|shape| shape.lock().ok())
                        .map(|mut shape| shape.getLineProperty()))
                    .flatten()
                    .and_then(|element| element.downcast_ref::<drawing::cn::wps::moffice::drawing::line::LineProperty::LineProperty>()
                        .map(|line| line.getWidth()).or_else(|| {
                            element.downcast_ref::<std::sync::Mutex<drawing::cn::wps::moffice::drawing::line::LineProperty::LineProperty>>()
                                .and_then(|line| line.lock().ok()).map(|line| line.getWidth())
                        }));
                (id.to_string(), width)
            }).collect::<std::collections::BTreeMap<_, _>>());
    let main = doc.getMainDocument().ok_or("no main story")?;
    let table_info_present = main.getTableInfos().is_some();
    let table_count = main.getTableInfos().map(|infos| infos.size());
    let nested_table_count = main.getTableInfos().map(|infos| infos.nestedTableCount());
    let root_table = main.getTableInfos().and_then(|infos| infos.getTableByIndex(0));
    // Java KTableManager.findNextRowStart reads PAPX 223/224 and its row-end
    // path reads 225. Observe those imported properties before the cache.
    let mut papx_entries = 0usize;
    let mut papx_in_table = 0usize;
    let mut papx_row_ends = 0usize;
    let mut papx_table_width = 0usize;
    let mut papx_table_levels = std::collections::BTreeMap::<i32, usize>::new();
    let mut first_table_cp = None;
    let mut first_level2_cp = None;
    let mut table_paragraphs = Vec::new();
    if let Some(papx) = main.getPapxTable() {
        let mut entry = IPapxTable::begin(papx);
        while let Some(current) = entry {
            if current.isEnd() || papx_entries > 100_000 {
                break;
            }
            papx_entries += 1;
            if let Some(prop) = PapxEntry::getProperty(&*current) {
                if prop.getBoolean__int_boolean(223, false) {
                    papx_in_table += 1;
                    first_table_cp.get_or_insert(current.position());
                    let level = prop.getInt__int_int(224, 1);
                    if level >= 2 {
                        first_level2_cp.get_or_insert(current.position());
                    }
                    *papx_table_levels.entry(level).or_default() += 1;
                    let length = (current.endCp() - current.position()).clamp(0, 512) as usize;
                    let mut chars = vec!['\0'; length];
                    if let Some(rope) = main.getTextRope() {
                        unsafe { &*rope }.getText(current.position(), current.position() + length as i32, &mut chars, 0);
                    }
                    table_paragraphs.push(serde_json::json!({
                        "startCp": current.position(),
                        "endCp": current.endCp(),
                        "layer": level,
                        "hasPos312": prop.contains(312),
                        "hasPos313": prop.contains(313),
                        "hasPos314": prop.contains(314),
                        "pos312": prop.getValue(312).and_then(|v| v.as_ref().as_any().downcast_ref::<PositionCode>()).map(|pc| [pc.getVertPos(), pc.getHorzPos()]),
                        "rowEnd": prop.getBoolean__int_boolean(225, false),
                        "text": chars.iter().collect::<String>()
                    }));
                }
                if prop.getBoolean__int_boolean(225, false) {
                    papx_row_ends += 1;
                }
                if prop.contains(295) {
                    papx_table_width += 1;
                }
            }
            entry = PapxEntry::getNext(&*current);
        }
    }
    let level2_table = first_level2_cp.and_then(|cp| {
        main.getTableInfos()
            .and_then(|infos| infos.getTableInfoByLevel(cp, 2))
    });
    let level2_first_row_prop = level2_table.as_ref().and_then(|table| table.rows().first())
        .map(|row| TableTool::getFinalRowProperty(main, row.getEndCP() - 1));
    let shape_anchor_container_present = main.getShapeAnchors().is_some();
    let shape_anchor_count = main.getShapeAnchors().map(|anchors| anchors.size());
    let ole_anchor_cps = [1030, 1036, 1037, 1039].into_iter().map(|id| {
        (id.to_string(), main.getShapeAnchors().and_then(|anchors| anchors.findById(id)).map(|anchor| anchor.getCP()))
    }).collect::<std::collections::BTreeMap<_, _>>();
    let around_first_page_break = main.getTextRope().map(|rope| {
        let start = 680.min(main.getLength());
        let end = 740.min(main.getLength());
        let mut chars = vec!['\0'; (end - start) as usize];
        unsafe { &*rope }.getText(start, end, &mut chars, 0);
        chars.into_iter().collect::<String>()
    });
    // Read-only CP windows make a Stage2 line identifiable in the imported
    // UTF-16 story; raw document.xml offsets do not have the same CP meaning.
    let cp_windows = env::args().skip(2).filter_map(|range| {
        let (start, end) = range.split_once(':')?;
        let start = start.parse::<i32>().ok()?;
        let end = end.parse::<i32>().ok()?;
        if start < 0 || end < start || end > main.getLength() { return None; }
        let rope = main.getTextRope()?;
        let mut chars = vec!['\0'; (end - start) as usize];
        unsafe { &*rope }.getText(start, end, &mut chars, 0);
        Some(serde_json::json!({"startCp": start, "endCp": end,
            "text": chars.into_iter().collect::<String>()}))
    }).collect::<Vec<_>>();
    let field_triples = main.getPLCField().map(|plc| {
        plc.getFieldTriples().into_iter().filter(|triple| {
            (6334..6653).contains(&triple.getStartCP())
        }).map(|triple| {
            serde_json::json!({"startCp": triple.getStartCP(),
                "separatorCp": triple.getSepCP(), "endCp": triple.getEndCP(),
                "fieldType": triple.getFieldType()})
        }).collect::<Vec<_>>()
    }).unwrap_or_default();
    let all_field_triple_count = main.getPLCField().map(|plc| plc.getFieldTriples().len());
    let section_container_present = main.getPLCSection().is_some();
    let first_section_cp = main.getPLCSection().and_then(|sections| sections.getFirstSection()).map(|section| section.position());
    let last_section_cp = main.getPLCSection().and_then(|sections| sections.getLastSection()).map(|section| section.position());
    // Java TextDocument.getPageSetup(cp) is now directly observable through
    // SubDocument.readLockAll and the same section lookup used by the product.
    let page_setup_cp0 = doc.getPageSetup(0).map(|setup| serde_json::json!({
        "width": setup.getWidth(),
        "height": setup.getHeight(),
        "marginLeft": setup.getMarginLeft(),
        "marginRight": setup.getMarginRight(),
        "marginTop": setup.getMarginTop(),
        "marginBottom": setup.getMarginBottom(),
    }));
    let merged_page_setup_cp0 = main.getPLCSection()
        .and_then(|sections| sections.getSection(0))
        .and_then(|section| {
            let merged = doc.getMergedSepx(&section.getPropertySet());
            merged.getValue(676)
                .and_then(|value| value.as_any().downcast_ref::<PageSetup>().copied())
        }).map(|setup| serde_json::json!({
            "width": setup.getWidth(), "height": setup.getHeight(),
            "marginLeft": setup.getMarginLeft(), "marginRight": setup.getMarginRight(),
            "marginTop": setup.getMarginTop(), "marginBottom": setup.getMarginBottom(),
        }));
    // Observe every change in the public Java-named getSection(cp) result.
    // The probe does not walk Rust's private linked-list representation.
    let mut section_entries = Vec::new();
    let mut previous_section_cp = None;
    if let Some(sections) = main.getPLCSection() {
        for cp in 0..=main.getLength() {
            if let Some(section) = sections.getSection(cp) {
                if previous_section_cp != Some(section.position()) {
                    let raw = section.getPropertySet();
                    let raw_setup = raw.getValue(676)
                        .and_then(|value| value.as_any().downcast_ref::<PageSetup>().copied());
                    let effective_setup = doc.getPageSetup(cp);
                    section_entries.push(serde_json::json!({
                        "queryCp": cp,
                        "cp": section.position(),
                        "breakCode": raw.getInt__int_int(681, 2),
                        "textFlow": raw.getInt__int_int(714, 0),
                        "rawPageSetup": raw_setup.map(|setup| serde_json::json!({
                            "width": setup.getWidth(), "height": setup.getHeight(),
                            "marginLeft": setup.getMarginLeft(), "marginRight": setup.getMarginRight(),
                            "marginTop": setup.getMarginTop(), "marginBottom": setup.getMarginBottom(),
                        })),
                        "effectivePageSetup": effective_setup.map(|setup| serde_json::json!({
                            "width": setup.getWidth(), "height": setup.getHeight(),
                            "marginLeft": setup.getMarginLeft(), "marginRight": setup.getMarginRight(),
                            "marginTop": setup.getMarginTop(), "marginBottom": setup.getMarginBottom(),
                        })),
                    }));
                    previous_section_cp = Some(section.position());
                }
            }
        }
    }
    let root_tables = main.getTableInfos().map(|infos| {
        (0..infos.size()).filter_map(|i| infos.getTableByIndex(i))
            .map(|table| table_json(&table)).collect::<Vec<_>>()
    }).unwrap_or_default();
    let root_row_ranges = root_tables.iter().flat_map(|table| table["rows"].as_array().into_iter().flatten())
        .map(|row| (row["startCp"].as_i64().unwrap_or_default() as i32,
                    row["endCp"].as_i64().unwrap_or_default() as i32)).collect::<Vec<_>>();
    let same_table_pairs = root_row_ranges.windows(2).map(|pair| {
        let (s1,e1) = pair[0]; let (s2,e2) = pair[1];
        serde_json::json!({"row1Start":s1,"row2Start":s2,
            "same":TableTool::isInTheSameTable(main, s1,e1,s2,e2)})
    }).collect::<Vec<_>>();
    println!(
        "{}",
        serde_json::json!({
            "source": format!("{:?}", outcome.source),
            "panic_kind": outcome.panic_kind.as_ref().map(|kind| format!("{kind:?}")),
            "main_story_cp_length": main.getLength(),
            "is_modified": doc.isModified(),
            "is_auto_save": doc.isAutoSave(),
            "table_info_present": table_info_present,
            "table_infos_size": table_count,
            "nested_table_count": nested_table_count,
            "table0_start": root_table.as_ref().map(|table| table.getStartCP()),
            "table0_end": root_table.as_ref().map(|table| table.getEndCP()),
            "table0_level": root_table.as_ref().map(|table| table.getLevel()),
            "level2_cp": first_level2_cp,
            "level2_start": level2_table.as_ref().map(|table| table.getStartCP()),
            "level2_end": level2_table.as_ref().map(|table| table.getEndCP()),
            "level2_table_level": level2_table.as_ref().map(|table| table.getLevel()),
            "level2_first_row_pos_props": level2_first_row_prop.as_ref().map(|prop| serde_json::json!({
                "has312": prop.contains(312), "has313": prop.contains(313), "has314": prop.contains(314),
                "positionCode": prop.getValue(312).and_then(|v| v.as_ref().as_any().downcast_ref::<PositionCode>()).map(|pc| [pc.getVertPos(), pc.getHorzPos()])
            })),
            "papx_entries": papx_entries,
            "papx_in_table": papx_in_table,
            "papx_row_ends": papx_row_ends,
            "papx_table_width": papx_table_width,
            "papx_table_levels": papx_table_levels,
            "tableParagraphs": table_paragraphs,
            "rootTables": root_tables,
            "sameTablePairs": same_table_pairs,
            "first_table_cp": first_table_cp,
            "shape_anchor_container_present": shape_anchor_container_present,
            "shape_anchor_count": shape_anchor_count,
            "ole_anchor_cps": ole_anchor_cps,
            "ole_shape_present": ole_shape_present,
            "ole_shape_line_widths": ole_shape_line_widths,
            "aroundFirstPageBreak": around_first_page_break,
            "cpWindows": cp_windows,
            "tableFieldTriples": field_triples,
            "allFieldTripleCount": all_field_triple_count,
            "section_container_present": section_container_present,
            "firstSectionCp": first_section_cp,
            "lastSectionCp": last_section_cp,
            "pageSetupCp0": page_setup_cp0,
            "mergedPageSetupCp0": merged_page_setup_cp0,
            "sectionEntriesByQuery": section_entries,
        })
    );
    Ok(())
}
