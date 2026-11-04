#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

// Java: import cn.wps.kfc.numfmt.IWordFormatter
use crate::cn::wps::kfc::numfmt::IWordFormatter::IWordFormatter;
// Java: import cn.wps.kfc.numfmt.analyze.IWordFmtAnalyzer
use crate::cn::wps::kfc::numfmt::analyze::IWordFmtAnalyzer::IWordFmtAnalyzer;
// Java: import cn.wps.kfc.numfmt.analyze.WordFmtAnalyzer
use crate::cn::wps::kfc::numfmt::analyze::WordFmtAnalyzer;
// Java: import cn.wps.kfc.numfmt.exception.WordFmtException
use crate::cn::wps::kfc::numfmt::exception::WordFmtException;
// Java: import cn.wps.kfc.numfmt.format.NumFmtResult
use crate::cn::wps::kfc::numfmt::format::NumFmtResult;
// Java: import cn.wps.kfc.numfmt.format.WordFmtParam
use crate::cn::wps::kfc::numfmt::format::WordFmtParam;
// Java: import cn.wps.kfc.numfmt.format.type.IFormat
use crate::cn::wps::kfc::numfmt::format::r#type::IFormat;
// Java: import cn.wps.kfc.numfmt.global.GlobalEnv
use crate::cn::wps::kfc::numfmt::global::GlobalEnv;
// Java: import cn.wps.kfc.numfmt.numbering.Numberings
// use crate::cn::wps::kfc::numfmt::numbering::Numberings;
// Java: import cn.wps.kfc.numfmt.util.DateUtil
use crate::cn::wps::kfc::numfmt::util::DateUtil;
// Java: import java.util.Date
use crate::cn::wps::kfc::numfmt::IWPSFormatter::IWPSFormatter;

// Java: public class WordFormatter implements IWordFormatter
pub struct WordFormatter {
    // Java field: WordFmtCompiler mWordFmtCompiler
    mWordFmtCompiler: i32,
    // Java field: IWordFmtAnalyzer mWordFmtAnalyzer
    pub(crate) mWordFmtAnalyzer: i32,
    // Java field: WordFmtParam mNumFmtParam
    mNumFmtParam: i32,
    // Java field: NumFmtResult mNumFmtResult
    pub(crate) mNumFmtResult: i32,
    // Java field: Numberings mNumberings
    pub(crate) mNumberings: i32,
}

// Java field: String COMMA_SEPARATOR
pub(crate) const COMMA_SEPARATOR: &str = ",";
// Java field: String INVERTED_COMMA_SEPARATOR
pub(crate) const INVERTED_COMMA_SEPARATOR: &str = "\u{060C}";

// Java: public class WordFormatter implements IWordFormatter
impl WordFormatter {
    // Java: WordFormatter(short lcid)
    pub fn new(_lcid: i16) -> Self {
        WordFormatter {
            mWordFmtCompiler: 0,
            mWordFmtAnalyzer: 0,
            mNumFmtParam: 0,
            mNumFmtResult: 0,
            mNumberings: 0,
        }
    }

    // Java: static IWordFormatter createInstance(short lcid)
    pub fn createInstance(lcid: i16) -> Self {
        // Java: return new WordFormatter(lcid);
        Self::new(lcid)
    }

    // Java: void destroy()
    pub fn destroy_impl(&mut self) {
        // Java: this.mWordFmtCompiler = null;
        // Java: this.mNumFmtParam = null;
        // Java: this.mNumFmtResult = null;
        // Java: this.mNumberings = null;
        todo!("WordFormatter::destroy")
    }

    // Java: void resetLCID(short lcid)
    pub fn resetLCID_impl(&mut self, _lcid: i16) {
        // Java: GlobalEnv.setLCID(lcid);
        todo!("WordFormatter::resetLCID")
    }

    // Java: String formatDateTime(Date date, String formatCode, short language, byte calendar, boolean hasRTLFlag, short ssEastAsiaLID, short ssLatinLID, short ssBidiLID)
    pub fn formatDateTime_impl(&self, _date: i64, _format_code: &str, _language: i16, _calendar: i8, _has_rtl_flag: bool, _ss_east_asia_lid: i16, _ss_latin_lid: i16, _ss_bidi_lid: i16) -> String {
        // Java: this.mNumFmtParam.mHasRTLFlag = hasRTLFlag || language == 1025;
        // Java: this.mNumFmtParam.mDefEastAsiaLID = ssEastAsiaLID;
        // Java: this.mNumFmtParam.mDefLatinLID = ssLatinLID;
        // Java: this.mNumFmtParam.mDefBidiLID = ssBidiLID;
        // Java: formatCode = WordFmtAnalyzer.split(formatCode);
        // Java: double value = DateUtil.getExcelDate(date, false);
        // Java: IFormat formatter = this.mWordFmtCompiler.compile(value, formatCode, language, calendar);
        // Java: if (formatter != null)
        {
            // Java: formatter.format(value, this.mNumFmtParam, this.mNumFmtResult);
            // Java: this.correctResult(this.mNumFmtParam, this.mNumFmtResult);
            // Java: return this.mNumFmtResult.numfmtStr();
        }
        // Java: else
        {
            // Java: return formatCode;
        }
        todo!("WordFormatter::formatDateTime")
    }

    // Java: void correctResult(WordFmtParam fmtParam, NumFmtResult fmtResult)
    pub fn correctResult(&self, _fmt_param: &i32, _fmt_result: &mut i32) {
        // Java: if (fmtParam != null && fmtResult != null && fmtParam.mHasRTLFlag)
        {
            // Java: String oldSeparator = ",";
            // Java: String newSeparator = "،";
            // Java: fmtResult.correctResult(oldSeparator, newSeparator);
        }
        todo!("WordFormatter::correctResult")
    }

    // Java: String formatNumeric(double value, String formatCode)
    pub fn formatNumeric_impl(&self, _value: f64, _format_code: &str) -> String {
        // Java: formatCode = WordFmtAnalyzer.split(formatCode);
        // Java: IFormat formatter = this.mWordFmtCompiler.compile(value, formatCode);
        // Java: if (formatter != null)
        {
            // Java: formatter.format(value, this.mNumFmtParam, this.mNumFmtResult);
            // Java: return this.mNumFmtResult.numfmtStr();
        }
        // Java: else
        {
            // Java: return formatCode;
        }
        todo!("WordFormatter::formatNumeric")
    }

    // Java: String formatGeneral(Object value, int nfc)
    pub fn formatGeneral_impl(&self, _value: &dyn std::any::Any, _nfc: i32) -> String {
        // Java: if (!(value instanceof String))
        {
            // Java: if (value instanceof Long)
            {
                // Java: return this.mNumberings.getNumbering((Long)value, nfc);
            }
            // Java: if (!(value instanceof Date))
            {
                // Java: throw new WordFmtException("value is illegal");
            }
        }
        // Java: return null;
        todo!("WordFormatter::formatGeneral")
    }

    // Java: int analyze(String formatCode, short type)
    pub fn analyze_impl(&self, _format_code: &str, _type: i16) -> i32 {
        // Java: if (this.mWordFmtAnalyzer == null)
        {
            // Java: this.mWordFmtAnalyzer = new WordFmtAnalyzer();
        }
        // Java: return this.mWordFmtAnalyzer.analyze(formatCode, type);
        todo!("WordFormatter::analyze")
    }
}

impl IWPSFormatter for WordFormatter {
    fn destroy(&mut self) {
        self.destroy_impl()
    }

    fn resetLCID(&mut self, var1: i16) {
        self.resetLCID_impl(var1)
    }
}

impl IWordFmtAnalyzer for WordFormatter {
    fn analyze(&self, var1: &str, var2: i16) -> i32 {
        self.analyze_impl(var1, var2)
    }
}

impl IWordFormatter for WordFormatter {
    fn formatDateTime(&self, var1: i64, var2: &str, var3: i16, var4: i8, var5: bool, var6: i16, var7: i16, var8: i16) -> String {
        self.formatDateTime_impl(var1, var2, var3, var4, var5, var6, var7, var8)
    }

    fn formatNumeric(&self, var1: f64, var3: &str) -> String {
        self.formatNumeric_impl(var1, var3)
    }

    fn formatGeneral(&self, var1: &dyn std::any::Any, var2: i32) -> String {
        self.formatGeneral_impl(var1, var2)
    }
}
