use super::{
    PieceContext::PieceContext,
    PlaceHolder::{ulen, Holder, PlaceHolder},
    PlaceHolderContext::PlaceHolderContext,
};
use crate::cn::wps::kfc::numfmt::{
    format::{NumConverter::NumConverter, NumFmtResult::NumFmtResult, NumFmtStyle::NumFmtStyle},
    global::{
        GlobalEnv::{localeStrManager, GlobalEnv},
        LocaleResID,
        NCLAccesser::NCLAccesser,
    },
    math::{
        GregCalc::GregCalc,
        HebrewCalc::HebrewCalc,
        TimeCalc::{PInt, TimeCalc, TimeStruct},
    },
    resource::{
        IResourceParser::IResourceParser, LocaleResAccessor::LocaleResAccessor, ResConstString,
        ResourceLoader::ResourceLoader,
    },
    util::NumberToStr::NumberToStr,
};
// java public abstract class PlaceHolderDate extends PlaceHolder
pub struct PlaceHolderDate {
    pub base: PlaceHolder,
}
impl PlaceHolderDate {
    pub fn new(id: i16) -> Self {
        Self {
            base: PlaceHolder::new(id),
        }
    }
}

// java public static class PlaceHolderYYear extends PlaceHolderDate
pub struct PlaceHolderYYear {
    pub base: PlaceHolderDate,
}
impl PlaceHolderYYear {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderDate::new(23),
        }
    }
}
impl Holder for PlaceHolderYYear {
    // java public void compile(StringBuffer result,AnalyzeContext context){if(this.mHolderStr.length()>=3){result.append("yyyy");}else{result.append("yy");}}
    fn compile_value(&mut self,result:&mut String,_context:&mut crate::cn::wps::kfc::numfmt::analyze::AnalyzeContext::AnalyzeContext){result.push_str(if ulen(&self.base().mHolderStr)>=3{"yyyy"}else{"yy"});}
    fn base(&self) -> &PlaceHolder {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        let len = ulen(&self.base.base.mHolderStr);
        let mut year = tm.mYear;
        if len <= 2 {
            year %= 100;
        }
        let mut text = NumberToStr::int2Str(year, 2);
        if s.mCalendarType == 3 {
            text = tm.mYear.to_string();
        }
        let trans = if s.mCalendarType == 3 && ulen(&text) <= 2 {
            NumConverter::convertNumeric(s.mDBNumber, s.mNumeralShape, false, year as f64)
        } else {
            NumConverter::transDigits(&text, 0, text.len(), s.mNumeralShape)
        };
        r.mNumFmtStr.push_str(&trans);
    }
    fn decompile_value(&mut self, result: &mut String, _: &mut PieceContext) {
        self.base().repeatLocalChar(
            result,
            localeStrManager
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .getYear()
                .chars()
                .next()
                .unwrap(),
        );
    }
}

// java public static class PlaceHolderGYear extends PlaceHolderDate
pub struct PlaceHolderGYear {
    pub base: PlaceHolderDate,
}
impl PlaceHolderGYear {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderDate::new(20),
        }
    }
}
impl Holder for PlaceHolderGYear {
    // java public void compile(StringBuffer result,AnalyzeContext context){int count=this.mHolderStr.length();if(count==1){result.append("g");}else if(count==2){result.append("gg");}else{result.append("ggg");}}
    fn compile_value(&mut self,result:&mut String,_context:&mut crate::cn::wps::kfc::numfmt::analyze::AnalyzeContext::AnalyzeContext){let count=ulen(&self.base().mHolderStr);result.push_str(if count==1{"g"}else if count==2{"gg"}else{"ggg"});}
    fn base(&self) -> &PlaceHolder {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        if s.mCalendarType == 3 || s.mLocaleCodeID == 1041 {
            let mut f = c.mFormatterContext.borrow_mut();
            let tm = f.mTime.as_mut().unwrap();
            if tm.mEra == -1 {
                let mut ignored = PInt { mInteger: 0 };
                tm.mEra = TimeCalc::greTime2JpTime(tm, &mut ignored);
            }
            let len = ulen(&self.base.base.mHolderStr);
            let index = if len <= 3 { len - 1 } else { 2 };
            r.mNumFmtStr
                .push_str(&ResConstString::get_jp_era()[index][tm.mEra as usize]);
        }
    }
}

// java public static class PlaceHolderEYear extends PlaceHolderDate
pub struct PlaceHolderEYear {
    pub base: PlaceHolderDate,
}
impl PlaceHolderEYear {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderDate::new(21),
        }
    }
}
impl Holder for PlaceHolderEYear {
    // java public void compile(StringBuffer result,AnalyzeContext context){if(this.mHolderStr.length()>=2){result.append("ee");}else{result.append("e");}}
    fn compile_value(&mut self,result:&mut String,_context:&mut crate::cn::wps::kfc::numfmt::analyze::AnalyzeContext::AnalyzeContext){result.push_str(if ulen(&self.base().mHolderStr)>=2{"ee"}else{"e"});}
    fn base(&self) -> &PlaceHolder {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        let mut year = PInt { mInteger: tm.mYear };
        if s.mCalendarType != 3 && s.mLocaleCodeID == 1041 {
            TimeCalc::greTime2JpTime(tm, &mut year);
        }
        let text = NumberToStr::int2Str(year.mInteger, 2);
        let trans = if ulen(&text) <= 2 {
            NumConverter::convertNumeric(s.mDBNumber, s.mNumeralShape, false, year.mInteger as f64)
        } else {
            NumConverter::transDigits(&text, 0, text.len(), s.mNumeralShape)
        };
        r.mNumFmtStr.push_str(&trans);
    }
}

// java public static class PlaceHolderBYear extends PlaceHolderDate
pub struct PlaceHolderBYear {
    pub base: PlaceHolderDate,
}
impl PlaceHolderBYear {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderDate::new(22),
        }
    }
}
impl Holder for PlaceHolderBYear {
    // java public void compile(StringBuffer result,AnalyzeContext context){for(String item:ConstToken.TOKEN_b_CHILDREN){if(CompareUtil.isStringEqual(this.mHolderStr,0,item,true)){result.append(item);return;}}if(this.mHolderStr.length()>=3){result.append("bbbb");}else{result.append("bb");}}
    fn compile_value(&mut self,result:&mut String,_context:&mut crate::cn::wps::kfc::numfmt::analyze::AnalyzeContext::AnalyzeContext){for item in ["B1","B2"]{if crate::cn::wps::kfc::numfmt::util::CompareUtil::CompareUtil::isStringEqual(&self.base().mHolderStr,0,item,true){result.push_str(item);return;}}result.push_str(if ulen(&self.base().mHolderStr)>=3{"bbbb"}else{"bb"});}
    fn base(&self) -> &PlaceHolder {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        if self
            .base
            .base
            .mHolderStr
            .get(..2)
            .is_some_and(|t| t.eq_ignore_ascii_case("B1") || t.eq_ignore_ascii_case("B2"))
        {
            return;
        }
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        let mut year = tm.mYear;
        if s.mCalendarType == 0 {
            year = TimeCalc::greYear2BuddYear(year);
        } else if s.mCalendarType != 7 {
            year = TimeCalc::otherTime2greYear(tm, s.mCalendarType);
            if year < 0 {
                let mut greg = TimeStruct::default();
                GregCalc::double2GregTimeExcel(f.mFormatValue, &mut greg, s.mUse1904);
                year = greg.mYear;
            }
        }
        if ulen(&self.base.base.mHolderStr) <= 2 {
            year %= 100;
        }
        let text = NumberToStr::int2Str(year, 2);
        r.mNumFmtStr.push_str(&NumConverter::transDigits(
            &text,
            0,
            text.len(),
            s.mNumeralShape,
        ));
    }
}

// java public static class PlaceHolderMonth extends PlaceHolderDate
pub struct PlaceHolderMonth {
    pub base: PlaceHolderDate,
}
impl PlaceHolderMonth {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderDate::new(24),
        }
    }
}
impl Holder for PlaceHolderMonth {
    // java public void compile(StringBuffer result,AnalyzeContext context){int num=this.mHolderStr.length()>5?4:this.mHolderStr.length();while(num-->0){result.append('m');}}
    fn compile_value(&mut self,result:&mut String,_context:&mut crate::cn::wps::kfc::numfmt::analyze::AnalyzeContext::AnalyzeContext){let len=ulen(&self.base().mHolderStr);let mut num=if len>5{4}else{len};while num>0{num-=1;result.push('m');}}
    fn base(&self) -> &PlaceHolder {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        let len = ulen(&self.base.base.mHolderStr);
        let text = if len < 3 {
            NumConverter::convertTimeCase(
                s.mNumeralShape,
                &NumberToStr::int2Str(tm.mMonth, len as i32),
                false,
            )
        } else if s.mCalendarType == 8 && s.mLocaleCodeID != 1037 {
            HebrewCalc::getHebrewMonth(tm.mYear, tm.mMonth)
        } else {
            let load = NCLAccesser::getCalenderDefaultLCID(s.mCalendarType, s.mLocaleCodeID);
            self.getLocaleMonthDisplay(s.mCalendarType, load, len, tm.mMonth)
        };
        r.mNumFmtStr.push_str(&text);
    }
    fn decompile_value(&mut self, result: &mut String, _: &mut PieceContext) {
        self.base().repeatLocalChar(
            result,
            localeStrManager
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .getMonth()
                .chars()
                .next()
                .unwrap(),
        );
    }
}
impl PlaceHolderMonth {
    // java getLocaleMonthDisplay chooses calendar-specific resource only for calendars 6,23,10.
    fn getLocaleMonthDisplay(&self, calendar: i8, load: i16, len: usize, month: i32) -> String {
        let id = match len {
            3 => LocaleResID::LR_SHORT_MONTH,
            5 => LocaleResID::LR_SHORTEST_MONTH,
            _ => LocaleResID::LR_LONG_MONTH,
        };
        if matches!(calendar, 6 | 23 | 10) {
            let file = NCLAccesser::getLoadCalendarFile(calendar, load);
            let parser = ResourceLoader::getSaxParser(&file).expect("calendar resource");
            parser.getArray(if len == 3 {
                "short-month"
            } else if len == 5 {
                "shortest-month"
            } else {
                "long-month"
            })[month as usize - 1]
                .clone()
        } else {
            LocaleResAccessor::getLocaleResource(GlobalEnv::getValidLCID(load)).getArray(id)
                [month as usize - 1]
                .clone()
        }
    }
}

// java public static class PlaceHolderDay extends PlaceHolderDate
pub struct PlaceHolderDay {
    pub base: PlaceHolderDate,
}
impl PlaceHolderDay {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderDate::new(25),
        }
    }
}
impl Holder for PlaceHolderDay {
    // java public void compile(StringBuffer result,AnalyzeContext context){if(this.mHolderStr.length()==1){result.append('d');}else{result.append("dd");}}
    fn compile_value(&mut self,result:&mut String,_context:&mut crate::cn::wps::kfc::numfmt::analyze::AnalyzeContext::AnalyzeContext){result.push_str(if ulen(&self.base().mHolderStr)==1{"d"}else{"dd"});}
    fn base(&self) -> &PlaceHolder {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        let f = c.mFormatterContext.borrow();
        let day = f.mTime.as_ref().unwrap().mMonthDay;
        let text = NumberToStr::int2Str(day, ulen(&self.base.base.mHolderStr) as i32);
        r.mNumFmtStr.push_str(&NumConverter::convertTimeCase(
            s.mNumeralShape,
            &text,
            false,
        ));
    }
    fn decompile_value(&mut self, result: &mut String, _: &mut PieceContext) {
        self.base().repeatLocalChar(
            result,
            localeStrManager
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .getDay()
                .chars()
                .next()
                .unwrap(),
        );
    }
}

// java public static class PlaceHolderDWeek extends PlaceHolderDate
pub struct PlaceHolderDWeek {
    pub base: PlaceHolderDate,
}
impl PlaceHolderDWeek {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderDate::new(32),
        }
    }
}
impl Holder for PlaceHolderDWeek {
    // java public void compile(StringBuffer result,AnalyzeContext context){if(this.mHolderStr.length()>=4){result.append("dddd");}else{result.append("ddd");}}
    fn compile_value(&mut self,result:&mut String,_context:&mut crate::cn::wps::kfc::numfmt::analyze::AnalyzeContext::AnalyzeContext){result.push_str(if ulen(&self.base().mHolderStr)>=4{"dddd"}else{"ddd"});}
    fn base(&self) -> &PlaceHolder {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        let len = ulen(&self.base.base.mHolderStr);
        let text = if s.mCalendarType == 8 && s.mLocaleCodeID != 1037 {
            HebrewCalc::getHebrewWeek(tm.mWeekDay)
        } else {
            let load = NCLAccesser::getCalenderDefaultLCID(s.mCalendarType, s.mLocaleCodeID);
            self.getLocaleWeekDisplay(s.mCalendarType, load, len, tm.mWeekDay)
        };
        r.mNumFmtStr.push_str(&text);
    }
    fn decompile_value(&mut self, result: &mut String, _: &mut PieceContext) {
        self.base().repeatLocalChar(
            result,
            localeStrManager
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .getDay()
                .chars()
                .next()
                .unwrap(),
        );
    }
}
impl PlaceHolderDWeek {
    pub fn new_with_id(id: i16) -> Self {
        Self {
            base: PlaceHolderDate::new(id),
        }
    }
    fn getLocaleWeekDisplay(&self, calendar: i8, load: i16, len: usize, weekday: i32) -> String {
        let id = if len == 3 {
            LocaleResID::LR_SHORT_WEEK
        } else {
            LocaleResID::LR_LONG_WEEK
        };
        if matches!(calendar, 6 | 23 | 10) {
            let file = NCLAccesser::getLoadCalendarFile(calendar, load);
            let parser = ResourceLoader::getSaxParser(&file).expect("calendar resource");
            parser.getArray(if len == 3 { "short-week" } else { "long-week" })[weekday as usize]
                .clone()
        } else {
            LocaleResAccessor::getLocaleResource(GlobalEnv::getValidLCID(load)).getArray(id)
                [weekday as usize]
                .clone()
        }
    }
}
// java public static class PlaceHolderAWeek extends PlaceHolderDWeek
pub struct PlaceHolderAWeek {
    pub base: PlaceHolderDWeek,
}
impl PlaceHolderAWeek {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderDWeek::new_with_id(33),
        }
    }
}
impl Holder for PlaceHolderAWeek {
    // java public void compile(StringBuffer result,AnalyzeContext context){if(this.mHolderStr.length()>=4){result.append("aaaa");}else{result.append("aaa");}}
    fn compile_value(&mut self,result:&mut String,_context:&mut crate::cn::wps::kfc::numfmt::analyze::AnalyzeContext::AnalyzeContext){result.push_str(if ulen(&self.base().mHolderStr)>=4{"aaaa"}else{"aaa"});}
    fn base(&self) -> &PlaceHolder {
        self.base.base()
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        self.base.base_mut()
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        self.base.format_value(c, s, r);
    }
    fn decompile_value(&mut self, r: &mut String, p: &mut PieceContext) {
        self.base.decompile_value(r, p);
    }
}
// java public static class PlaceHolderThaiDate extends PlaceHolderDate
pub struct PlaceHolderThaiDate {
    pub base: PlaceHolderDate,
}
impl PlaceHolderThaiDate {
    pub fn new(id: i16) -> Self {
        Self {
            base: PlaceHolderDate::new(id),
        }
    }
    // java convertLoadCodeID always yields 1054 after the original calendar LCID call.
    fn convertLoadCodeID(s: &NumFmtStyle) -> i16 {
        let _load = NCLAccesser::getCalenderDefaultLCID(s.mCalendarType, s.mLocaleCodeID);
        1054
    }
    fn convertNumeric(s: &NumFmtStyle, value: i32, reserved: i32) -> String {
        let shape = if s.mNumeralShape == 0 {
            13
        } else {
            s.mNumeralShape
        };
        NumConverter::convertTimeCase(shape, &NumberToStr::int2Str(value, reserved), false)
    }
}

// java public static class PlaceHolderThaiYear extends PlaceHolderThaiDate
pub struct PlaceHolderThaiYear {
    pub base: PlaceHolderThaiDate,
}
impl PlaceHolderThaiYear {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderThaiDate::new(38),
        }
    }
}
impl Holder for PlaceHolderThaiYear {
    fn base(&self) -> &PlaceHolder {
        &self.base.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base.base.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        let mut year = tm.mYear;
        if !s.mHasNCL || s.mCalendarType == 0 {
            year = TimeCalc::greYear2BuddYear(year);
        }
        if ulen(&self.base.base.base.mHolderStr) <= 2 {
            year %= 100;
        }
        r.mNumFmtStr
            .push_str(&PlaceHolderThaiDate::convertNumeric(s, year, 2));
    }
}

// java public static class PlaceHolderThaiMonth extends PlaceHolderThaiDate
pub struct PlaceHolderThaiMonth {
    pub base: PlaceHolderThaiDate,
}
impl PlaceHolderThaiMonth {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderThaiDate::new(37),
        }
    }
}
impl Holder for PlaceHolderThaiMonth {
    fn base(&self) -> &PlaceHolder {
        &self.base.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base.base.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        let f = c.mFormatterContext.borrow();
        let month = f.mTime.as_ref().unwrap().mMonth;
        let len = ulen(&self.base.base.base.mHolderStr);
        let load = PlaceHolderThaiDate::convertLoadCodeID(s);
        let text = if len <= 2 {
            PlaceHolderThaiDate::convertNumeric(s, month, len as i32)
        } else {
            let id = match len {
                3 => LocaleResID::LR_SHORT_MONTH,
                5 => LocaleResID::LR_SHORTEST_MONTH,
                _ => LocaleResID::LR_LONG_MONTH,
            };
            LocaleResAccessor::getLocaleResource(GlobalEnv::getValidLCID(load)).getArray(id)
                [month as usize - 1]
                .clone()
        };
        r.mNumFmtStr.push_str(&text);
    }
}

// java public static class PlaceHolderThaiDay extends PlaceHolderThaiDate
pub struct PlaceHolderThaiDay {
    pub base: PlaceHolderThaiDate,
}
impl PlaceHolderThaiDay {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderThaiDate::new(36),
        }
    }
}
impl Holder for PlaceHolderThaiDay {
    fn base(&self) -> &PlaceHolder {
        &self.base.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base.base.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        let f = c.mFormatterContext.borrow();
        let tm = f.mTime.as_ref().unwrap();
        let len = ulen(&self.base.base.base.mHolderStr);
        let load = PlaceHolderThaiDate::convertLoadCodeID(s);
        let text = if len <= 2 {
            PlaceHolderThaiDate::convertNumeric(s, tm.mMonthDay, len as i32)
        } else {
            let id = if len == 3 {
                LocaleResID::LR_SHORT_WEEK
            } else {
                LocaleResID::LR_LONG_WEEK
            };
            LocaleResAccessor::getLocaleResource(GlobalEnv::getValidLCID(load)).getArray(id)
                [tm.mWeekDay as usize]
                .clone()
        };
        r.mNumFmtStr.push_str(&text);
    }
}
