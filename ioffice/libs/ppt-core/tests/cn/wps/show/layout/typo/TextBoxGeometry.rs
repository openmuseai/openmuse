// Test 02_function_text_box_geometry — BaseTextLayout.setTextRect 内缩矩形（pl-05）
// 冻结矩阵 FUNCTION.md §3.2 TG1–TG8。期望按 BaseTextLayout.java:88-137 逐步手算。
use ppt_core::cn::wps::show::layout::typo::TextBoxGeometry::{text_box_geometry, RectF};
use wpslayout::cn::wps::text::layout::data::TextBoxData::TextBoxData;

fn insets(l: f32, t: f32, r: f32, b: f32) -> TextBoxData {
    let mut d = TextBoxData::new();
    d.setLeftInset(l);
    d.setTopInset(t);
    d.setRightInset(r);
    d.setBottomInset(b);
    d
}

fn assert_rect(got: RectF, left: f32, top: f32, right: f32, bottom: f32) {
    let eps = 0.01;
    assert!(
        (got.left - left).abs() < eps
            && (got.top - top).abs() < eps
            && (got.right - right).abs() < eps
            && (got.bottom - bottom).abs() < eps,
        "got=({:?}) expected=({}, {}, {}, {})",
        (got.left, got.top, got.right, got.bottom),
        left,
        top,
        right,
        bottom
    );
    assert!(got.left <= got.right + 1e-5);
    assert!(got.top <= got.bottom + 1e-5);
}

#[test]
fn tg1_base_insets() {
    // left=100, right=1000-100=900, top=50, bottom=500-50=450
    let r = text_box_geometry(&insets(100.0, 50.0, 100.0, 50.0), 1000.0, 500.0);
    assert_rect(r, 100.0, 50.0, 900.0, 450.0);
}

#[test]
fn tg2_left_gt_width_atten() {
    // left=1200 > 1000 → right=1200; right-=0; left>width atten=(1200-1000)/2=100 → (1100,0,1100,500)
    let r = text_box_geometry(&insets(1200.0, 0.0, 0.0, 0.0), 1000.0, 500.0);
    assert_rect(r, 1100.0, 0.0, 1100.0, 500.0);
}

#[test]
fn tg3_right_lt_left_then_right_lt_zero_atten() {
    // left=0; right=1000-1200=-200; right<left → left=-200;
    // right<0 atten=100 → (-100, 0, -100, 500)
    let r = text_box_geometry(&insets(0.0, 0.0, 1200.0, 0.0), 1000.0, 500.0);
    assert_rect(r, -100.0, 0.0, -100.0, 500.0);
}

#[test]
fn tg4_top_gt_height_atten() {
    // top=600 > 500 → bottom=600; bottom-=0; top>height atten=50 → (0,550,1000,550)
    let r = text_box_geometry(&insets(0.0, 600.0, 0.0, 0.0), 1000.0, 500.0);
    assert_rect(r, 0.0, 550.0, 1000.0, 550.0);
}

#[test]
fn tg5_bottom_lt_top_then_bottom_lt_zero() {
    // bottom=500-700=-200; bottom<top → top=-200; bottom<0 atten=100 → (0,-100,1000,-100)
    let r = text_box_geometry(&insets(0.0, 0.0, 0.0, 700.0), 1000.0, 500.0);
    assert_rect(r, 0.0, -100.0, 1000.0, -100.0);
}

#[test]
fn tg6_right_clamp_no_negative_width() {
    // left=500; right=1000-900=100; right<left → left=100; 1000×400 → (100,0,100,400)
    let r = text_box_geometry(&insets(500.0, 0.0, 900.0, 0.0), 1000.0, 400.0);
    assert_rect(r, 100.0, 0.0, 100.0, 400.0);
}

#[test]
fn tg7_left_and_top_overflow_order() {
    // left=1000 on w=800: left=1000>800 → right=1000; right-=0=1000; left>width atten=100 → left=right=900
    // top=700 on h=600: top=700>600 → bottom=700; top>height atten=50 → top=bottom=650
    let r = text_box_geometry(&insets(1000.0, 700.0, 0.0, 0.0), 800.0, 600.0);
    assert_rect(r, 900.0, 650.0, 900.0, 650.0);
}

#[test]
fn tg8_zero_and_negative_box() {
    let z = text_box_geometry(&insets(0.0, 0.0, 0.0, 0.0), 0.0, 500.0);
    assert_rect(z, 0.0, 0.0, 0.0, 500.0);
    // width=-10: left=0>-10 → right=0; then left>width atten=5 → (-5,-5);
    // then right<0 atten=2.5 → (-2.5, 0, -2.5, 500)（Java 四个 if 顺序执行，非 else-if）
    let n = text_box_geometry(&insets(0.0, 0.0, 0.0, 0.0), -10.0, 500.0);
    assert_rect(n, -2.5, 0.0, -2.5, 500.0);
}
