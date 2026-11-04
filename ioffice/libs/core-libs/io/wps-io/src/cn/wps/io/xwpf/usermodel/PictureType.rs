// Java: import cn.wps.moffice.util.StringUtil
// use util::org::apache::poi::util::StringUtil;
// Java: import java.util.HashMap
use std::collections::HashMap;

// Java field: HashMap<Integer, PictureType> PICTURE_TYPE_BY_OOXML_ID
static PICTURE_TYPE_BY_OOXML_ID: std::sync::OnceLock<std::sync::Mutex<HashMap<i32, PictureType>>> = std::sync::OnceLock::new();

// Java: public enum PictureType
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PictureType {
    EMF, WMF, PICT, JPEG, JPG, PNG, DIB, GIF, TIFF, TIF,
    EPS, BMP, WPG, WDP, SVG, PCX, PSD, TGA,
}

// Java: public enum PictureType
impl PictureType {
    // Java field: HashMap<Integer, PictureType> PICTURE_TYPE_BY_OOXML_ID
    // Java field: String contentType
    // Java field: String extension
    // Java field: int ooxmlId

    // Java: PictureType(String contentType, String extension, int ooxmlId)
    pub fn content_type(&self) -> &'static str {
      // Java: this.contentType = contentType;
      // Java: this.extension = extension;
      // Java: this.ooxmlId = ooxmlId;
      match self {
          PictureType::EMF => "image/x-emf",
          PictureType::WMF => "image/x-wmf",
          PictureType::PICT => "image/x-pict",
          PictureType::JPEG | PictureType::JPG => "image/jpeg",
          PictureType::PNG => "image/png",
          PictureType::DIB => "image/dib",
          PictureType::GIF => "image/gif",
          PictureType::TIFF | PictureType::TIF => "image/tiff",
          PictureType::EPS => "image/x-eps",
          PictureType::BMP => "image/x-bmp",
          PictureType::WPG => "image/x-wpg",
          PictureType::WDP => "image/vnd.ms-photo",
          PictureType::SVG => "image/svg+xml",
          PictureType::PCX => "image/pcx",
          PictureType::PSD => "image/psd",
          PictureType::TGA => "image/tag",
      }
    }

    // Java: String getContentType()
    pub fn getContentType(&self) -> String {
      // Java: return this.contentType;
      self.content_type().to_string()
    }

    // Java: String getExtension()
    pub fn getExtension(&self) -> String {
      // Java: return this.extension;
      match self {
          PictureType::EMF => "emf",
          PictureType::WMF => "wmf",
          PictureType::PICT => "pict",
          PictureType::JPEG => "jpeg",
          PictureType::JPG => "jpg",
          PictureType::PNG => "png",
          PictureType::DIB => "dib",
          PictureType::GIF => "gif",
          PictureType::TIFF => "tiff",
          PictureType::TIF => "tif",
          PictureType::EPS => "eps",
          PictureType::BMP => "bmp",
          PictureType::WPG => "wpg",
          PictureType::WDP => "wdp",
          PictureType::SVG => "svg",
          PictureType::PCX => "pcx",
          PictureType::PSD => "psd",
          PictureType::TGA => "tag",
      }.to_string()
    }

    // Java: int getOoxmlId()
    pub fn getOoxmlId(&self) -> i32 {
      // Java: return this.ooxmlId;
      match self {
          PictureType::EMF => 2,
          PictureType::WMF => 3,
          PictureType::PICT => 4,
          PictureType::JPEG | PictureType::JPG => 5,
          PictureType::PNG => 6,
          PictureType::DIB => 7,
          PictureType::GIF => 8,
          PictureType::TIFF | PictureType::TIF => 9,
          PictureType::EPS => 10,
          PictureType::BMP => 11,
          PictureType::WPG => 12,
          PictureType::WDP => 13,
          PictureType::SVG => 14,
          PictureType::PCX => 15,
          PictureType::PSD => 16,
          PictureType::TGA => 17,
      }
    }

    // Java: boolean compareExtensition(String ext)
    pub fn compareExtensition(&self, ext: &str) -> bool {
      // Java: return this.extension.equalsIgnoreCase(ext);
      self.getExtension().eq_ignore_ascii_case(ext)
    }

    // Java: static PictureType getPictureType(String extension)
    pub fn getPictureType(extension: &str) -> Option<PictureType> {
      // Java: if (StringUtil.isEmpty(extension))
      if extension.is_empty() {
         // Java: return null;
         return None;
      }
      let all = [
          PictureType::JPEG, PictureType::JPG, PictureType::PNG,
          PictureType::BMP, PictureType::DIB, PictureType::WMF,
          PictureType::EMF, PictureType::GIF, PictureType::PICT,
          PictureType::TIFF, PictureType::TIF, PictureType::EPS,
          PictureType::PCX, PictureType::PSD, PictureType::TGA,
          PictureType::SVG, PictureType::WDP, PictureType::WPG,
      ];
      // Java: else if (JPEG.compareExtensition(extension))
      // Java: return JPEG;
      // Java: else if (JPG.compareExtensition(extension))
      // Java: return JPG;
      // Java: else if (PNG.compareExtensition(extension))
      // Java: return PNG;
      // Java: else if (BMP.compareExtensition(extension))
      // Java: return BMP;
      // Java: else if (DIB.compareExtensition(extension))
      // Java: return DIB;
      // Java: else if (WMF.compareExtensition(extension))
      // Java: return WMF;
      // Java: else if (EMF.compareExtensition(extension))
      // Java: return EMF;
      // Java: else if (GIF.compareExtensition(extension))
      // Java: return GIF;
      // Java: else if (PICT.compareExtensition(extension))
      // Java: return PICT;
      // Java: else if (TIFF.compareExtensition(extension))
      // Java: return TIFF;
      // Java: else if (TIF.compareExtensition(extension))
      // Java: return TIF;
      // Java: else if (EPS.compareExtensition(extension))
      // Java: return EPS;
      // Java: else if (PCX.compareExtensition(extension))
      // Java: return PCX;
      // Java: else if (PSD.compareExtensition(extension))
      // Java: return PSD;
      // Java: else if (TGA.compareExtensition(extension))
      // Java: return TGA;
      // Java: else if (SVG.compareExtensition(extension))
      // Java: return SVG;
      // Java: else if (WDP.compareExtensition(extension))
      // Java: return WDP;
      // Java: else
      // Java: return WPG.compareExtensition(extension) ? WPG : null;
      for pt in &all {
          if pt.compareExtensition(extension) {
              return Some(*pt);
          }
      }
      None
    }
}
