// Java: import cn.wps.kfc.api.map.ILongObjectMap
// Java: import cn.wps.kfc.impl.map.WLongObjectHashMap

use std::collections::HashMap;
use std::sync::OnceLock;

fn get_number_to_name() -> &'static HashMap<i64, &'static str> {
    static MAP: OnceLock<HashMap<i64, &'static str>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::with_capacity(40);
        // Java: numberToName.put(0L, "VT_EMPTY");
        m.insert(0, "VT_EMPTY");
        // Java: numberToName.put(1L, "VT_NULL");
        m.insert(1, "VT_NULL");
        // Java: numberToName.put(2L, "VT_I2");
        m.insert(2, "VT_I2");
        // Java: numberToName.put(3L, "VT_I4");
        m.insert(3, "VT_I4");
        // Java: numberToName.put(4L, "VT_R4");
        m.insert(4, "VT_R4");
        // Java: numberToName.put(5L, "VT_R8");
        m.insert(5, "VT_R8");
        // Java: numberToName.put(6L, "VT_CY");
        m.insert(6, "VT_CY");
        // Java: numberToName.put(7L, "VT_DATE");
        m.insert(7, "VT_DATE");
        // Java: numberToName.put(8L, "VT_BSTR");
        m.insert(8, "VT_BSTR");
        // Java: numberToName.put(9L, "VT_DISPATCH");
        m.insert(9, "VT_DISPATCH");
        // Java: numberToName.put(10L, "VT_ERROR");
        m.insert(10, "VT_ERROR");
        // Java: numberToName.put(11L, "VT_BOOL");
        m.insert(11, "VT_BOOL");
        // Java: numberToName.put(12L, "VT_VARIANT");
        m.insert(12, "VT_VARIANT");
        // Java: numberToName.put(13L, "VT_UNKNOWN");
        m.insert(13, "VT_UNKNOWN");
        // Java: numberToName.put(14L, "VT_DECIMAL");
        m.insert(14, "VT_DECIMAL");
        // Java: numberToName.put(16L, "VT_I1");
        m.insert(16, "VT_I1");
        // Java: numberToName.put(17L, "VT_UI1");
        m.insert(17, "VT_UI1");
        // Java: numberToName.put(18L, "VT_UI2");
        m.insert(18, "VT_UI2");
        // Java: numberToName.put(19L, "VT_UI4");
        m.insert(19, "VT_UI4");
        // Java: numberToName.put(20L, "VT_I8");
        m.insert(20, "VT_I8");
        // Java: numberToName.put(21L, "VT_UI8");
        m.insert(21, "VT_UI8");
        // Java: numberToName.put(22L, "VT_INT");
        m.insert(22, "VT_INT");
        // Java: numberToName.put(23L, "VT_UINT");
        m.insert(23, "VT_UINT");
        // Java: numberToName.put(24L, "VT_VOID");
        m.insert(24, "VT_VOID");
        // Java: numberToName.put(25L, "VT_HRESULT");
        m.insert(25, "VT_HRESULT");
        // Java: numberToName.put(26L, "VT_PTR");
        m.insert(26, "VT_PTR");
        // Java: numberToName.put(27L, "VT_SAFEARRAY");
        m.insert(27, "VT_SAFEARRAY");
        // Java: numberToName.put(28L, "VT_CARRAY");
        m.insert(28, "VT_CARRAY");
        // Java: numberToName.put(29L, "VT_USERDEFINED");
        m.insert(29, "VT_USERDEFINED");
        // Java: numberToName.put(30L, "VT_LPSTR");
        m.insert(30, "VT_LPSTR");
        // Java: numberToName.put(31L, "VT_LPWSTR");
        m.insert(31, "VT_LPWSTR");
        // Java: numberToName.put(64L, "VT_FILETIME");
        m.insert(64, "VT_FILETIME");
        // Java: numberToName.put(65L, "VT_BLOB");
        m.insert(65, "VT_BLOB");
        // Java: numberToName.put(66L, "VT_STREAM");
        m.insert(66, "VT_STREAM");
        // Java: numberToName.put(67L, "VT_STORAGE");
        m.insert(67, "VT_STORAGE");
        // Java: numberToName.put(68L, "VT_STREAMED_OBJECT");
        m.insert(68, "VT_STREAMED_OBJECT");
        // Java: numberToName.put(69L, "VT_STORED_OBJECT");
        m.insert(69, "VT_STORED_OBJECT");
        // Java: numberToName.put(70L, "VT_BLOB_OBJECT");
        m.insert(70, "VT_BLOB_OBJECT");
        // Java: numberToName.put(71L, "VT_CF");
        m.insert(71, "VT_CF");
        // Java: numberToName.put(72L, "VT_CLSID");
        m.insert(72, "VT_CLSID");
        m
    })
}

fn get_number_to_length() -> &'static HashMap<i64, i32> {
    static MAP: OnceLock<HashMap<i64, i32>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::with_capacity(40);
        // Java: numberToLength.put(0L, LENGTH_0);
        m.insert(0, 0);
        // Java: numberToLength.put(1L, LENGTH_UNKNOWN);
        m.insert(1, -2);
        // Java: numberToLength.put(2L, LENGTH_2);
        m.insert(2, 2);
        // Java: numberToLength.put(3L, LENGTH_4);
        m.insert(3, 4);
        // Java: numberToLength.put(4L, LENGTH_4);
        m.insert(4, 4);
        // Java: numberToLength.put(5L, LENGTH_8);
        m.insert(5, 8);
        // Java: numberToLength.put(6L, LENGTH_UNKNOWN);
        m.insert(6, -2);
        // Java: numberToLength.put(7L, LENGTH_UNKNOWN);
        m.insert(7, -2);
        // Java: numberToLength.put(8L, LENGTH_UNKNOWN);
        m.insert(8, -2);
        // Java: numberToLength.put(9L, LENGTH_UNKNOWN);
        m.insert(9, -2);
        // Java: numberToLength.put(10L, LENGTH_UNKNOWN);
        m.insert(10, -2);
        // Java: numberToLength.put(11L, LENGTH_UNKNOWN);
        m.insert(11, -2);
        // Java: numberToLength.put(12L, LENGTH_UNKNOWN);
        m.insert(12, -2);
        // Java: numberToLength.put(13L, LENGTH_UNKNOWN);
        m.insert(13, -2);
        // Java: numberToLength.put(14L, LENGTH_UNKNOWN);
        m.insert(14, -2);
        // Java: numberToLength.put(16L, LENGTH_UNKNOWN);
        m.insert(16, -2);
        // Java: numberToLength.put(17L, LENGTH_UNKNOWN);
        m.insert(17, -2);
        // Java: numberToLength.put(18L, LENGTH_UNKNOWN);
        m.insert(18, -2);
        // Java: numberToLength.put(19L, LENGTH_UNKNOWN);
        m.insert(19, -2);
        // Java: numberToLength.put(20L, LENGTH_UNKNOWN);
        m.insert(20, -2);
        // Java: numberToLength.put(21L, LENGTH_UNKNOWN);
        m.insert(21, -2);
        // Java: numberToLength.put(22L, LENGTH_UNKNOWN);
        m.insert(22, -2);
        // Java: numberToLength.put(23L, LENGTH_UNKNOWN);
        m.insert(23, -2);
        // Java: numberToLength.put(24L, LENGTH_UNKNOWN);
        m.insert(24, -2);
        // Java: numberToLength.put(25L, LENGTH_UNKNOWN);
        m.insert(25, -2);
        // Java: numberToLength.put(26L, LENGTH_UNKNOWN);
        m.insert(26, -2);
        // Java: numberToLength.put(27L, LENGTH_UNKNOWN);
        m.insert(27, -2);
        // Java: numberToLength.put(28L, LENGTH_UNKNOWN);
        m.insert(28, -2);
        // Java: numberToLength.put(29L, LENGTH_UNKNOWN);
        m.insert(29, -2);
        // Java: numberToLength.put(30L, LENGTH_VARIABLE);
        m.insert(30, -1);
        // Java: numberToLength.put(31L, LENGTH_UNKNOWN);
        m.insert(31, -2);
        // Java: numberToLength.put(64L, LENGTH_8);
        m.insert(64, 8);
        // Java: numberToLength.put(65L, LENGTH_UNKNOWN);
        m.insert(65, -2);
        // Java: numberToLength.put(66L, LENGTH_UNKNOWN);
        m.insert(66, -2);
        // Java: numberToLength.put(67L, LENGTH_UNKNOWN);
        m.insert(67, -2);
        // Java: numberToLength.put(68L, LENGTH_UNKNOWN);
        m.insert(68, -2);
        // Java: numberToLength.put(69L, LENGTH_UNKNOWN);
        m.insert(69, -2);
        // Java: numberToLength.put(70L, LENGTH_UNKNOWN);
        m.insert(70, -2);
        // Java: numberToLength.put(71L, LENGTH_UNKNOWN);
        m.insert(71, -2);
        // Java: numberToLength.put(72L, LENGTH_UNKNOWN);
        m.insert(72, -2);
        m
    })
}

// Java field: int VT_EMPTY
pub const VT_EMPTY: i32 = 0;
// Java field: int VT_NULL
pub const VT_NULL: i32 = 1;
// Java field: int VT_I2
pub const VT_I2: i32 = 2;
// Java field: int VT_I4
pub const VT_I4: i32 = 3;
// Java field: int VT_R4
pub const VT_R4: i32 = 4;
// Java field: int VT_R8
pub const VT_R8: i32 = 5;
// Java field: int VT_CY
pub const VT_CY: i32 = 6;
// Java field: int VT_DATE
pub const VT_DATE: i32 = 7;
// Java field: int VT_BSTR
pub const VT_BSTR: i32 = 8;
// Java field: int VT_DISPATCH
pub const VT_DISPATCH: i32 = 9;
// Java field: int VT_ERROR
pub const VT_ERROR: i32 = 10;
// Java field: int VT_BOOL
pub const VT_BOOL: i32 = 11;
// Java field: int VT_VARIANT
pub const VT_VARIANT: i32 = 12;
// Java field: int VT_UNKNOWN
pub const VT_UNKNOWN: i32 = 13;
// Java field: int VT_DECIMAL
pub const VT_DECIMAL: i32 = 14;
// Java field: int VT_I1
pub const VT_I1: i32 = 16;
// Java field: int VT_UI1
pub const VT_UI1: i32 = 17;
// Java field: int VT_UI2
pub const VT_UI2: i32 = 18;
// Java field: int VT_UI4
pub const VT_UI4: i32 = 19;
// Java field: int VT_I8
pub const VT_I8: i32 = 20;
// Java field: int VT_UI8
pub const VT_UI8: i32 = 21;
// Java field: int VT_INT
pub const VT_INT: i32 = 22;
// Java field: int VT_UINT
pub const VT_UINT: i32 = 23;
// Java field: int VT_VOID
pub const VT_VOID: i32 = 24;
// Java field: int VT_HRESULT
pub const VT_HRESULT: i32 = 25;
// Java field: int VT_PTR
pub const VT_PTR: i32 = 26;
// Java field: int VT_SAFEARRAY
pub const VT_SAFEARRAY: i32 = 27;
// Java field: int VT_CARRAY
pub const VT_CARRAY: i32 = 28;
// Java field: int VT_USERDEFINED
pub const VT_USERDEFINED: i32 = 29;
// Java field: int VT_LPSTR
pub const VT_LPSTR: i32 = 30;
// Java field: int VT_LPWSTR
pub const VT_LPWSTR: i32 = 31;
// Java field: int VT_FILETIME
pub const VT_FILETIME: i32 = 64;
// Java field: int VT_BLOB
pub const VT_BLOB: i32 = 65;
// Java field: int VT_STREAM
pub const VT_STREAM: i32 = 66;
// Java field: int VT_STORAGE
pub const VT_STORAGE: i32 = 67;
// Java field: int VT_STREAMED_OBJECT
pub const VT_STREAMED_OBJECT: i32 = 68;
// Java field: int VT_STORED_OBJECT
pub const VT_STORED_OBJECT: i32 = 69;
// Java field: int VT_BLOB_OBJECT
pub const VT_BLOB_OBJECT: i32 = 70;
// Java field: int VT_CF
pub const VT_CF: i32 = 71;
// Java field: int VT_CLSID
pub const VT_CLSID: i32 = 72;
// Java field: int VT_VECTOR
pub const VT_VECTOR: i32 = 4096;
// Java field: int VT_ARRAY
pub const VT_ARRAY: i32 = 8192;
// Java field: int VT_BYREF
pub const VT_BYREF: i32 = 16384;
// Java field: int VT_RESERVED
pub const VT_RESERVED: i32 = 32768;
// Java field: int VT_ILLEGAL
pub const VT_ILLEGAL: i32 = 65535;
// Java field: int VT_ILLEGALMASKED
pub const VT_ILLEGALMASKED: i32 = 4095;
// Java field: int VT_TYPEMASK
pub const VT_TYPEMASK: i32 = 4095;
// Java field: Integer LENGTH_UNKNOWN
pub const LENGTH_UNKNOWN: i32 = -2;
// Java field: Integer LENGTH_VARIABLE
pub const LENGTH_VARIABLE: i32 = -1;
// Java field: Integer LENGTH_0
pub const LENGTH_0: i32 = 0;
// Java field: Integer LENGTH_2
pub const LENGTH_2: i32 = 2;
// Java field: Integer LENGTH_4
pub const LENGTH_4: i32 = 4;
// Java field: Integer LENGTH_8
pub const LENGTH_8: i32 = 8;

// Java: public class Variant
pub struct Variant;

// Java: public class Variant
impl Variant {
    // Java: Variant()
    pub fn new() -> Self {
        // Java: super();
        Variant
    }

    // Java: String getVariantName(long variantType)
    pub fn getVariantName(variantType: i64) -> &'static str {
        // Java: String name = (String)numberToName.get(variantType);
        let name = get_number_to_name().get(&variantType);
        // Java: return name != null ? name : "unknown variant type";
        match name {
            Some(n) => n,
            None => "unknown variant type",
        }
    }

    // Java: int getVariantLength(long variantType)
    pub fn getVariantLength(variantType: i64) -> i32 {
        // Java: Integer length = (Integer)numberToLength.get(variantType);
        let length = get_number_to_length().get(&variantType);
        // Java: return length == null ? -2 : length;
        match length {
            Some(l) => *l,
            None => -2,
        }
    }
}
