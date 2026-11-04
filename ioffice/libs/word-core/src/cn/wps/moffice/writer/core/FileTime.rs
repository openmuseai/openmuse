// Java: import java.util.Date
use stubs::java::util::Date::*;

// java public class FileTime
#[derive(Default)]
pub struct FileTime {
    // java long mEditTime;
    pub(crate) mEditTime: i64,
    // java Date mCreateTime;
    pub(crate) mCreateTime: Option<Date>,
    // java Date mLastSaveTime;
    pub(crate) mLastSaveTime: Option<Date>,
    // java Date mFileOpenTime;
    pub(crate) mFileOpenTime: Option<Date>,
}

// java public class FileTime
impl FileTime {
    // java public  FileTime(long editTime, Date createTime, Date lastSaveTime, Date fileOpenTime)
    pub fn FileTime(editTime: i64, createTime: Option<Date>, lastSaveTime: Option<Date>, fileOpenTime: Option<Date>) -> Self {
      // java this.mEditTime = editTime;
      Self { mEditTime: editTime, mCreateTime: createTime, mLastSaveTime: lastSaveTime, mFileOpenTime: fileOpenTime }
      // java this.mCreateTime = createTime;
      // java this.mLastSaveTime = lastSaveTime;
      // java this.mFileOpenTime = fileOpenTime;
    }

    // java public long getEditTime()
    pub fn getEditTime(&self) -> i64 {
      // java return this.mEditTime;
      return self.mEditTime;
    }

    // java public void setEditTime(long editTime)
    pub fn setEditTime__long(&mut self, editTime: i64) {
      // java this.mEditTime = editTime;
      self.mEditTime = editTime;
    }

    // java public Date getCreateTime()
    pub fn getCreateTime(&self) -> Option<Date> {
      // java return this.mCreateTime;
      return self.mCreateTime.clone();
    }

    // java public void setEditTime(Date createTime)
    pub fn setEditTime__date(&mut self, createTime: Option<Date>) {
      // java this.mCreateTime = createTime;
      self.mCreateTime = createTime;
    }

    // java public Date getLastSaveTime()
    pub fn getLastSaveTime(&self) -> Option<Date> {
      // java return this.mLastSaveTime;
      return self.mLastSaveTime.clone();
    }

    // java public void setLastSaveTime(Date lastSaveTime)
    pub fn setLastSaveTime(&mut self, lastSaveTime: Option<Date>) {
      // java this.mLastSaveTime = lastSaveTime;
      self.mLastSaveTime = lastSaveTime;
    }

    // java public Date getFileOpenTime()
    pub fn getFileOpenTime(&self) -> Option<Date> {
      // java return this.mFileOpenTime;
      return self.mFileOpenTime.clone();
    }

    // java public void setFileOpenTime(Date lastSaveTime)
    pub fn setFileOpenTime(&mut self, lastSaveTime: Option<Date>) {
      // java this.mFileOpenTime = lastSaveTime;
      self.mFileOpenTime = lastSaveTime;
    }

}
