// function_id=03_function_stage1_compare
// cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_stage1_compare

use word_core::cn::wps::moffice::writer::api::stage1::PropDump::{Stage1Papx, Stage1Props};
use word_core::cn::wps::moffice::writer::api::stage1::Stage1Compare::{Stage1Compare, Stage1Doc};

fn doc(text: &str, jc: Option<i32>) -> Stage1Doc {
    Stage1Doc {
        text: text.to_string(),
        props: Stage1Props {
            papx: vec![Stage1Papx {
                start_cp: 0,
                end_cp: 1,
                p193: jc,
                ..Stage1Papx::default()
            }],
            chpx: vec![],
        },
    }
}

/// SC1 — identical Stage1 is Ok.
#[test]
fn sc1_equal_ok() {
    let a = doc("Hello\r", Some(1));
    assert!(Stage1Compare::diff(&a, &a).is_ok());
}

/// SC2 — text mismatch reports path text.
#[test]
fn sc2_text_diff() {
    let j = doc("Hello\r", Some(1));
    let r = doc("Hellp\r", Some(1));
    let err = Stage1Compare::diff(&j, &r).expect_err("text");
    assert_eq!(err.path, "text");
}

/// SC3 — 193 0 vs 1 is papx[0], not swallowed as default.
#[test]
fn sc3_sprm_diff() {
    let j = doc("Hello\r", Some(1));
    let r = doc("Hello\r", Some(0));
    let err = Stage1Compare::diff(&j, &r).expect_err("sprm");
    assert_eq!(err.path, "papx[0]");
}

/// SC4 — illegal JSON is parse err, not panic.
#[test]
fn sc4_bad_json() {
    let err = Stage1Compare::diff_json("{", "{}").expect_err("json");
    assert!(err.path.contains("parse"), "{}", err.path);
}

/// SC6 — papx length mismatch.
#[test]
fn sc6_papx_len() {
    let j = doc("A", Some(1));
    let mut r = doc("A", Some(1));
    r.props.papx.clear();
    let err = Stage1Compare::diff(&j, &r).expect_err("len");
    assert_eq!(err.path, "papx.len");
}

/// SC7 — chpx span mismatch.
#[test]
fn sc7_chpx_diff() {
    use word_core::cn::wps::moffice::writer::api::stage1::PropDump::Stage1Chpx;
    let mut j = doc("A", None);
    let mut r = doc("A", None);
    j.props.chpx.push(Stage1Chpx {
        start_cp: 0,
        end_cp: 1,
        sz_twip: Some(240),
        bold: None,
        east_asia: None,
    });
    r.props.chpx.push(Stage1Chpx {
        start_cp: 0,
        end_cp: 1,
        sz_twip: Some(360),
        bold: None,
        east_asia: None,
    });
    let err = Stage1Compare::diff(&j, &r).expect_err("chpx");
    assert_eq!(err.path, "chpx[0]");
}
