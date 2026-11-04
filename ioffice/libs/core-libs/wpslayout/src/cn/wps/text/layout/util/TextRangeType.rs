// Java: public enum TextRangeType
#[derive(Clone, Copy, PartialEq)]
pub enum TextRangeType {
    Text_None,
    Text_Text,
    Text_Digit,
    Text_Normal,
    Text_ThousandsFloat,
    Text_ThousandsInt,
    Text_Float,
}

impl TextRangeType {
    // Java: TextRangeType()
}
