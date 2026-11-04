#![allow(non_snake_case, dead_code, unused_imports)]

/// Soft usable AutoNumFieldCache (Document.getAutoNumField).
/// Full java2rust for AutoNumFieldCache.java still pending.
#[derive(Default)]
pub struct AutoNumFieldCache {
    pub(crate) n: i32,
    // java private Document mDocument;
    pub(crate) mDocument: Option<*mut (dyn crate::cn::wps::moffice::writer::core::Document::Document + 'static)>,
}

impl AutoNumFieldCache {
    // rust: Java `public AutoNumFieldCache(Document mdocument)`
    //   `this.mDocument = mdocument;`
    //   `mdocument.getDispatcher().addDocumentListener(this);` — dispatcher 仍 DEFERRED
    pub fn AutoNumFieldCache(
        mdocument: &dyn crate::cn::wps::moffice::writer::core::Document::Document,
    ) -> Self {
        Self {
            n: 0,
            mDocument: Some(unsafe {
                std::mem::transmute::<
                    *const dyn crate::cn::wps::moffice::writer::core::Document::Document,
                    *mut (dyn crate::cn::wps::moffice::writer::core::Document::Document + 'static),
                >(
                    mdocument
                        as *const dyn crate::cn::wps::moffice::writer::core::Document::Document,
                )
            }),
        }
    }

    pub fn new() -> Self {
        Self::default()
    }
    pub fn dispose(&mut self) {
        self.mDocument = None;
    }

    // java public int getAutoNum(long range)
    pub fn getAutoNum(&self, _range: i64) -> i32 {
        self.n
    }
}
