use crate::java_runtime::Rc;

use super::KmoBook::KmoBook;

// Java: cn.wps.moss.app.KmoBooks.loadTemplates(KmoBook).
pub struct KmoBooks{
    // java List<KmoBook> books=new ArrayList();
    books:crate::java_runtime::RefCell<Vec<Rc<KmoBook>>>,
    // java protected KmoBookOpenEventListenerGroup bookOpenListenerGroup=new KmoBookOpenEventListenerGroup();
    pub(super) bookOpenListenerGroup:Rc<super::KmoBookOpenEventListenerGroup::KmoBookOpenEventListenerGroup>,
}

impl KmoBooks {
    pub fn new()->Self{Self{books:crate::java_runtime::RefCell::new(vec![]),bookOpenListenerGroup:Rc::new(super::KmoBookOpenEventListenerGroup::KmoBookOpenEventListenerGroup::new())}}
    // java String lang = KmoLocale.getDefault().toString(); return LanguageCode.getLanguageType(lang);
    pub fn getLanguageType() -> util::org::apache::poi::util::LanguageType::LanguageType {
        crate::cn::wps::et::ss::constant::LanguageCode::LanguageCode::getLanguageType(
            &super::KmoLocale::KmoLocale::getDefault(),
        )
    }

    // java public static FontRecord createFontRecord(String fontName)
    pub fn createFontRecord(fontName:&str)->crate::cn::wps::moss::filefmt::biff8::record::FontRecord::FontRecord {
        use crate::cn::wps::moss::filefmt::biff8::record::FontRecord::FontRecord;
        // java FontRecord retval=new FontRecord(); retval.setFontHeight((short)220); retval.setAttributes((short)0); retval.setColorPaletteIndex(32767); retval.setBoldWeight((short)400); retval.setFontName(fontName); return retval;
        let mut retval=FontRecord::FontRecord(); retval.setFontHeight(220);retval.setAttributes(0);retval.setColorPaletteIndex(32767);retval.setBoldWeight(400);retval.setFontName(fontName.into());retval
    }
    // java private static String getFontName()
    pub fn getFontName()->&'static str {
        use util::org::apache::poi::util::LanguageType::*;
        match Self::getLanguageType() {
            LANGUAGE_CHINESE_SIMPLIFIED=>"宋体",
            LANGUAGE_CHINESE_HONGKONG|LANGUAGE_CHINESE_TRADITIONAL=>"新細明體",
            LANGUAGE_JAPANESE=>"ＭＳ Ｐゴシック",LANGUAGE_RUSSIAN=>"Calibri",_=>"Arial"
        }
    }
    // java public void loadTemplates(KmoBook book)
    pub fn loadTemplates(book: &Rc<KmoBook>) {
        use super::xf::{KmoXF::KmoXF,KmoStyle::KmoStyle};
        let manager = book.getXFManager().borrow();
        // java if(xfs.getNumExFormats()<=0) {
        if manager.getNumExFormats()<=0 {
            // java for(int i=0;i<4;++i) xfs.createFont(createFontRecord(fontName));
            for _ in 0..4 {manager.createFont(&Self::createFontRecord(Self::getFontName()));}
            // java for(int i=0;i<22;++i) xfs.createXF(KmoXF.createDefaultExtendedFormat(i));
            for i in 0..22 {manager.createXF(&KmoXF::createDefaultExtendedFormat(i));}
            // java for(int i=0;i<6;++i) xfs.addStyle(new KmoStyle(createStyle(i)));
            for (ixf,builtin) in [(16,3),(17,6),(18,4),(19,7),(0,0),(20,5)] {manager.addStyle(KmoStyle::KmoStyle(ixf,builtin,-1));}
        }
    }
    // java public void addBookOpenEventListener(KmoBookOpenEventListener listener){this.bookOpenListenerGroup.add(listener);}
    pub fn addBookOpenEventListener(&self,listener:Option<Rc<dyn super::event::KmoBookOpenEventListener::KmoBookOpenEventListener>>){self.bookOpenListenerGroup.add(listener);}
    // java public void clearAllBookOpenEventListener(){this.bookOpenListenerGroup.clear();}
    pub fn clearAllBookOpenEventListener(&self){self.bookOpenListenerGroup.clear();}
}
