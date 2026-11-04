#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public interface PictureData
pub trait PictureData {
    // Java: byte[] getData()
    fn getData(&self) -> Vec<u8>;

    // Java: String suggestFileExtension()
    fn suggestFileExtension(&self) -> String;

    // Java: String getUniqueFileName()
    fn getUniqueFileName(&self) -> String;

    // Java: XWPFPictureData.PictureDataSource getDataSource()
    fn getDataSource(&self) -> Option<crate::io::xwpf::usermodel::XWPFPictureData::PictureDataSource<'_>>;
}
