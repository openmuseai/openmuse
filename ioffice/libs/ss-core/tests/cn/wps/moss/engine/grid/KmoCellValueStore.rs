use std::panic::{catch_unwind, AssertUnwindSafe};

use ss_core::cn::wps::moss::engine::grid::KmoCellValueStore::{KmoCellValueStore, ID_BLANK};
use ss_core::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData;
use ss_core::org::apache::poi::ss::usermodel::ErrorConstants::ErrorConstants;

fn store() -> KmoCellValueStore {
    KmoCellValueStore::KmoCellValueStore()
}

fn panic_has(f: impl FnOnce()) -> String {
    let err = catch_unwind(AssertUnwindSafe(f)).expect_err("expected panic");
    err.downcast_ref::<String>()
        .cloned()
        .or_else(|| err.downcast_ref::<&str>().map(|s| (*s).to_string()))
        .unwrap_or_default()
}

#[test]
fn vs1_double_cache_and_roundtrip_keeps_number() {
    let mut s = store();
    let zero = s.newDoubleValue(0.0);
    let again = s.newDoubleValue(0.0);
    let top = s.newDoubleValue(128.0);
    let half = s.newDoubleValue(1.5);
    let neg = s.newDoubleValue(-1.5);
    let past = s.newDoubleValue(129.0);
    assert_eq!(zero, again);
    assert_ne!(zero, ID_BLANK);
    assert_eq!(top, s.newDoubleValue(128.0));
    assert_ne!(half, past);
    assert_eq!(s.doubleValue(zero), 0.0);
    assert_eq!(s.doubleValue(top), 128.0);
    assert_eq!(s.doubleValue(half), 1.5);
    assert_eq!(s.doubleValue(neg), -1.5);
    assert_eq!(s.doubleValue(past), 129.0);
    assert!(KmoCellValueStore::isNumericId(half));
    assert!(!KmoCellValueStore::isStringId(half));
    assert_eq!(KmoCellValueStore::identifyTypeByID(half), 1);
    let msg = panic_has(|| {
        let mut s = store();
        let id = s.newDoubleValue(1.5);
        let _ = s.stringValue(id);
    });
    assert!(msg.contains("IllegalArgumentException"), "{msg}");
}

#[test]
fn vs2_string_dedup_null_empty_and_sst_index() {
    let mut s = store();
    let a = s.newStringValue(Some("a"));
    let a2 = s.newStringValue(Some("a"));
    let b = s.newStringValue(Some("b"));
    let empty = s.newStringValue(Some(""));
    let empty2 = s.newStringValue(Some(""));
    let nil = s.newStringValue(None);
    let nil2 = s.newStringValue(None);
    assert_eq!(a, a2);
    assert_ne!(a, b);
    assert_eq!(empty, empty2);
    assert_ne!(nil, nil2);
    assert_eq!(s.stringValue(a), "a");
    assert_eq!(s.stringValue(b), "b");
    assert_eq!(s.stringValue(empty), "");
    assert_eq!(s.stringValue(nil), "");
    assert_eq!(s.getStringBySSTId(0), "a");
    assert_eq!(s.getStringBySSTId(1), "b");
    assert_eq!(s.getStringBySSTId(a), "");
    assert_eq!(s.getStringBySSTId(99), "");
    assert_eq!(KmoCellValueStore::identifyTypeByID(a), 2);
    assert!(KmoCellValueStore::isStringId(a));
    assert_eq!(s.getUniqueCount(), 5);
    assert!(s.isValidId(0));
    assert!(!s.isValidId(5));
    let msg = panic_has(|| {
        store().getEngineStrStoreId(5);
    });
    assert!(msg.contains("IllegalArgumentException"), "{msg}");
    assert_eq!(s.getEngineStrStoreId(0), a);
}

#[test]
fn vs3_bool_and_error_are_not_blank_or_strings() {
    let s = store();
    let yes = s.newBooleanValue(true);
    let no = s.newBooleanValue(false);
    assert_eq!(yes, s.newBooleanValue(true));
    assert_eq!(no, s.newBooleanValue(false));
    assert_ne!(yes, no);
    assert!(s.booleanValue(yes));
    assert!(!s.booleanValue(no));
    assert!(!KmoCellValueStore::isBlankId(no));
    assert!(KmoCellValueStore::isBlankId(ID_BLANK));
    assert!(KmoCellValueStore::isBoolId(yes));
    assert_eq!(KmoCellValueStore::identifyTypeByID(yes), 5);
    for code in [0, 7, 15, 23, 29, 36, 42] {
        let id = s.newErrorValue(code);
        assert_eq!(s.errorValue(id) as i32, code);
        assert!(KmoCellValueStore::isErrId(id));
        assert_ne!(id, ID_BLANK);
        assert_eq!(ErrorConstants::getText(code).is_empty(), false);
        assert_eq!(ErrorConstants::getErrCode(Some(ErrorConstants::getText(code).as_str())), code);
    }
    let msg = panic_has(|| {
        store().newErrorValue(1);
    });
    assert!(msg.contains("IllegalStateException"), "{msg}");
    let bad = panic_has(|| {
        let _ = ErrorConstants::getText(1);
    });
    assert!(bad.contains("IllegalArgumentException"), "{bad}");
}

#[test]
fn vs4_type_3_and_4_read_back_as_string_type() {
    let mut s = store();
    let numeric = s.newDoubleValue(0.0);
    let type3 = numeric.wrapping_mul(3);
    let type4 = s.newFmlString("cache");
    let type4b = s.newFmlString("cache");
    assert_eq!(type4, type4b);
    assert_eq!(KmoCellValueStore::identifyTypeByID(type3), 2);
    assert_eq!(KmoCellValueStore::identifyTypeByID(type4), 2);
    assert!(KmoCellValueStore::stringTypeHasRun(type3));
    assert!(KmoCellValueStore::isRunsString(type3));
    assert!(!KmoCellValueStore::stringTypeHasRun(type4));
    assert!(KmoCellValueStore::isStringId(type3));
    assert!(KmoCellValueStore::isStringId(type4));
    assert_eq!(s.stringValue(type4), "cache");
    let plain = s.newStringValue(Some("plain"));
    assert!(!s.isRunsString2(plain));
    let nan = s.newDoubleValue(f64::NAN);
    assert!(s.doubleValue(nan).is_nan());
    assert_ne!(nan, numeric);
}

#[test]
fn vs5_formula_record_and_tokens_roundtrip() {
    let mut s = store();
    let mut formula = KmoFormulaData::KmoFormulaData();
    formula.setTokens(3, 4);
    formula.setCacheValueId(9);
    formula.setCalcFlags(2);
    let id = s.newFormulaValue(&formula);
    assert!(KmoCellValueStore::isFormulaId(id));
    assert_eq!(KmoCellValueStore::identifyTypeByID(id), 7);
    let mut back = KmoFormulaData::KmoFormulaData();
    s.formulaValue(id, &mut back);
    assert_eq!(back.getTokensId(), 3);
    assert_eq!(back.getTokensPlainSize(), 4);
    assert_eq!(back.getCacheValueId(), 9);
    assert_eq!(back.getCalcFlags(), 2);
    assert_eq!(s.formulaCachaId(id), 9);
    s.setFormulaCalcFlag(id, 6);
    assert_eq!(s.getFormulaCalcFlag(id), 6);
    assert_eq!(s.newFormulaTokens(None), 0);
    assert_eq!(s.newFormulaTokens(Some(&[])), 0);
    let tok = s.newFormulaTokens(Some(&[1, 2, 3]));
    assert_eq!(tok, 1);
    assert!(s.formulaTokens(0).is_empty());
    assert_eq!(s.formulaTokens(tok), vec![1, 2, 3]);
    let mut cleared = KmoFormulaData::KmoFormulaData();
    cleared.setTokens(1, 1);
    cleared.clear();
    assert_eq!(cleared.getTokensId(), 0);
    assert_eq!(cleared.write__1().len(), 11);
    s.releaseStoreData();
    let msg = panic_has(|| {
        let mut fresh = store();
        let _ = fresh.doubleValue(200);
    });
    assert!(msg.contains("IllegalArgumentException"), "{msg}");
}
