use crate::cn::wps::moffice::writer::cache::Stage2Rows::Stage2Line;

pub fn normalize_lines(mut rows: Vec<Stage2Line>) -> Result<Vec<Stage2Line>, String> {
    rows.sort_by_key(|row| (row.page, row.line));
    let mut previous: Option<&Stage2Line> = None;
    for row in &rows {
        if row.page < 0 || row.line < 0 || row.start_cp < 0 || row.end_cp < row.start_cp {
            return Err(format!("invalid line coordinates at page {} line {}", row.page, row.line));
        }
        if row.width < 0 || row.height < 0 {
            return Err(format!("negative geometry at page {} line {}", row.page, row.line));
        }
        if let Some(prev) = previous {
            if prev.page == row.page && prev.line == row.line {
                return Err(format!("duplicate page/line {},{}", row.page, row.line));
            }
        }
        previous = Some(row);
    }
    Ok(rows)
}
