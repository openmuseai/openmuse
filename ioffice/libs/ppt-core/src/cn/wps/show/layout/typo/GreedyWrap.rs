//! MVP 贪心断行（pl-05-01）。对照 `RichTextLayout.typo` 的行区间语义，**不是** Java 一致断行。

use super::TextBoxGeometry::RectF;
use wpslayout::cn::wps::text::layout::data::SpanInfo::SpanInfo;
use wpslayout::cn::wps::text::layout::data::TextBoxData::TextBoxData;
use wpslayout::cn::wps::text::layout::data::TextData::TextData;

// java public class ShowTypoLine
#[derive(Clone, Debug, PartialEq)]
pub struct ShowTypoLine {
    // java String text
    pub text: String,
    // java int startCp
    pub startCp: i32,
    // java int endCp
    pub endCp: i32,
    // java float x
    pub x: f32,
    // java float y
    pub y: f32,
    // java float w
    pub w: f32,
    // java float h
    pub h: f32,
}

// java public class GreedyWrap
pub struct GreedyWrap {}

impl GreedyWrap {
    // java Vec greedy_wrap(TextBoxData data, RectF textRect, float[][] advances)
    pub fn greedy_wrap(
        data: &TextBoxData,
        text_rect: RectF,
        advances: &[Vec<f32>],
    ) -> Vec<ShowTypoLine> {
        // java if (data.isEmpty()) return empty
        if data.isEmpty() {
            return Vec::new();
        }
        let max_width = text_rect.right - text_rect.left;
        let wrap = data.isWrapTextInShape();
        let mut lines: Vec<ShowTypoLine> = Vec::new();
        let mut y = text_rect.top;
        let datas = data.getTextDatas();
        for (pi, td) in datas.iter().enumerate() {
            let adv = advances.get(pi).map(|v| v.as_slice()).unwrap_or(&[]);
            let para_lines = Self::wrap_one_para(td, max_width, wrap, adv, text_rect.left, &mut y);
            lines.extend(para_lines);
        }
        Self::apply_anchor(&mut lines, text_rect, data.getAnchor());
        lines
    }

    fn wrap_one_para(
        td: &TextData,
        max_width: f32,
        wrap: bool,
        advances: &[f32],
        x0: f32,
        y: &mut f32,
    ) -> Vec<ShowTypoLine> {
        let source = td.mSourceText.as_deref().unwrap_or("");
        let chars: Vec<char> = source.chars().filter(|c| *c != '\r').collect();
        let token_w = |a: usize, b: usize| -> f32 {
            let mut w = 0.0;
            for i in a..b {
                w += advances.get(i).copied().unwrap_or(0.0);
            }
            w
        };
        let mut flush = |start: usize, end: usize, w: f32, y: &mut f32, out: &mut Vec<ShowTypoLine>| {
            let text: String = if start < end {
                chars[start..end].iter().collect()
            } else {
                String::new()
            };
            let mut h: f32 = 0.0;
            if start < end {
                for i in start..end {
                    h = h.max(span_height_at(td, i as i32));
                }
            } else {
                h = span_height_at(td, 0);
            }
            out.push(ShowTypoLine {
                text,
                startCp: start as i32,
                endCp: end as i32,
                x: x0,
                y: *y,
                w,
                h,
            });
            *y += h;
        };
        let mut out: Vec<ShowTypoLine> = Vec::new();
        if chars.is_empty() {
            flush(0, 0, 0.0, y, &mut out);
            return out;
        }
        if !wrap {
            flush(0, chars.len(), token_w(0, chars.len()), y, &mut out);
            return out;
        }
        let tokens = tokenize(&chars);
        let mut line_start: usize = 0;
        let mut line_end: usize = 0;
        let mut line_w: f32 = 0.0;
        for (ts, te) in tokens {
            let tw = token_w(ts, te);
            let is_space = chars[ts..te].iter().all(|c| c.is_whitespace());
            if line_end == line_start {
                line_start = ts;
                line_end = te;
                line_w = tw;
                continue;
            }
            if line_w + tw <= max_width {
                line_end = te;
                line_w += tw;
            } else {
                flush(line_start, line_end, line_w, y, &mut out);
                if is_space {
                    line_start = te;
                    line_end = te;
                    line_w = 0.0;
                } else {
                    line_start = ts;
                    line_end = te;
                    line_w = tw;
                }
            }
        }
        if line_end > line_start {
            flush(line_start, line_end, line_w, y, &mut out);
        }
        out
    }

    fn apply_anchor(lines: &mut [ShowTypoLine], text_rect: RectF, anchor: i32) {
        if lines.is_empty() {
            return;
        }
        let avail = text_rect.bottom - text_rect.top;
        let last = lines.last().unwrap();
        let total_h = last.y + last.h - text_rect.top;
        let extra = match anchor {
            1 => avail - total_h,
            2 => (avail - total_h) / 2.0,
            _ => 0.0,
        };
        if extra.abs() < 1e-6 {
            return;
        }
        for ln in lines.iter_mut() {
            ln.y += extra;
        }
    }
}

fn span_height_at(td: &TextData, cp: i32) -> f32 {
    let spans: Vec<&SpanInfo> = std::iter::once(&td.mFirstSpan)
        .chain(td.mOtherSpans.iter())
        .collect();
    for sp in spans {
        if cp >= sp.mStartPos && cp < sp.mEndPos {
            return sp.mFontSize;
        }
    }
    td.mFirstSpan.mFontSize
}

fn is_cjk(c: char) -> bool {
    matches!(c as u32,
        0x3400..=0x4DBF | 0x4E00..=0x9FFF | 0xF900..=0xFAFF |
        0x3040..=0x30FF | 0xAC00..=0xD7AF)
}

fn tokenize(chars: &[char]) -> Vec<(usize, usize)> {
    let mut i = 0;
    let mut out = Vec::new();
    while i < chars.len() {
        let c = chars[i];
        if is_cjk(c) {
            out.push((i, i + 1));
            i += 1;
        } else if c.is_ascii_alphabetic() {
            let s = i;
            i += 1;
            while i < chars.len() && chars[i].is_ascii_alphabetic() {
                i += 1;
            }
            out.push((s, i));
        } else {
            out.push((i, i + 1));
            i += 1;
        }
    }
    out
}

/// 入口别名（测试与 FUNCTION.md `greedy_wrap`）。
pub fn greedy_wrap(
    data: &TextBoxData,
    text_rect: RectF,
    advances: &[Vec<f32>],
) -> Vec<ShowTypoLine> {
    GreedyWrap::greedy_wrap(data, text_rect, advances)
}
