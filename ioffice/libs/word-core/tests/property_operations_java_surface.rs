use std::sync::Arc;

use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::moffice::property::PropertySet::PropertyValue;
use word_core::cn::wps::moffice::writer::data::PropertyOperations::PropertyOperations;
use word_core::cn::wps::moffice::writer::data::KTextStream::PropertyActionImpl;
use word_core::cn::wps::moffice::writer::data::TextStream_NormalFilter::TextStream_NormalFilter;

#[test]
fn java_property_operation_codes_and_operands() {
    let mut props = MutablePropertySet::new();
    props.setInt(10, 240);
    let props = props.freeze();

    for (operation, expected) in [
        (PropertyOperations::merge__propertyset(&props), 2),
        (PropertyOperations::clear__propertyset(&props), 3),
        (PropertyOperations::replace__propertyset(&props), 4),
    ] {
        assert_eq!(operation.getOp(), expected);
        assert_eq!(operation.getOperand().getInt(10), 240);
    }
    let single = PropertyOperations::replace__int_object(37, &360_i32);
    assert_eq!(single.getOp(), 1);
    assert_eq!(single.getOperand().getInt(37), 360);
}

#[test]
fn java_property_operation_optional_hints_are_stored_and_removed() {
    let mut props = MutablePropertySet::new();
    props.setInt(10, 240);
    let props = props.freeze();
    let key: PropertyValue = Arc::new(String::from("revision"));
    let value: PropertyValue = Arc::new(String::from("keep"));
    let mut operation = PropertyOperations::create__int_propertyset_object_object(
        3, &props, Some(key), Some(value),
    );
    let equivalent_key = String::from("revision");
    let found = operation.getHint(&equivalent_key).expect("Java HashMap value key");
    assert_eq!(found.as_any().downcast_ref::<String>().map(String::as_str), Some("keep"));
    operation.removeHint(&equivalent_key);
    assert!(operation.getHint(&equivalent_key).is_none());
}

#[test]
fn java_normal_filter_interprets_owned_action_in_order() {
    let mut base = MutablePropertySet::new();
    base.setInt(10, 100);
    base.setInt(11, 200);
    let base = base.freeze();
    let mut merge = MutablePropertySet::new();
    merge.setInt(10, 300);
    merge.setInt(12, 400);
    let merge = merge.freeze();
    let mut clear = MutablePropertySet::new();
    clear.setInt(11, 0);
    let clear = clear.freeze();

    let mut action = PropertyActionImpl::default();
    action.addOperation(PropertyOperations::merge__propertyset(&merge));
    action.addOperation(PropertyOperations::clear__propertyset(&clear));
    let result = TextStream_NormalFilter::interpret(Some(&base), &action).freeze();
    assert_eq!(result.getInt(10), 300);
    assert_eq!(result.getInt(12), 400);
    assert!(!result.contains(11));

    action.clear();
    let unchanged = TextStream_NormalFilter::interpret(Some(&base), &action).freeze();
    assert_eq!(unchanged.getInt(10), 100);
    assert_eq!(unchanged.getInt(11), 200);
}

#[test]
fn java_replace_keeps_special_run_table_and_language_properties() {
    let mut base = MutablePropertySet::new();
    base.setInt(28, 1);
    base.setInt(223, 7);
    base.setInt(57, 9);
    base.setInt(10, 100);
    let base = base.freeze();
    let mut replacement = MutablePropertySet::new();
    replacement.setInt(12, 200);
    let replacement = replacement.freeze();

    let mut action = PropertyActionImpl::default();
    action.addOperation(PropertyOperations::replace__propertyset(&replacement));
    let result = TextStream_NormalFilter::interpret(Some(&base), &action).freeze();
    assert_eq!(result.getInt(28), 1);
    assert_eq!(result.getInt(223), 7);
    assert_eq!(result.getInt(57), 9);
    assert_eq!(result.getInt(12), 200);
    assert!(!result.contains(10));
}
