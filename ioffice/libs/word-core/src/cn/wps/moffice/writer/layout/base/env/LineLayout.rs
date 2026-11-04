#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine;
use crate::cn::wps::moffice::writer::layout::base::env::ContentWidth::ContentWidth;
use crate::cn::wps::moffice::writer::layout::base::env::LineAlign::LineAlign;
use crate::cn::wps::moffice::writer::layout::base::env::LineBox::{LineBox, LineSpacing};
use crate::cn::wps::moffice::writer::layout::base::env::LineWrap::LineWrap;

/// wr-11 measured run: per-cp advances + channel metrics + text.
#[derive(Clone, Debug)]
pub struct LayoutRun {
    pub text: String,
    pub advances: Vec<i32>,
    pub ascent: i32,
    pub descent: i32,
}

/// PAPX 193/208/220/217/214 plus previous paragraph's after (for collapse).
#[derive(Clone, Debug)]
pub struct ParaPapx {
    pub jc: i32,
    pub first_line_indent: i32,
    pub spacing: LineSpacing,
    pub para_before: i32,
    pub para_after: i32,
    pub prev_para_after: i32,
}

impl Default for ParaPapx {
    fn default() -> Self {
        ParaPapx {
            jc: 0,
            first_line_indent: 0,
            spacing: LineSpacing::auto(1.0),
            para_before: 0,
            para_after: 0,
            prev_para_after: 0,
        }
    }
}

/// wr-12 feature exit: 01–04 into existing `TypoTextLine`. No pagination, no wr-04 Wrap.
pub fn layout_lines(runs: &[LayoutRun], content_width: i32, papx: &ParaPapx) -> Vec<TypoTextLine> {
    let (text, advances, ascent, descent) = flatten_runs(runs);
    if text.is_empty() {
        return Vec::new();
    }

    let chs: Vec<char> = text.chars().collect();
    let n = chs.len();
    let mut out: Vec<TypoTextLine> = Vec::new();
    let mut offset: usize = 0;
    let mut first = true;

    while offset < n {
        let max_width = ContentWidth::lineMaxWidth(content_width, papx.first_line_indent, first);
        let rest_text: String = chs[offset..].iter().collect();
        let ranges = LineWrap::wrap(&advances[offset..], &rest_text, max_width);
        if ranges.is_empty() {
            break;
        }
        let r = ranges[0];
        let start = offset as i32 + r.start_cp;
        let end = offset as i32 + r.end_cp;
        if end as usize <= offset {
            break;
        }
        offset = end as usize;
        first = false;
        out.push(fill_pending_line(start, end, r.width, max_width, ascent, descent, papx));
    }

    let last = out.len().saturating_sub(1);
    for (i, line) in out.iter_mut().enumerate() {
        let is_first = i == 0;
        let is_last = i == last;
        let box_ = LineBox::measure(
            ascent,
            descent,
            papx.spacing,
            papx.para_before,
            papx.para_after,
            is_first,
            is_last,
            papx.prev_para_after,
        );
        line.setTextRectHeight(box_.height);
        line.setAscDesPos(box_.baseline, box_.height - box_.baseline);
        line.setFirstLineOfPara(is_first);
        line.setLastLineOfPara(is_last);
    }
    out
}

fn flatten_runs(runs: &[LayoutRun]) -> (String, Vec<i32>, i32, i32) {
    let mut text = String::new();
    let mut advances = Vec::new();
    let mut ascent = 0i32;
    let mut descent = 0i32;
    for run in runs {
        text.push_str(&run.text);
        advances.extend_from_slice(&run.advances);
        if run.ascent > ascent {
            ascent = run.ascent;
        }
        if run.descent > descent {
            descent = run.descent;
        }
    }
    (text, advances, ascent, descent)
}

fn fill_pending_line(
    start: i32,
    end: i32,
    width: i32,
    line_max_width: i32,
    _ascent: i32,
    _descent: i32,
    papx: &ParaPapx,
) -> TypoTextLine {
    let mut line = TypoTextLine::new();
    line.setStartCP(start);
    line.setEndCP(end);
    line.setWidth(width);
    line.setLeft(LineAlign::alignX(papx.jc, width, line_max_width));
    line
}
