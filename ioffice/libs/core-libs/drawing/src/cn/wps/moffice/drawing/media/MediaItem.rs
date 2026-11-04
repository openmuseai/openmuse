#![allow(non_snake_case, dead_code, unused_variables)]

// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.util.zip.model.ZipBaseModel
use core_libs_util::util::zip::model::ZipBaseModel::ZipBaseModel;

use super::MediaTypeEnum::MediaTypeEnum;
use std::sync::{Arc, Mutex};

// Java: public class MediaItem
pub struct MediaItem {
    // Java field: int id
    pub(crate) id: i32,
    // Java field: String name
    pub(crate) name: Option<String>,
    // Java field: String path
    pub(crate) path: Option<String>,
    // Java field: MediaTypeEnum type
    pub(crate) media_type: MediaTypeEnum,
    // Java field: boolean mIsShared
    mIsShared: bool,
    // Java field: ZipBaseModel mZipBaseModel
    pub(crate) mZipBaseModel: Option<Arc<Mutex<ZipBaseModel>>>,
}

// Java: public class MediaItem
impl MediaItem {
    // Java: MediaItem(int id, String name, String path)
    pub fn new(id: i32, name: String, path: Option<String>) -> Self {
      // Java: super();
      // Java: this.type = MediaTypeEnum.PICTURE;
      // Java: this.id = id;
      // Java: this.name = name;
      // Java: this.path = path;
      MediaItem {
          id,
          name: Some(name),
          path,
          media_type: MediaTypeEnum::PICTURE,
          mIsShared: false,
          mZipBaseModel: None,
      }
    }

    // Java: MediaItem(int id, String name, String path, ZipBaseModel zipBaseModel)
    pub fn new_with_zip(id: i32, name: String, path: Option<String>, zipBaseModel: Option<Arc<Mutex<ZipBaseModel>>>) -> Self {
      // Java: super();
      // Java: this.type = MediaTypeEnum.PICTURE;
      // Java: this.id = id;
      // Java: this.name = name;
      // Java: this.path = path;
      // Java: this.mZipBaseModel = zipBaseModel;
      MediaItem {
          id,
          name: Some(name),
          path,
          media_type: MediaTypeEnum::PICTURE,
          mIsShared: false,
          mZipBaseModel: zipBaseModel,
      }
    }

    // Java: void dispose(String dir)
    pub fn dispose(&mut self, dir: &str) {
      // Java: File file = this.type == MediaTypeEnum.OLE ? new File(this.path) : new File(dir, this.name);
      let file_path = if self.media_type == MediaTypeEnum::OLE {
          self.path.clone().unwrap_or_default()
      } else {
          format!("{}{}", dir, self.name.as_deref().unwrap_or(""))
      };
      // Java: if (!this.mIsShared)
      if !self.mIsShared
      {
         // Java: if (file.exists() && !this.mIsShared)
         if std::path::Path::new(&file_path).exists() && !self.mIsShared
         {
            // Java: file.delete();
            let _ = std::fs::remove_file(&file_path);
         }
         // Java: this.path = null;
         self.path = None;
      }
      // Java: this.name = null;
      self.name = None;
      // Java: this.clearZipBaseModel(!this.mIsShared);
      self.clearZipBaseModel(!self.mIsShared);
    }

    // Java: void clearZipBaseModel(boolean isDeleteFile)
    pub fn clearZipBaseModel(&mut self, isDeleteFile: bool) {
      // Java: if (this.mZipBaseModel != null)
      if self.mZipBaseModel.is_some()
      {
         // Java: this.mZipBaseModel.dispose(isDeleteFile);
         self.mZipBaseModel.as_ref().unwrap().lock().unwrap().dispose(isDeleteFile);
         // Java: this.mZipBaseModel = null;
         self.mZipBaseModel = None;
      }
    }

    // Java: ZipBaseModel getZipBaseModel()
    pub fn getZipBaseModel(&self) -> &Option<Arc<Mutex<ZipBaseModel>>> {
      // Java: return this.mZipBaseModel;
      &self.mZipBaseModel
    }

    // Java: void setZipBaseModel(ZipBaseModel zipBaseModel)
    pub fn setZipBaseModel(&mut self, zipBaseModel: Option<Arc<Mutex<ZipBaseModel>>>) {
      // Java: this.mZipBaseModel = zipBaseModel;
      self.mZipBaseModel = zipBaseModel;
    }

    // Java: void setShare(boolean value)
    pub fn setShare(&mut self, value: bool) {
      // Java: this.mIsShared = value;
      self.mIsShared = value;
    }

    // Java: void setID(int id)
    pub fn setID(&mut self, id: i32) {
      // Java: this.id = id;
      self.id = id;
    }

    // Java: int getID()
    pub fn getID(&self) -> i32 {
      // Java: return this.id;
      self.id
    }

    // Java: void setName(String name)
    pub fn setName(&mut self, name: String) {
      // Java: this.name = name;
      self.name = Some(name);
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
      // Java: return this.name;
      self.name.clone().unwrap_or_default()
    }

    // Java: void setPath(String path)
    pub fn setPath(&mut self, path: String) {
      // Java: this.path = path;
      self.path = Some(path);
      // Java: this.clearZipBaseModel(false);
      self.clearZipBaseModel(false);
    }

    // Java: String getPath()
    pub fn getPath(&mut self) -> String {
      // Java: if (this.path != null)
      if self.path.is_some()
      {
         // Java: return this.path;
         return self.path.clone().unwrap();
      }
      // Java: else if (this.mZipBaseModel == null)
      else if self.mZipBaseModel.is_none()
      {
         // Java: return null;
         return String::new();
      }
      // Java: else
      else
      {
         // Java: File file = this.mZipBaseModel.getFile(true);
         let file = self.mZipBaseModel.as_ref().and_then(|model| model.lock().unwrap().getFile(true).clone());
         // Java: return file != null && file.length() > 0L ? file.getAbsolutePath() : null;
         return file.filter(|path| path.metadata().is_ok_and(|metadata| metadata.len() > 0))
             .map(|path| if path.is_absolute() { path } else {
                 std::env::current_dir().expect("current directory").join(path)
             }.to_string_lossy().into_owned())
             .unwrap_or_default();
      }
    }

    // Java: void setType(MediaTypeEnum type)
    pub fn setType(&mut self, media_type: MediaTypeEnum) {
      // Java: this.type = type;
      self.media_type = media_type;
    }

    // Java: MediaTypeEnum getType()
    pub fn getType(&self) -> MediaTypeEnum {
      // Java: return this.type;
      self.media_type
    }

}
