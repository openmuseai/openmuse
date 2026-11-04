use crate::{java_runtime::{Rc,RefCell},cn::wps::et::ss::multlang::LangRes::LangRes};
use util::org::apache::poi::util::LanguageType::LanguageType;
pub type BookRefs=Rc<RefCell<Box<[Option<String>]>>>;
// java public class RendererOption {
pub struct RendererOption{
    // java public final char DECIMAL_SEPARATOR;
    pub DECIMAL_SEPARATOR:u16,
    // java public final char PARAMETER_SEPARATOR;
    pub PARAMETER_SEPARATOR:u16,
    // java public final char UNION_SEPARATOR;
    pub UNION_SEPARATOR:u16,
    // java public final char CONST_ARRAY_SEPARATOR;
    pub CONST_ARRAY_SEPARATOR:u16,
    // java public final char CONST_ARRAY_ROW_SEPARATOR;
    pub CONST_ARRAY_ROW_SEPARATOR:u16,
    // java public String[] ooxmlBookRefs=null;
    pub ooxmlBookRefs:Option<BookRefs>,
    // java public int formulaType;
    pub formulaType:i32,
}
#[cfg(test)]
#[path="RendererOption.review.rs"]
mod review;
// java private static RendererOption ooxmlRendererOption=null;
static ooxmlRendererOption:std::sync::LazyLock<RefCell<Option<Rc<RefCell<RendererOption>>>>>=std::sync::LazyLock::new(||RefCell::new(None));
impl RendererOption{
    // java private RendererOption(char decimalSeparator,char parameterSeparator,char unionSeparator,char constArraySeparator,char constArrayRowSeparator,String[] ooxmlBookRefs,int formulaType){this.DECIMAL_SEPARATOR=decimalSeparator;this.PARAMETER_SEPARATOR=parameterSeparator;this.UNION_SEPARATOR=unionSeparator;this.CONST_ARRAY_SEPARATOR=constArraySeparator;this.CONST_ARRAY_ROW_SEPARATOR=constArrayRowSeparator;this.ooxmlBookRefs=ooxmlBookRefs;this.formulaType=formulaType;}
    fn RendererOption(decimalSeparator:u16,parameterSeparator:u16,unionSeparator:u16,constArraySeparator:u16,constArrayRowSeparator:u16,ooxmlBookRefs:Option<BookRefs>,formulaType:i32)->Self{Self{DECIMAL_SEPARATOR:decimalSeparator,PARAMETER_SEPARATOR:parameterSeparator,UNION_SEPARATOR:unionSeparator,CONST_ARRAY_SEPARATOR:constArraySeparator,CONST_ARRAY_ROW_SEPARATOR:constArrayRowSeparator,ooxmlBookRefs,formulaType}}
    // java public static RendererOption createDefaultOption(){return new RendererOption('.',',',',',',',';',(String[])null,0);}
    pub fn createDefaultOption()->Rc<RefCell<Self>>{Rc::new(RefCell::new(Self::RendererOption(46,44,44,44,59,None,0)))}
    // java public static RendererOption createOOXMLRendererOption(String[] ooxmlBookRefs){if(ooxmlRendererOption==null){ooxmlRendererOption=new RendererOption('.',',',',',',',';',ooxmlBookRefs,6);}return ooxmlRendererOption;}
    pub fn createOOXMLRendererOption(ooxmlBookRefs:Option<BookRefs>)->Rc<RefCell<Self>>{let mut slot=ooxmlRendererOption.borrow_mut();if slot.is_none(){*slot=Some(Rc::new(RefCell::new(Self::RendererOption(46,44,44,44,59,ooxmlBookRefs,6))));}slot.as_ref().unwrap().clone()}
    // java public static RendererOption createByLanguage(LanguageType lang){LangRes ml=LangRes.createByLangType(lang);return new RendererOption(ml.SEPERATORS.DECIMAL_SEPARATOR,ml.SEPERATORS.PARAMETER_SEPARATOR,ml.SEPERATORS.UNION_SEPARATOR,ml.SEPERATORS.CONST_ARRAY_SEPARATOR,ml.SEPERATORS.CONST_ARRAY_ROW_SEPARATOR,(String[])null,0);}
    pub fn createByLanguage(lang:Option<LanguageType>)->Rc<RefCell<Self>>{let ml=LangRes::createByLangType(lang.expect("NullPointerException"));Rc::new(RefCell::new(Self::RendererOption(ml.SEPERATORS.DECIMAL_SEPARATOR,ml.SEPERATORS.PARAMETER_SEPARATOR,ml.SEPERATORS.UNION_SEPARATOR,ml.SEPERATORS.CONST_ARRAY_SEPARATOR,ml.SEPERATORS.CONST_ARRAY_ROW_SEPARATOR,None,0)))}
    // java public static void destroy(){ooxmlRendererOption=null;}
    pub fn destroy(){*ooxmlRendererOption.borrow_mut()=None;}
}
