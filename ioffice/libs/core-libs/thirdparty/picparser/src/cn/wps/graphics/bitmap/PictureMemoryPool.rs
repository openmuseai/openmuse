#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

use std::collections::LinkedList;

// Java: import cn.wps.base.log.Log
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.graphics.Bitmap
// Java: import cn.wps.graphics.PictureInfo
// Java: import cn.wps.graphics.Bitmap.CompressFormat
// Java: import cn.wps.graphics.Bitmap.FromType
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException
use std::io;
// Java: import java.util.Iterator
// Java: import java.util.LinkedList
// Java: import java.util.List

use crate::cn::wps::graphics::bitmap::IPictureMemoryPool::IPictureMemoryPool;

pub struct Bitmap {
    pub width: i32,
    pub height: i32,
    pub recycled: bool,
}
impl Bitmap {
    pub fn getWidth(&self) -> i32 { self.width }
    pub fn getHeight(&self) -> i32 { self.height }
    pub fn getAllocationByteCount(&self) -> i32 { self.width * self.height * 4 }
    pub fn isRecycled(&self) -> bool { self.recycled }
    pub fn isMaxSize(&self) -> bool { false }
    pub fn recycle(&mut self) { self.recycled = true; }
    pub fn createScaleBitmap(&mut self, _w: i32, _h: i32) -> Bitmap { Bitmap { width: _w, height: _h, recycled: false } }
    pub fn setFromType(&mut self, _ft: i32) {}
    pub fn compress(&self, _fmt: i32, _quality: i32, _out: &mut Vec<u8>) -> bool { true }
}

pub trait IPicture {
    fn getPath(&self) -> &str;
    fn isFixedSize(&self) -> bool;
    fn isTooSmall(&self, bmpW: i32, bmpH: i32, expW: i32, expH: i32) -> bool;
    fn setDiskCache(&mut self, cache: i32);
}

pub trait IBitmapRef {
    fn get(&self) -> Option<&Bitmap>;
    fn getPicture(&self) -> i32;
}

pub struct BitmapRef;
pub struct BitmapRefNode {
    pub bmpRef: BitmapRef,
}
pub struct NormalPicture;
pub struct PictureInfo;

// Java field: String TAG
pub(crate) const TAG: &str = "PictureMemoryPool";

// Java: public class PictureMemoryPool implements IPictureMemoryPool
pub struct PictureMemoryPool {
    // Java field: int maxMemorySize
    pub(crate) maxMemorySize: i32,
    // Java field: int currentBmpSize
    pub(crate) currentBmpSize: i32,
    // Java field: List<Bitmap> reuseBitmap
    pub(crate) reuseBitmap: Vec<Bitmap>,
    // Java field: int reuseBmpSize
    pub(crate) reuseBmpSize: i32,
}

// Java: public class PictureMemoryPool implements IPictureMemoryPool
impl PictureMemoryPool {
    // Java: abstract abstract BitmapRefNode getBitmapRefNode(String var1)
    pub fn getBitmapRefNode(&self, _var1: &str) -> Option<&BitmapRefNode> {
        None
    }

    // Java: abstract abstract boolean add(String var1, BitmapRefNode var2, int var3)
    pub fn add(&mut self, _var1: &str, _var2: BitmapRefNode, _var3: i32) -> bool {
        false
    }

    // Java: abstract abstract boolean remove(BitmapRefNode var1, boolean var2)
    pub fn remove(&mut self, _var1: &BitmapRefNode, _var2: bool) -> bool {
        false
    }

    // Java: PictureMemoryPool()
    pub fn new() -> Self {
      // Java: super();
      PictureMemoryPool {
          maxMemorySize: 7340032,
          currentBmpSize: 0,
          reuseBitmap: Vec::new(),
          reuseBmpSize: 0,
      }
    }

    // Java: void onOutOfMemory()
    pub fn onOutOfMemory(&mut self) {
      // Java: this.maxMemorySize = (int)((double)this.maxMemorySize * 0.8);
      self.maxMemorySize = (self.maxMemorySize as f64 * 0.8) as i32;
      // Java: this.check(0);
      self.check(0);
      // Java: this.reuseBitmap.clear();
      self.reuseBitmap.clear();
      // Java: this.reuseBmpSize = 0;
      self.reuseBmpSize = 0;
    }

    // Java: void clearMemory()
    pub fn clearMemory(&mut self) {
      // Java: this.check(this.maxMemorySize);
      let max = self.maxMemorySize;
      self.check(max);
    }

    // Java: void setMaxMemorySize(int value)
    pub fn setMaxMemorySize(&mut self, value: i32) {
      // Java: this.maxMemorySize = value;
      self.maxMemorySize = value;
      // Java: this.check(0);
      self.check(0);
    }

    // Java: int getMaxMemorySize()
    pub fn getMaxMemorySize(&self) -> i32 {
      // Java: return this.maxMemorySize;
      self.maxMemorySize
    }

    // Java: BitmapRefNode createBitmapRefNode(IPicture pic, Bitmap bitmap)
    fn createBitmapRefNode(&self, _pic: i32, _bitmap: &Bitmap) -> BitmapRefNode {
      // Java: return new BitmapRefNode(new BitmapRef(bitmap, pic));
      BitmapRefNode { bmpRef: BitmapRef }
    }

    // Java: boolean addPictureToMap(IPicture pic, Bitmap bitmap, int expectedWidth, int expectedHeight)
    pub fn addPictureToMap_impl(&mut self, _pic: i32, _bitmap: i32, expectedWidth: i32, expectedHeight: i32) -> bool {
      // Java: if (pic != null && bitmap != null)
      if true
      {
         // Java: int bmpSize = bitmap.getWidth() * bitmap.getHeight() * 4;
         let mut _bmpSize = expectedWidth * expectedHeight * 4;
         // Java: if (!this.check(bmpSize))
         if !self.check(_bmpSize)
         {
            // Java: int dstWidth = expectedWidth;
            let mut dstWidth = expectedWidth;
            // Java: int dstHeight = expectedHeight;
            let mut dstHeight = expectedHeight;
            // Java: if (!this.check(expectedWidth * expectedHeight * 4))
            if !self.check(expectedWidth * expectedHeight * 4)
            {
               // Java: float rate = (float)Math.sqrt((double)(0.5F * (float)this.getMaxMemorySize() / (float)bmpSize));
               let rate = (0.5_f32 * self.getMaxMemorySize() as f32 / _bmpSize as f32).sqrt();
               // Java: dstWidth = (int)((float)bitmap.getWidth() * rate);
               dstWidth = (expectedWidth as f32 * rate) as i32;
               // Java: dstHeight = (int)((float)bitmap.getHeight() * rate);
               dstHeight = (expectedHeight as f32 * rate) as i32;
            }
            // Java: bitmap = bitmap.createScaleBitmap(dstWidth, dstHeight);
            // Java: bmpSize = dstWidth * dstHeight * 4;
            _bmpSize = dstWidth * dstHeight * 4;
         }
         // Java: BitmapRefNode node = this.createBitmapRefNode(pic, bitmap);
         // Java: return this.add(pic.getPath(), node, bitmap.getAllocationByteCount());
         return false;
      }
      // Java: else
      else
      {
         // Java: return false;
         return false;
      }
    }

    // Java: Bitmap getBitmap(IPicture pic, int expectedWidth, int expectedHeight, boolean canDistortion, boolean autoRemove)
    pub fn getBitmap_impl(&self, _pic: i32, expectedWidth: i32, expectedHeight: i32, canDistortion: bool, autoRemove: bool) -> Option<i32> {
      // Java: if (expectedWidth > 0 && expectedHeight > 0)
      if expectedWidth > 0 && expectedHeight > 0
      {
         // Java: if (pic == null)
         if false
         {
            // Java: return null;
            return None;
         }
         // Java: else
         else
         {
            // Java: BitmapRefNode bmpNode = this.getBitmapRefNode(pic.getPath());
            // Java: Bitmap bitmap = this.checkBmpSize(bmpNode, pic, expectedWidth, expectedHeight, canDistortion, autoRemove);
            // Java: if (null != bitmap)
            {
               // Java: bitmap.setFromType(FromType.MEMORY);
            }
            // Java: return bitmap;
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

    // Java: Bitmap checkBmpSize(BitmapRefNode bmpNode, IPicture pic, int expectedWidth, int expectedHeight, boolean canDistortion, boolean autoRemove)
    fn checkBmpSize(&self, bmpNode: Option<&BitmapRefNode>, _pic: i32, expectedWidth: i32, expectedHeight: i32, canDistortion: bool, autoRemove: bool) -> Option<i32> {
      // Java: if (null == bmpNode)
      if bmpNode.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: Bitmap bitmap = bmpNode.bmpRef.get();
         // Java: if (canDistortion)
         if canDistortion
         {
            // Java: return bitmap;
            return None;
         }
         // Java: else if (pic.isFixedSize())
         else if false
         {
            // Java: return bitmap;
            return None;
         }
         // Java: else
         else
         {
            // Java: if (!bitmap.isMaxSize() && pic.isTooSmall(bitmap.getWidth(), bitmap.getHeight(), expectedWidth, expectedHeight))
            {
               // Java: if (autoRemove)
               if autoRemove
               {
                  // Java: this.remove(bmpNode, false);
               }
               // Java: bitmap = null;
            }
            // Java: return bitmap;
            return None;
         }
      }
    }

    // Java: int getLoaction(List<Bitmap> reuse, int size)
    pub fn getLoaction(&self, reuse: &Vec<Bitmap>, size: i32) -> i32 {
      // Java: int loc = 0;
      let mut loc: i32 = 0;
      // Java: for(Bitmap bitmap : reuse)
      for bitmap in reuse.iter()
      {
         // Java: if (bitmap.getAllocationByteCount() >= size)
         if bitmap.getAllocationByteCount() >= size
         {
            // Java: break;
            break;
         }
         // Java: ++loc;
         loc += 1;
      }
      // Java: return loc;
      loc
    }

    // Java: Bitmap getReuseBitmap(int width, int height, boolean bExact)
    pub fn getReuseBitmap_impl(&mut self, width: i32, height: i32, bExact: bool) -> Option<Bitmap> {
      // Java: int size = width * height * 4;
      let size = width * height * 4;
      // Java: if (this.reuseBmpSize < size)
      if self.reuseBmpSize < size
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: Iterator<Bitmap> it = this.reuseBitmap.iterator();
         let mut idx = 0usize;
         // Java: while(true)
         loop
         {
            // Java: if (it.hasNext())
            if idx < self.reuseBitmap.len()
            {
               // Java: Bitmap bitmap = (Bitmap)it.next();
               let bitmap = &self.reuseBitmap[idx];
               // Java: if (bitmap.getAllocationByteCount() < size)
               if bitmap.getAllocationByteCount() < size
               {
                  // Java: continue;
                  idx += 1;
                  continue;
               }
               // Java: if (!bExact || bExact && bitmap.getWidth() == width && bitmap.getHeight() == height)
               if !bExact || (bExact && bitmap.getWidth() == width && bitmap.getHeight() == height)
               {
                  // Java: this.reuseBitmap.remove(bitmap);
                  let removed = self.reuseBitmap.remove(idx);
                  // Java: this.reuseBmpSize -= bitmap.getAllocationByteCount();
                  self.reuseBmpSize -= removed.getAllocationByteCount();
                  // Java: return bitmap;
                  return Some(removed);
               }
               idx += 1;
            } else {
                break;
            }
            // Java: return null;
         }
         return None;
      }
    }

    // Java: boolean recycleBmp(IBitmapRef ref, boolean addDiskCache)
    pub fn recycleBmp(&mut self, _bitmapRef: i32, addDiskCache: bool) -> bool {
      // Java: boolean result = true;
      let mut result = true;
      // Java: Bitmap bitmap = ref.get();
      // Java: if (bitmap != null && !bitmap.isRecycled())
      if true
      {
         // Java: synchronized(bitmap)
         {
            // Java: if (addDiskCache)
            if addDiskCache
            {
               // Java: result = this.addDiskCache(ref.getPicture(), bitmap);
               result = self.addDiskCache(0, 0);
            }
            // Java: int bmpSize = bitmap.getAllocationByteCount();
            let bmpSize: i32 = 0;
            // Java: if (this.currentBmpSize + this.reuseBmpSize < this.maxMemorySize)
            if self.currentBmpSize + self.reuseBmpSize < self.maxMemorySize
            {
               // Java: int size = bitmap.getAllocationByteCount();
               let _size: i32 = 0;
               // Java: int loc = this.getLoaction(this.reuseBitmap, size);
               let _loc = {
                   let mut loc = 0;
                   for bmp in self.reuseBitmap.iter() {
                       if bmp.getAllocationByteCount() >= _size { break; }
                       loc += 1;
                   }
                   loc
               };
               // Java: this.reuseBitmap.add(loc, bitmap);
               // Java: this.reuseBmpSize += size;
               self.reuseBmpSize += _size;
            }
            // Java: else
            else
            {
               // Java: bitmap.recycle();
               // Java: Bitmap var11 = null;
            }
            // Java: this.currentBmpSize -= bmpSize;
            self.currentBmpSize -= bmpSize;
         }
      }
      // Java: return result;
      result
    }

    // Java: boolean addDiskCache(IPicture pic, Bitmap bitmap)
    fn addDiskCache(&self, _pic: i32, _bitmap: i32) -> bool {
      // Java: String newPath = Platform.getPicturePool().createNewPath(pic, bitmap, false);
      let newPath: Option<String> = None;
      // Java: if (newPath == null)
      if newPath.is_none()
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: boolean result = this.saveBitmap(bitmap, newPath);
         let result = self.saveBitmap(newPath.as_ref().unwrap());
         // Java: if (result)
         if result
         {
         }
         // Java: File file = new File(newPath);
         // Java: PictureInfo info = new PictureInfo(newPath, 1, bitmap.getWidth(), bitmap.getHeight(), (int)file.length());
         // Java: NormalPicture diskCache = new NormalPicture(info);
         // Java: pic.setDiskCache(diskCache);
         // Java: return result;
         result
      }
    }

    // Java: boolean saveBitmap(Bitmap bitmap, String bitName)
    fn saveBitmap(&self, bitName: &str) -> bool {
      // Java: try
      {
         // Java: FileOutputStream fOut = null;
         let mut fOut: Option<Vec<u8>> = None;
         // Java: try
         {
            // Java: fOut = new FileOutputStream(bitName);
            fOut = Some(Vec::new());
         }
         // Java: catch (FileNotFoundException e)
         if fOut.is_none()
         {
            // Java: Log.e(TAG, "FileNotFoundException", e);
         }
         // Java: if (null == fOut)
         if fOut.is_none()
         {
            // Java: return false;
            return false;
         }
         // Java: else
         else
         {
            // Java: bitmap.compress(CompressFormat.PNG, 100, fOut);
            // Java: fOut.close();
            // Java: return true;
            return true;
         }
      }
      // Java: catch (IOException e)
      // Java: Log.e(TAG, "IOException", e);
      // Java: return false;
    }

    fn check(&mut self, _size: i32) -> bool {
        self.currentBmpSize + _size <= self.maxMemorySize
    }
}

impl IPictureMemoryPool for PictureMemoryPool {
    fn onOutOfMemory(&self) {}
    fn check(&self, _var1: i32) -> bool { true }
    fn check_with_flag(&self, _var1: i32, _var2: bool) -> bool { true }
    fn clearMemory(&self) {}
    fn setMaxMemorySize(&mut self, _var1: i32) {}
    fn getMaxMemorySize(&self) -> i32 { self.maxMemorySize }
    fn dispose(&mut self) {}
    fn clear(&mut self) {}
    fn getBitmap(&self, _var1: &crate::cn::wps::graphics::bitmap::IPictureMemoryPool::IPicture, _var2: i32, _var3: i32, _var4: bool, _var5: bool) -> Option<crate::cn::wps::graphics::bitmap::IPictureMemoryPool::Bitmap> { None }
    fn addPictureToMap(&mut self, _var1: &crate::cn::wps::graphics::bitmap::IPictureMemoryPool::IPicture, _var2: crate::cn::wps::graphics::bitmap::IPictureMemoryPool::Bitmap, _var3: i32, _var4: i32) -> bool { false }
    fn getReuseBitmap(&self, _var1: i32, _var2: i32, _var3: bool) -> Option<crate::cn::wps::graphics::bitmap::IPictureMemoryPool::Bitmap> { None }
}
