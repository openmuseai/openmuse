#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_ExtTextOut extends WMFRecord
pub struct WMF_ExtTextOut {
    // Java field: String TAG
    // Java field: int x
    x: i32,
    // Java field: int y
    y: i32,
    // Java field: int options
    options: i32,
    // Java field: Rectangle rcText
    rcText: Option<i32>,
    // Java field: byte[] textByte
    textByte: Option<Vec<u8>>,
    // Java field: float[] widths
    widths: Option<Vec<f32>>,
    // Java field: float[] dstWidths
    dstWidths: Option<Vec<f32>>,
    // Java field: String text
    text: Option<String>,
}

pub(crate) const TAG: &str = "";

impl WMF_ExtTextOut {
    // Java: WMF_ExtTextOut()
    pub fn new() -> Self {
        // Java: super();
        WMF_ExtTextOut {
            x: 0,
            y: 0,
            options: 0,
            rcText: None,
            textByte: None,
            widths: None,
            dstWidths: None,
            text: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: int finalPos = wmf.getCurrentPos() + len;
        // Java: WMF_ExtTextOut tag = new WMF_ExtTextOut();
        // Java: tag.y = wmf.readShort();
        // Java: tag.x = wmf.readShort();
        // Java: int stringLength = wmf.readUnsignedShort();
        // Java: tag.options = wmf.readUnsignedShort();
        // Java: if ((tag.options & WMF_ExtTextOutOptions.ETO_CLIPPED) != 0 || (tag.options & WMF_ExtTextOutOptions.ETO_OPAQUE) != 0)
        {
        // Java: wmf.skip(8);
        }
        // Java: tag.textByte = wmf.readByte(stringLength);
        // Java: if ((stringLength & 1) == 1)
        {
        // Java: wmf.skip(1);
        }
        // Java: if (wmf.getCurrentPos() < finalPos)
        {
        // Java: tag.widths = new float[stringLength];
        // Java: for(int i = 0; i < stringLength; ++i)
        {
        // Java: int space = wmf.readUnsignedShort();
        // Java: tag.widths[i] = (float)space;
        }
        }
        // Java: wmf.setCurrentPos(finalPos);
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: this.parserTextAndWidth(renderer.getDC().getFont(), renderer.getAffectCharsetSet());
        // Java: renderer.drawTextW(this.text, this.x, this.y, this.dstWidths);
    }

    // Java: private void parserTextAndWidth(GDIFont gdiFont, Set<String> affectCharsetSet)
    pub fn parserTextAndWidth(&self, _gdiFont: i32) {
        // Java: if (null == this.text && this.textByte.length != 0 && gdiFont != null)
        {
        // Java: this.text = readWmfString(this.textByte, gdiFont, affectCharsetSet);
        // Java: int textLen = this.text.length();
        // Java: if (this.widths != null)
        {
        // Java: if (this.widths.length == textLen)
        {
        // Java: this.dstWidths = this.widths;
        }
        // Java: else if (this.widths.length == textLen * 2)
        {
        // Java: this.dstWidths = new float[textLen];
        // Java: for(int i = 0; i < textLen; ++i)
        {
        // Java: this.dstWidths[i] = this.widths[i * 2] + this.widths[i * 2 + 1];
        }
        }
        // Java: else if (this.widths.length > textLen && this.widths.length < textLen * 2)
        {
        // Java: this.dstWidths = new float[textLen];
        // Java: int widthsLen = this.widths.length;
        // Java: int textIndex = 0;
        // Java: for(int i = 0; i < widthsLen; ++i)
        {
        // Java: if (this.widths[i] > 0.0F)
        {
        // Java: this.dstWidths[textIndex] = this.widths[i];
        // Java: ++textIndex;
        // Java: if (textIndex == textLen)
        {
        // Java: break;
        }
        }
        }
        }
        }
        }
    }

    // Java: private static boolean isGB2312(byte[] input)
    pub fn isGB2312(_input: i32) -> bool {
        // Java: if (input.length >= 2 && input.length % 2 == 0)
        {
        // Java: int h = input[0] & 255;
        // Java: int l = input[1] & 255;
        // Java: return h >= 176 && h <= 247 && l >= 161 && l <= 254;
        }
        // Java: else
        {
        // Java: return false;
        }
        false
    }

    // Java: private static boolean isGBK(byte[] input)
    pub fn isGBK(_input: i32) -> bool {
        // Java: if (input.length >= 2 && input.length % 2 == 0)
        {
        // Java: int h = input[0] & 255;
        // Java: int l = input[1] & 255;
        // Java: return h >= 129 && h <= 254 && l >= 64 && l <= 254;
        }
        // Java: else
        {
        // Java: return false;
        }
        false
    }

    // Java: private static String adjustCharset(String oriCharset, byte[] bytes)
    pub fn adjustCharset(_oriCharset: i32, _bytes: i32) -> String {
        // Java: String adjustCharset = oriCharset;
        // Java: if (oriCharset == "ISO-8859-1")
        {
        // Java: if (isGB2312(bytes))
        {
        // Java: adjustCharset = "GB2312";
        }
        // Java: else if (isGBK(bytes))
        {
        // Java: adjustCharset = "GBK";
        }
        }
        // Java: return adjustCharset;
        String::new()
    }

    // Java: public static String readWmfString(byte[] bytes, GDIFont gdiFont, Set<String> affectCharsetSet)
    pub fn readWmfString(_bytes: i32, _gdiFont: i32) -> String {
        // Java: String text = "";
        // Java: String charset = gdiFont.charset;
        // Java: String familyName = gdiFont.familyName;
        // Java: if (charset.equals("DEFAULT"))
        {
        // Java: charset = CharsetUtils.guessCharset(affectCharsetSet, WMF_DibStretchBlt.class.getSimpleName());
        }
        // Java: if (!charset.equals("SYMBOL") && !familyName.equals("Symbol"))
        {
        // Java: if (needRevise(bytes))
        {
        // Java: text = revise(bytes);
        }
        // Java: else if (charset.length() > 0)
        {
        // Java: try
        {
        // Java: if (!SymbolConverter.isSymbolFont(familyName))
        {
        // Java: charset = adjustCharset(charset, bytes);
        }
        // Java: text = new String(bytes, charset);
        }
        // Java: catch (UnsupportedEncodingException var8)
        {
        }
        }
        // Java: else
        {
        // Java: text = new String(bytes);
        }
        }
        // Java: else
        {
        // Java: int i = 0;
        // Java: for(int len = bytes.length; i < len; ++i)
        {
        // Java: text = text + (char)(bytes[i] & 255);
        }
        }
        // Java: return text;
        String::new()
    }

    // Java: private static String revise(byte[] text)
    pub fn revise(_text: i32) -> String {
        // Java: String revise = "";
        // Java: if (text.length == 1 && text[0] == -120)
        {
        // Java: revise = revise + 'ˆ';
        }
        // Java: return revise;
        String::new()
    }

    // Java: private static boolean needRevise(byte[] text)
    pub fn needRevise(_text: i32) -> bool {
        // Java: boolean bRevise = false;
        // Java: if (text.length == 1 && text[0] == -120)
        {
        // Java: bRevise = true;
        }
        // Java: return bRevise;
        false
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString() + "(" + this.text + ", " + this.x + ", " + this.y + ", " + (null == this.dstWidths ? "null" : Arrays.toString(this.dstWidths)) + ")";
        String::new()
    }

}
