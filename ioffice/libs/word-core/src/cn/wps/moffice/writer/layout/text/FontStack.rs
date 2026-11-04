//! wr-03 FontStack: ttf-parser measure backend (no platform .so).
//! Java twin for Writer layout advances is `AndroidTypeface`; this type is
//! Rust-only glue (name → file → Face, pt↔twip). Font table parsing is not
//! hand-written.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use ttf_parser::{Face, Tag};

/// `pt_to_twip(n) = round(n × 20)` (1pt = 20 twip).
pub fn pt_to_twip(pt: f64) -> i32 {
    (pt * 20.0).round() as i32
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FontErrorKind {
    Io,
    Parse,
    MissingGlyph,
}

#[derive(Debug, Clone)]
pub struct FontError {
    pub kind: FontErrorKind,
    pub message: String,
}

impl FontError {
    fn io(msg: impl Into<String>) -> Self {
        FontError {
            kind: FontErrorKind::Io,
            message: msg.into(),
        }
    }
    fn parse(msg: impl Into<String>) -> Self {
        FontError {
            kind: FontErrorKind::Parse,
            message: msg.into(),
        }
    }
    fn missing_glyph(ch: char) -> Self {
        FontError {
            kind: FontErrorKind::MissingGlyph,
            message: format!("no glyph for U+{:04X}", ch as u32),
        }
    }
}

impl std::fmt::Display for FontError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for FontError {}

#[derive(Clone)]
struct FontFaceBlob {
    bytes: Vec<u8>,
    collection_index: u32,
    path: Option<PathBuf>,
}

impl FontFaceBlob {
    #[cfg(not(target_family = "wasm"))]
    fn from_path(path: &Path) -> Result<Self, FontError> {
        let bytes = std::fs::read(path).map_err(|e| FontError::io(format!("{}: {e}", path.display())))?;
        let blob = FontFaceBlob {
            bytes,
            collection_index: 0,
            path: Some(path.to_path_buf()),
        };
        blob.parse_face()?;
        Ok(blob)
    }

    #[cfg(target_family = "wasm")]
    fn from_path(path: &Path) -> Result<Self, FontError> {
        Err(FontError::io(format!(
            "{}: wasm cannot read host font paths; inject TTF bytes",
            path.display()
        )))
    }

    fn parse_face(&self) -> Result<Face<'_>, FontError> {
        Face::parse(&self.bytes, self.collection_index).map_err(|e| FontError::parse(format!("{e:?}")))
    }
}

/// Dual-backend slot: MVP is ttf-parser only. FreeTypeJNI remains unused.
pub struct FontStack {
    default: FontFaceBlob,
    name_map: HashMap<String, FontFaceBlob>,
    // Java Android Paint resolves fallback faces in the host's configured order.
    // Preserve injection order rather than iterating HashMap's random order.
    name_order: Vec<String>,
}

/// Fields consumed by Java AndroidTypeface.doCalcLayoutMetricsInfo.
pub struct WriterLayoutMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub latin_external: f32,
    pub is_cjk_font: bool,
}

impl FontStack {
    pub fn from_font_file(path: impl AsRef<Path>) -> Result<Self, FontError> {
        Ok(FontStack {
            default: FontFaceBlob::from_path(path.as_ref())?,
            name_map: HashMap::new(),
            name_order: Vec::new(),
        })
    }

    /// Load a face from in-memory TTF/OTF/TTC bytes (Flutter / wasm font injection).
    pub fn from_font_bytes(bytes: impl Into<Vec<u8>>) -> Result<Self, FontError> {
        Self::from_font_bytes_indexed(bytes, 0)
    }

    pub fn from_font_bytes_indexed(
        bytes: impl Into<Vec<u8>>,
        collection_index: u32,
    ) -> Result<Self, FontError> {
        let blob = FontFaceBlob {
            bytes: bytes.into(),
            collection_index,
            path: None,
        };
        blob.parse_face()?;
        Ok(FontStack {
            default: blob,
            name_map: HashMap::new(),
            name_order: Vec::new(),
        })
    }

    /// Probe common system font dirs; first parseable TTF/OTF wins.
    /// Wasm has no host font directory — the host must inject TTF bytes.
    pub fn default() -> Result<Self, FontError> {
        #[cfg(target_family = "wasm")]
        {
            return Err(FontError::io(
                "no default system font on wasm; inject TTF bytes via FontHost",
            ));
        }
        #[cfg(not(target_family = "wasm"))]
        {
            const CANDIDATES: &[&str] = &[
                "/System/Library/Fonts/Supplemental/Arial.ttf",
                "/System/Library/Fonts/Supplemental/Andale Mono.ttf",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
                "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
                "/system/fonts/Roboto-Regular.ttf",
            ];
            for p in CANDIDATES {
                if let Ok(stack) = FontStack::from_font_file(p) {
                    return Ok(stack);
                }
            }
            Err(FontError::io("no default system font found"))
        }
    }

    pub fn register_name(&mut self, name: impl Into<String>, path: impl AsRef<Path>) -> Result<(), FontError> {
        let name = name.into();
        let blob = FontFaceBlob::from_path(path.as_ref())?;
        if !self.name_map.contains_key(&name) {
            self.name_order.push(name.clone());
        }
        self.name_map.insert(name, blob);
        Ok(())
    }

    /// Register an in-memory face under a family / blob id (FontHost name_map).
    pub fn register_name_bytes(
        &mut self,
        name: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<(), FontError> {
        self.register_name_bytes_indexed(name, bytes, 0)
    }

    pub fn register_name_bytes_indexed(
        &mut self,
        name: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
        collection_index: u32,
    ) -> Result<(), FontError> {
        let blob = FontFaceBlob {
            bytes: bytes.into(),
            collection_index,
            path: None,
        };
        blob.parse_face()?;
        let name = name.into();
        if !self.name_map.contains_key(&name) {
            self.name_order.push(name.clone());
        }
        self.name_map.insert(name, blob);
        Ok(())
    }

    fn blob_has_char(blob: &FontFaceBlob, ch: char) -> bool {
        blob.parse_face()
            .ok()
            .and_then(|f| f.glyph_index(ch))
            .is_some()
    }

    fn covering_blob(&self, font_name: Option<&str>, ch: char) -> Result<&FontFaceBlob, FontError> {
        let preferred = self.blob_for(font_name);
        if Self::blob_has_char(preferred, ch) {
            return Ok(preferred);
        }
        for name in &self.name_order {
            let blob = &self.name_map[name];
            if Self::blob_has_char(blob, ch) {
                return Ok(blob);
            }
        }
        if Self::blob_has_char(&self.default, ch) {
            return Ok(&self.default);
        }
        Err(FontError::missing_glyph(ch))
    }

    /// Named face only (no chain). Used to pick `DrawText.font_id`.
    pub fn has_glyph(&self, font_name: Option<&str>, ch: char) -> bool {
        Self::blob_has_char(self.blob_for(font_name), ch)
    }

    /// First injected fallback id whose cmap contains `ch`.
    pub fn face_id_for_char(&self, preferred: Option<&str>, ch: char) -> Option<String> {
        if let Some(n) = preferred {
            if let Some(b) = self.name_map.get(n) {
                if Self::blob_has_char(b, ch) {
                    return Some(n.to_string());
                }
            }
        }
        for id in &self.name_order {
            let blob = &self.name_map[id];
            if Self::blob_has_char(blob, ch) {
                return Some(id.clone());
            }
        }
        None
    }

    fn blob_for(&self, name: Option<&str>) -> &FontFaceBlob {
        if let Some(n) = name {
            if let Some(b) = self.name_map.get(n) {
                return b;
            }
        }
        &self.default
    }

    /// Unknown names fall back to the default face (no panic).
    pub fn font_id(&self, name: &str) -> Result<Face<'_>, FontError> {
        self.blob_for(Some(name)).parse_face()
    }

    pub fn glyph_count(&self) -> Result<u16, FontError> {
        Ok(self.default.parse_face()?.number_of_glyphs())
    }

    /// `advance_twip = round(hmtx_advance * pt * 20 / units_per_em)`.
    /// Prefer `font_name`, then other injected faces (Paint/Minikin role = extra TTF, never `width=size`).
    pub fn glyph_advance_twip(&self, font_name: Option<&str>, ch: char, size_pt: f64) -> Result<i32, FontError> {
        let face = self.covering_blob(font_name, ch)?.parse_face()?;
        let gid = face.glyph_index(ch).ok_or_else(|| FontError::missing_glyph(ch))?;
        let adv = face.glyph_hor_advance(gid).unwrap_or(0) as f64;
        let upem = face.units_per_em() as f64;
        if upem == 0.0 {
            return Err(FontError::parse("units_per_em is 0"));
        }
        Ok(((adv * size_pt * 20.0) / upem).round() as i32)
    }

    pub fn name_hits_file(&self, name: &str) -> bool {
        self.name_map.contains_key(name)
    }

    pub fn default_path(&self) -> Option<&Path> {
        self.default.path.as_deref()
    }

    /// hhea ascender/descender in twip at `size_pt`.
    pub fn line_metrics_twip(&self, size_pt: f64) -> Result<(i32, i32), FontError> {
        let (ascent, descent, _) = self.line_box_twip(None, size_pt)?;
        Ok((ascent, descent))
    }

    /// Named face only: missing cmap on that blob is an error (no covering chain).
    pub fn glyph_advance_twip_on_face(
        &self,
        font_name: Option<&str>,
        ch: char,
        size_pt: f64,
    ) -> Result<i32, FontError> {
        let blob = self.blob_for(font_name);
        let face = blob.parse_face()?;
        let gid = face
            .glyph_index(ch)
            .ok_or_else(|| FontError::missing_glyph(ch))?;
        let adv = face.glyph_hor_advance(gid).unwrap_or(0) as f64;
        let upem = face.units_per_em() as f64;
        units_to_twip(adv, size_pt, upem)
    }

    // rust: ★wr-29 04 —— Java `ITypeface.getWriterLayoutAdavancesEM` 对等：
    //       hmtx 原始 advance（font units，EM 单位）。GraphFactory 宽 =
    //       `advance_units × fontSize / srcEmSize`（srcEmSize := units_per_em），
    //       与 `units_to_twip` 的 `units × size_pt × 20 / upem` 同式。
    pub fn glyph_hmtx_units(&self, font_name: Option<&str>, ch: char) -> Result<u32, FontError> {
        let face = self.covering_blob(font_name, ch)?.parse_face()?;
        let gid = face
            .glyph_index(ch)
            .ok_or_else(|| FontError::missing_glyph(ch))?;
        Ok(face.glyph_hor_advance(gid).unwrap_or(0) as u32)
    }

    /// Return the glyph advance in the caller's EM. AndroidTypeface falls back
    /// through Paint at `srcEmSize`, so fallback faces must use that same unit.
    pub fn glyph_hmtx_units_for_em(
        &self,
        font_name: Option<&str>,
        ch: char,
        target_em: f32,
    ) -> Result<i32, FontError> {
        let face = self.covering_blob(font_name, ch)?.parse_face()?;
        let gid = face
            .glyph_index(ch)
            .ok_or_else(|| FontError::missing_glyph(ch))?;
        let source_em = face.units_per_em() as f32;
        if source_em <= 0.0 || target_em <= 0.0 {
            return Err(FontError::parse("invalid units_per_em"));
        }
        let advance = face.glyph_hor_advance(gid).unwrap_or(0) as f32;
        Ok((advance * target_em / source_em + 0.5) as i32)
    }

    /// Android Paint uses the selected face's missing-glyph box when cmap has
    /// no entry. Keep that advance data-driven from glyph zero in the face.
    pub fn missing_glyph_hmtx_units_for_em(
        &self,
        font_name: Option<&str>,
        target_em: f32,
    ) -> Result<i32, FontError> {
        let face = self.blob_for(font_name).parse_face()?;
        let source_em = face.units_per_em() as f32;
        if source_em <= 0.0 || target_em <= 0.0 {
            return Err(FontError::parse("invalid units_per_em"));
        }
        let advance = face.glyph_hor_advance(ttf_parser::GlyphId(0)).unwrap_or(0) as f32;
        Ok((advance * target_em / source_em + 0.5) as i32)
    }

    // rust: ★wr-29 04 —— `ITypeface.getSrcEmSize()` 对等（Java writer 布局的 EM = upem）。
    pub fn units_per_em(&self, font_name: Option<&str>) -> Result<f32, FontError> {
        let face = self.blob_for(font_name).parse_face()?;
        let upem = face.units_per_em() as f32;
        if upem == 0.0 {
            return Err(FontError::parse("units_per_em is 0"));
        }
        Ok(upem)
    }

    // rust: ★wr-29 04 —— hhea ascender/descender（font units）。
    //       Java `getWriterLayoutAscent`/`getWriterLayoutDescent` 对等。
    pub fn hmtx_ascent_descent(&self, font_name: Option<&str>) -> Result<(f32, f32), FontError> {
        let face = self.blob_for(font_name).parse_face()?;
        Ok((face.ascender() as f32, face.descender() as f32))
    }

    /// Java FreeTypeJNI.TTFHeader feeds positive win/tm descent values to
    /// AbstractTypeface.doGetLatinAscentAndDescent and doGetLatinExternal.
    pub fn writer_layout_metrics(&self, font_name: Option<&str>) -> Result<WriterLayoutMetrics, FontError> {
        use thirdparty_font::cn::wps::font::r#impl::AbstractTypeface::AbstractTypeface;
        use thirdparty_font::cn::wps::util::PrimitivePackTool::PrimitivePackTool;
        let face = self.blob_for(font_name).parse_face()?;
        let os2 = face.raw_face().table(Tag::from_bytes(b"OS/2"));
        let read_u16 = |offset: usize| -> Option<u16> {
            let bytes = os2?.get(offset..offset + 2)?;
            Some(u16::from_be_bytes([bytes[0], bytes[1]]))
        };
        let read_u32 = |offset: usize| -> Option<u32> {
            let bytes = os2?.get(offset..offset + 4)?;
            Some(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        };
        let tm_ascent = face.ascender() as f32;
        let tm_descent = -(face.descender() as f32);
        let win_ascent = read_u16(74).map(f32::from).unwrap_or(tm_ascent);
        let win_descent = read_u16(76).map(f32::from).unwrap_or(tm_descent);
        // Java AbstractTypeface.isCJKCodePageRange(codePageRange1).
        let is_cjk_font = AbstractTypeface::isCJKCodePageRange(read_u32(78).unwrap_or(0) as i32);
        // Java AbstractTypeface.doGetLatinAscentAndDescent/doGetLatinExternal.
        let packed = AbstractTypeface::doGetLatinAscentAndDescent(
            win_ascent, win_descent, tm_ascent, tm_descent, is_cjk_font);
        let ascent = PrimitivePackTool::unpackFirstFloat(packed);
        let descent = PrimitivePackTool::unpackSecondFloat(packed);
        let latin_external = AbstractTypeface::doGetLatinExternal(
            win_ascent, win_descent, tm_ascent, tm_descent, face.line_gap() as f32, is_cjk_font);
        Ok(WriterLayoutMetrics { ascent, descent, latin_external, is_cjk_font })
    }

    /// FreeTypeJNI.TTFHeader's OS/2 script metrics, evaluated by
    /// AndroidTypeface.doCalcLayoutMetricsInfo through AbstractTypeface.
    pub fn writer_script_factors(&self, font_name: Option<&str>) -> Result<(f32, f32), FontError> {
        use thirdparty_font::cn::wps::font::r#impl::AbstractTypeface::AbstractTypeface;
        let face = self.blob_for(font_name).parse_face()?;
        let Some(os2) = face.raw_face().table(Tag::from_bytes(b"OS/2")) else {
            return Ok((0.6, 0.6));
        };
        let read = |offset: usize| -> Option<f32> {
            let bytes = os2.get(offset..offset + 2)?;
            Some(i16::from_be_bytes([bytes[0], bytes[1]]) as f32)
        };
        let (Some(sub_size), Some(sub_offset), Some(super_size), Some(super_offset)) =
            (read(12), read(16), read(20), read(24)) else {
            return Ok((0.6, 0.6));
        };
        let em = face.units_per_em() as f32;
        Ok((
            AbstractTypeface::doGetScriptFactor(sub_offset, sub_size, super_offset, super_size, em, true),
            AbstractTypeface::doGetScriptFactor(sub_offset, sub_size, super_offset, super_size, em, false),
        ))
    }

    /// AndroidTypeface.initLalayoutMetricsInfo's final Paint.FontMetrics path.
    /// The source Android Typeface is a sans-serif face; it remains separate
    /// from the document family and from the glyph fallback chain.
    pub fn paint_source_metrics(&self) -> Result<(f32, f32), FontError> {
        let source = if self.name_hits_file("Roboto") { Some("Roboto") } else { None };
        let face = self.blob_for(source).parse_face()?;
        let em = face.units_per_em() as f32;
        if em <= 0.0 { return Err(FontError::parse("invalid units_per_em")); }
        Ok((face.ascender() as f32 * 100.0 / em,
            -(face.descender() as f32) * 100.0 / em))
    }

    /// Java AbstractTypeface.doGetCharSpaceRatio uses OS/2 xAvgCharWidth,
    /// not the horizontal advance of the U+0020 space glyph.
    pub fn writer_char_space_ratio(&self, font_name: Option<&str>) -> Result<f32, FontError> {
        let face = self.blob_for(font_name).parse_face()?;
        let Some(os2) = face.raw_face().table(Tag::from_bytes(b"OS/2")) else {
            return Ok(4.0);
        };
        if os2.len() < 4 {
            return Ok(4.0);
        }
        let avg_char_width = i16::from_be_bytes([os2[2], os2[3]]) as f32;
        Ok(face.units_per_em() as f32 * 2.0 / avg_char_width)
    }

    /// hhea ascender / descender / lineGap in twip at `size_pt` for a named face.
    pub fn line_box_twip(
        &self,
        font_name: Option<&str>,
        size_pt: f64,
    ) -> Result<(i32, i32, i32), FontError> {
        let face = self.blob_for(font_name).parse_face()?;
        let upem = face.units_per_em() as f64;
        if upem == 0.0 {
            return Err(FontError::parse("units_per_em is 0"));
        }
        let scale = size_pt * 20.0 / upem;
        let ascent = (face.ascender() as f64 * scale).round() as i32;
        let descent = (face.descender() as f64 * scale).round() as i32;
        let line_gap = (face.line_gap() as f64 * scale).round() as i32;
        Ok((ascent, descent, line_gap))
    }
}

/// Blue-book / FontStack: `round(units × size_pt × 20 / upem)`.
/// Equivalent to `round(units × size_twip / upem)` when `size_twip = size_pt × 20`.
/// FEATURE.md's extra `× 20` after `size_twip` would 20× the result and is not used.
pub fn units_to_twip(units: f64, size_pt: f64, upem: f64) -> Result<i32, FontError> {
    if upem == 0.0 {
        return Err(FontError::parse("units_per_em is 0"));
    }
    Ok(((units * size_pt * 20.0) / upem).round() as i32)
}

pub fn units_to_twip_from_size_twip(
    units: f64,
    size_twip: i32,
    upem: f64,
) -> Result<i32, FontError> {
    units_to_twip(units, (size_twip as f64) / 20.0, upem)
}
