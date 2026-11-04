#![allow(non_snake_case, unused_variables, dead_code, unused_imports)]

// Java: import android.graphics.Bitmap
// Java: import android.graphics.Bitmap.Config
// Java: import android.os.Build.VERSION
// Java: import cn.wps.graphics.data.emf.EMFImageInfo
use crate::cn::wps::graphics::data::emf::EMFImageInfo::EMFImageInfo;
// Java: import cn.wps.graphics.data.reader.EMFParser
// Java: import cn.wps.graphics.gdi.PixelImage
use crate::cn::wps::graphics::gdi::PixelImage::{PixelImage, BitmapConfig};
use crate::cn::wps::base::IBufferReader::IBufferReader;

// Java: public class EMFImageParser_24 extends EMFImageParser
pub struct EMFImageParser_24 {
    _private: (),
}

// Java: public class EMFImageParser_24 extends EMFImageParser
impl EMFImageParser_24 {
    // Java: EMFImageParser_24()
    pub fn new() -> Self {
        // Java: super();
        EMFImageParser_24 { _private: () }
    }

    // Java: PixelImage read(EMFParser emf, EMFImageInfo bmi, int start, int end, int width, int height)
    pub fn read(&self, emf: &mut dyn IBufferReader, bmi: &EMFImageInfo, start: i32, end: i32, width: i32, height: i32) -> Option<PixelImage> {
        // Java: Bitmap.Config config = VERSION.SDK_INT < 28 && VERSION.SDK_INT > 25 ? Config.RGB_565 : Config.ARGB_4444;
        let config = BitmapConfig::ARGB_4444;
        // Java: PixelImage result = new PixelImage(width, height, config);
        let mut result = PixelImage::new(width, height, config);
        // Java: int[] colors = new int[width * height];
        let mut colors = vec![0i32; (width * height) as usize];
        // Java: int byteRow = width * 3;
        let byteRow = width * 3;
        // Java: int factBytes = width * 24 + 31 >> 5 << 2;
        let factBytes = ((width * 24 + 31) >> 5) << 2;
        // Java: int dValue = factBytes - byteRow;
        let dValue = factBytes - byteRow;
        // Java: if (start > emf.getCurrentPos())
        if start > emf.getCurrentPos()
        {
            // Java: emf.setCurrentPos(start);
            emf.setCurrentPos(start);
        }
        // Java: for(int x = 0; x < height; ++x)
        for x in 0..height
        {
            // Java: for(int y = 0; y < width; ++y)
            for y in 0..width
            {
                // Java: colors[(height - x - 1) * width + y] = emf.readBGR();
                colors[((height - x - 1) * width + y) as usize] = emf.readBGR();
            }
            // Java: emf.skip(dValue);
            emf.skip(dValue);
        }
        // Java: result.setColors(colors);
        result.setColors_vec(colors);
        // Java: return result;
        Some(result)
    }
}
