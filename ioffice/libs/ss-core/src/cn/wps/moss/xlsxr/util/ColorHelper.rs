use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use crate::cn::wps::moss::core::theme::{ThemeColorUtil::ThemeColorUtil,ColorScheme::ColorScheme};
// java public final class ColorHelper
pub struct ColorHelper;
impl ColorHelper {
// java public static void setPaletteColor(KmoPalette palette, int index, String hexValue) {
// java int len = hexValue.length();
// java int r = 0;
// java int g = 0;
// java int b = 0;
// java switch (len) {
// java case 1:
// java case 2:
// java b = ValueHelper.hexStringToInt(hexValue);
// java break;
// java case 3:
// java g = ValueHelper.hexStringToInt(hexValue.substring(0, 1));
// java b = ValueHelper.hexStringToInt(hexValue.substring(1, 3));
// java break;
// java case 4:
// java g = ValueHelper.hexStringToInt(hexValue.substring(0, 2));
// java b = ValueHelper.hexStringToInt(hexValue.substring(2, 4));
// java break;
// java case 5:
// java r = ValueHelper.hexStringToInt(hexValue.substring(0, 1));
// java g = ValueHelper.hexStringToInt(hexValue.substring(1, 3));
// java b = ValueHelper.hexStringToInt(hexValue.substring(3, 5));
// java break;
// java case 6:
// java r = ValueHelper.hexStringToInt(hexValue.substring(0, 2));
// java g = ValueHelper.hexStringToInt(hexValue.substring(2, 4));
// java b = ValueHelper.hexStringToInt(hexValue.substring(4, 6));
// java break;
// java case 7:
// java r = ValueHelper.hexStringToInt(hexValue.substring(1, 3));
// java g = ValueHelper.hexStringToInt(hexValue.substring(3, 5));
// java b = ValueHelper.hexStringToInt(hexValue.substring(5, 7));
// java break;
// java case 8:
// java r = ValueHelper.hexStringToInt(hexValue.substring(2, 4));
// java g = ValueHelper.hexStringToInt(hexValue.substring(4, 6));
// java b = ValueHelper.hexStringToInt(hexValue.substring(6, 8));
// java break;
// java default:
// java throw new IllegalStateException(hexValue);
// java }
// java
// java palette.setColorAtIndex((short)index, (byte)r, (byte)g, (byte)b);
// java }
pub fn setPaletteColor(palette:&crate::cn::wps::moss::core::book::KmoPalette::KmoPalette,index:i32,hexValue:&str) {
let u:Vec<u16>=hexValue.encode_utf16().collect();let hex=|a:usize,b:usize|i64::from_str_radix(String::from_utf16_lossy(&u[a..b]).trim(),16).map_or(0,|v|v as i32);let(mut red,mut green,mut blue)=(0,0,0);match u.len(){1|2=>blue=hex(0,u.len()),3=>{green=hex(0,1);blue=hex(1,3);},4=>{green=hex(0,2);blue=hex(2,4);},5=>{red=hex(0,1);green=hex(1,3);blue=hex(3,5);},6=>{red=hex(0,2);green=hex(2,4);blue=hex(4,6);},7=>{red=hex(1,3);green=hex(3,5);blue=hex(5,7);},8=>{red=hex(2,4);green=hex(4,6);blue=hex(6,8);},_=>panic!("IllegalStateException: {hexValue}")}palette.setColorAtIndex(index as i16,red as i8,green as i8,blue as i8);
}

    // java public static int getColor(XmlRoAttr xmlColor,int autoColor)
    pub fn getColor(a:&dyn XmlRoAttr,autoColor:i32)->i32 {
        let tint=||{let mut t=a.get_by_name(5157).map_or(0.0,|x|x.to_float());if t>1.0||t< -1.0{t/=100000.0;}t};
        // java if((tmp=xmlColor.getByName(5155))!=null) {ret=ValueHelper.colorToInt(tmp.toString(),16); ... ret=ThemeColorUtil.colorWithTint2RGB(r,g,b,tint);ret|=-16777216;}
        if let Some(v)=a.get_by_name(5155){let v=v.to_string();let color=ThemeColorUtil::colorToInt(Some(&v),16);return ThemeColorUtil::colorWithTint2RGB((color>>16)&255,(color>>8)&255,color&255,tint())| -16777216;}
        // java else if((tmp=xmlColor.getByName(5156))!=null) {ret=ColorScheme.getColor(theme,tint);ret|=-16777216;}
        if let Some(v)=a.get_by_name(5156){return ColorScheme::getColor(v.to_int(),tint())| -16777216;}
        // java else if((tmp=xmlColor.getByName(5154))!=null) ret=tmp.toInt();else if(xmlColor.getByName(5153)!=null) ret=autoColor;return ret;
        if let Some(v)=a.get_by_name(5154){v.to_int()}else if a.get_by_name(5153).is_some(){autoColor}else{0}
    }
}
