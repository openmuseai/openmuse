#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class PALETTE extends TIFFImageType
pub struct PALETTE {
    _private: (),
}

impl PALETTE {
    // Java: PALETTE(TIFFImage tiff)
    pub fn new() -> Self {
        // Java: super(tiff);
        PALETTE { _private: () }
    }

    // Java: public void rendered(TIFFTile tile)
    pub fn rendered(&self, _tile: i32) {
        // Java: int offsetX = tile.offsetX;
        // Java: int offsetY = tile.offsetY;
        // Java: int inSample = TIFFTile.inSample;
        // Java: int width = tile.width;
        // Java: int height = tile.height;
        // Java: int idx = 0;
        // Java: TIFFTile.intSamples = this.intPreRendered(TIFFTile.intSamples, width);
        // Java: for(int i = offsetY; i < height; i += inSample)
        {
            // Java: for(int j = offsetX; j < width; j += inSample)
            {
                // Java: int sampleIdx = TIFFTile.intSamples[i * width + j];
                // Java: int r = this.tiff.r[sampleIdx] & 255;
                // Java: int g = this.tiff.g[sampleIdx] & 255;
                // Java: int b = this.tiff.b[sampleIdx] & 255;
                // Java: TIFFTile.pixels[idx++] = -16777216 | r << 16 | g << 8 | b;
            }
        }
    }
}
