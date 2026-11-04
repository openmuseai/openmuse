#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut, unused_assignments)]

// Java: import cn.wps.kfc.numfmt.global.GlobalEnv
use crate::cn::wps::kfc::numfmt::global::GlobalEnv;
// Java: import cn.wps.kfc.numfmt.util.NumFmtUtil
use crate::cn::wps::kfc::numfmt::util::NumFmtUtil;

// Java field: Section ARROBAS_SECTION
static ARROBAS_SECTION: std::sync::OnceLock<Section> = std::sync::OnceLock::new();
// Java field: char[] GENERAL
static GENERAL: &[char] = &['G', 'e', 'n', 'e', 'r', 'a', 'l'];
// Java field: Section GENERAL_SECTION
static GENERAL_SECTION: std::sync::OnceLock<Section> = std::sync::OnceLock::new();

// Java: public class NumFmtSection
pub struct NumFmtSection {
    // Java field: Section[] mSections
    pub(crate) mSections: Vec<Section>,
    // Java field: int mFmtCount
    pub(crate) mFmtCount: i32,
    // Java field: Section mPosSection
    pub(crate) mPosSection: Option<usize>,
    // Java field: Section mNegSection
    pub(crate) mNegSection: Option<usize>,
    // Java field: Section mZeroSection
    pub(crate) mZeroSection: Option<usize>,
    // Java field: Section mTextSection
    pub(crate) mTextSection: Option<usize>,
}

// Java: public class NumFmtSection
impl NumFmtSection {
    // Java: NumFmtSection()
    pub fn new() -> Self {
        // Java: super();
        NumFmtSection {
            mSections: vec![Section::new(), Section::new(), Section::new(), Section::new(),Section::new_with_code("General"),Section::new_with_code("@")],
            mFmtCount: 0,
            mPosSection: None,
            mNegSection: None,
            mZeroSection: None,
            mTextSection: None,
        }
    }

    // Java: final int getFmtCount()
    pub fn getFmtCount(&self) -> i32 {
        // Java: return this.mFmtCount;
        self.mFmtCount
    }

    // Java: final boolean hasCondition()
    pub fn hasCondition(&self) -> bool {
        // Java: return this.mPosSection.mHasCondition || this.mNegSection.mHasCondition;
        let pos_cond = self.mPosSection.map_or(false, |idx| self.mSections[idx].mHasCondition);
        let neg_cond = self.mNegSection.map_or(false, |idx| self.mSections[idx].mHasCondition);
        pos_cond || neg_cond
    }

    // Java: String getPosCode()
    pub fn getPosCode(&self) -> &str {
        // Java: assert this.mPosSection != null;
        debug_assert!(self.mPosSection.is_some());
        // Java: return this.mPosSection.mFormatCode;
        &self.mSections[self.mPosSection.unwrap()].mFormatCode
    }

    // Java: String getNegCode()
    pub fn getNegCode(&self) -> &str {
        // Java: assert this.mNegSection != null;
        debug_assert!(self.mNegSection.is_some());
        // Java: return this.mNegSection.mFormatCode;
        &self.mSections[self.mNegSection.unwrap()].mFormatCode
    }

    // Java: String getZeroCode()
    pub fn getZeroCode(&self) -> &str {
        // Java: assert this.mZeroSection != null;
        debug_assert!(self.mZeroSection.is_some());
        // Java: return this.mZeroSection.mFormatCode;
        &self.mSections[self.mZeroSection.unwrap()].mFormatCode
    }

    // Java: String getTextCode()
    pub fn getTextCode(&self) -> &str {
        // Java: assert this.mTextSection != null;
        debug_assert!(self.mTextSection.is_some());
        // Java: return this.mTextSection.mFormatCode;
        &self.mSections[self.mTextSection.unwrap()].mFormatCode
    }

    // Java: String getFormatSection(String value)
    pub fn getFormatSection_str(&self, _value: &str) -> &str {
        // Java: assert this.mTextSection != null;
        debug_assert!(self.mTextSection.is_some());
        // Java: return this.mTextSection.mFormatCode;
        &self.mSections[self.mTextSection.unwrap()].mFormatCode
    }

    // Java: String getFormatSection(double value)
    pub fn getFormatSection_double(&self, value: f64) -> String {
        // Java: Section section = this.extractSection(value);
        let section = self.extractSection(value);
        // Java: if (section == null)
        if section.is_none() {
            // Java: return NumFmtUtil.invalidNumberSignString;
            return "#".repeat(255);
        }
        // Java: else if (section.mHasArrobas)
        else if self.mSections[section.unwrap()].mHasArrobas {
            // Java: return section.mFormatCode;
            return self.mSections[section.unwrap()].mFormatCode.clone();
        }
        // Java: else if (section.mHasF400)
        else if self.mSections[section.unwrap()].mHasF400 {
            // Java: return GlobalEnv.localeStrManager.getF4800()[0];
            return GlobalEnv::localeStrManager.lock().unwrap_or_else(std::sync::PoisonError::into_inner).getF4800()[0].clone();
        }
        // Java: else
        else {
            // Java: return section.mHasF800 ? GlobalEnv.localeStrManager.getF4800()[1] : section.mFormatCode;
            if self.mSections[section.unwrap()].mHasF800 {
                return GlobalEnv::localeStrManager.lock().unwrap_or_else(std::sync::PoisonError::into_inner).getF4800()[1].clone();
            } else {
                return self.mSections[section.unwrap()].mFormatCode.clone();
            }
        }
    }

    // Java: Section extractSection(double value)
    fn extractSection(&self, value: f64) -> Option<usize> {
        // Java: FormatCondition condition = null;
        // Java: boolean enterPosCond = false;
        let mut enterPosCond = false;
        // Java: boolean enterNegCond = false;
        let mut enterNegCond = false;
        // Java: boolean enterZeroCond = false;
        let mut enterZeroCond = false;
        // Java: if (this.mPosSection.mHasCondition)
        if self.mPosSection.map_or(false, |idx| self.mSections[idx].mHasCondition) {
            // Java: condition = FormatCondition.getInstance(this.mPosSection.mOperator, this.mPosSection.mConstant);
            // Java: if (condition.pass(value))
            if {let section=&self.mSections[self.mPosSection.unwrap()]; super::FormatCondition::FormatCondition::getInstance(section.mOperator.as_deref().unwrap(),section.mConstant.as_deref().unwrap()).pass(value)} {
                // Java: return this.mPosSection;
                return self.mPosSection;
            }
            // Java: enterPosCond = true;
            enterPosCond = true;
        }
        // Java: else if (value > (double)0.0F)
        else if value > 0.0f64 {
            // Java: return this.mPosSection;
            return self.mPosSection;
        }
        // Java: if (this.mNegSection.mHasCondition)
        if self.mNegSection.map_or(false, |idx| self.mSections[idx].mHasCondition) {
            // Java: condition = FormatCondition.getInstance(this.mNegSection.mOperator, this.mNegSection.mConstant);
            // Java: if (condition.pass(value))
            if {let section=&self.mSections[self.mNegSection.unwrap()]; super::FormatCondition::FormatCondition::getInstance(section.mOperator.as_deref().unwrap(),section.mConstant.as_deref().unwrap()).pass(value)} {
                // Java: return this.mNegSection;
                return self.mNegSection;
            }
            // Java: enterNegCond = true;
            enterNegCond = true;
        }
        // Java: else if (value < (double)0.0F)
        else if value < 0.0f64 {
            // Java: return this.mNegSection;
            return self.mNegSection;
        }
        // Java: if (this.mZeroSection.mHasCondition)
        if self.mZeroSection.map_or(false, |idx| self.mSections[idx].mHasCondition) {
            // Java: condition = FormatCondition.getInstance(this.mZeroSection.mOperator, this.mZeroSection.mConstant);
            // Java: if (condition.pass(value))
            if {let section=&self.mSections[self.mZeroSection.unwrap()]; super::FormatCondition::FormatCondition::getInstance(section.mOperator.as_deref().unwrap(),section.mConstant.as_deref().unwrap()).pass(value)} {
                // Java: return this.mZeroSection;
                return self.mZeroSection;
            }
            // Java: enterZeroCond = true;
            enterZeroCond = true;
        }
        // Java: else if (value == (double)0.0F && (!enterNegCond || this.mFmtCount > 2))
        else if value == 0.0f64 && (!enterNegCond || self.mFmtCount > 2) {
            // Java: return this.mZeroSection;
            return self.mZeroSection;
        }
        // Java: if (!enterZeroCond && this.mFmtCount > 2)
        if !enterZeroCond && self.mFmtCount > 2 {
            // Java: return this.mZeroSection;
            return self.mZeroSection;
        }
        // Java: else if (enterPosCond && !enterNegCond)
        else if enterPosCond && !enterNegCond {
            // Java: return this.mNegSection;
            return self.mNegSection;
        }
        // Java: else
        else {
            // Java: return null;
            return None;
        }
    }

    // Java: boolean reset(String formatCode)
    pub fn reset(&mut self, formatCode: &str) -> bool {
        // Java: this.mFmtCount = this.collect(formatCode);
        self.mFmtCount = self.collect(formatCode);
        // Java: if (this.mFmtCount < 1)
        if self.mFmtCount < 1 {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: boolean hasArrobas = false;
            let mut hasArrobas = false;
            // Java: switch (this.mFmtCount)
            match self.mFmtCount {
                // Java: case 1:
                1 => {
                    // Java: this.mPosSection = this.mZeroSection = this.mNegSection = this.mSections[0];
                    self.mPosSection = Some(0);
                    self.mZeroSection = Some(0);
                    self.mNegSection = Some(0);
                    // Java: if (this.mPosSection.mHasCondition && !this.mPosSection.isPureGeneral())
                    if self.mSections[0].mHasCondition && !self.mSections[0].isPureGeneral() {
                        // Java: this.mZeroSection = this.mNegSection = GENERAL_SECTION;
                        self.mZeroSection = Some(4);
                        self.mNegSection = Some(4);
                    }
                    // Java: this.mTextSection = !this.mPosSection.mHasArrobas && !this.mPosSection.isPureGeneral() ? ARROBAS_SECTION : this.mPosSection;
                    if !self.mSections[0].mHasArrobas && !self.mSections[0].isPureGeneral() {
                        self.mTextSection = Some(5);
                    } else {
                        self.mTextSection = Some(0);
                    }
                    // Java: break;
                    // LLM Rust match: implicit break at end of arm
                }
                // Java: case 2:
                2 => {
                    // Java: this.mPosSection = this.mZeroSection = this.mSections[0];
                    self.mPosSection = Some(0);
                    self.mZeroSection = Some(0);
                    // Java: hasArrobas = this.mSections[1].mHasArrobas;
                    hasArrobas = self.mSections[1].mHasArrobas;
                    // Java: this.mNegSection = hasArrobas ? this.mPosSection : this.mSections[1];
                    self.mNegSection = if hasArrobas { Some(0) } else { Some(1) };
                    // Java: this.mTextSection = hasArrobas ? this.mSections[1] : ARROBAS_SECTION;
                    self.mTextSection = if hasArrobas { Some(1) } else { Some(5) };
                    // Java: if (!hasArrobas && this.mPosSection.isPureGeneral())
                    if !hasArrobas && self.mSections[0].isPureGeneral() {
                        // Java: this.mTextSection = this.mPosSection;
                        self.mTextSection = Some(0);
                    }
                    // Java: break;
                    // LLM Rust match: implicit break at end of arm
                }
                // Java: case 3:
                3 => {
                    // Java: this.mPosSection = this.mSections[0];
                    self.mPosSection = Some(0);
                    // Java: this.mNegSection = this.mSections[1];
                    self.mNegSection = Some(1);
                    // Java: hasArrobas = this.mSections[2].mHasArrobas;
                    hasArrobas = self.mSections[2].mHasArrobas;
                    // Java: this.mZeroSection = hasArrobas ? this.mPosSection : this.mSections[2];
                    self.mZeroSection = if hasArrobas { Some(0) } else { Some(2) };
                    // Java: this.mTextSection = hasArrobas ? this.mSections[2] : ARROBAS_SECTION;
                    self.mTextSection = if hasArrobas { Some(2) } else { Some(5) };
                    // Java: if (!hasArrobas && this.mPosSection.isPureGeneral())
                    if !hasArrobas && self.mSections[0].isPureGeneral() {
                        // Java: this.mTextSection = this.mPosSection;
                        self.mTextSection = Some(0);
                    }
                    // Java: break;
                    // LLM Rust match: implicit break at end of arm
                }
                // Java: case 4:
                4 => {
                    // Java: this.mPosSection = this.mSections[0];
                    self.mPosSection = Some(0);
                    // Java: this.mNegSection = this.mSections[1];
                    self.mNegSection = Some(1);
                    // Java: this.mZeroSection = this.mSections[2];
                    self.mZeroSection = Some(2);
                    // Java: this.mTextSection = this.mSections[3];
                    self.mTextSection = Some(3);
                }
                // Java: default:
                _ => {}
            }
            // Java: return true;
            return true;
        }
    }

    // Java: int collect(String formatCode)
    fn collect(&mut self, formatCode: &str) -> i32 {
        // Java: int len = formatCode.length();
        let len = formatCode.len();
        // Java: int start = 0;
        let mut start: usize = 0;
        // Java: int end = 0;
        let mut end: usize = 0;
        // Java: int groupIdx = 0;
        let mut groupIdx: usize = 0;
        // Java: boolean isInSquare = false;
        let mut isInSquare = false;
        // Java: boolean isInRef = false;
        let mut isInRef = false;
        // Java: boolean isEscape = false;
        let mut isEscape = false;
        // Java: int squareStart = 0;
        let mut squareStart: usize = 0;
        // Java: int squareEnd = 0;
        let mut squareEnd: usize = 0;
        // Java: int i = 0;
        let mut i: usize = 0;
        // Java: for(int size = this.mSections.length; i < size; ++i)
        let size = 4;
        for i in 0..size {
            // Java: this.mSections[i].clear();
            self.mSections[i].clear();
        }
        let chars: Vec<char> = formatCode.chars().collect();
        // Java: for(i = 0; i < len && groupIdx < 4; ++i)
        i = 0;
        while i < chars.len() && groupIdx < 4 {
            // Java: char c = formatCode.charAt(i);
            let c = chars[i];
            // Java: boolean isMetaChar = !isInRef && !isEscape && !isInSquare;
            let isMetaChar = !isInRef && !isEscape && !isInSquare;
            // Java: switch (c)
            match c {
                    // Java: case '!':
                    // LLM Rust match: combined with case '\\' into '!' | '\\' arm
                    // Java: case '\\':
                    '!' | '\\' => {
                        // Java: isEscape = isMetaChar;
                        isEscape = isMetaChar;
                        // Java: break;
                        // LLM Rust match: implicit break at end of arm
                    }
                    // Java: case '"':
                    '"' => {
                        // Java: if (!isEscape)
                        if !isEscape {
                            // Java: isInRef = !isInRef;
                            isInRef = !isInRef;
                        }
                        // Java: break;
                        // LLM Rust match: implicit break at end of arm
                    }
                    // Java: case '*':
                    // LLM Rust match: combined with case '_' into '*' | '_' arm
                    // Java: case '_':
                    '*' | '_' => {
                        // Java: isEscape = isMetaChar;
                        isEscape = isMetaChar;
                        // Java: break;
                        // LLM Rust match: implicit break at end of arm
                    }
                    // Java: case ';':
                    ';' => {
                    // Java: if (isMetaChar)
                    if isMetaChar {
                        // Java: this.mSections[groupIdx].mFormatCode = formatCode.substring(start, i);
                        self.mSections[groupIdx].mFormatCode = chars[start..i].iter().collect();
                        // Java: start = i + 1;
                        start = i + 1;
                        // Java: ++groupIdx;
                        groupIdx += 1;
                    }
                    // Java: break;
                    // LLM Rust match: implicit break at end of arm
                }
                // Java: case '@':
                '@' => {
                    // Java: this.mSections[groupIdx].mHasArrobas = isMetaChar;
                    self.mSections[groupIdx].mHasArrobas = isMetaChar;
                    // Java: break;
                    // LLM Rust match: implicit break at end of arm
                }
                // Java: case '[':
                '[' => {
                    // Java: isInSquare = !isEscape && !isInRef;
                    isInSquare = !isEscape && !isInRef;
                    // Java: squareStart = isInSquare ? i : 0;
                    squareStart = if isInSquare { i } else { 0 };
                    // Java: break;
                    // LLM Rust match: implicit break at end of arm
                }
                // Java: case ']':
                ']' => {
                    // Java: if (!isEscape && !isInRef)
                    if !isEscape && !isInRef {
                        // Java: squareEnd = isInSquare ? i : 0;
                        squareEnd = if isInSquare { i } else { 0 };
                        // Java: if (squareStart < squareEnd)
                        if squareStart < squareEnd {
                            // Java: this.mSections[groupIdx].extractCond(formatCode, squareStart + 1, squareEnd);
                            let src_str: String = chars.iter().collect();
                            self.mSections[groupIdx].extractCond(&src_str, squareStart + 1, squareEnd);
                        }
                        // Java: squareEnd = 0;
                        squareEnd = 0;
                        // Java: squareStart = 0;
                        squareStart = 0;
                        // Java: isInSquare = false;
                        isInSquare = false;
                    }
                }
                // Java: default:
                _ => {}
            }
            // Java: if (c != '!' && c != '\\' && c != '*' && c != '_')
            if c != '!' && c != '\\' && c != '*' && c != '_' {
                // Java: isEscape = false;
                isEscape = false;
            }
            i += 1;
        }
        // Java: if (start <= len && groupIdx < 4)
        if start <= chars.len() && groupIdx < 4 {
            // Java: this.mSections[groupIdx].mFormatCode = start < len ? formatCode.substring(start, len) : "";
            self.mSections[groupIdx].mFormatCode = if start < chars.len() {
                chars[start..chars.len()].iter().collect()
            } else {
                String::new()
            };
            // Java: ++groupIdx;
            groupIdx += 1;
        }
        // Java: return groupIdx;
        groupIdx as i32
    }

    // Java: static short splitGroup(String formatCode, String[] groups)
    pub fn splitGroup(formatCode: &str, groups: &mut [Option<String>; 4]) -> i16 {
        // Java: if (formatCode.indexOf(";") == -1 && formatCode.indexOf("；") == -1)
        if !formatCode.contains(";") && !formatCode.contains("；") {
            // Java: groups[0] = formatCode;
            groups[0] = Some(formatCode.to_string());
            // Java: return 1;
            return 1;
        }
        // Java: else
        else {
            // Java: int len = formatCode.length();
            let chars: Vec<char> = formatCode.chars().collect();
            let len = chars.len();
            // Java: int start = 0;
            let mut start: usize = 0;
            // Java: int end = 0;
            let mut end: usize = 0;
            // Java: short groupIdx = 0;
            let mut groupIdx: i16 = 0;
            // Java: boolean isInSquare = false;
            let mut isInSquare = false;
            // Java: boolean isInRef = false;
            let mut isInRef = false;
            // Java: boolean isEscape = false;
            let mut isEscape = false;
            // Java: for(int i = 0; i < len && groupIdx < 4; ++i)
            let mut i: usize = 0;
            while i < len && groupIdx < 4 {
                // Java: char c = formatCode.charAt(i);
                let c = chars[i];
                // Java: boolean isMetaChar = !isInRef && !isEscape && !isInSquare;
                let isMetaChar = !isInRef && !isEscape && !isInSquare;
                // Java: switch (c)
                match c {
                    // Java: case '!':
                    // LLM Rust match: combined with case '\\' into '!' | '\\' arm
                    // Java: case '\\':
                    '!' | '\\' => {
                        // Java: isEscape = isMetaChar;
                        isEscape = isMetaChar;
                        // Java: break;
                    // LLM Rust match: implicit break at end of arm
                    }
                    // Java: case '"':
                    '"' => {
                        // Java: if (!isEscape)
                        if !isEscape {
                            // Java: isInRef = !isInRef;
                            isInRef = !isInRef;
                        }
                        // Java: break;
                    // LLM Rust match: implicit break at end of arm
                    }
                    // Java: case '*':
                    // LLM Rust match: combined with case '_' into '*' | '_' arm
                    // Java: case '_':
                    '*' | '_' => {
                        // Java: isEscape = isMetaChar;
                        isEscape = isMetaChar;
                        // Java: break;
                    // LLM Rust match: implicit break at end of arm
                    }
                    // Java: case ';':
                    // LLM Rust match: combined with case '；' into ';' | '；' arm
                    // Java: case '；':
                    ';' | '\u{FF1B}' => {
                        // Java: if (isMetaChar)
                        if isMetaChar {
                            // Java: groups[groupIdx] = formatCode.substring(start, i);
                            groups[groupIdx as usize] = Some(chars[start..i].iter().collect());
                            // Java: start = i + 1;
                            start = i + 1;
                            // Java: ++groupIdx;
                            groupIdx += 1;
                        }
                        // Java: break;
                    // LLM Rust match: implicit break at end of arm
                    }
                    // Java: case '[':
                    '[' => {
                        // Java: isInSquare = !isEscape && !isInRef;
                        isInSquare = !isEscape && !isInRef;
                        // Java: break;
                    // LLM Rust match: implicit break at end of arm
                    }
                    // Java: case ']':
                    ']' => {
                        // Java: if (!isEscape && !isInRef)
                        if !isEscape && !isInRef {
                            // Java: isInSquare = false;
                            isInSquare = false;
                        }
                    }
                    // Java: default:
                    _ => {}
                }
                // Java: if (c != '!' && c != '\\' && c != '*' && c != '_')
                if c != '!' && c != '\\' && c != '*' && c != '_' {
                    // Java: isEscape = false;
                    isEscape = false;
                }
                i += 1;
            }
            // Java: if (start <= len && groupIdx < 4)
            if start <= len && (groupIdx as usize) < 4 {
                // Java: groups[groupIdx] = start < len ? formatCode.substring(start, len) : "";
                groups[groupIdx as usize] = Some(if start < len {
                    chars[start..len].iter().collect()
                } else {
                    String::new()
                });
                // Java: ++groupIdx;
                groupIdx += 1;
            }
            // Java: return groupIdx;
            return groupIdx;
        }
    }
}

// Java: public class Section
pub struct Section {
    // Java field: char[] F400
    // LLM Java inner class: static arrays moved to module-level constants below
    // Java field: char[] F800
    // LLM Java inner class: static arrays moved to module-level constants below
    // Java field: String mFormatCode
    pub(crate) mFormatCode: String,
    // Java field: String mOperator
    pub(crate) mOperator: Option<String>,
    // Java field: String mConstant
    pub(crate) mConstant: Option<String>,
    // Java field: boolean mHasCondition
    pub(crate) mHasCondition: bool,
    // Java field: boolean mHasArrobas
    pub(crate) mHasArrobas: bool,
    // Java field: boolean mHasF400
    pub(crate) mHasF400: bool,
    // Java field: boolean mHasF800
    pub(crate) mHasF800: bool,
    // Java field: boolean mHasDollar
    pub(crate) mHasDollar: bool,
}

static F400: &[char] = &['\u{F400}', '\u{F401}', '\u{F402}', '\u{F403}', '\u{F404}'];
static F800: &[char] = &['\u{F800}', '\u{F801}', '\u{F802}', '\u{F803}', '\u{F804}'];

// Java: public class Section
impl Section {
    // Java: Section()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.mFormatCode = "";
        // LLM Rust Constructor: assigned via struct literal
        Section {
            mFormatCode: String::new(),
            mOperator: None,
            mConstant: None,
            mHasCondition: false,
            mHasArrobas: false,
            mHasF400: false,
            mHasF800: false,
            mHasDollar: false,
        }
    }

    // Java: Section(String formatCode)
    pub fn new_with_code(formatCode: &str) -> Self {
        // Java: super();
        // Java: this.mFormatCode = formatCode;
        // LLM Rust Constructor: assigned via struct literal
        Section {
            mFormatCode: formatCode.to_string(),
            mOperator: None,
            mConstant: None,
            mHasCondition: false,
            mHasArrobas: false,
            mHasF400: false,
            mHasF800: false,
            mHasDollar: false,
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mFormatCode = "";
        self.mFormatCode = String::new();
        // Java: this.mOperator = null;
        self.mOperator = None;
        // Java: this.mConstant = null;
        self.mConstant = None;
        // Java: this.mHasCondition = false;
        self.mHasCondition = false;
        // Java: this.mHasArrobas = false;
        self.mHasArrobas = false;
        // Java: this.mHasF400 = false;
        self.mHasF400 = false;
        // Java: this.mHasF800 = false;
        self.mHasF800 = false;
        // Java: this.mHasDollar = false;
        self.mHasDollar = false;
    }

    // Java: boolean isPureGeneral()
    pub fn isPureGeneral(&self) -> bool {
        // Java: if (!this.mHasDollar && !this.mHasArrobas)
        if !self.mHasDollar && !self.mHasArrobas {
            // Java: int index = this.mFormatCode.lastIndexOf(93);
            let index = self.mFormatCode.chars().collect::<Vec<_>>().iter().rposition(|c|*c==']').map(|i|i as i32).unwrap_or(-1);
            // Java: int cursor = 0;
            let mut cursor: usize = 0;
            // Java: int len = this.mFormatCode.length();
            let chars: Vec<char> = self.mFormatCode.chars().collect();
            let len = chars.len() as i32;
            // Java: if (len - (index + 1) != NumFmtSection.GENERAL.length)
            if (len - (index + 1)) != GENERAL.len() as i32 {
                // Java: return false;
                return false;
            }
            // Java: else
            else {
                // Java: for(int i = index + 1; i < len; ++i)
                let mut i = (index + 1) as usize;
                while i < len as usize {
                    // Java: if (this.mFormatCode.charAt(i) != NumFmtSection.GENERAL[cursor++])
                    if chars[i] != GENERAL[cursor] {
                        // Java: return false;
                        return false;
                    }
                    cursor += 1;
                    i += 1;
                }
                // Java: return true;
                return true;
            }
        }
        // Java: else
        else {
            // Java: return false;
            return false;
        }
    }

    // Java: void extractCond(String src, int start, int end)
    pub fn extractCond(&mut self, src: &str, start: usize, end: usize) {
        let chars: Vec<char> = src.chars().collect();
        // Java: char c = src.charAt(start);
        let mut c = chars[start];
        // Java: switch (c)
        match c {
            // Java: case '$':
            '$' => {
                // Java: this.mHasDollar = true;
                self.mHasDollar = true;
                // Java: boolean f400Flag = false;
                let mut f400Flag = false;
                // Java: int index = F400.length - 1;
                let mut index = F400.len() as i32 - 1;
                // Java: if (end - start > 5)
                if end - start > 5 {
                    // Java: for(int i = end - 1; i > start + 1 && index >= 0; --i)
                    let mut i = end - 1;
                    while i > start + 1 && index >= 0 {
                        // Java: c = src.charAt(i);
                        c = chars[i];
                        // Java: if (c != F400[index] && c != F800[index])
                        if c != F400[index as usize] && c != F800[index as usize] {
                            // Java: break;
                    // LLM Rust match: implicit break at end of arm
                            break;
                        }
                        // Java: if (index == 1)
                        if index == 1 {
                            // Java: f400Flag = c == F400[1];
                            f400Flag = c == F400[1];
                        }
                        // Java: --index;
                        index -= 1;
                        i -= 1;
                    }
                    // Java: if (index < 0)
                    if index < 0 {
                        // Java: this.mHasF400 = f400Flag;
                        self.mHasF400 = f400Flag;
                        // Java: this.mHasF800 = !f400Flag;
                        self.mHasF800 = !f400Flag;
                    }
                }
                // Java: break;
                    // LLM Rust match: implicit break at end of arm
            }
            // Java: case '<':
            // LLM Rust match: combined with case '=' and '>' into '<' | '=' | '>' arm
            // Java: case '=':
            // LLM Rust match: combined into '<' | '=' | '>' arm
            // Java: case '>':
            '<' | '=' | '>' => {
                // Java: // int index;
                let mut index = end as i32 - 1;
                // Java: for(index = end - 1; index >= start; --index)
                while index >= start as i32 {
                    // Java: c = src.charAt(index);
                    c = chars[index as usize];
                    // Java: if (c == '=' || c == '<' || c == '>')
                    if c == '=' || c == '<' || c == '>' {
                        // Java: break;
                    // LLM Rust match: implicit break at end of arm
                        break;
                    }
                    index -= 1;
                }
                // Java: assert index > -1;
                debug_assert!(index > -1);
                // Java: this.mHasCondition = true;
                self.mHasCondition = true;
                // Java: this.mOperator = src.substring(start, index + 1);
                self.mOperator = Some(chars[start..(index + 1) as usize].iter().collect());
                // Java: this.mConstant = src.substring(index + 1, end);
                self.mConstant = Some(chars[(index + 1) as usize..end].iter().collect());
            }
            // Java: default:
            _ => {}
        }
    }
}
impl NumFmtSection {
    // Rust identity adapter for Java's singleton invalidNumberSignString result.
    pub(crate) fn isMissingSection(&self,value:f64)->bool{self.extractSection(value).is_none()}
}
