// Java: cn.wps.moffice.writer.data.value.AutoCaptionType
//
// ★M-STAGE1 P2 —— 新建类型。全仓此前缺失（0 hit），是 `CapationsImporter`
//   依赖闭包的底层成员（`CapationsImporter.java:5`）。
//   按 §24.4 判据：闭包内类型须自底向上逐个做实。
//
// Java 原文 23 行，零外部依赖（仅 `Assert`）。结构与同目录 `DocVar` 同构。

// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;

// Java: public class AutoCaptionType
#[derive(Clone)]
pub struct AutoCaptionType {
    // java private String mName = null;
    mName: Option<String>,
    // java private String mCaption = null;
    mCaption: Option<String>,
}

impl AutoCaptionType {
    // java public AutoCaptionType(String name, String caption)
    pub fn AutoCaptionType(name: String, caption: String) -> Self {
        // java Assert.assertNotNull("name should not be null", name);
        Assert::assertNotNull_msg("name should not be null", &Some(name.clone()));
        // java Assert.assertNotNull("caption should not be null", caption);
        Assert::assertNotNull_msg("caption should not be null", &Some(caption.clone()));
        // java this.mName = name;
        // java this.mCaption = caption;
        AutoCaptionType {
            mName: Some(name),
            mCaption: Some(caption),
        }
    }

    // java public String getName()
    pub fn getName(&self) -> Option<String> {
        // java return this.mName;
        return self.mName.clone();
    }

    // java public String getCaption()
    pub fn getCaption(&self) -> Option<String> {
        // java return this.mCaption;
        return self.mCaption.clone();
    }
}
