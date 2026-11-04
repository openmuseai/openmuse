#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

// Java: import cn.wps.kfc.numfmt.format.NumFmtResult
use crate::cn::wps::kfc::numfmt::format::NumFmtResult;
// Java: import cn.wps.kfc.numfmt.format.WordFmtParam
use crate::cn::wps::kfc::numfmt::format::WordFmtParam;
// Java: import cn.wps.kfc.numfmt.format.WordFmtStyle
use crate::cn::wps::kfc::numfmt::format::WordFmtStyle;

// Java: public class WordFmtDateTime extends NumFmtDateTime
pub struct WordFmtDateTime {
}

// Java: public class WordFmtDateTime extends NumFmtDateTime
impl WordFmtDateTime {
    // Java: WordFmtDateTime()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.mNumFmtStyle = new WordFmtStyle();
        WordFmtDateTime {
        }
    }

    // Java: void doFormat(NumFmtResult result)
    pub fn doFormat(&mut self, _result: &mut i32) {
        // Java: WordFmtParam wdFmtParam = (WordFmtParam)this.mNumFmtParam;
        // Java: WordFmtStyle wdFmtStyle = (WordFmtStyle)this.mNumFmtStyle;
        // Java: wdFmtStyle.mHasRTLFlag = wdFmtParam.mHasRTLFlag;
        // Java: wdFmtStyle.mDefEastAsiaLID = wdFmtParam.mDefEastAsiaLID;
        // Java: wdFmtStyle.mDefLatinLID = wdFmtParam.mDefLatinLID;
        // Java: wdFmtStyle.mDefBidiLID = wdFmtParam.mDefLatinLID;
        // Java: super.doFormat(result);
        todo!("WordFmtDateTime::doFormat")
    }
}
