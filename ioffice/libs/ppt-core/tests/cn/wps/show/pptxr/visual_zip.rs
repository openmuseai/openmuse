use std::io::{Cursor, Write};

use zip::write::SimpleFileOptions;
use zip::ZipWriter;

pub const NS: &str = r#"xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships""#;
pub const REL_NS: &str = r#"xmlns="http://schemas.openxmlformats.org/package/2006/relationships""#;
pub const SLIDE_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide";
pub const LAYOUT_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout";
pub const MASTER_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster";
pub const THEME_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";
pub const IMAGE_TYPE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
/// ★W6c-6 —— OOXML 包的**根关系**类型。`_rels/.rels` 里的这条关系是
/// `OxfrPackage::getOfficeDocument()` 定位 `ppt/presentation.xml` 的唯一入口
/// （Java `PptxReader.getPresPart` 的首步）。原 fixture 只写了
/// `ppt/_rels/presentation.xml.rels`，**漏了根 `_rels/.rels`**
/// ⇒ `getOfficeDocument()` 恒为 `None` ⇒ `getPresPart` 抛
/// `FileDamaged("Not OOXML file")`，使约 20 个走 `open_pptx_document_bytes`
/// 的目标全部失败（长期被记为"先存失败/visual fixture 路径问题"）。
/// owner: tests.show.pptxr.visual_zip    wave: M-PPT-KMOSLIDE-W6c-6
pub const OFFICE_DOCUMENT_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";

/// OOXML 包根关系部件 `_rels/.rels`（每个 PPTX 必备）。
pub fn root_rels() -> String {
    format!(
        r#"<Relationships {REL_NS}><Relationship Id="rIdPres" Type="{OFFICE_DOCUMENT_TYPE}" Target="ppt/presentation.xml"/></Relationships>"#
    )
}

pub const PNG_1X1: &[u8] = &[
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53,
    0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08, 0xD7, 0x63, 0xF8, 0xCF, 0xC0, 0x00,
    0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05, 0xFE, 0xD4, 0xEF, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45,
    0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
];

pub fn zip_pptx(files: &[(&str, &[u8])]) -> Vec<u8> {
    let mut buf = Cursor::new(Vec::new());
    // rust: ★W6c-6 —— **自动补齐 OOXML 根关系 `_rels/.rels`**。
    //   合法 OOXML 包必备该部件（`OxfrPackage::getOfficeDocument()` → `PptxReader::getPresPart`
    //   的首步）。历史上各测试各自拼 zip 时普遍漏写它 ⇒ `open_pptx_document_bytes`
    //   恒返回 `FileDamaged("Not OOXML file")`，约 20 个目标被误记为"先存失败"。
    //   在此统一补齐：调用方若已显式提供 `_rels/.rels`（或自带其它根关系），不覆盖。
    //   owner: tests.show.pptxr.visual_zip    wave: M-PPT-KMOSLIDE-W6c-6
    let has_root = files.iter().any(|(n, _)| *n == "_rels/.rels");
    let root = root_rels();
    {
        let mut z = ZipWriter::new(&mut buf);
        let opt = SimpleFileOptions::default();
        if !has_root {
            z.start_file("_rels/.rels", opt).unwrap();
            z.write_all(root.as_bytes()).unwrap();
        }
        for (name, body) in files {
            z.start_file(*name, opt).unwrap();
            z.write_all(body).unwrap();
        }
        z.finish().unwrap();
    }
    buf.into_inner()
}

pub fn zip_pptx_str(files: &[(&str, &str)]) -> Vec<u8> {
    let owned: Vec<(String, Vec<u8>)> = files
        .iter()
        .map(|(n, b)| ((*n).to_string(), b.as_bytes().to_vec()))
        .collect();
    let refs: Vec<(&str, &[u8])> = owned.iter().map(|(n, b)| (n.as_str(), b.as_slice())).collect();
    zip_pptx(&refs)
}

pub fn presentation_xml() -> String {
    format!(
        r#"<p:presentation {NS}><p:sldSz cx="12192000" cy="6858000"/><p:sldIdLst><p:sldId id="256" r:id="rId1"/></p:sldIdLst></p:presentation>"#
    )
}

pub fn presentation_rels(extra: &str) -> String {
    format!(
        r#"<Relationships {REL_NS}><Relationship Id="rId1" Type="{SLIDE_TYPE}" Target="slides/slide1.xml"/>{extra}</Relationships>"#
    )
}

pub fn theme_xml() -> String {
    r#"<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"><a:themeElements><a:clrScheme name="Office"><a:dk1><a:srgbClr val="000000"/></a:dk1><a:lt1><a:srgbClr val="FFFFFF"/></a:lt1><a:dk2><a:srgbClr val="1F497D"/></a:dk2><a:lt2><a:srgbClr val="EEECE1"/></a:lt2><a:accent1><a:srgbClr val="4472C4"/></a:accent1></a:clrScheme></a:themeElements></a:theme>"#.into()
}

pub fn layout_xml() -> String {
    format!(
        r#"<p:sldLayout {NS}><p:cSld><p:spTree><p:sp><p:nvSpPr><p:cNvPr id="2" name="Title"/><p:cNvSpPr/><p:nvPr><p:ph type="ctrTitle" idx="0"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="100" y="200"/><a:ext cx="3000" cy="400"/></a:xfrm><a:solidFill><a:srgbClr val="112233"/></a:solidFill></p:spPr><p:txBody><a:p><a:r><a:t>Layout Title</a:t></a:r></a:p></p:txBody></p:sp></p:spTree></p:cSld></p:sldLayout>"#
    )
}

pub fn master_xml() -> String {
    format!(
        r#"<p:sldMaster {NS}><p:cSld><p:spTree><p:sp><p:nvSpPr><p:cNvPr id="2" name="Title"/><p:cNvSpPr/><p:nvPr><p:ph type="title" idx="0"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="1" y="1"/><a:ext cx="2" cy="2"/></a:xfrm></p:spPr></p:sp></p:spTree></p:cSld></p:sldMaster>"#
    )
}

pub fn visual_pptx(slide_body: &str, with_layout: bool, with_theme: bool, with_image: bool) -> Vec<u8> {
    let mut files: Vec<(String, Vec<u8>)> = vec![
        ("[Content_Types].xml".into(), b"<Types/>".to_vec()),
        // rust: ★W6c-6 —— 根关系（见 `OFFICE_DOCUMENT_TYPE` 注）。
        ("_rels/.rels".into(), root_rels().into_bytes()),
        ("ppt/presentation.xml".into(), presentation_xml().into_bytes()),
    ];
    let mut pres_extra = String::new();
    if with_theme {
        pres_extra.push_str(&format!(
            r#"<Relationship Id="rIdTheme" Type="{THEME_TYPE}" Target="theme/theme1.xml"/>"#
        ));
        files.push(("ppt/theme/theme1.xml".into(), theme_xml().into_bytes()));
    }
    files.push((
        "ppt/_rels/presentation.xml.rels".into(),
        presentation_rels(&pres_extra).into_bytes(),
    ));
    files.push((
        "ppt/slides/slide1.xml".into(),
        format!(r#"<p:sld {NS}><p:cSld><p:spTree>{slide_body}</p:spTree></p:cSld></p:sld>"#)
            .into_bytes(),
    ));
    let mut slide_rels = String::new();
    if with_layout {
        slide_rels.push_str(&format!(
            r#"<Relationship Id="rIdL" Type="{LAYOUT_TYPE}" Target="../slideLayouts/slideLayout1.xml"/>"#
        ));
        files.push(("ppt/slideLayouts/slideLayout1.xml".into(), layout_xml().into_bytes()));
        files.push((
            "ppt/slideLayouts/_rels/slideLayout1.xml.rels".into(),
            format!(
                r#"<Relationships {REL_NS}><Relationship Id="rIdM" Type="{MASTER_TYPE}" Target="../slideMasters/slideMaster1.xml"/></Relationships>"#
            )
            .into_bytes(),
        ));
        files.push(("ppt/slideMasters/slideMaster1.xml".into(), master_xml().into_bytes()));
    }
    if with_image {
        slide_rels.push_str(&format!(
            r#"<Relationship Id="rIdImg" Type="{IMAGE_TYPE}" Target="../media/image1.png"/>"#
        ));
        files.push(("ppt/media/image1.png".into(), PNG_1X1.to_vec()));
    }
    files.push((
        "ppt/slides/_rels/slide1.xml.rels".into(),
        format!(r#"<Relationships {REL_NS}>{slide_rels}</Relationships>"#).into_bytes(),
    ));
    let refs: Vec<(&str, &[u8])> = files.iter().map(|(n, b)| (n.as_str(), b.as_slice())).collect();
    zip_pptx(&refs)
}

/// Same as `visual_pptx` with optional `p:sld` / `p:sldLayout` attributes (e.g. ` showMasterSp="0"`).
pub fn visual_pptx_show_master(slide_body: &str, sld_attrs: &str, layout_attrs: &str) -> Vec<u8> {
    let mut files: Vec<(String, Vec<u8>)> = vec![
        ("[Content_Types].xml".into(), b"<Types/>".to_vec()),
        // rust: ★W6c-6 —— 根关系（见 `OFFICE_DOCUMENT_TYPE` 注）。
        ("_rels/.rels".into(), root_rels().into_bytes()),
        ("ppt/presentation.xml".into(), presentation_xml().into_bytes()),
    ];
    files.push((
        "ppt/_rels/presentation.xml.rels".into(),
        presentation_rels("").into_bytes(),
    ));
    files.push((
        "ppt/slides/slide1.xml".into(),
        format!(r#"<p:sld {NS}{sld_attrs}><p:cSld><p:spTree>{slide_body}</p:spTree></p:cSld></p:sld>"#)
            .into_bytes(),
    ));
    let layout = format!(
        r#"<p:sldLayout {NS}{layout_attrs}><p:cSld><p:spTree><p:sp><p:nvSpPr><p:cNvPr id="2" name="Title"/><p:cNvSpPr/><p:nvPr><p:ph type="ctrTitle" idx="0"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="100" y="200"/><a:ext cx="3000" cy="400"/></a:xfrm><a:solidFill><a:srgbClr val="112233"/></a:solidFill></p:spPr><p:txBody><a:p><a:r><a:t>Layout Title</a:t></a:r></a:p></p:txBody></p:sp></p:spTree></p:cSld></p:sldLayout>"#
    );
    files.push(("ppt/slideLayouts/slideLayout1.xml".into(), layout.into_bytes()));
    files.push((
        "ppt/slideLayouts/_rels/slideLayout1.xml.rels".into(),
        format!(
            r#"<Relationships {REL_NS}><Relationship Id="rIdM" Type="{MASTER_TYPE}" Target="../slideMasters/slideMaster1.xml"/></Relationships>"#
        )
        .into_bytes(),
    ));
    files.push(("ppt/slideMasters/slideMaster1.xml".into(), master_xml().into_bytes()));
    files.push((
        "ppt/slides/_rels/slide1.xml.rels".into(),
        format!(
            r#"<Relationships {REL_NS}><Relationship Id="rIdL" Type="{LAYOUT_TYPE}" Target="../slideLayouts/slideLayout1.xml"/></Relationships>"#
        )
        .into_bytes(),
    ));
    let refs: Vec<(&str, &[u8])> = files.iter().map(|(n, b)| (n.as_str(), b.as_slice())).collect();
    zip_pptx(&refs)
}
