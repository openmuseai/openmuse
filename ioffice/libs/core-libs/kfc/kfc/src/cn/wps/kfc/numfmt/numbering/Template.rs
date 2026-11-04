#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class Template extends NumberingBase
pub struct Template {
    pub(crate) base: NumberingBase,
    // Java field: String mTemplate
    mTemplate: Option<String>,
}

// Java: public class Template extends NumberingBase
impl Template {
    // Java: Template()
    pub fn new() -> Self {
        // Java: super();
        Template {
            base: NumberingBase::new(),
            mTemplate: None,
        }
    }

    // Java: void setPageCode(int nfc)
    pub fn setPageCode(&mut self, _nfc: i32) {
        // Java: switch (nfc)
        {
            // Java: case 57:
            // Java: this.mTemplate = "-%s-";
            // Java: default:
        }
        todo!("Template::setPageCode")
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: String strValue = this.longToStr(page);
        // Java: String result = String.format(this.mTemplate, strValue);
        // Java: return result;
        todo!("Template::convert")
    }
}

impl INumbering for Template {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
