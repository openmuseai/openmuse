//! wr-03 FontHost: injected face table + display-name schemes.
//!
//! Java twins (not copied into `stubs`):
//! - `cn.wps.font.FontHost` — table after `loadCache` / `initCache`
//! - `FontTool.addFontMapScheme` — `FontHostSettings.SchemaItem[]`
//! - `TypefaceFactory.createTypeface` — family file → `ITypeface`
//!
//! The host (Flutter / tests) plays `Platform.getAllFontsDirectory`: it reads
//! bytes and injects them. This type must not probe system font dirs.

use std::collections::HashMap;

use super::FontStack::{FontError, FontErrorKind, FontStack};

/// Java `FontHostSettings.SchemaItem`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchemaItem {
    pub display_font_name: String,
    pub actual_font_name: String,
    pub is_selectable_in_font_list: bool,
}

impl SchemaItem {
    pub fn map(display: impl Into<String>, actual: impl Into<String>) -> Self {
        SchemaItem {
            display_font_name: display.into(),
            actual_font_name: actual.into(),
            is_selectable_in_font_list: true,
        }
    }
}

/// One face the host already resolved to bytes (Java: a row in `IFontTable`).
#[derive(Clone, Debug)]
pub struct InjectedFace {
    pub id: String,
    pub bytes: Vec<u8>,
    pub collection_index: u32,
}

/// Family table. First injected face is the default stack (MVP single default).
#[derive(Debug)]
pub struct FontHost {
    default_id: String,
    table: HashMap<String, InjectedFace>,
    schemes: Vec<SchemaItem>,
}

// rust: ★wr-29 04 —— Java `FontHost.getFontTable()` 的静态字体表对等：
//       Java 的静态 sFontTable 可由 setFontTable 替换。Rust 保留进程级表，
//       但每次注入都替换 Arc；一次失败的缺字布局后，Flutter 会带 CJK 面重试。
//       OnceLock::set 只接受第一次字体表，会让重试和后续文档沿用旧字宽。
static FONT_TABLE: std::sync::RwLock<Option<std::sync::Arc<FontStack>>> =
    std::sync::RwLock::new(None);

/// Java `FontHost.setFontTable(...)` —— 注入进程字体表（测试 setUp / CLI / 产品）。
pub fn set_font_table(stack: std::sync::Arc<FontStack>) {
    // java private static void setFontTable(IFontTable fontTable) { sFontTable = fontTable; }
    *FONT_TABLE.write().expect("FontHost font table poisoned") = Some(stack);
}

/// Java `FontHost.getFontTable()` —— 返回已注入的表；未注入时为 None。
pub fn get_font_table() -> Option<std::sync::Arc<FontStack>> {
    // java IFontTable ret = sFontTable; ... return ret;
    FONT_TABLE.read().expect("FontHost font table poisoned").clone()
}

impl FontHost {
    /// Java `FontHost.loadCache` analogue: table is already populated by the host.
    pub fn from_faces(faces: Vec<InjectedFace>) -> Result<Self, FontError> {
        if faces.is_empty() {
            return Err(FontError {
                kind: FontErrorKind::Io,
                message: "FontHost: no injected faces (host must supply TTF bytes)".into(),
            });
        }
        let default_id = faces[0].id.clone();
        let mut table = HashMap::new();
        for face in faces {
            if face.id.is_empty() {
                return Err(FontError {
                    kind: FontErrorKind::Parse,
                    message: "FontHost: empty face id".into(),
                });
            }
            ttf_parser::Face::parse(&face.bytes, face.collection_index).map_err(|e| FontError {
                kind: FontErrorKind::Parse,
                message: format!("{e:?}"),
            })?;
            table.insert(face.id.clone(), face);
        }
        Ok(FontHost {
            default_id,
            table,
            schemes: Vec::new(),
        })
    }

    /// Java `FontTool.addFontMapScheme`.
    pub fn add_font_map_scheme(&mut self, schemes: impl IntoIterator<Item = SchemaItem>) {
        self.schemes.extend(schemes);
    }

    pub fn default_font_id(&self) -> &str {
        &self.default_id
    }

    /// Display name → actual family in the table, else the default id.
    pub fn resolve_actual_name(&self, docx_name: Option<&str>) -> &str {
        let Some(name) = docx_name.filter(|s| !s.is_empty()) else {
            return &self.default_id;
        };
        if let Some((id, _)) = self.table.get_key_value(name) {
            return id.as_str();
        }
        for scheme in &self.schemes {
            if scheme.display_font_name == name {
                if let Some((id, _)) = self.table.get_key_value(scheme.actual_font_name.as_str()) {
                    return id.as_str();
                }
            }
        }
        &self.default_id
    }

    /// Java `TypefaceFactory.createTypeface` → measure handle.
    pub fn into_font_stack(self) -> Result<FontStack, FontError> {
        let default = self.table.get(&self.default_id).ok_or_else(|| FontError {
            kind: FontErrorKind::Parse,
            message: "FontHost: missing default face".into(),
        })?;
        let mut stack = FontStack::from_font_bytes_indexed(
            default.bytes.clone(),
            default.collection_index,
        )?;
        stack.register_name_bytes_indexed(
            self.default_id.clone(),
            default.bytes.clone(),
            default.collection_index,
        )?;
        for (id, face) in &self.table {
            if id != &self.default_id {
                stack.register_name_bytes_indexed(
                    id.clone(),
                    face.bytes.clone(),
                    face.collection_index,
                )?;
            }
        }
        for scheme in &self.schemes {
            if let Some(actual) = self.table.get(&scheme.actual_font_name) {
                stack.register_name_bytes_indexed(
                    scheme.display_font_name.clone(),
                    actual.bytes.clone(),
                    actual.collection_index,
                )?;
            }
        }
        Ok(stack)
    }
}
