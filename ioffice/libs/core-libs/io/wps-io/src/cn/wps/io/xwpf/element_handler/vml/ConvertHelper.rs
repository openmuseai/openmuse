#![allow(non_snake_case)]

use std::any::Any;
use crate::cn::wps::io::dom::io::WMap::WMap;
use crate::cn::wps::io::opt::AttributesImpl::AttributesImpl;
use core_libs_util::cn::wps::base::io::TypeConversion::TypeConversion;

// Java: public class ConvertHelper
pub struct ConvertHelper;

impl ConvertHelper {
    // Java: public static final String EMPTY_NAMESPACE = "";
    pub const EMPTY_NAMESPACE: &'static str = "";

    // Java: public ConvertHelper()
    pub fn new() -> Self { Self }

    fn attributes(attributes: &dyn Any) -> &AttributesImpl {
        // The SAX boundary supplies AttributesImpl. A different object is a wiring error,
        // not a missing XML attribute, so it must not be turned into Java null.
        attributes.downcast_ref::<AttributesImpl>()
            .expect("VML ConvertHelper requires SAX AttributesImpl")
    }

    // Java: public static String getString4EmptyNamespace(Attributes attributes, String name)
    pub fn getString4EmptyNamespace(attributes: &dyn Any, name: &str) -> Option<String> {
        Self::getString__Attributes_String_String(attributes, name, Self::EMPTY_NAMESPACE)
    }

    // Java: public static String getString(Attributes attributes, String name)
    pub fn getString(attributes: &dyn Any, name: &str) -> Option<String> {
        Self::getString__Attributes_String_String(attributes, name, WMap::MAIN_NAMESPACING())
    }

    // Java: public static String getString(Attributes attributes, String name, String namespace)
    pub fn getString__Attributes_String_String(attributes: &dyn Any, name: &str, namespace: &str) -> Option<String> {
        let attributes = Self::attributes(attributes);
        if attributes.getLength() == 0 { None } else { attributes.getValue_by_uri(namespace, name) }
    }

    // Java: public static Boolean toBoolean(Attributes attributes, String name)
    pub fn toBoolean(attributes: &dyn Any, name: &str) -> Option<bool> {
        Self::toBoolean__Attributes_String_String(attributes, name, Self::EMPTY_NAMESPACE)
    }

    // Java: private static Boolean toBoolean(Attributes attributes, String name, String namespace)
    fn toBoolean__Attributes_String_String(attributes: &dyn Any, name: &str, namespace: &str) -> Option<bool> {
        let value = Self::getString__Attributes_String_String(attributes, name, namespace)?;
        Some(!Self::isFalse(&value))
    }

    // Java: private static boolean isFalse(String str)
    fn isFalse(value: &str) -> bool {
        value.eq_ignore_ascii_case("off") || value == "0" || value.eq_ignore_ascii_case("false") || value.eq_ignore_ascii_case("f")
    }

    // Java: public static Boolean toBoolean4STTrueFalse(Attributes attributes, String name)
    pub fn toBoolean4STTrueFalse(attributes: &dyn Any, name: &str) -> Option<bool> {
        Self::toBoolean4STTrueFalse__Attributes_String_String(attributes, name, Self::EMPTY_NAMESPACE)
    }

    // Java: public static Boolean toBoolean4STTrueFalse(Attributes attributes, String name, String namespace)
    pub fn toBoolean4STTrueFalse__Attributes_String_String(attributes: &dyn Any, name: &str, namespace: &str) -> Option<bool> {
        Some(matches!(Self::getString__Attributes_String_String(attributes, name, namespace)?.as_str(), "true" | "t"))
    }

    // Java: public static Float getFloat4EmptyNamespace(Attributes attributes, String name)
    pub fn getFloat4EmptyNamespace(attributes: &dyn Any, name: &str) -> Option<f32> {
        Self::getFloat(attributes, name, Self::EMPTY_NAMESPACE)
    }

    // Java: public static Float getFloat(Attributes attributes, String name, String namespace)
    pub fn getFloat(attributes: &dyn Any, name: &str, namespace: &str) -> Option<f32> {
        TypeConversion::string2Float(&Self::attributes(attributes).getValue_by_uri(namespace, name)?)
    }

    // Java: public static Integer getInt4EmptyNamespace(Attributes attributes, String name)
    pub fn getInt4EmptyNamespace(attributes: &dyn Any, name: &str) -> Option<i32> {
        TypeConversion::string2Integer(&Self::attributes(attributes).getValue_by_uri(Self::EMPTY_NAMESPACE, name)?)
    }

    // Java: public static Long getLong4EmptyNamespace(Attributes attributes, String name)
    pub fn getLong4EmptyNamespace(attributes: &dyn Any, name: &str) -> Option<i64> {
        TypeConversion::string2Long(&Self::attributes(attributes).getValue_by_uri(Self::EMPTY_NAMESPACE, name)?)
    }

    // Java: public static Integer getHex4EmptyNamespace(Attributes attributes, String name)
    pub fn getHex4EmptyNamespace(attributes: &dyn Any, name: &str) -> Option<i32> {
        TypeConversion::hexString2Integer(&Self::attributes(attributes).getValue_by_uri(Self::EMPTY_NAMESPACE, name)?)
    }
}
