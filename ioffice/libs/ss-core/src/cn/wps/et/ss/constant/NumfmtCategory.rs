// java package cn.wps.et.ss.constant;
// java
// java import cn.wps.kfc.numfmt.format.NumFmtSection;
// java import cn.wps.kfc.numfmt.util.DateUtil;
// java import java.util.regex.Pattern;
// java
// java public enum NumfmtCategory {
// java NONE,
// java GENERAL,
// java NUMBER,
// java ACCOUNTING,
// java CURRENCY,
// java DATE,
// java TIME,
// java DATE_TIME,
// java PERCENTAGE,
// java FRACTION,
// java SCIENTIC,
// java TEXT;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum NumfmtCategory{NONE,GENERAL,NUMBER,ACCOUNTING,CURRENCY,DATE,TIME,DATE_TIME,PERCENTAGE,FRACTION,SCIENTIC,TEXT}

impl NumfmtCategory {
    // java public static NumfmtCategory getNumfmtCat(short numfmt, String fmtstr) {
    pub fn getNumfmtCat(numfmt: i16, fmtstr: &str) -> Self {
        use NumfmtCategory::*;
        // java switch (numfmt) {
        match numfmt {
            // java case 0: return GENERAL;
            0 => GENERAL,
            // java case 1: case 2: case 3: case 4: case 37: case 38: case 39: case 40: return NUMBER;
            1..=4 | 37..=40 => NUMBER,
            // java case 5: case 6: case 7: case 8: return CURRENCY;
            5..=8 => CURRENCY,
            // java case 9: case 10: return PERCENTAGE;
            9 | 10 => PERCENTAGE,
            // java case 11: case 48: return SCIENTIC;
            11 | 48 => SCIENTIC,
            // java case 12: case 13: return FRACTION;
            12 | 13 => FRACTION,
            // java case 14: case 15: case 16: case 17: return DATE;
            14..=17 => DATE,
            // java case 18: case 19: case 20: case 21: case 45: case 46: case 47: return TIME;
            18..=21 | 45..=47 => TIME,
            // java case 22: return DATE_TIME;
            22 => DATE_TIME,
            // java case 41: case 42: case 43: case 44: return ACCOUNTING;
            41..=44 => ACCOUNTING,
            // java case 49: return TEXT;
            49 => TEXT,
            // java default:
            _ => {
                // java NumFmtSection fmtSection = new NumFmtSection(); fmtSection.reset(fmtstr);
                let mut fmtSection = kfc::cn::wps::kfc::numfmt::format::NumFmtSection::NumFmtSection::new();
                fmtSection.reset(fmtstr);
                // java String positive = fmtSection.getPosCode();
                let positive = fmtSection.getPosCode();
                // java String negtive = fmtSection.getNegCode();
                let negtive = fmtSection.getNegCode();
                // java String zero = fmtSection.getZeroCode();
                let zero = fmtSection.getZeroCode();
                // java boolean emptynum = positive.length() == 0 && negtive.length() == 0 && zero.length() == 0;
                let emptynum = positive.is_empty() && negtive.is_empty() && zero.is_empty();
                // java if (fmtstr.endsWith("%")) { return PERCENTAGE; }
                if fmtstr.ends_with('%') { PERCENTAGE }
                // java else if (fmtstr.indexOf(64) <= -1 || fmtSection.getFmtCount() != 1 && !emptynum) {
                else if !fmtstr.contains('@') || fmtSection.getFmtCount() != 1 && !emptynum {
                    // java int t = DateUtil.isDateTimeFormat(numfmt, fmtstr);
                    let t = kfc::cn::wps::kfc::numfmt::util::DateUtil::DateUtil::isDateTimeFormat(numfmt as i32, fmtstr);
                    // java if (t == 1) { return DATE; }
                    if t == 1 { DATE }
                    // java else if (t == 2) { return TIME; }
                    else if t == 2 { TIME }
                    // java else if (t == 3) { return DATE_TIME; }
                    else if t == 3 { DATE_TIME }
                    // java else if (!fmtstr.equalsIgnoreCase("\"$\"#,##0_);[Red](\"$\"#,##0)") && !fmtstr.equalsIgnoreCase("￥#,##0;[Red]￥-#,##0") && !fmtstr.equalsIgnoreCase("\"$\"#,##0.00_);[Red](\"$\"#,##0.00)") && !fmtstr.equalsIgnoreCase("￥#,##0.00;[Red]￥-#,##0.00")) {
                    else if !["\"$\"#,##0_);[Red](\"$\"#,##0)", "￥#,##0;[Red]￥-#,##0", "\"$\"#,##0.00_);[Red](\"$\"#,##0.00)", "￥#,##0.00;[Red]￥-#,##0.00"].iter().any(|s| fmtstr.eq_ignore_ascii_case(s)) {
                        // java if (Pattern.matches(".*\\?/[\\?\\d].*", fmtstr)) { return FRACTION; }
                        // rust: Android Pattern dot excludes the five line terminators; \d uses Unicode decimal digits.
                        static FRACTION_PATTERN: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| crate::java_runtime::Pattern::compile(r"\A[^\n\r\x85\u{2028}\u{2029}]*\?/[?\d][^\n\r\x85\u{2028}\u{2029}]*\z").unwrap());
                        if FRACTION_PATTERN.is_match(fmtstr) { FRACTION }
                        else {
                            // java return !Pattern.matches("^.*(#,##)?(0\\.)(0){1,}.*", fmtstr) && !Pattern.matches("^.*(#,##)?0(_)?.*", fmtstr) ? NONE : NUMBER;
                            static NUMBER1: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| regex::Regex::new(r"\A[^\n\r\x85\u{2028}\u{2029}]*(#,##)?(0\.)(0)+[^\n\r\x85\u{2028}\u{2029}]*\z").unwrap());
                            static NUMBER2: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| regex::Regex::new(r"\A[^\n\r\x85\u{2028}\u{2029}]*(#,##)?0(_)?[^\n\r\x85\u{2028}\u{2029}]*\z").unwrap());
                            if !NUMBER1.is_match(fmtstr) && !NUMBER2.is_match(fmtstr) { NONE } else { NUMBER }
                        }
                    }
                    // java else { return CURRENCY; }
                    else { CURRENCY }
                }
                // java else { return TEXT; }
                else { TEXT }
            }
        }
    }
}
