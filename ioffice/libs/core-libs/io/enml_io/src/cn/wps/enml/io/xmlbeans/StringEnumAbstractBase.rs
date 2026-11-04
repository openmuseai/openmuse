#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Map

// Java: public class StringEnumAbstractBase
pub struct StringEnumAbstractBase {
    // Java field: String mStr
    pub(crate) mStr: String,
    // Java field: int mInt
    pub(crate) mInt: i32,
}

// Java: public class Table
pub struct Table {
    // Java field: Map<String, StringEnumAbstractBase> mMapString
    pub(crate) mMapString: HashMap<String, StringEnumAbstractBase>,
}

// Java: public class StringEnumAbstractBase
impl StringEnumAbstractBase {
    // Java: StringEnumAbstractBase(String s, int i)
    pub fn new(s: String, i: i32) -> Self {
      // Java: super();
      // Java: this.mStr = s;
      // Java: this.mInt = i;
      StringEnumAbstractBase {
          mStr: s,
          mInt: i,
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> &str {
      // Java: return this.mStr;
      &self.mStr
    }

    // Java: int intValue()
    pub fn intValue(&self) -> i32 {
      // Java: return this.mInt;
      self.mInt
    }
}

// Java: public class Table
impl Table {
    // Java: Table(StringEnumAbstractBase[] array)
    pub fn new(array: Vec<StringEnumAbstractBase>) -> Self {
         // Java: super();
         // Java: this.mMapString = new HashMap(array.length);
         let mut mMapString = HashMap::with_capacity(array.len());
         // Java: for(int i = 0; i < array.length; ++i)
         for item in array {
            // Java: this.mMapString.put(array[i].mStr, array[i]);
            let key = item.mStr.clone();
            mMapString.insert(key, item);
         }
         Table { mMapString }
    }

    // Java: StringEnumAbstractBase forString(String s)
    pub fn forString(&self, s: &str) -> Option<&StringEnumAbstractBase> {
         // Java: return (StringEnumAbstractBase)this.mMapString.get(s);
         self.mMapString.get(s)
    }
}
