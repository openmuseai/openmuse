//! Read-only PAPX probe for a DOCX main-story CP. Uses the product importer.
use std::{env, fs};

use word_core::cn::wps::moffice::writer::api::DocumentOpen;
use word_core::cn::wps::moffice::writer::data::IPapxTable::{Entry, IPapxTable};
use word_core::cn::wps::moffice::writer::layout::base::env::PropertyTool::PropertyTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let path = args.next().ok_or("usage: numbering_parity_probe DOCX CP")?;
    let cp: i32 = args.next().ok_or("missing CP")?.parse()?;
    let bytes = fs::read(path)?;
    let (mut document, outcome) = DocumentOpen::open_boxed_text_document_from_docx_outcome(&bytes, "")
        .map_err(|error| format!("DOCX import failed: {error:?}"))?;
    let (num_id, level) = {
        let main = document.getMainDocument().ok_or("no main story")?;
        let table = main.getPapxTable().ok_or("no PAPX")?;
        let entry = IPapxTable::seek(table, cp).ok_or("CP not found")?;
        let props = Entry::getProperty(&*entry).ok_or("PAPX without properties")?;
        (props.getInt__int_int(235, -1), props.getInt__int_int(234, 0))
    };
    let lfo = document.getAutoNumTable().getLfoDataTable().getDataByIlfo(num_id);
    let lsid = lfo.as_ref().map(|data| data.getLsid());
    let override_level = lfo.as_ref().and_then(|data| data.getLfoLvlByIlvl(level)).and_then(|data| data.getLvlfData());
    let list_level = lsid.and_then(|lsid| document.getAutoNumTable().getLstDataTable().getDataByLsid(lsid))
        .and_then(|data| data.getLvlfDataByIlvl(level));
    let chosen_level = override_level.or(list_level);
    let level_props = chosen_level.as_ref().and_then(|level| level.getParaProp());
    let main = document.getMainDocument().ok_or("no main story")?;
    let table = main.getPapxTable().ok_or("no PAPX")?;
    let entry = IPapxTable::seek(table, cp).ok_or("CP not found")?;
    let props = Entry::getProperty(&*entry).ok_or("PAPX without properties")?;
    let mut tool = PropertyTool::PropertyTool(main, 256, 256);
    let final_props = tool.getFinalParaProp(&*entry, true);
    let final_left_indent = final_props.getInt__int_int(210, 0);
    let final_first_indent = final_props.getInt__int_int(208, 0);
    let paragraph_start = entry.position();
    let paragraph_end = entry.endCp();
    let text_doc = document.getMainDocumentMut().ok_or("no main story")?;
    let paragraph_text: String = (paragraph_start..paragraph_end)
        .map(|cp| text_doc.charAt(cp))
        .collect();
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
        "source": format!("{:?}", outcome.source),
        "cp": cp,
        "paragraph_start": paragraph_start,
        "paragraph_end": paragraph_end,
        "paragraph_text": paragraph_text,
        "num_id_present": props.contains(235),
        "lfo_found": lfo.is_some(),
        "lsid": lsid,
        "level_props_found": level_props.is_some(),
        "level_start_at": chosen_level.as_ref().map(|level| level.getStartAt()),
        "level_nfc": chosen_level.as_ref().map(|level| level.getNfc()),
        "level_follow": chosen_level.as_ref().map(|level| level.getFollow()),
        "level_template": chosen_level.as_ref().and_then(|level| level.getXstStr()),
        "level_placeholder_positions": chosen_level.as_ref().map(|level| level.getrgbxchNums()),
        "level_left_indent": level_props.as_ref().map(|props| props.getInt__int_int(210, 0)),
        "level_first_indent": level_props.as_ref().map(|props| props.getInt__int_int(208, 0)),
        "num_id": props.getInt__int_int(235, -1),
        "level": props.getInt__int_int(234, -1),
        "left_indent": props.getInt__int_int(210, 0),
        "first_indent": props.getInt__int_int(208, 0),
        "final_left_indent": final_left_indent,
        "final_first_indent": final_first_indent,
    }))?);
    Ok(())
}
