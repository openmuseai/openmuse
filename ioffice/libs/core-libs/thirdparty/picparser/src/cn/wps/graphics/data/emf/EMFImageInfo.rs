// Java: import cn.wps.graphics.data.reader.EMFParser
use crate::cn::wps::base::IBufferReader::IBufferReader;

// Java: public class EMFImageInfo
pub struct EMFImageInfo {
    // Java field: int size
    pub(crate) size: i32,
    // Java field: int width
    pub(crate) width: i32,
    // Java field: int height
    pub(crate) height: i32,
    // Java field: int bitCount
    pub(crate) bitCount: i32,
    // Java field: int compression
    pub(crate) compression: i32,
    // Java field: int clrUsed
    pub(crate) clrUsed: i32,
    // Java field: int sourceConstantAlpha
    pub(crate) sourceConstantAlpha: i32,
    // Java field: int alphaFormat
    pub(crate) alphaFormat: i32,
    // Java field: boolean special
    pub(crate) special: bool,
}

// Java field: int BI_RGB
pub(crate) const BI_RGB: i32 = 0;
// Java field: int BI_RLE8
pub(crate) const BI_RLE8: i32 = 1;
// Java field: int BI_RLE4
pub(crate) const BI_RLE4: i32 = 2;
// Java field: int BI_BITFIELDS
pub(crate) const BI_BITFIELDS: i32 = 3;
// Java field: int BI_JPEG
pub(crate) const BI_JPEG: i32 = 4;
// Java field: int BI_PNG
pub(crate) const BI_PNG: i32 = 5;
// Java field: int AC_SRC_ALPHA
pub(crate) const AC_SRC_ALPHA: i32 = 1;

// Java: public class EMFImageInfo
impl EMFImageInfo {
    // Java: EMFImageInfo()
    pub fn new() -> Self {
      // Java: super();
      EMFImageInfo {
          size: 0,
          width: 0,
          height: 0,
          bitCount: 0,
          compression: 0,
          clrUsed: 0,
          sourceConstantAlpha: 255,
          alphaFormat: 0,
          special: false,
      }
    }

    // Java: static EMFImageInfo read(EMFParser emf)
    pub fn read(emf: &mut dyn IBufferReader) -> EMFImageInfo {
      // Java: EMFImageInfo info = new EMFImageInfo();
      let mut info = EMFImageInfo::new();
      // Java: info.readInfo(emf);
      info.readInfo(emf);
      // Java: return info;
      info
    }

    // Java: void readInfo(EMFParser emf)
    pub fn readInfo(&mut self, emf: &mut dyn IBufferReader) {
      // Java: this.size = emf.readUnsignedInt();
      self.size = emf.readUnsignedInt_one();
      // Java: this.width = emf.readInt();
      self.width = emf.readInt_one();
      // Java: this.height = emf.readInt();
      self.height = emf.readInt_one();
      // Java: emf.skip(2);
      emf.skip(2);
      // Java: this.bitCount = emf.readUnsignedShort();
      self.bitCount = emf.readUnsignedShort();
      // Java: this.compression = emf.readUnsignedInt();
      self.compression = emf.readUnsignedInt_one();
      // Java: emf.skip(12);
      emf.skip(12);
      // Java: this.clrUsed = emf.readUnsignedInt();
      self.clrUsed = emf.readUnsignedInt_one();
      // Java: emf.skip(4);
      emf.skip(4);
    }

    // Java: void readBlend(EMFParser emf)
    pub fn readBlend(&mut self, emf: &mut dyn IBufferReader) {
      // Java: emf.skip(2);
      emf.skip(2);
      // Java: this.sourceConstantAlpha = emf.readUnsignedByte();
      self.sourceConstantAlpha = emf.readUnsignedByte_one();
      // Java: this.alphaFormat = emf.readUnsignedByte();
      self.alphaFormat = emf.readUnsignedByte_one();
    }

    // Java: boolean isAlphaFormat()
    pub fn isAlphaFormat(&self) -> bool {
      // Java: return this.alphaFormat == 1;
      self.alphaFormat == AC_SRC_ALPHA
    }

}
