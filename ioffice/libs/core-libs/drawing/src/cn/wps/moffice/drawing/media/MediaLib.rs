#![allow(non_snake_case, dead_code, unused_variables, unused_mut, unused_assignments)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.Log
// Java: import cn.wps.moffice.drawing.smartshape.CDL
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.moffice.slim.ISlimTool
// Java: import cn.wps.moffice.slim.ISlimToolManager
// Java: import cn.wps.moffice.util.FileUtil
// Java: import cn.wps.util.zip.model.ZipBaseModel
use core_libs_util::util::zip::model::ZipBaseModel::ZipBaseModel;
// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.Vector

use super::IMediaLib::{IMediaLib, MediaLibEventListenerTrait};
use super::MediaTypeEnum::MediaTypeEnum;
use super::MediaItem::MediaItem;
use std::sync::{Arc, Mutex};
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java Vector<MediaItem> exposes the same mutable MediaItem reference.
fn media_item(element: &Element) -> Option<&Mutex<MediaItem>> {
    element.as_ref()?.downcast_ref::<Mutex<MediaItem>>()
}

// Java field: String TAG
pub(crate) const TAG: &str = "MediaLib";
// Java: public class MediaLib implements IMediaLib
pub struct MediaLib {
    // Java field: Vector<MediaItem> mediaVector
    pub(crate) mediaVector: Option<Vec<Element>>,
    // Java field: Vector<MediaItem> oleVector
    pub(crate) oleVector: Option<Vec<Element>>,
    // Java field: String mediaPath
    mediaPath: Option<String>,
    // Java field: IMediaLib mediaLib
    // (static in Java, not used as instance field in Rust)
    // Java field: ArrayList<String> mSupportList
    mSupportList: Option<Vec<String>>,
}

// Java: public class MediaLib implements IMediaLib
impl MediaLib {
    // Java: MediaLib(String mediaPath)
    pub fn new(mediaPath: String) -> Self {
      // Java: super();
      // Java: this.mediaPath = mediaPath;
      let mut lib = MediaLib {
          mediaVector: Some(Vec::new()),
          oleVector: Some(Vec::new()),
          mediaPath: Some(mediaPath.clone()),
          mSupportList: None,
      };
      // Java: File file = new File(mediaPath);
      // Java: if (!file.exists())
      {
         // Java: file.mkdirs();
         let _ = std::fs::create_dir_all(&mediaPath);
      }
      lib
    }

    // Java: static IMediaLib createMediaLib(String mediaPath)
    pub fn createMediaLib(mediaPath: String) -> MediaLib {
      // Java: return new MediaLib(mediaPath);
      MediaLib::new(mediaPath)
    }

    // Java: void dispose()
    pub fn dispose_impl(&mut self) {
      // Java: if (this.mediaVector != null)
      if self.mediaVector.is_some()
      {
         // Java: int i = 0;
         let mut i = 0;
         // Java: for(int len = this.mediaVector.size(); i < len; ++i)
         let len = self.mediaVector.as_ref().unwrap().len();
         while i < len
         {
            // Java: MediaItem pItem = (MediaItem)this.mediaVector.get(i);
            let pItem = &self.mediaVector.as_ref().unwrap()[i];
            // Java: if (pItem != null)
            if let Some(item) = media_item(pItem) {
               // Java: pItem.dispose(this.mediaPath);
               item.lock().unwrap().dispose(self.mediaPath.as_deref().unwrap_or(""));
            }
            i += 1;
         }
      }
      // Java: if (this.oleVector != null)
      if self.oleVector.is_some()
      {
         // Java: int j = 0;
         let mut j = 0;
         // Java: for(int len = this.oleVector.size(); j < len; ++j)
         let len = self.oleVector.as_ref().unwrap().len();
         while j < len
         {
            // Java: MediaItem pItem = (MediaItem)this.oleVector.get(j);
            let pItem = &self.oleVector.as_ref().unwrap()[j];
            // Java: if (pItem != null)
            if let Some(item) = media_item(pItem) {
               // Java: pItem.dispose(this.mediaPath);
               item.lock().unwrap().dispose(self.mediaPath.as_deref().unwrap_or(""));
            }
            j += 1;
         }
      }
      // Java: this.mediaVector = null;
      self.mediaVector = None;
      // Java: this.oleVector = null;
      self.oleVector = None;
      // Java: this.mediaPath = null;
      self.mediaPath = None;
      // Java: if (null != this.mSupportList)
      if self.mSupportList.is_some()
      {
         // Java: this.mSupportList = null;
         self.mSupportList = None;
      }
      // Java: CDL.dispose();
      // TODO: CDL.dispose not yet available
    }

    // Java: void registerMediaLibEventListener(IMediaLib.MediaLibEventListener listener)
    pub fn registerMediaLibEventListener_impl(&mut self, _listener: Box<dyn MediaLibEventListenerTrait>) {
    }

    // Java: int addMedia(byte[] data, MediaTypeEnum type)
    pub fn addMedia_from_bytes_impl(&mut self, data: &[u8], mediaType: MediaTypeEnum) -> i32 {
      // Java: int id = -1;
      let mut id: i32 = -1;
      // Java: if (this.mediaVector == null)
      if self.mediaVector.is_none()
      {
         // Java: return id;
         return id;
      }
      // Java: else
      else
      {
         // Java: if (type == MediaTypeEnum.PICTURE)
         if mediaType == MediaTypeEnum::PICTURE
         {
            // Java: id = this.mediaVector.size();
            id = self.mediaVector.as_ref().unwrap().len() as i32;
         }
         // Java: else if (type == MediaTypeEnum.OLE)
         else if mediaType == MediaTypeEnum::OLE
         {
            // Java: id = this.oleVector.size();
            id = self.oleVector.as_ref().unwrap().len() as i32;
         }
         // Java: String name = id + ".m";
         let name = format!("{}.m", id);
         // Java: String path = this.mediaPath + name;
         let path = format!("{}{}", self.mediaPath.as_deref().unwrap_or(""), &name);
         // Java: File file = new File(path);
         // Java: if (!file.exists())
         if !std::path::Path::new(&path).exists()
         {
            // Java: try
            {
               // Java: file.createNewFile();
               let _ = std::fs::File::create(&path);
            }
            // Java: catch (IOException e)
            {
               // Java: Log.e(TAG, "IOException", e);
            }
         }
         // Java: FileOutputStream outputStream = null;
         // Java: try
         {
            // Java: outputStream = new FileOutputStream(path);
            // Java: outputStream.write(data);
            let _ = std::fs::write(&path, data);
         }
         // Java: catch (Exception e)
         {
            // Java: e.printStackTrace();
         }
         // Java: finally
         {
            // Java: FileUtil.closeQuietly(outputStream);
         }
         // Java: MediaItem item = new MediaItem(id, name, (String)null);
         let mut item = MediaItem::new(id, name, None);
         // Java: item.setType(type);
         item.setType(mediaType);
         // Java: if (type == MediaTypeEnum.PICTURE)
         if mediaType == MediaTypeEnum::PICTURE
         {
            // Java: this.mediaVector.add(item);
            self.mediaVector.as_mut().unwrap().push(Some(Arc::new(Mutex::new(item))));
         }
         // Java: else if (type == MediaTypeEnum.OLE)
         else if mediaType == MediaTypeEnum::OLE
         {
            // Java: this.oleVector.add(item);
            self.oleVector.as_mut().unwrap().push(Some(Arc::new(Mutex::new(item))));
         }
         // Java: return id;
         return id;
      }
    }

    // Java: boolean isSupportPicType(String name)
    pub fn isSupportPicType(&mut self, name: &str) -> bool {
      // Java: if (null != name && name.length() >= 1)
      if !name.is_empty()
      {
         // Java: String suffix = name.substring(name.lastIndexOf(".") + 1, name.length());
         let suffix = name.rfind('.').map(|idx| &name[idx + 1..]).unwrap_or("");
         // Java: if (null != suffix && suffix.length() >= 1)
         if !suffix.is_empty()
         {
            // Java: if (null == this.mSupportList)
            if self.mSupportList.is_none()
            {
               // Java: this.mSupportList = this.initSupportPicList();
               self.mSupportList = Some(Self::initSupportPicList());
            }
            // Java: return this.mSupportList.contains(suffix.toLowerCase());
            return self.mSupportList.as_ref().unwrap().contains(&suffix.to_lowercase());
         }
         // Java: else
         else
         {
            // Java: return false;
            return false;
         }
      }
      // Java: else
      else
      {
         // Java: return false;
         return false;
      }
    }

    // Java: ArrayList initSupportPicList()
    pub fn initSupportPicList() -> Vec<String> {
      // Java: ArrayList<String> supportList = new ArrayList();
      let mut supportList = Vec::new();
      // Java: supportList.add("jpg");
      supportList.push("jpg".to_string());
      // Java: supportList.add("jpeg");
      supportList.push("jpeg".to_string());
      // Java: supportList.add("jpe");
      supportList.push("jpe".to_string());
      // Java: supportList.add("jfjf");
      supportList.push("jfjf".to_string());
      // Java: supportList.add("jfif");
      supportList.push("jfif".to_string());
      // Java: supportList.add("png");
      supportList.push("png".to_string());
      // Java: supportList.add("bmp");
      supportList.push("bmp".to_string());
      // Java: supportList.add("wmf");
      supportList.push("wmf".to_string());
      // Java: supportList.add("emf");
      supportList.push("emf".to_string());
      // Java: supportList.add("gif");
      supportList.push("gif".to_string());
      // Java: supportList.add("dib");
      supportList.push("dib".to_string());
      // Java: supportList.add("pict");
      supportList.push("pict".to_string());
      // Java: supportList.add("tiff");
      supportList.push("tiff".to_string());
      // Java: supportList.add("tif");
      supportList.push("tif".to_string());
      // Java: return supportList;
      supportList
    }

    // Java: int addMedia(String name, String path, MediaTypeEnum type)
    pub fn addMedia_impl(&mut self, name: &str, path: &str, mediaType: MediaTypeEnum) -> i32 {
      // Java: return this.addMedia(name, path, type, (ZipBaseModel)null);
      self.addMedia_with_zip_impl(name, path, mediaType, None)
    }

    // Java: int addMedia(String name, String path, MediaTypeEnum type, ZipBaseModel zipBaseModel)
    pub fn addMedia_with_zip_impl(&mut self, name: &str, path: &str, mediaType: MediaTypeEnum, zipBaseModel: Option<Arc<Mutex<ZipBaseModel>>>) -> i32 {
      // Java: if (type == MediaTypeEnum.PICTURE)
      if mediaType == MediaTypeEnum::PICTURE
      {
         // Java: IOAssert.assertTrue(this.isSupportPicType(name));
         debug_assert!(self.isSupportPicType(name));
      }
      // Java: int id = -1;
      let mut id: i32 = -1;
      // Java: if (this.mediaVector == null)
      if self.mediaVector.is_none()
      {
         // Java: return id;
         return id;
      }
      // Java: else
      else
      {
         // Java: ISlimTool slimTool = ISlimToolManager.instance().getSlimTool(3);
         // Java: slimTool.checkSlim(path != null ? path : zipBaseModel.getFile(true));
         // TODO: ISlimToolManager not yet available
         // Java: if (path != null && path.equals(this.mediaPath + name))
         let media_path_name = format!("{}{}", self.mediaPath.as_deref().unwrap_or(""), name);
         if !path.is_empty() && path == media_path_name
         {
            // Java: path = null;
            // path set to empty in Rust context
         }
         // Java: if (type == MediaTypeEnum.PICTURE)
         if mediaType == MediaTypeEnum::PICTURE
         {
            // Java: id = this.mediaVector.size();
            id = self.mediaVector.as_ref().unwrap().len() as i32;
            // Java: MediaItem item = new MediaItem(id, name, path, zipBaseModel);
            let item = MediaItem::new_with_zip(
                id, name.to_string(),
                if path.is_empty() || path == media_path_name { None } else { Some(path.to_string()) },
                zipBaseModel,
            );
            // Java: this.mediaVector.add(item);
            self.mediaVector.as_mut().unwrap().push(Some(Arc::new(Mutex::new(item))));
         }
         // Java: else if (type == MediaTypeEnum.OLE)
         else if mediaType == MediaTypeEnum::OLE
         {
            // Java: id = this.oleVector.size();
            id = self.oleVector.as_ref().unwrap().len() as i32;
            // Java: MediaItem item = new MediaItem(id, name, path);
            let mut item = MediaItem::new(id, name.to_string(), if path.is_empty() { None } else { Some(path.to_string()) });
            // Java: item.setType(MediaTypeEnum.OLE);
            item.setType(MediaTypeEnum::OLE);
            // Java: this.oleVector.add(item);
            self.oleVector.as_mut().unwrap().push(Some(Arc::new(Mutex::new(item))));
         }
         // Java: return id;
         return id;
      }
    }

    // Java: MediaItem getItem(int id, MediaTypeEnum type)
    pub fn getItem_by_id(&self, id: i32, mediaType: MediaTypeEnum) -> Element {
      // Java: if (id < 0)
      if id < 0
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: MediaItem item = null;
         let mut item: Element = None;
         // Java: if (type == MediaTypeEnum.PICTURE)
         if mediaType == MediaTypeEnum::PICTURE
         {
            // Java: if (this.mediaVector != null && id < this.mediaVector.size())
            if self.mediaVector.is_some() && (id as usize) < self.mediaVector.as_ref().unwrap().len()
            {
               // Java: item = (MediaItem)this.mediaVector.elementAt(id);
               item = self.mediaVector.as_ref().unwrap()[id as usize].clone();
            }
         }
         // Java: else if (type == MediaTypeEnum.OLE && this.oleVector != null && id < this.oleVector.size())
         else if mediaType == MediaTypeEnum::OLE && self.oleVector.is_some() && (id as usize) < self.oleVector.as_ref().unwrap().len()
         {
            // Java: item = (MediaItem)this.oleVector.elementAt(id);
            item = self.oleVector.as_ref().unwrap()[id as usize].clone();
         }
         // Java: return item;
         item
      }
    }

    // Java: String getUncompressedPath(MediaItem item)
    pub fn getUncompressedPath(&self, item: &Element) -> String {
      // Java: if (item == null)
      if item.is_none()
      {
         // Java: return null;
         return String::new();
      }
      // Java: else
      else
      {
         let Some(item) = media_item(item) else { return String::new(); };
         let mut item = item.lock().unwrap();
         // Java: ZipBaseModel zipBaseModel = item.getZipBaseModel();
         let zip_base_model = item.mZipBaseModel.as_ref();
         // Java: if (zipBaseModel == null)
         let Some(zip_base_model) = zip_base_model else { return String::new(); };
         // Java: else
         {
            // Java: File uncompressFile = zipBaseModel.getUncompressFile();
            let file = zip_base_model.lock().unwrap().getUncompressFile_default().clone();
            // Java: return uncompressFile == null ? null : uncompressFile.getAbsolutePath();
            return file.map(|path| if path.is_absolute() { path } else {
                std::env::current_dir().expect("current directory").join(path)
            }.to_string_lossy().into_owned()).unwrap_or_default();
         }
      }
    }

    // Java: String getAllPath(MediaItem item)
    pub fn getAllPath(&self, item: &Element) -> String {
      // Java: return this.getAllPath(item, false);
      self.getAllPath_uncompressed(item, false)
    }

    // Java: String getAllPath(MediaItem item, boolean uncompressed)
    pub fn getAllPath_uncompressed(&self, item: &Element, uncompressed: bool) -> String {
      // Java: if (item == null)
      if item.is_none()
      {
         // Java: return null;
         return String::new();
      }
      // Java: else
      else
      {
         // Java: String path = null;
         let mut path = String::new();
         // Java: if (uncompressed)
         if uncompressed
         {
            // Java: path = this.getUncompressedPath(item);
            path = self.getUncompressedPath(item);
         }
         let Some(media_item) = media_item(item) else { return String::new(); };
         let mut media_item = media_item.lock().unwrap();
         // Java: if (path == null)
         if path.is_empty()
         {
            // Java: path = item.getPath();
            path = media_item.getPath();
         }
         // Java: if (path == null)
         if path.is_empty()
         {
            // Java: String name = item.getName();
            let name = media_item.getName();
            // Java: if (name != null) path = this.mediaPath + name;
            if !name.is_empty() { path = format!("{}{}", self.mediaPath.as_deref().unwrap_or(""), name); }
         }
         // Java: return path;
         path
      }
    }

    // Java: MediaItem getItem(String name, MediaTypeEnum type)
    pub fn getItem_by_name(&self, name: &str, mediaType: MediaTypeEnum) -> Element {
      // Java: MediaItem item = null;
      let mut item: Element = None;
      // Java: if (type == MediaTypeEnum.PICTURE)
      if mediaType == MediaTypeEnum::PICTURE
      {
         // Java: if (this.mediaVector != null)
         if self.mediaVector.is_some()
         {
            // Java: int i = 0;
            let mut i = 0;
            // Java: for(int len = this.mediaVector.size(); i < len; ++i)
            let len = self.mediaVector.as_ref().unwrap().len();
            while i < len
            {
               // Java: MediaItem it = (MediaItem)this.mediaVector.elementAt(i);
               let it = &self.mediaVector.as_ref().unwrap()[i];
               // Java: if (it.getName().equals(name))
               if media_item(it).is_some_and(|item| item.lock().unwrap().getName() == name) {
                  // Java: item = it;
                  item = it.clone();
                  // Java: break;
                  break;
               }
               i += 1;
            }
         }
      }
      // Java: else if (type == MediaTypeEnum.OLE && this.oleVector != null)
      else if mediaType == MediaTypeEnum::OLE && self.oleVector.is_some()
      {
         // Java: int i = 0;
         let mut i = 0;
         // Java: for(int len = this.oleVector.size(); i < len; ++i)
         let len = self.oleVector.as_ref().unwrap().len();
         while i < len
         {
            // Java: MediaItem it = (MediaItem)this.oleVector.elementAt(i);
            let it = &self.oleVector.as_ref().unwrap()[i];
            // Java: if (it.getName().equals(name))
            if media_item(it).is_some_and(|item| item.lock().unwrap().getName() == name) {
               // Java: item = it;
               item = it.clone();
               // Java: break;
               break;
            }
            i += 1;
         }
      }
      // Java: return item;
      item
    }

    // Java: String getMediaByID(int id, MediaTypeEnum type)
    pub fn getMediaByID_impl(&self, id: i32, mediaType: MediaTypeEnum) -> String {
      // Java: MediaItem item = this.getItem(id, type);
      let item = self.getItem_by_id(id, mediaType);
      // Java: return this.getAllPath(item);
      self.getAllPath(&item)
    }

    // Java: String getMediaByName(String name, MediaTypeEnum type)
    pub fn getMediaByName_impl(&self, name: &str, mediaType: MediaTypeEnum) -> String {
      // Java: MediaItem item = this.getItem(name, type);
      let item = self.getItem_by_name(name, mediaType);
      // Java: return this.getAllPath(item);
      self.getAllPath(&item)
    }

    // Java: int getMediaIDByName(String name, MediaTypeEnum type)
    pub fn getMediaIDByName_impl(&self, name: &str, mediaType: MediaTypeEnum) -> i32 {
      // Java: MediaItem item = this.getItem(name, type);
      let item = self.getItem_by_name(name, mediaType);
      // Java: return item != null ? item.getID() : -1;
      media_item(&item).map_or(-1, |item| item.lock().unwrap().getID())
    }

    // Java: int size()
    pub fn size_impl(&self) -> i32 {
      // Java: return this.mediaVector == null ? 0 : this.mediaVector.size() + this.oleVector.size();
      if self.mediaVector.is_none() { 0 } else {
          (self.mediaVector.as_ref().unwrap().len() + self.oleVector.as_ref().map(|v| v.len()).unwrap_or(0)) as i32
      }
    }

    // Java: int mediaSize()
    pub fn mediaSize_impl(&self) -> i32 {
      // Java: return this.mediaVector == null ? 0 : this.mediaVector.size();
      self.mediaVector.as_ref().map(|v| v.len() as i32).unwrap_or(0)
    }

    // Java: int oleSize()
    pub fn oleSize_impl(&self) -> i32 {
      // Java: return this.oleVector == null ? 0 : this.oleVector.size();
      self.oleVector.as_ref().map(|v| v.len() as i32).unwrap_or(0)
    }

    // Java: String getMediaByIndex(int index, MediaTypeEnum type)
    pub fn getMediaByIndex_impl(&self, index: i32, mediaType: MediaTypeEnum) -> String {
      // Java: return this.getMediaByID(index, type);
      self.getMediaByID_impl(index, mediaType)
    }

    // Java: int getMediaIDByIndex(int index, MediaTypeEnum type)
    pub fn getMediaIDByIndex_impl(&self, index: i32, _mediaType: MediaTypeEnum) -> i32 {
      // Java: return index;
      index
    }

    // Java: MediaItem getMediaByID2(int id, MediaTypeEnum type)
    pub fn getMediaByID2_impl(&self, id: i32, mediaType: MediaTypeEnum) -> Element {
      // Java: return this.getItem(id, type);
      self.getItem_by_id(id, mediaType)
    }

}

impl IMediaLib for MediaLib {
    fn registerMediaLibEventListener(&mut self, var1: Box<dyn MediaLibEventListenerTrait>) {
        self.registerMediaLibEventListener_impl(var1);
    }
    fn dispose(&mut self) { self.dispose_impl(); }
    fn addMedia(&mut self, var1: &str, var2: &str, var3: MediaTypeEnum) -> i32 {
        self.addMedia_impl(var1, var2, var3)
    }
    fn addMedia_with_zip(&mut self, var1: &str, var2: &str, var3: MediaTypeEnum, var4: Arc<Mutex<ZipBaseModel>>) -> i32 {
        self.addMedia_with_zip_impl(var1, var2, var3, Some(var4))
    }
    fn addMedia_from_bytes(&mut self, var1: &[u8], var2: MediaTypeEnum) -> i32 {
        self.addMedia_from_bytes_impl(var1, var2)
    }
    fn getMediaByID(&self, var1: i32, var2: MediaTypeEnum) -> String {
        self.getMediaByID_impl(var1, var2)
    }
    fn getMediaByID2(&self, var1: i32, var2: MediaTypeEnum) -> Element {
        self.getMediaByID2_impl(var1, var2)
    }
    fn getMediaByName(&self, var1: &str, var2: MediaTypeEnum) -> String {
        self.getMediaByName_impl(var1, var2)
    }
    fn getMediaIDByName(&self, var1: &str, var2: MediaTypeEnum) -> i32 {
        self.getMediaIDByName_impl(var1, var2)
    }
    fn size(&self) -> i32 { self.size_impl() }
    fn mediaSize(&self) -> i32 { self.mediaSize_impl() }
    fn oleSize(&self) -> i32 { self.oleSize_impl() }
    fn getMediaByIndex(&self, var1: i32, var2: MediaTypeEnum) -> String {
        self.getMediaByIndex_impl(var1, var2)
    }
    fn getMediaIDByIndex(&self, var1: i32, var2: MediaTypeEnum) -> i32 {
        self.getMediaIDByIndex_impl(var1, var2)
    }
}
