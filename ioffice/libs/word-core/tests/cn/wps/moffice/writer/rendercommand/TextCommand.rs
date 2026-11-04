use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::{
    RenderCommand, STYLE_BOLD,
};
use word_core::cn::wps::moffice::writer::rendercommand::TextCommand::{emit_text, TextRunEmit};

fn run(page: i32, align: i32, run_x: i32, text: &str) -> TextRunEmit {
    TextRunEmit {
        page,
        line_top: 100,
        align_x: align,
        run_x,
        text: text.into(),
        font_id: "f".into(),
        size: 240,
        color: 0x112233,
        bold: false,
        xpos: Vec::new(),
    }
}

#[test]
fn t1_simple() {
    let RenderCommand::Text(t) = emit_text(&run(0, 0, 0, "Hi")) else {
        panic!("not text");
    };
    assert_eq!(t.x, 0);
    assert_eq!(t.y, 100);
    assert_eq!(t.text, "Hi");
}

#[test]
fn t2_align() {
    let RenderCommand::Text(t) = emit_text(&run(0, 50, 0, "Hi")) else {
        panic!();
    };
    assert_eq!(t.x, 50);
}

#[test]
fn t3_second_run_x() {
    let RenderCommand::Text(t) = emit_text(&run(0, 0, 80, "B")) else {
        panic!();
    };
    assert_eq!(t.x, 80);
}

#[test]
fn t4_style_color() {
    let mut r = run(0, 0, 0, "B");
    r.bold = true;
    r.color = 0xFF0000;
    let RenderCommand::Text(t) = emit_text(&r) else {
        panic!();
    };
    assert_eq!(t.style & STYLE_BOLD, STYLE_BOLD);
    assert_eq!(t.color, 0xFF0000);
}

#[test]
fn t5_page() {
    let RenderCommand::Text(t) = emit_text(&run(2, 0, 0, "P")) else {
        panic!();
    };
    assert_eq!(t.page, 2);
}
