use std::collections::HashMap;
use core_libs_io::io::opt::AttributesImpl::AttributesImpl;
use core_libs_io::io::opt::XmlStringBuilder::XmlStringBuilder;
use core_libs_io::io::xwpf::element_handler::vml::ConvertHelper::ConvertHelper;

fn attribute(name: &str, value: &str) -> AttributesImpl {
    let mut attrs = AttributesImpl::new_with_maps(HashMap::new(), None);
    let mut name_builder = XmlStringBuilder::new();
    for ch in name.chars() { name_builder.append_char(ch); }
    let mut value_builder = XmlStringBuilder::new();
    for ch in value.chars() { value_builder.append_char(ch); }
    attrs.addAttriteName(&name_builder);
    attrs.addAttriteValue(&value_builder);
    attrs
}

#[test]
fn java_vml_attribute_conversion_rules() {
    let absent = AttributesImpl::new_with_maps(HashMap::new(), None);
    assert_eq!(ConvertHelper::getString4EmptyNamespace(&absent, "Type"), None);
    assert_eq!(ConvertHelper::toBoolean(&absent, "filled"), None);

    assert_eq!(ConvertHelper::getString4EmptyNamespace(&attribute("Type", "Embed"), "Type"), Some("Embed".into()));
    for value in ["off", "OFF", "0", "false", "F"] {
        assert_eq!(ConvertHelper::toBoolean(&attribute("filled", value), "filled"), Some(false));
    }
    assert_eq!(ConvertHelper::toBoolean(&attribute("filled", "anything"), "filled"), Some(true));
    assert_eq!(ConvertHelper::toBoolean4STTrueFalse(&attribute("filled", "true"), "filled"), Some(true));
    assert_eq!(ConvertHelper::toBoolean4STTrueFalse(&attribute("filled", "TRUE"), "filled"), Some(false));
    assert_eq!(ConvertHelper::getInt4EmptyNamespace(&attribute("ShapeID", "42"), "ShapeID"), Some(42));
    assert_eq!(ConvertHelper::getLong4EmptyNamespace(&attribute("ShapeID", "42"), "ShapeID"), Some(42));
    assert_eq!(ConvertHelper::getFloat4EmptyNamespace(&attribute("size", "1.5"), "size"), Some(1.5));
    assert_eq!(ConvertHelper::getHex4EmptyNamespace(&attribute("color", "ff"), "color"), Some(255));
}
