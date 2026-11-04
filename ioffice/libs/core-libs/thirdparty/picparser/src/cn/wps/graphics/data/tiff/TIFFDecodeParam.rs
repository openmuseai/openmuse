// Java: public class TIFFDecodeParam
pub struct TIFFDecodeParam {
    // Java field: boolean decodePaletteAsShorts
    pub(crate) decodePaletteAsShorts: bool,
    // Java field: int ifdOffset
    pub(crate) ifdOffset: i32,
    // Java field: boolean convertJPEGYCbCrToRGB
    pub(crate) convertJPEGYCbCrToRGB: bool,
}

// Java: public class TIFFDecodeParam
impl TIFFDecodeParam {
    // Java: TIFFDecodeParam()
    pub fn new() -> Self {
        // Java: super();
        Self {
            decodePaletteAsShorts: false,
            ifdOffset: 0,
            convertJPEGYCbCrToRGB: true,
        }
    }

    // Java: void setDecodePaletteAsShorts(boolean decodePaletteAsShorts)
    pub fn setDecodePaletteAsShorts(&mut self, decode_palette_as_shorts: bool) {
        // Java: this.decodePaletteAsShorts = decodePaletteAsShorts;
        self.decodePaletteAsShorts = decode_palette_as_shorts;
    }

    // Java: boolean getDecodePaletteAsShorts()
    pub fn getDecodePaletteAsShorts(&self) -> bool {
        // Java: return this.decodePaletteAsShorts;
        self.decodePaletteAsShorts
    }

    // Java: byte decode16BitsTo8Bits(int s)
    pub fn decode16BitsTo8Bits(&self, s: i32) -> i8 {
        // Java: return (byte)(s >> 8 & '\uffff');
        ((s >> 8) & 0xff) as i8
    }

    // Java: byte decodeSigned16BitsTo8Bits(short s)
    pub fn decodeSigned16BitsTo8Bits(&self, s: i16) -> i8 {
        // Java: return (byte)(s + Short.MIN_VALUE >> 8);
        ((s.wrapping_add(i16::MIN)) >> 8) as i8
    }

    // Java: void setIFDOffset(int offset)
    pub fn setIFDOffset(&mut self, offset: i32) {
        // Java: this.ifdOffset = offset;
        self.ifdOffset = offset;
    }

    // Java: int getIFDOffset()
    pub fn getIFDOffset(&self) -> i32 {
        // Java: return this.ifdOffset;
        self.ifdOffset
    }

    // Java: void setJPEGDecompressYCbCrToRGB(boolean convertJPEGYCbCrToRGB)
    pub fn setJPEGDecompressYCbCrToRGB(&mut self, convert: bool) {
        // Java: this.convertJPEGYCbCrToRGB = convertJPEGYCbCrToRGB;
        self.convertJPEGYCbCrToRGB = convert;
    }

    // Java: boolean getJPEGDecompressYCbCrToRGB()
    pub fn getJPEGDecompressYCbCrToRGB(&self) -> bool {
        // Java: return this.convertJPEGYCbCrToRGB;
        self.convertJPEGYCbCrToRGB
    }
}
