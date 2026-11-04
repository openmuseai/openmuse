#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.graphics.data.reader.BufferReader
// Java: import java.io.IOException

use std::io;
use crate::cn::wps::graphics::data::reader::BufferReader::BufferReader;
use crate::cn::wps::graphics::data::tiff::TIFFDecodeParam::TIFFDecodeParam;

pub struct TIFFImage;
pub struct TIFFDirectory;
impl TIFFDirectory {
    pub fn getNumDirectories(_input: &BufferReader) -> io::Result<i32> { Ok(0) }
}
impl TIFFImage {
    pub fn new(_input: &BufferReader, _param: &TIFFDecodeParam, _page: i32) -> Self { TIFFImage }
}

// Java: public class TIFFImageDecoder
pub struct TIFFImageDecoder {
    // Java field: int TIFF_IMAGE_WIDTH
    // Java field: int TIFF_IMAGE_LENGTH
    // Java field: int TIFF_BITS_PER_SAMPLE
    // Java field: int TIFF_COMPRESSION
    // Java field: int TIFF_PHOTOMETRIC_INTERPRETATION
    // Java field: int TIFF_THRESHHOLDING
    // Java field: int TIFF_FILL_ORDER
    // Java field: int TIFF_STRIP_OFFSETS
    // Java field: int TIFF_SAMPLES_PER_PIXEL
    // Java field: int TIFF_ROWS_PER_STRIP
    // Java field: int TIFF_STRIP_BYTE_COUNTS
    // Java field: int TIFF_X_RESOLUTION
    // Java field: int TIFF_Y_RESOLUTION
    // Java field: int TIFF_PLANAR_CONFIGURATION
    // Java field: int TIFF_T4_OPTIONS
    // Java field: int TIFF_T6_OPTIONS
    // Java field: int TIFF_RESOLUTION_UNIT
    // Java field: int TIFF_PREDICTOR
    // Java field: int TIFF_COLORMAP
    // Java field: int TIFF_TILE_WIDTH
    // Java field: int TIFF_TILE_LENGTH
    // Java field: int TIFF_TILE_OFFSETS
    // Java field: int TIFF_TILE_BYTE_COUNTS
    // Java field: int TIFF_EXTRA_SAMPLES
    // Java field: int TIFF_SAMPLE_FORMAT
    // Java field: int TIFF_S_MIN_SAMPLE_VALUE
    // Java field: int TIFF_S_MAX_SAMPLE_VALUE
    // Java field: BufferReader input
    pub(crate) input: BufferReader,
    // Java field: TIFFDecodeParam param
    pub(crate) param: TIFFDecodeParam,
}

pub(crate) const TIFF_IMAGE_WIDTH: i32 = 256;
pub(crate) const TIFF_IMAGE_LENGTH: i32 = 257;
pub(crate) const TIFF_BITS_PER_SAMPLE: i32 = 258;
pub(crate) const TIFF_COMPRESSION: i32 = 259;
pub(crate) const TIFF_PHOTOMETRIC_INTERPRETATION: i32 = 262;
pub(crate) const TIFF_THRESHHOLDING: i32 = 263;
pub(crate) const TIFF_FILL_ORDER: i32 = 266;
pub(crate) const TIFF_STRIP_OFFSETS: i32 = 273;
pub(crate) const TIFF_SAMPLES_PER_PIXEL: i32 = 277;
pub(crate) const TIFF_ROWS_PER_STRIP: i32 = 278;
pub(crate) const TIFF_STRIP_BYTE_COUNTS: i32 = 279;
pub(crate) const TIFF_X_RESOLUTION: i32 = 282;
pub(crate) const TIFF_Y_RESOLUTION: i32 = 283;
pub(crate) const TIFF_PLANAR_CONFIGURATION: i32 = 284;
pub(crate) const TIFF_T4_OPTIONS: i32 = 292;
pub(crate) const TIFF_T6_OPTIONS: i32 = 293;
pub(crate) const TIFF_RESOLUTION_UNIT: i32 = 296;
pub(crate) const TIFF_PREDICTOR: i32 = 317;
pub(crate) const TIFF_COLORMAP: i32 = 320;
pub(crate) const TIFF_TILE_WIDTH: i32 = 322;
pub(crate) const TIFF_TILE_LENGTH: i32 = 323;
pub(crate) const TIFF_TILE_OFFSETS: i32 = 324;
pub(crate) const TIFF_TILE_BYTE_COUNTS: i32 = 325;
pub(crate) const TIFF_EXTRA_SAMPLES: i32 = 338;
pub(crate) const TIFF_SAMPLE_FORMAT: i32 = 339;
pub(crate) const TIFF_S_MIN_SAMPLE_VALUE: i32 = 340;
pub(crate) const TIFF_S_MAX_SAMPLE_VALUE: i32 = 341;

// Java: public class TIFFImageDecoder
impl TIFFImageDecoder {
    // Java: TIFFImageDecoder(BufferReader input, TIFFDecodeParam param)
    pub fn new(input: BufferReader, param: TIFFDecodeParam) -> Self {
        // Java: super();
        // Java: this.input = input;
        // Java: this.param = param;
        TIFFImageDecoder { input, param }
    }

    // Java: int getNumPages()
    pub fn getNumPages(&self) -> io::Result<i32> {
        // Java: return TIFFDirectory.getNumDirectories(this.input);
        TIFFDirectory::getNumDirectories(&self.input)
    }

    // Java: TIFFImage decodeAsRenderedImage(int page)
    pub fn decodeAsRenderedImage(&self, page: i32) -> io::Result<TIFFImage> {
        // Java: if (page >= 0 && page < this.getNumPages())
        if page >= 0 && page < self.getNumPages()? {
            // Java: return new TIFFImage(this.input, this.param, page);
            Ok(TIFFImage::new(&self.input, &self.param, page))
        }
        // Java: else
        else {
            // Java: throw new IOException("TIFFImageDecoder0");
            Err(io::Error::new(io::ErrorKind::Other, "TIFFImageDecoder0"))
        }
    }
}
