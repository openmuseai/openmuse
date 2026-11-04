#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class YCBCR_SUB extends TIFFImageType
pub struct YCBCR_SUB {
    _private: (),
}

impl YCBCR_SUB {
    // Java: YCBCR_SUB(TIFFImage tiff)
    pub fn new() -> Self {
        // Java: super(tiff);
        YCBCR_SUB { _private: () }
    }

    // Java: public void rendered(TIFFTile tile)
    pub fn rendered(&self, _tile: i32) {
        // Java: int pixelsPerDataUnit = this.tiff.chromaSubH * this.tiff.chromaSubV;
        // Java: int numH = this.tiff.tileWidth / this.tiff.chromaSubH;
        // Java: int numV = this.tiff.tileHeight / this.tiff.chromaSubV;
        // Java: int samplesPerDataUnit = pixelsPerDataUnit * 3;
        // Java: int[] samples = new int[samplesPerDataUnit];
        // Java: int bOffset = 0;
        // Java: int offsetCb = pixelsPerDataUnit;
        // Java: int offsetCr = pixelsPerDataUnit + 1;
        // Java: int x = 0;
        // Java: int y = 0;
        // Java: for(int j = 0; j < numV; ++j)
        {
            // Java: x = 0;
            // Java: for(int i = 0; i < numH; ++i)
            {
                // Java: int Cb = unCompression[bOffset + offsetCb];
                // Java: int Cr = unCompression[bOffset + offsetCr];
                // Java: for(int k = 0; k < samplesPerDataUnit; samples[k++] = Cr)
                {
                    // Java: samples[k++] = unCompression[bOffset++];
                    // Java: samples[k++] = Cb;
                }
                // Java: bOffset += 2;
                // Java: this.setYCbCrIntoPixels(TIFFTile.pixels, samples, x, y, samplesPerDataUnit);
                // Java: x += this.tiff.chromaSubH;
            }
            // Java: y += this.tiff.chromaSubV;
        }
    }

    // Java: private void setYCbCrIntoPixels(int[] pixel, int[] samples, int x, int y, int samplesPerDataUnit)
    fn setYCbCrIntoPixels(&self, _pixel: i32, _samples: i32, _x: i32, _y: i32, _samplesPerDataUnit: i32) {
        // Java: int offsetUpLeft = y * this.tiff.tileWidth + x;
        // Java: int offset = 0;
        // Java: int i = 0;
        // Java: while(i < samplesPerDataUnit)
        {
            // Java: int Y = samples[i++];
            // Java: int Cb = samples[i++];
            // Java: int Cr = samples[i++];
            // Java: Y += Y < 0 ? 255 : 0;
            // Java: Cb += Cb < 0 ? 127 : -128;
            // Java: Cr += Cr < 0 ? 127 : -128;
            // Java: int r = Y + Cr + (Cr >> 2) + (Cr >> 3) + (Cr >> 5);
            // Java: if (r < 0)
            {
                // Java: r = 0;
            }
            // Java: else if (r > 255)
            {
                // Java: r = 255;
            }
            // Java: int g = Y - (Cb >> 2) + (Cb >> 4) + (Cb >> 5) - (Cr >> 1) + (Cr >> 3) + (Cr >> 4) + (Cr >> 5);
            // Java: if (g < 0)
            {
                // Java: g = 0;
            }
            // Java: else if (g > 255)
            {
                // Java: g = 255;
            }
            // Java: int b = Y + Cb + (Cb >> 1) + (Cb >> 2) + (Cb >> 6);
            // Java: if (b < 0)
            {
                // Java: b = 0;
            }
            // Java: else if (b > 255)
            {
                // Java: b = 255;
            }
            // Java: pixel[offsetUpLeft + offset++] = -16777216 | r << 16 | g << 8 | b;
            // Java: if (offset % this.tiff.chromaSubH == 0)
            {
                // Java: offsetUpLeft += this.tiff.tileWidth;
                // Java: offset = 0;
            }
        }
    }
}
