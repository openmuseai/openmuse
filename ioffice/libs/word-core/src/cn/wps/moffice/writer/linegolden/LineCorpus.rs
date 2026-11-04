use crate::cn::wps::moffice::writer::cache::Stage2Json::from_stage2_json;
use crate::cn::wps::moffice::writer::linegolden::LineCompare::{compare_lines, LineDiff};

#[derive(Clone, Debug)]
pub struct CorpusCase<'a> {
    pub name: &'a str,
    pub java_json: &'a str,
    pub rust_json: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorpusFailure {
    pub case: String,
    pub diff: LineDiff,
}

pub fn run_corpus(cases: &[CorpusCase<'_>], tolerance_twip: i32) -> Result<(), CorpusFailure> {
    for case in cases {
        let expected = from_stage2_json(case.java_json).map_err(|_| CorpusFailure {
            case: case.name.into(),
            diff: LineDiff { index: 0, field: "java-json".into(), expected: 0, actual: 0 },
        })?;
        let actual = from_stage2_json(case.rust_json).map_err(|_| CorpusFailure {
            case: case.name.into(),
            diff: LineDiff { index: 0, field: "rust-json".into(), expected: 0, actual: 0 },
        })?;
        compare_lines(expected, actual, tolerance_twip).map_err(|diff| CorpusFailure {
            case: case.name.into(),
            diff,
        })?;
    }
    Ok(())
}
