use crate::cn::wps::moffice::writer::api::LayoutDocument::{FontBlob, LayoutError, ViewParams};
use crate::cn::wps::moffice::writer::cache::RustStage2::from_layout;
use crate::cn::wps::moffice::writer::cache::Stage2Json::from_stage2_json;
use crate::cn::wps::moffice::writer::linegolden::LineCompare::{compare_lines, LineDiff};

#[derive(Clone, Debug)]
pub struct LiveCase<'a> {
    pub name: &'a str,
    pub docx: Option<&'a [u8]>,
    pub fonts: &'a [FontBlob],
    pub view: &'a ViewParams,
    pub java_json: Option<&'a str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorpusFailure {
    pub case: String,
    pub diff: LineDiff,
}

pub fn run_live_corpus(cases: &[LiveCase<'_>], tolerance_twip: i32) -> Result<(), CorpusFailure> {
    for case in cases {
        let java_json = case.java_json.ok_or_else(|| CorpusFailure {
            case: case.name.into(),
            diff: LineDiff {
                index: 0,
                field: "java-json".into(),
                expected: 0,
                actual: 0,
            },
        })?;
        let docx = case.docx.ok_or_else(|| CorpusFailure {
            case: case.name.into(),
            diff: LineDiff {
                index: 0,
                field: "missing-docx".into(),
                expected: 0,
                actual: 0,
            },
        })?;
        let expected = from_stage2_json(java_json).map_err(|_| CorpusFailure {
            case: case.name.into(),
            diff: LineDiff {
                index: 0,
                field: "java-json".into(),
                expected: 0,
                actual: 0,
            },
        })?;
        let actual = from_layout(docx, case.fonts, case.view).map_err(|e: LayoutError| CorpusFailure {
            case: case.name.into(),
            diff: LineDiff {
                index: 0,
                field: format!("layout:{}", e.message),
                expected: 0,
                actual: 0,
            },
        })?;
        compare_lines(expected, actual, tolerance_twip).map_err(|diff| CorpusFailure {
            case: case.name.into(),
            diff,
        })?;
    }
    Ok(())
}
