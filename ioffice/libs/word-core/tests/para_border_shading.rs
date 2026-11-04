#![allow(non_snake_case)]

use std::sync::Arc;
use drawing::cn::wps::moffice::drawing::graphics::SHD::SHD;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use word_core::cn::wps::moffice::writer::data::value::BRC::BRC;
use word_core::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
use word_core::cn::wps::moffice::writer::layout::base::env::ParaPropEnv::ParaPropEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use word_core::cn::wps::moffice::writer::layout::base::ParaLayouter::ParaLayouter;

#[test]
fn java_border_and_shading_property_slots() {
    let mut props = MutablePropertySet::new();
    props.setValue(230, Some(Arc::new(BRC::new__float_int(1.0, 1))));
    props.setValue(229, Some(Arc::new(BRC::new__float_int(1.0, 0))));
    props.setValue(228, Some(Arc::new(SHD::with_all(-2, 0x00ff00, 1))));
    let mut env = ParaPropEnv::default();
    env.paraProperty = Some(props.freeze());
    let mut layouter = ParaLayouter::ParaLayouter(DocumentEnv::default(), ViewEnv::default(), MainState::default());

    layouter.calcBRC(&mut env);
    layouter.calcSHD(&mut env);
    assert_eq!(env.brcLeft.as_ref().map(BRC::getBorderType), Some(1));
    assert!(env.brcTop.is_none());
    assert!(env.vaildBRC);
    assert!(env.paraSHD.is_some());

    env.paraProperty = Some(MutablePropertySet::new().freeze());
    layouter.calcBRC(&mut env);
    layouter.calcSHD(&mut env);
    assert!(!env.vaildBRC);
    assert!(env.paraSHD.is_none());
}
