use word_core::cn::wps::moffice::writer::layout::base::text::Wrap::{wrap, LayoutRun};

/// W1
#[test]
fn w1_single_run_fits() {
    let lines = wrap(
        &[LayoutRun {
            text: "Hi".into(),
            width: 40,
        }],
        200,
    );
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].width, 40);
}

/// W2
#[test]
fn w2_two_runs_wrap_between() {
    let lines = wrap(
        &[
            LayoutRun {
                text: "AAAA".into(),
                width: 80,
            },
            LayoutRun {
                text: "BBBB".into(),
                width: 80,
            },
        ],
        100,
    );
    assert_eq!(lines.len(), 2);
}

/// W3
#[test]
fn w3_wrap_at_space() {
    let lines = wrap(
        &[LayoutRun {
            text: "hello world".into(),
            width: 110,
        }],
        70,
    );
    assert!(lines.len() >= 2, "got {} lines", lines.len());
}

/// W4
#[test]
fn w4_long_word_force_break() {
    let lines = wrap(
        &[LayoutRun {
            text: "AAAA".into(),
            width: 400,
        }],
        50,
    );
    assert!(lines.len() >= 2);
    assert!(lines.len() <= 4);
}

/// W5
#[test]
fn w5_empty_para_one_line() {
    let lines = wrap(&[], 200);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].width, 0);
}

/// W6
#[test]
fn w6_cr_ends_line() {
    let lines = wrap(
        &[LayoutRun {
            text: "ab\r".into(),
            width: 40,
        }],
        200,
    );
    assert!(!lines.is_empty());
    assert!(lines[0].width <= 40);
}
