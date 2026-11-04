// java package cn.wps.et.ss.constant;
use util::org::apache::poi::util::LanguageType::*;
use super::{RecogniseOption::RecogniseOption,NumfmtCategory::NumfmtCategory};
use crate::java_runtime::{Rc,RefCell};
static INST:std::sync::Mutex<Option<Rc<RefCell<ConstRecognisor>>>>=std::sync::Mutex::new(None);
// java private static final Set<Short> smLangs = new HashSet(); static initializer adds the original 13 short values.
const SM_LANGS:[i16;13]=[1036,1031,1040,1049,1034,1041,1033,2057,5124,1028,3076,4100,2052];
// java public class ConstRecognisor
pub struct ConstRecognisor {
    // java private StateMachine stateMachine = null;
    stateMachine:Option<super::super::statemachine::StateMachine::StateMachine>,
    // java private boolean preferStateMachine = false;
    preferStateMachine: bool,
    // java private boolean useStateMachine = false;
    useStateMachine: bool,
    // java private final RecogniseOption options = new RecogniseOption();
    options:RecogniseOption,
}
impl ConstRecognisor {
    // java public ConstRecognisor() {
    pub fn ConstRecognisor() -> Self {
        let mut value=Self{stateMachine:None,preferStateMachine:false,useStateMachine:false,options:RecogniseOption::RecogniseOption()};
        // java this.setLangeuage(LanguageType.LANGUAGE_ENGLISH_US, true);
        value.setLangeuage__2(LANGUAGE_ENGLISH_US,true);value
    }
    // java public ConstRecognisor(LanguageType language) {
    pub fn ConstRecognisor__2(language:LanguageType)->Self{
        let mut value=Self{stateMachine:None,preferStateMachine:false,useStateMachine:false,options:RecogniseOption::RecogniseOption()};
        // java this.setLangeuage(language, true);
        value.setLangeuage__2(language,true);value
    }
    // java public static ConstRecognisor getInstance() {
    pub fn getInstance()->Rc<RefCell<Self>>{
        let mut inst=INST.lock().unwrap();
        // java if (inst == null) { inst = new ConstRecognisor(); }
        if inst.is_none(){*inst=Some(Rc::new(RefCell::new(Self::ConstRecognisor())));}
        // java return inst;
        inst.as_ref().unwrap().clone()
    }
    // java public void preferStateMachine(boolean b) {
    pub fn preferStateMachine(&mut self,b:bool){
        // java this.preferStateMachine = b;
        self.preferStateMachine=b;
        // java if (!this.preferStateMachine) { this.useStateMachine = false; } else { this.useStateMachine = smLangs.contains(this.options.language.getValue()); }
        if !self.preferStateMachine{self.useStateMachine=false;}else{self.useStateMachine=SM_LANGS.contains(&self.options.language.getValue());}
    }
    // java public void setKeepTailZero(boolean keep) { this.options.keepTailZero = keep; }
    pub fn setKeepTailZero(&mut self,keep:bool){self.options.keepTailZero=keep;}
    // java public void setForceText(boolean b) { this.options.forceText = b; }
    pub fn setForceText(&mut self,b:bool){self.options.forceText=b;}
    // java public void setUsingDate1904(boolean date1904) {
    pub fn setUsingDate1904(&mut self,date1904:bool){
        // java this.options.date1904 = date1904;
        self.options.date1904=date1904;
        // java if (this.stateMachine != null) { this.stateMachine.setUsing1904(date1904); }
        if let Some(state)=self.stateMachine.as_mut(){state.setUsing1904(date1904);}
    }
    // java public void setPreferedNumfmtCategory(NumfmtCategory cat) {
    pub fn setPreferedNumfmtCategory(&mut self,cat:NumfmtCategory){
        // java this.options.preferedNumfmt = cat;
        self.options.preferedNumfmt=cat;
        // java if (this.stateMachine != null) { this.stateMachine.setNumfmtCat(cat); }
        if let Some(state)=self.stateMachine.as_mut(){state.setNumfmtCat(cat);}
    }
    // java public void setLangeuage(LanguageType lang) { this.setLangeuage(lang, false); }
    pub fn setLangeuage(&mut self,lang:LanguageType){self.setLangeuage__2(lang,false);}
    // java public void setLangeuage(LanguageType lang, boolean force) {
    pub fn setLangeuage__2(&mut self,lang:LanguageType,force:bool){
        // java if (force || !this.options.language.equals(lang)) {
        if force||self.options.language!=lang{
            // java this.options.language = lang;
            self.options.language=lang;
            // java if (this.stateMachine != null) { this.stateMachine.setLanguage(lang.getValue()); }
            if let Some(state)=self.stateMachine.as_mut(){state.setLanguage(lang.getValue());}
            // java if (this.preferStateMachine) { this.useStateMachine = smLangs.contains(lang.getValue()); }
            if self.preferStateMachine{self.useStateMachine=SM_LANGS.contains(&lang.getValue());}
        }
    }
    // java public void setOption(RecogniseOption opt) {
    pub fn setOption(&mut self,opt:&RecogniseOption){
        // java this.setForceText(opt.forceText);
        self.setForceText(opt.forceText);
        // java this.setUsingDate1904(opt.date1904);
        self.setUsingDate1904(opt.date1904);
        // java this.setPreferedNumfmtCategory(opt.preferedNumfmt);
        self.setPreferedNumfmtCategory(opt.preferedNumfmt);
        // java this.setLangeuage(opt.language);
        self.setLangeuage(opt.language);
    }

// java private StateMachine getStateMachine() {
fn getStateMachine(&mut self)->&mut super::super::statemachine::StateMachine::StateMachine {
    // java if (this.stateMachine == null) {
    if self.stateMachine.is_none(){
        // java this.stateMachine = new StateMachine();
        self.stateMachine=Some(super::super::statemachine::StateMachine::StateMachine::new());
        // java this.stateMachine.setUsing1904(this.options.date1904);
        self.stateMachine.as_mut().unwrap().setUsing1904(self.options.date1904);
        // java this.stateMachine.setNumfmtCat(this.options.preferedNumfmt);
        self.stateMachine.as_mut().unwrap().setNumfmtCat(self.options.preferedNumfmt);
        // java this.stateMachine.setLanguage(this.options.language.getValue());
        self.stateMachine.as_mut().unwrap().setLanguage(self.options.language.getValue());
    }
    // java return this.stateMachine;
    self.stateMachine.as_mut().unwrap()
}

    // java public static String _wrapRegChar(char c) {
    pub fn _wrapRegChar(c:u16)->Vec<u16>{
        // java if ((c & '\uff00') != 0) { return Character.toString(c); }
        if c&0xff00!=0{vec![c]}
        // java else { return (c < 'a' || c > 'z') && (c < 'A' || c > 'Z') && (c < '0' || c > '9') ? "\\" + c : Character.toString(c); }
        else if !(97..=122).contains(&c)&&!(65..=90).contains(&c)&&!(48..=57).contains(&c){vec![92,c]}else{vec![c]}
    }
    // java static RecogniseResult getDateTime(String s, int beginIndex, int endIndex, RecogniseOption opt) {
    pub fn getDateTime(s:&[u16],beginIndex:usize,endIndex:usize,opt:&mut RecogniseOption)->Option<super::RecogniseResult::RecogniseResult>{
        use super::super::manual::{DateRecognisor::DateRecognisor,TimeRecognisor::TimeRecognisor};
        // java String subs = s.substring(beginIndex, endIndex); int subs_len = subs.length(); LangRes lang_res = opt.getLangRes();
        let subs=&s[beginIndex..endIndex];let subs_len=subs.len();let lang_res=opt.getLangRes();
        // java int endOfSpace; for(int startOfSpace = subs.indexOf(32); startOfSpace >= 0; startOfSpace = subs.indexOf(32, endOfSpace)) {
        let mut startOfSpace=subs.iter().position(|c|*c==32);
        while let Some(start)=startOfSpace{
            // java for(endOfSpace = startOfSpace + 1; endOfSpace < subs_len && subs.charAt(endOfSpace) == ' '; ++endOfSpace) { }
            let mut endOfSpace=start+1;while endOfSpace<subs_len&&subs[endOfSpace]==32{endOfSpace+=1;}
            // java if (endOfSpace >= subs_len) { break; }
            if endOfSpace>=subs_len{break;}
            // java RecogniseResult rs = DateRecognisor.getDate(subs, 0, startOfSpace, opt.date1904, opt.language, lang_res);
            let rs=DateRecognisor::getDate(subs,0,start,opt.date1904,opt.language,&lang_res);
            // java if (rs != null) {
            if let Some(rs)=rs{
                // java Double date = rs.getDblValue(); rs = TimeRecognisor.getCommonTime(subs, endOfSpace, subs_len, lang_res.SEPERATORS.DECIMAL_SEPARATOR);
                let date=rs.getDblValue().unwrap();let mut rs=TimeRecognisor::getCommonTime(subs,endOfSpace,subs_len,lang_res.SEPERATORS.DECIMAL_SEPARATOR);
                // java if (rs == null) { rs = TimeRecognisor.getEnglishAMPMTime(subs, endOfSpace, subs_len, lang_res.SEPERATORS.DECIMAL_SEPARATOR); }
                if rs.is_none(){rs=TimeRecognisor::getEnglishAMPMTime(subs,endOfSpace,subs_len,lang_res.SEPERATORS.DECIMAL_SEPARATOR);}
                // java if (rs == null && lang_res.AM != null && !lang_res.AM.equalsIgnoreCase("AM")) {
                if rs.is_none()&&lang_res.AM.as_ref().is_some_and(|am|!crate::java_runtime::Character::equalsIgnoreCaseAscii(&am.encode_utf16().collect::<Vec<_>>(),b"AM")){
                    // java rs = TimeRecognisor.getAMPMTime(subs, endOfSpace, subs_len, lang_res.SEPERATORS.DECIMAL_SEPARATOR, lang_res.AM, lang_res.PM);
                    rs=TimeRecognisor::getAMPMTime(subs,endOfSpace,subs_len,lang_res.SEPERATORS.DECIMAL_SEPARATOR,lang_res.AM.as_ref().unwrap(),lang_res.PM.as_ref().unwrap());
                }
                // java if (rs != null) {
                if let Some(rs)=rs{
                    // java RecogniseResult ret = new RecogniseResult(rs.getDblValue() + date); ret.setSuggestedNumfmt(22); ret.setDateFlag(); ret.setTimeFlag(); return ret;
                    let mut ret=super::RecogniseResult::RecogniseResult::RecogniseResult__2(Some(rs.getDblValue().unwrap()+date));ret.setSuggestedNumfmt__2(22);ret.setDateFlag();ret.setTimeFlag();return Some(ret);
                }
            }
            startOfSpace=subs[endOfSpace..].iter().position(|c|*c==32).map(|i|i+endOfSpace);
        }
        // java return null;
        None
    }
    // java static Boolean getBool(String s, int beginIndex, int endIndex) {
    pub(crate) fn getBool(s:&[u16],beginIndex:usize,endIndex:usize)->Option<bool>{
        // java if (s.equalsIgnoreCase("true")) { return true; }
        if crate::java_runtime::Character::equalsIgnoreCaseAscii(s,b"true"){Some(true)}
        // java else { return s.equalsIgnoreCase("false") ? false : null; }
        else if crate::java_runtime::Character::equalsIgnoreCaseAscii(s,b"false"){Some(false)}else{None}
    }
    // java static RecogniseResult getErr(String s, int beginIndex, int endIndex, RecogniseOption opt) {
    pub(crate) fn getErr(s:&[u16],beginIndex:usize,endIndex:usize,opt:&RecogniseOption)->Option<super::RecogniseResult::RecogniseResult>{
        use crate::java_runtime::Character::equalsIgnoreCaseAscii as eq;
        // java if (s.charAt(beginIndex) != '#') { return null; }
        if s[beginIndex]!=35{return None;}
        // java String ss = s.substring(beginIndex, endIndex); int errorCode = 0;
        let ss=&s[beginIndex..endIndex];let errorCode;
        // java if (ss.equalsIgnoreCase("#NULL!")) { errorCode = 0; }
        if eq(ss,b"#NULL!"){errorCode=0;}
        // java else if (ss.equalsIgnoreCase("#DIV/0!")) { errorCode = 7; }
        else if eq(ss,b"#DIV/0!"){errorCode=7;}
        // java else if (ss.equalsIgnoreCase("#VALUE!")) { errorCode = 15; }
        else if eq(ss,b"#VALUE!"){errorCode=15;}
        // java else if (ss.equalsIgnoreCase("#REF!")) { errorCode = 23; }
        else if eq(ss,b"#REF!"){errorCode=23;}
        // java else if (ss.equalsIgnoreCase("#NAME?")) { errorCode = 29; }
        else if eq(ss,b"#NAME?"){errorCode=29;}
        // java else if (ss.equalsIgnoreCase("#NUM!")) { errorCode = 36; }
        else if eq(ss,b"#NUM!"){errorCode=36;}
        else{
            // java if (!ss.equalsIgnoreCase("#N/A")) { return null; } errorCode = 42;
            if !eq(ss,b"#N/A"){return None;}errorCode=42;
        }
        // java RecogniseResult ret = new RecogniseResult((double)errorCode); ret.setSuggestedNumfmt(0); ret.setErrorFlag(); return ret;
        let mut ret=super::RecogniseResult::RecogniseResult::RecogniseResult__2(Some(errorCode as f64));ret.setSuggestedNumfmt__2(0);ret.setErrorFlag();Some(ret)
    }
    // java static RecogniseResult getString(String s, int beginIndex, int endIndex, RecogniseOption opt) {
    pub(crate) fn getString(s:&[u16],beginIndex:usize,endIndex:usize,opt:&RecogniseOption)->super::RecogniseResult::RecogniseResult{
        // java RecogniseResult ret = new RecogniseResult(s); ret.setSuggestedNumfmt(0);
        let mut ret=super::RecogniseResult::RecogniseResult::RecogniseResult(Some(s.to_vec()));ret.setSuggestedNumfmt__2(0);
        // java if (LangHelper.startsWithSingleQuote(s)) { ret.setSqleadingFlag(); }
        if super::LangHelper::LangHelper::startsWithSingleQuote(s){ret.setSqleadingFlag();}
        // java if (s.startsWith("http://") || s.startsWith("https://") || s.startsWith("ftp://") || s.startsWith("mailto://") || s.startsWith("file://") || s.startsWith("www.") || s.startsWith("ftp.") || hyperlink_email.matcher(s).matches()) { ret.setHyperlinkFlag(); }
        static EMAIL:std::sync::LazyLock<regex::Regex>=std::sync::LazyLock::new(||crate::java_runtime::Pattern::compileMatches(r"^[A-Za-z0-9]+([\w\-\+\.])*[A-Za-z0-9]@[A-Za-z0-9]+([\w\-\+\.])*[A-Za-z0-9]\.[A-Za-z]{2,8}$",false).unwrap());
        if s.starts_with(&"http://".encode_utf16().collect::<Vec<_>>())||s.starts_with(&"https://".encode_utf16().collect::<Vec<_>>())||s.starts_with(&"ftp://".encode_utf16().collect::<Vec<_>>())||s.starts_with(&"mailto://".encode_utf16().collect::<Vec<_>>())||s.starts_with(&"file://".encode_utf16().collect::<Vec<_>>())||s.starts_with(&"www.".encode_utf16().collect::<Vec<_>>())||s.starts_with(&"ftp.".encode_utf16().collect::<Vec<_>>())||String::from_utf16(s).is_ok_and(|s|EMAIL.is_match(&s)){ret.setHyperlinkFlag();}
        // java return ret;
        ret
    }
// java public static String preprocessConstRecognize(String s) {
pub fn preprocessConstRecognize(s:&[u16])->Option<Vec<u16>> {
    // java if (s != null && s.length() != 0) {
    if !s.is_empty(){
        // java s = LangHelper.toHalfHornString(s);
        let s=super::LangHelper::LangHelper::toHalfHornString(s);
        // java int slen = s.length(); StringBuilder ret = new StringBuilder(slen); int continualSpaceCount = 0;
        let slen=s.len();let mut ret=Vec::with_capacity(slen);let mut continualSpaceCount=0;
        // java for(int i = 0; i < slen; ++i) { char w = s.charAt(i);
        for i in 0..slen{let mut w=s[i];
            // java if (w == '\n') { return null; }
            if w==10{return None;}
            // java if (w == ' ') { ++continualSpaceCount; }
            if w==32{continualSpaceCount+=1;}
            else{
                // java if (continualSpaceCount > 0) {
                if continualSpaceCount>0{
                    // java if (ret.length() > 0) { for(int k = 0; k < continualSpaceCount; ++k) { ret.append(' '); } }
                    if !ret.is_empty(){for _ in 0..continualSpaceCount{ret.push(32);}}
                    // java continualSpaceCount = 0;
                    continualSpaceCount=0;
                }
                // java w = Character.toLowerCase(w);
                w=crate::java_runtime::Character::toLowerCase(w);
                // java if (w == '，') { w = ','; }
                if w==0xff0c{w=44;}
                // java ret.append(w);
                ret.push(w);
            }
        }
        // java if (ret.length() != 0 && ret.length() < 32768) { return ret.toString(); } else { return null; }
        if !ret.is_empty()&&ret.len()<32768{Some(ret)}else{None}
    // java } else { return s; }
    }else{Some(s.to_vec())}
}

    // java private String getDigitsRes(String s, int index, DigitsResResult digitsResResult) {
    fn getDigitsRes(&self,s:&[u16],index:i32,digitsResResult:&mut super::super::multlang::res::DigitsResResult::DigitsResResult)->Option<Vec<u16>>{
        use super::super::multlang::res::DigitsRes::DigitsRes;
        // java digitsResResult.setContent(s);
        digitsResResult.setContent(Some(s));
        // java s = DigitsRes.translateDigits(s, DigitsRes.DIGITS[index]);
        let s=DigitsRes::translateDigits(s,&DigitsRes::DIGITS[index as usize].encode_utf16().collect::<Vec<_>>());
        // java if (s != null && s.length() != 0) {
        if !s.is_empty(){
            // java if (digitsResResult.diff(s)) { digitsResResult.flag = DigitsRes.addFlag(index, digitsResResult.flag); }
            if digitsResResult.diff(Some(&s)){digitsResResult.flag=DigitsRes::addFlag(index,digitsResResult.flag);}
            // java return s;
            Some(s)
        // java } else { return null; }
        }else{None}
    }
// java public RecogniseResult recognise(String s) {
pub fn recognise(&mut self,s:Option<&[u16]>)->crate::cn::wps::et::ss::formula::evaluator::EvalResult<Option<super::RecogniseResult::RecogniseResult>> {
    use super::{LangHelper::LangHelper,RecogniseResult::RecogniseResult,PercentRecognisor::PercentRecognisor};
    use super::super::{manual::{IntegerRecognisor::IntegerRecognisor,FloatRecognisor::FloatRecognisor,SenticRecognisor::SenticRecognisor,FractionRecognisor::FractionRecognisor,DateRecognisor::DateRecognisor,TimeRecognisor::TimeRecognisor,CurrencyRecognisor::CurrencyRecognisor},multlang::res::DigitsResResult::DigitsResResult};
    // java if (s != null && s.length() != 0) {
    if let Some(s)=s.filter(|s|!s.is_empty()){
        // java try { if (this.options.forceText || this.options.preferedNumfmt == NumfmtCategory.TEXT || LangHelper.startsWithSingleQuote(s)) { return getString(s, 0, s.length(), this.options); } }
        let forced=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||if self.options.forceText||self.options.preferedNumfmt==NumfmtCategory::TEXT||LangHelper::startsWithSingleQuote(s){Some(Self::getString(s,0,s.len(),&self.options))}else{None}));
        // java catch (Throwable e) { IOLog.d(TAG, "", e); return null; }
        let forced=match forced{Ok(v)=>v,Err(_)=>return Ok(None)};if forced.is_some(){return Ok(forced);}
        // java if (LangHelper.isOprEqual(s.charAt(0)) && s.length() > 1) { return null; } else {
        if LangHelper::isOprEqual(s[0])&&s.len()>1{Ok(None)}else{
            // java s = preprocessConstRecognize(s); if (s == null) { return null; } else if (this.useStateMachine) {
            let s=Self::preprocessConstRecognize(s);if s.is_none(){Ok(None)}else if self.useStateMachine{
                // java RecogniseResultRl rs = this.getStateMachine().parse(s); return rs == null ? null : new RecogniseResult(rs);
                Ok(self.getStateMachine().parse(&s.unwrap()).map(RecogniseResult::fromState))
            // java } else {
            }else{
                let s=s.unwrap();
                // java DigitsResResult digitsResResult = new DigitsResResult(); digitsResResult.reset();
                let mut digitsResResult=DigitsResResult::new();digitsResResult.reset();
                // java s = this.getDigitsRes(s, 1, digitsResResult);
                let s=self.getDigitsRes(&s,1,&mut digitsResResult).expect("NullPointerException");
                // java s = this.getDigitsRes(s, 2, digitsResResult);
                let s=self.getDigitsRes(&s,2,&mut digitsResResult).expect("NullPointerException");
                // java s = this.getDigitsRes(s, 3, digitsResResult);
                let s=self.getDigitsRes(&s,3,&mut digitsResResult).expect("NullPointerException");
                // java s = this.getDigitsRes(s, 4, digitsResResult);
                let s=self.getDigitsRes(&s,4,&mut digitsResResult).expect("NullPointerException");
                // java Boolean b = getBool(s, 0, s.length()); if (b != null) {
                let b=Self::getBool(&s,0,s.len());if let Some(b)=b{
                    // java RecogniseResult ret = new RecogniseResult(b); ret.setSuggestedNumfmt(0); return ret;
                    let mut ret=RecogniseResult::RecogniseResult__3(Some(b));ret.setSuggestedNumfmt__2(0);Ok(Some(ret))
                // java } else {
                }else{
                    // java RecogniseResult rs = IntegerRecognisor.getInt(s, 0, s.length(), this.options.getLangRes().SEPERATORS.GROUP_SEPARATOR, this.options.getLangRes().SEPERATORS.DECIMAL_SEPARATOR);
                    let mut rs=IntegerRecognisor::getInt(&s,0,s.len(),self.options.getLangRes().SEPERATORS.GROUP_SEPARATOR,self.options.getLangRes().SEPERATORS.DECIMAL_SEPARATOR);
                    // java if (rs != null) { RecogniseResult drr = digitsResResult.getResult(); if (drr != null) { return drr; } }
                    if rs.is_some(){let drr=digitsResResult.getResult();if drr.is_some(){return Ok(drr);}}
                    // java if (rs == null) { rs = FloatRecognisor.getFloat(s, 0, s.length(), this.options.getLangRes().SEPERATORS.GROUP_SEPARATOR, this.options.getLangRes().SEPERATORS.DECIMAL_SEPARATOR); }
                    if rs.is_none(){rs=FloatRecognisor::getFloat(&s,0,s.len(),self.options.getLangRes().SEPERATORS.GROUP_SEPARATOR,self.options.getLangRes().SEPERATORS.DECIMAL_SEPARATOR);}
                    // java if (rs != null && this.options.keepTailZero && this.matchTailZero(rs, s)) { return rs; } else {
                    if rs.as_mut().is_some_and(|rs|self.options.keepTailZero&&self.matchTailZero(rs,&s)){Ok(rs)}else{
                        // java if (rs == null) { rs = SenticRecognisor.getSentic(s, 0, s.length(), this.options.getLangRes().SEPERATORS.GROUP_SEPARATOR, this.options.getLangRes().SEPERATORS.DECIMAL_SEPARATOR); }
                        if rs.is_none(){rs=SenticRecognisor::getSentic(&s,0,s.len(),self.options.getLangRes().SEPERATORS.GROUP_SEPARATOR,self.options.getLangRes().SEPERATORS.DECIMAL_SEPARATOR);}
                        // java if (rs != null) {
                        if let Some(mut rs)=rs{
                            // java if (this.options.preferedNumfmt == NumfmtCategory.PERCENTAGE && !rs.hasPercentageFlag()) {
                            if self.options.preferedNumfmt==NumfmtCategory::PERCENTAGE&&!rs.hasPercentageFlag(){
                                // java double v = BigDecimal.valueOf(rs.getDblValue()).divide(BigDecimal.valueOf((double)100.0F)).doubleValue();
                                let v=crate::java_runtime::BigDecimal::valueOf(rs.getDblValue().unwrap()).divide(&crate::java_runtime::BigDecimal::valueOf(100.0)).doubleValue();
                                // java rs.setValue(v); rs.setPercentageFlage();
                                rs.setValue__2(Some(v));rs.setPercentageFlage();
                            }
                            // java return rs;
                            Ok(Some(rs))
                        // java } else {
                        }else{
                            // java rs = PercentRecognisor.getPercent(s, 0, s.length(), this.options.getLangRes().SEPERATORS.GROUP_SEPARATOR, this.options.getLangRes().SEPERATORS.DECIMAL_SEPARATOR);
                            let rs=PercentRecognisor::getPercent(&s,0,s.len(),self.options.getLangRes().SEPERATORS.GROUP_SEPARATOR,self.options.getLangRes().SEPERATORS.DECIMAL_SEPARATOR);
                            // java if (rs != null) { return rs; } else {
                            if rs.is_some(){Ok(rs)}else{
                                // java rs = FractionRecognisor.getFraction(s, 0, s.length(), this.options.getLangRes(), this.options.preferedNumfmt);
                                let rs=FractionRecognisor::getFraction(&s,0,s.len(),&self.options.getLangRes(),self.options.preferedNumfmt);
                                // java if (rs != null) { return rs; } else {
                                if rs.is_some(){Ok(rs)}else{
                                    // java rs = getErr(s, 0, s.length(), this.options);
                                    let rs=Self::getErr(&s,0,s.len(),&self.options);
                                    // java if (rs != null) { return rs; } else {
                                    if rs.is_some(){Ok(rs)}else{
                                        // java rs = DateRecognisor.getDate(s, 0, s.length(), this.options.date1904, this.options.language, this.options.getLangRes());
                                        let rs=DateRecognisor::getDate(&s,0,s.len(),self.options.date1904,self.options.language,&self.options.getLangRes());
                                        // java if (rs != null) { return rs; } else {
                                        if rs.is_some(){Ok(rs)}else{
                                            // java rs = TimeRecognisor.getTime(s, 0, s.length(), this.options.language, this.options.getLangRes());
                                            let rs=TimeRecognisor::getTime(&s,0,s.len(),self.options.language,&self.options.getLangRes());
                                            // java if (rs != null) { return rs; } else {
                                            if rs.is_some(){Ok(rs)}else{
                                                // java rs = getDateTime(s, 0, s.length(), this.options);
                                                let rs=Self::getDateTime(&s,0,s.len(),&mut self.options);
                                                // java if (rs != null) { return rs; } else {
                                                if rs.is_some(){Ok(rs)}else{
                                                    // java rs = CurrencyRecognisor.getCurrency(s, 0, s.length(), this.options.getLangRes()); return rs != null ? rs : null;
                                                    let rs=CurrencyRecognisor::getCurrency(&s,0,s.len(),&self.options.getLangRes());Ok(rs)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    // java } else { return null; }
    }else{Ok(None)}
}
    // java private boolean matchTailZero(RecogniseResult rs, String s) {
    fn matchTailZero(&self,rs:&mut super::RecogniseResult::RecogniseResult,s:&[u16])->bool{
        // java if (rs.getBuiltInNumfmtIdx() <= 0 && (this.options.preferedNumfmt != NumfmtCategory.PERCENTAGE || rs.hasPercentageFlag())) {
        if rs.getBuiltInNumfmtIdx()<=0&&(self.options.preferedNumfmt!=NumfmtCategory::PERCENTAGE||rs.hasPercentageFlag()){
            // java Matcher matcher = NUM_TAIL_ZERO.matcher(s); if (!matcher.matches()) { return false; }
            static PATTERN:std::sync::LazyLock<regex::Regex>=std::sync::LazyLock::new(||crate::java_runtime::Pattern::compileMatches(r"[+-]?(\d+)\.(\d*?)(0+)$",false).unwrap());
            let Ok(s)=String::from_utf16(s)else{return false;};let Some(matcher)=PATTERN.captures(&s)else{return false;};
            // java String group1 = matcher.group(1); String group2 = matcher.group(2); String group3 = matcher.group(3);
            let group1=&matcher[1];let group2=&matcher[2];let group3=&matcher[3];
            // java if (group1.length() + group2.length() < 15) {
            if group1.encode_utf16().count()+group2.encode_utf16().count()<15{
                // java StringBuilder formatCode = new StringBuilder("0."); int length = group2.length();
                let mut formatCode=String::from("0.");let length=group2.encode_utf16().count();
                // java for(int i = 0; i < length; ++i) { formatCode.append('0'); }
                for _ in 0..length{formatCode.push('0');}
                // java int i = 0; for(int len = group3.length(); i < len; ++i) { formatCode.append("0"); }
                for _ in 0..group3.encode_utf16().count(){formatCode.push('0');}
                // java rs.setSuggestedNumfmt(-1); rs.setSuggestedNumfmt(formatCode.toString());
                rs.setSuggestedNumfmt__2(-1);rs.setSuggestedNumfmt(Some(formatCode));
            }
            // java return true;
            true
        // java } else { return false; }
        }else{false}
    }
}
