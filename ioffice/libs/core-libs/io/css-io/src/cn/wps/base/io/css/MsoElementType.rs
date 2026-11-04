use std::collections::HashMap;

// Java field: HashMap<String, MsoElementType> sMap
// Java field: String COMMENT
// Java field: String COMMENT_LIST
// Java field: String DROPCAP_DROPPED
// Java field: String DROPCAP_IN_MARGIN
// Java field: String ENDNOTE
// Java field: String ENDNOTE_CONTINUATION_NOTICE
// Java field: String ENDNOTE_CONTINUATION_SEPARATOR
// Java field: String ENDNOTE_LIST
// Java field: String ENDNOTE_SEPARATOR
// Java field: String FIELD_BEGIN
// Java field: String FIELD_END
// Java field: String FIELD_SEPARATOR
// Java field: String FOOTER
// Java field: String FOOTNOTE
// Java field: String FOOTNOTE_CONTINUATION_NOTICE
// Java field: String FOOTNOTE_CONTINUATION_SEPARATOR
// Java field: String FOOTNOTE_LIST
// Java field: String FOOTNOTE_SEPARATOR
// Java field: String FRAME
// Java field: String HEADER
// Java field: String NONE
// Java field: String PARAGRAPH_MARK_PROPERTIES
// Java field: String TABLE_HEAD

// Java: public enum MsoElementType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MsoElementType {
    COMMENT,
    COMMENT_LIST,
    DROPCAP_DROPPED,
    DROPCAP_IN_MARGIN,
    ENDNOTE,
    ENDNOTE_CONTINUATION_NOTICE,
    ENDNOTE_CONTINUATION_SEPARATOR,
    ENDNOTE_LIST,
    ENDNOTE_SEPARATOR,
    FIELD_BEGIN,
    FIELD_END,
    FIELD_SEPARATOR,
    FOOTER,
    FOOTNOTE,
    FOOTNOTE_CONTINUATION_NOTICE,
    FOOTNOTE_CONTINUATION_SEPARATOR,
    FOOTNOTE_LIST,
    FOOTNOTE_SEPARATOR,
    FRAME,
    HEADER,
    NONE,
    PARAGRAPH_MARK_PROPERTIES,
    TABLE_HEAD,
}

// Java: public enum MsoElementType
impl MsoElementType {
    // Java: MsoElementType(String value)
    pub fn name_str(&self) -> &'static str {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", MsoElementType.NAME.sMap);
        // Java: MsoElementType.NAME.sMap.put(value, this);
        match self {
            Self::COMMENT => "comment",
            Self::COMMENT_LIST => "comment-list",
            Self::DROPCAP_DROPPED => "dropcap-dropped",
            Self::DROPCAP_IN_MARGIN => "dropcap-in-margin",
            Self::ENDNOTE => "endnote",
            Self::ENDNOTE_CONTINUATION_NOTICE => "endnote-continuation-notice",
            Self::ENDNOTE_CONTINUATION_SEPARATOR => "endnote-continuation-separator",
            Self::ENDNOTE_LIST => "endnote-list",
            Self::ENDNOTE_SEPARATOR => "endnote-separator",
            Self::FIELD_BEGIN => "field-begin",
            Self::FIELD_END => "field-end",
            Self::FIELD_SEPARATOR => "field-separator",
            Self::FOOTER => "footer",
            Self::FOOTNOTE => "footnote",
            Self::FOOTNOTE_CONTINUATION_NOTICE => "footnote-continuation-notice",
            Self::FOOTNOTE_CONTINUATION_SEPARATOR => "footnote-continuation-separator",
            Self::FOOTNOTE_LIST => "footnote-list",
            Self::FOOTNOTE_SEPARATOR => "footnote-separator",
            Self::FRAME => "frame",
            Self::HEADER => "header",
            Self::NONE => "none",
            Self::PARAGRAPH_MARK_PROPERTIES => "paragraph-mark-properties",
            Self::TABLE_HEAD => "table-head",
        }
    }

    // Java: static MsoElementType fromValue(String v)
    pub fn from_value(v: &str) -> Option<MsoElementType> {
        // Java: KSAssert.assertNotNull("NAME.sMap should not be null!", MsoElementType.NAME.sMap);
        // Java: return (MsoElementType)MsoElementType.NAME.sMap.get(v);
        ALL_VARIANTS.iter().find(|a| a.name_str() == v).copied()
    }
}

// Java: public class NAME
pub struct NAME;

// Java: public class NAME
impl NAME {
    // Java field: String COMMENT
    pub const COMMENT: &str = "comment";
    // Java field: String COMMENT_LIST
    pub const COMMENT_LIST: &str = "comment-list";
    // Java field: String DROPCAP_DROPPED
    pub const DROPCAP_DROPPED: &str = "dropcap-dropped";
    // Java field: String DROPCAP_IN_MARGIN
    pub const DROPCAP_IN_MARGIN: &str = "dropcap-in-margin";
    // Java field: String ENDNOTE
    pub const ENDNOTE: &str = "endnote";
    // Java field: String ENDNOTE_CONTINUATION_NOTICE
    pub const ENDNOTE_CONTINUATION_NOTICE: &str = "endnote-continuation-notice";
    // Java field: String ENDNOTE_CONTINUATION_SEPARATOR
    pub const ENDNOTE_CONTINUATION_SEPARATOR: &str = "endnote-continuation-separator";
    // Java field: String ENDNOTE_LIST
    pub const ENDNOTE_LIST: &str = "endnote-list";
    // Java field: String ENDNOTE_SEPARATOR
    pub const ENDNOTE_SEPARATOR: &str = "endnote-separator";
    // Java field: String FIELD_BEGIN
    pub const FIELD_BEGIN: &str = "field-begin";
    // Java field: String FIELD_END
    pub const FIELD_END: &str = "field-end";
    // Java field: String FIELD_SEPARATOR
    pub const FIELD_SEPARATOR: &str = "field-separator";
    // Java field: String FOOTER
    pub const FOOTER: &str = "footer";
    // Java field: String FOOTNOTE
    pub const FOOTNOTE: &str = "footnote";
    // Java field: String FOOTNOTE_CONTINUATION_NOTICE
    pub const FOOTNOTE_CONTINUATION_NOTICE: &str = "footnote-continuation-notice";
    // Java field: String FOOTNOTE_CONTINUATION_SEPARATOR
    pub const FOOTNOTE_CONTINUATION_SEPARATOR: &str = "footnote-continuation-separator";
    // Java field: String FOOTNOTE_LIST
    pub const FOOTNOTE_LIST: &str = "footnote-list";
    // Java field: String FOOTNOTE_SEPARATOR
    pub const FOOTNOTE_SEPARATOR: &str = "footnote-separator";
    // Java field: String FRAME
    pub const FRAME: &str = "frame";
    // Java field: String HEADER
    pub const HEADER: &str = "header";
    // Java field: String NONE
    pub const NONE: &str = "none";
    // Java field: String PARAGRAPH_MARK_PROPERTIES
    pub const PARAGRAPH_MARK_PROPERTIES: &str = "paragraph-mark-properties";
    // Java field: String TABLE_HEAD
    pub const TABLE_HEAD: &str = "table-head";

    // Java: NAME()
    pub fn new() -> Self {
        // Java: super();
        NAME
    }
}

const ALL_VARIANTS: &[MsoElementType] = &[MsoElementType::COMMENT, MsoElementType::COMMENT_LIST, MsoElementType::DROPCAP_DROPPED, MsoElementType::DROPCAP_IN_MARGIN, MsoElementType::ENDNOTE, MsoElementType::ENDNOTE_CONTINUATION_NOTICE, MsoElementType::ENDNOTE_CONTINUATION_SEPARATOR, MsoElementType::ENDNOTE_LIST, MsoElementType::ENDNOTE_SEPARATOR, MsoElementType::FIELD_BEGIN, MsoElementType::FIELD_END, MsoElementType::FIELD_SEPARATOR, MsoElementType::FOOTER, MsoElementType::FOOTNOTE, MsoElementType::FOOTNOTE_CONTINUATION_NOTICE, MsoElementType::FOOTNOTE_CONTINUATION_SEPARATOR, MsoElementType::FOOTNOTE_LIST, MsoElementType::FOOTNOTE_SEPARATOR, MsoElementType::FRAME, MsoElementType::HEADER, MsoElementType::NONE, MsoElementType::PARAGRAPH_MARK_PROPERTIES, MsoElementType::TABLE_HEAD];
