//! Read-only probe for the DOCX shape anchor control character consumed by
//! Java GraphFactory.createNodes. It does not alter model or layout state.
use std::{env, fs};

use word_core::cn::wps::moffice::writer::api::DocumentOpen;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    java_pipeline_pages, FontBlob, ViewParams,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let path = args.next().ok_or("usage: anchor_control_probe DOCX SHAPE_ID")?;
    let shape_id: i32 = args.next().ok_or("missing SHAPE_ID")?.parse()?;
    let bytes = fs::read(path)?;
    let (doc, _) = DocumentOpen::open_boxed_text_document_from_docx_outcome(&bytes, "")
        .map_err(|error| format!("DOCX import failed: {error:?}"))?;
    let main = doc.getMainDocument().ok_or("no main story")?;
    let anchor = main
        .getShapeAnchors()
        .and_then(|anchors| anchors.findById(shape_id))
        .ok_or("shape anchor missing")?;
    let cp = anchor.getCP();
    let start = (cp - 2).max(0);
    let end = (cp + 3).min(main.getLength());
    let mut chars = vec!['\0'; (end - start) as usize];
    let rope = main.getTextRope().ok_or("main story text rope missing")?;
    unsafe { &*rope }.getText(start, end, &mut chars, 0);
    println!(
        "{}",
        serde_json::json!({
            "shapeId": shape_id,
            "anchorCp": cp,
            "windowStartCp": start,
            "characters": chars.iter().enumerate().map(|(offset, ch)| serde_json::json!({
                "cp": start + offset as i32,
                "codepoint": *ch as u32,
                "character": ch.to_string(),
            })).collect::<Vec<_>>(),
        })
    );
    if let (Some(latin_path), Some(cjk_path)) = (args.next(), args.next()) {
        let fonts = [
            FontBlob { id: "default".into(), bytes: fs::read(latin_path)? },
            FontBlob { id: "CJKFallback".into(), bytes: fs::read(cjk_path)? },
        ];
        let pages = java_pipeline_pages(&doc, &fonts, &ViewParams::default_a4())?;
        let matching_lines = pages.iter().flat_map(|page| page.lines.iter().map(move |row| (page.page_index, &row.line)))
            .filter(|(_, line)| line.getStartCP() <= cp && cp < line.getEndCP())
            .map(|(page, line)| serde_json::json!({
                "page": page,
                "startCp": line.getStartCP(),
                "endCp": line.getEndCP(),
                "x": line.getLeft(),
                "y": line.getYPos(),
                "width": line.getWidth(),
                "textRectHeight": line.getTextRectHeight(),
                "ascentPos": line.getAscentPos(),
                "descentPos": line.getDescentPos(),
            })).collect::<Vec<_>>();
        println!("{}", serde_json::json!({"pageCount": pages.len(), "anchorLines": matching_lines}));
    }
    Ok(())
}
