use std::fs;
use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::layout::text::BaseTextLayout::LineMetrics;

/// RTL1 — body layout sources must not import RichTextLayout.
#[test]
fn rtl1_layout_tree_has_no_rich_text_layout() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/cn/wps/moffice/writer/layout");
    let mut hits = Vec::new();
    walk(&root, &mut hits);
    assert!(
        hits.is_empty(),
        "RichTextLayout must not appear in word-core layout: {hits:?}"
    );
}

fn walk(dir: &std::path::Path, hits: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(&p, hits);
        } else if p.extension().and_then(|s| s.to_str()) == Some("rs") {
            if let Ok(text) = fs::read_to_string(&p) {
                if text.contains("RichTextLayout") {
                    hits.push(p.display().to_string());
                }
            }
        }
    }
}

#[test]
fn rtl2_line_metrics_contract_fields() {
    let m = LineMetrics {
        start_cp: 0,
        end_cp: 4,
        x_off: 10,
        top: 100,
        bottom: 340,
        baseline: 280,
        width: 800,
    };
    assert_eq!(m.height(), 240);
    assert_eq!(m.end_cp - m.start_cp, 4);
}
