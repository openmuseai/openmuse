use crate::cn::wps::moffice::writer::cache::Stage2Rows::Stage2Line;
use crate::cn::wps::moffice::writer::linegolden::LineNormalize::normalize_lines;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineDiff {
    pub index: usize,
    pub field: String,
    pub expected: i32,
    pub actual: i32,
}

pub fn compare_lines(
    expected: Vec<Stage2Line>,
    actual: Vec<Stage2Line>,
    geometry_tolerance_twip: i32,
) -> Result<(), LineDiff> {
    let expected = normalize_lines(expected).map_err(|_| LineDiff {
        index: 0,
        field: "expected-invalid".into(),
        expected: 0,
        actual: 0,
    })?;
    let actual = normalize_lines(actual).map_err(|_| LineDiff {
        index: 0,
        field: "actual-invalid".into(),
        expected: 0,
        actual: 0,
    })?;
    if expected.len() != actual.len() {
        return Err(LineDiff {
            index: expected.len().min(actual.len()),
            field: "line-count".into(),
            expected: expected.len() as i32,
            actual: actual.len() as i32,
        });
    }
    let tol = geometry_tolerance_twip.max(0);
    for (index, (e, a)) in expected.iter().zip(&actual).enumerate() {
        for (field, ev, av, exact) in [
            ("page", e.page, a.page, true),
            ("line", e.line, a.line, true),
            ("start_cp", e.start_cp, a.start_cp, true),
            ("end_cp", e.end_cp, a.end_cp, true),
            ("x", e.x, a.x, false),
            ("y", e.y, a.y, false),
            ("width", e.width, a.width, false),
            ("height", e.height, a.height, false),
        ] {
            let different = if exact { ev != av } else { ev.abs_diff(av) > tol as u32 };
            if different {
                return Err(LineDiff {
                    index,
                    field: field.into(),
                    expected: ev,
                    actual: av,
                });
            }
        }
    }
    Ok(())
}
