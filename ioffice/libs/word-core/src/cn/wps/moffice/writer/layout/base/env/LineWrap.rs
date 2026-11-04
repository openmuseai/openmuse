#![allow(non_snake_case)]

/// wr-12 I/O: wr-11 per-cp advances + 01 max_width → line cp ranges + width.
///
/// Java twin is `SubLineLayouter.addRun` overflow then
/// `SubLineLayouterBase.doAddExcessGraphs` / `rollback` (WestLayout space
/// rollback). Do **not** call wr-04 `layout/base/text/Wrap.rs`.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineRange {
    pub start_cp: i32,
    pub end_cp: i32,
    pub width: i32,
}

pub struct LineWrap {}

impl LineWrap {
    fn width_of(advances: &[i32], start: usize, end: usize) -> i32 {
        advances
            .get(start..end)
            .map(|s| s.iter().copied().sum())
            .unwrap_or(0)
    }

    // java if (run.getRightWithBdr() - this.mMaxWidth >= 2) { continueAddRun = false; }
    // java continueAddRun = this.doAddExcessGraphs(..., run, this.mMaxWidth, layoutState);
    // java langLayout.rollBackForPartialWord(...); // space stays on the current line
    pub fn wrap(advances: &[i32], chars: &str, max_width: i32) -> Vec<LineRange> {
        let chs: Vec<char> = chars.chars().collect();
        let n = chs.len();
        if n == 0 {
            return Vec::new();
        }

        let mut lines: Vec<LineRange> = Vec::new();
        let mut line_start: usize = 0;
        let mut accum: i32 = 0;
        let mut last_break_after: usize = 0;
        let mut i: usize = 0;

        while i < n {
            let ch = chs[i];
            let adv = advances.get(i).copied().unwrap_or(0);

            // java if (TypoRun.isLineBreak(lastGraphsPos.getGraphs().runType)) { return false; }
            if ch == '\r' {
                accum += adv;
                lines.push(LineRange {
                    start_cp: line_start as i32,
                    end_cp: (i + 1) as i32,
                    width: accum,
                });
                return lines;
            }

            let overflow = accum + adv - max_width >= 2;
            // java TypoRun.isSpace: runType == 2 || 17 — U+0020 only here (NBSP is type 27)
            if overflow && ch != ' ' {
                if last_break_after > line_start {
                    lines.push(LineRange {
                        start_cp: line_start as i32,
                        end_cp: last_break_after as i32,
                        width: Self::width_of(advances, line_start, last_break_after),
                    });
                    line_start = last_break_after;
                    accum = Self::width_of(advances, line_start, i);
                    last_break_after = line_start;
                    continue;
                }
                if i > line_start {
                    lines.push(LineRange {
                        start_cp: line_start as i32,
                        end_cp: i as i32,
                        width: accum,
                    });
                    line_start = i;
                    accum = 0;
                    last_break_after = line_start;
                    continue;
                }
                // java if (subLineEnv.atLeastOneChar) keep the overflowing first graph
                accum += adv;
                i += 1;
                if i < n && accum - max_width >= 2 {
                    lines.push(LineRange {
                        start_cp: line_start as i32,
                        end_cp: i as i32,
                        width: accum,
                    });
                    line_start = i;
                    accum = 0;
                    last_break_after = line_start;
                }
                continue;
            }

            accum += adv;
            if ch == ' ' {
                last_break_after = i + 1;
            }
            i += 1;
        }

        if line_start < n {
            lines.push(LineRange {
                start_cp: line_start as i32,
                end_cp: n as i32,
                width: accum,
            });
        }
        lines
    }
}
