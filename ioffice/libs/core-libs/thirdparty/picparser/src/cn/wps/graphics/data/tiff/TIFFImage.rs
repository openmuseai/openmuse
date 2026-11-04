#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class TIFFImage
pub struct TIFFImage {
    // Java field: int TYPE_UNSUPPORTED
    // Java field: int TYPE_BILEVEL
    // Java field: int TYPE_GRAY
    // Java field: int TYPE_GRAY_ALPHA
    // Java field: int TYPE_PALETTE
    // Java field: int TYPE_RGB
    // Java field: int TYPE_RGB_ALPHA
    // Java field: int TYPE_CMYK
    // Java field: int TYPE_YCBCR_SUB
    // Java field: int TYPE_LOGL
    // Java field: int TYPE_LOGLUV
    // Java field: int TIFF_JPEG_TABLES
    // Java field: int TIFF_YCBCR_SUBSAMPLING
    // Java field: BufferReader stream
    pub stream: Option<i32>,
    // Java field: TIFFDecodeParam param
    pub param: Option<i32>,
    // Java field: TIFFImageType imageRendered
    pub imageRendered: Option<i32>,
    // Java field: Rect bounds
    pub bounds: Option<i32>,
    // Java field: int tileWidth
    pub tileWidth: i32,
    // Java field: int tileHeight
    pub tileHeight: i32,
    // Java field: int tilesX
    pub tilesX: i32,
    // Java field: int tilesY
    pub tilesY: i32,
    // Java field: long[] tileOffsets
    pub tileOffsets: Option<i32>,
    // Java field: long[] tileByteCounts
    pub tileByteCounts: Option<i32>,
    // Java field: int sampleSize
    pub sampleSize: i32,
    // Java field: int samplesFormat
    pub samplesFormat: i32,
    // Java field: int numBands
    pub numBands: i32,
    // Java field: int samplesPerPixel
    pub samplesPerPixel: i32,
    // Java field: int imageType
    pub imageType: i32,
    // Java field: char[] colormap
    pub colormap: Option<i32>,
    // Java field: byte[] r
    pub r: Option<i32>,
    // Java field: byte[] g
    pub g: Option<i32>,
    // Java field: byte[] b
    pub b: Option<i32>,
    // Java field: int chromaSubH
    pub chromaSubH: i32,
    // Java field: int chromaSubV
    pub chromaSubV: i32,
    // Java field: int compression
    pub compression: i32,
    // Java field: long tiffT4Options
    pub tiffT4Options: i64,
    // Java field: long tiffT6Options
    pub tiffT6Options: i64,
    // Java field: int fillOrder
    pub fillOrder: i32,
    // Java field: LZWDecoder lzwDecoder
    pub lzwDecoder: Option<i32>,
    // Java field: int predictor
    pub predictor: i32,
    // Java field: boolean colorConvertJPEG
    pub colorConvertJPEG: bool,
    // Java field: Inflater inflater
    pub inflater: Option<i32>,
    // Java field: FaxDecoder decoder
    pub decoder: Option<i32>,
    // Java field: boolean isBigEndian
    pub isBigEndian: bool,
    // Java field: boolean isWhiteZero
    pub isWhiteZero: bool,
    // Java field: boolean isPlanarSeparate
    pub isPlanarSeparate: bool,
    // Java field: boolean tiled
    pub tiled: bool,
}

const TYPE_UNSUPPORTED: i32 = -1;
const TYPE_BILEVEL: i32 = 0;
const TYPE_GRAY: i32 = 1;
const TYPE_GRAY_ALPHA: i32 = 2;
const TYPE_PALETTE: i32 = 3;
const TYPE_RGB: i32 = 4;
const TYPE_RGB_ALPHA: i32 = 5;
const TYPE_CMYK: i32 = 6;
const TYPE_YCBCR_SUB: i32 = 7;
const TYPE_LOGL: i32 = 8;
const TYPE_LOGLUV: i32 = 9;
const TIFF_JPEG_TABLES: i32 = 347;
const TIFF_YCBCR_SUBSAMPLING: i32 = 530;

impl TIFFImage {
    // Java: private long[] getFieldAsLongs(TIFFField field)
    fn getFieldAsLongs(&self, _field: i32) -> i32 {
        // Java: long[] value = null;
        // Java: if (field.getType() == 3)
        {
            // Java: char[] charValue = field.getAsChars();
            // Java: value = new long[charValue.length];
            // Java: for(int i = 0; i < charValue.length; ++i)
            {
                // Java: value[i] = (long)(charValue[i] & '\uffff');
            }
        }
        // Java: else
        {
            // Java: if (field.getType() != 4)
            {
                // Java: throw new RuntimeException();
            }
            // Java: value = field.getAsLongs();
        }
        // Java: return value;
        0
    }

    // Java: TIFFImage(BufferReader stream, TIFFDecodeParam param, int directory)
    pub fn new() -> Self {
        // Java: super();
        // Java: this.stream = stream;
        // Java: if (param == null)
        {
            // Java: param = new TIFFDecodeParam();
        }
        // Java: else
        {
            // Java: this.param = param;
        }
        // Java: TIFFDirectory dir = param.getIFDOffset() == -1 ? new TIFFDirectory(stream, directory) : new TIFFDirectory(stream, param.getIFDOffset(), directory);
        // Java: this.bounds = new Rect(0, 0, (int)dir.getFieldAsLong(256), (int)dir.getFieldAsLong(257));
        // Java: this.initTile(dir);
        // Java: TIFFField sfield = dir.getField(277);
        // Java: this.samplesPerPixel = sfield == null ? 1 : (int)sfield.getAsLong(0);
        // Java: this.checkPlanarConfiguration(dir);
        // Java: this.initSampleSize(dir);
        // Java: this.checkSampleFormat(dir);
        // Java: TIFFField efield = dir.getField(338);
        // Java: if (efield == null)
        {
            // Java: boolean var10000 = false;
        }
        // Java: else
        {
            // Java: int var8 = (int)efield.getAsLong(0);
        }
        // Java: this.numBands = this.samplesPerPixel;
        // Java: this.initCompression(dir);
        // Java: this.initImageType(dir);
        // Java: this.isBigEndian = dir.isBigEndian();
        // Java: this.newImageRender();
        TIFFImage {
            stream: None,
            param: None,
            imageRendered: None,
            bounds: None,
            tileWidth: 0,
            tileHeight: 0,
            tilesX: 0,
            tilesY: 0,
            tileOffsets: None,
            tileByteCounts: None,
            sampleSize: 0,
            samplesFormat: 0,
            numBands: 0,
            samplesPerPixel: 0,
            imageType: 0,
            colormap: None,
            r: None,
            g: None,
            b: None,
            chromaSubH: 0,
            chromaSubV: 0,
            compression: 0,
            tiffT4Options: 0,
            tiffT6Options: 0,
            fillOrder: 0,
            lzwDecoder: None,
            predictor: 0,
            colorConvertJPEG: false,
            inflater: None,
            decoder: None,
            isBigEndian: false,
            isWhiteZero: false,
            isPlanarSeparate: false,
            tiled: false,
        }
    }

    // Java: private void checkPlanarConfiguration(TIFFDirectory dir)
    fn checkPlanarConfiguration(&mut self) {
        // Java: TIFFField planarConfigurationField = dir.getField(284);
        // Java: char[] planarConfiguration = planarConfigurationField == null ? new char[]{'\u0001'} : planarConfigurationField.getAsChars();
        // Java: this.isPlanarSeparate = planarConfiguration[0] == 2;
    }

    // Java: private void initTile(TIFFDirectory dir)
    fn initTile(&mut self) {
        // Java: if (dir.getField(324) != null)
        {
            // Java: this.tiled = true;
            // Java: this.tileWidth = (int)dir.getFieldAsLong(322);
            // Java: this.tileHeight = (int)dir.getFieldAsLong(323);
            // Java: this.tileOffsets = dir.getField(324).getAsLongs();
            // Java: this.tileByteCounts = this.getFieldAsLongs(dir.getField(325));
        }
        // Java: else
        {
            // Java: this.tiled = false;
            // Java: this.tileWidth = dir.getField(322) != null ? (int)dir.getFieldAsLong(322) : this.bounds.width();
            // Java: TIFFField field = dir.getField(278);
            // Java: if (field == null)
            {
                // Java: this.tileHeight = dir.getField(323) != null ? (int)dir.getFieldAsLong(323) : this.bounds.height();
            }
            // Java: else
            {
                // Java: long lon = field.getAsLong(0);
                // Java: long infinity = 1L;
                // Java: infinity = (infinity << 32) - 1L;
                // Java: if (lon == (long)((int)infinity))
                {
                    // Java: this.tileHeight = this.bounds.height();
                }
                // Java: else
                {
                    // Java: this.tileHeight = (int)lon;
                }
            }
            // Java: TIFFField tileOffsetsField = dir.getField(273);
            // Java: if (tileOffsetsField != null)
            {
                // Java: this.tileOffsets = this.getFieldAsLongs(tileOffsetsField);
            }
            // Java: TIFFField tileByteCountsField = dir.getField(279);
            // Java: if (tileByteCountsField != null)
            {
                // Java: this.tileByteCounts = this.getFieldAsLongs(tileByteCountsField);
            }
        }
        // Java: this.tilesX = (this.bounds.width() + this.tileWidth - 1) / this.tileWidth;
        // Java: this.tilesY = (this.bounds.height() + this.tileHeight - 1) / this.tileHeight;
    }

    // Java: private void checkSampleFormat(TIFFDirectory dir)
    fn checkSampleFormat(&mut self) {
        // Java: TIFFField sampleFormatField = dir.getField(339);
        // Java: char[] sampleFormat;
        // Java: if (sampleFormatField != null)
        {
            // Java: sampleFormat = sampleFormatField.getAsChars();
            // Java: for(int l = 1; l < sampleFormat.length; ++l)
            {
                // Java: if (sampleFormat[l] != sampleFormat[0])
                {
                }
            }
        }
        // Java: else
        {
            // Java: sampleFormat = new char[]{'\u0001'};
        }
        // Java: this.samplesFormat = sampleFormat[0];
    }

    // Java: private void initSampleSize(TIFFDirectory dir)
    fn initSampleSize(&mut self) {
        // Java: TIFFField bitsField = dir.getField(258);
        // Java: char[] bitsPerSample = null;
        // Java: if (bitsField != null)
        {
            // Java: bitsPerSample = bitsField.getAsChars();
        }
        // Java: else
        {
            // Java: bitsPerSample = new char[]{'\u0001'};
            // Java: for(int i = 1; i < bitsPerSample.length; ++i)
            {
                // Java: if (bitsPerSample[i] != bitsPerSample[0])
                {
                }
            }
        }
        // Java: this.sampleSize = bitsPerSample[0];
    }

    // Java: private void newImageRender()
    fn newImageRender(&mut self) {
        // Java: switch (this.imageType)
        {
            // Java: case 0:
            // Java: this.imageRendered = new BILEVEL(this);
            // Java: break;
            // Java: case 1:
            // Java: this.imageRendered = new GRAY(this);
            // Java: break;
            // Java: case 2:
            // Java: this.imageRendered = new GRAY_ALPHA(this);
            // Java: break;
            // Java: case 3:
            // Java: this.imageRendered = new PALETTE(this);
            // Java: break;
            // Java: case 4:
            // Java: this.imageRendered = new RGB(this);
            // Java: break;
            // Java: case 5:
            // Java: this.imageRendered = new RGB_ALPHA(this);
            // Java: break;
            // Java: case 6:
            // Java: this.imageRendered = new CMYK(this);
            // Java: break;
            // Java: case 7:
            // Java: this.imageRendered = new YCBCR_SUB(this);
            // Java: break;
            // Java: case 8:
            // Java: this.imageRendered = new LOGL(this);
            // Java: break;
            // Java: case 9:
            // Java: this.imageRendered = new LOGLUV(this);
        }
    }

    // Java: private void initImageType(TIFFDirectory dir)
    fn initImageType(&mut self) {
        // Java: int photometricType = (int)dir.getFieldAsLong(262);
        // Java: this.imageType = -1;
        // Java: switch (photometricType)
        {
            // Java: case 0:
            // Java: this.isWhiteZero = true;
            // Java: case 1:
            // Java: if (this.samplesPerPixel == 1)
            {
                // Java: if (this.sampleSize == 1)
                {
                    // Java: this.imageType = 0;
                }
                // Java: else
                {
                    // Java: this.imageType = 1;
                }
            }
            // Java: else if (this.samplesPerPixel == 2)
            {
                // Java: this.imageType = 2;
            }
            // Java: break;
            // Java: case 2:
            // Java: if (this.samplesPerPixel == 3)
            {
                // Java: this.imageType = 4;
            }
            // Java: else if (this.samplesPerPixel == 4)
            {
                // Java: this.imageType = 5;
            }
            // Java: break;
            // Java: case 3:
            // Java: if (this.samplesPerPixel == 1)
            {
                // Java: this.imageType = 3;
                // Java: this.initRGBColormap(dir);
            }
            // Java: break;
            // Java: case 4:
            // Java: if (this.sampleSize == 1 && this.samplesPerPixel == 1)
            {
                // Java: this.imageType = 0;
            }
            // Java: break;
            // Java: case 5:
            // Java: this.imageType = 6;
            // Java: break;
            // Java: case 6:
            // Java: if (this.compression == 7 && this.sampleSize == 8 && this.samplesPerPixel == 3)
            {
                // Java: this.colorConvertJPEG = this.param.getJPEGDecompressYCbCrToRGB();
                // Java: this.imageType = 4;
            }
            // Java: else
            {
                // Java: TIFFField chromaField = dir.getField(530);
                // Java: if (chromaField != null)
                {
                    // Java: this.chromaSubH = chromaField.getAsInt(0);
                    // Java: this.chromaSubV = chromaField.getAsInt(1);
                }
                // Java: else
                {
                    // Java: this.chromaSubH = this.chromaSubV = 2;
                }
                // Java: if (this.chromaSubH * this.chromaSubV != 1 && this.sampleSize == 8 && this.samplesPerPixel == 3)
                {
                    // Java: this.imageType = 7;
                }
            }
            // Java: break;
            // Java: case 32844:
            // Java: if (this.compression == 34676 && (this.sampleSize == 16 || this.sampleSize == 32) && this.samplesPerPixel == 1)
            {
                // Java: this.imageType = 8;
            }
            // Java: break;
            // Java: case 32845:
            // Java: if (this.compression == 34677 && this.sampleSize == 24 && this.samplesPerPixel == 3)
            {
                // Java: this.imageType = 9;
            }
        }
        // Java: if (this.imageType == -1)
        {
        }
    }

    // Java: private void initRGBColormap(TIFFDirectory dir)
    fn initRGBColormap(&mut self) {
        // Java: TIFFField cfield = dir.getField(320);
        // Java: if (cfield != null)
        {
            // Java: this.colormap = cfield.getAsChars();
        }
        // Java: int bandLength = this.colormap.length / 3;
        // Java: this.r = new byte[bandLength];
        // Java: this.g = new byte[bandLength];
        // Java: this.b = new byte[bandLength];
        // Java: int gIndex = bandLength;
        // Java: int bIndex = bandLength * 2;
        // Java: if (this.samplesFormat == 2)
        {
            // Java: for(int i = 0; i < bandLength; ++i)
            {
                // Java: this.r[i] = this.param.decodeSigned16BitsTo8Bits((short)this.colormap[i]);
                // Java: this.g[i] = this.param.decodeSigned16BitsTo8Bits((short)this.colormap[gIndex + i]);
                // Java: this.b[i] = this.param.decodeSigned16BitsTo8Bits((short)this.colormap[bIndex + i]);
            }
        }
        // Java: else
        {
            // Java: for(int i = 0; i < bandLength; ++i)
            {
                // Java: this.r[i] = this.param.decode16BitsTo8Bits(this.colormap[i] & '\uffff');
                // Java: this.g[i] = this.param.decode16BitsTo8Bits(this.colormap[gIndex + i] & '\uffff');
                // Java: this.b[i] = this.param.decode16BitsTo8Bits(this.colormap[bIndex + i] & '\uffff');
            }
        }
    }

    // Java: private void initCompression(TIFFDirectory dir)
    fn initCompression(&mut self) {
        // Java: TIFFField compField = dir.getField(259);
        // Java: this.compression = compField == null ? -1 : compField.getAsInt(0);
        // Java: switch (this.compression)
        {
            // Java: case 1:
            // Java: case 6:
            // Java: case 7:
            // Java: case 32773:
            // Java: case 34676:
            // Java: case 34677:
            // Java: default:
            // Java: break;
            // Java: case 2:
            // Java: case 3:
            // Java: case 4:
            // Java: if (this.sampleSize != 1)
            {
            }
            // Java: if (this.compression == 3)
            {
                // Java: TIFFField t4OptionsField = dir.getField(292);
                // Java: if (t4OptionsField != null)
                {
                    // Java: this.tiffT4Options = t4OptionsField.getAsLong(0);
                }
                // Java: else
                {
                    // Java: this.tiffT4Options = 0L;
                }
            }
            // Java: if (this.compression == 4)
            {
                // Java: TIFFField t6OptionsField = dir.getField(293);
                // Java: if (t6OptionsField != null)
                {
                    // Java: this.tiffT6Options = t6OptionsField.getAsLong(0);
                }
                // Java: else
                {
                    // Java: this.tiffT6Options = 0L;
                }
            }
            // Java: TIFFField fillOrderField = dir.getField(266);
            // Java: if (fillOrderField != null)
            {
                // Java: this.fillOrder = fillOrderField.getAsInt(0);
            }
            // Java: else
            {
                // Java: this.fillOrder = 1;
            }
            // Java: this.decoder = new FaxDecoder(this.fillOrder, this.tileWidth, this.tileHeight);
            // Java: break;
            // Java: case 5:
            // Java: TIFFField predictorField = dir.getField(317);
            // Java: this.predictor = predictorField == null ? 1 : predictorField.getAsInt(0);
            // Java: this.lzwDecoder = new LZWDecoder(this.tileWidth, this.predictor, this.samplesPerPixel);
            // Java: break;
            // Java: case 8:
            // Java: case 32946:
            // Java: this.inflater = new Inflater();
        }
    }

    // Java: public TIFFDirectory getPrivateIFD(int offset)
    pub fn getPrivateIFD(&self, _offset: i32) -> i32 {
        // Java: return new TIFFDirectory(this.stream, offset, 0);
        0
    }

    // Java: public TIFFTile getTile(int tileX, int tileY)
    pub fn getTile(&self, _tileX: i32, _tileY: i32) -> i32 {
        // Java: if (tileX >= 0 && tileX < this.tilesX && tileY >= 0 && tileY >= this.tilesY)
        {
        }
        // Java: int width = tileX == this.tilesX - 1 ? this.bounds.width() - tileX * this.tileWidth : this.tileWidth;
        // Java: int height = tileY == this.tilesY - 1 ? this.bounds.height() - tileY * this.tileHeight : this.tileHeight;
        // Java: int save_offset = this.stream.getCurrentPos();
        // Java: int bytesInThisTile;
        // Java: int lastEightBits;
        // Java: if ((lastEightBits = this.tileWidth * this.sampleSize % 8) == 0)
        {
            // Java: bytesInThisTile = width * this.sampleSize / 8 * height * this.numBands;
        }
        // Java: else
        {
            // Java: bytesInThisTile = (width * this.sampleSize / 8 + 1) * height * this.numBands;
        }
        // Java: int noUseBits = 8 - lastEightBits;
        // Java: TIFFTile tile = new TIFFTile(tileX, tileY, width, height, noUseBits);
        // Java: if (this.isPlanarSeparate && (this.imageType == 4 || this.imageType == 6))
        {
            // Java: int tileIdx = tileY * this.tilesX + tileX;
            // Java: int idx = 0;
            // Java: int[] offset = new int[this.numBands];
            // Java: for(int i = 0; i < this.numBands; ++i)
            {
                // Java: offset[i] = (int)this.tileOffsets[idx + tileIdx];
                // Java: idx += this.tilesX * this.tilesY;
            }
            // Java: this.imageRendered.decoder(this.stream.getByte(), offset, this.numBands, bytesInThisTile);
        }
        // Java: else
        {
            // Java: this.imageRendered.decoder(this.stream.getByte(), (int)this.tileOffsets[tileY * this.tilesX + tileX], (int)this.tileByteCounts[tileY * this.tilesX + tileX], bytesInThisTile);
        }
        // Java: this.stream.setCurrentPos(save_offset);
        // Java: this.imageRendered.rendered(tile);
        // Java: return tile;
        0
    }

    // Java: public void readShorts(int shortCount, short[] shortArray)
    pub fn readShorts(&self, _shortCount: i32, _shortArray: i32) {
        // Java: int byteCount = 2 * shortCount;
        // Java: byte[] byteArray = new byte[byteCount];
        // Java: this.stream.read(byteArray);
        // Java: this.interpretBytesAsShorts(byteArray, shortArray, shortCount);
    }

    // Java: public void readInts(int intCount, int[] intArray)
    pub fn readInts(&self, _intCount: i32, _intArray: i32) {
        // Java: int byteCount = 4 * intCount;
        // Java: byte[] byteArray = new byte[byteCount];
        // Java: this.stream.read(byteArray);
        // Java: this.interpretBytesAsInts(byteArray, intArray, intCount);
    }

    // Java: public void interpretBytesAsShorts(byte[] byteArray, short[] shortArray, int shortCount)
    pub fn interpretBytesAsShorts(&self, _byteArray: i32, _shortArray: i32, _shortCount: i32) {
        // Java: int j = 0;
        // Java: if (this.isBigEndian)
        {
            // Java: for(int i = 0; i < shortCount; ++i)
            {
                // Java: int firstByte = byteArray[j++] & 255;
                // Java: int secondByte = byteArray[j++] & 255;
                // Java: shortArray[i] = (short)((firstByte << 8) + secondByte);
            }
        }
        // Java: else
        {
            // Java: for(int i = 0; i < shortCount; ++i)
            {
                // Java: int firstByte = byteArray[j++] & 255;
                // Java: int secondByte = byteArray[j++] & 255;
                // Java: shortArray[i] = (short)((secondByte << 8) + firstByte);
            }
        }
    }

    // Java: public void interpretBytesAsInts(byte[] byteArray, int[] intArray, int intCount)
    pub fn interpretBytesAsInts(&self, _byteArray: i32, _intArray: i32, _intCount: i32) {
        // Java: int j = 0;
        // Java: if (this.isBigEndian)
        {
            // Java: for(int i = 0; i < intCount; ++i)
            {
                // Java: intArray[i] = (byteArray[j++] & 255) << 24 | (byteArray[j++] & 255) << 16 | (byteArray[j++] & 255) << 8 | byteArray[j++] & 255;
            }
        }
        // Java: else
        {
            // Java: for(int i = 0; i < intCount; ++i)
            {
                // Java: intArray[i] = byteArray[j++] & 255 | (byteArray[j++] & 255) << 8 | (byteArray[j++] & 255) << 16 | (byteArray[j++] & 255) << 24;
            }
        }
    }

    // Java: public void close()
    pub fn close(&mut self) {
        // Java: this.stream.close();
        // Java: if (this.r != null && this.g != null && this.b != null)
        {
            // Java: this.r = this.g = this.b = null;
        }
        // Java: this.imageRendered.free();
    }
}
