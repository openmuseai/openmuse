// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::OnceLock;

// Java field: HashMap<String, CssStyleEnum> mMap
fn get_map() -> &'static HashMap<String, CssStyleEnum> {
    static MAP: OnceLock<HashMap<String, CssStyleEnum>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut m = HashMap::new();
        for &v in ALL_VARIANTS {
            m.insert(v.name().to_string(), v);
        }
        m
    })
}

// Java: public enum CssStyleEnum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CssStyleEnum {
    // Java field: String mName
    Unknown, FLIP, HEIGHT, LEFT, MARGIN_BOTTOM, MARGIN_LEFT, MARGIN_RIGHT, MARGIN_TOP,
    MSO_POSITION_HORIZONTAL, MSO_POSITION_HORIZONTAL_RELATIVE,
    MSO_POSITION_VERTICAL, MSO_POSITION_VERTICAL_RELATIVE,
    MSO_WRAP_DISTANCE_BOTTOM, MSO_WRAP_DISTANCE_LEFT, MSO_WRAP_DISTANCE_RIGHT, MSO_WRAP_DISTANCE_TOP,
    MSO_WRAP_EDITED, MSO_WRAP_STYLE, POSITION, ROTATION, TOP, VISIBILITY, WIDTH, Z_INDEX,
    DIRECTION, LAYOUT_FLOW, MSO_DIRECTION_ALT, MSO_FIT_SHAPE_TO_TEXT, MSO_FIT_TEXT_TO_SHAPE,
    MSO_LAYOUT_FLOW_ALT, MSO_NEXT_TEXTBOX, MSO_ROTATE, MSO_TXT_SCALE, V_TEXT_ANCHOR,
    FONT, FONT_FAMILY, FONT_SIZE, FONT_STYLE, FONT_VARIANT, FONT_WEIGHT,
    MSO_TEXT_SHADOW, TEXT_DECORATION, V_ROTATE_LETTERS, V_SAME_LETTER_HEIGHTS,
    V_TEXT_ALIGN, V_TEXT_KERN, V_TEXT_REVERSE, V_TEXT_SPACING_MODE, V_TEXT_SPACING,
    TEXT_ALIGN, MSO_LEFT_PERCENT, MSO_TOP_PERCENT, MSO_WIDTH_PERCENT, MSO_HEIGHT_PERCENT,
    MSO_WIDTH_RELATIVE, MSO_HEIGHT_RELATIVE, MSO_BIDI_FONT_SIZE,
    MSO_ASCII_FONT_FAMILY, MSO_FAREAST_FONT_FAMILY, MSO_HANSI_FONT_FAMILY, MSO_BIDI_FONT_FAMILY,
    TEXT_JUSTIFY, TEXT_KASHIDA, MSO_MARGIN_TOP_ALT, MSO_MARGIN_BOTTOM_ALT,
    MSO_PARA_MARGIN_TOP, MSO_PARA_MARGIN_BOTTOM, COLOR, TEXT_INDENT, MSO_CHAR_INDENT_COUNT,
    MSO_PARA_MARGIN_RIGHT, MSO_PARA_MARGIN_LEFT, LINE_HEIGHT, MSO_LINE_HEIGHT_RULE,
    TAB_INTERVAL, MSO_STYLE_NAME, MSO_STYLE_PARENT, MSO_HIDE, DISPLAY,
    BACKGROUND, MSO_PATTERN, BACKGROUND_COLOR, TEXT_TRANSFORM, LETTER_SPACING,
    MSO_FONT_KERNING, LAYOUT_GRID_MODE, MSO_NO_PROOF,
    MSO_ANSI_LANGUAGE, MSO_FAREAST_LANGUAGE, MSO_BIDI_LANGUAGE,
    VERTICAL_ALIGN, MSO_FONT_WIDTH, TEXT_EFFECT, TEXT_SHADOW, TEXT_ANIMATION,
    FONT_EMPHASIZE, MSO_TEXT_RAISE, TEXT_UNDERLINE, TEXT_FIT,
    MSO_HEADER_MARGIN, MSO_FOOTER_MARGIN, SIZE, LAYOUT_GRID, MARGIN,
    MSO_PAGE_ORIENTATION, MSO_GUTTER_MARGIN, MSO_VERTICAL_PAGE_ALIGN,
    MSO_LINE_NUMBERS_START, MSO_LINE_NUMBERS_COUNT_BY, MSO_LINE_NUMBERS_DISTANCE,
    MSO_LINE_NUMBERS_RESTART, MSO_PADDING_ALT, PADDING, MSO_OUTLINE_LEVEL,
    MSO_LAYOUT_GRID_ALIGN, MSO_MIRROR_INDENTS, TAB_STOPS, MSO_PAGINATION, MSO_TAB_COUNT,
    MSO_HEIGHT_RULE, TEXT_LINE_THROUGH, MSO_GUTTER_POSITION, MSO_FACING_PAGES, MSO_TITLE_PAGE,
    MSO_BORDER_COLOR_ALT, MSO_BORDER_WIDTH_ALT, MSO_BORDER_TOP_ALT, MSO_BORDER_LEFT_ALT,
    MSO_BORDER_BOTTOM_ALT, MSO_BORDER_RIGHT_ALT, BORDER_COLOR, BORDER_WIDTH,
    BORDER_TOP, BORDER_LEFT, BORDER_RIGHT, BORDER_BOTTOM,
    MSO_BORDER_ALT, MSO_BORDER_SHADOW, MSO_MIRROR_MARGINS, BORDER,
    PAGE_BREAK_BEFORE, PAGE_BREAK_AFTER, MSO_BREAK_TYPE,
    MSO_LINE_BREAK_OVERRIDE, PUNCTUATION_WRAP, WORD_BREAK, PUNCTUATION_TRIM, TEXT_AUTOSPACE,
    PAGE_BREAK_INSIDE, MSO_TABLE_LSPACE, MSO_TABLE_RSPACE, MSO_TABLE_TSPACE, MSO_TABLE_BSPACE,
    MSO_ROW_MARGIN_LEFT, MSO_ROW_MARGIN_RIGHT,
    MSO_TABLE_ANCHOR_HORIZONTAL, MSO_TABLE_ANCHOR_VERTICAL, MSO_TABLE_LEFT, MSO_TABLE_TOP,
    MSO_TABLE_OVERLAP, MSO_COMMENT_REFERENCE, MSO_COMMENT_CONTINUATION,
    MSO_COMMENT_DATE, MSO_COMMENT_AUTHOR, MSO_BOOKMARK, MSO_ELEMENT,
    MSO_TSTYLE_SHADING, MSO_TABLE_CONDITION, MSO_SPECIAL_CHARACTER, MSO_STYLE_QFORMAT,
    MSO_TABLE_LAYOUT_ALT, MSO_SYMBOL_FONT_FAMILY, MSO_CHAR_TYPE, MSO_CELL_SPECIAL,
    CLEAR, MSO_DIAGONAL_DOWN, MSO_DIAGONAL_UP, MSO_PROTECTION, WHITE_SPACE,
    MsoYftiTblLook, MsoTextControl, MsoNumberFormat, MsoShading,
}

// Java: public class NAME
pub struct NAME {
}

const ALL_VARIANTS: &[CssStyleEnum] = &[
    CssStyleEnum::Unknown, CssStyleEnum::FLIP, CssStyleEnum::HEIGHT, CssStyleEnum::LEFT,
    CssStyleEnum::MARGIN_BOTTOM, CssStyleEnum::MARGIN_LEFT, CssStyleEnum::MARGIN_RIGHT,
    CssStyleEnum::MARGIN_TOP, CssStyleEnum::MSO_POSITION_HORIZONTAL,
    CssStyleEnum::MSO_POSITION_HORIZONTAL_RELATIVE, CssStyleEnum::MSO_POSITION_VERTICAL,
    CssStyleEnum::MSO_POSITION_VERTICAL_RELATIVE, CssStyleEnum::MSO_WRAP_DISTANCE_BOTTOM,
    CssStyleEnum::MSO_WRAP_DISTANCE_LEFT, CssStyleEnum::MSO_WRAP_DISTANCE_RIGHT,
    CssStyleEnum::MSO_WRAP_DISTANCE_TOP, CssStyleEnum::MSO_WRAP_EDITED,
    CssStyleEnum::MSO_WRAP_STYLE, CssStyleEnum::POSITION, CssStyleEnum::ROTATION,
    CssStyleEnum::TOP, CssStyleEnum::VISIBILITY, CssStyleEnum::WIDTH, CssStyleEnum::Z_INDEX,
    CssStyleEnum::DIRECTION, CssStyleEnum::LAYOUT_FLOW, CssStyleEnum::MSO_DIRECTION_ALT,
    CssStyleEnum::MSO_FIT_SHAPE_TO_TEXT, CssStyleEnum::MSO_FIT_TEXT_TO_SHAPE,
    CssStyleEnum::MSO_LAYOUT_FLOW_ALT, CssStyleEnum::MSO_NEXT_TEXTBOX, CssStyleEnum::MSO_ROTATE,
    CssStyleEnum::MSO_TXT_SCALE, CssStyleEnum::V_TEXT_ANCHOR, CssStyleEnum::FONT,
    CssStyleEnum::FONT_FAMILY, CssStyleEnum::FONT_SIZE, CssStyleEnum::FONT_STYLE,
    CssStyleEnum::FONT_VARIANT, CssStyleEnum::FONT_WEIGHT, CssStyleEnum::MSO_TEXT_SHADOW,
    CssStyleEnum::TEXT_DECORATION, CssStyleEnum::V_ROTATE_LETTERS, CssStyleEnum::V_SAME_LETTER_HEIGHTS,
    CssStyleEnum::V_TEXT_ALIGN, CssStyleEnum::V_TEXT_KERN, CssStyleEnum::V_TEXT_REVERSE,
    CssStyleEnum::V_TEXT_SPACING_MODE, CssStyleEnum::V_TEXT_SPACING, CssStyleEnum::TEXT_ALIGN,
    CssStyleEnum::MSO_LEFT_PERCENT, CssStyleEnum::MSO_TOP_PERCENT, CssStyleEnum::MSO_WIDTH_PERCENT,
    CssStyleEnum::MSO_HEIGHT_PERCENT, CssStyleEnum::MSO_WIDTH_RELATIVE, CssStyleEnum::MSO_HEIGHT_RELATIVE,
    CssStyleEnum::MSO_BIDI_FONT_SIZE, CssStyleEnum::MSO_ASCII_FONT_FAMILY,
    CssStyleEnum::MSO_FAREAST_FONT_FAMILY, CssStyleEnum::MSO_HANSI_FONT_FAMILY,
    CssStyleEnum::MSO_BIDI_FONT_FAMILY, CssStyleEnum::TEXT_JUSTIFY, CssStyleEnum::TEXT_KASHIDA,
    CssStyleEnum::MSO_MARGIN_TOP_ALT, CssStyleEnum::MSO_MARGIN_BOTTOM_ALT,
    CssStyleEnum::MSO_PARA_MARGIN_TOP, CssStyleEnum::MSO_PARA_MARGIN_BOTTOM,
    CssStyleEnum::COLOR, CssStyleEnum::TEXT_INDENT, CssStyleEnum::MSO_CHAR_INDENT_COUNT,
    CssStyleEnum::MSO_PARA_MARGIN_RIGHT, CssStyleEnum::MSO_PARA_MARGIN_LEFT,
    CssStyleEnum::LINE_HEIGHT, CssStyleEnum::MSO_LINE_HEIGHT_RULE, CssStyleEnum::TAB_INTERVAL,
    CssStyleEnum::MSO_STYLE_NAME, CssStyleEnum::MSO_STYLE_PARENT, CssStyleEnum::MSO_HIDE,
    CssStyleEnum::DISPLAY, CssStyleEnum::BACKGROUND, CssStyleEnum::MSO_PATTERN,
    CssStyleEnum::BACKGROUND_COLOR, CssStyleEnum::TEXT_TRANSFORM, CssStyleEnum::LETTER_SPACING,
    CssStyleEnum::MSO_FONT_KERNING, CssStyleEnum::LAYOUT_GRID_MODE, CssStyleEnum::MSO_NO_PROOF,
    CssStyleEnum::MSO_ANSI_LANGUAGE, CssStyleEnum::MSO_FAREAST_LANGUAGE, CssStyleEnum::MSO_BIDI_LANGUAGE,
    CssStyleEnum::VERTICAL_ALIGN, CssStyleEnum::MSO_FONT_WIDTH, CssStyleEnum::TEXT_EFFECT,
    CssStyleEnum::TEXT_SHADOW, CssStyleEnum::TEXT_ANIMATION, CssStyleEnum::FONT_EMPHASIZE,
    CssStyleEnum::MSO_TEXT_RAISE, CssStyleEnum::TEXT_UNDERLINE, CssStyleEnum::TEXT_FIT,
    CssStyleEnum::MSO_HEADER_MARGIN, CssStyleEnum::MSO_FOOTER_MARGIN, CssStyleEnum::SIZE,
    CssStyleEnum::LAYOUT_GRID, CssStyleEnum::MARGIN, CssStyleEnum::MSO_PAGE_ORIENTATION,
    CssStyleEnum::MSO_GUTTER_MARGIN, CssStyleEnum::MSO_VERTICAL_PAGE_ALIGN,
    CssStyleEnum::MSO_LINE_NUMBERS_START, CssStyleEnum::MSO_LINE_NUMBERS_COUNT_BY,
    CssStyleEnum::MSO_LINE_NUMBERS_DISTANCE, CssStyleEnum::MSO_LINE_NUMBERS_RESTART,
    CssStyleEnum::MSO_PADDING_ALT, CssStyleEnum::PADDING, CssStyleEnum::MSO_OUTLINE_LEVEL,
    CssStyleEnum::MSO_LAYOUT_GRID_ALIGN, CssStyleEnum::MSO_MIRROR_INDENTS, CssStyleEnum::TAB_STOPS,
    CssStyleEnum::MSO_PAGINATION, CssStyleEnum::MSO_TAB_COUNT, CssStyleEnum::MSO_HEIGHT_RULE,
    CssStyleEnum::TEXT_LINE_THROUGH, CssStyleEnum::MSO_GUTTER_POSITION,
    CssStyleEnum::MSO_FACING_PAGES, CssStyleEnum::MSO_TITLE_PAGE,
    CssStyleEnum::MSO_BORDER_COLOR_ALT, CssStyleEnum::MSO_BORDER_WIDTH_ALT,
    CssStyleEnum::MSO_BORDER_TOP_ALT, CssStyleEnum::MSO_BORDER_LEFT_ALT,
    CssStyleEnum::MSO_BORDER_BOTTOM_ALT, CssStyleEnum::MSO_BORDER_RIGHT_ALT,
    CssStyleEnum::BORDER_COLOR, CssStyleEnum::BORDER_WIDTH, CssStyleEnum::BORDER_TOP,
    CssStyleEnum::BORDER_LEFT, CssStyleEnum::BORDER_RIGHT, CssStyleEnum::BORDER_BOTTOM,
    CssStyleEnum::MSO_BORDER_ALT, CssStyleEnum::MSO_BORDER_SHADOW, CssStyleEnum::MSO_MIRROR_MARGINS,
    CssStyleEnum::BORDER, CssStyleEnum::PAGE_BREAK_BEFORE, CssStyleEnum::PAGE_BREAK_AFTER,
    CssStyleEnum::MSO_BREAK_TYPE, CssStyleEnum::MSO_LINE_BREAK_OVERRIDE,
    CssStyleEnum::PUNCTUATION_WRAP, CssStyleEnum::WORD_BREAK, CssStyleEnum::PUNCTUATION_TRIM,
    CssStyleEnum::TEXT_AUTOSPACE, CssStyleEnum::PAGE_BREAK_INSIDE,
    CssStyleEnum::MSO_TABLE_LSPACE, CssStyleEnum::MSO_TABLE_RSPACE,
    CssStyleEnum::MSO_TABLE_TSPACE, CssStyleEnum::MSO_TABLE_BSPACE,
    CssStyleEnum::MSO_ROW_MARGIN_LEFT, CssStyleEnum::MSO_ROW_MARGIN_RIGHT,
    CssStyleEnum::MSO_TABLE_ANCHOR_HORIZONTAL, CssStyleEnum::MSO_TABLE_ANCHOR_VERTICAL,
    CssStyleEnum::MSO_TABLE_LEFT, CssStyleEnum::MSO_TABLE_TOP, CssStyleEnum::MSO_TABLE_OVERLAP,
    CssStyleEnum::MSO_COMMENT_REFERENCE, CssStyleEnum::MSO_COMMENT_CONTINUATION,
    CssStyleEnum::MSO_COMMENT_DATE, CssStyleEnum::MSO_COMMENT_AUTHOR, CssStyleEnum::MSO_BOOKMARK,
    CssStyleEnum::MSO_ELEMENT, CssStyleEnum::MSO_TSTYLE_SHADING, CssStyleEnum::MSO_TABLE_CONDITION,
    CssStyleEnum::MSO_SPECIAL_CHARACTER, CssStyleEnum::MSO_STYLE_QFORMAT,
    CssStyleEnum::MSO_TABLE_LAYOUT_ALT, CssStyleEnum::MSO_SYMBOL_FONT_FAMILY,
    CssStyleEnum::MSO_CHAR_TYPE, CssStyleEnum::MSO_CELL_SPECIAL, CssStyleEnum::CLEAR,
    CssStyleEnum::MSO_DIAGONAL_DOWN, CssStyleEnum::MSO_DIAGONAL_UP, CssStyleEnum::MSO_PROTECTION,
    CssStyleEnum::WHITE_SPACE, CssStyleEnum::MsoYftiTblLook, CssStyleEnum::MsoTextControl,
    CssStyleEnum::MsoNumberFormat, CssStyleEnum::MsoShading,
];

// Java: public enum CssStyleEnum
impl CssStyleEnum {
    // Java: CssStyleEnum(String name)
    pub fn name(&self) -> &'static str {
      // Java: Assert.assertNotNull("name should not be null!", name);
      // Java: Assert.assertNotNull("NAME.mMap should not be null!", CssStyleEnum.NAME.mMap);
      // Java: this.mName = name;
      // Java: CssStyleEnum.NAME.mMap.put(this.mName, this);
      match self {
        Self::Unknown => "unknown", Self::FLIP => "flip", Self::HEIGHT => "height",
        Self::LEFT => "left", Self::MARGIN_BOTTOM => "margin-bottom",
        Self::MARGIN_LEFT => "margin-left", Self::MARGIN_RIGHT => "margin-right",
        Self::MARGIN_TOP => "margin-top", Self::MSO_POSITION_HORIZONTAL => "mso-position-horizontal",
        Self::MSO_POSITION_HORIZONTAL_RELATIVE => "mso-position-horizontal-relative",
        Self::MSO_POSITION_VERTICAL => "mso-position-vertical",
        Self::MSO_POSITION_VERTICAL_RELATIVE => "mso-position-vertical-relative",
        Self::MSO_WRAP_DISTANCE_BOTTOM => "mso-wrap-distance-bottom",
        Self::MSO_WRAP_DISTANCE_LEFT => "mso-wrap-distance-left",
        Self::MSO_WRAP_DISTANCE_RIGHT => "mso-wrap-distance-right",
        Self::MSO_WRAP_DISTANCE_TOP => "mso-wrap-distance-top",
        Self::MSO_WRAP_EDITED => "mso-wrap-edited", Self::MSO_WRAP_STYLE => "mso-wrap-style",
        Self::POSITION => "position", Self::ROTATION => "rotation", Self::TOP => "top",
        Self::VISIBILITY => "visibility", Self::WIDTH => "width", Self::Z_INDEX => "z-index",
        Self::DIRECTION => "direction", Self::LAYOUT_FLOW => "layout-flow",
        Self::MSO_DIRECTION_ALT => "mso-direction-alt",
        Self::MSO_FIT_SHAPE_TO_TEXT => "mso-fit-shape-to-text",
        Self::MSO_FIT_TEXT_TO_SHAPE => "mso-fit-text-to-shape",
        Self::MSO_LAYOUT_FLOW_ALT => "mso-layout-flow-alt",
        Self::MSO_NEXT_TEXTBOX => "mso-next-textbox", Self::MSO_ROTATE => "mso-rotate",
        Self::MSO_TXT_SCALE => "mso-text-scale", Self::V_TEXT_ANCHOR => "v-text-anchor",
        Self::FONT => "font", Self::FONT_FAMILY => "font-family", Self::FONT_SIZE => "font-size",
        Self::FONT_STYLE => "font-style", Self::FONT_VARIANT => "font-variant",
        Self::FONT_WEIGHT => "font-weight", Self::MSO_TEXT_SHADOW => "mso-text-shadow",
        Self::TEXT_DECORATION => "text-decoration", Self::V_ROTATE_LETTERS => "v-rotate-letters",
        Self::V_SAME_LETTER_HEIGHTS => "v-same-letter-heights",
        Self::V_TEXT_ALIGN => "v-text-align", Self::V_TEXT_KERN => "v-text-kern",
        Self::V_TEXT_REVERSE => "v-text-reverse", Self::V_TEXT_SPACING_MODE => "v-text-spacing-mode",
        Self::V_TEXT_SPACING => "v-text-spacing", Self::TEXT_ALIGN => "text-align",
        Self::MSO_LEFT_PERCENT => "mso-left-percent", Self::MSO_TOP_PERCENT => "mso-top-percent",
        Self::MSO_WIDTH_PERCENT => "mso-width-percent", Self::MSO_HEIGHT_PERCENT => "mso-height-percent",
        Self::MSO_WIDTH_RELATIVE => "mso-width-relative", Self::MSO_HEIGHT_RELATIVE => "mso-height-relative",
        Self::MSO_BIDI_FONT_SIZE => "mso-bidi-font-size",
        Self::MSO_ASCII_FONT_FAMILY => "mso-ascii-font-family",
        Self::MSO_FAREAST_FONT_FAMILY => "mso-fareast-font-family",
        Self::MSO_HANSI_FONT_FAMILY => "mso-hansi-font-family",
        Self::MSO_BIDI_FONT_FAMILY => "mso-bidi-font-family",
        Self::TEXT_JUSTIFY => "text-justify", Self::TEXT_KASHIDA => "text-kashida",
        Self::MSO_MARGIN_TOP_ALT => "mso-margin-top-alt",
        Self::MSO_MARGIN_BOTTOM_ALT => "mso-margin-bottom-alt",
        Self::MSO_PARA_MARGIN_TOP => "mso-para-margin-top",
        Self::MSO_PARA_MARGIN_BOTTOM => "mso-para-margin-bottom",
        Self::COLOR => "color", Self::TEXT_INDENT => "text-indent",
        Self::MSO_CHAR_INDENT_COUNT => "mso-char-indent-count",
        Self::MSO_PARA_MARGIN_RIGHT => "mso-para-margin-right",
        Self::MSO_PARA_MARGIN_LEFT => "mso-para-margin-left",
        Self::LINE_HEIGHT => "line-height", Self::MSO_LINE_HEIGHT_RULE => "mso-line-height-rule",
        Self::TAB_INTERVAL => "tab-interval", Self::MSO_STYLE_NAME => "mso-style-name",
        Self::MSO_STYLE_PARENT => "mso-style-parent", Self::MSO_HIDE => "mso-hide",
        Self::DISPLAY => "display", Self::BACKGROUND => "background",
        Self::MSO_PATTERN => "mso-pattern", Self::BACKGROUND_COLOR => "background-color",
        Self::TEXT_TRANSFORM => "text-transform", Self::LETTER_SPACING => "letter-spacing",
        Self::MSO_FONT_KERNING => "mso-font-kerning", Self::LAYOUT_GRID_MODE => "layout-grid-mode",
        Self::MSO_NO_PROOF => "mso-no-proof", Self::MSO_ANSI_LANGUAGE => "mso-ansi-language",
        Self::MSO_FAREAST_LANGUAGE => "mso-fareast-language",
        Self::MSO_BIDI_LANGUAGE => "mso-bidi-language", Self::VERTICAL_ALIGN => "vertical-align",
        Self::MSO_FONT_WIDTH => "mso-font-width", Self::TEXT_EFFECT => "text-effect",
        Self::TEXT_SHADOW => "text-shadow", Self::TEXT_ANIMATION => "text-animation",
        Self::FONT_EMPHASIZE => "font-emphasize", Self::MSO_TEXT_RAISE => "mso-text-raise",
        Self::TEXT_UNDERLINE => "text-underline", Self::TEXT_FIT => "text-fit",
        Self::MSO_HEADER_MARGIN => "mso-header-margin", Self::MSO_FOOTER_MARGIN => "mso-footer-margin",
        Self::SIZE => "size", Self::LAYOUT_GRID => "layout-grid", Self::MARGIN => "margin",
        Self::MSO_PAGE_ORIENTATION => "mso-page-orientation",
        Self::MSO_GUTTER_MARGIN => "mso-gutter-margin",
        Self::MSO_VERTICAL_PAGE_ALIGN => "mso-vertical-page-align",
        Self::MSO_LINE_NUMBERS_START => "mso-line-numbers-start",
        Self::MSO_LINE_NUMBERS_COUNT_BY => "mso-line-numbers-count-by",
        Self::MSO_LINE_NUMBERS_DISTANCE => "mso-line-numbers-distance",
        Self::MSO_LINE_NUMBERS_RESTART => "mso-line-numbers-restart",
        Self::MSO_PADDING_ALT => "mso-padding-alt", Self::PADDING => "padding",
        Self::MSO_OUTLINE_LEVEL => "mso-outline-level",
        Self::MSO_LAYOUT_GRID_ALIGN => "mso-layout-grid-align",
        Self::MSO_MIRROR_INDENTS => "mso-mirror-indents", Self::TAB_STOPS => "tab-stops",
        Self::MSO_PAGINATION => "mso-pagination", Self::MSO_TAB_COUNT => "mso-tab-count",
        Self::MSO_HEIGHT_RULE => "mso-height-rule", Self::TEXT_LINE_THROUGH => "text-line-through",
        Self::MSO_GUTTER_POSITION => "mso-gutter-position",
        Self::MSO_FACING_PAGES => "mso-facing-pages", Self::MSO_TITLE_PAGE => "mso-title-page",
        Self::MSO_BORDER_COLOR_ALT => "mso-border-color-alt",
        Self::MSO_BORDER_WIDTH_ALT => "mso-border-width-alt",
        Self::MSO_BORDER_TOP_ALT => "mso-border-top-alt",
        Self::MSO_BORDER_LEFT_ALT => "mso-border-left-alt",
        Self::MSO_BORDER_BOTTOM_ALT => "mso-border-bottom-alt",
        Self::MSO_BORDER_RIGHT_ALT => "mso-border-right-alt",
        Self::BORDER_COLOR => "border-color", Self::BORDER_WIDTH => "border-width",
        Self::BORDER_TOP => "border-top", Self::BORDER_LEFT => "border-left",
        Self::BORDER_RIGHT => "border-right", Self::BORDER_BOTTOM => "border-bottom",
        Self::MSO_BORDER_ALT => "mso-border-alt", Self::MSO_BORDER_SHADOW => "mso-border-shadow",
        Self::MSO_MIRROR_MARGINS => "mso-mirror-margins", Self::BORDER => "border",
        Self::PAGE_BREAK_BEFORE => "page-break-before", Self::PAGE_BREAK_AFTER => "page-break-after",
        Self::MSO_BREAK_TYPE => "mso-break-type",
        Self::MSO_LINE_BREAK_OVERRIDE => "mso-line-break-override",
        Self::PUNCTUATION_WRAP => "punctuation-wrap", Self::WORD_BREAK => "word-break",
        Self::PUNCTUATION_TRIM => "punctuation-trim", Self::TEXT_AUTOSPACE => "text-autospace",
        Self::PAGE_BREAK_INSIDE => "page-break-inside",
        Self::MSO_TABLE_LSPACE => "mso-table-lspace", Self::MSO_TABLE_RSPACE => "mso-table-rspace",
        Self::MSO_TABLE_TSPACE => "mso-table-tspace", Self::MSO_TABLE_BSPACE => "mso-table-bspace",
        Self::MSO_ROW_MARGIN_LEFT => "mso-row-margin-left",
        Self::MSO_ROW_MARGIN_RIGHT => "mso-row-margin-right",
        Self::MSO_TABLE_ANCHOR_HORIZONTAL => "mso-table-anchor-horizontal",
        Self::MSO_TABLE_ANCHOR_VERTICAL => "mso-table-anchor-vertical",
        Self::MSO_TABLE_LEFT => "mso-table-left", Self::MSO_TABLE_TOP => "mso-table-top",
        Self::MSO_TABLE_OVERLAP => "mso-table-overlap",
        Self::MSO_COMMENT_REFERENCE => "mso-comment-reference",
        Self::MSO_COMMENT_CONTINUATION => "mso-comment-continuation",
        Self::MSO_COMMENT_DATE => "mso-comment-date",
        Self::MSO_COMMENT_AUTHOR => "mso-comment-author", Self::MSO_BOOKMARK => "mso-bookmark",
        Self::MSO_ELEMENT => "mso-element", Self::MSO_TSTYLE_SHADING => "mso-tstyle-shading",
        Self::MSO_TABLE_CONDITION => "mso-table-condition",
        Self::MSO_SPECIAL_CHARACTER => "mso-special-character",
        Self::MSO_STYLE_QFORMAT => "mso-style-qformat",
        Self::MSO_TABLE_LAYOUT_ALT => "mso-table-layout-alt",
        Self::MSO_SYMBOL_FONT_FAMILY => "mso-symbol-font-family",
        Self::MSO_CHAR_TYPE => "mso-char-type", Self::MSO_CELL_SPECIAL => "mso-cell-special",
        Self::CLEAR => "clear", Self::MSO_DIAGONAL_DOWN => "mso-diagonal-down",
        Self::MSO_DIAGONAL_UP => "mso-diagonal-up", Self::MSO_PROTECTION => "mso-protection",
        Self::WHITE_SPACE => "white-space", Self::MsoYftiTblLook => "mso-yfti-tbllook",
        Self::MsoTextControl => "mso-text-control", Self::MsoNumberFormat => "mso-number-format",
        Self::MsoShading => "mso-shading",
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return this.mName;
      self.name().to_string()
    }

    // Java: static CssStyleEnum getCssStyleEnum(String name)
    pub fn getCssStyleEnum(name: &str) -> CssStyleEnum {
      // Java: Assert.assertNotNull("name should not be null!", name);
      // Java: Assert.assertNotNull("NAME.mMap should not be null!", CssStyleEnum.NAME.mMap);
      // Java: CssStyleEnum ret = (CssStyleEnum)CssStyleEnum.NAME.mMap.get(name.toLowerCase());
      let lower = name.to_lowercase();
      let map = get_map();
      // Java: return null == ret ? Unknown : ret;
      map.get(&lower).copied().unwrap_or(CssStyleEnum::Unknown)
    }

}

// Java: public class NAME
impl NAME {
    // Java: NAME()
    pub fn new() -> Self {
         // Java: super();
         NAME {}
    }

}
