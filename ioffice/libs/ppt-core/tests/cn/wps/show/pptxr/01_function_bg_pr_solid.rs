//! pp-17-01 p:bgPr 实色。矩阵 FUNCTION.md T01–T04。

#[path = "visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes;
use ppt_core::cn::wps::show::pptxr::pptx_mvp::parse_bg_pr_solid_fill;
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
    files.push((
        "ppt/_rels/presentation.xml.rels",
        presentation_rels(&extra),
    ));
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
fn t01_bgpr_srgb_sets_has_background() {
    let csld = r#"<p:bg><p:bgPr><a:solidFill><a:srgbClr val="112233"/></a:solidFill></p:bgPr></p:bg><p:spTree></p:spTree>"#;
    let doc = open_pptx_document_bytes(&slide_zip(csld, false)).unwrap();
    assert!(doc.slides[0].has_background);
    assert_eq!(doc.slides[0].bg_fill, Some(0xFF112233));
}

#[test]
fn t02_bgpr_schemeclr_uses_theme() {
    let csld = r#"<p:bg><p:bgPr><a:solidFill><a:schemeClr val="tx1"/></a:solidFill></p:bgPr></p:bg><p:spTree></p:spTree>"#;
    let doc = open_pptx_document_bytes(&slide_zip(csld, true)).unwrap();
    assert!(doc.slides[0].has_background);
    assert!(doc.slides[0].bg_fill.is_some());
}

#[test]
fn t03_no_p_bg_stays_false() {
    let csld = r#"<p:spTree></p:spTree>"#;
    let doc = open_pptx_document_bytes(&slide_zip(csld, false)).unwrap();
    assert!(!doc.slides[0].has_background);
    assert!(doc.slides[0].bg_fill.is_none());
}

#[test]
fn t04_bgref_only_not_this_function() {
    let xml = r#"<p:sld><p:cSld><p:bg><p:bgRef idx="1001"><a:schemeClr val="bg1"/></p:bgRef></p:bg></p:cSld></p:sld>"#;
    assert!(parse_bg_pr_solid_fill(xml, &Default::default()).is_none());
}
