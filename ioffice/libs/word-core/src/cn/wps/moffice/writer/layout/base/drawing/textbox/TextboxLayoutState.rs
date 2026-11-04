use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use super::TextboxStateEnvoy::TextboxStateEnvoy;

/// Java `TextboxLayoutState extends LayoutState`. Rust owns its base state.
pub struct TextboxLayoutState {
    pub base: LayoutState,
    // Java: public boolean needAdjustLastParaBrc = false;
    pub needAdjustLastParaBrc: bool,
    // Java: public boolean needCollectExcessInfo = false;
    pub needCollectExcessInfo: bool,
    // Java: public int maxLineCoreWidth;
    pub maxLineCoreWidth: i32,
    // Java: public int maxLeftExcessedCellMargin;
    pub maxLeftExcessedCellMargin: i32,
    // Java: public int maxRightExcessedCellMargin;
    pub maxRightExcessedCellMargin: i32,
    // Java: public int minParaBrcLeft;
    pub minParaBrcLeft: i32,
    // Java: public int maxParaBrcRight;
    pub maxParaBrcRight: i32,
    envoy: TextboxStateEnvoy,
}

impl TextboxLayoutState {
    // Java: public TextboxLayoutState(TextboxStateEnvoy stateEnvoy)
    pub fn new(state_envoy: Option<TextboxStateEnvoy>) -> Self {
        // Java: super(stateEnvoy != null ? stateEnvoy : new TextboxStateEnvoy());
        let mut envoy = state_envoy.unwrap_or_default();
        let base_envoy = envoy.takeSuperEnvoy();
        Self {
            base: LayoutState::LayoutState(Some(base_envoy)),
            needAdjustLastParaBrc: false,
            needCollectExcessInfo: false,
            maxLineCoreWidth: 0,
            maxLeftExcessedCellMargin: 0,
            maxRightExcessedCellMargin: 0,
            minParaBrcLeft: 0,
            maxParaBrcRight: 0,
            envoy,
        }
    }

    // Java: public TextboxStateEnvoy getEnvoy()
    pub fn getEnvoy(&mut self) -> &mut TextboxStateEnvoy {
        // Java: return (TextboxStateEnvoy)super.getEnvoy();
        // Rebind after a Rust move, as LayoutState.getEnvoy does.
        self.envoy.mLayoutState = self as *const Self;
        self.envoy.mSuperEnvoy = self.base.getEnvoy() as *mut _;
        &mut self.envoy
    }
}

