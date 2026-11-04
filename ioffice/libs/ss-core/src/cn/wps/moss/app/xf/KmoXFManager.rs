use super::{
    KmoCellXF::KmoCellXF, KmoFont::KmoFont, KmoFormat::KmoFormat, KmoStyle::KmoStyle,
    KmoStyleXF::KmoStyleXF, KmoXF::KmoXF,
};
use crate::cn::wps::moss::filefmt::biff8::record::{
    ExtendedFormatRecord::ExtendedFormatRecord, FontRecord::FontRecord,
};
use crate::java_runtime::{Cell, Rc, RefCell};
use kfc::cn::wps::kfc::numfmt::{builtin::BuiltinFormats::BuiltinFormats, i18n::KmoLang::KmoLang};
use std::sync::{LazyLock, RwLock};
static BUILTIN_FORMATS: LazyLock<RwLock<Vec<Rc<KmoFormat>>>> =
    LazyLock::new(|| RwLock::new(Vec::new()));

// java package cn.wps.moss.app.xf;
// java import cn.wps.kfc.numfmt.builtin.BuiltinFormats;
// java import cn.wps.kfc.numfmt.global.NumFmtLocaleHandler;
// java import cn.wps.kfc.numfmt.i18n.KmoLang;
// java import cn.wps.kfc.numfmt.i18n.KmoLocaleChanged;
// java import cn.wps.moss.app.KmoBooks;
// java import cn.wps.moss.filefmt.biff8.record.ExtendedFormatRecord;
// java import cn.wps.moss.filefmt.biff8.record.FontRecord;
// java import cn.wps.moss.filefmt.biff8.record.FormatRecord;
// java import java.util.ArrayList;
// java import java.util.Collections;
// java import java.util.List;

// java public final class KmoXFManager {
pub struct KmoXFManager {
 kmoTableStyles:RefCell<Option<Rc<super::KmoTableStyles::KmoTableStyles>>>,
    xfList: RefCell<Vec<Option<Rc<KmoXF>>>>,
    fontList: RefCell<Vec<Rc<KmoFont>>>,
    formatList: RefCell<Vec<Rc<KmoFormat>>>,
    styleList: RefCell<Vec<Rc<KmoStyle>>>,
    nextFormatCode: Cell<i32>,
    pub normalCharWidth: Cell<f32>,
    pub normalCharWidthTwip: Cell<f32>,
}

impl KmoXFManager {
    pub fn KmoXFManager() -> Self {
        // rust: static {loadBuiltinFormats();NumFmtLocaleHandler.addKmoLocaleChanged(new KmoLocaleChanged(){...});}
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(||{Self::loadBuiltinFormats();kfc::cn::wps::kfc::numfmt::global::NumFmtLocaleHandler::NumFmtLocaleHandler::addKmoLocaleChanged(Rc::new(BuiltinLocaleNotifier));});
        Self {kmoTableStyles:RefCell::new(None),
            xfList: RefCell::new(Vec::new()),
            fontList: RefCell::new(Vec::new()),
            formatList: RefCell::new(Vec::new()),
            styleList: RefCell::new(Vec::new()),
            nextFormatCode: Cell::new(164),
            normalCharWidth: Cell::new(7.0),
            normalCharWidthTwip: Cell::new(7.0),
        }
    }

    // java public static void loadBuiltinFormats() {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:29
    // java public static void loadBuiltinFormats() {
    // java short lcid = KmoLang.getSystemLCID();
    // java BuiltinFormats.loadFormatCodes(lcid);
    // java String[] formats = BuiltinFormats.getAll();
    // java builtinFormats = new KmoFormat[formats.length];
    // java
    // java for(int i = 0; i < formats.length; ++i) {
    // java builtinFormats[i] = new KmoFormat(i, formats[i]);
    // java }
    // java
    // java }
    pub fn loadBuiltinFormats() {
        // java short lcid = KmoLang.getSystemLCID(); BuiltinFormats.loadFormatCodes(lcid);
        BuiltinFormats::loadFormatCodes(KmoLang::getSystemLCID());
        // rust: builtinFormats = new KmoFormat[formats.length]; for(int i=0; i<formats.length; ++i) builtinFormats[i]=new KmoFormat(i,formats[i]);
        *BUILTIN_FORMATS.write().unwrap() = BuiltinFormats::getAll()
            .into_iter()
            .enumerate()
            .map(|(i, f)| Rc::new(KmoFormat::KmoFormat(i as i32, f)))
            .collect();
    }

    // java public void setNormalCharWidth(float width) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:41
    // java public void setNormalCharWidth(float width) {
    // java this.normalCharWidth = width;
    // java }
    pub fn setNormalCharWidth(&self, width: f32) {
        self.normalCharWidth.set(width);
    }

    // java public void setNormalCharWidthTwip(float width) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:45
    // java public void setNormalCharWidthTwip(float width) {
    // java this.normalCharWidthTwip = width;
    // java }
    pub fn setNormalCharWidthTwip(&self, width: f32) {
        self.normalCharWidthTwip.set(width);
    }

    // java public int getNumExFormats() {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:49
    // java public int getNumExFormats() {
    // java return this.xfList.size();
    // java }
    pub fn getNumExFormats(&self) -> i32 {
        // java return this.xfList.size();
        self.xfList.borrow().len() as i32
    }

    // java public int getNumFonts() {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:53
    // java public int getNumFonts() {
    // java return this.fontList.size();
    // java }
    pub fn getNumFonts(&self) -> i32 {
        self.fontList.borrow().len() as i32
    }

    // java public List<KmoFormat> getAllNumfmts() {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:57
    // java public List<KmoFormat> getAllNumfmts() {
    // java return this.formatList;
    // java }
    pub fn getAllNumfmts(&self) -> Vec<Rc<KmoFormat>> {
        self.formatList.borrow().clone()
    }

    // java public List<KmoStyle> getAllStyles() {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:61
    // java public List<KmoStyle> getAllStyles() {
    // java return this.styleList;
    // java }
    pub fn getAllStyles(&self) -> Vec<Rc<KmoStyle>> {
        self.styleList.borrow().clone()
    }

    // java public List<KmoFont> getAllFonts() {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:65
    // java public List<KmoFont> getAllFonts() {
    // java return this.fontList;
    // java }
    pub fn getAllFonts(&self) -> Vec<Rc<KmoFont>> {
        self.fontList.borrow().clone()
    }

    // java private int addXF(KmoXF xf) {
    fn addXF(&self,xf:Option<Rc<KmoXF>>)->i32 {
        // java this.xfList.add(xf);
        self.xfList.borrow_mut().push(xf);
        // java return this.xfList.size() - 1;
        self.xfList.borrow().len()as i32-1
    }

    // java public KmoXF getDefaultXF() {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:74
    // java public KmoXF getDefaultXF() {
    // java return this.getXF(15);
    // java }
    pub fn getDefaultXF(&self) -> Option<Rc<KmoXF>> {
        self.getXF(15)
    }

    // java public KmoXF getUnLockXF() {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:78
    // java public KmoXF getUnLockXF() {
    // java return this.getXF(21);
    // java }
    pub fn getUnLockXF(&self) -> Option<Rc<KmoXF>> {
        self.getXF(21)
    }

    // java public KmoXF getXF(int index) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:82
    // java public KmoXF getXF(int index) {
    // java if (this.xfList.size() == 0) {
    // java return new KmoCellXF();
    // java } else if (index >= 0 && index < this.xfList.size()) {
    // java KmoXF xf = (KmoXF)this.xfList.get(index);
    // java if (xf == null) {
    // java return null;
    // java } else {
    // java if (xf.getFont() == null) {
    // java KmoFont font = this.getFont(xf.getFontIndex());
    // java if (font != null) {
    // java xf.setFont(font);
    // java }
    // java }
    // java
    // java if (xf.getNumFormat() == null) {
    // java KmoFormat format = this.getFormat(xf.getFormatIndex());
    // java if (format != null) {
    // java xf.setNumFormat(format.getFormatString());
    // java }
    // java }
    // java
    // java return xf;
    // java }
    // java } else {
    // java return null;
    // java }
    // java }
    pub fn getXF(&self, index: i32) -> Option<Rc<KmoXF>> {
        // rust: if (this.xfList.size()==0) return new KmoCellXF();
        if self.xfList.borrow().is_empty() {
            return Some(Rc::new(KmoCellXF::KmoCellXF().base));
        }
        // rust: else if(index>=0 && index<this.xfList.size()) ... else return null;
        let xf = self
            .xfList
            .borrow()
            .get(usize::try_from(index).ok()?)
            .cloned()
            .flatten()?;
        // rust: if(xf.getFont()==null) {KmoFont font=this.getFont(xf.getFontIndex()); if(font!=null) xf.setFont(font);}
        if xf.getFont().is_none() {
            xf.setFont(self.getFont(xf.getFontIndex() as i32));
        }
        // rust: if(xf.getNumFormat()==null) {KmoFormat format=this.getFormat(xf.getFormatIndex()); if(format!=null) xf.setNumFormat(format.getFormatString());}
        if xf.getNumFormat().is_none() {
            if let Some(format) = self.getFormat(xf.getFormatIndex() as i32) {
                xf.setNumFormat(format.getFormatString());
            }
        }
        Some(xf)
    }

    // java public int createCellXF() {
    pub fn createCellXF(&self) -> i32 {
        // java if (this.xfList.size() == 0) {
        // java return -1;
        if self.xfList.borrow().is_empty() { return -1; }
        // java KmoXF styleXF = (KmoXF)this.xfList.get(0);
        let styleXF = self.xfList.borrow()[0].clone();
        // java if (!(styleXF instanceof KmoStyleXF)) {
        // java return -1;
        let Some(styleXF) = styleXF.filter(|xf| xf.isStyle) else { return -1; };
        // java this.xfList.add(KmoCellXF.copyFromParentStyle((KmoStyleXF)styleXF));
        // rust: xfList stores superclass handles; copyFromParent is copyFromParentStyle's exact data operation.
        let xf = KmoCellXF::KmoCellXF();
        xf.base.copyFromParent(&styleXF);
        self.xfList.borrow_mut().push(Some(Rc::new(xf.base)));
        // java return this.xfList.size() - 1;
        self.xfList.borrow().len() as i32 - 1
    }

    // java public int cloneCellXF(int ixf) {
    pub fn cloneCellXF(&self, ixf: i32) -> i32 {
        // java if (ixf >= this.xfList.size()) {
        // java return -1;
        if ixf >= self.xfList.borrow().len() as i32 { return -1; }
        // java KmoXF xf = (KmoXF)this.xfList.get(ixf);
        assert!(ixf >= 0, "IndexOutOfBoundsException");
        let xf = self.xfList.borrow()[ixf as usize].clone().expect("NullPointerException KmoXF");
        let copy = KmoCellXF::KmoCellXF();
        // java if (xf instanceof KmoStyleXF) {
        if xf.isStyle {
        // java this.xfList.add(KmoCellXF.copyFromParentStyle((KmoStyleXF)xf));
            copy.base.copyFromParent(&xf);
            self.xfList.borrow_mut().push(Some(Rc::new(copy.base)));
        } else {
        // java this.xfList.add(KmoCellXF.copyOf((KmoCellXF)xf));
            copy.base.completeCopy(&xf);
            self.xfList.borrow_mut().push(Some(Rc::new(copy.base)));
        }
        // java return this.xfList.size() - 1;
        self.xfList.borrow().len() as i32 - 1
    }

    // java public int getNewIXF(KmoXF oldXF, KmoXF opXF, KmoXFMask mask) {
    pub fn getNewIXF(&self) {
        // java KmoXF xf = KmoXF.createXF(oldXF);
        // java if (mask.isAppliedProtection()) {
        // java if (mask.isLock()) {
        // java xf.setLocked(opXF.isLocked());
        // java if (mask.isHidden()) {
        // java xf.setHidden(opXF.isHidden());
        // java xf.setValidCellOptions(true);
        // java if (mask.isAppliedAlign()) {
        // java if (mask.isAlcH()) {
        // java xf.setAlignment(opXF.getAlignment());
        // java if (mask.isAlcV()) {
        // java xf.setVerticalAlignment(opXF.getVerticalAlignment());
        // java if (mask.isIndent()) {
        // java xf.setIndent(opXF.getIndent());
        // java if (mask.isWrap()) {
        // java xf.setWrapText(opXF.getWrapText());
        // java if (mask.isShrinkToFit()) {
        // java xf.setShrinkToFit(opXF.getShrinkToFit());
        // java if (mask.isTrot()) {
        // java xf.setRotation(opXF.getRotation());
        // java xf.setValidAlignment(true);
        // java if (mask.isAppliedBorder()) {
        // java if (mask.isDgLeft()) {
        // java xf.setBorderLeft(opXF.getBorderLeft());
        // java if (mask.isIcvLeft()) {
        // java xf.setLeftBorderPaletteIdx(opXF.getLeftBorderPaletteIdx());
        // java if (mask.isDgBottom()) {
        // java xf.setBorderBottom(opXF.getBorderBottom());
        // java if (mask.isIcvBottom()) {
        // java xf.setBottomBorderPaletteIdx(opXF.getBottomBorderPaletteIdx());
        // java if (mask.isDgRight()) {
        // java xf.setBorderRight(opXF.getBorderRight());
        // java if (mask.isIcvRight()) {
        // java xf.setRightBorderPaletteIdx(opXF.getRightBorderPaletteIdx());
        // java if (mask.isDgTop()) {
        // java xf.setBorderTop(opXF.getBorderTop());
        // java if (mask.isIcvTop()) {
        // java xf.setTopBorderPaletteIdx(opXF.getTopBorderPaletteIdx());
        // java if (mask.isDgDiagDown() && mask.isDgDiagUp()) {
        // java if (oldXF.getAdtlDiagLineStyle() == 0) {
        // java xf.setDiag((byte)0);
        // java xf.setAdtlDiagLineStyle((short)0);
        // java xf.setDiag((byte)3);
        // java xf.setAdtlDiagLineStyle(opXF.getAdtlDiagLineStyle());
        // java } else if (mask.isDgDiagDown()) {
        // java if (oldXF.getAdtlDiagLineStyle() == 0) {
        // java oldXF.setDiag((byte)0);
        // java xf.setDiag((byte)0);
        // java if (oldXF.getDiag() == 3) {
        // java if (opXF.getAdtlDiagLineStyle() == 0) {
        // java xf.setDiag((byte)2);
        // java xf.setDiag((byte)3);
        // java xf.setAdtlDiag(opXF.getAdtlDiag());
        // java xf.setAdtlDiagLineStyle(opXF.getAdtlDiagLineStyle());
        // java if (oldXF.getDiag() == 2 && opXF.getAdtlDiagLineStyle() != 0) {
        // java xf.setDiag((byte)3);
        // java xf.setAdtlDiag(opXF.getAdtlDiag());
        // java xf.setAdtlDiagLineStyle(opXF.getAdtlDiagLineStyle());
        // java if (oldXF.getDiag() == 1) {
        // java if (opXF.getAdtlDiagLineStyle() != 0) {
        // java xf.setAdtlDiag(opXF.getAdtlDiag());
        // java xf.setAdtlDiagLineStyle(opXF.getAdtlDiagLineStyle());
        // java xf.setDiag((byte)0);
        // java xf.setAdtlDiagLineStyle((short)0);
        // java if (oldXF.getDiag() == 0 && opXF.getAdtlDiagLineStyle() != 0) {
        // java xf.setDiag((byte)1);
        // java xf.setAdtlDiag(opXF.getAdtlDiag());
        // java xf.setAdtlDiagLineStyle(opXF.getAdtlDiagLineStyle());
        // java } else if (mask.isDgDiagUp()) {
        // java if (oldXF.getAdtlDiagLineStyle() == 0) {
        // java oldXF.setDiag((byte)0);
        // java xf.setDiag((byte)0);
        // java if (oldXF.getDiag() == 3) {
        // java if (opXF.getAdtlDiagLineStyle() == 0) {
        // java xf.setDiag((byte)1);
        // java xf.setDiag((byte)3);
        // java xf.setAdtlDiag(opXF.getAdtlDiag());
        // java xf.setAdtlDiagLineStyle(opXF.getAdtlDiagLineStyle());
        // java if (oldXF.getDiag() == 2) {
        // java if (opXF.getAdtlDiagLineStyle() != 0) {
        // java xf.setAdtlDiag(opXF.getAdtlDiag());
        // java xf.setAdtlDiagLineStyle(opXF.getAdtlDiagLineStyle());
        // java xf.setDiag((byte)0);
        // java xf.setAdtlDiagLineStyle((short)0);
        // java if (oldXF.getDiag() == 1 && opXF.getAdtlDiagLineStyle() != 0) {
        // java xf.setDiag((byte)3);
        // java xf.setAdtlDiag(opXF.getAdtlDiag());
        // java xf.setAdtlDiagLineStyle(opXF.getAdtlDiagLineStyle());
        // java if (oldXF.getDiag() == 0 && opXF.getAdtlDiagLineStyle() != 0) {
        // java xf.setDiag((byte)2);
        // java xf.setAdtlDiag(opXF.getAdtlDiag());
        // java xf.setAdtlDiagLineStyle(opXF.getAdtlDiagLineStyle());
        // java xf.setValidBorder(true);
        // java if (mask.isAppliedPattern()) {
        // java if (mask.isFillStyle()) {
        // java xf.setAdtlFillPattern(opXF.getAdtlFillPattern());
        // java if (mask.isFillFore()) {
        // java xf.setFillForeground(opXF.getFillForeground());
        // java if (mask.isFillBack()) {
        // java xf.setFillBackground(opXF.getFillBackgroudWithoutTrans());
        // java if (mask.isGradientMarked()) {
        // java xf.setXFGradient(opXF.getXFGradient());
        // java xf.setValidPattern(true);
        // java if (mask.isNumfmt()) {
        // java int newFormatIndex = this.checkFormat(opXF.getNumFormat());
        // java xf.setFormatIndex((short)newFormatIndex);
        // java xf.setNumFormat(opXF.getNumFormat());
        // java xf.setValidFormat(true);
        // java if (mask.isAppliedFont()) {
        // java KmoFont opFont = opXF.getFont();
        // java KmoFont xfFont = xf.getFont();
        // java if (xfFont == null) {
        // java xfFont = this.getFont(xf.getFontIndex());
        /* DEFERRED_NON_FUNCTION
        // java KmoFont font = new KmoFont(xfFont);
         */
        // java if (mask.isFontBold()) {
        // java font.setBoldWeight(opFont.getBoldWeight());
        // java if (mask.isFontCharSet()) {
        // java font.setCharset(opFont.getCharset());
        // java if (mask.isFontFamily()) {
        // java font.setFamily(opFont.getFamily());
        // java if (mask.isFontHeight()) {
        // java font.setFontHeight(opFont.getFontHeight());
        // java if (mask.isFontIcv()) {
        // java font.setColorPaletteIndex(opFont.getColorPaletteIndex());
        // java if (mask.isFontItalic()) {
        // java font.setItalic(opFont.isItalic());
        // java if (mask.isFontName()) {
        // java font.setFontName(opFont.getFontName());
        // java if (mask.isFontSSS()) {
        // java font.setSuperSubScript(opFont.getSuperSubScript());
        // java if (mask.isFontStrikeout()) {
        // java font.setStrikeout(opFont.isStrikeout());
        // java if (mask.isFontUls()) {
        // java font.setUnderline(opFont.getUnderline());
        // java int newFontIndex = this.fontList.indexOf(font);
        // java if (newFontIndex >= 4) {
        // java ++newFontIndex;
        // java if (newFontIndex == -1) {
        // java newFontIndex = this.addFont(font);
        // java xf.setFontIndex((short)newFontIndex);
        // java xf.setFont(font);
        // java xf.setValidFont(true);
        // java return this.checkXFIndex(xf);
        todo!("DEFERRED_NON_FUNCTION");
    }

    // java public int checkXFIndex(KmoXF xf) {
    pub fn checkXFIndex(&self,xf:Option<Rc<KmoXF>>)->i32 {
        // java int formatIndex = this.checkFormat(xf.getNumFormat());
        let xf=xf.expect("NullPointerException");let formatIndex=self.checkFormat(xf.getNumFormat().as_deref());
        // java xf.setFormatIndex((short)formatIndex);
        xf.setFormatIndex(formatIndex as i16);
        // java KmoFont font = xf.getFont();
        let mut font=xf.getFont();
        // java if (font == null) {
        // java font = this.getFont(xf.getFontIndex());
        if font.is_none(){font=Some(self.getFont(xf.getFontIndex()as i32));}
        // java }
        // java int fontIndex = this.checkFontIndex(font);
        let font=font.expect("NullPointerException");let fontIndex=self.checkFontIndex(font.clone());
        // java xf.setFontIndex((short)fontIndex);
        xf.setFontIndex(fontIndex as i16);
        // java xf.setFont(font);
        xf.setFont(font);
        // java int xfIndex = this.xfList.indexOf(xf);
        let mut xfIndex=self.xfList.borrow().iter().position(|other|other.as_ref().is_some_and(|other|xf.as_ref()==other.as_ref())).map_or(-1,|i|i as i32);
        // java if (xfIndex == -1) {
        // java xfIndex = this.addXF(xf);
        if xfIndex==-1{xfIndex=self.addXF(Some(xf));}
        // java }
        // java return xfIndex;
        xfIndex
    }

    // java public int createXF(ExtendedFormatRecord xfRec) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:413
    // java public int createXF(ExtendedFormatRecord xfRec) {
    // java if (xfRec.getXFType() == 0) {
    // java this.xfList.add(new KmoCellXF(xfRec));
    // java } else {
    // java this.xfList.add(new KmoStyleXF(xfRec));
    // java }
    // java
    // java return this.xfList.size() - 1;
    // java }
    pub fn createXF(&self, xfRec: &ExtendedFormatRecord) -> i32 {
        // rust: if(xfRec.getXFType()==0) this.xfList.add(new KmoCellXF(xfRec)); else this.xfList.add(new KmoStyleXF(xfRec));
        let xf = if xfRec.getXFType() == 0 {
            KmoCellXF::fromRecord(xfRec).base
        } else {
            KmoStyleXF::fromRecord(xfRec).base
        };
        self.xfList.borrow_mut().push(Some(Rc::new(xf)));
        self.getNumExFormats() - 1
    }

    // java public int createNullXFs(int number) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:423
    // java public int createNullXFs(int number) {
    // java if (number <= 0) {
    // java throw new IllegalArgumentException(Integer.toString(number) + " may be wrong.");
    // java } else {
    // java int ret = this.xfList.size();
    // java
    // java while(number-- > 0) {
    // java this.xfList.add(null);
    // java }
    // java
    // java return ret;
    // java }
    // java }
    pub fn createNullXFs(&self, number: i32) -> i32 {
        assert!(
            number > 0,
            "IllegalArgumentException: {number} may be wrong."
        );
        let ret = self.getNumExFormats();
        self.xfList.borrow_mut().extend((0..number).map(|_| None));
        ret
    }

    // java public int setNullXF(int location, ExtendedFormatRecord xfRec) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:437
    // java public int setNullXF(int location, ExtendedFormatRecord xfRec) {
    // java if (this.xfList.get(location) != null) {
    // java throw new IllegalArgumentException(Integer.toString(location) + " may be wrong.");
    // java } else {
    // java if (xfRec.getXFType() == 0) {
    // java this.xfList.set(location, new KmoCellXF(xfRec));
    // java } else {
    // java this.xfList.set(location, new KmoStyleXF(xfRec));
    // java }
    // java
    // java return location;
    // java }
    // java }
    pub fn setNullXF(&self, location: i32, xfRec: &ExtendedFormatRecord) -> i32 {
        let mut list = self.xfList.borrow_mut();
        assert!(
            list[location as usize].is_none(),
            "IllegalArgumentException: {location} may be wrong."
        );
        list[location as usize] = Some(Rc::new(KmoXF::fromRecord(xfRec)));
        location
    }

    // java public short checkFontIndex(KmoFont font) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:451
    // java public short checkFontIndex(KmoFont font) {
    // java int newFontIndex = this.fontList.indexOf(font);
    // java if (newFontIndex >= 4) {
    // java ++newFontIndex;
    // java }
    // java
    // java if (newFontIndex == -1) {
    // java newFontIndex = this.addFont(font);
    // java }
    // java
    // java return (short)newFontIndex;
    // java }
    pub fn checkFontIndex(&self, font: Rc<KmoFont>) -> i16 {
        let mut index = self.getFontIndexOf(&font);
        if index >= 4 {
            index += 1;
        }
        if index == -1 {
            index = self.addFont(font);
        }
        index as i16
    }

    // java public int getFontIndexOf(KmoFont font) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:464
    // java public int getFontIndexOf(KmoFont font) {
    // java return this.fontList.indexOf(font);
    // java }
    pub fn getFontIndexOf(&self, font: &KmoFont) -> i32 {
        self.fontList
            .borrow()
            .iter()
            .position(|candidate| candidate.as_ref() == font)
            .map_or(-1, |index| index as i32)
    }

    // java public int createDefaultFont() {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:468
    // java public int createDefaultFont() {
    // java KmoFont font = new KmoFont();
    // java this.fontList.add(font);
    // java int size = this.fontList.size();
    // java return size > 4 ? size : size - 1;
    // java }
    pub fn createDefaultFont(&self) -> i32 {
        self.addFont(Rc::new(KmoFont::KmoFont()))
    }

    // java public KmoFont findFontByProp(int colorIndex, int attributes, short boldWeight, short subScript, short fontHeight, byte underline, byte family, byte charset, String fontName, boolean italics, boolean outline, boolean shadow, boolean strike) {
    pub fn findFontByProp(
        &self,
        colorIndex: i32,
        attributes: i32,
        boldWeight: i16,
        subScript: i16,
        fontHeight: i16,
        underline: i8,
        family: i8,
        charset: i8,
        fontName: String,
        italics: bool,
        outline: bool,
        shadow: bool,
        strike: bool,
    ) -> Rc<KmoFont> {
        let temp = Rc::new(KmoFont::KmoFont());
        temp.setBoldWeight(boldWeight);
        temp.setCharset(charset);
        temp.setColorPaletteIndex(colorIndex);
        temp.setFamily(family);
        temp.setFontHeight(fontHeight);
        temp.setFontName(fontName);
        temp.setItalic(italics);
        temp.setMacoutline(outline);
        temp.setMacshadow(shadow);
        temp.setStrikeout(strike);
        temp.setSuperSubScript(subScript);
        temp.setUnderline(underline);
        let index = self.getFontIndexOf(&temp);
        if index == -1 {
            self.fontList.borrow_mut().push(temp);
            let size = self.fontList.borrow().len() as i32;
            self.getFont(size)
        } else {
            self.getFont(if index >= 4 { index + 1 } else { index })
        }
    }

    // java public KmoFont findFontByProp(KmoFont font) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:499
    // java public KmoFont findFontByProp(KmoFont font) {
    // java return this.findFontByProp(font.getColorPaletteIndex(), 0, font.getBoldWeight(), font.getSuperSubScript(), font.getFontHeight(), font.getUnderline(), font.getFamily(), font.getCharset(), font.getFontName(), font.isItalic(), font.isMacoutlined(), font.isMacshadowed(), font.isStrikeout());
    // java }
    pub fn findFontByProp__2(&self, font: &KmoFont) -> Rc<KmoFont> {
        self.findFontByProp(
            font.getColorPaletteIndex(),
            0,
            font.getBoldWeight(),
            font.getSuperSubScript(),
            font.getFontHeight(),
            font.getUnderline(),
            font.getFamily(),
            font.getCharset(),
            font.getFontName(),
            font.isItalic(),
            font.isMacoutlined(),
            font.isMacshadowed(),
            font.isStrikeout(),
        )
    }

    // java public int createFont(FontRecord fontRec) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:503
    // java public int createFont(FontRecord fontRec) {
    // java this.fontList.add(new KmoFont(fontRec));
    // java int size = this.fontList.size();
    // java return size > 4 ? size : size - 1;
    // java }
    pub fn createFont(&self, fontRec: &FontRecord) -> i32 {
        self.addFont(Rc::new(KmoFont::fromRecord(fontRec)))
    }

    // java public int addFont(KmoFont font) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:509
    // java public int addFont(KmoFont font) {
    // java this.fontList.add(font);
    // java int size = this.fontList.size();
    // java return size > 4 ? size : size - 1;
    // java }
    pub fn addFont(&self, font: Rc<KmoFont>) -> i32 {
        self.fontList.borrow_mut().push(font);
        let size = self.fontList.borrow().len() as i32;
        if size > 4 {
            size
        } else {
            size - 1
        }
    }

    // java public KmoFont getFont(int index) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:515
    // java public KmoFont getFont(int index) {
    // java if (index > 4) {
    // java --index;
    // java }
    // java
    // java if (index >= 0 && index < this.fontList.size()) {
    // java return (KmoFont)this.fontList.get(index);
    // java } else {
    // java if (this.fontList.size() == 0) {
    // java this.createFont(KmoBooks.createFontRecord("宋体"));
    // java }
    // java
    // java return (KmoFont)this.fontList.get(0);
    // java }
    // java }
    pub fn getFont(&self, mut index: i32) -> Rc<KmoFont> {
        // rust: if(index>4) --index;
        if index > 4 {
            index -= 1;
        }
        // rust: if(index>=0 && index<this.fontList.size()) return this.fontList.get(index);
        if index >= 0 {
            if let Some(font) = self.fontList.borrow().get(index as usize) {
                return Rc::clone(font);
            }
        }
        // rust: if(this.fontList.size()==0) this.createFont(KmoBooks.createFontRecord("宋体")); return this.fontList.get(0);
        if self.fontList.borrow().is_empty() {
            self.createFont(
                &crate::cn::wps::moss::app::KmoBooks::KmoBooks::createFontRecord("宋体"),
            );
        }
        Rc::clone(&self.fontList.borrow()[0])
    }

    // java public int getFontCount() {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:531
    // java public int getFontCount() {
    // java return this.fontList.size();
    // java }
    pub fn getFontCount(&self) -> i32 {
        self.getNumFonts()
    }

    // java public void createFormat(FormatRecord fmtRec) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:535
    // java public void createFormat(FormatRecord fmtRec) {
    // java this.formatList.add(new KmoFormat(fmtRec));
    // java if (fmtRec.getIndexCode() >= this.nextFormatCode) {
    // java this.nextFormatCode = fmtRec.getIndexCode() + 1;
    // java }
    // java
    // java }
    pub fn createFormat(
        &self,
        fmtRec: &crate::cn::wps::moss::filefmt::biff8::record::FormatRecord::FormatRecord,
    ) {
        // java this.formatList.add(new KmoFormat(fmtRec));
        self.formatList
            .borrow_mut()
            .push(Rc::new(KmoFormat::fromRecord(fmtRec)));
        // java if (fmtRec.getIndexCode() >= this.nextFormatCode) { this.nextFormatCode = fmtRec.getIndexCode() + 1; }
        if fmtRec.getIndexCode() >= self.nextFormatCode.get() {
            self.nextFormatCode.set(fmtRec.getIndexCode() + 1);
        }
    }

    // java public void addFormat(int formatId, String numfmt) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:543
    // java public void addFormat(int formatId, String numfmt) {
    // java this.formatList.add(new KmoFormat(formatId, numfmt));
    // java if (formatId >= this.nextFormatCode) {
    // java this.nextFormatCode = formatId + 1;
    // java }
    // java
    // java }
    pub fn addFormat(&self, formatId: i32, numfmt: String) {
        self.formatList
            .borrow_mut()
            .push(Rc::new(KmoFormat::KmoFormat(formatId, numfmt)));
        if formatId >= self.nextFormatCode.get() {
            self.nextFormatCode.set(formatId + 1);
        }
    }

    // java public int checkFormat(String numfmt) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:551
    // java public int checkFormat(String numfmt) {
    // java synchronized(this.formatList) {
    // java int i = 0;
    // java
    // java for(int len = this.formatList.size(); i < len; ++i) {
    // java KmoFormat fmt = (KmoFormat)this.formatList.get(i);
    // java if (numfmt != null && fmt != null && numfmt.equals(fmt.getFormatString())) {
    // java return fmt.getIndexCode();
    // java }
    // java }
    // java }
    // java
    // java int i = 0;
    // java
    // java for(int len = builtinFormats.length; i < len; ++i) {
    // java if (builtinFormats[i].getFormatString().equals(numfmt)) {
    // java return builtinFormats[i].getIndexCode();
    // java }
    // java }
    // java
    // java KmoFormat newfmt = new KmoFormat(this.nextFormatCode, numfmt);
    // java this.formatList.add(newfmt);
    // java ++this.nextFormatCode;
    // java return newfmt.getIndexCode();
    // java }
    pub fn checkFormat(&self, numfmt: Option<&str>) -> i32 {
        // rust: synchronized(this.formatList) { ... numfmt.equals(fmt.getFormatString()) ... }
        for fmt in self.formatList.borrow().iter() {
            if numfmt.is_some() && numfmt == fmt.getFormatString().as_deref() {
                return fmt.getIndexCode();
            }
        }
        if BUILTIN_FORMATS.read().unwrap().is_empty() {
            Self::loadBuiltinFormats();
        }
        // rust: for (...) if(builtinFormats[i].getFormatString().equals(numfmt)) return builtinFormats[i].getIndexCode();
        for fmt in BUILTIN_FORMATS.read().unwrap().iter() {
            if fmt.getFormatString().as_deref() == numfmt {
                return fmt.getIndexCode();
            }
        }
        // java KmoFormat newfmt=new KmoFormat(this.nextFormatCode,numfmt); this.formatList.add(newfmt); ++this.nextFormatCode; return newfmt.getIndexCode();
        let id = self.nextFormatCode.get();
        self.formatList
            .borrow_mut()
            .push(Rc::new(KmoFormat::KmoFormat(id, numfmt.map(str::to_owned))));
        self.nextFormatCode.set(id + 1);
        id
    }

    // java public int addStyle(KmoStyle style) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:577
    // java public int addStyle(KmoStyle style) {
    // java this.styleList.add(style);
    // java return this.styleList.size() - 1;
    // java }
    pub fn addStyle(&self, style: KmoStyle) -> i32 {
        self.styleList.borrow_mut().push(Rc::new(style));
        self.styleList.borrow().len() as i32 - 1
    }

    // java public KmoFormat getFormat(int indexCode) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:582
    // java public KmoFormat getFormat(int indexCode) {
    // java synchronized(this.formatList) {
    // java for(KmoFormat fmt : this.formatList) {
    // java if (fmt != null && fmt.getIndexCode() == indexCode) {
    // java return fmt;
    // java }
    // java }
    // java }
    // java
    // java return indexCode >= 0 && indexCode < builtinFormats.length ? builtinFormats[indexCode] : null;
    // java }
    pub fn getFormat(&self, indexCode: i32) -> Option<Rc<KmoFormat>> {
        for fmt in self.formatList.borrow().iter() {
            if fmt.getIndexCode() == indexCode {
                return Some(Rc::clone(fmt));
            }
        }
        if BUILTIN_FORMATS.read().unwrap().is_empty() {
            Self::loadBuiltinFormats();
        }
        BUILTIN_FORMATS
            .read()
            .unwrap()
            .get(usize::try_from(indexCode).ok()?)
            .cloned()
    }

    // java public KmoXF getNormalStyle() {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:594
    // java public KmoXF getNormalStyle() {
    // java return this.getBuiltinStyle(0);
    // java }
    pub fn getNormalStyle(&self) -> Option<Rc<KmoXF>> {
        self.getBuiltinStyle(0)
    }

    // java public int getBuiltingStyleIXF(int builtinNum) {
    pub fn getBuiltingStyleIXF(&self, builtinNum: i32) -> i32 {
        for style in self.styleList.borrow().iter() {
            if style.isBuiltinStyle() && style.getBuiltinStyle() == builtinNum {
                let ixf = style.getIXF() as i32;
                return if ixf >= 0 && ixf < self.getNumExFormats() {
                    ixf
                } else {
                    0
                };
            }
        }
        if builtinNum == 8 || builtinNum == 9 {
            todo!("DEFERRED_NON_FUNCTION: hyperlink font XF mask");
        }
        let defaults = [0, 0, 0, 16, 18, 20, 17, 19];
        let ixf = self.createXF(&KmoXF::createDefaultExtendedFormat(
            defaults[builtinNum as usize],
        ));
        self.addStyle(KmoStyle::KmoStyle(ixf, builtinNum, -1));
        ixf
    }

    // java public KmoXF getBuiltinStyle(int builtinNum) {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:659
    // java public KmoXF getBuiltinStyle(int builtinNum) {
    // java return this.getXF(this.getBuiltingStyleIXF(builtinNum));
    // java }
    pub fn getBuiltinStyle(&self, builtinNum: i32) -> Option<Rc<KmoXF>> {
        self.getXF(self.getBuiltingStyleIXF(builtinNum))
    }

    // java public KmoXF getUserStyle(String styleName) {
    pub fn getUserStyle(&self) {
        // java synchronized(this.styleList) {
        /* DEFERRED_NON_FUNCTION
        // java int i = 0;
         */
        // java for(int size = this.styleList.size(); i < size; ++i) {
        // java KmoStyle st = (KmoStyle)this.styleList.get(i);
        // java if (!st.isBuiltinStyle() && st.name.equals(styleName)) {
        // java return this.getXF(st.getIXF());
        // java return null;
        todo!("DEFERRED_NON_FUNCTION");
    }

    // java public KmoFormat[] getBuiltinFormats() {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:678
    // java public KmoFormat[] getBuiltinFormats() {
    // java return builtinFormats;
    // java }
    pub fn getBuiltinFormats(&self) -> Vec<Rc<KmoFormat>> {
        BUILTIN_FORMATS.read().unwrap().clone()
    }

    // java public void iniXFOption() {

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:682
    // java public void iniXFOption() {
    // java int i = 0;
    // java
    // java for(int size = this.xfList.size(); i < size; ++i) {
    // java KmoXF xf = (KmoXF)this.xfList.get(i);
    // java if (xf != null) {
    // java xf.initOption();
    // java }
    // java }
    // java
    // java }
    pub fn iniXFOption(&self) {
        // java int i = 0;
        // java for(int size = this.xfList.size(); i < size; ++i) {
        // java KmoXF xf = (KmoXF)this.xfList.get(i);
        // java if (xf != null) { xf.initOption(); }
        for xf in self.xfList.borrow().iter().flatten() {
            xf.initOption();
        }
    }

    // java public String toString() {
    pub fn toString(&self) {
        /* DEFERRED_NON_FUNCTION
        // java StringBuilder sb = new StringBuilder();
         */
        // java sb.append("xf-count");
        // java sb.append(this.xfList.size());
        // java sb.append(" style-count");
        // java sb.append(this.styleList.size());
        // java return sb.toString();
        todo!("DEFERRED_NON_FUNCTION");
    }

    // java public KmoTableStyles getKmoTableStyles() {


    // java public void setKmoTableStyles(KmoTableStyles kmoTableStyles) {


    // java public void localeChanged() {
    pub fn localeChanged(&self) {
        // java KmoXFManager.loadBuiltinFormats();
        todo!("DEFERRED_NON_FUNCTION");
    }
}

struct BuiltinLocaleNotifier;
impl kfc::cn::wps::kfc::numfmt::i18n::KmoLocaleChanged::KmoLocaleChanged for BuiltinLocaleNotifier {
    // java public void localeChanged(){KmoXFManager.loadBuiltinFormats();}

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFManager.java:714
    // java public void localeChanged() {
    // java KmoXFManager.loadBuiltinFormats();
    // java }
    fn localeChanged(&self) {
        KmoXFManager::loadBuiltinFormats();
    }
}
impl KmoXFManager{// java public KmoTableStyles getKmoTableStyles() {
// java return this.kmoTableStyles;
// java }
pub fn getKmoTableStyles(&self)->Option<Rc<super::KmoTableStyles::KmoTableStyles>> {
self.kmoTableStyles.borrow().clone()
}
// java public void setKmoTableStyles(KmoTableStyles kmoTableStyles) {
// java this.kmoTableStyles = kmoTableStyles;
// java }
pub fn setKmoTableStyles(&self,v:Option<Rc<super::KmoTableStyles::KmoTableStyles>>) {
*self.kmoTableStyles.borrow_mut()=v;
}
}
