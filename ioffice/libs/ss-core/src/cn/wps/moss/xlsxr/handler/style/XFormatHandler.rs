use super::StylesShared;
use crate::cn::wps::moss::filefmt::biff8::record::ExtendedFormatRecord::ExtendedFormatRecord;
use crate::java_runtime::{Rc, RefCell};
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler, XmlRoAttr::XmlRoAttr};
// java public class XFormatHandler extends DefaultElementHandler
pub struct XFormatHandler {
    pub(super) shared: Rc<StylesShared>,
}
impl XFormatHandler {
    // java public XFormatHandler(KmoBook kmoBook,StylesTable stylesTable)
    pub fn XFormatHandler(shared: Rc<StylesShared>) -> Self {
        Self { shared }
    }
    // java this.isStyleXf=isStyleXf;

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:37
    // java public void init(boolean isStyleXf) {
    // java this.isStyleXf = isStyleXf;
    // java }
    pub fn init(&self, isStyleXf: bool) {
        self.shared.isStyleXf.set(isStyleXf);
    }
    // java this.mXFIndex=xfIndex;

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:41
    // java public void setXFIndex(int xfIndex) {
    // java this.mXFIndex = xfIndex;
    // java }
    pub fn setXFIndex(&self, xfIndex: i32) {
        self.shared.mXFIndex.set(xfIndex);
    }
    fn style(&self) -> Rc<RefCell<ExtendedFormatRecord>> {
        self.shared
            .style
            .borrow()
            .as_ref()
            .expect("XFormatHandler.style")
            .clone()
    }
    fn install(&self, style: ExtendedFormatRecord) {
        *self.shared.style.borrow_mut() = Some(Rc::new(RefCell::new(style)));
    }
    // java public void startElement(int element,XmlRoAttr attrsNotCopy)
    pub fn startElement(&self, _element: i32, a: &dyn XmlRoAttr) {
        let s = &self.shared;
        // java if(this.isStyleXf) {
        if s.isStyleXf.get() {
            // java if(this.bFirstStyleXf) {this.bFirstStyleXf=false; this.style=new ExtendedFormatRecord(); this.updateDefaultNumFormat(attrsNotCopy,0); this.kmoXFManager.getXF(0).copyToRecord(this.style); this.stylesTable.addStyleXf(this.style);this.stylesTable.addStyleId(0);return;}
            if s.bFirstStyleXf.get() {
                s.bFirstStyleXf.set(false);
                self.install(ExtendedFormatRecord::ExtendedFormatRecord());
                self.updateDefaultNumFormat(a, 0);
                s.book
                    .getXFManager()
                    .borrow()
                    .getXF(0)
                    .unwrap()
                    .copyToRecord(&mut self.style().borrow_mut());
                s.table.borrow().addStyleXf(self.style());
                s.table.borrow().addStyleId(0);
                return;
            }
            // java this.style=new ExtendedFormatRecord();this.style.setParentIndex((short)4095);this.style.setXFType((short)1);
            self.install(ExtendedFormatRecord::ExtendedFormatRecord());
            self.style().borrow_mut().setParentIndex(4095);
            self.style().borrow_mut().setXFType(1);
            // rust: if((tmp=attrsNotCopy.getByName(5169))==null||tmp.toBoolean()) applyFont=true; applyFont(fontId or 1,applyFont);
            self.applyFont(
                a.get_by_name(4667).map_or(1, |x| x.to_int()),
                a.get_by_name(5169).is_none_or(|x| x.to_boolean()),
            );
            // rust: if((tmp=attrsNotCopy.getByName(5170))==null||tmp.toBoolean()) applyFill=true; applyFill(fillId or 1,applyFill);
            self.applyFill(
                a.get_by_name(5164).map_or(1, |x| x.to_int()),
                a.get_by_name(5170).is_none_or(|x| x.to_boolean()),
            );
            // rust: if((tmp=attrsNotCopy.getByName(5171))==null||tmp.toBoolean()) applyBorder=true; applyBorders(borderId or 1,applyBorder);
            self.applyBorders(
                a.get_by_name(5165).map_or(1, |x| x.to_int()),
                a.get_by_name(5171).is_none_or(|x| x.to_boolean()),
            );
            // rust: if((tmp=attrsNotCopy.getByName(5168))==null||tmp.toBoolean()) applyNumberFormat=true; applyNumberFormat(numFmtId or 1,applyNumberFormat);
            self.applyNumberFormat(
                a.get_by_name(4319).map_or(1, |x| x.to_short()),
                a.get_by_name(5168).is_none_or(|x| x.to_boolean()),
            );
            // java if((tmp=attrsNotCopy.getByName(5172))==null||tmp.toBoolean()) {this.style.setValidAlignment(true);this.style.setVerticalAlignment((short)2);}
            if a.get_by_name(5172).is_none_or(|x| x.to_boolean()) {
                self.style().borrow_mut().setValidAlignment(true);
                self.style().borrow_mut().setVerticalAlignment(2);
            }
            // java this.stylesTable.addStyleXf(this.style);int styleId=this.kmoXFManager.createXF(this.style);this.stylesTable.addStyleId(styleId);
            s.table.borrow().addStyleXf(self.style());
            let styleId = s
                .book
                .getXFManager()
                .borrow()
                .createXF(&self.style().borrow());
            s.table.borrow().addStyleId(styleId);
        } else {
            // java if(this.bFirstCellXf) {
            if s.bFirstCellXf.get() {
                // java this.style=new ExtendedFormatRecord();this.updateDefaultNumFormat(attrsNotCopy,15);this.kmoXFManager.getXF(15).copyToRecord(this.style);
                self.install(ExtendedFormatRecord::ExtendedFormatRecord());
                self.updateDefaultNumFormat(a, 15);
                s.book
                    .getXFManager()
                    .borrow()
                    .getXF(15)
                    .unwrap()
                    .copyToRecord(&mut self.style().borrow_mut());
                // java if(this.stylesTable.getStyleXfSize()==0) {ExtendedFormatRecord xfRec=new ExtendedFormatRecord();this.updateDefaultNumFormat(attrsNotCopy,0);this.kmoXFManager.getXF(0).copyToRecord(xfRec);this.stylesTable.addStyleXf(xfRec);this.stylesTable.addStyleId(0);}
                if s.table.borrow().getStyleXfSize() == 0 {
                    let mut xfRec = ExtendedFormatRecord::ExtendedFormatRecord();
                    self.updateDefaultNumFormat(a, 0);
                    s.book
                        .getXFManager()
                        .borrow()
                        .getXF(0)
                        .unwrap()
                        .copyToRecord(&mut xfRec);
                    s.table.borrow().addStyleXf(Rc::new(RefCell::new(xfRec)));
                    s.table.borrow().addStyleId(0);
                }
                return;
            }
            // rust: int xfId=0;if((tmp=attrsNotCopy.getByName(5166))!=null) xfId=tmp.toInt();this.style=this.stylesTable.getStyleXfAt(xfId);
            let xfId = a.get_by_name(5166).map_or(0, |x| x.to_int());
            let parent = s.table.borrow().getStyleXfAt(xfId);
            // java if(null==this.style) {this.style=new ExtendedFormatRecord();this.kmoXFManager.getXF(0).copyToRecord(this.style);}
            let parent = parent.unwrap_or_else(|| {
                let mut rec = ExtendedFormatRecord::ExtendedFormatRecord();
                s.book
                    .getXFManager()
                    .borrow()
                    .getXF(0)
                    .unwrap()
                    .copyToRecord(&mut rec);
                Rc::new(RefCell::new(rec))
            });
            // java int styleId=this.stylesTable.getStyleId(xfId);ExtendedFormatRecord cellStyle=new ExtendedFormatRecord();cellStyle.cloneStyleFrom(this.style);cellStyle.setXFType((short)0);this.style=cellStyle;this.style.setParentIndex((short)styleId);
            let mut cellStyle = ExtendedFormatRecord::ExtendedFormatRecord();
            cellStyle.cloneStyleFrom(&parent.borrow());
            cellStyle.setXFType(0);
            cellStyle.setParentIndex(s.table.borrow().getStyleId(xfId) as i16);
            self.install(cellStyle);
            // rust: applyFont defaults false for cell XF; explicit true attribute only.
            self.applyFont(
                a.get_by_name(4667).map_or(0, |x| x.to_int()),
                a.get_by_name(5169).is_some_and(|x| x.to_boolean()),
            );
            // rust: this.cleanFill();this.applyFill(fillId or 0,applyFill);
            self.cleanFill();
            self.applyFill(
                a.get_by_name(5164).map_or(0, |x| x.to_int()),
                a.get_by_name(5170).is_some_and(|x| x.to_boolean()),
            );
            // rust: this.applyBorders(borderId or 0,applyBorder);
            self.applyBorders(
                a.get_by_name(5165).map_or(0, |x| x.to_int()),
                a.get_by_name(5171).is_some_and(|x| x.to_boolean()),
            );
            // rust: this.applyNumberFormat(numFmtId or 0,applyNumberFormat);
            self.applyNumberFormat(
                a.get_by_name(4319).map_or(0, |x| x.to_short()),
                a.get_by_name(5168).is_some_and(|x| x.to_boolean()),
            );
            // rust: if((tmp=attrsNotCopy.getByName(5172))!=null && tmp.toBoolean()) this.style.setAttrAlignment(true);
            if a.get_by_name(5172).is_some_and(|x| x.to_boolean()) {
                self.style().borrow_mut().setAttrAlignment(true);
            }
            // rust: this.cleanAlignment();if(attrsNotCopy.getByName(5173)!=null) this.cleanProtection();
            self.cleanAlignment();
            if a.get_by_name(5173).is_some() {
                self.cleanProtection();
            }
        }
        // rust: if((var21=attrsNotCopy.getByName(4730))!=null) this.style.set123Prefix(var21.toBoolean()); this.style.setLocked(true);
        if let Some(v) = a.get_by_name(4730) {
            self.style().borrow_mut().set123Prefix(v.to_boolean());
        }
        self.style().borrow_mut().setLocked(true);
    }
    // java private void updateDefaultNumFormat(XmlRoAttr attrsNotCopy,int defaultIndex)

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:210
    // java private void updateDefaultNumFormat(XmlRoAttr attrsNotCopy, int defaultIndex) {
    // java if (attrsNotCopy.getByName(4319) != null) {
    // java short numFmtid = attrsNotCopy.getByName(4319).toShort();
    // java short formatIndex = this.stylesTable.getformatCode(numFmtid);
    // java if (null == this.kmoXFManager.getXF(defaultIndex)) {
    // java return;
    // java }
    // java
    // java this.kmoXFManager.getXF(defaultIndex).setFormatIndex(formatIndex);
    // java KmoFormat kmoFormat = this.kmoXFManager.getFormat(formatIndex);
    // java if (null == kmoFormat) {
    // java return;
    // java }
    // java
    // java String formatString = kmoFormat.getFormatString();
    // java if (null == formatString) {
    // java return;
    // java }
    // java
    // java this.kmoXFManager.getXF(defaultIndex).setNumFormat(formatString);
    // java }
    // java
    // java }
    pub fn updateDefaultNumFormat(&self, a: &dyn XmlRoAttr, defaultIndex: i32) {
        // rust: if(attrsNotCopy.getByName(4319)!=null) {short formatIndex=this.stylesTable.getformatCode(numFmtid); ...}
        if let Some(v) = a.get_by_name(4319) {
            let formatIndex = self.shared.table.borrow().getformatCode(v.to_short());
            let manager = self.shared.book.getXFManager().borrow();
            let Some(xf) = manager.getXF(defaultIndex) else {
                return;
            };
            xf.setFormatIndex(formatIndex);
            let Some(fmt) = manager.getFormat(formatIndex as i32) else {
                return;
            };
            xf.setNumFormat(fmt.getFormatString());
        }
    }
    // java private void cleanAlignment()

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:243
    // java private void cleanAlignment() {
    // java this.style.setAlignment((short)0);
    // java this.style.setWrapText(false);
    // java this.style.setVerticalAlignment((short)2);
    // java this.style.setIndent((short)0);
    // java this.style.setRotation((short)0);
    // java }
    pub fn cleanAlignment(&self) {
        let p = self.style();
        let mut x = p.borrow_mut();
        x.setAlignment(0);
        x.setWrapText(false);
        x.setVerticalAlignment(2);
        x.setIndent(0);
        x.setRotation(0);
    }
    // java private void cleanProtection() {this.style.setLocked(true);this.style.setHidden(false);}

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:251
    // java private void cleanProtection() {
    // java this.style.setLocked(true);
    // java this.style.setHidden(false);
    // java }
    pub fn cleanProtection(&self) {
        let p = self.style();
        let mut x = p.borrow_mut();
        x.setLocked(true);
        x.setHidden(false);
    }
    // java private void cleanFill() {this.style.setFillBackground(64);this.style.setFillForeground(0);}

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:256
    // java private void cleanFill() {
    // java this.style.setFillBackground(64);
    // java this.style.setFillForeground(0);
    // java }
    pub fn cleanFill(&self) {
        let p = self.style();
        let mut x = p.borrow_mut();
        x.setFillBackground(64);
        x.setFillForeground(0);
    }
    // java this.style.setFormatIndex(this.stylesTable.getformatCode(numFmtid));this.style.setValidFormat(applyNumberFormat);

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:261
    // java private void applyNumberFormat(short numFmtid, boolean applyNumberFormat) {
    // java this.style.setFormatIndex(this.stylesTable.getformatCode(numFmtid));
    // java this.style.setValidFormat(applyNumberFormat);
    // java }
    pub fn applyNumberFormat(&self, numFmtid: i16, apply: bool) {
        let p = self.style();
        let mut x = p.borrow_mut();
        x.setFormatIndex(self.shared.table.borrow().getformatCode(numFmtid));
        x.setValidFormat(apply);
    }
    // java this.style.setFontIndex((short)this.stylesTable.getFontAt(fontId));this.style.setValidFont(applyFont);

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:266
    // java private void applyFont(int fontId, boolean applyFont) {
    // java this.style.setFontIndex((short)this.stylesTable.getFontAt(fontId));
    // java this.style.setValidFont(applyFont);
    // java }
    pub fn applyFont(&self, fontId: i32, apply: bool) {
        let p = self.style();
        let mut x = p.borrow_mut();
        x.setFontIndex(self.shared.table.borrow().getFontAt(fontId) as i16);
        x.setValidFont(apply);
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:271
    // java private void applyFill(int fillId, boolean applyFill) {
    // java PatternFormatting pattern = this.stylesTable.getPatternFillAt(fillId);
    // java if (pattern != null) {
    // java this.style.setAdtlFillPattern((short)pattern.getFillPattern());
    // java int index = -1;
    // java if ((index = pattern.getFillBackgroundColor()) != 127) {
    // java this.style.setFillBackground(index);
    // java }
    // java
    // java if ((index = pattern.getFillForegroundColor()) != 127) {
    // java this.style.setFillForeground(index);
    // java }
    // java
    // java if (pattern.getFillPattern() == 32) {
    // java this.style.setXFGradient(pattern.getXFGradient());
    // java }
    // java
    // java this.style.setValidPattern(applyFill);
    // java }
    // java }
    pub fn applyFill(&self, fillId: i32, apply: bool) {
        // java PatternFormatting pattern=this.stylesTable.getPatternFillAt(fillId);if(pattern!=null) {
        if let Some(pattern) = self.shared.table.borrow().getPatternFillAt(fillId) {
            let pattern = pattern.borrow();
            let p = self.style();
            let mut x = p.borrow_mut();
            // rust: this.style.setAdtlFillPattern((short)pattern.getFillPattern()); if((index=pattern.getFillBackgroundColor())!=127) this.style.setFillBackground(index); if((index=pattern.getFillForegroundColor())!=127) this.style.setFillForeground(index);
            x.setAdtlFillPattern(pattern.getFillPattern() as i16);
            if pattern.getFillBackgroundColor() != 127 {
                x.setFillBackground(pattern.getFillBackgroundColor());
            }
            if pattern.getFillForegroundColor() != 127 {
                x.setFillForeground(pattern.getFillForegroundColor());
            }
            // rust: if(pattern.getFillPattern()==32) this.style.setXFGradient(pattern.getXFGradient());this.style.setValidPattern(applyFill);
            if pattern.getFillPattern() == 32 {
                x.setXFGradient(pattern.getXFGradient());
            }
            x.setValidPattern(apply);
        }
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:292
    // java private void applyBorders(int borderId, boolean applyBorder) {
    // java BorderFormatting bor = this.stylesTable.getBorderAt(borderId);
    // java if (null != bor) {
    // java if ((short)bor.getBorderRight() != 0 || applyBorder) {
    // java this.style.setRightBorderPaletteIdx(bor.getRightBorderColor());
    // java this.style.setBorderRight((short)bor.getBorderRight());
    // java }
    // java
    // java if ((short)bor.getBorderLeft() != 0 || applyBorder) {
    // java this.style.setLeftBorderPaletteIdx(bor.getLeftBorderColor());
    // java this.style.setBorderLeft((short)bor.getBorderLeft());
    // java }
    // java
    // java if ((short)bor.getBorderTop() != 0 || applyBorder) {
    // java this.style.setTopBorderPaletteIdx(bor.getTopBorderColor());
    // java this.style.setBorderTop((short)bor.getBorderTop());
    // java }
    // java
    // java if ((short)bor.getBorderBottom() != 0 || applyBorder) {
    // java this.style.setBottomBorderPaletteIdx(bor.getBottomBorderColor());
    // java this.style.setBorderBottom((short)bor.getBorderBottom());
    // java }
    // java
    // java if (bor.isBackwardDiagonalOn()) {
    // java this.setBackwardDiagonalOn(true);
    // java }
    // java
    // java if (bor.isForwardDiagonalOn()) {
    // java this.setForwardDiagonalOn(true);
    // java }
    // java
    // java if ((short)bor.getBorderDiagonal() != 0 || applyBorder) {
    // java this.style.setAdtlDiag(bor.getDiagonalBorderColor());
    // java this.style.setAdtlDiagLineStyle((short)bor.getBorderDiagonal());
    // java }
    // java
    // java this.style.setValidBorder(applyBorder);
    // java }
    // java }
    pub fn applyBorders(&self, borderId: i32, apply: bool) {
        // java BorderFormatting bor=this.stylesTable.getBorderAt(borderId);if(null!=bor) {
        if let Some(bor) = self.shared.table.borrow().getBorderAt(borderId) {
            let bor = bor.borrow();
            let p = self.style();
            let mut x = p.borrow_mut();
            // java if((short)bor.getBorderRight()!=0||applyBorder) {this.style.setRightBorderPaletteIdx(bor.getRightBorderColor());this.style.setBorderRight((short)bor.getBorderRight());}
            if bor.getBorderRight() as i16 != 0 || apply {
                x.setRightBorderPaletteIdx(bor.getRightBorderColor());
                x.setBorderRight(bor.getBorderRight() as i16);
            }
            // java if((short)bor.getBorderLeft()!=0||applyBorder) {this.style.setLeftBorderPaletteIdx(bor.getLeftBorderColor());this.style.setBorderLeft((short)bor.getBorderLeft());}
            if bor.getBorderLeft() as i16 != 0 || apply {
                x.setLeftBorderPaletteIdx(bor.getLeftBorderColor());
                x.setBorderLeft(bor.getBorderLeft() as i16);
            }
            // java if((short)bor.getBorderTop()!=0||applyBorder) {this.style.setTopBorderPaletteIdx(bor.getTopBorderColor());this.style.setBorderTop((short)bor.getBorderTop());}
            if bor.getBorderTop() as i16 != 0 || apply {
                x.setTopBorderPaletteIdx(bor.getTopBorderColor());
                x.setBorderTop(bor.getBorderTop() as i16);
            }
            // java if((short)bor.getBorderBottom()!=0||applyBorder) {this.style.setBottomBorderPaletteIdx(bor.getBottomBorderColor());this.style.setBorderBottom((short)bor.getBorderBottom());}
            if bor.getBorderBottom() as i16 != 0 || apply {
                x.setBottomBorderPaletteIdx(bor.getBottomBorderColor());
                x.setBorderBottom(bor.getBorderBottom() as i16);
            }
            // rust: if(bor.isBackwardDiagonalOn()) this.setBackwardDiagonalOn(true);if(bor.isForwardDiagonalOn()) this.setForwardDiagonalOn(true);
            if bor.isBackwardDiagonalOn() {
                let diag = x.getDiag();
                x.setDiag(diag | 1);
            }
            if bor.isForwardDiagonalOn() {
                let diag = x.getDiag();
                x.setDiag(diag | 2);
            }
            // java if((short)bor.getBorderDiagonal()!=0||applyBorder) {this.style.setAdtlDiag(bor.getDiagonalBorderColor());this.style.setAdtlDiagLineStyle((short)bor.getBorderDiagonal());}this.style.setValidBorder(applyBorder);
            if bor.getBorderDiagonal() as i16 != 0 || apply {
                x.setAdtlDiag(bor.getDiagonalBorderColor());
                x.setAdtlDiagLineStyle(bor.getBorderDiagonal() as i16);
            }
            x.setValidBorder(apply);
        }
    }
    // java public void setBackwardDiagonalOn(boolean on)

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:332
    // java public void setBackwardDiagonalOn(boolean on) {
    // java byte diag = this.style.getDiag();
    // java if (on) {
    // java diag = (byte)(diag | 1);
    // java } else {
    // java diag = (byte)(diag & 2);
    // java }
    // java
    // java this.style.setDiag(diag);
    // java }
    pub fn setBackwardDiagonalOn(&self, on: bool) {
        let p = self.style();
        let mut x = p.borrow_mut();
        let diag = x.getDiag();
        x.setDiag(if on { diag | 1 } else { diag & 2 });
    }
    // java public void setForwardDiagonalOn(boolean on)

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:343
    // java public void setForwardDiagonalOn(boolean on) {
    // java byte diag = this.style.getDiag();
    // java if (on) {
    // java diag = (byte)(diag | 2);
    // java } else {
    // java diag = (byte)(diag & 1);
    // java }
    // java
    // java this.style.setDiag(diag);
    // java }
    pub fn setForwardDiagonalOn(&self, on: bool) {
        let p = self.style();
        let mut x = p.borrow_mut();
        let diag = x.getDiag();
        x.setDiag(if on { diag | 2 } else { diag & 1 });
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:354
    // java public void endElement(int element) {
    // java if (!this.isStyleXf) {
    // java if (this.bFirstCellXf) {
    // java this.bFirstCellXf = false;
    // java } else {
    // java if (null == this.style) {
    // java return;
    // java }
    // java
    // java if (this.mXFIndex == -1) {
    // java this.kmoXFManager.createXF(this.style);
    // java } else {
    // java this.kmoXFManager.setNullXF(this.mXFIndex, this.style);
    // java }
    // java }
    // java }
    // java
    // java }
    pub fn endElement(&self, _: i32) {
        // rust: if(!this.isStyleXf) {if(this.bFirstCellXf) this.bFirstCellXf=false;else {if(null==this.style)return;if(this.mXFIndex==-1)this.kmoXFManager.createXF(this.style);else this.kmoXFManager.setNullXF(this.mXFIndex,this.style);}}
        let s = &self.shared;
        if !s.isStyleXf.get() {
            if s.bFirstCellXf.get() {
                s.bFirstCellXf.set(false);
            } else if let Some(style) = s.style.borrow().as_ref() {
                let manager = s.book.getXFManager().borrow();
                if s.mXFIndex.get() == -1 {
                    manager.createXF(&style.borrow());
                } else {
                    manager.setNullXF(s.mXFIndex.get(), &style.borrow());
                }
            }
        }
    }
}
impl ElementHandler for XFormatHandler {
    fn startElement(&self, e: i32, a: &dyn XmlRoAttr) {
        XFormatHandler::startElement(self, e, a);
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:354
    // java public void endElement(int element) {
    // java if (!this.isStyleXf) {
    // java if (this.bFirstCellXf) {
    // java this.bFirstCellXf = false;
    // java } else {
    // java if (null == this.style) {
    // java return;
    // java }
    // java
    // java if (this.mXFIndex == -1) {
    // java this.kmoXFManager.createXF(this.style);
    // java } else {
    // java this.kmoXFManager.setNullXF(this.mXFIndex, this.style);
    // java }
    // java }
    // java }
    // java
    // java }
    fn endElement(&self, e: i32) {
        XFormatHandler::endElement(self, e);
    }
    fn characters(&self, _: &str) {}

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:234
    // java public ElementHandler enterSubElement(int subElement) {
    // java if (subElement == 4668) {
    // java this.alignHandler.init(this.style);
    // java return this.alignHandler;
    // java } else {
    // java return subElement == 5163 ? this.protectionHandler : null;
    // java }
    // java }
    fn enterSubElement(&self, e: i32) -> Option<Box<dyn ElementHandler>> {
        // rust: if(subElement==4668) {this.alignHandler.init(this.style);return this.alignHandler;}else return subElement==5163?this.protectionHandler:null;
        match e {
            4668 => Some(Box::new(super::AlignmentHandler::AlignmentHandler::new(
                self.style(),
            ))),
            5163 => Some(Box::new(ProtectionHandler(self.style()))),
            _ => None,
        }
    }
}
// java private final class ProtectionHandler extends DefaultElementHandler
pub struct ProtectionHandler(Rc<RefCell<ExtendedFormatRecord>>);
impl ElementHandler for ProtectionHandler {
    fn startElement(&self, _: i32, a: &dyn XmlRoAttr) {
        // rust: if((tmp=attrsNotCopy.getByName(4168))!=null) XFormatHandler.this.style.setLocked(tmp.toBoolean());if((tmp=attrsNotCopy.getByName(4407))!=null) XFormatHandler.this.style.setHidden(tmp.toBoolean());
        if let Some(x) = a.get_by_name(4168) {
            self.0.borrow_mut().setLocked(x.to_boolean());
        }
        if let Some(x) = a.get_by_name(4407) {
            self.0.borrow_mut().setHidden(x.to_boolean());
        }
    }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/handler/style/XFormatHandler.java:354
    // java public void endElement(int element) {
    // java if (!this.isStyleXf) {
    // java if (this.bFirstCellXf) {
    // java this.bFirstCellXf = false;
    // java } else {
    // java if (null == this.style) {
    // java return;
    // java }
    // java
    // java if (this.mXFIndex == -1) {
    // java this.kmoXFManager.createXF(this.style);
    // java } else {
    // java this.kmoXFManager.setNullXF(this.mXFIndex, this.style);
    // java }
    // java }
    // java }
    // java
    // java }
    fn endElement(&self, _: i32) {}
    fn characters(&self, _: &str) {}
    fn enterSubElement(&self, _: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
}
