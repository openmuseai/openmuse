// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::*;

// java private static final String DEF_STYLEID = "";
const DEF_STYLEID: &str = "";

// java public final class IOStyle
#[derive(Default)]
pub struct IOStyle {
    // java Long mUiPriority;
    pub mUiPriority: Option<i64>,
    // java String mType;
    pub mType: Option<String>,
    // java String mName;
    pub mName: Option<String>,
    // java String mNext;
    pub mNext: Option<String>,
    // java String mLink;
    pub mLink: Option<String>,
    // java String mStyleId;
    pub mStyleId: Option<String>,
    // java String mBasedOn;
    pub mBasedOn: Option<String>,
    // java Boolean mQFormat;
    pub mQFormat: Option<bool>,
    // java public int mTblStyleRPrId;
    pub mTblStyleRPrId: i32,
    // java public int mTblStylePPrId;
    pub mTblStylePPrId: i32,
    // java public int mTblStyleTblPrId;
    pub mTblStyleTblPrId: i32,
    // java int mCoreId;
    pub mCoreId: i32,
    // java int mCoreType;
    pub mCoreType: i32,
    // java MutablePropertySet mCorePrSet = new MutablePropertySet();
    pub mCorePrSet: MutablePropertySet,
    // java Boolean mIsCustom;
    pub mIsCustom: Option<bool>,
}

// java public final class IOStyle
impl IOStyle {
    // java  void convertTypeAndId(StyleId2Sti styleId2Sti)
    pub fn convertTypeAndId(&mut self, styleId2Sti: &mut StyleId2Sti) {
      // java this.mCoreType = convertStypeType(this.mType);
      self.mCoreType = Self::convertStypeType(self.mType.as_deref());
      // java this.mCoreId = this.insertId2StiMap(this.mCoreType, styleId2Sti);
      self.mCoreId = self.insertId2StiMap(self.mCoreType, styleId2Sti);
      // java IOAssert.assertNotSame("a style always has a valid sti", 4095, this.mCoreId);
      IOAssert::assertNotSame_msg("a style always has a valid sti", &4095i32, &self.mCoreId);
      if self.mCoreId == 4095 {
        panic!("a style always has a valid sti");
      }
    }

    // java private int insertId2StiMap(int type, StyleId2Sti styleId2Sti)
    pub fn insertId2StiMap(&mut self, r#type: i32, styleId2Sti: &mut StyleId2Sti) -> i32 {
      // java boolean isCustom = this.mIsCustom == null ? false : this.mIsCustom;
      let isCustom: bool = self.mIsCustom.unwrap_or(false);
      // java String styleId = this.rectifyStyleId();
      let styleId: String = self.rectifyStyleId();
      // java return null != this.mName && this.mName.length() > 0 ? styleId2Sti.put(styleId, this.mName, type, isCustom) : styleId2Sti.put(styleId);
      if let Some(name) = self.mName.as_ref() {
        if name.len() > 0 {
          return styleId2Sti.put__string_string_int_boolean(&styleId, name, r#type, isCustom);
        }
      }
      return styleId2Sti.put__string(&styleId);
    }

    // java public String rectifyStyleId()
    pub fn rectifyStyleId(&self) -> String {
      // java if (null != this.mStyleId && this.mStyleId.length() > 0)
      if let Some(mStyleId) = self.mStyleId.as_ref() {
        if mStyleId.len() > 0 {
          // java return this.mStyleId;
          return mStyleId.clone();
        }
      }
      // java else
      // java return null != this.mName ? this.mName : "";
      if let Some(mName) = self.mName.as_ref() {
        return mName.clone();
      }
      return DEF_STYLEID.to_string();
    }

    // java private static int convertStypeType(String type)
    pub fn convertStypeType(r#type: Option<&str>) -> i32 {
      // java int coreType = 1;
      let mut coreType: i32 = 1;
      // java if ("paragraph".equals(type))
      if r#type == Some("paragraph") {
        // java coreType = 1;
        coreType = 1;
      }
      // java else if ("character".equals(type))
      else if r#type == Some("character") {
        // java coreType = 2;
        coreType = 2;
      }
      // java else if ("table".equals(type))
      else if r#type == Some("table") {
        // java coreType = 3;
        coreType = 3;
      }
      // java else if ("numbering".equals(type))
      else if r#type == Some("numbering") {
        // java coreType = 4;
        coreType = 4;
      }
      // java else
      else {
        // java IOAssert.fail("it should not reach here");
        IOAssert::fail_msg("it should not reach here");
        panic!("it should not reach here");
      }
      // java return coreType;
      return coreType;
    }

}
