#![allow(dead_code, unused_variables, unused_mut, unreachable_code, non_snake_case, unused_imports, non_camel_case_types)]

type Bitmap = i32;
type Canvas = i32;
type ColorMatrix = i32;
type ColorMatrixColorFilter = i32;
type Paint_stub = i32;
type Config_stub = i32;
type File_stub = i32;
type FileOutputStream = i32;
type FileUtil = i32;
type PictureFormat = i32;

// Java: import android.graphics.Bitmap

// Java: import android.graphics.Canvas

// Java: import android.graphics.ColorMatrix

// Java: import android.graphics.ColorMatrixColorFilter

// Java: import android.graphics.Paint

// Java: import android.graphics.Bitmap.Config

// Java: import cn.wps.moffice.kfs.File

// Java: import cn.wps.moffice.kfs.FileOutputStream

// Java: import cn.wps.moffice.util.FileUtil

// Java: import java.io.IOException

// Java: public class BMPWriter
pub struct BMPWriter {
    // Java field: int[] mLinePixels
    mLinePixels: Option<Vec<i32>>,
    // Java field: byte[] mLineBytes
    mLineBytes: Option<Vec<u8>>,
}

// Java field: int BYTESIZE
pub(crate) const BYTESIZE: i32 = 8;

// Java: public class BMPHEADER
pub struct BMPHEADER {
    // Java field: short bfType
    pub(crate) bfType: i16,
    // Java field: int bfSize
    pub(crate) bfSize: i32,
    // Java field: short bfReserved1
    pub(crate) bfReserved1: i16,
    // Java field: short bfReserved2
    pub(crate) bfReserved2: i16,
    // Java field: int bfOffBits
    pub(crate) bfOffBits: i32,
}

// Java: public class BMPINFOHEADER
pub struct BMPINFOHEADER {
    // Java field: int biSize
    pub(crate) biSize: i32,
    // Java field: int biWidth
    pub(crate) biWidth: i32,
    // Java field: int biHeight
    pub(crate) biHeight: i32,
    // Java field: short biPlanes
    pub(crate) biPlanes: i16,
    // Java field: short biBitCount
    pub(crate) biBitCount: i16,
    // Java field: int biCompression
    pub(crate) biCompression: i32,
    // Java field: int biSizeImage
    pub(crate) biSizeImage: i32,
    // Java field: int biXPelsPerMeter
    pub(crate) biXPelsPerMeter: i32,
    // Java field: int biYPelsPerMeter
    pub(crate) biYPelsPerMeter: i32,
    // Java field: int biClrUsed
    pub(crate) biClrUsed: i32,
    // Java field: int biClrImportant
    pub(crate) biClrImportant: i32,
}

// Java: public class RGBQUAD
pub struct RGBQUAD {
    // Java field: int color
    color: i32,
}

// Java: public class BMPWriter
impl BMPWriter {
    // Java: BMPWriter()
    pub fn new(&self) {
      // Java: super();

    }

    // Java: boolean write(Bitmap bitmap, String path, PictureFormat format)
    pub fn write(&self, _bitmap: i32, _path: String, _format: i32) -> bool {
      // Java: if (PictureTool.isGray(format))

      {
         // Java: bitmap = getGrayBitmap(bitmap);

      }
      // Java: int bitCount = PictureTool.getBitCount(format);

      // Java: int width = bitmap.getWidth();

      // Java: int lineWidth = width * bitCount / 8;

      // Java: if (width * bitCount % 8 > 0)

      {
         // Java: ++lineWidth;

      }
      // Java: int lineWidth4 = getRoundNum(lineWidth, 4);

      // Java: int extra = lineWidth4 - lineWidth;

      // Java: int height = bitmap.getHeight();

      // Java: int dataSize = lineWidth4 * height + this.getColorIndexSize(format);

      // Java: BMPHEADER bmpHeader = new BMPHEADER();

      // Java: bmpHeader.bfSize = 54 + dataSize;

      // Java: bmpHeader.bfOffBits = 54;

      // Java: BMPINFOHEADER infoHeader = new BMPINFOHEADER();

      // Java: infoHeader.biSize = 40;

      // Java: infoHeader.biWidth = width;

      // Java: infoHeader.biHeight = height;

      // Java: infoHeader.biBitCount = (short)bitCount;

      // Java: infoHeader.biSizeImage = dataSize;

      // Java: byte[] headerBytes = this.getHeaderBuffer(bmpHeader);

      // Java: byte[] InfoBytes = this.getInfoHeaderBuffer(infoHeader);

      // Java: try

      {
         // Java: File file = new File(path);

         // Java: FileOutputStream out = new FileOutputStream(file);

         // Java: out.write(headerBytes, 0, headerBytes.length);

         // Java: out.write(InfoBytes, 0, InfoBytes.length);

         // Java: int colorIndex = PictureTool.getColorIndexCount(format);

         // Java: if (colorIndex > 0)

         {
            // Java: byte[] byteColorIndexs = this.getColorIndex(colorIndex);

            // Java: out.write(byteColorIndexs);

         }
         // Java: byte[] noneBytes = new byte[1];

         // Java: noneBytes[0] = 0;

         // Java: for(int y = height - 1; y >= 0; --y)

         {
            // Java: int[] pixels = this.getLinePixels(bitmap, y);

            // Java: byte[] bytes = this.getPixelBuffer(pixels, format);

            // Java: out.write(bytes);

            // Java: for(int i = 0; i < extra; ++i)

            {
               // Java: out.write(noneBytes);

            }
         }
         // Java: FileUtil.closeQuietly(out);

      }
      // Java: catch (IOException e)

      {
         // Java: e.printStackTrace();

      }
      // Java: return true;

      false
    }

    // Java: static int getRoundNum(int n, int round)
    pub fn getRoundNum(&self, _n: i32, _round: i32) -> i32 {
      // Java: int y = n % round;

      // Java: return y > 0 ? n + (round - y) : n;

      0
    }

    // Java: int[] getLinePixels(Bitmap bitmap, int y)
    pub fn getLinePixels(&self, _bitmap: i32, _y: i32) -> Vec<i32> {
      // Java: int width = bitmap.getWidth();

      // Java: if (this.mLinePixels == null || this.mLinePixels.length != width)

      {
         // Java: this.mLinePixels = new int[width];

      }
      // Java: bitmap.getPixels(this.mLinePixels, 0, width, 0, y, width, 1);

      // Java: return this.mLinePixels;

      Vec::new()
    }

    // Java: byte[] getPixelBuffer(int[] pixels, PictureFormat format)
    pub fn getPixelBuffer(&self, _pixels: Vec<i32>, _format: i32) -> Vec<u8> {
      // Java: int width = pixels.length;

      // Java: if (format != PictureFormat.BMP24 && format != PictureFormat.BMP24GRAY)

      {
         // Java: if (format == PictureFormat.BMP8GRAY)

         {
            // Java: int length = width;

            // Java: if (this.mLineBytes == null || this.mLineBytes.length != width)

            {
               // Java: this.mLineBytes = new byte[width];

            }
            // Java: for(int x = 0; x < length; ++x)

            {
               // Java: colorToBytes8(pixels[x], this.mLineBytes, x);

            }
         }
         // Java: else if (format == PictureFormat.BMP1)

         {
            // Java: int length = width / 8;

            // Java: if (width % 8 > 0)

            {
               // Java: ++length;

            }
            // Java: if (this.mLineBytes == null || this.mLineBytes.length != length)

            {
               // Java: this.mLineBytes = new byte[length];

            }
            // Java: for(int x = 0; x < length; ++x)

            {
               // Java: colorToBytes1(pixels, this.mLineBytes, x);

            }
         }
      }
      // Java: else

      {
         // Java: int length = width * 3;

         // Java: if (this.mLineBytes == null || this.mLineBytes.length != length)

         {
            // Java: this.mLineBytes = new byte[length];

         }
         // Java: for(int x = 0; x < width; ++x)

         {
            // Java: if (pixels[x] == 0)

            {
               // Java: colorToBytes24(16777215, this.mLineBytes, x * 3);

            }
            // Java: else

            {
               // Java: colorToBytes24(pixels[x], this.mLineBytes, x * 3);

            }
         }
      }
      // Java: return this.mLineBytes;

      Vec::new()
    }

    // Java: byte[] getColorIndex(int count)
    pub fn getColorIndex(&self, _count: i32) -> Vec<u8> {
      // Java: byte[] colors = new byte[count * 4];

      // Java: if (count == 2)

      {
         // Java: colorToBytes32(0, colors, 0);

         // Java: colorToBytes32(16777215, colors, 4);

      }
      // Java: else if (count == 256)

      {
         // Java: for(int i = 0; i < count; ++i)

         {
            // Java: int clr = (i << 16) + (i << 8) + i;

            // Java: colorToBytes32(clr, colors, i * 4);

         }
      }
      // Java: return colors;

      Vec::new()
    }

    // Java: int getColorIndexSize(PictureFormat format)
    pub fn getColorIndexSize(&self, _format: i32) -> i32 {
      // Java: int size = 0;

      // Java: switch (format)

      {
         // Java: case BMP1:

            // Java: size = 8;

            // Java: break;

         // Java: case BMP8GRAY:

            // Java: size = 1024;

      }
      // Java: return size;

      0
    }

    // Java: byte[] getHeaderBuffer(BMPHEADER header)
    pub fn getHeaderBuffer(&self, _header: i32) -> Vec<u8> {
      // Java: byte[] bytes = new byte[14];

      // Java: int offset = 0;

      // Java: shortToBytes(header.bfType, bytes, offset);

      // Java: offset += 2;

      // Java: intToBytes(header.bfSize, bytes, offset);

      // Java: offset += 4;

      // Java: shortToBytes(header.bfReserved1, bytes, offset);

      // Java: offset += 2;

      // Java: shortToBytes(header.bfReserved2, bytes, offset);

      // Java: offset += 2;

      // Java: intToBytes(header.bfOffBits, bytes, offset);

      // Java: return bytes;

      Vec::new()
    }

    // Java: byte[] getInfoHeaderBuffer(BMPINFOHEADER header)
    pub fn getInfoHeaderBuffer(&self, _header: i32) -> Vec<u8> {
      // Java: byte[] bytes = new byte[40];

      // Java: int offset = 0;

      // Java: intToBytes(header.biSize, bytes, offset);

      // Java: offset += 4;

      // Java: intToBytes(header.biWidth, bytes, offset);

      // Java: offset += 4;

      // Java: intToBytes(header.biHeight, bytes, offset);

      // Java: offset += 4;

      // Java: shortToBytes(header.biPlanes, bytes, offset);

      // Java: offset += 2;

      // Java: shortToBytes(header.biBitCount, bytes, offset);

      // Java: offset += 2;

      // Java: intToBytes(header.biCompression, bytes, offset);

      // Java: offset += 4;

      // Java: intToBytes(header.biSizeImage, bytes, offset);

      // Java: offset += 4;

      // Java: intToBytes(header.biXPelsPerMeter, bytes, offset);

      // Java: offset += 4;

      // Java: intToBytes(header.biYPelsPerMeter, bytes, offset);

      // Java: offset += 4;

      // Java: intToBytes(header.biClrUsed, bytes, offset);

      // Java: offset += 4;

      // Java: intToBytes(header.biClrImportant, bytes, offset);

      // Java: return bytes;

      Vec::new()
    }

    // Java: static void intToBytes(int n, byte[] bytes, int offset)
    pub fn intToBytes(&self, _n: i32, _bytes: Vec<u8>, _offset: i32) {
      // Java: bytes[offset] = (byte)(n >> 0 & 255);

      // Java: bytes[offset + 1] = (byte)(n >> 8 & 255);

      // Java: bytes[offset + 2] = (byte)(n >> 16 & 255);

      // Java: bytes[offset + 3] = (byte)(n >> 24 & 255);

    }

    // Java: static void colorToBytes32(int clr, byte[] bytes, int offset)
    pub fn colorToBytes32(&self, _clr: i32, _bytes: Vec<u8>, _offset: i32) {
      // Java: bytes[offset] = (byte)(clr >> 0 & 255);

      // Java: bytes[offset + 1] = (byte)(clr >> 8 & 255);

      // Java: bytes[offset + 2] = (byte)(clr >> 16 & 255);

      // Java: bytes[offset + 3] = (byte)(clr >> 24 & 255);

    }

    // Java: static void colorToBytes24(int clr, byte[] bytes, int offset)
    pub fn colorToBytes24(&self, _clr: i32, _bytes: Vec<u8>, _offset: i32) {
      // Java: bytes[offset] = (byte)(clr >> 0 & 255);

      // Java: bytes[offset + 1] = (byte)(clr >> 8 & 255);

      // Java: bytes[offset + 2] = (byte)(clr >> 16 & 255);

    }

    // Java: static void colorToBytes8(int clr, byte[] bytes, int offset)
    pub fn colorToBytes8(&self, _clr: i32, _bytes: Vec<u8>, _offset: i32) {
      // Java: int n = (clr >> 0 & 255) + (clr >> 8 & 255) + (clr >> 16 & 255);

      // Java: bytes[offset] = (byte)(n / 3);

    }

    // Java: static void colorToBytes1(int[] clrs, byte[] bytes, int offset)
    pub fn colorToBytes1(&self, _clrs: Vec<i32>, _bytes: Vec<u8>, _offset: i32) {
      // Java: byte b = 0;

      // Java: int last = clrs.length - 8 * offset;

      // Java: int size = last < 8 ? last : 8;

      // Java: for(int i = 0; i < size; ++i)

      {
         // Java: int clr = clrs[8 * offset + i];

         // Java: int n = (clr >> 0 & 255) + (clr >> 8 & 255) + (clr >> 16 & 255);

         // Java: byte value = (byte)(n > 700 ? 1 : 0);

         // Java: b = (byte)(b | value << 7 - i & 255);

      }
      // Java: bytes[offset] = b;

    }

    // Java: static void shortToBytes(short n, byte[] bytes, int offset)
    pub fn shortToBytes(&self, _n: i16, _bytes: Vec<u8>, _offset: i32) {
      // Java: bytes[offset] = (byte)(n >> 0 & 255);

      // Java: bytes[offset + 1] = (byte)(n >> 8 & 255);

    }

    // Java: static Bitmap getGrayBitmap(Bitmap bitmap)
    pub fn getGrayBitmap(&self, _bitmap: i32) -> i32 {
      // Java: int height = bitmap.getHeight();

      // Java: int width = bitmap.getWidth();

      // Java: Bitmap bmpGrayscale = Bitmap.createBitmap(width, height, Config.RGB_565);

      // Java: Canvas c = new Canvas(bmpGrayscale);

      // Java: Paint paint = new Paint();

      // Java: ColorMatrix cm = new ColorMatrix();

      // Java: cm.setSaturation(0.0F);

      // Java: ColorMatrixColorFilter f = new ColorMatrixColorFilter(cm);

      // Java: paint.setColorFilter(f);

      // Java: c.drawBitmap(bitmap, 0.0F, 0.0F, paint);

      // Java: return bmpGrayscale;

      0
    }

}

// Java: public class BMPHEADER
impl BMPHEADER {
    // Java: BMPHEADER()
    pub fn new_2(&self) {
         // Java: super();

    }

}

// Java: public class BMPINFOHEADER
impl BMPINFOHEADER {
    // Java: BMPINFOHEADER()
    pub fn new_3(&self) {
         // Java: super();

    }

}

// Java: public class RGBQUAD
impl RGBQUAD {
    // Java: RGBQUAD()
    pub fn RGBQUAD(&self) {
         // Java: super();

    }

}
