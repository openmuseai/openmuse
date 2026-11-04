// Java: public interface RecipientData
pub trait RecipientData {
    // Java: abstract boolean dumpTempFile(String var1)
    fn dumpTempFile(&self, var1: &str) -> bool;

    // Java: abstract String getUniqueFileName()
    fn getUniqueFileName(&self) -> String;

}
