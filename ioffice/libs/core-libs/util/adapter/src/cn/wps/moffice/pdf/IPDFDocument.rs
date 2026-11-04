// Java: import java.util.concurrent.TimeoutException

// Java: public interface IPDFDocument
pub trait IPDFDocument {
    // Java: abstract void closePDF()
    fn closePDF(&mut self);

    // Java: abstract void closePDFSafetyBlocked()
    fn closePDFSafetyBlocked(&mut self);

    // Java: abstract void closePDFSafetyNonBlocked()
    fn closePDFSafetyNonBlocked(&mut self);

    // Java: abstract IPDFPage newPage(double var1, double var3)
    fn newPage(&mut self, _var1: f64, _var3: f64) -> Box<dyn std::any::Any>;

    // Java: abstract IPDFPage createNewPage(int var1, double var2, double var4)
    fn createNewPage(&mut self, _var1: i32, _var2: f64, _var4: f64) -> Box<dyn std::any::Any>;

    // Java: abstract boolean export(String var1, IFileOp var2)
    fn export(&self, _var1: &str, _var2: &dyn std::any::Any) -> bool;

    // Java: abstract int getPageCount()
    fn getPageCount(&self) -> i32;

    // Java: abstract void recyclePage(IPDFPage var1)
    fn recyclePage(&mut self, _var1: Box<dyn std::any::Any>);

}
