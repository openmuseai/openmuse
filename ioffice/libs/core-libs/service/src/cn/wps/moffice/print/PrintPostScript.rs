// Java: import android.graphics.Bitmap
use stubs::android::graphics::Bitmap::*;
// Java: import android.graphics.Bitmap.CompressFormat
use stubs::android::graphics::Bitmap::*;
// Java: import android.util.Log
use stubs::android::util::Log::*;
// Java: import cn.wps.moffice.kfs.File
use kfs_app::cn::wps::moffice::kfs::File::*;
// Java: import cn.wps.moffice.kfs.FileWriter
use kfs_app::cn::wps::moffice::kfs::FileWriter::*;
// Java: import cn.wps.moffice.util.FileUtil
use util::cn::wps::moffice::util::FileUtil::*;
// Java: import java.io.BufferedWriter
use stubs::java::io::BufferedWriter::*;
// Java: import java.io.ByteArrayOutputStream
use stubs::java::io::ByteArrayOutputStream::*;

// Java field:    private static final String SHOW_PAGE = "\nshowpage\n";
const SHOW_PAGE: &str = "\nshowpage\n";
// Java field:    private static final String PAGE_PORTRAIT = "/Orientation 0";
const PAGE_PORTRAIT: &str = "/Orientation 0";
// Java field:    private static final String PAGE_LANDSCAPE = "/Orientation 3";
const PAGE_LANDSCAPE: &str = "/Orientation 3";
// Java field:    private static final String PAGE_COPIES = "/NumCopies ";
const PAGE_COPIES: &str = "/NumCopies ";
// Java field:    private static final String PAGE_COLLATE = "/Collate ";
const PAGE_COLLATE: &str = "/Collate ";
// Java field:    private static final char RETURN = '\n';
const RETURN: u16 = '\n';
// Java field:    private static final char SPACE = ' ';
const SPACE: u16 = ' ';
// Java field:    private static final char IMAGE_END_FLAG = '>';
const IMAGE_END_FLAG: u16 = '>';
// Java field:    private static final short BMP_HEX_DECODE = 0;
const BMP_HEX_DECODE: i16 = 0;
// Java field:    private static final short JPEG_HEX_DCT_DECODE = 1;
const JPEG_HEX_DCT_DECODE: i16 = 1;

// Java: public class PrintPostScript implements IPrintExport 
#[derive(Default)]
pub struct PrintPostScript {
    // Java field:    private StringBuffer mDataBuffer = null;
    mDataBuffer: StringBuffer,
    // Java field:    private BufferedWriter mPsWriter = null;
    mPsWriter: BufferedWriter,
    // Java field:    private boolean isOpen = false;
    isOpen: bool,
    // Java field:    private int mPageCopies;
    mPageCopies: i32,
    // Java field:    private boolean mCollate;
    mCollate: bool,
}

// Java: public interface IPrintExport 
impl IPrintExport for PrintPostScript {
    // Java: public boolean createDocument(String fullFilePath) 
    pub fn createDocument(&mut self, fullFilePath: String) -> bool {
      // Java: File out = new File(fullFilePath);
      let out: File = File::new(fullFilePath);
      // Java: try
      {
         // Java: FileUtil.newFile(out.getPath());
         FileUtil::newFile(out.getPath());
         // Java: this.mPsWriter = new BufferedWriter(new FileWriter(out));
         self.mPsWriter = BufferedWriter::new(FileWriter::new(out));
      }
      // Java: catch (Exception e)
      {
         // Java: e.printStackTrace();
         e.printStackTrace();
         // Java: Log.w("print", "open ps file failed");
         Log::w("print", "open ps file failed");
         // Java: return false;
         return false;
      }
      // Java: if (this.mDataBuffer == null)
      if self.mDataBuffer.is_none() {
         // Java: this.mDataBuffer = new StringBuffer(1024);
         self.mDataBuffer = StringBuffer::new(1024);
      }
      // Java: this.mPageCopies = 1;
      self.mPageCopies = 1;
      // Java: this.mCollate = true;
      self.mCollate = true;
      // Java: this.isOpen = true;
      self.isOpen = true;
      // Java: return true;
      return true;
    }

    // Java: public void closeDocument() 
    pub fn closeDocument(&mut self) {
      // Java: if (this.isOpen)
      if self.isOpen {
         // Java: try
         {
            // Java: this.mPsWriter.flush();
            self.mPsWriter.flush();
            // Java: this.mPsWriter.close();
            self.mPsWriter.close();
         }
         // Java: catch (Exception var2)
         {
            // Java: Log.w("print", "print ps failed");
            Log::w("print", "print ps failed");
            // Java: this.mPsWriter = null;
            self.mPsWriter = None;
         }
         // Java: this.isOpen = false;
         self.isOpen = false;
      }
    }

    // Java: public void destroy() 
    pub fn destroy(&mut self) {
      // Java: this.mDataBuffer = null;
      self.mDataBuffer = None;
      // Java: this.mPsWriter = null;
      self.mPsWriter = None;
    }

    // Java: public boolean addPage(Bitmap bitmap, int pageWidth, int pageHeight, boolean portrait) 
    pub fn addPage(&mut self, bitmap: Bitmap, pageWidth: i32, pageHeight: i32, portrait: bool) -> bool {
      // Java: try
      {
         // Java: ByteArrayOutputStream stream = new ByteArrayOutputStream();
         let stream: ByteArrayOutputStream = ByteArrayOutputStream::new();
         // Java: bitmap.compress(CompressFormat.JPEG, 100, stream);
         bitmap.compress(CompressFormat::JPEG, 100, stream);
         // Java: int width = bitmap.getWidth();
         let width: i32 = bitmap.getWidth();
         // Java: int height = bitmap.getHeight();
         let height: i32 = bitmap.getHeight();
         // Java: this.mPsWriter.write(this.genPsHeader(width, height, pageWidth, pageHeight, portrait, (short)1));
         self.mPsWriter.write(self.genPsHeader(width, height, pageWidth, pageHeight, portrait, ((1) as i16)));
         // Java: byte[] jpegData = stream.toByteArray();
         let jpegData: Vec<u8> = stream.toByteArray();
         // Java: int step = width * 16;
         let step: i32 = width * 16;
         let index: i32 = 0;
         // Java: for(int index = 0; index < jpegData.length; index += step)
         while index < jpegData.length {
            // Java: this.mDataBuffer.setLength(0);
            self.mDataBuffer.setLength(0);
            // Java: bytesToHexString(jpegData, index, index + step, this.mDataBuffer);
            self.bytesToHexString(jpegData, index, index + step, self.mDataBuffer);
            // Java: this.mPsWriter.write(this.mDataBuffer.toString());
            self.mPsWriter.write(self.mDataBuffer.toString());
            // Java: this.mPsWriter.write(10);
            self.mPsWriter.write(10);
            index = index + step;
         }
         // Java: this.mPsWriter.write(62);
         self.mPsWriter.write(62);
         // Java: this.mPsWriter.write("\nshowpage\n");
         self.mPsWriter.write("\nshowpage\n");
         // Java: return true;
         return true;
      }
      // Java: catch (Exception var11)
      {
         // Java: Log.w("print", "print ps failed");
         Log::w("print", "print ps failed");
         // Java: return false;
         return false;
      }
      // Java: catch (OutOfMemoryError var12)
      {
         // Java: Log.w("print", "out of memory error");
         Log::w("print", "out of memory error");
         // Java: return false;
         return false;
      }
    }

}

// Java: public class PrintPostScript implements IPrintExport 
impl PrintPostScript {
    // Java: public PrintPostScript() 
    pub fn new() -> PrintPostScript {
      Self { mDataBuffer: Default::default(), mPsWriter: Default::default(), isOpen: Default::default(), mPageCopies: Default::default(), mCollate: Default::default() }
    }

    // Java: public void setPageCopies(int copy, boolean collate) 
    pub fn setPageCopies(&mut self, copy: i32, collate: bool) {
      // Java: this.mPageCopies = copy > 1 ? copy : 1;
      self.mPageCopies = (if copy > 1 { copy } else { 1 });
      // Java: this.mCollate = collate;
      self.mCollate = collate;
    }

    // Java: public boolean addPageUncompress(Bitmap bitmap, int pageWidth, int pageHeight, boolean portrait) 
    pub fn addPageUncompress(&mut self, bitmap: Bitmap, pageWidth: i32, pageHeight: i32, portrait: bool) -> bool {
      // Java: int width = bitmap.getWidth();
      let width: i32 = bitmap.getWidth();
      // Java: int height = bitmap.getHeight();
      let height: i32 = bitmap.getHeight();
      // Java: try
      {
         // Java: this.mPsWriter.write(this.genPsHeader(width, height, pageWidth, pageHeight, portrait, (short)0));
         self.mPsWriter.write(self.genPsHeader(width, height, pageWidth, pageHeight, portrait, ((0) as i16)));
         let y: i32 = 0;
         // Java: for(int y = 0; y < bitmap.getHeight(); ++y)
         while y < bitmap.getHeight() {
            // Java: this.mDataBuffer.setLength(0);
            self.mDataBuffer.setLength(0);
            let x: i32 = 0;
            // Java: for(int x = 0; x < bitmap.getWidth(); ++x)
            while x < bitmap.getWidth() {
               // Java: rgbToHexString(bitmap.getPixel(x, y), this.mDataBuffer);
               self.rgbToHexString(bitmap.getPixel(x, y), self.mDataBuffer);
               x += 1;
            }
            // Java: this.mPsWriter.write(this.mDataBuffer.toString());
            self.mPsWriter.write(self.mDataBuffer.toString());
            // Java: this.mPsWriter.write(10);
            self.mPsWriter.write(10);
            y += 1;
         }
         // Java: this.mPsWriter.write("\nshowpage\n");
         self.mPsWriter.write("\nshowpage\n");
         // Java: return true;
         return true;
      }
      // Java: catch (Exception var9)
      {
         // Java: Log.w("print", "print ps failed");
         Log::w("print", "print ps failed");
         // Java: return false;
         return false;
      }
    }

    // Java: private String genPsHeader(int width, int height, int pageWidth, int pageHeight, boolean portrait, short decode) 
    pub fn genPsHeader(&mut self, width: i32, height: i32, pageWidth: i32, pageHeight: i32, portrait: bool, decode: i16) -> String {
      // Java: this.mDataBuffer.setLength(0);
      self.mDataBuffer.setLength(0);
      // Java: this.mDataBuffer.append("<</PageSize [");
      self.mDataBuffer.append("<</PageSize [");
      // Java: this.mDataBuffer.append(pageWidth);
      self.mDataBuffer.append(pageWidth);
      // Java: this.mDataBuffer.append(' ');
      self.mDataBuffer.append(' ' as u16);
      // Java: this.mDataBuffer.append(pageHeight);
      self.mDataBuffer.append(pageHeight);
      // Java: this.mDataBuffer.append(']');
      self.mDataBuffer.append(']' as u16);
      // Java: this.mDataBuffer.append(' ');
      self.mDataBuffer.append(' ' as u16);
      // Java: if (portrait)
      if portrait {
         // Java: this.mDataBuffer.append("/Orientation 0");
         self.mDataBuffer.append("/Orientation 0");
      }
      // Java: else
      else {
         // Java: this.mDataBuffer.append("/Orientation 3");
         self.mDataBuffer.append("/Orientation 3");
      }
      // Java: if (this.mPageCopies > 1)
      if self.mPageCopies > 1 {
         // Java: this.mDataBuffer.append(' ');
         self.mDataBuffer.append(' ' as u16);
         // Java: this.mDataBuffer.append("/NumCopies ");
         self.mDataBuffer.append("/NumCopies ");
         // Java: this.mDataBuffer.append(this.mPageCopies);
         self.mDataBuffer.append(self.mPageCopies);
         // Java: this.mDataBuffer.append(' ');
         self.mDataBuffer.append(' ' as u16);
         // Java: this.mDataBuffer.append("/Collate ");
         self.mDataBuffer.append("/Collate ");
         // Java: this.mDataBuffer.append(this.mCollate);
         self.mDataBuffer.append(self.mCollate);
      }
      // Java: this.mDataBuffer.append(">> setpagedevice\n");
      self.mDataBuffer.append(">> setpagedevice\n");
      // Java: this.mDataBuffer.append(pageWidth);
      self.mDataBuffer.append(pageWidth);
      // Java: this.mDataBuffer.append(' ');
      self.mDataBuffer.append(' ' as u16);
      // Java: this.mDataBuffer.append(pageHeight);
      self.mDataBuffer.append(pageHeight);
      // Java: this.mDataBuffer.append(" scale");
      self.mDataBuffer.append(" scale");
      // Java: this.mDataBuffer.append('\n');
      self.mDataBuffer.append('\n' as u16);
      // Java: this.mDataBuffer.append(width);
      self.mDataBuffer.append(width);
      // Java: this.mDataBuffer.append(' ');
      self.mDataBuffer.append(' ' as u16);
      // Java: this.mDataBuffer.append(height);
      self.mDataBuffer.append(height);
      // Java: this.mDataBuffer.append(" 8 [");
      self.mDataBuffer.append(" 8 [");
      // Java: this.mDataBuffer.append(width);
      self.mDataBuffer.append(width);
      // Java: this.mDataBuffer.append(" 0 0 -");
      self.mDataBuffer.append(" 0 0 -");
      // Java: this.mDataBuffer.append(height);
      self.mDataBuffer.append(height);
      // Java: this.mDataBuffer.append(" 0 ");
      self.mDataBuffer.append(" 0 ");
      // Java: this.mDataBuffer.append(height);
      self.mDataBuffer.append(height);
      // Java: this.mDataBuffer.append("]\n");
      self.mDataBuffer.append("]\n");
      // Java: if (decode == 1)
      if decode == 1 {
         // Java: this.mDataBuffer.append("currentfile\n/ASCIIHexDecode filter\n/DCTDecode filter\nfalse 3 colorimage\n");
         self.mDataBuffer.append("currentfile\n/ASCIIHexDecode filter\n/DCTDecode filter\nfalse 3 colorimage\n");
      }
      // Java: else
      else {
         // Java: this.mDataBuffer.append("{currentfile 3 ");
         self.mDataBuffer.append("{currentfile 3 ");
         // Java: this.mDataBuffer.append(width);
         self.mDataBuffer.append(width);
         // Java: this.mDataBuffer.append(" mul string readhexstring pop} bind\n false 3 colorimage\n");
         self.mDataBuffer.append(" mul string readhexstring pop} bind\n false 3 colorimage\n");
      }
      // Java: return this.mDataBuffer.toString();
      return self.mDataBuffer.toString();
    }

    // Java: private static void bytesToHexString(byte[] src, int start, int len, StringBuffer buffer) 
    pub fn bytesToHexString(src: Vec<u8>, start: i32, len: i32, buffer: StringBuffer) {
      let i: i32 = start;
      // Java: for(int i = start; i < len && i < src.length; ++i)
      while i < len && i < src.length {
         // Java: byte b = src[i];
         let b: u8 = src[i];
         // Java: buffer.append(toHexChar(b >>> 4 & 15));
         buffer.append(self.toHexChar(b >> 4 & 15));
         // Java: buffer.append(toHexChar(b & 15));
         buffer.append(self.toHexChar(b & 15));
         i += 1;
      }
    }

    // Java: private static void rgbToHexString(int argb, StringBuffer buffer) 
    pub fn rgbToHexString(argb: i32, buffer: StringBuffer) {
      // Java: byteToHexString((byte)(argb >> 16 & 255), buffer);
      self.byteToHexString(((argb >> 16 & 255) as u8), buffer);
      // Java: byteToHexString((byte)(argb >> 8 & 255), buffer);
      self.byteToHexString(((argb >> 8 & 255) as u8), buffer);
      // Java: byteToHexString((byte)(argb & 255), buffer);
      self.byteToHexString(((argb & 255) as u8), buffer);
    }

    // Java: private static void byteToHexString(byte b, StringBuffer buffer) 
    pub fn byteToHexString(b: u8, buffer: StringBuffer) {
      // Java: buffer.append(toHexChar(b >>> 4 & 15));
      buffer.append(self.toHexChar(b >> 4 & 15));
      // Java: buffer.append(toHexChar(b & 15));
      buffer.append(self.toHexChar(b & 15));
    }

    // Java: private static char toHexChar(int b) 
    pub fn toHexChar(b: i32) -> u16 {
      // Java: return 0 <= b && b < 16 ? (char)(b < 10 ? b + 48 : b + 97 - 10) : '\u0000';
      return (if 0 <= b && b < 16 { (((if b < 10 { b + 48 } else { b + 97 - 10 })) as u16) } else { '\u0000' as u16 });
    }

}
