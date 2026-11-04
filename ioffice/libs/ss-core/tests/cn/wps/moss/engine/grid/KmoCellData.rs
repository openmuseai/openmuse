use std::panic::{catch_unwind, AssertUnwindSafe};

use ss_core::cn::wps::moss::engine::grid::KmoCellData::KmoCellData;
use ss_core::cn::wps::moss::util::KmoBitField::KmoBitField;
use ss_core::cn::wps::moss::util::KmoConstraintCheck::KmoConstraintCheck;
use ss_core::org::apache::poi::ss::SpreadsheetVersion::{CUR_VERSION, SpreadsheetVersion};

fn blank7() -> [u8; 7] {
    [0; 7]
}

#[test]
fn cd1_id_roundtrip_including_sign_bit() {
    let mut buff = blank7();
    KmoCellData::setId(&mut buff, 0x01020304);
    assert_eq!(KmoCellData::getId(&buff), 0x01020304);
    KmoCellData::setId(&mut buff, i32::MIN);
    assert_eq!(buff[6], 0x80);
    assert_eq!(KmoCellData::getId(&buff), i32::MIN);
    KmoCellData::setId(&mut buff, -1);
    assert_eq!(buff[3..7], [0xff, 0xff, 0xff, 0xff]);
    assert_eq!(KmoCellData::getId(&buff), -1);
}

#[test]
fn cd2_ixf_keeps_unsigned_16_bits() {
    let mut buff = blank7();
    KmoCellData::setIXF(&mut buff, 0x1abcd);
    assert_eq!(KmoCellData::getIXF(&buff), 0xabcd);
    KmoCellData::setIXF(&mut buff, 0xffff);
    assert_eq!(KmoCellData::getIXF(&buff), 65535);
}

#[test]
fn cd3_set_type_writes_whole_byte_get_type_masks_low_two_bits() {
    let mut buff = blank7();
    buff[2] = 0b0001_1000;
    KmoCellData::setType(&mut buff, 0x1f);
    assert_eq!(buff[2], 0x1f);
    assert_eq!(KmoCellData::getType(&buff), 3);
}

#[test]
fn cd4_write_read_preserves_flags_and_masks_formula_type() {
    let mut cell = KmoCellData::default();
    cell.ixf = 0x1234;
    cell.formulaType = 5;
    cell.hasComment = true;
    cell.isPicInCell = true;
    cell.id = 0x89abcdef_u32 as i32;
    let mut buff = blank7();
    cell.write(&mut buff);
    assert_eq!(KmoCellData::getType(&buff), 1);
    let mut back = KmoCellData::default();
    back.read__1(&mut buff);
    assert_eq!(back.ixf, 0x1234);
    assert_eq!(back.formulaType, 1);
    assert!(back.hasComment);
    assert!(back.isPicInCell);
    assert_eq!(back.id, 0x89abcdef_u32 as i32);
}

#[test]
fn cd5_read_with_offset_and_null_blank_clear() {
    let mut raw = [0u8; 10];
    let cell = KmoCellData {
        ixf: 4,
        formulaType: 1,
        hasComment: true,
        isPicInCell: false,
        id: 9,
    };
    let encoded = cell.write(&mut raw[2..9]);
    assert_eq!(encoded.len(), 7);
    let mut decoded = KmoCellData::default();
    decoded.read__bytearr_int(&raw, 2);
    assert_eq!(decoded.ixf, 4);
    assert_eq!(decoded.formulaType, 1);
    assert!(decoded.hasComment);
    assert!(!decoded.isPicInCell);
    assert_eq!(decoded.id, 9);
    assert!(!decoded.isNull());
    assert!(!decoded.isBlank());
    decoded.clear();
    assert!(decoded.isNull());
    decoded.ixf = 15;
    assert!(decoded.isBlank());
}

#[test]
fn cd6_equals_and_hash_ignore_comment_only_in_hash() {
    let a = KmoCellData {
        ixf: 3,
        formulaType: 1,
        hasComment: false,
        isPicInCell: false,
        id: 2,
    };
    let b = KmoCellData {
        hasComment: true,
        ..a
    };
    assert!(a.equals(Some(&a)));
    assert!(!a.equals(None));
    assert!(!a.equals(Some(&b)));
    assert_eq!(a.hashCode(), b.hashCode());
    assert_eq!(a.hashCode(), 507473);
    let wide = KmoCellData {
        id: i32::MIN,
        ..KmoCellData::default()
    };
    let _ = wide.hashCode();
}

#[test]
fn cd7_write_rejects_length_other_than_7() {
    let cell = KmoCellData::default();
    let short_buf = [0u8; 6];
    let caught = catch_unwind(AssertUnwindSafe(|| {
        let mut buf = short_buf;
        cell.write(&mut buf);
    }));
    assert!(caught.is_err());
    let mut long_buf = [0u8; 8];
    let caught = catch_unwind(AssertUnwindSafe(|| {
        cell.write(&mut long_buf);
    }));
    assert!(caught.is_err());
    let _ = short_buf;
}

#[test]
fn cd8_short_buffer_panics_like_java_index() {
    let caught = catch_unwind(|| KmoCellData::getId(&[0, 1, 2]));
    assert!(caught.is_err());
}

#[test]
fn bf1_masks_match_cell_layout_constants() {
    let formula = KmoBitField::KmoBitField(3);
    let comment = KmoBitField::KmoBitField(8);
    let pic = KmoBitField::KmoBitField(16);
    assert_eq!(formula.getValue(0x1f), 3);
    assert_eq!(formula.setValue(0, 5), 1);
    assert_eq!(comment.getValue(0b0001_1000), 1);
    assert!(comment.isSet(8));
    assert!(!comment.isAllSet(1));
    assert!(pic.isAllSet(16));
    assert_eq!(comment.set(0), 8);
    assert_eq!(comment.clear(0xff), 0xff & !8);
    assert_eq!(formula.setBoolean(0, true), 3);
    assert_eq!(formula.setBoolean(7, false), 4);
    assert_eq!(KmoBitField::KmoBitField(0).getValue(0xff), 0);
}

#[test]
fn bf2_short_and_byte_truncate() {
    let field = KmoBitField::KmoBitField(0xff);
    assert_eq!(field.setByte(0), -1);
    assert_eq!(field.setByte(-1), -1i8);
    assert_eq!(field.clearByte(-1), 0);
    assert_eq!(field.getShortValue(0x0100), 0);
    assert_eq!(field.setShortValue(0, 0x1ff), 0xff);
}

#[test]
fn sv1_version_limits_and_default_current() {
    assert_eq!(SpreadsheetVersion::EXCEL97.getMaxRows(), 65536);
    assert_eq!(SpreadsheetVersion::EXCEL97.getLastRowIndex(), 65535);
    assert_eq!(SpreadsheetVersion::EXCEL97.getMaxColumns(), 256);
    assert_eq!(SpreadsheetVersion::EXCEL97.getLastColumnIndex(), 255);
    assert_eq!(SpreadsheetVersion::EXCEL97.getMaxFunctionArgs(), 30);
    assert_eq!(SpreadsheetVersion::EXCEL97.getMaxConditionalFormats(), 3);
    assert_eq!(SpreadsheetVersion::EXCEL97.getMaxTextLength(), 32767);
    assert_eq!(SpreadsheetVersion::EXCEL2007.getMaxRows(), 1048576);
    assert_eq!(SpreadsheetVersion::EXCEL2007.getLastRowIndex(), 1048575);
    assert_eq!(SpreadsheetVersion::EXCEL2007.getMaxColumns(), 16384);
    assert_eq!(SpreadsheetVersion::EXCEL2007.getLastColumnIndex(), 16383);
    assert_eq!(SpreadsheetVersion::EXCEL2007.getMaxFunctionArgs(), 255);
    assert_eq!(
        SpreadsheetVersion::EXCEL2007.getMaxConditionalFormats(),
        2147483647
    );
    assert_eq!(SpreadsheetVersion::EXCEL2007.getMaxTextLength(), 32767);
    assert_eq!(*CUR_VERSION.lock().unwrap(), SpreadsheetVersion::EXCEL97);
    *CUR_VERSION.lock().unwrap() = SpreadsheetVersion::EXCEL2007;
    assert_eq!(*CUR_VERSION.lock().unwrap(), SpreadsheetVersion::EXCEL2007);
    *CUR_VERSION.lock().unwrap() = SpreadsheetVersion::EXCEL97;
}

#[test]
fn cc1_argument_and_index_bounds() {
    KmoConstraintCheck::checkArgument(true);
    assert!(catch_unwind(|| KmoConstraintCheck::checkArgument(false)).is_err());
    assert_eq!(KmoConstraintCheck::checkElementIndex__int_int(0, 3), 0);
    assert_eq!(KmoConstraintCheck::checkElementIndex__int_int(2, 3), 2);
    assert!(catch_unwind(|| KmoConstraintCheck::checkElementIndex__int_int(3, 3)).is_err());
    assert!(catch_unwind(|| KmoConstraintCheck::checkElementIndex__int_int(-1, 3)).is_err());
    assert_eq!(KmoConstraintCheck::checkPositionIndex__int_int(3, 3), 3);
    assert!(catch_unwind(|| KmoConstraintCheck::checkPositionIndex__int_int(4, 3)).is_err());
    assert!(catch_unwind(|| KmoConstraintCheck::checkElementIndex__int_int_string(0, -1, "index")).is_err());
    KmoConstraintCheck::checkPositionIndexes(0, 2, 2);
    assert!(catch_unwind(|| KmoConstraintCheck::checkPositionIndexes(1, 0, 2)).is_err());
}

#[test]
fn cc2_format_and_null_message() {
    assert_eq!(
        KmoConstraintCheck::format(Some("%s (%s)"), &["index".into(), "1".into()]),
        "index (1)"
    );
    assert_eq!(
        KmoConstraintCheck::format(Some("%s"), &["a".into(), "b".into()]),
        "a [b]"
    );
    assert_eq!(
        KmoConstraintCheck::format(Some("plain"), &["a".into()]),
        "plain [a]"
    );
    assert_eq!(KmoConstraintCheck::format(None, &[]), "null");
    assert_eq!(KmoConstraintCheck::checkNotNull(Some(7)), 7);
    assert!(catch_unwind(|| KmoConstraintCheck::checkNotNull(None::<i32>)).is_err());
    assert!(catch_unwind(|| {
        KmoConstraintCheck::checkArgument__boolean_object(false, None);
    })
    .is_err());
    assert_eq!(
        KmoConstraintCheck::badElementIndex(-2, 4, "index"),
        "index (-2) must not be negative"
    );
    assert_eq!(
        KmoConstraintCheck::badPositionIndexes(2, 1, 4),
        "end index (1) must not be less than start index (2)"
    );
}
