#![allow(non_snake_case, unused_variables, dead_code, unused_imports)]

// Java: import android.graphics.Bitmap.Config
// Java: import cn.wps.graphics.data.emf.EMFImageInfo
use crate::cn::wps::graphics::data::emf::EMFImageInfo::EMFImageInfo;
// Java: import cn.wps.graphics.data.reader.EMFParser
// Java: import cn.wps.graphics.gdi.PixelImage
use crate::cn::wps::graphics::gdi::PixelImage::{PixelImage, BitmapConfig};
use crate::cn::wps::base::IBufferReader::IBufferReader;

// Java: public class EMFImageParser_1 extends EMFImageParser
pub struct EMFImageParser_1 {
    _private: (),
}

// Java: public class EMFImageParser_1 extends EMFImageParser
impl EMFImageParser_1 {
    // Java: EMFImageParser_1()
    pub fn new() -> Self {
        // Java: super();
        EMFImageParser_1 { _private: () }
    }

    // Java: PixelImage read(EMFParser emf, EMFImageInfo bmi, int offBits, int end, int width, int height)
    pub fn read(&self, emf: &mut dyn IBufferReader, bmi: &EMFImageInfo, offBits: i32, end: i32, width: i32, height: i32) -> Option<PixelImage> {
        // Java: PixelImage pixelImage = new PixelImage(width, height, Config.RGB_565);
        let mut pixelImage = PixelImage::new(width, height, BitmapConfig::RGB_565);
        // Java: int color1 = -16777216;
        let mut color1: i32 = -16777216;
        // Java: int color2 = -1;
        let mut color2: i32 = -1;
        // Java: if (!bmi.special)
        if !bmi.special
        {
            // Java: color1 = emf.readColorRe();
            color1 = emf.readColorRe();
            // Java: color2 = emf.readColorRe();
            color2 = emf.readColorRe();
        }
        // Java: int byteRow = 0;
        let mut byteRow: i32 = 0;
        // Java: if (width % 8 == 0)
        if width % 8 == 0
        {
            // Java: byteRow = width / 8;
            byteRow = width / 8;
        }
        // Java: else
        else
        {
            // Java: byteRow = width / 8 + 1;
            byteRow = width / 8 + 1;
        }
        // Java: int factBytes = width * 1 + 31 >> 5 << 2;
        let factBytes = ((width * 1 + 31) >> 5) << 2;
        // Java: int dValue = factBytes - byteRow;
        let dValue = factBytes - byteRow;
        // Java: if (offBits > emf.getCurrentPos())
        if offBits > emf.getCurrentPos()
        {
            // Java: emf.setCurrentPos(offBits);
            emf.setCurrentPos(offBits);
        }
        // Java: int[] colors = new int[width * height];
        let mut colors = vec![0i32; (width * height) as usize];
        // Java: for(int x = 1; x <= height; ++x)
        for x in 1..=height
        {
            // Java: int loopTime = 8;
            let mut loopTime = 8;
            // Java: for(int y = 0; y < byteRow; ++y)
            for y in 0..byteRow
            {
                // Java: int colorValue = emf.readUnsignedByte();
                let colorValue = emf.readUnsignedByte_one();
                // Java: if (y == byteRow - 1 && width % 8 != 0)
                if y == byteRow - 1 && width % 8 != 0
                {
                    // Java: loopTime = width % 8;
                    loopTime = width % 8;
                }
                // Java: for(int m = 0; m < loopTime; ++m)
                for m in 0..loopTime
                {
                    // Java: int value = colorValue >> 8 - m - 1 & 1;
                    let value = (colorValue >> (8 - m - 1)) & 1;
                    // Java: if (value == 0)
                    if value == 0
                    {
                        // Java: colors[(height - x) * width + 8 * y + m] = color1;
                        colors[((height - x) * width + 8 * y + m) as usize] = color1;
                    }
                    // Java: else
                    else
                    {
                        // Java: colors[(height - x) * width + 8 * y + m] = color2;
                        colors[((height - x) * width + 8 * y + m) as usize] = color2;
                    }
                }
            }
            // Java: emf.skip(dValue);
            emf.skip(dValue);
        }
        // Java: pixelImage.setColors(colors);
        pixelImage.setColors_vec(colors);
        // Java: emf.setCurrentPos(end);
        emf.setCurrentPos(end);
        // Java: return pixelImage;
        Some(pixelImage)
    }
}
