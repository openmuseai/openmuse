#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

use std::collections::HashMap;

// Java: import android.graphics.Canvas
// Java: import android.graphics.Paint
// Java: import android.graphics.Path
// Java: import android.graphics.Rect
// Java: import android.graphics.RectF
// Java: import android.graphics.Path.Direction
// Java: import android.util.LruCache
// Java: import cn.wps.graphics.bitmap.IPicture
// Java: import cn.wps.graphics.bitmap.IPictureDrawer
// Java: import cn.wps.graphics.data.MetaFileHeader
use crate::cn::wps::graphics::data::MetaFileHeader::MetaFileHeader;
// Java: import cn.wps.graphics.data.MetaFileRecord
use crate::cn::wps::graphics::data::MetaFileRecord::MetaFileRecord;
// Java: import cn.wps.graphics.data.wmf.WMFPictureInfo
use crate::cn::wps::graphics::data::wmf::WMFPictureInfo::WMFPictureInfo;
// Java: import cn.wps.graphics.gdi.MetaFilePainter
use crate::cn::wps::graphics::gdi::MetaFilePainter::MetaFilePainter;
// Java: import java.util.List

pub struct Canvas;
pub struct Paint;
pub struct Path;
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
pub struct RectF {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl RectF {
    pub fn width(&self) -> f32 { self.right - self.left }
    pub fn height(&self) -> f32 { self.bottom - self.top }
}

pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<String, V>,
    _k: std::marker::PhantomData<K>,
}
impl<K, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        LruCache { capacity, map: HashMap::new(), _k: std::marker::PhantomData }
    }
    pub fn get(&self, key: &str) -> Option<&V> { self.map.get(key) }
    pub fn put(&mut self, key: String, value: V) { self.map.insert(key, value); }
}

pub trait IPicture {
    fn getPictureInfo(&self) -> &PictureInfo;
}

pub trait IPictureDrawer {
    fn drawBitmap_trait(&self, canvas: i32, pic: i32, paint: i32, display_rect: i32, clip: bool) -> bool;
}

pub struct PictureInfo {
    pub path: Option<String>,
    pub width: i32,
    pub height: i32,
    pub pic_type: i32,
}
impl PictureInfo {
    pub fn isMetaFile(&self) -> bool {
        self.pic_type == 5 || self.pic_type == 6
    }
}

// Java field: String TAG
pub(crate) const TAG: &str = "MetaFileDrawer";

// Java: public class MetaFileDrawer implements IPictureDrawer
pub struct MetaFileDrawer {
    // Java field: LruCache<String, MetaFileStruct> mRecordsCache
    mRecordsCache: LruCache<String, MetaFileStruct>,
    // Java field: float[] mValues
    mValues: Vec<f32>,
    // Java field: Rect mBounds
    mBounds: Rect,
    // Java field: Path mClipPath
    mClipPath: Path,
}

// Java: public class MetaFileStruct
pub struct MetaFileStruct {
    // Java field: MetaFileHeader mHeader
    pub(crate) mHeader: i32,
    // Java field: List<MetaFileRecord> mListRecord
    pub(crate) mListRecord: Vec<MetaFileRecord>,
}

// Java: public class MetaFileDrawer implements IPictureDrawer
impl MetaFileDrawer {
    // Java: MetaFileDrawer()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.mRecordsCache = new LruCache(200);
      MetaFileDrawer {
          mRecordsCache: LruCache::new(200),
          mValues: vec![0.0; 9],
          mBounds: Rect { left: 0, top: 0, right: 0, bottom: 0 },
          mClipPath: Path,
      }
    }

    // Java: MetaFileDrawer(LruCache<String, MetaFileStruct> recordsCache)
    pub fn new_with_cache(recordsCache: LruCache<String, MetaFileStruct>) -> Self {
      // Java: super();
      // Java: this.mRecordsCache = recordsCache;
      MetaFileDrawer {
          mRecordsCache: recordsCache,
          mValues: vec![0.0; 9],
          mBounds: Rect { left: 0, top: 0, right: 0, bottom: 0 },
          mClipPath: Path,
      }
    }

    // Java: boolean drawBitmap(Object canvas, IPicture pic, Object paint, Object displayRectF, boolean bClip)
    pub fn drawBitmap_pic(&self, canvas: i32, pic: &dyn IPicture, paint: i32, displayRectF: i32, bClip: bool) -> bool {
      // Java: return this.drawBitmap(canvas, pic.getPictureInfo(), paint, displayRectF, bClip);
      self.drawBitmap_info(pic.getPictureInfo(), bClip)
    }

    // Java: boolean drawBitmap(Object canvas, PictureInfo info, Object paint, Object displayRectF)
    pub fn drawBitmap_no_clip(&self, info: &PictureInfo) -> bool {
      // Java: return this.drawBitmap(canvas, info, paint, displayRectF, true);
      self.drawBitmap_info(info, true)
    }

    // Java: boolean drawBitmap(Object canvas, PictureInfo info, Object paint, Object displayRectF, boolean bClip)
    pub fn drawBitmap_info(&self, info: &PictureInfo, bClip: bool) -> bool {
      // Java: if (info instanceof WMFPictureInfo)
      {
         // Java: PictureInfo embedInfo = ((WMFPictureInfo)info).getEmbedInfo();
         // Java: if (embedInfo != null)
         {
            // Java: info = embedInfo;
         }
      }
      // Java: String path = info.path;
      let path = &info.path;
      // Java: if (path != null && info.isMetaFile())
      if path.is_some() && info.isMetaFile()
      {
         // Java: MetaFilePainter renderer = this.getMetaPainter(info, path);
         let _renderer = self.getMetaPainter(info, path.as_ref().unwrap());
         // Java: if (info instanceof WMFPictureInfo)
         {
            // Java: PictureInfo embedInfo = ((WMFPictureInfo)info).getEmbedInfo();
            // Java: if (embedInfo != null)
            {
               // Java: info = embedInfo;
            }
         }
         // Java: return this.fastDraw((Canvas)canvas, renderer, info, (Paint)paint, (RectF)displayRectF, bClip);
         return self.fastDraw(_renderer, info, bClip);
      }
      // Java: else
      else
      {
         // Java: return false;
         return false;
      }
    }

    // Java: MetaFilePainter getMetaPainter(PictureInfo info, String path)
    pub fn getMetaPainter(&self, info: &PictureInfo, path: &str) -> Option<i32> {
      // Java: MetaFileStruct metaRecords = (MetaFileStruct)this.mRecordsCache.get(path);
      let metaRecords = self.mRecordsCache.get(path);
      // Java: if (null != metaRecords)
      if metaRecords.is_some()
      {
         // Java: return new MetaFilePainter(metaRecords.getHeader(), metaRecords.getListRecord());
         return Some(0);
      }
      // Java: else
      else
      {
         // Java: MetaFilePainter renderer = null;
         let mut renderer: Option<i32> = None;
         // Java: try
         {
            // Java: renderer = MetaFilePainter.getMetaFilePainter(info, path);
            let r = MetaFilePainter::getMetaFilePainter(0, path);
            renderer = Some(r);
            // Java: if (renderer == null)
            if renderer.is_none()
            {
               // Java: return null;
               return None;
            }
            // Java: MetaFileHeader header = renderer.getHeader();
            // Java: List<MetaFileRecord> listRecord = renderer.getRecords();
            // Java: if (listRecord.size() < 1000)
            {
               // Java: this.mRecordsCache.put(path, new MetaFileStruct(header, listRecord));
            }
         }
         // Java: catch (OutOfMemoryError var7)
         {
         }
         // Java: catch (Exception var8)
         {
         }
         // Java: return renderer;
         renderer
      }
    }

    // Java: boolean fastDraw(Canvas canvas, MetaFilePainter renderer, PictureInfo info, Paint paint, RectF displayRectF, boolean bClip)
    fn fastDraw(&self, renderer: Option<i32>, info: &PictureInfo, bClip: bool) -> bool {
      // Java: if (null == renderer)
      if renderer.is_none()
      {
         // Java: return false;
         return false;
      }
      // Java: else
      else
      {
         // Java: boolean drawSuccess = false;
         let mut drawSuccess = false;
         // Java: try
         {
            // Java: canvas.save();
            // Java: canvas.translate(displayRectF.left, displayRectF.top);
            // Java: canvas.scale(displayRectF.width() / (float)info.width, displayRectF.height() / (float)info.height);
            // Java: canvas.getClipBounds(this.mBounds);
            // Java: canvas.getMatrix().getValues(this.mValues);
            // Java: if (bClip)
            if bClip
            {
               // Java: float scaleX = this.mValues[0];
               let scaleX = self.mValues[0];
               // Java: float scaleY = this.mValues[4];
               let scaleY = self.mValues[4];
               // Java: this.mClipPath.reset();
               // Java: this.mClipPath.addRect((float)this.mBounds.left / scaleX, (float)this.mBounds.top / scaleY, (float)this.mBounds.right / scaleX, (float)this.mBounds.bottom / scaleY, Direction.CW);
               // Java: renderer.paint(canvas, this.mClipPath, paint, (float)info.width, (float)info.height);
            }
            // Java: else
            else
            {
               // Java: renderer.paint(canvas, (Path)null, paint, (float)info.width, (float)info.height);
            }
            // Java: canvas.restore();
            // Java: drawSuccess = true;
            drawSuccess = true;
         }
         // Java: catch (OutOfMemoryError var10)
         {
         }
         // Java: catch (Exception var11)
         {
         }
         // Java: return drawSuccess;
         drawSuccess
      }
    }

}

// Java: public class MetaFileStruct
impl MetaFileStruct {
    pub fn new(header: i32, list_record: Vec<MetaFileRecord>) -> Self {
        MetaFileStruct { mHeader: header, mListRecord: list_record }
    }

    // Java: MetaFileHeader getHeader()
    pub fn getHeader(&self) -> i32 {
         // Java: return this.mHeader;
         self.mHeader
    }

    // Java: List<MetaFileRecord> getListRecord()
    pub fn getListRecord(&self) -> &Vec<MetaFileRecord> {
         // Java: return this.mListRecord;
         &self.mListRecord
    }

}

impl IPictureDrawer for MetaFileDrawer {
    fn drawBitmap_trait(&self, _canvas: i32, _pic: i32, _paint: i32, _display_rect: i32, _clip: bool) -> bool {
        false
    }
}
