// function_id=04_function_stage3_golden
// cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_stage3_golden
// UPDATE_GOLDENS=1 rewrites frozen Java-shaped Stage3 JSON (latin FT/hmtx path only).

use std::fs;
use std::path::PathBuf;
use word_core::cn::wps::moffice::writer::api::LayoutDocument::{
    layout_document, FontBlob, ViewParams,
};
use word_core::cn::wps::moffice::writer::rendercommand::Stage3Normalize::{
    canonical_hash, canonical_json, first_divergence, normalize_frame, NormalizedStage3,
};

fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn goldens() -> PathBuf {
    fixtures().join("p4_stage3")
}

fn mvp_font() -> FontBlob {
    FontBlob {
        id: "default".into(),
        bytes: fs::read(fixtures().join("p1_layout_document/MvpLatin.ttf")).expect("MvpLatin"),
    }
}

struct Case {
    id: &'static str,
    docx: PathBuf,
    view: ViewParams,
}

fn cases() -> Vec<Case> {
    let f = fixtures();
    let mut wrap = ViewParams::default_a4();
    wrap.content_width_twip = Some(800);
    vec![
        Case {
            id: "hello",
            docx: f.join("01_function_uniffi_export/plain-one-para.docx"),
            view: ViewParams::default_a4(),
        },
        Case {
            id: "two-para",
            docx: f.join("p1_layout_document/two-para.docx"),
            view: ViewParams::default_a4(),
        },
        Case {
            id: "hello-world-wrap",
            docx: f.join("p1_layout_document/hello-world.docx"),
            view: wrap,
        },
    ]
}

fn layout_case(c: &Case) -> NormalizedStage3 {
    let bytes = fs::read(&c.docx).unwrap_or_else(|_| panic!("missing {}", c.docx.display()));
    let frame = layout_document(&bytes, &[mvp_font()], &c.view).expect("layout");
    normalize_frame(&frame)
}

fn golden_path(id: &str) -> PathBuf {
    goldens().join(format!("{id}.stage3.json"))
}

fn java_path(id: &str) -> PathBuf {
    goldens().join(format!("java/{id}.stage3.json"))
}

fn load_expected(id: &str) -> NormalizedStage3 {
    let java = java_path(id);
    let path = if java.exists() {
        java
    } else {
        golden_path(id)
    };
    let s = fs::read_to_string(&path).unwrap_or_else(|_| {
        panic!(
            "missing golden {}. Run UPDATE_GOLDENS=1 cargo test --test 04_function_stage3_golden",
            path.display()
        )
    });
    serde_json::from_str(&s).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// G1 — ≥3 latin fixtures: Rust normalize equals frozen Stage3 (or Java dump if present).
#[test]
fn g1_three_latin_fixtures_match_goldens() {
    fs::create_dir_all(goldens()).unwrap();
    let update = std::env::var("UPDATE_GOLDENS").ok().as_deref() == Some("1");
    for c in cases() {
        let got = layout_case(&c);
        if update {
            let json = serde_json::to_string_pretty(&got).unwrap();
            fs::write(golden_path(c.id), json + "\n").unwrap();
            let h = canonical_hash(&got).unwrap();
            fs::write(goldens().join(format!("{}.hash", c.id)), h + "\n").unwrap();
            continue;
        }
        let expected = load_expected(c.id);
        if let Some(diff) = first_divergence(&got, &expected) {
            panic!("{}: {diff}", c.id);
        }
        assert_eq!(
            canonical_json(&got).unwrap(),
            canonical_json(&expected).unwrap(),
            "{}",
            c.id
        );
        let frozen_hash = fs::read_to_string(goldens().join(format!("{}.hash", c.id)))
            .unwrap()
            .trim()
            .to_string();
        assert_eq!(canonical_hash(&got).unwrap(), frozen_hash, "{} hash", c.id);
    }
    assert!(
        cases().len() >= 3,
        "G1 requires at least 3 latin fixtures"
    );
}

/// G1b — first-divergence helper names page/x/y/ch (CI output contract).
#[test]
fn g1b_divergence_mentions_xy() {
    let a = layout_case(&cases()[0]);
    let mut b = a.clone();
    match &mut b.commands[0] {
        word_core::cn::wps::moffice::writer::rendercommand::Stage3Normalize::NormalizedCmd::Glyph { x, .. } => {
            *x += 1;
        }
        _ => panic!("expected glyph"),
    }
    let msg = first_divergence(&a, &b).expect("diff");
    assert!(msg.contains("x="), "{msg}");
    assert!(msg.contains("page="), "{msg}");
}

/// Color 0 and 0xFF000000 collapse (Java/Flutter ink).
#[test]
fn g1c_ink_black_canonical() {
    use word_core::cn::wps::moffice::writer::rendercommand::RenderCommand::{
        DrawText, RenderCommand,
    };
    use word_core::cn::wps::moffice::writer::rendercommand::Stage3Normalize::normalize_commands;
    let t = DrawText {
        page: 0,
        x: 0,
        y: 0,
        text: "A".into(),
        font_id: "d".into(),
        size: 240,
        color: 0xFF000000,
        style: 0,
        xpos: vec![0],
    };
    let n = normalize_commands(2, &[RenderCommand::Text(t)]);
    match &n.commands[0] {
        word_core::cn::wps::moffice::writer::rendercommand::Stage3Normalize::NormalizedCmd::Glyph { color, ch, .. } => {
            assert_eq!(*color, 0);
            assert_eq!(ch, "A");
        }
        other => panic!("{other:?}"),
    }
}

#[derive(serde::Deserialize)]
struct PaintGap {
    id: String,
    #[serde(default)]
    silent_skip: bool,
    trigger: String,
    rust_behavior: String,
}

/// G2 — known Paint-gap list is explicit; CI must not silently skip those cases.
#[test]
fn g2_paint_gap_list_is_explicit() {
    let raw = fs::read_to_string(goldens().join("paint-gap.json")).expect("paint-gap.json");
    let gaps: Vec<PaintGap> = serde_json::from_str(&raw).expect("json");
    assert!(gaps.len() >= 3, "document at least 3 known gaps");
    for g in &gaps {
        assert!(!g.id.is_empty());
        assert!(
            !g.silent_skip,
            "{} must not be a silent skip — record the gap",
            g.id
        );
        assert!(!g.trigger.is_empty());
        assert!(!g.rust_behavior.is_empty());
    }
}
