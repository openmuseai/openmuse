// function_id=01_function_compress_env
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::layout::base::line::LineCompressEnv::LineCompressEnv;

fn env() -> LineCompressEnv {
    let mut e = LineCompressEnv::LineCompressEnv();
    e.reset();
    e
}

#[test]
fn CE1_reset_clears_excess_and_sums() {
    let mut e = env();
    e.addSpaceCompressableSpace(1, 10);
    e.consumeCompressableSpaces(1, 3);
    e.reset();
    assert_eq!(e.firstExcessSpaceGraphIndex, i32::MIN);
    assert_eq!(e.firstExcessPunctGraphIndex, i32::MIN);
    assert_eq!(e.getLeftedCompressableSpaceSum(), 0);
    assert!(e.isFirstCompress());
}

#[test]
fn CE2_add_space_zero_skips_after() {
    let mut e = env();
    e.addSpaceCompressableSpace(3, 0);
    assert_eq!(e.getCompressAfter(3), 0);
    assert_eq!(e.getMaxSpaceCompressableSpaceSum(), 0);
}

#[test]
fn CE3_add_space_records_after_and_lefted() {
    let mut e = env();
    e.addSpaceCompressableSpace(3, 10);
    assert_eq!(e.getCompressAfter(3), 10);
    assert_eq!(e.getLeftedCompressableSpaceSum(), 10);
}

#[test]
fn CE4_add_punct_before_after() {
    let mut e = env();
    e.addPunctCompressableSpace(1, 4, 5, 9);
    assert_eq!(e.getCompressBefore(1), 4);
    assert_eq!(e.getCompressAfter(1), 5);
    assert_eq!(e.getLeftedCompressableSpaceSum(), 9);
}

#[test]
fn CE5_add_other_zero_then_n() {
    let mut e = env();
    e.addOtherCompressableSpace(2, 0);
    assert_eq!(e.getCompressAfter(2), 0);
    e.addOtherCompressableSpace(2, 7);
    assert_eq!(e.getCompressAfter(2), 7);
    assert_eq!(e.getLeftedCompressableSpaceSum(), 7);
}

#[test]
fn CE6_duplicate_graph_index_accumulates_after() {
    let mut e = env();
    e.addSpaceCompressableSpace(1, 4);
    e.addSpaceCompressableSpace(1, 6);
    assert_eq!(e.getCompressAfter(1), 10);
    assert_eq!(e.getLeftedCompressableSpaceSum(), 10);
}

#[test]
fn CE7_consume_need_zero_records_but_used_stays_zero() {
    let mut e = env();
    e.addSpaceCompressableSpace(0, 8);
    e.consumeCompressableSpaces(0, 0);
    assert!(!e.isFirstCompress());
    assert_eq!(e.getUsedCompressableSpaceSum(), 0);
    assert_eq!(e.getLeftedCompressableSpaceSum(), 8);
}

#[test]
fn CE8_consume_crosses_space_punct_other() {
    let mut e = env();
    e.addSpaceCompressableSpace(0, 10);
    e.addPunctCompressableSpace(1, 0, 0, 10);
    e.addOtherCompressableSpace(2, 10);
    e.consumeCompressableSpaces(5, 25);
    assert_eq!(e.getUsedSpaceCompressableSpaceSum(), 10);
    assert_eq!(e.getUsedPunctCompressableSpaceSum(), 10);
    assert_eq!(e.getUsedOtherCompressableSpaceSum(), 5);
    assert_eq!(e.getUsedCompressableSpaceSum(), 25);
    assert_eq!(e.getLeftedCompressableSpaceSum(), 5);
}

#[test]
fn CE9_restore_with_no_compressed_zeros_used() {
    let mut e = env();
    e.addSpaceCompressableSpace(0, 8);
    e.restoreCompressState(99);
    assert_eq!(e.getUsedCompressableSpaceSum(), 0);
}

#[test]
fn CE10_restore_hits_first_consume_slot() {
    let mut e = env();
    e.addSpaceCompressableSpace(0, 20);
    e.consumeCompressableSpaces(1, 4);
    e.consumeCompressableSpaces(2, 3);
    assert_eq!(e.getUsedSpaceCompressableSpaceSum(), 7);
    e.restoreCompressState(1);
    assert_eq!(e.getUsedSpaceCompressableSpaceSum(), 4);
}

#[test]
fn CE11_get_compress_before_unknown_is_zero() {
    let e = env();
    assert_eq!(e.getCompressBefore(99), 0);
}

#[test]
fn CE12_first_compressed_graph_index() {
    let mut e = env();
    assert_eq!(e.getFirstCompressedGraphIndex(), i32::MIN);
    e.addSpaceCompressableSpace(0, 4);
    e.consumeCompressableSpaces(7, 1);
    assert_eq!(e.getFirstCompressedGraphIndex(), 7);
}

#[test]
fn CE13_set_get_final_compress() {
    let mut e = env();
    e.setFinalCompress(42);
    assert_eq!(e.getFinalCompress(), 42);
}

#[test]
fn CE14_dispose_nulls_lists_like_java() {
    let mut e = env();
    e.addSpaceCompressableSpace(1, 4);
    e.dispose();
    let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = e.getCompressAfter(1);
    }));
    assert!(panicked.is_err(), "Java NPE after dispose; must not silently return");
}

#[test]
fn CE15_lefted_after_ce8_style_consume() {
    let mut e = env();
    e.addSpaceCompressableSpace(0, 10);
    e.addPunctCompressableSpace(1, 0, 0, 10);
    e.addOtherCompressableSpace(2, 10);
    e.consumeCompressableSpaces(5, 25);
    assert_eq!(e.getLeftedCompressableSpaceSum(), 5);
}
