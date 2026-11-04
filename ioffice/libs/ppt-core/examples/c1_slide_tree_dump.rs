//! C1（`samples/ppt-render/flutter/assets/Sample_ppt.pptx`）形状树逐页逐类型 dump。
//!
//! 走**产品真实入口** `PptxrSlides::open → SlideHandler → LayoutCommonSlideData → LayoutSwGroup`，
//! 输出每页 `kmo_sp_root` 的 depth1 逐类型（sp/pic/grpSp/graphicFrame/cxnSp）、depth1 总数、
//! recursive 总数、recursive 逐类型、`p:cNvPr` 计数与 `p:ph` 计数。
//!
//! 由 `rewrite-review/scripts/oracle_diff_c1.sh` 消费，与
//! `rewrite-review/milestone-PptxrSlideAssembly/corpus/c1_slides_oracle.json` 逐页逐类型对比
//! （A13 回归门闩）。`03_function_pres_open::g02` 只断言总数，逐类型的偏差会被总数掩盖
//! （§18.2 的「slide8 16 grpSp」即此类误判，§20 已用本 dump 的证据推翻）。
//! 直接 `cargo run --example c1_slide_tree_dump` 可看原始表格。

use std::cell::RefCell;
use std::rc::Rc;

use ppt_core::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use ppt_core::cn::wps::show::pptxr::PptxReader::PptxReader;
use ppt_core::cn::wps::show::util::layout_shape::KmoPresentation;

const C1: &str = "../../samples/ppt-render/flutter/assets/Sample_ppt.pptx";

fn count_nodes(root: &KmoShowShape) -> usize {
    (0..root.childCount())
        .map(|i| 1 + count_nodes(root.childAt(i)))
        .sum::<usize>()
        + 0
}

fn recount(root: &KmoShowShape, counts: &mut [usize; 5]) {
    for c in 0..root.childCount() {
        let child = root.childAt(c);
        match child.r#type() {
            0 => counts[2] += 1,
            1 => counts[0] += 1,
            2 => counts[1] += 1,
            3 => counts[4] += 1,
            4 => counts[3] += 1,
            _ => {}
        }
        recount(child, counts);
    }
}

fn count_ph(root: &KmoShowShape) -> usize {
    (0..root.childCount())
        .map(|i| {
            let child = root.childAt(i);
            usize::from(child.isPlaceHolder()) + count_ph(child)
        })
        .sum()
}

fn main() {
    let model = Rc::new(RefCell::new(KmoPresentation::default()));
    let mut reader = PptxReader::PptxReader(Some(model.clone()), C1, None, false, None);
    reader.open().expect("PptxReader.open");
    let p = model.borrow();
    println!("slides={}", p.slides.len());
    println!(
        "{:>5} {:>4} {:>4} {:>5} {:>5} {:>5} | {:>4} {:>4} {:>6}",
        "slide", "sp", "pic", "grpSp", "gFrm", "cxnSp", "d1tot", "rec", "cnvpr"
    );
    let mut tally = [0usize; 5];
    for (i, slide) in p.slides.iter().enumerate() {
        let Some(root) = slide.kmo_sp_root.as_ref() else {
            println!("slide{}: NO kmo_sp_root", i + 1);
            continue;
        };
        let mut counts = [0usize; 5];
        for c in 0..root.childCount() {
            match root.childAt(c).r#type() {
                0 => counts[2] += 1,
                1 => counts[0] += 1,
                2 => counts[1] += 1,
                3 => counts[4] += 1,
                4 => counts[3] += 1,
                _ => {}
            }
        }
        for k in 0..5 {
            tally[k] += counts[k];
        }
        let mut rec_counts = [0usize; 5];
        recount(root, &mut rec_counts);
        println!(
            "{:>5} {:>4} {:>4} {:>5} {:>5} {:>5} | {:>4} {:>4} {:>6}  ph={} | rec {:?}",
            i + 1,
            counts[0],
            counts[1],
            counts[2],
            counts[3],
            counts[4],
            root.childCount(),
            count_nodes(root),
            slide.cnvpr_count,
            count_ph(root),
            rec_counts,
        );
    }
    println!(
        "TALLY sp={} pic={} grpSp={} graphicFrame={} cxnSp={}",
        tally[0], tally[1], tally[2], tally[3], tally[4]
    );
}
