use crate::cn::wps::moffice::writer::rendercommand::RenderCommand::RenderCommand;
use crate::cn::wps::moffice::writer::rendercommand::StyleRuns::{coalesce_style_runs, StyledRun};
use crate::cn::wps::moffice::writer::rendercommand::TextCommand::{emit_text, TextRunEmit};

#[derive(Clone, Debug)]
pub struct StyledLine {
    pub page: i32,
    pub x: i32,
    pub baseline: i32,
    pub runs: Vec<StyledRun>,
}

pub fn emit_styled_line(line: &StyledLine) -> Vec<RenderCommand> {
    let mut run_x = 0i32;
    coalesce_style_runs(&line.runs)
        .into_iter()
        .map(|run| {
            let command = emit_text(&TextRunEmit {
                page: line.page,
                line_top: line.baseline,
                align_x: line.x,
                run_x,
                text: run.text,
                font_id: run.font_id,
                size: run.size_twip,
                color: run.color,
                bold: run.bold,
                xpos: Vec::new(),
            });
            run_x = run_x.saturating_add(run.advance);
            command
        })
        .collect()
}
