use ss_core::cn::wps::et::ss::formula::constant::ConstantValueParser::{ConstantValue, ConstantValueParser};

#[test]
fn java_array_constant_retains_unpaired_utf16_surrogates() {
    // java ConstantValueParser grbit 2 -> StringUtil.readUnicodeString:
    // one wide UTF-16 code unit U+D800 followed by one ordinary wide string.
    let bytes = [2, 1, 0, 1, 0x00, 0xd8, 2, 2, 0, 1, 0x2d, 0x4e, 0x00, 0xde];
    let mut offset = 0;
    let parsed = ConstantValueParser::parse(&bytes, &mut offset, 2).unwrap();
    assert_eq!(offset, bytes.len());
    assert_eq!(parsed[0], ConstantValue::StringUtf16(vec![0xd800]));
    assert_eq!(parsed[1], ConstantValue::StringUtf16(vec![0x4e2d, 0xde00]));
    assert_eq!(ConstantValueParser::getEncodedSize(&parsed), bytes.len());
    let mut encoded = Vec::new();
    ConstantValueParser::encode(&mut encoded, &parsed);
    assert_eq!(encoded, bytes);
}
