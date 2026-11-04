use word_core::cn::wps::moffice::writer::io::reader::dmlReader::context::{
    Cxn::Cxn, DmlShapeCtxt::DmlShapeCtxt, FormulaType::FormulaType, Gds::Gds,
};

fn java_hash(value: &str) -> i32 {
    value.encode_utf16().fold(0_i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
}

#[test]
fn predefined_formulas_keep_java_parsing_and_cache_behavior() {
    let gds = Gds::new();
    assert!(!gds.borrow().isEmpty());
    assert_eq!(Gds::getPredefinedConstant(Gds::CD4, 10, 20), Some(5_400_000));
    // Java has the literal "*/w" without a space; FormulaType returns ERROR.
    assert_eq!(gds.borrow().get(Some(java_hash("hc")), Some(10_000), Some(20_000)), None);
    assert_eq!(gds.borrow().get(Some(java_hash("hd2")), Some(10_000), Some(20_000)), Some(10_000));
    // Fmla caches its first non-null result even when the caller supplies new dimensions.
    assert_eq!(gds.borrow().get(Some(java_hash("hd2")), Some(10_000), Some(40_000)), Some(10_000));
    assert_eq!(FormulaType::toType("*/"), FormulaType::MUL_DIV);
}

#[test]
fn custom_formula_and_connection_hashes_follow_java() {
    let gds = Gds::new();
    gds.borrow_mut().add("adj1", "val -1.5");
    assert_eq!(gds.borrow().get(Some(Gds::ADJ1), None, None), Some(-1));
    let mut cxn = Cxn::new();
    cxn.setXY("wd2", "hd2");
    cxn.setAng("cd4");
    assert_eq!(cxn.mX, Some(java_hash("wd2")));
    assert_eq!(cxn.mY, Some(java_hash("hd2")));
    assert_eq!(cxn.mAng, Some(java_hash("cd4")));
    let shape_ctxt = DmlShapeCtxt::new();
    assert!(shape_ctxt.mFillOn && shape_ctxt.mLineFillOn && shape_ctxt.mBlipFillRotWithShape);
    assert!(shape_ctxt.mShape.is_none() && shape_ctxt.mGdlst.is_none() && shape_ctxt.mDuotone.is_none());
}
