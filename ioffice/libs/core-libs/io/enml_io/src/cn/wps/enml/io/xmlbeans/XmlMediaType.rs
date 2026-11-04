#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import org.dom4j.Attribute

use super::XmlEnum::{XmlEnum, Attribute};
use super::StringEnumAbstractBase::{StringEnumAbstractBase, Table};
use std::sync::OnceLock;

// Java field: int INT_GIF
pub(crate) const INT_GIF: i32 = 0;
// Java field: int INT_JPEG
pub(crate) const INT_JPEG: i32 = 1;
// Java field: int INT_PNG
pub(crate) const INT_PNG: i32 = 2;

fn get_enum_table() -> &'static Table {
    // Java field: StringEnumAbstractBase.Table table
    static TABLE: OnceLock<Table> = OnceLock::new();
    TABLE.get_or_init(|| {
        Table::new(vec![
            StringEnumAbstractBase::new("image/gif".to_string(), INT_GIF),
            StringEnumAbstractBase::new("image/jpeg".to_string(), INT_JPEG),
            StringEnumAbstractBase::new("image/png".to_string(), INT_PNG),
        ])
    })
}

// Java: public class XmlMediaType extends XmlEnum
pub struct XmlMediaType {
    base: XmlEnum,
}

// Java: public class XmlMediaType extends XmlEnum
impl XmlMediaType {
    // Java: XmlMediaType(Attribute attr)
    pub fn new(attr: Attribute) -> Self {
      // Java: super(attr, XmlMediaType.Enum.table);
      XmlMediaType {
          base: XmlEnum::new(attr, None),
      }
    }

    pub fn value(&self) -> Option<&StringEnumAbstractBase> {
        self.base.value()
    }

    pub fn toString_val(&self) -> &str {
        self.base.toString_val()
    }
}

// Java: public class Enum
pub struct MediaEnum {
    base: StringEnumAbstractBase,
}

// Java: public class Enum
impl MediaEnum {
    // Java: Enum(String s, int i)
    pub fn new(s: String, i: i32) -> Self {
         // Java: super(s, i);
         MediaEnum {
             base: StringEnumAbstractBase::new(s, i),
         }
    }
}
