// Java: public class CustomXmlInfo
pub struct CustomXmlInfo {
    // Java field: String mPathItem
    pub mPathItem: String,
    // Java field: String mPathItemProp
    pub mPathItemProp: String,
}

// Java: public class CustomXmlInfo
impl CustomXmlInfo {
    // Java: CustomXmlInfo(String item, String itemProp)
    pub fn CustomXmlInfo(item: String, itemProp: String) -> Self {
        // Java: super(); this.mPathItem = item; this.mPathItemProp = itemProp;
        Self::new(item, itemProp)
    }

    // rust: no-java（Rust 惯用别名，crate 内沿用 `new`）
    pub fn new(item: String, itemProp: String) -> Self {
      // Java: super();
      // Java: this.mPathItem = item;
      // Java: this.mPathItemProp = itemProp;
      CustomXmlInfo { mPathItem: item, mPathItemProp: itemProp }
    }

}
