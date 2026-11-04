use super::KmoFont::KmoFont;
use super::KmoXFData::KmoXFData;
use crate::cn::wps::moss::filefmt::biff8::record::{
    common::XFGradient::{GradientRef, XFGradient},
    ExtendedFormatRecord::ExtendedFormatRecord,
    FontRecord::FontRecord,
};
use crate::java_runtime::{Rc, RefCell};
use kfc::cn::wps::memor4j::{
    AtomSnapshot::SharedAtom,
    KmoAtom::KmoAtom,
    KmoAtomData::{KmoAtomData, KmoAtomDataSub},
};
// java public abstract class KmoXF extends KmoAtom<KmoXFData>
pub struct KmoXF {
    atom: Rc<RefCell<KmoAtom<KmoXFData>>>,
    _owner: SharedAtom,
    pub(crate) isStyle: bool,
}
impl KmoXF {
    pub(crate) fn fromData(data: KmoXFData, isStyle: bool) -> Self {
        let (atom, owner) = super::snapshot::bind(data);
        Self {
            atom,
            _owner: owner,
            isStyle,
        }
    }
    // java KmoXF() { super(new KmoXFData()); }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:144
    // java KmoXF() {
    // java super(new KmoXFData());
    // java }
    pub fn KmoXF() -> Self {
        Self::fromData(KmoXFData::new(), false)
    }
    // java KmoXF(ExtendedFormatRecord xfRec)

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:148
    // java KmoXF(ExtendedFormatRecord xfRec) {
    // java super(new KmoXFData());
    // java ((KmoXFData)this.rawData()).copyFromRecord(xfRec);
    // java }
    pub fn fromRecord(rec: &ExtendedFormatRecord) -> Self {
        let mut data = KmoXFData::new();
        data.copyFromRecord(rec);
        Self::fromData(data, rec.getXFType() != 0)
    }
    // java public void setFontIndex(short index) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).field_1_font_index = index;
    // java ((KmoXFData)this.rawData()).font = null;
    // java }
    pub fn setFontIndex(&self, index: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.field_1_font_index = index;
        data.font = None;
    }
    // java public void setFont(KmoFont font) {
    // java ((KmoXFData)this.rawData()).font = font;
    // java }
    pub fn setFont(&self, font: Rc<KmoFont>) {
        self.setFont__nullable(Some(font));
    }
    // java public void setFont(KmoFont font) { ((KmoXFData)this.rawData()).font = font; }
    // Native nullable Java-reference transport; no beforeWrite in the original setter.
    pub fn setFont__nullable(&self,font:Option<Rc<KmoFont>>){
        let mut atom = self.atom.borrow_mut();
        let mut data = atom.rawData_mut();
        data.font = font;
    }
    // java public void setFormatIndex(short index) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).field_2_format_index = index;
    // java ((KmoXFData)this.rawData()).numfmt = null;
    // java }
    pub fn setFormatIndex(&self, index: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.field_2_format_index = index;
        data.numfmt = None;
    }
    // java public void setNumFormat(String numfmt) {
    // java ((KmoXFData)this.rawData()).numfmt = numfmt;
    // java }
    pub fn setNumFormat(&self, numfmt: Option<String>) {
        let mut atom = self.atom.borrow_mut();
        let mut data = atom.rawData_mut();
        data.numfmt = numfmt;
    }
    // java public void setLocked(boolean locked) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).isLocked = locked;
    // java ((KmoXFData)this.rawData()).field_3_cell_options = _locked.setShortBoolean(((KmoXFData)this.rawData()).field_3_cell_options, locked);
    // java }
    pub fn setLocked(&self, locked: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.isLocked = locked;
        data.field_3_cell_options =
            (((data.field_3_cell_options) as i32 & !1) | if locked { 1 } else { 0 }) as i16;
    }
    // java public void setHidden(boolean hidden) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).isHidden = hidden;
    // java ((KmoXFData)this.rawData()).field_3_cell_options = _hidden.setShortBoolean(((KmoXFData)this.rawData()).field_3_cell_options, hidden);
    // java }
    pub fn setHidden(&self, hidden: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.isHidden = hidden;
        data.field_3_cell_options =
            (((data.field_3_cell_options) as i32 & !2) | if hidden { 2 } else { 0 }) as i16;
    }
    // java public void setXFType(short type) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).xfType = type;
    // java ((KmoXFData)this.rawData()).field_3_cell_options = _xf_type.setShortValue(((KmoXFData)this.rawData()).field_3_cell_options, type);
    // java }
    pub fn setXFType(&self, r#type: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.xfType = r#type;
        data.field_3_cell_options = (((data.field_3_cell_options) as i32 & !4)
            | ((r#type as i32).wrapping_shl(2) & 4)) as i16;
    }
    // java public void set123Prefix(boolean prefix) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData())._123prefix = prefix;
    // java ((KmoXFData)this.rawData()).field_3_cell_options = _123_prefix.setShortBoolean(((KmoXFData)this.rawData()).field_3_cell_options, prefix);
    // java }
    pub fn set123Prefix(&self, prefix: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data._123prefix = prefix;
        data.field_3_cell_options =
            (((data.field_3_cell_options) as i32 & !8) | if prefix { 8 } else { 0 }) as i16;
    }
    // java public void setParentIndex(short parent) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).parentIndex = parent;
    // java ((KmoXFData)this.rawData()).field_3_cell_options = _parent_index.setShortValue(((KmoXFData)this.rawData()).field_3_cell_options, parent);
    // java }
    pub fn setParentIndex(&self, parent: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.parentIndex = parent;
        data.field_3_cell_options = (((data.field_3_cell_options) as i32 & !65520)
            | ((parent as i32).wrapping_shl(4) & 65520)) as i16;
    }
    // java public void setAlignment(short align) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).alignment = align;
    // java ((KmoXFData)this.rawData()).field_4_alignment_options = _alignment.setShortValue(((KmoXFData)this.rawData()).field_4_alignment_options, align);
    // java }
    pub fn setAlignment(&self, align: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.alignment = align;
        data.field_4_alignment_options = (((data.field_4_alignment_options) as i32 & !7)
            | ((align as i32).wrapping_shl(0) & 7)) as i16;
    }
    // java public void setWrapText(boolean wrapped) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).isWrapText = wrapped;
    // java ((KmoXFData)this.rawData()).field_4_alignment_options = _wrap_text.setShortBoolean(((KmoXFData)this.rawData()).field_4_alignment_options, wrapped);
    // java }
    pub fn setWrapText(&self, wrapped: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.isWrapText = wrapped;
        data.field_4_alignment_options =
            (((data.field_4_alignment_options) as i32 & !8) | if wrapped { 8 } else { 0 }) as i16;
    }
    // java public void setVerticalAlignment(short align) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).verticalAlignment = align;
    // java ((KmoXFData)this.rawData()).field_4_alignment_options = _vertical_alignment.setShortValue(((KmoXFData)this.rawData()).field_4_alignment_options, align);
    // java }
    pub fn setVerticalAlignment(&self, align: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.verticalAlignment = align;
        data.field_4_alignment_options = (((data.field_4_alignment_options) as i32 & !112)
            | ((align as i32).wrapping_shl(4) & 112))
            as i16;
    }
    // java public void setJustifyLast(short justify) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).justifyLast = justify;
    // java ((KmoXFData)this.rawData()).field_4_alignment_options = _justify_last.setShortValue(((KmoXFData)this.rawData()).field_4_alignment_options, justify);
    // java }
    pub fn setJustifyLast(&self, justify: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.justifyLast = justify;
        data.field_4_alignment_options = (((data.field_4_alignment_options) as i32 & !128)
            | ((justify as i32).wrapping_shl(7) & 128))
            as i16;
    }
    // java public void setRotation(short rotation) {
    // java if (rotation < 0) {
    // java rotation = (short)(90 - rotation);
    // java }
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).rotation = rotation;
    // java ((KmoXFData)this.rawData()).field_4_alignment_options = _rotation.setShortValue(((KmoXFData)this.rawData()).field_4_alignment_options, rotation);
    // java }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:235
    // java public void setRotation(short rotation) {
    // java if (rotation < 0) {
    // java rotation = (short)(90 - rotation);
    // java }
    // java
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).rotation = rotation;
    // java ((KmoXFData)this.rawData()).field_4_alignment_options = _rotation.setShortValue(((KmoXFData)this.rawData()).field_4_alignment_options, rotation);
    // java }
    pub fn setRotation(&self, rotation: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        let rotation = if rotation < 0 {
            90 - rotation
        } else {
            rotation
        };
        data.rotation = rotation;
        data.field_4_alignment_options = (((data.field_4_alignment_options) as i32 & !65280)
            | ((rotation as i32).wrapping_shl(8) & 65280))
            as i16;
    }
    // java public void setIndent(short indent) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).indent = indent;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _indent.setShortValue(((KmoXFData)this.rawData()).field_5_indention_options, indent);
    // java }
    pub fn setIndent(&self, indent: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.indent = indent;
        data.field_5_indention_options = (((data.field_5_indention_options) as i32 & !15)
            | ((indent as i32).wrapping_shl(0) & 15))
            as i16;
    }
    // java public void setShrinkToFit(boolean shrink) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).isShrink2Fit = shrink;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _shrink_to_fit.setShortBoolean(((KmoXFData)this.rawData()).field_5_indention_options, shrink);
    // java }
    pub fn setShrinkToFit(&self, shrink: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.isShrink2Fit = shrink;
        data.field_5_indention_options =
            (((data.field_5_indention_options) as i32 & !16) | if shrink { 16 } else { 0 }) as i16;
    }
    // java public void setMergeCells(boolean merge) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).isMergeCells = merge;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _merge_cells.setShortBoolean(((KmoXFData)this.rawData()).field_5_indention_options, merge);
    // java }
    pub fn setMergeCells(&self, merge: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.isMergeCells = merge;
        data.field_5_indention_options =
            (((data.field_5_indention_options) as i32 & !32) | if merge { 32 } else { 0 }) as i16;
    }
    // java public void setReadingOrder(short order) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).readingOrder = order;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _reading_order.setShortValue(((KmoXFData)this.rawData()).field_5_indention_options, order);
    // java }
    pub fn setReadingOrder(&self, order: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.readingOrder = order;
        data.field_5_indention_options = (((data.field_5_indention_options) as i32 & !192)
            | ((order as i32).wrapping_shl(6) & 192))
            as i16;
    }
    // java public void setAttrFormat(boolean attr) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).isAttrFormat = attr;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _attr_format.setShortBoolean(((KmoXFData)this.rawData()).field_5_indention_options, attr);
    // java }
    pub fn setAttrFormat(&self, attr: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.isAttrFormat = attr;
        data.field_5_indention_options = (((data.field_5_indention_options) as i32 & !1024)
            | if attr { 1024 } else { 0 }) as i16;
    }
    // java public void setValidFormat(boolean valid) {
    // java boolean isStyle = this.getXFType() == 1;
    // java this.beforeWrite();
    // java valid = isStyle ? !valid : valid;
    // java ((KmoXFData)this.rawData()).isAttrFormat = valid;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _attr_format.setShortBoolean(((KmoXFData)this.rawData()).field_5_indention_options, valid);
    // java }
    pub fn setValidFormat(&self, valid: bool) {
        let isStyle = self.getXFType() == 1;
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        let valid = if isStyle { !valid } else { valid };
        data.isAttrFormat = valid;
        data.field_5_indention_options = (((data.field_5_indention_options) as i32 & !1024)
            | if valid { 1024 } else { 0 }) as i16;
    }
    // java public void setAttrFont(boolean attr) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).isAttrFont = attr;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _attr_font.setShortBoolean(((KmoXFData)this.rawData()).field_5_indention_options, attr);
    // java }
    pub fn setAttrFont(&self, attr: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.isAttrFont = attr;
        data.field_5_indention_options = (((data.field_5_indention_options) as i32 & !2048)
            | if attr { 2048 } else { 0 }) as i16;
    }
    // java public void setValidFont(boolean valid) {
    // java boolean isStyle = this.getXFType() == 1;
    // java this.beforeWrite();
    // java valid = isStyle ? !valid : valid;
    // java ((KmoXFData)this.rawData()).isAttrFont = valid;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _attr_font.setShortBoolean(((KmoXFData)this.rawData()).field_5_indention_options, valid);
    // java }
    pub fn setValidFont(&self, valid: bool) {
        let isStyle = self.getXFType() == 1;
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        let valid = if isStyle { !valid } else { valid };
        data.isAttrFont = valid;
        data.field_5_indention_options = (((data.field_5_indention_options) as i32 & !2048)
            | if valid { 2048 } else { 0 }) as i16;
    }
    // java public void setAttrAlignment(boolean attr) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).isAttrAlignment = attr;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _attr_alignment.setShortBoolean(((KmoXFData)this.rawData()).field_5_indention_options, attr);
    // java }
    pub fn setAttrAlignment(&self, attr: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.isAttrAlignment = attr;
        data.field_5_indention_options = (((data.field_5_indention_options) as i32 & !4096)
            | if attr { 4096 } else { 0 }) as i16;
    }
    // java public void setValidAlignment(boolean valid) {
    // java boolean isStyle = this.getXFType() == 1;
    // java this.beforeWrite();
    // java valid = isStyle ? !valid : valid;
    // java ((KmoXFData)this.rawData()).isAttrAlignment = valid;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _attr_alignment.setShortBoolean(((KmoXFData)this.rawData()).field_5_indention_options, valid);
    // java }
    pub fn setValidAlignment(&self, valid: bool) {
        let isStyle = self.getXFType() == 1;
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        let valid = if isStyle { !valid } else { valid };
        data.isAttrAlignment = valid;
        data.field_5_indention_options = (((data.field_5_indention_options) as i32 & !4096)
            | if valid { 4096 } else { 0 }) as i16;
    }
    // java public void setAttrBorder(boolean attr) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).isAttrBorder = attr;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _attr_border.setShortBoolean(((KmoXFData)this.rawData()).field_5_indention_options, attr);
    // java }
    pub fn setAttrBorder(&self, attr: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.isAttrBorder = attr;
        data.field_5_indention_options = (((data.field_5_indention_options) as i32 & !8192)
            | if attr { 8192 } else { 0 }) as i16;
    }
    // java public void setValidBorder(boolean valid) {
    // java boolean isStyle = this.getXFType() == 1;
    // java this.beforeWrite();
    // java valid = isStyle ? !valid : valid;
    // java ((KmoXFData)this.rawData()).isAttrBorder = valid;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _attr_border.setShortBoolean(((KmoXFData)this.rawData()).field_5_indention_options, valid);
    // java }
    pub fn setValidBorder(&self, valid: bool) {
        let isStyle = self.getXFType() == 1;
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        let valid = if isStyle { !valid } else { valid };
        data.isAttrBorder = valid;
        data.field_5_indention_options = (((data.field_5_indention_options) as i32 & !8192)
            | if valid { 8192 } else { 0 }) as i16;
    }
    // java public void setAttrPattern(boolean attr) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).isAttrPattern = attr;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _attr_pattern.setShortBoolean(((KmoXFData)this.rawData()).field_5_indention_options, attr);
    // java }
    pub fn setAttrPattern(&self, attr: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.isAttrPattern = attr;
        data.field_5_indention_options = (((data.field_5_indention_options) as i32 & !16384)
            | if attr { 16384 } else { 0 }) as i16;
    }
    // java public void setValidPattern(boolean valid) {
    // java boolean isStyle = this.getXFType() == 1;
    // java this.beforeWrite();
    // java valid = isStyle ? !valid : valid;
    // java ((KmoXFData)this.rawData()).isAttrPattern = valid;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _attr_pattern.setShortBoolean(((KmoXFData)this.rawData()).field_5_indention_options, valid);
    // java }
    pub fn setValidPattern(&self, valid: bool) {
        let isStyle = self.getXFType() == 1;
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        let valid = if isStyle { !valid } else { valid };
        data.isAttrPattern = valid;
        data.field_5_indention_options = (((data.field_5_indention_options) as i32 & !16384)
            | if valid { 16384 } else { 0 }) as i16;
    }
    // java public void setAttrCellOptions(boolean attr) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).isAttrOptions = attr;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _attr_options.setShortBoolean(((KmoXFData)this.rawData()).field_5_indention_options, attr);
    // java }
    pub fn setAttrCellOptions(&self, attr: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.isAttrOptions = attr;
        data.field_5_indention_options = (((data.field_5_indention_options) as i32 & !32768)
            | if attr { 32768 } else { 0 }) as i16;
    }
    // java public void setValidCellOptions(boolean valid) {
    // java boolean isStyle = this.getXFType() == 1;
    // java this.beforeWrite();
    // java valid = isStyle ? !valid : valid;
    // java ((KmoXFData)this.rawData()).isAttrOptions = valid;
    // java ((KmoXFData)this.rawData()).field_5_indention_options = _attr_options.setShortBoolean(((KmoXFData)this.rawData()).field_5_indention_options, valid);
    // java }
    pub fn setValidCellOptions(&self, valid: bool) {
        let isStyle = self.getXFType() == 1;
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        let valid = if isStyle { !valid } else { valid };
        data.isAttrOptions = valid;
        data.field_5_indention_options = (((data.field_5_indention_options) as i32 & !32768)
            | if valid { 32768 } else { 0 }) as i16;
    }
    // java public void setBorderLeft(short border) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).borderLeft = border;
    // java ((KmoXFData)this.rawData()).field_6_border_options = _border_left.setShortValue(((KmoXFData)this.rawData()).field_6_border_options, border);
    // java }
    pub fn setBorderLeft(&self, border: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.borderLeft = border;
        data.field_6_border_options = (((data.field_6_border_options) as i32 & !15)
            | ((border as i32).wrapping_shl(0) & 15)) as i16;
    }
    // java public void setBorderRight(short border) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).borderRight = border;
    // java ((KmoXFData)this.rawData()).field_6_border_options = _border_right.setShortValue(((KmoXFData)this.rawData()).field_6_border_options, border);
    // java }
    pub fn setBorderRight(&self, border: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.borderRight = border;
        data.field_6_border_options = (((data.field_6_border_options) as i32 & !240)
            | ((border as i32).wrapping_shl(4) & 240)) as i16;
    }
    // java public void setBorderTop(short border) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).borderTop = border;
    // java ((KmoXFData)this.rawData()).field_6_border_options = _border_top.setShortValue(((KmoXFData)this.rawData()).field_6_border_options, border);
    // java }
    pub fn setBorderTop(&self, border: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.borderTop = border;
        data.field_6_border_options = (((data.field_6_border_options) as i32 & !3840)
            | ((border as i32).wrapping_shl(8) & 3840))
            as i16;
    }
    // java public void setBorderBottom(short border) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).borderBottom = border;
    // java ((KmoXFData)this.rawData()).field_6_border_options = _border_bottom.setShortValue(((KmoXFData)this.rawData()).field_6_border_options, border);
    // java }
    pub fn setBorderBottom(&self, border: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.borderBottom = border;
        data.field_6_border_options = (((data.field_6_border_options) as i32 & !61440)
            | ((border as i32).wrapping_shl(12) & 61440))
            as i16;
    }
    // java public void setLeftBorderPaletteIdx(int border) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).border_left_color = border;
    // java }
    pub fn setLeftBorderPaletteIdx(&self, border: i32) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.border_left_color = border;
    }
    // java public void setRightBorderPaletteIdx(int border) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).border_right_color = border;
    // java }
    pub fn setRightBorderPaletteIdx(&self, border: i32) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.border_right_color = border;
    }
    // java public void setDiag(byte diag) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).border_diag_flag = diag;
    // java }
    pub fn setDiag(&self, diag: i8) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.border_diag_flag = diag;
    }
    // java public void setTopBorderPaletteIdx(int border) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).border_top_color = border;
    // java }
    pub fn setTopBorderPaletteIdx(&self, border: i32) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.border_top_color = border;
    }
    // java public void setBottomBorderPaletteIdx(int border) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).border_bottom_color = border;
    // java }
    pub fn setBottomBorderPaletteIdx(&self, border: i32) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.border_bottom_color = border;
    }
    // java public void setAdtlDiag(int diag) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).border_diag_color = diag;
    // java }
    pub fn setAdtlDiag(&self, diag: i32) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.border_diag_color = diag;
    }
    // java public void setAdtlDiagLineStyle(short diag) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).adtlDiagLineStyle = diag;
    // java ((KmoXFData)this.rawData()).field_8_adtl_palette_options = _adtl_diag_line_style.setValue(((KmoXFData)this.rawData()).field_8_adtl_palette_options, diag);
    // java }
    pub fn setAdtlDiagLineStyle(&self, diag: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.adtlDiagLineStyle = diag;
        data.field_8_adtl_palette_options =
            (((data.field_8_adtl_palette_options) as i32 & !31457280)
                | ((diag as i32).wrapping_shl(21) & 31457280)) as i32;
    }
    // java public void setAdtlFillPattern(short fill) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).adtlFillPattern = fill;
    // java ((KmoXFData)this.rawData()).field_8_adtl_palette_options = _adtl_fill_pattern.setValue(((KmoXFData)this.rawData()).field_8_adtl_palette_options, fill);
    // java }
    pub fn setAdtlFillPattern(&self, fill: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.adtlFillPattern = fill;
        data.field_8_adtl_palette_options =
            (((data.field_8_adtl_palette_options) as i32 & !-67108864)
                | ((fill as i32).wrapping_shl(26) & -67108864)) as i32;
    }
    // java public void setFillForeground(int color) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).fill_fore_color = color;
    // java }
    pub fn setFillForeground(&self, color: i32) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.fill_fore_color = color;
    }
    // java public void setFillBackground(int color) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).fill_back_color = color;
    // java }
    pub fn setFillBackground(&self, color: i32) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.fill_back_color = color;
    }
    // java public short getFontIndex() {
    // java return ((KmoXFData)this.rawData()).field_1_font_index;
    // java }
    pub fn getFontIndex(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.field_1_font_index
    }
    // java public KmoFont getFont() {
    // java return ((KmoXFData)this.rawData()).font;
    // java }
    pub fn getFont(&self) -> Option<Rc<KmoFont>> {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.font.clone()
    }
    // java public short getFormatIndex() {
    // java return ((KmoXFData)this.rawData()).field_2_format_index;
    // java }
    pub fn getFormatIndex(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.field_2_format_index
    }
    // java public String getNumFormat() {
    // java return ((KmoXFData)this.rawData()).numfmt;
    // java }
    pub fn getNumFormat(&self) -> Option<String> {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.numfmt.clone()
    }
    // java public boolean isLocked() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.isLocked : _locked.isSet(data.field_3_cell_options);
    // java }
    pub fn isLocked(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isLocked
        } else {
            ((data.field_3_cell_options) as i32 & 1) != 0
        }
    }
    // java public boolean isHidden() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.isHidden : _hidden.isSet(data.field_3_cell_options);
    // java }
    pub fn isHidden(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isHidden
        } else {
            ((data.field_3_cell_options) as i32 & 2) != 0
        }
    }
    // java public short getXFType() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.xfType : _xf_type.getShortValue(data.field_3_cell_options);
    // java }
    pub fn getXFType(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.xfType
        } else {
            (((data.field_3_cell_options) as u32 & (4i32 as u32)) >> 2) as i16
        }
    }
    // java public boolean get123Prefix() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data._123prefix : _123_prefix.isSet(data.field_3_cell_options);
    // java }
    pub fn get123Prefix(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data._123prefix
        } else {
            ((data.field_3_cell_options) as i32 & 8) != 0
        }
    }
    // java public short getParentIndex() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.parentIndex : _parent_index.getShortValue(data.field_3_cell_options);
    // java }
    pub fn getParentIndex(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.parentIndex
        } else {
            (((data.field_3_cell_options) as u32 & (65520i32 as u32)) >> 4) as i16
        }
    }
    // java public short getAlignment() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.alignment : _alignment.getShortValue(data.field_4_alignment_options);
    // java }
    pub fn getAlignment(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.alignment
        } else {
            (((data.field_4_alignment_options) as u32 & (7i32 as u32)) >> 0) as i16
        }
    }
    // java public boolean getWrapText() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.isWrapText : _wrap_text.isSet(data.field_4_alignment_options);
    // java }
    pub fn getWrapText(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isWrapText
        } else {
            ((data.field_4_alignment_options) as i32 & 8) != 0
        }
    }
    // java public short getVerticalAlignment() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.verticalAlignment : _vertical_alignment.getShortValue(data.field_4_alignment_options);
    // java }
    pub fn getVerticalAlignment(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.verticalAlignment
        } else {
            (((data.field_4_alignment_options) as u32 & (112i32 as u32)) >> 4) as i16
        }
    }
    // java public short getJustifyLast() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.justifyLast : _justify_last.getShortValue(data.field_4_alignment_options);
    // java }
    pub fn getJustifyLast(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.justifyLast
        } else {
            (((data.field_4_alignment_options) as u32 & (128i32 as u32)) >> 7) as i16
        }
    }
    // java public short getRotation() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java short rotation;
    // java if (data.hasInit) {
    // java rotation = data.rotation;
    // java } else {
    // java rotation = _rotation.getShortValue(data.field_4_alignment_options);
    // java }
    // java if (rotation == 255) {
    // java return rotation;
    // java } else {
    // java if (rotation > 90) {
    // java rotation = (short)(90 - rotation);
    // java }
    // java return rotation;
    // java }
    // java }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:490
    // java public short getRotation() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java short rotation;
    // java if (data.hasInit) {
    // java rotation = data.rotation;
    // java } else {
    // java rotation = _rotation.getShortValue(data.field_4_alignment_options);
    // java }
    // java
    // java if (rotation == 255) {
    // java return rotation;
    // java } else {
    // java if (rotation > 90) {
    // java rotation = (short)(90 - rotation);
    // java }
    // java
    // java return rotation;
    // java }
    // java }
    pub fn getRotation(&self) -> i16 {
        let data_guard = self.atom.borrow();
        let data = data_guard.rawData();
        let rotation = if data.hasInit {
            data.rotation
        } else {
            ((data.field_4_alignment_options as u16) >> 8) as i16
        };
        if rotation == 255 {
            rotation
        } else if rotation > 90 {
            90 - rotation
        } else {
            rotation
        }
    }
    // java public short getIndent() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.indent : _indent.getShortValue(data.field_5_indention_options);
    // java }
    pub fn getIndent(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.indent
        } else {
            (((data.field_5_indention_options) as u32 & (15i32 as u32)) >> 0) as i16
        }
    }
    // java public boolean getShrinkToFit() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.isShrink2Fit : _shrink_to_fit.isSet(data.field_5_indention_options);
    // java }
    pub fn getShrinkToFit(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isShrink2Fit
        } else {
            ((data.field_5_indention_options) as i32 & 16) != 0
        }
    }
    // java public boolean getMergeCells() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.isMergeCells : _merge_cells.isSet(data.field_5_indention_options);
    // java }
    pub fn getMergeCells(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isMergeCells
        } else {
            ((data.field_5_indention_options) as i32 & 32) != 0
        }
    }
    // java public short getReadingOrder() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.readingOrder : _reading_order.getShortValue(data.field_5_indention_options);
    // java }
    pub fn getReadingOrder(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.readingOrder
        } else {
            (((data.field_5_indention_options) as u32 & (192i32 as u32)) >> 6) as i16
        }
    }
    // java public boolean getAttrFormat() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.isAttrFormat : _attr_format.isSet(data.field_5_indention_options);
    // java }
    pub fn getAttrFormat(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isAttrFormat
        } else {
            ((data.field_5_indention_options) as i32 & 1024) != 0
        }
    }
    // java public boolean isValidFormat() {
    // java boolean isStyle = this.getXFType() == 1;
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java boolean attr;
    // java if (data.hasInit) {
    // java attr = data.isAttrFormat;
    // java } else {
    // java attr = _attr_format.isSet(data.field_5_indention_options);
    // java }
    // java return isStyle ? !attr : attr;
    // java }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:535
    // java public boolean isValidFormat() {
    // java boolean isStyle = this.getXFType() == 1;
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java boolean attr;
    // java if (data.hasInit) {
    // java attr = data.isAttrFormat;
    // java } else {
    // java attr = _attr_format.isSet(data.field_5_indention_options);
    // java }
    // java
    // java return isStyle ? !attr : attr;
    // java }
    pub fn isValidFormat(&self) -> bool {
        let isStyle = self.getXFType() == 1;
        let attr = self.getAttrFormat();
        if isStyle {
            !attr
        } else {
            attr
        }
    }
    // java public boolean getAttrFont() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.isAttrFont : _attr_font.isSet(data.field_5_indention_options);
    // java }
    pub fn getAttrFont(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isAttrFont
        } else {
            ((data.field_5_indention_options) as i32 & 2048) != 0
        }
    }
    // java public boolean isValidFont() {
    // java boolean isStyle = this.getXFType() == 1;
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java boolean attr;
    // java if (data.hasInit) {
    // java attr = data.isAttrFont;
    // java } else {
    // java attr = _attr_font.isSet(data.field_5_indention_options);
    // java }
    // java return isStyle ? !attr : attr;
    // java }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:553
    // java public boolean isValidFont() {
    // java boolean isStyle = this.getXFType() == 1;
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java boolean attr;
    // java if (data.hasInit) {
    // java attr = data.isAttrFont;
    // java } else {
    // java attr = _attr_font.isSet(data.field_5_indention_options);
    // java }
    // java
    // java return isStyle ? !attr : attr;
    // java }
    pub fn isValidFont(&self) -> bool {
        let isStyle = self.getXFType() == 1;
        let attr = self.getAttrFont();
        if isStyle {
            !attr
        } else {
            attr
        }
    }
    // java public boolean getAttrAlignment() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.isAttrAlignment : _attr_alignment.isSet(data.field_5_indention_options);
    // java }
    pub fn getAttrAlignment(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isAttrAlignment
        } else {
            ((data.field_5_indention_options) as i32 & 4096) != 0
        }
    }
    // java public boolean isValidAlignment() {
    // java boolean isStyle = this.getXFType() == 1;
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java boolean attr;
    // java if (data.hasInit) {
    // java attr = data.isAttrAlignment;
    // java } else {
    // java attr = _attr_alignment.isSet(data.field_5_indention_options);
    // java }
    // java return isStyle ? !attr : attr;
    // java }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:571
    // java public boolean isValidAlignment() {
    // java boolean isStyle = this.getXFType() == 1;
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java boolean attr;
    // java if (data.hasInit) {
    // java attr = data.isAttrAlignment;
    // java } else {
    // java attr = _attr_alignment.isSet(data.field_5_indention_options);
    // java }
    // java
    // java return isStyle ? !attr : attr;
    // java }
    pub fn isValidAlignment(&self) -> bool {
        let isStyle = self.getXFType() == 1;
        let attr = self.getAttrAlignment();
        if isStyle {
            !attr
        } else {
            attr
        }
    }
    // java public boolean getAttrBorder() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.isAttrBorder : _attr_border.isSet(data.field_5_indention_options);
    // java }
    pub fn getAttrBorder(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isAttrBorder
        } else {
            ((data.field_5_indention_options) as i32 & 8192) != 0
        }
    }
    // java public boolean isValidBorder() {
    // java boolean isStyle = this.getXFType() == 1;
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java boolean attr;
    // java if (data.hasInit) {
    // java attr = data.isAttrBorder;
    // java } else {
    // java attr = _attr_border.isSet(data.field_5_indention_options);
    // java }
    // java return isStyle ? !attr : attr;
    // java }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:589
    // java public boolean isValidBorder() {
    // java boolean isStyle = this.getXFType() == 1;
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java boolean attr;
    // java if (data.hasInit) {
    // java attr = data.isAttrBorder;
    // java } else {
    // java attr = _attr_border.isSet(data.field_5_indention_options);
    // java }
    // java
    // java return isStyle ? !attr : attr;
    // java }
    pub fn isValidBorder(&self) -> bool {
        let isStyle = self.getXFType() == 1;
        let attr = self.getAttrBorder();
        if isStyle {
            !attr
        } else {
            attr
        }
    }
    // java public boolean getAttrPattern() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.isAttrPattern : _attr_pattern.isSet(data.field_5_indention_options);
    // java }
    pub fn getAttrPattern(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isAttrPattern
        } else {
            ((data.field_5_indention_options) as i32 & 16384) != 0
        }
    }
    // java public boolean isValidPattern() {
    // java boolean isStyle = this.getXFType() == 1;
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java boolean attr;
    // java if (data.hasInit) {
    // java attr = data.isAttrPattern;
    // java } else {
    // java attr = _attr_pattern.isSet(data.field_5_indention_options);
    // java }
    // java return isStyle ? !attr : attr;
    // java }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:607
    // java public boolean isValidPattern() {
    // java boolean isStyle = this.getXFType() == 1;
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java boolean attr;
    // java if (data.hasInit) {
    // java attr = data.isAttrPattern;
    // java } else {
    // java attr = _attr_pattern.isSet(data.field_5_indention_options);
    // java }
    // java
    // java return isStyle ? !attr : attr;
    // java }
    pub fn isValidPattern(&self) -> bool {
        let isStyle = self.getXFType() == 1;
        let attr = self.getAttrPattern();
        if isStyle {
            !attr
        } else {
            attr
        }
    }
    // java public boolean getAttrCellOptions() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.isAttrOptions : _attr_options.isSet(data.field_5_indention_options);
    // java }
    pub fn getAttrCellOptions(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isAttrOptions
        } else {
            ((data.field_5_indention_options) as i32 & 32768) != 0
        }
    }
    // java public boolean isValidCellOptions() {
    // java boolean isStyle = this.getXFType() == 1;
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java boolean attr;
    // java if (data.hasInit) {
    // java attr = data.isAttrOptions;
    // java } else {
    // java attr = _attr_options.isSet(data.field_5_indention_options);
    // java }
    // java return isStyle ? !attr : attr;
    // java }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:625
    // java public boolean isValidCellOptions() {
    // java boolean isStyle = this.getXFType() == 1;
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java boolean attr;
    // java if (data.hasInit) {
    // java attr = data.isAttrOptions;
    // java } else {
    // java attr = _attr_options.isSet(data.field_5_indention_options);
    // java }
    // java
    // java return isStyle ? !attr : attr;
    // java }
    pub fn isValidCellOptions(&self) -> bool {
        let isStyle = self.getXFType() == 1;
        let attr = self.getAttrCellOptions();
        if isStyle {
            !attr
        } else {
            attr
        }
    }
    // java public short getBorderLeft() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.borderLeft : _border_left.getShortValue(data.field_6_border_options);
    // java }
    pub fn getBorderLeft(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.borderLeft
        } else {
            (((data.field_6_border_options) as u32 & (15i32 as u32)) >> 0) as i16
        }
    }
    // java public short getBorderRight() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.borderRight : _border_right.getShortValue(data.field_6_border_options);
    // java }
    pub fn getBorderRight(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.borderRight
        } else {
            (((data.field_6_border_options) as u32 & (240i32 as u32)) >> 4) as i16
        }
    }
    // java public short getBorderTop() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.borderTop : _border_top.getShortValue(data.field_6_border_options);
    // java }
    pub fn getBorderTop(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.borderTop
        } else {
            (((data.field_6_border_options) as u32 & (3840i32 as u32)) >> 8) as i16
        }
    }
    // java public short getBorderBottom() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.borderBottom : _border_bottom.getShortValue(data.field_6_border_options);
    // java }
    pub fn getBorderBottom(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.borderBottom
        } else {
            (((data.field_6_border_options) as u32 & (61440i32 as u32)) >> 12) as i16
        }
    }
    // java public boolean hasBorder() {
    // java return ((KmoXFData)this.rawData()).field_6_border_options != 0;
    // java }
    pub fn hasBorder(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.field_6_border_options != 0
    }
    // java public boolean hasFill() {
    // java return this.getAdtlFillPattern() != 0;
    // java }
    pub fn hasFill(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        self.getAdtlFillPattern() != 0
    }
    // java public int getLeftBorderPaletteIdx() {
    // java return ((KmoXFData)this.rawData()).border_left_color;
    // java }
    pub fn getLeftBorderPaletteIdx(&self) -> i32 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.border_left_color
    }
    // java public int getRightBorderPaletteIdx() {
    // java return ((KmoXFData)this.rawData()).border_right_color;
    // java }
    pub fn getRightBorderPaletteIdx(&self) -> i32 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.border_right_color
    }
    // java public byte getDiag() {
    // java return ((KmoXFData)this.rawData()).border_diag_flag;
    // java }
    pub fn getDiag(&self) -> i8 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.border_diag_flag
    }
    // java public int getTopBorderPaletteIdx() {
    // java return ((KmoXFData)this.rawData()).border_top_color;
    // java }
    pub fn getTopBorderPaletteIdx(&self) -> i32 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.border_top_color
    }
    // java public int getBottomBorderPaletteIdx() {
    // java return ((KmoXFData)this.rawData()).border_bottom_color;
    // java }
    pub fn getBottomBorderPaletteIdx(&self) -> i32 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.border_bottom_color
    }
    // java public int getAdtlDiag() {
    // java return ((KmoXFData)this.rawData()).border_diag_color;
    // java }
    pub fn getAdtlDiag(&self) -> i32 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.border_diag_color
    }
    // java public short getAdtlDiagLineStyle() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.adtlDiagLineStyle : (short)_adtl_diag_line_style.getValue(data.field_8_adtl_palette_options);
    // java }
    pub fn getAdtlDiagLineStyle(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.adtlDiagLineStyle
        } else {
            ((((data.field_8_adtl_palette_options) as u32 & (31457280i32 as u32)) >> 21) as i32)
                as i16
        }
    }
    // java public short getAdtlFillPattern() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java return data.hasInit ? data.adtlFillPattern : (short)_adtl_fill_pattern.getValue(data.field_8_adtl_palette_options);
    // java }
    pub fn getAdtlFillPattern(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.adtlFillPattern
        } else {
            ((((data.field_8_adtl_palette_options) as u32 & (-67108864i32 as u32)) >> 26) as i32)
                as i16
        }
    }
    // java public int getFillForeground() {
    // java return ((KmoXFData)this.rawData()).fill_fore_color;
    // java }
    pub fn getFillForeground(&self) -> i32 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.fill_fore_color
    }
    // java public int getFillBackground() {
    // java int colorIdx = ((KmoXFData)this.rawData()).fill_back_color;
    // java if (colorIdx == 65) {
    // java colorIdx = 64;
    // java }
    // java return colorIdx;
    // java }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:704
    // java public int getFillBackground() {
    // java int colorIdx = ((KmoXFData)this.rawData()).fill_back_color;
    // java if (colorIdx == 65) {
    // java colorIdx = 64;
    // java }
    // java
    // java return colorIdx;
    // java }
    pub fn getFillBackground(&self) -> i32 {
        let colorIdx = self.atom.borrow().rawData().fill_back_color;
        if colorIdx == 65 {
            64
        } else {
            colorIdx
        }
    }
    // java public int getFillBackgroudWithoutTrans() {
    // java return ((KmoXFData)this.rawData()).fill_back_color;
    // java }
    pub fn getFillBackgroudWithoutTrans(&self) -> i32 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.fill_back_color
    }
    // java public boolean equalsInMeaning(KmoXF xf)

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:1131
    // java public boolean equalsInMeaning(KmoXF xf) {
    // java if (xf == this) {
    // java return true;
    // java } else if (!xf.getNumFormat().equals(this.getNumFormat())) {
    // java return false;
    // java } else {
    // java return !xf.getFont().equals(this.getFont()) ? false : ((KmoXFData)xf.rawData()).equalsInMeaning((KmoXFData)this.rawData());
    // java }
    // java }
    pub fn equalsInMeaning(&self, xf: &Self) -> bool {
        if std::ptr::eq(self, xf) {
            return true;
        }
        if Some(xf.getNumFormat().expect("NullPointerException")) != self.getNumFormat() {
            return false;
        }
        if Some(xf.getFont().expect("NullPointerException")) != self.getFont() {
            return false;
        }
        xf.atom
            .borrow()
            .rawData()
            .equalsInMeaning(&self.atom.borrow().rawData())
    }
    // java public void initOption() {
    // java KmoXFData data = (KmoXFData)this.rawData();
    // java short cellOption = data.field_3_cell_options;
    // java data.hasInit = true;
    // java data.isLocked = _locked.isSet(cellOption);
    // java data.isHidden = _hidden.isSet(cellOption);
    // java data.xfType = _xf_type.getShortValue(cellOption);
    // java data._123prefix = _123_prefix.isSet(cellOption);
    // java data.parentIndex = _parent_index.getShortValue(cellOption);
    // java short alignmentOptions = data.field_4_alignment_options;
    // java data.alignment = _alignment.getShortValue(alignmentOptions);
    // java data.isWrapText = _wrap_text.isSet(alignmentOptions);
    // java data.verticalAlignment = _vertical_alignment.getShortValue(alignmentOptions);
    // java data.justifyLast = _justify_last.getShortValue(alignmentOptions);
    // java data.rotation = _rotation.getShortValue(alignmentOptions);
    // java short indentionOption = data.field_5_indention_options;
    // java data.indent = _indent.getShortValue(indentionOption);
    // java data.isShrink2Fit = _shrink_to_fit.isSet(indentionOption);
    // java data.isMergeCells = _merge_cells.isSet(indentionOption);
    // java data.readingOrder = _reading_order.getShortValue(indentionOption);
    // java data.isAttrFormat = _attr_format.isSet(indentionOption);
    // java data.isAttrFont = _attr_font.isSet(indentionOption);
    // java data.isAttrAlignment = _attr_alignment.isSet(indentionOption);
    // java data.isAttrBorder = _attr_border.isSet(indentionOption);
    // java data.isAttrPattern = _attr_pattern.isSet(indentionOption);
    // java data.isAttrOptions = _attr_options.isSet(indentionOption);
    // java short borderOption = data.field_6_border_options;
    // java data.borderLeft = _border_left.getShortValue(borderOption);
    // java data.borderRight = _border_right.getShortValue(borderOption);
    // java data.borderTop = _border_top.getShortValue(borderOption);
    // java data.borderBottom = _border_bottom.getShortValue(borderOption);
    // java int adtlPaletteOption = data.field_8_adtl_palette_options;
    // java data.adtlDiagLineStyle = (short)_adtl_diag_line_style.getValue(adtlPaletteOption);
    // java data.adtlFillPattern = (short)_adtl_fill_pattern.getValue(adtlPaletteOption);
    // java if (data.font != null) {
    // java data.font.initOption();
    // java }
    // java
    // java }
    pub fn initOption(&self) {
        let mut atom = self.atom.borrow_mut();
        let mut data = atom.rawData_mut();
        data.hasInit = true;
        data.isLocked = ((data.field_3_cell_options) as i32 & 1) != 0;
        data.isHidden = ((data.field_3_cell_options) as i32 & 2) != 0;
        data.xfType = (((data.field_3_cell_options) as u32 & (4i32 as u32)) >> 2) as i16;
        data._123prefix = ((data.field_3_cell_options) as i32 & 8) != 0;
        data.parentIndex = (((data.field_3_cell_options) as u32 & (65520i32 as u32)) >> 4) as i16;
        data.alignment = (((data.field_4_alignment_options) as u32 & (7i32 as u32)) >> 0) as i16;
        data.isWrapText = ((data.field_4_alignment_options) as i32 & 8) != 0;
        data.verticalAlignment =
            (((data.field_4_alignment_options) as u32 & (112i32 as u32)) >> 4) as i16;
        data.justifyLast =
            (((data.field_4_alignment_options) as u32 & (128i32 as u32)) >> 7) as i16;
        data.rotation = ((data.field_4_alignment_options as u16 >> 8) & 255) as i16;
        data.indent = (((data.field_5_indention_options) as u32 & (15i32 as u32)) >> 0) as i16;
        data.isShrink2Fit = ((data.field_5_indention_options) as i32 & 16) != 0;
        data.isMergeCells = ((data.field_5_indention_options) as i32 & 32) != 0;
        data.readingOrder =
            (((data.field_5_indention_options) as u32 & (192i32 as u32)) >> 6) as i16;
        data.isAttrFormat = ((data.field_5_indention_options) as i32 & 1024) != 0;
        data.isAttrFont = ((data.field_5_indention_options) as i32 & 2048) != 0;
        data.isAttrAlignment = ((data.field_5_indention_options) as i32 & 4096) != 0;
        data.isAttrBorder = ((data.field_5_indention_options) as i32 & 8192) != 0;
        data.isAttrPattern = ((data.field_5_indention_options) as i32 & 16384) != 0;
        data.isAttrOptions = ((data.field_5_indention_options) as i32 & 32768) != 0;
        data.borderLeft = (((data.field_6_border_options) as u32 & (15i32 as u32)) >> 0) as i16;
        data.borderRight = (((data.field_6_border_options) as u32 & (240i32 as u32)) >> 4) as i16;
        data.borderTop = (((data.field_6_border_options) as u32 & (3840i32 as u32)) >> 8) as i16;
        data.borderBottom =
            (((data.field_6_border_options) as u32 & (61440i32 as u32)) >> 12) as i16;
        data.adtlDiagLineStyle = ((((data.field_8_adtl_palette_options) as u32
            & (31457280i32 as u32))
            >> 21) as i32) as i16;
        data.adtlFillPattern = ((((data.field_8_adtl_palette_options) as u32
            & (-67108864i32 as u32))
            >> 26) as i32) as i16;
        if let Some(font) = &data.font {
            font.initOption();
        }
    }
    // java public void setXFGradient(XFGradient gradient) {
    // java this.beforeWrite();
    // java ((KmoXFData)this.rawData()).gradient = gradient;
    // java }
    pub fn setXFGradient(&self, gradient: Option<GradientRef>) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.gradient = gradient;
    }
    // java public XFGradient getXFGradient() {
    // java return ((KmoXFData)this.rawData()).gradient;
    // java }
    pub fn getXFGradient(&self) -> Option<GradientRef> {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.gradient.clone()
    }

    // java public String toString() {
    pub fn toString(&self)->String{
        use std::fmt::Write;
        // java StringBuilder buffer = new StringBuilder(); buffer.append("[EXTENDEDFORMAT]\n");
        let mut buffer=String::from("[EXTENDEDFORMAT]\n");let hex=|v:i32|format!("{:x}",v as u32);
        // java if (this.getXFType() == 1) { buffer.append(" STYLE_RECORD_TYPE\n"); } else if (this.getXFType() == 0) { buffer.append(" CELL_RECORD_TYPE\n"); }
        if self.getXFType()==1{buffer.push_str(" STYLE_RECORD_TYPE\n");}else if self.getXFType()==0{buffer.push_str(" CELL_RECORD_TYPE\n");}
        // java buffer.append("    .fontindex       = ").append(Integer.toHexString(this.getFontIndex())).append("\n");
        writeln!(buffer,"    .fontindex       = {}",hex(self.getFontIndex()as i32)).unwrap();
        // java buffer.append("    .formatindex     = ").append(Integer.toHexString(this.getFormatIndex())).append("\n");
        writeln!(buffer,"    .formatindex     = {}",hex(self.getFormatIndex()as i32)).unwrap();
        // java buffer.append("    .celloptions     = ").append(Integer.toHexString(((KmoXFData)this.rawData()).field_3_cell_options)).append("\n");
        writeln!(buffer,"    .celloptions     = {}",hex(self.atom.borrow().rawData().field_3_cell_options as i32)).unwrap();
        // java buffer.append("          .islocked  = ").append(this.isLocked()).append("\n");
        writeln!(buffer,"          .islocked  = {}",self.isLocked()).unwrap();
        // java buffer.append("          .ishidden  = ").append(this.isHidden()).append("\n");
        writeln!(buffer,"          .ishidden  = {}",self.isHidden()).unwrap();
        // java buffer.append("          .recordtype= ").append(Integer.toHexString(this.getXFType())).append("\n");
        writeln!(buffer,"          .recordtype= {}",hex(self.getXFType()as i32)).unwrap();
        // java buffer.append("          .parentidx = ").append(Integer.toHexString(this.getParentIndex())).append("\n");
        writeln!(buffer,"          .parentidx = {}",hex(self.getParentIndex()as i32)).unwrap();
        // java buffer.append("    .alignmentoptions= ").append(Integer.toHexString(((KmoXFData)this.rawData()).field_4_alignment_options)).append("\n");
        writeln!(buffer,"    .alignmentoptions= {}",hex(self.atom.borrow().rawData().field_4_alignment_options as i32)).unwrap();
        // java buffer.append("          .alignment = ").append(this.getAlignment()).append("\n");
        writeln!(buffer,"          .alignment = {}",self.getAlignment()).unwrap();
        // java buffer.append("          .wraptext  = ").append(this.getWrapText()).append("\n");
        writeln!(buffer,"          .wraptext  = {}",self.getWrapText()).unwrap();
        // java buffer.append("          .valignment= ").append(Integer.toHexString(this.getVerticalAlignment())).append("\n");
        writeln!(buffer,"          .valignment= {}",hex(self.getVerticalAlignment()as i32)).unwrap();
        // java buffer.append("          .justlast  = ").append(Integer.toHexString(this.getJustifyLast())).append("\n");
        writeln!(buffer,"          .justlast  = {}",hex(self.getJustifyLast()as i32)).unwrap();
        // java buffer.append("          .rotation  = ").append(Integer.toHexString(this.getRotation())).append("\n");
        writeln!(buffer,"          .rotation  = {}",hex(self.getRotation()as i32)).unwrap();
        // java buffer.append("    .indentionoptions= ").append(Integer.toHexString(((KmoXFData)this.rawData()).field_5_indention_options)).append("\n");
        writeln!(buffer,"    .indentionoptions= {}",hex(self.atom.borrow().rawData().field_5_indention_options as i32)).unwrap();
        // java buffer.append("          .indent    = ").append(Integer.toHexString(this.getIndent())).append("\n");
        writeln!(buffer,"          .indent    = {}",hex(self.getIndent()as i32)).unwrap();
        // java buffer.append("          .shrinktoft= ").append(this.getShrinkToFit()).append("\n");
        writeln!(buffer,"          .shrinktoft= {}",self.getShrinkToFit()).unwrap();
        // java buffer.append("          .mergecells= ").append(this.getMergeCells()).append("\n");
        writeln!(buffer,"          .mergecells= {}",self.getMergeCells()).unwrap();
        // java buffer.append("          .readngordr= ").append(Integer.toHexString(this.getReadingOrder())).append("\n");
        writeln!(buffer,"          .readngordr= {}",hex(self.getReadingOrder()as i32)).unwrap();
        // java buffer.append("          .validFormat= ").append(this.isValidFormat()).append("\n");
        writeln!(buffer,"          .validFormat= {}",self.isValidFormat()).unwrap();
        // java buffer.append("          .validFont= ").append(this.isValidFont()).append("\n");
        writeln!(buffer,"          .validFont= {}",self.isValidFont()).unwrap();
        // java buffer.append("          .validAlignment= ").append(this.isValidAlignment()).append("\n");
        writeln!(buffer,"          .validAlignment= {}",self.isValidAlignment()).unwrap();
        // java buffer.append("          .validBorder= ").append(this.isValidBorder()).append("\n");
        writeln!(buffer,"          .validBorder= {}",self.isValidBorder()).unwrap();
        // java buffer.append("          .validPatern= ").append(this.isValidPattern()).append("\n");
        writeln!(buffer,"          .validPatern= {}",self.isValidPattern()).unwrap();
        // java buffer.append("          .validCelloption= ").append(this.isValidCellOptions()).append("\n");
        writeln!(buffer,"          .validCelloption= {}",self.isValidCellOptions()).unwrap();
        // java buffer.append("    .borderoptns     = ").append(Integer.toHexString(((KmoXFData)this.rawData()).field_6_border_options)).append("\n");
        writeln!(buffer,"    .borderoptns     = {}",hex(self.atom.borrow().rawData().field_6_border_options as i32)).unwrap();
        // java buffer.append("          .lftln     = ").append(Integer.toHexString(this.getBorderLeft())).append("\n");
        writeln!(buffer,"          .lftln     = {}",hex(self.getBorderLeft()as i32)).unwrap();
        // java buffer.append("          .lftCor     = ").append(Integer.toHexString(this.getLeftBorderPaletteIdx())).append("\n");
        writeln!(buffer,"          .lftCor     = {}",hex(self.getLeftBorderPaletteIdx())).unwrap();
        // java buffer.append("          .rgtln     = ").append(Integer.toHexString(this.getBorderRight())).append("\n");
        writeln!(buffer,"          .rgtln     = {}",hex(self.getBorderRight()as i32)).unwrap();
        // java buffer.append("          .rgtCor     = ").append(Integer.toHexString(this.getRightBorderPaletteIdx())).append("\n");
        writeln!(buffer,"          .rgtCor     = {}",hex(self.getRightBorderPaletteIdx())).unwrap();
        // java buffer.append("          .topln     = ").append(Integer.toHexString(this.getBorderTop())).append("\n");
        writeln!(buffer,"          .topln     = {}",hex(self.getBorderTop()as i32)).unwrap();
        // java buffer.append("          .topCor     = ").append(Integer.toHexString(this.getTopBorderPaletteIdx())).append("\n");
        writeln!(buffer,"          .topCor     = {}",hex(self.getTopBorderPaletteIdx())).unwrap();
        // java buffer.append("          .btmln     = ").append(Integer.toHexString(this.getBorderBottom())).append("\n");
        writeln!(buffer,"          .btmln     = {}",hex(self.getBorderBottom()as i32)).unwrap();
        // java buffer.append("          .btmCor     = ").append(Integer.toHexString(this.getBottomBorderPaletteIdx())).append("\n");
        writeln!(buffer,"          .btmCor     = {}",hex(self.getBottomBorderPaletteIdx())).unwrap();
        // java buffer.append("          .grbitDiag     = ").append(Integer.toHexString(this.getDiag())).append("\n");
        writeln!(buffer,"          .grbitDiag     = {}",hex(self.getDiag()as i32)).unwrap();
        // java buffer.append("          .diagln     = ").append(Integer.toHexString(this.getAdtlDiagLineStyle())).append("\n");
        writeln!(buffer,"          .diagln     = {}",hex(self.getAdtlDiagLineStyle()as i32)).unwrap();
        // java buffer.append("          .diagCor     = ").append(Integer.toHexString(this.getAdtlDiag())).append("\n");
        writeln!(buffer,"          .diagCor     = {}",hex(self.getAdtlDiag())).unwrap();
        // java buffer.append("          .leftborder= ").append(Integer.toHexString(this.getLeftBorderPaletteIdx())).append("\n");
        writeln!(buffer,"          .leftborder= {}",hex(self.getLeftBorderPaletteIdx())).unwrap();
        // java buffer.append("          .rghtborder= ").append(Integer.toHexString(this.getRightBorderPaletteIdx())).append("\n");
        writeln!(buffer,"          .rghtborder= {}",hex(self.getRightBorderPaletteIdx())).unwrap();
        // java buffer.append("          .diag      = ").append(Integer.toHexString(this.getDiag())).append("\n");
        writeln!(buffer,"          .diag      = {}",hex(self.getDiag()as i32)).unwrap();
        // java buffer.append("    .paleteoptn2     = ").append(Integer.toHexString(((KmoXFData)this.rawData()).field_8_adtl_palette_options)).append("\n");
        writeln!(buffer,"    .paleteoptn2     = {}",hex(self.atom.borrow().rawData().field_8_adtl_palette_options)).unwrap();
        // java buffer.append("          .topborder = ").append(Integer.toHexString(this.getTopBorderPaletteIdx())).append("\n");
        writeln!(buffer,"          .topborder = {}",hex(self.getTopBorderPaletteIdx())).unwrap();
        // java buffer.append("          .botmborder= ").append(Integer.toHexString(this.getBottomBorderPaletteIdx())).append("\n");
        writeln!(buffer,"          .botmborder= {}",hex(self.getBottomBorderPaletteIdx())).unwrap();
        // java buffer.append("          .adtldiag  = ").append(Integer.toHexString(this.getAdtlDiag())).append("\n");
        writeln!(buffer,"          .adtldiag  = {}",hex(self.getAdtlDiag())).unwrap();
        // java buffer.append("          .diaglnstyl= ").append(Integer.toHexString(this.getAdtlDiagLineStyle())).append("\n");
        writeln!(buffer,"          .diaglnstyl= {}",hex(self.getAdtlDiagLineStyle()as i32)).unwrap();
        // java buffer.append("          .fillpattrn= ").append(Integer.toHexString(this.getAdtlFillPattern())).append("\n");
        writeln!(buffer,"          .fillpattrn= {}",hex(self.getAdtlFillPattern()as i32)).unwrap();
        // java buffer.append("          .foreground= ").append(Integer.toHexString(this.getFillForeground())).append("\n");
        writeln!(buffer,"          .foreground= {}",hex(self.getFillForeground())).unwrap();
        // java buffer.append("          .background= ").append(Integer.toHexString(this.getFillBackground())).append("\n");
        writeln!(buffer,"          .background= {}",hex(self.getFillBackground())).unwrap();
        // java buffer.append("[/EXTENDEDFORMAT]\n"); return buffer.toString();
        buffer.push_str("[/EXTENDEDFORMAT]\n");buffer
    }
    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:1079
    // java public void copyToRecord(ExtendedFormatRecord xfRec) {
    // java ((KmoXFData)this.rawData()).copyToRecord(xfRec);
    // java }
    pub fn copyToRecord(&self, rec: &mut ExtendedFormatRecord) {
        self.atom.borrow().rawData().copyToRecord(rec);
    }
    // java public BorderFormatting getBorderFormatting() {
    pub fn getBorderFormatting(&self)->Rc<RefCell<crate::cn::wps::moss::filefmt::biff8::record::cf::BorderFormatting::BorderFormatting>>{
        use crate::cn::wps::moss::filefmt::biff8::record::cf::BorderFormatting::BorderFormatting;
        // java BorderFormatting bformat = new BorderFormatting();
        let mut bformat=BorderFormatting::BorderFormatting();
        // java bformat.setLeftBorderColor(((KmoXFData)this.rawData()).border_left_color); bformat.setBorderLeft(this.getBorderLeft());
        bformat.setLeftBorderColor(self.atom.borrow().rawData().border_left_color);bformat.setBorderLeft(self.getBorderLeft()as i32);
        // java bformat.setRightBorderColor(((KmoXFData)this.rawData()).border_right_color); bformat.setBorderRight(this.getBorderRight());
        bformat.setRightBorderColor(self.atom.borrow().rawData().border_right_color);bformat.setBorderRight(self.getBorderRight()as i32);
        // java bformat.setTopBorderColor(((KmoXFData)this.rawData()).border_top_color); bformat.setBorderTop(this.getBorderTop());
        bformat.setTopBorderColor(self.atom.borrow().rawData().border_top_color);bformat.setBorderTop(self.getBorderTop()as i32);
        // java bformat.setBottomBorderColor(((KmoXFData)this.rawData()).border_bottom_color); bformat.setBorderBottom(this.getBorderBottom());
        bformat.setBottomBorderColor(self.atom.borrow().rawData().border_bottom_color);bformat.setBorderBottom(self.getBorderBottom()as i32);
        // java bformat.setDiagonalBorderColor(((KmoXFData)this.rawData()).border_diag_color); bformat.setBorderDiagonal(this.getAdtlDiagLineStyle());
        bformat.setDiagonalBorderColor(self.atom.borrow().rawData().border_diag_color);bformat.setBorderDiagonal(self.getAdtlDiagLineStyle()as i32);
        // java short diagFlag = (short)this.getDiag();
        let diagFlag=self.getDiag()as i16;
        // java if ((diagFlag & 1) > 0) { bformat.setBackwardDiagonalOn(true); }
        if diagFlag&1>0{bformat.setBackwardDiagonalOn(true);}
        // java if ((diagFlag & 2) > 0) { bformat.setForwardDiagonalOn(true); }
        if diagFlag&2>0{bformat.setForwardDiagonalOn(true);}
        // java return bformat;
        Rc::new(RefCell::new(bformat))
    }
    // java public PatternFormatting getPatternFormating() {
    pub fn getPatternFormating(&self)->Rc<RefCell<crate::cn::wps::moss::filefmt::biff8::record::cf::PatternFormatting::PatternFormatting>>{
        use crate::cn::wps::moss::filefmt::biff8::record::cf::PatternFormatting::PatternFormatting;
        // java PatternFormatting pformat = new PatternFormatting(); pformat.setFillPattern(this.getAdtlFillPattern());
        let mut pformat=PatternFormatting::PatternFormatting();pformat.setFillPattern(self.getAdtlFillPattern()as i32);
        // java pformat.setFillBackgroundColor(((KmoXFData)this.rawData()).fill_back_color);
        pformat.setFillBackgroundColor(self.atom.borrow().rawData().fill_back_color);
        // java pformat.setFillForegroundColor(((KmoXFData)this.rawData()).fill_fore_color);
        pformat.setFillForegroundColor(self.atom.borrow().rawData().fill_fore_color);
        // java pformat.setXfGradient(((KmoXFData)this.rawData()).gradient); return pformat;
        pformat.setXfGradient(self.atom.borrow().rawData().gradient.clone());Rc::new(RefCell::new(pformat))
    }
    // rust: public void completeCopy(KmoXF source)

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:153
    // java public void completeCopy(KmoXF xf) {
    // java ((KmoXFData)this.rawData()).completeCopy((KmoXFData)xf.rawData());
    // java }
    pub fn completeCopy(&self, source: &Self) {
        let source = source.atom.borrow().rawData().clone();
        self.atom.borrow_mut().rawData_mut().completeCopy(&source);
    }
    // java public void copyFromParent(KmoXF xf) { ((KmoXFData)this.rawData()).copyFromParentXF((KmoXFData)xf.rawData()); }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:157
    // java public void copyFromParent(KmoXF xf) {
    // java ((KmoXFData)this.rawData()).copyFromParentXF((KmoXFData)xf.rawData());
    // java }
    pub fn copyFromParent(&self, source: &Self) {
        let source = source.atom.borrow().rawData().clone();
        self.atom
            .borrow_mut()
            .rawData_mut()
            .copyFromParentXF(&source);
    }
    // java public static KmoXF createXF(KmoXF xf)

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:128
    // java public static KmoXF createXF(KmoXF xf) {
    // java if (xf == null) {
    // java return new KmoCellXF();
    // java } else {
    // java KmoXF ret;
    // java if (xf instanceof KmoCellXF) {
    // java ret = new KmoCellXF();
    // java } else {
    // java ret = new KmoStyleXF();
    // java }
    // java
    // java ret.completeCopy(xf);
    // java return ret;
    // java }
    // java }
    pub fn createXF(source: Option<&Self>) -> Self {
        let result = Self::fromData(KmoXFData::new(), source.is_some_and(|s| s.isStyle));
        if let Some(source) = source {
            result.completeCopy(source);
        }
        result
    }
    // java public static ExtendedFormatRecord createDefaultExtendedFormat(int id) {
    // java ExtendedFormatRecord retval = new ExtendedFormatRecord();
    // java switch (id) {
    // java case 0:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)0);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 1:
    // java retval.setFontIndex((short)1);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 2:
    // java retval.setFontIndex((short)1);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 3:
    // java retval.setFontIndex((short)2);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 4:
    // java retval.setFontIndex((short)2);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 5:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 6:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 7:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 8:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 9:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 10:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 11:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 12:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 13:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 14:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 15:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)1);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)0);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 16:
    // java retval.setFontIndex((short)1);
    // java retval.setFormatIndex((short)43);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-2048);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 17:
    // java retval.setFontIndex((short)1);
    // java retval.setFormatIndex((short)41);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-2048);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 18:
    // java retval.setFontIndex((short)1);
    // java retval.setFormatIndex((short)44);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-2048);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 19:
    // java retval.setFontIndex((short)1);
    // java retval.setFormatIndex((short)42);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-2048);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 20:
    // java retval.setFontIndex((short)1);
    // java retval.setFormatIndex((short)9);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-2048);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 21:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)0);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)0);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 22:
    // java retval.setFontIndex((short)6);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)1);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)23552);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 23:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)49);
    // java retval.setCellOptions((short)1);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)23552);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 24:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)8);
    // java retval.setCellOptions((short)1);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)23552);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 25:
    // java retval.setFontIndex((short)6);
    // java retval.setFormatIndex((short)8);
    // java retval.setCellOptions((short)1);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)23552);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java }
    // java return retval;
    // java }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXF.java:726
    // java public static ExtendedFormatRecord createDefaultExtendedFormat(int id) {
    // java ExtendedFormatRecord retval = new ExtendedFormatRecord();
    // java switch (id) {
    // java case 0:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)0);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 1:
    // java retval.setFontIndex((short)1);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 2:
    // java retval.setFontIndex((short)1);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 3:
    // java retval.setFontIndex((short)2);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 4:
    // java retval.setFontIndex((short)2);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 5:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 6:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 7:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 8:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 9:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 10:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 11:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 12:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 13:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 14:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-3072);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 15:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)1);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)0);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 16:
    // java retval.setFontIndex((short)1);
    // java retval.setFormatIndex((short)43);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-2048);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 17:
    // java retval.setFontIndex((short)1);
    // java retval.setFormatIndex((short)41);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-2048);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 18:
    // java retval.setFontIndex((short)1);
    // java retval.setFormatIndex((short)44);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-2048);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 19:
    // java retval.setFontIndex((short)1);
    // java retval.setFormatIndex((short)42);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-2048);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 20:
    // java retval.setFontIndex((short)1);
    // java retval.setFormatIndex((short)9);
    // java retval.setCellOptions((short)-11);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)-2048);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 21:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)0);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)0);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 22:
    // java retval.setFontIndex((short)6);
    // java retval.setFormatIndex((short)0);
    // java retval.setCellOptions((short)1);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)23552);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 23:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)49);
    // java retval.setCellOptions((short)1);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)23552);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 24:
    // java retval.setFontIndex((short)0);
    // java retval.setFormatIndex((short)8);
    // java retval.setCellOptions((short)1);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)23552);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java break;
    // java case 25:
    // java retval.setFontIndex((short)6);
    // java retval.setFormatIndex((short)8);
    // java retval.setCellOptions((short)1);
    // java retval.setAlignmentOptions((short)16);
    // java retval.setIndentionOptions((short)23552);
    // java retval.setBorderOptions((short)0);
    // java retval.setPaletteOptions((short)0);
    // java retval.setAdtlPaletteOptions(0);
    // java retval.setFillPaletteOptions((short)8384);
    // java }
    // java
    // java return retval;
    // java }
    pub fn createDefaultExtendedFormat(id: i32) -> ExtendedFormatRecord {
        let mut retval = ExtendedFormatRecord::ExtendedFormatRecord();
        match id {
            0 => {
                retval.setFontIndex(0);
                retval.setFormatIndex(0);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(0);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            1 => {
                retval.setFontIndex(1);
                retval.setFormatIndex(0);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-3072);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            2 => {
                retval.setFontIndex(1);
                retval.setFormatIndex(0);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-3072);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            3 => {
                retval.setFontIndex(2);
                retval.setFormatIndex(0);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-3072);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            4 => {
                retval.setFontIndex(2);
                retval.setFormatIndex(0);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-3072);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            5 => {
                retval.setFontIndex(0);
                retval.setFormatIndex(0);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-3072);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            6 => {
                retval.setFontIndex(0);
                retval.setFormatIndex(0);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-3072);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            7 => {
                retval.setFontIndex(0);
                retval.setFormatIndex(0);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-3072);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            8 => {
                retval.setFontIndex(0);
                retval.setFormatIndex(0);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-3072);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            9 => {
                retval.setFontIndex(0);
                retval.setFormatIndex(0);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-3072);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            10 => {
                retval.setFontIndex(0);
                retval.setFormatIndex(0);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-3072);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            11 => {
                retval.setFontIndex(0);
                retval.setFormatIndex(0);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-3072);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            12 => {
                retval.setFontIndex(0);
                retval.setFormatIndex(0);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-3072);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            13 => {
                retval.setFontIndex(0);
                retval.setFormatIndex(0);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-3072);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            14 => {
                retval.setFontIndex(0);
                retval.setFormatIndex(0);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-3072);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            15 => {
                retval.setFontIndex(0);
                retval.setFormatIndex(0);
                retval.setCellOptions(1);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(0);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            16 => {
                retval.setFontIndex(1);
                retval.setFormatIndex(43);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-2048);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            17 => {
                retval.setFontIndex(1);
                retval.setFormatIndex(41);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-2048);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            18 => {
                retval.setFontIndex(1);
                retval.setFormatIndex(44);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-2048);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            19 => {
                retval.setFontIndex(1);
                retval.setFormatIndex(42);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-2048);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            20 => {
                retval.setFontIndex(1);
                retval.setFormatIndex(9);
                retval.setCellOptions(-11);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(-2048);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            21 => {
                retval.setFontIndex(0);
                retval.setFormatIndex(0);
                retval.setCellOptions(0);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(0);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            22 => {
                retval.setFontIndex(6);
                retval.setFormatIndex(0);
                retval.setCellOptions(1);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(23552);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            23 => {
                retval.setFontIndex(0);
                retval.setFormatIndex(49);
                retval.setCellOptions(1);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(23552);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            24 => {
                retval.setFontIndex(0);
                retval.setFormatIndex(8);
                retval.setCellOptions(1);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(23552);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            25 => {
                retval.setFontIndex(6);
                retval.setFormatIndex(8);
                retval.setCellOptions(1);
                retval.setAlignmentOptions(16);
                retval.setIndentionOptions(23552);
                retval.setBorderOptions(0);
                retval.setPaletteOptions(0);
                retval.setAdtlPaletteOptions(0);
                retval.setFillPaletteOptions(8384);
            }
            _ => {}
        }
        retval
    }
}

impl PartialEq for KmoXF {
    fn eq(&self, xf: &Self) -> bool {
        std::ptr::eq(self, xf) || *xf.atom.borrow().rawData() == *self.atom.borrow().rawData()
    }
}
impl KmoXF{
// java public static KmoXF shareEmptyCellXF() {
// java return EMPTY_CELL_XF;
// java }
pub fn shareEmptyCellXF()->Rc<Self> {
static EMPTY:std::sync::LazyLock<Rc<KmoXF>>=std::sync::LazyLock::new(||Rc::new(KmoXF::createXF(None)));EMPTY.clone()
}
}
