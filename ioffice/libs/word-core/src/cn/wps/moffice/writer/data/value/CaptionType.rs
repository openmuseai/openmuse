// Java: cn.wps.moffice.writer.data.value.CaptionType
//
// ★M-STAGE1 P2 —— 新建类型。全仓此前缺失（0 hit），是 `CapationsImporter`
//   依赖闭包的成员（`CapationsImporter.java:6`）。
//
// Java 原文 76 行，零外部依赖（仅 `Assert`）。
// ★字段类型映射：`Integer`→`Option<i32>`、`Boolean`→`Option<bool>`，
//   且 Java 每个 setter 都有 `assertNotNull` ⇒ Rust 侧参数为 `i32`/`bool`（按值非空）。

// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;

// Java: public class CaptionType
#[derive(Default, Clone)]
pub struct CaptionType {
    // java private String mName = null;
    mName: Option<String>,
    // java private Integer mPos = null;
    mPos: Option<i32>,
    // java private Boolean mChapNum = null;
    mChapNum: Option<bool>,
    // java private Integer mHeading = null;
    mHeading: Option<i32>,
    // java private Boolean mNoLabel = null;
    mNoLabel: Option<bool>,
    // java private Integer mNumFmt = null;
    mNumFmt: Option<i32>,
    // java private Integer mSep = null;
    mSep: Option<i32>,
}

impl CaptionType {
    // java public CaptionType(String name)
    pub fn CaptionType(name: String) -> Self {
        // java Assert.assertNotNull("name should not be null", name);
        Assert::assertNotNull_msg("name should not be null", &Some(name.clone()));
        // java this.mName = name;
        CaptionType {
            mName: Some(name),
            ..Default::default()
        }
    }

    // java public void setPos(Integer pos)
    pub fn setPos(&mut self, pos: i32) {
        // java Assert.assertNotNull("pos should not be null", pos);
        Assert::assertNotNull_msg("pos should not be null", &Some(pos));
        // java this.mPos = pos;
        self.mPos = Some(pos);
    }

    // java public void setChapNum(Boolean chapNum)
    pub fn setChapNum(&mut self, chapNum: bool) {
        // java Assert.assertNotNull("chapNum should not be null", chapNum);
        Assert::assertNotNull_msg("chapNum should not be null", &Some(chapNum));
        // java this.mChapNum = chapNum;
        self.mChapNum = Some(chapNum);
    }

    // java public void setHeading(Integer heading)
    pub fn setHeading(&mut self, heading: i32) {
        // java Assert.assertNotNull("heading should not be null", heading);
        Assert::assertNotNull_msg("heading should not be null", &Some(heading));
        // java this.mHeading = heading;
        self.mHeading = Some(heading);
    }

    // java public void setNoLabel(Boolean noLabel)
    pub fn setNoLabel(&mut self, noLabel: bool) {
        // java Assert.assertNotNull("noLabel should not be null", noLabel);
        Assert::assertNotNull_msg("noLabel should not be null", &Some(noLabel));
        // java this.mNoLabel = noLabel;
        self.mNoLabel = Some(noLabel);
    }

    // java public void setNumFmt(Integer numFmt)
    pub fn setNumFmt(&mut self, numFmt: i32) {
        // java Assert.assertNotNull("numFmt should not be null", numFmt);
        Assert::assertNotNull_msg("numFmt should not be null", &Some(numFmt));
        // java this.mNumFmt = numFmt;
        self.mNumFmt = Some(numFmt);
    }

    // java public void setSep(Integer sep)
    pub fn setSep(&mut self, sep: i32) {
        // java Assert.assertNotNull("sep should not be null", sep);
        Assert::assertNotNull_msg("sep should not be null", &Some(sep));
        // java this.mSep = sep;
        self.mSep = Some(sep);
    }

    // java public String getName()
    pub fn getName(&self) -> Option<String> {
        // java return this.mName;
        return self.mName.clone();
    }

    // java public Integer getPos()
    pub fn getPos(&self) -> Option<i32> {
        // java return this.mPos;
        return self.mPos;
    }

    // java public Boolean getChapNum()
    pub fn getChapNum(&self) -> Option<bool> {
        // java return this.mChapNum;
        return self.mChapNum;
    }

    // java public Integer getHeading()
    pub fn getHeading(&self) -> Option<i32> {
        // java return this.mHeading;
        return self.mHeading;
    }

    // java public Boolean getNoLabel()
    pub fn getNoLabel(&self) -> Option<bool> {
        // java return this.mNoLabel;
        return self.mNoLabel;
    }

    // java public Integer getNumFmt()
    pub fn getNumFmt(&self) -> Option<i32> {
        // java return this.mNumFmt;
        return self.mNumFmt;
    }

    // java public Integer getSep()
    pub fn getSep(&self) -> Option<i32> {
        // java return this.mSep;
        return self.mSep;
    }
}
