use crate::{java_runtime::javaCharacterIsDigit,org::apache::poi::ss::util::CellReference::CellReference};pub struct ValueHelper;impl ValueHelper{
// java public static int toInt(String value) {
// java if (null == value) {
// java return 0;
// java } else {
// java Integer tmp = TypeConversion.string2Integer(value);
// java int result = null == tmp ? 0 : tmp;
// java return result;
// java }
// java }
pub fn toInt(s:Option<&str>)->i32{s.and_then(util::cn::wps::base::io::TypeConversion::TypeConversion::string2Integer).unwrap_or(0)}
// java public static int hexStringToInt(String value) {
pub fn hexStringToInt(value:Option<&str>)->i32{
    // java if (null == value) { return 0; }
    if value.is_none(){0}else{
        // java Integer tmp = TypeConversion.hexString2Integer(value);
        let tmp=util::cn::wps::base::io::TypeConversion::TypeConversion::hexString2Integer(value.unwrap());
        // java int result = null == tmp ? 0 : tmp; return result;
        tmp.unwrap_or(0)
    }
}
// java public static boolean toBoolean(String value) {
pub fn toBoolean(value:Option<&str>)->bool{
    // java if (null == value) { return false; }
    let Some(value)=value else{return false;};
    // java else { return value.equals("1") || value.toLowerCase().equals("true") || value.toLowerCase().equals("on"); }
    // These two ASCII tokens contain no locale-sensitive I/J/S/K case mappings.
    value=="1"||value.to_lowercase()=="true"||value.to_lowercase()=="on"
}
// java public static int getColumnIndex(String pos) {
// java int loc = 0;
// java
// java for(int length = pos.length(); loc < length; ++loc) {
// java char ch = pos.charAt(loc);
// java if (Character.isDigit(ch)) {
// java break;
// java }
// java }
// java
// java return CellReference.convertColStringToIndex(pos.substring(0, loc));
// java }
pub fn getColumnIndex(s:&str)->i32{let units=s.encode_utf16().collect::<Vec<_>>();let loc=units.iter().position(|c|javaCharacterIsDigit(*c)).unwrap_or(units.len());CellReference::convertColStringToIndex(&String::from_utf16(&units[..loc]).expect("UTF16 cell column")).expect("column")}
// java public static int getRowIndex(String pos) {
// java int loc = 0;
// java
// java for(int length = pos.length(); loc < length; ++loc) {
// java char ch = pos.charAt(loc);
// java if (Character.isDigit(ch)) {
// java break;
// java }
// java }
// java
// java int ret = toInt(pos.substring(loc)) - 1;
// java ret = Math.max(ret, 0);
// java return ret;
// java }
pub fn getRowIndex(s:&str)->i32{let units=s.encode_utf16().collect::<Vec<_>>();let loc=units.iter().position(|c|javaCharacterIsDigit(*c)).unwrap_or(units.len());Self::toInt(Some(&String::from_utf16(&units[loc..]).expect("UTF16 cell row"))).wrapping_sub(1).max(0)}
}
