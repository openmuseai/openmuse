#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class PictureParser
pub struct PictureParser {
    // Java field: String TAG
    _private: (),
}

pub(crate) const TAG: &str = "PictureParser";

impl PictureParser {
    // Java: PictureParser()
    pub fn new() -> Self {
        // Java: super();
        PictureParser { _private: () }
    }

    // Java: public static PictureInfo getPictureInfo(String path)
    pub fn getPictureInfo_path(_path: &str) -> i32 {
        // Java: InputStream stream = null;
        // Java: PictureInfo info = null;
        // Java: try
        {
            // Java: stream = new FileInputStream(path);
            // Java: info = getPictureInfo(stream, path);
            // Java: stream.close();
        }
        // Java: catch (Exception var4)
        {
        }
        // Java: return info;
        0
    }

    // Java: public static PictureInfo getPictureInfo(InputStream stream)
    pub fn getPictureInfo_stream(_stream: i32) -> i32 {
        // Java: return getPictureInfo(stream, (String)null);
        0
    }

    // Java: public static PictureInfo getPictureInfo(InputStream stream, String path)
    pub fn getPictureInfo_stream_path(_stream: i32, _path: i32) -> i32 {
        // Java: PictureInfo info = null;
        // Java: BufferOptions options = null;
        // Java: BufferedInputStream bin = null;
        // Java: BufferReader reader = null;
        // Java: try
        {
            // Java: int length = stream.available();
            // Java: bin = new BufferedInputStream(stream);
            // Java: int formatType = getFormatType(bin);
            // Java: boolean checkPngChunkSize = VERSION.SDK_INT == 26 || VERSION.SDK_INT == 27;
            // Java: if (formatType == 1 && checkPngChunkSize && PNGParser.isUserTrunkDataTooLarge(bin))
            {
                // Java: info = new PictureInfo(path, formatType, -1, -1, length);
                // Java: return info;
            }
            // Java: if (formatType == 3 || formatType == 1 || formatType == 2 || formatType == 4 || formatType == 8 || formatType == 9)
            {
                // Java: options = new BufferOptions(true);
                // Java: BitmapFactory.decodeStream(bin, (Rect)null, options);
                // Java: info = new PictureInfo(path, formatType, options.outWidth, options.outHeight, length);
                // Java: return info;
            }
            // Java: if (formatType != 6)
            {
                // Java: if (formatType == 5)
                {
                    // Java: EMFHeader header = EMFParser.getHeader(bin);
                    // Java: if (header != null)
                    {
                        // Java: info = new EMFPictureInfo(path, formatType, header, length);
                        // Java: return info;
                    }
                    // Java: return null;
                }
                // Java: if (formatType == 7)
                {
                    // Java: reader = new BufferReader(bin);
                    // Java: TIFFDirectory dir = new TIFFDirectory(reader, 0, true);
                    // Java: int originWidth = (int)dir.getFieldAsLong(256);
                    // Java: int originHeight = (int)dir.getFieldAsLong(257);
                    // Java: info = new PictureInfo(path, formatType, originWidth, originHeight, length);
                    // Java: int dpix = 96;
                    // Java: int dpiy = 96;
                    // Java: TIFFField field = dir.getField(282);
                    // Java: if (field != null)
                    {
                        // Java: int[] rat = field.getAsSRational(0);
                        // Java: if (rat != null && rat.length > 1 && rat[1] != 0)
                        {
                            // Java: dpix = rat[0] / rat[1];
                        }
                    }
                    // Java: field = dir.getField(283);
                    // Java: if (field != null)
                    {
                        // Java: int[] rat = field.getAsSRational(0);
                        // Java: if (rat != null && rat.length > 1 && rat[1] != 0)
                        {
                            // Java: dpiy = rat[0] / rat[1];
                        }
                    }
                    // Java: info.dpi = new PointF(dpix > 0 ? (float)dpix : 96.0F, dpiy > 0 ? (float)dpiy : 96.0F);
                    // Java: return info;
                }
                // Java: Object var26 = null;
                // Java: return (PictureInfo)var26;
            }
            // Java: Size size = WMFParser.getSize(bin);
            // Java: if (size != null)
            {
                // Java: info = new WMFPictureInfo(path, formatType, (int)Math.ceil((double)size.getWidth()), (int)Math.ceil((double)size.getHeight()), length);
                // Java: return info;
            }
        }
        // Java: catch (OutOfMemoryError var20)
        {
            // Java: return info;
        }
        // Java: catch (Exception var21)
        {
            // Java: return info;
        }
        // Java: finally
        {
            // Java: if (null != bin)
            {
                // Java: bin.lightClose();
                // Java: BufferedInputStream var24 = null;
            }
            // Java: if (null != options)
            {
                // Java: options.dispose();
                // Java: BufferOptions var23 = null;
            }
            // Java: if (null != reader)
            {
                // Java: reader.close();
                // Java: BufferReader var25 = null;
            }
        }
        // Java: return null;
        0
    }

    // Java: private static Size getMetaSize(InputStream is, int formatType)
    fn getMetaSize(_is: i32, _formatType: i32) -> i32 {
        // Java: return formatType == 6 ? WMFParser.getSize(is) : EMFParser.getSize(is);
        0
    }

    // Java: public static int getFormatType(InputStream is)
    pub fn getFormatType(_is: i32) -> i32 {
        // Java: try
        {
            // Java: if (PicTypeParser.isBMP(is))
            {
                // Java: return 3;
            }
            // Java: if (PicTypeParser.isPNG(is))
            {
                // Java: return 1;
            }
            // Java: if (PicTypeParser.isJPG(is))
            {
                // Java: return 2;
            }
            // Java: if (PicTypeParser.isGIF(is))
            {
                // Java: return 4;
            }
            // Java: if (PicTypeParser.isEMF(is))
            {
                // Java: return 5;
            }
            // Java: if (PicTypeParser.isWMF(is))
            {
                // Java: return 6;
            }
            // Java: if (PicTypeParser.isTIFF(is))
            {
                // Java: return 7;
            }
            // Java: if (PicTypeParser.isWebp(is))
            {
                // Java: return 8;
            }
            // Java: if (PicTypeParser.isHeic(is))
            {
                // Java: return 9;
            }
        }
        // Java: catch (Exception var2)
        {
        }
        // Java: return 0;
        0
    }

    // Java: public static synchronized PictureInfo syncGetPictureInfo(String path)
    pub fn syncGetPictureInfo(_path: &str) -> i32 {
        // Java: return getPictureInfo(path);
        0
    }

    // Java: public static final PointF getDPI(PictureInfo info)
    pub fn getDPI(_info: i32) -> i32 {
        // Java: if (info.dpi != null)
        {
            // Java: return info.dpi;
        }
        // Java: else
        {
            // Java: info.dpi = new PointF(96.0F, 96.0F);
            // Java: int type = info.type;
            // Java: if (type == 2 || type == 1 || type == 5)
            {
                // Java: try
                {
                    // Java: InputStream stream = new FileInputStream(info.path);
                    // Java: if (type == 1)
                    {
                        // Java: byte[] buf = new byte[32];
                        // Java: int size = stream.read(buf, 0, 8);
                        // Java: if (size == 8 && buf[0] == -119 && buf[1] == 80 && buf[2] == 78 && buf[3] == 71 && buf[4] == 13 && buf[5] == 10 && buf[6] == 26 && buf[7] == 10)
                        {
                            // Java: while(stream.available() > 20)
                            {
                                // Java: size = stream.read(buf, 0, 8);
                                // Java: if (size < 8)
                                {
                                    // Java: break;
                                }
                                // Java: if (buf[4] == 112 && buf[5] == 72 && buf[6] == 89 && buf[7] == 115)
                                {
                                    // Java: stream.read(buf, 8, 10);
                                    // Java: if (buf[16] == 1)
                                    {
                                        // Java: info.dpi.x = (float)Math.round((float)((buf[8] & 255) << 24 | (buf[9] & 255) << 16 | (buf[10] & 255) << 8 | buf[11] & 255) * 0.0254F);
                                        // Java: info.dpi.y = (float)Math.round((float)((buf[12] & 255) << 24 | (buf[13] & 255) << 16 | (buf[14] & 255) << 8 | buf[15] & 255) * 0.0254F);
                                    }
                                    // Java: break;
                                }
                                // Java: int len = (buf[0] & 255) << 24 | (buf[1] & 255) << 16 | (buf[2] & 255) << 8 | buf[3] & 255;
                                // Java: stream.skip((long)(len + 4));
                            }
                        }
                    }
                    // Java: else if (type == 5)
                    {
                        // Java: EMFHeader emfHeader = ((EMFPictureInfo)info).mHeader;
                        // Java: info.dpi.x = (float)emfHeader.getDpiX();
                        // Java: info.dpi.y = (float)emfHeader.getDpiY();
                    }
                    // Java: else
                    {
                        // Java: int size = 32;
                        // Java: byte[] buf = new byte[size];
                        // Java: stream.read(buf);
                        // Java: if (buf[0] == -1 && buf[1] == -40)
                        {
                            // Java: if (buf[2] == -1 && buf[3] == -32)
                            {
                                // Java: if (buf[13] == 1)
                                {
                                    // Java: info.dpi.x = (float)((buf[14] & 255) << 8 | buf[15] & 255);
                                    // Java: info.dpi.y = (float)((buf[16] & 255) << 8 | buf[17] & 255);
                                }
                                // Java: else if (buf[13] == 2)
                                {
                                    // Java: info.dpi.x = (float)Math.round((float)((buf[14] & 255) << 8 | buf[15] & 255) * 2.54F);
                                    // Java: info.dpi.y = (float)Math.round((float)((buf[16] & 255) << 8 | buf[17] & 255) * 2.54F);
                                }
                            }
                            // Java: else if (buf[2] == -1 && buf[3] == -31)
                            {
                                // Java: size = (buf[4] & 255) << 8 | buf[5] & 255;
                                // Java: byte[] bufExif = new byte[size];
                                // Java: System.arraycopy(buf, 12, bufExif, 0, 20);
                                // Java: buf = bufExif;
                                // Java: stream.read(bufExif, 20, size - 20);
                                // Java: boolean bLittleEndian = bufExif[0] == 73;
                                // Java: int n = getNum(bufExif, 8, 2, bLittleEndian);
                                // Java: int unit = -1;
                                // Java: boolean bFindX = false;
                                // Java: boolean bFindY = false;
                                // Java: for(int i = 0; i < n; ++i)
                                {
                                    // Java: int pos = 10 + i * 12;
                                    // Java: int sign = getNum(buf, pos, 2, bLittleEndian);
                                    // Java: if (sign == 282)
                                    {
                                        // Java: info.dpi.x = (float)getValue(buf, pos, bLittleEndian);
                                        // Java: bFindX = true;
                                    }
                                    // Java: else if (sign == 283)
                                    {
                                        // Java: info.dpi.y = (float)getValue(buf, pos, bLittleEndian);
                                        // Java: bFindY = true;
                                    }
                                    // Java: else if (sign == 296)
                                    {
                                        // Java: unit = getValue(buf, pos, bLittleEndian);
                                    }
                                    // Java: if (bFindX && bFindY && unit >= 0)
                                    {
                                        // Java: break;
                                    }
                                }
                                // Java: if (unit == 3)
                                {
                                    // Java: if (bFindX)
                                    {
                                        // Java: PointF var10000 = info.dpi;
                                        // Java: var10000.x = (float)((double)var10000.x * 2.54);
                                    }
                                    // Java: if (bFindY)
                                    {
                                        // Java: PointF var23 = info.dpi;
                                        // Java: var23.y = (float)((double)var23.y * 2.54);
                                    }
                                }
                            }
                        }
                    }
                    // Java: stream.close();
                }
                // Java: catch (FileNotFoundException e)
                {
                    // Java: e.printStackTrace();
                }
                // Java: catch (IOException e)
                {
                    // Java: e.printStackTrace();
                }
                // Java: if (info.dpi.x <= 0.0F)
                {
                    // Java: info.dpi.x = 96.0F;
                }
                // Java: if (info.dpi.y <= 0.0F)
                {
                    // Java: info.dpi.y = 96.0F;
                }
            }
            // Java: return info.dpi;
        }
        0
    }

    // Java: protected static final int getValue(byte[] buf, int pos, boolean bLittleEndian)
    pub fn getValue(_buf: i32, _pos: i32, _bLittleEndian: bool) -> i32 {
        // Java: int val = 0;
        // Java: int format = getNum(buf, pos + 2, 2, bLittleEndian);
        // Java: if (format == 3)
        {
            // Java: val = getNum(buf, pos + 8, 2, bLittleEndian);
        }
        // Java: else if (format == 4)
        {
            // Java: val = getNum(buf, pos + 8, 4, bLittleEndian);
        }
        // Java: else if (format == 5)
        {
            // Java: int offset = getNum(buf, pos + 8, 4, bLittleEndian);
            // Java: float molecule = (float)getNum(buf, offset, 4, bLittleEndian);
            // Java: float denominator = (float)getNum(buf, offset + 4, 4, bLittleEndian);
            // Java: if (denominator != 0.0F)
            {
                // Java: val = Math.round(molecule / denominator);
            }
        }
        // Java: return val;
        0
    }

    // Java: protected static final int getNum(byte[] buf, int start, int len, boolean bLittleEndian)
    pub fn getNum(_buf: i32, _start: i32, _len: i32, _bLittleEndian: bool) -> i32 {
        // Java: if (start + len >= buf.length)
        {
            // Java: return 0;
        }
        // Java: else
        {
            // Java: int n = 0;
            // Java: if (bLittleEndian)
            {
                // Java: for(int i = len - 1; i >= 0; --i)
                {
                    // Java: n = n << 8 | buf[start + i] & 255;
                }
            }
            // Java: else
            {
                // Java: for(int i = 0; i < len; ++i)
                {
                    // Java: n = n << 8 | buf[start + i] & 255;
                }
            }
            // Java: return n;
        }
        0
    }

    // Java: public static void rotateBitmapPathIfNeeded(String path)
    pub fn rotateBitmapPathIfNeeded(_path: &str) {
        // Java: int degree = getBitmapDegree(path);
        // Java: if (degree > 0)
        {
            // Java: BitmapFactory.Options opts = new BitmapFactory.Options();
            // Java: opts.inSampleSize = 2;
            // Java: Bitmap bmp = null;
            // Java: FileInputStream fis = null;
            // Java: try
            {
                // Java: fis = new FileInputStream(path);
                // Java: bmp = BitmapFactory.decodeStream(fis, (Rect)null, opts);
                // Java: if (bmp != null)
                {
                    // Java: Bitmap newbitmap = rotateBitmapByDegree(bmp, degree);
                    // Java: saveBitmapAsPic(newbitmap, path, false, 100);
                    // Java: newbitmap.recycle();
                }
            }
            // Java: catch (Throwable var14)
            {
            }
            // Java: finally
            {
                // Java: if (fis != null)
                {
                    // Java: try
                    {
                        // Java: fis.close();
                    }
                    // Java: catch (IOException e)
                    {
                        // Java: e.printStackTrace();
                    }
                }
            }
        }
    }

    // Java: public static boolean saveBitmapAsPic(Bitmap bitmap, String bitName, boolean jpeg, int quality)
    pub fn saveBitmapAsPic(_bitmap: i32, _bitName: &str, _jpeg: bool, _quality: i32) -> bool {
        // Java: FileOutputStream fOut = null;
        // Java: try
        {
            // Java: fOut = new FileOutputStream(bitName);
        }
        // Java: catch (FileNotFoundException e)
        {
            // Java: KSLog.e(TAG, "FileNotFoundException", e);
        }
        // Java: if (null == fOut)
        {
            // Java: return false;
        }
        // Java: else if (bitmap == null)
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: boolean res = bitmap.compress(jpeg ? CompressFormat.JPEG : CompressFormat.PNG, quality, fOut);
            // Java: FileUtil.closeQuietly(fOut);
            // Java: return res;
        }
        false
    }

    // Java: private static Bitmap rotateBitmapByDegree(Bitmap bm, int degree)
    fn rotateBitmapByDegree(_bm: i32, _degree: i32) -> i32 {
        // Java: Bitmap returnBm = null;
        // Java: Matrix matrix = new Matrix();
        // Java: matrix.postRotate((float)degree);
        // Java: try
        {
            // Java: returnBm = Bitmap.createBitmap(bm, 0, 0, bm.getWidth(), bm.getHeight(), matrix, true);
        }
        // Java: catch (OutOfMemoryError var5)
        {
        }
        // Java: if (returnBm == null)
        {
            // Java: returnBm = bm;
        }
        // Java: if (bm != returnBm)
        {
            // Java: bm.recycle();
        }
        // Java: return returnBm;
        0
    }

    // Java: private static int getBitmapDegree(String path)
    fn getBitmapDegree(_path: &str) -> i32 {
        // Java: int degree = 0;
        // Java: try
        {
            // Java: ExifInterface exifInterface = new ExifInterface(path);
            // Java: int orientation = exifInterface.getAttributeInt("Orientation", 1);
            // Java: switch (orientation)
            {
                // Java: case 3:
                // Java: degree = 180;
                // Java: break;
                // Java: case 6:
                // Java: degree = 90;
                // Java: break;
                // Java: case 8:
                // Java: degree = 270;
            }
        }
        // Java: catch (Exception var4)
        {
        }
        // Java: return degree;
        0
    }
}
