//! Read-only product importer probe for the first table row geometry.
use std::{env, fs};
use word_core::cn::wps::moffice::writer::api::DocumentOpen;
use word_core::cn::wps::moffice::writer::data::table::TableTool::TableTool;
use word_core::cn::wps::moffice::writer::data::value::DefTable::DefTable;
use word_core::cn::wps::moffice::writer::data::value::DefTableBuilder::DefTableBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).ok_or("usage: table_geometry_probe DOCX")?;
    let bytes = fs::read(path)?;
    let (doc, _) = DocumentOpen::open_boxed_text_document_from_docx_outcome(&bytes, "")
        .map_err(|e| format!("DOCX import failed: {e:?}"))?;
    let main = doc.getMainDocument().ok_or("no main story")?;
    let mut table = main.getTableInfos().and_then(|infos| infos.getTableByIndex(0)).ok_or("no table")?;
    let row_end = table.rows().first().ok_or("no row")?.getEndCP();
    let prop = TableTool::getFinalRowProperty(main, row_end - 1);
    let value = prop.getValue(306);
    println!("row_end={row_end} prop306={} as_builder={} as_def_table={} size={}",
        value.is_some(),
        value.is_some_and(|v| v.as_any().downcast_ref::<DefTableBuilder>().is_some()),
        value.is_some_and(|v| v.as_any().downcast_ref::<DefTable>().is_some()),
        prop.size());
    let _ = table.getJC();
    let resolved_widths = (0..table.rows()[0].cells().len())
        .map(|i| table.cell_width(0, i)).collect::<Vec<_>>();
    let row = &table.rows()[0];
    let rgdxa: Vec<i32> = (0..row.rgdxa_count()).map(|i| row.rgdxa(i)).collect();
    println!("rgdxa={rgdxa:?} cells={} widths={resolved_widths:?}", row.cells().len());
    Ok(())
}
