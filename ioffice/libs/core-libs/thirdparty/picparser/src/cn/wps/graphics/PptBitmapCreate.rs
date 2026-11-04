#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class PptBitmapCreate implements IBitmapCreator
pub struct PptBitmapCreate {
    // Java field: String TAG
    // Java field: IMemoryHelper memoryHelper
    memoryHelper: Option<i32>,
    // Java field: Bitmap.Config config
    config: Option<i32>,
}

impl PptBitmapCreate {
    // Java: PptBitmapCreate()
    pub fn new() -> Self {
        // Java: this((IMemoryHelper)null);
        PptBitmapCreate {
            memoryHelper: None,
            config: None,
        }
    }

    // Java: public void setConfig(Bitmap.Config config)
    pub fn setConfig(&mut self, _config: i32) {
        // Java: this.config = config;
    }

    // Java: PptBitmapCreate(IMemoryHelper memoryHelper)
    pub fn new_2() -> Self {
        // Java: super();
        // Java: this.memoryHelper = null;
        // Java: this.config = Config.ARGB_8888;
        // Java: this.memoryHelper = memoryHelper;
        PptBitmapCreate {
            memoryHelper: None,
            config: None,
        }
    }

    // Java: public Bitmap createRegionBitmap(PictureInfo info, Rect rect, int width, int height)
    pub fn createRegionBitmap(&self, _info: i32, _rect: i32, _width: i32, _height: i32) -> i32 {
        // Java: Bitmap bitmap = null;
        // Java: if (VERSION.SDK_INT >= 10)
        {
            // Java: if (info.type != 1 && info.type != 2)
            {
                // Java: throw new IOException("bitmap decode unsupported");
            }
            // Java: BitmapRegionDecoder bitmapRegionDecoder = BitmapRegionDecoder.newInstance(info.path, false);
            // Java: BitmapFactory.Options options = new BitmapFactory.Options();
            // Java: int originWidth = info.width;
            // Java: int originHeight = info.height;
            // Java: int scale = (int)(Math.min((double)1.0F * (double)originWidth / (double)width, (double)1.0F * (double)originHeight / (double)height) + (double)0.5F);
            // Java: scale = scale < 1 ? 1 : scale;
            // Java: scale = scale > 3 ? scale - 1 : scale;
            // Java: options.inJustDecodeBounds = false;
            // Java: options.inSampleSize = scale;
            // Java: options.inPreferredConfig = this.config;
            // Java: options.inDither = true;
            // Java: bitmap = bitmapRegionDecoder.decodeRegion(rect, options);
        }
        // Java: return bitmap;
        0
    }

    // Java: public Bitmap createBitmap(PictureInfo info, int width, int height)
    pub fn createBitmap(&self, _info: i32, _width: i32, _height: i32) -> i32 {
        // Java: Bitmap bitmap = null;
        // Java: try
        {
            // Java: bitmap = this.createBitmap(info, width, height, true);
        }
        // Java: catch (IOException e)
        {
            // Java: e.printStackTrace();
        }
        // Java: return bitmap;
        0
    }

    // Java: public Bitmap createBitmap(PictureInfo info, int width, int height, boolean isAppropriate)
    pub fn createBitmap_appropriate(&self, _info: i32, _width: i32, _height: i32, _isAppropriate: bool) -> i32 {
        // Java: Bitmap bitmap = null;
        // Java: if (info.isAndroidFlie())
        {
            // Java: bitmap = this.createAndroidBitmap(info, width, height);
        }
        // Java: else if (info.isMetaFile())
        {
            // Java: bitmap = this.createMetaBitmap(info, width, height);
        }
        // Java: else if (info.type == 7)
        {
            // Java: bitmap = this.createRasterBitmap(info, width, height);
        }
        // Java: boolean isNeedScaled = VERSION.SDK_INT < 15 ? bitmap != null && !isAppropriate : false;
        // Java: if (isNeedScaled && width != bitmap.getWidth() && height != bitmap.getHeight())
        {
            // Java: try
            {
                // Java: Bitmap scaleBitmap = null;
                // Java: scaleBitmap = Bitmap.createScaledBitmap(bitmap, width, height, false);
                // Java: bitmap.recycle();
                // Java: bitmap = scaleBitmap;
            }
            // Java: catch (OutOfMemoryError var8)
            {
            }
        }
        // Java: return bitmap;
        0
    }

    // Java: public Bitmap createAndroidBitmap(PictureInfo info, int expectedWidth, int expectedHeight)
    pub fn createAndroidBitmap(&self, _info: i32, _expectedWidth: i32, _expectedHeight: i32) -> i32 {
        // Java: String path = info.path;
        // Java: int originWidth = info.width;
        // Java: int originHeight = info.height;
        // Java: FileInputStream fileStream = null;
        // Java: BitmapFactory.Options options = null;
        // Java: Bitmap bitmap = null;
        // Java: try
        {
            // Java: int scale = (int)(Math.min((double)1.0F * (double)originWidth / (double)expectedWidth, (double)1.0F * (double)originHeight / (double)expectedHeight) + (double)0.5F);
            // Java: scale = scale < 1 ? 1 : scale;
            // Java: scale = scale > 3 ? scale - 1 : scale;
            // Java: if (this.memoryHelper != null)
            {
                // Java: int size = expectedWidth * expectedHeight * 4;
                // Java: size += originWidth * originHeight * 4 / (scale * scale);
                // Java: this.memoryHelper.gcForNewObject(size);
            }
            // Java: fileStream = new FileInputStream(path);
            // Java: options = new BitmapFactory.Options();
            // Java: options.inJustDecodeBounds = false;
            // Java: options.inSampleSize = scale;
            // Java: options.inPreferredConfig = this.config;
            // Java: options.inDither = true;
            // Java: Rect outPadding = new Rect();
            // Java: bitmap = BitmapFactory.decodeStream(fileStream, outPadding, options);
            // Java: fileStream.close();
            // Java: boolean isNeedScaled = VERSION.SDK_INT < 11 ? scale < 3 : false;
            // Java: if (isNeedScaled && null != bitmap && ((double)expectedWidth < (double)bitmap.getWidth() * 0.8 || (double)expectedHeight < (double)bitmap.getHeight() * 0.8))
            {
                // Java: Bitmap scaleBitmap = null;
                // Java: scaleBitmap = Bitmap.createScaledBitmap(bitmap, expectedWidth, expectedHeight, false);
                // Java: bitmap.recycle();
                // Java: bitmap = scaleBitmap;
            }
        }
        // Java: catch (OutOfMemoryError var14)
        {
            // Java: if (this.memoryHelper != null)
            {
                // Java: this.memoryHelper.onOutOfMemory();
            }
            // Java: Rect outPadding = new Rect();
            // Java: if (options != null)
            {
                // Java: ++options.inSampleSize;
            }
            // Java: if (bitmap != null)
            {
                // Java: bitmap = BitmapFactory.decodeFileDescriptor(fileStream.getFD(), outPadding, options);
            }
        }
        // Java: return bitmap;
        0
    }

    // Java: public Bitmap createMetaBitmap(PictureInfo info, int expectedWidth, int expectedHeight)
    pub fn createMetaBitmap(&self, _info: i32, _expectedWidth: i32, _expectedHeight: i32) -> i32 {
        // Java: String path = info.path;
        // Java: int originWidth = info.width;
        // Java: int originHeight = info.height;
        // Java: float scale = (float)Math.min((double)originWidth / (double)expectedWidth, (double)originHeight / (double)expectedHeight);
        // Java: if (scale > 3.0F)
        {
            // Java: --scale;
        }
        // Java: int actualWidth = (int)((float)originWidth / scale);
        // Java: int actualHeight = (int)((float)originHeight / scale);
        // Java: if (this.memoryHelper != null)
        {
            // Java: int size = actualHeight * actualWidth * 4;
            // Java: this.memoryHelper.gcForNewObject(size);
        }
        // Java: InputStream fileStream = null;
        // Java: MetaFileParser stream = null;
        // Java: Bitmap bitmap = null;
        // Java: MetaFilePainter renderer = null;
        // Java: byte[] bytes = null;
        // Java: boolean drawBitmapSucceed = false;
        // Java: try
        {
            // Java: fileStream = new FileInputStream(path);
            // Java: bytes = new byte[fileStream.available()];
            // Java: fileStream.read(bytes);
            // Java: fileStream.close();
            // Java: stream = (MetaFileParser)(info.type == 6 ? new WMFParser(bytes) : new EMFParser(bytes));
            // Java: stream.run();
            // Java: renderer = new MetaFilePainter(stream.getHeader(), stream.getRecords());
            // Java: bitmap = Bitmap.createBitmap(actualWidth, actualHeight, this.config);
            // Java: Canvas canvas = new Canvas(bitmap);
            // Java: canvas.save();
            // Java: canvas.scale(1.0F / scale, 1.0F / scale);
            // Java: renderer.paint(canvas, (float)originWidth, (float)originHeight);
            // Java: drawBitmapSucceed = true;
            // Java: canvas.restore();
        }
        // Java: catch (Exception var20)
        {
        }
        // Java: finally
        {
            // Java: byte[] var27 = null;
            // Java: if (renderer != null)
            {
                // Java: renderer.destroy();
            }
            // Java: if (bitmap != null && !drawBitmapSucceed)
            {
                // Java: bitmap.recycle();
                // Java: Bitmap var25 = null;
                // Java: return null;
            }
        }
        // Java: return bitmap;
        0
    }

    // Java: public Bitmap createRasterBitmap(PictureInfo info, int expectedWidth, int expectedHeight)
    pub fn createRasterBitmap(&self, _info: i32, _expectedWidth: i32, _expectedHeight: i32) -> i32 {
        // Java: String path = info.path;
        // Java: int originWidth = info.width;
        // Java: int originHeight = info.height;
        // Java: float scale = (float)Math.min((double)originWidth / (double)expectedWidth, (double)originHeight / (double)expectedHeight);
        // Java: int inSample = 1;
        // Java: if (scale > 8.0F)
        {
            // Java: inSample = 8;
        }
        // Java: else if (scale > 4.0F)
        {
            // Java: inSample = 4;
        }
        // Java: else if (scale > 2.0F)
        {
            // Java: inSample = 2;
        }
        // Java: else
        {
            // Java: inSample = 1;
        }
        // Java: int actualWidth = (originWidth + inSample - 1) / inSample;
        // Java: int actualHeight = (originHeight + inSample - 1) / inSample;
        // Java: BufferReader input = null;
        // Java: TIFFDecodeParam param = null;
        // Java: TIFFImageDecoder decoder = null;
        // Java: TIFFImage image = null;
        // Java: Bitmap bitmap = null;
        // Java: boolean drawBitmapSucceed = false;
        // Java: try
        {
            // Java: InputStream is = new FileInputStream(path);
            // Java: input = new BufferReader(is);
            // Java: param = new TIFFDecodeParam();
            // Java: decoder = new TIFFImageDecoder(input, param);
            // Java: image = decoder.decodeAsRenderedImage(0);
            // Java: bitmap = Bitmap.createBitmap(actualWidth, actualHeight, this.config);
            // Java: TIFFTile tile = null;
            // Java: TIFFTile.init(inSample);
            // Java: int x = 0;
            // Java: for(int i = 0; i < image.tilesX; ++i)
            {
                // Java: int y = 0;
                // Java: for(int j = 0; j < image.tilesY; ++j)
                {
                    // Java: tile = image.getTile(i, j);
                    // Java: int stride = tile.sampledWidth;
                    // Java: int width = tile.sampledWidth;
                    // Java: int height = tile.sampledHeight;
                    // Java: int[] piexl = TIFFTile.pixels;
                    // Java: bitmap.setPixels(piexl, 0, stride, x, y, width, height);
                    // Java: y += tile.sampledHeight;
                }
                // Java: x += tile.sampledWidth;
            }
            // Java: drawBitmapSucceed = true;
        }
        // Java: catch (Exception var27)
        {
        }
        // Java: if (drawBitmapSucceed)
        {
            // Java: image.close();
            // Java: TIFFTile.init(0);
        }
        // Java: return bitmap;
        0
    }
}
