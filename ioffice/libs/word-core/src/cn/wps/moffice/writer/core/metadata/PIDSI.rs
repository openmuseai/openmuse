// Java: import java.util.Date
use stubs::java::util::Date::*;
// Java: import cn.wps.moffice.writer.core.metadata.SummaryBase
use crate::cn::wps::moffice::writer::core::metadata::SummaryBase::*;

// java public class PIDSI extends SummaryBase
#[derive(Default, Clone)]
pub struct PIDSI {
    // java SummaryBase super_
    pub(crate) super_: SummaryBase,
}

// java public class PIDSI extends SummaryBase
impl PIDSI {
    // java public String getTitle()
    pub fn getTitle(&self) -> String {
      // java return (String)this.mProperty.getValue(2);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(2).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setTitle(String category)
    pub fn setTitle(&mut self, category: &str) {
      // java this.mProperty.setString(2, category.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(2, category.to_string());
    }

    // java public String getSubject()
    pub fn getSubject(&self) -> String {
      // java return (String)this.mProperty.getValue(3);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(3).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setSubject(String subject)
    pub fn setSubject(&mut self, subject: &str) {
      // java this.mProperty.setString(3, subject.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(3, subject.to_string());
    }

    // java public String getAuthor()
    pub fn getAuthor(&self) -> String {
      // java return (String)this.mProperty.getValue(4);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(4).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setAuthor(String author)
    pub fn setAuthor(&mut self, author: &str) {
      // java this.mProperty.setString(4, author.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(4, author.to_string());
    }

    // java public String getKeyWords()
    pub fn getKeyWords(&self) -> String {
      // java return (String)this.mProperty.getValue(5);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(5).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setKeyWords(String keyWords)
    pub fn setKeyWords(&mut self, keyWords: &str) {
      // java this.mProperty.setString(5, keyWords.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(5, keyWords.to_string());
    }

    // java public String getComments()
    pub fn getComments(&self) -> String {
      // java return (String)this.mProperty.getValue(6);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(6).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setComments(String comments)
    pub fn setComments(&mut self, comments: &str) {
      // java this.mProperty.setString(6, comments.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(6, comments.to_string());
    }

    // java public String getTemplate()
    pub fn getTemplate(&self) -> String {
      // java return (String)this.mProperty.getValue(7);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(7).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setTemplate(String template)
    pub fn setTemplate(&mut self, template: &str) {
      // java this.mProperty.setString(7, template.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(7, template.to_string());
    }

    // java public String getLastAuthor()
    pub fn getLastAuthor(&self) -> String {
      // java return (String)this.mProperty.getValue(8);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(8).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setLastAuthor(String author)
    pub fn setLastAuthor(&mut self, author: &str) {
      // java this.mProperty.setString(8, author.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(8, author.to_string());
    }

    // java public Integer getRevNumber()
    pub fn getRevNumber(&self) -> i32 {
      // java return (Integer)this.mProperty.getValue(9);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(9).and_then(|v| v.downcast_ref::<i32>().copied()).unwrap_or(0);
    }

    // java public void setRevNumber(int number)
    pub fn setRevNumber(&mut self, number: i32) {
      // java this.mProperty.setInt(9, number);
      (self.mProperty.as_mut().expect("mProperty")).setInt(9, number);
    }

    // java public Long getEditTime()
    pub fn getEditTime(&self) -> i64 {
      // java return (Long)this.mProperty.getValue(10);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(10).and_then(|v| v.downcast_ref::<i64>().copied()).unwrap_or(0);
    }

    // java public void setEditTime(long time)
    pub fn setEditTime(&mut self, time: i64) {
      // java this.mProperty.setValue(10, Some(std::sync::Arc::new(time.clone()) as std::sync::Arc<dyn std::any::Any + Send + Sync>));
      (self.mProperty.as_mut().expect("mProperty")).setValue(10, Some(std::sync::Arc::new(time.clone()) as std::sync::Arc<dyn std::any::Any + Send + Sync>));
    }

    // java @Deprecated public Date getLastPrintTime()
    pub fn getLastPrintTime(&self) -> Date {
      // java return (Date)this.mProperty.getValue(11);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(11).and_then(|v| v.downcast_ref::<Date>().cloned()).unwrap_or_default();
    }

    // java @Deprecated public void setLastPrintTime(Date time)
    pub fn setLastPrintTime(&mut self, time: &Date) {
      // java this.mProperty.setValue(11, Some(std::sync::Arc::new(time.clone()) as std::sync::Arc<dyn std::any::Any + Send + Sync>));
      (self.mProperty.as_mut().expect("mProperty")).setValue(11, Some(std::sync::Arc::new(time.clone()) as std::sync::Arc<dyn std::any::Any + Send + Sync>));
    }

    // java @Deprecated public Date getCreateTime()
    pub fn getCreateTime(&self) -> Date {
      // java return (Date)this.mProperty.getValue(12);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(12).and_then(|v| v.downcast_ref::<Date>().cloned()).unwrap_or_default();
    }

    // java @Deprecated public void setCreateTime(Date time)
    pub fn setCreateTime(&mut self, time: &Date) {
      // java this.mProperty.setValue(12, Some(std::sync::Arc::new(time.clone()) as std::sync::Arc<dyn std::any::Any + Send + Sync>));
      (self.mProperty.as_mut().expect("mProperty")).setValue(12, Some(std::sync::Arc::new(time.clone()) as std::sync::Arc<dyn std::any::Any + Send + Sync>));
    }

    // java @Deprecated public Date getLastSaveTime()
    pub fn getLastSaveTime(&self) -> Date {
      // java return (Date)this.mProperty.getValue(13);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(13).and_then(|v| v.downcast_ref::<Date>().cloned()).unwrap_or_default();
    }

    // java @Deprecated public void setLastSaveTime(Date time)
    pub fn setLastSaveTime(&mut self, time: &Date) {
      // java this.mProperty.setValue(13, Some(std::sync::Arc::new(time.clone()) as std::sync::Arc<dyn std::any::Any + Send + Sync>));
      (self.mProperty.as_mut().expect("mProperty")).setValue(13, Some(std::sync::Arc::new(time.clone()) as std::sync::Arc<dyn std::any::Any + Send + Sync>));
    }

    // java public Integer getPageCount()
    pub fn getPageCount(&self) -> i32 {
      // java return (Integer)this.mProperty.getValue(14);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(14).and_then(|v| v.downcast_ref::<i32>().copied()).unwrap_or(0);
    }

    // java public void setPageCount(int count)
    pub fn setPageCount(&mut self, count: i32) {
      // java this.mProperty.setInt(14, count);
      (self.mProperty.as_mut().expect("mProperty")).setInt(14, count);
    }

    // java public Integer getWordCount()
    pub fn getWordCount(&self) -> i32 {
      // java return (Integer)this.mProperty.getValue(15);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(15).and_then(|v| v.downcast_ref::<i32>().copied()).unwrap_or(0);
    }

    // java public void setWordCount(int count)
    pub fn setWordCount(&mut self, count: i32) {
      // java this.mProperty.setInt(15, count);
      (self.mProperty.as_mut().expect("mProperty")).setInt(15, count);
    }

    // java public Integer getCharCount()
    pub fn getCharCount(&self) -> i32 {
      // java return (Integer)this.mProperty.getValue(16);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(16).and_then(|v| v.downcast_ref::<i32>().copied()).unwrap_or(0);
    }

    // java public void setCharCount(int count)
    pub fn setCharCount(&mut self, count: i32) {
      // java this.mProperty.setInt(16, count);
      (self.mProperty.as_mut().expect("mProperty")).setInt(16, count);
    }

    // java public String getAppName()
    pub fn getAppName(&self) -> String {
      // java return (String)this.mProperty.getValue(18);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(18).and_then(|v| v.downcast_ref::<String>().cloned()).unwrap_or_default();
    }

    // java public void setAppName(String name)
    pub fn setAppName(&mut self, name: &str) {
      // java this.mProperty.setString(18, name.to_string());
      (self.mProperty.as_mut().expect("mProperty")).setString(18, name.to_string());
    }

    // java public Integer getDocSecurity()
    pub fn getDocSecurity(&self) -> i32 {
      // java return (Integer)this.mProperty.getValue(19);
      return (self.mProperty.as_ref().expect("mProperty")).getValue(19).and_then(|v| v.downcast_ref::<i32>().copied()).unwrap_or(0);
    }

    // java public void setDocSecurity(int security)
    pub fn setDocSecurity(&mut self, security: i32) {
      // java this.mProperty.setInt(19, security);
      (self.mProperty.as_mut().expect("mProperty")).setInt(19, security);
    }

}

impl std::ops::Deref for PIDSI {
    type Target = SummaryBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PIDSI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
