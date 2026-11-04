use crate::cn::wps::text::layout::typo::TextAutoNum::TextAutoNum;

// Java field: int TYPE_PLAIN
pub(crate) const TYPE_PLAIN: i32 = 0;
// Java field: int TYPE_PERIOD
pub(crate) const TYPE_PERIOD: i32 = 1;
// Java field: int TYPE_PAREN_BOTH
pub(crate) const TYPE_PAREN_BOTH: i32 = 2;
// Java field: int TYPE_PAREN_R
pub(crate) const TYPE_PAREN_R: i32 = 3;

// Java: public class ArabicPlain extends TextAutoNum
pub struct ArabicPlain {
    // Java field: int mType
    mType: i32,
}

// Java: public class ArabicPlain extends TextAutoNum
impl ArabicPlain {
    // Java: ArabicPlain(int type)
    pub fn new(r#type: i32) -> Self {
        // Java: super();
        // Java: this.mType = type;
        ArabicPlain { mType: r#type }
    }

    // Java: String convert(int page)
    pub fn convert(&self, page: i32) -> String {
        // Java: String text = intToStr(page);
        let mut text = TextAutoNum::intToStr(page);
        // Java: switch (this.mType)
        match self.mType
        {
            // Java: case 1:
            1 => {
                // Java: text = text + ".";
                text = text + ".";
                // Java: break;
            }
            // Java: case 2:
            2 => {
                // Java: text = "(" + text + ")";
                text = format!("({})", text);
                // Java: break;
            }
            // Java: case 3:
            3 => {
                // Java: text = text + ")";
                text = text + ")";
            }
            _ => {}
        }
        // Java: return text;
        text
    }
}
