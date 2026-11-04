#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.base.assertion.Assert
// Java: import cn.wps.moffice.util.StringUtil
use crate::moffice::util::StringUtil::StringUtil;
use super::TypeConversion::TypeConversion;

// Java: public class Unit
pub trait UnitTrait {
    // Java: abstract void setUnit(String var1)
    fn setUnit(&mut self, unit: &str);

    fn magnitude_mut(&mut self) -> &mut Option<f32>;
}

// Java: public class Unit
pub struct Unit {
    // Java field: Float mMagnitude
    pub mMagnitude: Option<f32>,
}

// Java: public class Unit
impl Unit {
    // Java: Unit()
    pub fn new() -> Self {
        // Java: super();
        Self { mMagnitude: None }
    }

    // Java: void setValue(String value)
    pub fn setValue(&mut self, value: &str) {
        // Java: Assert.assertNotNull("value should not be true", value);
        // Java: value = trim(value);
        let value = Self::trim(value);
        // Java: if (null != value && value.length() > 0)
        if let Some(ref value) = value {
            if !value.is_empty()
            {
                // Java: int i = StringUtil.findLastDigitFromEnd(value);
                let i = StringUtil::findLastDigitFromEnd(value);
                // Java: if (i < 0)
                if i < 0
                {
                    // Java: return;
                    return;
                }
                let idx = (i + 1) as usize;
                // Java: String magnitude = value.substring(0, i + 1);
                let magnitude = &value[..idx];
                // Java: if (null != magnitude && magnitude.length() > 0)
                if !magnitude.is_empty()
                {
                    // Java: this.setMagnitude(magnitude);
                    self.setMagnitude(magnitude);
                }
                // Java: String unit = value.substring(i + 1);
                let unit = &value[idx..];
                // Java: if (null != unit)
                {
                    // Java: this.setUnit(unit);
                }
            }
        }
    }

    // Java: void setMagnitude(String strMagnitude)
    fn setMagnitude(&mut self, str_magnitude: &str) {
        // Java: this.mMagnitude = TypeConversion.string2Float(strMagnitude);
        self.mMagnitude = TypeConversion::string2Float(str_magnitude);
    }

    // Java: static String trim(String value)
    pub fn trim(value: &str) -> Option<String> {
        // Java: Assert.assertNotNull("value should not be true", value);
        // Java: value = value.trim();
        let value = value.trim();
        // Java: String SEPARATORS = " |,";
        // Java: _parts: &[String] = value.split(" |,");
        let parts: Vec<&str> = value.split(&[' ', '|', ','][..]).collect();
        // Java: Assert.assertNotNull("parts shoud not be null!", parts);
        // Java: int size = parts.length;
        let size = parts.len();
        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size
        {
            // Java: if (parts[i].length() > 0)
            if !parts[i].is_empty()
            {
                // Java: return parts[i];
                return Some(parts[i].to_string());
            }
        }
        // Java: return null;
        None
    }

    // Java: abstract void setUnit(String var1)
}
