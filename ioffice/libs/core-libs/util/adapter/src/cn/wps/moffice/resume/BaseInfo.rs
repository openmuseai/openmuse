// Java: import com.google.gson.annotations.Expose
// Java: import com.google.gson.annotations.SerializedName

use super::BaseData::BaseData;

// Java: public class BaseInfo extends BaseData
pub struct BaseInfo {
    pub(crate) base: BaseData,
    // Java field: String mName
    pub(crate) mName: String,
    // Java field: String mGender
    pub(crate) mGender: String,
    // Java field: String mAge
    pub(crate) mAge: String,
    // Java field: String mPhone
    pub(crate) mPhone: String,
    // Java field: String mMail
    pub(crate) mMail: String,
    // Java field: String mUrl
    pub(crate) mUrl: String,
    // Java field: String mAvatarContent
    mAvatarContent: Option<String>,
    // Java field: boolean mHideAvatar
    mHideAvatar: bool,
    // Java field: String mBase64
    pub(crate) mBase64: String,
    // Java field: String mLocalUrl
    pub(crate) mLocalUrl: String,
}

// Java: public class BaseInfo extends BaseData
impl BaseInfo {
    // Java: BaseInfo()
    pub fn new() -> Self {
      // Java: super();
      BaseInfo {
          base: BaseData::new(),
          mName: String::new(),
          mGender: String::new(),
          mAge: String::new(),
          mPhone: String::new(),
          mMail: String::new(),
          mUrl: String::new(),
          mAvatarContent: None,
          mHideAvatar: false,
          mBase64: String::new(),
          mLocalUrl: String::new(),
      }
    }

    // Java: void setHideAvatar(boolean hideAvatar)
    pub fn setHideAvatar(&mut self, _hideAvatar: bool) {
      // Java: this.mHideAvatar = hideAvatar;
      self.mHideAvatar = _hideAvatar;
    }

    // Java: void setBase64(String base64)
    pub fn setBase64(&mut self, _base64: &str) {
      // Java: this.mBase64 = base64;
      self.mBase64 = _base64.to_string();
    }

    // Java: String getBase64()
    pub fn getBase64(&self) -> &str {
      // Java: return this.mBase64;
      &self.mBase64
    }

    // Java: void setAvatarContent(String avatarContent)
    pub fn setAvatarContent(&mut self, _avatarContent: &str) {
      // Java: this.mAvatarContent = avatarContent;
      self.mAvatarContent = Some(_avatarContent.to_string());
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
      // Java: return this.mName;
      &self.mName
    }

    // Java: String getGender()
    pub fn getGender(&self) -> &str {
      // Java: return this.mGender;
      &self.mGender
    }

    // Java: String getAge()
    pub fn getAge(&self) -> &str {
      // Java: return this.mAge;
      &self.mAge
    }

    // Java: String getPhone()
    pub fn getPhone(&self) -> &str {
      // Java: return this.mPhone;
      &self.mPhone
    }

    // Java: String getMail()
    pub fn getMail(&self) -> &str {
      // Java: return this.mMail;
      &self.mMail
    }

    // Java: String getUrl()
    pub fn getUrl(&self) -> &str {
      // Java: return this.mUrl;
      &self.mUrl
    }

    // Java: String getLocalUrl()
    pub fn getLocalUrl(&self) -> &str {
      // Java: return this.mLocalUrl;
      &self.mLocalUrl
    }

    // Java: void setLocalUrl(String localUrl)
    pub fn setLocalUrl(&mut self, _localUrl: &str) {
      // Java: this.mLocalUrl = localUrl;
      self.mLocalUrl = _localUrl.to_string();
    }

}
