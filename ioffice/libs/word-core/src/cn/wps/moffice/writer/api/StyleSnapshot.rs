//! Read-only presentation metadata extracted from the DOCX owned by a session.
//! The editable document and layout remain in `TextDocument`; this is a versioned
//! projection for the product's style gallery, not another document model.

use std::collections::BTreeSet;
use std::io::{Cursor, Read};

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StyleSnapshot {
    pub schema_version: u32,
    pub styles: Vec<StyleInfo>,
    pub used_style_ids: Vec<String>,
    pub active_style_key: Option<String>,
    pub default_size_half_points: i32,
    pub default_color: Option<String>,
    pub default_bold: bool,
    pub default_italic: bool,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StyleInfo {
    pub style_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub is_default: bool,
    pub q_format: bool,
    pub semi_hidden: bool,
    pub unhide_when_used: bool,
    pub linked_char_shell: bool,
    pub ui_priority: i32,
    pub font_family: Option<String>,
    pub size_half_points: Option<i32>,
    pub color: Option<String>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub caps: Option<String>,
}

fn local(bytes: &[u8]) -> &[u8] {
    bytes.rsplit(|b| *b == b':').next().unwrap_or(bytes)
}

fn attr(e: &BytesStart<'_>, key: &[u8]) -> Option<String> {
    e.attributes().flatten().find_map(|a| {
        (local(a.key.as_ref()) == key)
            .then(|| a.unescape_value().ok().map(|value| value.into_owned()))
            .flatten()
    })
}

fn enabled(e: &BytesStart<'_>) -> bool {
    !matches!(attr(e, b"val").as_deref(), Some("0" | "false" | "off"))
}

fn fill_run_style(style: &mut StyleInfo, e: &BytesStart<'_>) {
    match local(e.name().as_ref()) {
        b"rFonts" => style.font_family = attr(e, b"ascii").or_else(|| attr(e, b"hAnsi")),
        b"sz" => style.size_half_points = attr(e, b"val").and_then(|x| x.parse().ok()),
        b"color" => style.color = attr(e, b"val"),
        b"b" => style.bold = enabled(e),
        b"i" => style.italic = enabled(e),
        b"u" => style.underline = enabled(e),
        b"strike" | b"dstrike" => style.strike = enabled(e),
        b"caps" => {
            if enabled(e) {
                style.caps = Some("all".into());
            }
        }
        _ => {}
    }
}

fn parse_styles(xml: &[u8]) -> StyleSnapshot {
    let mut result = StyleSnapshot {
        schema_version: 1,
        default_size_half_points: 22,
        ..Default::default()
    };
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut current: Option<StyleInfo> = None;
    let mut in_run = false;
    let mut in_defaults = false;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                let name = e.name();
                let tag = local(name.as_ref());
                if tag == b"style" {
                    current = Some(StyleInfo {
                        style_id: attr(e, b"styleId").unwrap_or_default(),
                        kind: attr(e, b"type").unwrap_or_default(),
                        is_default: matches!(attr(e, b"default").as_deref(), Some("1" | "true")),
                        ..Default::default()
                    });
                } else if tag == b"docDefaults" {
                    in_defaults = true;
                } else if tag == b"rPr" {
                    in_run = true;
                } else if let Some(style) = current.as_mut() {
                    match tag {
                        b"name" => style.name = attr(e, b"val").unwrap_or_default(),
                        b"qFormat" => style.q_format = enabled(e),
                        b"semiHidden" => style.semi_hidden = enabled(e),
                        b"unhideWhenUsed" => style.unhide_when_used = enabled(e),
                        b"uiPriority" => {
                            style.ui_priority =
                                attr(e, b"val").and_then(|x| x.parse().ok()).unwrap_or(0)
                        }
                        b"link" => style.linked_char_shell = style.kind == "character",
                        _ if in_run => fill_run_style(style, e),
                        _ => {}
                    }
                } else if in_defaults && in_run {
                    let mut defaults = StyleInfo::default();
                    fill_run_style(&mut defaults, e);
                    if let Some(v) = defaults.size_half_points {
                        result.default_size_half_points = v;
                    }
                    if defaults.color.is_some() {
                        result.default_color = defaults.color;
                    }
                    result.default_bold |= defaults.bold;
                    result.default_italic |= defaults.italic;
                }
            }
            Ok(Event::End(ref e)) => match local(e.name().as_ref()) {
                b"style" => {
                    if let Some(style) = current.take() {
                        if !style.style_id.is_empty() {
                            result.styles.push(StyleInfo {
                                name: if style.name.is_empty() {
                                    style.style_id.clone()
                                } else {
                                    style.name.clone()
                                },
                                ..style
                            });
                        }
                    }
                }
                b"rPr" => in_run = false,
                b"docDefaults" => in_defaults = false,
                _ => {}
            },
            Ok(Event::Eof) | Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    result
}

fn used_styles(xml: &[u8]) -> Vec<String> {
    let mut reader = Reader::from_reader(xml);
    let mut buf = Vec::new();
    let mut used = BTreeSet::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                if matches!(local(e.name().as_ref()), b"pStyle" | b"rStyle") {
                    if let Some(id) = attr(e, b"val") {
                        used.insert(id);
                    }
                }
            }
            Ok(Event::Eof) | Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    used.into_iter().collect()
}

/// The DOCX package has already passed through `open_text_document`; missing
/// optional styles metadata therefore produces an empty snapshot, not an error.
pub fn from_docx(docx: &[u8]) -> StyleSnapshot {
    let Ok(mut archive) = zip::ZipArchive::new(Cursor::new(docx)) else {
        return StyleSnapshot::default();
    };
    let mut snapshot = StyleSnapshot {
        schema_version: 1,
        default_size_half_points: 22,
        ..Default::default()
    };
    if let Ok(mut part) = archive.by_name("word/styles.xml") {
        let mut xml = Vec::new();
        if part.read_to_end(&mut xml).is_ok() {
            snapshot = parse_styles(&xml);
        }
    }
    if let Ok(mut part) = archive.by_name("word/document.xml") {
        let mut xml = Vec::new();
        if part.read_to_end(&mut xml).is_ok() {
            snapshot.used_style_ids = used_styles(&xml);
        }
    }
    snapshot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_quick_style_flags_and_used_ids() {
        let xml = br#"<w:styles xmlns:w="x"><w:docDefaults><w:rPrDefault><w:rPr><w:sz w:val="24"/></w:rPr></w:rPrDefault></w:docDefaults><w:style w:type="paragraph" w:styleId="Normal" w:default="1"><w:name w:val="Normal"/><w:qFormat/></w:style><w:style w:type="paragraph" w:styleId="CustomTitle"><w:name w:val="Custom Title"/><w:qFormat/><w:uiPriority w:val="3"/><w:semiHidden/><w:unhideWhenUsed/><w:rPr><w:b/><w:sz w:val="36"/><w:color w:val="FF0000"/></w:rPr></w:style></w:styles>"#;
        let result = parse_styles(xml);
        assert_eq!(result.default_size_half_points, 24);
        assert_eq!(result.styles.len(), 2);
        assert_eq!(result.styles[1].style_id, "CustomTitle");
        assert!(
            result.styles[1].q_format
                && result.styles[1].semi_hidden
                && result.styles[1].unhide_when_used
        );
        assert_eq!(result.styles[1].size_half_points, Some(36));
        assert_eq!(used_styles(br#"<w:document><w:pStyle w:val="CustomTitle"/><w:rStyle w:val="Emphasis"/></w:document>"#), vec!["CustomTitle", "Emphasis"]);
    }

    #[test]
    fn reads_styles_from_real_docx_package() {
        let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../samples/docx-mvp-render/flutter/assets/sample1/sample2.docx");
        let bytes = std::fs::read(file).unwrap();
        let snapshot = from_docx(&bytes);
        assert_eq!(snapshot.schema_version, 1);
        assert!(snapshot
            .styles
            .iter()
            .any(|s| s.style_id == "1" && s.name == "Normal"));
    }
}
