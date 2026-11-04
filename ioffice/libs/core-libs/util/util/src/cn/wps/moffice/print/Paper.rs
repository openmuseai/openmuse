#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::sync::OnceLock;

static paper_A3: OnceLock<Paper> = OnceLock::new();
static paper_A4: OnceLock<Paper> = OnceLock::new();
static paper_A5: OnceLock<Paper> = OnceLock::new();
static paper_B4: OnceLock<Paper> = OnceLock::new();
static paper_B5: OnceLock<Paper> = OnceLock::new();

// Java: public class Paper
pub struct Paper {
    // Java field: float width
    pub(crate) width: f32,
    // Java field: float height
    pub(crate) height: f32,
    // Java field: Paper paper_A3
    // Java field: Paper paper_A4
    // Java field: Paper paper_A5
    // Java field: Paper paper_B4
    // Java field: Paper paper_B5
}

// Java: public class Paper
impl Paper {
    // Java: Paper(float w, float h)
    pub fn new(w: f32, h: f32) -> Self {
        // Java: super();
        // Java: this.width = w;
        // Java: this.height = h;
        Paper { width: w, height: h }
    }

    pub fn get_paper_a3() -> &'static Paper {
        paper_A3.get_or_init(|| Paper::new(841.85, 1190.55))
    }

    pub fn get_paper_a4() -> &'static Paper {
        paper_A4.get_or_init(|| Paper::new(595.28, 841.85))
    }

    pub fn get_paper_a5() -> &'static Paper {
        paper_A5.get_or_init(|| Paper::new(419.5, 595.28))
    }

    pub fn get_paper_b4() -> &'static Paper {
        paper_B4.get_or_init(|| Paper::new(728.51, 1031.82))
    }

    pub fn get_paper_b5() -> &'static Paper {
        paper_B5.get_or_init(|| Paper::new(515.89, 728.46))
    }
}
