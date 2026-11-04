// Java: public interface IENMLDocumentImporter
pub trait IENMLDocumentImporter {
    // Java: abstract void onStartDocument()
    fn onStartDocument(&mut self);

    // Java: abstract void onEndDocument()
    fn onEndDocument(&mut self);

    // Java: abstract void onStartBlock()
    fn onStartBlock(&mut self);

    // Java: abstract void onEndBlock()
    fn onEndBlock(&mut self);

    // Java: abstract void onCharacters(String var1)
    fn onCharacters(&mut self, var1: &str);

    // Java: abstract void onInsertLineBreak()
    fn onInsertLineBreak(&mut self);

    // Java: abstract void onInsertParaBreak()
    fn onInsertParaBreak(&mut self);

    // Java: abstract void onImportMedia(IENMLMedia var1)
    fn onImportMedia(&mut self, var1: &dyn std::any::Any);

}
