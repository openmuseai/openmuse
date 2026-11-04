#![allow(non_snake_case, unused_variables, dead_code, unused_imports)]

// Java: import android.graphics.Bitmap.Config
// Java: import cn.wps.graphics.data.emf.EMFImageInfo
use crate::cn::wps::graphics::data::emf::EMFImageInfo::EMFImageInfo;
// Java: import cn.wps.graphics.data.reader.EMFParser
// Java: import cn.wps.graphics.gdi.PixelImage
use crate::cn::wps::graphics::gdi::PixelImage::{PixelImage, BitmapConfig};
use crate::cn::wps::base::IBufferReader::IBufferReader;

// Java: public class EMFImageParser_16 extends EMFImageParser
pub struct EMFImageParser_16 {
    _private: (),
}

// Java: public class EMFImageParser_16 extends EMFImageParser
impl EMFImageParser_16 {
    // Java: EMFImageParser_16()
    pub fn new() -> Self {
        // Java: super();
        EMFImageParser_16 { _private: () }
    }

    // Java: PixelImage read(EMFParser emf, EMFImageInfo bmi, int start, int end, int width, int height)
    pub fn read(&self, emf: &mut dyn IBufferReader, bmi: &EMFImageInfo, start: i32, end: i32, width: i32, height: i32) -> Option<PixelImage> {
        // Java: int redShift = 0;
        let mut redShift: i32 = 0;
        // Java: int greenShift = 0;
        let mut greenShift: i32 = 0;
        // Java: int blueShift = 0;
        let mut blueShift: i32 = 0;
        // Java: int maskR = 0;
        let mut maskR: i32 = 0;
        // Java: int maskG = 0;
        let mut maskG: i32 = 0;
        // Java: int maskB = 0;
        let mut maskB: i32 = 0;
        // Java: if (bmi.compression == 0)
        if bmi.compression == 0
        {
            // Java: redShift = 17;
            redShift = 17;
            // Java: greenShift = 22;
            greenShift = 22;
            // Java: blueShift = 27;
            blueShift = 27;
            // Java: maskR = 31744;
            maskR = 31744;
            // Java: maskG = 992;
            maskG = 992;
            // Java: maskB = 31;
            maskB = 31;
        }
        // Java: else if (bmi.compression == 3)
        else if bmi.compression == 3
        {
            // Java: maskR = emf.readUnsignedInt();
            maskR = emf.readUnsignedInt_one();
            // Java: maskG = emf.readUnsignedInt();
            maskG = emf.readUnsignedInt_one();
            // Java: maskB = emf.readUnsignedInt();
            maskB = emf.readUnsignedInt_one();
            // Java: redShift = this.getShiftBits(maskR);
            redShift = self.getShiftBits(maskR);
            // Java: greenShift = this.getShiftBits(maskG);
            greenShift = self.getShiftBits(maskG);
            // Java: blueShift = this.getShiftBits(maskB);
            blueShift = self.getShiftBits(maskB);
        }
        // Java: PixelImage result = new PixelImage(width, height, Config.RGB_565);
        let mut result = PixelImage::new(width, height, BitmapConfig::RGB_565);
        // Java: int[] colors = new int[width * height];
        let mut colors = vec![0i32; (width * height) as usize];
        // Java: int byteRow = width * 2;
        let byteRow = width * 2;
        // Java: int factBytes = width * 16 + 31 >> 5 << 2;
        let factBytes = ((width * 16 + 31) >> 5) << 2;
        // Java: int dValue = factBytes - byteRow;
        let dValue = factBytes - byteRow;
        // Java: if (start > emf.getCurrentPos())
        if start > emf.getCurrentPos()
        {
            // Java: emf.setCurrentPos(start);
            emf.setCurrentPos(start);
        }
        // Java: for(int x = 1; x <= height; ++x)
        for x in 1..=height
        {
            // Java: for(int y = 0; y < width; ++y)
            for y in 0..width
            {
                // Java: int colorByte = emf.readUnsignedShort();
                let colorByte = emf.readUnsignedShort();
                // Java: int r = (colorByte & maskR) << redShift >>> 8;
                let r = ((colorByte & maskR) << redShift) as u32 >> 8;
                // Java: int g = (colorByte & maskG) << greenShift >>> 16;
                let g = ((colorByte & maskG) << greenShift) as u32 >> 16;
                // Java: int b = (colorByte & maskB) << blueShift >>> 24;
                let b = ((colorByte & maskB) << blueShift) as u32 >> 24;
                // Java: colors[(height - x) * width + y] = -16777216 | r | g | b;
                colors[((height - x) * width + y) as usize] = -16777216i32 | r as i32 | g as i32 | b as i32;
            }
            // Java: emf.skip(dValue);
            emf.skip(dValue);
        }
        // Java: result.setColors(colors);
        result.setColors_vec(colors);
        // Java: return result;
        Some(result)
    }

    // Java: int getShiftBits(int mask)
    pub fn getShiftBits(&self, mask: i32) -> i32 {
        // Java: int shiftBits;
        let mut shiftBits: i32 = 0;
        let mut mask = mask;
        // Java: for(shiftBits = 0; (mask & Integer.MIN_VALUE) == 0; ++shiftBits)
        while (mask & i32::MIN) == 0
        {
            // Java: mask <<= 1;
            mask <<= 1;
            shiftBits += 1;
        }
        // Java: return shiftBits;
        shiftBits
    }
}
