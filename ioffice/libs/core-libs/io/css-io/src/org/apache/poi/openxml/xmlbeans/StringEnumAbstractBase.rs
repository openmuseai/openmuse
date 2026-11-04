// Java: import cn.wps.base.assertion.Assert
// Java: import java.util.HashMap
// Java: import java.util.Map
use std::collections::HashMap;

// Java: public class StringEnumAbstractBase
#[derive(Clone, Debug)]
pub struct StringEnumAbstractBase {
    // Java field: String str
    pub(crate) str_val: String,
    // Java field: int integer
    pub(crate) integer: i32,
}

// Java: public class Table
pub struct Table {
    // Java field: Map<String, StringEnumAbstractBase> mMapString
    pub(crate) mMapString: HashMap<String, StringEnumAbstractBase>,
}

// Java: public class StringEnumAbstractBase
impl StringEnumAbstractBase {
    // Java: StringEnumAbstractBase(String s, int i)
    pub fn new(s: &str, i: i32) -> Self {
        // Java: super();
        // Java: this.str = toLowerCase(s);
        let lower = Self::toLowerCase(s);
        // Java: this.integer = i;
        StringEnumAbstractBase { str_val: lower, integer: i }
    }

    // Java: String toString()
    pub fn toString(&self) -> &str {
        // Java: return this.str;
        &self.str_val
    }

    // Java: int intValue()
    pub fn intValue(&self) -> i32 {
        // Java: return this.integer;
        self.integer
    }

    // Java: static String toLowerCase(String value)
    fn toLowerCase(value: &str) -> String {
        // Java: Assert.assertNotNull("value should not be null!", value);
        // Java: return value.toLowerCase();
        value.to_lowercase()
    }
}

// Java: public class Table
impl Table {
    // Java: Table(StringEnumAbstractBase[] array)
    pub fn new(array: &[StringEnumAbstractBase]) -> Self {
        // Java: super();
        // Java: int len = array.length;
        let len = array.len();
        // Java: this.mMapString = new HashMap(len);
        let mut map = HashMap::with_capacity(len);
        // Java: for(int i = 0; i < len; ++i)
        for i in 0..len {
            // Java: this.mMapString.put(array[i].str, array[i]);
            map.insert(array[i].str_val.clone(), array[i].clone());
        }
        Table { mMapString: map }
    }

    // Java: StringEnumAbstractBase forString(String s)
    pub fn forString(&self, s: &str) -> Option<&StringEnumAbstractBase> {
        // Java: return (StringEnumAbstractBase)this.mMapString.get(StringEnumAbstractBase.toLowerCase(s));
        let lower = s.to_lowercase();
        self.mMapString.get(&lower)
    }
}
