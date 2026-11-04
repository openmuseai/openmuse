// Java: cn.wps.moffice.writer.data.theme.font.SupplementalFont
//
// ★M-STAGE1 P2-8 —— 由 3 行 "Compile-surface shell"（`pub struct SupplementalFont;`）**做实**。
//   Java 原文 31 行：3 个字段 + 3 对访问器。

// java public class SupplementalFont
#[derive(Default, Clone, Debug)]
pub struct SupplementalFont {
    // java private int mScript;
    mScript: i32,
    // java private String mTypeface;
    mTypeface: Option<String>,
    // java private String mScriptStr;
    mScriptStr: Option<String>,
}

// java public class SupplementalFont
impl SupplementalFont {
    // rust: Java 无显式构造器（走默认构造）⇒ `new()` 为 Rust 侧便利方法，
    //   不对应 Java 成员，故不写 `// java` 方法注释。
    pub fn new() -> Self {
        Self::default()
    }

    // java public int getScript()
    pub fn getScript(&self) -> i32 {
        // java return this.mScript;
        return self.mScript;
    }

    // java public void setScript(int script)
    pub fn setScript(&mut self, script: i32) {
        // java this.mScript = script;
        self.mScript = script;
    }

    // java public String getScriptStr()
    pub fn getScriptStr(&self) -> Option<String> {
        // java return this.mScriptStr;
        return self.mScriptStr.clone();
    }

    // java public void setScriptStr(String script)
    pub fn setScriptStr(&mut self, script: Option<String>) {
        // java this.mScriptStr = script;
        self.mScriptStr = script;
    }

    // java public String getTypeface()
    pub fn getTypeface(&self) -> Option<String> {
        // java return this.mTypeface;
        return self.mTypeface.clone();
    }

    // java public void setTypeface(String typeface)
    pub fn setTypeface(&mut self, typeface: Option<String>) {
        // java this.mTypeface = typeface;
        self.mTypeface = typeface;
    }
}
