// Java: import org.xml.sax.Attributes

// Java: public interface IDmlTextImporter extends IDmlCommonImporter
pub trait IDmlTextImporter {
    // Java: abstract void startShadow(int var1, Attributes var2)
    fn startShadow(&mut self, var1: i32, var2: &dyn std::any::Any);

    // Java: abstract void endShadow(int var1)
    fn endShadow(&mut self, var1: i32);

}
