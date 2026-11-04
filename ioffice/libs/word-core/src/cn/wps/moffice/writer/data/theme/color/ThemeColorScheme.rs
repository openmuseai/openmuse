// Java: cn.wps.moffice.writer.data.theme.color.ThemeColorScheme
//
// ★M-STAGE1 P2-9 —— 由**破损桩**（签名引用未定义类型 `Integer`）做实。
//   Java 原文 34 行：2 个 Map 字段 + 2 组 add/get + setName/getName。
//
// rust: `WIntObjectHashMap<Integer>` ⇒ Rust 用 `HashMap<i32, i32>`。
//   Java `getSchemeColor(int)` 未命中时返回 `null`（`Integer`），
//   故 Rust 侧返回 `Option<i32>` —— 观测语义与 Java 一致。
//   （仓内 `WIntObjectHashMap::get` 返回 `&Element` 且未命中给 `Element::default()`，
//    与 Java 的 `null` 语义不符，故不采用。）

// Java: import java.util.HashMap
use std::collections::HashMap;

// java public class ThemeColorScheme
#[derive(Default, Clone)]
pub struct ThemeColorScheme {
    // java private String mColorSchemeName;
    mColorSchemeName: Option<String>,
    // java private HashMap<String, Integer> mClrScheme = new HashMap();
    mClrScheme: HashMap<String, i32>,
    // java private WIntObjectHashMap<Integer> mColorSchemeIndex2Rgb = new WIntObjectHashMap();
    mColorSchemeIndex2Rgb: HashMap<i32, i32>,
}

// java public class ThemeColorScheme
impl ThemeColorScheme {
    // rust: Java 无显式构造器（走默认构造 + 字段初始化器）⇒ `new()` 为 Rust 侧便利方法。
    pub fn new() -> Self {
        Self::default()
    }

    // java public void addSchemeColor(String name, int color)
    pub fn addSchemeColor(&mut self, name: String, color: i32) {
        // java this.mClrScheme.put(name, color);
        self.mClrScheme.insert(name, color);
    }

    // java public void addSchemeColor(int colorSchemeIndex, int color)
    pub fn addSchemeColor_1(&mut self, colorSchemeIndex: i32, color: i32) {
        // java this.mColorSchemeIndex2Rgb.put(colorSchemeIndex, color);
        self.mColorSchemeIndex2Rgb.insert(colorSchemeIndex, color);
    }

    // java public Integer getSchemeColor(String name)
    pub fn getSchemeColor(&self, name: String) -> Option<i32> {
        // java return (Integer)this.mClrScheme.get(name);
        return self.mClrScheme.get(&name).copied();
    }

    // java public Integer getSchemeColor(int colorSchemeIndex)
    pub fn getSchemeColor_1(&self, colorSchemeIndex: i32) -> Option<i32> {
        // java return (Integer)this.mColorSchemeIndex2Rgb.get(colorSchemeIndex);
        return self.mColorSchemeIndex2Rgb.get(&colorSchemeIndex).copied();
    }

    // java public void setName(String name)
    pub fn setName(&mut self, name: Option<String>) {
        // java this.mColorSchemeName = name;
        self.mColorSchemeName = name;
    }

    // java public String getName()
    pub fn getName(&self) -> Option<String> {
        // java return this.mColorSchemeName;
        return self.mColorSchemeName.clone();
    }
}
