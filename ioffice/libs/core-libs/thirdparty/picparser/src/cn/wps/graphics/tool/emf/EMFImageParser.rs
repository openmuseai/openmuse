// Java: import cn.wps.graphics.data.emf.EMFImageInfo
use crate::cn::wps::graphics::data::emf::EMFImageInfo::EMFImageInfo;
// Java: import cn.wps.graphics.data.reader.EMFParser
// Java: import cn.wps.graphics.gdi.PixelImage
use crate::cn::wps::graphics::gdi::PixelImage::PixelImage;
use crate::cn::wps::base::IBufferReader::IBufferReader;

// Java: public class EMFImageParser
pub struct EMFImageParser {
    _private: (),
}

// Java field: String TAG
const TAG: Option<&str> = None;

// Java: public class EMFImageParser
impl EMFImageParser {
    // Java: EMFImageParser()
    pub fn new() -> Self {
      // Java: super();
      EMFImageParser { _private: () }
    }

    // Java: abstract PixelImage read(EMFParser var1, EMFImageInfo var2, int var3, int var4, int var5, int var6)
    pub fn read(&self, _var1: &mut dyn IBufferReader, _var2: &EMFImageInfo, _var3: i32, _var4: i32, _var5: i32, _var6: i32) -> Option<PixelImage> {
      None
    }

    // Java: static PixelImage readImage(EMFParser emf, EMFImageInfo bmi, int width, int height, int size)
    pub fn readImage(emf: &mut dyn IBufferReader, bmi: &EMFImageInfo, width: i32, height: i32, size: i32) -> Option<PixelImage> {
      // Java: return readImage(emf, bmi, width, height, emf.getCurrentPos(), size);
      let off_bits = emf.getCurrentPos();
      Self::readImage_with_offset(emf, bmi, width, height, off_bits, size)
    }

    // Java: static PixelImage readImage(EMFParser emf, EMFImageInfo bmi, int width, int height, int offBits, int size)
    pub fn readImage_with_offset(emf: &mut dyn IBufferReader, bmi: &EMFImageInfo, width: i32, height: i32, offBits: i32, size: i32) -> Option<PixelImage> {
      // Java: PixelImage image = null;
      let mut image: Option<PixelImage> = None;
      // Java: int finalPos = emf.getCurrentPos() + size;
      let finalPos = emf.getCurrentPos() + size;
      // Java: try
      {
         // Java: EMFImageParser parser = getParser(bmi.bitCount);
         let parser = Self::getParser(bmi.bitCount);
         // Java: if (parser != null)
         if let Some(p) = parser {
            // Java: image = parser.read(emf, bmi, offBits, finalPos, width, height);
            image = p.read(emf, bmi, offBits, finalPos, width, height);
         }
      }
      // Java: catch (OutOfMemoryError var12)
      // Java: finally
      {
         // Java: emf.setCurrentPos(finalPos);
         emf.setCurrentPos(finalPos);
      }
      // Java: return image;
      image
    }

    // Java: static EMFImageParser getParser(int bitCount)
    pub fn getParser(bitCount: i32) -> Option<EMFImageParser> {
      // Java: switch (bitCount)
      match bitCount {
         // Java: case 1:
         1 => {
            // Java: return new EMFImageParser_1();
            None
         }
         // Java: case 4:
         4 => {
            // Java: return new EMFImageParser_4();
            None
         }
         // Java: case 8:
         8 => {
            // Java: return new EMFImageParser_8();
            None
         }
         // Java: case 16:
         16 => {
            // Java: return new EMFImageParser_16();
            None
         }
         // Java: case 24:
         24 => {
            // Java: return new EMFImageParser_24();
            None
         }
         // Java: case 32:
         32 => {
            // Java: return new EMFImageParser_32();
            None
         }
         // Java: default:
         _ => {
            // Java: return null;
            None
         }
      }
    }
}
