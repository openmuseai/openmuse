use super::KmoFont::KmoFont;
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
// java class KmoXFData extends KmoAtomData
#[derive(Clone)]
pub struct KmoXFData {
    pub base: KmoAtomData,
    // java protected short field_1_font_index;
    pub(crate) field_1_font_index: i16,
    // java protected short field_2_format_index;
    pub(crate) field_2_format_index: i16,
    // rust: protected boolean hasInit;
    pub(crate) hasInit: bool,
    // java protected short field_3_cell_options;
    pub(crate) field_3_cell_options: i16,
    // java protected boolean isLocked;
    pub(crate) isLocked: bool,
    // java protected boolean isHidden;
    pub(crate) isHidden: bool,
    // java protected short xfType;
    pub(crate) xfType: i16,
    // java protected boolean _123prefix;
    pub(crate) _123prefix: bool,
    // java protected short parentIndex;
    pub(crate) parentIndex: i16,
    // java protected short field_4_alignment_options;
    pub(crate) field_4_alignment_options: i16,
    // java protected short alignment;
    pub(crate) alignment: i16,
    // java protected boolean isWrapText;
    pub(crate) isWrapText: bool,
    // java protected short verticalAlignment;
    pub(crate) verticalAlignment: i16,
    // java protected short justifyLast;
    pub(crate) justifyLast: i16,
    // java protected short rotation;
    pub(crate) rotation: i16,
    // java protected short field_5_indention_options;
    pub(crate) field_5_indention_options: i16,
    // java protected short indent;
    pub(crate) indent: i16,
    // java protected boolean isShrink2Fit;
    pub(crate) isShrink2Fit: bool,
    // java protected boolean isMergeCells;
    pub(crate) isMergeCells: bool,
    // java protected short readingOrder;
    pub(crate) readingOrder: i16,
    // java protected boolean isAttrFormat;
    pub(crate) isAttrFormat: bool,
    // java protected boolean isAttrFont;
    pub(crate) isAttrFont: bool,
    // java protected boolean isAttrAlignment;
    pub(crate) isAttrAlignment: bool,
    // java protected boolean isAttrBorder;
    pub(crate) isAttrBorder: bool,
    // java protected boolean isAttrPattern;
    pub(crate) isAttrPattern: bool,
    // java protected boolean isAttrOptions;
    pub(crate) isAttrOptions: bool,
    // java protected short field_6_border_options;
    pub(crate) field_6_border_options: i16,
    // java protected short borderLeft;
    pub(crate) borderLeft: i16,
    // java protected short borderRight;
    pub(crate) borderRight: i16,
    // java protected short borderTop;
    pub(crate) borderTop: i16,
    // java protected short borderBottom;
    pub(crate) borderBottom: i16,
    // java protected int border_left_color;
    pub(crate) border_left_color: i32,
    // java protected int border_right_color;
    pub(crate) border_right_color: i32,
    // java protected byte border_diag_flag;
    pub(crate) border_diag_flag: i8,
    // java protected int field_8_adtl_palette_options;
    pub(crate) field_8_adtl_palette_options: i32,
    // java protected short adtlDiagLineStyle;
    pub(crate) adtlDiagLineStyle: i16,
    // java protected short adtlFillPattern;
    pub(crate) adtlFillPattern: i16,
    // java protected int border_top_color;
    pub(crate) border_top_color: i32,
    // java protected int border_bottom_color;
    pub(crate) border_bottom_color: i32,
    // java protected int border_diag_color;
    pub(crate) border_diag_color: i32,
    // java protected int fill_fore_color;
    pub(crate) fill_fore_color: i32,
    // java protected int fill_back_color;
    pub(crate) fill_back_color: i32,
    // java protected XFGradient gradient;
    pub(crate) gradient: Option<GradientRef>,
    // java protected KmoFont font;
    pub(crate) font: Option<Rc<KmoFont>>,
    // java protected String numfmt;
    pub(crate) numfmt: Option<String>,
}
impl KmoXFData {
    pub fn new() -> Self {
        Self {
            base: KmoAtomData::new(),
            field_1_font_index: 0,
            field_2_format_index: 0,
            hasInit: false,
            field_3_cell_options: 0,
            isLocked: false,
            isHidden: false,
            xfType: 0,
            _123prefix: false,
            parentIndex: 0,
            field_4_alignment_options: 0,
            alignment: 0,
            isWrapText: false,
            verticalAlignment: 0,
            justifyLast: 0,
            rotation: 0,
            field_5_indention_options: 0,
            indent: 0,
            isShrink2Fit: false,
            isMergeCells: false,
            readingOrder: 0,
            isAttrFormat: false,
            isAttrFont: false,
            isAttrAlignment: false,
            isAttrBorder: false,
            isAttrPattern: false,
            isAttrOptions: false,
            field_6_border_options: 0,
            borderLeft: 0,
            borderRight: 0,
            borderTop: 0,
            borderBottom: 0,
            border_left_color: 0,
            border_right_color: 0,
            border_diag_flag: 0,
            field_8_adtl_palette_options: 0,
            adtlDiagLineStyle: 0,
            adtlFillPattern: 0,
            border_top_color: 0,
            border_bottom_color: 0,
            border_diag_color: 0,
            fill_fore_color: 0,
            fill_back_color: 0,
            gradient: None,
            font: None,
            numfmt: None,
        }
    }
    // java protected void completeCopy(KmoXFData xf) {
    // java this.field_1_font_index = xf.field_1_font_index;
    // java this.field_2_format_index = xf.field_2_format_index;
    // java this.field_3_cell_options = xf.field_3_cell_options;
    // java this.field_4_alignment_options = xf.field_4_alignment_options;
    // java this.field_5_indention_options = xf.field_5_indention_options;
    // java this.field_6_border_options = xf.field_6_border_options;
    // java this.border_left_color = xf.border_left_color;
    // java this.border_right_color = xf.border_right_color;
    // java this.border_diag_flag = xf.border_diag_flag;
    // java this.field_8_adtl_palette_options = xf.field_8_adtl_palette_options;
    // java this.border_top_color = xf.border_top_color;
    // java this.border_bottom_color = xf.border_bottom_color;
    // java this.border_diag_color = xf.border_diag_color;
    // java this.fill_back_color = xf.fill_back_color;
    // java this.fill_fore_color = xf.fill_fore_color;
    // java if (xf.font != null) {
    // java this.font = KmoFont.createFont(xf.font);
    // java } else {
    // java this.font = null;
    // java }
    // java this.numfmt = xf.numfmt;
    // java this.copyOption(xf);
    // java this.copyGradient(xf);
    // java }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFData.java:206
    // java protected void completeCopy(KmoXFData xf) {
    // java this.field_1_font_index = xf.field_1_font_index;
    // java this.field_2_format_index = xf.field_2_format_index;
    // java this.field_3_cell_options = xf.field_3_cell_options;
    // java this.field_4_alignment_options = xf.field_4_alignment_options;
    // java this.field_5_indention_options = xf.field_5_indention_options;
    // java this.field_6_border_options = xf.field_6_border_options;
    // java this.border_left_color = xf.border_left_color;
    // java this.border_right_color = xf.border_right_color;
    // java this.border_diag_flag = xf.border_diag_flag;
    // java this.field_8_adtl_palette_options = xf.field_8_adtl_palette_options;
    // java this.border_top_color = xf.border_top_color;
    // java this.border_bottom_color = xf.border_bottom_color;
    // java this.border_diag_color = xf.border_diag_color;
    // java this.fill_back_color = xf.fill_back_color;
    // java this.fill_fore_color = xf.fill_fore_color;
    // java if (xf.font != null) {
    // java this.font = KmoFont.createFont(xf.font);
    // java } else {
    // java this.font = null;
    // java }
    // java
    // java this.numfmt = xf.numfmt;
    // java this.copyOption(xf);
    // java this.copyGradient(xf);
    // java }
    pub fn completeCopy(&mut self, source: &Self) {
        let base = self.base.clone();
        *self = source.clone();
        self.base = base;
        self.font = source
            .font
            .as_ref()
            .map(|font| Rc::new(KmoFont::createFont(font)));
        self.copyGradient(source);
    }
    // rust: protected void copyGradient(KmoXFData xf)

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFData.java:161
    // java private void copyGradient(KmoXFData xf) {
    // java if (xf.gradient == null) {
    // java this.gradient = null;
    // java } else {
    // java this.gradient = new XFGradient();
    // java this.gradient.isLinear = xf.gradient.isLinear;
    // java this.gradient.degree = xf.gradient.degree;
    // java this.gradient.left = xf.gradient.left;
    // java this.gradient.right = xf.gradient.right;
    // java this.gradient.top = xf.gradient.top;
    // java this.gradient.bottom = xf.gradient.bottom;
    // java this.gradient.colorStops.addAll(xf.gradient.colorStops);
    // java }
    // java }
    pub fn copyGradient(&mut self, source: &Self) {
        // rust: if (xf.gradient != null) { this.gradient = new XFGradient();
        // java this.gradient.isLinear = xf.gradient.isLinear; this.gradient.degree = xf.gradient.degree;
        // java this.gradient.left = xf.gradient.left; this.gradient.right = xf.gradient.right;
        // java this.gradient.top = xf.gradient.top; this.gradient.bottom = xf.gradient.bottom;
        // rust: this.gradient.colorStops.addAll(xf.gradient.colorStops); } else { this.gradient = null; }
        self.gradient = source
            .gradient
            .as_ref()
            .map(|g| Rc::new(RefCell::new(g.borrow().clone())));
    }
    // java public boolean equalsInMeaning(KmoXFData data)

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFData.java:87
    // java public boolean equalsInMeaning(KmoXFData data) {
    // java if (KmoXF._hidden.clear(KmoXF._locked.clear(data.field_3_cell_options)) != KmoXF._hidden.clear(KmoXF._locked.clear(this.field_3_cell_options))) {
    // java return false;
    // java } else if (KmoXF._merge_cells.isSet(data.field_5_indention_options) != KmoXF._merge_cells.isSet(this.field_5_indention_options)) {
    // java return false;
    // java } else if (KmoXF._shrink_to_fit.isSet(data.field_5_indention_options) != KmoXF._shrink_to_fit.isSet(this.field_5_indention_options)) {
    // java return false;
    // java } else if (KmoXF._reading_order.isSet(data.field_5_indention_options) != KmoXF._reading_order.isSet(this.field_5_indention_options)) {
    // java return false;
    // java } else if (data.adtlFillPattern != this.adtlFillPattern) {
    // java return false;
    // java } else {
    // java if (this.adtlFillPattern == 32) {
    // java if (!XFGradient.equals(data.gradient, this.gradient)) {
    // java return false;
    // java }
    // java } else if (this.adtlFillPattern != 0 && (data.fill_fore_color != this.fill_fore_color || data.fill_back_color != this.fill_back_color)) {
    // java return false;
    // java }
    // java
    // java if (data.field_4_alignment_options == this.field_4_alignment_options && data.field_6_border_options == this.field_6_border_options && data.border_left_color == this.border_left_color && data.border_right_color == this.border_right_color && data.border_diag_flag == this.border_diag_flag && data.field_8_adtl_palette_options == this.field_8_adtl_palette_options && data.border_top_color == this.border_top_color && data.border_bottom_color == this.border_bottom_color && data.border_diag_color == this.border_diag_color) {
    // java return true;
    // java } else {
    // java return false;
    // java }
    // java }
    // java }
    pub fn equalsInMeaning(&self, data: &Self) -> bool {
        if (data.field_3_cell_options & !3) != (self.field_3_cell_options & !3) {
            return false;
        }
        for bit in [32, 16, 192] {
            if (data.field_5_indention_options & bit != 0)
                != (self.field_5_indention_options & bit != 0)
            {
                return false;
            }
        }
        self.equalFill(data)
            && self.field_4_alignment_options == data.field_4_alignment_options
            && self.field_6_border_options == data.field_6_border_options
            && self.border_left_color == data.border_left_color
            && self.border_right_color == data.border_right_color
            && self.border_diag_flag == data.border_diag_flag
            && self.field_8_adtl_palette_options == data.field_8_adtl_palette_options
            && self.border_top_color == data.border_top_color
            && self.border_bottom_color == data.border_bottom_color
            && self.border_diag_color == data.border_diag_color
    }
    // rust: Shared literal fill checks from Java equals and equalsInMeaning.
    fn equalFill(&self, data: &Self) -> bool {
        if data.adtlFillPattern != self.adtlFillPattern {
            return false;
        }
        if self.adtlFillPattern == 32 {
            match (&data.gradient, &self.gradient) {
                (None, None) => true,
                (Some(a), Some(b)) => *a.borrow() == *b.borrow(),
                _ => false,
            }
        } else {
            self.adtlFillPattern == 0
                || (data.fill_fore_color == self.fill_fore_color
                    && data.fill_back_color == self.fill_back_color)
        }
    }
    // java protected void copyFromRecord(ExtendedFormatRecord xfRec) {
    // java this.field_1_font_index = xfRec.getFontIndex();
    // java short formatIndex = xfRec.getFormatIndex();
    // java if (formatIndex < 0) {
    // java formatIndex = 0;
    // java }
    // java this.field_2_format_index = formatIndex;
    // java this.field_3_cell_options = xfRec.getCellOptions();
    // java this.field_4_alignment_options = xfRec.getAlignmentOptions();
    // java this.field_5_indention_options = xfRec.getIndentionOptions();
    // java this.field_6_border_options = xfRec.getBorderOptions();
    // java this.border_left_color = xfRec.getLeftBorderPaletteIdx();
    // java this.border_right_color = xfRec.getRightBorderPaletteIdx();
    // java this.border_diag_flag = xfRec.getDiag();
    // java this.field_8_adtl_palette_options = xfRec.getAdtlPaletteOptions();
    // java this.border_top_color = xfRec.getTopBorderPaletteIdx();
    // java this.border_bottom_color = xfRec.getBottomBorderPaletteIdx();
    // java this.border_diag_color = xfRec.getAdtlDiag();
    // java this.fill_back_color = xfRec.getFillBackground();
    // java this.fill_fore_color = xfRec.getFillForeground();
    // java this.gradient = xfRec.getXFGradient();
    // java }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFData.java:115
    // java protected void copyFromRecord(ExtendedFormatRecord xfRec) {
    // java this.field_1_font_index = xfRec.getFontIndex();
    // java short formatIndex = xfRec.getFormatIndex();
    // java if (formatIndex < 0) {
    // java formatIndex = 0;
    // java }
    // java
    // java this.field_2_format_index = formatIndex;
    // java this.field_3_cell_options = xfRec.getCellOptions();
    // java this.field_4_alignment_options = xfRec.getAlignmentOptions();
    // java this.field_5_indention_options = xfRec.getIndentionOptions();
    // java this.field_6_border_options = xfRec.getBorderOptions();
    // java this.border_left_color = xfRec.getLeftBorderPaletteIdx();
    // java this.border_right_color = xfRec.getRightBorderPaletteIdx();
    // java this.border_diag_flag = xfRec.getDiag();
    // java this.field_8_adtl_palette_options = xfRec.getAdtlPaletteOptions();
    // java this.border_top_color = xfRec.getTopBorderPaletteIdx();
    // java this.border_bottom_color = xfRec.getBottomBorderPaletteIdx();
    // java this.border_diag_color = xfRec.getAdtlDiag();
    // java this.fill_back_color = xfRec.getFillBackground();
    // java this.fill_fore_color = xfRec.getFillForeground();
    // java this.gradient = xfRec.getXFGradient();
    // java }
    pub fn copyFromRecord(&mut self, xfRec: &ExtendedFormatRecord) {
        self.field_1_font_index = xfRec.getFontIndex();
        self.field_3_cell_options = xfRec.getCellOptions();
        self.field_4_alignment_options = xfRec.getAlignmentOptions();
        self.field_5_indention_options = xfRec.getIndentionOptions();
        self.field_6_border_options = xfRec.getBorderOptions();
        self.border_left_color = xfRec.getLeftBorderPaletteIdx();
        self.border_right_color = xfRec.getRightBorderPaletteIdx();
        self.border_diag_flag = xfRec.getDiag();
        self.field_8_adtl_palette_options = xfRec.getAdtlPaletteOptions();
        self.border_top_color = xfRec.getTopBorderPaletteIdx();
        self.border_bottom_color = xfRec.getBottomBorderPaletteIdx();
        self.border_diag_color = xfRec.getAdtlDiag();
        self.fill_back_color = xfRec.getFillBackground();
        self.fill_fore_color = xfRec.getFillForeground();
        self.gradient = xfRec.getXFGradient();
        self.field_2_format_index = xfRec.getFormatIndex().max(0);
    }
    // java protected void copyFromParentXF(KmoXFData xf) {
    // java this.field_1_font_index = xf.field_1_font_index;
    // java this.field_2_format_index = xf.field_2_format_index;
    // java this.field_3_cell_options = xf.field_3_cell_options;
    // java this.field_4_alignment_options = xf.field_4_alignment_options;
    // java this.field_5_indention_options = (short)(xf.field_5_indention_options & 255);
    // java this.field_6_border_options = xf.field_6_border_options;
    // java this.border_left_color = xf.border_left_color;
    // java this.border_right_color = xf.border_right_color;
    // java this.border_diag_flag = xf.border_diag_flag;
    // java this.field_8_adtl_palette_options = xf.field_8_adtl_palette_options;
    // java this.border_top_color = xf.border_top_color;
    // java this.border_bottom_color = xf.border_bottom_color;
    // java this.border_diag_color = xf.border_diag_color;
    // java this.fill_back_color = xf.fill_back_color;
    // java this.fill_fore_color = xf.fill_fore_color;
    // java this.font = xf.font;
    // java this.numfmt = xf.numfmt;
    // java this.copyOption(xf);
    // java this.copyGradient(xf);
    // java }
    pub fn copyFromParentXF(&mut self, source: &Self) {
        let base = self.base.clone();
        *self = source.clone();
        self.base = base;
        self.field_5_indention_options = source.field_5_indention_options & 255;
        self.copyGradient(source);
    }
    // java protected void copyToRecord(ExtendedFormatRecord xfRec) {
    // java xfRec.setFontIndex(this.field_1_font_index);
    // java xfRec.setFormatIndex(this.field_2_format_index);
    // java xfRec.setCellOptions(this.field_3_cell_options);
    // java xfRec.setAlignmentOptions(this.field_4_alignment_options);
    // java xfRec.setIndentionOptions(this.field_5_indention_options);
    // java xfRec.setBorderOptions(this.field_6_border_options);
    // java xfRec.setLeftBorderPaletteIdx(this.border_left_color);
    // java xfRec.setRightBorderPaletteIdx(this.border_right_color);
    // java xfRec.setDiag(this.border_diag_flag);
    // java xfRec.setAdtlPaletteOptions(this.field_8_adtl_palette_options);
    // java xfRec.setTopBorderPaletteIdx(this.border_top_color);
    // java xfRec.setBottomBorderPaletteIdx(this.border_bottom_color);
    // java xfRec.setAdtlDiag(this.border_diag_color);
    // java xfRec.setFillBackground(this.fill_back_color);
    // java xfRec.setFillForeground(this.fill_fore_color);
    // java if (this.adtlFillPattern == 32) {
    // java xfRec.setXFGradient(this.gradient);
    // java }
    // java }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoXFData.java:233
    // java protected void copyToRecord(ExtendedFormatRecord xfRec) {
    // java xfRec.setFontIndex(this.field_1_font_index);
    // java xfRec.setFormatIndex(this.field_2_format_index);
    // java xfRec.setCellOptions(this.field_3_cell_options);
    // java xfRec.setAlignmentOptions(this.field_4_alignment_options);
    // java xfRec.setIndentionOptions(this.field_5_indention_options);
    // java xfRec.setBorderOptions(this.field_6_border_options);
    // java xfRec.setLeftBorderPaletteIdx(this.border_left_color);
    // java xfRec.setRightBorderPaletteIdx(this.border_right_color);
    // java xfRec.setDiag(this.border_diag_flag);
    // java xfRec.setAdtlPaletteOptions(this.field_8_adtl_palette_options);
    // java xfRec.setTopBorderPaletteIdx(this.border_top_color);
    // java xfRec.setBottomBorderPaletteIdx(this.border_bottom_color);
    // java xfRec.setAdtlDiag(this.border_diag_color);
    // java xfRec.setFillBackground(this.fill_back_color);
    // java xfRec.setFillForeground(this.fill_fore_color);
    // java if (this.adtlFillPattern == 32) {
    // java xfRec.setXFGradient(this.gradient);
    // java }
    // java
    // java }
    pub fn copyToRecord(&self, rec: &mut ExtendedFormatRecord) {
        rec.setFontIndex(self.field_1_font_index.clone());
        rec.setFormatIndex(self.field_2_format_index.clone());
        rec.setCellOptions(self.field_3_cell_options.clone());
        rec.setAlignmentOptions(self.field_4_alignment_options.clone());
        rec.setIndentionOptions(self.field_5_indention_options.clone());
        rec.setBorderOptions(self.field_6_border_options.clone());
        rec.setLeftBorderPaletteIdx(self.border_left_color.clone());
        rec.setRightBorderPaletteIdx(self.border_right_color.clone());
        rec.setDiag(self.border_diag_flag.clone());
        rec.setAdtlPaletteOptions(self.field_8_adtl_palette_options.clone());
        rec.setTopBorderPaletteIdx(self.border_top_color.clone());
        rec.setBottomBorderPaletteIdx(self.border_bottom_color.clone());
        rec.setAdtlDiag(self.border_diag_color.clone());
        rec.setFillBackground(self.fill_back_color.clone());
        rec.setFillForeground(self.fill_fore_color.clone());
        if self.adtlFillPattern == 32 {
            rec.setXFGradient(self.gradient.clone());
        }
    }
}
impl KmoAtomDataSub for KmoXFData {
    fn as_base(&self) -> &KmoAtomData {
        &self.base
    }
    fn as_base_mut(&mut self) -> &mut KmoAtomData {
        &mut self.base
    }
    fn copy(&self) -> Self {
        let mut data = Self::new();
        data.completeCopy(self);
        data
    }
}

impl PartialEq for KmoXFData {
    fn eq(&self, data: &Self) -> bool {
        self.equalFill(data)
            && self.field_4_alignment_options == data.field_4_alignment_options
            && self.field_6_border_options == data.field_6_border_options
            && self.border_left_color == data.border_left_color
            && self.border_right_color == data.border_right_color
            && self.border_diag_flag == data.border_diag_flag
            && self.field_8_adtl_palette_options == data.field_8_adtl_palette_options
            && self.border_top_color == data.border_top_color
            && self.border_bottom_color == data.border_bottom_color
            && self.border_diag_color == data.border_diag_color
            && self.field_1_font_index == data.field_1_font_index
            && self.field_2_format_index == data.field_2_format_index
            && self.field_3_cell_options == data.field_3_cell_options
            && self.field_5_indention_options == data.field_5_indention_options
    }
}
