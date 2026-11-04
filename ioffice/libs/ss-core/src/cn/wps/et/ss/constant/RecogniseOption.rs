use super::NumfmtCategory::NumfmtCategory;
use crate::{java_runtime::{Rc,RefCell},cn::wps::et::ss::multlang::LangRes::LangRes};
use util::org::apache::poi::util::LanguageType::{LanguageType,LANGUAGE_ENGLISH_US};
// java public final class RecogniseOption {
pub struct RecogniseOption{
    // java boolean keepTailZero;
    pub keepTailZero:bool,
    // java boolean forceText;
    pub forceText:bool,
    // java boolean date1904;
    pub date1904:bool,
    // java NumfmtCategory preferedNumfmt;
    pub preferedNumfmt:NumfmtCategory,
    // java LanguageType language;
    pub language:LanguageType,
    // java private LangRes langRes = null;
    langRes:RefCell<Option<Rc<LangRes>>>,
}
impl RecogniseOption{
    // java public RecogniseOption() {
    pub fn RecogniseOption()->Self{
        // java this.keepTailZero = false;
        // java this.forceText = false;
        // java this.date1904 = false;
        // java this.preferedNumfmt = NumfmtCategory.GENERAL;
        // java this.language = LanguageType.LANGUAGE_ENGLISH_US;
        Self{keepTailZero:false,forceText:false,date1904:false,preferedNumfmt:NumfmtCategory::GENERAL,language:LANGUAGE_ENGLISH_US,langRes:RefCell::new(None)}
    }
    // java public RecogniseOption(boolean keepTailZero, boolean forceText, boolean date1904, NumfmtCategory numfmtCat, LanguageType lang) {
    pub fn RecogniseOption__2(keepTailZero:bool,forceText:bool,date1904:bool,numfmtCat:NumfmtCategory,lang:LanguageType)->Self{
        // java this.keepTailZero = keepTailZero;
        // java this.forceText = forceText;
        // java this.date1904 = date1904;
        // java this.preferedNumfmt = numfmtCat;
        // java this.language = lang;
        Self{keepTailZero,forceText,date1904,preferedNumfmt:numfmtCat,language:lang,langRes:RefCell::new(None)}
    }
    // java public void setForceText(boolean ft) { this.forceText = ft; }
    pub fn setForceText(&mut self,ft:bool){self.forceText=ft;}
    // java public LangRes getLangRes() {
    pub fn getLangRes(&self)->Rc<LangRes>{
        // java if (this.langRes == null) { this.langRes = LangRes.createByLangType(this.language); }
        if self.langRes.borrow().is_none(){*self.langRes.borrow_mut()=Some(Rc::new(LangRes::createByLangType(self.language)));}
        // java return this.langRes;
        self.langRes.borrow().as_ref().unwrap().clone()
    }
}
