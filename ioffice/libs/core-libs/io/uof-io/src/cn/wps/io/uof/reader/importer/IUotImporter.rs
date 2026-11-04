use super::UotContext::UotContext;

// Java: public interface IUotImporter extends IUofImporter
pub trait IUotImporter {
    // Java: abstract UotContext getContext()
    fn getContext(&self) -> &UotContext;

    // Java: abstract void importParagraphStart()
    fn importParagraphStart(&self);

    // Java: abstract void importParagraphEnd()
    fn importParagraphEnd(&self);

    // Java: abstract void importRunStart()
    fn importRunStart(&self);

    // Java: abstract void importRunEnd()
    fn importRunEnd(&self);

    // Java: abstract void importText(char[] var1, int var2, int var3)
    fn importText(&self, var1: &[char], var2: i32, var3: i32);

}
