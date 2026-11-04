//! Paginate a line stream by content height and form-feed.

#[derive(Clone, Debug)]
pub struct FlowLine {
    pub height: i32,
    pub has_form_feed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageFill {
    pub start_line: usize,
    pub end_line: usize,
    pub used_height: i32,
    pub last_line_of_page: bool,
}

pub fn flow_lines_into_pages(lines: &[FlowLine], content_height: i32) -> Vec<PageFill> {
    if lines.is_empty() {
        return Vec::new();
    }
    let mut pages = Vec::new();
    let mut start = 0usize;
    let mut used = 0i32;
    for (i, line) in lines.iter().enumerate() {
        let overflow = used > 0 && content_height > 0 && used + line.height > content_height;
        if overflow {
            pages.push(PageFill {
                start_line: start,
                end_line: i,
                used_height: used,
                last_line_of_page: true,
            });
            start = i;
            used = 0;
        }
        used += line.height;
        if line.has_form_feed {
            pages.push(PageFill {
                start_line: start,
                end_line: i + 1,
                used_height: used,
                last_line_of_page: true,
            });
            start = i + 1;
            used = 0;
        }
    }
    if start < lines.len() {
        pages.push(PageFill {
            start_line: start,
            end_line: lines.len(),
            used_height: used,
            last_line_of_page: true,
        });
    }
    pages
}
