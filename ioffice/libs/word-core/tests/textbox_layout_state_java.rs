use word_core::cn::wps::moffice::writer::layout::base::drawing::textbox::TextboxLayoutState::TextboxLayoutState;
use word_core::cn::wps::moffice::writer::layout::base::drawing::textbox::TextboxStateEnvoy::TextboxStateEnvoy;

#[test]
fn textbox_state_envoy_reads_moved_state() {
    // Java TextboxLayoutState(null) constructs the textbox envoy and starts
    // all excess measurements at zero. The state may move before getEnvoy.
    let state = TextboxLayoutState::new(None);
    let mut states = Vec::new();
    states.push(state);
    let state = &mut states[0];
    assert!(!state.getEnvoy().isNeedAdjustLastParaBrc());
    state.needAdjustLastParaBrc = true;
    assert!(state.getEnvoy().isNeedAdjustLastParaBrc());
    let base_ptr = &mut state.base as *mut _ as *mut ();
    assert_eq!(state.getEnvoy().superEnvoy().getLayoutState(), base_ptr);
    assert_eq!(state.maxLineCoreWidth, 0);
    assert_eq!(state.maxLeftExcessedCellMargin, 0);
    assert_eq!(state.maxRightExcessedCellMargin, 0);
    assert_eq!(state.minParaBrcLeft, 0);
    assert_eq!(state.maxParaBrcRight, 0);

    // Java accepts a caller-provided TextboxStateEnvoy as well as null.
    let mut supplied = TextboxLayoutState::new(Some(TextboxStateEnvoy::default()));
    assert!(!supplied.getEnvoy().isNeedAdjustLastParaBrc());
    let supplied_base = &mut supplied.base as *mut _ as *mut ();
    assert_eq!(supplied.getEnvoy().superEnvoy().getLayoutState(), supplied_base);
}
