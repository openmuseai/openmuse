use super::{PieceContext::PieceContext, PlaceHolderContext::PlaceHolderContext};
use crate::cn::wps::kfc::numfmt::{
    builtin::BuiltinColors::BuiltinColors,
    format::{NumConverter::NumConverter, NumFmtResult::NumFmtResult, NumFmtStyle::NumFmtStyle},
    global::{
        GlobalEnv::{localeStrManager, GlobalEnv},
        NCLAccesser::NCLAccesser,
    },
};
use std::sync::{Arc, Mutex};
use crate::cn::wps::kfc::numfmt::analyze::AnalyzeContext::AnalyzeContext;
pub const HALF_SPACE: char = ' ';
pub const FULL_SPACE: char = '　';
pub const FILL_ZERO: char = '0';
// java public abstract class PlaceHolder
pub struct PlaceHolder {
    pub mPlaceHolderID: i16,
    pub mHolderStr: String,
}
impl PlaceHolder {
    pub fn new(id: i16) -> Self {
        Self {
            mPlaceHolderID: id,
            mHolderStr: String::new(),
        }
    }
    pub fn getHolderStr(&self) -> &str {
        &self.mHolderStr
    }
    // java this.mHolderStr=holderStr;
    pub fn setHolderStr(&mut self, holderStr: String) {
        self.mHolderStr = holderStr;
    }
    pub fn destroy(&mut self) {}
    pub fn clear(&mut self) {}
    // java for(int len=this.mHolderStr.length();i<len;++i) result.append(c);
    pub fn repeatLocalChar(&self, result: &mut String, c: char) {
        for _ in 0..ulen(&self.mHolderStr) {
            result.push(c);
        }
    }
}
pub type HolderRef = Arc<Mutex<Box<dyn Holder>>>;
// Rust vtable preserves Java format/decompile virtual dispatch.
pub trait Holder: Send {
    fn base(&self) -> &PlaceHolder;
    fn base_mut(&mut self) -> &mut PlaceHolder;
    // java public void compile(StringBuffer result, AnalyzeContext context) { result.append(this.mHolderStr); }
    fn compile_value(&mut self,result:&mut String,_context:&mut crate::cn::wps::kfc::numfmt::analyze::AnalyzeContext::AnalyzeContext){result.push_str(&self.base().mHolderStr);}
    fn format_value(
        &mut self,
        context: &mut PlaceHolderContext,
        style: &NumFmtStyle,
        result: &mut NumFmtResult,
    );
    // java public void decompile(StringBuffer result,PieceContext pieceContext){result.append(this.mHolderStr);}
    fn decompile_value(&mut self, result: &mut String, _: &mut PieceContext) {
        result.push_str(&self.base().mHolderStr);
    }
}
pub(crate) fn ulen(s: &str) -> usize {
    s.encode_utf16().count()
}
pub(crate) fn usub(s: &str, start: usize, end: usize) -> String {
    String::from_utf16(&s.encode_utf16().collect::<Vec<_>>()[start..end])
        .expect("unpaired UTF16 string boundary")
}
// java private abstract static class PlaceHolderDigit extends PlaceHolder
pub struct PlaceHolderDigit {
    pub base: PlaceHolder,
    pub mIsFillChar: bool,
}
impl PlaceHolderDigit {
    // java public void compile(StringBuffer result, AnalyzeContext context) {
    pub fn compile(&self,result:&mut String,context:&mut AnalyzeContext){
        // java if(context.leftDigiRemind>0){this.compileInt(result,context);}else{super.compile(result,context);}
        if context.leftDigiRemind>0{self.compileInt(result,context);}else{result.push_str(&self.base.mHolderStr);}
        // java if(context.insertContent.length()>0){boolean toAdd=false;switch(context.type){case 8:if(context.curPos==context.lastIntPartPos){toAdd=true;}break;case 16:if(context.curPos==context.lastDigitBeforeE){toAdd=true;}break;case 512:if(context.curPos==context.lastDigitalPos){toAdd=true;}}if(toAdd){result.append(context.insertContent);context.insertContent.setLength(0);}}
        if !context.insertContent.is_empty(){let mut toAdd=false;match context.r#type{8=>{if context.curPos==context.lastIntPartPos{toAdd=true;}},16=>{if context.curPos==context.lastDigitBeforeE{toAdd=true;}},512=>{if context.curPos==context.lastDigitalPos{toAdd=true;}},_=>{}}if toAdd{result.push_str(&context.insertContent);context.insertContent.clear();}}
    }
    // java private void compileInt(StringBuffer result, AnalyzeContext context) {
    fn compileInt(&self,result:&mut String,context:&mut AnalyzeContext){
        // java if(context.thouSepInIntPart){if(context.leftDigiRemind==context.leftDigital&&context.leftDigital<=3){result.append("#,");int need=3-context.leftDigital;while(need-->0){result.append('#');}}
        if context.thouSepInIntPart{if context.leftDigiRemind==context.leftDigital&&context.leftDigital<=3{result.push_str("#,");let mut need=3-context.leftDigital;while need>0{need-=1;result.push('#');}}
            // java for(int i=0;i<this.mHolderStr.length();++i){result.append(this.mHolderStr.charAt(i));--context.leftDigiRemind;if(context.leftDigiRemind%3==0&&context.leftDigiRemind>0){result.append(',');}}
            for i in 0..ulen(&self.base.mHolderStr){result.push_str(&usub(&self.base.mHolderStr,i,i+1));context.leftDigiRemind-=1;if context.leftDigiRemind%3==0&&context.leftDigiRemind>0{result.push(',');}}
        // java }else{result.append(this.mHolderStr);context.leftDigiRemind-=this.mHolderStr.length();}
        }else{result.push_str(&self.base.mHolderStr);context.leftDigiRemind-=ulen(&self.base.mHolderStr)as i32;}
    }
    pub fn new(id: i16, fillChar: bool) -> Self {
        Self {
            base: PlaceHolder::new(id),
            mIsFillChar: fillChar,
        }
    }
    pub fn format(
        &self,
        c: &mut PlaceHolderContext,
        s: &NumFmtStyle,
        r: &mut NumFmtResult,
        fill: char,
    ) {
        // java if(context.mOffset>=0)this.formatIntPart(...);else this.formatDecimalPart(...);
        if c.mOffset >= 0 {
            self.formatIntPart(c, s, r, fill);
        } else {
            self.formatDecimalPart(c, s, r, fill);
        }
    }
    fn formatIntPart(
        &self,
        c: &mut PlaceHolderContext,
        s: &NumFmtStyle,
        r: &mut NumFmtResult,
        fill: char,
    ) {
        let tokenLen = ulen(&self.base.mHolderStr) as i32;
        let frame = c.mFormatterContext.clone();
        let f = frame.borrow();
        let integerLen = ulen(&f.mInteger) as i32;
        // java int intervalLen=3;if(GlobalEnv.getDefaultLCID()==1081)intervalLen=2;
        let interval = if GlobalEnv::getDefaultLCID() == 1081 {
            2
        } else {
            3
        };
        // java if(context.isFirstDigitEx()&&integerLen>=context.mLDigitHolderCount) ...
        if c.isFirstDigitEx() && integerLen >= c.mLDigitHolderCount {
            c.mLDigitOffset -= tokenLen;
            if c.mLDigitOffset <= c.mOffset {
                let sub = NumConverter::transDigits(
                    &f.mInteger,
                    0,
                    (c.mOffset - c.mLDigitOffset) as usize,
                    s.mNumeralShape,
                );
                if s.mHasThousSep {
                    let len = ulen(&sub);
                    let mut index = len % interval;
                    if index > 0 {
                        let text = usub(&sub, 0, index);
                        r.mNumFmtStr.push_str(&text);
                        r.mNumRealValue.push_str(&text);
                        c.mOffset -= index as i32;
                    }
                    let separator = localeStrManager
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .getThousSepator();
                    while index < len {
                        if index > 0 {
                            r.mNumFmtStr.push_str(&separator);
                            r.mNumRealValue.push_str(&separator);
                        }
                        let text = usub(&sub, index, index + interval);
                        r.mNumFmtStr.push_str(&text);
                        r.mNumRealValue.push_str(&text);
                        c.mOffset -= interval as i32;
                        index += interval;
                    }
                    c.mShowSeparator = true;
                } else {
                    r.mNumFmtStr.push_str(&sub);
                    r.mNumRealValue.push_str(&sub);
                    c.mOffset -= ulen(&sub) as i32;
                }
            }
        } else {
            let mut appended = false;
            let start = ulen(&r.mNumFmtStr);
            for _ in 0..tokenLen {
                if c.mLDigitOffset <= c.mOffset {
                    let ch = f
                        .mInteger
                        .chars()
                        .nth((integerLen - c.mOffset) as usize)
                        .unwrap();
                    r.mNumFmtStr.push(ch);
                    r.mNumRealValue.push(ch);
                    c.mOffset -= 1;
                    appended = true;
                } else if self.mIsFillChar {
                    r.mNumFmtStr.push(fill);
                    r.mNumRealValue.push(fill);
                }
                c.mLDigitOffset -= 1;
            }
            let end = ulen(&r.mNumFmtStr);
            NumConverter::transDigits_buffer(&mut r.mNumFmtStr, start, end, s.mNumeralShape);
            if s.mHasThousSep && (self.mIsFillChar || appended) {
                c.mShowSeparator = true;
            }
            if c.mLDigitOffset <= 0 {
                c.mShowSeparator = false;
            }
        }
    }
    fn formatDecimalPart(
        &self,
        c: &mut PlaceHolderContext,
        s: &NumFmtStyle,
        r: &mut NumFmtResult,
        fill: char,
    ) {
        // java if(context.mRDigitOffset<context.mRDigitHolderCount) { ... append decimal or fill, advance offset;transDigits(result,...);}
        let tokenLen = ulen(&self.base.mHolderStr);
        if c.mRDigitOffset < c.mRDigitHolderCount {
            let frame = c.mFormatterContext.clone();
            let f = frame.borrow();
            let digits: Vec<_> = f.mDecimal.chars().collect();
            let start = ulen(&r.mNumFmtStr);
            for _ in 0..tokenLen {
                if (c.mRDigitOffset as usize) < digits.len() {
                    r.mNumFmtStr.push(digits[c.mRDigitOffset as usize]);
                    r.mNumRealValue.push(digits[c.mRDigitOffset as usize]);
                } else if self.mIsFillChar {
                    r.mNumFmtStr.push(fill);
                    r.mNumRealValue.push(fill);
                }
                c.mRDigitOffset += 1;
            }
            let end = ulen(&r.mNumFmtStr);
            NumConverter::transDigits_buffer(&mut r.mNumFmtStr, start, end, s.mNumeralShape);
        }
    }
}

// java public static class PlaceHolderSharp extends PlaceHolderDigit
pub struct PlaceHolderSharp {
    pub base: PlaceHolderDigit,
}
impl PlaceHolderSharp {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderDigit::new(2, false),
        }
    }
}
impl Holder for PlaceHolderSharp {
    fn compile_value(&mut self,r:&mut String,c:&mut AnalyzeContext){self.base.compile(r,c)}
    fn base(&self) -> &PlaceHolder {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base.base
    }
    // java protected char getFillChar() {return '#';}
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        self.base.format(c, s, r, '#');
    }
}

// java public static class PlaceHolderZero extends PlaceHolderDigit
pub struct PlaceHolderZero {
    pub base: PlaceHolderDigit,
}
impl PlaceHolderZero {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderDigit::new(3, true),
        }
    }
}
impl Holder for PlaceHolderZero {
    fn compile_value(&mut self,r:&mut String,c:&mut AnalyzeContext){self.base.compile(r,c)}
    fn base(&self) -> &PlaceHolder {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base.base
    }
    // java protected char getFillChar() {return '0';}
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        self.base.format(c, s, r, '0');
    }
}

// java public static class PlaceHolderAsk extends PlaceHolderDigit
pub struct PlaceHolderAsk {
    pub base: PlaceHolderDigit,
}
impl PlaceHolderAsk {
    pub fn new() -> Self {
        Self {
            base: PlaceHolderDigit::new(4, true),
        }
    }
}
impl Holder for PlaceHolderAsk {
    fn compile_value(&mut self,r:&mut String,c:&mut AnalyzeContext){self.base.compile(r,c)}
    fn base(&self) -> &PlaceHolder {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base.base
    }
    // java protected char getFillChar() {return ' ';}
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        self.base.format(c, s, r, ' ');
    }
}

// java public static class PlaceHolderGeneral extends PlaceHolder
pub struct PlaceHolderGeneral {
    pub base: PlaceHolder,
}
impl PlaceHolderGeneral {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(1),
        }
    }
}
impl Holder for PlaceHolderGeneral {
    // java public void compile(StringBuffer result,AnalyzeContext context){result.append("General");}
    fn compile_value(&mut self,result:&mut String,_context:&mut AnalyzeContext){result.push_str("General");}
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        let frame = c.mFormatterContext.clone();
        let f = frame.borrow();
        // java if(context.mFormatterContext.valueIsText()) append formatText;
        if let Some(text) = &f.mFormatText {
            r.mNumFmtStr.push_str(text);
        } else {
            // java general shape -> convertGeneral, discard its leading minus (base adds it); otherwise convertNumeric.
            if s.mDBNumber == 0 && !NCLAccesser::isCaseShape(s.mNumeralShape) {
                let text =
                    NumConverter::convertGeneral(s.mNumeralShape, f.mFormatValue, s.mMaxLength);
                if !text.is_empty() {
                    let start = if text.starts_with('-') { 1 } else { 0 };
                    localeStrManager
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .replaceDecPointLocal(&mut r.mNumFmtStr, &text[start..]);
                }
            } else {
                let converted = NumConverter::convertNumeric(
                    s.mDBNumber,
                    s.mNumeralShape,
                    s.mIsRMBScale,
                    f.mFormatValue.abs(),
                );
                localeStrManager
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .replaceDecPointLocal(&mut r.mNumFmtStr, &converted);
            }
        }
    }
    fn decompile_value(&mut self, result: &mut String, context: &mut PieceContext) {
        result.push_str(
            &localeStrManager
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .getGeneral(),
        );
    }
}

// java public static class PlaceHolderThousSep extends PlaceHolder
pub struct PlaceHolderThousSep {
    pub base: PlaceHolder,
}
impl PlaceHolderThousSep {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(8),
        }
    }
}
impl Holder for PlaceHolderThousSep {
    // java public void compile(StringBuffer result,AnalyzeContext context) { }
    fn compile_value(&mut self,_result:&mut String,_context:&mut AnalyzeContext){}
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        // java if(context.mShowSeparator) append space after space, otherwise local separator; context.mShowSeparator=false;
        if c.mShowSeparator {
            if r.mNumFmtStr.ends_with(' ') {
                r.mNumFmtStr.push(' ');
            } else {
                r.mNumFmtStr.push_str(
                    &localeStrManager
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .getThousSepator(),
                );
            }
            c.mShowSeparator = false;
        }
    }
    fn decompile_value(&mut self, result: &mut String, context: &mut PieceContext) {
        self.base.repeatLocalChar(
            result,
            localeStrManager
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .getThousSepator()
                .chars()
                .next()
                .unwrap(),
        );
    }
}

// java public static class PlaceHolderDeciPoint extends PlaceHolder
pub struct PlaceHolderDeciPoint {
    pub base: PlaceHolder,
}
impl PlaceHolderDeciPoint {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(9),
        }
    }
}
impl Holder for PlaceHolderDeciPoint {
    // java public void compile(StringBuffer result,AnalyzeContext context){int i=0;for(int len=this.mHolderStr.length();i<len;++i){if(context.type==8192&&i!=len-1){result.append("\\.");}else{result.append('.');}}}
    fn compile_value(&mut self,result:&mut String,context:&mut AnalyzeContext){let mut i=0;let len=ulen(&self.base.mHolderStr);while i<len{if context.r#type==8192&&i!=len-1{result.push_str("\\.");}else{result.push('.');}i+=1;}}
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        // java if(no left placeholders && mOffset!=0) append integer; mOffset=-1; repeat local decimal point.
        if c.mLDigitHolderCount == 0 && c.mOffset != 0 {
            let f = c.mFormatterContext.borrow();
            r.mNumFmtStr.push_str(&f.mInteger);
            r.mNumRealValue.push_str(&f.mInteger);
        }
        c.mOffset = -1;
        let dec = localeStrManager
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .getDecimalPoint();
        for _ in 0..ulen(&self.base.mHolderStr) {
            r.mNumFmtStr.push_str(&dec);
            r.mNumRealValue.push_str(&dec);
        }
    }
    fn decompile_value(&mut self, result: &mut String, context: &mut PieceContext) {
        self.base.repeatLocalChar(
            result,
            localeStrManager
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .getDecimalPoint()
                .chars()
                .next()
                .unwrap(),
        );
    }
}

// java public static class PlaceHolderFillChar extends PlaceHolder
pub struct PlaceHolderFillChar {
    pub base: PlaceHolder,
}
impl PlaceHolderFillChar {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(10),
        }
    }
}
impl Holder for PlaceHolderFillChar {
    // java public void compile(StringBuffer result,AnalyzeContext context){if(this.mHolderStr.length()>0){result.append('*').append(this.mHolderStr.charAt(0));}}
    fn compile_value(&mut self,result:&mut String,_context:&mut AnalyzeContext){if !self.base.mHolderStr.is_empty(){result.push('*');result.push_str(&usub(&self.base.mHolderStr,0,1));}}
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        c.mFillPos = ulen(&r.mNumFmtStr) as i32;
        c.mFillChar = self.base.mHolderStr.chars().next().unwrap_or(' ');
    }
    fn decompile_value(&mut self, result: &mut String, context: &mut PieceContext) {
        if let Some(ch) = self.base.mHolderStr.chars().next() {
            result.push('*');
            result.push(ch);
        }
    }
}

// java public static class PlaceHolderUnderline extends PlaceHolder
pub struct PlaceHolderUnderline {
    pub base: PlaceHolder,
}
impl PlaceHolderUnderline {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(11),
        }
    }
}
impl Holder for PlaceHolderUnderline {
    // java public void compile(StringBuffer result,AnalyzeContext context){if(this.mHolderStr.length()>0){result.append('_').append(this.mHolderStr.charAt(0));}}
    fn compile_value(&mut self,result:&mut String,_context:&mut AnalyzeContext){if !self.base.mHolderStr.is_empty(){result.push('_');result.push_str(&usub(&self.base.mHolderStr,0,1));}}
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        if let Some(ch) = self.base.mHolderStr.chars().next() {
            r.mNumFmtStr.push(
                if (' '..='\u{7f}').contains(&ch) || ('｡'..='ﾟ').contains(&ch) {
                    ' '
                } else {
                    '　'
                },
            );
        }
    }
    fn decompile_value(&mut self, result: &mut String, context: &mut PieceContext) {
        if let Some(ch) = self.base.mHolderStr.chars().next() {
            result.push('_');
            result.push(ch);
        }
    }
}

// java public static class PlaceHolderText extends PlaceHolder
pub struct PlaceHolderText {
    pub base: PlaceHolder,
}
impl PlaceHolderText {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(12),
        }
    }
}
impl Holder for PlaceHolderText {
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        // java String formatText=...;if(formatText.length()!=0) for(holder length) append text.
        let f = c.mFormatterContext.borrow();
        let text = f
            .mFormatText
            .as_ref()
            .expect("PlaceHolderText numeric value");
        if !text.is_empty() {
            for _ in 0..ulen(&self.base.mHolderStr) {
                r.mNumFmtStr.push_str(text);
            }
        }
    }
}

// java public static class PlaceHolderColor extends PlaceHolder
pub struct PlaceHolderColor {
    pub base: PlaceHolder,
}
impl PlaceHolderColor {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(16),
        }
    }
}
impl Holder for PlaceHolderColor {
    // java public void compile(StringBuffer result,AnalyzeContext context){String innerColor=BuiltinColors.getInnerColorName(this.mHolderStr);result.append("[").append(innerColor).append("]");}
    fn compile_value(&mut self,result:&mut String,_context:&mut AnalyzeContext){let innerColor=BuiltinColors::getInnerColorName(&self.base.mHolderStr);result.push('[');result.push_str(&innerColor);result.push(']');}
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        r.mNumFmtColorIdx = BuiltinColors::getColorIdx(&self.base.mHolderStr);
    }
    fn decompile_value(&mut self, result: &mut String, context: &mut PieceContext) {
        result.push('[');
        result.push_str(&BuiltinColors::getOuterColorName(&self.base.mHolderStr));
        result.push(']');
    }
}

// java public static class PlaceHolderCondition extends PlaceHolder
pub struct PlaceHolderCondition {
    pub base: PlaceHolder,
}
impl PlaceHolderCondition {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(17),
        }
    }
}
impl Holder for PlaceHolderCondition {
    // java public void compile(StringBuffer result,AnalyzeContext context) {
    fn compile_value(&mut self,result:&mut String,_context:&mut AnalyzeContext){
        // java String szPattern=VariantToken.DEFAULT_BRACKET_CHILDREN[3];Pattern pattern=Pattern.compile(szPattern);Matcher matcher=pattern.matcher(this.mHolderStr);if(matcher.lookingAt()){
        let szPattern=super::VariantToken::VariantToken::DEFAULT_BRACKET_CHILDREN[3];let pattern=regex::Regex::new(szPattern).unwrap();if let Some(matcher)=pattern.find(&self.base.mHolderStr).filter(|m|m.start()==0){
            // java String comparator=matcher.group();String value=this.mHolderStr.substring(matcher.end(),this.mHolderStr.length());String doublePattern="\\s*(-?\\d*\\.?\\d*)";String sciencePattern="\\s*(-?\\d*.?\\d*[Ee]\\+?-?\\d*.?\\d*)";matcher=Pattern.compile(sciencePattern).matcher(value);if(!matcher.lookingAt()){matcher=Pattern.compile(doublePattern).matcher(value);}
            let comparator=matcher.as_str();let value=&self.base.mHolderStr[matcher.end()..];let doublePattern=r"[ \t\n\x0b\x0c\r]*(-?[0-9]*\.?[0-9]*)";let sciencePattern=r"[ \t\n\x0b\x0c\r]*(-?[0-9]*.?[0-9]*[Ee]\+?-?[0-9]*.?[0-9]*)";let science=regex::Regex::new(sciencePattern).unwrap();let decimal=regex::Regex::new(doublePattern).unwrap();let mut matcher=science.find(value).filter(|m|m.start()==0);if matcher.is_none(){matcher=decimal.find(value).filter(|m|m.start()==0);}
            // java if(matcher.lookingAt()){String szNum=matcher.group();double dNum=0.0F;try{dNum=Double.parseDouble(szNum);}catch(Exception var14){}szNum=NumberToStr.double2String(dNum,15);result.append('[').append(comparator).append(szNum).append(']');}else{result.append('[').append(comparator).append('0').append(']');}
            if let Some(matcher)=matcher{let szNum=matcher.as_str();let mut dNum=0.0;let parsed=szNum.trim_matches(|c:char|c<=' ').parse::<f64>();if let Ok(value)=parsed{dNum=value;}let szNum=crate::cn::wps::kfc::numfmt::util::NumberToStr::NumberToStr::double2String_precision(dNum,15);result.push('[');result.push_str(comparator);result.push_str(&szNum);result.push(']');}else{result.push('[');result.push_str(comparator);result.push('0');result.push(']');}
        }
    }
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {}
    fn decompile_value(&mut self, result: &mut String, context: &mut PieceContext) {
        result.push('[');
        result.push_str(&self.base.mHolderStr);
        result.push(']');
    }
}

// java public static class PlaceHolderDBNum extends PlaceHolder
pub struct PlaceHolderDBNum {
    pub base: PlaceHolder,
}
impl PlaceHolderDBNum {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(19),
        }
    }
}
impl Holder for PlaceHolderDBNum {
    // java public void compile(StringBuffer result,AnalyzeContext context){String DBNum="DBNum";assert this.mHolderStr.charAt(DBNum.length())>='0'&&this.mHolderStr.charAt(DBNum.length())<='4';result.append('[').append(DBNum).append(this.mHolderStr.charAt(DBNum.length())).append(']');}
    fn compile_value(&mut self,result:&mut String,_context:&mut AnalyzeContext){let DBNum="DBNum";let c=usub(&self.base.mHolderStr,DBNum.len(),DBNum.len()+1);debug_assert!(("0"..="4").contains(&c.as_str()));result.push('[');result.push_str(DBNum);result.push_str(&c);result.push(']');}
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {}
    fn decompile_value(&mut self, result: &mut String, context: &mut PieceContext) {
        result.push('[');
        result.push_str(&self.base.mHolderStr);
        result.push(']');
    }
}

// java public static class PlaceHolderNCL extends PlaceHolder
pub struct PlaceHolderNCL {
    pub base: PlaceHolder,
}
impl PlaceHolderNCL {
    // java private String normalNCL(String src,int start,int end) {
    fn normalNCL(&self,src:&str,start:usize,end:usize)->String{
        // Native byte offsets transport the original substring boundary; all
        // following length, truncation and character operations are UTF16.
        let source:Vec<_>=src[start..end].encode_utf16().collect();
        // java int subLen=end-start;StringBuffer buffer=new StringBuffer();buffer.append(src,subLen>8?end-8:start,end);int sum=0;int i=0;
        let subLen=source.len();let buffer=&source[if subLen>8{subLen-8}else{0}..];let mut sum=0i32;let mut i=0;
        // java for(int len=buffer.length();i<len;++i){char c=buffer.charAt(i);sum+=this.getActualValue(c)<<(len-i-1)*4;}
        let len=buffer.len();while i<len{let c=buffer[i];sum=sum.wrapping_add(self.getActualValue(c).wrapping_shl(((len-i-1)*4)as u32));i+=1;}
        // java buffer.setLength(0);buffer.append(Integer.toHexString(sum));i=0;for(int len=buffer.length();i<len;++i){char c=buffer.charAt(i);if(Character.isLowerCase(c)){buffer.setCharAt(i,Character.toUpperCase(c));}}return buffer.toString();
        let mut buffer=format!("{:x}",sum as u32).into_bytes();i=0;let len=buffer.len();while i<len{let c=buffer[i];if c.is_ascii_lowercase(){buffer[i]=c.to_ascii_uppercase();}i+=1;}String::from_utf8(buffer).unwrap()
    }
    // java private int getActualValue(char c){if(Character.isDigit(c)){return c-48;}else if(Character.isLetter(c)){char c1=Character.toLowerCase(c);return c1-97+10;}else{return c-97;}}
    fn getActualValue(&self,c:u16)->i32{use crate::native_runtime::Character;if Character::isDigit(c){c as i32-48}else if Character::isLetter(c){let c1=Character::toLowerCase(c);c1 as i32-97+10}else{c as i32-97}}
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(18),
        }
    }
}
impl Holder for PlaceHolderNCL {
    // java public void compile(StringBuffer result,AnalyzeContext context){int index=this.mHolderStr.indexOf(45);if(index==-1){super.compile(result,context);}else{result.append('[').append(this.mHolderStr,0,index);if(index+1<this.mHolderStr.length()){String ncl=this.normalNCL(this.mHolderStr,index+1,this.mHolderStr.length());result.append('-').append(ncl);}result.append(']');}}
    fn compile_value(&mut self,result:&mut String,_context:&mut AnalyzeContext){if let Some(index)=self.base.mHolderStr.find('-'){result.push('[');result.push_str(&self.base.mHolderStr[..index]);if index+1<self.base.mHolderStr.len(){let ncl=self.normalNCL(&self.base.mHolderStr,index+1,self.base.mHolderStr.len());result.push('-');result.push_str(&ncl);}result.push(']');}else{result.push('[');result.push_str(&self.base.mHolderStr);result.push(']');}}
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        // java append currency between $ and -, unless RMB uppercase scale suppresses it.
        let text = &self.base.mHolderStr;
        let end = text.find('-').unwrap_or(text.len());
        if ulen(text) > 1 && !(s.mIsRMBScale && text == "$RMB" && s.mNumeralShape == 31) {
            r.mNumFmtStr.push_str(&text[1..end]);
        }
    }
    fn decompile_value(&mut self, result: &mut String, context: &mut PieceContext) {
        result.push('[');
        result.push_str(&self.base.mHolderStr);
        result.push(']');
    }
}

// java public static class PlaceHolderFraction extends PlaceHolder
pub struct PlaceHolderFraction {
    pub base: PlaceHolder,
}
impl PlaceHolderFraction {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(5),
        }
    }
}
impl Holder for PlaceHolderFraction {
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        if !c.mFormatterContext.borrow().mInteger.is_empty() {
            r.mNumFmtStr.push_str(&self.base.mHolderStr);
        } else {
            r.mNumFmtStr.push(' ');
        }
    }
}

// java public static class PlaceHolderScience extends PlaceHolder
pub struct PlaceHolderScience {
    pub base: PlaceHolder,
}
impl PlaceHolderScience {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(7),
        }
    }
}
impl Holder for PlaceHolderScience {
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        r.mNumFmtStr.push_str(&self.base.mHolderStr);
    }
}

// java public static class PlaceHolderPercent extends PlaceHolder
pub struct PlaceHolderPercent {
    pub base: PlaceHolder,
}
impl PlaceHolderPercent {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(6),
        }
    }
}
impl Holder for PlaceHolderPercent {
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        r.mNumFmtStr.push_str(&self.base.mHolderStr);
    }
}

// java public static class PlaceHolderColon extends PlaceHolder
pub struct PlaceHolderColon {
    pub base: PlaceHolder,
}
impl PlaceHolderColon {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(35),
        }
    }
}
impl Holder for PlaceHolderColon {
    // java public void compile(StringBuffer result,AnalyzeContext context){if(context.type==8192){result.append(':');}else{result.append("\\:");}}
    fn compile_value(&mut self,result:&mut String,context:&mut AnalyzeContext){if context.r#type==8192{result.push(':');}else{result.push_str("\\:");}}
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        r.mNumFmtStr.push_str(&self.base.mHolderStr);
    }
}

// java public static class PlaceHolderPlainDigit extends PlaceHolder
pub struct PlaceHolderPlainDigit {
    pub base: PlaceHolder,
}
impl PlaceHolderPlainDigit {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(15),
        }
    }
}
impl Holder for PlaceHolderPlainDigit {
    // java public void compile(StringBuffer result,AnalyzeContext context){if(context.fractionPos!=-1&&context.curPos>context.fractionPos&&context.curPos<=context.fractionEndPos){super.compile(result,context);}else{int i=0;for(int len=this.mHolderStr.length();i<len;++i){result.append('\\').append(this.mHolderStr.charAt(i));}}}
    fn compile_value(&mut self,result:&mut String,context:&mut AnalyzeContext){if context.fractionPos!=-1&&context.curPos>context.fractionPos&&context.curPos<=context.fractionEndPos{result.push_str(&self.base.mHolderStr);}else{let mut i=0;let len=ulen(&self.base.mHolderStr);while i<len{result.push('\\');result.push_str(&usub(&self.base.mHolderStr,i,i+1));i+=1;}}}
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        r.mNumFmtStr.push_str(&self.base.mHolderStr);
    }
}

// java public static class PlaceHolderPlainText extends PlaceHolder
pub struct PlaceHolderPlainText {
    pub base: PlaceHolder,
}
impl PlaceHolderPlainText {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(0),
        }
    }
}
impl Holder for PlaceHolderPlainText {
    // java public void compile(StringBuffer result,AnalyzeContext context){int i=0;for(int len=this.mHolderStr.length();i<len;++i){char c=this.mHolderStr.charAt(i);if(c!=165){result.append('\\').append(c);}else{result.append('"').append(c).append('"');}}}
    fn compile_value(&mut self,result:&mut String,_context:&mut AnalyzeContext){let mut i=0;let len=ulen(&self.base.mHolderStr);while i<len{let c=usub(&self.base.mHolderStr,i,i+1);if c!="¥"{result.push('\\');result.push_str(&c);}else{result.push('"');result.push_str(&c);result.push('"');}i+=1;}}
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        r.mNumFmtStr.push_str(&self.base.mHolderStr);
    }
    fn decompile_value(&mut self, result: &mut String, context: &mut PieceContext) {
        for ch in self.base.mHolderStr.chars() {
            if !"$+-()!^~{}<>= ,'".contains(ch)
                && !"cfijklopqtuvwxz".contains(ch.to_ascii_lowercase())
            {
                if (ch as u32) < 256 {
                    result.push_str(
                        &localeStrManager
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner)
                            .getEscape(),
                    );
                    result.push(ch);
                } else {
                    result.push('"');
                    result.push(ch);
                    result.push('"');
                }
            } else {
                result.push(ch);
            }
        }
    }
}

// java public static class PlaceHolderDoubleQuatation extends PlaceHolder
pub struct PlaceHolderDoubleQuatation {
    pub base: PlaceHolder,
}
impl PlaceHolderDoubleQuatation {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(13),
        }
    }
}
impl Holder for PlaceHolderDoubleQuatation {
    // java public void compile(StringBuffer result,AnalyzeContext context){if(this.mHolderStr.indexOf("\\")!=-1&&GlobalEnv.getDefaultLCID()==1041){result.append('"').append("¥").append('"');}else{result.append('"').append(this.mHolderStr).append('"');}}
    fn compile_value(&mut self,result:&mut String,_context:&mut AnalyzeContext){if self.base.mHolderStr.contains('\\')&&GlobalEnv::getDefaultLCID()==1041{result.push_str("\"¥\"");}else{result.push('"');result.push_str(&self.base.mHolderStr);result.push('"');}}
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        r.mNumFmtStr.push_str(&self.base.mHolderStr);
    }
    fn decompile_value(&mut self, result: &mut String, context: &mut PieceContext) {
        if self.base.mHolderStr != "¥" && self.base.mHolderStr != "￥" {
            result.push('"');
            result.push_str(&self.base.mHolderStr);
            result.push('"');
        } else {
            result.push_str(&self.base.mHolderStr);
        }
    }
}

// java public static class PlaceHolderSingleQuatation extends PlaceHolder
pub struct PlaceHolderSingleQuatation {
    pub base: PlaceHolder,
}
impl PlaceHolderSingleQuatation {
    pub fn new() -> Self {
        Self {
            base: PlaceHolder::new(14),
        }
    }
}
impl Holder for PlaceHolderSingleQuatation {
    fn base(&self) -> &PlaceHolder {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolder {
        &mut self.base
    }
    fn format_value(&mut self, c: &mut PlaceHolderContext, s: &NumFmtStyle, r: &mut NumFmtResult) {
        r.mNumFmtStr.push_str(&self.base.mHolderStr);
    }
}

// rust: Existing Word facade, outside the Excel read closure.
pub struct WPlaceHolderX {}
pub struct WPlaceHolderApostrophe {}
pub struct WPlaceHolderSign {}
impl WPlaceHolderX {
    // Java: WPlaceHolderX()
    pub fn new() -> Self {
        // Java: super((short)42, true);
        WPlaceHolderX {}
    }

    // Java: void format(PlaceHolderContext context, NumFmtStyle numFmtStyle, NumFmtResult result)
    pub fn format(&self, _context: i32, _num_fmt_style: i32, _result: i32) {
        // Java: this.mIsFillChar = context.mOffset >= 0;
        // Java: super.format(context, numFmtStyle, result);
        todo!("WPlaceHolderX::format")
    }

    // Java: char getFillChar()
    pub fn getFillChar(&self) -> char {
        // Java: return ' ';
        ' '
    }
}
impl WPlaceHolderApostrophe {
    // Java: WPlaceHolderApostrophe()
    pub fn new() -> Self {
        // Java: super((short)44);
        WPlaceHolderApostrophe {}
    }

    // Java: void format(PlaceHolderContext context, NumFmtStyle numFmtStyle, NumFmtResult result)
    pub fn format(&self, _context: i32, _num_fmt_style: i32, _result: i32) {}
}
impl WPlaceHolderSign {
    // Java: WPlaceHolderSign()
    pub fn new() -> Self {
        // Java: super((short)43);
        WPlaceHolderSign {}
    }

    // Java: void format(PlaceHolderContext context, NumFmtStyle numFmtStyle, NumFmtResult result)
    pub fn format(&self, _context: i32, _num_fmt_style: i32, _result: i32) {
        // Java: char c = this.mHolderStr.charAt(0);
        // Java: boolean notNeg = context.mFormatterContext.mFormatValue >= (double)0.0F;
        // Java: if (notNeg && c == '-')
        {
            // Java: result.mNumFmtStr.append(' ');
        }
        // Java: else if (notNeg)
        {
            // Java: result.mNumFmtStr.append(this.mHolderStr);
        }
        // Java: else
        {
            // Java: for(int i = 0; i < this.mHolderStr.length(); ++i)
            {
                // Java: result.mNumFmtStr.append('-');
            }
        }
        todo!("WPlaceHolderSign::format")
    }
}
