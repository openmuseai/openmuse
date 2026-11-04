#![allow(non_snake_case)]

use serde_json::{json, Value};
use word_core::cn::wps::moffice::writer::api::DocumentOpen;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::layout::base::text::ChpxRuns::ChpxRuns;

fn collection(fonts: Option<word_core::cn::wps::moffice::writer::data::theme::font::FontCollection::FontCollection>) -> Value {
    match fonts {
        None => json!({}),
        Some(fonts) => json!({
            "latin": fonts.getLatinFont().map(|f| f.getFontName()).unwrap_or_default(),
            "eastAsia": fonts.getEastAsian().map(|f| f.getFontName()).unwrap_or_default(),
            "complexScript": fonts.getComplexScript().map(|f| f.getFontName()).unwrap_or_default(),
            "supplemental": fonts.getSupplementalFont().unwrap_or_default().iter().map(|f| json!({
                "script": f.getScriptStr().unwrap_or_default(),
                "typeface": f.getTypeface().unwrap_or_default()
            })).collect::<Vec<_>>()
        }),
    }
}

/// Compare the product importer with the separate Android Java capture.
#[test]
fn theme_scheme_and_raw_chpx_match_android_capture() {
    let bytes = std::fs::read("tests/fixtures/m3_09h/theme-font-sample2.docx").unwrap();
    let mut doc = Box::new(TextDocument::TextDocument__boolean(true));
    doc._create();
    DocumentOpen::fill_text_document_from_docx(&mut doc, &bytes, "").unwrap();
    let theme = doc.getTheme_mut().expect("theme");
    let theme = unsafe { &mut *theme };
    let scheme = unsafe { &mut *theme.getThemeFontScheme_mut() };
    let table = doc.getMainDocument().unwrap().getChpxTable().unwrap();
    let spans = ChpxRuns::walkChpx(&*table, 0, 22);
    let chpx = [0, 14, 15].iter().map(|cp| {
        let span = spans.iter().find(|span| span.start_cp <= *cp && *cp < span.end_cp).unwrap();
        json!({
            "queryCp": cp,
            "startCp": span.start_cp,
            "endCp": span.end_cp,
            "font": span.property.getString__int_string(3, ""),
            "eastAsiaFont": span.property.getString__int_string(4, ""),
            "property35": span.property.getString__int_string(35, ""),
            "property36": span.property.getString__int_string(36, "")
        })
    }).collect::<Vec<_>>();
    let observed = json!({
        "themePresent": true,
        "packagePartPresent": theme.getPackagePart().is_some(),
        "schemeName": scheme.getName().unwrap_or_default(),
        "major": collection(scheme.getMajorFont()),
        "minor": collection(scheme.getMinorFont()),
        "chpx": chpx
    });
    if let Ok(path) = std::env::var("THEME_FONT_PROBE_OUTPUT") {
        std::fs::write(path, serde_json::to_vec_pretty(&observed).unwrap()).unwrap();
    }
    let java: Value = serde_json::from_slice(&std::fs::read(
        "rewrite-review/milestones/layout-java-pipeline/evidence/sample2/m3-09k/java/theme-fonts.json"
    ).unwrap()).unwrap();
    assert_eq!(observed, java);
}
