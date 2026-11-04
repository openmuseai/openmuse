// Java: cn.wps.moffice.writer.data.value.MathProperty
//
// ★M-STAGE1 P2 —— 从 auto-generated 空壳（12 行 / 2 处 `todo!()`、无字段）**做实**。
//   它是 `SettingsImporter.correctMathPr()` 的必经依赖（`new MathProperty(...)`）。
//
// Java 原文：16 个 `public static final int` 常量 + 1 个字段 + 构造器 + 1 个 getter。
// ★这 16 个常量与 `SettingsImporter` [S1] 批次写入的属性位**一一对应**
//   （598–613），是 MathPr 段的权威编号表。

// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;

// java public class MathProperty
#[derive(Default, Clone)]
pub struct MathProperty {
    // java private PropertySet mMathPropSet = null;
    mMathPropSet: Option<PropertySet>,
}

impl MathProperty {
    // java public static final int document_mathFont = 598;
    pub const document_mathFont: i32 = 598;
    // java public static final int document_brkBin = 599;
    pub const document_brkBin: i32 = 599;
    // java public static final int document_brkBinSub = 600;
    pub const document_brkBinSub: i32 = 600;
    // java public static final int document_smallFrac = 601;
    pub const document_smallFrac: i32 = 601;
    // java public static final int document_dispDef = 602;
    pub const document_dispDef: i32 = 602;
    // java public static final int document_lMargin = 603;
    pub const document_lMargin: i32 = 603;
    // java public static final int document_rMargin = 604;
    pub const document_rMargin: i32 = 604;
    // java public static final int document_defJc = 605;
    pub const document_defJc: i32 = 605;
    // java public static final int document_preSp = 606;
    pub const document_preSp: i32 = 606;
    // java public static final int document_postSp = 607;
    pub const document_postSp: i32 = 607;
    // java public static final int document_interSp = 608;
    pub const document_interSp: i32 = 608;
    // java public static final int document_intraSp = 609;
    pub const document_intraSp: i32 = 609;
    // java public static final int document_wrapIndent = 610;
    pub const document_wrapIndent: i32 = 610;
    // java public static final int document_wrapRight = 611;
    pub const document_wrapRight: i32 = 611;
    // java public static final int document_intLim = 612;
    pub const document_intLim: i32 = 612;
    // java public static final int document_naryLim = 613;
    pub const document_naryLim: i32 = 613;

    // java public MathProperty(PropertySet propSet)
    pub fn MathProperty(propSet: PropertySet) -> Self {
        // java Assert.assertNotNull("propSet should not be null", propSet);
        // rust: `PropertySet` 按值传入 ⇒ 天然非空，断言语义由类型系统保证。
        // java this.mMathPropSet = propSet;
        MathProperty { mMathPropSet: Some(propSet) }
    }

    // java public PropertySet getMathPropertySet()
    pub fn getMathPropertySet(&self) -> Option<PropertySet> {
        // java return this.mMathPropSet;
        return self.mMathPropSet.clone();
    }
}
