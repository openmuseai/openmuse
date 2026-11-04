//! Read-only per-line dump from the same Java-shaped product layout entry.
//! Usage: cargo run --example java_pipeline_line_probe -- DOCX LATIN_TTF CJK_TTC
use std::{env, fs};

use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    java_pipeline_pages, open_text_document, FontBlob, ViewParams,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let docx = fs::read(args.next().ok_or("missing DOCX")?)?;
    let latin = fs::read(args.next().ok_or("missing LATIN_TTF")?)?;
    let cjk = fs::read(args.next().ok_or("missing CJK_TTC")?)?;
    let fonts = [
        FontBlob { id: "default".into(), bytes: latin },
        FontBlob { id: "CJKFallback".into(), bytes: cjk },
    ];
    let doc = open_text_document(&docx)?;
    let pages = java_pipeline_pages(&doc, &fonts, &ViewParams::default_a4())?;
    let rope = doc.getMainDocument().and_then(|main| main.getTextRope());
    let rows = pages.iter().flat_map(|page| page.lines.iter().enumerate().map(move |(index, item)| {
        let start = item.line.getStartCP();
        let end = item.line.getEndCP();
        let mut chars = vec!['\0'; (end - start).max(0) as usize];
        if let Some(rope) = rope {
            unsafe { &*rope }.getText(start, end, &mut chars, 0);
        }
        serde_json::json!({
            "page": page.page_index,
            "line": index,
            "start_cp": item.line.getStartCP(),
            "end_cp": item.line.getEndCP(),
            "width": item.line.getWidth(),
            "text": chars.iter().collect::<String>(),
            "advances": item.layout_advances,
            "embeds": item.embeds,
        })
    })).collect::<Vec<_>>();
    println!("{}", serde_json::to_string_pretty(&rows)?);
    Ok(())
}
