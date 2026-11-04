//! Heap `DocumentSession` on the native side (wr-18..20). Dart holds this opaque.

use std::sync::Mutex;

use flutter_rust_bridge::frb;
use word_core::cn::wps::moffice::writer::api::DocumentSession::DocumentSession;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    FontBlob as CoreFontBlob, LayoutFrame as CoreLayoutFrame, ViewParams as CoreViewParams,
};
use word_core::cn::wps::moffice::writer::api::SelectionFill;
use word_core::cn::wps::moffice::writer::api::SessionCaret::{cp_for_point, geom_for_cp};
use word_core::cn::wps::moffice::writer::api::SessionEdit::SessionEdit;

use super::{frame_from_core, map_err, to_core_view, FontBlob, LayoutFrame, ViewParams};

/// Caret in page-absolute twip + visible cp (DrawText order).
#[derive(Clone, Debug)]
pub struct CaretHit {
    pub cp: i32,
    pub page: i32,
    pub x: i32,
    pub y: i32,
    pub height: i32,
}

struct SessionState {
    doc: DocumentSession,
    fonts: Vec<CoreFontBlob>,
    view: CoreViewParams,
    base: Option<CoreLayoutFrame>,
    sel: Option<(i32, i32)>,
}

/// FRB opaque: `TextDocument` is not auto-Send; all access is through this mutex.
#[frb(opaque)]
pub struct WordSession {
    inner: Mutex<SessionState>,
}

unsafe impl Send for WordSession {}
unsafe impl Sync for WordSession {}

fn lock(s: &WordSession) -> Result<std::sync::MutexGuard<'_, SessionState>, String> {
    s.inner.lock().map_err(|_| "session mutex poisoned".into())
}

fn to_core_fonts(fonts: Vec<FontBlob>) -> Vec<CoreFontBlob> {
    fonts
        .into_iter()
        .map(|f| CoreFontBlob {
            id: f.id,
            bytes: f.bytes,
        })
        .collect()
}

fn paint(state: &mut SessionState) -> Result<CoreLayoutFrame, String> {
    let base = state.doc.layout(&state.fonts, &state.view).map_err(map_err)?;
    let painted = if let Some((a, b)) = state.sel {
        let mut f = base.clone();
        f.commands = SelectionFill::overlay(&base, a, b);
        f
    } else {
        base.clone()
    };
    state.base = Some(base);
    Ok(painted)
}

impl WordSession {
    pub fn open(
        docx: Vec<u8>,
        fonts: Vec<FontBlob>,
        view: ViewParams,
    ) -> Result<WordSession, String> {
        if view.require_injected_fonts && fonts.is_empty() {
            return Err("Font: require_injected_fonts but fonts is empty".into());
        }
        let doc = DocumentSession::open(&docx).map_err(map_err)?;
        Ok(WordSession {
            inner: Mutex::new(SessionState {
                doc,
                fonts: to_core_fonts(fonts),
                view: to_core_view(&view),
                base: None,
                sel: None,
            }),
        })
    }

    pub fn layout(&self) -> Result<LayoutFrame, String> {
        let mut g = lock(self)?;
        frame_from_core(paint(&mut g)?)
    }

    #[frb(sync)]
    pub fn set_view(&self, view: ViewParams) -> Result<LayoutFrame, String> {
        let mut g = lock(self)?;
        g.view = to_core_view(&view);
        frame_from_core(paint(&mut g)?)
    }

    #[frb(sync)]
    pub fn main_length(&self) -> Result<i32, String> {
        Ok(lock(self)?.doc.main_length())
    }

    /// Versioned, read-only gallery metadata from the opened DOCX.
    #[frb(sync)]
    pub fn style_snapshot_json(&self) -> Result<String, String> {
        serde_json::to_string(lock(self)?.doc.style_snapshot()).map_err(|e| e.to_string())
    }

    /// GenOffice-compatible text count on the live session, inclusive of
    /// edits but outside layout/keystroke dispatch.
    #[frb(sync)]
    pub fn word_count(&self, start: i32, end: i32) -> Result<i32, String> {
        Ok(lock(self)?.doc.word_count(start, end))
    }

    /// Read-only, versioned Word Count dialog data from the live document.
    #[frb(sync)]
    pub fn text_statistics_json(&self) -> Result<String, String> {
        serde_json::to_string(&lock(self)?.doc.text_statistics()).map_err(|e| e.to_string())
    }

    #[frb(sync)]
    pub fn caret_at(&self, page: i32, x: i32, y: i32) -> Result<Option<CaretHit>, String> {
        let mut g = lock(self)?;
        if g.base.is_none() {
            paint(&mut g)?;
        }
        let Some(base) = g.base.as_ref() else {
            return Ok(None);
        };
        let Some(cp) = cp_for_point(base, page, x, y) else {
            return Ok(None);
        };
        let Some((p, cx, cy, h)) = geom_for_cp(base, cp) else {
            return Ok(Some(CaretHit {
                cp,
                page,
                x,
                y,
                height: 240,
            }));
        };
        Ok(Some(CaretHit {
            cp,
            page: p,
            x: cx,
            y: cy,
            height: h,
        }))
    }

    #[frb(sync)]
    pub fn set_selection(&self, start: i32, end: i32) -> Result<LayoutFrame, String> {
        let mut g = lock(self)?;
        g.sel = Some((start, end));
        frame_from_core(paint(&mut g)?)
    }

    #[frb(sync)]
    pub fn clear_selection(&self) -> Result<LayoutFrame, String> {
        let mut g = lock(self)?;
        g.sel = None;
        frame_from_core(paint(&mut g)?)
    }

    #[frb(sync)]
    pub fn insert_text(&self, cp: i32, text: String) -> Result<LayoutFrame, String> {
        let mut g = lock(self)?;
        g.sel = None;
        SessionEdit::insert(&mut g.doc, cp, &text).map_err(map_err)?;
        frame_from_core(paint(&mut g)?)
    }

    #[frb(sync)]
    pub fn remove_range(&self, start: i32, end: i32) -> Result<LayoutFrame, String> {
        let mut g = lock(self)?;
        g.sel = None;
        SessionEdit::remove(&mut g.doc, start, end).map_err(map_err)?;
        frame_from_core(paint(&mut g)?)
    }

    #[frb(sync)]
    pub fn append_paragraph(&self, text: String) -> Result<LayoutFrame, String> {
        let mut g = lock(self)?;
        g.sel = None;
        g.doc.append_paragraph(&text).map_err(map_err)?;
        frame_from_core(paint(&mut g)?)
    }
}
