#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class PicFormatConvertUtils
pub struct PicFormatConvertUtils {
    // Java field: int WMF
    // Java field: int JPG
    // Java field: int PNG
    // Java field: int BMP
    // Java field: TextRunBmpLruCache TXT_BMP_CACHE
    _private: (),
}

// Java: public class TextRunBmpLruCache
pub struct TextRunBmpLruCache {
    // Java field: int MAX_SIZE
    _private: (),
}

// Java: public class BmpKey
pub struct BmpKey {
    // Java field: String path
    path: String,
    // Java field: int w
    w: i32,
    // Java field: int h
    h: i32,
}

impl BmpKey {
    // Java: BmpKey(String path, int w, int h)
    pub fn new(_path: String, _w: i32, _h: i32) -> Self {
        // Java: super();
        // Java: this.path = path;
        // Java: this.w = w;
        // Java: this.h = h;
        BmpKey { path: _path, w: _w, h: _h }
    }

    // Java: public int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.path.hashCode() + this.w + this.h;
        0
    }

    // Java: public boolean equals(Object o)
    pub fn equals(&self, _o: i32) -> bool {
        // Java: if (o != null && o instanceof BmpKey)
        {
            // Java: BmpKey key = (BmpKey)o;
            // Java: if (key == this)
            {
                // Java: return true;
            }
            // Java: else
            {
                // Java: return this.w == key.w && this.h == key.h && this.path.equals(key.path);
            }
        }
        // Java: else
        {
            // Java: return false;
        }
        false
    }
}

impl TextRunBmpLruCache {
    // Java: TextRunBmpLruCache()
    pub fn new() -> Self {
        // Java: super(8388608);
        TextRunBmpLruCache { _private: () }
    }

    // Java: protected int sizeOf(BmpKey key, Bitmap value)
    pub fn sizeOf(&self, _key: i32, _value: i32) -> i32 {
        // Java: return value.getByteCount();
        0
    }
}

impl PicFormatConvertUtils {
    // Java: PicFormatConvertUtils()
    pub fn new() -> Self {
        // Java: super();
        PicFormatConvertUtils { _private: () }
    }

    // Java: public static boolean convert(String srcPicPath, String desPicPath, Context context)
    pub fn convert_context(_srcPicPath: &str, _desPicPath: &str, _context: i32) -> bool {
        // Java: int width = BaseDisplayUtil.getDisplayWidth(context);
        // Java: int height = BaseDisplayUtil.getDisplayHeight(context);
        // Java: return convert(srcPicPath, desPicPath, width, height, false);
        false
    }

    // Java: public static boolean convertByData(String srcPicPath, String desPicPath, Context context)
    pub fn convertByData_context(_srcPicPath: &str, _desPicPath: &str, _context: i32) -> bool {
        // Java: int width = BaseDisplayUtil.getDisplayWidth(context);
        // Java: int height = BaseDisplayUtil.getDisplayHeight(context);
        // Java: return convert(srcPicPath, desPicPath, width, height, true);
        false
    }

    // Java: public static boolean convertByData(String srcPicPath, String desPicPath, int width, int height)
    pub fn convertByData(_srcPicPath: &str, _desPicPath: &str, _width: i32, _height: i32) -> bool {
        // Java: return convert(srcPicPath, desPicPath, width, height, true);
        false
    }

    // Java: private static boolean convert(String srcPicPath, String desPicPath, int width, int height, boolean extensionByData)
    fn convert(_srcPicPath: &str, _desPicPath: &str, _width: i32, _height: i32, _extensionByData: bool) -> bool {
        // Java: File srcFile = new File(srcPicPath);
        // Java: if (!srcFile.exists())
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: boolean isSaved = false;
            // Java: String srcExt = getFileExtension(srcPicPath);
            // Java: if (extensionByData)
            {
                // Java: srcExt = getSuffix(srcPicPath);
            }
            // Java: if (srcExt.equals(""))
            {
                // Java: return false;
            }
            // Java: else
            {
                // Java: String desExt = getFileExtension(desPicPath);
                // Java: if (extensionByData)
                {
                    // Java: desExt = getSuffix(desPicPath);
                }
                // Java: if (desExt.equals(""))
                {
                    // Java: return false;
                }
                // Java: else
                {
                    // Java: boolean isNeedConvert = !srcExt.equalsIgnoreCase(desExt);
                    // Java: if (isNeedConvert)
                    {
                        // Java: Bitmap bitmap = null;
                        // Java: if (srcExt.equalsIgnoreCase("wmf"))
                        {
                            // Java: bitmap = decodeBitmap(srcPicPath, width, height);
                        }
                        // Java: else
                        {
                            // Java: bitmap = decodeBitmap(srcPicPath);
                        }
                        // Java: isSaved = savePicBitmap(bitmap, desPicPath, extensionByData);
                    }
                    // Java: else
                    {
                        // Java: isSaved = copyPicture(srcPicPath, desPicPath);
                    }
                    // Java: return isSaved;
                }
            }
        }
        false
    }

    // Java: public static Bitmap getOrCreateBitmap(String srcPicPath, int width, int height)
    pub fn getOrCreateBitmap(_srcPicPath: &str, _width: i32, _height: i32) -> i32 {
        // Java: BmpKey key = new BmpKey(srcPicPath, width, height);
        // Java: Bitmap ret = (Bitmap)TXT_BMP_CACHE.get(key);
        // Java: if (ret == null)
        {
            // Java: ret = createBitmap(srcPicPath, width, height);
            // Java: if (ret != null)
            {
                // Java: TXT_BMP_CACHE.put(key, ret);
            }
        }
        // Java: return ret;
        0
    }

    // Java: public static Bitmap createBitmap(String srcPicPath, int width, int height)
    pub fn createBitmap(_srcPicPath: &str, _width: i32, _height: i32) -> i32 {
        // Java: File srcFile = new File(srcPicPath);
        // Java: if (!srcFile.exists())
        {
            // Java: return null;
        }
        // Java: else
        {
            // Java: String srcExt = getFileExtension(srcPicPath);
            // Java: if (srcExt.equals(""))
            {
                // Java: return null;
            }
            // Java: else if (!srcExt.equalsIgnoreCase("wmf"))
            {
                // Java: return decodeBitmap(srcPicPath);
            }
            // Java: else
            {
                // Java: while(width > 100)
                {
                    // Java: width /= 10;
                }
                // Java: while(height > 100)
                {
                    // Java: height /= 10;
                }
                // Java: return decodeBitmap(srcPicPath, width, height);
            }
        }
        0
    }

    // Java: public static boolean copyPicture(String srcPicPath, String desPicPath)
    pub fn copyPicture(_srcPicPath: &str, _desPicPath: &str) -> bool {
        // Java: File srcFile = new File(srcPicPath);
        // Java: File destFile = new File(desPicPath);
        // Java: if (!srcFile.exists())
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: boolean result = false;
            // Java: File tmpFile = null;
            // Java: try
            {
                // Java: tmpFile = File.createTempFile("ppt_pic_save", (String)null, destFile.getParentFile());
                // Java: result = FileUtil.copyFile(srcFile.getAbsolutePath(), tmpFile.getAbsolutePath()) && tmpFile.renameTo(destFile);
                // Java: if (result)
                {
                    // Java: tmpFile = null;
                }
            }
            // Java: catch (IOException var10)
            {
            }
            // Java: finally
            {
                // Java: if (null != tmpFile)
                {
                    // Java: tmpFile.delete();
                }
            }
            // Java: return result;
        }
        false
    }

    // Java: private static Bitmap decodeBitmap(String path)
    fn decodeBitmap_path(_path: &str) -> i32 {
        // Java: PictureInfo info = PictureParser.syncGetPictureInfo(path);
        // Java: return info != null ? decodeBitmap(path, info.width, info.height) : null;
        0
    }

    // Java: public static Bitmap decodeBitmap(String path, int width, int height)
    pub fn decodeBitmap(_path: &str, _width: i32, _height: i32) -> i32 {
        // Java: try
        {
            // Java: return decodeRegionBitmap(path, (Rect)null, width, height);
        }
        // Java: catch (IOException e)
        {
            // Java: e.printStackTrace();
            // Java: return null;
        }
        0
    }

    // Java: private static Bitmap decodeRegionBitmap(String path, Rect rect, int width, int height)
    fn decodeRegionBitmap(_path: &str, _rect: i32, _width: i32, _height: i32) -> i32 {
        // Java: Bitmap localBitmap = null;
        // Java: try
        {
            // Java: localBitmap = decodeBitmapWithReducingQualityWhenLowMemory(path, rect, (float)width, (float)height);
        }
        // Java: catch (OutOfMemoryError var8)
        {
        }
        // Java: if (localBitmap == null)
        {
            // Java: try
            {
                // Java: localBitmap = decodeBitmapWithReducingQualityWhenLowMemory(path, rect, (float)width, (float)height);
            }
            // Java: catch (OutOfMemoryError var7)
            {
            }
        }
        // Java: if (localBitmap == null)
        {
            // Java: try
            {
                // Java: localBitmap = decodeBitmapWithReducingSizeWhenLowMemory(path, rect, (float)width, (float)height);
            }
            // Java: catch (OutOfMemoryError var6)
            {
            }
        }
        // Java: return localBitmap;
        0
    }

    // Java: private static Bitmap decodeBitmapWithReducingQualityWhenLowMemory(String path, Rect rect, float width, float height)
    fn decodeBitmapWithReducingQualityWhenLowMemory(_path: &str, _rect: i32, _width: f32, _height: f32) -> i32 {
        // Java: PictureInfo info = PictureParser.syncGetPictureInfo(path);
        // Java: if (info == null)
        {
            // Java: return null;
        }
        // Java: else
        {
            // Java: if (width <= 0.0F)
            {
                // Java: width = (float)info.getWidth();
            }
            // Java: if (height <= 0.0F)
            {
                // Java: height = (float)info.getHeight();
            }
            // Java: PptBitmapCreate creator = new PptBitmapCreate();
            // Java: Bitmap localBitmap;
            // Java: if (null != rect)
            {
                // Java: localBitmap = creator.createRegionBitmap(info, rect, (int)width, (int)height);
            }
            // Java: else
            {
                // Java: localBitmap = creator.createBitmap(info, (int)width, (int)height, false);
            }
            // Java: if (null == localBitmap)
            {
                // Java: creator.setConfig(Config.ARGB_4444);
                // Java: if (null != rect)
                {
                    // Java: localBitmap = creator.createRegionBitmap(info, rect, (int)width, (int)height);
                }
                // Java: else
                {
                    // Java: localBitmap = creator.createBitmap(info, (int)width, (int)height, false);
                }
            }
            // Java: return localBitmap;
        }
        0
    }

    // Java: private static Bitmap decodeBitmapWithReducingSizeWhenLowMemory(String path, Rect rect, float width, float height)
    fn decodeBitmapWithReducingSizeWhenLowMemory(_path: &str, _rect: i32, _width: f32, _height: f32) -> i32 {
        // Java: Bitmap localBitmap = null;
        // Java: int properW = (int)((double)width * 0.8);
        // Java: int properH = (int)((double)height * 0.8);
        // Java: for(int tryTimes = 3; tryTimes != 0 && properW != 0 && properH != 0 && localBitmap == null; --tryTimes)
        {
            // Java: localBitmap = decodeBitmapWithReducingQualityWhenLowMemory(path, rect, (float)properW, (float)properH);
            // Java: properW = (int)((double)properW * 0.8);
            // Java: properH = (int)((double)properH * 0.8);
        }
        // Java: return localBitmap;
        0
    }

    // Java: public static String getFileExtension(String path)
    pub fn getFileExtension(_path: &str) -> String {
        // Java: if (path != null && !path.equals(""))
        {
            // Java: String[] picExtension = path.split("\\.");
            // Java: return picExtension.length <= 0 ? "" : picExtension[picExtension.length - 1];
        }
        // Java: else
        {
            // Java: return "";
        }
        String::new()
    }

    // Java: private static int type(String ext)
    fn type_ext(_ext: &str) -> i32 {
        // Java: int type = -1;
        // Java: if (ext.equalsIgnoreCase("bmp"))
        {
            // Java: type = 3;
        }
        // Java: else if (ext.equalsIgnoreCase("png"))
        {
            // Java: type = 2;
        }
        // Java: else if (ext.equalsIgnoreCase("jpg"))
        {
            // Java: type = 1;
        }
        // Java: else if (ext.equalsIgnoreCase("wmf"))
        {
            // Java: type = 0;
        }
        // Java: return type;
        0
    }

    // Java: private static boolean savePicBitmap(Bitmap bitmap, String dstPath, boolean extensionByData)
    fn savePicBitmap(_bitmap: i32, _dstPath: &str, _extensionByData: bool) -> bool {
        // Java: String extension = getFileExtension(dstPath);
        // Java: if (extensionByData)
        {
            // Java: extension = getSuffix(dstPath);
        }
        // Java: int type = 0;
        // Java: if (extension.equals(""))
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: type = type(extension);
            // Java: if (type == -1)
            {
                // Java: return false;
            }
            // Java: else
            {
                // Java: try
                {
                    // Java: switch (type)
                    {
                        // Java: case 2:
                        // Java: bitmap.compress(CompressFormat.PNG, 80, new FileOutputStream(dstPath));
                        // Java: break;
                        // Java: case 3:
                        // Java: (new BMPWriter()).write(bitmap, dstPath, PictureFormat.BMP24);
                        // Java: break;
                        // Java: default:
                        // Java: bitmap = convertAlpha2White(bitmap);
                        // Java: bitmap.compress(CompressFormat.JPEG, 80, new FileOutputStream(dstPath));
                    }
                    // Java: return true;
                }
                // Java: catch (FileNotFoundException e)
                {
                    // Java: e.printStackTrace();
                    // Java: return false;
                }
            }
        }
        false
    }

    // Java: private static Bitmap convertAlpha2White(Bitmap bitmap)
    fn convertAlpha2White(_bitmap: i32) -> i32 {
        // Java: if (!bitmap.hasAlpha())
        {
            // Java: return bitmap;
        }
        // Java: else
        {
            // Java: Bitmap newBitmap = bitmap;
            // Java: if (!bitmap.isMutable())
            {
                // Java: Bitmap.Config config = bitmap.getConfig();
                // Java: config = config == null ? Config.ARGB_8888 : config;
                // Java: newBitmap = bitmap.copy(config, true);
            }
            // Java: int width = bitmap.getWidth();
            // Java: int height = bitmap.getHeight();
            // Java: for(int x = 0; x < width; ++x)
            {
                // Java: for(int y = 0; y < height; ++y)
                {
                    // Java: int color = bitmap.getPixel(x, y);
                    // Java: if (Color.alpha(color) == 0)
                    {
                        // Java: newBitmap.setPixel(x, y, -1);
                    }
                }
            }
            // Java: return newBitmap;
        }
        0
    }

    // Java: public static boolean hasAlphaBackgroud(Bitmap bitmap)
    pub fn hasAlphaBackgroud(_bitmap: i32) -> bool {
        // Java: if (!bitmap.hasAlpha())
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: int width = bitmap.getWidth();
            // Java: int height = bitmap.getHeight();
            // Java: for(int x = 0; x < width; ++x)
            {
                // Java: for(int y = 0; y < height; ++y)
                {
                    // Java: int color = bitmap.getPixel(x, y);
                    // Java: if (Color.alpha(color) == 0)
                    {
                        // Java: return true;
                    }
                }
            }
            // Java: return false;
        }
        false
    }

    // Java: public static String getSuffix(String filePath)
    pub fn getSuffix(_filePath: &str) -> String {
        // Java: PictureInfo info = PictureParser.getPictureInfo(filePath);
        // Java: if (null == info)
        {
            // Java: return "png";
        }
        // Java: else
        {
            // Java: int type = info.getType();
            // Java: switch (type)
            {
                // Java: case 1:
                // Java: return "png";
                // Java: case 2:
                // Java: return "jpeg";
                // Java: case 3:
                // Java: return "bmp";
                // Java: case 4:
                // Java: return "gif";
                // Java: case 5:
                // Java: return "emf";
                // Java: case 6:
                // Java: return "wmf";
                // Java: case 7:
                // Java: return "tiff";
                // Java: default:
                // Java: return "png";
            }
        }
        String::from("png")
    }
}
