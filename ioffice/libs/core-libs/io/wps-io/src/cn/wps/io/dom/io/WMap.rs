use crate::io::simple::StringIntMap::StringIntMap;
use crate::io::simple::XmlName2Id::XmlName2Id;

static OBJ: std::sync::OnceLock<WMap> = std::sync::OnceLock::new();

// ── Java 可赋值静态字段 `WMap.MAIN_NAMESPACING` / `WMap.R_NAMESPACING` 的运行期载体 ──
// 详见 `impl WMap` 内 `MAIN_NAMESPACING()` 上方的实现说明。
// 初值 = Java 字段初值（transition），故未调用 `initTagNamespace` 时行为与 Java 一致。
static MAIN_NAMESPACING_STATE: std::sync::RwLock<&'static str> =
    std::sync::RwLock::new("http://schemas.openxmlformats.org/wordprocessingml/2006/main");
static R_NAMESPACING_STATE: std::sync::RwLock<&'static str> =
    std::sync::RwLock::new("http://schemas.openxmlformats.org/officeDocument/2006/relationships");

/// 读取一个命名空间静态量；**中毒不 panic**（Java 无该概念）⇒ 取回内部值继续。
fn read_namespacing(lock: &'static std::sync::RwLock<&'static str>) -> &'static str {
    match lock.read() {
        Ok(guard) => *guard,
        // 仅在「持锁线程 panic」后发生；Java 语义下仍应可读 ⇒ 用 into_inner 取回
        Err(poisoned) => *poisoned.into_inner(),
    }
}

/// 写入一个命名空间静态量；**中毒不 panic**（同上）。
fn write_namespacing(lock: &'static std::sync::RwLock<&'static str>, value: &'static str) {
    match lock.write() {
        Ok(mut guard) => *guard = value,
        Err(poisoned) => *poisoned.into_inner() = value,
    }
}

// Java: public class WMap implements XmlName2Id
pub struct WMap {
    // Java field: String PREFIX
    // Java field: String MAIN_NAMESPACING
    // Java field: String R_NAMESPACING
    // Java field: String NUM_FMT
    // Java field: String DOCUMENT
    // Java field: String BACKGROUND
    // Java field: String BODY
    // Java field: String PICT
    // Java field: String P
    // Java field: String PPR
    // Java field: String R
    // Java field: String RPR
    // Java field: String RPRCHANGE
    // Java field: String PPRCHANGE
    // Java field: String T
    // Java field: String HYPERLINK
    // Java field: String SECTPR
    // Java field: String BR
    // Java field: String DRAWING
    // Java field: String COMMENTRANGESTART
    // Java field: String COMMENTRANGEEND
    // Java field: String COMMENTREFERENCE
    // Java field: String BOOKMARKSTART
    // Java field: String BOOKMARKEND
    // Java field: String FOOTNOTEREFERENCE
    // Java field: String ENDNOTEREFERENCE
    // Java field: String INS
    // Java field: String DEL
    // Java field: String DELTEXT
    // Java field: String OBJECT
    // Java field: String INSTRTEXT
    // Java field: String DELINSTRTEXT
    // Java field: String NOBREAKHYPHEN
    // Java field: String SOFTHYPHEN
    // Java field: String DAYSHORT
    // Java field: String MONTHSHORT
    // Java field: String YEARSHORT
    // Java field: String DAYLONG
    // Java field: String MONTHLONG
    // Java field: String YEARLONG
    // Java field: String ANNOTATIONREF
    // Java field: String FOOTNOTEREF
    // Java field: String ENDNOTEREF
    // Java field: String SEPARATOR
    // Java field: String CONTINUATIONSEPARATOR
    // Java field: String SYM
    // Java field: String PGNUM
    // Java field: String CR
    // Java field: String TAB
    // Java field: String FLDCHAR
    // Java field: String SDT
    // Java field: String SDTCONTENT
    // Java field: String SMARTTAG
    // Java field: String DIR
    // Java field: String BDO
    // Java field: String FLDSIMPLE
    // Java field: String COMMENT
    // Java field: String COMMENTS
    // Java field: String FOOTNOTES
    // Java field: String FOOTNOTE
    // Java field: String ENDNOTES
    // Java field: String ENDNOTE
    // Java field: String PPR_JC
    // Java field: String PPR_IND
    // Java field: String PPR_PBDR
    // Java field: String PPR_PBDR_TOP
    // Java field: String PPR_PBDR_LEFT
    // Java field: String PPR_PBDR_BOTTOM
    // Java field: String PPR_PBDR_RIGHT
    // Java field: String PPR_PBDR_BETEEN
    // Java field: String PPR_PBDR_BAR
    // Java field: String PPR_PSTYLE
    // Java field: String PPR_KINSOKU
    // Java field: String PPR_NUMPR
    // Java field: String PPR_ILVL
    // Java field: String PPR_NUMID
    // Java field: String PPR_TEXTALIGNMENT
    // Java field: String PPR_TABS
    // Java field: String PPR_BIDI
    // Java field: String PPR_CONTEXTUAL_SPACING
    // Java field: String PPR_KEEPLINES
    // Java field: String PPR_KEEPNEXT
    // Java field: String PPR_PAGE_BREAK_BEFORE
    // Java field: String PPR_OVER_FLOW_PUNCT
    // Java field: String PPR_TOP_LINE_PUNCT
    // Java field: String PPR_WIDOW_CONTROL
    // Java field: String PPR_WORD_WRAP
    // Java field: String PPR_AUTO_SPACE_DE
    // Java field: String PPR_AUTO_SPACE_DN
    // Java field: String PPR_MIRRORINDENTS
    // Java field: String PPR_OUTLINELVL
    // Java field: String PPR_SUPPRESSAUTOHYPHENS
    // Java field: String PPR_SUPPRESSLINENUMBERS
    // Java field: String PPR_TEXTBOXTIGHTWRAP
    // Java field: String PPR_TEXT_DIRECTION
    // Java field: String PPR_FRAME_PR
    // Java field: String PPR_SUPPRESSOVERLAP
    // Java field: String PPR_ADJUSTRIGHTIND
    // Java field: String RPR_RFONTS
    // Java field: String RPR_B
    // Java field: String RPR_BCS
    // Java field: String RPR_I
    // Java field: String RPR_ICS
    // Java field: String RPR_CAPS
    // Java field: String RPR_SMALLCAPS
    // Java field: String RPR_STRIKE
    // Java field: String RPR_DSTRIKE
    // Java field: String RPR_U
    // Java field: String RPR_SHD
    // Java field: String RPR_SZ
    // Java field: String RPR_SZCS
    // Java field: String RPR_EM
    // Java field: String RPR_VERTALIGN
    // Java field: String RPR_COLOR
    // Java field: String RPR_SPACING
    // Java field: String RPR_KERN
    // Java field: String RPR_RSTYLE
    // Java field: String RPR_HIGHTLIGHT
    // Java field: String RPR_VANISH
    // Java field: String RPR_POSITION
    // Java field: String RPR_W
    // Java field: String RPR_RTL
    // Java field: String RPR_CS
    // Java field: String RPR_EMBOSS
    // Java field: String RPR_IMPRINT
    // Java field: String RPR_NOPROOF
    // Java field: String RPR_OUTLINE
    // Java field: String RPR_SHADOW
    // Java field: String RPR_SNAPTOGRID
    // Java field: String RPR_SPECVANISH
    // Java field: String RPR_WEBHIDDEN
    // Java field: String RPR_EFFECT
    // Java field: String RPR_EASTASIANLAYOUT
    // Java field: String RPR_FITTEXT
    // Java field: String RPR_BDR
    // Java field: String RPR_LANG
    // Java field: String RPR_GLOW
    // Java field: String RPR_REFLECTION
    // Java field: String RPR_TEXTOUTLINE
    // Java field: String RPR_TEXTFILL
    // Java field: String RPR_SCENE3D
    // Java field: String RPR_PROPS3D
    // Java field: String RPR_LIGATURES
    // Java field: String RPR_NUMFORM
    // Java field: String RPR_NUMSPACING
    // Java field: String RPR_STYLISTICSETS
    // Java field: String RPR_CNTXALTS
    // Java field: String TBL
    // Java field: String TBLPR
    // Java field: String TBLPRCHANGE
    // Java field: String TBLGRIDCHANGE
    // Java field: String TRPRCHANGE
    // Java field: String TCPRCHANGE
    // Java field: String TBLPREX
    // Java field: String TBLPREXCHANGE
    // Java field: String TBLPR_TBLSTYLE
    // Java field: String TBLPR_TBLPPR
    // Java field: String TBLPR_TBLOVERLAP
    // Java field: String TBLPR_BIDIVISUAL
    // Java field: String TBLPR_TBLSTYLEROWBANDSIZE
    // Java field: String TBLPR_TBLSTYLECOLBANDSIZE
    // Java field: String TBLPR_TBLW
    // Java field: String TBLPR_TBLCELLSPACING
    // Java field: String TBLPR_TBLIND
    // Java field: String TBLPR_TBLBORDERS
    // Java field: String TBLPR_TBLLAYOUT
    // Java field: String TBLPR_TBLCELLMAR
    // Java field: String TBLPR_TBLLOOK
    // Java field: String TBLPR_TBLCAPTION
    // Java field: String TBLPR_TBLDESCRIPTION
    // Java field: String TBLPR_START
    // Java field: String TBLPR_END
    // Java field: String TBLPR_INSIDEH
    // Java field: String TBLPR_INSIDEV
    // Java field: String TRPR_CNFSTYLE
    // Java field: String TRPR_DIVID
    // Java field: String TRPR_GRIDBEFORE
    // Java field: String TRPR_GRIDAFTER
    // Java field: String TRPR_WBEFORE
    // Java field: String TRPR_WAFTER
    // Java field: String TRPR_CANTSPLIT
    // Java field: String TRPR_TRHEIGHT
    // Java field: String TRPR_TBLHEADER
    // Java field: String TRPR_HIDDEN
    // Java field: String TCPR_TCW
    // Java field: String TCPR_GRIDSPAN
    // Java field: String TCPR_HMERGE
    // Java field: String TCPR_VMERGE
    // Java field: String TCPR_TCBORDERS
    // Java field: String TCPR_NOWRAP
    // Java field: String TCPR_TCMAR
    // Java field: String TCPR_TCFITTEXT
    // Java field: String TCPR_VALIGN
    // Java field: String TCPR_HIDEMARK
    // Java field: String TCPR_HEADERS
    // Java field: String TCPR_TL2BR
    // Java field: String TCPR_TR2BL
    // Java field: String TC
    // Java field: String TR
    // Java field: String TCPR
    // Java field: String TRPR
    // Java field: String TBLGRID
    // Java field: String GRIDCOL
    // Java field: String SECTPR_CHANGE
    // Java field: String SECTPR_FOOTNOTEPR
    // Java field: String SECTPR_ENDNOTEPR
    // Java field: String SECTPR_TYPE
    // Java field: String SECTPR_PGSZ
    // Java field: String SECTPR_PGMAR
    // Java field: String SECTPR_PAPERSRC
    // Java field: String SECTPR_PGBORDERS
    // Java field: String SECTPR_LNNUMTYPE
    // Java field: String SECTPR_PGNUMTYPE
    // Java field: String SECTPR_COLS
    // Java field: String SECTPR_FORMPROT
    // Java field: String SECTPR_NOENDNOTE
    // Java field: String SECTPR_TITLEPG
    // Java field: String SECTPR_RTLGUTTER
    // Java field: String SECTPR_DOCGRID
    // Java field: String SECTPR_PRINTERSETTINGS
    // Java field: String SECTPR_COL
    // Java field: String SECTPR_HEADERREFERENCE
    // Java field: String SECTPR_FOOTERREFERENCE
    // Java field: String SECTPR_POS
    // Java field: String SECTPR_NUMSTART
    // Java field: String SECTPR_NUMRESTART
    // Java field: String FONTS
    // Java field: String FONT
    // Java field: String ALTNAME
    // Java field: String PANOSE1
    // Java field: String CHARSET
    // Java field: String FAMILY
    // Java field: String NOTTRUETYPE
    // Java field: String PITCH
    // Java field: String SIG
    // Java field: String EMBEDREGULAR
    // Java field: String EMBEDBOLD
    // Java field: String EMBEDITALIC
    // Java field: String EMBEDBOLDITALIC
    // Java field: String SETTINGS
    // Java field: String VIEW
    // Java field: String ZOOM
    // Java field: String BORDERS_DO_NOT_SURROUND_HEADER
    // Java field: String BORDERS_DO_NOT_SURROUND_FOOTER
    // Java field: String PROOF_STATE
    // Java field: String DEFAULT_TAB_STOP
    // Java field: String DRAWING_GRID_VERTICAL_SPACING
    // Java field: String DISPLAY_HORIZONTAL_DRAWING_GRID_EVERY
    // Java field: String DISPLAY_VERTICAL_DRAWING_GRID_EVERY
    // Java field: String CHARACTER_SPACING_CONTROL
    // Java field: String SPACE_FOR_UL
    // Java field: String BALANCE_SINGLE_BYTE_DOUBLE_BYTE_WIDTH
    // Java field: String DO_NOT_LEAVE_BACKS_LASH_ALONE
    // Java field: String UL_TRAIL_SPACE
    // Java field: String DO_NOT_EXPAND_SHIFT_RETURN
    // Java field: String ADJUST_LINE_HEIGHT_IN_TABLE
    // Java field: String USE_FE_LAYOUT
    // Java field: String COMPAT_SETTING
    // Java field: String USE_SINGLE_BORDER_FOR_CONTIGUOUS_CELLS
    // Java field: String WP_JUSTIFICATION
    // Java field: String NO_TAB_HANGIND
    // Java field: String NO_LEADING
    // Java field: String NO_COLUMN_BALANCE
    // Java field: String NO_EXTRA_LINESPACING
    // Java field: String SPACING_IN_WHOLE_POINTS
    // Java field: String LINE_WRAP_LIKE_WORD6
    // Java field: String PRINT_BODY_TEXT_BEFORE_HEADER
    // Java field: String PRINT_COL_BLACK
    // Java field: String WP_SPACE_WIDTH
    // Java field: String SHOW_BREAKS_IN_FRAMES
    // Java field: String SUB_FONT_BY_SIZE
    // Java field: String SUPPRESS_BOTTOM_SPACING
    // Java field: String SUPPRESS_TOP_SPACING
    // Java field: String SUPPRESS_SPACING_AT_TOP_OF_PAGE
    // Java field: String SUPPRESS_TOP_SPACING_WP
    // Java field: String SUPPRESS_SP_BF_AFTER_PG_BRK
    // Java field: String SWAP_BORDERS_FACING_PAGES
    // Java field: String CONV_MAIL_MERGE_ESC
    // Java field: String TRUNCATE_FONT_HEIGHTS_LIKE_WP6
    // Java field: String MW_SMALL_CAPS
    // Java field: String USE_PRINTER_METRICS
    // Java field: String DO_NOT_SUPPRESS_PARAGRAPH_BORDERS
    // Java field: String WRAP_TRAIL_SPACES
    // Java field: String FOOTNOTE_LAYOUT_LIKE_WW8
    // Java field: String SHAPE_LAYOUT_LIKE_WW8
    // Java field: String ALIGN_TABLES_ROW_BY_ROW
    // Java field: String FORGET_LAST_TAB_ALIGNMENT
    // Java field: String AUTO_SPACE_LIKE_WORD95
    // Java field: String NO_SPACE_RAISE_LOWER
    // Java field: String DO_NOT_USE_HTML_PARAGRAPH_AUTO_SPACING
    // Java field: String LAYOUT_RAW_TABLE_WIDTH
    // Java field: String LAYOUT_TABLE_ROWS_APART
    // Java field: String USE_WORD97_LINE_BREAK_RULES
    // Java field: String DO_NOT_BREAK_WRAPPED_TABLES
    // Java field: String DO_NOT_SNAP_TO_GRID_IN_CELL
    // Java field: String SELECT_FLD_WITH_FIRST_OR_LAST_CHAR
    // Java field: String APPLY_BREAKING_RULES
    // Java field: String DO_NOT_WRAP_TEXT_WITH_PUNCT
    // Java field: String DO_NOT_USE_EAST_ASIAN_BREAK_RULES
    // Java field: String USE_WORD2002_TABLE_STYLE_RULES
    // Java field: String GROW_AUTO_FIT
    // Java field: String USE_NORMAL_STYLE_FOR_LIST
    // Java field: String DO_NOT_USE_INDENT_AS_NUMBERING_TAB_STOP
    // Java field: String USE_ALT_KINSO_KU_LINE_BREAK_RULES
    // Java field: String ALLOW_SPACE_OF_SAME_STYLE_IN_TABLE
    // Java field: String DO_NOT_SUPPRESS_INDENT_ATION
    // Java field: String DO_NOT_AUTO_FIT_CONSTRAINED_TABLES
    // Java field: String AUTO_FIT_TO_FIRST_FIXED_WIDTH_CELL
    // Java field: String UNDERLINE_TAB_IN_NUM_LIST
    // Java field: String DISPLAY_HANGUL_FIXED_WIDTH
    // Java field: String SPLIT_PG_BREAK_AND_PARA_MARK
    // Java field: String DO_NOT_VERT_ALIGN_CELL_WITH_SP
    // Java field: String DO_NOT_BREAK_CONSTRAINED_FORCED_TABLE
    // Java field: String DO_NOT_VERT_ALIGN_IN_TXBX
    // Java field: String USE_ANSI_KERNING_PAIRS
    // Java field: String CACHED_COL_BALANCE
    // Java field: String REMOVE_PERSONAL_INFORMATION
    // Java field: String REMOVE_DATE_AND_TIME
    // Java field: String DO_NOT_DISPLAY_PAGE_BOUNDARIES
    // Java field: String DISPLAY_BACKGROUND_SHAPE
    // Java field: String PRINT_POST_SCRIPT_OVER_TEXT
    // Java field: String PRINT_FRACTIONAL_CHARACTER_WIDTH
    // Java field: String PRINT_FORMS_DATA
    // Java field: String EMBED_TRUE_TYPE_FONTS
    // Java field: String EMBED_SYSTEM_FONTS
    // Java field: String SAVE_SUBSET_FONTS
    // Java field: String SAVE_FORMS_DATA
    // Java field: String MIRROR_MARGINS
    // Java field: String ALIGN_BORDERS_AND_EDGES
    // Java field: String GUTTER_AT_TOP
    // Java field: String HIDE_SPELLING_ERRORS
    // Java field: String HIDE_GRAMMATICAL_ERRORS
    // Java field: String FORMS_DESIGN
    // Java field: String ATTACHED_TEMPLATE
    // Java field: String LINK_STYLES
    // Java field: String STYLE_PANE_FORMAT_FILTER
    // Java field: String STYLE_PANE_SORT_METHOD
    // Java field: String DOCUMENT_TYPE
    // Java field: String REVISION_VIEW
    // Java field: String TRACK_REVISIONS
    // Java field: String DO_NOT_TRACK_MOVES
    // Java field: String DO_NOT_TRACK_FORMATTING
    // Java field: String DOCUMENT_PROTECTION
    // Java field: String AUTO_FORMAT_OVERRIDE
    // Java field: String STYLE_LOCK_THEME
    // Java field: String style_Lock_QF_Set
    // Java field: String AUTO_HYPHEN_ATION
    // Java field: String CONSECUTIVE_HYPHEN_LIMIT
    // Java field: String HYPHENATION_ZONE
    // Java field: String DO_NOT_HYPHENATE_CAPS
    // Java field: String SHOW_ENVELOPE
    // Java field: String SUMMARY_LENGTH
    // Java field: String CLICK_AND_TYPE_STYLE
    // Java field: String DEFAULT_TABLE_STYLE
    // Java field: String EVEN_AND_ODD_HEADERS
    // Java field: String BOOK_FOLD_REV_PRINTING
    // Java field: String BOOK_FOLD_PRINTING
    // Java field: String BOOK_FOLD_PRINTING_SHEETS
    // Java field: String DRAWING_GRID_HORIZONTAL_SPACING
    // Java field: String DO_NOT_USE_MARGINS_FOR_DRAWING_GRID_ORIGIN
    // Java field: String DRAWING_GRID_HORIZONTAL_ORIGIN
    // Java field: String DRAWING_GRID_VERTICAL_ORIGIN
    // Java field: String DO_NOT_SHADE_FORM_DATA
    // Java field: String NO_PUNCTUATION_KERNING
    // Java field: String PRINT_TWO_ON_ONE
    // Java field: String STRICT_FIRST_AND_LAST_CHARS
    // Java field: String NO_LINE_BREAKS_AFTER
    // Java field: String SAVE_PREVIEW_PICTURE
    // Java field: String DO_NOT_VALIDATE_AGAINST_SCHEMA
    // Java field: String SAVE_INVALID_XML
    // Java field: String IGNORE_MIXED_CONTENT
    // Java field: String ALWAYS_SHOW_PLACE_HOLDER_TEXT
    // Java field: String DO_NOT_DEMARCATE_INVALID_XML
    // Java field: String SAVE_XML_DATA_ONLY
    // Java field: String USE_XSLT_WHEN_SAVING
    // Java field: String SHOW_XML_TAGS
    // Java field: String ALWAYS_MERGE_EMPTY_NAMESPACE
    // Java field: String UPDATE_FIELDS
    // Java field: String DOC_VARS
    // Java field: String DOC_VAR
    // Java field: String ATTACHED_SCHEMA
    // Java field: String THEME_FONT_LANG
    // Java field: String CLR_SCHEME_MAPPING
    // Java field: String DO_NOT_INCLUDE_SUB_DOCS_IN_STATS
    // Java field: String DO_NOT_AUTO_COMPRESS_PICTURES
    // Java field: String FORCE_UP_GRADE
    // Java field: String CAPTIONS
    // Java field: String CAPTION
    // Java field: String AUTO_CAPTIONS
    // Java field: String AUTO_CAPTION
    // Java field: String READ_MODE_INK_LOCK_DOWN
    // Java field: String SMART_TAG_TYPE
    // Java field: String DO_NOT_EMBED_SMART_TAGS
    // Java field: String DECIMAL_SYMBOL
    // Java field: String LIST_SEPARATOR
    // Java field: String NO_LINE_BREAKS_BEFORE
    // Java field: String RSID
    // Java field: String RSIDS
    // Java field: String STYLES
    // Java field: String DOC_DEFAULTS
    // Java field: String RPR_DEFAULT
    // Java field: String PPR_DEFAULT
    // Java field: String STYLE
    // Java field: String NAME
    // Java field: String BASED_ON
    // Java field: String NEXT
    // Java field: String LINK
    // Java field: String QFORMAT
    // Java field: String UI_PRIORITY
    // Java field: String TBL_STYLE_PR
    // Java field: String NUMBERING
    // Java field: String ABSTRACT_NUM
    // Java field: String NUM
    // Java field: String ABSTRACT_NUM_ID
    // Java field: String LVL_OVERRIDE
    // Java field: String START_OVERRIDE
    // Java field: String LVL
    // Java field: String LVL_JC
    // Java field: String IS_LGL
    // Java field: String LVL_TEXT
    // Java field: String SUFF
    // Java field: String LVL_RESTART
    // Java field: String TMPL
    // Java field: String MULTI_LEVEL_TYPE
    // Java field: String STYLE_LINK
    // Java field: String NUM_STYLE_LINK
    // Java field: String NSID
    // Java field: String NUM_PIC_BULLET
    // Java field: String NUM_ID_MAC_AT_CLEAN_UP
    // Java field: String LEGACY
    // Java field: String LVL_PIC_BULLET_ID
    // Java field: String TXBXCONTENT
    // Java field: WMap obj
    // Java field: StringIntMap mapobj
    pub(crate) mapobj: StringIntMap,
}

// Java: public class WMap implements XmlName2Id
impl WMap {
    pub const PREFIX: &'static str = "w";

    // ────────────────────────────────────────────────────────────────────────
    // Java field: String MAIN_NAMESPACING / String R_NAMESPACING
    //
    // ★Java 语义：这两个是**可赋值静态字段**（public static String，非 final）——
    //   `OpenXmlNamespaceHelper.initTransitionNamespace/initStrictNamespace/initXml03Namespace`
    //   会按文件类型把它们**在运行期改写**（transition / strict / xml03 三套词表）。
    //
    // ★Rust 等价实现（本节的核心）：原实现把两者建模为 `pub const &'static str` ⇒ **不可写**，
    //   导致 `OpenXmlNamespaceHelper` 的三处赋值只能停车（DEFERRED_NON_FUNCTION），
    //   strict/xml03 的命名空间切换**永远不生效**（实测差异见 `50_function_xwpf_ns_mode` NS8）。
    //
    //   现在改为 **`RwLock<&'static str>` 运行期可变静态**：
    //     · 载体选 `&'static str` 而非 `String`：Java 侧三套候选值**全是字符串字面量**，
    //       故 `&'static str` 足以完整承载 Java 的赋值语义，且**读取零分配、写入零拷贝**；
    //       （若用 `String`，61 处读取点都要额外 `.clone()`/取 `&`，并把热路径变成分配路径。）
    //     · `RwLock` 而非 `Mutex`：读取是热路径（解析期每属性一次），写入只在**开包时**一次。
    //     · 读/写均**不 panic**（Java 无 `PoisonError` 概念）⇒ 中毒后取回内部值继续用。
    //
    //   读取入口保持 **Java 字段同名**（`WMap::MAIN_NAMESPACING()` / `WMap::R_NAMESPACING()`），
    //   调用点因此只多一对括号，且 §3.8 的「同名字段标识符」判据仍成立。
    // ────────────────────────────────────────────────────────────────────────

    // Java field: String MAIN_NAMESPACING
    /// 读当前主命名空间（Java `WMap.MAIN_NAMESPACING` 的字段读语义）。
    #[allow(non_snake_case)]
    pub fn MAIN_NAMESPACING() -> &'static str {
        read_namespacing(&MAIN_NAMESPACING_STATE)
    }

    // Java field: String R_NAMESPACING
    /// 读当前关系命名空间（Java `WMap.R_NAMESPACING` 的字段读语义）。
    #[allow(non_snake_case)]
    pub fn R_NAMESPACING() -> &'static str {
        read_namespacing(&R_NAMESPACING_STATE)
    }

    /// 写主命名空间（Java `WMap.MAIN_NAMESPACING = <literal>;` 的赋值语义）。
    ///
    /// ★参数取 `&'static str`：Java 的三套候选值均为字面量；这使赋值与读取同为零拷贝，
    ///   并让「运行期改写」不需要任何分配。若将来需要非字面量值，须改成 `String` 载体。
    #[allow(non_snake_case)]
    pub fn set_MAIN_NAMESPACING(value: &'static str) {
        write_namespacing(&MAIN_NAMESPACING_STATE, value);
    }

    /// 写关系命名空间（Java `WMap.R_NAMESPACING = <literal>;` 的赋值语义）。
    #[allow(non_snake_case)]
    pub fn set_R_NAMESPACING(value: &'static str) {
        write_namespacing(&R_NAMESPACING_STATE, value);
    }

    // ── 三套词表的字面量（Java 侧散落在 `OpenXmlNamespaceHelper` 的三个 init* 方法里；
    //    Rust 侧集中放在这里，供 `OpenXmlNamespaceHelper` 与用例引用，避免字面量重复散落） ──
    /// transition（默认）主命名空间 —— 与 Java 字段初值一致（`WMap.java`）。
    pub const MAIN_NAMESPACING_TRANSITIONAL: &'static str =
        "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    /// strict 主命名空间（ISO/IEC 29500 Strict）。
    pub const MAIN_NAMESPACING_STRICT: &'static str =
        "http://purl.oclc.org/ooxml/wordprocessingml/main";
    /// xml03（Word 2003 WordprocessingML）主命名空间。
    pub const MAIN_NAMESPACING_XML03: &'static str =
        "http://schemas.microsoft.com/office/word/2003/wordml";

    /// transition（默认）关系命名空间 —— 与 Java 字段初值一致。
    pub const R_NAMESPACING_TRANSITIONAL: &'static str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    /// strict 关系命名空间（ISO/IEC 29500 Strict）。
    pub const R_NAMESPACING_STRICT: &'static str =
        "http://purl.oclc.org/ooxml/officeDocument/relationships";

    const NUM_FMT: &'static str = "numFmt";
    const DOCUMENT: &'static str = "document";
    const BACKGROUND: &'static str = "background";
    const BODY: &'static str = "body";
    const PICT: &'static str = "pict";
    const P: &'static str = "p";
    const PPR: &'static str = "pPr";
    const R: &'static str = "r";
    const RPR: &'static str = "rPr";
    const RPRCHANGE: &'static str = "rPrChange";
    const PPRCHANGE: &'static str = "pPrChange";
    const T: &'static str = "t";
    const HYPERLINK: &'static str = "hyperlink";
    const SECTPR: &'static str = "sectPr";
    const BR: &'static str = "br";
    const DRAWING: &'static str = "drawing";
    const COMMENTRANGESTART: &'static str = "commentRangeStart";
    const COMMENTRANGEEND: &'static str = "commentRangeEnd";
    const COMMENTREFERENCE: &'static str = "commentReference";
    const BOOKMARKSTART: &'static str = "bookmarkStart";
    const BOOKMARKEND: &'static str = "bookmarkEnd";
    const FOOTNOTEREFERENCE: &'static str = "footnoteReference";
    const ENDNOTEREFERENCE: &'static str = "endnoteReference";
    const INS: &'static str = "ins";
    const DEL: &'static str = "del";
    const DELTEXT: &'static str = "delText";
    const OBJECT: &'static str = "object";
    const INSTRTEXT: &'static str = "instrText";
    const DELINSTRTEXT: &'static str = "delInstrText";
    const NOBREAKHYPHEN: &'static str = "noBreakHyphen";
    const SOFTHYPHEN: &'static str = "softHyphen";
    const DAYSHORT: &'static str = "dayShort";
    const MONTHSHORT: &'static str = "monthShort";
    const YEARSHORT: &'static str = "yearShort";
    const DAYLONG: &'static str = "dayLong";
    const MONTHLONG: &'static str = "monthLong";
    const YEARLONG: &'static str = "yearLong";
    const ANNOTATIONREF: &'static str = "annotationRef";
    const FOOTNOTEREF: &'static str = "footnoteRef";
    const ENDNOTEREF: &'static str = "endnoteRef";
    const SEPARATOR: &'static str = "separator";
    const CONTINUATIONSEPARATOR: &'static str = "continuationSeparator";
    const SYM: &'static str = "sym";
    const PGNUM: &'static str = "pgNum";
    const CR: &'static str = "cr";
    const TAB: &'static str = "tab";
    const FLDCHAR: &'static str = "fldChar";
    const SDT: &'static str = "sdt";
    const SDTCONTENT: &'static str = "sdtContent";
    const SMARTTAG: &'static str = "smartTag";
    const DIR: &'static str = "dir";
    const BDO: &'static str = "bdo";
    const FLDSIMPLE: &'static str = "fldSimple";
    const COMMENT: &'static str = "comment";
    const COMMENTS: &'static str = "comments";
    const FOOTNOTES: &'static str = "footnotes";
    const FOOTNOTE: &'static str = "footnote";
    const ENDNOTES: &'static str = "endnotes";
    const ENDNOTE: &'static str = "endnote";
    const PPR_JC: &'static str = "jc";
    const PPR_IND: &'static str = "ind";
    const PPR_PBDR: &'static str = "pBdr";
    const PPR_PBDR_TOP: &'static str = "top";
    const PPR_PBDR_LEFT: &'static str = "left";
    const PPR_PBDR_BOTTOM: &'static str = "bottom";
    const PPR_PBDR_RIGHT: &'static str = "right";
    const PPR_PBDR_BETEEN: &'static str = "between";
    const PPR_PBDR_BAR: &'static str = "bar";
    const PPR_PSTYLE: &'static str = "pStyle";
    const PPR_KINSOKU: &'static str = "kinsoku";
    const PPR_NUMPR: &'static str = "numPr";
    const PPR_ILVL: &'static str = "ilvl";
    const PPR_NUMID: &'static str = "numId";
    const PPR_TEXTALIGNMENT: &'static str = "textAlignment";
    const PPR_TABS: &'static str = "tabs";
    const PPR_BIDI: &'static str = "bidi";
    const PPR_CONTEXTUAL_SPACING: &'static str = "contextualSpacing";
    const PPR_KEEPLINES: &'static str = "keepLines";
    const PPR_KEEPNEXT: &'static str = "keepNext";
    const PPR_PAGE_BREAK_BEFORE: &'static str = "pageBreakBefore";
    const PPR_OVER_FLOW_PUNCT: &'static str = "overflowPunct";
    const PPR_TOP_LINE_PUNCT: &'static str = "topLinePunct";
    const PPR_WIDOW_CONTROL: &'static str = "widowControl";
    const PPR_WORD_WRAP: &'static str = "wordWrap";
    const PPR_AUTO_SPACE_DE: &'static str = "autoSpaceDE";
    const PPR_AUTO_SPACE_DN: &'static str = "autoSpaceDN";
    const PPR_MIRRORINDENTS: &'static str = "mirrorIndents";
    const PPR_OUTLINELVL: &'static str = "outlineLvl";
    const PPR_SUPPRESSAUTOHYPHENS: &'static str = "suppressAutoHyphens";
    const PPR_SUPPRESSLINENUMBERS: &'static str = "suppressLineNumbers";
    const PPR_TEXTBOXTIGHTWRAP: &'static str = "textboxTightWrap";
    const PPR_TEXT_DIRECTION: &'static str = "textDirection";
    const PPR_FRAME_PR: &'static str = "framePr";
    const PPR_SUPPRESSOVERLAP: &'static str = "suppressOverlap";
    const PPR_ADJUSTRIGHTIND: &'static str = "adjustRightInd";
    const RPR_RFONTS: &'static str = "rFonts";
    const RPR_B: &'static str = "b";
    const RPR_BCS: &'static str = "bCs";
    const RPR_I: &'static str = "i";
    const RPR_ICS: &'static str = "iCs";
    const RPR_CAPS: &'static str = "caps";
    const RPR_SMALLCAPS: &'static str = "smallCaps";
    const RPR_STRIKE: &'static str = "strike";
    const RPR_DSTRIKE: &'static str = "dstrike";
    const RPR_U: &'static str = "u";
    const RPR_SHD: &'static str = "shd";
    const RPR_SZ: &'static str = "sz";
    const RPR_SZCS: &'static str = "szCs";
    const RPR_EM: &'static str = "em";
    const RPR_VERTALIGN: &'static str = "vertAlign";
    const RPR_COLOR: &'static str = "color";
    const RPR_SPACING: &'static str = "spacing";
    const RPR_KERN: &'static str = "kern";
    const RPR_RSTYLE: &'static str = "rStyle";
    const RPR_HIGHTLIGHT: &'static str = "highlight";
    const RPR_VANISH: &'static str = "vanish";
    const RPR_POSITION: &'static str = "position";
    const RPR_W: &'static str = "w";
    const RPR_RTL: &'static str = "rtl";
    const RPR_CS: &'static str = "cs";
    const RPR_EMBOSS: &'static str = "emboss";
    const RPR_IMPRINT: &'static str = "imprint";
    const RPR_NOPROOF: &'static str = "noProof";
    const RPR_OUTLINE: &'static str = "outline";
    const RPR_SHADOW: &'static str = "shadow";
    const RPR_SNAPTOGRID: &'static str = "snapToGrid";
    const RPR_SPECVANISH: &'static str = "specVanish";
    const RPR_WEBHIDDEN: &'static str = "webHidden";
    const RPR_EFFECT: &'static str = "effect";
    const RPR_EASTASIANLAYOUT: &'static str = "eastAsianLayout";
    const RPR_FITTEXT: &'static str = "fitText";
    const RPR_BDR: &'static str = "bdr";
    const RPR_LANG: &'static str = "lang";
    const RPR_GLOW: &'static str = "glow";
    const RPR_REFLECTION: &'static str = "reflection";
    const RPR_TEXTOUTLINE: &'static str = "textOutline";
    const RPR_TEXTFILL: &'static str = "textFill";
    const RPR_SCENE3D: &'static str = "scene3d";
    const RPR_PROPS3D: &'static str = "props3d";
    const RPR_LIGATURES: &'static str = "ligatures";
    const RPR_NUMFORM: &'static str = "numForm";
    const RPR_NUMSPACING: &'static str = "numSpacing";
    const RPR_STYLISTICSETS: &'static str = "stylisticSets";
    const RPR_CNTXALTS: &'static str = "cntxtAlts";
    const TBL: &'static str = "tbl";
    const TBLPR: &'static str = "tblPr";
    const TBLPRCHANGE: &'static str = "tblPrChange";
    const TBLGRIDCHANGE: &'static str = "tblGridChange";
    const TRPRCHANGE: &'static str = "trPrChange";
    const TCPRCHANGE: &'static str = "tcPrChange";
    const TBLPREX: &'static str = "tblPrEx";
    const TBLPREXCHANGE: &'static str = "tblPrExChange";
    const TBLPR_TBLSTYLE: &'static str = "tblStyle";
    const TBLPR_TBLPPR: &'static str = "tblpPr";
    const TBLPR_TBLOVERLAP: &'static str = "tblOverlap";
    const TBLPR_BIDIVISUAL: &'static str = "bidiVisual";
    const TBLPR_TBLSTYLEROWBANDSIZE: &'static str = "tblStyleRowBandSize";
    const TBLPR_TBLSTYLECOLBANDSIZE: &'static str = "tblStyleColBandSize";
    const TBLPR_TBLW: &'static str = "tblW";
    const TBLPR_TBLCELLSPACING: &'static str = "tblCellSpacing";
    const TBLPR_TBLIND: &'static str = "tblInd";
    const TBLPR_TBLBORDERS: &'static str = "tblBorders";
    const TBLPR_TBLLAYOUT: &'static str = "tblLayout";
    const TBLPR_TBLCELLMAR: &'static str = "tblCellMar";
    const TBLPR_TBLLOOK: &'static str = "tblLook";
    const TBLPR_TBLCAPTION: &'static str = "tblCaption";
    const TBLPR_TBLDESCRIPTION: &'static str = "tblDescription";
    const TBLPR_START: &'static str = "start";
    const TBLPR_END: &'static str = "end";
    const TBLPR_INSIDEH: &'static str = "insideH";
    const TBLPR_INSIDEV: &'static str = "insideV";
    const TRPR_CNFSTYLE: &'static str = "cnfStyle";
    const TRPR_DIVID: &'static str = "divId";
    const TRPR_GRIDBEFORE: &'static str = "gridBefore";
    const TRPR_GRIDAFTER: &'static str = "gridAfter";
    const TRPR_WBEFORE: &'static str = "wBefore";
    const TRPR_WAFTER: &'static str = "wAfter";
    const TRPR_CANTSPLIT: &'static str = "cantSplit";
    const TRPR_TRHEIGHT: &'static str = "trHeight";
    const TRPR_TBLHEADER: &'static str = "tblHeader";
    const TRPR_HIDDEN: &'static str = "hidden";
    const TCPR_TCW: &'static str = "tcW";
    const TCPR_GRIDSPAN: &'static str = "gridSpan";
    const TCPR_HMERGE: &'static str = "hMerge";
    const TCPR_VMERGE: &'static str = "vMerge";
    const TCPR_TCBORDERS: &'static str = "tcBorders";
    const TCPR_NOWRAP: &'static str = "noWrap";
    const TCPR_TCMAR: &'static str = "tcMar";
    const TCPR_TCFITTEXT: &'static str = "tcFitText";
    const TCPR_VALIGN: &'static str = "vAlign";
    const TCPR_HIDEMARK: &'static str = "hideMark";
    const TCPR_HEADERS: &'static str = "headers";
    const TCPR_TL2BR: &'static str = "tl2br";
    const TCPR_TR2BL: &'static str = "tr2bl";
    const TC: &'static str = "tc";
    const TR: &'static str = "tr";
    const TCPR: &'static str = "tcPr";
    const TRPR: &'static str = "trPr";
    const TBLGRID: &'static str = "tblGrid";
    const GRIDCOL: &'static str = "gridCol";
    const SECTPR_CHANGE: &'static str = "sectPrChange";
    const SECTPR_FOOTNOTEPR: &'static str = "footnotePr";
    const SECTPR_ENDNOTEPR: &'static str = "endnotePr";
    const SECTPR_TYPE: &'static str = "type";
    const SECTPR_PGSZ: &'static str = "pgSz";
    const SECTPR_PGMAR: &'static str = "pgMar";
    const SECTPR_PAPERSRC: &'static str = "paperSrc";
    const SECTPR_PGBORDERS: &'static str = "pgBorders";
    const SECTPR_LNNUMTYPE: &'static str = "lnNumType";
    const SECTPR_PGNUMTYPE: &'static str = "pgNumType";
    const SECTPR_COLS: &'static str = "cols";
    const SECTPR_FORMPROT: &'static str = "formProt";
    const SECTPR_NOENDNOTE: &'static str = "noEndnote";
    const SECTPR_TITLEPG: &'static str = "titlePg";
    const SECTPR_RTLGUTTER: &'static str = "rtlGutter";
    const SECTPR_DOCGRID: &'static str = "docGrid";
    const SECTPR_PRINTERSETTINGS: &'static str = "printerSettings";
    const SECTPR_COL: &'static str = "col";
    const SECTPR_HEADERREFERENCE: &'static str = "headerReference";
    const SECTPR_FOOTERREFERENCE: &'static str = "footerReference";
    const SECTPR_POS: &'static str = "pos";
    const SECTPR_NUMSTART: &'static str = "numStart";
    const SECTPR_NUMRESTART: &'static str = "numRestart";
    const FONTS: &'static str = "fonts";
    const FONT: &'static str = "font";
    const ALTNAME: &'static str = "altName";
    const PANOSE1: &'static str = "panose1";
    const CHARSET: &'static str = "charset";
    const FAMILY: &'static str = "family";
    const NOTTRUETYPE: &'static str = "notTrueType";
    const PITCH: &'static str = "pitch";
    const SIG: &'static str = "sig";
    const EMBEDREGULAR: &'static str = "embedRegular";
    const EMBEDBOLD: &'static str = "embedBold";
    const EMBEDITALIC: &'static str = "embedItalic";
    const EMBEDBOLDITALIC: &'static str = "embedBoldItalic";
    const SETTINGS: &'static str = "settings";
    const VIEW: &'static str = "view";
    const ZOOM: &'static str = "zoom";
    const BORDERS_DO_NOT_SURROUND_HEADER: &'static str = "bordersDoNotSurroundHeader";
    const BORDERS_DO_NOT_SURROUND_FOOTER: &'static str = "bordersDoNotSurroundFooter";
    const PROOF_STATE: &'static str = "proofState";
    const DEFAULT_TAB_STOP: &'static str = "defaultTabStop";
    const DRAWING_GRID_VERTICAL_SPACING: &'static str = "drawingGridVerticalSpacing";
    const DISPLAY_HORIZONTAL_DRAWING_GRID_EVERY: &'static str = "displayHorizontalDrawingGridEvery";
    const DISPLAY_VERTICAL_DRAWING_GRID_EVERY: &'static str = "displayVerticalDrawingGridEvery";
    const CHARACTER_SPACING_CONTROL: &'static str = "characterSpacingControl";
    const SPACE_FOR_UL: &'static str = "spaceForUL";
    const BALANCE_SINGLE_BYTE_DOUBLE_BYTE_WIDTH: &'static str = "balanceSingleByteDoubleByteWidth";
    const DO_NOT_LEAVE_BACKS_LASH_ALONE: &'static str = "doNotLeaveBackslashAlone";
    const UL_TRAIL_SPACE: &'static str = "ulTrailSpace";
    const DO_NOT_EXPAND_SHIFT_RETURN: &'static str = "doNotExpandShiftReturn";
    const ADJUST_LINE_HEIGHT_IN_TABLE: &'static str = "adjustLineHeightInTable";
    const USE_FE_LAYOUT: &'static str = "useFELayout";
    const COMPAT_SETTING: &'static str = "compatSetting";
    const USE_SINGLE_BORDER_FOR_CONTIGUOUS_CELLS: &'static str = "useSingleBorderforContiguousCells";
    const WP_JUSTIFICATION: &'static str = "wpJustification";
    const NO_TAB_HANGIND: &'static str = "noTabHangInd";
    const NO_LEADING: &'static str = "noLeading";
    const NO_COLUMN_BALANCE: &'static str = "noColumnBalance";
    const NO_EXTRA_LINESPACING: &'static str = "noExtraLineSpacing";
    const SPACING_IN_WHOLE_POINTS: &'static str = "spacingInWholePoints";
    const LINE_WRAP_LIKE_WORD6: &'static str = "lineWrapLikeWord6";
    const PRINT_BODY_TEXT_BEFORE_HEADER: &'static str = "printBodyTextBeforeHeader";
    const PRINT_COL_BLACK: &'static str = "printColBlack";
    const WP_SPACE_WIDTH: &'static str = "wpSpaceWidth";
    const SHOW_BREAKS_IN_FRAMES: &'static str = "showBreaksInFrames";
    const SUB_FONT_BY_SIZE: &'static str = "subFontBySize";
    const SUPPRESS_BOTTOM_SPACING: &'static str = "suppressBottomSpacing";
    const SUPPRESS_TOP_SPACING: &'static str = "suppressTopSpacing";
    const SUPPRESS_SPACING_AT_TOP_OF_PAGE: &'static str = "suppressSpacingAtTopOfPage";
    const SUPPRESS_TOP_SPACING_WP: &'static str = "suppressTopSpacingWP";
    const SUPPRESS_SP_BF_AFTER_PG_BRK: &'static str = "suppressSpBfAfterPgBrk";
    const SWAP_BORDERS_FACING_PAGES: &'static str = "swapBordersFacingPages";
    const CONV_MAIL_MERGE_ESC: &'static str = "convMailMergeEsc";
    const TRUNCATE_FONT_HEIGHTS_LIKE_WP6: &'static str = "truncateFontHeightsLikeWP6";
    const MW_SMALL_CAPS: &'static str = "mwSmallCaps";
    const USE_PRINTER_METRICS: &'static str = "usePrinterMetrics";
    const DO_NOT_SUPPRESS_PARAGRAPH_BORDERS: &'static str = "doNotSuppressParagraphBorders";
    const WRAP_TRAIL_SPACES: &'static str = "wrapTrailSpaces";
    const FOOTNOTE_LAYOUT_LIKE_WW8: &'static str = "footnoteLayoutLikeWW8";
    const SHAPE_LAYOUT_LIKE_WW8: &'static str = "shapeLayoutLikeWW8";
    const ALIGN_TABLES_ROW_BY_ROW: &'static str = "alignTablesRowByRow";
    const FORGET_LAST_TAB_ALIGNMENT: &'static str = "forgetLastTabAlignment";
    const AUTO_SPACE_LIKE_WORD95: &'static str = "autoSpaceLikeWord95";
    const NO_SPACE_RAISE_LOWER: &'static str = "noSpaceRaiseLower";
    const DO_NOT_USE_HTML_PARAGRAPH_AUTO_SPACING: &'static str = "doNotUseHTMLParagraphAutoSpacing";
    const LAYOUT_RAW_TABLE_WIDTH: &'static str = "layoutRawTableWidth";
    const LAYOUT_TABLE_ROWS_APART: &'static str = "layoutTableRowsApart";
    const USE_WORD97_LINE_BREAK_RULES: &'static str = "useWord97LineBreakRules";
    const DO_NOT_BREAK_WRAPPED_TABLES: &'static str = "doNotBreakWrappedTables";
    const DO_NOT_SNAP_TO_GRID_IN_CELL: &'static str = "doNotSnapToGridInCell";
    const SELECT_FLD_WITH_FIRST_OR_LAST_CHAR: &'static str = "selectFldWithFirstOrLastChar";
    const APPLY_BREAKING_RULES: &'static str = "applyBreakingRules";
    const DO_NOT_WRAP_TEXT_WITH_PUNCT: &'static str = "doNotWrapTextWithPunct";
    const DO_NOT_USE_EAST_ASIAN_BREAK_RULES: &'static str = "doNotUseEastAsianBreakRules";
    const USE_WORD2002_TABLE_STYLE_RULES: &'static str = "useWord2002TableStyleRules";
    const GROW_AUTO_FIT: &'static str = "growAutofit";
    const USE_NORMAL_STYLE_FOR_LIST: &'static str = "useNormalStyleForList";
    const DO_NOT_USE_INDENT_AS_NUMBERING_TAB_STOP: &'static str = "doNotUseIndentAsNumberingTabStop";
    const USE_ALT_KINSO_KU_LINE_BREAK_RULES: &'static str = "useAltKinsokuLineBreakRules";
    const ALLOW_SPACE_OF_SAME_STYLE_IN_TABLE: &'static str = "allowSpaceOfSameStyleInTable";
    const DO_NOT_SUPPRESS_INDENT_ATION: &'static str = "doNotSuppressIndentation";
    const DO_NOT_AUTO_FIT_CONSTRAINED_TABLES: &'static str = "doNotAutofitConstrainedTables";
    const AUTO_FIT_TO_FIRST_FIXED_WIDTH_CELL: &'static str = "autofitToFirstFixedWidthCell";
    const UNDERLINE_TAB_IN_NUM_LIST: &'static str = "underlineTabInNumList";
    const DISPLAY_HANGUL_FIXED_WIDTH: &'static str = "displayHangulFixedWidth";
    const SPLIT_PG_BREAK_AND_PARA_MARK: &'static str = "splitPgBreakAndParaMark";
    const DO_NOT_VERT_ALIGN_CELL_WITH_SP: &'static str = "doNotVertAlignCellWithSp";
    const DO_NOT_BREAK_CONSTRAINED_FORCED_TABLE: &'static str = "doNotBreakConstrainedForcedTable";
    const DO_NOT_VERT_ALIGN_IN_TXBX: &'static str = "doNotVertAlignInTxbx";
    const USE_ANSI_KERNING_PAIRS: &'static str = "useAnsiKerningPairs";
    const CACHED_COL_BALANCE: &'static str = "cachedColBalance";
    const REMOVE_PERSONAL_INFORMATION: &'static str = "removePersonalInformation";
    const REMOVE_DATE_AND_TIME: &'static str = "removeDateAndTime";
    const DO_NOT_DISPLAY_PAGE_BOUNDARIES: &'static str = "doNotDisplayPageBoundaries";
    const DISPLAY_BACKGROUND_SHAPE: &'static str = "displayBackgroundShape";
    const PRINT_POST_SCRIPT_OVER_TEXT: &'static str = "printPostScriptOverText";
    const PRINT_FRACTIONAL_CHARACTER_WIDTH: &'static str = "printFractionalCharacterWidth";
    const PRINT_FORMS_DATA: &'static str = "printFormsData";
    const EMBED_TRUE_TYPE_FONTS: &'static str = "embedTrueTypeFonts";
    const EMBED_SYSTEM_FONTS: &'static str = "embedSystemFonts";
    const SAVE_SUBSET_FONTS: &'static str = "saveSubsetFonts";
    const SAVE_FORMS_DATA: &'static str = "saveFormsData";
    const MIRROR_MARGINS: &'static str = "mirrorMargins";
    const ALIGN_BORDERS_AND_EDGES: &'static str = "alignBordersAndEdges";
    const GUTTER_AT_TOP: &'static str = "gutterAtTop";
    const HIDE_SPELLING_ERRORS: &'static str = "hideSpellingErrors";
    const HIDE_GRAMMATICAL_ERRORS: &'static str = "hideGrammaticalErrors";
    const FORMS_DESIGN: &'static str = "formsDesign";
    const ATTACHED_TEMPLATE: &'static str = "attachedTemplate";
    const LINK_STYLES: &'static str = "linkStyles";
    const STYLE_PANE_FORMAT_FILTER: &'static str = "stylePaneFormatFilter";
    const STYLE_PANE_SORT_METHOD: &'static str = "stylePaneSortMethod";
    const DOCUMENT_TYPE: &'static str = "documentType";
    const REVISION_VIEW: &'static str = "revisionView";
    const TRACK_REVISIONS: &'static str = "trackRevisions";
    const DO_NOT_TRACK_MOVES: &'static str = "doNotTrackMoves";
    const DO_NOT_TRACK_FORMATTING: &'static str = "doNotTrackFormatting";
    const DOCUMENT_PROTECTION: &'static str = "documentProtection";
    const AUTO_FORMAT_OVERRIDE: &'static str = "autoFormatOverride";
    const STYLE_LOCK_THEME: &'static str = "styleLockTheme";
    const style_Lock_QF_Set: &'static str = "styleLockQFSet";
    const AUTO_HYPHEN_ATION: &'static str = "autoHyphenation";
    const CONSECUTIVE_HYPHEN_LIMIT: &'static str = "consecutiveHyphenLimit";
    const HYPHENATION_ZONE: &'static str = "hyphenationZone";
    const DO_NOT_HYPHENATE_CAPS: &'static str = "doNotHyphenateCaps";
    const SHOW_ENVELOPE: &'static str = "showEnvelope";
    const SUMMARY_LENGTH: &'static str = "summaryLength";
    const CLICK_AND_TYPE_STYLE: &'static str = "clickAndTypeStyle";
    const DEFAULT_TABLE_STYLE: &'static str = "defaultTableStyle";
    const EVEN_AND_ODD_HEADERS: &'static str = "evenAndOddHeaders";
    const BOOK_FOLD_REV_PRINTING: &'static str = "bookFoldRevPrinting";
    const BOOK_FOLD_PRINTING: &'static str = "bookFoldPrinting";
    const BOOK_FOLD_PRINTING_SHEETS: &'static str = "bookFoldPrintingSheets";
    const DRAWING_GRID_HORIZONTAL_SPACING: &'static str = "drawingGridHorizontalSpacing";
    const DO_NOT_USE_MARGINS_FOR_DRAWING_GRID_ORIGIN: &'static str = "doNotUseMarginsForDrawingGridOrigin";
    const DRAWING_GRID_HORIZONTAL_ORIGIN: &'static str = "drawingGridHorizontalOrigin";
    const DRAWING_GRID_VERTICAL_ORIGIN: &'static str = "drawingGridVerticalOrigin";
    const DO_NOT_SHADE_FORM_DATA: &'static str = "doNotShadeFormData";
    const NO_PUNCTUATION_KERNING: &'static str = "noPunctuationKerning";
    const PRINT_TWO_ON_ONE: &'static str = "printTwoOnOne";
    const STRICT_FIRST_AND_LAST_CHARS: &'static str = "strictFirstAndLastChars";
    const NO_LINE_BREAKS_AFTER: &'static str = "noLineBreaksAfter";
    const SAVE_PREVIEW_PICTURE: &'static str = "savePreviewPicture";
    const DO_NOT_VALIDATE_AGAINST_SCHEMA: &'static str = "doNotValidateAgainstSchema";
    const SAVE_INVALID_XML: &'static str = "saveInvalidXml";
    const IGNORE_MIXED_CONTENT: &'static str = "ignoreMixedContent";
    const ALWAYS_SHOW_PLACE_HOLDER_TEXT: &'static str = "alwaysShowPlaceholderText";
    const DO_NOT_DEMARCATE_INVALID_XML: &'static str = "doNotDemarcateInvalidXml";
    const SAVE_XML_DATA_ONLY: &'static str = "saveXmlDataOnly";
    const USE_XSLT_WHEN_SAVING: &'static str = "useXSLTWhenSaving";
    const SHOW_XML_TAGS: &'static str = "showXMLTags";
    const ALWAYS_MERGE_EMPTY_NAMESPACE: &'static str = "alwaysMergeEmptyNamespace";
    const UPDATE_FIELDS: &'static str = "updateFields";
    const DOC_VARS: &'static str = "docVars";
    const DOC_VAR: &'static str = "docVar";
    const ATTACHED_SCHEMA: &'static str = "attachedSchema";
    const THEME_FONT_LANG: &'static str = "themeFontLang";
    const CLR_SCHEME_MAPPING: &'static str = "clrSchemeMapping";
    const DO_NOT_INCLUDE_SUB_DOCS_IN_STATS: &'static str = "doNotIncludeSubdocsInStats";
    const DO_NOT_AUTO_COMPRESS_PICTURES: &'static str = "doNotAutoCompressPictures";
    const FORCE_UP_GRADE: &'static str = "forceUpgrade";
    const CAPTIONS: &'static str = "captions";
    const CAPTION: &'static str = "caption";
    const AUTO_CAPTIONS: &'static str = "autoCaptions";
    const AUTO_CAPTION: &'static str = "autoCaption";
    const READ_MODE_INK_LOCK_DOWN: &'static str = "readModeInkLockDown";
    const SMART_TAG_TYPE: &'static str = "smartTagType";
    const DO_NOT_EMBED_SMART_TAGS: &'static str = "doNotEmbedSmartTags";
    const DECIMAL_SYMBOL: &'static str = "decimalSymbol";
    const LIST_SEPARATOR: &'static str = "listSeparator";
    const NO_LINE_BREAKS_BEFORE: &'static str = "noLineBreaksBefore";
    const RSID: &'static str = "rsid";
    const RSIDS: &'static str = "rsids";
    const STYLES: &'static str = "styles";
    const DOC_DEFAULTS: &'static str = "docDefaults";
    const RPR_DEFAULT: &'static str = "rPrDefault";
    const PPR_DEFAULT: &'static str = "pPrDefault";
    const STYLE: &'static str = "style";
    const NAME: &'static str = "name";
    const BASED_ON: &'static str = "basedOn";
    const NEXT: &'static str = "next";
    const LINK: &'static str = "link";
    const QFORMAT: &'static str = "qFormat";
    const UI_PRIORITY: &'static str = "uiPriority";
    const TBL_STYLE_PR: &'static str = "tblStylePr";
    const NUMBERING: &'static str = "numbering";
    const ABSTRACT_NUM: &'static str = "abstractNum";
    const NUM: &'static str = "num";
    const ABSTRACT_NUM_ID: &'static str = "abstractNumId";
    const LVL_OVERRIDE: &'static str = "lvlOverride";
    const START_OVERRIDE: &'static str = "startOverride";
    const LVL: &'static str = "lvl";
    const LVL_JC: &'static str = "lvlJc";
    const IS_LGL: &'static str = "isLgl";
    const LVL_TEXT: &'static str = "lvlText";
    const SUFF: &'static str = "suff";
    const LVL_RESTART: &'static str = "lvlRestart";
    const TMPL: &'static str = "tmpl";
    const MULTI_LEVEL_TYPE: &'static str = "multiLevelType";
    const STYLE_LINK: &'static str = "styleLink";
    const NUM_STYLE_LINK: &'static str = "numStyleLink";
    const NSID: &'static str = "nsid";
    const NUM_PIC_BULLET: &'static str = "numPicBullet";
    const NUM_ID_MAC_AT_CLEAN_UP: &'static str = "numIdMacAtCleanup";
    const LEGACY: &'static str = "legacy";
    const LVL_PIC_BULLET_ID: &'static str = "lvlPicBulletId";
    const TXBXCONTENT: &'static str = "txbxContent";

    // Java: void init()
    fn init(&mut self) {
        // Java: this.mapobj.put("p", 112);
        self.mapobj.put("p", 112);
        // Java: this.mapobj.put("r", 114);
        self.mapobj.put("r", 114);
        // Java: this.mapobj.put("pPr", 110226);
        self.mapobj.put("pPr", 110226);
        // Java: this.mapobj.put("rPr", 112148);
        self.mapobj.put("rPr", 112148);
        // Java: this.mapobj.put("rPrChange", -2087069980);
        self.mapobj.put("rPrChange", -2087069980);
        // Java: this.mapobj.put("pPrChange", 1527838946);
        self.mapobj.put("pPrChange", 1527838946);
        // Java: this.mapobj.put("t", 116);
        self.mapobj.put("t", 116);
        // Java: this.mapobj.put("pict", 3440682);
        self.mapobj.put("pict", 3440682);
        // Java: this.mapobj.put("hyperlink", 751294566);
        self.mapobj.put("hyperlink", 751294566);
        // Java: this.mapobj.put("sectPr", -906275931);
        self.mapobj.put("sectPr", -906275931);
        // Java: this.mapobj.put("br", 3152);
        self.mapobj.put("br", 3152);
        // Java: this.mapobj.put("drawing", 1913009182);
        self.mapobj.put("drawing", 1913009182);
        // Java: this.mapobj.put("bookmarkStart", 358940876);
        self.mapobj.put("bookmarkStart", 358940876);
        // Java: this.mapobj.put("bookmarkEnd", -768354363);
        self.mapobj.put("bookmarkEnd", -768354363);
        // Java: this.mapobj.put("commentRangeStart", 1466661092);
        self.mapobj.put("commentRangeStart", 1466661092);
        // Java: this.mapobj.put("commentRangeEnd", 2066314717);
        self.mapobj.put("commentRangeEnd", 2066314717);
        // Java: this.mapobj.put("commentReference", -541840468);
        self.mapobj.put("commentReference", -541840468);
        // Java: this.mapobj.put("footnoteReference", 734762443);
        self.mapobj.put("footnoteReference", 734762443);
        // Java: this.mapobj.put("endnoteReference", -1062047106);
        self.mapobj.put("endnoteReference", -1062047106);
        // Java: this.mapobj.put("ins", 104430);
        self.mapobj.put("ins", 104430);
        // Java: this.mapobj.put("del", 99339);
        self.mapobj.put("del", 99339);
        // Java: this.mapobj.put("delText", 1549942744);
        self.mapobj.put("delText", 1549942744);
        // Java: this.mapobj.put("object", -1023368385);
        self.mapobj.put("object", -1023368385);
        // Java: this.mapobj.put("instrText", 44025401);
        self.mapobj.put("instrText", 44025401);
        // Java: this.mapobj.put("delInstrText", 1219280494);
        self.mapobj.put("delInstrText", 1219280494);
        // Java: this.mapobj.put("noBreakHyphen", 2001013392);
        self.mapobj.put("noBreakHyphen", 2001013392);
        // Java: this.mapobj.put("softHyphen", -462474756);
        self.mapobj.put("softHyphen", -462474756);
        // Java: this.mapobj.put("dayShort", 1919873600);
        self.mapobj.put("dayShort", 1919873600);
        // Java: this.mapobj.put("monthShort", -1639972388);
        self.mapobj.put("monthShort", -1639972388);
        // Java: this.mapobj.put("yearShort", -491345345);
        self.mapobj.put("yearShort", -491345345);
        // Java: this.mapobj.put("dayLong", 1447202872);
        self.mapobj.put("dayLong", 1447202872);
        // Java: this.mapobj.put("monthLong", -1300030180);
        self.mapobj.put("monthLong", -1300030180);
        // Java: this.mapobj.put("yearLong", -1540072359);
        self.mapobj.put("yearLong", -1540072359);
        // Java: this.mapobj.put("annotationRef", -784674076);
        self.mapobj.put("annotationRef", -784674076);
        // Java: this.mapobj.put("footnoteRef", 429190931);
        self.mapobj.put("footnoteRef", 429190931);
        // Java: this.mapobj.put("endnoteRef", 692063750);
        self.mapobj.put("endnoteRef", 692063750);
        // Java: this.mapobj.put("separator", 1732829925);
        self.mapobj.put("separator", 1732829925);
        // Java: this.mapobj.put("continuationSeparator", 150772334);
        self.mapobj.put("continuationSeparator", 150772334);
        // Java: this.mapobj.put("sym", 114375);
        self.mapobj.put("sym", 114375);
        // Java: this.mapobj.put("pgNum", 106581519);
        self.mapobj.put("pgNum", 106581519);
        // Java: this.mapobj.put("cr", 3183);
        self.mapobj.put("cr", 3183);
        // Java: this.mapobj.put("fldChar", -777505580);
        self.mapobj.put("fldChar", -777505580);
        // Java: this.mapobj.put("tab", 114581);
        self.mapobj.put("tab", 114581);
        // Java: this.mapobj.put("sdt", 113731);
        self.mapobj.put("sdt", 113731);
        // Java: this.mapobj.put("sdtContent", 1437683382);
        self.mapobj.put("sdtContent", 1437683382);
        // Java: this.mapobj.put("smartTag", -600772335);
        self.mapobj.put("smartTag", -600772335);
        // Java: this.mapobj.put("dir", 99469);
        self.mapobj.put("dir", 99469);
        // Java: this.mapobj.put("bdo", 97389);
        self.mapobj.put("bdo", 97389);
        // Java: this.mapobj.put("fldSimple", 600796080);
        self.mapobj.put("fldSimple", 600796080);
        // Java: this.mapobj.put("document", 861720859);
        self.mapobj.put("document", 861720859);
        // Java: this.mapobj.put("background", -1332194002);
        self.mapobj.put("background", -1332194002);
        // Java: this.mapobj.put("body", 3029410);
        self.mapobj.put("body", 3029410);
        // Java: this.mapobj.put("tbl", 114622);
        self.mapobj.put("tbl", 114622);
        // Java: this.mapobj.put("tblPr", 110154336);
        self.mapobj.put("tblPr", 110154336);
        // Java: this.mapobj.put("tr", 3710);
        self.mapobj.put("tr", 3710);
        // Java: this.mapobj.put("trPr", 3567904);
        self.mapobj.put("trPr", 3567904);
        // Java: this.mapobj.put("tblPrChange", 96307760);
        self.mapobj.put("tblPrChange", 96307760);
        // Java: this.mapobj.put("tblGridChange", 1513751796);
        self.mapobj.put("tblGridChange", 1513751796);
        // Java: this.mapobj.put("trPrChange", 887167216);
        self.mapobj.put("trPrChange", 887167216);
        // Java: this.mapobj.put("tcPrChange", -2065786911);
        self.mapobj.put("tcPrChange", -2065786911);
        // Java: this.mapobj.put("tblPrEx", -1515863245);
        self.mapobj.put("tblPrEx", -1515863245);
        // Java: this.mapobj.put("tblPrExChange", 1998672067);
        self.mapobj.put("tblPrExChange", 1998672067);
        // Java: this.mapobj.put("comment", 950398559);
        self.mapobj.put("comment", 950398559);
        // Java: this.mapobj.put("footnotes", -638658797);
        self.mapobj.put("footnotes", -638658797);
        // Java: this.mapobj.put("footnote", 395040096);
        self.mapobj.put("footnote", 395040096);
        // Java: this.mapobj.put("endnotes", 1739265734);
        self.mapobj.put("endnotes", 1739265734);
        // Java: this.mapobj.put("endnote", -1606462643);
        self.mapobj.put("endnote", -1606462643);
        // Java: this.mapobj.put("comments", -602415628);
        self.mapobj.put("comments", -602415628);
        // Java: this.mapobj.put("jc", 3385);
        self.mapobj.put("jc", 3385);
        // Java: this.mapobj.put("ind", 104415);
        self.mapobj.put("ind", 104415);
        // Java: this.mapobj.put("pBdr", 3403232);
        self.mapobj.put("pBdr", 3403232);
        // Java: this.mapobj.put("top", 115029);
        self.mapobj.put("top", 115029);
        // Java: this.mapobj.put("left", 3317767);
        self.mapobj.put("left", 3317767);
        // Java: this.mapobj.put("bottom", -1383228885);
        self.mapobj.put("bottom", -1383228885);
        // Java: this.mapobj.put("right", 108511772);
        self.mapobj.put("right", 108511772);
        // Java: this.mapobj.put("between", -216634360);
        self.mapobj.put("between", -216634360);
        // Java: this.mapobj.put("bar", 97299);
        self.mapobj.put("bar", 97299);
        // Java: this.mapobj.put("pStyle", -1008274655);
        self.mapobj.put("pStyle", -1008274655);
        // Java: this.mapobj.put("kinsoku", -710169706);
        self.mapobj.put("kinsoku", -710169706);
        // Java: this.mapobj.put("numPr", 105180200);
        self.mapobj.put("numPr", 105180200);
        // Java: this.mapobj.put("ilvl", 3235609);
        self.mapobj.put("ilvl", 3235609);
        // Java: this.mapobj.put("numId", 105179969);
        self.mapobj.put("numId", 105179969);
        // Java: this.mapobj.put("textAlignment", 2042756918);
        self.mapobj.put("textAlignment", 2042756918);
        // Java: this.mapobj.put("tabs", 3552126);
        self.mapobj.put("tabs", 3552126);
        // Java: this.mapobj.put("bidi", 3023628);
        self.mapobj.put("bidi", 3023628);
        // Java: this.mapobj.put("contextualSpacing", -1290382414);
        self.mapobj.put("contextualSpacing", -1290382414);
        // Java: this.mapobj.put("keepLines", -1765433734);
        self.mapobj.put("keepLines", -1765433734);
        // Java: this.mapobj.put("keepNext", -56893416);
        self.mapobj.put("keepNext", -56893416);
        // Java: this.mapobj.put("pageBreakBefore", 2121002831);
        self.mapobj.put("pageBreakBefore", 2121002831);
        // Java: this.mapobj.put("overflowPunct", 593931864);
        self.mapobj.put("overflowPunct", 593931864);
        // Java: this.mapobj.put("topLinePunct", -145004591);
        self.mapobj.put("topLinePunct", -145004591);
        // Java: this.mapobj.put("widowControl", 1096374979);
        self.mapobj.put("widowControl", 1096374979);
        // Java: this.mapobj.put("wordWrap", 28472948);
        self.mapobj.put("wordWrap", 28472948);
        // Java: this.mapobj.put("autoSpaceDE", 891721560);
        self.mapobj.put("autoSpaceDE", 891721560);
        // Java: this.mapobj.put("autoSpaceDN", 891721569);
        self.mapobj.put("autoSpaceDN", 891721569);
        // Java: this.mapobj.put("mirrorIndents", 791768200);
        self.mapobj.put("mirrorIndents", 791768200);
        // Java: this.mapobj.put("outlineLvl", -877517696);
        self.mapobj.put("outlineLvl", -877517696);
        // Java: this.mapobj.put("suppressAutoHyphens", 1360298717);
        self.mapobj.put("suppressAutoHyphens", 1360298717);
        // Java: this.mapobj.put("suppressLineNumbers", -243955263);
        self.mapobj.put("suppressLineNumbers", -243955263);
        // Java: this.mapobj.put("textboxTightWrap", -485730646);
        self.mapobj.put("textboxTightWrap", -485730646);
        // Java: this.mapobj.put("textDirection", -687708974);
        self.mapobj.put("textDirection", -687708974);
        // Java: this.mapobj.put("framePr", -607253425);
        self.mapobj.put("framePr", -607253425);
        // Java: this.mapobj.put("suppressOverlap", 1692151762);
        self.mapobj.put("suppressOverlap", 1692151762);
        // Java: this.mapobj.put("adjustRightInd", -1769114414);
        self.mapobj.put("adjustRightInd", -1769114414);
        // Java: this.mapobj.put("rFonts", -963181390);
        self.mapobj.put("rFonts", -963181390);
        // Java: this.mapobj.put("b", 98);
        self.mapobj.put("b", 98);
        // Java: this.mapobj.put("bCs", 96370);
        self.mapobj.put("bCs", 96370);
        // Java: this.mapobj.put("bdr", 97392);
        self.mapobj.put("bdr", 97392);
        // Java: this.mapobj.put("caps", 3046113);
        self.mapobj.put("caps", 3046113);
        // Java: this.mapobj.put("cntxtAlts", -992841809);
        self.mapobj.put("cntxtAlts", -992841809);
        // Java: this.mapobj.put("color", 94842723);
        self.mapobj.put("color", 94842723);
        // Java: this.mapobj.put("cs", 3184);
        self.mapobj.put("cs", 3184);
        // Java: this.mapobj.put("dstrike", 1959036182);
        self.mapobj.put("dstrike", 1959036182);
        // Java: this.mapobj.put("eastAsianLayout", -1550498575);
        self.mapobj.put("eastAsianLayout", -1550498575);
        // Java: this.mapobj.put("effect", -1306084975);
        self.mapobj.put("effect", -1306084975);
        // Java: this.mapobj.put("em", 3240);
        self.mapobj.put("em", 3240);
        // Java: this.mapobj.put("emboss", -1299729387);
        self.mapobj.put("emboss", -1299729387);
        // Java: this.mapobj.put("fitText", -848112418);
        self.mapobj.put("fitText", -848112418);
        // Java: this.mapobj.put("glow", 3175821);
        self.mapobj.put("glow", 3175821);
        // Java: this.mapobj.put("highlight", -681210700);
        self.mapobj.put("highlight", -681210700);
        // Java: this.mapobj.put("i", 105);
        self.mapobj.put("i", 105);
        // Java: this.mapobj.put("iCs", 103097);
        self.mapobj.put("iCs", 103097);
        // Java: this.mapobj.put("imprint", 1926118409);
        self.mapobj.put("imprint", 1926118409);
        // Java: this.mapobj.put("kern", 3288342);
        self.mapobj.put("kern", 3288342);
        // Java: this.mapobj.put("lang", 3314158);
        self.mapobj.put("lang", 3314158);
        // Java: this.mapobj.put("ligatures", 486258120);
        self.mapobj.put("ligatures", 486258120);
        // Java: this.mapobj.put("noProof", 2096380931);
        self.mapobj.put("noProof", 2096380931);
        // Java: this.mapobj.put("numForm", -2001340054);
        self.mapobj.put("numForm", -2001340054);
        // Java: this.mapobj.put("numSpacing", -520163875);
        self.mapobj.put("numSpacing", -520163875);
        // Java: this.mapobj.put("outline", -1106245566);
        self.mapobj.put("outline", -1106245566);
        // Java: this.mapobj.put("position", 747804969);
        self.mapobj.put("position", 747804969);
        // Java: this.mapobj.put("props3d", -309119999);
        self.mapobj.put("props3d", -309119999);
        // Java: this.mapobj.put("reflection", -1366299605);
        self.mapobj.put("reflection", -1366299605);
        // Java: this.mapobj.put("textOutline", -1946182059);
        self.mapobj.put("textOutline", -1946182059);
        // Java: this.mapobj.put("rStyle", -951016353);
        self.mapobj.put("rStyle", -951016353);
        // Java: this.mapobj.put("rtl", 113258);
        self.mapobj.put("rtl", 113258);
        // Java: this.mapobj.put("scene3d", 1914645533);
        self.mapobj.put("scene3d", 1914645533);
        // Java: this.mapobj.put("shadow", -903579360);
        self.mapobj.put("shadow", -903579360);
        // Java: this.mapobj.put("shd", 113839);
        self.mapobj.put("shd", 113839);
        // Java: this.mapobj.put("smallCaps", -1623742328);
        self.mapobj.put("smallCaps", -1623742328);
        // Java: this.mapobj.put("snapToGrid", -1996049205);
        self.mapobj.put("snapToGrid", -1996049205);
        // Java: this.mapobj.put("spacing", -2012158909);
        self.mapobj.put("spacing", -2012158909);
        // Java: this.mapobj.put("specVanish", -1541847530);
        self.mapobj.put("specVanish", -1541847530);
        // Java: this.mapobj.put("strike", -891985998);
        self.mapobj.put("strike", -891985998);
        // Java: this.mapobj.put("stylisticSets", 1002557153);
        self.mapobj.put("stylisticSets", 1002557153);
        // Java: this.mapobj.put("sz", 3687);
        self.mapobj.put("sz", 3687);
        // Java: this.mapobj.put("szCs", 3545399);
        self.mapobj.put("szCs", 3545399);
        // Java: this.mapobj.put("textFill", -1004056496);
        self.mapobj.put("textFill", -1004056496);
        // Java: this.mapobj.put("u", 117);
        self.mapobj.put("u", 117);
        // Java: this.mapobj.put("vanish", -823764357);
        self.mapobj.put("vanish", -823764357);
        // Java: this.mapobj.put("vertAlign", -1415507884);
        self.mapobj.put("vertAlign", -1415507884);
        // Java: this.mapobj.put("w", 119);
        self.mapobj.put("w", 119);
        // Java: this.mapobj.put("webHidden", -1466137058);
        self.mapobj.put("webHidden", -1466137058);
        // Java: this.mapobj.put("tblStyle", 255759507);
        self.mapobj.put("tblStyle", 255759507);
        // Java: this.mapobj.put("tblpPr", -880153068);
        self.mapobj.put("tblpPr", -880153068);
        // Java: this.mapobj.put("tblOverlap", 1755679401);
        self.mapobj.put("tblOverlap", 1755679401);
        // Java: this.mapobj.put("bidiVisual", 860953964);
        self.mapobj.put("bidiVisual", 860953964);
        // Java: this.mapobj.put("tblStyleRowBandSize", -634428099);
        self.mapobj.put("tblStyleRowBandSize", -634428099);
        // Java: this.mapobj.put("tblStyleColBandSize", -1042798877);
        self.mapobj.put("tblStyleColBandSize", -1042798877);
        // Java: this.mapobj.put("tblW", 3553369);
        self.mapobj.put("tblW", 3553369);
        // Java: this.mapobj.put("tblCellSpacing", -1986705949);
        self.mapobj.put("tblCellSpacing", -1986705949);
        // Java: this.mapobj.put("tblInd", -880189631);
        self.mapobj.put("tblInd", -880189631);
        // Java: this.mapobj.put("tblBorders", -1380755415);
        self.mapobj.put("tblBorders", -1380755415);
        // Java: this.mapobj.put("tblLayout", -879337336);
        self.mapobj.put("tblLayout", -879337336);
        // Java: this.mapobj.put("tblCellMar", -784869634);
        self.mapobj.put("tblCellMar", -784869634);
        // Java: this.mapobj.put("tblLook", -1515984003);
        self.mapobj.put("tblLook", -1515984003);
        // Java: this.mapobj.put("tblCaption", -895426488);
        self.mapobj.put("tblCaption", -895426488);
        // Java: this.mapobj.put("tblDescription", -499594914);
        self.mapobj.put("tblDescription", -499594914);
        // Java: this.mapobj.put("start", 109757538);
        self.mapobj.put("start", 109757538);
        // Java: this.mapobj.put("end", 100571);
        self.mapobj.put("end", 100571);
        // Java: this.mapobj.put("insideH", 1957244876);
        self.mapobj.put("insideH", 1957244876);
        // Java: this.mapobj.put("insideV", 1957244890);
        self.mapobj.put("insideV", 1957244890);
        // Java: this.mapobj.put("cnfStyle", -1713877738);
        self.mapobj.put("cnfStyle", -1713877738);
        // Java: this.mapobj.put("divId", 95595916);
        self.mapobj.put("divId", 95595916);
        // Java: this.mapobj.put("gridBefore", 196061797);
        self.mapobj.put("gridBefore", 196061797);
        // Java: this.mapobj.put("gridAfter", 1252369974);
        self.mapobj.put("gridAfter", 1252369974);
        // Java: this.mapobj.put("wBefore", 224704214);
        self.mapobj.put("wBefore", 224704214);
        // Java: this.mapobj.put("wAfter", -824916059);
        self.mapobj.put("wAfter", -824916059);
        // Java: this.mapobj.put("cantSplit", 946779862);
        self.mapobj.put("cantSplit", 946779862);
        // Java: this.mapobj.put("trHeight", 556545349);
        self.mapobj.put("trHeight", 556545349);
        // Java: this.mapobj.put("tblHeader", -990885909);
        self.mapobj.put("tblHeader", -990885909);
        // Java: this.mapobj.put("hidden", -1217487446);
        self.mapobj.put("hidden", -1217487446);
        // Java: this.mapobj.put("tcW", 114632);
        self.mapobj.put("tcW", 114632);
        // Java: this.mapobj.put("gridSpan", 318038960);
        self.mapobj.put("gridSpan", 318038960);
        // Java: this.mapobj.put("hMerge", -1243302736);
        self.mapobj.put("hMerge", -1243302736);
        // Java: this.mapobj.put("vMerge", -842494622);
        self.mapobj.put("vMerge", -842494622);
        // Java: this.mapobj.put("tcBorders", -1034858408);
        self.mapobj.put("tcBorders", -1034858408);
        // Java: this.mapobj.put("noWrap", -1040545365);
        self.mapobj.put("noWrap", -1040545365);
        // Java: this.mapobj.put("tcMar", 110154863);
        self.mapobj.put("tcMar", 110154863);
        // Java: this.mapobj.put("tcFitText", -1950215313);
        self.mapobj.put("tcFitText", -1950215313);
        // Java: this.mapobj.put("vAlign", -853376977);
        self.mapobj.put("vAlign", -853376977);
        // Java: this.mapobj.put("hideMark", -1774131409);
        self.mapobj.put("hideMark", -1774131409);
        // Java: this.mapobj.put("headers", 795307910);
        self.mapobj.put("headers", 795307910);
        // Java: this.mapobj.put("tl2br", 110397066);
        self.mapobj.put("tl2br", 110397066);
        // Java: this.mapobj.put("tr2bl", 110575806);
        self.mapobj.put("tr2bl", 110575806);
        // Java: this.mapobj.put("tc", 3695);
        self.mapobj.put("tc", 3695);
        // Java: this.mapobj.put("tcPr", 3553489);
        self.mapobj.put("tcPr", 3553489);
        // Java: this.mapobj.put("tblGrid", -1516130268);
        self.mapobj.put("tblGrid", -1516130268);
        // Java: this.mapobj.put("gridCol", 287338586);
        self.mapobj.put("gridCol", 287338586);
        // Java: this.mapobj.put("sectPrChange", 1469137845);
        self.mapobj.put("sectPrChange", 1469137845);
        // Java: this.mapobj.put("footnotePr", 1676412802);
        self.mapobj.put("footnotePr", 1676412802);
        // Java: this.mapobj.put("endnotePr", -1917338065);
        self.mapobj.put("endnotePr", -1917338065);
        // Java: this.mapobj.put("type", 3575610);
        self.mapobj.put("type", 3575610);
        // Java: this.mapobj.put("pgSz", 3438270);
        self.mapobj.put("pgSz", 3438270);
        // Java: this.mapobj.put("pgMar", 106579943);
        self.mapobj.put("pgMar", 106579943);
        // Java: this.mapobj.put("paperSrc", 1117993144);
        self.mapobj.put("paperSrc", 1117993144);
        // Java: this.mapobj.put("pgBorders", 281298896);
        self.mapobj.put("pgBorders", 281298896);
        // Java: this.mapobj.put("lnNumType", 462761982);
        self.mapobj.put("lnNumType", 462761982);
        // Java: this.mapobj.put("pgNumType", -1786859031);
        self.mapobj.put("pgNumType", -1786859031);
        // Java: this.mapobj.put("cols", 3059443);
        self.mapobj.put("cols", 3059443);
        // Java: this.mapobj.put("formProt", 473571915);
        self.mapobj.put("formProt", 473571915);
        // Java: this.mapobj.put("noEndnote", -1014885812);
        self.mapobj.put("noEndnote", -1014885812);
        // Java: this.mapobj.put("titlePg", -1307249041);
        self.mapobj.put("titlePg", -1307249041);
        // Java: this.mapobj.put("rtlGutter", -378400731);
        self.mapobj.put("rtlGutter", -378400731);
        // Java: this.mapobj.put("docGrid", 1827547294);
        self.mapobj.put("docGrid", 1827547294);
        // Java: this.mapobj.put("printerSettings", 1061563997);
        self.mapobj.put("printerSettings", 1061563997);
        // Java: this.mapobj.put("col", 98688);
        self.mapobj.put("col", 98688);
        // Java: this.mapobj.put("headerReference", -974517442);
        self.mapobj.put("headerReference", -974517442);
        // Java: this.mapobj.put("footerReference", 1797375984);
        self.mapobj.put("footerReference", 1797375984);
        // Java: this.mapobj.put("pos", 111188);
        self.mapobj.put("pos", 111188);
        // Java: this.mapobj.put("numStart", -1899860868);
        self.mapobj.put("numStart", -1899860868);
        // Java: this.mapobj.put("numRestart", -1705465943);
        self.mapobj.put("numRestart", -1705465943);
        // Java: this.mapobj.put("fonts", 97615364);
        self.mapobj.put("fonts", 97615364);
        // Java: this.mapobj.put("font", 3148879);
        self.mapobj.put("font", 3148879);
        // Java: this.mapobj.put("altName", -904959020);
        self.mapobj.put("altName", -904959020);
        // Java: this.mapobj.put("panose1", -796767379);
        self.mapobj.put("panose1", -796767379);
        // Java: this.mapobj.put("charset", 739074380);
        self.mapobj.put("charset", 739074380);
        // Java: this.mapobj.put("family", -1281860764);
        self.mapobj.put("family", -1281860764);
        // Java: this.mapobj.put("notTrueType", -2031710757);
        self.mapobj.put("notTrueType", -2031710757);
        // Java: this.mapobj.put("pitch", 106677056);
        self.mapobj.put("pitch", 106677056);
        // Java: this.mapobj.put("sig", 113873);
        self.mapobj.put("sig", 113873);
        // Java: this.mapobj.put("embedRegular", -1450986877);
        self.mapobj.put("embedRegular", -1450986877);
        // Java: this.mapobj.put("embedBold", -1409488642);
        self.mapobj.put("embedBold", -1409488642);
        // Java: this.mapobj.put("embedItalic", -1399181719);
        self.mapobj.put("embedItalic", -1399181719);
        // Java: this.mapobj.put("embedBoldItalic", 1681905870);
        self.mapobj.put("embedBoldItalic", 1681905870);
        // Java: this.mapobj.put("settings", 1434631203);
        self.mapobj.put("settings", 1434631203);
        // Java: this.mapobj.put("view", 3619493);
        self.mapobj.put("view", 3619493);
        // Java: this.mapobj.put("zoom", 3744723);
        self.mapobj.put("zoom", 3744723);
        // Java: this.mapobj.put("bordersDoNotSurroundHeader", 1625197708);
        self.mapobj.put("bordersDoNotSurroundHeader", 1625197708);
        // Java: this.mapobj.put("bordersDoNotSurroundFooter", 1577607066);
        self.mapobj.put("bordersDoNotSurroundFooter", 1577607066);
        // Java: this.mapobj.put("proofState", -1813563987);
        self.mapobj.put("proofState", -1813563987);
        // Java: this.mapobj.put("defaultTabStop", 450482102);
        self.mapobj.put("defaultTabStop", 450482102);
        // Java: this.mapobj.put("drawingGridVerticalSpacing", -1810657591);
        self.mapobj.put("drawingGridVerticalSpacing", -1810657591);
        // Java: this.mapobj.put("displayHorizontalDrawingGridEvery", 1403465629);
        self.mapobj.put("displayHorizontalDrawingGridEvery", 1403465629);
        // Java: this.mapobj.put("displayVerticalDrawingGridEvery", -519350417);
        self.mapobj.put("displayVerticalDrawingGridEvery", -519350417);
        // Java: this.mapobj.put("characterSpacingControl", 650683171);
        self.mapobj.put("characterSpacingControl", 650683171);
        // Java: this.mapobj.put("spaceForUL", 481385402);
        self.mapobj.put("spaceForUL", 481385402);
        // Java: this.mapobj.put("balanceSingleByteDoubleByteWidth", -1268124223);
        self.mapobj.put("balanceSingleByteDoubleByteWidth", -1268124223);
        // Java: this.mapobj.put("doNotLeaveBackslashAlone", -1933025740);
        self.mapobj.put("doNotLeaveBackslashAlone", -1933025740);
        // Java: this.mapobj.put("ulTrailSpace", 1326464151);
        self.mapobj.put("ulTrailSpace", 1326464151);
        // Java: this.mapobj.put("doNotExpandShiftReturn", -209259536);
        self.mapobj.put("doNotExpandShiftReturn", -209259536);
        // Java: this.mapobj.put("adjustLineHeightInTable", 4960031);
        self.mapobj.put("adjustLineHeightInTable", 4960031);
        // Java: this.mapobj.put("useFELayout", 2036560720);
        self.mapobj.put("useFELayout", 2036560720);
        // Java: this.mapobj.put("compatSetting", -238953362);
        self.mapobj.put("compatSetting", -238953362);
        // Java: this.mapobj.put("useSingleBorderforContiguousCells", -860883301);
        self.mapobj.put("useSingleBorderforContiguousCells", -860883301);
        // Java: this.mapobj.put("wpJustification", 886002841);
        self.mapobj.put("wpJustification", 886002841);
        // Java: this.mapobj.put("noTabHangInd", -2076330663);
        self.mapobj.put("noTabHangInd", -2076330663);
        // Java: this.mapobj.put("noLeading", 641935877);
        self.mapobj.put("noLeading", 641935877);
        // Java: this.mapobj.put("noColumnBalance", -692770203);
        self.mapobj.put("noColumnBalance", -692770203);
        // Java: this.mapobj.put("noExtraLineSpacing", -684373152);
        self.mapobj.put("noExtraLineSpacing", -684373152);
        // Java: this.mapobj.put("spacingInWholePoints", 1780463506);
        self.mapobj.put("spacingInWholePoints", 1780463506);
        // Java: this.mapobj.put("lineWrapLikeWord6", 2029039703);
        self.mapobj.put("lineWrapLikeWord6", 2029039703);
        // Java: this.mapobj.put("printBodyTextBeforeHeader", 843004104);
        self.mapobj.put("printBodyTextBeforeHeader", 843004104);
        // Java: this.mapobj.put("printColBlack", -626055124);
        self.mapobj.put("printColBlack", -626055124);
        // Java: this.mapobj.put("wpSpaceWidth", 1276243993);
        self.mapobj.put("wpSpaceWidth", 1276243993);
        // Java: this.mapobj.put("showBreaksInFrames", 411569660);
        self.mapobj.put("showBreaksInFrames", 411569660);
        // Java: this.mapobj.put("subFontBySize", -790730617);
        self.mapobj.put("subFontBySize", -790730617);
        // Java: this.mapobj.put("suppressBottomSpacing", -1426390301);
        self.mapobj.put("suppressBottomSpacing", -1426390301);
        // Java: this.mapobj.put("suppressTopSpacing", 9168323);
        self.mapobj.put("suppressTopSpacing", 9168323);
        // Java: this.mapobj.put("suppressSpacingAtTopOfPage", -1636932838);
        self.mapobj.put("suppressSpacingAtTopOfPage", -1636932838);
        // Java: this.mapobj.put("suppressTopSpacingWP", 220826588);
        self.mapobj.put("suppressTopSpacingWP", 220826588);
        // Java: this.mapobj.put("suppressSpBfAfterPgBrk", 1148496030);
        self.mapobj.put("suppressSpBfAfterPgBrk", 1148496030);
        // Java: this.mapobj.put("swapBordersFacingPages", -1424803146);
        self.mapobj.put("swapBordersFacingPages", -1424803146);
        // Java: this.mapobj.put("convMailMergeEsc", -1891304888);
        self.mapobj.put("convMailMergeEsc", -1891304888);
        // Java: this.mapobj.put("truncateFontHeightsLikeWP6", 963411439);
        self.mapobj.put("truncateFontHeightsLikeWP6", 963411439);
        // Java: this.mapobj.put("mwSmallCaps", 170752510);
        self.mapobj.put("mwSmallCaps", 170752510);
        // Java: this.mapobj.put("usePrinterMetrics", -1572734800);
        self.mapobj.put("usePrinterMetrics", -1572734800);
        // Java: this.mapobj.put("doNotSuppressParagraphBorders", 1934592950);
        self.mapobj.put("doNotSuppressParagraphBorders", 1934592950);
        // Java: this.mapobj.put("wrapTrailSpaces", 1766037289);
        self.mapobj.put("wrapTrailSpaces", 1766037289);
        // Java: this.mapobj.put("footnoteLayoutLikeWW8", 1179686487);
        self.mapobj.put("footnoteLayoutLikeWW8", 1179686487);
        // Java: this.mapobj.put("shapeLayoutLikeWW8", 1063095734);
        self.mapobj.put("shapeLayoutLikeWW8", 1063095734);
        // Java: this.mapobj.put("alignTablesRowByRow", -1908583373);
        self.mapobj.put("alignTablesRowByRow", -1908583373);
        // Java: this.mapobj.put("forgetLastTabAlignment", -216851567);
        self.mapobj.put("forgetLastTabAlignment", -216851567);
        // Java: this.mapobj.put("noSpaceRaiseLower", -17550310);
        self.mapobj.put("noSpaceRaiseLower", -17550310);
        // Java: this.mapobj.put("autoSpaceLikeWord95", 1691035156);
        self.mapobj.put("autoSpaceLikeWord95", 1691035156);
        // Java: this.mapobj.put("doNotUseHTMLParagraphAutoSpacing", -962302288);
        self.mapobj.put("doNotUseHTMLParagraphAutoSpacing", -962302288);
        // Java: this.mapobj.put("layoutRawTableWidth", 985587158);
        self.mapobj.put("layoutRawTableWidth", 985587158);
        // Java: this.mapobj.put("layoutTableRowsApart", -553493737);
        self.mapobj.put("layoutTableRowsApart", -553493737);
        // Java: this.mapobj.put("useWord97LineBreakRules", 2027445307);
        self.mapobj.put("useWord97LineBreakRules", 2027445307);
        // Java: this.mapobj.put("doNotBreakWrappedTables", -1325812493);
        self.mapobj.put("doNotBreakWrappedTables", -1325812493);
        // Java: this.mapobj.put("doNotSnapToGridInCell", 508827450);
        self.mapobj.put("doNotSnapToGridInCell", 508827450);
        // Java: this.mapobj.put("selectFldWithFirstOrLastChar", -2068700649);
        self.mapobj.put("selectFldWithFirstOrLastChar", -2068700649);
        // Java: this.mapobj.put("applyBreakingRules", 1545631590);
        self.mapobj.put("applyBreakingRules", 1545631590);
        // Java: this.mapobj.put("doNotWrapTextWithPunct", 71947221);
        self.mapobj.put("doNotWrapTextWithPunct", 71947221);
        // Java: this.mapobj.put("doNotUseEastAsianBreakRules", 818602208);
        self.mapobj.put("doNotUseEastAsianBreakRules", 818602208);
        // Java: this.mapobj.put("useWord2002TableStyleRules", 1486454403);
        self.mapobj.put("useWord2002TableStyleRules", 1486454403);
        // Java: this.mapobj.put("growAutofit", -2134705041);
        self.mapobj.put("growAutofit", -2134705041);
        // Java: this.mapobj.put("useNormalStyleForList", -334642556);
        self.mapobj.put("useNormalStyleForList", -334642556);
        // Java: this.mapobj.put("doNotUseIndentAsNumberingTabStop", -87807397);
        self.mapobj.put("doNotUseIndentAsNumberingTabStop", -87807397);
        // Java: this.mapobj.put("useAltKinsokuLineBreakRules", -814900736);
        self.mapobj.put("useAltKinsokuLineBreakRules", -814900736);
        // Java: this.mapobj.put("allowSpaceOfSameStyleInTable", 489375826);
        self.mapobj.put("allowSpaceOfSameStyleInTable", 489375826);
        // Java: this.mapobj.put("doNotSuppressIndentation", 389251180);
        self.mapobj.put("doNotSuppressIndentation", 389251180);
        // Java: this.mapobj.put("doNotAutofitConstrainedTables", -905658143);
        self.mapobj.put("doNotAutofitConstrainedTables", -905658143);
        // Java: this.mapobj.put("autofitToFirstFixedWidthCell", -1702461273);
        self.mapobj.put("autofitToFirstFixedWidthCell", -1702461273);
        // Java: this.mapobj.put("underlineTabInNumList", -1879087818);
        self.mapobj.put("underlineTabInNumList", -1879087818);
        // Java: this.mapobj.put("displayHangulFixedWidth", 959575805);
        self.mapobj.put("displayHangulFixedWidth", 959575805);
        // Java: this.mapobj.put("splitPgBreakAndParaMark", 1820694550);
        self.mapobj.put("splitPgBreakAndParaMark", 1820694550);
        // Java: this.mapobj.put("doNotVertAlignCellWithSp", -124054063);
        self.mapobj.put("doNotVertAlignCellWithSp", -124054063);
        // Java: this.mapobj.put("doNotBreakConstrainedForcedTable", 869235382);
        self.mapobj.put("doNotBreakConstrainedForcedTable", 869235382);
        // Java: this.mapobj.put("doNotVertAlignInTxbx", 1908795051);
        self.mapobj.put("doNotVertAlignInTxbx", 1908795051);
        // Java: this.mapobj.put("useAnsiKerningPairs", -736154697);
        self.mapobj.put("useAnsiKerningPairs", -736154697);
        // Java: this.mapobj.put("cachedColBalance", 316436478);
        self.mapobj.put("cachedColBalance", 316436478);
        // Java: this.mapobj.put("removePersonalInformation", -195827800);
        self.mapobj.put("removePersonalInformation", -195827800);
        // Java: this.mapobj.put("removeDateAndTime", -1937589614);
        self.mapobj.put("removeDateAndTime", -1937589614);
        // Java: this.mapobj.put("doNotDisplayPageBoundaries", -1648882127);
        self.mapobj.put("doNotDisplayPageBoundaries", -1648882127);
        // Java: this.mapobj.put("displayBackgroundShape", -314910223);
        self.mapobj.put("displayBackgroundShape", -314910223);
        // Java: this.mapobj.put("printPostScriptOverText", -588222375);
        self.mapobj.put("printPostScriptOverText", -588222375);
        // Java: this.mapobj.put("printFractionalCharacterWidth", -1136057609);
        self.mapobj.put("printFractionalCharacterWidth", -1136057609);
        // Java: this.mapobj.put("printFormsData", -1088528308);
        self.mapobj.put("printFormsData", -1088528308);
        // Java: this.mapobj.put("embedTrueTypeFonts", -1601567069);
        self.mapobj.put("embedTrueTypeFonts", -1601567069);
        // Java: this.mapobj.put("embedSystemFonts", -646125444);
        self.mapobj.put("embedSystemFonts", -646125444);
        // Java: this.mapobj.put("saveSubsetFonts", -319044795);
        self.mapobj.put("saveSubsetFonts", -319044795);
        // Java: this.mapobj.put("saveFormsData", -169608324);
        self.mapobj.put("saveFormsData", -169608324);
        // Java: this.mapobj.put("mirrorMargins", -312379450);
        self.mapobj.put("mirrorMargins", -312379450);
        // Java: this.mapobj.put("alignBordersAndEdges", -974208031);
        self.mapobj.put("alignBordersAndEdges", -974208031);
        // Java: this.mapobj.put("gutterAtTop", 1942426567);
        self.mapobj.put("gutterAtTop", 1942426567);
        // Java: this.mapobj.put("hideSpellingErrors", 905124775);
        self.mapobj.put("hideSpellingErrors", 905124775);
        // Java: this.mapobj.put("hideGrammaticalErrors", -1354586665);
        self.mapobj.put("hideGrammaticalErrors", -1354586665);
        // Java: this.mapobj.put("formsDesign", -1114891571);
        self.mapobj.put("formsDesign", -1114891571);
        // Java: this.mapobj.put("attachedTemplate", 1597753918);
        self.mapobj.put("attachedTemplate", 1597753918);
        // Java: this.mapobj.put("linkStyles", 843501948);
        self.mapobj.put("linkStyles", 843501948);
        // Java: this.mapobj.put("stylePaneFormatFilter", -1434364504);
        self.mapobj.put("stylePaneFormatFilter", -1434364504);
        // Java: this.mapobj.put("stylePaneSortMethod", 1976320440);
        self.mapobj.put("stylePaneSortMethod", 1976320440);
        // Java: this.mapobj.put("documentType", -1473196299);
        self.mapobj.put("documentType", -1473196299);
        // Java: this.mapobj.put("revisionView", -1250426592);
        self.mapobj.put("revisionView", -1250426592);
        // Java: this.mapobj.put("trackRevisions", -1109412115);
        self.mapobj.put("trackRevisions", -1109412115);
        // Java: this.mapobj.put("doNotTrackMoves", 1030286495);
        self.mapobj.put("doNotTrackMoves", 1030286495);
        // Java: this.mapobj.put("doNotTrackFormatting", -332797016);
        self.mapobj.put("doNotTrackFormatting", -332797016);
        // Java: this.mapobj.put("documentProtection", 751788628);
        self.mapobj.put("documentProtection", 751788628);
        // Java: this.mapobj.put("autoFormatOverride", 1734155378);
        self.mapobj.put("autoFormatOverride", 1734155378);
        // Java: this.mapobj.put("styleLockTheme", -1647305555);
        self.mapobj.put("styleLockTheme", -1647305555);
        // Java: this.mapobj.put("styleLockQFSet", -1651106543);
        self.mapobj.put("styleLockQFSet", -1651106543);
        // Java: this.mapobj.put("autoHyphenation", -1200535596);
        self.mapobj.put("autoHyphenation", -1200535596);
        // Java: this.mapobj.put("consecutiveHyphenLimit", -1706783841);
        self.mapobj.put("consecutiveHyphenLimit", -1706783841);
        // Java: this.mapobj.put("hyphenationZone", 675671887);
        self.mapobj.put("hyphenationZone", 675671887);
        // Java: this.mapobj.put("doNotHyphenateCaps", -569660999);
        self.mapobj.put("doNotHyphenateCaps", -569660999);
        // Java: this.mapobj.put("showEnvelope", 1394213357);
        self.mapobj.put("showEnvelope", 1394213357);
        // Java: this.mapobj.put("summaryLength", 170588716);
        self.mapobj.put("summaryLength", 170588716);
        // Java: this.mapobj.put("clickAndTypeStyle", 814737448);
        self.mapobj.put("clickAndTypeStyle", 814737448);
        // Java: this.mapobj.put("defaultTableStyle", -1202800732);
        self.mapobj.put("defaultTableStyle", -1202800732);
        // Java: this.mapobj.put("evenAndOddHeaders", -938949228);
        self.mapobj.put("evenAndOddHeaders", -938949228);
        // Java: this.mapobj.put("bookFoldRevPrinting", -2113273106);
        self.mapobj.put("bookFoldRevPrinting", -2113273106);
        // Java: this.mapobj.put("bookFoldPrinting", 1370596031);
        self.mapobj.put("bookFoldPrinting", 1370596031);
        // Java: this.mapobj.put("bookFoldPrintingSheets", 1170313875);
        self.mapobj.put("bookFoldPrintingSheets", 1170313875);
        // Java: this.mapobj.put("drawingGridHorizontalSpacing", -1583930533);
        self.mapobj.put("drawingGridHorizontalSpacing", -1583930533);
        // Java: this.mapobj.put("doNotUseMarginsForDrawingGridOrigin", 126608775);
        self.mapobj.put("doNotUseMarginsForDrawingGridOrigin", 126608775);
        // Java: this.mapobj.put("drawingGridHorizontalOrigin", 529214734);
        self.mapobj.put("drawingGridHorizontalOrigin", 529214734);
        // Java: this.mapobj.put("drawingGridVerticalOrigin", -1556309024);
        self.mapobj.put("drawingGridVerticalOrigin", -1556309024);
        // Java: this.mapobj.put("doNotShadeFormData", -1228639629);
        self.mapobj.put("doNotShadeFormData", -1228639629);
        // Java: this.mapobj.put("noPunctuationKerning", -1185891341);
        self.mapobj.put("noPunctuationKerning", -1185891341);
        // Java: this.mapobj.put("printTwoOnOne", 1839947496);
        self.mapobj.put("printTwoOnOne", 1839947496);
        // Java: this.mapobj.put("strictFirstAndLastChars", -347335209);
        self.mapobj.put("strictFirstAndLastChars", -347335209);
        // Java: this.mapobj.put("noLineBreaksAfter", 955468371);
        self.mapobj.put("noLineBreaksAfter", 955468371);
        // Java: this.mapobj.put("savePreviewPicture", -59805965);
        self.mapobj.put("savePreviewPicture", -59805965);
        // Java: this.mapobj.put("doNotValidateAgainstSchema", 1410250724);
        self.mapobj.put("doNotValidateAgainstSchema", 1410250724);
        // Java: this.mapobj.put("saveInvalidXml", 705016829);
        self.mapobj.put("saveInvalidXml", 705016829);
        // Java: this.mapobj.put("ignoreMixedContent", 956434160);
        self.mapobj.put("ignoreMixedContent", 956434160);
        // Java: this.mapobj.put("alwaysShowPlaceholderText", 1771222516);
        self.mapobj.put("alwaysShowPlaceholderText", 1771222516);
        // Java: this.mapobj.put("doNotDemarcateInvalidXml", 73898596);
        self.mapobj.put("doNotDemarcateInvalidXml", 73898596);
        // Java: this.mapobj.put("saveXmlDataOnly", 424792560);
        self.mapobj.put("saveXmlDataOnly", 424792560);
        // Java: this.mapobj.put("useXSLTWhenSaving", 1290881598);
        self.mapobj.put("useXSLTWhenSaving", 1290881598);
        // Java: this.mapobj.put("showXMLTags", 1791649555);
        self.mapobj.put("showXMLTags", 1791649555);
        // Java: this.mapobj.put("alwaysMergeEmptyNamespace", -434582953);
        self.mapobj.put("alwaysMergeEmptyNamespace", -434582953);
        // Java: this.mapobj.put("updateFields", -1322964766);
        self.mapobj.put("updateFields", -1322964766);
        // Java: this.mapobj.put("docVars", 1827978116);
        self.mapobj.put("docVars", 1827978116);
        // Java: this.mapobj.put("docVar", -1326506289);
        self.mapobj.put("docVar", -1326506289);
        // Java: this.mapobj.put("attachedSchema", 2093929573);
        self.mapobj.put("attachedSchema", 2093929573);
        // Java: this.mapobj.put("themeFontLang", -1918151738);
        self.mapobj.put("themeFontLang", -1918151738);
        // Java: this.mapobj.put("clrSchemeMapping", -1762893632);
        self.mapobj.put("clrSchemeMapping", -1762893632);
        // Java: this.mapobj.put("doNotIncludeSubdocsInStats", 283782879);
        self.mapobj.put("doNotIncludeSubdocsInStats", 283782879);
        // Java: this.mapobj.put("doNotAutoCompressPictures", 1176767214);
        self.mapobj.put("doNotAutoCompressPictures", 1176767214);
        // Java: this.mapobj.put("forceUpgrade", 275028145);
        self.mapobj.put("forceUpgrade", 275028145);
        // Java: this.mapobj.put("captions", -50093235);
        self.mapobj.put("captions", -50093235);
        // Java: this.mapobj.put("caption", 552573414);
        self.mapobj.put("caption", 552573414);
        // Java: this.mapobj.put("autoCaptions", -223951332);
        self.mapobj.put("autoCaptions", -223951332);
        // Java: this.mapobj.put("autoCaption", -838508233);
        self.mapobj.put("autoCaption", -838508233);
        // Java: this.mapobj.put("readModeInkLockDown", -701177766);
        self.mapobj.put("readModeInkLockDown", -701177766);
        // Java: this.mapobj.put("smartTagType", -1989671957);
        self.mapobj.put("smartTagType", -1989671957);
        // Java: this.mapobj.put("doNotEmbedSmartTags", 1478011665);
        self.mapobj.put("doNotEmbedSmartTags", 1478011665);
        // Java: this.mapobj.put("decimalSymbol", 1575613513);
        self.mapobj.put("decimalSymbol", 1575613513);
        // Java: this.mapobj.put("listSeparator", 971121639);
        self.mapobj.put("listSeparator", 971121639);
        // Java: this.mapobj.put("noLineBreaksBefore", -417953304);
        self.mapobj.put("noLineBreaksBefore", -417953304);
        // Java: this.mapobj.put("rsid", 3510044);
        self.mapobj.put("rsid", 3510044);
        // Java: this.mapobj.put("rsids", 108811479);
        self.mapobj.put("rsids", 108811479);
        // Java: this.mapobj.put("styles", -891774750);
        self.mapobj.put("styles", -891774750);
        // Java: this.mapobj.put("docDefaults", -1780079702);
        self.mapobj.put("docDefaults", -1780079702);
        // Java: this.mapobj.put("rPrDefault", 531200397);
        self.mapobj.put("rPrDefault", 531200397);
        // Java: this.mapobj.put("pPrDefault", 924227407);
        self.mapobj.put("pPrDefault", 924227407);
        // Java: this.mapobj.put("style", 109780401);
        self.mapobj.put("style", 109780401);
        // Java: this.mapobj.put("name", 3373707);
        self.mapobj.put("name", 3373707);
        // Java: this.mapobj.put("basedOn", -332612366);
        self.mapobj.put("basedOn", -332612366);
        // Java: this.mapobj.put("next", 3377907);
        self.mapobj.put("next", 3377907);
        // Java: this.mapobj.put("link", 3321850);
        self.mapobj.put("link", 3321850);
        // Java: this.mapobj.put("qFormat", -681243704);
        self.mapobj.put("qFormat", -681243704);
        // Java: this.mapobj.put("uiPriority", 872749144);
        self.mapobj.put("uiPriority", 872749144);
        // Java: this.mapobj.put("tblStylePr", 971752949);
        self.mapobj.put("tblStylePr", 971752949);
        // Java: this.mapobj.put("numbering", 1649937401);
        self.mapobj.put("numbering", 1649937401);
        // Java: this.mapobj.put("abstractNum", -717178876);
        self.mapobj.put("abstractNum", -717178876);
        // Java: this.mapobj.put("num", 109446);
        self.mapobj.put("num", 109446);
        // Java: this.mapobj.put("abstractNumId", -2014130113);
        self.mapobj.put("abstractNumId", -2014130113);
        // Java: this.mapobj.put("lvlOverride", 447078926);
        self.mapobj.put("lvlOverride", 447078926);
        // Java: this.mapobj.put("startOverride", 657113678);
        self.mapobj.put("startOverride", 657113678);
        // Java: this.mapobj.put("lvl", 107554);
        self.mapobj.put("lvl", 107554);
        // Java: this.mapobj.put("numFmt", -1034390745);
        self.mapobj.put("numFmt", -1034390745);
        // Java: this.mapobj.put("lvlJc", 103361787);
        self.mapobj.put("lvlJc", 103361787);
        // Java: this.mapobj.put("isLgl", 100472007);
        self.mapobj.put("isLgl", 100472007);
        // Java: this.mapobj.put("lvlText", 546733167);
        self.mapobj.put("lvlText", 546733167);
        // Java: this.mapobj.put("suff", 3541666);
        self.mapobj.put("suff", 3541666);
        // Java: this.mapobj.put("lvlRestart", -567736435);
        self.mapobj.put("lvlRestart", -567736435);
        // Java: this.mapobj.put("tmpl", 3564085);
        self.mapobj.put("tmpl", 3564085);
        // Java: this.mapobj.put("multiLevelType", 717051813);
        self.mapobj.put("multiLevelType", 717051813);
        // Java: this.mapobj.put("styleLink", 1805058379);
        self.mapobj.put("styleLink", 1805058379);
        // Java: this.mapobj.put("numStyleLink", 1093209189);
        self.mapobj.put("numStyleLink", 1093209189);
        // Java: this.mapobj.put("nsid", 3390880);
        self.mapobj.put("nsid", 3390880);
        // Java: this.mapobj.put("numPicBullet", 992624198);
        self.mapobj.put("numPicBullet", 992624198);
        // Java: this.mapobj.put("numIdMacAtCleanup", 397458499);
        self.mapobj.put("numIdMacAtCleanup", 397458499);
        // Java: this.mapobj.put("legacy", -1106578487);
        self.mapobj.put("legacy", -1106578487);
        // Java: this.mapobj.put("lvlPicBulletId", 1017748453);
        self.mapobj.put("lvlPicBulletId", 1017748453);
        // Java: this.mapobj.put("txbxContent", 163404095);
        self.mapobj.put("txbxContent", 163404095);
    }

    // Java: WMap()
    pub fn new() -> Self {
        // Java: super();
        let mut wmap = WMap {
            mapobj: StringIntMap::new(11),
        };
        // Java: this.init();
        wmap.init();
        wmap
    }

    // Java: static WMap getInstance()
    pub fn getInstance() -> &'static WMap {
        // Java: if (obj == null)
        // Java: obj = new WMap();
        let obj = OBJ.get_or_init(|| WMap::new());
        // Java: return obj;
        obj
    }

    // Java: int getId(String uri, String name)
    pub fn getId_str(&self, _uri: &str, name: &str) -> i32 {
        // Java: return this.mapobj.get(name);
        self.mapobj.get(name)
    }

    // Java: int getId(int prefix, int name)
    pub fn getId_int(&self, _prefix: i32, name: i32) -> i32 {
        // Java: return this.mapobj.get(name);
        self.mapobj.get_by_int(name)
    }
}

impl XmlName2Id for WMap {
    // Java: abstract int getId(String var1, String var2)
    fn getId_str(&self, var1: &str, var2: &str) -> i32 {
        self.mapobj.get(var2)
    }

    // Java: abstract int getId(int var1, int var2)
    fn getId_int(&self, _var1: i32, var2: i32) -> i32 {
        self.mapobj.get_by_int(var2)
    }
}
