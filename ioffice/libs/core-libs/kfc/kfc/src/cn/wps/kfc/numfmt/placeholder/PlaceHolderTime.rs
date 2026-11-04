use super::{
    PieceContext::PieceContext,
    PlaceHolder::{ulen, usub, Holder, PlaceHolder},
    PlaceHolderContext::PlaceHolderContext,
};
use crate::cn::wps::kfc::numfmt::{
    format::{NumConverter::NumConverter, NumFmtResult::NumFmtResult, NumFmtStyle::NumFmtStyle},
    global::{
        GlobalEnv::{localeStrManager, GlobalEnv},
        LocaleResID,
    },
    math::TimeCalc::TimeCalc,
    resource::LocaleResAccessor::LocaleResAccessor,
    util::NumberToStr::NumberToStr,
};
// java public abstract class PlaceHolderTime extends PlaceHolder
pub struct PlaceHolderTime {
    pub base: PlaceHolder,
    pub mIsElapsed: bool,
}
impl PlaceHolderTime {
    pub fn new(id: i16) -> Self {
        Self {
            base: PlaceHolder::new(id),
            mIsElapsed: false,
        }
    }
}
// java private abstract static class WPlaceHolderTime extends PlaceHolderTime
pub struct WPlaceHolderTime {
    pub base: PlaceHolderTime,
}
impl WPlaceHolderTime {
    pub fn new(id: i16) -> Self {
        Self {
            base: PlaceHolderTime::new(id),
        }
    }
}
// java private abstract static class PlaceHolderElapseTime extends PlaceHolderTime
pub struct PlaceHolderElapseTime {
    pub base: PlaceHolderTime,
}
impl PlaceHolderElapseTime {
    // java public void compile(StringBuffer result,AnalyzeContext context){if(this.mHolderStr.length()>0){result.append('[');result.append(NumFmtUtil.fillChar(this.getInnerChar(),this.mHolderStr.length()));result.append(']');}}
    pub fn compile(&self,result:&mut String,innerChar:char){if !self.base.base.mHolderStr.is_empty(){result.push('[');result.push_str(&innerChar.to_string().repeat(ulen(&self.base.base.mHolderStr)));result.push(']');}}
    pub fn new(id: i16) -> Self {
        let mut base = PlaceHolderTime::new(id);
        base.mIsElapsed = true;
        Self { base }
    }
}
// Rust virtual getTime/convertTimeCase slots correspond to the original protected methods.
trait TimeHolder: Send {
    // Native virtual slot; the original PlaceHolder default appends mHolderStr.
    fn compile_time(&self,result:&mut String){result.push_str(&self.timeBase().base.mHolderStr);}
    fn timeBase(&self) -> &PlaceHolderTime;
    fn timeBase_mut(&mut self) -> &mut PlaceHolderTime;
    fn getTime(&self, c: &PlaceHolderContext, len: i32) -> String;
    fn shape(&self, s: &NumFmtStyle) -> i8 {
        s.mNumeralShape
    }
    fn localChar(&self) -> char;
    fn isWordSubclass(&self) -> bool {
        false
    }
}
impl<T: TimeHolder> Holder for T {
    fn compile_value(&mut self,result:&mut String,_context:&mut crate::cn::wps::kfc::numfmt::analyze::AnalyzeContext::AnalyzeContext){self.compile_time(result);}
    fn base(&self) -> &PlaceHolder {
        &self.timeBase().base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.timeBase_mut().base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        // java WPlaceHolderTime.format delegates directly to super.format when contextType==2.
        assert!(
            !self.isWordSubclass() || c.mContextType == 2,
            "DEFERRED_NON_FUNCTION: WPlaceHolderTime Word context"
        );
        // java String strTime=this.getTime(context,mHolderStr.length());result.mNumFmtStr.append(this.convertTimeCase(numFmtStyle,strTime));
        let text = self.getTime(c, ulen(&self.base().mHolderStr) as i32);
        r.mNumFmtStr.push_str(&NumConverter::convertTimeCase(
            self.shape(s),
            &text,
            self.timeBase().mIsElapsed,
        ));
    }
    fn decompile_value(&mut self, result: &mut String, _: &mut PieceContext) {
        let elapsed = self.timeBase().mIsElapsed;
        if elapsed && !self.base().mHolderStr.is_empty() {
            result.push('[');
        }
        self.base().repeatLocalChar(result, self.localChar());
        if elapsed && !self.base().mHolderStr.is_empty() {
            result.push(']');
        }
    }
}
// java public static class PlaceHolderHour extends WPlaceHolderTime
pub struct PlaceHolderHour {
    pub base: WPlaceHolderTime,
}
impl PlaceHolderHour {
    pub fn new() -> Self {
        Self {
            base: WPlaceHolderTime::new(26),
        }
    }
}
impl TimeHolder for PlaceHolderHour {
    // java public void compile(StringBuffer result,AnalyzeContext context){if(this.mHolderStr.length()>=2){result.append("hh");}else{result.append("h");}}
    fn compile_time(&self,result:&mut String){result.push_str(if ulen(&self.timeBase().base.mHolderStr)>=2{"hh"}else{"h"});}
    fn timeBase(&self) -> &PlaceHolderTime {
        &self.base.base
    }
    fn timeBase_mut(&mut self) -> &mut PlaceHolderTime {
        &mut self.base.base
    }
    fn getTime(&self, c: &PlaceHolderContext, len: i32) -> String {
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        let mut hour = tm.mHour;
        if !c.mIs24HR {
            hour %= 12;
            if hour == 0 {
                hour = 12;
            }
        }
        NumberToStr::int2Str(hour, len)
    }
    fn localChar(&self) -> char {
        localeStrManager
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .getHour()
            .chars()
            .next()
            .unwrap()
    }
    fn isWordSubclass(&self) -> bool {
        true
    }
}
// java public static class PlaceHolderMin extends WPlaceHolderTime
pub struct PlaceHolderMin {
    pub base: WPlaceHolderTime,
}
impl PlaceHolderMin {
    pub fn new() -> Self {
        Self {
            base: WPlaceHolderTime::new(27),
        }
    }
}
impl TimeHolder for PlaceHolderMin {
    // java public void compile(StringBuffer result,AnalyzeContext context){if(this.mHolderStr.length()==1){result.append("m");}else{result.append("mm");}}
    fn compile_time(&self,result:&mut String){result.push_str(if ulen(&self.timeBase().base.mHolderStr)==1{"m"}else{"mm"});}
    fn timeBase(&self) -> &PlaceHolderTime {
        &self.base.base
    }
    fn timeBase_mut(&mut self) -> &mut PlaceHolderTime {
        &mut self.base.base
    }
    fn getTime(&self, c: &PlaceHolderContext, len: i32) -> String {
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        NumberToStr::int2Str(tm.mMinute, len)
    }
    fn localChar(&self) -> char {
        localeStrManager
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .getMinute()
            .chars()
            .next()
            .unwrap()
    }
    fn isWordSubclass(&self) -> bool {
        true
    }
}
// java public static class PlaceHolderSec extends WPlaceHolderTime
pub struct PlaceHolderSec {
    pub base: WPlaceHolderTime,
}
impl PlaceHolderSec {
    pub fn new() -> Self {
        Self {
            base: WPlaceHolderTime::new(28),
        }
    }
}
impl TimeHolder for PlaceHolderSec {
    // java public void compile(StringBuffer result,AnalyzeContext context){if(this.mHolderStr.length()==1){result.append("s");}else{result.append("ss");}}
    fn compile_time(&self,result:&mut String){result.push_str(if ulen(&self.timeBase().base.mHolderStr)==1{"s"}else{"ss"});}
    fn timeBase(&self) -> &PlaceHolderTime {
        &self.base.base
    }
    fn timeBase_mut(&mut self) -> &mut PlaceHolderTime {
        &mut self.base.base
    }
    fn getTime(&self, c: &PlaceHolderContext, len: i32) -> String {
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        NumberToStr::int2Str(tm.mSecond, len)
    }
    fn localChar(&self) -> char {
        localeStrManager
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .getSecond()
            .chars()
            .next()
            .unwrap()
    }
    fn isWordSubclass(&self) -> bool {
        true
    }
}
// java public static class PlaceHolderElapseHour extends PlaceHolderElapseTime
pub struct PlaceHolderElapseHour {
    pub base: PlaceHolderElapseTime,
}
impl PlaceHolderElapseHour {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderElapseTime::new(29),
        }
    }
}
impl TimeHolder for PlaceHolderElapseHour {
    // java protected char getInnerChar(){return 'h';} // inherited PlaceHolderElapseTime.compile
    fn compile_time(&self,result:&mut String){self.base.compile(result,'h');}
    fn timeBase(&self) -> &PlaceHolderTime {
        &self.base.base
    }
    fn timeBase_mut(&mut self) -> &mut PlaceHolderTime {
        &mut self.base.base
    }
    fn getTime(&self, c: &PlaceHolderContext, len: i32) -> String {
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        NumberToStr::int2Str(TimeCalc::getElapsedHours(f.mFormatValue), len)
    }
    fn localChar(&self) -> char {
        localeStrManager
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .getHour()
            .chars()
            .next()
            .unwrap()
    }
}
// java public static class PlaceHolderElapseMin extends PlaceHolderElapseTime
pub struct PlaceHolderElapseMin {
    pub base: PlaceHolderElapseTime,
}
impl PlaceHolderElapseMin {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderElapseTime::new(30),
        }
    }
}
impl TimeHolder for PlaceHolderElapseMin {
    // java protected char getInnerChar(){return 'm';} // inherited PlaceHolderElapseTime.compile
    fn compile_time(&self,result:&mut String){self.base.compile(result,'m');}
    fn timeBase(&self) -> &PlaceHolderTime {
        &self.base.base
    }
    fn timeBase_mut(&mut self) -> &mut PlaceHolderTime {
        &mut self.base.base
    }
    fn getTime(&self, c: &PlaceHolderContext, len: i32) -> String {
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        NumberToStr::int2Str(TimeCalc::getElapsedMinutes(f.mFormatValue), len)
    }
    fn localChar(&self) -> char {
        localeStrManager
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .getMinute()
            .chars()
            .next()
            .unwrap()
    }
}
// java public static class PlaceHolderElapseSec extends PlaceHolderElapseTime
pub struct PlaceHolderElapseSec {
    pub base: PlaceHolderElapseTime,
}
impl PlaceHolderElapseSec {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderElapseTime::new(31),
        }
    }
}
impl TimeHolder for PlaceHolderElapseSec {
    // java protected char getInnerChar(){return 's';} // inherited PlaceHolderElapseTime.compile
    fn compile_time(&self,result:&mut String){self.base.compile(result,'s');}
    fn timeBase(&self) -> &PlaceHolderTime {
        &self.base.base
    }
    fn timeBase_mut(&mut self) -> &mut PlaceHolderTime {
        &mut self.base.base
    }
    fn getTime(&self, c: &PlaceHolderContext, len: i32) -> String {
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        NumberToStr::long2Str(TimeCalc::getElapsedSeconds(f.mFormatValue), len)
    }
    fn localChar(&self) -> char {
        localeStrManager
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .getSecond()
            .chars()
            .next()
            .unwrap()
    }
}
// java public static class PlaceHolderThaiHour extends PlaceHolderTime
pub struct PlaceHolderThaiHour {
    pub base: PlaceHolderTime,
}
impl PlaceHolderThaiHour {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderTime::new(39),
        }
    }
}
impl TimeHolder for PlaceHolderThaiHour {
    fn timeBase(&self) -> &PlaceHolderTime {
        &self.base
    }
    fn timeBase_mut(&mut self) -> &mut PlaceHolderTime {
        &mut self.base
    }
    fn getTime(&self, c: &PlaceHolderContext, len: i32) -> String {
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        let mut hour = tm.mHour;
        if !c.mIs24HR {
            hour %= 12;
            if hour == 0 {
                hour = 12;
            }
        }
        NumberToStr::int2Str(hour, len)
    }
    fn localChar(&self) -> char {
        localeStrManager
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .getHour()
            .chars()
            .next()
            .unwrap()
    }
    // java byte numShape=numFmtStyle.mNumeralShape;if(numShape==0)numShape=13;
    fn shape(&self, s: &NumFmtStyle) -> i8 {
        if s.mNumeralShape == 0 {
            13
        } else {
            s.mNumeralShape
        }
    }
}
// java public static class PlaceHolderThaiMinute extends PlaceHolderTime
pub struct PlaceHolderThaiMinute {
    pub base: PlaceHolderTime,
}
impl PlaceHolderThaiMinute {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderTime::new(40),
        }
    }
}
impl TimeHolder for PlaceHolderThaiMinute {
    fn timeBase(&self) -> &PlaceHolderTime {
        &self.base
    }
    fn timeBase_mut(&mut self) -> &mut PlaceHolderTime {
        &mut self.base
    }
    fn getTime(&self, c: &PlaceHolderContext, len: i32) -> String {
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        NumberToStr::int2Str(tm.mMinute, len)
    }
    fn localChar(&self) -> char {
        localeStrManager
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .getMinute()
            .chars()
            .next()
            .unwrap()
    }
    // java byte numShape=numFmtStyle.mNumeralShape;if(numShape==0)numShape=13;
    fn shape(&self, s: &NumFmtStyle) -> i8 {
        if s.mNumeralShape == 0 {
            13
        } else {
            s.mNumeralShape
        }
    }
}
// java public static class PlaceHolderThaiSecond extends PlaceHolderTime
pub struct PlaceHolderThaiSecond {
    pub base: PlaceHolderTime,
}
impl PlaceHolderThaiSecond {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderTime::new(41),
        }
    }
}
impl TimeHolder for PlaceHolderThaiSecond {
    fn timeBase(&self) -> &PlaceHolderTime {
        &self.base
    }
    fn timeBase_mut(&mut self) -> &mut PlaceHolderTime {
        &mut self.base
    }
    fn getTime(&self, c: &PlaceHolderContext, len: i32) -> String {
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        NumberToStr::int2Str(tm.mSecond, len)
    }
    fn localChar(&self) -> char {
        localeStrManager
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .getSecond()
            .chars()
            .next()
            .unwrap()
    }
    // java byte numShape=numFmtStyle.mNumeralShape;if(numShape==0)numShape=13;
    fn shape(&self, s: &NumFmtStyle) -> i8 {
        if s.mNumeralShape == 0 {
            13
        } else {
            s.mNumeralShape
        }
    }
}
// java public static class PlaceHolderAmPm extends PlaceHolderTime
pub struct PlaceHolderAmPm {
    pub base: PlaceHolderTime,
}
impl PlaceHolderAmPm {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderTime::new(34),
        }
    }
}
impl Holder for PlaceHolderAmPm {
    fn base(&self) -> &PlaceHolder {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        let f = c.mFormatterContext.borrow();
        let isAm = TimeCalc::isAm(f.mTime.as_ref().unwrap().mHour);
        let token = &self.base.base.mHolderStr;
        // java Chinese AM/PM and a/p preserve their literal halves; other AM/PM uses locale resources.
        if token != "上午/下午" && !token.eq_ignore_ascii_case("a/p") {
            let lcid = if !s.mHasNCL || s.mLocaleCodeID == 0 {
                GlobalEnv::getDefaultLCID()
            } else {
                s.mLocaleCodeID
            };
            let cache = LocaleResAccessor::getLocaleResource(lcid);
            let item = cache.getItem(LocaleResID::LR_AM_PM);
            if !item.isEmpty() {
                r.mNumFmtStr
                    .push_str(&item.mContent.as_ref().unwrap()[if isAm { 0 } else { 1 }]);
            }
        } else {
            let (am, pm) = token.split_once('/').expect("AM/PM delimiter");
            r.mNumFmtStr.push_str(if isAm { am } else { pm });
        }
    }
}
