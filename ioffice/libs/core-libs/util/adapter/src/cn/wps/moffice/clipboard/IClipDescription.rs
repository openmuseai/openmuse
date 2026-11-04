// Java: public interface IClipDescription
pub trait IClipDescription {
    // Java: abstract String[] filterMimeTypes(String var1)
    fn filterMimeTypes(&self, var1: &str) -> Vec<String>;

}
