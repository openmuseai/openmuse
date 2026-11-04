#![allow(non_snake_case)]
#![allow(dead_code)]

// Java: import cn.wps.kfc.numfmt.format.NumFmtResult
use super::format::NumFmtResult::NumFmtResult;
use super::IWPSFormatter::IWPSFormatter as IWPSFormatterTrait;

// Java: public interface INumFormatter extends IWPSFormatter
pub trait INumFormatterTrait: IWPSFormatterTrait {
    // Java: abstract void format(String var1, String var2, int var3, NumFmtResult var4)
    fn format_str(&mut self, var1: &str, var2: &str, var3: i32, var4: &mut NumFmtResult);

    // Java: abstract void format(double var1, String var3, int var4, boolean var5, NumFmtResult var6)
    fn format_num(&mut self, var1: f64, var3: &str, var4: i32, var5: bool, var6: &mut NumFmtResult);

    // Java: abstract double formatValue(double var1, boolean var3, String var4)
    fn formatValue(&self, var1: f64, var3: bool, var4: &str) -> f64;

    // Java: abstract String compile(String var1)
    fn compile(&self, var1: &str) -> Option<String>;

    // Java: abstract String decompile(String var1)
    fn decompile(&self, var1: &str) -> String;

}

// Java: extends IWPSFormatter
pub struct IWPSFormatter;

pub struct INumFormatter {
    pub parent: IWPSFormatter,
}

impl std::ops::Deref for INumFormatter {
    type Target = IWPSFormatter;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
