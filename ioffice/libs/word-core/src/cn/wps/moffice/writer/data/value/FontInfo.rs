// Java: cn.wps.moffice.writer.data.value.FontInfo
//
// ★M-STAGE1 P2 —— 本文件原为 **compile-surface shell**（3 行 `pub struct FontInfo;`，
//   零字段零方法），遮蔽了 Java 的 304 行实现。它是 `di03_seam` 影子长期存在的
//   根本原因之一：`FontTable` 需要 `getFontName()` 等访问器，而 shell 没有
//   ⇒ 影子只能把整条调用链挡成 `{}`（静默丢弃）。见 `M_STAGE1_BASELINE.md` §16.3。
//
// 本文件按 Java 原文**逐字段、逐常量、逐访问器**对齐：
//   - 17 个实例字段（含 `mPanoseConstrast` 的**原文拼写**，不"纠正"为 Contrast）；
//   - ~115 个 `public static final` 常量；
//   - 34 个 getter/setter，**Java 的 `logTrue` 判定与 setter 内的夹取逻辑逐字保留**。
//
// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;

// Java: public final class FontInfo
#[derive(Clone, Debug)]
pub struct FontInfo {
    // java private byte mCharacterPitch = 0;
    mCharacterPitch: i8,
    // java private boolean mTrueType = false;
    mTrueType: bool,
    // java private byte mFontFamily = 0;
    mFontFamily: i8,
    // java private short mWeight = 400;
    mWeight: i16,
    // java private short mCharacterSet = 0;
    mCharacterSet: i16,
    // java private String mFontName = "";
    mFontName: String,
    // java private String mAlternativeName = "";
    mAlternativeName: String,
    // java private byte mPanoseFamilyType = 0;
    mPanoseFamilyType: i8,
    // java private byte mPanoseSerifStyle = 0;
    mPanoseSerifStyle: i8,
    // java private byte mPanoseWeight = 0;
    mPanoseWeight: i8,
    // java private byte mPanoseProportion;
    mPanoseProportion: i8,
    // java private byte mPanoseConstrast;
    mPanoseConstrast: i8,
    // java private byte mPanoseStrokeVariation;
    mPanoseStrokeVariation: i8,
    // java private byte mPanoseArmStyle;
    mPanoseArmStyle: i8,
    // java private byte mPanoseLetterform;
    mPanoseLetterform: i8,
    // java private byte mPanoseMidline;
    mPanoseMidline: i8,
    // java private byte mPanoseHeight;
    mPanoseHeight: i8,
    // java private byte[] mFontSignature;
    mFontSignature: Option<Vec<i8>>,
}

impl Default for FontInfo {
    fn default() -> Self {
        // java 字段初值：mWeight = 400，mFontName = ""，mAlternativeName = ""，其余 0/false/null
        Self {
            mCharacterPitch: 0,
            mTrueType: false,
            mFontFamily: 0,
            mWeight: 400,
            mCharacterSet: 0,
            mFontName: String::new(),
            mAlternativeName: String::new(),
            mPanoseFamilyType: 0,
            mPanoseSerifStyle: 0,
            mPanoseWeight: 0,
            mPanoseProportion: 0,
            mPanoseConstrast: 0,
            mPanoseStrokeVariation: 0,
            mPanoseArmStyle: 0,
            mPanoseLetterform: 0,
            mPanoseMidline: 0,
            mPanoseHeight: 0,
            mFontSignature: None,
        }
    }
}

impl FontInfo {
    // java public static final byte DefaultPitch = 0;
    pub const DefaultPitch: i8 = 0;
    // java public static final byte FixedPitch = 1;
    pub const FixedPitch: i8 = 1;
    // java public static final byte VariablePitch = 2;
    pub const VariablePitch: i8 = 2;
    // java public static final byte NONE = 0;
    pub const NONE: i8 = 0;
    // java public static final byte Roman_Serif = 1;
    pub const Roman_Serif: i8 = 1;
    // java public static final byte Swiss_Sans_serif = 2;
    pub const Swiss_Sans_serif: i8 = 2;
    // java public static final byte Modern_Monospace = 3;
    pub const Modern_Monospace: i8 = 3;
    // java public static final byte Script_Curisive = 4;
    pub const Script_Curisive: i8 = 4;
    // java public static final byte Decorative_Fantasy = 5;
    pub const Decorative_Fantasy: i8 = 5;
    // java public static final short ANSI_CHARSET = 0;
    pub const ANSI_CHARSET: i16 = 0;
    // java public static final short DEFAULT_CHARSET = 1;
    pub const DEFAULT_CHARSET: i16 = 1;
    // java public static final short SYMBOL_CHARSET = 2;
    pub const SYMBOL_CHARSET: i16 = 2;
    // java public static final short SHIFTJIS_CHARSET = 128;
    pub const SHIFTJIS_CHARSET: i16 = 128;
    // java public static final short HANGEUL_CHARSET = 129;
    pub const HANGEUL_CHARSET: i16 = 129;
    // java public static final short HANGUL_CHARSET = 129;
    pub const HANGUL_CHARSET: i16 = 129;
    // java public static final short GB2312_CHARSET = 134;
    pub const GB2312_CHARSET: i16 = 134;
    // java public static final short CHINESEBIG5_CHARSET = 136;
    pub const CHINESEBIG5_CHARSET: i16 = 136;
    // java public static final short OEM_CHARSET = 255;
    pub const OEM_CHARSET: i16 = 255;
    // java public static final short JOHAB_CHARSET = 130;
    pub const JOHAB_CHARSET: i16 = 130;
    // java public static final short HEBREW_CHARSET = 177;
    pub const HEBREW_CHARSET: i16 = 177;
    // java public static final short ARABIC_CHARSET = 178;
    pub const ARABIC_CHARSET: i16 = 178;
    // java public static final short GREEK_CHARSET = 161;
    pub const GREEK_CHARSET: i16 = 161;
    // java public static final short TURKISH_CHARSET = 162;
    pub const TURKISH_CHARSET: i16 = 162;
    // java public static final short VIETNAMESE_CHARSET = 163;
    pub const VIETNAMESE_CHARSET: i16 = 163;
    // java public static final short THAI_CHARSET = 222;
    pub const THAI_CHARSET: i16 = 222;
    // java public static final short EASTEUROPE_CHARSET = 238;
    pub const EASTEUROPE_CHARSET: i16 = 238;
    // java public static final short RUSSIAN_CHARSET = 204;
    pub const RUSSIAN_CHARSET: i16 = 204;
    // java public static final short MAC_CHARSET = 77;
    pub const MAC_CHARSET: i16 = 77;
    // java public static final short BALTIC_CHARSET = 186;
    pub const BALTIC_CHARSET: i16 = 186;
    // java public static final byte PAN_ANY = 0;
    pub const PAN_ANY: i8 = 0;
    // java public static final byte PAN_NO_FIT = 1;
    pub const PAN_NO_FIT: i8 = 1;
    // java public static final byte PAN_FAMILY_TEXT_DISPLAY = 2;
    pub const PAN_FAMILY_TEXT_DISPLAY: i8 = 2;
    // java public static final byte PAN_FAMILY_SCRIPT = 3;
    pub const PAN_FAMILY_SCRIPT: i8 = 3;
    // java public static final byte PAN_FAMILY_DECORATIVE = 4;
    pub const PAN_FAMILY_DECORATIVE: i8 = 4;
    // java public static final byte PAN_FAMILY_PICTORIAL = 5;
    pub const PAN_FAMILY_PICTORIAL: i8 = 5;
    // java public static final byte PAN_SERIF_COVE = 2;
    pub const PAN_SERIF_COVE: i8 = 2;
    // java public static final byte PAN_SERIF_OBTUSE_COVE = 3;
    pub const PAN_SERIF_OBTUSE_COVE: i8 = 3;
    // java public static final byte PAN_SERIF_SQUARE_COVE = 4;
    pub const PAN_SERIF_SQUARE_COVE: i8 = 4;
    // java public static final byte PAN_SERIF_OBTUSE_SQUARE_COVE = 5;
    pub const PAN_SERIF_OBTUSE_SQUARE_COVE: i8 = 5;
    // java public static final byte PAN_SERIF_SQUARE = 6;
    pub const PAN_SERIF_SQUARE: i8 = 6;
    // java public static final byte PAN_SERIF_THIN = 7;
    pub const PAN_SERIF_THIN: i8 = 7;
    // java public static final byte PAN_SERIF_BONE = 8;
    pub const PAN_SERIF_BONE: i8 = 8;
    // java public static final byte PAN_SERIF_EXAGGERATED = 9;
    pub const PAN_SERIF_EXAGGERATED: i8 = 9;
    // java public static final byte PAN_SERIF_TRIANGLE = 10;
    pub const PAN_SERIF_TRIANGLE: i8 = 10;
    // java public static final byte PAN_SERIF_NORMAL_SANS = 11;
    pub const PAN_SERIF_NORMAL_SANS: i8 = 11;
    // java public static final byte PAN_SERIF_OBTUSE_SANS = 12;
    pub const PAN_SERIF_OBTUSE_SANS: i8 = 12;
    // java public static final byte PAN_SERIF_PERP_SANS = 13;
    pub const PAN_SERIF_PERP_SANS: i8 = 13;
    // java public static final byte PAN_SERIF_FLARED = 14;
    pub const PAN_SERIF_FLARED: i8 = 14;
    // java public static final byte PAN_SERIF_ROUNDED = 15;
    pub const PAN_SERIF_ROUNDED: i8 = 15;
    // java public static final byte PAN_WEIGHT_VERY_LIGHT = 2;
    pub const PAN_WEIGHT_VERY_LIGHT: i8 = 2;
    // java public static final byte PAN_WEIGHT_LIGHT = 3;
    pub const PAN_WEIGHT_LIGHT: i8 = 3;
    // java public static final byte PAN_WEIGHT_THIN = 4;
    pub const PAN_WEIGHT_THIN: i8 = 4;
    // java public static final byte PAN_WEIGHT_BOOK = 5;
    pub const PAN_WEIGHT_BOOK: i8 = 5;
    // java public static final byte PAN_WEIGHT_MEDIUM = 6;
    pub const PAN_WEIGHT_MEDIUM: i8 = 6;
    // java public static final byte PAN_WEIGHT_DEMI = 7;
    pub const PAN_WEIGHT_DEMI: i8 = 7;
    // java public static final byte PAN_WEIGHT_BOLD = 8;
    pub const PAN_WEIGHT_BOLD: i8 = 8;
    // java public static final byte PAN_WEIGHT_HEAVY = 9;
    pub const PAN_WEIGHT_HEAVY: i8 = 9;
    // java public static final byte PAN_WEIGHT_BLACK = 10;
    pub const PAN_WEIGHT_BLACK: i8 = 10;
    // java public static final byte PAN_WEIGHT_NORD = 11;
    pub const PAN_WEIGHT_NORD: i8 = 11;
    // java public static final byte PAN_PROP_OLD_STYLE = 2;
    pub const PAN_PROP_OLD_STYLE: i8 = 2;
    // java public static final byte PAN_PROP_MODERN = 3;
    pub const PAN_PROP_MODERN: i8 = 3;
    // java public static final byte PAN_PROP_EVEN_WIDTH = 4;
    pub const PAN_PROP_EVEN_WIDTH: i8 = 4;
    // java public static final byte PAN_PROP_EXPANDED = 5;
    pub const PAN_PROP_EXPANDED: i8 = 5;
    // java public static final byte PAN_PROP_CONDENSED = 6;
    pub const PAN_PROP_CONDENSED: i8 = 6;
    // java public static final byte PAN_PROP_VERY_EXPANDED = 7;
    pub const PAN_PROP_VERY_EXPANDED: i8 = 7;
    // java public static final byte PAN_PROP_VERY_CONDENSED = 8;
    pub const PAN_PROP_VERY_CONDENSED: i8 = 8;
    // java public static final byte PAN_PROP_MONOSPACED = 9;
    pub const PAN_PROP_MONOSPACED: i8 = 9;
    // java public static final byte PAN_CONTRAST_NONE = 2;
    pub const PAN_CONTRAST_NONE: i8 = 2;
    // java public static final byte PAN_CONTRAST_VERY_LOW = 3;
    pub const PAN_CONTRAST_VERY_LOW: i8 = 3;
    // java public static final byte PAN_CONTRAST_LOW = 4;
    pub const PAN_CONTRAST_LOW: i8 = 4;
    // java public static final byte PAN_CONTRAST_MEDIUM_LOW = 5;
    pub const PAN_CONTRAST_MEDIUM_LOW: i8 = 5;
    // java public static final byte PAN_CONTRAST_MEDIUM = 6;
    pub const PAN_CONTRAST_MEDIUM: i8 = 6;
    // java public static final byte PAN_CONTRAST_MEDIUM_HIGH = 7;
    pub const PAN_CONTRAST_MEDIUM_HIGH: i8 = 7;
    // java public static final byte PAN_CONTRAST_HIGH = 8;
    pub const PAN_CONTRAST_HIGH: i8 = 8;
    // java public static final byte PAN_CONTRAST_VERY_HIGH = 9;
    pub const PAN_CONTRAST_VERY_HIGH: i8 = 9;
    // java public static final byte PAN_NO_VARIATION = 2;
    pub const PAN_NO_VARIATION: i8 = 2;
    // java public static final byte PAN_GRADUA_LDIAGONAL = 3;
    pub const PAN_GRADUA_LDIAGONAL: i8 = 3;
    // java public static final byte PAN_GRADUAL_TRANSITIONAL = 4;
    pub const PAN_GRADUAL_TRANSITIONAL: i8 = 4;
    // java public static final byte PAN_GRADUAL_VERTICAL = 5;
    pub const PAN_GRADUAL_VERTICAL: i8 = 5;
    // java public static final byte PAN_GRADUAL_HORIZONTAL = 6;
    pub const PAN_GRADUAL_HORIZONTAL: i8 = 6;
    // java public static final byte PAN_RAPID_VERTICAL = 7;
    pub const PAN_RAPID_VERTICAL: i8 = 7;
    // java public static final byte PAN_RAPID_HORIZONTAL = 8;
    pub const PAN_RAPID_HORIZONTAL: i8 = 8;
    // java public static final byte PAN_INSTANT_VERTICAL = 9;
    pub const PAN_INSTANT_VERTICAL: i8 = 9;
    // java public static final byte PAN_INSTANT_HORIZONTAL = 10;
    pub const PAN_INSTANT_HORIZONTAL: i8 = 10;
    // java public static final byte PAN_STRAIGHT_ARMS_HORZ = 2;
    pub const PAN_STRAIGHT_ARMS_HORZ: i8 = 2;
    // java public static final byte PAN_STRAIGHT_ARMS_WEDGE = 3;
    pub const PAN_STRAIGHT_ARMS_WEDGE: i8 = 3;
    // java public static final byte PAN_STRAIGHT_ARMS_VERT = 4;
    pub const PAN_STRAIGHT_ARMS_VERT: i8 = 4;
    // java public static final byte PAN_STRAIGHT_ARMS_SINGLE_SERIF = 5;
    pub const PAN_STRAIGHT_ARMS_SINGLE_SERIF: i8 = 5;
    // java public static final byte PAN_STRAIGHT_ARMS_DOUBLE_SERIF = 6;
    pub const PAN_STRAIGHT_ARMS_DOUBLE_SERIF: i8 = 6;
    // java public static final byte PAN_BENT_ARMS_HORZ = 7;
    pub const PAN_BENT_ARMS_HORZ: i8 = 7;
    // java public static final byte PAN_BENT_ARMS_WEDGE = 8;
    pub const PAN_BENT_ARMS_WEDGE: i8 = 8;
    // java public static final byte PAN_BENT_ARMS_VERT = 9;
    pub const PAN_BENT_ARMS_VERT: i8 = 9;
    // java public static final byte PAN_BENT_ARMS_SINGLE_SERIF = 10;
    pub const PAN_BENT_ARMS_SINGLE_SERIF: i8 = 10;
    // java public static final byte PAN_BENT_ARMS_DOUBLE_SERIF = 11;
    pub const PAN_BENT_ARMS_DOUBLE_SERIF: i8 = 11;
    // java public static final byte PAN_LETT_NORMAL_CONTACT = 2;
    pub const PAN_LETT_NORMAL_CONTACT: i8 = 2;
    // java public static final byte PAN_LETT_NORMAL_WEIGHTED = 3;
    pub const PAN_LETT_NORMAL_WEIGHTED: i8 = 3;
    // java public static final byte PAN_LETT_NORMAL_BOXED = 4;
    pub const PAN_LETT_NORMAL_BOXED: i8 = 4;
    // java public static final byte PAN_LETT_NORMAL_FLATTENED = 5;
    pub const PAN_LETT_NORMAL_FLATTENED: i8 = 5;
    // java public static final byte PAN_LETT_NORMAL_ROUNDED = 6;
    pub const PAN_LETT_NORMAL_ROUNDED: i8 = 6;
    // java public static final byte PAN_LETT_NORMAL_OFF_CENTER = 7;
    pub const PAN_LETT_NORMAL_OFF_CENTER: i8 = 7;
    // java public static final byte PAN_LETT_NORMAL_SQUARE = 8;
    pub const PAN_LETT_NORMAL_SQUARE: i8 = 8;
    // java public static final byte PAN_LETT_OBLIQUE_CONTACT = 9;
    pub const PAN_LETT_OBLIQUE_CONTACT: i8 = 9;
    // java public static final byte PAN_LETT_OBLIQUE_WEIGHTED = 10;
    pub const PAN_LETT_OBLIQUE_WEIGHTED: i8 = 10;
    // java public static final byte PAN_LETT_OBLIQUE_BOXED = 11;
    pub const PAN_LETT_OBLIQUE_BOXED: i8 = 11;
    // java public static final byte PAN_MIDLINE_STANDARD_TRIMMED = 2;
    pub const PAN_MIDLINE_STANDARD_TRIMMED: i8 = 2;
    // java public static final byte PAN_MIDLINE_STANDARD_POINTED = 3;
    pub const PAN_MIDLINE_STANDARD_POINTED: i8 = 3;
    // java public static final byte PAN_MIDLINE_STANDARD_SERIFED = 4;
    pub const PAN_MIDLINE_STANDARD_SERIFED: i8 = 4;
    // java public static final byte PAN_MIDLINE_HIGH_TRIMMED = 5;
    pub const PAN_MIDLINE_HIGH_TRIMMED: i8 = 5;
    // java public static final byte PAN_MIDLINE_HIGH_POINTED = 6;
    pub const PAN_MIDLINE_HIGH_POINTED: i8 = 6;
    // java public static final byte PAN_MIDLINE_HIGH_SERIFED = 7;
    pub const PAN_MIDLINE_HIGH_SERIFED: i8 = 7;
    // java public static final byte PAN_MIDLINE_CONSTANT_TRIMMED = 8;
    pub const PAN_MIDLINE_CONSTANT_TRIMMED: i8 = 8;
    // java public static final byte PAN_MIDLINE_CONSTANT_POINTED = 9;
    pub const PAN_MIDLINE_CONSTANT_POINTED: i8 = 9;
    // java public static final byte PAN_MIDLINE_CONSTANT_SERIFED = 10;
    pub const PAN_MIDLINE_CONSTANT_SERIFED: i8 = 10;
    // java public static final byte PAN_MIDLINE_LOW_TRIMMED = 11;
    pub const PAN_MIDLINE_LOW_TRIMMED: i8 = 11;
    // java public static final byte PAN_MIDLINE_LOW_POINTED = 12;
    pub const PAN_MIDLINE_LOW_POINTED: i8 = 12;
    // java public static final byte PAN_MIDLINE_LOW_SERIFED = 13;
    pub const PAN_MIDLINE_LOW_SERIFED: i8 = 13;
    // java public static final byte PAN_XHEIGHT_CONSTANT_SMALL = 2;
    pub const PAN_XHEIGHT_CONSTANT_SMALL: i8 = 2;
    // java public static final byte PAN_XHEIGHT_CONSTANT_STD = 3;
    pub const PAN_XHEIGHT_CONSTANT_STD: i8 = 3;
    // java public static final byte PAN_XHEIGHT_CONSTANT_LARGE = 4;
    pub const PAN_XHEIGHT_CONSTANT_LARGE: i8 = 4;
    // java public static final byte PAN_XHEIGHT_DUCKING_SMALL = 5;
    pub const PAN_XHEIGHT_DUCKING_SMALL: i8 = 5;
    // java public static final byte PAN_XHEIGHT_DUCKING_STD = 6;
    pub const PAN_XHEIGHT_DUCKING_STD: i8 = 6;
    // java public static final byte PAN_XHEIGHT_DUCKING_LARGE = 7;
    pub const PAN_XHEIGHT_DUCKING_LARGE: i8 = 7;

    // java public FontInfo()
    pub fn FontInfo() -> Self {
        Self::default()
    }

    // java public byte getCharacterPitch()
    pub fn getCharacterPitch(&self) -> i8 {
        // java return this.mCharacterPitch;
        return self.mCharacterPitch;
    }

    // java public void setCharacterPitch(byte value)
    pub fn setCharacterPitch(&mut self, value: i8) {
        // java Assert.logTrue("DefaultPitch <= value && value <= VariablePitch", 0 <= value && value <= 2);
        Assert::logTrue_msg("DefaultPitch <= value && value <= VariablePitch", 0 <= value && value <= 2);
        // java this.mCharacterPitch = value;
        self.mCharacterPitch = value;
    }

    // java public boolean isTrueType()
    pub fn isTrueType(&self) -> bool {
        // java return this.mTrueType;
        return self.mTrueType;
    }

    // java public void setTrueType(boolean value)
    pub fn setTrueType(&mut self, value: bool) {
        // java this.mTrueType = value;
        self.mTrueType = value;
    }

    // java public byte getFontFamily()
    pub fn getFontFamily(&self) -> i8 {
        // java return this.mFontFamily;
        return self.mFontFamily;
    }

    // java public void setFontFamily(byte value)
    pub fn setFontFamily(&mut self, value: i8) {
        // java Assert.logTrue("NONE <= value && value <= Decorative_Fantasy", 0 <= value && value <= 5);
        Assert::logTrue_msg("NONE <= value && value <= Decorative_Fantasy", 0 <= value && value <= 5);
        // java this.mFontFamily = value;
        self.mFontFamily = value;
    }

    // java public short getWeight()
    pub fn getWeight(&self) -> i16 {
        // java return this.mWeight;
        return self.mWeight;
    }

    // java public void setWeight(short value)
    pub fn setWeight(&mut self, value: i16) {
        // java Assert.logTrue("0 <= value && value <= 1000", 0 <= value && value <= 1000);
        Assert::logTrue_msg("0 <= value && value <= 1000", 0 <= value && value <= 1000);
        // java this.mWeight = value;
        self.mWeight = value;
    }

    // java public short getCharacterSet()
    pub fn getCharacterSet(&self) -> i16 {
        // java return this.mCharacterSet;
        return self.mCharacterSet;
    }

    // java public void setCharacterSet(short value)
    pub fn setCharacterSet(&mut self, value: i16) {
        // java Assert.logTrue("ANSI_CHARSET <= value && value <= OEM_CHARSET", 0 <= value && value <= 255);
        Assert::logTrue_msg("ANSI_CHARSET <= value && value <= OEM_CHARSET", 0 <= value && value <= 255);
        // java this.mCharacterSet = value;
        self.mCharacterSet = value;
    }

    // java public String getFontName()
    pub fn getFontName(&self) -> String {
        // java return this.mFontName;
        return self.mFontName.clone();
    }

    // java public void setFontName(String value)
    pub fn setFontName(&mut self, value: String) {
        // java Assert.assertNotNull("value should not be null!", value);
        Assert::assertNotNull_msg("value should not be null!", &Some(value.clone()));
        // java this.mFontName = value;
        self.mFontName = value;
    }

    // java public String getAlternativeName()
    pub fn getAlternativeName(&self) -> String {
        // java return this.mAlternativeName;
        return self.mAlternativeName.clone();
    }

    // java public void setAlternativeName(String value)
    pub fn setAlternativeName(&mut self, value: String) {
        // java Assert.assertNotNull("value should not be null!", value);
        Assert::assertNotNull_msg("value should not be null!", &Some(value.clone()));
        // java this.mAlternativeName = value;
        self.mAlternativeName = value;
    }

    // java public byte getPanoseFamilyType()
    pub fn getPanoseFamilyType(&self) -> i8 {
        // java return this.mPanoseFamilyType;
        return self.mPanoseFamilyType;
    }

    // java public void setPanoseFamilyType(byte type)
    pub fn setPanoseFamilyType(&mut self, r#type: i8) {
        // java Assert.logTrue("PAN_ANY <= type && type <= PAN_FAMILY_PICTORIAL", 0 <= type && type <= 5);
        Assert::logTrue_msg("PAN_ANY <= type && type <= PAN_FAMILY_PICTORIAL", 0 <= r#type && r#type <= 5);
        // java this.mPanoseFamilyType = type;
        self.mPanoseFamilyType = r#type;
    }

    // java public byte getPanoseSerifStyle()
    pub fn getPanoseSerifStyle(&self) -> i8 {
        // java return this.mPanoseSerifStyle;
        return self.mPanoseSerifStyle;
    }

    // java public void setPanoseSerifStyle(byte value)
    pub fn setPanoseSerifStyle(&mut self, value: i8) {
        // java Assert.logTrue("PAN_ANY <= value && value <= PAN_SERIF_ROUNDED", 0 <= value && value <= 15);
        Assert::logTrue_msg("PAN_ANY <= value && value <= PAN_SERIF_ROUNDED", 0 <= value && value <= 15);
        // java this.mPanoseSerifStyle = value;
        self.mPanoseSerifStyle = value;
    }

    // java public byte getPanoseWeight()
    pub fn getPanoseWeight(&self) -> i8 {
        // java return this.mPanoseWeight;
        return self.mPanoseWeight;
    }

    // java public void setPanoseWeight(byte value)
    pub fn setPanoseWeight(&mut self, value: i8) {
        // java this.mPanoseWeight = 0 <= value && value <= 11 ? value : 3;
        self.mPanoseWeight = if 0 <= value && value <= 11 { value } else { 3 };
    }

    // java public byte getPanoseProportion()
    pub fn getPanoseProportion(&self) -> i8 {
        // java return this.mPanoseProportion;
        return self.mPanoseProportion;
    }

    // java public void setPanoseProportion(byte value)
    pub fn setPanoseProportion(&mut self, value: i8) {
        // java this.mPanoseProportion = 0 <= value && value <= 9 ? value : 9;
        self.mPanoseProportion = if 0 <= value && value <= 9 { value } else { 9 };
    }

    // java public byte getPanoseConstrast()
    pub fn getPanoseConstrast(&self) -> i8 {
        // java return this.mPanoseConstrast;
        return self.mPanoseConstrast;
    }

    // java public void setPanoseConstrast(byte value)
    pub fn setPanoseConstrast(&mut self, value: i8) {
        // java this.mPanoseConstrast = 0 <= value && value <= 9 ? value : 2;
        self.mPanoseConstrast = if 0 <= value && value <= 9 { value } else { 2 };
    }

    // java public byte getPanoseStrokeVariation()
    pub fn getPanoseStrokeVariation(&self) -> i8 {
        // java return this.mPanoseStrokeVariation;
        return self.mPanoseStrokeVariation;
    }

    // java public void setPanoseStrokeVariation(byte value)
    pub fn setPanoseStrokeVariation(&mut self, value: i8) {
        // java this.mPanoseArmStyle = 0 <= value && value <= 10 ? value : 2;
        // ★Java 原文即写 `this.mPanoseArmStyle`（疑为原代码笔误，非本移植引入）；
        //   按纪律**忠实保留**，不"纠正"为 mPanoseStrokeVariation。
        self.mPanoseArmStyle = if 0 <= value && value <= 10 { value } else { 2 };
    }

    // java public byte getPanoseArmStyle()
    pub fn getPanoseArmStyle(&self) -> i8 {
        // java return this.mPanoseArmStyle;
        return self.mPanoseArmStyle;
    }

    // java public void setPanoseArmStyle(byte value)
    pub fn setPanoseArmStyle(&mut self, value: i8) {
        // java Assert.logTrue("PAN_ANY <= value && value <= PAN_BENT_ARMS_DOUBLE_SERIF", 0 <= value && value <= 11);
        Assert::logTrue_msg("PAN_ANY <= value && value <= PAN_BENT_ARMS_DOUBLE_SERIF", 0 <= value && value <= 11);
        // java this.mPanoseArmStyle = value;
        self.mPanoseArmStyle = value;
    }

    // java public byte getPanoseLetterform()
    pub fn getPanoseLetterform(&self) -> i8 {
        // java return this.mPanoseLetterform;
        return self.mPanoseLetterform;
    }

    // java public void setPanoseLetterform(byte value)
    pub fn setPanoseLetterform(&mut self, value: i8) {
        // java Assert.logTrue("PAN_ANY <= value && value <= PAN_LETT_OBLIQUE_BOXED", 0 <= value && value <= 11);
        Assert::logTrue_msg("PAN_ANY <= value && value <= PAN_LETT_OBLIQUE_BOXED", 0 <= value && value <= 11);
        // java this.mPanoseLetterform = value;
        self.mPanoseLetterform = value;
    }

    // java public byte getPanoseMidline()
    pub fn getPanoseMidline(&self) -> i8 {
        // java return this.mPanoseMidline;
        return self.mPanoseMidline;
    }

    // java public void setPanoseMidline(byte value)
    pub fn setPanoseMidline(&mut self, value: i8) {
        // java Assert.logTrue("PAN_ANY <= value && value <= PAN_MIDLINE_LOW_SERIFED", 0 <= value && value <= 13);
        Assert::logTrue_msg("PAN_ANY <= value && value <= PAN_MIDLINE_LOW_SERIFED", 0 <= value && value <= 13);
        // java this.mPanoseMidline = value;
        self.mPanoseMidline = value;
    }

    // java public byte getPanoseHeight()
    pub fn getPanoseHeight(&self) -> i8 {
        // java return this.mPanoseHeight;
        return self.mPanoseHeight;
    }

    // java public void setPanoseHeight(byte value)
    pub fn setPanoseHeight(&mut self, value: i8) {
        // java Assert.logTrue("PAN_ANY <= value && value <= PAN_XHEIGHT_DUCKING_LARGE", 0 <= value && value <= 7);
        Assert::logTrue_msg("PAN_ANY <= value && value <= PAN_XHEIGHT_DUCKING_LARGE", 0 <= value && value <= 7);
        // java this.mPanoseHeight = value;
        self.mPanoseHeight = value;
    }

    // java public byte[] getFontSignature()
    pub fn getFontSignature(&self) -> Option<Vec<i8>> {
        // java return this.mFontSignature;
        return self.mFontSignature.clone();
    }

    // java public void setFontSignature(byte[] bytes)
    pub fn setFontSignature(&mut self, bytes: Vec<i8>) {
        // java Assert.assertNotNull("bytes should not be null!", bytes);
        Assert::assertNotNull_msg("bytes should not be null!", &Some(bytes.clone()));
        // java Assert.assertTrue("24 == bytes.length should be true!", 24 == bytes.length);
        Assert::assertTrue_msg("24 == bytes.length should be true!", 24 == bytes.len() as i32);
        // java this.mFontSignature = bytes;
        self.mFontSignature = Some(bytes);
    }
}
