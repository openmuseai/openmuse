// function_id=20_function_zip_opf_name
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 20_function_zip_opf_name

use core_libs_io::io::opc::internal::ZipHelper::ZipHelper;
use core_libs_io::io::xwpf::util::ZipInputStreamZipEntrySource::ZipInputStreamZipEntrySource;
use std::io::Cursor;
use std::path::PathBuf;

fn opc(name: &str) -> String {
    ZipHelper::getOPCNameFromZipItemName(name)
}

const SAMPLE: &[&str] = &[
    "word/document.xml",
    "/word/document.xml",
    "[Content_Types].xml",
    "_rels/.rels",
    "",
    "a",
    "/",
    "//word/x.xml",
    r"word\document.xml",
    "word/",
    "word/../evil.xml",
    "文档/正文.xml",
    "e\u{301}",
    "\u{0}ab",
    "WORD/DOC.XML",
    "word/_rels/document.xml.rels",
    "docProps/core.xml",
    "a..b/c.xml",
    "word/..",
    r"..\evil.xml",
];

#[test]
fn o_relative_name_gets_leading_slash() {
    assert_eq!(opc("word/document.xml"), "/word/document.xml");
}

#[test]
fn o_absolute_name_unchanged() {
    assert_eq!(opc("/word/document.xml"), "/word/document.xml");
}

#[test]
fn o_content_types_name_prefixed() {
    assert_eq!(opc("[Content_Types].xml"), "/[Content_Types].xml");
}

#[test]
fn o_rels_name_prefixed() {
    assert_eq!(opc("_rels/.rels"), "/_rels/.rels");
}

#[test]
fn o_empty_name_becomes_root_slash() {
    assert_eq!(opc(""), "/");
}

#[test]
fn o_single_char_prefixed() {
    assert_eq!(opc("a"), "/a");
}

#[test]
fn o_root_slash_unchanged() {
    assert_eq!(opc("/"), "/");
}

#[test]
fn o_double_leading_slash_not_collapsed() {
    assert_eq!(opc("//word/x.xml"), "//word/x.xml");
}

#[test]
fn o_backslash_is_not_replaced() {
    assert_eq!(opc(r"word\document.xml"), r"/word\document.xml");
}

#[test]
fn o_trailing_slash_not_rejected() {
    assert_eq!(opc("word/"), "/word/");
}

#[test]
fn o_dotdot_is_not_checked_here() {
    assert_eq!(opc("word/../evil.xml"), "/word/../evil.xml");
}

#[test]
fn o_non_ascii_name_prefixed() {
    assert_eq!(opc("文档/正文.xml"), "/文档/正文.xml");
}

#[test]
fn o_unicode_not_normalized() {
    assert_eq!(opc("e\u{301}"), "/e\u{301}");
}

#[test]
fn o_nul_byte_preserved() {
    assert_eq!(opc("\u{0}ab"), "/\u{0}ab");
}

#[test]
fn o_case_is_preserved() {
    assert_eq!(opc("WORD/DOC.XML"), "/WORD/DOC.XML");
}

#[test]
fn o_output_is_input_or_slash_prefixed() {
    for name in SAMPLE {
        let out = opc(name);
        assert!(
            out == *name || out.get(1..) == Some(*name),
            "unexpected transform {name:?} -> {out:?}"
        );
    }
}

#[test]
fn o_output_always_starts_with_slash() {
    for name in SAMPLE {
        assert!(opc(name).starts_with('/'), "{name:?}");
    }
}

#[test]
fn o_output_length_delta_at_most_one() {
    for name in SAMPLE {
        let out = opc(name);
        let delta = out.len() as isize - name.len() as isize;
        assert!(delta == 0 || delta == 1, "{name:?} -> {out:?}");
    }
}

#[test]
fn o_application_is_idempotent() {
    for name in SAMPLE {
        let once = opc(name);
        assert_eq!(opc(&once), once, "{name:?}");
    }
}

#[test]
fn o_output_is_deterministic() {
    for _ in 0..100 {
        assert_eq!(opc("word/document.xml"), "/word/document.xml");
    }
}

#[test]
fn o_function_is_stateless() {
    for name in SAMPLE {
        let a = opc(name);
        let _ = opc("[Content_Types].xml");
        let b = opc(name);
        assert_eq!(a, b, "{name:?}");
    }
}

#[test]
fn c1_all_entry_names_normalize() {
    let bytes = std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx"),
    )
    .unwrap();
    let mut input = Cursor::new(bytes);
    let source = ZipInputStreamZipEntrySource::try_new(&mut input).unwrap();
    for entry in source.getEntries().as_ref().unwrap() {
        let name = entry.getName();
        let out = opc(name);
        assert!(out.starts_with('/'), "{name}");
        if name.starts_with('/') {
            assert_eq!(out, name);
        } else {
            assert_eq!(out, format!("/{name}"));
        }
    }
}

#[test]
fn o_reverse_handles_empty_without_panic() {
    assert_eq!(ZipHelper::getZipItemNameFromOPCName(""), "");
}
