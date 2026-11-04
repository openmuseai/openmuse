// Java: import java.io.InputStream

// Java: public interface IPictureData
pub trait IPictureData {
    // Java: abstract InputStream getInputStream()
    fn getInputStream(&self) -> Option<Box<dyn std::io::Read>>;

    // Java: abstract byte[] getData()
    fn getData(&self) -> Vec<u8>;

    // Java: abstract String suggestFileExtension()
    fn suggestFileExtension(&self) -> String;

    // Java: abstract String getUniqueFileName()
    fn getUniqueFileName(&self) -> String;

    // Java: abstract Integer getWidth()
    fn getWidth(&self) -> Option<i32>;

    // Java: abstract Integer getHeight()
    fn getHeight(&self) -> Option<i32>;

}
