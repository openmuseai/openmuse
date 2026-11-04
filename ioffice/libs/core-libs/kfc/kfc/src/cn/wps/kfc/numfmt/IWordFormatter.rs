#![allow(non_snake_case)]
#![allow(dead_code)]

// Java: import cn.wps.kfc.numfmt.analyze.IWordFmtAnalyzer
use super::analyze::IWordFmtAnalyzer::IWordFmtAnalyzer;
use super::IWPSFormatter::IWPSFormatter;
// Java: import java.util.Date
// chrono or std::time can be used for dates

// Java: public interface IWordFormatter extends IWPSFormatter
pub trait IWordFormatter: IWPSFormatter + IWordFmtAnalyzer {
    // Java: abstract String formatDateTime(Date var1, String var2, short var3, byte var4, boolean var5, short var6, short var7, short var8)
    fn formatDateTime(&self, var1: i64, var2: &str, var3: i16, var4: i8, var5: bool, var6: i16, var7: i16, var8: i16) -> String;

    // Java: abstract String formatNumeric(double var1, String var3)
    fn formatNumeric(&self, var1: f64, var3: &str) -> String;

    // Java: abstract String formatGeneral(Object var1, int var2)
    fn formatGeneral(&self, var1: &dyn std::any::Any, var2: i32) -> String;

}
