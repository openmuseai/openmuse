//! Line height from font size (pt) and PAPX space before/after (twip).

#[derive(Clone, Debug, Default)]
pub struct ParaLineMetrics {
    pub font_size_pt: f64,
    pub line_spacing: f64,
    pub space_before_twip: i32,
    pub space_after_twip: i32,
}

impl ParaLineMetrics {
    pub fn default_para() -> Self {
        ParaLineMetrics {
            font_size_pt: 12.0,
            line_spacing: 1.0,
            space_before_twip: 0,
            space_after_twip: 0,
        }
    }
}

/// `ceil(ceil(pt × 20) × lineSpacing)`
pub fn line_height(para: &ParaLineMetrics) -> i32 {
    let spacing = if para.line_spacing <= 0.0 {
        1.0
    } else {
        para.line_spacing
    };
    let grid = (para.font_size_pt * 20.0).ceil();
    (grid * spacing).ceil() as i32
}

pub fn first_line_top(para: &ParaLineMetrics) -> i32 {
    para.space_before_twip
}

pub fn last_line_after(para: &ParaLineMetrics) -> i32 {
    para.space_after_twip
}
