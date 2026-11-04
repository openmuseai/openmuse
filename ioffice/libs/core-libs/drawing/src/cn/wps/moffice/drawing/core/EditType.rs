#![allow(non_camel_case_types)]

// Java: public enum EditType
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EditType {
    type_none,
    type_moveing,
    type_rotation,
    type_scaleing,
    type_clip,
    type_adjusting,
}

// Java: public enum EditType
impl EditType {
    // Java: EditType()
}
