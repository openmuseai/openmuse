use std::sync::OnceLock;

// Java field: int MAX_PAGE
pub(crate) const MAX_PAGE: i32 = 32767;
// Java field: PlainTextAutoNum DEFAULT_AUTO_NUM
#[allow(non_upper_case_globals)]
static DEFAULT_AUTO_NUM: OnceLock<TextAutoNum> = OnceLock::new();

fn get_default_auto_num() -> &'static TextAutoNum {
    DEFAULT_AUTO_NUM.get_or_init(|| PlainTextAutoNum::new())
}

#[derive(Clone)]
enum AutoNumKind {
    Plain,
    ArabicPlain(i32),
}

// Java: public class TextAutoNum
#[derive(Clone)]
pub struct TextAutoNum {
    kind: AutoNumKind,
}

// Java: public class PlainTextAutoNum
pub struct PlainTextAutoNum;

impl PlainTextAutoNum {
    pub fn new() -> TextAutoNum {
        TextAutoNum { kind: AutoNumKind::Plain }
    }
}

// Java: public class TextAutoNum
impl TextAutoNum {
    // Java: TextAutoNum()
    pub fn new() -> Self {
        // Java: super();
        TextAutoNum { kind: AutoNumKind::Plain }
    }

    pub fn new_arabic_plain(start: i32) -> Self {
        TextAutoNum { kind: AutoNumKind::ArabicPlain(start) }
    }

    // Java: String getNumbering(int page)
    pub fn getNumbering(&self, page: i32) -> String {
        // Java: page = amendNumRange(page);
        let page = TextAutoNum::amendNumRange(page);
        // Java: return this.convert(page);
        self.convert(page)
    }

    // Java: abstract String convert(int var1)
    pub fn convert(&self, var1: i32) -> String {
        match &self.kind {
            AutoNumKind::Plain => TextAutoNum::intToStr(var1),
            AutoNumKind::ArabicPlain(_start) => {
                var1.to_string()
            }
        }
    }

    // Java: static int amendNumRange(int value)
    pub fn amendNumRange(value: i32) -> i32 {
        // Java: return (value - 1) % 32767 + 1;
        (value - 1) % 32767 + 1
    }

    // Java: static String intToStr(int value)
    pub fn intToStr(value: i32) -> String {
        // Java: return Integer.toString(value);
        value.to_string()
    }
}

// Java: public class Factory
pub struct Factory;

// Java: public class Factory
impl Factory {
    // Java: static TextAutoNum create(int textAutoNumScheme)
    pub fn create(textAutoNumScheme: i32) -> TextAutoNum {
        // Java: TextAutoNum autoNum;
        let autoNum;
        // Java: switch (textAutoNumScheme)
        match textAutoNumScheme
        {
            // Java: case 2:
            2 => {
                // Java: autoNum = new ArabicPlain(3);
                autoNum = TextAutoNum::new_arabic_plain(3);
                // Java: break;
            }
            // Java: case 3:
            3 => {
                // Java: autoNum = new ArabicPlain(1);
                autoNum = TextAutoNum::new_arabic_plain(1);
                // Java: break;
            }
            // Java: case 12:
            12 => {
                // Java: autoNum = new ArabicPlain(2);
                autoNum = TextAutoNum::new_arabic_plain(2);
                // Java: break;
            }
            // Java: case 13:
            13 => {
                // Java: autoNum = new ArabicPlain(0);
                autoNum = TextAutoNum::new_arabic_plain(0);
                // Java: break;
            }
            // Java: default:
            _ => {
                // Java: autoNum = DEFAULT_AUTO_NUM;
                autoNum = get_default_auto_num().clone();
            }
        }
        // Java: return autoNum;
        autoNum
    }
}
