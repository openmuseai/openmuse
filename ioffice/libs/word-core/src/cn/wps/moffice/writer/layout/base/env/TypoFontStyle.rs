#![allow(non_snake_case)]

/// Layout-min TypoFontStyle: names + size twip + b/i. No ITypeface / ascent (wr-11).
#[derive(Clone, Debug, PartialEq)]
pub struct TypoFontStyle {
    // java public String ascii
    pub ascii: String,
    // java public String eastAsia
    pub eastAsia: String,
    // java public String fontName
    pub fontName: String,
    // java public int fontSize
    pub fontSize: i32,
    // java public boolean bold
    pub bold: bool,
    // java public boolean italic
    pub italic: bool,
    // java public int fontColor
    pub fontColor: i32,
    // java public int underline
    pub underline: i32,
}

impl TypoFontStyle {
    // java public TypoFontStyle()
    pub fn TypoFontStyle() -> Self {
        Self {
            ascii: "Times New Roman".to_string(),
            eastAsia: "宋体".to_string(),
            fontName: "Times New Roman".to_string(),
            fontSize: 200,
            bold: false,
            italic: false,
            fontColor: -16777216,
            underline: 0,
        }
    }
}

impl Default for TypoFontStyle {
    fn default() -> Self {
        Self::TypoFontStyle()
    }
}
