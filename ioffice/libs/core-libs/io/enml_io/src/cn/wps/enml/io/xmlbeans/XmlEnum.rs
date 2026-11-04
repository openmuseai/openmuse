#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import org.dom4j.Attribute

use super::StringEnumAbstractBase::{StringEnumAbstractBase, Table};

pub struct Attribute {
    value: String,
}

impl Attribute {
    pub fn new(value: String) -> Self {
        Attribute { value }
    }

    pub fn getValue(&self) -> &str {
        &self.value
    }
}

// Java: public class XmlEnum
pub struct XmlEnum {
    // Java field: Attribute mAttr
    pub(crate) mAttr: Attribute,
    // Java field: StringEnumAbstractBase.Table mTable
    pub(crate) mTable: Option<Table>,
}

// Java: public class XmlEnum
impl XmlEnum {
    // Java: XmlEnum(Attribute attr, StringEnumAbstractBase.Table table)
    pub fn new(attr: Attribute, table: Option<Table>) -> Self {
      // Java: super();
      // Java: this.mAttr = attr;
      // Java: this.mTable = table;
      XmlEnum {
          mAttr: attr,
          mTable: table,
      }
    }

    // Java: StringEnumAbstractBase value()
    pub fn value(&self) -> Option<&StringEnumAbstractBase> {
      // Java: return this.toEnum();
      self.toEnum()
    }

    // Java: StringEnumAbstractBase toEnum()
    pub fn toEnum(&self) -> Option<&StringEnumAbstractBase> {
      // Java: Assert.assertNotNull("mTable should not be null", this.mTable);
      // Java: if (this.mTable == null)
      if self.mTable.is_none() {
         // Java: return null;
         return None;
      }
      // Java: else
      else {
         // Java: String str = this.toString();
         let str_val = self.toString_val();
         // Java: return str != null && str.length() > 0 ? this.mTable.forString(str.trim()) : null;
         if !str_val.is_empty() {
             self.mTable.as_ref().unwrap().forString(str_val.trim())
         } else {
             None
         }
      }
    }

    // Java: String toString()
    pub fn toString_val(&self) -> &str {
      // Java: return this.mAttr.getValue();
      self.mAttr.getValue()
    }
}
