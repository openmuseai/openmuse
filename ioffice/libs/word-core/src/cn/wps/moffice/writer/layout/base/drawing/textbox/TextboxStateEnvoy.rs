use super::TextboxLayoutState::TextboxLayoutState;
use crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy;

/// Java `TextboxStateEnvoy extends SuperEnvoy<TextboxLayoutState>`.
pub struct TextboxStateEnvoy {
    // Java: protected T mLayoutState;
    // Rust keeps the Java back-reference and rebinds it after a move.
    pub(crate) mLayoutState: *const TextboxLayoutState,
    // Java inheritance gives the typed envoy the same SuperEnvoy object
    // retained by LayoutState; this pointer forwards to that object.
    pub(crate) mSuperEnvoy: *mut SuperEnvoy,
    // The caller-supplied Java envoy is handed to LayoutState's constructor.
    // Once moved there, mSuperEnvoy points to that same base object.
    super_envoy_value: Option<SuperEnvoy>,
}

impl Default for TextboxStateEnvoy {
    fn default() -> Self {
        Self {
            mLayoutState: std::ptr::null(),
            mSuperEnvoy: std::ptr::null_mut(),
            super_envoy_value: Some(SuperEnvoy::default()),
        }
    }
}

impl TextboxStateEnvoy {
    pub(crate) fn takeSuperEnvoy(&mut self) -> SuperEnvoy {
        self.super_envoy_value.take().unwrap_or_default()
    }

    // Java: public boolean isNeedAdjustLastParaBrc()
    pub fn isNeedAdjustLastParaBrc(&self) -> bool {
        // Java: return (this.mLayoutState).needAdjustLastParaBrc;
        unsafe { self.mLayoutState.as_ref() }
            .expect("TextboxStateEnvoy has no TextboxLayoutState")
            .needAdjustLastParaBrc
    }

    pub fn superEnvoy(&mut self) -> &mut SuperEnvoy {
        // Set together with mLayoutState by TextboxLayoutState.getEnvoy.
        unsafe { self.mSuperEnvoy.as_mut() }
            .expect("TextboxStateEnvoy has no SuperEnvoy")
    }
}
