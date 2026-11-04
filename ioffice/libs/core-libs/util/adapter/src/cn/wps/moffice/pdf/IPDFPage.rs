// Java: public interface IPDFPage
pub trait IPDFPage {
    // Java: abstract boolean addImage(Object var1, Object var2)
    fn addImage(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any) -> bool;

    // Java: abstract boolean addJpegImage(Object var1, Object var2)
    fn addJpegImage(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any) -> bool;

    // Java: abstract void addNoteStyle(int var1)
    fn addNoteStyle(&self, _var1: i32);

    // Java: abstract void setBackground(int var1)
    fn setBackground(&self, _var1: i32);

    // Java: abstract void parsePage(boolean var1)
    fn parsePage(&self, _var1: bool);

}
