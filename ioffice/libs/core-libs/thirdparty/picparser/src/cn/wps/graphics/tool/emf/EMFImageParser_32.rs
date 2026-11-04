#![allow(non_snake_case, unused_variables, dead_code, unused_imports)]

// Java: import android.graphics.Bitmap.Config
// Java: import cn.wps.graphics.data.emf.EMFImageInfo
use crate::cn::wps::graphics::data::emf::EMFImageInfo::EMFImageInfo;
// Java: import cn.wps.graphics.data.reader.EMFParser
// Java: import cn.wps.graphics.gdi.PixelImage
use crate::cn::wps::graphics::gdi::PixelImage::{PixelImage, BitmapConfig};
use crate::cn::wps::base::IBufferReader::IBufferReader;

// Java: public class EMFImageParser_32 extends EMFImageParser
pub struct EMFImageParser_32 {
    _private: (),
}

// Java: public class EMFImageParser_32 extends EMFImageParser
impl EMFImageParser_32 {
    // Java: EMFImageParser_32()
    pub fn new() -> Self {
        // Java: super();
        EMFImageParser_32 { _private: () }
    }

    // Java: PixelImage read(EMFParser emf, EMFImageInfo bmi, int start, int end, int width, int height)
    pub fn read(&self, emf: &mut dyn IBufferReader, bmi: &EMFImageInfo, start: i32, end: i32, width: i32, height: i32) -> Option<PixelImage> {
        // Java: if (bmi.compression == 0)
        if bmi.compression == 0
        {
            // Java: if (start > emf.getCurrentPos())
            if start > emf.getCurrentPos()
            {
                // Java: emf.setCurrentPos(start);
                emf.setCurrentPos(start);
            }
            // Java: return this.readBuffer32_BI_RGB(emf, bmi, end - start, width, height);
            return self.readBuffer32_BI_RGB(emf, bmi, end - start, width, height);
        }
        // Java: else
        else
        {
            // Java: return bmi.compression == 3 ? this.readBuffer32_BitFields(emf, start, width, height) : null;
            if bmi.compression == 3 {
                return self.readBuffer32_BitFields(emf, start, width, height);
            } else {
                return None;
            }
        }
    }

    // Java: PixelImage readBuffer32_BitFields(EMFParser emf, int start, int width, int height)
    fn readBuffer32_BitFields(&self, emf: &mut dyn IBufferReader, start: i32, width: i32, height: i32) -> Option<PixelImage> {
        // Java: int maskR = emf.readInt();
        let maskR = emf.readInt_one();
        // Java: int maskG = emf.readInt();
        let maskG = emf.readInt_one();
        // Java: int maskB = emf.readInt();
        let maskB = emf.readInt_one();
        // Java: if (start > emf.getCurrentPos())
        if start > emf.getCurrentPos()
        {
            // Java: emf.setCurrentPos(start);
            emf.setCurrentPos(start);
        }
        // Java: PixelImage pixelImage = new PixelImage(width, height, Config.ARGB_4444);
        let mut pixelImage = PixelImage::new(width, height, BitmapConfig::ARGB_4444);
        // Java: int[] colors = new int[width * height];
        let mut colors = vec![0i32; (width * height) as usize];
        // Java: for(int y = 1; y <= height; ++y)
        for y in 1..=height
        {
            // Java: for(int x = 1; x <= width; ++x)
            for x in 1..=width
            {
                // Java: int colorInt = emf.readInt();
                let colorInt = emf.readInt_one();
                // Java: int r = colorInt & maskR;
                let r = colorInt & maskR;
                // Java: int g = colorInt & maskG;
                let g = colorInt & maskG;
                // Java: int b = colorInt & maskB;
                let b = colorInt & maskB;
                // Java: colors[x - 1 + (height - y) * width] = -16777216 | r | g | b;
                colors[(x - 1 + (height - y) * width) as usize] = -16777216i32 | r | g | b;
            }
        }
        // Java: pixelImage.setColors(colors);
        pixelImage.setColors_vec(colors);
        // Java: return pixelImage;
        Some(pixelImage)
    }

    // Java: PixelImage readBuffer32_BI_RGB(EMFParser emf, EMFImageInfo bmi, int len, int width, int height)
    fn readBuffer32_BI_RGB(&self, emf: &mut dyn IBufferReader, bmi: &EMFImageInfo, len: i32, mut width: i32, height: i32) -> Option<PixelImage> {
        // Java: if (width * height != len / 4)
        if width * height != len / 4
        {
            // Java: width = len / 4 / height;
            width = len / 4 / height;
        }
        // Java: PixelImage result = new PixelImage(width, height, Config.ARGB_4444);
        let mut result = PixelImage::new(width, height, BitmapConfig::ARGB_4444);
        // Java: int[] data = emf.readUnsignedInt(len / 4);
        let data = emf.readUnsignedInt_n(len / 4);
        // Java: int off = 0;
        let mut off: usize = 0;
        // Java: int sourceConstantAlpha = bmi.sourceConstantAlpha;
        let sourceConstantAlpha = bmi.sourceConstantAlpha;
        // Java: boolean isAlphaFormat = bmi.isAlphaFormat();
        let isAlphaFormat = bmi.isAlphaFormat();
        // Java: int size = data.length;
        let size = data.len();
        // Java: int[] pixels = new int[size];
        let mut pixels = vec![0i32; size];
        // Java: for(int y = 1; y <= height && off < size; ++y)
        for y in 1..=height
        {
            if off >= size { break; }
            // Java: int heightOffset = (height - y) * width;
            let heightOffset = (height - y) * width;
            // Java: for(int x = 0; x < width && off < size; ++x)
            for x in 0..width
            {
                if off >= size { break; }
                // Java: int pixel = data[off++];
                let pixel = data[off];
                off += 1;
                // Java: int alpha = sourceConstantAlpha;
                let mut alpha = sourceConstantAlpha;
                // Java: if (isAlphaFormat)
                if isAlphaFormat
                {
                    // Java: alpha = (pixel & -16777216) >> 24;
                    alpha = (pixel & -16777216i32) >> 24;
                    // Java: if (alpha == -1)
                    if alpha == -1
                    {
                        // Java: alpha = 255;
                        alpha = 255;
                    }
                    // Java: if (sourceConstantAlpha != 255)
                    if sourceConstantAlpha != 255
                    {
                        // Java: alpha = alpha * sourceConstantAlpha / 255;
                        alpha = alpha * sourceConstantAlpha / 255;
                    }
                }
                // Java: int color = alpha << 24 | pixel & 16777215;
                let color = (alpha << 24) | (pixel & 16777215);
                // Java: pixels[heightOffset + x] = color;
                pixels[(heightOffset + x) as usize] = color;
            }
        }
        // Java: result.setColors(pixels);
        result.setColors_vec(pixels);
        // Java: return result;
        Some(result)
    }
}
