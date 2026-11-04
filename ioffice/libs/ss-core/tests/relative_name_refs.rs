use ss_core::cn::wps::et::ss::calcchain::FormulaHelper::FormulaHelper;
use ss_core::cn::wps::et::ss::formula::ptg::Ptg::{Ptg, PtgKind};
use ss_core::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

#[test]
fn relative_name_ref_resolves_only_relative_coordinates() {
    // Java Ref3DPtg sid 58: 0x8000 is row relative; column 7 is absolute.
    let ref_ptg = Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Ref3D { extern_index: 2, row: 4, col_raw: 0x8007, wide: true }, class: 0x20 };
    let result = FormulaHelper::replaceRelativeNamePtgs(&[ref_ptg], 10, 20, SpreadsheetVersion::EXCEL2007);
    assert_eq!(result[0].getSid(), 58);
    assert_eq!(result[0].class, 0x20);
    let PtgKind::Ref3D { extern_index, row, col_raw, .. } = result[0].kind else { panic!("Ref3D") };
    assert_eq!((extern_index, row, col_raw), (2, 14, 7));
}

#[test]
fn relative_name_area_normalizes_endpoint_order_after_resolution() {
    // Java Area3DPtg sid 59: first endpoint relative, last absolute.
    let area_ptg = Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Area3D {
        extern_index: 3, first_row: 8, last_row: 5,
        first_col_raw: 0x4009, last_col_raw: 4, wide: true,
    }, class: 0x40 };
    let result = FormulaHelper::replaceRelativeNamePtgs(&[area_ptg], 0, 2, SpreadsheetVersion::EXCEL2007);
    assert_eq!(result[0].getSid(), 59);
    assert_eq!(result[0].class, 0x40);
    let PtgKind::Area3D { first_row, last_row, first_col_raw, last_col_raw, .. } = result[0].kind else { panic!("Area3D") };
    assert_eq!((first_row, last_row, first_col_raw, last_col_raw), (5, 8, 4, 11));
}

#[test]
fn formula_helper_is_same_compares_only_token_class_like_java() {
    let left = [Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Name { index: 1 }, class: 0x20 }];
    let different_token_same_class = [Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Name { index: 99 }, class: 0x20 }];
    let different_class = [Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Name { index: 1 }, class: 0x40 }];
    assert!(FormulaHelper::isSame(Some(&left), Some(&different_token_same_class)));
    assert!(!FormulaHelper::isSame(Some(&left), Some(&different_class)));
    assert!(!FormulaHelper::isSame(Some(&left), Some(&[])));
    assert!(!FormulaHelper::isSame(None, Some(&left)));
}

#[test]
fn external_3d_name_refs_keep_both_sheet_indices_and_resolve_base_coordinates() {
    // java Ref3DExtPtg/Area3DExtPtg extend RefPtgBase/AreaPtgBase in replaceRelativeNamePtgs.
    let reference = Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Ref3DExt {
        sheet_index1: 2, sheet_index2: 4, row: 4, col_raw: 0xc007, wide: true,
    }, class: 0x20 };
    let area = Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Area3DExt {
        sheet_index1: 3, sheet_index2: 5, first_row: 8, last_row: 5,
        first_col_raw: 0x4009, last_col_raw: 4, wide: true,
    }, class: 0x40 };
    let result = FormulaHelper::replaceRelativeNamePtgs(&[reference, area], 10, 2, SpreadsheetVersion::EXCEL2007);
    match result[0].kind {
        PtgKind::Ref3DExt { sheet_index1, sheet_index2, row, col_raw, .. } =>
            assert_eq!((sheet_index1, sheet_index2, row, col_raw), (2, 4, 14, 9)),
        _ => panic!("Ref3DExt"),
    }
    match result[1].kind {
        PtgKind::Area3DExt { sheet_index1, sheet_index2, first_row, last_row, first_col_raw, last_col_raw, .. } =>
            assert_eq!((sheet_index1, sheet_index2, first_row, last_row, first_col_raw, last_col_raw), (3, 5, 5, 8, 4, 11)),
        _ => panic!("Area3DExt"),
    }
    assert_eq!((result[0].class, result[1].class), (0x20, 0x40));
}

#[test]
fn external_3d_conversion_uses_java_base_class_and_concrete_class_casts() {
    let ref_ext = Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Ref3DExt {
        sheet_index1: 1, sheet_index2: 2, row: 65_536, col_raw: 1, wide: true,
    }, class: 0 };
    let area_ext = Ptg { java_identity:ss_core::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Area3DExt {
        sheet_index1: 1, sheet_index2: 2, first_row: 0, last_row: 1_048_575,
        first_col_raw: 0, last_col_raw: 16_383, wide: true,
    }, class: 0 };
    // java formulaType 0/3 casts to the base class; Ref outside Excel97 becomes RefError,
    // and a whole-sheet Area is reduced to Excel97 limits.
    let converted = FormulaHelper::convertFromExcel2007ToExcel97(&[ref_ext.clone(), area_ext.clone()], 0, false);
    assert!(matches!(converted[0].kind, PtgKind::RefError));
    match converted[1].kind {
        PtgKind::Area3DExt { sheet_index1, sheet_index2, last_row, last_col_raw, .. } =>
            assert_eq!((sheet_index1, sheet_index2, last_row, last_col_raw), (1, 2, 65_535, 255)),
        _ => panic!("Area3DExt"),
    }
    // java formulaType 4, replaceNPtg and replaceWithNPtg cast the same sid to
    // Ref3DPtg/Area3DPtg; the external-name subclasses fail that cast.
    assert!(std::panic::catch_unwind(|| FormulaHelper::convertFromExcel2007ToExcel97(&[ref_ext.clone()], 4, true)).is_err());
    assert!(std::panic::catch_unwind(|| FormulaHelper::replaceNPtgs(&[ref_ext.clone()], 0, 0, SpreadsheetVersion::EXCEL2007)).is_err());
    assert!(std::panic::catch_unwind(|| FormulaHelper::replaceWithNPtgs(&[area_ext.clone()], 0, 0, SpreadsheetVersion::EXCEL2007)).is_err());
}
