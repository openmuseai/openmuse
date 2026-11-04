#![allow(non_camel_case_types)]

// Java: public enum TileFlipMode
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileFlipMode {
    none,
    x,
    y,
    xy,
}

// Java: public enum TileFlipMode
impl TileFlipMode {
    // Java: TileFlipMode()
    // LLM JavaLine Aggregation: Rust enums have no explicit private constructor; variants are constructed directly
}
