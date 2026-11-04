// Java: import cn.wps.base.log.IOLog
// Java: import java.io.IOException
// Java: import org.apache.poi.hwpf.usermodel.Picture
use crate::org::apache::poi::hwpf::usermodel::Picture::Picture;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;

// Java field: String TAG
pub const TAG: &str = "";
// Java field: int TYPE_WORDART1
pub const TYPE_WORDART1: i32 = 4;
// Java field: int TYPE_WORDART2
pub const TYPE_WORDART2: i32 = 6;
// Java field: int TYPE_IMAGE
pub const TYPE_IMAGE: i32 = 8;
// Java field: int TYPE_IMAGE_WORD2000
pub const TYPE_IMAGE_WORD2000: i32 = 0;
// Java field: int TYPE_IMAGE_PASTED_FROM_CLIPBOARD
pub const TYPE_IMAGE_PASTED_FROM_CLIPBOARD: i32 = 10;
// Java field: int TYPE_IMAGE_UNKNOWN
pub const TYPE_IMAGE_UNKNOWN: i32 = 12;
// Java field: int TYPE_IMAGE_PASTED_FROM_CLIPBOARD_WORD2000
pub const TYPE_IMAGE_PASTED_FROM_CLIPBOARD_WORD2000: i32 = 2;
// Java field: int TYPE_HORIZONTAL_LINE
pub const TYPE_HORIZONTAL_LINE: i32 = 14;
// Java field: int BLOCK_TYPE_OFFSET
pub const BLOCK_TYPE_OFFSET: i32 = 14;
// Java field: int MM_MODE_TYPE_OFFSET
pub const MM_MODE_TYPE_OFFSET: i32 = 6;
// Java field: char CHAR_EMBSHAPE
pub const CHAR_EMBSHAPE: char = '\u{0001}';
// Java field: char CHAR_DRAWINGOBJ
pub const CHAR_DRAWINGOBJ: char = '\u{0008}';

// Java: public class PicturesTable
pub struct PicturesTable {
    // Java field: DocumentInputStream _dataStream
    pub(crate) _dataStream: Option<DocumentInputStream>,
    // Java field: DocumentInputStream _mainStream
    pub(crate) _mainStream: Option<DocumentInputStream>,
}

// Java: public class PicturesTable
impl PicturesTable {
    // Java: PicturesTable(DocumentInputStream _dataStream, DocumentInputStream _mainStream)
    pub fn new(data_stream: DocumentInputStream, main_stream: DocumentInputStream) -> Self {
      // Java: super();
      let mut this = PicturesTable {
          _dataStream: None,
          _mainStream: None,
      };
      // Java: this._dataStream = _dataStream;
      this._dataStream = Some(data_stream);
      // Java: this._mainStream = _mainStream;
      this._mainStream = Some(main_stream);
      this
    }

    // Java: DocumentInputStream getMainStream()
    pub fn getMainStream(&self) -> Option<&DocumentInputStream> {
      // Java: return this._mainStream;
      self._mainStream.as_ref()
    }

    // Java: boolean isPictureRecognized(short blockType, short mappingModeOfMETAFILEPICT)
    pub fn isPictureRecognized(&self, block_type: i16, mapping_mode: i16) -> bool {
      // Java: return blockType == 8 || blockType == 4 || blockType == 6 || blockType == 10 || blockType == 12 || blockType == 0 && mappingModeOfMETAFILEPICT == 100 || blockType == 2 && mappingModeOfMETAFILEPICT == 100;
      block_type == 8 || block_type == 4 || block_type == 6 || block_type == 10 || block_type == 12 || (block_type == 0 && mapping_mode == 100) || (block_type == 2 && mapping_mode == 100)
    }

    // Java: static short getBlockType(DocumentInputStream dataStream, int pictOffset)
    pub fn getBlockType(data_stream: &mut DocumentInputStream, pict_offset: i32) -> i16 {
      // Java: dataStream.seek((long)(pictOffset + 14));
      data_stream.seek((pict_offset + 14) as i64);
      // Java: return dataStream.readShort();
      data_stream.readShort()
    }

    // Java: static short getMmMode(DocumentInputStream dataStream, int pictOffset)
    pub fn getMmMode(data_stream: &mut DocumentInputStream, pict_offset: i32) -> i16 {
      // Java: dataStream.seek((long)(pictOffset + 6));
      data_stream.seek((pict_offset + 6) as i64);
      // Java: return dataStream.readShort();
      data_stream.readShort()
    }

    // Java: Picture extractPicture(int picOffset, boolean fillBytes)
    pub fn extractPicture(&mut self, pic_offset: i32, fill_bytes: bool) -> Option<Picture> {
      // Java: try
      {
         // Java: return new Picture(picOffset, this._dataStream, fillBytes);
         if let Some(ref mut ds) = self._dataStream {
             return Some(Picture::new(pic_offset, ds, fill_bytes));
         }
         return None;
      }
      // Java: catch (IllegalArgumentException e)
      // Java: IOLog.e(TAG, "IllegalArgumentException", e);
      // Java: return null;
      // Java: catch (RuntimeException e)
      // Java: IOLog.e(TAG, "RuntimeException", e);
      // Java: return null;
    }

    // Java: boolean isBlockContainsImage(int i)
    pub fn isBlockContainsImage(&mut self, i: i32) -> bool {
      // Java: try
      {
         // Java: return null != this._dataStream && this.isLegalIndex(i) ? this.isPictureRecognized(getBlockType(this._dataStream, i), getMmMode(this._dataStream, i)) : false;
         if self._dataStream.is_some() && self.isLegalIndex(i) {
             let block_type = Self::getBlockType(self._dataStream.as_mut().unwrap(), i);
             let mm_mode = Self::getMmMode(self._dataStream.as_mut().unwrap(), i);
             return self.isPictureRecognized(block_type, mm_mode);
         }
         return false;
      }
      // Java: catch (Throwable e)
      // Java: IOLog.e(TAG, "Throwable", e);
      // Java: return false;
    }

    // Java: boolean isBlockContainsHorizontalLine(int i)
    pub fn isBlockContainsHorizontalLine(&mut self, i: i32) -> bool {
      // Java: try
      {
         // Java: return null != this._dataStream && this.isLegalIndex(i) ? getBlockType(this._dataStream, i) == 14 && getMmMode(this._dataStream, i) == 100 : false;
         if self._dataStream.is_some() && self.isLegalIndex(i) {
             let block_type = Self::getBlockType(self._dataStream.as_mut().unwrap(), i);
             let mm_mode = Self::getMmMode(self._dataStream.as_mut().unwrap(), i);
             return block_type == 14 && mm_mode == 100;
         }
         return false;
      }
      // Java: catch (Throwable e)
      // Java: IOLog.e(TAG, "Throwable", e);
      // Java: return false;
    }

    // Java: boolean isLegalIndex(int i)
    pub fn isLegalIndex(&self, i: i32) -> bool {
      // Java: return i != -1 && (long)i - this._dataStream.tell() <= (long)this._dataStream.available();
      if let Some(ref ds) = self._dataStream {
          i != -1 && (i as i64 - ds.tell()) <= ds.available() as i64
      } else {
          false
      }
    }

}
