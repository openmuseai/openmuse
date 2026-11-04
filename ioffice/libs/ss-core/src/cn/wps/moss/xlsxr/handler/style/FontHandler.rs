use super::StylesShared;
use crate::cn::wps::moss::{
    filefmt::biff8::record::FontRecord::FontRecord, xlsxr::util::ColorHelper::ColorHelper,
};
use crate::java_runtime::{Rc, RefCell};
use kfc_kfc_alpha::kfc::xpxml::reader::{
    ElementAttrHandler::ElementAttrHandler, XmlRoAttr::XmlRoAttr,
};
// java public final class FontHandler implements ElementAttrHandler
pub struct FontHandler {
    pub(super) shared: Rc<StylesShared>,
    mFont: RefCell<Option<Rc<RefCell<FontRecord>>>>,
    isFromDxf: bool,
    hasStylesTable:bool,
}
impl FontHandler {
    pub fn new(shared: Rc<StylesShared>, isFromDxf: bool) -> Self {
        Self {
            shared,
            isFromDxf,hasStylesTable:!isFromDxf,
            mFont: RefCell::new(None),
        }
    }
// java public FontHandler(KmoBook kmoBook, boolean isFromDxf) {
// java this.stylesTable = null;
// java this.kmoXFManager = kmoBook.getXFManager();
// java this.bNeedUpdateFont = false;
// java this.isFromDxf = isFromDxf;
// java }
    pub fn fromBook(book:Rc<crate::cn::wps::moss::app::KmoBook::KmoBook>,isFromDxf:bool)->Self{let shared=StylesShared::new(book,Rc::new(RefCell::new(crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable::StylesTable())));shared.bNeedUpdateFont.set(false);Self{shared,mFont:RefCell::new(None),isFromDxf,hasStylesTable:false}}
    // java public void init(FontRecord font) { this.mFont = font; }
    pub fn init(&self, font: Rc<RefCell<FontRecord>>) {
        *self.mFont.borrow_mut() = Some(font);
    }

    // java private boolean isValueTrue(XmlRoAttr attrs,int element)

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/FontHandler.java:130
    // java private boolean isValueTrue(XmlRoAttr attrs, int element) {
    // java XmlRoAttr tmp;
    // java if ((tmp = attrs.getByName(element)) != null) {
    // java if ((tmp = tmp.getByName(4114)) != null) {
    // java return tmp.toBoolean();
    // java } else {
    // java return true;
    // java }
    // java } else {
    // java return false;
    // java }
    // java }
    fn isValueTrue(a: &dyn XmlRoAttr, e: i32) -> bool {
        a.get_by_name(e)
            .is_some_and(|v| v.get_by_name(4114).is_none_or(|x| x.to_boolean()))
    }
    // java public static short getVertAlignType(String value)

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/FontHandler.java:157
    // java public static short getVertAlignType(String value) {
    // java if (value.equals("superscript")) {
    // java return 1;
    // java } else {
    // java return (short)(value.equals("subscript") ? 2 : 0);
    // java }
    // java }
    pub fn getVertAlignType(value: &str) -> i16 {
        match value {
            "superscript" => 1,
            "subscript" => 2,
            _ => 0,
        }
    }
    fn name<'a>(a: &'a dyn XmlRoAttr) -> Option<&'a dyn XmlRoAttr> {
        a.get_by_name(4217)
            .or_else(|| a.get_by_name(4654))
            .and_then(|x| x.get_by_name(4114))
    }
}
impl ElementAttrHandler for FontHandler {
    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/FontHandler.java:38
    // java public void addElementAttr(int element, XmlRoAttr attrsNotCopy) {
    // java if (this.mFont == null) {
    // java this.mFont = new FontRecord();
    // java }
    // java
    // java XmlRoAttr tmp;
    // java if (((tmp = attrsNotCopy.getByName(4217)) != null || (tmp = attrsNotCopy.getByName(4654)) != null) && (tmp = tmp.getByName(4114)) != null) {
    // java this.mFont.setFontName(tmp.toString());
    // java } else {
    // java String defaultName = this.kmoXFManager.getFont(0).getFontName();
    // java this.mFont.setFontName(defaultName);
    // java }
    // java
    // java if (attrsNotCopy.getByName(4341) != null) {
    // java this.mFont.setBoldOption();
    // java if (this.isValueTrue(attrsNotCopy, 4341)) {
    // java this.mFont.setBoldWeight((short)700);
    // java }
    // java }
    // java
    // java if (attrsNotCopy.getByName(4152) != null) {
    // java this.mFont.setItalicOption();
    // java if (this.isValueTrue(attrsNotCopy, 4152)) {
    // java this.mFont.setItalic(true);
    // java }
    // java }
    // java
    // java if ((tmp = attrsNotCopy.getByName(4358)) != null) {
    // java this.mFont.setUnderLineOption();
    // java if ((tmp = tmp.getByName(4114)) != null) {
    // java this.mFont.setUnderline(UnderlineTypeMap.getUnderline(tmp.toString()));
    // java } else {
    // java this.mFont.setUnderline(UnderlineTypeMap.getUnderline((String)null));
    // java }
    // java }
    // java
    // java if ((tmp = attrsNotCopy.getByName(4662)) != null && (tmp = tmp.getByName(4114)) != null) {
    // java float size = tmp.toFloat() * 20.0F;
    // java this.mFont.setFontHeight((short)((int)size));
    // java } else if (!this.isFromDxf) {
    // java short defaultHeight = this.kmoXFManager.getFont(0).getFontHeight();
    // java this.mFont.setFontHeight(defaultHeight);
    // java }
    // java
    // java if (attrsNotCopy.getByName(4657) != null) {
    // java this.mFont.setStrikeOption();
    // java if (this.isValueTrue(attrsNotCopy, 4657)) {
    // java this.mFont.setStrikeout(true);
    // java }
    // java }
    // java
    // java if ((tmp = attrsNotCopy.getByName(4663)) != null && (tmp = tmp.getByName(4114)) != null) {
    // java this.mFont.setSuperSubScript(getVertAlignType(tmp.toString()));
    // java }
    // java
    // java if ((tmp = attrsNotCopy.getByName(4655)) != null && (tmp = tmp.getByName(4114)) != null) {
    // java int charset = tmp.toInt();
    // java byte cs = (byte)charset;
    // java if (charset > 127) {
    // java cs = (byte)(charset - 256);
    // java }
    // java
    // java this.mFont.setCharset(cs);
    // java }
    // java
    // java if ((tmp = attrsNotCopy.getByName(4661)) != null) {
    // java this.mFont.setColorOption();
    // java
    // java try {
    // java int value = ColorHelper.getColor(tmp, 32767);
    // java this.mFont.setColorPaletteIndex(value);
    // java } catch (Throwable e) {
    // java IOLog.d(TAG, "Throwable", e);
    // java }
    // java } else {
    // java int defaultColor = this.kmoXFManager.getFont(0).getColorPaletteIndex();
    // java this.mFont.setColorPaletteIndex(defaultColor);
    // java }
    // java
    // java if (this.stylesTable != null) {
    // java int fontId = this.kmoXFManager.createFont(this.mFont);
    // java this.stylesTable.addFont(fontId);
    // java this.mFont = null;
    // java }
    // java
    // java if (this.bNeedUpdateFont && this.kmoXFManager != null) {
    // java this.bNeedUpdateFont = false;
    // java this.updateFirstFont(attrsNotCopy);
    // java }
    // java
    // java }
    fn addElementAttr(&self, _: i32, a: &dyn XmlRoAttr) {
        // rust: if(this.mFont==null) this.mFont=new FontRecord();
        let font_ref = self
            .mFont
            .borrow_mut()
            .get_or_insert_with(|| Rc::new(RefCell::new(FontRecord::FontRecord())))
            .clone();
        let mut font = font_ref.borrow_mut();
        let manager = self.shared.book.getXFManager().borrow();
        // rust: name/rFont val, otherwise this.kmoXFManager.getFont(0).getFontName().
        font.setFontName(
            Self::name(a)
                .map(|x| x.to_string())
                .unwrap_or_else(|| manager.getFont(0).getFontName()),
        );
        // rust: if(attrsNotCopy.getByName(4341)!=null) {this.mFont.setBoldOption();if(this.isValueTrue(attrsNotCopy,4341))this.mFont.setBoldWeight((short)700);}
        if a.get_by_name(4341).is_some() {
            font.setBoldOption();
            if Self::isValueTrue(a, 4341) {
                font.setBoldWeight(700);
            }
        }
        // rust: if(attrsNotCopy.getByName(4152)!=null) {this.mFont.setItalicOption();if(this.isValueTrue(attrsNotCopy,4152))this.mFont.setItalic(true);}
        if a.get_by_name(4152).is_some() {
            font.setItalicOption();
            if Self::isValueTrue(a, 4152) {
                font.setItalic(true);
            }
        }
        // rust: underline defaults single when val missing or unknown.
        if let Some(x) = a.get_by_name(4358) {
            font.setUnderLineOption();
            let value = x.get_by_name(4114).map(|x| x.to_string());
            font.setUnderline(super::UnderlineTypeMap::UnderlineTypeMap::getUnderline(
                value.as_deref(),
            ));
        }
        // rust: float size=tmp.toFloat()*20.0F;this.mFont.setFontHeight((short)((int)size));else defaultHeight=getFont(0).getFontHeight();
        if let Some(size) = a.get_by_name(4662).and_then(|x| x.get_by_name(4114)) {
            font.setFontHeight((size.to_float() * 20.0) as i32 as i16);
        } else if !self.isFromDxf {
            font.setFontHeight(manager.getFont(0).getFontHeight());
        }
        // rust: if(strike exists) setStrikeOption();if(isValueTrue) setStrikeout(true);
        if a.get_by_name(4657).is_some() {
            font.setStrikeOption();
            if Self::isValueTrue(a, 4657) {
                font.setStrikeout(true);
            }
        }
        // rust: vertAlign val -> getVertAlignType.
        if let Some(x) = a.get_by_name(4663).and_then(|x| x.get_by_name(4114)) {
            font.setSuperSubScript(Self::getVertAlignType(&x.to_string()));
        }
        // rust: byte cs=(byte)charset;if(charset>127)cs=(byte)(charset-256);this.mFont.setCharset(cs);
        if let Some(x) = a.get_by_name(4655).and_then(|x| x.get_by_name(4114)) {
            let cs = x.to_int();
            font.setCharset(if cs > 127 {
                cs.wrapping_sub(256) as i8
            } else {
                cs as i8
            });
        }
        // rust: color: setColorOption; try ColorHelper.getColor(tmp,32767), catch Throwable.
        if let Some(x) = a.get_by_name(4661) {
            font.setColorOption();
            if let Ok(color) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                ColorHelper::getColor(x, 32767)
            })) {
                font.setColorPaletteIndex(color);
            }
        } else {
            font.setColorPaletteIndex(manager.getFont(0).getColorPaletteIndex());
        }
        // java int fontId=this.kmoXFManager.createFont(this.mFont);this.stylesTable.addFont(fontId);this.mFont=null;
        if self.hasStylesTable {
            let id = manager.createFont(&font);
            self.shared.table.borrow().addFont(id);
            self.mFont.borrow_mut().take();
        }
        // java if(this.bNeedUpdateFont&&this.kmoXFManager!=null){this.bNeedUpdateFont=false;this.updateFirstFont(attrsNotCopy);}
        if !self.isFromDxf && self.shared.bNeedUpdateFont.get() {
            self.shared.bNeedUpdateFont.set(false);
            let first = manager.getFont(0);
            // rust: firstFont.setFontName(tmp.toString());firstFont.setFontHeight((short)((int)(tmp.toFloat()*20.0F)));
            if let Some(x) = Self::name(a) {
                first.setFontName(x.to_string());
            }
            if let Some(x) = a.get_by_name(4662).and_then(|x| x.get_by_name(4114)) {
                first.setFontHeight((x.to_float() * 20.0) as i32 as i16);
            }
        }
    }
}
