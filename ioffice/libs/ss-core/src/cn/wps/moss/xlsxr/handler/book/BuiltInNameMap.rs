use std::collections::HashMap;
use std::sync::LazyLock;
// java public class BuiltInNameMap {
pub struct BuiltInNameMap;
// java private static HashMap<String, Byte> builtInMap = new HashMap(8);
static builtInMap:LazyLock<HashMap<&'static str,i8>>=LazyLock::new(||{
    let mut map=HashMap::with_capacity(8);
    // java builtInMap.put("_xlnm._FilterDatabase", (byte)13);
    map.insert("_xlnm._FilterDatabase",13);
    // java builtInMap.put("_xlnm.Print_Area", (byte)6);
    map.insert("_xlnm.Print_Area",6);
    // java builtInMap.put("_xlnm.Print_Titles", (byte)7);
    map.insert("_xlnm.Print_Titles",7);
    // java builtInMap.put("_xlnm.Extract", (byte)3);
    map.insert("_xlnm.Extract",3);
    // java builtInMap.put("_xlnm.Criteria", (byte)5);
    map.insert("_xlnm.Criteria",5);
    // java builtInMap.put("_xlnm.Consolidate_Area", (byte)0);
    map.insert("_xlnm.Consolidate_Area",0);
    // java builtInMap.put("_xlnm.Database", (byte)4);
    map.insert("_xlnm.Database",4);
    // java builtInMap.put("_xlnm.Sheet_Title", (byte)12);
    map.insert("_xlnm.Sheet_Title",12);
    // java builtInMap.put("_FilterDatabase", (byte)13);
    map.insert("_FilterDatabase",13);
    // java builtInMap.put("Print_Area", (byte)6);
    map.insert("Print_Area",6);
    // java builtInMap.put("Print_Titles", (byte)7);
    map.insert("Print_Titles",7);
    // java builtInMap.put("Extract", (byte)3);
    map.insert("Extract",3);
    // java builtInMap.put("Criteria", (byte)5);
    map.insert("Criteria",5);
    // java builtInMap.put("Consolidate_Area", (byte)0);
    map.insert("Consolidate_Area",0);
    // java builtInMap.put("Database", (byte)4);
    map.insert("Database",4);
    // java builtInMap.put("Sheet_Title", (byte)12);
    map.insert("Sheet_Title",12);
    map
});
impl BuiltInNameMap{
    // java public static byte getBuiltIn(String value) {return (Byte)builtInMap.get(value);}
    pub fn getBuiltIn(value:Option<&str>)->i8{*value.and_then(|s|builtInMap.get(s)).expect("NullPointerException")}
    // java public static boolean hasBuiltIn(String value) {return builtInMap.containsKey(value);}
    pub fn hasBuiltIn(value:Option<&str>)->bool{value.is_some_and(|s|builtInMap.contains_key(s))}
}
