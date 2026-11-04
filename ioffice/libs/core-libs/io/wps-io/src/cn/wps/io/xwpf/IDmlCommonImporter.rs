// Java: import org.xml.sax.Attributes

// Java: public interface IDmlCommonImporter extends IImporterBase
pub trait IDmlCommonImporter: crate::io::xwpf::IImporterBase::IImporterBase {
    // Java: abstract void startGlow(int var1, Attributes var2)
    fn startGlow(&mut self, var1: i32, var2: &dyn std::any::Any);

    // Java: abstract void endGlow(int var1)
    fn endGlow(&mut self, var1: i32);

    // Java: abstract void startReflection(Attributes var1)
    fn startReflection(&mut self, var1: &dyn std::any::Any);

    // Java: abstract void startGradFill(int var1, int var2, Attributes var3)
    fn startGradFill(&mut self, var1: i32, var2: i32, var3: &dyn std::any::Any);

    // Java: abstract void endGradFill(int var1)
    fn endGradFill(&mut self, var1: i32);

    // Java: abstract void startGrpFill(int var1, int var2, Attributes var3)
    fn startGrpFill(&mut self, var1: i32, var2: i32, var3: &dyn std::any::Any);

    // Java: abstract void endGrpFill(int var1)
    fn endGrpFill(&mut self, var1: i32);

    // Java: abstract void startSolidFill(int var1, int var2, Attributes var3)
    fn startSolidFill(&mut self, var1: i32, var2: i32, var3: &dyn std::any::Any);

    // Java: abstract void endSolidFill(int var1, int var2)
    fn endSolidFill(&mut self, var1: i32, var2: i32);

    // Java: abstract void startGs(int var1, int var2, Attributes var3)
    fn startGs(&mut self, var1: i32, var2: i32, var3: &dyn std::any::Any);

    // Java: abstract void endGs(int var1, int var2)
    fn endGs(&mut self, var1: i32, var2: i32);

    // Java: abstract void startLin(int var1, int var2, Attributes var3)
    fn startLin(&mut self, var1: i32, var2: i32, var3: &dyn std::any::Any);

    // Java: abstract void endLin(int var1)
    fn endLin(&mut self, var1: i32);

    // Java: abstract void startPath(int var1, int var2, Attributes var3)
    fn startPath(&mut self, var1: i32, var2: i32, var3: &dyn std::any::Any);

    // Java: abstract void endPath(int var1)
    fn endPath(&mut self, var1: i32);

    // Java: abstract void startTileRect(int var1, int var2, Attributes var3)
    fn startTileRect(&mut self, var1: i32, var2: i32, var3: &dyn std::any::Any);

    // Java: abstract void endTileRect(int var1)
    fn endTileRect(&mut self, var1: i32);

    // Java: abstract void startLn(int var1, Attributes var2)
    fn startLn(&mut self, var1: i32, var2: &dyn std::any::Any);

    // Java: abstract void endLn(int var1)
    fn endLn(&mut self, var1: i32);

    // Java: abstract void startNoFill(int var1)
    fn startNoFill(&mut self, var1: i32);

}
