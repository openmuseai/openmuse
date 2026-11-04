use word_core::cn::wps::moffice::writer::rendercommand::DecoCommand::{
    emit_deco, DecoStyle, UnderlineKind,
};
use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::RenderCommand;

fn base() -> DecoStyle {
    DecoStyle {
        page: 0,
        x: 10,
        y: 20,
        w: 100,
        h: 240,
        baseline: 200,
        line_color: 1,
        ..Default::default()
    }
}

#[test]
fn d1_underline() {
    let mut s = base();
    s.underline = UnderlineKind::Single;
    let cmds = emit_deco(&s);
    assert_eq!(cmds.len(), 1);
    assert!(matches!(cmds[0], RenderCommand::Line(_)));
}

#[test]
fn d2_double() {
    let mut s = base();
    s.underline = UnderlineKind::Double;
    assert_eq!(emit_deco(&s).len(), 2);
}

#[test]
fn d3_wave() {
    let mut s = base();
    s.underline = UnderlineKind::Wave;
    let RenderCommand::Line(l) = &emit_deco(&s)[0] else {
        panic!();
    };
    assert_eq!(l.kind, "wave");
}

#[test]
fn d4_bg() {
    let mut s = base();
    s.bg_color = Some(0xABCDEF);
    let cmds = emit_deco(&s);
    assert_eq!(cmds.len(), 1);
    assert!(matches!(cmds[0], RenderCommand::Rect(_)));
}

#[test]
fn d5_none() {
    assert!(emit_deco(&base()).is_empty());
}
