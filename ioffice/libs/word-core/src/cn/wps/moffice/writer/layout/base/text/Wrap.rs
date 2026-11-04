//! Greedy wrap against content width. Natural break at space; overlong word is forced.

#[derive(Clone, Debug)]
pub struct LayoutRun {
    pub text: String,
    pub width: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunSlice {
    pub run_index: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineBreak {
    pub slices: Vec<RunSlice>,
    pub width: i32,
}

fn char_width(run: &LayoutRun, ch: char) -> i32 {
    if ch == '\r' || ch == '\n' {
        return 0;
    }
    let n = run.text.chars().filter(|c| *c != '\r' && *c != '\n').count().max(1);
    (run.width / n as i32).max(0)
}

pub fn wrap(runs: &[LayoutRun], content_width: i32) -> Vec<LineBreak> {
    if runs.is_empty() {
        return vec![LineBreak {
            slices: vec![],
            width: 0,
        }];
    }
    let mut lines: Vec<LineBreak> = Vec::new();
    let mut cur_slices: Vec<RunSlice> = Vec::new();
    let mut cur_w = 0i32;

    fn flush(lines: &mut Vec<LineBreak>, slices: &mut Vec<RunSlice>, w: &mut i32) {
        let width = *w;
        lines.push(LineBreak {
            slices: std::mem::take(slices),
            width,
        });
        *w = 0;
    }

    for (ri, run) in runs.iter().enumerate() {
        let chars: Vec<char> = run.text.chars().collect();
        let mut i = 0usize;
        while i < chars.len() {
            let ch = chars[i];
            if ch == '\r' {
                if !cur_slices.is_empty() || cur_w > 0 {
                    flush(&mut lines, &mut cur_slices, &mut cur_w);
                } else {
                    flush(&mut lines, &mut cur_slices, &mut cur_w);
                }
                i += 1;
                continue;
            }
            let cw = char_width(run, ch);
            let is_space = ch == ' ';
            if !is_space && cur_w > 0 && cur_w + cw > content_width && content_width > 0 {
                // try wrap at last space in current line
                if let Some(sp) = last_space_break(&cur_slices, runs) {
                    split_flush_at_space(&mut lines, &mut cur_slices, &mut cur_w, runs, sp);
                } else {
                    flush(&mut lines, &mut cur_slices, &mut cur_w);
                }
            }
            if !is_space && cur_w == 0 && cw > content_width && content_width > 0 {
                // force one character
                cur_slices.push(RunSlice {
                    run_index: ri,
                    start: i,
                    end: i + 1,
                });
                cur_w = cw;
                flush(&mut lines, &mut cur_slices, &mut cur_w);
                i += 1;
                continue;
            }
            if cur_w + cw > content_width && content_width > 0 && cur_w > 0 && is_space {
                flush(&mut lines, &mut cur_slices, &mut cur_w);
                i += 1;
                continue;
            }
            push_char(&mut cur_slices, ri, i);
            cur_w += cw;
            i += 1;
        }
    }
    if !cur_slices.is_empty() || lines.is_empty() {
        trim_trailing_spaces(&mut cur_slices, runs, &mut cur_w);
        flush(&mut lines, &mut cur_slices, &mut cur_w);
    }
    lines
}

fn push_char(slices: &mut Vec<RunSlice>, ri: usize, i: usize) {
    if let Some(last) = slices.last_mut() {
        if last.run_index == ri && last.end == i {
            last.end = i + 1;
            return;
        }
    }
    slices.push(RunSlice {
        run_index: ri,
        start: i,
        end: i + 1,
    });
}

struct SpaceBreak {
    slice_i: usize,
    char_i: usize,
}

fn last_space_break(slices: &[RunSlice], runs: &[LayoutRun]) -> Option<SpaceBreak> {
    for (si, sl) in slices.iter().enumerate().rev() {
        let chars: Vec<char> = runs[sl.run_index].text.chars().collect();
        for ci in (sl.start..sl.end).rev() {
            if chars.get(ci) == Some(&' ') {
                return Some(SpaceBreak {
                    slice_i: si,
                    char_i: ci,
                });
            }
        }
    }
    None
}

fn split_flush_at_space(
    lines: &mut Vec<LineBreak>,
    slices: &mut Vec<RunSlice>,
    w: &mut i32,
    runs: &[LayoutRun],
    sp: SpaceBreak,
) {
    let mut head = slices[..sp.slice_i].to_vec();
    let sl = slices[sp.slice_i].clone();
    if sp.char_i > sl.start {
        head.push(RunSlice {
            run_index: sl.run_index,
            start: sl.start,
            end: sp.char_i,
        });
    }
    let mut tail: Vec<RunSlice> = Vec::new();
    if sp.char_i + 1 < sl.end {
        tail.push(RunSlice {
            run_index: sl.run_index,
            start: sp.char_i + 1,
            end: sl.end,
        });
    }
    tail.extend_from_slice(&slices[sp.slice_i + 1..]);
    let hw = width_of(&head, runs);
    lines.push(LineBreak {
        slices: head,
        width: hw,
    });
    *slices = tail;
    *w = width_of(slices, runs);
}

fn width_of(slices: &[RunSlice], runs: &[LayoutRun]) -> i32 {
    let mut w = 0;
    for sl in slices {
        let chars: Vec<char> = runs[sl.run_index].text.chars().collect();
        for i in sl.start..sl.end {
            if let Some(ch) = chars.get(i) {
                w += char_width(&runs[sl.run_index], *ch);
            }
        }
    }
    w
}

fn trim_trailing_spaces(slices: &mut Vec<RunSlice>, runs: &[LayoutRun], w: &mut i32) {
    while let Some(last) = slices.last_mut() {
        let chars: Vec<char> = runs[last.run_index].text.chars().collect();
        if last.end > last.start && chars.get(last.end - 1) == Some(&' ') {
            *w -= char_width(&runs[last.run_index], ' ');
            last.end -= 1;
            if last.end == last.start {
                slices.pop();
            }
        } else {
            break;
        }
    }
}
