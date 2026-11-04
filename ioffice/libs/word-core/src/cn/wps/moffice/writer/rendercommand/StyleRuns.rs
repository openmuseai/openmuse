#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StyledRun {
    pub text: String,
    pub font_id: String,
    pub size_twip: i32,
    pub bold: bool,
    pub color: u32,
    /// Total measured advance, supplied by the same FontHost used by wr-11.
    pub advance: i32,
}

impl StyledRun {
    fn same_style(&self, other: &Self) -> bool {
        self.font_id == other.font_id
            && self.size_twip == other.size_twip
            && self.bold == other.bold
            && self.color == other.color
    }
}

pub fn coalesce_style_runs(runs: &[StyledRun]) -> Vec<StyledRun> {
    let mut out: Vec<StyledRun> = Vec::new();
    for run in runs {
        if run.text.is_empty() {
            continue;
        }
        if let Some(last) = out.last_mut() {
            if last.same_style(run) {
                last.text.push_str(&run.text);
                last.advance = last.advance.saturating_add(run.advance);
                continue;
            }
        }
        out.push(run.clone());
    }
    out
}
