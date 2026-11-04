#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables, unused_mut)]

use std::sync::{Arc, Mutex, OnceLock};
use std::any::Any;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java field: String TAG
pub(crate) const TAG: &str = "BitmapUtils";
// Java field: Bitmap.Config BYTE_BITMAP_CONFIG
pub(crate) const BYTE_BITMAP_CONFIG: i32 = 4; // Config.ARGB_4444
// Java field: int IO_BUFFER_SIZE
pub(crate) const IO_BUFFER_SIZE: i32 = 4096;
// Java field: int count
static count: OnceLock<Mutex<i32>> = OnceLock::new();
fn get_count() -> &'static Mutex<i32> { count.get_or_init(|| Mutex::new(0)) }
// Java field: float IN_PT
pub(crate) const IN_PT: f32 = 71.9989_f32;

// Java: public class BitmapUtils
pub struct BitmapUtils {
}

// Java: public class BitmapUtils
impl BitmapUtils {
    // Java: BitmapUtils()
    pub fn new() -> Self {
      // Java: super();
      BitmapUtils {}
    }

    // Java: static Bitmap loadBetterBitmap(String fileName)
    pub fn loadBetterBitmap(fileName: &str) -> Element {
      // Java: try
      {
         // Java: File openDocumentFile = new File(fileName);
         // TODO: BitmapFactory not available
         // Java: BitmapFactory.Options o = new BitmapFactory.Options();
         // Java: o.inJustDecodeBounds = true;
         // Java: BitmapFactory.decodeStream(new FileInputStream(openDocumentFile), (Rect)null, o);
         // Java: int REQUIRED_SIZE = 500;
         let REQUIRED_SIZE = 500_i32;
         // Java: int scale;
         let mut scale: i32;
         // Java: for(scale = 1; o.outWidth / scale / 2 >= 500 || o.outHeight / scale / 2 >= 500; scale *= 2)
         scale = 1;
         {
         }
         // Java: BitmapFactory.Options o2 = new BitmapFactory.Options();
         // Java: o2.inSampleSize = scale;
         // Java: return BitmapFactory.decodeStream(new FileInputStream(openDocumentFile), (Rect)null, o2);
         // TODO: BitmapFactory not available
         return None;
      }
      // Java: catch (FileNotFoundException e)
      {
         // Java: KSLog.e(TAG, "FileNotFoundException", e);
         // Java: return null;
      }
    }

    // Java: static Bitmap loadPreviewBitmap(String fileName, int width, int height)
    pub fn loadPreviewBitmap(fileName: &str, width: i32, height: i32) -> Element {
      // Java: BitmapFactory.Options options = new BitmapFactory.Options();
      // Java: options.inJustDecodeBounds = false;
      // Java: options.inSampleSize = 1;
      // Java: Bitmap bm = null;
      let mut bm: Element = None;
      // Java: try
      {
         // Java: bm = BitmapFactory.decodeFile(fileName, options);
         // TODO: BitmapFactory not available
         // Java: if (bm != null)
         if bm.is_some()
         {
            // Java: Bitmap resizebm = createScaledBitmap(bm, width <= 0 ? 1 : width, height <= 0 ? 1 : height, true, true);
            // TODO: createScaledBitmap requires Bitmap
            // Java: bm.recycle();
            // Java: return resizebm;
         }
      }
      // Java: catch (OutOfMemoryError e)
      {
         // Java: KSLog.e(TAG, "OutOfMemoryError", e);
         // Java: if (null != bm)
         if bm.is_some()
         {
            // Java: bm.recycle();
            // Java: Bitmap var7 = null;
            bm = None;
         }
         // Java: System.gc();
         // Java: Platform.setKillProcess(true);
         // TODO: Platform not available
      }
      // Java: return null;
      None
    }

    // Java: static Bitmap createScaledBitmap(Bitmap bm, int dstWidth, int dstHeight, boolean filter, boolean isCutRoud)
    pub fn createScaledBitmap_with_filter(bm: &Element, dstWidth: i32, dstHeight: i32, filter: bool, isCutRoud: bool) -> Element {
      // Java: Matrix m = new Matrix();
      // TODO: Matrix not available
      // Java: float POINT_TO_PIXEL = (float)Platform.getDisplayMetrics().densityDpi / 71.9989F;
      let POINT_TO_PIXEL: f32 = 1.0_f32; // TODO: Platform not available
      // Java: int width = bm.getWidth();
      let width: i32 = 0; // TODO: Bitmap not available
      // Java: int height = bm.getHeight();
      let height: i32 = 0; // TODO: Bitmap not available
      // Java: float sx = (float)dstWidth / (float)width;
      let sx: f32 = if width > 0 { dstWidth as f32 / width as f32 } else { 1.0_f32 };
      // Java: float sy = (float)dstHeight / (float)height;
      let sy: f32 = if height > 0 { dstHeight as f32 / height as f32 } else { 1.0_f32 };
      // Java: m.setScale(sx, sy);
      // Java: Bitmap b = null;
      let mut b: Element = None;
      // Java: int margin = (int)((double)0.5F * (double)POINT_TO_PIXEL * (double)sx);
      let mut margin: i32 = (0.5_f64 * POINT_TO_PIXEL as f64 * sx as f64) as i32;
      // Java: margin = Math.max(1, margin);
      margin = margin.max(1);
      // Java: if (isCutRoud)
      if isCutRoud
      {
         // Java: b = Bitmap.createBitmap(bm, margin, margin, width - margin, height - margin, m, filter);
         // TODO: Bitmap.createBitmap not available
      }
      // Java: else
      else
      {
         // Java: b = Bitmap.createBitmap(bm, 0, 0, width, height, m, filter);
         // TODO: Bitmap.createBitmap not available
      }
      // Java: return b;
      b
    }

    // Java: static Bitmap loadBitmap(InputStream is)
    pub fn loadBitmap_from_stream(is: &Element) -> Element {
      // Java: if (is != null)
      if is.is_some()
      {
         // Java: try
         {
            // Java: return BitmapFactory.decodeStream(is);
            // TODO: BitmapFactory not available
         }
         // Java: catch (OutOfMemoryError e)
         {
            // Java: KSLog.e(TAG, "OutOfMemoryError", e);
            // Java: System.gc();
            // Java: Platform.setKillProcess(true);
            // TODO: Platform not available
         }
      }
      // Java: return null;
      None
    }

    // Java: static Bitmap loadBitmap(Context c, String fileName)
    pub fn loadBitmap_from_context(c: &Element, fileName: &str) -> Element {
      // Java: ParcelFileDescriptor pfd;
      // Java: try
      {
         // Java: pfd = c.getContentResolver().openFileDescriptor(Uri.parse("file://" + fileName), "r");
         // TODO: Context/ParcelFileDescriptor not available
      }
      // Java: catch (IOException var6)
      {
         // Java: return null;
         return None;
      }
      // Java: FileDescriptor fd = pfd.getFileDescriptor();
      // Java: BitmapFactory.Options options = new BitmapFactory.Options();
      // Java: options.inTempStorage = new byte[10240];
      // Java: options.inSampleSize = 1;
      // Java: options.inJustDecodeBounds = true;
      // Java: BitmapFactory.decodeFileDescriptor(fd, (Rect)null, options);
      // Java: options.inSampleSize = computeSampleSize(options, 400);
      // Java: options.inJustDecodeBounds = false;
      // Java: options.inDither = false;
      // Java: options.inPreferredConfig = Config.ARGB_8888;
      // Java: Bitmap sourceBitmap = BitmapFactory.decodeFileDescriptor(fd, (Rect)null, options);
      // TODO: BitmapFactory not available
      // Java: if (count < 4)
      let mut cnt = get_count().lock().unwrap();
      if *cnt < 4
      {
         // Java: ++count;
         *cnt += 1;
      }
      // Java: else
      else
      {
         // Java: count = 0;
         *cnt = 0;
         // Java: System.gc();
      }
      // Java: return sourceBitmap;
      None
    }

    // Java: static int computeSampleSize(BitmapFactory.Options options, int target)
    pub fn computeSampleSize(out_width: i32, out_height: i32, target: i32) -> i32 {
      // Java: int w = options.outWidth;
      let w = out_width;
      // Java: int h = options.outHeight;
      let h = out_height;
      // Java: int candidateW = w / target;
      let candidateW = w / target;
      // Java: int candidateH = h / target;
      let candidateH = h / target;
      // Java: int candidate = Math.max(candidateW, candidateH);
      let candidate = candidateW.max(candidateH);
      // Java: if (candidate == 0)
      if candidate == 0
      {
         // Java: return 1;
         return 1;
      }
      // Java: else
      else
      {
         // Java: KSLog.d(TAG, "for w/h " + w + "/" + h + " returning " + candidate + "(" + w / candidate + " / " + h / candidate);
         // Java: return candidate;
         return candidate;
      }
    }

    // Java: static Bitmap loadBitmapAll(String fileName)
    pub fn loadBitmapAll(fileName: &str) -> Element {
      // Java: BitmapFactory.Options bfOptions = new BitmapFactory.Options();
      // Java: bfOptions.inDither = false;
      // Java: bfOptions.inPurgeable = true;
      // Java: bfOptions.inInputShareable = true;
      // Java: bfOptions.inTempStorage = new byte['耀'];
      // Java: Bitmap bitmap = null;
      let mut bitmap: Element = None;
      // Java: try
      {
         // Java: ParcelFileDescriptor fileDes = ParcelFileDescriptor.open(new File(fileName), 268435456);
         // Java: bitmap = BitmapFactory.decodeFileDescriptor(fileDes.getFileDescriptor(), (Rect)null, bfOptions);
         // TODO: BitmapFactory/ParcelFileDescriptor not available
      }
      // Java: catch (FileNotFoundException e)
      {
         // Java: KSLog.e(TAG, "FileNotFoundException", e);
      }
      // Java: catch (OutOfMemoryError e)
      {
         // Java: KSLog.e(TAG, "OutOfMemoryError", e);
         // Java: if (null != bitmap)
         if bitmap.is_some()
         {
            // Java: bitmap.recycle();
            // Java: bitmap = null;
            bitmap = None;
         }
         // Java: System.gc();
         // Java: Platform.setKillProcess(true);
         // TODO: Platform not available
      }
      // Java: return bitmap;
      bitmap
    }

    // Java: static int[] getImageWH(String path)
    pub fn getImageWH(path: Option<&str>) -> Vec<i32> {
      // Java: int[] wh = new int[]{-1, -1};
      let mut wh = vec![-1_i32, -1_i32];
      // Java: if (path == null)
      if path.is_none()
      {
         // Java: return wh;
         return wh;
      }
      // Java: else
      else
      {
         // Java: File file = new File(path);
         // Java: InputStream in = null;
         // Java: if (file.exists() && !file.isDirectory())
         // TODO: File operations stubbed
         {
            // Java: try
            {
               // Java: BitmapFactory.Options options = new BitmapFactory.Options();
               // Java: options.inJustDecodeBounds = true;
               // Java: in = new FileInputStream(path);
               // Java: BitmapFactory.decodeStream(in, (Rect)null, options);
               // Java: wh[0] = options.outWidth;
               // Java: wh[1] = options.outHeight;
               // TODO: BitmapFactory not available
            }
            // Java: catch (Exception e)
            {
               // Java: KSLog.e(TAG, "Exception", e);
            }
            // Java: finally
            {
               // Java: closeStream(in);
            }
         }
         // Java: return wh;
         return wh;
      }
    }

    // Java: static boolean saveBitmapAsPic(Bitmap bitmap, String bitName, boolean jpeg)
    pub fn saveBitmapAsPic(bitmap: &Element, bitName: &str, jpeg: bool) -> bool {
      // Java: FileOutputStream fOut = null;
      // Java: try
      {
         // Java: fOut = new FileOutputStream(bitName);
         // TODO: FileOutputStream not available
      }
      // Java: catch (FileNotFoundException e)
      {
         // Java: Log.e(TAG, "FileNotFoundException", e);
      }
      // Java: if (null == fOut)
      {
         // Java: return false;
         return false;
      }
      // Java: else
      {
         // Java: boolean res = bitmap.compress(jpeg ? CompressFormat.JPEG : CompressFormat.PNG, 100, fOut);
         // Java: FileUtil.closeQuietly(fOut);
         // Java: return res;
         // TODO: Bitmap.compress not available
      }
    }

    // Java: static boolean saveBitmap(Bitmap bitmap, String bitName)
    pub fn saveBitmap(bitmap: &Element, bitName: &str) -> bool {
      // Java: return saveBitmap(bitmap, bitName, false);
      Self::saveBitmap_with_jpeg(bitmap, bitName, false)
    }

    // Java: static boolean saveBitmap(Bitmap bitmap, String bitName, boolean jpeg)
    pub fn saveBitmap_with_jpeg(bitmap: &Element, bitName: &str, jpeg: bool) -> bool {
      // Java: return saveBitmapAsPic(bitmap, bitName, jpeg);
      Self::saveBitmapAsPic(bitmap, bitName, jpeg)
    }

    // Java: static Bitmap imageWithContentsOfFile(String fileName)
    pub fn imageWithContentsOfFile(fileName: &str) -> Element {
      // Java: Bitmap bitmap = null;
      let mut bitmap: Element = None;
      // Java: InputStream in = null;
      // Java: BufferedOutputStream out = null;
      // Java: try
      {
         // Java: in = new BufferedInputStream(new FileInputStream(new File(fileName)), 4096);
         // Java: ByteArrayOutputStream dataStream = new ByteArrayOutputStream();
         // Java: out = new BufferedOutputStream(dataStream, 4096);
         // Java: FileUtil.copy(in, out);
         // Java: out.flush();
         // Java: byte[] data = dataStream.toByteArray();
         // Java: bitmap = BitmapFactory.decodeByteArray(data, 0, data.length);
         // TODO: BitmapFactory not available
      }
      // Java: catch (Exception e)
      {
         // Java: KSLog.e(TAG, "Could not load Bitmap from: " + fileName, e);
      }
      // Java: finally
      {
         // Java: closeStream(in);
         // Java: closeStream(out);
      }
      // Java: return bitmap;
      bitmap
    }

    // Java: static void closeStream(Closeable stream)
    pub fn closeStream(_stream: &Element) {
      // Java: if (stream != null)
      if _stream.is_some()
      {
         // Java: try
         {
            // Java: stream.close();
            // TODO: Closeable not available in Rust
         }
         // Java: catch (IOException e)
         {
            // Java: KSLog.e(TAG, "Could not close stream", e);
         }
      }
    }

    // Java: static void recycleBitmap(MyBitmap bitmap)
    pub fn recycleBitmap(bitmap: &mut Option<super::super::moffice::drawing::bitmap::MyBitmap::MyBitmap>) {
      // Java: if (null != bitmap && !bitmap.isRecycled())
      if let Some(ref mut bmp) = bitmap {
          if !bmp.isRecycled()
          {
             // Java: bitmap.recycle();
             bmp.recycle();
             // Java: MyBitmap var1 = null;
          }
      }
    }

    // Java: static Bitmap createScaledBitmap(Bitmap bitmap, Rect bmpRect, int dstWidth, int dstHeight)
    pub fn createScaledBitmap_with_rect(bitmap: &Element, bmpRect: Option<[i32; 4]>, dstWidth: i32, mut dstHeight: i32) -> Element {
      // Java: if (null != bitmap && null != bmpRect)
      if bitmap.is_some() && bmpRect.is_some()
      {
         let rect = bmpRect.unwrap();
         // Java: int bmpWidth = bmpRect.width();
         let bmpWidth = rect[2] - rect[0];
         // Java: int bmpHeight = bmpRect.height();
         let bmpHeight = rect[3] - rect[1];
         // Java: if (bmpWidth >= bitmap.getWidth() && bmpHeight >= bitmap.getHeight())
         // TODO: Bitmap.getWidth/getHeight not available
         if bmpWidth > 0 && bmpHeight > 0
         {
            // Java: dstHeight = (int)(1.0F * (float)bmpHeight * (float)dstWidth / (float)bmpWidth);
            dstHeight = (1.0_f32 * bmpHeight as f32 * dstWidth as f32 / bmpWidth as f32) as i32;
            // Java: Bitmap dstBitmap = Bitmap.createBitmap(dstWidth, dstHeight, Config.RGB_565);
            // Java: Canvas canvas = new Canvas(dstBitmap);
            // Java: canvas.drawBitmap(bitmap, bmpRect, new Rect(0, 0, dstWidth, dstHeight), (Paint)null);
            // Java: return dstBitmap;
            // TODO: Bitmap/Canvas not available
            return None;
         }
         // Java: else
         else
         {
            // Java: KSLog.w(TAG, "Source Bitmap is invalid!!!");
            // Java: return null;
            return None;
         }
      }
      // Java: else
      else
      {
         // Java: return null;
         return None;
      }
    }

    // Java: static boolean saveBitmapAsBytes(Bitmap bitmap, String bytesPath)
    pub fn saveBitmapAsBytes(bitmap: &Element, bytesPath: &str) -> bool {
      // Java: int width = bitmap.getWidth();
      let width: i32 = 0; // TODO: Bitmap not available
      // Java: int height = bitmap.getHeight();
      let height: i32 = 0; // TODO: Bitmap not available
      // Java: byte[] bytes = new byte[width * height * 4 + 8];
      let mut bytes: Vec<u8> = vec![0u8; (width * height * 4 + 8) as usize];
      // Java: int offset = 0;
      let mut offset: usize = 0;
      // Java: bytes[offset++] = (byte)(width >>> 24);
      bytes[offset] = ((width as u32) >> 24) as u8; offset += 1;
      // Java: bytes[offset++] = (byte)(width >>> 16);
      bytes[offset] = ((width as u32) >> 16) as u8; offset += 1;
      // Java: bytes[offset++] = (byte)(width >>> 8);
      bytes[offset] = ((width as u32) >> 8) as u8; offset += 1;
      // Java: bytes[offset++] = (byte)width;
      bytes[offset] = width as u8; offset += 1;
      // Java: bytes[offset++] = (byte)(height >>> 24);
      bytes[offset] = ((height as u32) >> 24) as u8; offset += 1;
      // Java: bytes[offset++] = (byte)(height >>> 16);
      bytes[offset] = ((height as u32) >> 16) as u8; offset += 1;
      // Java: bytes[offset++] = (byte)(height >>> 8);
      bytes[offset] = ((height as u32) >> 8) as u8; offset += 1;
      // Java: bytes[offset++] = (byte)height;
      bytes[offset] = height as u8; offset += 1;
      // Java: ByteBuffer buffer = ByteBuffer.wrap(bytes, offset, width * height * 4);
      // Java: bitmap.copyPixelsToBuffer(buffer);
      // TODO: Bitmap.copyPixelsToBuffer not available
      // Java: try
      {
         // Java: FileOutputStream fos = new FileOutputStream(bytesPath);
         // Java: fos.write(bytes);
         // Java: FileUtil.closeQuietly(fos);
         // Java: return true;
         // TODO: FileOutputStream not available
         return false;
      }
      // Java: catch (IOException var8)
      {
         // Java: KSLog.e(TAG, "failed to save bitmap to bytes !");
         // Java: return false;
      }
    }

    // Java: static Bitmap loadBitmapFromBytes(String bytesPath)
    pub fn loadBitmapFromBytes(bytesPath: &str) -> Element {
      // Java: byte[] bytes = new byte[8];
      let mut bytes: Vec<u8> = vec![0u8; 8];
      // Java: try
      {
         // Java: FileInputStream fis = new FileInputStream(bytesPath);
         // Java: fis.read(bytes, 0, 8);
         // TODO: FileInputStream not available
         // Java: int[] widthHeight = byteToInt(bytes, 8);
         let widthHeight = Self::byteToInt(&bytes, 8);
         // Java: bytes = new byte[widthHeight[0] * widthHeight[1] * 4];
         bytes = vec![0u8; (widthHeight[0] * widthHeight[1] * 4) as usize];
         // Java: fis.read(bytes);
         // Java: fis.close();
         // Java: ByteBuffer buffer = ByteBuffer.wrap(bytes);
         // Java: Bitmap bitmap = Bitmap.createBitmap(widthHeight[0], widthHeight[1], Config.ARGB_4444);
         // Java: bitmap.copyPixelsFromBuffer(buffer);
         // Java: return bitmap;
         // TODO: Bitmap not available
         return None;
      }
      // Java: catch (IOException var6)
      {
         // Java: KSLog.e(TAG, "failed to load bitmap from bytes !");
         // Java: return null;
      }
    }

    // Java: static int[] byteToInt(byte[] bytes, int length)
    pub fn byteToInt(bytes: &[u8], length: i32) -> Vec<i32> {
      // Java: int[] intArray = new int[length / 4];
      let mut intArray = vec![0_i32; (length / 4) as usize];
      // Java: int j = 3;
      let mut j: i32 = 3;
      // Java: int intIndex = 0;
      let mut intIndex: usize = 0;
      // Java: int intValue = 0;
      let mut intValue: i32 = 0;
      // Java: for(int i = 0; i < length; ++i)
      for i in 0..length as usize
      {
         // Java: intValue += (bytes[i] & 255) << 8 * j;
         intValue += ((bytes[i] as i32) & 255) << (8 * j);
         // Java: if (0 == j--)
         let old_j = j;
         j -= 1;
         if 0 == old_j
         {
            // Java: intArray[intIndex++] = intValue;
            intArray[intIndex] = intValue;
            intIndex += 1;
            // Java: j = 3;
            j = 3;
            // Java: intValue = 0;
            intValue = 0;
         }
      }
      // Java: return intArray;
      intArray
    }

    // Java: static int sizeOfBitmap(Bitmap bitmap)
    pub fn sizeOfBitmap(_bitmap: &Element) -> i32 {
      // Java: return VERSION.SDK_INT < 12 ? bitmap.getRowBytes() * bitmap.getHeight() : bitmap.getByteCount();
      // TODO: Bitmap not available
      0
    }

    // Java: static void fastBlur(int[] pix, int w, int h, int radius)
    pub fn fastBlur(pix: &mut [i32], w: i32, h: i32, radius: i32) {
      // Java: if (radius >= 1)
      if radius >= 1
      {
         // Java: int wm = w - 1;
         let wm = w - 1;
         // Java: int hm = h - 1;
         let hm = h - 1;
         // Java: int wh = w * h;
         let wh = w * h;
         // Java: int div = radius + radius + 1;
         let div = radius + radius + 1;
         // Java: int[] r = new int[wh];
         let mut r = vec![0_i32; wh as usize];
         // Java: int[] g = new int[wh];
         let mut g = vec![0_i32; wh as usize];
         // Java: int[] b = new int[wh];
         let mut b_arr = vec![0_i32; wh as usize];
         // Java: int[] vmin = new int[Math.max(w, h)];
         let mut vmin = vec![0_i32; w.max(h) as usize];
         // Java: int divsum = div + 1 >> 1;
         let mut divsum = (div + 1) >> 1;
         // Java: divsum *= divsum;
         divsum *= divsum;
         // Java: int[] dv = new int[256 * divsum];
         let mut dv = vec![0_i32; (256 * divsum) as usize];
         // Java: for(int i = 0; i < 256 * divsum; ++i)
         for i in 0..(256 * divsum) as usize
         {
            // Java: dv[i] = i / divsum;
            dv[i] = i as i32 / divsum;
         }
         // Java: int yi = 0;
         let mut yi: usize = 0;
         // Java: int yw = 0;
         let mut yw: usize = 0;
         // Java: int[][] stack = new int[div][3];
         let mut stack: Vec<[i32; 3]> = vec![[0_i32; 3]; div as usize];
         // Java: int r1 = radius + 1;
         let r1 = radius + 1;
         // Java: for(int y = 0; y < h; ++y)
         for y in 0..h
         {
            // Java: int bsum = 0;
            let mut bsum = 0_i32;
            // Java: int gsum = 0;
            let mut gsum = 0_i32;
            // Java: int rsum = 0;
            let mut rsum = 0_i32;
            // Java: int boutsum = 0;
            let mut boutsum = 0_i32;
            // Java: int goutsum = 0;
            let mut goutsum = 0_i32;
            // Java: int routsum = 0;
            let mut routsum = 0_i32;
            // Java: int binsum = 0;
            let mut binsum = 0_i32;
            // Java: int ginsum = 0;
            let mut ginsum = 0_i32;
            // Java: int rinsum = 0;
            let mut rinsum = 0_i32;
            // Java: for(int var47 = -radius; var47 <= radius; ++var47)
            for var47 in -radius..=radius
            {
               // Java: int p = pix[yi + Math.min(wm, Math.max(var47, 0))];
               let p = pix[yi + var47.max(0).min(wm) as usize];
               // Java: int[] sir = stack[var47 + radius];
               let sir_idx = (var47 + radius) as usize;
               // Java: sir[0] = (p & 16711680) >> 16;
               stack[sir_idx][0] = (p & 0x00FF0000) >> 16;
               // Java: sir[1] = (p & '\uff00') >> 8;
               stack[sir_idx][1] = (p & 0x0000FF00) >> 8;
               // Java: sir[2] = p & 255;
               stack[sir_idx][2] = p & 255;
               // Java: int rbs = r1 - Math.abs(var47);
               let rbs = r1 - var47.abs();
               // Java: rsum += sir[0] * rbs;
               rsum += stack[sir_idx][0] * rbs;
               // Java: gsum += sir[1] * rbs;
               gsum += stack[sir_idx][1] * rbs;
               // Java: bsum += sir[2] * rbs;
               bsum += stack[sir_idx][2] * rbs;
               // Java: if (var47 > 0)
               if var47 > 0
               {
                  // Java: rinsum += sir[0];
                  rinsum += stack[sir_idx][0];
                  // Java: ginsum += sir[1];
                  ginsum += stack[sir_idx][1];
                  // Java: binsum += sir[2];
                  binsum += stack[sir_idx][2];
               }
               // Java: else
               else
               {
                  // Java: routsum += sir[0];
                  routsum += stack[sir_idx][0];
                  // Java: goutsum += sir[1];
                  goutsum += stack[sir_idx][1];
                  // Java: boutsum += sir[2];
                  boutsum += stack[sir_idx][2];
               }
            }
            // Java: int stackpointer = radius;
            let mut stackpointer = radius;
            // Java: for(int x = 0; x < w; ++x)
            for x in 0..w
            {
               // Java: r[yi] = dv[rsum];
               r[yi] = dv[rsum as usize];
               // Java: g[yi] = dv[gsum];
               g[yi] = dv[gsum as usize];
               // Java: b[yi] = dv[bsum];
               b_arr[yi] = dv[bsum as usize];
               // Java: rsum -= routsum;
               rsum -= routsum;
               // Java: gsum -= goutsum;
               gsum -= goutsum;
               // Java: bsum -= boutsum;
               bsum -= boutsum;
               // Java: int stackstart = stackpointer - radius + div;
               let stackstart = stackpointer - radius + div;
               // Java: int[] sir = stack[stackstart % div];
               let sir_idx = (stackstart % div) as usize;
               // Java: routsum -= sir[0];
               routsum -= stack[sir_idx][0];
               // Java: goutsum -= sir[1];
               goutsum -= stack[sir_idx][1];
               // Java: boutsum -= sir[2];
               boutsum -= stack[sir_idx][2];
               // Java: if (y == 0)
               if y == 0
               {
                  // Java: vmin[x] = Math.min(x + radius + 1, wm);
                  vmin[x as usize] = (x + radius + 1).min(wm);
               }
               // Java: int p = pix[yw + vmin[x]];
               let p = pix[yw + vmin[x as usize] as usize];
               // Java: sir[0] = (p & 16711680) >> 16;
               stack[sir_idx][0] = (p & 0x00FF0000) >> 16;
               // Java: sir[1] = (p & '\uff00') >> 8;
               stack[sir_idx][1] = (p & 0x0000FF00) >> 8;
               // Java: sir[2] = p & 255;
               stack[sir_idx][2] = p & 255;
               // Java: rinsum += sir[0];
               rinsum += stack[sir_idx][0];
               // Java: ginsum += sir[1];
               ginsum += stack[sir_idx][1];
               // Java: binsum += sir[2];
               binsum += stack[sir_idx][2];
               // Java: rsum += rinsum;
               rsum += rinsum;
               // Java: gsum += ginsum;
               gsum += ginsum;
               // Java: bsum += binsum;
               bsum += binsum;
               // Java: stackpointer = (stackpointer + 1) % div;
               stackpointer = (stackpointer + 1) % div;
               // Java: sir = stack[stackpointer % div];
               let sir_idx2 = (stackpointer % div) as usize;
               // Java: routsum += sir[0];
               routsum += stack[sir_idx2][0];
               // Java: goutsum += sir[1];
               goutsum += stack[sir_idx2][1];
               // Java: boutsum += sir[2];
               boutsum += stack[sir_idx2][2];
               // Java: rinsum -= sir[0];
               rinsum -= stack[sir_idx2][0];
               // Java: ginsum -= sir[1];
               ginsum -= stack[sir_idx2][1];
               // Java: binsum -= sir[2];
               binsum -= stack[sir_idx2][2];
               // Java: ++yi;
               yi += 1;
            }
            // Java: yw += w;
            yw += w as usize;
         }
         // Java: for(int x = 0; x < w; ++x)
         for x in 0..w
         {
            // Java: int bsum = 0;
            let mut bsum = 0_i32;
            // Java: int gsum = 0;
            let mut gsum = 0_i32;
            // Java: int rsum = 0;
            let mut rsum = 0_i32;
            // Java: int boutsum = 0;
            let mut boutsum = 0_i32;
            // Java: int goutsum = 0;
            let mut goutsum = 0_i32;
            // Java: int routsum = 0;
            let mut routsum = 0_i32;
            // Java: int binsum = 0;
            let mut binsum = 0_i32;
            // Java: int ginsum = 0;
            let mut ginsum = 0_i32;
            // Java: int rinsum = 0;
            let mut rinsum = 0_i32;
            // Java: int yp = -radius * w;
            let mut yp = -radius * w;
            // Java: for(int var48 = -radius; var48 <= radius; ++var48)
            for var48 in -radius..=radius
            {
               // Java: yi = Math.max(0, yp) + x;
               yi = (yp.max(0) + x) as usize;
               // Java: int[] sir = stack[var48 + radius];
               let sir_idx = (var48 + radius) as usize;
               // Java: sir[0] = r[yi];
               stack[sir_idx][0] = r[yi];
               // Java: sir[1] = g[yi];
               stack[sir_idx][1] = g[yi];
               // Java: sir[2] = b[yi];
               stack[sir_idx][2] = b_arr[yi];
               // Java: int rbs = r1 - Math.abs(var48);
               let rbs = r1 - var48.abs();
               // Java: rsum += r[yi] * rbs;
               rsum += r[yi] * rbs;
               // Java: gsum += g[yi] * rbs;
               gsum += g[yi] * rbs;
               // Java: bsum += b[yi] * rbs;
               bsum += b_arr[yi] * rbs;
               // Java: if (var48 > 0)
               if var48 > 0
               {
                  // Java: rinsum += sir[0];
                  rinsum += stack[sir_idx][0];
                  // Java: ginsum += sir[1];
                  ginsum += stack[sir_idx][1];
                  // Java: binsum += sir[2];
                  binsum += stack[sir_idx][2];
               }
               // Java: else
               else
               {
                  // Java: routsum += sir[0];
                  routsum += stack[sir_idx][0];
                  // Java: goutsum += sir[1];
                  goutsum += stack[sir_idx][1];
                  // Java: boutsum += sir[2];
                  boutsum += stack[sir_idx][2];
               }
               // Java: if (var48 < hm)
               if var48 < hm
               {
                  // Java: yp += w;
                  yp += w;
               }
            }
            // Java: yi = x;
            yi = x as usize;
            // Java: int stackpointer = radius;
            let mut stackpointer = radius;
            // Java: for(int var46 = 0; var46 < h; ++var46)
            for var46 in 0..h
            {
               // Java: pix[yi] = -16777216 & pix[yi] | dv[rsum] << 16 | dv[gsum] << 8 | dv[bsum];
               pix[yi] = (-16777216_i32 & pix[yi]) | (dv[rsum as usize] << 16) | (dv[gsum as usize] << 8) | dv[bsum as usize];
               // Java: rsum -= routsum;
               rsum -= routsum;
               // Java: gsum -= goutsum;
               gsum -= goutsum;
               // Java: bsum -= boutsum;
               bsum -= boutsum;
               // Java: int stackstart = stackpointer - radius + div;
               let stackstart = stackpointer - radius + div;
               // Java: int[] sir = stack[stackstart % div];
               let sir_idx = (stackstart % div) as usize;
               // Java: routsum -= sir[0];
               routsum -= stack[sir_idx][0];
               // Java: goutsum -= sir[1];
               goutsum -= stack[sir_idx][1];
               // Java: boutsum -= sir[2];
               boutsum -= stack[sir_idx][2];
               // Java: if (x == 0)
               if x == 0
               {
                  // Java: vmin[var46] = Math.min(var46 + r1, hm) * w;
                  vmin[var46 as usize] = (var46 + r1).min(hm) * w;
               }
               // Java: int p = x + vmin[var46];
               let p = (x + vmin[var46 as usize]) as usize;
               // Java: sir[0] = r[p];
               stack[sir_idx][0] = r[p];
               // Java: sir[1] = g[p];
               stack[sir_idx][1] = g[p];
               // Java: sir[2] = b[p];
               stack[sir_idx][2] = b_arr[p];
               // Java: rinsum += sir[0];
               rinsum += stack[sir_idx][0];
               // Java: ginsum += sir[1];
               ginsum += stack[sir_idx][1];
               // Java: binsum += sir[2];
               binsum += stack[sir_idx][2];
               // Java: rsum += rinsum;
               rsum += rinsum;
               // Java: gsum += ginsum;
               gsum += ginsum;
               // Java: bsum += binsum;
               bsum += binsum;
               // Java: stackpointer = (stackpointer + 1) % div;
               stackpointer = (stackpointer + 1) % div;
               // Java: sir = stack[stackpointer];
               let sir_idx2 = stackpointer as usize;
               // Java: routsum += sir[0];
               routsum += stack[sir_idx2][0];
               // Java: goutsum += sir[1];
               goutsum += stack[sir_idx2][1];
               // Java: boutsum += sir[2];
               boutsum += stack[sir_idx2][2];
               // Java: rinsum -= sir[0];
               rinsum -= stack[sir_idx2][0];
               // Java: ginsum -= sir[1];
               ginsum -= stack[sir_idx2][1];
               // Java: binsum -= sir[2];
               binsum -= stack[sir_idx2][2];
               // Java: yi += w;
               yi += w as usize;
            }
         }
      }
    }

}
