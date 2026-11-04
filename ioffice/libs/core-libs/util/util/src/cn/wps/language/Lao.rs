#![allow(non_snake_case, dead_code, unused_variables)]

// Java: public class Lao
pub struct Lao;

// Java: public class Lao
impl Lao {
    // Java: Lao()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: static boolean isLao(char ch)
    pub fn isLao(ch: char) -> bool {
        // Java: return ch >= 3712 && ch < 3840;
        let c = ch as u32;
        c >= 3712 && c < 3840
    }

    // Java: static boolean isSign(char ch)
    pub fn isSign(ch: char) -> bool {
        // Java: return ch == 3761 || ch >= 3764 && ch < 3773 && ch != 3770 || ch >= 3784 && ch < 3790;
        let c = ch as u32;
        c == 3761 || (c >= 3764 && c < 3773 && c != 3770) || (c >= 3784 && c < 3790)
    }
}
