//! R15 页面差异诊断工具（PLAN §35.4 的未闭合项）。
//!
//! 用途：把 C1 指定页的 `RenderCommand` 序列 dump 成**逐条一行**的文本，
//! 供 `diff` 定位「条数相同、指纹不同」的真实字段。
//!
//! 用法：
//! ```bash
//! cargo run --example r15_page_diff -- 3            # dump slide3（1-based）
//! cargo run --example r15_page_diff -- 3 > /tmp/a.txt
//! cargo run --example r15_page_diff -- 9 11 12 15 19
//! ```
//!
//! 每条命令输出为 `serde_json` 单行（字段序稳定），故 `diff` 可直接定位差异字段。

use ppt_core::cn::wps::show::render::mvp_pipeline::render_pptx_pages;
use ppt_core::cn::wps::show::render::rendercommand::RenderCommand;

const C1: &str = "../../samples/ppt-render/flutter/assets/Sample_ppt.pptx";

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// 命令类型名（`RenderCommand` 的 variant 名），用于折叠大载荷时保留可比性。
fn cmd_type(c: &RenderCommand) -> &'static str {
    match c {
        RenderCommand::DrawRect { .. } => "DrawRect",
        RenderCommand::DrawPath { .. } => "DrawPath",
        RenderCommand::DrawText { .. } => "DrawText",
        RenderCommand::DrawImage { .. } => "DrawImage",
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let bytes = std::fs::read(C1).expect("Sample_ppt.pptx");
    let pages = render_pptx_pages(&bytes).expect("render_pptx_pages(C1)");
    let targets: Vec<usize> = if args.is_empty() {
        (1..=pages.len()).collect()
    } else {
        args.iter()
            .map(|a| a.parse::<usize>().expect("page number"))
            .collect()
    };
    for p in targets {
        let idx = p - 1;
        let cmds = &pages[idx];
        println!("### slide{p}  len={}", cmds.len());
        for (i, c) in cmds.iter().enumerate() {
            let mut s = serde_json::to_string(c).expect("serialize");
            // 图片 base64 载荷会淹没 diff：折叠成哈希（等价可比）。
            if s.len() > 400 {
                let fold_hash = fnv1a64(s.as_bytes());
                let fold_len = s.len();
                s = format!(
                    "{{\"type\":\"{}\",\"__folded_len\":{},\"__fold_hash\":\"{:016x}\"}}",
                    cmd_type(c),
                    fold_len,
                    fold_hash
                );
            }
            println!("{p:>3}.{i:<4} {s}");
        }
    }
}
