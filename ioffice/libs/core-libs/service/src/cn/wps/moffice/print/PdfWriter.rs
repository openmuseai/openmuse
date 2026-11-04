// Java: import android.graphics.Bitmap
use stubs::android::graphics::Bitmap::*;
// Java: import android.graphics.Point
use stubs::android::graphics::Point::*;
// Java: import android.graphics.Bitmap.CompressFormat
use stubs::android::graphics::Bitmap::*;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_app::cn::wps::moffice::kfs::RandomAccessFile::*;
// Java: import cn.wps.moffice.util.FileUtil
use util::cn::wps::moffice::util::FileUtil::*;
// Java: import java.io.ByteArrayOutputStream
use stubs::java::io::ByteArrayOutputStream::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.util.ArrayList
use stubs::java::util::ArrayList::*;
// Java: import java.util.Locale
use stubs::java::util::Locale::*;

// Java: public class PdfWriter 
pub struct PdfWriter {
    // Java field:    private int mCurObjIndex = 0;
    mCurObjIndex: i32,
    // Java field:    private long mStartXref = 0L;
    mStartXref: i64,
    // Java field:    private int mPageCount = 0;
    mPageCount: i32,
    // Java field:    private ArrayList<Long> mObjStartPos = new ArrayList();
    mObjStartPos: Vec<i64>,
    // Java field:    private ArrayList<Point> mPagesSize = new ArrayList();
    mPagesSize: Vec<Point>,
    // Java field:    private RandomAccessFile mWriter = null;
    mWriter: RandomAccessFile,
    // Java field:    private StringBuilder mStringBuilder = new StringBuilder();
    mStringBuilder: StringBuilder,
    // Java field:    private final String OBJ_TEMPLATE = "<<\n/Length %d\n/Type /XObject\n/Subtype /Image\n/Name /Im%d\n/Width %d\n/Height %d\n/BitsPerComponent 8\n/ColorSpace /DeviceRGB\n/Filter /DCTDecode >>\n";
    OBJ_TEMPLATE: String,
}

// Java: public class PdfWriter 
impl PdfWriter {
    // Java: public PdfWriter() 
    pub fn new() -> PdfWriter {
      Self { mCurObjIndex: Default::default(), mStartXref: Default::default(), mPageCount: Default::default(), mObjStartPos: Default::default(), mPagesSize: Default::default(), mWriter: Default::default(), mStringBuilder: Default::default(), OBJ_TEMPLATE: Default::default() }
    }

    // Java: public boolean createPdf(String filename) 
    pub fn createPdf(&mut self, filename: &str) -> bool {
      // Java: try
      {
         // Java: FileUtil.newFile(filename);
         FileUtil.newFile(filename);
         // Java: this.mWriter = new RandomAccessFile(filename, "rw");
         self.mWriter = RandomAccessFile::new(filename, "rw");
         // Java: this.mWriter.setLength(0L);
         self.mWriter.setLength(0);
         // Java: this.writeHeader();
         self.writeHeader();
         // Java: return true;
         return true;
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
         e.printStackTrace();
         // Java: return false;
         return false;
      }
    }

    // Java: public void close() 
    pub fn close(&mut self) {
      // Java: try
      {
         // Java: if (this.mWriter == null)
         if self.mWriter.is_none() {
            // Java: return;
            return;
         }
         // Java: this.writePagesTree();
         self.writePagesTree();
         // Java: this.mWriter.close();
         self.mWriter.close();
         // Java: this.reset();
         self.reset();
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
         e.printStackTrace();
      }
    }

    // Java: public boolean addPageObj(Bitmap bitmap, int pageWidth, int pageHeight, boolean portrait) 
    pub fn addPageObj(&mut self, bitmap: Bitmap, pageWidth: i32, pageHeight: i32, portrait: bool) -> bool {
      // Java: try
      {
         // Java: if (this.mWriter == null)
         if self.mWriter.is_none() {
            // Java: return false;
            return false;
         }
         // Java: else
         else {
            // Java: this.addObj();
            self.addObj();
            // Java: this.savePageSize(pageWidth, pageHeight, portrait);
            self.savePageSize(pageWidth, pageHeight, portrait);
            // Java: ByteArrayOutputStream baos = new ByteArrayOutputStream();
            let baos: ByteArrayOutputStream = ByteArrayOutputStream::new();
            // Java: bitmap.compress(CompressFormat.JPEG, 100, baos);
            bitmap.compress(CompressFormat.JPEG, 100, baos);
            // Java: byte[] byteData = baos.toByteArray();
            let byteData: Vec<i8> = baos.toByteArray();
            // Java: this.mWriter.write(String.format(Locale.ENGLISH, "%d %d obj\n", this.mCurObjIndex, 0).getBytes());
            self.mWriter.write(String.format(Locale.ENGLISH, "%d %d obj\n", self.mCurObjIndex, 0).getBytes());
            // Java: this.mWriter.write(String.format(Locale.ENGLISH, "<<\n/Length %d\n/Type /XObject\n/Subtype /Image\n/Name /Im%d\n/Width %d\n/Height %d\n/BitsPerComponent 8\n/ColorSpace /DeviceRGB\n/Filter /DCTDecode >>\n", byteData.length, this.mCurObjIndex, bitmap.getWidth(), bitmap.getHeight()).getBytes());
            self.mWriter.write(String.format(Locale.ENGLISH, "<<\n/Length %d\n/Type /XObject\n/Subtype /Image\n/Name /Im%d\n/Width %d\n/Height %d\n/BitsPerComponent 8\n/ColorSpace /DeviceRGB\n/Filter /DCTDecode >>\n", byteData.length, self.mCurObjIndex, bitmap.getWidth(), bitmap.getHeight()).getBytes());
            // Java: this.mWriter.write("stream\n".getBytes());
            self.mWriter.write("stream\n".getBytes());
            // Java: this.mWriter.write(byteData);
            self.mWriter.write(byteData);
            // Java: this.mWriter.write("\nendstream\nendobj\n".getBytes());
            self.mWriter.write("\nendstream\nendobj\n".getBytes());
            // Java: return true;
            return true;
         }
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
         e.printStackTrace();
         // Java: return false;
         return false;
      }
      // Java: catch (OutOfMemoryError var8)
      {
         // Java: return false;
         return false;
      }
    }

    // Java: private final long getFileOffset() 
    pub fn getFileOffset(&mut self) -> i64 {
      // Java: long offset = -1L;
      let offset: i64 = -1;
      // Java: try
      {
         // Java: offset = this.mWriter.getFilePointer();
         offset = self.mWriter.getFilePointer();
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
         e.printStackTrace();
      }
      // Java: return offset;
      return offset;
    }

    // Java: private final void addObj() 
    pub fn addObj(&mut self) {
      // Java: ++this.mCurObjIndex;
      self.mCurObjIndex;
      // Java: this.mObjStartPos.add(this.getFileOffset());
      self.mObjStartPos.add(self.getFileOffset());
    }

    // Java: private void reset() 
    pub fn reset(&mut self) {
      // Java: this.mCurObjIndex = 0;
      self.mCurObjIndex = 0;
      // Java: this.mStartXref = 0L;
      self.mStartXref = 0;
      // Java: this.mPageCount = 0;
      self.mPageCount = 0;
      // Java: this.mObjStartPos.clear();
      self.mObjStartPos.clear();
      // Java: this.mPagesSize.clear();
      self.mPagesSize.clear();
    }

    // Java: private void writeHeader() 
    pub fn writeHeader(&mut self) {
      // Java: try
      {
         // Java: this.reset();
         self.reset();
         // Java: this.mWriter.write("%PDF-1.5\n%忏嫌\n".getBytes());
         self.mWriter.write("%PDF-1.5\n%忏嫌\n".getBytes());
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
         e.printStackTrace();
      }
    }

    // Java: private void savePageSize(int pageWidth, int pageHeight, boolean portrait) 
    pub fn savePageSize(&mut self, pageWidth: i32, pageHeight: i32, portrait: bool) {
      // Java: this.mPagesSize.add(new Point(pageWidth, pageHeight));
      self.mPagesSize.add(Point::new(pageWidth, pageHeight));
    }

    // Java: private void writePagesTree() 
    pub fn writePagesTree(&mut self) {
      // Java: this.mPageCount = this.mCurObjIndex;
      self.mPageCount = self.mCurObjIndex;
      // Java: this.writeCatalogObj();
      self.writeCatalogObj();
      // Java: this.writePagesObj();
      self.writePagesObj();
      // Java: this.writeXref();
      self.writeXref();
      // Java: this.writeTrailer();
      self.writeTrailer();
    }

    // Java: private void writeCatalogObj() 
    pub fn writeCatalogObj(&mut self) {
      // Java: try
      {
         // Java: this.addObj();
         self.addObj();
         // Java: this.mWriter.write(String.format(Locale.ENGLISH, "%d 0 obj\n<<\n/Type /Catalog\n/Pages %d 0 R\n/PageLayout/OneColumn\n/OpenAction[%d 0 R/FitH 10000]\n>>\nendobj\n", this.mCurObjIndex, this.mCurObjIndex + 1, this.mCurObjIndex + 2).getBytes());
         self.mWriter.write(String.format(Locale.ENGLISH, "%d 0 obj\n<<\n/Type /Catalog\n/Pages %d 0 R\n/PageLayout/OneColumn\n/OpenAction[%d 0 R/FitH 10000]\n>>\nendobj\n", self.mCurObjIndex, self.mCurObjIndex + 1, self.mCurObjIndex + 2).getBytes());
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
         e.printStackTrace();
      }
    }

    // Java: private void writePagesObj() 
    pub fn writePagesObj(&mut self) {
      // Java: try
      {
         // Java: this.addObj();
         self.addObj();
         // Java: int nParentIndex = this.mCurObjIndex;
         let nParentIndex: i32 = self.mCurObjIndex;
         // Java: int nPageIndex = this.mCurObjIndex + 1;
         let nPageIndex: i32 = self.mCurObjIndex + 1;
         // Java: this.mStringBuilder.setLength(0);
         self.mStringBuilder.setLength(0);
         // Java: this.mStringBuilder.append("[");
         self.mStringBuilder.append("[");
         // Java: for(int i = 0; i < this.mPageCount; ++i)
         if true {
            // Java: this.mStringBuilder.append(nPageIndex).append(" 0 R ");
            self.mStringBuilder.append(nPageIndex).append(" 0 R ");
            // Java: nPageIndex += 2;
            nPageIndex = nPageIndex + 2;
         }
         // Java: this.mStringBuilder.append("]");
         self.mStringBuilder.append("]");
         // Java: this.mWriter.write(String.format(Locale.ENGLISH, "%d 0 obj\n<<\n/Type /Pages\n/Kids %s\n/Count %d\n>>\nendobj\n", this.mCurObjIndex, this.mStringBuilder, this.mPageCount).getBytes());
         self.mWriter.write(String.format(Locale.ENGLISH, "%d 0 obj\n<<\n/Type /Pages\n/Kids %s\n/Count %d\n>>\nendobj\n", self.mCurObjIndex, self.mStringBuilder, self.mPageCount).getBytes());
         // Java: for(int i = 0; i < this.mPageCount; ++i)
         if true {
            // Java: Point pt = (Point)this.mPagesSize.get(i);
            let pt: Point = (self.mPagesSize.get(i)) as Point;
            // Java: this.writePageObj(nParentIndex, i + 1, pt.x, pt.y);
            self.writePageObj(nParentIndex, i + 1, pt.x, pt.y);
         }
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
         e.printStackTrace();
      }
    }

    // Java: private void writePageObj(int nParentIndex, int nPageNum, int nPageWidth, int nPageHeight) 
    pub fn writePageObj(&mut self, nParentIndex: i32, nPageNum: i32, nPageWidth: i32, nPageHeight: i32) {
      // Java: try
      {
         // Java: this.addObj();
         self.addObj();
         // Java: this.mWriter.write(String.format(Locale.ENGLISH, "%d 0 obj\n<<\n/Type /Page\n/Parent %d 0 R\n/MediaBox [0.0000 0.0000 %d %d]\n/Contents %d 0 R\n/Resources <<\n/XObject <<\n/Im%d %d 0 R >>\n/ProcSet [/ImageC]\n>>\n>>\nendobj\n", this.mCurObjIndex, nParentIndex, nPageWidth, nPageHeight, this.mCurObjIndex + 1, nPageNum, nPageNum).getBytes());
         self.mWriter.write(String.format(Locale.ENGLISH, "%d 0 obj\n<<\n/Type /Page\n/Parent %d 0 R\n/MediaBox [0.0000 0.0000 %d %d]\n/Contents %d 0 R\n/Resources <<\n/XObject <<\n/Im%d %d 0 R >>\n/ProcSet [/ImageC]\n>>\n>>\nendobj\n", self.mCurObjIndex, nParentIndex, nPageWidth, nPageHeight, self.mCurObjIndex + 1, nPageNum, nPageNum).getBytes());
         // Java: this.addObj();
         self.addObj();
         // Java: String str = String.format(Locale.ENGLISH, "q  %d 0 0 %d 0 0 cm /Im%d Do Q\n", nPageWidth, nPageHeight, nPageNum);
         let str: String = String.format(Locale.ENGLISH, "q  %d 0 0 %d 0 0 cm /Im%d Do Q\n", nPageWidth, nPageHeight, nPageNum);
         // Java: this.mWriter.write(String.format(Locale.ENGLISH, "%d 0 obj\n<<\n/Length %d\n>>\nstream\n%s\nendstream\nendobj\n", this.mCurObjIndex, str.length(), str).getBytes());
         self.mWriter.write(String.format(Locale.ENGLISH, "%d 0 obj\n<<\n/Length %d\n>>\nstream\n%s\nendstream\nendobj\n", self.mCurObjIndex, str.length(), str).getBytes());
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
         e.printStackTrace();
      }
    }

    // Java: private void writeXref() 
    pub fn writeXref(&mut self) {
      // Java: try
      {
         // Java: this.mStartXref = this.getFileOffset();
         self.mStartXref = self.getFileOffset();
         // Java: int size = this.mObjStartPos.size();
         let size: i32 = self.mObjStartPos.size();
         // Java: int totalSize = size + 1;
         let totalSize: i32 = size + 1;
         // Java: this.mWriter.write(String.format(Locale.ENGLISH, "xref\n0 %d\n", totalSize).getBytes());
         self.mWriter.write(String.format(Locale.ENGLISH, "xref\n0 %d\n", totalSize).getBytes());
         // Java: this.mWriter.write(String.format(Locale.ENGLISH, "%010d 65535 f\r\n", 0).getBytes());
         self.mWriter.write(String.format(Locale.ENGLISH, "%010d 65535 f\r\n", 0).getBytes());
         // Java: for(int i = 0; i < size; ++i)
         if true {
            // Java: this.mWriter.write(String.format(Locale.ENGLISH, "%010d 00000 n\r\n", this.mObjStartPos.get(i)).getBytes());
            self.mWriter.write(String.format(Locale.ENGLISH, "%010d 00000 n\r\n", self.mObjStartPos.get(i)).getBytes());
         }
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
         e.printStackTrace();
      }
    }

    // Java: private void writeTrailer() 
    pub fn writeTrailer(&mut self) {
      // Java: try
      {
         // Java: this.mWriter.write(String.format(Locale.ENGLISH, "trailer\n<<\n/Size %d\n/Root %d 0 R\n>>\nstartxref\n%d\n%%%%EOF", this.mObjStartPos.size() + 1, this.mPageCount + 1, this.mStartXref).getBytes());
         self.mWriter.write(String.format(Locale.ENGLISH, "trailer\n<<\n/Size %d\n/Root %d 0 R\n>>\nstartxref\n%d\n%%%%EOF", self.mObjStartPos.size() + 1, self.mPageCount + 1, self.mStartXref).getBytes());
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
         e.printStackTrace();
      }
    }

}
