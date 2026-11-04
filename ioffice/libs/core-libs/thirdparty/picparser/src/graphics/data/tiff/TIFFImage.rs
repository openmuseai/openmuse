// Java: import android.graphics.Rect
use util::cn::wps::graphics::Rect;
// Java: import cn.wps.graphics.data.decoder.FaxDecoder
use thirdparty_picparser::cn::wps::graphics::data::decoder::FaxDecoder;
// Java: import cn.wps.graphics.data.decoder.LZWDecoder
use thirdparty_picparser::cn::wps::graphics::data::decoder::LZWDecoder;
// Java: import cn.wps.graphics.data.reader.BufferReader
use thirdparty_picparser::cn::wps::graphics::data::reader::BufferReader;
// Java: import cn.wps.graphics.image.rendered.tiff.BILEVEL
use thirdparty_picparser::graphics::image::rendered::tiff::BILEVEL;
// Java: import cn.wps.graphics.image.rendered.tiff.CMYK
use thirdparty_picparser::graphics::image::rendered::tiff::CMYK;
// Java: import cn.wps.graphics.image.rendered.tiff.GRAY
use thirdparty_picparser::graphics::image::rendered::tiff::GRAY;
// Java: import cn.wps.graphics.image.rendered.tiff.GRAY_ALPHA
use thirdparty_picparser::graphics::image::rendered::tiff::GRAY_ALPHA;
// Java: import cn.wps.graphics.image.rendered.tiff.LOGL
use thirdparty_picparser::graphics::image::rendered::tiff::LOGL;
// Java: import cn.wps.graphics.image.rendered.tiff.LOGLUV
use thirdparty_picparser::graphics::image::rendered::tiff::LOGLUV;
// Java: import cn.wps.graphics.image.rendered.tiff.PALETTE
use thirdparty_picparser::graphics::image::rendered::tiff::PALETTE;
// Java: import cn.wps.graphics.image.rendered.tiff.RGB
use thirdparty_picparser::graphics::image::rendered::tiff::RGB;
// Java: import cn.wps.graphics.image.rendered.tiff.RGB_ALPHA
use thirdparty_picparser::graphics::image::rendered::tiff::RGB_ALPHA;
// Java: import cn.wps.graphics.image.rendered.tiff.TIFFImageType
use thirdparty_picparser::graphics::image::rendered::tiff::TIFFImageType;
// Java: import cn.wps.graphics.image.rendered.tiff.YCBCR_SUB
use thirdparty_picparser::graphics::image::rendered::tiff::YCBCR_SUB;
// Java: import java.io.IOException
// TODO_RUST_CODE use ???; // java.io.IOException
// Java: import java.util.zip.Inflater
// TODO_RUST_CODE use ???; // java.util.zip.Inflater

// Java field: int TYPE_UNSUPPORTED
pub(crate) const TYPE_UNSUPPORTED: i32 = -1;
// Java field: int TYPE_BILEVEL
pub(crate) const TYPE_BILEVEL: i32 = 0;
// Java field: int TYPE_GRAY
pub(crate) const TYPE_GRAY: i32 = 1;
// Java field: int TYPE_GRAY_ALPHA
pub(crate) const TYPE_GRAY_ALPHA: i32 = 2;
// Java field: int TYPE_PALETTE
pub(crate) const TYPE_PALETTE: i32 = 3;
// Java field: int TYPE_RGB
pub(crate) const TYPE_RGB: i32 = 4;
// Java field: int TYPE_RGB_ALPHA
pub(crate) const TYPE_RGB_ALPHA: i32 = 5;
// Java field: int TYPE_CMYK
pub(crate) const TYPE_CMYK: i32 = 6;
// Java field: int TYPE_YCBCR_SUB
pub(crate) const TYPE_YCBCR_SUB: i32 = 7;
// Java field: int TYPE_LOGL
pub(crate) const TYPE_LOGL: i32 = 8;
// Java field: int TYPE_LOGLUV
pub(crate) const TYPE_LOGLUV: i32 = 9;
// Java field: int TIFF_JPEG_TABLES
pub(crate) const TIFF_JPEG_TABLES: i32 = 347;
// Java field: int TIFF_YCBCR_SUBSAMPLING
pub(crate) const TIFF_YCBCR_SUBSAMPLING: i32 = 530;

// Java: public class TIFFImage
pub struct TIFFImage {
    // Java field: BufferReader stream
    stream: Option<BufferReader>,
    // Java field: TIFFDecodeParam param
    param: Option<TIFFDecodeParam>,
    // Java field: TIFFImageType imageRendered
    imageRendered: Option<TIFFImageType>,
    // Java field: Rect bounds
    pub(crate) bounds: Arc<Mutex<Rect>>,
    // Java field: int tileWidth
    pub(crate) tileWidth: i32,
    // Java field: int tileHeight
    pub(crate) tileHeight: i32,
    // Java field: int tilesX
    pub(crate) tilesX: i32,
    // Java field: int tilesY
    pub(crate) tilesY: i32,
    // Java field: long[] tileOffsets
    tileOffsets: Option<Vec<i64>>,
    // Java field: long[] tileByteCounts
    tileByteCounts: Option<Vec<i64>>,
    // Java field: int sampleSize
    pub(crate) sampleSize: i32,
    // Java field: int samplesFormat
    samplesFormat: i32,
    // Java field: int numBands
    pub(crate) numBands: i32,
    // Java field: int samplesPerPixel
    samplesPerPixel: i32,
    // Java field: int imageType
    imageType: i32,
    // Java field: char[] colormap
    colormap: Option<Vec<char>>,
    // Java field: byte[] r
    pub(crate) r: Arc<Mutex<Vec<u8>>>,
    // Java field: byte[] g
    pub(crate) g: Arc<Mutex<Vec<u8>>>,
    // Java field: byte[] b
    pub(crate) b: Arc<Mutex<Vec<u8>>>,
    // Java field: int chromaSubH
    pub(crate) chromaSubH: i32,
    // Java field: int chromaSubV
    pub(crate) chromaSubV: i32,
    // Java field: int compression
    pub(crate) compression: i32,
    // Java field: long tiffT4Options
    pub(crate) tiffT4Options: i64,
    // Java field: long tiffT6Options
    pub(crate) tiffT6Options: i64,
    // Java field: int fillOrder
    fillOrder: i32,
    // Java field: LZWDecoder lzwDecoder
    pub(crate) lzwDecoder: Arc<Mutex<LZWDecoder>>,
    // Java field: int predictor
    pub(crate) predictor: i32,
    // Java field: boolean colorConvertJPEG
    colorConvertJPEG: bool,
    // Java field: Inflater inflater
    pub(crate) inflater: Arc<Mutex<Inflater>>,
    // Java field: FaxDecoder decoder
    pub(crate) decoder: Arc<Mutex<FaxDecoder>>,
    // Java field: boolean isBigEndian
    pub(crate) isBigEndian: bool,
    // Java field: boolean isWhiteZero
    pub(crate) isWhiteZero: bool,
    // Java field: boolean isPlanarSeparate
    pub(crate) isPlanarSeparate: bool,
    // Java field: boolean tiled
    tiled: bool,
}

// Java: public class TIFFImage
impl TIFFImage {
    // Java: long[] getFieldAsLongs(TIFFField field)
    pub fn getFieldAsLongs(&self, _field: i32) -> long[] {
      // Java: long[] value = null;
      // TODO_RUST_CODE
      // Java: if (field.getType() == 3)
        // TODO_RUST_CODE
      {
         // Java: char[] charValue = field.getAsChars();
         // TODO_RUST_CODE
         // Java: value = new long[charValue.length];
         // TODO_RUST_CODE
         // Java: for(int i = 0; i < charValue.length; ++i)
            // TODO_RUST_CODE
         {
            // Java: value[i] = (long)(charValue[i] & '\uffff');
            // TODO_RUST_CODE
         }
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: if (field.getType() != 4)
            // TODO_RUST_CODE
         {
            // Java: throw new RuntimeException();
            // TODO_RUST_CODE
         }
         // Java: value = field.getAsLongs();
         // TODO_RUST_CODE
      }
      // Java: return value;
      // TODO_RUST_CODE
    }

    // Java: TIFFImage(BufferReader stream, TIFFDecodeParam param, int directory)
    pub fn TIFFImage(&self, _stream: i32, _param: i32, directory: i32) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.stream = stream;
      // TODO_RUST_CODE
      // Java: if (param == null)
        // TODO_RUST_CODE
      {
         // Java: param = new TIFFDecodeParam();
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.param = param;
         // TODO_RUST_CODE
      }
      // Java: TIFFDirectory dir = param.getIFDOffset() == -1 ? new TIFFDirectory(stream, directory) : new TIFFDirectory(stream, param.getIFDOffset(), directory);
      // TODO_RUST_CODE
      // Java: this.bounds = new Rect(0, 0, (int)dir.getFieldAsLong(256), (int)dir.getFieldAsLong(257));
      // TODO_RUST_CODE
      // Java: this.initTile(dir);
      // TODO_RUST_CODE
      // Java: TIFFField sfield = dir.getField(277);
      // TODO_RUST_CODE
      // Java: this.samplesPerPixel = sfield == null ? 1 : (int)sfield.getAsLong(0);
      // TODO_RUST_CODE
      // Java: this.checkPlanarConfiguration(dir);
      // TODO_RUST_CODE
      // Java: this.initSampleSize(dir);
      // TODO_RUST_CODE
      // Java: this.checkSampleFormat(dir);
      // TODO_RUST_CODE
      // Java: TIFFField efield = dir.getField(338);
      // TODO_RUST_CODE
      // Java: if (efield == null)
        // TODO_RUST_CODE
      {
         // Java: boolean var10000 = false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: int var8 = (int)efield.getAsLong(0);
         // TODO_RUST_CODE
      }
      // Java: this.numBands = this.samplesPerPixel;
      // TODO_RUST_CODE
      // Java: this.initCompression(dir);
      // TODO_RUST_CODE
      // Java: this.initImageType(dir);
      // TODO_RUST_CODE
      // Java: this.isBigEndian = dir.isBigEndian();
      // TODO_RUST_CODE
      // Java: this.newImageRender();
      // TODO_RUST_CODE
    }

    // Java: void checkPlanarConfiguration(TIFFDirectory dir)
    pub fn checkPlanarConfiguration(&self, _dir: i32) {
      // Java: TIFFField planarConfigurationField = dir.getField(284);
      // TODO_RUST_CODE
      // Java: char[] planarConfiguration = planarConfigurationField == null ? new char[]{'\u0001'} : planarConfigurationField.getAsChars();
      // TODO_RUST_CODE
      // Java: this.isPlanarSeparate = planarConfiguration[0] == 2;
      // TODO_RUST_CODE
    }

    // Java: void initTile(TIFFDirectory dir)
    pub fn initTile(&self, _dir: i32) {
      // Java: if (dir.getField(324) != null)
        // TODO_RUST_CODE
      {
         // Java: this.tiled = true;
         // TODO_RUST_CODE
         // Java: this.tileWidth = (int)dir.getFieldAsLong(322);
         // TODO_RUST_CODE
         // Java: this.tileHeight = (int)dir.getFieldAsLong(323);
         // TODO_RUST_CODE
         // Java: this.tileOffsets = dir.getField(324).getAsLongs();
         // TODO_RUST_CODE
         // Java: this.tileByteCounts = this.getFieldAsLongs(dir.getField(325));
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.tiled = false;
         // TODO_RUST_CODE
         // Java: this.tileWidth = dir.getField(322) != null ? (int)dir.getFieldAsLong(322) : this.bounds.width();
         // TODO_RUST_CODE
         // Java: TIFFField field = dir.getField(278);
         // TODO_RUST_CODE
         // Java: if (field == null)
            // TODO_RUST_CODE
         {
            // Java: this.tileHeight = dir.getField(323) != null ? (int)dir.getFieldAsLong(323) : this.bounds.height();
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: long lon = field.getAsLong(0);
            // TODO_RUST_CODE
            // Java: long infinity = 1L;
            // TODO_RUST_CODE
            // Java: infinity = (infinity << 32) - 1L;
            // TODO_RUST_CODE
            // Java: if (lon == (long)((int)infinity))
                // TODO_RUST_CODE
            {
               // Java: this.tileHeight = this.bounds.height();
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: this.tileHeight = (int)lon;
               // TODO_RUST_CODE
            }
         }
         // Java: TIFFField tileOffsetsField = dir.getField(273);
         // TODO_RUST_CODE
         // Java: if (tileOffsetsField != null)
            // TODO_RUST_CODE
         {
            // Java: this.tileOffsets = this.getFieldAsLongs(tileOffsetsField);
            // TODO_RUST_CODE
         }
         // Java: TIFFField tileByteCountsField = dir.getField(279);
         // TODO_RUST_CODE
         // Java: if (tileByteCountsField != null)
            // TODO_RUST_CODE
         {
            // Java: this.tileByteCounts = this.getFieldAsLongs(tileByteCountsField);
            // TODO_RUST_CODE
         }
      }
      // Java: this.tilesX = (this.bounds.width() + this.tileWidth - 1) / this.tileWidth;
      // TODO_RUST_CODE
      // Java: this.tilesY = (this.bounds.height() + this.tileHeight - 1) / this.tileHeight;
      // TODO_RUST_CODE
    }

    // Java: void checkSampleFormat(TIFFDirectory dir)
    pub fn checkSampleFormat(&self, _dir: i32) {
      // Java: TIFFField sampleFormatField = dir.getField(339);
      // TODO_RUST_CODE
      // Java: char[] sampleFormat;
      // TODO_RUST_CODE
      // Java: if (sampleFormatField != null)
        // TODO_RUST_CODE
      {
         // Java: sampleFormat = sampleFormatField.getAsChars();
         // TODO_RUST_CODE
         // Java: for(int l = 1; l < sampleFormat.length; ++l)
            // TODO_RUST_CODE
         {
            // Java: if (sampleFormat[l] != sampleFormat[0])
                // TODO_RUST_CODE
            {
            }
         }
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: sampleFormat = new char[]{'\u0001'};
         // TODO_RUST_CODE
      }
      // Java: this.samplesFormat = sampleFormat[0];
      // TODO_RUST_CODE
    }

    // Java: void initSampleSize(TIFFDirectory dir)
    pub fn initSampleSize(&self, _dir: i32) {
      // Java: TIFFField bitsField = dir.getField(258);
      // TODO_RUST_CODE
      // Java: char[] bitsPerSample = null;
      // TODO_RUST_CODE
      // Java: if (bitsField != null)
        // TODO_RUST_CODE
      {
         // Java: bitsPerSample = bitsField.getAsChars();
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: bitsPerSample = new char[]{'\u0001'};
         // TODO_RUST_CODE
         // Java: for(int i = 1; i < bitsPerSample.length; ++i)
            // TODO_RUST_CODE
         {
            // Java: if (bitsPerSample[i] != bitsPerSample[0])
                // TODO_RUST_CODE
            {
            }
         }
      }
      // Java: this.sampleSize = bitsPerSample[0];
      // TODO_RUST_CODE
    }

    // Java: void newImageRender()
    pub fn newImageRender(&self) {
      // Java: switch (this.imageType)
        // TODO_RUST_CODE
      {
         // Java: case 0:
         // TODO_RUST_CODE
            // Java: this.imageRendered = new BILEVEL(this);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 1:
         // TODO_RUST_CODE
            // Java: this.imageRendered = new GRAY(this);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 2:
         // TODO_RUST_CODE
            // Java: this.imageRendered = new GRAY_ALPHA(this);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 3:
         // TODO_RUST_CODE
            // Java: this.imageRendered = new PALETTE(this);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 4:
         // TODO_RUST_CODE
            // Java: this.imageRendered = new RGB(this);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 5:
         // TODO_RUST_CODE
            // Java: this.imageRendered = new RGB_ALPHA(this);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 6:
         // TODO_RUST_CODE
            // Java: this.imageRendered = new CMYK(this);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 7:
         // TODO_RUST_CODE
            // Java: this.imageRendered = new YCBCR_SUB(this);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 8:
         // TODO_RUST_CODE
            // Java: this.imageRendered = new LOGL(this);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 9:
         // TODO_RUST_CODE
            // Java: this.imageRendered = new LOGLUV(this);
            // TODO_RUST_CODE
      }
    }

    // Java: void initImageType(TIFFDirectory dir)
    pub fn initImageType(&self, _dir: i32) {
      // Java: int photometricType = (int)dir.getFieldAsLong(262);
      // TODO_RUST_CODE
      // Java: this.imageType = -1;
      // TODO_RUST_CODE
      // Java: switch (photometricType)
        // TODO_RUST_CODE
      {
         // Java: case 0:
         // TODO_RUST_CODE
            // Java: this.isWhiteZero = true;
            // TODO_RUST_CODE
         // Java: case 1:
         // TODO_RUST_CODE
            // Java: if (this.samplesPerPixel == 1)
            // TODO_RUST_CODE
            {
               // Java: if (this.sampleSize == 1)
                // TODO_RUST_CODE
               {
                  // Java: this.imageType = 0;
                  // TODO_RUST_CODE
               }
               // Java: else
                // TODO_RUST_CODE
               {
                  // Java: this.imageType = 1;
                  // TODO_RUST_CODE
               }
            }
            // Java: else if (this.samplesPerPixel == 2)
            // TODO_RUST_CODE
            {
               // Java: this.imageType = 2;
               // TODO_RUST_CODE
            }
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 2:
         // TODO_RUST_CODE
            // Java: if (this.samplesPerPixel == 3)
            // TODO_RUST_CODE
            {
               // Java: this.imageType = 4;
               // TODO_RUST_CODE
            }
            // Java: else if (this.samplesPerPixel == 4)
            // TODO_RUST_CODE
            {
               // Java: this.imageType = 5;
               // TODO_RUST_CODE
            }
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 3:
         // TODO_RUST_CODE
            // Java: if (this.samplesPerPixel == 1)
            // TODO_RUST_CODE
            {
               // Java: this.imageType = 3;
               // TODO_RUST_CODE
               // Java: this.initRGBColormap(dir);
               // TODO_RUST_CODE
            }
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 4:
         // TODO_RUST_CODE
            // Java: if (this.sampleSize == 1 && this.samplesPerPixel == 1)
            // TODO_RUST_CODE
            {
               // Java: this.imageType = 0;
               // TODO_RUST_CODE
            }
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 5:
         // TODO_RUST_CODE
            // Java: this.imageType = 6;
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 6:
         // TODO_RUST_CODE
            // Java: if (this.compression == 7 && this.sampleSize == 8 && this.samplesPerPixel == 3)
            // TODO_RUST_CODE
            {
               // Java: this.colorConvertJPEG = this.param.getJPEGDecompressYCbCrToRGB();
               // TODO_RUST_CODE
               // Java: this.imageType = 4;
               // TODO_RUST_CODE
            }
            // Java: else
            // TODO_RUST_CODE
            {
               // Java: TIFFField chromaField = dir.getField(530);
               // TODO_RUST_CODE
               // Java: if (chromaField != null)
                // TODO_RUST_CODE
               {
                  // Java: this.chromaSubH = chromaField.getAsInt(0);
                  // TODO_RUST_CODE
                  // Java: this.chromaSubV = chromaField.getAsInt(1);
                  // TODO_RUST_CODE
               }
               // Java: else
                // TODO_RUST_CODE
               {
                  // Java: this.chromaSubH = this.chromaSubV = 2;
                  // TODO_RUST_CODE
               }
               // Java: if (this.chromaSubH * this.chromaSubV != 1 && this.sampleSize == 8 && this.samplesPerPixel == 3)
                // TODO_RUST_CODE
               {
                  // Java: this.imageType = 7;
                  // TODO_RUST_CODE
               }
            }
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 32844:
         // TODO_RUST_CODE
            // Java: if (this.compression == 34676 && (this.sampleSize == 16 || this.sampleSize == 32) && this.samplesPerPixel == 1)
            // TODO_RUST_CODE
            {
               // Java: this.imageType = 8;
               // TODO_RUST_CODE
            }
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 32845:
         // TODO_RUST_CODE
            // Java: if (this.compression == 34677 && this.sampleSize == 24 && this.samplesPerPixel == 3)
            // TODO_RUST_CODE
            {
               // Java: this.imageType = 9;
               // TODO_RUST_CODE
            }
      }
      // Java: if (this.imageType == -1)
        // TODO_RUST_CODE
      {
      }
    }

    // Java: void initRGBColormap(TIFFDirectory dir)
    pub fn initRGBColormap(&self, _dir: i32) {
      // Java: TIFFField cfield = dir.getField(320);
      // TODO_RUST_CODE
      // Java: if (cfield != null)
        // TODO_RUST_CODE
      {
         // Java: this.colormap = cfield.getAsChars();
         // TODO_RUST_CODE
      }
      // Java: int bandLength = this.colormap.length / 3;
      // TODO_RUST_CODE
      // Java: this.r = new byte[bandLength];
      // TODO_RUST_CODE
      // Java: this.g = new byte[bandLength];
      // TODO_RUST_CODE
      // Java: this.b = new byte[bandLength];
      // TODO_RUST_CODE
      // Java: int gIndex = bandLength;
      // TODO_RUST_CODE
      // Java: int bIndex = bandLength * 2;
      // TODO_RUST_CODE
      // Java: if (this.samplesFormat == 2)
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < bandLength; ++i)
            // TODO_RUST_CODE
         {
            // Java: this.r[i] = this.param.decodeSigned16BitsTo8Bits((short)this.colormap[i]);
            // TODO_RUST_CODE
            // Java: this.g[i] = this.param.decodeSigned16BitsTo8Bits((short)this.colormap[gIndex + i]);
            // TODO_RUST_CODE
            // Java: this.b[i] = this.param.decodeSigned16BitsTo8Bits((short)this.colormap[bIndex + i]);
            // TODO_RUST_CODE
         }
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < bandLength; ++i)
            // TODO_RUST_CODE
         {
            // Java: this.r[i] = this.param.decode16BitsTo8Bits(this.colormap[i] & '\uffff');
            // TODO_RUST_CODE
            // Java: this.g[i] = this.param.decode16BitsTo8Bits(this.colormap[gIndex + i] & '\uffff');
            // TODO_RUST_CODE
            // Java: this.b[i] = this.param.decode16BitsTo8Bits(this.colormap[bIndex + i] & '\uffff');
            // TODO_RUST_CODE
         }
      }
    }

    // Java: void initCompression(TIFFDirectory dir)
    pub fn initCompression(&self, _dir: i32) {
      // Java: TIFFField compField = dir.getField(259);
      // TODO_RUST_CODE
      // Java: this.compression = compField == null ? -1 : compField.getAsInt(0);
      // TODO_RUST_CODE
      // Java: switch (this.compression)
        // TODO_RUST_CODE
      {
         // Java: case 1:
         // TODO_RUST_CODE
         // Java: case 6:
         // TODO_RUST_CODE
         // Java: case 7:
         // TODO_RUST_CODE
         // Java: case 32773:
         // TODO_RUST_CODE
         // Java: case 34676:
         // TODO_RUST_CODE
         // Java: case 34677:
         // TODO_RUST_CODE
         // Java: default:
         // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 2:
         // TODO_RUST_CODE
         // Java: case 3:
         // TODO_RUST_CODE
         // Java: case 4:
         // TODO_RUST_CODE
            // Java: if (this.sampleSize != 1)
            // TODO_RUST_CODE
            {
            }
            // Java: if (this.compression == 3)
            // TODO_RUST_CODE
            {
               // Java: TIFFField t4OptionsField = dir.getField(292);
               // TODO_RUST_CODE
               // Java: if (t4OptionsField != null)
                // TODO_RUST_CODE
               {
                  // Java: this.tiffT4Options = t4OptionsField.getAsLong(0);
                  // TODO_RUST_CODE
               }
               // Java: else
                // TODO_RUST_CODE
               {
                  // Java: this.tiffT4Options = 0L;
                  // TODO_RUST_CODE
               }
            }
            // Java: if (this.compression == 4)
            // TODO_RUST_CODE
            {
               // Java: TIFFField t6OptionsField = dir.getField(293);
               // TODO_RUST_CODE
               // Java: if (t6OptionsField != null)
                // TODO_RUST_CODE
               {
                  // Java: this.tiffT6Options = t6OptionsField.getAsLong(0);
                  // TODO_RUST_CODE
               }
               // Java: else
                // TODO_RUST_CODE
               {
                  // Java: this.tiffT6Options = 0L;
                  // TODO_RUST_CODE
               }
            }
            // Java: TIFFField fillOrderField = dir.getField(266);
            // TODO_RUST_CODE
            // Java: if (fillOrderField != null)
            // TODO_RUST_CODE
            {
               // Java: this.fillOrder = fillOrderField.getAsInt(0);
               // TODO_RUST_CODE
            }
            // Java: else
            // TODO_RUST_CODE
            {
               // Java: this.fillOrder = 1;
               // TODO_RUST_CODE
            }
            // Java: this.decoder = new FaxDecoder(this.fillOrder, this.tileWidth, this.tileHeight);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 5:
         // TODO_RUST_CODE
            // Java: TIFFField predictorField = dir.getField(317);
            // TODO_RUST_CODE
            // Java: this.predictor = predictorField == null ? 1 : predictorField.getAsInt(0);
            // TODO_RUST_CODE
            // Java: this.lzwDecoder = new LZWDecoder(this.tileWidth, this.predictor, this.samplesPerPixel);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 8:
         // TODO_RUST_CODE
         // Java: case 32946:
         // TODO_RUST_CODE
            // Java: this.inflater = new Inflater();
            // TODO_RUST_CODE
      }
    }

    // Java: TIFFDirectory getPrivateIFD(int offset)
    pub fn getPrivateIFD(&self, offset: i32) -> TIFFDirectory {
      // Java: return new TIFFDirectory(this.stream, offset, 0);
      // TODO_RUST_CODE
    }

    // Java: TIFFTile getTile(int tileX, int tileY)
    pub fn getTile(&self, tileX: i32, tileY: i32) -> TIFFTile {
      // Java: if (tileX >= 0 && tileX < this.tilesX && tileY >= 0 && tileY >= this.tilesY)
        // TODO_RUST_CODE
      {
      }
      // Java: int width = tileX == this.tilesX - 1 ? this.bounds.width() - tileX * this.tileWidth : this.tileWidth;
      // TODO_RUST_CODE
      // Java: int height = tileY == this.tilesY - 1 ? this.bounds.height() - tileY * this.tileHeight : this.tileHeight;
      // TODO_RUST_CODE
      // Java: int save_offset = this.stream.getCurrentPos();
      // TODO_RUST_CODE
      // Java: int bytesInThisTile;
      // TODO_RUST_CODE
      // Java: int lastEightBits;
      // TODO_RUST_CODE
      // Java: if ((lastEightBits = this.tileWidth * this.sampleSize % 8) == 0)
        // TODO_RUST_CODE
      {
         // Java: bytesInThisTile = width * this.sampleSize / 8 * height * this.numBands;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: bytesInThisTile = (width * this.sampleSize / 8 + 1) * height * this.numBands;
         // TODO_RUST_CODE
      }
      // Java: int noUseBits = 8 - lastEightBits;
      // TODO_RUST_CODE
      // Java: TIFFTile tile = new TIFFTile(tileX, tileY, width, height, noUseBits);
      // TODO_RUST_CODE
      // Java: if (this.isPlanarSeparate && (this.imageType == 4 || this.imageType == 6))
        // TODO_RUST_CODE
      {
         // Java: int tileIdx = tileY * this.tilesX + tileX;
         // TODO_RUST_CODE
         // Java: int idx = 0;
         // TODO_RUST_CODE
         // Java: int[] offset = new int[this.numBands];
         // TODO_RUST_CODE
         // Java: for(int i = 0; i < this.numBands; ++i)
            // TODO_RUST_CODE
         {
            // Java: offset[i] = (int)this.tileOffsets[idx + tileIdx];
            // TODO_RUST_CODE
            // Java: idx += this.tilesX * this.tilesY;
            // TODO_RUST_CODE
         }
         // Java: this.imageRendered.decoder(this.stream.getByte(), offset, this.numBands, bytesInThisTile);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this.imageRendered.decoder(this.stream.getByte(), (int)this.tileOffsets[tileY * this.tilesX + tileX], (int)this.tileByteCounts[tileY * this.tilesX + tileX], bytesInThisTile);
         // TODO_RUST_CODE
      }
      // Java: this.stream.setCurrentPos(save_offset);
      // TODO_RUST_CODE
      // Java: this.imageRendered.rendered(tile);
      // TODO_RUST_CODE
      // Java: return tile;
      // TODO_RUST_CODE
    }

    // Java: void readShorts(int shortCount, short[] shortArray)
    pub fn readShorts(&self, shortCount: i32, _shortArray: Vec<i16>) {
      // Java: int byteCount = 2 * shortCount;
      // TODO_RUST_CODE
      // Java: byte[] byteArray = new byte[byteCount];
      // TODO_RUST_CODE
      // Java: this.stream.read(byteArray);
      // TODO_RUST_CODE
      // Java: this.interpretBytesAsShorts(byteArray, shortArray, shortCount);
      // TODO_RUST_CODE
    }

    // Java: void readInts(int intCount, int[] intArray)
    pub fn readInts(&self, intCount: i32, _intArray: Vec<i32>) {
      // Java: int byteCount = 4 * intCount;
      // TODO_RUST_CODE
      // Java: byte[] byteArray = new byte[byteCount];
      // TODO_RUST_CODE
      // Java: this.stream.read(byteArray);
      // TODO_RUST_CODE
      // Java: this.interpretBytesAsInts(byteArray, intArray, intCount);
      // TODO_RUST_CODE
    }

    // Java: void interpretBytesAsShorts(byte[] byteArray, short[] shortArray, int shortCount)
    pub fn interpretBytesAsShorts(&self, _byteArray: Vec<u8>, _shortArray: Vec<i16>, shortCount: i32) {
      // Java: int j = 0;
      // TODO_RUST_CODE
      // Java: if (this.isBigEndian)
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < shortCount; ++i)
            // TODO_RUST_CODE
         {
            // Java: int firstByte = byteArray[j++] & 255;
            // TODO_RUST_CODE
            // Java: int secondByte = byteArray[j++] & 255;
            // TODO_RUST_CODE
            // Java: shortArray[i] = (short)((firstByte << 8) + secondByte);
            // TODO_RUST_CODE
         }
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < shortCount; ++i)
            // TODO_RUST_CODE
         {
            // Java: int firstByte = byteArray[j++] & 255;
            // TODO_RUST_CODE
            // Java: int secondByte = byteArray[j++] & 255;
            // TODO_RUST_CODE
            // Java: shortArray[i] = (short)((secondByte << 8) + firstByte);
            // TODO_RUST_CODE
         }
      }
    }

    // Java: void interpretBytesAsInts(byte[] byteArray, int[] intArray, int intCount)
    pub fn interpretBytesAsInts(&self, _byteArray: Vec<u8>, _intArray: Vec<i32>, intCount: i32) {
      // Java: int j = 0;
      // TODO_RUST_CODE
      // Java: if (this.isBigEndian)
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < intCount; ++i)
            // TODO_RUST_CODE
         {
            // Java: intArray[i] = (byteArray[j++] & 255) << 24 | (byteArray[j++] & 255) << 16 | (byteArray[j++] & 255) << 8 | byteArray[j++] & 255;
            // TODO_RUST_CODE
         }
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < intCount; ++i)
            // TODO_RUST_CODE
         {
            // Java: intArray[i] = byteArray[j++] & 255 | (byteArray[j++] & 255) << 8 | (byteArray[j++] & 255) << 16 | (byteArray[j++] & 255) << 24;
            // TODO_RUST_CODE
         }
      }
    }

    // Java: void close()
    pub fn close(&self) {
      // Java: this.stream.close();
      // TODO_RUST_CODE
      // Java: if (this.r != null && this.g != null && this.b != null)
        // TODO_RUST_CODE
      {
         // Java: this.r = this.g = this.b = null;
         // TODO_RUST_CODE
      }
      // Java: this.imageRendered.free();
      // TODO_RUST_CODE
    }

}
