// Java: import java.io.IOException
// Java: import java.io.InputStream
use std::io::Read;

// Java: public interface IAssetManager
pub trait IAssetManager {
    // Java: abstract String[] list(String var1)
    fn list(&self, var1: &str) -> Vec<String>;

    // Java: abstract InputStream open(String var1)
    fn open(&self, var1: &str) -> Option<Box<dyn Read>>;

}
