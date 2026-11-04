// Java: cn.wps.moffice.writer.data.theme.format.ThemeFormatScheme
//
// ★M-STAGE1 P2-9 —— 由**破损桩**（签名引用未定义类型 `FillBase` / `LineProperty` /
//   `EffectStyle`）做实。Java 原文 79 行：4 个 List 字段 + 4 组 add/get/size + name。

// Java: import cn.wps.moffice.drawing.graphics.FillBase
use drawing::cn::wps::moffice::drawing::graphics::FillBase::FillBase;
// Java: import cn.wps.moffice.drawing.line.LineProperty
use drawing::cn::wps::moffice::drawing::line::LineProperty::LineProperty;
// Java: import java.util.ArrayList
use crate::cn::wps::moffice::writer::data::theme::format::EffectStyle::EffectStyle;

// java public static final int STYLELST_MIN_SIZE = 3;
pub const STYLELST_MIN_SIZE: i32 = 3;

// java public class ThemeFormatScheme
#[derive(Default)]
pub struct ThemeFormatScheme {
    // java private String mName;
    mName: Option<String>,
    // java private List<LineProperty> mLnStyleLst = new ArrayList(3);
    mLnStyleLst: Vec<LineProperty>,
    // java private List<FillBase> mFillStyleLst = new ArrayList(3);
    mFillStyleLst: Vec<FillBase>,
    // java private List<EffectStyle> mEffectStyleLst = new ArrayList(3);
    mEffectStyleLst: Vec<EffectStyle>,
    // java private List<FillBase> mBgFillStyleLst = new ArrayList(3);
    mBgFillStyleLst: Vec<FillBase>,
}

// java public class ThemeFormatScheme
impl ThemeFormatScheme {
    // rust: Java 无显式构造器（走默认构造 + 字段初始化器 `new ArrayList(3)`）⇒
    //   `new()` 为 Rust 侧便利方法。Java 的 `new ArrayList(3)` 只是容量提示，
    //   Rust `Vec::new()` 观测等价（`size()` 均为 0）。
    pub fn new() -> Self {
        Self::default()
    }

    // java public String getName()
    pub fn getName(&self) -> Option<String> {
        // java return this.mName;
        return self.mName.clone();
    }

    // java public void setName(String name)
    pub fn setName(&mut self, name: Option<String>) {
        // java this.mName = name;
        self.mName = name;
    }

    // java public int addShapeFillStyle(FillBase fillBase)
    pub fn addShapeFillStyle(&mut self, fillBase: FillBase) -> i32 {
        // java int size = this.mFillStyleLst.size();
        let size: i32 = self.mFillStyleLst.len() as i32;
        // java this.mFillStyleLst.add(fillBase);
        self.mFillStyleLst.push(fillBase);
        // java return size;
        return size;
    }

    // java public FillBase getShapeFillStyle(int index)
    pub fn getShapeFillStyle(&mut self, index: i32) -> Option<&mut FillBase> {
        // java return null != this.mFillStyleLst && index >= 0 && index < this.mFillStyleLst.size() ? (FillBase)this.mFillStyleLst.get(index) : null;
        return if index >= 0 && (index as usize) < self.mFillStyleLst.len() {
            self.mFillStyleLst.get_mut(index as usize)
        } else {
            None
        };
    }

    // java public int getShapeFillStyleSize()
    pub fn getShapeFillStyleSize(&self) -> i32 {
        // java return this.mFillStyleLst != null ? this.mFillStyleLst.size() : 0;
        return self.mFillStyleLst.len() as i32;
    }

    // java public int addBgFillStyle(FillBase fillBase)
    pub fn addBgFillStyle(&mut self, fillBase: FillBase) -> i32 {
        // java int size = this.mBgFillStyleLst.size();
        let size: i32 = self.mBgFillStyleLst.len() as i32;
        // java this.mBgFillStyleLst.add(fillBase);
        self.mBgFillStyleLst.push(fillBase);
        // java return size;
        return size;
    }

    // java public FillBase getBgFillStyle(int index)
    pub fn getBgFillStyle(&mut self, index: i32) -> Option<&mut FillBase> {
        // java return null != this.mBgFillStyleLst && index >= 0 && index < this.mBgFillStyleLst.size() ? (FillBase)this.mBgFillStyleLst.get(index) : null;
        return if index >= 0 && (index as usize) < self.mBgFillStyleLst.len() {
            self.mBgFillStyleLst.get_mut(index as usize)
        } else {
            None
        };
    }

    // java public int getBgFillStyleSize()
    pub fn getBgFillStyleSize(&self) -> i32 {
        // java return this.mBgFillStyleLst != null ? this.mBgFillStyleLst.size() : 0;
        return self.mBgFillStyleLst.len() as i32;
    }

    // java public int addLineStyle(LineProperty lineProperty)
    pub fn addLineStyle(&mut self, lineProperty: LineProperty) -> i32 {
        // java int size = this.mLnStyleLst.size();
        let size: i32 = self.mLnStyleLst.len() as i32;
        // java this.mLnStyleLst.add(lineProperty);
        self.mLnStyleLst.push(lineProperty);
        // java return size;
        return size;
    }

    // java public LineProperty getLineStyle(int index)
    pub fn getLineStyle(&mut self, index: i32) -> Option<&mut LineProperty> {
        // java return null != this.mLnStyleLst && index >= 0 && index < this.mLnStyleLst.size() ? (LineProperty)this.mLnStyleLst.get(index) : null;
        return if index >= 0 && (index as usize) < self.mLnStyleLst.len() {
            self.mLnStyleLst.get_mut(index as usize)
        } else {
            None
        };
    }

    // java public int getLineStyleSize()
    pub fn getLineStyleSize(&self) -> i32 {
        // java return this.mLnStyleLst != null ? this.mLnStyleLst.size() : 0;
        return self.mLnStyleLst.len() as i32;
    }

    // java public int addEffectStyle(EffectStyle effect)
    pub fn addEffectStyle(&mut self, effect: EffectStyle) -> i32 {
        // java int size = this.mEffectStyleLst.size();
        let size: i32 = self.mEffectStyleLst.len() as i32;
        // java this.mEffectStyleLst.add(effect);
        self.mEffectStyleLst.push(effect);
        // java return size;
        return size;
    }

    // java public EffectStyle getEffectStyle(int index)
    pub fn getEffectStyle(&mut self, index: i32) -> Option<&mut EffectStyle> {
        // java return null != this.mEffectStyleLst && index >= 0 && index < this.mEffectStyleLst.size() ? (EffectStyle)this.mEffectStyleLst.get(index) : null;
        return if index >= 0 && (index as usize) < self.mEffectStyleLst.len() {
            self.mEffectStyleLst.get_mut(index as usize)
        } else {
            None
        };
    }

    // java public int getEffectStyleSize()
    pub fn getEffectStyleSize(&self) -> i32 {
        // java return this.mEffectStyleLst != null ? this.mEffectStyleLst.size() : 0;
        return self.mEffectStyleLst.len() as i32;
    }
}
