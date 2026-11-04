#![allow(non_snake_case, dead_code, unused_imports)]
use crate::cn::wps::moffice::writer::core::Document::Document;

/// Java `AutoWordStat(Document document)` — stores `mDoc` / `mCore`.
pub struct AutoWordStat {
    // java private final Document mDoc;
    pub(crate) mDoc: Option<*mut (dyn Document + 'static)>,
}

impl Default for AutoWordStat {
    fn default() -> Self {
        Self { mDoc: None }
    }
}

impl AutoWordStat {
    // java public AutoWordStat(Document document)
    pub fn AutoWordStat(document: &dyn Document) -> Self {
        Self {
            // java this.mDoc = document;
            // java this.mCore = this.mDoc.getTextDocument();
            // java this.mCallbacks = new ArrayList();
            mDoc: Some(unsafe {
                std::mem::transmute::<*const dyn Document, *mut (dyn Document + 'static)>(
                    document as *const dyn Document,
                )
            }),
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn core_is_some(&self) -> bool {
        unsafe {
            self.mDoc
                .and_then(|d| (*d).getTextDocument())
                .is_some()
        }
    }
}
