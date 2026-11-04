use word_core::cn::wps::moffice::writer::linegolden::LineCorpus::{run_corpus, CorpusCase};
const ONE: &str =
    r#"[{"page":0,"line":0,"start_cp":0,"end_cp":1,"x":0,"y":0,"width":100,"height":20}]"#;
const LATIN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/03_function_line_corpus/java/latin.json"
));
const CJK: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/03_function_line_corpus/java/cjk.json"
));
const HEADING: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/03_function_line_corpus/java/heading.json"
));
const BOLD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/03_function_line_corpus/java/bold.json"
));
const MULTIPAGE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/03_function_line_corpus/java/multipage.json"
));
#[test]
fn CP1_required_java_golden_categories_parse_and_compare() {
    let cases = [
        CorpusCase {
            name: "latin",
            java_json: LATIN,
            rust_json: LATIN,
        },
        CorpusCase {
            name: "cjk",
            java_json: CJK,
            rust_json: CJK,
        },
        CorpusCase {
            name: "heading",
            java_json: HEADING,
            rust_json: HEADING,
        },
        CorpusCase {
            name: "bold",
            java_json: BOLD,
            rust_json: BOLD,
        },
        CorpusCase {
            name: "multipage",
            java_json: MULTIPAGE,
            rust_json: MULTIPAGE,
        },
    ];
    assert!(run_corpus(&cases, 0).is_ok());
}
#[test]
fn CP2_names_first_failing_fixture() {
    let bad = ONE.replace("\"end_cp\":1", "\"end_cp\":2");
    let e = run_corpus(
        &[CorpusCase {
            name: "cjk",
            java_json: ONE,
            rust_json: &bad,
        }],
        0,
    )
    .unwrap_err();
    assert_eq!(e.case, "cjk");
    assert_eq!(e.diff.field, "end_cp");
}
#[test]
fn CP3_bad_java_json_is_failure() {
    let e = run_corpus(
        &[CorpusCase {
            name: "bad",
            java_json: "{",
            rust_json: ONE,
        }],
        0,
    )
    .unwrap_err();
    assert_eq!(e.diff.field, "java-json");
}
#[test]
fn CP4_bad_rust_json_is_failure() {
    let e = run_corpus(
        &[CorpusCase {
            name: "bad-rust",
            java_json: ONE,
            rust_json: "{",
        }],
        0,
    )
    .unwrap_err();
    assert_eq!(e.diff.field, "rust-json");
}
