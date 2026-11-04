use std::fs;

use word_core_ffi::api::{layout_document, FontBlob, RenderCommandWire, ViewParams};

#[test]
fn sample2_ffi_uses_java_layout_lines() {
    let docx = fs::read("../flutter/assets/sample1/sample2.docx").expect("sample2.docx");
    let fonts = vec![
        FontBlob {
            id: "default".into(),
            bytes: fs::read("../../../libs/word-core/assets/MvpLatin.ttf").expect("MvpLatin"),
        },
        FontBlob {
            // Glyph fallback, not a registered 宋体 source family on the Java device.
            id: "CJKFallback".into(),
            bytes: fs::read("/System/Library/Fonts/Supplemental/Arial Unicode.ttf")
                .expect("CJK face"),
        },
    ];
    let frame = layout_document(docx, fonts, ViewParams::default_a4()).expect("FFI layout");
    let title = frame.commands.iter().find_map(|command| {
        if let RenderCommandWire::Text { page: 0, x, text, font_id, .. } = command {
            Some((*x, text.as_str(), font_id.as_str()))
        } else {
            None
        }
    }).expect("first page title");
    // Java PKM110 Stage2 title x=1789 relative to the 1800 twip page margin.
    assert_eq!(title.0, 3589);
    assert!(title.1.contains("大学生"));
    assert_eq!(title.2, "CJKFallback");
    let texts: Vec<(i32, &str)> = frame.commands.iter().filter_map(|command| {
        if let RenderCommandWire::Text { page, text, .. } = command {
            Some((*page, text.as_str()))
        } else {
            None
        }
    }).collect();
    let start = texts.iter().position(|(_, text)| text.contains("近年来")).expect("body start");
    let end = texts.iter().position(|(_, text)| text.contains("然而")).expect("next para");
    assert_eq!(frame.page.page_count, 8);
    assert!(end - start >= 3, "body commands: {:?}", &texts[start..=end]);
    assert!(!texts.iter().any(|(page, text)| *page == 0 && text.contains("三、农村中小学教育现状")));
}
