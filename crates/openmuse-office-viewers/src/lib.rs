//! Bounded, network-free view engines for Mobile Office formats.

use quick_xml::{Reader, events::Event};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    io::{Cursor, Read},
};
use zip::ZipArchive;

pub const ABI_VERSION: u32 = 1;
pub const MAX_ARCHIVE_BYTES: usize = 64 * 1024 * 1024;
pub const MAX_ENTRY_BYTES: u64 = 32 * 1024 * 1024;
pub const MAX_TOTAL_UNCOMPRESSED_BYTES: u64 = 128 * 1024 * 1024;
pub const MAX_ENTRIES: usize = 4096;
pub const MAX_CELLS: usize = 1_000_000;
pub const MAX_CELL_BYTES: usize = 1024 * 1024;
pub const MAX_PDF_OBJECTS: usize = 200_000;
pub const MAX_PDF_PAGES: usize = 10_000;
pub const MAX_PDF_TEXT_BYTES: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewerInspection {
    pub schema: &'static str,
    pub profile: &'static str,
    pub paragraphs: Vec<String>,
    pub capabilities: Vec<&'static str>,
}

#[derive(Debug)]
struct PackagePart {
    name: String,
    bytes: Vec<u8>,
}

pub fn inspect_xlsx(bytes: &[u8]) -> Result<ViewerInspection, ViewerError> {
    let parts = read_parts(bytes)?;
    for required in [
        "[Content_Types].xml",
        "_rels/.rels",
        "xl/workbook.xml",
        "xl/_rels/workbook.xml.rels",
    ] {
        if !parts.iter().any(|part| part.name == required) {
            return Err(ViewerError::MissingPart(required));
        }
    }
    for part in parts.iter().filter(|part| part.name.ends_with(".rels")) {
        reject_external_relationships(&part.bytes)?;
    }
    let shared = match parts
        .iter()
        .find(|part| part.name == "xl/sharedStrings.xml")
    {
        Some(part) => parse_shared_strings(&part.bytes)?,
        None => Vec::new(),
    };
    let workbook = parts
        .iter()
        .find(|part| part.name == "xl/workbook.xml")
        .expect("required part checked");
    let workbook_relationships = parts
        .iter()
        .find(|part| part.name == "xl/_rels/workbook.xml.rels")
        .expect("required part checked");
    let sheets = parse_workbook(&workbook.bytes)?;
    let targets = parse_workbook_relationships(&workbook_relationships.bytes)?;
    let mut paragraphs = Vec::new();
    let mut cells = 0_usize;
    for (sheet_name, relationship_id) in sheets {
        let target = targets
            .get(&relationship_id)
            .ok_or(ViewerError::InvalidRelationship)?;
        let worksheet = parts
            .iter()
            .find(|part| &part.name == target)
            .ok_or(ViewerError::InvalidRelationship)?;
        parse_worksheet(
            &sheet_name,
            &worksheet.bytes,
            &shared,
            &mut cells,
            &mut paragraphs,
        )?;
    }
    Ok(ViewerInspection {
        schema: "openmuse.office.xlsx-inspection@1",
        profile: "view-only",
        paragraphs,
        capabilities: vec!["view"],
    })
}

pub fn inspect_pptx(bytes: &[u8]) -> Result<ViewerInspection, ViewerError> {
    let parts = read_parts(bytes)?;
    for required in [
        "[Content_Types].xml",
        "_rels/.rels",
        "ppt/presentation.xml",
        "ppt/_rels/presentation.xml.rels",
    ] {
        if !parts.iter().any(|part| part.name == required) {
            return Err(ViewerError::MissingPart(required));
        }
    }
    for part in parts.iter().filter(|part| part.name.ends_with(".rels")) {
        reject_external_relationships(&part.bytes)?;
    }
    let presentation = parts
        .iter()
        .find(|part| part.name == "ppt/presentation.xml")
        .expect("required part checked");
    let presentation_relationships = parts
        .iter()
        .find(|part| part.name == "ppt/_rels/presentation.xml.rels")
        .expect("required part checked");
    let slide_ids = parse_slide_order(&presentation.bytes)?;
    let targets = parse_presentation_relationships(&presentation_relationships.bytes)?;
    let mut paragraphs = Vec::new();
    let mut text_runs = 0_usize;
    for (index, relationship_id) in slide_ids.iter().enumerate() {
        let target = targets
            .get(relationship_id)
            .ok_or(ViewerError::InvalidRelationship)?;
        let slide = parts
            .iter()
            .find(|part| &part.name == target)
            .ok_or(ViewerError::InvalidRelationship)?;
        parse_slide(index + 1, &slide.bytes, &mut text_runs, &mut paragraphs)?;
    }
    Ok(ViewerInspection {
        schema: "openmuse.office.pptx-inspection@1",
        profile: "view-only",
        paragraphs,
        capabilities: vec!["view"],
    })
}

pub fn inspect_pdf(bytes: &[u8]) -> Result<ViewerInspection, ViewerError> {
    if bytes.is_empty() || bytes.len() > MAX_ARCHIVE_BYTES || !bytes.starts_with(b"%PDF-") {
        return Err(ViewerError::InvalidPdf);
    }
    let document = lopdf::Document::load_mem(bytes).map_err(|_| ViewerError::InvalidPdf)?;
    if document.is_encrypted() {
        return Err(ViewerError::EncryptedPdfDenied);
    }
    if document.objects.len() > MAX_PDF_OBJECTS {
        return Err(ViewerError::LimitExceeded);
    }
    for object in document.objects.values() {
        reject_active_pdf_object(object)?;
    }
    let pages = document.get_pages();
    if pages.is_empty() || pages.len() > MAX_PDF_PAGES {
        return Err(ViewerError::LimitExceeded);
    }
    let mut paragraphs = Vec::new();
    let mut total_text = 0_usize;
    for page_number in pages.keys() {
        let text = document
            .extract_text(&[*page_number])
            .map_err(|_| ViewerError::InvalidPdf)?;
        let mut emitted = false;
        for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
            if line.contains('\0') || line.len() > MAX_CELL_BYTES {
                return Err(ViewerError::LimitExceeded);
            }
            total_text = total_text
                .checked_add(line.len())
                .ok_or(ViewerError::LimitExceeded)?;
            if total_text > MAX_PDF_TEXT_BYTES {
                return Err(ViewerError::LimitExceeded);
            }
            paragraphs.push(format!("Page {page_number}\t{line}"));
            emitted = true;
        }
        if !emitted {
            paragraphs.push(format!("Page {page_number}\t[无可提取文本]"));
        }
    }
    Ok(ViewerInspection {
        schema: "openmuse.office.pdf-inspection@1",
        profile: "text-view-only",
        paragraphs,
        capabilities: vec!["view"],
    })
}

fn reject_active_pdf_object(object: &lopdf::Object) -> Result<(), ViewerError> {
    match object {
        lopdf::Object::Dictionary(dictionary) => reject_active_pdf_dictionary(dictionary),
        lopdf::Object::Stream(stream) => reject_active_pdf_dictionary(&stream.dict),
        lopdf::Object::Array(values) => {
            for value in values {
                reject_active_pdf_object(value)?;
            }
            Ok(())
        }
        lopdf::Object::Name(name) if denied_pdf_name(name) => Err(ViewerError::ActivePdfDenied),
        _ => Ok(()),
    }
}

fn reject_active_pdf_dictionary(dictionary: &lopdf::Dictionary) -> Result<(), ViewerError> {
    for (key, value) in dictionary.iter() {
        if denied_pdf_name(key) {
            return Err(ViewerError::ActivePdfDenied);
        }
        reject_active_pdf_object(value)?;
    }
    Ok(())
}

fn denied_pdf_name(value: &[u8]) -> bool {
    const DENIED: &[&[u8]] = &[
        b"AA",
        b"AcroForm",
        b"EmbeddedFile",
        b"Filespec",
        b"JavaScript",
        b"JS",
        b"Launch",
        b"OpenAction",
        b"RichMedia",
        b"URI",
        b"XFA",
    ];
    DENIED.contains(&value)
}

fn parse_slide_order(xml: &[u8]) -> Result<Vec<String>, ViewerError> {
    let mut reader = Reader::from_reader(xml);
    let mut ids = Vec::new();
    loop {
        match reader.read_event().map_err(|_| ViewerError::InvalidXml)? {
            Event::Start(tag) | Event::Empty(tag)
                if local_name(tag.name().as_ref()) == b"sldId" =>
            {
                let mut relationship_id = None;
                for attribute in tag.attributes().with_checks(true) {
                    let attribute = attribute.map_err(|_| ViewerError::InvalidXml)?;
                    if local_name(attribute.key.as_ref()) == b"id"
                        && attribute.key.as_ref().contains(&b':')
                    {
                        relationship_id = Some(
                            attribute
                                .decode_and_unescape_value(reader.decoder())
                                .map_err(|_| ViewerError::InvalidXml)?
                                .into_owned(),
                        );
                    }
                }
                ids.push(relationship_id.ok_or(ViewerError::InvalidRelationship)?);
                if ids.len() > 10_000 {
                    return Err(ViewerError::LimitExceeded);
                }
            }
            Event::DocType(_) => return Err(ViewerError::DocTypeDenied),
            Event::Eof if ids.is_empty() => return Err(ViewerError::InvalidRelationship),
            Event::Eof => return Ok(ids),
            _ => {}
        }
    }
}

fn parse_presentation_relationships(xml: &[u8]) -> Result<HashMap<String, String>, ViewerError> {
    let mut reader = Reader::from_reader(xml);
    let mut relationships = HashMap::new();
    loop {
        match reader.read_event().map_err(|_| ViewerError::InvalidXml)? {
            Event::Start(tag) | Event::Empty(tag)
                if local_name(tag.name().as_ref()) == b"Relationship" =>
            {
                let mut id = None;
                let mut target = None;
                let mut relationship_type = None;
                let mut external = false;
                for attribute in tag.attributes().with_checks(true) {
                    let attribute = attribute.map_err(|_| ViewerError::InvalidXml)?;
                    let value = attribute
                        .decode_and_unescape_value(reader.decoder())
                        .map_err(|_| ViewerError::InvalidXml)?
                        .into_owned();
                    match local_name(attribute.key.as_ref()) {
                        b"Id" => id = Some(value),
                        b"Target" => target = Some(value),
                        b"Type" => relationship_type = Some(value),
                        b"TargetMode" if value.eq_ignore_ascii_case("external") => external = true,
                        _ => {}
                    }
                }
                if external {
                    return Err(ViewerError::ExternalRelationship);
                }
                if relationship_type
                    .as_deref()
                    .is_some_and(|value| value.ends_with("/slide"))
                {
                    let id = id.ok_or(ViewerError::InvalidRelationship)?;
                    let target = normalize_part_target(
                        &target.ok_or(ViewerError::InvalidRelationship)?,
                        "ppt",
                        "ppt/slides/",
                    )?;
                    if relationships.insert(id, target).is_some() {
                        return Err(ViewerError::InvalidRelationship);
                    }
                }
            }
            Event::DocType(_) => return Err(ViewerError::DocTypeDenied),
            Event::Eof => return Ok(relationships),
            _ => {}
        }
    }
}

fn normalize_part_target(
    target: &str,
    root: &str,
    required_prefix: &str,
) -> Result<String, ViewerError> {
    if target.is_empty()
        || target.contains('\\')
        || target.contains(':')
        || target.split('/').any(|segment| segment == "..")
    {
        return Err(ViewerError::InvalidRelationship);
    }
    let trimmed = target.trim_start_matches('/');
    let resolved = if trimmed.starts_with(&format!("{root}/")) {
        trimmed.to_owned()
    } else {
        format!("{root}/{trimmed}")
    };
    if !resolved.starts_with(required_prefix) || !resolved.ends_with(".xml") {
        return Err(ViewerError::InvalidRelationship);
    }
    Ok(resolved)
}

fn parse_slide(
    slide_number: usize,
    xml: &[u8],
    text_runs: &mut usize,
    output: &mut Vec<String>,
) -> Result<(), ViewerError> {
    let mut reader = Reader::from_reader(xml);
    let mut paragraph = String::new();
    let mut in_paragraph = false;
    let mut in_text = false;
    let mut emitted = false;
    loop {
        match reader.read_event().map_err(|_| ViewerError::InvalidXml)? {
            Event::Start(tag) => match local_name(tag.name().as_ref()) {
                b"p" => {
                    in_paragraph = true;
                    paragraph.clear();
                }
                b"t" if in_paragraph => in_text = true,
                _ => {}
            },
            Event::Text(text) if in_text => {
                paragraph.push_str(&text.decode().map_err(|_| ViewerError::InvalidXml)?);
                if paragraph.len() > MAX_CELL_BYTES {
                    return Err(ViewerError::LimitExceeded);
                }
            }
            Event::CData(text) if in_text => {
                paragraph.push_str(&text.decode().map_err(|_| ViewerError::InvalidXml)?);
                if paragraph.len() > MAX_CELL_BYTES {
                    return Err(ViewerError::LimitExceeded);
                }
            }
            Event::End(tag) => match local_name(tag.name().as_ref()) {
                b"t" => {
                    in_text = false;
                    *text_runs = text_runs.checked_add(1).ok_or(ViewerError::LimitExceeded)?;
                    if *text_runs > MAX_CELLS {
                        return Err(ViewerError::LimitExceeded);
                    }
                }
                b"p" => {
                    in_paragraph = false;
                    if !paragraph.is_empty() {
                        output.push(format!("Slide {slide_number}\t{paragraph}"));
                        emitted = true;
                    }
                }
                _ => {}
            },
            Event::DocType(_) => return Err(ViewerError::DocTypeDenied),
            Event::Eof => {
                if !emitted {
                    output.push(format!("Slide {slide_number}"));
                }
                return Ok(());
            }
            _ => {}
        }
    }
}

fn parse_workbook(xml: &[u8]) -> Result<Vec<(String, String)>, ViewerError> {
    let mut reader = Reader::from_reader(xml);
    let mut sheets = Vec::new();
    loop {
        match reader.read_event().map_err(|_| ViewerError::InvalidXml)? {
            Event::Start(tag) | Event::Empty(tag)
                if local_name(tag.name().as_ref()) == b"sheet" =>
            {
                let mut name = None;
                let mut relationship_id = None;
                for attribute in tag.attributes().with_checks(true) {
                    let attribute = attribute.map_err(|_| ViewerError::InvalidXml)?;
                    let value = attribute
                        .decode_and_unescape_value(reader.decoder())
                        .map_err(|_| ViewerError::InvalidXml)?
                        .into_owned();
                    match local_name(attribute.key.as_ref()) {
                        b"name" => name = Some(value),
                        b"id" => relationship_id = Some(value),
                        _ => {}
                    }
                }
                let name = name.ok_or(ViewerError::InvalidRelationship)?;
                let relationship_id = relationship_id.ok_or(ViewerError::InvalidRelationship)?;
                if name.is_empty() || name.len() > 1024 || relationship_id.is_empty() {
                    return Err(ViewerError::InvalidRelationship);
                }
                sheets.push((name, relationship_id));
                if sheets.len() > 1024 {
                    return Err(ViewerError::LimitExceeded);
                }
            }
            Event::DocType(_) => return Err(ViewerError::DocTypeDenied),
            Event::Eof if sheets.is_empty() => return Err(ViewerError::InvalidRelationship),
            Event::Eof => return Ok(sheets),
            _ => {}
        }
    }
}

fn parse_workbook_relationships(xml: &[u8]) -> Result<HashMap<String, String>, ViewerError> {
    let mut reader = Reader::from_reader(xml);
    let mut relationships = HashMap::new();
    loop {
        match reader.read_event().map_err(|_| ViewerError::InvalidXml)? {
            Event::Start(tag) | Event::Empty(tag)
                if local_name(tag.name().as_ref()) == b"Relationship" =>
            {
                let mut id = None;
                let mut target = None;
                let mut external = false;
                for attribute in tag.attributes().with_checks(true) {
                    let attribute = attribute.map_err(|_| ViewerError::InvalidXml)?;
                    let value = attribute
                        .decode_and_unescape_value(reader.decoder())
                        .map_err(|_| ViewerError::InvalidXml)?
                        .into_owned();
                    match local_name(attribute.key.as_ref()) {
                        b"Id" => id = Some(value),
                        b"Target" => target = Some(value),
                        b"TargetMode" if value.eq_ignore_ascii_case("external") => external = true,
                        _ => {}
                    }
                }
                if external {
                    return Err(ViewerError::ExternalRelationship);
                }
                let id = id.ok_or(ViewerError::InvalidRelationship)?;
                let target =
                    normalize_workbook_target(&target.ok_or(ViewerError::InvalidRelationship)?)?;
                if relationships.insert(id, target).is_some() {
                    return Err(ViewerError::InvalidRelationship);
                }
            }
            Event::DocType(_) => return Err(ViewerError::DocTypeDenied),
            Event::Eof => return Ok(relationships),
            _ => {}
        }
    }
}

fn normalize_workbook_target(target: &str) -> Result<String, ViewerError> {
    if target.is_empty()
        || target.contains('\\')
        || target.contains(':')
        || target.split('/').any(|segment| segment == "..")
    {
        return Err(ViewerError::InvalidRelationship);
    }
    let trimmed = target.trim_start_matches('/');
    let resolved = if trimmed.starts_with("xl/") {
        trimmed.to_owned()
    } else {
        format!("xl/{trimmed}")
    };
    if !resolved.starts_with("xl/worksheets/") || !resolved.ends_with(".xml") {
        return Err(ViewerError::InvalidRelationship);
    }
    Ok(resolved)
}

fn read_parts(bytes: &[u8]) -> Result<Vec<PackagePart>, ViewerError> {
    if bytes.is_empty() || bytes.len() > MAX_ARCHIVE_BYTES {
        return Err(ViewerError::LimitExceeded);
    }
    let mut archive =
        ZipArchive::new(Cursor::new(bytes)).map_err(|_| ViewerError::InvalidArchive)?;
    if archive.len() > MAX_ENTRIES {
        return Err(ViewerError::LimitExceeded);
    }
    let mut total = 0_u64;
    let mut names = HashSet::new();
    let mut parts = Vec::with_capacity(archive.len());
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|_| ViewerError::InvalidArchive)?;
        let name = entry.name().to_owned();
        if name.is_empty()
            || name.starts_with('/')
            || name.starts_with('\\')
            || name.contains('\\')
            || name.split('/').any(|segment| segment == "..")
            || !names.insert(name.clone())
        {
            return Err(ViewerError::UnsafePackagePath);
        }
        if entry.is_dir() {
            continue;
        }
        total = total
            .checked_add(entry.size())
            .ok_or(ViewerError::LimitExceeded)?;
        if entry.size() > MAX_ENTRY_BYTES
            || total > MAX_TOTAL_UNCOMPRESSED_BYTES
            || (entry.compressed_size() > 0 && entry.size() / entry.compressed_size().max(1) > 200)
        {
            return Err(ViewerError::LimitExceeded);
        }
        let mut part = Vec::with_capacity(entry.size() as usize);
        entry
            .read_to_end(&mut part)
            .map_err(|_| ViewerError::InvalidArchive)?;
        parts.push(PackagePart { name, bytes: part });
    }
    Ok(parts)
}

fn reject_external_relationships(xml: &[u8]) -> Result<(), ViewerError> {
    let mut reader = Reader::from_reader(xml);
    loop {
        match reader.read_event().map_err(|_| ViewerError::InvalidXml)? {
            Event::Start(tag) | Event::Empty(tag) => {
                for attribute in tag.attributes().with_checks(true) {
                    let attribute = attribute.map_err(|_| ViewerError::InvalidXml)?;
                    if local_name(attribute.key.as_ref()) == b"TargetMode"
                        && attribute
                            .decode_and_unescape_value(reader.decoder())
                            .map_err(|_| ViewerError::InvalidXml)?
                            .eq_ignore_ascii_case("external")
                    {
                        return Err(ViewerError::ExternalRelationship);
                    }
                }
            }
            Event::DocType(_) => return Err(ViewerError::DocTypeDenied),
            Event::Eof => return Ok(()),
            _ => {}
        }
    }
}

fn parse_shared_strings(xml: &[u8]) -> Result<Vec<String>, ViewerError> {
    let mut reader = Reader::from_reader(xml);
    let mut values = Vec::new();
    let mut current = String::new();
    let mut in_item = false;
    let mut in_text = false;
    loop {
        match reader.read_event().map_err(|_| ViewerError::InvalidXml)? {
            Event::Start(tag) => match local_name(tag.name().as_ref()) {
                b"si" => {
                    in_item = true;
                    current.clear();
                }
                b"t" if in_item => in_text = true,
                _ => {}
            },
            Event::Text(text) if in_text => {
                current.push_str(&text.decode().map_err(|_| ViewerError::InvalidXml)?);
                if current.len() > MAX_CELL_BYTES {
                    return Err(ViewerError::LimitExceeded);
                }
            }
            Event::CData(text) if in_text => {
                current.push_str(&text.decode().map_err(|_| ViewerError::InvalidXml)?);
                if current.len() > MAX_CELL_BYTES {
                    return Err(ViewerError::LimitExceeded);
                }
            }
            Event::End(tag) => match local_name(tag.name().as_ref()) {
                b"t" => in_text = false,
                b"si" => {
                    values.push(std::mem::take(&mut current));
                    in_item = false;
                    if values.len() > MAX_CELLS {
                        return Err(ViewerError::LimitExceeded);
                    }
                }
                _ => {}
            },
            Event::DocType(_) => return Err(ViewerError::DocTypeDenied),
            Event::Eof => return Ok(values),
            _ => {}
        }
    }
}

fn parse_worksheet(
    name: &str,
    xml: &[u8],
    shared: &[String],
    cells: &mut usize,
    paragraphs: &mut Vec<String>,
) -> Result<(), ViewerError> {
    let mut reader = Reader::from_reader(xml);
    let mut row = Vec::<String>::new();
    let mut cell = String::new();
    let mut cell_type = String::new();
    let mut cell_column = None;
    let mut in_value = false;
    let mut in_inline_text = false;
    loop {
        match reader.read_event().map_err(|_| ViewerError::InvalidXml)? {
            Event::Start(tag) => match local_name(tag.name().as_ref()) {
                b"row" => row.clear(),
                b"c" => {
                    cell.clear();
                    cell_type.clear();
                    cell_column = None;
                    for attribute in tag.attributes().with_checks(true) {
                        let attribute = attribute.map_err(|_| ViewerError::InvalidXml)?;
                        match local_name(attribute.key.as_ref()) {
                            b"t" => {
                                cell_type = attribute
                                    .decode_and_unescape_value(reader.decoder())
                                    .map_err(|_| ViewerError::InvalidXml)?
                                    .into_owned()
                            }
                            b"r" => {
                                let reference = attribute
                                    .decode_and_unescape_value(reader.decoder())
                                    .map_err(|_| ViewerError::InvalidXml)?;
                                cell_column = column_index(&reference);
                            }
                            _ => {}
                        }
                    }
                }
                b"v" => in_value = true,
                b"t" if cell_type == "inlineStr" => in_inline_text = true,
                _ => {}
            },
            Event::Text(text) if in_value || in_inline_text => {
                cell.push_str(&text.decode().map_err(|_| ViewerError::InvalidXml)?);
                if cell.len() > MAX_CELL_BYTES {
                    return Err(ViewerError::LimitExceeded);
                }
            }
            Event::CData(text) if in_value || in_inline_text => {
                cell.push_str(&text.decode().map_err(|_| ViewerError::InvalidXml)?);
                if cell.len() > MAX_CELL_BYTES {
                    return Err(ViewerError::LimitExceeded);
                }
            }
            Event::End(tag) => match local_name(tag.name().as_ref()) {
                b"v" => in_value = false,
                b"t" => in_inline_text = false,
                b"c" => {
                    let value = if cell_type == "s" {
                        cell.parse::<usize>()
                            .ok()
                            .and_then(|index| shared.get(index))
                            .cloned()
                            .ok_or(ViewerError::InvalidSharedString)?
                    } else {
                        std::mem::take(&mut cell)
                    };
                    let column = cell_column.unwrap_or(row.len());
                    if column > MAX_CELLS || column < row.len() {
                        return Err(ViewerError::InvalidCellReference);
                    }
                    row.resize(column, String::new());
                    row.push(value);
                    *cells = cells.checked_add(1).ok_or(ViewerError::LimitExceeded)?;
                    if *cells > MAX_CELLS {
                        return Err(ViewerError::LimitExceeded);
                    }
                }
                b"row" => {
                    let trimmed = row
                        .iter()
                        .rposition(|value| !value.is_empty())
                        .map_or(0, |i| i + 1);
                    row.truncate(trimmed);
                    paragraphs.push(format!("{}\t{}", name, row.join("\t")));
                }
                _ => {}
            },
            Event::DocType(_) => return Err(ViewerError::DocTypeDenied),
            Event::Eof => return Ok(()),
            _ => {}
        }
    }
}

fn column_index(reference: &str) -> Option<usize> {
    let mut value = 0_usize;
    let mut found = false;
    for byte in reference.bytes().take_while(u8::is_ascii_alphabetic) {
        found = true;
        value = value
            .checked_mul(26)?
            .checked_add(usize::from(byte.to_ascii_uppercase().checked_sub(b'A')?) + 1)?;
    }
    found.then(|| value - 1)
}

fn local_name(name: &[u8]) -> &[u8] {
    name.rsplit(|byte| *byte == b':').next().unwrap_or(name)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ViewerError {
    #[error("invalid Office archive")]
    InvalidArchive,
    #[error("unsafe package path")]
    UnsafePackagePath,
    #[error("Office input exceeds resource limits")]
    LimitExceeded,
    #[error("missing required part {0}")]
    MissingPart(&'static str),
    #[error("invalid Office XML")]
    InvalidXml,
    #[error("XML document types are denied")]
    DocTypeDenied,
    #[error("external relationships are denied")]
    ExternalRelationship,
    #[error("invalid shared string index")]
    InvalidSharedString,
    #[error("invalid cell reference")]
    InvalidCellReference,
    #[error("invalid workbook relationship")]
    InvalidRelationship,
    #[error("invalid PDF")]
    InvalidPdf,
    #[error("encrypted PDF is denied")]
    EncryptedPdfDenied,
    #[error("active PDF content is denied")]
    ActivePdfDenied,
}

#[repr(C)]
pub struct OpenMuseOfficeViewerBuffer {
    pub ptr: *mut u8,
    pub len: usize,
    pub capacity: usize,
    pub status: i32,
}

impl OpenMuseOfficeViewerBuffer {
    fn from_vec(mut bytes: Vec<u8>, status: i32) -> Self {
        let result = Self {
            ptr: bytes.as_mut_ptr(),
            len: bytes.len(),
            capacity: bytes.capacity(),
            status,
        };
        std::mem::forget(bytes);
        result
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn openmuse_office_viewers_abi_version() -> u32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
/// Inspect a bounded XLSX buffer.
///
/// # Safety
/// `xlsx` must point to `xlsx_len` readable bytes. The returned buffer must be
/// freed exactly once with [`openmuse_office_viewer_buffer_free`].
pub unsafe extern "C" fn openmuse_xlsx_inspect(
    xlsx: *const u8,
    xlsx_len: usize,
) -> OpenMuseOfficeViewerBuffer {
    ffi_call(|| {
        if xlsx.is_null() || xlsx_len == 0 || xlsx_len > MAX_ARCHIVE_BYTES {
            return Err(ViewerError::LimitExceeded);
        }
        let bytes = unsafe { std::slice::from_raw_parts(xlsx, xlsx_len) };
        let inspection = inspect_xlsx(bytes)?;
        serde_json::to_vec(&inspection).map_err(|_| ViewerError::InvalidXml)
    })
}

#[unsafe(no_mangle)]
/// Inspect a bounded PPTX buffer.
///
/// # Safety
/// `pptx` must point to `pptx_len` readable bytes. The returned buffer must be
/// freed exactly once with [`openmuse_office_viewer_buffer_free`].
pub unsafe extern "C" fn openmuse_pptx_inspect(
    pptx: *const u8,
    pptx_len: usize,
) -> OpenMuseOfficeViewerBuffer {
    ffi_call(|| {
        if pptx.is_null() || pptx_len == 0 || pptx_len > MAX_ARCHIVE_BYTES {
            return Err(ViewerError::LimitExceeded);
        }
        let bytes = unsafe { std::slice::from_raw_parts(pptx, pptx_len) };
        let inspection = inspect_pptx(bytes)?;
        serde_json::to_vec(&inspection).map_err(|_| ViewerError::InvalidXml)
    })
}

#[unsafe(no_mangle)]
/// Inspect a bounded PDF into a text-only compatibility view.
///
/// # Safety
/// `pdf` must point to `pdf_len` readable bytes. The returned buffer must be
/// freed exactly once with [`openmuse_office_viewer_buffer_free`].
pub unsafe extern "C" fn openmuse_pdf_inspect(
    pdf: *const u8,
    pdf_len: usize,
) -> OpenMuseOfficeViewerBuffer {
    ffi_call(|| {
        if pdf.is_null() || pdf_len == 0 || pdf_len > MAX_ARCHIVE_BYTES {
            return Err(ViewerError::LimitExceeded);
        }
        let bytes = unsafe { std::slice::from_raw_parts(pdf, pdf_len) };
        let inspection = inspect_pdf(bytes)?;
        serde_json::to_vec(&inspection).map_err(|_| ViewerError::InvalidPdf)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn openmuse_office_viewer_buffer_free(buffer: OpenMuseOfficeViewerBuffer) {
    if !buffer.ptr.is_null() {
        unsafe { drop(Vec::from_raw_parts(buffer.ptr, buffer.len, buffer.capacity)) };
    }
}

fn ffi_call(
    operation: impl FnOnce() -> Result<Vec<u8>, ViewerError>,
) -> OpenMuseOfficeViewerBuffer {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(operation)) {
        Ok(Ok(bytes)) => OpenMuseOfficeViewerBuffer::from_vec(bytes, 0),
        Ok(Err(error)) => OpenMuseOfficeViewerBuffer::from_vec(error.to_string().into_bytes(), 1),
        Err(_) => OpenMuseOfficeViewerBuffer::from_vec(b"viewer panic contained".to_vec(), 1),
    }
}
