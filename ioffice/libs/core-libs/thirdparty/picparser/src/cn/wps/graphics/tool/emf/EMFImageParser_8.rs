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

// Java: public class EMFImageParser_8 extends EMFImageParser
pub struct EMFImageParser_8 {
    _private: (),
}

// Java: public class EMFImageParser_8 extends EMFImageParser
impl EMFImageParser_8 {
    // Java: EMFImageParser_8()
    pub fn new() -> Self {
        // Java: super();
        EMFImageParser_8 { _private: () }
    }

    // Java: PixelImage read(EMFParser emf, EMFImageInfo bmi, int start, int end, int width, int height)
    pub fn read(&self, emf: &mut dyn IBufferReader, bmi: &EMFImageInfo, start: i32, end: i32, width: i32, height: i32) -> Option<PixelImage> {
        // Java: int colorsUsed = bmi.clrUsed;
        let mut colorsUsed = bmi.clrUsed;
        // Java: if (colorsUsed > 256)
        if colorsUsed > 256
        {
            // Java: return null;
            return None;
        }
        // Java: else
        {
        // Java: if (colorsUsed == 0)
        if colorsUsed == 0
        {
            // Java: colorsUsed = 256;
            colorsUsed = 256;
        }
        // Java: int[] colorTable = new int[256];
        let mut colorTable = vec![0i32; 256];
        // Java: for(int i = 0; i < colorsUsed; ++i)
        for i in 0..colorsUsed as usize
        {
            // Java: colorTable[i] = emf.readColorRe();
            colorTable[i] = emf.readColorRe();
        }
        // Java: int factBytes = width * 8 + 31 >> 5 << 2;
        let factBytes = ((width * 8 + 31) >> 5) << 2;
        // Java: int dValue = factBytes - width;
        let dValue = factBytes - width;
        // Java: byte[] tableIndexs = new byte[factBytes * height];
        let mut tableIndexs = vec![0u8; (factBytes * height) as usize];
        // Java: if (start > emf.getCurrentPos())
        if start > emf.getCurrentPos()
        {
            // Java: emf.setCurrentPos(start);
            emf.setCurrentPos(start);
        }
        // Java: switch (bmi.compression)
        match bmi.compression {
            // Java: case 0:
            0 => {
                // Java: emf.read(tableIndexs, end - start);
                emf.read_n(&mut tableIndexs, end - start);
                // Java: break;
            }
            // Java: case 1:
            1 => {
                // Java: RleDecoder.rle8Decoder(emf.getByte(), emf.getCurrentPos() + 1, end - start, factBytes * height, factBytes, tableIndexs);
                tableIndexs = RleDecoder::rle8Decoder(&[], emf.getCurrentPos() + 1, end - start, factBytes * height, factBytes, Some(tableIndexs));
                // Java: emf.skip(end - start);
                emf.skip(end - start);
            }
            _ => {}
        }
        // Java: PixelImage result = new PixelImage(width, height, Config.RGB_565);
        let mut result = PixelImage::new(width, height, BitmapConfig::RGB_565);
        // Java: int index = 0;
        let mut index: usize = 0;
        // Java: int[] colors = new int[tableIndexs.length];
        let mut colors = vec![0i32; tableIndexs.len()];
        // Java: for(int y = 1; y <= height; ++y)
        for y in 1..=height
        {
            // Java: for(int x = 0; x < width; ++x)
            for x in 0..width
            {
                // Java: colors[x + (height - y) * width] = colorTable[tableIndexs[index++] & 255];
                if index < tableIndexs.len() {
                    let ti = (tableIndexs[index] & 255) as usize;
                    colors[(x + (height - y) * width) as usize] = colorTable[ti];
                    index += 1;
                }
            }
            // Java: index += dValue;
            index += dValue as usize;
        }
        // Java: result.setColors(colors);
        result.setColors_vec(colors);
        // Java: return result;
        Some(result)
        }
    }
}
