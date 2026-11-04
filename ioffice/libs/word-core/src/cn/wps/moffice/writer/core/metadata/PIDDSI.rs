// Java: import cn.wps.moffice.writer.core.metadata.SummaryBase
use crate::cn::wps::moffice::writer::core::metadata::SummaryBase::*;
// Java: import cn.wps.moffice.writer.core.metadata.Version
use crate::cn::wps::moffice::writer::core::metadata::Version::*;

// java public class PIDDSI extends SummaryBase
#[derive(Default, Clone)]
pub struct PIDDSI {
    // java SummaryBase super_
    pub(crate) super_: SummaryBase,
}

// java public class PIDDSI extends SummaryBase
impl PIDDSI {
    // java public String getCategory()
    pub fn getCategory(&self) -> String {
      // java return (String)this.mProperty.getValue(2);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(2).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setCategory(String category)
    pub fn setCategory(&mut self, category: &str) {
      // java this.mProperty.setString(2, category.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(2, category.to_string());
    }

    // java public String getPresentationFormat()
    pub fn getPresentationFormat(&self) -> String {
      // java return (String)this.mProperty.getValue(3);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(3).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setPresentationFormat(String format)
    pub fn setPresentationFormat(&mut self, format: &str) {
      // java this.mProperty.setString(3, format.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(3, format.to_string());
    }

    // java public Integer getByteCount()
    pub fn getByteCount(&self) -> i32 {
      // java return (Integer)this.mProperty.getValue(4);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(4).and_then(|v| v.downcast_ref::<i32>().copied()).unwrap_or(0);
    }

    // java public void setByteCount(int bytes)
    pub fn setByteCount(&mut self, bytes: i32) {
      // java this.mProperty.setInt(4, bytes);
      (self.mProperty.as_mut().expect("mProperty")).setInt(4, bytes);
    }

    // java public Integer getLineCount()
    pub fn getLineCount(&self) -> i32 {
      // java return (Integer)this.mProperty.getValue(5);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(5).and_then(|v| v.downcast_ref::<i32>().copied()).unwrap_or(0);
    }

    // java public void setLineCount(int lines)
    pub fn setLineCount(&mut self, lines: i32) {
      // java this.mProperty.setInt(5, lines);
      (self.mProperty.as_mut().expect("mProperty")).setInt(5, lines);
    }

    // java public Integer getParaCount()
    pub fn getParaCount(&self) -> i32 {
      // java return (Integer)this.mProperty.getValue(6);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(6).and_then(|v| v.downcast_ref::<i32>().copied()).unwrap_or(0);
    }

    // java public void setParaCount(int paras)
    pub fn setParaCount(&mut self, paras: i32) {
      // java this.mProperty.setInt(6, paras);
      (self.mProperty.as_mut().expect("mProperty")).setInt(6, paras);
    }

    // java public Integer getSlideCount()
    pub fn getSlideCount(&self) -> i32 {
      // java return (Integer)this.mProperty.getValue(7);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(7).and_then(|v| v.downcast_ref::<i32>().copied()).unwrap_or(0);
    }

    // java public void setSlideCount(int slides)
    pub fn setSlideCount(&mut self, slides: i32) {
      // java this.mProperty.setInt(7, slides);
      (self.mProperty.as_mut().expect("mProperty")).setInt(7, slides);
    }

    // java public Integer getNoteCount()
    pub fn getNoteCount(&self) -> i32 {
      // java return (Integer)this.mProperty.getValue(8);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(8).and_then(|v| v.downcast_ref::<i32>().copied()).unwrap_or(0);
    }

    // java public void setNoteCount(int notes)
    pub fn setNoteCount(&mut self, notes: i32) {
      // java this.mProperty.setInt(8, notes);
      (self.mProperty.as_mut().expect("mProperty")).setInt(8, notes);
    }

    // java public Integer getHiddenSlideCount()
    pub fn getHiddenSlideCount(&self) -> i32 {
      // java return (Integer)this.mProperty.getValue(9);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(9).and_then(|v| v.downcast_ref::<i32>().copied()).unwrap_or(0);
    }

    // java public void setHiddenSlideCount(int slides)
    pub fn setHiddenSlideCount(&mut self, slides: i32) {
      // java this.mProperty.setInt(9, slides);
      (self.mProperty.as_mut().expect("mProperty")).setInt(9, slides);
    }

    // java public Integer getMMClipCount()
    pub fn getMMClipCount(&self) -> i32 {
      // java return (Integer)this.mProperty.getValue(10);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(10).and_then(|v| v.downcast_ref::<i32>().copied()).unwrap_or(0);
    }

    // java public void setMMClipCount(int clips)
    pub fn setMMClipCount(&mut self, clips: i32) {
      // java this.mProperty.setInt(10, clips);
      (self.mProperty.as_mut().expect("mProperty")).setInt(10, clips);
    }

    // java public Boolean getScale()
    pub fn getScale(&self) -> bool {
      // java return (Boolean)this.mProperty.getValue(11);
      return todo!("cast → bool");
    }

    // java public void setScale(boolean scale)
    pub fn setScale(&mut self, scale: bool) {
      // java this.mProperty.setBoolean(11, scale);
      (self.mProperty.as_mut().expect("mProperty")).setBoolean(11, scale);
    }

    // java public String getManager()
    pub fn getManager(&self) -> String {
      // java return (String)this.mProperty.getValue(14);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(14).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setManager(String manager)
    pub fn setManager(&mut self, manager: &str) {
      // java this.mProperty.setString(14, manager.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(14, manager.to_string());
    }

    // java public String getCompany()
    pub fn getCompany(&self) -> String {
      // java return (String)this.mProperty.getValue(15);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(15).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setCompany(String company)
    pub fn setCompany(&mut self, company: &str) {
      // java this.mProperty.setString(15, company.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(15, company.to_string());
    }

    // java public Boolean getLinksDirty()
    pub fn getLinksDirty(&self) -> bool {
      // java return (Boolean)this.mProperty.getValue(16);
      return todo!("cast → bool");
    }

    // java public void setLinksDirty(boolean linksDirty)
    pub fn setLinksDirty(&mut self, linksDirty: bool) {
      // java this.mProperty.setBoolean(16, linksDirty);
      (self.mProperty.as_mut().expect("mProperty")).setBoolean(16, linksDirty);
    }

    // java public Integer getCCHWithSpace()
    pub fn getCCHWithSpace(&self) -> i32 {
      // java return (Integer)this.mProperty.getValue(17);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(17).and_then(|v| v.downcast_ref::<i32>().copied()).unwrap_or(0);
    }

    // java public void setCCHWithSpace(int cch)
    pub fn setCCHWithSpace(&mut self, cch: i32) {
      // java this.mProperty.setInt(17, cch);
      (self.mProperty.as_mut().expect("mProperty")).setInt(17, cch);
    }

    // java public Boolean getSharedDoc()
    pub fn getSharedDoc(&self) -> bool {
      // java return (Boolean)this.mProperty.getValue(18);
      return todo!("cast → bool");
    }

    // java public void setSharedDoc(boolean shared)
    pub fn setSharedDoc(&mut self, shared: bool) {
      // java this.mProperty.setBoolean(18, shared);
      (self.mProperty.as_mut().expect("mProperty")).setBoolean(18, shared);
    }

    // java public String getLinkBase()
    pub fn getLinkBase(&self) -> String {
      // java return (String)this.mProperty.getValue(19);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(19).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setLinkBase(String linkBase)
    pub fn setLinkBase(&mut self, linkBase: &str) {
      // java this.mProperty.setString(19, linkBase.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(19, linkBase.to_string());
    }

    // java public Boolean getHyperlinksChanged()
    pub fn getHyperlinksChanged(&self) -> bool {
      // java return (Boolean)this.mProperty.getValue(21);
      return todo!("cast → bool");
    }

    // java public void setHyperlinksChanged(boolean changed)
    pub fn setHyperlinksChanged(&mut self, changed: bool) {
      // java this.mProperty.setBoolean(21, changed);
      (self.mProperty.as_mut().expect("mProperty")).setBoolean(21, changed);
    }

    // java public Version getVersion()
    pub fn getVersion(&self) -> Version {
      // java return (Version)this.mProperty.getValue(22);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(22)
        .and_then(|v| v.downcast_ref::<Version>().cloned()).unwrap_or_default();
    }

    // java public void setVersion(Version version)
    pub fn setVersion(&mut self, version: &Version) {
      // java this.mProperty.setValue(22, version);
      (self.mProperty.as_mut().expect("mProperty")).setValue(22,
        Some(std::sync::Arc::new(version.clone()) as std::sync::Arc<dyn std::any::Any + Send + Sync>));
    }

    // java public String getContentType()
    pub fn getContentType(&self) -> String {
      // java return (String)this.mProperty.getValue(24);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(24).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setContentType(String type)
    pub fn setContentType(&mut self, r#type: &str) {
      // java this.mProperty.setString(24, type.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(24, r#type.to_string());
    }

    // java public String getContentStatus()
    pub fn getContentStatus(&self) -> String {
      // java return (String)this.mProperty.getValue(25);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(25).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setContentStatus(String status)
    pub fn setContentStatus(&mut self, status: &str) {
      // java this.mProperty.setString(25, status.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(25, status.to_string());
    }

    // java public String getLanguage()
    pub fn getLanguage(&self) -> String {
      // java return (String)this.mProperty.getValue(26);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(26).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setLanguage(String language)
    pub fn setLanguage(&mut self, language: &str) {
      // java this.mProperty.setString(26, language.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(26, language.to_string());
    }

    // java public String getDocVersion()
    pub fn getDocVersion(&self) -> String {
      // java return (String)this.mProperty.getValue(27);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(27).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setDocVersion(String docVersion)
    pub fn setDocVersion(&mut self, docVersion: &str) {
      // java this.mProperty.setString(27, docVersion.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(27, docVersion.to_string());
    }

}

impl std::ops::Deref for PIDDSI {
    type Target = SummaryBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PIDDSI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
