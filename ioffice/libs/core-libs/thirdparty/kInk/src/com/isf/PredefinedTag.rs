// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::OnceLock;

// Java field: int TAG_INK_SPACE_RECT
pub(crate) const TAG_INK_SPACE_RECT: i32 = 0;
// Java field: int TAG_GUID_TABLE
pub(crate) const TAG_GUID_TABLE: i32 = 1;
// Java field: int TAG_DRAW_ATTRS_TABLE
pub(crate) const TAG_DRAW_ATTRS_TABLE: i32 = 2;
// Java field: int TAG_DRAW_ATTRS_BLOCK
pub(crate) const TAG_DRAW_ATTRS_BLOCK: i32 = 3;
// Java field: int TAG_STROKE_DESC_TABLE
pub(crate) const TAG_STROKE_DESC_TABLE: i32 = 4;
// Java field: int TAG_STROKE_DESC_BLOCK
pub(crate) const TAG_STROKE_DESC_BLOCK: i32 = 5;
// Java field: int TAG_BUTTONS
pub(crate) const TAG_BUTTONS: i32 = 6;
// Java field: int TAG_NO_X
pub(crate) const TAG_NO_X: i32 = 7;
// Java field: int TAG_NO_Y
pub(crate) const TAG_NO_Y: i32 = 8;
// Java field: int TAG_DIDX
pub(crate) const TAG_DIDX: i32 = 9;
// Java field: int TAG_STROKE
pub(crate) const TAG_STROKE: i32 = 10;
// Java field: int TAG_STROKE_PROPERTY_LIST
pub(crate) const TAG_STROKE_PROPERTY_LIST: i32 = 11;
// Java field: int TAG_POINT_PROPERTY
pub(crate) const TAG_POINT_PROPERTY: i32 = 12;
// Java field: int TAG_SIDX
pub(crate) const TAG_SIDX: i32 = 13;
// Java field: int TAG_COMPRESSION_HEADER
pub(crate) const TAG_COMPRESSION_HEADER: i32 = 14;
// Java field: int TAG_TRANSFORM_TABLE
pub(crate) const TAG_TRANSFORM_TABLE: i32 = 15;
// Java field: int TAG_TRANSFORM
pub(crate) const TAG_TRANSFORM: i32 = 16;
// Java field: int TAG_TRANSFORM_ISOTROPIC_SCALE
pub(crate) const TAG_TRANSFORM_ISOTROPIC_SCALE: i32 = 17;
// Java field: int TAG_TRANSFORM_ANISOTROPIC_SCALE
pub(crate) const TAG_TRANSFORM_ANISOTROPIC_SCALE: i32 = 18;
// Java field: int TAG_TRANSFORM_ROTATE
pub(crate) const TAG_TRANSFORM_ROTATE: i32 = 19;
// Java field: int TAG_TRANSFORM_TRANSLATE
pub(crate) const TAG_TRANSFORM_TRANSLATE: i32 = 20;
// Java field: int TAG_TRANSFORM_SCALE_AND_TRANSLATE
pub(crate) const TAG_TRANSFORM_SCALE_AND_TRANSLATE: i32 = 21;
// Java field: int TAG_TRANSFORM_QUAD
pub(crate) const TAG_TRANSFORM_QUAD: i32 = 22;
// Java field: int TAG_TIDX
pub(crate) const TAG_TIDX: i32 = 23;
// Java field: int TAG_METRIC_TABLE
pub(crate) const TAG_METRIC_TABLE: i32 = 24;
// Java field: int TAG_METRIC_BLOCK
pub(crate) const TAG_METRIC_BLOCK: i32 = 25;
// Java field: int TAG_MIDX
pub(crate) const TAG_MIDX: i32 = 26;
// Java field: int TAG_MANTISSA
pub(crate) const TAG_MANTISSA: i32 = 27;
// Java field: int TAG_PERSISTENT_FORMAT
pub(crate) const TAG_PERSISTENT_FORMAT: i32 = 28;
// Java field: int TAG_HIMETRIC_SIZE
pub(crate) const TAG_HIMETRIC_SIZE: i32 = 29;
// Java field: int TAG_STROKE_IDS
pub(crate) const TAG_STROKE_IDS: i32 = 30;
// Java field: int TAG_MICROSOFT_0x034
pub(crate) const TAG_MICROSOFT_0x034: i32 = 52;
// Java field: int DEFAULT_TAGS_NUMBER
pub(crate) const DEFAULT_TAGS_NUMBER: i32 = 100;
// Java field: int FIRST_CUSTOM_TAG_ID
pub(crate) const FIRST_CUSTOM_TAG_ID: i32 = 100;
// Java field: int CUSTOM_USER_ID
pub(crate) const CUSTOM_USER_ID: i32 = 104;
// Java field: String NAME_TAG_INK_SPACE_RECT
pub(crate) const NAME_TAG_INK_SPACE_RECT: &str = "TAG_INK_SPACE_RECT";
// Java field: String NAME_TAG_GUID_TABLE
pub(crate) const NAME_TAG_GUID_TABLE: &str = "TAG_GUID_TABLE";
// Java field: String NAME_TAG_DRAW_ATTRS_TABLE
pub(crate) const NAME_TAG_DRAW_ATTRS_TABLE: &str = "TAG_DRAW_ATTRS_TABLE";
// Java field: String NAME_TAG_DRAW_ATTRS_BLOCK
pub(crate) const NAME_TAG_DRAW_ATTRS_BLOCK: &str = "TAG_DRAW_ATTRS_BLOCK";
// Java field: String NAME_TAG_STROKE_DESC_TABLE
pub(crate) const NAME_TAG_STROKE_DESC_TABLE: &str = "TAG_STROKE_DESC_TABLE";
// Java field: String NAME_TAG_STROKE_DESC_BLOCK
pub(crate) const NAME_TAG_STROKE_DESC_BLOCK: &str = "TAG_STROKE_DESC_BLOCK";
// Java field: String NAME_TAG_BUTTONS
pub(crate) const NAME_TAG_BUTTONS: &str = "TAG_BUTTONS";
// Java field: String NAME_TAG_NO_X
pub(crate) const NAME_TAG_NO_X: &str = "TAG_NO_X";
// Java field: String NAME_TAG_NO_Y
pub(crate) const NAME_TAG_NO_Y: &str = "TAG_NO_Y";
// Java field: String NAME_TAG_DIDX
pub(crate) const NAME_TAG_DIDX: &str = "TAG_DIDX";
// Java field: String NAME_TAG_STROKE
pub(crate) const NAME_TAG_STROKE: &str = "TAG_STROKE";
// Java field: String NAME_TAG_STROKE_PROPERTY_LIST
pub(crate) const NAME_TAG_STROKE_PROPERTY_LIST: &str = "TAG_STROKE_PROPERTY_LIST";
// Java field: String NAME_TAG_POINT_PROPERTY
pub(crate) const NAME_TAG_POINT_PROPERTY: &str = "TAG_POINT_PROPERTY";
// Java field: String NAME_TAG_SIDX
pub(crate) const NAME_TAG_SIDX: &str = "TAG_SIDX";
// Java field: String NAME_TAG_COMPRESSION_HEADER
pub(crate) const NAME_TAG_COMPRESSION_HEADER: &str = "TAG_COMPRESSION_HEADER";
// Java field: String NAME_TAG_TRANSFORM_TABLE
pub(crate) const NAME_TAG_TRANSFORM_TABLE: &str = "TAG_TRANSFORM_TABLE";
// Java field: String NAME_TAG_TRANSFORM
pub(crate) const NAME_TAG_TRANSFORM: &str = "TAG_TRANSFORM";
// Java field: String NAME_TAG_TRANSFORM_ISOTROPIC_SCALE
pub(crate) const NAME_TAG_TRANSFORM_ISOTROPIC_SCALE: &str = "TAG_TRANSFORM_ISOTROPIC_SCALE";
// Java field: String NAME_TAG_TRANSFORM_ANISOTROPIC_SCALE
pub(crate) const NAME_TAG_TRANSFORM_ANISOTROPIC_SCALE: &str = "TAG_TRANSFORM_ANISOTROPIC_SCALE";
// Java field: String NAME_TAG_TRANSFORM_ROTATE
pub(crate) const NAME_TAG_TRANSFORM_ROTATE: &str = "TAG_TRANSFORM_ROTATE";
// Java field: String NAME_TAG_TRANSFORM_TRANSLATE
pub(crate) const NAME_TAG_TRANSFORM_TRANSLATE: &str = "TAG_TRANSFORM_TRANSLATE";
// Java field: String NAME_TAG_TRANSFORM_SCALE_AND_TRANSLATE
pub(crate) const NAME_TAG_TRANSFORM_SCALE_AND_TRANSLATE: &str = "TAG_TRANSFORM_SCALE_AND_TRANSLATE";
// Java field: String NAME_TAG_TRANSFORM_QUAD
pub(crate) const NAME_TAG_TRANSFORM_QUAD: &str = "TAG_TRANSFORM_QUAD";
// Java field: String NAME_TAG_TIDX
pub(crate) const NAME_TAG_TIDX: &str = "TAG_TIDX";
// Java field: String NAME_TAG_METRIC_TABLE
pub(crate) const NAME_TAG_METRIC_TABLE: &str = "TAG_METRIC_TABLE";
// Java field: String NAME_TAG_METRIC_BLOCK
pub(crate) const NAME_TAG_METRIC_BLOCK: &str = "TAG_METRIC_BLOCK";
// Java field: String NAME_TAG_MIDX
pub(crate) const NAME_TAG_MIDX: &str = "TAG_MIDX";
// Java field: String NAME_TAG_MANTISSA
pub(crate) const NAME_TAG_MANTISSA: &str = "TAG_MANTISSA";
// Java field: String NAME_TAG_PERSISTENT_FORMAT
pub(crate) const NAME_TAG_PERSISTENT_FORMAT: &str = "TAG_PERSISTENT_FORMAT";
// Java field: String NAME_TAG_HIMETRIC_SIZE
pub(crate) const NAME_TAG_HIMETRIC_SIZE: &str = "TAG_HIMETRIC_SIZE";
// Java field: String NAME_TAG_STROKE_IDS
pub(crate) const NAME_TAG_STROKE_IDS: &str = "TAG_STROKE_IDS";
// Java field: String NAME_DEFAULT_TAGS_NUMBER
pub(crate) const NAME_DEFAULT_TAGS_NUMBER: &str = "DEFAULT_TAGS_NUMBER";
// Java field: String NAME_FIRST_CUSTOM_TAG_ID
pub(crate) const NAME_FIRST_CUSTOM_TAG_ID: &str = "FIRST_CUSTOM_TAG_ID";

// Java field: HashMap<Integer, String> sInteger2StringMap
fn get_sInteger2StringMap() -> &'static HashMap<i32, &'static str> {
    static MAP: OnceLock<HashMap<i32, &str>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(0, "TAG_INK_SPACE_RECT");
        m.insert(1, "TAG_GUID_TABLE");
        m.insert(2, "TAG_DRAW_ATTRS_TABLE");
        m.insert(3, "TAG_DRAW_ATTRS_BLOCK");
        m.insert(4, "TAG_STROKE_DESC_TABLE");
        m.insert(5, "TAG_STROKE_DESC_BLOCK");
        m.insert(6, "TAG_BUTTONS");
        m.insert(7, "TAG_NO_X");
        m.insert(8, "TAG_NO_Y");
        m.insert(9, "TAG_DIDX");
        m.insert(10, "TAG_STROKE");
        m.insert(11, "TAG_STROKE_PROPERTY_LIST");
        m.insert(12, "TAG_POINT_PROPERTY");
        m.insert(13, "TAG_SIDX");
        m.insert(14, "TAG_COMPRESSION_HEADER");
        m.insert(15, "TAG_TRANSFORM_TABLE");
        m.insert(16, "TAG_TRANSFORM");
        m.insert(17, "TAG_TRANSFORM_ISOTROPIC_SCALE");
        m.insert(18, "TAG_TRANSFORM_ANISOTROPIC_SCALE");
        m.insert(19, "TAG_TRANSFORM_ROTATE");
        m.insert(20, "TAG_TRANSFORM_TRANSLATE");
        m.insert(21, "TAG_TRANSFORM_SCALE_AND_TRANSLATE");
        m.insert(22, "TAG_TRANSFORM_QUAD");
        m.insert(23, "TAG_TIDX");
        m.insert(24, "TAG_METRIC_TABLE");
        m.insert(25, "TAG_METRIC_BLOCK");
        m.insert(26, "TAG_MIDX");
        m.insert(27, "TAG_MANTISSA");
        m.insert(28, "TAG_PERSISTENT_FORMAT");
        m.insert(29, "TAG_HIMETRIC_SIZE");
        m.insert(30, "TAG_STROKE_IDS");
        m.insert(100, "FIRST_CUSTOM_TAG_ID");
        m
    })
}

// Java: public class PredefinedTag
pub struct PredefinedTag;

// Java: public class PredefinedTag
impl PredefinedTag {
    // Java: PredefinedTag()
    pub fn PredefinedTag() -> Self {
        // Java: super();
        PredefinedTag
    }

    // Java: static String toName(int id)
    pub fn toName(id: i32) -> String {
        // Java: return (String)sInteger2StringMap.get(id);
        get_sInteger2StringMap()
            .get(&id)
            .map(|s| s.to_string())
            .unwrap_or_default()
    }
}
