#![allow(non_snake_case)]
#[derive(Default, Debug, Clone)]
pub struct Character;
impl Character {
    pub fn new() -> Self { Self }

    /// Java `Character.toLowerCase(char)`
    pub fn toLowerCase(ch: char) -> char {
        ch.to_lowercase().next().unwrap_or(ch)
    }

    /// Java `Character.toUpperCase(char)`
    pub fn toUpperCase(ch: char) -> char {
        ch.to_uppercase().next().unwrap_or(ch)
    }

    /// Java `Character.isHighSurrogate(char)`
    pub fn isHighSurrogate(ch: char) -> bool {
        let u = ch as u32;
        (0xD800..=0xDBFF).contains(&u)
    }

    /// Java `Character.isLowSurrogate(char)`
    pub fn isLowSurrogate(ch: char) -> bool {
        let u = ch as u32;
        (0xDC00..=0xDFFF).contains(&u)
    }
}
