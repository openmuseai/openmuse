use word_core::cn::wps::moffice::writer::layout::base::PageBreak::{
    flow_lines_into_pages, FlowLine,
};

fn h(n: i32) -> FlowLine {
    FlowLine {
        height: n,
        has_form_feed: false,
    }
}

#[test]
fn b1_fill_two_pages() {
    let pages = flow_lines_into_pages(&[h(100), h(100), h(100)], 250);
    assert_eq!(pages.len(), 2);
    assert_eq!(pages[0].end_line - pages[0].start_line, 2);
    assert_eq!(pages[1].end_line - pages[1].start_line, 1);
}

#[test]
fn b2_exact_one_page() {
    let pages = flow_lines_into_pages(&[h(100), h(100)], 200);
    assert_eq!(pages.len(), 1);
}

#[test]
fn b3_single_line_overflow() {
    let pages = flow_lines_into_pages(&[h(300)], 250);
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].used_height, 300);
}

#[test]
fn b4_form_feed() {
    let mut ff = h(50);
    ff.has_form_feed = true;
    let pages = flow_lines_into_pages(&[h(50), ff, h(50)], 500);
    assert!(pages.len() >= 2);
}

#[test]
fn b5_last_line_flag() {
    let pages = flow_lines_into_pages(&[h(100), h(100), h(100)], 250);
    assert!(pages.iter().all(|p| p.last_line_of_page));
}

#[test]
fn b6_empty() {
    assert!(flow_lines_into_pages(&[], 250).is_empty());
}
