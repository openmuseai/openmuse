// Java: import cn.wps.graphics.data.tiff.TIFFImage
use thirdparty_picparser::graphics::data::tiff::TIFFImage;
// Java: import cn.wps.graphics.data.tiff.TIFFTile
use thirdparty_picparser::cn::wps::graphics::data::tiff::TIFFTile;

// Java: public class BILEVEL extends TIFFImageType
pub struct BILEVEL {
}

// Java: public class BILEVEL extends TIFFImageType
impl BILEVEL {
    // Java: BILEVEL(TIFFImage tiff)
    pub fn BILEVEL(&self, _tiff: i32) {
      // Java: super(tiff);
      // TODO_RUST_CODE
    }

    // Java: void rendered(TIFFTile tile)
    pub fn rendered(&self, _tile: i32) {
      // Java: int offsetX = tile.offsetX;
      // TODO_RUST_CODE
      // Java: int offsetY = tile.offsetY;
      // TODO_RUST_CODE
      // Java: int inSample = TIFFTile.inSample;
      // TODO_RUST_CODE
      // Java: int width = tile.width;
      // TODO_RUST_CODE
      // Java: int height = tile.height;
      // TODO_RUST_CODE
      // Java: int noUseBits = tile.getNoUseBits();
      // TODO_RUST_CODE
      // Java: int[] map = new int[2];
      // TODO_RUST_CODE
      // Java: if (this.tiff.isWhiteZero)
        // TODO_RUST_CODE
      {
         // Java: map[0] = -1;
         // TODO_RUST_CODE
         // Java: map[1] = -16777216;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: map[0] = -16777216;
         // TODO_RUST_CODE
         // Java: map[1] = -1;
         // TODO_RUST_CODE
      }
      // Java: int idx = 0;
      // TODO_RUST_CODE
      // Java: int value = 0;
      // TODO_RUST_CODE
      // Java: for(int i = offsetY; i < height; i += inSample)
        // TODO_RUST_CODE
      {
         // Java: for(int j = offsetX; j < width; j += inSample)
            // TODO_RUST_CODE
         {
            // Java: int colorValue = unCompression[(i * (width + noUseBits) + j) / 8] & 255;
            // TODO_RUST_CODE
            // Java: value = colorValue >> 7 - j % 8 & 1;
            // TODO_RUST_CODE
            // Java: TIFFTile.pixels[idx++] = map[value];
            // TODO_RUST_CODE
         }
      }
    }

}
