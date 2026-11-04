#![allow(non_snake_case, unused_variables, dead_code, unused_imports)]

// Java: import android.graphics.Bitmap.Config
// Java: import cn.wps.graphics.data.decoder.RleDecoder
// Java: import cn.wps.graphics.data.emf.EMFImageInfo
use crate::cn::wps::graphics::data::emf::EMFImageInfo::EMFImageInfo;
// Java: import cn.wps.graphics.data.reader.EMFParser
// Java: import cn.wps.graphics.gdi.PixelImage
use crate::cn::wps::graphics::gdi::PixelImage::{PixelImage, BitmapConfig};
use crate::cn::wps::base::IBufferReader::IBufferReader;
use crate::cn::wps::graphics::data::decoder::RleDecoder::RleDecoder;

// Java: public class EMFImageParser_4 extends EMFImageParser
pub struct EMFImageParser_4 {
    _private: (),
}

// Java: public class EMFImageParser_4 extends EMFImageParser
impl EMFImageParser_4 {
    // Java: EMFImageParser_4()
    pub fn new() -> Self {
        // Java: super();
        EMFImageParser_4 { _private: () }
    }

    // Java: PixelImage read(EMFParser emf, EMFImageInfo bmi, int offBits, int end, int width, int height)
    pub fn read(&self, emf: &mut dyn IBufferReader, bmi: &EMFImageInfo, offBits: i32, end: i32, width: i32, height: i32) -> Option<PixelImage> {
        // Java: int biBitCount = 4;
        let biBitCount = 4;
        // Java: PixelImage pixelImage = new PixelImage(width, height, Config.RGB_565);
        let mut pixelImage = PixelImage::new(width, height, BitmapConfig::RGB_565);
        // Java: pixelImage.initColors();
        pixelImage.initColors();
        // Java: int factBytes = width * biBitCount + 31 >> 5 << 2;
        let factBytes = ((width * biBitCount + 31) >> 5) << 2;
        // Java: int tableLength = bmi.clrUsed == 0 ? 16 : bmi.clrUsed;
        let tableLength = if bmi.clrUsed == 0 { 16 } else { bmi.clrUsed };
        // Java: int[] colorTable = new int[tableLength];
        let mut colorTable = vec![0i32; tableLength as usize];
        // Java: for(int x = 0; x < tableLength; ++x)
        for x in 0..tableLength as usize
        {
            // Java: colorTable[x] = emf.readColorRe();
            colorTable[x] = emf.readColorRe();
        }
        // Java: int byteRow = width / 2 + width % 2;
        let byteRow = width / 2 + width % 2;
        // Java: int dValue = factBytes - byteRow;
        let dValue = factBytes - byteRow;
        // Java: if (offBits > emf.getCurrentPos())
        if offBits > emf.getCurrentPos()
        {
            // Java: emf.setCurrentPos(offBits);
            emf.setCurrentPos(offBits);
        }
        // Java: if (bmi.compression == 0)
        if bmi.compression == 0
        {
            // Java: for(int y = 1; y <= height; ++y)
            for y in 1..=height
            {
                // Java: for(int x = 0; x < byteRow; ++x)
                for x in 0..byteRow
                {
                    // Java: int colorIndex = emf.readUnsignedByte();
                    let colorIndex = emf.readUnsignedByte_one();
                    // Java: int colorFrontIndex = colorIndex >> 4;
                    let colorFrontIndex = colorIndex >> 4;
                    // Java: int colorBackIndex = colorIndex & 15;
                    let colorBackIndex = colorIndex & 15;
                    // Java: pixelImage.setRGB(2 * x, height - y, colorTable[colorFrontIndex]);
                    pixelImage.setRGB(2 * x, height - y, colorTable[colorFrontIndex as usize]);
                    // Java: if (x == byteRow - 1 && width % 2 != 0)
                    if x == byteRow - 1 && width % 2 != 0
                    {
                        // Java: break;
                        break;
                    }
                    // Java: pixelImage.setRGB(2 * x + 1, height - y, colorTable[colorBackIndex]);
                    pixelImage.setRGB(2 * x + 1, height - y, colorTable[colorBackIndex as usize]);
                }
                // Java: emf.skip(dValue);
                emf.skip(dValue);
            }
        }
        // Java: else if (bmi.compression == 2)
        else if bmi.compression == 2
        {
            // Java: byte[] tableIndexs = new byte[factBytes * height];
            let mut tableIndexs = vec![0u8; (factBytes * height) as usize];
            // Java: tableIndexs = RleDecoder.rle4Decoder(emf.getByte(), emf.getCurrentPos(), end - emf.getCurrentPos(), factBytes * height, factBytes, tableIndexs);
            tableIndexs = RleDecoder::rle4Decoder(&[], emf.getCurrentPos(), end - emf.getCurrentPos(), factBytes * height, factBytes, Some(tableIndexs));
            // Java: int[] colors = new int[tableIndexs.length];
            let mut colors = vec![0i32; tableIndexs.len()];
            // Java: int index = 0;
            let mut index: usize = 0;
            // Java: for(int x = 1; x <= height; ++x)
            for x in 1..=height
            {
                // Java: for(int y = 0; y < byteRow; ++y)
                for y in 0..byteRow
                {
                    // Java: colors[x + (height - y) * width] = colorTable[tableIndexs[index++] & 255];
                    if index < tableIndexs.len() {
                        let ti = (tableIndexs[index] & 255) as usize;
                        if ti < colorTable.len() {
                            colors[(x + (height - y) * width) as usize] = colorTable[ti];
                        }
                        index += 1;
                    }
                }
                // Java: index += dValue;
                index += dValue as usize;
            }
            // Java: pixelImage.setColors(colors);
            pixelImage.setColors_vec(colors);
        }
        // Java: return pixelImage;
        Some(pixelImage)
    }
}
