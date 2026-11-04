#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

// Java: import android.annotation.SuppressLint
// Java: import android.graphics.BitmapFactory
// Java: import android.graphics.Rect
// Java: import android.graphics.Bitmap.CompressFormat
// Java: import android.graphics.Bitmap.Config
// Java: import android.util.LruCache
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.graphics.Bitmap
// Java: import cn.wps.graphics.BitmapCreator
// Java: import cn.wps.graphics.IMemoryHelper
// Java: import cn.wps.graphics.KBitmap
// Java: import cn.wps.graphics.MetaFileDrawer
// Java: import cn.wps.graphics.PictureInfo
// Java: import cn.wps.graphics.PictureParser
// Java: import cn.wps.graphics.opt.ExportContext
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.util.Pair
// Java: import java.io.BufferedInputStream
// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.util.Collection
// Java: import java.util.HashMap
// Java: import java.util.Map
use std::io;

use crate::cn::wps::graphics::bitmap::BitmapLruManager::BitmapLruManager;

pub struct Bitmap {
    pub width: i32,
    pub height: i32,
}
impl Bitmap {
    pub fn getWidth(&self) -> i32 { self.width }
    pub fn getHeight(&self) -> i32 { self.height }
}

pub struct PictureInfo {
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub pic_type: i32,
    pub byteLen: i32,
}
impl PictureInfo {
    pub fn isMetaFile(&self) -> bool { false }
    pub fn isAndroidFlie(&self) -> bool { false }
}

pub trait IPicture {
    fn getPath(&self) -> &str;
    fn getType(&self) -> i32;
    fn getWidth(&self) -> i32;
    fn getHeight(&self) -> i32;
    fn isSmall(&self) -> bool;
    fn dispose(&mut self);
    fn getDiskCache(&self, w: i32, h: i32) -> Option<i32>;
    fn getPictureInfo(&self) -> Option<&PictureInfo>;
    fn isTooSmall(&self, w: i32, h: i32, ew: i32, eh: i32) -> bool;
}

pub trait IPicturePool {}

pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
}
impl<K: std::hash::Hash + Eq, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self { LruCache { capacity, map: HashMap::new() } }
    pub fn get(&self, key: &K) -> Option<&V> { self.map.get(key) }
    pub fn put(&mut self, key: K, value: V) { self.map.insert(key, value); }
    pub fn evictAll(&mut self) { self.map.clear(); }
}

pub struct MetaFileStruct;
pub struct BitmapCreator;
pub trait IPictureDrawer {}

// Java field: String TAG
pub(crate) const TAG: &str = "PicturePool";
// Java field: int MAX_MEMORY_SIZE
static MAX_MEMORY_SIZE: OnceLock<Mutex<i32>> = OnceLock::new();

fn get_max_memory_size() -> i32 {
    *MAX_MEMORY_SIZE.get_or_init(|| Mutex::new(7340032)).lock().unwrap()
}

// Java: public class PicturePool implements IPicturePool
pub struct PicturePool {
    // Java field: BitmapCreator mBmpCreator
    mBmpCreator: Option<i32>,
    // Java field: LruCache<String, MetaFileDrawer.MetaFileStruct> mRecordsCache
    mRecordsCache: LruCache<String, MetaFileStruct>,
    // Java field: LruCache<String, Bitmap> mTransparentPicCache
    mTransparentPicCache: LruCache<String, Bitmap>,
    // Java field: IPictureMemoryPool mMemoryPool
    pub(crate) mMemoryPool: Option<BitmapLruManager>,
    // Java field: Map<Object, IPicture> picMap
    picMap: Option<HashMap<String, i32>>,
    // Java field: Map<IPicture, Pair<Integer, Integer>> maxDimensionsMap
    maxDimensionsMap: Option<HashMap<String, (i32, i32)>>,
}

// Java: public class PicturePool implements IPicturePool
impl PicturePool {
    // Java: void onOutOfMemory()
    pub fn onOutOfMemory(&mut self) {
      // Java: this.mMemoryPool.onOutOfMemory();
      if let Some(ref mut pool) = self.mMemoryPool {
          pool.base.onOutOfMemory();
      }
      // Java: this.mRecordsCache.evictAll();
      self.mRecordsCache.evictAll();
      // Java: this.mTransparentPicCache.evictAll();
      self.mTransparentPicCache.evictAll();
    }

    // Java: void setMaxMemorySize(int value)
    pub fn setMaxMemorySize(&mut self, value: i32) {
      // Java: this.mMemoryPool.setMaxMemorySize(value);
      if let Some(ref mut pool) = self.mMemoryPool {
          pool.base.setMaxMemorySize(value);
      }
    }

    // Java: int getMaxMemorySize()
    pub fn getMaxMemorySize(&self) -> i32 {
      // Java: return this.mMemoryPool.getMaxMemorySize();
      self.mMemoryPool.as_ref().map_or(0, |p| p.base.getMaxMemorySize())
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: this.clearPicMap();
      self.clearPicMap();
      // Java: this.picMap = null;
      self.picMap = None;
      // Java: if (null != this.mMemoryPool)
      if self.mMemoryPool.is_some()
      {
         // Java: this.mMemoryPool.dispose();
         if let Some(ref mut pool) = self.mMemoryPool {
             pool.dispose();
         }
         // Java: this.mMemoryPool = null;
         self.mMemoryPool = None;
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.clearPicMap();
      self.clearPicMap();
      // Java: if (null != this.mMemoryPool)
      if self.mMemoryPool.is_some()
      {
         // Java: this.mMemoryPool.clear();
         if let Some(ref mut pool) = self.mMemoryPool {
             pool.clear();
         }
      }
    }

    // Java: void clearPicMap()
    pub fn clearPicMap(&mut self) {
      // Java: if (null != this.picMap)
      if self.picMap.is_some()
      {
         // Java: Collection<IPicture> pics = this.picMap.values();
         // Java: if (null != pics)
         {
            // Java: for(IPicture pic : pics)
            {
               // Java: pic.dispose();
            }
         }
         // Java: this.picMap.clear();
         if let Some(ref mut map) = self.picMap {
             map.clear();
         }
      }
      // Java: this.mRecordsCache.evictAll();
      self.mRecordsCache.evictAll();
      // Java: this.mTransparentPicCache.evictAll();
      self.mTransparentPicCache.evictAll();
    }

    // Java: PictureInfo getInfoFromPool(String path)
    pub fn getInfoFromPool(&self, path: &str) -> Option<i32> {
      // Java: IPicture pic = this.getPictureFromPool(path);
      // Java: return null != pic ? pic.getPictureInfo() : null;
      None
    }

    // Java: IPicture getPictureFromPool(Object object)
    pub fn getPictureFromPool(&self, object: &str) -> Option<i32> {
      // Java: if (object == null)
      if object.is_empty()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: IPicture pic = (IPicture)this.picMap.get(object);
         let pic = self.picMap.as_ref().and_then(|m| m.get(object));
         // Java: if (pic != null)
         if pic.is_some()
         {
            // Java: return pic;
            return pic.copied();
         }
         // Java: else
         else
         {
            // Java: if (object instanceof String)
            {
               // Java: String path = (String)object;
               let _path = object;
               // Java: PictureInfo info = PictureParser.getPictureInfo(path);
               // Java: if (info != null)
               {
                  // Java: pic = (IPicture)(info.isMetaFile() ? new MetaPicture(info) : new NormalPicture(info));
               }
            }
            // Java: else if (object instanceof PictureInfo)
            {
               // Java: return this.getPictureFromPool(((PictureInfo)object).path);
            }
            // Java: if (pic != null)
            {
               // Java: this.picMap.put(object, pic);
            }
            // Java: return pic;
            return None;
         }
      }
    }

    // Java: Bitmap getBitmapFromPool(Object obj, int expectedWidth, int expectedHeight, boolean canDistortion, boolean needCreate)
    pub fn getBitmapFromPool_obj(&self, obj: &str, expectedWidth: i32, expectedHeight: i32, canDistortion: bool, needCreate: bool) -> Option<i32> {
      // Java: IPicture pic = this.getPictureFromPool(obj);
      let _pic = self.getPictureFromPool(obj);
      // Java: Bitmap bitmap = this.getBitmapFromPool(pic, expectedWidth, expectedHeight, canDistortion, needCreate);
      // Java: return bitmap;
      None
    }

    // Java: Bitmap getBitmapFromPool(IPicture pic, int expectedWidth, int expectedHeight, boolean canDistortion, boolean needCreate)
    pub fn getBitmapFromPool_pic(&self, _pic: i32, expectedWidth: i32, expectedHeight: i32, canDistortion: bool, needCreate: bool) -> Option<i32> {
      // Java: if (pic != null && pic.getType() != -1)
      if true
      {
         // Java: if (expectedWidth > 0 && expectedHeight > 0)
         if expectedWidth > 0 && expectedHeight > 0
         {
            // Java: Bitmap bitmap = this.mMemoryPool.getBitmap(pic, expectedWidth, expectedHeight, canDistortion, needCreate);
            let mut bitmap: Option<i32> = None;
            // Java: if (this.maxDimensionsMap != null)
            if self.maxDimensionsMap.is_some()
            {
               // Java: Pair<Integer, Integer> currentMax = (Pair)this.maxDimensionsMap.get(pic);
               // Java: if (currentMax == null || expectedWidth > (Integer)currentMax.first && expectedHeight > (Integer)currentMax.second)
               {
                  // Java: this.maxDimensionsMap.put(pic, Pair.create(expectedWidth, expectedHeight));
               }
               // Java: currentMax = (Pair)this.maxDimensionsMap.get(pic);
               // Java: if (ExportContext.isSwitchEnabled() && ExportContext.isPrintModeEnabled() && pic.getPictureInfo() != null && pic.getPictureInfo().isAndroidFlie())
               if false
               {
                  // Java: int inSampleSize = this.defSampleCalc(pic.getPictureInfo().width, pic.getPictureInfo().height, expectedWidth, expectedHeight);
                  let inSampleSize = self.defSampleCalc(0, 0, expectedWidth, expectedHeight);
                  // Java: int resizeSize = ExportContext.isLimitSample() ? this.limitCalcInSampleSize(pic.getPictureInfo().width, pic.getPictureInfo().height, (Integer)currentMax.first, (Integer)currentMax.second) : this.newCalcInSampleSize(pic.getPictureInfo().width, pic.getPictureInfo().height, (Integer)currentMax.first, (Integer)currentMax.second);
                  let resizeSize = self.newCalcInSampleSize(0, 0, expectedWidth, expectedHeight);
                  // Java: if (inSampleSize >= resizeSize && resizeSize > 0)
                  if inSampleSize >= resizeSize && resizeSize > 0
                  {
                     // Java: int bmpWidth = pic.getPictureInfo().width / resizeSize;
                     let bmpWidth = 0 / resizeSize;
                     // Java: int bmpHeight = pic.getPictureInfo().height / resizeSize;
                     let bmpHeight = 0 / resizeSize;
                     // Java: if (bitmap != null && bitmap.getWidth() >= bmpWidth && bitmap.getHeight() >= bmpHeight)
                     if bitmap.is_some()
                     {
                        // Java: return bitmap;
                        return bitmap;
                     }
                     // Java: Bitmap resizeBitmap = this.createBitmap(pic, expectedWidth, expectedHeight);
                     let resizeBitmap: Option<i32> = None;
                     // Java: if (resizeBitmap != null)
                     if resizeBitmap.is_some()
                     {
                        // Java: return resizeBitmap;
                        return resizeBitmap;
                     }
                  }
               }
            }
            // Java: if (bitmap != null)
            if bitmap.is_some()
            {
               // Java: return bitmap;
               return bitmap;
            }
            // Java: else
            else
            {
               // Java: bitmap = this.getBitmapFromDiskCache(pic, expectedWidth, expectedHeight, canDistortion, needCreate);
               // Java: if (bitmap != null)
               if bitmap.is_some()
               {
                  // Java: return bitmap;
                  return bitmap;
               }
               // Java: else
               else
               {
                  // Java: if (needCreate || pic.isSmall())
                  if needCreate
                  {
                     // Java: bitmap = this.createBitmap(pic, expectedWidth, expectedHeight);
                  }
                  // Java: return bitmap;
                  return bitmap;
               }
            }
         }
         // Java: else
         else
         {
            // Java: return null;
            return None;
         }
      }
      // Java: else
      else
      {
         // Java: return null;
         return None;
      }
    }

    // Java: int newCalcInSampleSize(int srcW, int srcH, int reqW, int reqH)
    pub fn newCalcInSampleSize(&self, srcW: i32, srcH: i32, reqW: i32, reqH: i32) -> i32 {
      // Java: float dymMax = 4.0F;
      let mut dymMax: f32 = 4.0;
      // Java: float sx = 1.0F * (float)srcW / (float)reqW;
      let mut sx = 1.0_f32 * srcW as f32 / reqW as f32;
      // Java: float sy = 1.0F * (float)srcH / (float)reqH;
      let mut sy = 1.0_f32 * srcH as f32 / reqH as f32;
      // Java: float max = Math.max(sx, sy);
      let max = sx.max(sy);
      // Java: if (max > 8.0F)
      if max > 8.0_f32
      {
         // Java: dymMax = 4.0F;
         dymMax = 4.0;
      }
      // Java: else
      else
      {
         // Java: dymMax = 8.0F;
         dymMax = 8.0;
      }
      // Java: if (srcW * srcH > 25000000)
      let mut srcW = srcW;
      let mut srcH = srcH;
      if srcW * srcH > 25000000
      {
         // Java: dymMax = 8.0F;
         dymMax = 8.0;
      }
      // Java: if (max > dymMax)
      if max > dymMax
      {
         // Java: float scale = dymMax / max;
         let scale = dymMax / max;
         // Java: srcW = (int)((float)srcW * scale);
         srcW = (srcW as f32 * scale) as i32;
         // Java: srcH = (int)((float)srcH * scale);
         srcH = (srcH as f32 * scale) as i32;
      }
      // Java: if (srcW > 3000 || srcH > 3000)
      if srcW > 3000 || srcH > 3000
      {
         // Java: sx = 1.0F * (float)srcW / 3000.0F;
         sx = 1.0_f32 * srcW as f32 / 3000.0;
         // Java: sy = 1.0F * (float)srcH / 3000.0F;
         sy = 1.0_f32 * srcH as f32 / 3000.0;
         // Java: float scale = Math.max(sx, sy);
         let scale = sx.max(sy);
         // Java: srcW = (int)((float)srcW / scale);
         srcW = (srcW as f32 / scale) as i32;
         // Java: srcH = (int)((float)srcH / scale);
         srcH = (srcH as f32 / scale) as i32;
      }
      // Java: int result = Math.max(1, Math.max(srcW / reqW, srcH / reqH));
      let result = 1.max((srcW / reqW).max(srcH / reqH));
      // Java: if (srcW * srcH < 1474560)
      if srcW * srcH < 1474560
      {
         // Java: if (result == 2)
         if result == 2
         {
            // Java: return 1;
            return 1;
         }
         // Java: if (result == 3 || result == 4)
         if result == 3 || result == 4
         {
            // Java: return 2;
            return 2;
         }
      }
      // Java: return result;
      result
    }

    // Java: int limitCalcInSampleSize(int srcW, int srcH, int reqW, int reqH)
    pub fn limitCalcInSampleSize(&self, srcW: i32, srcH: i32, reqW: i32, reqH: i32) -> i32 {
      // Java: float dymMax = 4.0F;
      let mut dymMax: f32 = 4.0;
      // Java: if (srcW * srcH > 25000000)
      let mut srcW = srcW;
      let mut srcH = srcH;
      if srcW * srcH > 25000000
      {
         // Java: dymMax = 8.0F;
         dymMax = 8.0;
      }
      // Java: float sx = 1.0F * (float)srcW / (float)reqW;
      let mut sx = 1.0_f32 * srcW as f32 / reqW as f32;
      // Java: float sy = 1.0F * (float)srcH / (float)reqH;
      let mut sy = 1.0_f32 * srcH as f32 / reqH as f32;
      // Java: float max = Math.max(sx, sy);
      let max = sx.max(sy);
      // Java: if (max > dymMax)
      if max > dymMax
      {
         // Java: float scale = dymMax / max;
         let scale = dymMax / max;
         // Java: srcW = (int)((float)srcW * scale);
         srcW = (srcW as f32 * scale) as i32;
         // Java: srcH = (int)((float)srcH * scale);
         srcH = (srcH as f32 * scale) as i32;
      }
      // Java: if (srcW > 3000 || srcH > 3000)
      if srcW > 3000 || srcH > 3000
      {
         // Java: sx = 1.0F * (float)srcW / 3000.0F;
         sx = 1.0_f32 * srcW as f32 / 3000.0;
         // Java: sy = 1.0F * (float)srcH / 3000.0F;
         sy = 1.0_f32 * srcH as f32 / 3000.0;
         // Java: float scale = Math.max(sx, sy);
         let scale = sx.max(sy);
         // Java: srcW = (int)((float)srcW / scale);
         srcW = (srcW as f32 / scale) as i32;
         // Java: srcH = (int)((float)srcH / scale);
         srcH = (srcH as f32 / scale) as i32;
      }
      // Java: return Math.max(1, Math.max(srcW / reqW, srcH / reqH));
      1.max((srcW / reqW).max(srcH / reqH))
    }

    // Java: int defSampleCalc(int w, int h, int reqW, int reqH)
    pub fn defSampleCalc(&self, w: i32, h: i32, reqW: i32, reqH: i32) -> i32 {
      // Java: int inSampleSize = Math.max(1, Math.min(w / reqW, h / reqH));
      let mut inSampleSize = 1.max((w / reqW.max(1)).min(h / reqH.max(1)));
      // Java: if (inSampleSize > 2 && inSampleSize % 2 == 1)
      if inSampleSize > 2 && inSampleSize % 2 == 1
      {
         // Java: --inSampleSize;
         inSampleSize -= 1;
      }
      // Java: return inSampleSize;
      inSampleSize
    }

    // Java: Bitmap getBitmapFromPoolError(IPicture pic, int expectedWidth, int expectedHeight)
    pub fn getBitmapFromPoolError(&self, _pic: i32, mut expectedWidth: i32, mut expectedHeight: i32) -> Option<i32> {
      // Java: if (pic != null && pic.getType() != -1)
      if true
      {
         // Java: if (expectedWidth > 0 && expectedHeight > 0)
         if expectedWidth > 0 && expectedHeight > 0
         {
            // Java: boolean autoRemove = true;
            let autoRemove = true;
            // Java: Bitmap bitmap = this.mMemoryPool.getBitmap(pic, expectedWidth, expectedHeight, false, autoRemove);
            let mut bitmap: Option<i32> = None;
            // Java: if (bitmap != null)
            if bitmap.is_some()
            {
               // Java: return bitmap;
               return bitmap;
            }
            // Java: else
            else
            {
               // Java: bitmap = this.getBitmapFromDiskCacheError(pic, expectedWidth, expectedHeight, false, true);
               // Java: if (bitmap != null)
               if bitmap.is_some()
               {
                  // Java: return bitmap;
                  return bitmap;
               }
               // Java: else
               else
               {
                  // Java: while(expectedHeight > 1 && expectedWidth > 1)
                  while expectedHeight > 1 && expectedWidth > 1
                  {
                     // Java: try
                     {
                        // Java: bitmap = this.createBitmapError(pic, expectedWidth, expectedHeight);
                     }
                     // Java: catch (OutOfMemoryError e)
                     {
                        // Java: e.printStackTrace();
                     }
                     // Java: if (bitmap != null)
                     if bitmap.is_some()
                     {
                        // Java: break;
                        break;
                     }
                     // Java: expectedHeight = (int)((float)expectedHeight * 0.8F);
                     expectedHeight = (expectedHeight as f32 * 0.8) as i32;
                     // Java: expectedWidth = (int)((float)expectedWidth * 0.8F);
                     expectedWidth = (expectedWidth as f32 * 0.8) as i32;
                  }
                  // Java: return bitmap;
                  return bitmap;
               }
            }
         }
         // Java: else
         else
         {
            // Java: return null;
            return None;
         }
      }
      // Java: else
      else
      {
         // Java: return null;
         return None;
      }
    }

    // Java: Bitmap getBitmapFromDiskCache(IPicture pic, int expectedWidth, int expectedHeight, boolean canDistortion, boolean needCreate)
    pub fn getBitmapFromDiskCache(&self, _pic: i32, expectedWidth: i32, expectedHeight: i32, canDistortion: bool, needCreate: bool) -> Option<i32> {
      // Java: NormalPicture diskCache = (NormalPicture)pic.getDiskCache(expectedWidth, expectedHeight);
      // Java: if (diskCache == null)
      if true
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: Bitmap bitmap = this.mMemoryPool.getBitmap(diskCache, expectedWidth, expectedHeight, canDistortion, needCreate);
         // Java: if (bitmap != null)
         if false
         {
            // Java: return bitmap;
            return None;
         }
         // Java: else
         else
         {
            // Java: boolean isDiskCacheValid = this.checkDiskCacheValid(diskCache, expectedWidth, expectedHeight, canDistortion);
            // Java: if (!isDiskCacheValid || !needCreate && !diskCache.isSmall())
            if true
            {
               // Java: return null;
               return None;
            }
            // Java: else
            else
            {
               // Java: bitmap = this.createBitmap(diskCache, expectedWidth, expectedHeight);
               // Java: return bitmap;
               return None;
            }
         }
      }
    }

    // Java: Bitmap getBitmapFromDiskCacheError(IPicture pic, int expectedWidth, int expectedHeight, boolean canDistortion, boolean needCreate)
    pub fn getBitmapFromDiskCacheError(&self, _pic: i32, expectedWidth: i32, expectedHeight: i32, canDistortion: bool, needCreate: bool) -> Option<i32> {
      // Java: NormalPicture diskCache = (NormalPicture)pic.getDiskCache(expectedWidth, expectedHeight);
      // Java: if (diskCache == null)
      if true
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: Bitmap bitmap = this.mMemoryPool.getBitmap(diskCache, expectedWidth, expectedHeight, canDistortion, needCreate);
         // Java: return bitmap != null ? bitmap : null;
         return None;
      }
    }

    // Java: boolean checkDiskCacheValid(NormalPicture diskCache, int expectedWidth, int expectedHeight, boolean canDistortion)
    pub fn checkDiskCacheValid(&self, _diskCache: i32, expectedWidth: i32, expectedHeight: i32, canDistortion: bool) -> bool {
      // Java: if (canDistortion)
      if canDistortion
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: boolean isTooSmall = diskCache.isTooSmall(diskCache.getWidth(), diskCache.getHeight(), expectedWidth, expectedHeight);
         let isTooSmall = false;
         // Java: return !isTooSmall;
         return !isTooSmall;
      }
    }

    // Java: Bitmap createBitmap(IPicture pic, int expectedWidth, int expectedHeight)
    pub fn createBitmap(&self, _pic: i32, expectedWidth: i32, expectedHeight: i32) -> Option<i32> {
      // Java: try
      {
         // Java: return this.createBitmapError(pic, expectedWidth, expectedHeight);
         return self.createBitmapError(_pic, expectedWidth, expectedHeight);
      }
      // Java: catch (OutOfMemoryError var5)
      // Java: return null;
    }

    // Java: Bitmap createBitmapError(IPicture pic, int expectedWidth, int expectedHeight)
    pub fn createBitmapError(&self, _pic: i32, expectedWidth: i32, expectedHeight: i32) -> Option<i32> {
      // Java: Bitmap bitmap = null;
      let mut bitmap: Option<i32> = None;
      // Java: int type = pic.getType();
      let pic_type = 0;
      // Java: if (type == 0 || type == 1)
      if pic_type == 0 || pic_type == 1
      {
         // Java: android.graphics.Bitmap aBitmap = this.mBmpCreator.createBitmap(pic.getPictureInfo(), expectedWidth, expectedHeight, this.mMemoryPool);
         // Java: if (aBitmap != null)
         if false
         {
            // Java: bitmap = new KBitmap(aBitmap);
         }
      }
      // Java: if (bitmap == null)
      if bitmap.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: this.mMemoryPool.addPictureToMap(pic, bitmap, expectedWidth, expectedHeight);
         // Java: return bitmap;
         return bitmap;
      }
    }

    // Java: PicturePool()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.mBmpCreator = new BitmapCreator(new IMemoryHelper()
      {
         // Java: public void onOutOfMemory()
         {
            // Java: if (null != PicturePool.this.mMemoryPool)
            {
               // Java: PicturePool.this.mMemoryPool.onOutOfMemory();
            }
         }
         // Java: public void gcForNewObject(int size)
         {
            // Java: if (null != PicturePool.this.mMemoryPool)
            {
               // Java: PicturePool.this.mMemoryPool.check(size);
            }
         }
      }
      // Java: this.mBmpCreator.setConfig(Config.ARGB_8888);
      PicturePool {
          mBmpCreator: None,
          mRecordsCache: LruCache::new(200),
          mTransparentPicCache: LruCache::new(10),
          mMemoryPool: Some(BitmapLruManager::new()),
          picMap: Some(HashMap::new()),
          maxDimensionsMap: Some(HashMap::new()),
      }
    }

    // Java: void clearMemory()
    pub fn clearMemory(&mut self) {
      // Java: this.mMemoryPool.clearMemory();
      if let Some(ref mut pool) = self.mMemoryPool {
          pool.base.clearMemory();
      }
    }

    // Java: String createNewPath(IPicture pic, Bitmap bitmap, boolean bNormal)
    pub fn createNewPath(&self, pic_path: &str, bitmap_w: i32, bitmap_h: i32, pic_type: i32, bNormal: bool) -> Option<String> {
      // Java: String newPath = null;
      let mut newPath: Option<String> = None;
      // Java: int type = pic.getType();
      let type_val = pic_type;
      // Java: if (type == 1 || bNormal)
      if type_val == 1 || bNormal
      {
         // Java: int w = bitmap.getWidth();
         let w = bitmap_w;
         // Java: int h = bitmap.getHeight();
         let h = bitmap_h;
         // Java: newPath = pic.getPath() + "_" + w + "x" + h + ".png";
         newPath = Some(format!("{}_{w}x{h}.png", pic_path));
      }
      // Java: return newPath;
      newPath
    }

    // Java: IPictureDrawer createPictureDrawer()
    pub fn createPictureDrawer(&self) -> i32 {
      // Java: return new MetaFileDrawer(this.mRecordsCache);
      0
    }

    // Java: Bitmap getTransparentBitmap(IPicture pic, int transClr, int width, int height)
    pub fn getTransparentBitmap(&mut self, pic_path: &str, transClr: i32, width: i32, height: i32) -> Option<i32> {
      // Java: if (pic == null)
      if pic_path.is_empty()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: String key = pic.getPath() + transClr;
         let key = format!("{}{}", pic_path, transClr);
         // Java: Bitmap bitmap = (Bitmap)this.mTransparentPicCache.get(key);
         let cached = self.mTransparentPicCache.get(&key);
         // Java: if (bitmap != null && (float)bitmap.getWidth() > (float)width * 0.8F && (float)bitmap.getWidth() < (float)width * 1.2F)
         if cached.map_or(false, |b| b.getWidth() as f32 > width as f32 * 0.8 && (b.getWidth() as f32) < width as f32 * 1.2)
         {
            // Java: return bitmap;
            return Some(0);
         }
         // Java: else
         else
         {
            // Java: Bitmap iBitmapSrc = this.getBitmapFromPoolError(pic, width, height);
            let iBitmapSrc = self.getBitmapFromPoolError(0, width, height);
            // Java: if (iBitmapSrc == null)
            if iBitmapSrc.is_none()
            {
               // Java: return null;
               return None;
            }
            // Java: else
            else
            {
               // Java: android.graphics.Bitmap bitmapSrc = iBitmapSrc != null ? ((KBitmap)iBitmapSrc).getBitmap() : null;
               // Java: if (bitmapSrc == null)
               if false
               {
                  // Java: return null;
                  return None;
               }
               // Java: else
               else
               {
                  // Java: int bw = bitmapSrc.getWidth();
                  let bw = width;
                  // Java: int bh = bitmapSrc.getHeight();
                  let bh = height;
                  // Java: int[] pixels = new int[bw];
                  let mut pixels = vec![0i32; bw as usize];
                  // Java: android.graphics.Bitmap newBitmap = android.graphics.Bitmap.createBitmap(bw, bh, Config.ARGB_8888);
                  // Java: int tolerance = 10;
                  let mut tolerance = 10;
                  // Java: int type = pic.getPictureInfo().type;
                  let pic_type = 0;
                  // Java: if (type == 2)
                  if pic_type == 2
                  {
                     // Java: tolerance = 900;
                     tolerance = 900;
                  }
                  // Java: else if (type == 7)
                  else if pic_type == 7
                  {
                     // Java: tolerance = 40;
                     tolerance = 40;
                  }
                  // Java: int r = transClr >> 16 & 255;
                  let r = (transClr >> 16) & 255;
                  // Java: int g = transClr >> 8 & 255;
                  let g = (transClr >> 8) & 255;
                  // Java: int b = transClr & 255;
                  let b = transClr & 255;
                  // Java: for(int i = 0; i < bh; ++i)
                  for i in 0..bh
                  {
                     // Java: bitmapSrc.getPixels(pixels, 0, bw, 0, i, bw, 1);
                     // Java: for(int j = 0; j < bw; ++j)
                     for j in 0..bw as usize
                     {
                        // Java: int p = pixels[j];
                        let p = pixels[j];
                        // Java: int rp = p >> 16 & 255;
                        let rp = (p >> 16) & 255;
                        // Java: int gp = p >> 8 & 255;
                        let gp = (p >> 8) & 255;
                        // Java: int bp = p & 255;
                        let bp = p & 255;
                        // Java: int dr = r - rp;
                        let dr = r - rp;
                        // Java: int dg = g - gp;
                        let dg = g - gp;
                        // Java: int db = b - bp;
                        let db = b - bp;
                        // Java: float rmean = (float)((r + rp) / 2);
                        let rmean = ((r + rp) / 2) as f32;
                        // Java: if ((2.0F + rmean / 256.0F) * (float)dr * (float)dr + (float)(4 * dg * dg) + (2.0F + (255.0F - rmean) / 256.0F) * (float)db * (float)db < (float)tolerance)
                        let color_dist = (2.0_f32 + rmean / 256.0) * dr as f32 * dr as f32 + (4 * dg * dg) as f32 + (2.0_f32 + (255.0 - rmean) / 256.0) * db as f32 * db as f32;
                        if color_dist < tolerance as f32
                        {
                           // Java: pixels[j] = 0;
                           pixels[j] = 0;
                        }
                     }
                     // Java: newBitmap.setPixels(pixels, 0, bw, 0, i, bw, 1);
                  }
                  // Java: bitmap = new KBitmap(newBitmap);
                  // Java: this.mTransparentPicCache.put(key, bitmap);
                  self.mTransparentPicCache.put(key, Bitmap { width: bw, height: bh });
                  // Java: return bitmap;
                  return Some(0);
               }
            }
         }
      }
    }

    // Java: static String saveToJpeg(String src)
    pub fn saveToJpeg(src: &str) -> Option<String> {
      // Java: try
      {
         // Java: File file = new File(src);
         let file_path = std::path::Path::new(src);
         // Java: if (file.exists())
         if file_path.exists()
         {
            // Java: InputStream stream = new FileInputStream(file);
            // Java: BufferedInputStream fileStream = new BufferedInputStream(stream);
            // Java: android.graphics.Bitmap bitmap = BitmapFactory.decodeStream(fileStream, (Rect)null, (BitmapFactory.Options)null);
            // Java: if (bitmap != null)
            if false
            {
               // Java: String dst = Platform.getCurrentPidTmpDir() + File.separator + System.currentTimeMillis() + ".jpg";
               let dst = format!("{}.jpg", src);
               // Java: File outFile = new File(dst);
               // Java: if (!outFile.exists())
               {
                  // Java: outFile.createNewFile();
               }
               // Java: OutputStream outStream = new FileOutputStream(outFile);
               // Java: bitmap.compress(CompressFormat.JPEG, 50, outStream);
               // Java: return dst;
               return Some(dst);
            }
         }
      }
      // Java: catch (FileNotFoundException e)
      {
         // Java: e.printStackTrace();
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
      }
      // Java: return null;
      None
    }

}

impl IPicturePool for PicturePool {
}
