// Java: public enum MediaTypeEnum
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MediaTypeEnum {
    PICTURE,
    OLE,
}

// Java: public enum MediaTypeEnum
impl MediaTypeEnum {
    // Java: MediaTypeEnum()
    pub fn new() -> Self {
        MediaTypeEnum::PICTURE
    }
}
