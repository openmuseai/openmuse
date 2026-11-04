// java package cn.wps.moss.xlsxr.handler.style;
// java
// java import java.util.HashMap;
// java
// java class PatternTypeMap {
// java    private static HashMap<String, Short> typeMap = new HashMap(19);
// java
// java    public static short getType(String value) {
// java       return null == typeMap.get(value) ? 0 : (Short)typeMap.get(value);
// java    }
// java
// java    static {
// java       typeMap.put("none", Short.valueOf((short)0));
// java       typeMap.put("solid", Short.valueOf((short)1));
// java       typeMap.put("mediumGray", Short.valueOf((short)2));
// java       typeMap.put("darkGray", Short.valueOf((short)3));
// java       typeMap.put("lightGray", Short.valueOf((short)4));
// java       typeMap.put("darkHorizontal", Short.valueOf((short)5));
// java       typeMap.put("darkVertical", Short.valueOf((short)6));
// java       typeMap.put("darkDown", Short.valueOf((short)7));
// java       typeMap.put("darkUp", Short.valueOf((short)8));
// java       typeMap.put("darkGrid", Short.valueOf((short)9));
// java       typeMap.put("darkTrellis", Short.valueOf((short)10));
// java       typeMap.put("lightHorizontal", Short.valueOf((short)11));
// java       typeMap.put("lightVertical", Short.valueOf((short)12));
// java       typeMap.put("lightDown", Short.valueOf((short)13));
// java       typeMap.put("lightUp", Short.valueOf((short)14));
// java       typeMap.put("lightGrid", Short.valueOf((short)15));
// java       typeMap.put("lightTrellis", Short.valueOf((short)16));
// java       typeMap.put("gray125", Short.valueOf((short)17));
// java       typeMap.put("gray0625", Short.valueOf((short)18));
// java    }
// java }
// java
pub struct PatternTypeMap;
impl PatternTypeMap {
    pub fn getType(value: &str) -> i16 {
        match value {
            "none" => 0,
            "solid" => 1,
            "mediumGray" => 2,
            "darkGray" => 3,
            "lightGray" => 4,
            "darkHorizontal" => 5,
            "darkVertical" => 6,
            "darkDown" => 7,
            "darkUp" => 8,
            "darkGrid" => 9,
            "darkTrellis" => 10,
            "lightHorizontal" => 11,
            "lightVertical" => 12,
            "lightDown" => 13,
            "lightUp" => 14,
            "lightGrid" => 15,
            "lightTrellis" => 16,
            "gray125" => 17,
            "gray0625" => 18,
            _ => 0,
        }
    }
}
