// java public final class OpcRelation
#[derive(Clone, Default)]
pub struct OpcRelation {
    // java private String mContentType;
    mContentType: Option<String>,
    // java private String mRelsType;
    mRelsType: Option<String>,
    // java private String mPartName;
    mPartName: Option<String>,
}

// Native Java nullable String call adapter; legacy non-null Rust callers keep
// their same three arguments, while null remains distinct from an empty value.
pub trait JavaStringArg{fn optional(self)->Option<String>;}
impl JavaStringArg for &str{fn optional(self)->Option<String>{Some(self.to_string())}}
impl JavaStringArg for String{fn optional(self)->Option<String>{Some(self)}}
impl JavaStringArg for &String{fn optional(self)->Option<String>{Some(self.clone())}}
impl JavaStringArg for Option<&str>{fn optional(self)->Option<String>{self.map(str::to_string)}}
impl JavaStringArg for Option<String>{fn optional(self)->Option<String>{self}}

// java public final class OpcRelation
impl OpcRelation {
    // java public  OpcRelation(String contentType, String relsType, String partName)
    pub fn OpcRelation(contentType: impl JavaStringArg, relsType: impl JavaStringArg, partName: impl JavaStringArg) -> Self {
      // java super();
      // rust: no-stmt
      // java this.mContentType = contentType;
      // java this.mRelsType = relsType;
      // java this.mPartName = partName;
      Self {
        mContentType: contentType.optional(),
        mRelsType: relsType.optional(),
        mPartName: partName.optional(),
      }
    }

    // java public String getContentType()
    pub fn getContentType(&self) -> Option<String> {
      // java return this.mContentType;
      return self.mContentType.clone();
    }

    // java public String getRelsType()
    pub fn getRelsType(&self) -> Option<String> {
      // java return this.mRelsType;
      return self.mRelsType.clone();
    }

    // java public String getPartName()
    pub fn getPartName(&self) -> Option<String> {
      // java return this.mPartName;
      return self.mPartName.clone();
    }

    // java public String getPartName(int index)
    pub fn getPartName__index(&self, index: i32) -> String {
      // java return this.mPartName.indexOf("#") == -1 ? this.mPartName : this.mPartName.replace("#", Integer.toString(index));
      let partName=self.mPartName.as_ref().expect("NullPointerException");
      if partName.find('#').is_none() {
        partName.clone()
      } else {
        partName.replace('#', &index.to_string())
      }
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuilder builder = new StringBuilder();
      let mut builder = String::new();
      // java builder.append("class OxfRelation:\n");
      builder.push_str("class OxfRelation:\n");
      // java builder.append("\tmContentType: ");
      builder.push_str("\tmContentType: ");
      // java builder.append(this.mContentType);
      builder.push_str(self.mContentType.as_deref().unwrap_or("null"));
      // java builder.append("\n\tmRelsType: ");
      builder.push_str("\n\tmRelsType: ");
      // java builder.append(this.mRelsType);
      builder.push_str(self.mRelsType.as_deref().unwrap_or("null"));
      // java builder.append("\n\tmPartName: ");
      builder.push_str("\n\tmPartName: ");
      // java builder.append(this.mPartName);
      builder.push_str(self.mPartName.as_deref().unwrap_or("null"));
      // java return builder.toString();
      return builder;
    }

}
