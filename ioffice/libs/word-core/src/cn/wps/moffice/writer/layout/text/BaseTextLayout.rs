//! Line geometry contract extracted from Java `TypoLayoutGen` / `Layout`
//! (`getLineCount/Width/Top/Bottom/Baseline/Start/End`). Body layout does
//! **not** call the wpslayout shape/textbox typo engine. wr-04 fills these
//! fields; wr-07 consumes them.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LineMetrics {
    pub start_cp: i32,
    pub end_cp: i32,
    pub x_off: i32,
    pub top: i32,
    pub bottom: i32,
    pub baseline: i32,
    pub width: i32,
}

impl LineMetrics {
    pub fn height(&self) -> i32 {
        self.bottom.saturating_sub(self.top)
    }
}
