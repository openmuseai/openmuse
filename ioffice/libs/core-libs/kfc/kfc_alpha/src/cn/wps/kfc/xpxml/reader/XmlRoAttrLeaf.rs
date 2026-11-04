// Java: public class XmlRoAttrLeaf implements XmlRoAttr
pub struct XmlRoAttrLeaf {
    // Java field: int mName
    pub(crate) mName: i32,
    // Java field: String mValue
    pub(crate) mValue: String,
}

// Java: public class XmlRoAttrLeaf implements XmlRoAttr
impl XmlRoAttrLeaf {
    // Java: XmlRoAttrLeaf()
    pub fn new() -> Self {
        // Java: super();
        XmlRoAttrLeaf {
            mName: 0,
            mValue: String::new(),
        }
    }

    // Java: XmlRoAttrLeaf(int name, String value)
    pub fn new_with_name(name: i32, value: &str) -> Self {
        // Java: super();
        // Java: this.mName = name;
        // Java: this.mValue = value;
        XmlRoAttrLeaf {
            mName: name,
            mValue: value.to_string(),
        }
    }

    // Java: void reset(int name, String value)
    pub fn reset(&mut self, name: i32, value: &str) {
        // Java: this.mName = name;
        self.mName = name;
        // Java: this.mValue = value;
        self.mValue = value.to_string();
    }

    // Java: String toString()
    pub fn to_string(&self) -> String {
        // Java: return this.mValue;
        self.mValue.clone()
    }

    // Java: int toInt()
    pub fn to_int(&self) -> i32 {
        // Java: if (this.mValue.length() < 10)
        if self.mValue.len() < 10 {
            // Java: try
            // Java: return Integer.parseInt(this.mValue);
            match self.mValue.parse::<i32>() {
                Ok(v) => return v,
                // Java: catch (NumberFormatException e)
                Err(_) => {
                    // Java: if ("INF".equals(this.mValue))
                    if self.mValue == "INF" {
                        // Java: return Integer.MAX_VALUE;
                        return i32::MAX;
                    }
                    // Java: else if ("-INF".equals(this.mValue))
                    else if self.mValue == "-INF" {
                        // Java: return Integer.MIN_VALUE;
                        return i32::MIN;
                    }
                    // Java: else if ("indefinite".equals(this.mValue))
                    else if self.mValue == "indefinite" {
                        // Java: return Integer.MAX_VALUE;
                        return i32::MAX;
                    }
                    // Java: else
                    else {
                        // Java: e.printStackTrace();
                        // Java: return 0;
                        return 0;
                    }
                }
            }
        }
        // Java: else
        else {
            // Java: try
            // Java: long value = Long.parseLong(this.mValue);
            match self.mValue.parse::<i64>() {
                Ok(value) => {
                    // Java: return value > 2147483647L ? Integer.MAX_VALUE : (int)value;
                    return if value > 2147483647 { i32::MAX } else { value as i32 };
                }
                // Java: catch (NumberFormatException e)
                Err(_) => {
                    // Java: if ("INF".equals(this.mValue))
                    if self.mValue == "INF" {
                        // Java: return Integer.MAX_VALUE;
                        return i32::MAX;
                    }
                    // Java: else if ("-INF".equals(this.mValue))
                    else if self.mValue == "-INF" {
                        // Java: return Integer.MIN_VALUE;
                        return i32::MIN;
                    }
                    // Java: else if ("indefinite".equals(this.mValue))
                    else if self.mValue == "indefinite" {
                        // Java: return Integer.MAX_VALUE;
                        return i32::MAX;
                    }
                    // Java: else
                    else {
                        // Java: e.printStackTrace();
                        // Java: return 0;
                        return 0;
                    }
                }
            }
        }
    }

    // Java: byte toByte()
    pub fn to_byte(&self) -> i8 {
        // Java: return this.mValue.length() < 3 ? Byte.parseByte(this.mValue) : (byte)this.toShort();
        if self.mValue.len() < 3 {
            self.mValue.parse::<i8>().unwrap_or(0)
        } else {
            self.to_short() as i8
        }
    }

    // Java: short toShort()
    pub fn to_short(&self) -> i16 {
        // Java: try
        // Java: return this.mValue.length() < 5 ? Short.parseShort(this.mValue) : (short)this.toInt();
        if self.mValue.len() < 5 {
            self.mValue.parse::<i16>().unwrap_or(0)
        } else {
            self.to_int() as i16
        }
        // Java: catch (Throwable var2)
        // Java: return 0;
    }

    // Java: boolean toBoolean()
    pub fn to_boolean(&self) -> bool {
        // Java: return this.mValue.equals("1") || this.mValue.toLowerCase().equals("true") || this.mValue.toLowerCase().equals("on");
        self.mValue == "1" || self.mValue.to_lowercase() == "true" || self.mValue.to_lowercase() == "on"
    }

    // Java: float toFloat()
    pub fn to_float(&self) -> f32 {
        // Java: try
        // Java: return Float.parseFloat(this.mValue);
        match self.mValue.parse::<f32>() {
            Ok(v) => v,
            // Java: catch (NumberFormatException e)
            Err(_) => {
                // Java: if ("INF".equals(this.mValue))
                if self.mValue == "INF" {
                    // Java: return Float.POSITIVE_INFINITY;
                    f32::INFINITY
                }
                // Java: else if ("-INF".equals(this.mValue))
                else if self.mValue == "-INF" {
                    // Java: return Float.NEGATIVE_INFINITY;
                    f32::NEG_INFINITY
                }
                // Java: else if ("indefinite".equals(this.mValue))
                else if self.mValue == "indefinite" {
                    // Java: return Float.POSITIVE_INFINITY;
                    f32::INFINITY
                }
                // Java: else
                else {
                    // Java: e.printStackTrace();
                    // Java: return 0.0F;
                    0.0f32
                }
            }
        }
        // Java: catch (Throwable var3)
        // Java: return 0.0F;
    }

    // Java: double toDouble()
    pub fn to_double(&self) -> f64 {
        // Java: try
        // Java: return Double.parseDouble(this.mValue);
        match self.mValue.parse::<f64>() {
            Ok(v) => v,
            // Java: catch (NumberFormatException e)
            Err(_) => {
                // Java: if ("INF".equals(this.mValue))
                if self.mValue == "INF" {
                    // Java: return Double.POSITIVE_INFINITY;
                    f64::INFINITY
                }
                // Java: else if ("-INF".equals(this.mValue))
                else if self.mValue == "-INF" {
                    // Java: return Double.NEGATIVE_INFINITY;
                    f64::NEG_INFINITY
                }
                // Java: else if ("indefinite".equals(this.mValue))
                else if self.mValue == "indefinite" {
                    // Java: return Double.POSITIVE_INFINITY;
                    f64::INFINITY
                }
                // Java: else
                else {
                    // Java: e.printStackTrace();
                    // Java: return (double)0.0F;
                    0.0f64
                }
            }
        }
    }

    // Java: long toLong()
    pub fn to_long(&self) -> i64 {
        // Java: try
        // Java: return Long.parseLong(this.mValue);
        match self.mValue.parse::<i64>() {
            Ok(v) => v,
            // Java: catch (NumberFormatException e)
            Err(_) => {
                // Java: if ("INF".equals(this.mValue))
                if self.mValue == "INF" {
                    // Java: return Long.MAX_VALUE;
                    i64::MAX
                }
                // Java: else if ("-INF".equals(this.mValue))
                else if self.mValue == "-INF" {
                    // Java: return Long.MIN_VALUE;
                    i64::MIN
                }
                // Java: else if ("indefinite".equals(this.mValue))
                else if self.mValue == "indefinite" {
                    // Java: return Long.MAX_VALUE;
                    i64::MAX
                }
                // Java: else
                else {
                    // Java: e.printStackTrace();
                    // Java: return 0L;
                    0i64
                }
            }
        }
    }

    // Java: double toPercentage()
    pub fn to_percentage(&self) -> f64 {
        // Java: if (this.mValue.endsWith("%"))
        if self.mValue.ends_with("%") {
            // Java: String tmpstr = this.mValue.substring(0, this.mValue.lastIndexOf("%"));
            let tmpstr = &self.mValue[..self.mValue.rfind('%').unwrap_or(0)];
            // Java: return Double.parseDouble(tmpstr) / (double)100.0F;
            return tmpstr.parse::<f64>().unwrap_or(0.0) / 100.0;
        }
        // Java: else
        // Java: Integer tmp = TypeConversion.string2Integer(this.mValue);
        let tmp = self.mValue.parse::<i32>().unwrap_or(0);
        // Java: tmp = null == tmp ? 0 : tmp;
        // Java: return (double)tmp / (double)100000.0F;
        tmp as f64 / 100000.0
    }

    // Java: XmlRoAttr getByName(int name)
    pub fn get_by_name(&self, _name: i32) -> ! {
        // Java: throw new RuntimeException("Not implemented");
        panic!("Not implemented");
    }

    // Java: XmlRoAttr getAttrAt(int index)
    pub fn get_attr_at(&self, _index: i32) -> ! {
        // Java: throw new RuntimeException("Not implemented");
        panic!("Not implemented");
    }

    // Java: int getNameAt(int index)
    pub fn get_name_at(&self, _index: i32) -> i32 {
        // Java: throw new RuntimeException("Not implemented");
        panic!("Not implemented");
    }

    // Java: int count()
    pub fn count(&self) -> i32 {
        // Java: throw new RuntimeException("Not implemented");
        panic!("Not implemented");
    }

    // Java: String getCharacters()
    pub fn get_characters(&self) -> String {
        // Java: throw new RuntimeException("Not implemented");
        panic!("Not implemented");
    }

    // Java: int getName()
    pub fn get_name(&self) -> i32 {
        // Java: return this.mName;
        self.mName
    }

    // Java: XmlRoAttr getByName(int name, boolean bLeaf)
    pub fn get_by_name_with_flag(&self, _name: i32, _b_leaf: bool) -> ! {
        // Java: throw new RuntimeException("Not implemented");
        panic!("Not implemented");
    }

    // Java: boolean isLeaf()
    pub fn is_leaf(&self) -> bool {
        // Java: return true;
        true
    }
}

impl super::XmlRoAttrValue::XmlRoAttrValue for XmlRoAttrLeaf {
    fn to_string(&self) -> String { XmlRoAttrLeaf::to_string(self) }
    fn to_int(&self) -> i32 { XmlRoAttrLeaf::to_int(self) }
    fn to_byte(&self) -> i8 { XmlRoAttrLeaf::to_byte(self) }
    fn to_short(&self) -> i16 { XmlRoAttrLeaf::to_short(self) }
    fn to_boolean(&self) -> bool { XmlRoAttrLeaf::to_boolean(self) }
    fn to_float(&self) -> f32 { XmlRoAttrLeaf::to_float(self) }
    fn to_double(&self) -> f64 { XmlRoAttrLeaf::to_double(self) }
    fn to_long(&self) -> i64 { XmlRoAttrLeaf::to_long(self) }
    fn to_percentage(&self) -> f64 { XmlRoAttrLeaf::to_percentage(self) }
}

impl super::XmlRoAttr::XmlRoAttr for XmlRoAttrLeaf {
    fn get_by_name(&self, _name: i32) -> Option<&dyn super::XmlRoAttr::XmlRoAttr> {
        panic!("Not implemented");
    }
    fn get_attr_at(&self, _index: i32) -> Option<&dyn super::XmlRoAttr::XmlRoAttr> {
        panic!("Not implemented");
    }
    fn get_name_at(&self, index: i32) -> i32 { XmlRoAttrLeaf::get_name_at(self, index) }
    fn count(&self) -> i32 { XmlRoAttrLeaf::count(self) }
    fn get_characters(&self) -> String { XmlRoAttrLeaf::get_characters(self) }
    fn get_name(&self) -> i32 { XmlRoAttrLeaf::get_name(self) }
    fn get_by_name_with_flag(&self, _name: i32, _flag: bool) -> Option<&dyn super::XmlRoAttr::XmlRoAttr> {
        panic!("Not implemented");
    }
    fn is_leaf(&self) -> bool { XmlRoAttrLeaf::is_leaf(self) }
}
