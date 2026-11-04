#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

// Java: import android.annotation.SuppressLint
// Java: import android.graphics.Bitmap
// Java: import android.graphics.BitmapFactory
// Java: import android.graphics.Canvas
// Java: import android.graphics.Rect
// Java: import android.graphics.Bitmap.Config
// Java: import android.os.Build.VERSION
// Java: import cn.wps.base.BufferOptions
use crate::cn::wps::base::BufferOptions::{BufferOptions, BitmapConfig};
// Java: import cn.wps.base.BufferedInputStream
use crate::cn::wps::base::BufferedInputStream::BufferedInputStream;
// Java: import cn.wps.graphics.bitmap.IPictureMemoryPool
use crate::cn::wps::graphics::bitmap::IPictureMemoryPool::IPictureMemoryPool;
// Java: import cn.wps.graphics.data.reader.BufferReader
use crate::cn::wps::graphics::data::reader::BufferReader::BufferReader;
// Java: import cn.wps.graphics.data.tiff.TIFFDecodeParam
use crate::cn::wps::graphics::data::tiff::TIFFDecodeParam::TIFFDecodeParam;
// Java: import cn.wps.graphics.data.tiff.TIFFImage
use crate::cn::wps::graphics::data::tiff::TIFFImageDecoder::TIFFImage;
// Java: import cn.wps.graphics.data.tiff.TIFFImageDecoder
use crate::cn::wps::graphics::data::tiff::TIFFImageDecoder::TIFFImageDecoder;
// Java: import cn.wps.graphics.data.tiff.TIFFTile
use crate::cn::wps::graphics::data::tiff::TIFFTile::TIFFTile;
// Java: import cn.wps.graphics.data.wmf.WMFPictureInfo
use crate::cn::wps::graphics::data::wmf::WMFPictureInfo::WMFPictureInfo;
// Java: import cn.wps.graphics.gdi.MetaFilePainter
use crate::cn::wps::graphics::gdi::MetaFilePainter::MetaFilePainter;
// Java: import cn.wps.graphics.opt.ExportContext
use crate::cn::wps::graphics::opt::ExportContext::ExportContext;
// Java: import cn.wps.moffice.kfs.FileInputStream
use core_libs_kfs::kfs::mfs::FileInputStream::FileInputStream;
// Java: import cn.wps.util.Pair
// Java: import java.io.IOException
use std::io;
// Java: import java.io.InputStream
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Map

use crate::cn::wps::graphics::IMemoryHelper::IMemoryHelper;
use crate::cn::wps::graphics::IBitmapCreator::{
    IBitmapCreator,
    PictureInfo as IBC_PictureInfo,
    Bitmap as IBC_Bitmap,
};

pub struct Bitmap {
    pub width: i32,
    pub height: i32,
    pub pixels: Option<Vec<i32>>,
    pub config: BitmapConfig,
}

impl Bitmap {
    pub fn createBitmap(width: i32, height: i32, config: BitmapConfig) -> Self {
        Bitmap { width, height, pixels: Some(vec![0; (width * height) as usize]), config }
    }

    pub fn createScaledBitmap(src: &Bitmap, width: i32, height: i32, _filter: bool) -> Self {
        Bitmap { width, height, pixels: Some(vec![0; (width * height) as usize]), config: src.config }
    }

    pub fn setPixels(&mut self, _pixels: &[i32], _offset: i32, _stride: i32, _x: i32, _y: i32, _w: i32, _h: i32) {
    }

    pub fn recycle(&mut self) {
        self.pixels = None;
    }
}

pub struct Canvas {
    _bitmap: i32,
}

impl Canvas {
    pub fn new(_bitmap: &Bitmap) -> Self {
        Canvas { _bitmap: 0 }
    }
    pub fn save(&self) {}
    pub fn scale(&self, _sx: f32, _sy: f32) {}
    pub fn restore(&self) {}
}

pub struct PictureInfo {
    pub path: Option<String>,
    pub width: i32,
    pub height: i32,
    pub pic_type: i32,
    pub byteLen: i32,
}

impl PictureInfo {
    pub fn isAndroidFlie(&self) -> bool {
        self.pic_type == 1 || self.pic_type == 2
    }

    pub fn isMetaFile(&self) -> bool {
        self.pic_type == 5 || self.pic_type == 6
    }

    pub fn isRasterFlie(&self) -> bool {
        self.pic_type == 7
    }
}

pub enum PictureInfoVariant {
    Base(PictureInfo),
    WMF(WMFPictureInfo),
}

pub struct BitmapFactory;
impl BitmapFactory {
    pub fn decodeStream(_stream: &BufferedInputStream, _rect: Option<i32>, _options: &BufferOptions) -> Option<Bitmap> {
        None
    }
}

const VERSION_SDK_INT: i32 = 21;

// Java field: String TAG
pub(crate) const TAG: &str = "BitmapCreator";
// Java field: int DEF_SAMPLE_CALC
pub(crate) const DEF_SAMPLE_CALC: i32 = 0;
// Java field: int PRODUCT_SAMPLE_CALC
pub(crate) const PRODUCT_SAMPLE_CALC: i32 = 1;
// Java field: float MIN_RATE_BETWEEN_BITMAP
pub(crate) const MIN_RATE_BETWEEN_BITMAP: f32 = 0.9;

// Java: public class BitmapCreator implements IBitmapCreator
pub struct BitmapCreator {
    // Java field: IMemoryHelper memoryHelper
    pub(crate) memoryHelper: Option<Box<dyn IMemoryHelper>>,
    // Java field: Bitmap.Config config
    pub(crate) config: BitmapConfig,
    // Java field: int mSampleCalcMeth
    pub(crate) mSampleCalcMeth: i32,
    // Java field: boolean mNeedScaled
    pub(crate) mNeedScaled: bool,
    // Java field: Map<PictureInfo, Pair<Integer, Integer>> maxDimensionsMap
    maxDimensionsMap: Option<HashMap<i64, (i32, i32)>>,
}

// Java: public class BitmapCreator implements IBitmapCreator
impl BitmapCreator {
    // Java: BitmapCreator()
    pub fn new() -> Self {
      // Java: this((IMemoryHelper)null);
      Self::new_with_helper(None)
    }

    // Java: void setConfig(Bitmap.Config config)
    pub fn setConfig(&mut self, config: BitmapConfig) {
      // Java: this.config = config;
      self.config = config;
    }

    // Java: BitmapCreator(IMemoryHelper memoryHelper)
    pub fn new_with_helper(memoryHelper: Option<Box<dyn IMemoryHelper>>) -> Self {
      // Java: super();
      // Java: this.memoryHelper = null;
      // Java: this.config = Config.ARGB_4444;
      // Java: this.mSampleCalcMeth = 0;
      // Java: this.mNeedScaled = true;
      // Java: this.maxDimensionsMap = new HashMap();
      // Java: this.memoryHelper = memoryHelper;
      BitmapCreator {
          memoryHelper,
          config: BitmapConfig::ARGB_4444,
          mSampleCalcMeth: 0,
          mNeedScaled: true,
          maxDimensionsMap: Some(HashMap::new()),
      }
    }

    // Java: int newCalcInSampleSize(int srcW, int srcH, int reqW, int reqH)
    pub fn newCalcInSampleSize(&self, mut srcW: i32, mut srcH: i32, reqW: i32, reqH: i32) -> i32 {
      // Java: float dymMax = 4.0F;
      let mut dymMax: f32 = 4.0;
      // Java: float sx = 1.0F * (float)srcW / (float)reqW;
      let mut sx: f32 = 1.0 * srcW as f32 / reqW as f32;
      // Java: float sy = 1.0F * (float)srcH / (float)reqH;
      let mut sy: f32 = 1.0 * srcH as f32 / reqH as f32;
      // Java: float max = Math.max(sx, sy);
      let max: f32 = sx.max(sy);
      // Java: if (max > 8.0F)
      if max > 8.0
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
      if srcW * srcH > 25000000
      {
         // Java: dymMax = 8.0F;
         dymMax = 8.0;
      }
      // Java: if (max > dymMax)
      if max > dymMax
      {
         // Java: float scale = dymMax / max;
         let scale: f32 = dymMax / max;
         // Java: srcW = (int)((float)srcW * scale);
         srcW = (srcW as f32 * scale) as i32;
         // Java: srcH = (int)((float)srcH * scale);
         srcH = (srcH as f32 * scale) as i32;
      }
      // Java: if (srcW > 3000 || srcH > 3000)
      if srcW > 3000 || srcH > 3000
      {
         // Java: sx = 1.0F * (float)srcW / 3000.0F;
         sx = 1.0 * srcW as f32 / 3000.0;
         // Java: sy = 1.0F * (float)srcH / 3000.0F;
         sy = 1.0 * srcH as f32 / 3000.0;
         // Java: float scale = Math.max(sx, sy);
         let scale: f32 = sx.max(sy);
         // Java: srcW = (int)((float)srcW / scale);
         srcW = (srcW as f32 / scale) as i32;
         // Java: srcH = (int)((float)srcH / scale);
         srcH = (srcH as f32 / scale) as i32;
      }
      // Java: int result = Math.max(1, Math.max(srcW / reqW, srcH / reqH));
      let result: i32 = 1.max((srcW / reqW).max(srcH / reqH));
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
    pub fn limitCalcInSampleSize(&self, mut srcW: i32, mut srcH: i32, reqW: i32, reqH: i32) -> i32 {
      // Java: float dymMax = 4.0F;
      let mut dymMax: f32 = 4.0;
      // Java: if (srcW * srcH > 25000000)
      if srcW * srcH > 25000000
      {
         // Java: dymMax = 8.0F;
         dymMax = 8.0;
      }
      // Java: float sx = 1.0F * (float)srcW / (float)reqW;
      let mut sx: f32 = 1.0 * srcW as f32 / reqW as f32;
      // Java: float sy = 1.0F * (float)srcH / (float)reqH;
      let mut sy: f32 = 1.0 * srcH as f32 / reqH as f32;
      // Java: float max = Math.max(sx, sy);
      let max: f32 = sx.max(sy);
      // Java: if (max > dymMax)
      if max > dymMax
      {
         // Java: float scale = dymMax / max;
         let scale: f32 = dymMax / max;
         // Java: srcW = (int)((float)srcW * scale);
         srcW = (srcW as f32 * scale) as i32;
         // Java: srcH = (int)((float)srcH * scale);
         srcH = (srcH as f32 * scale) as i32;
      }
      // Java: if (srcW > 3000 || srcH > 3000)
      if srcW > 3000 || srcH > 3000
      {
         // Java: sx = 1.0F * (float)srcW / 3000.0F;
         sx = 1.0 * srcW as f32 / 3000.0;
         // Java: sy = 1.0F * (float)srcH / 3000.0F;
         sy = 1.0 * srcH as f32 / 3000.0;
         // Java: float scale = Math.max(sx, sy);
         let scale: f32 = sx.max(sy);
         // Java: srcW = (int)((float)srcW / scale);
         srcW = (srcW as f32 / scale) as i32;
         // Java: srcH = (int)((float)srcH / scale);
         srcH = (srcH as f32 / scale) as i32;
      }
      // Java: return Math.max(1, Math.max(srcW / reqW, srcH / reqH));
      1.max((srcW / reqW).max(srcH / reqH))
    }

    // Java: int produceSampleCalc(int w, int h, int reqW, int reqH)
    pub fn produceSampleCalc(&self, w: i32, h: i32, reqW: i32, reqH: i32) -> i32 {
      // Java: int ori = w * h;
      let ori = w * h;
      // Java: int req = reqW * reqH;
      let req = reqW * reqH;
      // Java: if (ori <= req)
      if ori <= req
      {
         // Java: return 1;
         return 1;
      }
      // Java: else
      else
      {
         // Java: int inSampleSize;
         let mut inSampleSize: i32;
         // Java: for(inSampleSize = 2; ori / (inSampleSize * inSampleSize) > req; inSampleSize *= 2)
         inSampleSize = 2;
         while ori / (inSampleSize * inSampleSize) > req {
             inSampleSize *= 2;
         }
         {
         }
         // Java: return inSampleSize;
         return inSampleSize;
      }
    }

    // Java: int defSampleCalc(int w, int h, int reqW, int reqH)
    pub fn defSampleCalc(&self, w: i32, h: i32, reqW: i32, reqH: i32) -> i32 {
      // Java: int inSampleSize = Math.max(1, Math.min(w / reqW, h / reqH));
      let mut inSampleSize: i32 = 1.max((w / reqW).min(h / reqH));
      // Java: if (inSampleSize > 2 && inSampleSize % 2 == 1)
      if inSampleSize > 2 && inSampleSize % 2 == 1
      {
         // Java: --inSampleSize;
         inSampleSize -= 1;
      }
      // Java: return inSampleSize;
      inSampleSize
    }

    // Java: void setSampleCalcMethod(int type)
    pub fn setSampleCalcMethod(&mut self, r#type: i32) {
      // Java: this.mSampleCalcMeth = type;
      self.mSampleCalcMeth = r#type;
    }

    // Java: void setNeedScaled(boolean val)
    pub fn setNeedScaled(&mut self, val: bool) {
      // Java: this.mNeedScaled = val;
      self.mNeedScaled = val;
    }

    // Java: int sampleCalc(int w, int h, int reqW, int reqH)
    pub fn sampleCalc(&self, w: i32, h: i32, reqW: i32, reqH: i32) -> i32 {
      // Java: switch (this.mSampleCalcMeth)
      match self.mSampleCalcMeth
      {
         // Java: case 1:
         1 => {
            // Java: return this.produceSampleCalc(w, h, reqW, reqH);
            self.produceSampleCalc(w, h, reqW, reqH)
         }
         // Java: default:
         _ => {
            // Java: return this.defSampleCalc(w, h, reqW, reqH);
            self.defSampleCalc(w, h, reqW, reqH)
         }
      }
    }

    // Java: Bitmap createBitmap(PictureInfo info, int width, int height, IPictureMemoryPool memPool)
    pub fn createBitmap_with_pool(&self, info: &PictureInfo, width: i32, height: i32, memPool: Option<&dyn IPictureMemoryPool>) -> Option<Bitmap> {
      // Java: if (info == null)
      if info.path.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else if (info.isAndroidFlie())
      else if info.isAndroidFlie()
      {
         // Java: return this.createAndroidBitmap(info, width, height, memPool);
         return self.createAndroidBitmap(info, width, height, memPool);
      }
      // Java: else if (info.isMetaFile())
      else if info.isMetaFile()
      {
         // Java: return this.createMetaBitmap(info, width, height, memPool);
         return self.createMetaBitmap(info, width, height, memPool);
      }
      // Java: else
      else
      {
         // Java: return info.isRasterFlie() ? this.createRasterBitmap(info, width, height) : null;
         if info.isRasterFlie() { self.createRasterBitmap(info, width, height) } else { None }
      }
    }

    // Java: boolean isNeedScaled(int bmpWidth, int bmpHeight, int expectedWidth, int expectedHeight, int inSampleSize)
    pub fn isNeedScaled(&self, bmpWidth: i32, bmpHeight: i32, expectedWidth: i32, expectedHeight: i32, inSampleSize: i32) -> bool {
      // Java: return inSampleSize < 3 && isTooSmall(expectedWidth, expectedHeight, bmpWidth, bmpHeight);
      inSampleSize < 3 && Self::isTooSmall(expectedWidth, expectedHeight, bmpWidth, bmpHeight)
    }

    // Java: static boolean isTooSmall(int expectedWidth, int expectedHeight, int bmpWidth, int bmpHeight)
    pub fn isTooSmall(expectedWidth: i32, expectedHeight: i32, bmpWidth: i32, bmpHeight: i32) -> bool {
      // Java: float rate = 0.9F;
      let _rate: f32 = 0.9;
      // Java: return (float)bmpWidth < (float)expectedWidth * 0.9F || (float)bmpHeight < (float)expectedHeight * 0.9F;
      (bmpWidth as f32) < (expectedWidth as f32 * 0.9) || (bmpHeight as f32) < (expectedHeight as f32 * 0.9)
    }

    // Java: Bitmap createRasterBitmap(PictureInfo info, int width, int height)
    pub fn createRasterBitmap(&self, info: &PictureInfo, width: i32, height: i32) -> Option<Bitmap> {
      // Java: String path = info.path;
      let path = match &info.path {
          Some(p) => p.clone(),
          None => return None,
      };
      // Java: int originWidth = info.width;
      let originWidth = info.width;
      // Java: int originHeight = info.height;
      let originHeight = info.height;
      // Java: float scale = (float)Math.min((double)originWidth / (double)width, (double)originHeight / (double)height);
      let scale: f32 = ((originWidth as f64 / width as f64).min(originHeight as f64 / height as f64)) as f32;
      // Java: int inSample = 1;
      let inSample: i32;
      // Java: if (scale > 8.0F)
      if scale > 8.0
      {
         // Java: inSample = 8;
         inSample = 8;
      }
      // Java: else if (scale > 4.0F)
      else if scale > 4.0
      {
         // Java: inSample = 4;
         inSample = 4;
      }
      // Java: else if (scale > 2.0F)
      else if scale > 2.0
      {
         // Java: inSample = 2;
         inSample = 2;
      }
      // Java: else
      else
      {
         // Java: inSample = 1;
         inSample = 1;
      }
      // Java: int actualWidth = (originWidth + inSample - 1) / inSample;
      let actualWidth = (originWidth + inSample - 1) / inSample;
      // Java: int actualHeight = (originHeight + inSample - 1) / inSample;
      let actualHeight = (originHeight + inSample - 1) / inSample;
      // Java: BufferReader input = null;
      let mut input: Option<BufferReader> = None;
      // Java: TIFFDecodeParam param = null;
      let mut param: Option<TIFFDecodeParam> = None;
      // Java: TIFFImageDecoder decoder = null;
      // Java: TIFFImage image = null;
      // Java: Bitmap bitmap = null;
      let mut bitmap: Option<Bitmap> = None;
      // Java: boolean drawBitmapSucceed = false;
      let mut drawBitmapSucceed = false;
      // Java: try
      let result: Option<()> = (|| -> Option<()> {
         // Java: InputStream is = new FileInputStream(path);
         let mut is = FileInputStream::from_name(&path).ok()?;
         // Java: input = new BufferReader(is);
         let buf_reader = BufferReader::from_stream(&mut is);
         // Java: param = new TIFFDecodeParam();
         let tiff_param = TIFFDecodeParam::new();
         // Java: decoder = new TIFFImageDecoder(input, param);
         let decoder = TIFFImageDecoder::new(buf_reader, tiff_param);
         // Java: image = decoder.decodeAsRenderedImage(0);
         let _image = decoder.decodeAsRenderedImage(0).ok()?;
         // Java: bitmap = Bitmap.createBitmap(actualWidth, actualHeight, this.config);
         bitmap = Some(Bitmap::createBitmap(actualWidth, actualHeight, self.config));
         // Java: TIFFTile tile = null;
         // Java: TIFFTile.init(inSample);
         TIFFTile::init(inSample);
         // Java: int x = 0;
         let mut _x: i32 = 0;
         // Java: for(int i = 0; i < image.tilesX; ++i)
         // Note: TIFFImage is a stub, tiles iteration not functional yet
         {
            // Java: int y = 0;
            let mut _y: i32 = 0;
            // Java: for(int j = 0; j < image.tilesY; ++j)
            {
               // Java: tile = image.getTile(i, j);
               // Java: int stride = tile.sampledWidth;
               // Java: int w = tile.sampledWidth;
               // Java: int h = tile.sampledHeight;
               // Java: int[] piexl = TIFFTile.pixels;
               // Java: bitmap.setPixels(piexl, 0, stride, x, y, w, h);
               // Java: y += tile.sampledHeight;
            }
            // Java: x += tile.sampledWidth;
         }
         // Java: drawBitmapSucceed = true;
         drawBitmapSucceed = true;
         Some(())
      })();
      // Java: catch (OutOfMemoryError var31)
      if result.is_none()
      {
         // Java: if (null != this.memoryHelper)
         if let Some(ref helper) = self.memoryHelper
         {
            // Java: this.memoryHelper.onOutOfMemory();
            helper.onOutOfMemory();
         }
      }
      // Java: catch (Exception var32)
      // Java: finally
      {
         // Java: if (drawBitmapSucceed)
         if drawBitmapSucceed
         {
            // Java: image.close();
            // Java: TIFFTile.init(0);
            TIFFTile::init(0);
         }
      }
      // Java: return bitmap;
      bitmap
    }

    // Java: Bitmap createAndroidBitmap(PictureInfo info, int width, int height, IPictureMemoryPool memPool)
    pub fn createAndroidBitmap(&self, info: &PictureInfo, width: i32, height: i32, memPool: Option<&dyn IPictureMemoryPool>) -> Option<Bitmap> {
      // Java: String path = info.path;
      let path = match &info.path {
          Some(p) => p.clone(),
          None => return None,
      };
      // Java: BufferedInputStream fileStream = null;
      let mut fileStream: Option<BufferedInputStream> = None;
      // Java: BufferOptions options = null;
      let mut options: Option<BufferOptions> = None;
      // Java: Bitmap abitmap = null;
      let mut abitmap: Option<Bitmap> = None;
      // Java: try
      let result: Result<(), ()> = (|| -> Result<(), ()> {
         // Java: boolean newResize = false;
         let mut newResize = false;
         // Java: int inSampleSize = this.sampleCalc(info.width, info.height, width, height);
         let mut inSampleSize = self.sampleCalc(info.width, info.height, width, height);
         // Java: if (inSampleSize > 1 && this.maxDimensionsMap != null)
         if inSampleSize > 1 && self.maxDimensionsMap.is_some()
         {
            // Java: Pair<Integer, Integer> currentMax = (Pair)this.maxDimensionsMap.get(info);
            let map = self.maxDimensionsMap.as_ref().unwrap();
            let key = info.width as i64 * 100000 + info.height as i64;
            let currentMax = map.get(&key).copied();
            // Java: if (currentMax == null || width > (Integer)currentMax.first && height > (Integer)currentMax.second)
            if currentMax.is_none() || (currentMax.is_some() && width > currentMax.unwrap().0 && height > currentMax.unwrap().1)
            {
               // Java: this.maxDimensionsMap.put(info, Pair.create(width, height));
            }
            // Java: currentMax = (Pair)this.maxDimensionsMap.get(info);
            let currentMax = map.get(&key).copied().unwrap_or((width, height));
            // Java: if (ExportContext.isSwitchEnabled() && ExportContext.isPrintModeEnabled())
            if ExportContext::isSwitchEnabled() && ExportContext::isPrintModeEnabled()
            {
               // Java: int newSampleSize = ExportContext.isLimitSample() ? this.limitCalcInSampleSize(info.width, info.height, (Integer)currentMax.first, (Integer)currentMax.second) : this.newCalcInSampleSize(info.width, info.height, (Integer)currentMax.first, (Integer)currentMax.second);
               let newSampleSize = if ExportContext::isLimitSample() {
                   self.limitCalcInSampleSize(info.width, info.height, currentMax.0, currentMax.1)
               } else {
                   self.newCalcInSampleSize(info.width, info.height, currentMax.0, currentMax.1)
               };
               // Java: if (newSampleSize != inSampleSize && newSampleSize < inSampleSize)
               if newSampleSize != inSampleSize && newSampleSize < inSampleSize
               {
                  // Java: newResize = true;
                  newResize = true;
                  // Java: inSampleSize = newSampleSize;
                  inSampleSize = newSampleSize;
               }
            }
         }
         // Java: int bmpWidth = info.width / inSampleSize;
         let bmpWidth = info.width / inSampleSize;
         // Java: int bmpHeight = info.height / inSampleSize;
         let bmpHeight = info.height / inSampleSize;
         // Java: int size = bmpWidth * bmpHeight * 4;
         let size = bmpWidth * bmpHeight * 4;
         // Java: cn.wps.graphics.Bitmap bitmap = null;
         let mut bitmap_reuse: Option<i32> = None;
         // Java: if (memPool != null)
         if let Some(pool) = memPool
         {
            // Java: boolean bExact = VERSION.SDK_INT < 19;
            let bExact = VERSION_SDK_INT < 19;
            // Java: bitmap = memPool.getReuseBitmap(bmpWidth, bmpHeight, bExact);
            let _reuse = pool.getReuseBitmap(bmpWidth, bmpHeight, bExact);
            // Java: if (bitmap == null)
            if bitmap_reuse.is_none()
            {
               // Java: memPool.check(size, false);
               pool.check_with_flag(size, false);
               // Java: bitmap = memPool.getReuseBitmap(bmpWidth, bmpHeight, bExact);
               let _reuse2 = pool.getReuseBitmap(bmpWidth, bmpHeight, bExact);
            }
         }
         // Java: if (this.memoryHelper != null && bitmap == null)
         if self.memoryHelper.is_some() && bitmap_reuse.is_none()
         {
            // Java: this.memoryHelper.gcForNewObject(size);
            self.memoryHelper.as_ref().unwrap().gcForNewObject(size);
         }
         // Java: options = new BufferOptions(inSampleSize, this.config);
         options = Some(BufferOptions::with_sample_size(inSampleSize, self.config));
         // Java: fileStream = new BufferedInputStream(path);
         fileStream = BufferedInputStream::from_filename(&path).ok();
         // Java: if (bitmap != null)
         if bitmap_reuse.is_some()
         {
            // Java: options.inBitmap = ((KBitmap)bitmap).getBitmap();
         }
         // Java: abitmap = BitmapFactory.decodeStream(fileStream, (Rect)null, options);
         if let Some(ref stream) = fileStream {
             if let Some(ref opts) = options {
                 abitmap = BitmapFactory::decodeStream(stream, None, opts);
             }
         }
         // Java: fileStream.close();
         if let Some(ref mut stream) = fileStream {
             let _ = stream.close();
         }
         // Java: fileStream = null;
         fileStream = None;
         // Java: if (null != abitmap && this.mNeedScaled && this.isNeedScaled(bmpWidth, bmpHeight, width, height, inSampleSize) && !newResize)
         if abitmap.is_some() && self.mNeedScaled && self.isNeedScaled(bmpWidth, bmpHeight, width, height, inSampleSize) && !newResize
         {
            // Java: Bitmap oldBitmap = abitmap;
            // Java: abitmap = Bitmap.createScaledBitmap(abitmap, width, height, true);
            if let Some(ref old) = abitmap {
                abitmap = Some(Bitmap::createScaledBitmap(old, width, height, true));
            }
            // Java: oldBitmap.recycle();
         }
         Ok(())
      })();
      // Java: catch (OutOfMemoryError var26)
      if result.is_err()
      {
         // Java: if (null != this.memoryHelper)
         if let Some(ref helper) = self.memoryHelper
         {
            // Java: this.memoryHelper.onOutOfMemory();
            helper.onOutOfMemory();
         }
      }
      // Java: catch (Exception var27)
      // Java: finally
      {
         // Java: if (null != options)
         if let Some(ref mut opts) = options
         {
            // Java: options.dispose();
            opts.dispose();
            // Java: BufferOptions var31 = null;
         }
         // Java: if (null != fileStream)
         if let Some(ref mut stream) = fileStream
         {
            // Java: try
            {
               // Java: fileStream.close();
               let _ = stream.close();
               // Java: BufferedInputStream var30 = null;
            }
            // Java: catch (IOException e)
            {
               // Java: e.printStackTrace();
            }
         }
      }
      // Java: return abitmap;
      abitmap
    }

    // Java: Bitmap createMetaBitmap(PictureInfo info, int width, int height, IPictureMemoryPool memPool)
    pub fn createMetaBitmap(&self, info: &PictureInfo, width: i32, height: i32, memPool: Option<&dyn IPictureMemoryPool>) -> Option<Bitmap> {
      // Java: if (info instanceof WMFPictureInfo)
      {
         // Java: PictureInfo embedInfo = ((WMFPictureInfo)info).getEmbedInfo();
         // Java: if (embedInfo != null)
         {
            // Java: info = embedInfo;
         }
      }
      // Java: String path = info.path;
      let path = info.path.clone();
      // Java: if (path == null)
      if path.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      {
         // Java: int size = width * height * 4;
         let size = width * height * 4;
         // Java: Bitmap abitmap = null;
         let mut abitmap: Option<Bitmap> = None;
         // Java: if (memPool != null)
         if let Some(pool) = memPool
         {
            // Java: memPool.check(size, false);
            pool.check_with_flag(size, false);
         }
         // Java: if (this.memoryHelper != null && abitmap == null)
         if self.memoryHelper.is_some() && abitmap.is_none()
         {
            // Java: this.memoryHelper.gcForNewObject(size);
            self.memoryHelper.as_ref().unwrap().gcForNewObject(size);
         }
         // Java: MetaFilePainter renderer = null;
         let mut renderer: Option<i32> = None;
         // Java: boolean drawBitmapSucceed = false;
         let mut drawBitmapSucceed = false;
         // Java: Object var21;
         // Java: try
         let try_result: Result<Option<Bitmap>, ()> = (|| -> Result<Option<Bitmap>, ()> {
            // Java: renderer = MetaFilePainter.getMetaFilePainter(info, path);
            let path_ref = path.as_ref().unwrap();
            let _renderer_val = MetaFilePainter::getMetaFilePainter(0, path_ref);
            renderer = Some(_renderer_val);
            // Java: if (info instanceof WMFPictureInfo)
            // Java: PictureInfo embedInfo = ((WMFPictureInfo)info).getEmbedInfo();
            // Java: if (embedInfo != null)
            // Java: info = embedInfo;

            // Java: if (renderer != null)
            if renderer.is_some()
            {
               // Java: if (abitmap == null)
               if abitmap.is_none()
               {
                  // Java: abitmap = Bitmap.createBitmap(width, height, this.config);
                  abitmap = Some(Bitmap::createBitmap(width, height, self.config));
               }
               // Java: float sx = (float)width / (float)info.width;
               let _sx: f32 = width as f32 / info.width as f32;
               // Java: float sy = (float)height / (float)info.height;
               let _sy: f32 = height as f32 / info.height as f32;
               // Java: Canvas canvas = new Canvas(abitmap);
               // Java: canvas.save();
               // Java: canvas.scale(sx, sy);
               // Java: renderer.paint(canvas, (float)info.width, (float)info.height);
               // Java: drawBitmapSucceed = true;
               drawBitmapSucceed = true;
               // Java: canvas.restore();
               // Java: return abitmap;
               return Ok(abitmap.take());
            }
            // Java: var21 = null;
            Ok(None)
         })();
         // Java: catch (OutOfMemoryError var17)
         match try_result {
             Err(_) => {
                // Java: if (null != this.memoryHelper)
                if let Some(ref helper) = self.memoryHelper
                {
                   // Java: this.memoryHelper.onOutOfMemory();
                   helper.onOutOfMemory();
                }
                // Java: return abitmap;
                return abitmap;
             }
             Ok(result) => {
                if let Some(bmp) = result {
                    return Some(bmp);
                }
             }
         }
         // Java: catch (Exception var18)
         // Java: return abitmap;

         // Java: finally
         {
            // Java: if (renderer != null)
            if renderer.is_some()
            {
               // Java: renderer.destroy();
            }
            // Java: if (abitmap != null && !drawBitmapSucceed)
            if abitmap.is_some() && !drawBitmapSucceed
            {
               // Java: abitmap.recycle();
               if let Some(ref mut bmp) = abitmap {
                   bmp.recycle();
               }
               // Java: abitmap = null;
               abitmap = None;
            }
         }
         // Java: return (Bitmap)var21;
         abitmap
      }
    }

    // Java: Bitmap createBitmap(PictureInfo info, int width, int height)
    pub fn createBitmap(&self, info: &PictureInfo, width: i32, height: i32) -> Option<Bitmap> {
      // Java: return this.createBitmap(info, width, height, (IPictureMemoryPool)null);
      self.createBitmap_with_pool(info, width, height, None)
    }

}

impl IBitmapCreator for BitmapCreator {
    // Java: Bitmap createBitmap(PictureInfo info, int width, int height)
    fn createBitmap(&self, _var1: &IBC_PictureInfo, _var2: i32, _var3: i32) -> IBC_Bitmap {
        IBC_Bitmap
    }
}
