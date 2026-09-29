//! Bounded, network-free DOCX engine core and C ABI for Mobile adapters.

use quick_xml::{Reader, escape::resolve_xml_entity, events::Event};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::{Cursor, Read, Write};
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

pub const ABI_VERSION: u32 = 1;
pub const DEFAULT_MAX_ARCHIVE_BYTES: usize = 64 * 1024 * 1024;
pub const DEFAULT_MAX_ENTRY_BYTES: u64 = 32 * 1024 * 1024;
pub const DEFAULT_MAX_TOTAL_UNCOMPRESSED_BYTES: u64 = 128 * 1024 * 1024;
pub const DEFAULT_MAX_ENTRIES: usize = 2048;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DocxProfile {
    SimpleText,
    ViewOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocxInspection {
    pub schema: &'static str,
    pub paragraphs: Vec<String>,
    pub profile: DocxProfile,
    pub capabilities: Vec<&'static str>,
}

#[derive(Debug, Clone, Copy)]
pub struct DocxLimits {
    pub max_archive_bytes: usize,
    pub max_entry_bytes: u64,
    pub max_total_uncompressed_bytes: u64,
    pub max_entries: usize,
}

impl Default for DocxLimits {
    fn default() -> Self {
        Self {
            max_archive_bytes: DEFAULT_MAX_ARCHIVE_BYTES,
            max_entry_bytes: DEFAULT_MAX_ENTRY_BYTES,
            max_total_uncompressed_bytes: DEFAULT_MAX_TOTAL_UNCOMPRESSED_BYTES,
            max_entries: DEFAULT_MAX_ENTRIES,
        }
    }
}

pub fn inspect(bytes: &[u8]) -> Result<DocxInspection, DocxError> {
    inspect_with_limits(bytes, DocxLimits::default())
}

pub fn inspect_with_limits(bytes: &[u8], limits: DocxLimits) -> Result<DocxInspection, DocxError> {
    let parts = read_parts(bytes, limits)?;
    validate_package(&parts)?;
    let document = parts
        .iter()
        .find(|part| part.name == "word/document.xml")
        .ok_or(DocxError::MissingPart("word/document.xml"))?;
    let parsed = parse_document(&document.bytes)?;
    let capabilities = match parsed.profile {
        DocxProfile::SimpleText => vec!["view", "edit", "export"],
        DocxProfile::ViewOnly => vec!["view"],
    };
    Ok(DocxInspection {
        schema: "openmuse.office.docx-inspection@1",
        paragraphs: parsed.paragraphs,
        profile: parsed.profile,
        capabilities,
    })
}

pub fn export_simple(bytes: &[u8], paragraphs: &[String]) -> Result<Vec<u8>, DocxError> {
    let inspection = inspect(bytes)?;
    if inspection.profile != DocxProfile::SimpleText {
        return Err(DocxError::ExportUnsupported);
    }
    if paragraphs.len() > 100_000
        || paragraphs
            .iter()
            .any(|value| value.len() > 1024 * 1024 || value.contains('\0'))
    {
        return Err(DocxError::LimitExceeded);
    }
    build_simple_docx(paragraphs)
}

#[derive(Debug)]
struct PackagePart {
    name: String,
    bytes: Vec<u8>,
}

fn read_parts(bytes: &[u8], limits: DocxLimits) -> Result<Vec<PackagePart>, DocxError> {
    if bytes.is_empty() || bytes.len() > limits.max_archive_bytes {
        return Err(DocxError::LimitExceeded);
    }
    let mut archive = ZipArchive::new(Cursor::new(bytes)).map_err(|_| DocxError::InvalidArchive)?;
    if archive.len() > limits.max_entries {
        return Err(DocxError::LimitExceeded);
    }
    let mut total = 0_u64;
    let mut names = HashSet::new();
    let mut parts = Vec::with_capacity(archive.len());
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|_| DocxError::InvalidArchive)?;
        let name = entry.name().to_owned();
        if name.is_empty()
            || name.starts_with('/')
            || name.starts_with('\\')
            || name.contains('\\')
            || name.split('/').any(|segment| segment == "..")
            || !names.insert(name.clone())
        {
            return Err(DocxError::UnsafePackagePath);
        }
        if entry.is_dir() {
            continue;
        }
        if entry.size() > limits.max_entry_bytes {
            return Err(DocxError::LimitExceeded);
        }
        total = total
            .checked_add(entry.size())
            .ok_or(DocxError::LimitExceeded)?;
        if total > limits.max_total_uncompressed_bytes
            || (entry.compressed_size() > 0 && entry.size() / entry.compressed_size().max(1) > 200)
        {
            return Err(DocxError::LimitExceeded);
        }
        let mut part = Vec::with_capacity(entry.size() as usize);
        entry
            .read_to_end(&mut part)
            .map_err(|_| DocxError::InvalidArchive)?;
        parts.push(PackagePart { name, bytes: part });
    }
    Ok(parts)
}

fn validate_package(parts: &[PackagePart]) -> Result<(), DocxError> {
    for required in ["[Content_Types].xml", "_rels/.rels", "word/document.xml"] {
        if !parts.iter().any(|part| part.name == required) {
            return Err(DocxError::MissingPart(required));
        }
    }
    for part in parts.iter().filter(|part| part.name.ends_with(".rels")) {
        let text = std::str::from_utf8(&part.bytes).map_err(|_| DocxError::InvalidXml)?;
        if text.contains("TargetMode=\"External\"") || text.contains("TargetMode='External'") {
            return Err(DocxError::ExternalRelationship);
        }
    }
    Ok(())
}

struct ParsedDocument {
    paragraphs: Vec<String>,
    profile: DocxProfile,
}

fn parse_document(xml: &[u8]) -> Result<ParsedDocument, DocxError> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(false);
    let mut paragraphs = Vec::new();
    let mut current = String::new();
    let mut in_text = false;
    let mut paragraph_depth = 0_usize;
    let mut profile = DocxProfile::SimpleText;
    loop {
        match reader.read_event().map_err(|_| DocxError::InvalidXml)? {
            Event::Start(tag) => match local_name(tag.name().as_ref()) {
                b"p" => {
                    paragraph_depth += 1;
                    if paragraph_depth > 1 {
                        profile = DocxProfile::ViewOnly;
                    }
                }
                b"t" => in_text = true,
                b"tab" if paragraph_depth > 0 => current.push('\t'),
                b"br" | b"cr" if paragraph_depth > 0 => current.push('\n'),
                b"document" | b"body" | b"r" | b"sectPr" => {}
                _ => profile = DocxProfile::ViewOnly,
            },
            Event::Empty(tag) => match local_name(tag.name().as_ref()) {
                b"tab" if paragraph_depth > 0 => current.push('\t'),
                b"br" | b"cr" if paragraph_depth > 0 => current.push('\n'),
                b"sectPr" => {}
                _ => profile = DocxProfile::ViewOnly,
            },
            Event::Text(text) if in_text => {
                current.push_str(&text.decode().map_err(|_| DocxError::InvalidXml)?)
            }
            Event::CData(text) if in_text => {
                current.push_str(&text.decode().map_err(|_| DocxError::InvalidXml)?)
            }
            Event::GeneralRef(reference) if in_text => {
                if let Some(character) = reference
                    .resolve_char_ref()
                    .map_err(|_| DocxError::InvalidXml)?
                {
                    current.push(character);
                } else {
                    let name = reference.decode().map_err(|_| DocxError::InvalidXml)?;
                    current.push_str(resolve_xml_entity(&name).ok_or(DocxError::DocTypeDenied)?);
                }
            }
            Event::End(tag) => match local_name(tag.name().as_ref()) {
                b"t" => in_text = false,
                b"p" => {
                    paragraph_depth = paragraph_depth.saturating_sub(1);
                    if paragraph_depth == 0 {
                        paragraphs.push(std::mem::take(&mut current));
                    }
                }
                _ => {}
            },
            Event::DocType(_) => return Err(DocxError::DocTypeDenied),
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(ParsedDocument {
        paragraphs,
        profile,
    })
}

fn local_name(name: &[u8]) -> &[u8] {
    name.rsplit(|byte| *byte == b':').next().unwrap_or(name)
}

fn build_simple_docx(paragraphs: &[String]) -> Result<Vec<u8>, DocxError> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    write_part(&mut writer, "[Content_Types].xml", CONTENT_TYPES, options)?;
    write_part(&mut writer, "_rels/.rels", ROOT_RELS, options)?;
    let mut document = String::from(DOCUMENT_PREFIX);
    for paragraph in paragraphs {
        document.push_str("<w:p><w:r><w:t xml:space=\"preserve\">");
        escape_xml(&mut document, paragraph);
        document.push_str("</w:t></w:r></w:p>");
    }
    document.push_str(DOCUMENT_SUFFIX);
    write_part(
        &mut writer,
        "word/document.xml",
        document.as_bytes(),
        options,
    )?;
    writer
        .finish()
        .map(|value| value.into_inner())
        .map_err(|_| DocxError::ExportFailed)
}

fn write_part(
    writer: &mut ZipWriter<Cursor<Vec<u8>>>,
    name: &str,
    bytes: impl AsRef<[u8]>,
    options: SimpleFileOptions,
) -> Result<(), DocxError> {
    writer
        .start_file(name, options)
        .map_err(|_| DocxError::ExportFailed)?;
    writer
        .write_all(bytes.as_ref())
        .map_err(|_| DocxError::ExportFailed)
}

fn escape_xml(out: &mut String, value: &str) {
    for character in value.chars() {
        match character {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(character),
        }
    }
}

const CONTENT_TYPES: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>"#;
const ROOT_RELS: &[u8] = br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>"#;
const DOCUMENT_PREFIX: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body>"#;
const DOCUMENT_SUFFIX: &str = "<w:sectPr/></w:body></w:document>";

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum DocxError {
    #[error("invalid DOCX archive")]
    InvalidArchive,
    #[error("DOCX package contains an unsafe path")]
    UnsafePackagePath,
    #[error("DOCX exceeds a configured resource limit")]
    LimitExceeded,
    #[error("DOCX is missing required part {0}")]
    MissingPart(&'static str),
    #[error("DOCX XML is invalid")]
    InvalidXml,
    #[error("XML document types are denied")]
    DocTypeDenied,
    #[error("external relationships are denied")]
    ExternalRelationship,
    #[error("original-format export is unavailable for this document profile")]
    ExportUnsupported,
    #[error("DOCX export failed")]
    ExportFailed,
}

#[repr(C)]
pub struct OpenMuseDocxBuffer {
    pub ptr: *mut u8,
    pub len: usize,
    pub capacity: usize,
    pub status: i32,
}

impl OpenMuseDocxBuffer {
    fn success(bytes: Vec<u8>) -> Self {
        Self::from_vec(bytes, 0)
    }

    fn failure(error: impl ToString) -> Self {
        Self::from_vec(error.to_string().into_bytes(), 1)
    }

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
pub extern "C" fn openmuse_docx_abi_version() -> u32 {
    ABI_VERSION
}

#[unsafe(no_mangle)]
/// Inspect a DOCX byte buffer and return an owned JSON result.
///
/// # Safety
/// `docx` must point to `docx_len` readable bytes for the duration of this
/// call. The returned buffer must be released exactly once with
/// [`openmuse_docx_buffer_free`].
pub unsafe extern "C" fn openmuse_docx_inspect(
    docx: *const u8,
    docx_len: usize,
) -> OpenMuseDocxBuffer {
    ffi_call(|| {
        let bytes = unsafe { input_slice(docx, docx_len)? };
        serde_json::to_vec(&inspect(bytes)?).map_err(|_| DocxError::InvalidXml)
    })
}

#[unsafe(no_mangle)]
/// Export a supported simple-text DOCX using a JSON array of paragraphs.
///
/// # Safety
/// Both pointers must reference their declared readable lengths for the
/// duration of this call. The returned buffer must be released exactly once
/// with [`openmuse_docx_buffer_free`].
pub unsafe extern "C" fn openmuse_docx_export_simple(
    docx: *const u8,
    docx_len: usize,
    paragraphs_json: *const u8,
    paragraphs_json_len: usize,
) -> OpenMuseDocxBuffer {
    ffi_call(|| {
        let docx = unsafe { input_slice(docx, docx_len)? };
        let json = unsafe { input_slice(paragraphs_json, paragraphs_json_len)? };
        let paragraphs: Vec<String> =
            serde_json::from_slice(json).map_err(|_| DocxError::InvalidXml)?;
        export_simple(docx, &paragraphs)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn openmuse_docx_buffer_free(buffer: OpenMuseDocxBuffer) {
    if buffer.ptr.is_null() {
        return;
    }
    unsafe {
        drop(Vec::from_raw_parts(buffer.ptr, buffer.len, buffer.capacity));
    }
}

fn ffi_call(operation: impl FnOnce() -> Result<Vec<u8>, DocxError>) -> OpenMuseDocxBuffer {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(operation)) {
        Ok(Ok(bytes)) => OpenMuseDocxBuffer::success(bytes),
        Ok(Err(error)) => OpenMuseDocxBuffer::failure(error),
        Err(_) => OpenMuseDocxBuffer::failure("DOCX engine panic contained"),
    }
}

unsafe fn input_slice<'a>(pointer: *const u8, length: usize) -> Result<&'a [u8], DocxError> {
    if pointer.is_null() || length == 0 || length > DEFAULT_MAX_ARCHIVE_BYTES {
        return Err(DocxError::LimitExceeded);
    }
    Ok(unsafe { std::slice::from_raw_parts(pointer, length) })
}
