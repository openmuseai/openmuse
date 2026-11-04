// Java: import android.graphics.Bitmap
use stubs::android::graphics::Bitmap::*;

// Java: public class PrintPDF implements IPrintExport 
pub struct PrintPDF {
    // Java field:    private PdfWriter writer = new PdfWriter();
    writer: PdfWriter,
    // Java field:    private boolean isOpen = false;
    isOpen: bool,
}

// Java: public interface IPrintExport 
impl IPrintExport for PrintPDF {
    // Java: public boolean createDocument(String fullFilePath) 
    pub fn createDocument(&mut self, fullFilePath: &str) -> bool {
      // Java: this.isOpen = this.writer.createPdf(fullFilePath);
      self.isOpen = self.writer.createPdf(fullFilePath);
      // Java: return this.isOpen;
      return self.isOpen;
    }

    // Java: public void closeDocument() 
    pub fn closeDocument(&mut self) {
      // Java: if (this.isOpen)
      if self.isOpen {
         // Java: this.writer.close();
         self.writer.close();
         // Java: this.isOpen = false;
         self.isOpen = false;
      }
    }

    // Java: public void destroy() 
    pub fn destroy(&mut self) {
      // Java: this.writer = null;
      self.writer = ;
    }

    // Java: public boolean addPage(Bitmap bitmap, int pageWidth, int pageHeight, boolean portrait) 
    pub fn addPage(&mut self, bitmap: Bitmap, pageWidth: i32, pageHeight: i32, portrait: bool) -> bool {
      // Java: return !this.isOpen ? false : this.writer.addPageObj(bitmap, pageWidth, pageHeight, portrait);
      return (if !self.isOpen { false } else { self.writer.addPageObj(bitmap, pageWidth, pageHeight, portrait) });
    }

}

// Java: public class PrintPDF implements IPrintExport 
impl PrintPDF {
    // Java: public PrintPDF() 
    pub fn new() -> PrintPDF {
      Self { writer: Default::default(), isOpen: Default::default() }
    }

}
