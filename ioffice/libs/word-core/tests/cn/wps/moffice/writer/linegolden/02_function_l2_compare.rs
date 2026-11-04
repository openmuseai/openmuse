// function_id=02_function_l2_compare
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::cache::Stage2Rows::Stage2Line;
use word_core::cn::wps::moffice::writer::linegolden::LineCompare::compare_lines;

fn r() -> Stage2Line {
    Stage2Line {
        page: 0,
        line: 0,
        start_cp: 0,
        end_cp: 2,
        x: 10,
        y: 20,
        width: 30,
        height: 40,
        xpos: vec![10, 20],
    }
}

#[test]
fn LC1_cp_mismatch_fails() {
    let mut a = r();
    a.end_cp = 9;
    let d = compare_lines(vec![r()], vec![a], 99).unwrap_err();
    assert_eq!(d.field, "end_cp");
}

#[test]
fn LC2_x_over_tolerance_fails() {
    let mut a = r();
    a.x += 3;
    let d = compare_lines(vec![r()], vec![a], 1).unwrap_err();
    assert_eq!(d.field, "x");
}

#[test]
fn LC3_geometry_equal_ok() {
    assert!(compare_lines(vec![r()], vec![r()], 0).is_ok());
}

#[test]
fn LC4_xpos_only_still_ok() {
    let mut a = r();
    a.xpos = vec![99, 100, 101];
    assert!(
        compare_lines(vec![r()], vec![a], 0).is_ok(),
        "L2 must ignore xpos"
    );
}
