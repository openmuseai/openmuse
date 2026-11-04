// Java: public enum SubDocType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubDocType {
    MAIN_DOCUMENT,
    FOOTNOTE_DOCUMENT,
    HEADER_DOCUMENT,
    COMMENT_DOCUMENT,
    ENDNOTE_DOCUMENT,
    TEXTBOX_DOCUMENT,
    HEADERTEXTBOX_DOCUMENT,
}

// Java: SubDocType()
impl SubDocType {
    // Java: compiler-generated SubDocType.values() in declaration order.
    pub fn values() -> [Self; 7] {
        [
            Self::MAIN_DOCUMENT,
            Self::FOOTNOTE_DOCUMENT,
            Self::HEADER_DOCUMENT,
            Self::COMMENT_DOCUMENT,
            Self::ENDNOTE_DOCUMENT,
            Self::TEXTBOX_DOCUMENT,
            Self::HEADERTEXTBOX_DOCUMENT,
        ]
    }
}
