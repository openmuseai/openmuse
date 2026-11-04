//! Sample CLI: docx → Stage3 envelope JSON (stdout or -o).
//!
//! Fonts: the layout stage measures glyph advances, so a font blob must be
//! supplied. Previously this CLI hard-coded `&[]`, which made every document
//! containing a glyph outside the built-in fallback fail with
//! `MissingGlyph` — i.e. the only end-to-end entry point could not actually
//! render a normal corpus document. Fonts are now resolved (in order) from
//! `--font <path>` (repeatable), the `DOCX_MVP_FONT` env var, or the shipped
//! `assets/MvpLatin.ttf`. `--no-fonts` restores the old behaviour.

use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    java_pipeline_pages, layout_document, open_text_document, FontBlob, LayoutError, ViewParams,
};
use word_core::cn::wps::moffice::writer::cache::Stage2Json::to_stage2_json;

const DEFAULT_FONT_REL: &str = "assets/MvpLatin.ttf";

fn render_envelope(bytes: &[u8], fonts: &[FontBlob]) -> Result<String, LayoutError> {
    let frame = layout_document(bytes, fonts, &ViewParams::default_a4())?;
    frame.to_envelope_json()
}

/// Diagnostic export from the same Java-shaped layout path used by layout_document.
fn render_stage2(bytes: &[u8], fonts: &[FontBlob]) -> Result<String, String> {
    let doc = open_text_document(bytes).map_err(|e| e.message)?;
    word_core::cn::wps::moffice::writer::layout::base::LayoutTraceCollector::LayoutTraceCollector::beginStage2Capture();
    let _pages = java_pipeline_pages(&doc, fonts, &ViewParams::default_a4())
        .map_err(|e| e.message)?;
    let rows = word_core::cn::wps::moffice::writer::layout::base::LayoutTraceCollector::LayoutTraceCollector::takeStage2Rows();
    to_stage2_json(&rows).map_err(|e| e.to_string())
}

/// Locate the shipped default font without relying on the process CWD.
fn default_font_path() -> Option<PathBuf> {
    // 1. Path baked in at compile time (works for `cargo run` / `cargo test`).
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join(DEFAULT_FONT_REL);
    if manifest.is_file() {
        return Some(manifest);
    }
    // 2. Relative to the executable (target/debug/<bin> → up to the crate root).
    if let Ok(exe) = env::current_exe() {
        for up in 0..4 {
            let mut p = exe.clone();
            for _ in 0..=up {
                p = match p.parent() {
                    Some(parent) => parent.to_path_buf(),
                    None => break,
                };
            }
            let cand = p.join(DEFAULT_FONT_REL);
            if cand.is_file() {
                return Some(cand);
            }
        }
    }
    // 3. Relative to the CWD.
    let cwd = PathBuf::from(DEFAULT_FONT_REL);
    if cwd.is_file() {
        return Some(cwd);
    }
    None
}

fn load_font(path: &Path, id: &str) -> Result<FontBlob, String> {
    match fs::read(path) {
        Ok(bytes) => Ok(FontBlob {
            id: id.to_string(),
            bytes,
        }),
        Err(e) => Err(format!("read font {}: {e}", path.display())),
    }
}

fn usage(bin: &str) {
    eprintln!(
        "Usage: {bin} <file.docx> [-o out.json] [--stage2] [--font <path.ttf>]... [--font-id <id=path>]... [--no-fonts]\n\
         \n\
         Renders a .docx to the Stage3 envelope JSON (stdout, or -o <file>).\n\
         --stage2       Export the product layout path's Stage2 rows instead.\n\
         \n\
         Fonts:\n\
           --font <path>   Add a font blob (repeatable). The first one is used as\n\
                           the document's default font id.\n\
           --font-id <id=path>  Add a named font blob, e.g. 宋体=/path/font.ttf.\n\
           --no-fonts      Pass no font blobs (old behaviour; documents whose\n\
                           glyphs are absent from the built-in fallback will fail\n\
                           with `MissingGlyph`).\n\
           If neither is given, DOCX_MVP_FONT or the shipped {DEFAULT_FONT_REL}\n\
           is loaded automatically when found.\n\
         \n\
         Example:\n\
           cargo run --manifest-path libs/word-core/Cargo.toml --bin docx_mvp_render -- \\\n\
             libs/word-core/tests/fixtures/p1_layout_document/hello-world.docx"
    );
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "-h" || a == "--help") {
        usage(&args[0]);
        return ExitCode::SUCCESS;
    }

    let mut out_path: Option<String> = None;
    let mut in_path: Option<String> = None;
    let mut font_paths: Vec<(Option<String>, String)> = Vec::new();
    let mut no_fonts = false;
    let mut stage2 = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-o" => {
                i += 1;
                out_path = args.get(i).cloned();
            }
            "--font" => {
                i += 1;
                match args.get(i) {
                    Some(p) => font_paths.push((None, p.clone())),
                    None => {
                        eprintln!("--font requires a path");
                        return ExitCode::from(2);
                    }
                }
            }
            "--font-id" => {
                i += 1;
                match args.get(i).and_then(|s| s.split_once('=')) {
                    Some((id, path)) if !id.is_empty() && !path.is_empty() => {
                        font_paths.push((Some(id.to_string()), path.to_string()));
                    }
                    _ => {
                        eprintln!("--font-id requires id=path");
                        return ExitCode::from(2);
                    }
                }
            }
            "--no-fonts" => no_fonts = true,
            "--stage2" => stage2 = true,
            other if !other.starts_with('-') => in_path = Some(other.to_string()),
            other => {
                eprintln!("unknown option: {other}");
                usage(&args[0]);
                return ExitCode::from(2);
            }
        }
        i += 1;
    }

    // Resolve font blobs: explicit flags > env var > shipped default.
    if font_paths.is_empty() && !no_fonts {
        if let Ok(p) = env::var("DOCX_MVP_FONT") {
            if !p.is_empty() {
                font_paths.push((None, p));
            }
        } else if let Some(p) = default_font_path() {
            font_paths.push((None, p.to_string_lossy().into_owned()));
        }
    }

    let mut fonts: Vec<FontBlob> = Vec::new();
    for (idx, (named_id, p)) in font_paths.iter().enumerate() {
        let id = named_id.as_deref().unwrap_or(if idx == 0 { "default" } else { "font" });
        match load_font(Path::new(p), id) {
            Ok(b) => fonts.push(b),
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::from(2);
            }
        }
    }
    if fonts.is_empty() && !no_fonts {
        eprintln!(
            "warning: no font blob loaded; run from the crate root or pass --font <path.ttf> \
             (glyphs outside the built-in fallback will report MissingGlyph)"
        );
    }

    let bytes = if let Some(p) = in_path {
        match fs::read(&p) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("read {p}: {e}");
                return ExitCode::from(2);
            }
        }
    } else {
        let mut buf = Vec::new();
        if let Err(e) = io::stdin().read_to_end(&mut buf) {
            eprintln!("stdin: {e}");
            return ExitCode::from(2);
        }
        buf
    };

    let rendered = if stage2 {
        render_stage2(&bytes, &fonts)
    } else {
        render_envelope(&bytes, &fonts).map_err(|e| e.message)
    };
    match rendered {
        Ok(json) => {
            if let Some(p) = out_path {
                if let Err(e) = fs::write(&p, json.as_bytes()) {
                    eprintln!("write {p}: {e}");
                    return ExitCode::from(2);
                }
            } else {
                println!("{json}");
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(1)
        }
    }
}
