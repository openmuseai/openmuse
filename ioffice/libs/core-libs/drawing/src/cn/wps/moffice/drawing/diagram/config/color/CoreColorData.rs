#![allow(non_snake_case, dead_code)]

// Java: public enum ColorElementType
pub struct ColorElementType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorElementTypeEnum {
    SRgb,
    ScRgb,
    Hsl,
    Scheme,
    Preset,
    System,
    Palette,
    LegacyRef,
}

impl ColorElementType {
    // Java: ColorElementType()
    pub fn new() -> Self {
        ColorElementType
    }
}

// Java: public interface CoreColorData
pub trait CoreColorData {
    // Java: abstract ColorElementType type()
    fn r#type(&self) -> ColorElementTypeEnum;

}
