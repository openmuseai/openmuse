// Java: cn.wps.moffice.writer.data.value.DocVar
//
// ★M-STAGE1 P2 —— 新建类型。此前面全仓缺失（0 hit），是 `SettingsImporter`
//   依赖闭包中的一环（`SettingsImporter.java:20 import ...data.value.DocVar`）。
//   按 §24.4 判据，闭包内类型必须逐个做实后才能登记依赖者。
//
// Java 原文 23 行，零外部依赖（仅 `Assert`）。

// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;

// Java: public class DocVar
#[derive(Clone)]
pub struct DocVar {
    // java private String mName = null;
    mName: Option<String>,
    // java private String mVal = null;
    mVal: Option<String>,
}

impl DocVar {
    // java public DocVar(String name, String val)
    pub fn DocVar(name: String, val: String) -> Self {
        // java Assert.assertNotNull("name should not be null !", name);
        Assert::assertNotNull_msg("name should not be null !", &Some(name.clone()));
        // java Assert.assertNotNull("val should not be null !", val);
        Assert::assertNotNull_msg("val should not be null !", &Some(val.clone()));
        // java this.mName = name;
        // java this.mVal = val;
        DocVar {
            mName: Some(name),
            mVal: Some(val),
        }
    }

    // java public String getName()
    pub fn getName(&self) -> Option<String> {
        // java return this.mName;
        return self.mName.clone();
    }

    // java public String getVal()
    pub fn getVal(&self) -> Option<String> {
        // java return this.mVal;
        return self.mVal.clone();
    }
}
