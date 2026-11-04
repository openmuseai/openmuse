//! `57_function_parity_rust_dump` 的 CLI 入口（工装面）。
//!
//! 用法：
//! ```text
//! s01_package_dump <input.docx> [--out out.json]
//! ```
//!
//! 契约（57/FUNCTION.md §1.5）：
//! - **错误不抛穿** ⇒ 进程退出码为 **0**（失败也产出一份合法 JSON，`apiTrace.error` 有值）
//! - 输出为 canonical JSON（`schema-v1.json` 的 `canonical_rules`）
//! - 仅工装面；**不改**产品链（`api/DocumentOpen.rs`）

use core_libs_io::io::opc::Dump::dump_package;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut input: Option<String> = None;
    let mut out: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" | "-o" => {
                i += 1;
                out = args.get(i).cloned();
            }
            other => {
                if input.is_none() {
                    input = Some(other.to_string());
                }
            }
        }
        i += 1;
    }

    let Some(input) = input else {
        // 用法错误 ⇒ 退出码 2（契约：0=成功、1=比较不等、2=用法错误）
        eprintln!("usage: s01_package_dump <input.docx> [--out out.json]");
        std::process::exit(2);
    };

    let bytes = match std::fs::read(&input) {
        Ok(b) => b,
        Err(e) => {
            // 读不到文件也**不抛穿**：产出失败形态 JSON，退出码 0
            let detail = format!("read {}: {}", input, e);
            let json = format!(
                "{{\"schema\":\"s01.opc.v1\",\"apiTrace\":{{\"open_entry\":\"bytes\",\"part_count\":0,\"relationship_count\":0,\"main_part\":null,\"error\":{{\"kind\":\"io_error\",\"at\":\"bin::s01_package_dump#read\",\"detail\":{}}}}}\"}}",
                serde_json::Value::String(detail)
            );
            emit(&json, out.as_deref());
            return;
        }
    };

    let outcome = dump_package(&bytes);
    emit(&outcome.json, out.as_deref());
    // 契约：dump 自身恒成功（失败信息在 JSON 里）⇒ 退出码 0
}

fn emit(json: &str, out: Option<&str>) {
    match out {
        Some(path) => {
            let _ = std::fs::write(path, json.as_bytes());
            println!("{}", path);
        }
        None => {
            let mut so = std::io::stdout();
            let _ = so.write_all(json.as_bytes());
            let _ = so.write_all(b"\n");
        }
    }
}
