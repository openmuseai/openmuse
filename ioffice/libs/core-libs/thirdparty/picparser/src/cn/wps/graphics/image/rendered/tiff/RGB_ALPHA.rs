#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class RGB_ALPHA extends TIFFImageType
pub struct RGB_ALPHA {
    _private: (),
}

impl RGB_ALPHA {
    // Java: RGB_ALPHA(TIFFImage tiff)
    pub fn new() -> Self {
        // Java: super(tiff);
        RGB_ALPHA { _private: () }
    }

    // Java: public void rendered(TIFFTile tile)
    pub fn rendered(&self, _tile: i32) {
        // Java: int offsetX = tile.offsetX;
        // Java: int offsetY = tile.offsetY;
        // Java: int inSample = TIFFTile.inSample;
        // Java: int width = tile.width;
        // Java: int height = tile.height;
        // Java: int idx = 0;
        // Java: TIFFTile.byteSamples = this.bytePreRendered(TIFFTile.byteSamples, width);
        // Java: for(int i = offsetY; i < height; i += inSample)
        {
            // Java: for(int j = offsetX; j < width; j += inSample)
            {
                // Java: int sampleIdx = (i * width + j) * 4;
                // Java: int a = TIFFTile.byteSamples[sampleIdx + 3] & 255;
                // Java: int r = TIFFTile.byteSamples[sampleIdx] & 255;
                // Java: int g = TIFFTile.byteSamples[sampleIdx + 1] & 255;
                // Java: int b = TIFFTile.byteSamples[sampleIdx + 2] & 255;
                // Java: TIFFTile.pixels[idx++] = a << 24 | r << 16 | g << 8 | b;
            }
        }
    }
}
