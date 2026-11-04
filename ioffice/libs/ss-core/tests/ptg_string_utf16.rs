use ss_core::cn::wps::et::ss::formula::ptg::Ptg::{string_ptg, Ptg, PtgKind};
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

#[test]
fn java_string_ptg_uses_utf16_length_for_supplementary_character() {
    // java StringPtg.valueOf("😀"): String.length() is two UTF-16 code units.
    let token = string_ptg("😀".to_string());
    let mut encoded = Vec::new();
    token.write(&mut encoded);
    assert_eq!(token.getSize(), 7);
    assert_eq!(encoded, [23, 2, 1, 0x3d, 0xd8, 0x00, 0xde]);
    let decoded = Ptg::readTokens(encoded.len() as i32, &encoded, SpreadsheetVersion::EXCEL2007);
    assert!(matches!(&decoded[0].kind, PtgKind::Str(text) if text == "😀"));
}

#[test]
fn java_string_ptg_read_preserves_unpaired_surrogate_and_original_wide_flag() {
    for bytes in [
        vec![23, 1, 1, 0, 0xd8], // U+D800 by itself
        vec![23, 1, 1, 0x41, 0], // ASCII 'A' encoded with 16-bit flag
    ] {
        let decoded = Ptg::readTokens(bytes.len() as i32, &bytes, SpreadsheetVersion::EXCEL2007);
        assert_eq!(decoded.len(), 1);
        assert!(matches!(decoded[0].kind, PtgKind::StrRaw { wide: true, .. }));
        let mut encoded = Vec::new();
        decoded[0].write(&mut encoded);
        assert_eq!(decoded[0].getSize(), bytes.len() as i32);
        assert_eq!(encoded, bytes);
    }
}

#[test]
fn java_string_ptg_constructor_checks_utf16_units() {
    assert!(std::panic::catch_unwind(|| string_ptg("😀".repeat(128))).is_err());
}
