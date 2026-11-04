use word_core::cn::wps::moffice::writer::layout::base::env::ParaEnv::ParaEnv;
use word_core::cn::wps::moffice::writer::layout::base::env::Pools::Pools;

#[test]
fn paragraph_environment_recycles_previous_and_moves_current_at_end() {
    let mut pools = Pools::default();
    let mut env = ParaEnv::ParaEnv();
    let mut previous = pools.obtainParaPropEnv();
    previous.start = 10;
    previous.hasAutoNums = true;
    previous.autoNumString = Some("1.".into());
    env.prevParaPropEnv = Some(previous);

    let mut current = pools.obtainParaPropEnv();
    current.start = 20;
    env.curParaPropEnv = Some(current);

    env.saveBrcAndShdAjustInfo(&mut pools);
    assert!(env.curParaPropEnv.is_none());
    assert_eq!(env.prevParaPropEnv.as_ref().unwrap().start, 20);

    let recycled = pools.obtainParaPropEnv();
    assert!(!recycled.hasAutoNums);
    assert!(recycled.autoNumString.is_none());

    env.clearBrcAndShdAdjustInfo(&mut pools);
    assert!(env.prevParaPropEnv.is_none());
    assert!(env.curParaPropEnv.is_none());
    assert!(env.nextParaPropEnv.is_none());

    env.wordWrap = false;
    env.isRTL = true;
    env.prevParaPropEnv = Some(pools.obtainParaPropEnv());
    env.init(&mut pools);
    assert!(env.wordWrap);
    assert!(!env.isRTL);
    assert!(env.prevParaPropEnv.is_none());
}
