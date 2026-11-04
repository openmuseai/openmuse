// Java: import android.graphics.Bitmap
// (not available in Rust)
// Java: import android.graphics.Matrix
// (not available in Rust)
// Java: import android.graphics.Paint
// (not available in Rust)
// Java: import android.graphics.Path
// (not available in Rust)
// Java: import android.graphics.Rect
// (not available in Rust)
// Java: import android.graphics.RectF
// (not available in Rust)
// Java: import android.graphics.Typeface
// (not available in Rust)
// Java: import android.graphics.Paint.Style
// (not available in Rust)
// Java: import android.graphics.Path.FillType
// (not available in Rust)
// Java: import android.os.Build.VERSION
// (not available in Rust)
// Java: import cn.wps.font.FreeTypeJNI
// (not available in this crate)
// Java: import cn.wps.font.IFontFamily
// (not available in this crate)
// Java: import cn.wps.graphics.bitmap.IPicture
// (not available in this crate)
// Java: import java.nio.charset.Charset
// (Rust strings are UTF-8)
// Java: import java.util.Locale
// (mapped to format! macro)
use super::KStream::KStream;

// Java field: String Serif_Regular
#[allow(non_upper_case_globals)]
pub(crate) const Serif_Regular: &str = "/system/fonts/DroidSerif-Regular.ttf";
// Java field: String Sans
#[allow(non_upper_case_globals)]
pub(crate) const Sans: &str = "/system/fonts/DroidSans.ttf";
// Java field: String Sans_Fallback
#[allow(non_upper_case_globals)]
pub(crate) const Sans_Fallback: &str = "/system/fonts/DroidSansFallback.ttf";
// Java field: String SansHans_Regular
#[allow(non_upper_case_globals)]
pub(crate) const SansHans_Regular: &str = "/system/fonts/NotoSansHans-Regular.ttf";
// Java field: int[] sGlyph
#[allow(non_upper_case_globals)]
static sGlyph: std::sync::Mutex<[i32; 1]> = std::sync::Mutex::new([0]);
// Java field: int[] sUnicode
#[allow(non_upper_case_globals)]
static sUnicode: std::sync::Mutex<[i32; 1]> = std::sync::Mutex::new([0]);

const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";

// Java: public class KPdfUtils
pub struct KPdfUtils {
}

// Java: public class KPdfUtils
#[allow(non_snake_case)]
impl KPdfUtils {
    // Java: KPdfUtils()
    pub fn new() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static void appendRect(KStream content, RectF rect)
    pub fn appendRect_rectf(content: &mut KStream, left: f32, top: f32, right: f32, bottom_val: f32) {
      // Java: float bottom = Math.min(rect.bottom, rect.top);
      let bottom = f32::min(bottom_val, top);
      // Java: String text = toString(rect.left) + " " + toString(bottom) + " " + toString(rect.width()) + " " + toString(rect.height()) + " re\n";
      let width = (right - left).abs();
      let height = (bottom_val - top).abs();
      let text = format!("{} {} {} {} re\n", Self::float_to_string(left), Self::float_to_string(bottom), Self::float_to_string(width), Self::float_to_string(height));
      // Java: content.write(text);
      content.write_str(&text);
    }

    // Java: static void appendRect(KStream content, Rect rect)
    pub fn appendRect_rect(content: &mut KStream, left: i32, top: i32, right: i32, bottom_val: i32) {
      // Java: float bottom = (float)Math.min(rect.bottom, rect.top);
      let bottom = i32::min(bottom_val, top) as f32;
      // Java: String text = toString((float)rect.left) + " " + toString(bottom) + " " + toString((float)rect.width()) + " " + toString((float)rect.height()) + " re\n";
      let width = (right - left).abs() as f32;
      let height = (bottom_val - top).abs() as f32;
      let text = format!("{} {} {} {} re\n", Self::float_to_string(left as f32), Self::float_to_string(bottom), Self::float_to_string(width), Self::float_to_string(height));
      // Java: content.write(text);
      content.write_str(&text);
    }

    // Java: static void appendTransform(Matrix matrix, KStream stream)
    pub fn appendTransform(affs: &[f32; 6], stream: &mut KStream) {
      // Java: StringBuffer buffer = new StringBuffer();
      let mut buffer = String::new();
      // Java: float[] affs = asAffine(matrix);
      // Java: for(int i = 0; i < affs.length; ++i)
        for i in 0..affs.len()
      {
         // Java: buffer.append(toString(affs[i]) + " ");
         buffer.push_str(&Self::float_to_string(affs[i]));
         buffer.push(' ');
      }
      // Java: buffer.append("cm\n");
      buffer.push_str("cm\n");
      // Java: stream.write(buffer.toString());
      stream.write_str(&buffer);
    }

    // Java: static void paintPath(KStream content, Paint.Style style, Path.FillType fill)
    pub fn paintPath(content: &mut KStream, style: i32, fill: i32) {
      // Java: content.write(getPaintPath(style, fill));
      content.write_str(&Self::getPaintPath(style, fill));
    }

    // Java: static String getPaintPath(Paint.Style style, Path.FillType fill)
    pub fn getPaintPath(style: i32, fill: i32) -> String {
      // Java: String str = null;
      let mut s = String::new();
      // Java: if (style == Style.FILL)
        if style == 0
      {
         // Java: str = "f";
         s = "f".to_string();
      }
      // Java: else if (style == Style.FILL_AND_STROKE)
        else if style == 2
      {
         // Java: str = "B";
         s = "B".to_string();
      }
      // Java: else if (style == Style.STROKE)
        else if style == 1
      {
         // Java: str = "S";
         s = "S".to_string();
      }
      // Java: if (style != Style.STROKE && fill == FillType.EVEN_ODD)
        if style != 1 && fill == 1
      {
         // Java: str = str + "*";
         s.push('*');
      }
      // Java: str = str + "\n";
      s.push('\n');
      // Java: return str;
      s
    }

    // Java: static String getHex(int hex, int minDigits)
    pub fn getHex_int(mut hex: i32, mut min_digits: i32) -> String {
      // Java: StringBuffer buffer = new StringBuffer();
      let mut buffer = Vec::new();
      // Java: do
        loop
      {
         // Java: buffer.insert(0, "0123456789ABCDEF".charAt(hex & 15));
         buffer.insert(0, HEX_CHARS[(hex & 15) as usize] as char);
         // Java: hex >>= 4;
         hex >>= 4;
         // Java: --minDigits;
         min_digits -= 1;
         if hex == 0 { break; }
      }
      // Java: while(true)
        loop
      {
         // Java: --minDigits;
         min_digits -= 1;
         // Java: if (minDigits < 0)
            if min_digits < 0
         {
            // Java: return buffer.toString();
            return buffer.iter().collect();
         }
         // Java: buffer.insert(0, '0');
         buffer.insert(0, '0');
      }
    }

    // Java: static String getHex(char hex, int minDigits)
    pub fn getHex_char(mut hex: u16, mut min_digits: i32) -> String {
      // Java: StringBuffer buffer = new StringBuffer();
      let mut buffer = Vec::new();
      // Java: do
        loop
      {
         // Java: buffer.insert(0, "0123456789ABCDEF".charAt(hex & 15));
         buffer.insert(0, HEX_CHARS[(hex & 15) as usize] as char);
         // Java: hex = (char)(hex >> 4);
         hex >>= 4;
         // Java: --minDigits;
         min_digits -= 1;
         if hex == 0 { break; }
      }
      // Java: while(true)
        loop
      {
         // Java: --minDigits;
         min_digits -= 1;
         // Java: if (minDigits < 0)
            if min_digits < 0
         {
            // Java: return buffer.toString();
            return buffer.iter().collect();
         }
         // Java: buffer.insert(0, '0');
         buffer.insert(0, '0');
      }
    }

    // Java: static void emitPath(Path path, Paint.Style paintStyle, KStream stream)
    pub fn emitPath(_path: i32, _paint_style: i32, _stream: &mut KStream) {
    }

    // Java: static void drawFormXObject(int objectIndex, KStream stream)
    pub fn drawFormXObject(object_index: i32, stream: &mut KStream) {
      // Java: String text = String.format(Locale.ENGLISH, "/X%d Do\n", objectIndex);
      let text = format!("/X{} Do\n", object_index);
      // Java: stream.write(text);
      stream.write_str(&text);
    }

    // Java: static void applyGraphicState(int objectIndex, KStream stream)
    pub fn applyGraphicState(object_index: i32, stream: &mut KStream) {
      // Java: String text = String.format(Locale.ENGLISH, "/G%d gs\n", objectIndex);
      let text = format!("/G{} gs\n", object_index);
      // Java: stream.write(text);
      stream.write_str(&text);
    }

    // Java: static void appenCharGlyphIndex(int index, KStream stream)
    pub fn appenCharGlyphIndex_single(index: i32, stream: &mut KStream) {
      // Java: stream.write("<" + getHex((int)index, 4) + "> Tj\n");
      let text = format!("<{}> Tj\n", Self::getHex_int(index, 4));
      stream.write_str(&text);
    }

    // Java: static void appenCharGlyphIndex(int[] indexs, int start, int count, KStream stream)
    pub fn appenCharGlyphIndex_range(indexs: &[i32], start: i32, count: i32, stream: &mut KStream) {
      // Java: StringBuffer buffer = new StringBuffer();
      let mut buffer = String::new();
      // Java: buffer.append("<");
      buffer.push('<');
      // Java: boolean bIndex = false;
      let mut b_index = false;
      // Java: for(int i = start; i < start + count; ++i)
        for i in (start as usize)..((start + count) as usize)
      {
         // Java: if (indexs[i] > 0)
            if indexs[i] > 0
         {
            // Java: buffer.append(getHex((int)indexs[i], 4));
            buffer.push_str(&Self::getHex_int(indexs[i], 4));
            // Java: bIndex = true;
            b_index = true;
         }
      }
      // Java: buffer.append("> Tj\n");
      buffer.push_str("> Tj\n");
      // Java: if (bIndex)
        if b_index
      {
         // Java: stream.write(buffer.toString());
         stream.write_str(&buffer);
      }
    }

    // Java: static void moveTo(float x, float y, KStream stream)
    pub fn moveTo_stream(x: f32, y: f32, stream: &mut KStream) {
      // Java: String text = toString(x) + " " + toString(y) + " m\n";
      let text = format!("{} {} m\n", Self::float_to_string(x), Self::float_to_string(y));
      // Java: stream.write(text);
      stream.write_str(&text);
    }

    // Java: static void moveTo(float x, float y, StringBuffer buffer)
    pub fn moveTo_buf(x: f32, y: f32, buffer: &mut String) {
      // Java: buffer.append(toString(x) + " " + toString(y) + " m\n");
      buffer.push_str(&format!("{} {} m\n", Self::float_to_string(x), Self::float_to_string(y)));
    }

    // Java: static void closePath(StringBuffer buffer)
    pub fn closePath(buffer: &mut String) {
      // Java: buffer.append("h\n");
      buffer.push_str("h\n");
    }

    // Java: static void appendLine(float x, float y, KStream stream)
    pub fn appendLine_stream(x: f32, y: f32, stream: &mut KStream) {
      // Java: String text = toString(x) + " " + toString(y) + " l\n";
      let text = format!("{} {} l\n", Self::float_to_string(x), Self::float_to_string(y));
      // Java: stream.write(text);
      stream.write_str(&text);
    }

    // Java: static void appendLine(float x, float y, StringBuffer buffer)
    pub fn appendLine_buf(x: f32, y: f32, buffer: &mut String) {
      // Java: buffer.append(toString(x) + " " + toString(y) + " l\n");
      buffer.push_str(&format!("{} {} l\n", Self::float_to_string(x), Self::float_to_string(y)));
    }

    // Java: static void strokePath(KStream stream)
    pub fn strokePath(stream: &mut KStream) {
      // Java: paintPath(stream, Style.STROKE, FillType.WINDING);
      Self::paintPath(stream, 1, 0);
    }

    // Java: static void appendClose(StringBuffer buffer)
    pub fn appendClose(buffer: &mut String) {
      // Java: buffer.append("h\n");
      buffer.push_str("h\n");
    }

    // Java: static void emitPdfColor(int color, KStream stream)
    pub fn emitPdfColor(color: i32, stream: &mut KStream) {
      // Java: stream.write(getPdfColor(color));
      stream.write_str(&Self::getPdfColor(color));
    }

    // Java: static String getPdfColor(int color)
    pub fn getPdfColor(color: i32) -> String {
      // Java: return toString((float)(color >> 16 & 255) / 255.0F) + " " + toString((float)(color >> 8 & 255) / 255.0F) + " " + toString((float)(color & 255) / 255.0F) + " ";
      let r = ((color >> 16) & 255) as f32 / 255.0;
      let g = ((color >> 8) & 255) as f32 / 255.0;
      let b = (color & 255) as f32 / 255.0;
      format!("{} {} {} ", Self::float_to_string(r), Self::float_to_string(g), Self::float_to_string(b))
    }

    // Java: static float[] asAffine(Matrix matrix)
    pub fn asAffine(values: &[f32; 9]) -> [f32; 6] {
      // Java: float[] aff = new float[6];
      let mut aff = [0.0f32; 6];
      // Java: float[] values = new float[9];
      // Java: matrix.getValues(values);
      // Java: aff[0] = values[0];
      aff[0] = values[0];
      // Java: aff[1] = values[3];
      aff[1] = values[3];
      // Java: aff[2] = values[1];
      aff[2] = values[1];
      // Java: aff[3] = values[4];
      aff[3] = values[4];
      // Java: aff[4] = values[2];
      aff[4] = values[2];
      // Java: aff[5] = values[5];
      aff[5] = values[5];
      // Java: return aff;
      aff
    }

    // Java: static float[] asAffineIdentity()
    pub fn asAffineIdentity() -> [f32; 6] {
      // Java: float[] aff = new float[]{1.0F, 0.0F, 0.0F, 1.0F, 0.0F, 0.0F};
      let aff = [1.0f32, 0.0, 0.0, 1.0, 0.0, 0.0];
      // Java: return aff;
      aff
    }

    // Java: static byte[] getGBKBytes(String str)
    pub fn getGBKBytes(_s: &str) -> Vec<u8> {
      // Java: return str.getBytes(Charset.forName("GBK"));
      todo!("getGBKBytes requires GBK encoding support")
    }

    // Java: static String[] getFontPath(IFontFamily family, boolean isBold, boolean isItalic)
    pub fn getFontPath_with_style(_family_name: &str, _is_bold: bool, _is_italic: bool) -> Option<Vec<String>> {
      // Java: if (family == null)
        if false
      {
         // Java: return null;
         return None;
      }
      // Java: else
      {
         // Java: int style = 0;
         let mut _style = 0;
         // Java: if (isBold && isItalic)
            if _is_bold && _is_italic
         {
            // Java: style = 0;
            _style = 0;
         }
         // Java: else if (isBold)
            else if _is_bold
         {
            // Java: style = 1;
            _style = 1;
         }
         // Java: else if (isItalic)
            else if _is_italic
         {
            // Java: style = 2;
            _style = 2;
         }
         // Java: String[] paths = family.getFontFiles(style);
         // Java: if (paths != null && paths.length > 0)
         {
            // Java: return paths;
         }
         // Java: else
         {
            // Java: paths = family.getAllFontFiles();
            // Java: String path = null;
            let mut path: Option<String> = None;
            // Java: if (paths != null && paths.length > 0)
            {
               // Java: for(int i = 0; i < paths.length; ++i)
               {
                  // Java: if (paths[i] != null)
                  {
                     // Java: path = paths[i];
                     // Java: break;
                  }
               }
            }
            // Java: if (path == null)
                if path.is_none()
            {
               // Java: path = getSystemFontPath(family.getName());
               path = Self::getSystemFontPath(_family_name);
            }
            // Java: if (path != null)
                if let Some(p) = path
            {
               // Java: paths = new String[]{path};
               // Java: return paths;
               return Some(vec![p]);
            }
            // Java: else
            {
               // Java: return null;
               return None;
            }
         }
      }
    }

    // Java: static String[] getFontPath(IFontFamily family, Typeface typeface)
    pub fn getFontPath_with_typeface(_family_name: &str, _is_bold: bool, _is_italic: bool) -> Option<Vec<String>> {
      // Java: boolean isBold = typeface == null ? false : typeface.isBold();
      // Java: boolean isItalic = typeface == null ? false : typeface.isItalic();
      // Java: return getFontPath(family, isBold, isItalic);
      Self::getFontPath_with_style(_family_name, _is_bold, _is_italic)
    }

    // Java: static String getSystemFontPath(String familyName)
    pub fn getSystemFontPath(family_name: &str) -> Option<String> {
      // Java: if (familyName.equals("serif"))
        if family_name == "serif"
      {
         // Java: return VERSION.SDK_INT < 20 ? "/system/fonts/DroidSerif-Regular.ttf" : "/system/fonts/DroidSans.ttf";
         return Some("/system/fonts/DroidSans.ttf".to_string());
      }
      // Java: else if (familyName.equals("Droid Sans Fallback"))
        else if family_name == "Droid Sans Fallback"
      {
         // Java: return VERSION.SDK_INT < 22 ? "/system/fonts/DroidSansFallback.ttf" : "/system/fonts/NotoSansHans-Regular.ttf";
         return Some("/system/fonts/NotoSansHans-Regular.ttf".to_string());
      }
      // Java: else
      {
         // Java: return familyName.equals("Noto Sans SC") ? "/system/fonts/NotoSansHans-Regular.ttf" : null;
         return if family_name == "Noto Sans SC" { Some("/system/fonts/NotoSansHans-Regular.ttf".to_string()) } else { None };
      }
    }

    // Java: static boolean rejectBitmap(Bitmap bitmap)
    pub fn rejectBitmap(width: i32, height: i32) -> bool {
      // Java: return bitmap.getWidth() <= 0 || bitmap.getHeight() <= 0;
      width <= 0 || height <= 0
    }

    // Java: static boolean rejectPicture(IPicture picture)
    pub fn rejectPicture(width: i32, height: i32) -> bool {
      // Java: return picture.getWidth() <= 0 || picture.getHeight() <= 0;
      width <= 0 || height <= 0
    }

    // Java: static String toString(float val)
    pub fn float_to_string(val: f32) -> String {
      // Java: int intVal = (int)val;
      let int_val = val as i32;
      // Java: if (Math.abs((float)intVal - val) < 0.01F)
        if (int_val as f32 - val).abs() < 0.01
      {
         // Java: return Integer.toString(intVal);
         return int_val.to_string();
      }
      // Java: else
      {
         // Java: String text = Float.toString(val);
         let mut text = format!("{}", val);
         // Java: int len = text.length();
         let len = text.len();
         // Java: if (len > 7)
            if len > 7
         {
            // Java: int index = text.lastIndexOf(46);
            if let Some(index) = text.rfind('.')
            // Java: if (index > 0 && len - index > 6)
            { if index > 0 && len - index > 6
            {
               // Java: text = text.substring(0, index + 5);
               text = text[..index + 5].to_string();
            }}
         }
         // Java: return text;
         return text;
      }
    }

    // Java: static int getGlyphIndex(int text, String path, int index)
    pub fn getGlyphIndex_with_path(_text: i32, _path: &str, _index: i32) -> i32 {
      // Java: sUnicode[0] = text;
      // Java: long tf = KPdfTypeface.getTypeface(path, index);
      // Java: FreeTypeJNI.getGlyphIndexes(tf, sUnicode, 0, 1, sGlyph, 0);
      // Java: return sGlyph[0];
      todo!("getGlyphIndex requires FreeTypeJNI")
    }

    // Java: static int getGlyphIndex(int text, long tf)
    pub fn getGlyphIndex_with_tf(_text: i32, _tf: i64) -> i32 {
      // Java: sUnicode[0] = text;
      // Java: FreeTypeJNI.getGlyphIndexes(tf, sUnicode, 0, 1, sGlyph, 0);
      // Java: return sGlyph[0];
      todo!("getGlyphIndex requires FreeTypeJNI")
    }

    // Java: static void clearStatic()
    pub fn clearStatic() {
      // Java: KPdfFont.clearFontsStream();
      // Java: KPdfFont.clearFonts();
      // Java: KPdfFont.clearUsedFont();
      // Java: KDynamicMemoryStream.clearBlock();
      super::KDynamicMemoryStream::KDynamicMemoryStream::clearBlock();
      // Java: KPdfGraphicsState.clearGraphicsState();
    }

    // Java: static int bytesLength(String str)
    pub fn bytesLength_str(s: &str) -> i32 {
      // Java: int strlen = str.length();
      let chars: Vec<char> = s.chars().collect();
      let strlen = chars.len();
      // Java: int utflen = 0;
      let mut utflen: i32 = 0;
      // Java: for(int i = 0; i < strlen; ++i)
        for i in 0..strlen
      {
         // Java: int c = str.charAt(i);
         let c = chars[i] as i32;
         // Java: if (c >= 1 && c <= 127)
            if c >= 1 && c <= 127
         {
            // Java: ++utflen;
            utflen += 1;
         }
         // Java: else if (c > 2047)
            else if c > 2047
         {
            // Java: utflen += 3;
            utflen += 3;
         }
         // Java: else
            else
         {
            // Java: utflen += 2;
            utflen += 2;
         }
      }
      // Java: return utflen;
      utflen
    }

    // Java: static int bytesLength(char[] chars)
    pub fn bytesLength_chars(chars: &[char]) -> i32 {
      // Java: int strlen = chars.length;
      let strlen = chars.len();
      // Java: int utflen = 0;
      let mut utflen: i32 = 0;
      // Java: for(int i = 0; i < strlen; ++i)
        for i in 0..strlen
      {
         // Java: int c = chars[i];
         let c = chars[i] as i32;
         // Java: if (c >= 1 && c <= 127)
            if c >= 1 && c <= 127
         {
            // Java: ++utflen;
            utflen += 1;
         }
         // Java: else if (c > 2047)
            else if c > 2047
         {
            // Java: utflen += 3;
            utflen += 3;
         }
         // Java: else
            else
         {
            // Java: utflen += 2;
            utflen += 2;
         }
      }
      // Java: return utflen;
      utflen
    }

    // Java: static int getBytes(String str, byte[] buf, int off)
    pub fn getBytes_str(s: &str, buf: &mut [u8], off: i32) -> i32 {
      // Java: return getBytes((String)str, 0, str.length(), buf, off);
      let len = s.chars().count() as i32;
      Self::getBytes_str_range(s, 0, len, buf, off)
    }

    // Java: static int getBytes(String str, int str_off, int str_len, byte[] buf, int off)
    pub fn getBytes_str_range(s: &str, str_off: i32, str_len: i32, buf: &mut [u8], off: i32) -> i32 {
      // Java: int str_limit = str_off + str_len;
      let str_limit = (str_off + str_len) as usize;
      // Java: int count = off;
      let mut count = off as usize;
      // Java: int i;
      let chars: Vec<char> = s.chars().collect();
      let mut i: usize;
      // Java: for(i = str_off; i < str_limit; ++i)
        i = str_off as usize;
        while i < str_limit
      {
         // Java: int c = str.charAt(i);
         let c = chars[i] as i32;
         // Java: if (c < 1 || c > 127)
            if c < 1 || c > 127
         {
            // Java: break;
            break;
         }
         // Java: buf[count++] = (byte)c;
         buf[count] = c as u8;
         count += 1;
         i += 1;
      }
      // Java: for(; i < str_limit; ++i)
        while i < str_limit
      {
         // Java: int c = str.charAt(i);
         let c = chars[i] as i32;
         // Java: if (c >= 1 && c <= 127)
            if c >= 1 && c <= 127
         {
            // Java: buf[count++] = (byte)c;
            buf[count] = c as u8;
            count += 1;
         }
         // Java: else if (c > 2047)
            else if c > 2047
         {
            // Java: buf[count++] = (byte)(224 | c >> 12 & 15);
            buf[count] = (224 | (c >> 12) & 15) as u8;
            count += 1;
            // Java: buf[count++] = (byte)(128 | c >> 6 & 63);
            buf[count] = (128 | (c >> 6) & 63) as u8;
            count += 1;
            // Java: buf[count++] = (byte)(128 | c >> 0 & 63);
            buf[count] = (128 | c & 63) as u8;
            count += 1;
         }
         // Java: else
            else
         {
            // Java: buf[count++] = (byte)(192 | c >> 6 & 31);
            buf[count] = (192 | (c >> 6) & 31) as u8;
            count += 1;
            // Java: buf[count++] = (byte)(128 | c >> 0 & 63);
            buf[count] = (128 | c & 63) as u8;
            count += 1;
         }
         i += 1;
      }
      // Java: return count - off;
      (count - off as usize) as i32
    }

    // Java: static int getBytes(char[] chars, byte[] buf, int off)
    pub fn getBytes_chars(chars: &[char], buf: &mut [u8], off: i32) -> i32 {
      // Java: return getBytes((char[])chars, 0, chars.length, buf, off);
      Self::getBytes_chars_range(chars, 0, chars.len() as i32, buf, off)
    }

    // Java: static int getBytes(char[] chars, int str_off, int str_len, byte[] buf, int off)
    pub fn getBytes_chars_range(chars: &[char], str_off: i32, str_len: i32, buf: &mut [u8], off: i32) -> i32 {
      // Java: int str_limit = str_off + str_len;
      let str_limit = (str_off + str_len) as usize;
      // Java: int count = off;
      let mut count = off as usize;
      // Java: int i;
      let mut i: usize;
      // Java: for(i = str_off; i < str_limit; ++i)
        i = str_off as usize;
        while i < str_limit
      {
         // Java: int c = chars[i];
         let c = chars[i] as i32;
         // Java: if (c < 1 || c > 127)
            if c < 1 || c > 127
         {
            // Java: break;
            break;
         }
         // Java: buf[count++] = (byte)c;
         buf[count] = c as u8;
         count += 1;
         i += 1;
      }
      // Java: for(; i < str_limit; ++i)
        while i < str_limit
      {
         // Java: int c = chars[i];
         let c = chars[i] as i32;
         // Java: if (c >= 1 && c <= 127)
            if c >= 1 && c <= 127
         {
            // Java: buf[count++] = (byte)c;
            buf[count] = c as u8;
            count += 1;
         }
         // Java: else if (c > 2047)
            else if c > 2047
         {
            // Java: buf[count++] = (byte)(224 | c >> 12 & 15);
            buf[count] = (224 | (c >> 12) & 15) as u8;
            count += 1;
            // Java: buf[count++] = (byte)(128 | c >> 6 & 63);
            buf[count] = (128 | (c >> 6) & 63) as u8;
            count += 1;
            // Java: buf[count++] = (byte)(128 | c >> 0 & 63);
            buf[count] = (128 | c & 63) as u8;
            count += 1;
         }
         // Java: else
            else
         {
            // Java: buf[count++] = (byte)(192 | c >> 6 & 31);
            buf[count] = (192 | (c >> 6) & 31) as u8;
            count += 1;
            // Java: buf[count++] = (byte)(128 | c >> 0 & 63);
            buf[count] = (128 | c & 63) as u8;
            count += 1;
         }
         i += 1;
      }
      // Java: return count - off;
      (count - off as usize) as i32
    }

    // Java: static void appendCubic(float c1x, float c1y, float c2x, float c2y, float dx, float dy, StringBuffer buffer)
    pub fn appendCubic(c1x: f32, c1y: f32, c2x: f32, c2y: f32, dx: f32, dy: f32, buffer: &mut String) {
      // Java: buffer.append(toString(c1x) + " " + toString(c1y) + " " + toString(c2x) + " " + toString(c2y) + " " + toString(dx) + " " + toString(dy) + " c\n");
      buffer.push_str(&format!("{} {} {} {} {} {} c\n",
          Self::float_to_string(c1x), Self::float_to_string(c1y),
          Self::float_to_string(c2x), Self::float_to_string(c2y),
          Self::float_to_string(dx), Self::float_to_string(dy)));
    }

    // Java: static void appendQuad(float cx, float cy, float dx, float dy, StringBuffer buffer)
    pub fn appendQuad(cx: f32, cy: f32, dx: f32, dy: f32, buffer: &mut String) {
      // Java: buffer.append(toString(cx) + " " + toString(cy) + " " + toString(dx) + " " + toString(dy) + " y\n");
      buffer.push_str(&format!("{} {} {} {} y\n",
          Self::float_to_string(cx), Self::float_to_string(cy),
          Self::float_to_string(dx), Self::float_to_string(dy)));
    }

    // Java: static KPdfArray matrixToArray(Matrix matrix)
    pub fn matrixToArray(_values: &[f32; 9]) -> i32 {
      // Java: float[] vals = asAffine(matrix);
      // Java: KPdfArray pdfArray = new KPdfArray();
      // Java: for(int i = 0; i < vals.length; ++i)
      {
         // Java: pdfArray.appendFloat(vals[i]);
      }
      // Java: return pdfArray;
      todo!("matrixToArray requires KPdfArray")
    }

    // Java: static boolean isInvisibleChar(int c)
    pub fn isInvisibleChar(c: i32) -> bool {
      // Java: return c == 8206 || c == 8207;
      c == 8206 || c == 8207
    }

}
