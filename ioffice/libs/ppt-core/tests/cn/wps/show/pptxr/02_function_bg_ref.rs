//! pp-17-02 p:bgRef。矩阵 FUNCTION.md T01–T03。

#[path = "visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use visual_zip::{presentation_rels, presentation_xml, theme_xml, zip_pptx_str, NS, REL_NS, THEME_TYPE};

fn slide_zip(csld: &str, with_theme: bool) -> Vec<u8> {
    let mut extra = String::new();
    let mut files: Vec<(&str, String)> = vec![
        ("[Content_Types].xml", "<Types/>".into()),
        ("ppt/presentation.xml", presentation_xml()),
    ];
    if with_theme {
        extra = format!(
            r#"<Relationship Id="rIdTheme" Type="{THEME_TYPE}" Target="theme/theme1.xml"/>"#
        );
        files.push(("ppt/theme/theme1.xml", theme_xml()));
    }
    files.push(("ppt/_rels/presentation.xml.rels", presentation_rels(&extra)));
    files.push((
        "ppt/slides/slide1.xml",
        format!(r#"<p:sld {NS}><p:cSld>{csld}</p:cSld></p:sld>"#),
    ));
    files.push((
        "ppt/slides/_rels/slide1.xml.rels",
        format!(r#"<Relationships {REL_NS}></Relationships>"#),
    ));
    let refs: Vec<(&str, &str)> = files.iter().map(|(n, b)| (*n, b.as_str())).collect();
    zip_pptx_str(&refs)
}

#[test]
fn t01_bgref_with_theme() {
    let csld = r#"<p:bg><p:bgRef idx="1001"><a:schemeClr val="lt1"/></p:bgRef></p:bg><p:spTree></p:spTree>"#;
    let doc = open_pptx_document_bytes(&slide_zip(csld, true)).unwrap();
    assert!(doc.slides[0].has_background);
    assert!(doc.slides[0].bg_fill.is_some());
}

#[test]
fn t02_bad_idx() {
    let csld = r#"<p:bg><p:bgRef idx="99999"/></p:bg><p:spTree></p:spTree>"#;
    let doc = open_pptx_document_bytes(&slide_zip(csld, true)).unwrap();
    assert!(!doc.slides[0].has_background);
}

#[test]
fn t03_bgpr_wins_over_bgref() {
    let csld = r#"<p:bg><p:bgPr><a:solidFill><a:srgbClr val="AABBCC"/></a:solidFill></p:bgPr><p:bgRef idx="1001"><a:schemeClr val="lt1"/></p:bgRef></p:bg><p:spTree></p:spTree>"#;
    let doc = open_pptx_document_bytes(&slide_zip(csld, true)).unwrap();
    assert_eq!(doc.slides[0].bg_fill, Some(0xFFAABBCC));
}
