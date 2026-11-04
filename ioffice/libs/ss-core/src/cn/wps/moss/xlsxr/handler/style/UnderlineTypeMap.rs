// java package cn.wps.moss.xlsxr.handler.style;
// java
// java import java.util.HashMap;
// java
// java class UnderlineTypeMap {
// java    private static HashMap<String, Byte> underlineMap = new HashMap(5);
// java
// java    public static byte getUnderline(String value) {
// java       return value != null && underlineMap.containsKey(value) ? (Byte)underlineMap.get(value) : 1;
// java    }
// java
// java    static {
// java       underlineMap.put("none", new Byte((byte)0));
// java       underlineMap.put("single", new Byte((byte)1));
// java       underlineMap.put("double", new Byte((byte)2));
// java       underlineMap.put("doubleAccounting", new Byte((byte)34));
// java       underlineMap.put("singleAccounting", new Byte((byte)33));
// java    }
// java }
// java
pub struct UnderlineTypeMap;
impl UnderlineTypeMap {
    pub fn getUnderline(value: Option<&str>) -> i8 {
        match value.unwrap_or("") {
            "none" => 0,
            "single" => 1,
            "double" => 2,
            "doubleAccounting" => 34,
            "singleAccounting" => 33,
            _ => 1,
        }
    }
}
