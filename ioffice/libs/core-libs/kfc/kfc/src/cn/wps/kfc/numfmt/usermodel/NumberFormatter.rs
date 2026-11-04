use super::{NumFmtAnalyzer::NumFmtAnalyzer, NumFmtCompiler::NumFmtCompiler};
use crate::cn::wps::kfc::numfmt::{
    format::{r#type::IFormat::FormatValue, NumFmtParam::NumFmtParam, NumFmtResult::NumFmtResult},
    global::GlobalEnv::GlobalEnv,
    INumFormatter::INumFormatterTrait,
    IWPSFormatter::IWPSFormatter,
};
use std::{
    cell::{RefCell, RefMut},
    sync::Mutex,
};
// java private static final Object sLock=new Object();
static S_LOCK: Mutex<()> = Mutex::new(());
// java public class NumberFormatter implements INumFormatter
pub struct NumberFormatter {
    mNumFmtCompiler: RefCell<Option<NumFmtCompiler>>,
    mNumFmtAnalyzer: RefCell<Option<NumFmtAnalyzer>>,
    mNumFmtParam: RefCell<Option<NumFmtParam>>,
}
impl NumberFormatter {
    // rust: fields initialize compiler/param;constructor resetLCID(lcid).
    pub fn new(lcid: i16) -> Self {
        let mut f = Self {
            mNumFmtCompiler: RefCell::new(Some(NumFmtCompiler::new())),
            mNumFmtAnalyzer: RefCell::new(None),
            mNumFmtParam: RefCell::new(Some(NumFmtParam::new())),
        };
        f.resetLCID(lcid);
        f
    }
    // java return new NumberFormatter(lcid);

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumberFormatter.java:15
    // java public static INumFormatter createInstance(short lcid) {
    // java return new NumberFormatter(lcid);
    // java }
    pub fn createInstance(lcid: i16) -> Box<dyn INumFormatterTrait> {
        Box::new(Self::new(lcid))
    }
    // rust: lazy analyzer;resetDic(GlobalEnv.getDefaultLCID());return mNumFmtAnalyzer;

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumberFormatter.java:77
    // java private NumFmtAnalyzer getFmtAnalyzer() {
    // java if (this.mNumFmtAnalyzer == null) {
    // java this.mNumFmtAnalyzer = new NumFmtAnalyzer();
    // java }
    // java
    // java this.mNumFmtAnalyzer.resetDic(GlobalEnv.getDefaultLCID());
    // java return this.mNumFmtAnalyzer;
    // java }
    pub fn getFmtAnalyzer(&self) -> RefMut<'_, NumFmtAnalyzer> {
        let mut analyzer = self.mNumFmtAnalyzer.borrow_mut();
        if analyzer.is_none() {
            *analyzer = Some(NumFmtAnalyzer::new());
        }
        analyzer
            .as_mut()
            .unwrap()
            .resetDic(GlobalEnv::getDefaultLCID());
        RefMut::map(analyzer, |x| x.as_mut().unwrap())
    }
}
impl IWPSFormatter for NumberFormatter {
    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumberFormatter.java:19
    // java public void destroy() {
    // java this.mNumFmtCompiler = null;
    // java this.mNumFmtAnalyzer = null;
    // java this.mNumFmtParam = null;
    // java }
    fn destroy(&mut self) {
        *self.mNumFmtCompiler.borrow_mut() = None;
        *self.mNumFmtAnalyzer.borrow_mut() = None;
        *self.mNumFmtParam.borrow_mut() = None;
    }
    // rust: if(GlobalEnv.setLCID(lcid)&&mNumFmtAnalyzer!=null)mNumFmtAnalyzer.resetDic(lcid);

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumberFormatter.java:30
    // java public void resetLCID(short lcid) {
    // java if (GlobalEnv.setLCID(lcid) && this.mNumFmtAnalyzer != null) {
    // java this.mNumFmtAnalyzer.resetDic(lcid);
    // java }
    // java
    // java }
    fn resetLCID(&mut self, lcid: i16) {
        if GlobalEnv::setLCID(lcid) {
            if let Some(a) = self.mNumFmtAnalyzer.borrow_mut().as_mut() {
                a.resetDic(lcid);
            }
        }
    }
}
impl INumFormatterTrait for NumberFormatter {
    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumberFormatter.java:37
    // java public void format(String text, String formatCode, int maxLength, NumFmtResult result) {
    // java this.mNumFmtParam.mMaxLength = maxLength < 256 ? maxLength : -1;
    // java IFormat numFormatter = this.mNumFmtCompiler.compile(text, formatCode);
    // java if (numFormatter != null) {
    // java numFormatter.format(text, this.mNumFmtParam, result);
    // java if (result.mFillPos > -1) {
    // java result.richFormatInvalid = true;
    // java }
    // java } else {
    // java result.set(text, (Integer)null);
    // java }
    // java
    // java }
    fn format_str(&mut self, text: &str, code: &str, maxLength: i32, result: &mut NumFmtResult) {
        // rust: mNumFmtParam.mMaxLength=maxLength<256?maxLength:-1;IFormat numFormatter=mNumFmtCompiler.compile(text,formatCode);
        let mut param = self.mNumFmtParam.borrow_mut();
        let param = param.as_mut().expect("null NumFmtParam");
        param.mMaxLength = if maxLength < 256 { maxLength } else { -1 };
        let formatter = self
            .mNumFmtCompiler
            .borrow_mut()
            .as_mut()
            .expect("null NumFmtCompiler")
            .compile_text(text, code);
        // rust: if(numFormatter!=null){numFormatter.format(...);if(result.mFillPos> -1)result.richFormatInvalid=true;}else result.set(text,null);
        if let Some(formatter) = formatter {
            formatter
                .borrow_mut()
                .format(FormatValue::Text(text), param, result);
            if result.mFillPos > -1 {
                result.richFormatInvalid = true;
            }
        } else {
            result.set(text, None);
        }
    }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumberFormatter.java:51
    // java public void format(double value, String formatCode, int maxLength, boolean use1904, NumFmtResult result) {
    // java this.mNumFmtParam.mMaxLength = maxLength < 256 ? maxLength : -1;
    // java this.mNumFmtParam.isUse1904 = use1904;
    // java IFormat numFormatter = this.mNumFmtCompiler.compile(value, formatCode);
    // java synchronized(sLock) {
    // java numFormatter.format(value, this.mNumFmtParam, result);
    // java }
    // java }
    fn format_num(
        &mut self,
        value: f64,
        code: &str,
        maxLength: i32,
        use1904: bool,
        result: &mut NumFmtResult,
    ) {
        // rust: param maxLength/use1904,compile outside lock, synchronized(sLock){numFormatter.format(value,param,result);}
        let mut param = self.mNumFmtParam.borrow_mut();
        let param = param.as_mut().expect("null NumFmtParam");
        param.mMaxLength = if maxLength < 256 { maxLength } else { -1 };
        param.isUse1904 = use1904;
        let formatter = self
            .mNumFmtCompiler
            .borrow_mut()
            .as_mut()
            .expect("null NumFmtCompiler")
            .compile(value, code);
        let _guard = S_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        formatter
            .borrow_mut()
            .format(FormatValue::Number(value), param, result);
    }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumberFormatter.java:60
    // java public double formatValue(double value, boolean use1904, String formatCode) {
    // java this.mNumFmtParam.mMaxLength = 256;
    // java this.mNumFmtParam.isUse1904 = use1904;
    // java IFormat numFormatter = this.mNumFmtCompiler.compile(value, formatCode);
    // java synchronized(sLock) {
    // java return numFormatter.formatValue(value, this.mNumFmtParam);
    // java }
    // java }
    fn formatValue(&self, value: f64, use1904: bool, code: &str) -> f64 {
        // rust: mMaxLength=256;isUse1904=use1904;compile;synchronized(sLock){return numFormatter.formatValue(value,param);}
        let mut param = self.mNumFmtParam.borrow_mut();
        let param = param.as_mut().expect("null NumFmtParam");
        param.mMaxLength = 256;
        param.isUse1904 = use1904;
        let formatter = self
            .mNumFmtCompiler
            .borrow_mut()
            .as_mut()
            .expect("null NumFmtCompiler")
            .compile(value, code);
        let _guard = S_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let result = formatter.borrow_mut().formatValue(value, param);
        result
    }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumberFormatter.java:69
    // java public String compile(String sourceCode) {
    // java return this.getFmtAnalyzer().analyse(sourceCode);
    // java }
    fn compile(&self, source: &str) -> Option<String> {
        self.getFmtAnalyzer().analyse(source)
    }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumberFormatter.java:73
    // java public String decompile(String formatCode) {
    // java return this.mNumFmtCompiler.decompile(formatCode);
    // java }
    fn decompile(&self, code: &str) -> String {
        self.mNumFmtCompiler
            .borrow_mut()
            .as_mut()
            .expect("null NumFmtCompiler")
            .decompile(code)
    }
}
