// java package cn.wps.moss.xlsxr.handler.sheet;
// java
// java import java.util.HashMap;
// java
// java class ActivePaneMap {
// java private static HashMap<String, Byte> paneMap = new HashMap(5);
// java
// java public static byte getPane(String value) {
// java return value == null ? 2 : (Byte)paneMap.get(value);
// java }
// java
// java static {
// java paneMap.put("bottomLeft", (byte)2);
// java paneMap.put("bottomRight", (byte)0);
// java paneMap.put("topLeft", (byte)3);
// java paneMap.put("topRight", (byte)1);
// java }
// java }
pub struct ActivePaneMap;impl ActivePaneMap{pub fn getPane(value:Option<&str>)->i8{match value{None=>2,Some("bottomLeft")=>2,Some("bottomRight")=>0,Some("topLeft")=>3,Some("topRight")=>1,_=>panic!("NullPointerException ActivePaneMap")}}}
