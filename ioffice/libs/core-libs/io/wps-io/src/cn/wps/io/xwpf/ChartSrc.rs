// Java: public interface ChartSrc
pub trait ChartSrc {
    // Java: abstract String getUniqueFileName()
    fn getUniqueFileName(&self) -> String;

    // Java: abstract byte[] getData()
    fn getData(&self) -> Vec<u8>;

}
