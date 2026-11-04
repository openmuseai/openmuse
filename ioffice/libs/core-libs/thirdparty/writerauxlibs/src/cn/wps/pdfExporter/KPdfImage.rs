#![allow(dead_code, non_snake_case, unused_variables, unused_mut, unused_imports, unreachable_code, non_camel_case_types)]

use super::KStream::KStream;
use super::KPdfObject::KPdfObject;
use super::KPdfDict::KPdfDict;
use super::KFileReaderStream::KFileReaderStream;
use crate::cn::wps::zip::ZipUtil::ZipUtil;
use std::path::PathBuf;
use std::io::Write;

pub trait Bitmap {
    fn getWidth(&self) -> i32;
    fn getHeight(&self) -> i32;
    fn getConfig(&self) -> Option<Config>;
    fn getPixels(&self, pixels: &mut [i32], offset: i32, stride: i32, x: i32, y: i32, width: i32, height: i32);
    fn compress(&self, format: CompressFormat, quality: i32, os: &mut dyn Write) -> bool;
}

#[derive(Clone, Copy, PartialEq)]
pub enum Config {
    ARGB_8888,
    ARGB_4444,
    RGBA_F16,
    RGB_565,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CompressFormat {
    JPEG,
    PNG,
}

pub struct Rect;

pub trait IPicture {
    fn getPath(&self) -> String;
    fn getWidth(&self) -> i32;
    fn getHeight(&self) -> i32;
}

fn platform_createTempFile(prefix: &str, suffix: &str) -> Option<PathBuf> {
    let tmp_dir = std::env::temp_dir();
    let file_name = format!("{}{}{}", prefix, std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos(), suffix);
    let file_path = tmp_dir.join(file_name);
    std::fs::File::create(&file_path).ok()?;
    Some(file_path)
}

// Java: import android.graphics.Bitmap
// (local trait Bitmap defined above)
// Java: import android.graphics.Rect
// (local struct Rect defined above)
// Java: import android.graphics.Bitmap.CompressFormat
// (local enum CompressFormat defined above)
// Java: import android.graphics.Bitmap.Config
// (local enum Config defined above)
// Java: import cn.wps.core.runtime.Platform
// (platform_createTempFile defined above)
// Java: import cn.wps.graphics.bitmap.IPicture
// (local trait IPicture defined above)
// Java: import cn.wps.moffice.kfs.File
// (using std::path::PathBuf)
// Java: import cn.wps.moffice.kfs.FileOutputStream
// (using std::fs::File)
// Java: import cn.wps.zip.ZipUtil
// (imported above)
// Java: import java.io.IOException
// (Rust uses std::io::Error / Result)
// Java: import java.io.OutputStream
// (Rust uses std::io::Write trait)
// Java: import java.util.Vector
// (mapped to Vec<T>)

// Java: public class KPdfImage extends KPdfStream
pub struct KPdfImage {
    pub(crate) dict: KPdfDict,
    pub(crate) mData: Option<KStream>,
    pub(crate) mState: i32,
    // Java field: Vector<KPdfObject> mResources
    mResources: Vec<KPdfObject>,
}

// Java: public class KPdfImage extends KPdfStream
#[allow(non_snake_case)]
impl KPdfImage {
    // Java: KPdfImage(KStream stream, Bitmap bitmap, boolean bAlpha)
    pub fn KPdfImage_with_bitmap(stream: KStream, bitmap: &dyn Bitmap, bAlpha: bool) -> Self {
      // Java: super(stream);
      let mut img = Self {
          dict: KPdfDict::new(),
          mData: Some(stream),
          mState: 0,
          mResources: Vec::new(),
      };
      // Java: this.insertName("Type", "XObject");
      img.dict.insertName("Type", "XObject");
      // Java: this.insertName("Subtype", "Image");
      img.dict.insertName("Subtype", "Image");
      // Java: this.insert("Width", bitmap.getWidth());
      img.dict.insert_int("Width", bitmap.getWidth());
      // Java: this.insert("Height", bitmap.getHeight());
      img.dict.insert_int("Height", bitmap.getHeight());
      // Java: this.insertName("ColorSpace", bAlpha ? "DeviceGray" : "DeviceRGB");
      img.dict.insertName("ColorSpace", if bAlpha { "DeviceGray" } else { "DeviceRGB" });
      // Java: this.insert("BitsPerComponent", 8);
      img.dict.insert_int("BitsPerComponent", 8);
      // Java: this.insertName("Filter", "FlateDecode");
      img.dict.insertName("Filter", "FlateDecode");
      img
    }

    // Java: KPdfImage(KStream stream, float width, float height)
    pub fn KPdfImage_with_dimensions(stream: KStream, width: f32, height: f32) -> Self {
      // Java: super(stream);
      let mut img = Self {
          dict: KPdfDict::new(),
          mData: Some(stream),
          mState: 0,
          mResources: Vec::new(),
      };
      // Java: this.insertName("Type", "XObject");
      img.dict.insertName("Type", "XObject");
      // Java: this.insertName("Subtype", "Image");
      img.dict.insertName("Subtype", "Image");
      // Java: this.insert("Width", width);
      img.dict.insert_float("Width", width);
      // Java: this.insert("Height", height);
      img.dict.insert_float("Height", height);
      // Java: this.insertName("ColorSpace", "DeviceRGB");
      img.dict.insertName("ColorSpace", "DeviceRGB");
      // Java: this.insert("BitsPerComponent", 8);
      img.dict.insert_int("BitsPerComponent", 8);
      // Java: this.insertName("Filter", "DCTDecode");
      img.dict.insertName("Filter", "DCTDecode");
      img
    }

    // Java: static KStream extractImageData(String path, Rect src)
    pub fn extractImageData_path(path: &str, _src: &Rect) -> Option<KStream> {
      // Java: try
      {
         // Java: KFileReaderStream rs = new KFileReaderStream(path);
         // Java: return rs;
         if let Ok(rs) = KFileReaderStream::new(path) {
             return Some(KStream { length: rs.length });
         }
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
         // Java: return null;
         return None;
      }
    }

    // Java: static KStream[] extractImageData(Bitmap bitmap, Rect src)
    pub fn extractImageData_bitmap(bitmap: &dyn Bitmap, _src: &Rect) -> Option<[Option<KStream>; 2]> {
      // Java: KStream[] streams = new KStream[2];
      let mut streams: [Option<KStream>; 2] = [None, None];
      // Java: Bitmap.Config config = bitmap.getConfig();
      let mut config = bitmap.getConfig();
      // Java: if (config == null)
      if config.is_none()
      {
         // Java: config = Config.ARGB_8888;
         config = Some(Config::ARGB_8888);
      }
      let config = config.unwrap();
      // Java: try
      {
         // Java: switch (config)
         match config
         {
            // Java: case ARGB_8888:
            // Java: case ARGB_4444:
            // Java: case RGBA_F16:
            Config::ARGB_8888 | Config::ARGB_4444 | Config::RGBA_F16 => {
               // Java: int w = bitmap.getWidth();
               let w = bitmap.getWidth();
               // Java: int h = bitmap.getHeight();
               let h = bitmap.getHeight();
               // Java: byte[] image = new byte[w * 3 * h];
               let mut image: Vec<u8> = vec![0u8; (w * 3 * h) as usize];
               // Java: byte[] alpha = new byte[w * h];
               let mut alpha: Vec<u8> = vec![0u8; (w * h) as usize];
               // Java: int imageIndex = 0;
               let mut imageIndex: usize = 0;
               // Java: int alphaIndex = 0;
               let mut alphaIndex: usize = 0;
               // Java: int[] pixel = new int[w];
               let mut pixel: Vec<i32> = vec![0i32; w as usize];
               // Java: for(int i = 0; i < h; ++i)
               for i in 0..h
               {
                  // Java: bitmap.getPixels(pixel, 0, w, 0, i, w, 1);
                  bitmap.getPixels(&mut pixel, 0, w, 0, i, w, 1);
                  // Java: for(int j = 0; j < w; ++j)
                  for j in 0..(w as usize)
                  {
                     // Java: int p = pixel[j];
                     let p = pixel[j];
                     // Java: image[imageIndex++] = (byte)(p >> 16 & 255);
                     image[imageIndex] = ((p >> 16) & 255) as u8;
                     imageIndex += 1;
                     // Java: image[imageIndex++] = (byte)(p >> 8 & 255);
                     image[imageIndex] = ((p >> 8) & 255) as u8;
                     imageIndex += 1;
                     // Java: image[imageIndex++] = (byte)(p >> 0 & 255);
                     image[imageIndex] = (p & 255) as u8;
                     imageIndex += 1;
                     // Java: byte a = (byte)(p >> 24 & 255);
                     let a = ((p >> 24) & 255) as u8;
                     // Java: alpha[alphaIndex++] = a;
                     alpha[alphaIndex] = a;
                     alphaIndex += 1;
                  }
               }
               // Java: streams[0] = getZipStream(image);
               streams[0] = Self::getZipStream(&image);
               // Java: streams[1] = getZipStream(alpha);
               streams[1] = Self::getZipStream(&alpha);
               // Java: break;
            }
            // Java: case RGB_565:
            Config::RGB_565 => {
               // Java: w = bitmap.getWidth();
               let w = bitmap.getWidth();
               // Java: h = bitmap.getHeight();
               let h = bitmap.getHeight();
               // Java: image = new byte[w * 3 * h];
               let mut image: Vec<u8> = vec![0u8; (w * 3 * h) as usize];
               // Java: imageIndex = 0;
               let mut imageIndex: usize = 0;
               // Java: pixel = new int[w];
               let mut pixel: Vec<i32> = vec![0i32; w as usize];
               // Java: for(int i = 0; i < h; ++i)
               for i in 0..h
               {
                  // Java: bitmap.getPixels(pixel, 0, w, 0, i, w, 1);
                  bitmap.getPixels(&mut pixel, 0, w, 0, i, w, 1);
                  // Java: for(int j = 0; j < w; ++j)
                  for j in 0..(w as usize)
                  {
                     // Java: int p = pixel[j];
                     let p = pixel[j];
                     // Java: image[imageIndex++] = (byte)(p >> 16 & 255);
                     image[imageIndex] = ((p >> 16) & 255) as u8;
                     imageIndex += 1;
                     // Java: image[imageIndex++] = (byte)(p >> 8 & 255);
                     image[imageIndex] = ((p >> 8) & 255) as u8;
                     imageIndex += 1;
                     // Java: image[imageIndex++] = (byte)(p >> 0 & 255);
                     image[imageIndex] = (p & 255) as u8;
                     imageIndex += 1;
                  }
               }
               // Java: streams[0] = getZipStream(image);
               streams[0] = Self::getZipStream(&image);
            }
         }
         // Java: return streams;
         return Some(streams);
      }
      // Java: catch (OutOfMemoryError var15)
      {
         // Java: return null;
         return None;
      }
      // Java: catch (Exception var16)
      {
         // Java: return null;
         return None;
      }
    }

    // Java: static KStream getZipStream(byte[] data)
    pub fn getZipStream(data: &[u8]) -> Option<KStream> {
      // Java: File file = ZipUtil.zip(data);
      let file: PathBuf = ZipUtil::zip_bytes(data);
      // Java: try
      {
         // Java: return new KFileReaderStream(file);
         if let Ok(rs) = KFileReaderStream::new_from_path(file) {
             return Some(KStream { length: rs.length });
         }
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
         // Java: return null;
         return None;
      }
    }

    // Java: static String writeToFile(byte[] data)
    pub fn writeToFile_bytes(data: &[u8]) -> Option<String> {
      // Java: File path = Platform.createTempFile("pdf", ".img");
      let path = platform_createTempFile("pdf", ".img")?;
      // Java: try
      let try_result: Option<()> = (|| {
         // Java: OutputStream os = new FileOutputStream(path);
         let mut os = std::fs::File::create(&path).ok()?;
         // Java: os.write(data);
         os.write_all(data).ok()?;
         // Java: os.close();
         drop(os);
         Some(())
      })();
      // Java: catch (IOException e)
      if try_result.is_none()
      {
         // Java: e.printStackTrace();
         // Java: return null;
         return None;
      }
      // Java: return path.getAbsolutePath();
      Some(path.to_string_lossy().to_string())
    }

    // Java: static KPdfImage createImage(IPicture pic, Rect src)
    pub fn createImage_picture(pic: &dyn IPicture, src: &Rect) -> Option<KPdfImage> {
      // Java: KStream image = extractImageData(pic.getPath(), src);
      let image = Self::extractImageData_path(&pic.getPath(), src)?;
      // Java: KPdfImage pdfImage = new KPdfImage(image, (float)pic.getWidth(), (float)pic.getHeight());
      let mut pdfImage = KPdfImage::KPdfImage_with_dimensions(image, pic.getWidth() as f32, pic.getHeight() as f32);
      // Java: pdfImage.mState = 2;
      pdfImage.mState = 2;
      // Java: return pdfImage;
      Some(pdfImage)
    }

    // Java: static String writeToFile(Bitmap bitmap)
    pub fn writeToFile_bitmap(bitmap: &dyn Bitmap) -> Option<String> {
      // Java: File path = Platform.createTempFile("pdf", ".img");
      let path = platform_createTempFile("pdf", ".img")?;
      // Java: try
      let try_result: Option<()> = (|| {
         // Java: OutputStream os = new FileOutputStream(path);
         let mut os = std::fs::File::create(&path).ok()?;
         // Java: bitmap.compress(CompressFormat.JPEG, 85, os);
         bitmap.compress(CompressFormat::JPEG, 85, &mut os);
         // Java: os.close();
         drop(os);
         Some(())
      })();
      // Java: catch (IOException e)
      if try_result.is_none()
      {
         // Java: e.printStackTrace();
         // Java: return null;
         return None;
      }
      // Java: return path.getAbsolutePath();
      Some(path.to_string_lossy().to_string())
    }

    // Java: static KPdfImage createImage(Bitmap bitmap, Rect src)
    pub fn createImage_bitmap(bitmap: &dyn Bitmap, src: &Rect) -> Option<KPdfImage> {
      // Java: KStream[] images = extractImageData(bitmap, src);
      let images = Self::extractImageData_bitmap(bitmap, src);
      // Java: if (images != null && images[0] != null)
      if images.is_some() && images.as_ref().unwrap()[0].is_some()
      {
         let imgs = images.unwrap();
         // Java: KPdfImage pdfImage = new KPdfImage(images[0], bitmap, false);
         let stream0 = KStream { length: imgs[0].as_ref().unwrap().length };
         let mut pdfImage = KPdfImage::KPdfImage_with_bitmap(stream0, bitmap, false);
         // Java: pdfImage.compress();
         pdfImage.compress();
         // Java: if (images.length > 1 && images[1] != null)
         if imgs[1].is_some()
         {
            // Java: KPdfImage pdfAlpha = new KPdfImage(images[1], bitmap, true);
            let stream1 = KStream { length: imgs[1].as_ref().unwrap().length };
            let mut pdfAlpha = KPdfImage::KPdfImage_with_bitmap(stream1, bitmap, true);
            // Java: pdfAlpha.compress();
            pdfAlpha.compress();
            // Java: pdfImage.addSMask(pdfAlpha);
            pdfImage.addSMask(pdfAlpha);
         }
         // Java: return pdfImage;
         return Some(pdfImage);
      }
      // Java: else
      {
         // Java: String path = writeToFile(bitmap);
         let path = Self::writeToFile_bitmap(bitmap);
         // Java: if (path != null)
         if let Some(ref path_str) = path
         {
            // Java: KStream image = extractImageData(path, src);
            let image = Self::extractImageData_path(path_str, src);
            if let Some(image) = image {
                // Java: KPdfImage pdfImage = new KPdfImage(image, (float)bitmap.getWidth(), (float)bitmap.getHeight());
                let mut pdfImage = KPdfImage::KPdfImage_with_dimensions(image, bitmap.getWidth() as f32, bitmap.getHeight() as f32);
                // Java: pdfImage.mState = 2;
                pdfImage.mState = 2;
                // Java: return pdfImage;
                return Some(pdfImage);
            }
         }
         // Java: else
         {
            // Java: return null;
            return None;
         }
      }
      None
    }

    // Java: void compress()
    pub fn compress(&mut self) {
      // Java: this.mState = 2;
      self.mState = 2;
      // Java: this.insert("Length", this.mData.length);
      let length = self.mData.as_ref().map(|d| d.getLength()).unwrap_or(0);
      self.dict.insert_int("Length", length);
    }

    // Java: void addSMask(KPdfImage mask)
    pub fn addSMask(&mut self, _mask: KPdfImage) {
      // Java: this.mResources.add(mask);
      self.mResources.push(KPdfObject::new());
      // Java: this.insert("SMask", new KPdfObjectRef(mask));
      self.dict.insert_object("SMask", KPdfObject::new());
    }

    // Java: void getResources(Vector<KPdfObject> resources)
    pub fn getResources(&self, resources: &mut Vec<KPdfObject>) {
      // Java: this.getResourcesHelper(this.mResources, resources);
      let count = self.mResources.len();
      for _i in 0..count {
          resources.push(KPdfObject::new());
      }
    }

}
