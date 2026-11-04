// java public enum SelectionType
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum SelectionType {
    // java NONE
    #[default]
    NONE,
    // java IP
    IP,
    // java NORMAL
    NORMAL,
    // java INLINESHAPE
    INLINESHAPE,
    // java SHAPE
    SHAPE,
    // java SCALE
    SCALE,
    // java CLIP
    CLIP,
    // java ADJUST
    ADJUST,
    // java TABLEFRAME
    TABLEFRAME,
    // java TABLEROW
    TABLEROW,
    // java TABLECOLUMN
    TABLECOLUMN,
    // java ROTATION
    ROTATION,
    // java OLE
    OLE,
}

// java public enum SelectionType
impl SelectionType {
    // java public static boolean isShapeSelect(SelectionType type)
    pub fn isShapeSelect(r#type: SelectionType) -> bool {
        // java return type == SHAPE || type == INLINESHAPE || type == SCALE || type == CLIP || type == ROTATION || type == OLE;
        r#type == SelectionType::SHAPE
            || r#type == SelectionType::INLINESHAPE
            || r#type == SelectionType::SCALE
            || r#type == SelectionType::CLIP
            || r#type == SelectionType::ROTATION
            || r#type == SelectionType::OLE
    }

    // java public static boolean isTableSelect(SelectionType type)
    pub fn isTableSelect(r#type: SelectionType) -> bool {
        // java return type == TABLEROW || type == TABLECOLUMN;
        r#type == SelectionType::TABLEROW || r#type == SelectionType::TABLECOLUMN
    }

    // java public static boolean isNormalSelect(SelectionType type)
    pub fn isNormalSelect(r#type: SelectionType) -> bool {
        // java return type == NORMAL;
        r#type == SelectionType::NORMAL
    }

    // java public static boolean isTableFrameSelect(SelectionType type)
    pub fn isTableFrameSelect(r#type: SelectionType) -> bool {
        // java return type == TABLEFRAME;
        r#type == SelectionType::TABLEFRAME
    }
}
