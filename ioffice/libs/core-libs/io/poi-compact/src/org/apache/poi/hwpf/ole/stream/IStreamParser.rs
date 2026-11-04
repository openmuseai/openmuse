// java public interface IStreamParser
pub trait IStreamParser {
    // java  String getRawFilePath(String var1)
    fn getRawFilePath(&self, var1: &str) -> String;

}
