#![allow(non_snake_case)]

/// Java page overflow rule reduced to its pure I/O boundary.
pub struct PageFit;

impl PageFit {
    /// A page breaks before `line_height` only when it already owns a line.
    /// This preserves progress for a single line taller than the content box.
    pub fn shouldBreakBefore(
        used_height: i32,
        line_height: i32,
        content_height: i32,
        page_has_lines: bool,
    ) -> bool {
        if !page_has_lines {
            return false;
        }
        let used = used_height.max(0);
        let line = line_height.max(0);
        let limit = content_height.max(0);
        i64::from(used) + i64::from(line) > i64::from(limit)
    }
}
