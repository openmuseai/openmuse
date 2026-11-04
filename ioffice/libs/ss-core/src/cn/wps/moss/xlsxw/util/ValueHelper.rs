// java public final class ValueHelper {
// Method slice only: drawing anchor conversion and other overloads remain separate.
pub struct ValueHelper;
impl ValueHelper{
    // java private static final char[] HEXS = new char[]{'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'};
    const HEXS:[char;16]=['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'];
    // java public static String toString(int value) { String ret = TypeConversion.integer2String(value); return null != ret ? ret : ""; }
    pub fn toString(value:i32)->String{util::cn::wps::base::io::TypeConversion::TypeConversion::integer2String(value).unwrap_or_default()}
    // java public static String toHexColor(int value) {
    pub fn toHexColor(value:i32)->String{
        // java StringBuilder builder = new StringBuilder();
        let mut builder=String::new();
        // java int a = value >> 24 & 255;
        let a=value>>24&255;
        // java if (a > 0) {
        if a>0{
            // java int index = value >> 28 & 15; builder.append(HEXS[index]);
            let mut index=value>>28&15;builder.push(Self::HEXS[index as usize]);
            // java index = value >> 24 & 15; builder.append(HEXS[index]);
            index=value>>24&15;builder.push(Self::HEXS[index as usize]);
        }
        // java int index = value >> 20 & 15; builder.append(HEXS[index]);
        let mut index=value>>20&15;builder.push(Self::HEXS[index as usize]);
        // java index = value >> 16 & 15; builder.append(HEXS[index]);
        index=value>>16&15;builder.push(Self::HEXS[index as usize]);
        // java index = value >> 12 & 15; builder.append(HEXS[index]);
        index=value>>12&15;builder.push(Self::HEXS[index as usize]);
        // java index = value >> 8 & 15; builder.append(HEXS[index]);
        index=value>>8&15;builder.push(Self::HEXS[index as usize]);
        // java index = value >> 4 & 15; builder.append(HEXS[index]);
        index=value>>4&15;builder.push(Self::HEXS[index as usize]);
        // java index = value & 15; builder.append(HEXS[index]);
        index=value&15;builder.push(Self::HEXS[index as usize]);
        // java return builder.toString();
        builder
    }
}
