// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.base.log.IOLog
use util::base::log::IOLog::*;
// Java: import cn.wps.moffice.kfs.File
use stubs::cn::wps::moffice::kfs::File::*;
// Java: import cn.wps.moffice.kfs.FileOutputStream
use stubs::cn::wps::moffice::kfs::FileOutputStream::*;
// Java: import cn.wps.moffice.util.FileDataStorage
use util::moffice::util::FileDataStorage::*;
// Java: import cn.wps.moffice.util.FileUtil
use util::moffice::util::FileUtil::*;
// Java: import cn.wps.moffice.util.StreamUtil
use util::moffice::util::StreamUtil::*;
// Java: import cn.wps.poi.drawing.KPictureRegistry
use util::cn::wps::poi::drawing::KPictureRegistry::*;
// Java: import cn.wps.poi.drawing.PictureUtil
use crate::cn::wps::poi::drawing::PictureUtil::*;
// Java: import java.io.ByteArrayInputStream
use stubs::java::io::ByteArrayInputStream::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.io.InputStream
use stubs::java::io::InputStream::*;
// Java: import java.io.OutputStream
use stubs::java::io::OutputStream::*;
// Java: import java.util.UUID
use stubs::java::util::UUID::*;
// Java: import java.util.zip.InflaterInputStream
use stubs::java::util::zip::InflaterInputStream::*;
// Java: import org.apache.poi.hwpf.usermodel.AldusHeader
use crate::org::apache::poi::hwpf::usermodel::AldusHeader::*;
// Java: import org.apache.poi.hwpf.usermodel.BitmapFileHeader
use crate::org::apache::poi::hwpf::usermodel::BitmapFileHeader::*;
// Java: import org.apache.poi.poifs.common.IBlockDocumentInputStream
use crate::org::apache::poi::poifs::common::IBlockDocumentInputStream::*;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::*;
// Java: import org.apache.poi.util.HexDump
use util::poi_util::HexDump::*;
// Java: import org.apache.poi.util.LittleEndian
use util::poi_util::LittleEndian::*;
// Java: import org.apache.poi.ddf.EscherBlipRecord
use crate::org::apache::poi::ddf::EscherBlipRecord::*;
// Java: import org.apache.poi.ddf.EscherRecordFactory
use crate::org::apache::poi::ddf::EscherRecordFactory::*;

// Java field:    private static final String TAG = null;
const TAG: &str = "";
// Java field:    public static final short RECORD_ID_EMF = -4070;
pub const RECORD_ID_EMF: i16 = -4070;
// Java field:    public static final short RECORD_ID_WMF = -4069;
pub const RECORD_ID_WMF: i16 = -4069;
// Java field:    public static final short RECORD_ID_PICT = -4068;
pub const RECORD_ID_PICT: i16 = -4068;
// Java field:    public static final short SIGNATURE_EMF = 15680;
pub const SIGNATURE_EMF: i16 = 15680;
// Java field:    public static final short SIGNATURE_WMF = 8544;
pub const SIGNATURE_WMF: i16 = 8544;
// Java field:    public static final short SIGNATURE_PICT = 21536;
pub const SIGNATURE_PICT: i16 = 21536;
// Java field:    private static final int HEADER_SIZE = 8;
const HEADER_SIZE: i32 = 8;

// Java: public final class EscherMetafileBlip extends EscherBlipRecord 
#[derive(Default)]
pub struct EscherMetafileBlip {
    // Java field: EscherBlipRecord super_;
    pub super_: EscherBlipRecord,
    // Java field:    private int field_2_cb;
    field_2_cb: i32,
    // Java field:    private int field_3_rcBounds_x1;
    field_3_rcBounds_x1: i32,
    // Java field:    private int field_3_rcBounds_y1;
    field_3_rcBounds_y1: i32,
    // Java field:    private int field_3_rcBounds_x2;
    field_3_rcBounds_x2: i32,
    // Java field:    private int field_3_rcBounds_y2;
    field_3_rcBounds_y2: i32,
    // Java field:    private int field_4_ptSize_w;
    field_4_ptSize_w: i32,
    // Java field:    private int field_4_ptSize_h;
    field_4_ptSize_h: i32,
    // Java field:    private int field_5_cbSave;
    field_5_cbSave: i32,
    // Java field:    private byte field_6_fCompression;
    field_6_fCompression: u8,
    // Java field:    private byte field_7_fFilter;
    field_7_fFilter: u8,
    // Java field:    private byte[] remainingData;
    remainingData: Vec<u8>,
}

// Java: public class Rect 
#[derive(Default)]
pub struct Rect {
    // Java field:       public int left;
    pub left: i32,
    // Java field:       public int top;
    pub top: i32,
    // Java field:       public int right;
    pub right: i32,
    // Java field:       public int bottom;
    pub bottom: i32,
}

// Java: public class Point 
#[derive(Default)]
pub struct Point {
    // Java field:       public int x;
    pub x: i32,
    // Java field:       public int y;
    pub y: i32,
}

// Java: public final class EscherMetafileBlip extends EscherBlipRecord 
impl EscherMetafileBlip {
    // Java: public EscherMetafileBlip() 
    pub fn new() -> EscherMetafileBlip {
      Self { super_: EscherBlipRecord::new(), field_2_cb: Default::default(), field_3_rcBounds_x1: Default::default(), field_3_rcBounds_y1: Default::default(), field_3_rcBounds_x2: Default::default(), field_3_rcBounds_y2: Default::default(), field_4_ptSize_w: Default::default(), field_4_ptSize_h: Default::default(), field_5_cbSave: Default::default(), field_6_fCompression: Default::default(), field_7_fFilter: Default::default(), remainingData: Default::default() }
    }

    // Java: public int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory) throws IOException 
    pub fn fillFields(&mut self, data: DocumentInputStream, offset: i32, recordFactory: EscherRecordFactory) -> i32 {
      // Java: this.bytesAfterHeader = this.readHeader(data, offset);
      self.bytesAfterHeader = self.super_.readHeader(data, offset);
      // Java: int pos = offset + 8;
      let pos: i32 = offset + 8;
      // Java: data.seek((long)pos);
      data.seek(((pos) as i64));
      // Java: this.field_1_UID = new byte[16];
      self.field_1_UID = vec![Default::default(); 16];
      // Java: data.read(this.field_1_UID, 0, 16);
      data.read(self.field_1_UID, 0, 16);
      // Java: pos += 16;
      pos = pos + 16;
      // Java: if ((this.getOptions() ^ this.getSignature()) == 16)
      if self.super_.getOptions() ^ self.getSignature() == 16 {
         // Java: this.field_2_UID = new byte[16];
         self.field_2_UID = vec![Default::default(); 16];
         // Java: data.read(this.field_2_UID, 0, 16);
         data.read(self.field_2_UID, 0, 16);
         // Java: pos += 16;
         pos = pos + 16;
      }
      // Java: this.field_2_cb = data.readInt();
      self.field_2_cb = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_3_rcBounds_x1 = data.readInt();
      self.field_3_rcBounds_x1 = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_3_rcBounds_y1 = data.readInt();
      self.field_3_rcBounds_y1 = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_3_rcBounds_x2 = data.readInt();
      self.field_3_rcBounds_x2 = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_3_rcBounds_y2 = data.readInt();
      self.field_3_rcBounds_y2 = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_4_ptSize_w = data.readInt();
      self.field_4_ptSize_w = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_4_ptSize_h = data.readInt();
      self.field_4_ptSize_h = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_5_cbSave = data.readInt();
      self.field_5_cbSave = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_6_fCompression = data.readByte();
      self.field_6_fCompression = data.readByte();
      // Java: ++pos;
      { let _tmp = pos; pos += 1; _tmp };
      // Java: this.field_7_fFilter = data.readByte();
      self.field_7_fFilter = data.readByte();
      // Java: ++pos;
      { let _tmp = pos; pos += 1; _tmp };
      // Java: this.mark = pos;
      self.mark = pos;
      // Java: byte[] uid = this.getUID();
      let uid: Vec<u8> = self.getUID();
      // Java: if (uid != null && uid.length > 0)
      if uid.is_some() && uid.length > 0 {
         // Java: this.fileName = PictureUtil.generateName(uid);
         self.fileName = PictureUtil::generateName(uid);
      }
      // Java: else
      else {
         // Java: this.fileName = UUID.randomUUID().toString();
         self.fileName = UUID::randomUUID().toString();
      }
      // Java: pos += this.field_5_cbSave;
      pos = pos + self.field_5_cbSave;
      // Java: int remaining = this.bytesAfterHeader - pos + offset + 8;
      let remaining: i32 = self.bytesAfterHeader - pos + offset + 8;
      // Java: if (remaining > 0)
      if remaining > 0 {
         // Java: this.remainingData = new byte[remaining];
         self.remainingData = vec![Default::default(); remaining];
         // Java: data.read(this.remainingData, 0, remaining);
         data.read(self.remainingData, 0, remaining);
      }
      // Java: return this.bytesAfterHeader + 8;
      return self.bytesAfterHeader + 8;
    }

    // Java: public int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension) throws IOException 
    pub fn fillFields__iblockdocumentinputstream_int_escherrecordfactory_string_string(&mut self, data: IBlockDocumentInputStream, offset: i32, recordFactory: EscherRecordFactory, pictureDirectory: String, extension: String) -> i32 {
      // Java: int bytesAfterHeader = this.getRemainingBytes();
      let bytesAfterHeader: i32 = self.super_.getRemainingBytes();
      // Java: int pos = offset + 8;
      let pos: i32 = offset + 8;
      // Java: this.field_1_UID = new byte[16];
      self.field_1_UID = vec![Default::default(); 16];
      // Java: data.read(this.field_1_UID);
      data.read(self.field_1_UID);
      // Java: pos += 16;
      pos = pos + 16;
      // Java: if ((this.getOptions() ^ this.getSignature()) == 16)
      if self.super_.getOptions() ^ self.getSignature() == 16 {
         // Java: this.field_2_UID = new byte[16];
         self.field_2_UID = vec![Default::default(); 16];
         // Java: data.read(this.field_2_UID);
         data.read(self.field_2_UID);
         // Java: pos += 16;
         pos = pos + 16;
      }
      // Java: this.field_2_cb = data.readInt();
      self.field_2_cb = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_3_rcBounds_x1 = data.readInt();
      self.field_3_rcBounds_x1 = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_3_rcBounds_y1 = data.readInt();
      self.field_3_rcBounds_y1 = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_3_rcBounds_x2 = data.readInt();
      self.field_3_rcBounds_x2 = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_3_rcBounds_y2 = data.readInt();
      self.field_3_rcBounds_y2 = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_4_ptSize_w = data.readInt();
      self.field_4_ptSize_w = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_4_ptSize_h = data.readInt();
      self.field_4_ptSize_h = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_5_cbSave = data.readInt();
      self.field_5_cbSave = data.readInt();
      // Java: pos += 4;
      pos = pos + 4;
      // Java: this.field_6_fCompression = data.readByte();
      self.field_6_fCompression = data.readByte();
      // Java: ++pos;
      { let _tmp = pos; pos += 1; _tmp };
      // Java: this.field_7_fFilter = data.readByte();
      self.field_7_fFilter = data.readByte();
      // Java: ++pos;
      { let _tmp = pos; pos += 1; _tmp };
      // Java: byte[] pCompressedData = new byte[this.field_5_cbSave];
      let pCompressedData: Vec<u8> = vec![Default::default(); self.field_5_cbSave];
      // Java: data.read(pCompressedData);
      data.read(pCompressedData);
      // Java: File picFile = new File(pictureDirectory + PictureUtil.generateName(this.field_1_UID) + "." + extension);
      let picFile: File = File::new(pictureDirectory + PictureUtil::generateName(self.field_1_UID) + "." + extension);
      // Java: this.mPictureData = this.readPicture(pCompressedData, this.field_5_cbSave, this.field_6_fCompression, picFile, extension);
      self.mPictureData = self.readPicture__bytearr_int_int_file_string(pCompressedData, self.field_5_cbSave, self.field_6_fCompression, picFile, extension);
      // Java: KPictureRegistry.instance().register(this.mPictureData);
      KPictureRegistry::instance().register(self.mPictureData);
      // Java: pos += this.field_5_cbSave;
      pos = pos + self.field_5_cbSave;
      // Java: int remaining = bytesAfterHeader - pos + offset + 8;
      let remaining: i32 = bytesAfterHeader - pos + offset + 8;
      // Java: if (remaining > 0)
      if remaining > 0 {
         // Java: this.remainingData = new byte[remaining];
         self.remainingData = vec![Default::default(); remaining];
         // Java: data.read(this.remainingData);
         data.read(self.remainingData);
      }
      // Java: return bytesAfterHeader + 8;
      return bytesAfterHeader + 8;
    }

    // Java: public void readPicture(DocumentInputStream data, File picFile, String extension) throws IOException 
    pub fn readPicture(&mut self, data: DocumentInputStream, picFile: File, extension: String) {
      // Java: byte[] pCompressedData = new byte[this.field_5_cbSave];
      let pCompressedData: Vec<u8> = vec![Default::default(); self.field_5_cbSave];
      // Java: data.seek((long)this.mark);
      data.seek(((self.mark) as i64));
      // Java: data.read(pCompressedData, 0, this.field_5_cbSave);
      data.read(pCompressedData, 0, self.field_5_cbSave);
      // Java: this.mPictureData = this.readPicture(pCompressedData, this.field_5_cbSave, this.field_6_fCompression, picFile, extension);
      self.mPictureData = self.readPicture__bytearr_int_int_file_string(pCompressedData, self.field_5_cbSave, self.field_6_fCompression, picFile, extension);
      // Java: KPictureRegistry.instance().register(this.mPictureData);
      KPictureRegistry::instance().register(self.mPictureData);
    }

    // Java: private FileDataStorage readPicture(byte[] pCompressedData, int nCompressed, int nCompressMethod, File picFile, String extension) throws IOException 
    pub fn readPicture__bytearr_int_int_file_string(&mut self, pCompressedData: Vec<u8>, nCompressed: i32, nCompressMethod: i32, picFile: File, extension: String) -> FileDataStorage {
      // Java: FileDataStorage pCompressed = FileDataStorage.createFileDataStorage(pCompressedData);
      let pCompressed: FileDataStorage = FileDataStorage::createFileDataStorage(pCompressedData);
      // Java: FileOutputStream foutput = new FileOutputStream(picFile);
      let foutput: FileOutputStream = FileOutputStream::new(picFile);
      // Java: this.uncompressPicture(pCompressedData, extension, foutput, nCompressMethod);
      self.uncompressPicture(pCompressedData, extension, foutput, nCompressMethod);
      // Java: FileUtil.closeQuietly(foutput);
      FileUtil::closeQuietly(foutput);
      // Java: FileDataStorage pPictureData = new FileDataStorage(picFile);
      let pPictureData: FileDataStorage = FileDataStorage::new(picFile);
      // Java: pCompressed.setUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_COMPRESSION_METHOD", nCompressMethod);
      pCompressed.setUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_COMPRESSION_METHOD", nCompressMethod);
      // Java: pCompressed.setUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_CORE_PICTURE", pPictureData);
      pCompressed.setUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_CORE_PICTURE", pPictureData);
      // Java: pPictureData.setUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_RAW_DATA_MD4", this.field_1_UID);
      pPictureData.setUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_RAW_DATA_MD4", self.field_1_UID);
      // Java: pPictureData.setUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_RAW_COMPRESSED", pCompressed);
      pPictureData.setUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_RAW_COMPRESSED", pCompressed);
      // Java: return pPictureData;
      return pPictureData;
    }

    // Java: public int serialize(OutputStream stream) throws IOException 
    pub fn serialize(&mut self, stream: Box<dyn std::io::Write>) -> i32 {
      // Java: FileDataStorage pCompressed = PictureUtil.compressPicture(this.mPictureData);
      let pCompressed: FileDataStorage = PictureUtil::compressPicture(self.mPictureData);
      // Java: this.field_2_cb = this.mPictureData.getSize();
      self.field_2_cb = self.mPictureData.getSize();
      // Java: if (this.getRecordId() == -4069)
      if self.super_.getRecordId() == -4069 {
         // Java: this.field_2_cb -= 22;
         self.field_2_cb = self.field_2_cb - 22;
      }
      // Java: this.field_6_fCompression = (byte)PictureUtil.getCompressionMethod(pCompressed);
      self.field_6_fCompression = ((PictureUtil::getCompressionMethod(pCompressed)) as u8);
      // Java: this.field_5_cbSave = pCompressed.getSize();
      self.field_5_cbSave = pCompressed.getSize();
      // Java: this.calculateMD4();
      self.calculateMD4();
      // Java: int offset = 0;
      let offset: i32 = 0;
      // Java: int recordSize = this.getRecordSize();
      let recordSize: i32 = self.getRecordSize();
      // Java: byte[] data = new byte[recordSize - pCompressed.getSize()];
      let data: Vec<u8> = vec![Default::default(); recordSize - pCompressed.getSize()];
      // Java: LittleEndian.putShort(data, offset, this.getOptions());
      LittleEndian::putShort(data, offset, self.super_.getOptions());
      // Java: offset += 2;
      offset = offset + 2;
      // Java: LittleEndian.putShort(data, offset, this.getRecordId());
      LittleEndian::putShort(data, offset, self.super_.getRecordId());
      // Java: offset += 2;
      offset = offset + 2;
      // Java: LittleEndian.putInt(data, offset, recordSize - 8);
      LittleEndian::putInt(data, offset, recordSize - 8);
      // Java: offset += 4;
      offset = offset + 4;
      // Java: System.arraycopy(this.field_1_UID, 0, data, offset, this.field_1_UID.length);
      System::arraycopy(self.field_1_UID, 0, data, offset, self.field_1_UID.length);
      // Java: offset += this.field_1_UID.length;
      offset = offset + self.field_1_UID.length;
      // Java: if ((this.getOptions() ^ this.getSignature()) == 16)
      if self.super_.getOptions() ^ self.getSignature() == 16 {
         // Java: System.arraycopy(this.field_2_UID, 0, data, offset, this.field_2_UID.length);
         System::arraycopy(self.field_2_UID, 0, data, offset, self.field_2_UID.length);
         // Java: offset += this.field_2_UID.length;
         offset = offset + self.field_2_UID.length;
      }
      // Java: LittleEndian.putInt(data, offset, this.field_2_cb);
      LittleEndian::putInt(data, offset, self.field_2_cb);
      // Java: offset += 4;
      offset = offset + 4;
      // Java: LittleEndian.putInt(data, offset, this.field_3_rcBounds_x1);
      LittleEndian::putInt(data, offset, self.field_3_rcBounds_x1);
      // Java: offset += 4;
      offset = offset + 4;
      // Java: LittleEndian.putInt(data, offset, this.field_3_rcBounds_y1);
      LittleEndian::putInt(data, offset, self.field_3_rcBounds_y1);
      // Java: offset += 4;
      offset = offset + 4;
      // Java: LittleEndian.putInt(data, offset, this.field_3_rcBounds_x2);
      LittleEndian::putInt(data, offset, self.field_3_rcBounds_x2);
      // Java: offset += 4;
      offset = offset + 4;
      // Java: LittleEndian.putInt(data, offset, this.field_3_rcBounds_y2);
      LittleEndian::putInt(data, offset, self.field_3_rcBounds_y2);
      // Java: offset += 4;
      offset = offset + 4;
      // Java: LittleEndian.putInt(data, offset, this.field_4_ptSize_w);
      LittleEndian::putInt(data, offset, self.field_4_ptSize_w);
      // Java: offset += 4;
      offset = offset + 4;
      // Java: LittleEndian.putInt(data, offset, this.field_4_ptSize_h);
      LittleEndian::putInt(data, offset, self.field_4_ptSize_h);
      // Java: offset += 4;
      offset = offset + 4;
      // Java: LittleEndian.putInt(data, offset, this.field_5_cbSave);
      LittleEndian::putInt(data, offset, self.field_5_cbSave);
      // Java: offset += 4;
      offset = offset + 4;
      // Java: data[offset] = this.field_6_fCompression;
      data[offset] = self.field_6_fCompression;
      // Java: ++offset;
      { let _tmp = offset; offset += 1; _tmp };
      // Java: data[offset] = -2;
      data[offset] = -2;
      // Java: ++offset;
      { let _tmp = offset; offset += 1; _tmp };
      // Java: stream.write(data);
      stream.write(data);
      // Java: InputStream reader = pCompressed.getReader();
      let reader: Box<dyn std::io::Read> = pCompressed.getReader();
      // Java: StreamUtil.copyStream(reader, stream);
      StreamUtil::copyStream(reader, stream);
      // Java: reader.close();
      reader.close();
      // Java: if (this.remainingData != null)
      if self.remainingData.is_some() {
         // Java: stream.write(this.remainingData);
         stream.write(self.remainingData);
      }
      // Java: return recordSize;
      return recordSize;
    }

    // Java: public int getRecordSize() 
    pub fn getRecordSize(&mut self) -> i32 {
      // Java: FileDataStorage pCompressed;
      let pCompressed: FileDataStorage;
      // Java: try
      {
         // Java: pCompressed = PictureUtil.compressPicture(this.mPictureData);
         pCompressed = PictureUtil::compressPicture(self.mPictureData);
      }
      // Java: catch (IOException e)
      {
          let e: String = "exception".to_string();
         // Java: e.printStackTrace();
         e.printStackTrace();
         // Java: throw new RuntimeException("Can not compress the picture: " + this.mPictureData.getFile());
         panic!("{}", RuntimeException::new("Can not compress the picture: " + self.mPictureData.getFile()));
      }
      // Java: int size = 58 + pCompressed.getSize();
      let size: i32 = 58 + pCompressed.getSize();
      // Java: if (this.remainingData != null)
      if self.remainingData.is_some() {
         // Java: size += this.remainingData.length;
         size = size + self.remainingData.length;
      }
      // Java: if ((this.getOptions() ^ this.getSignature()) == 16)
      if self.super_.getOptions() ^ self.getSignature() == 16 {
         // Java: size += this.field_2_UID.length;
         size = size + self.field_2_UID.length;
      }
      // Java: return size;
      return size;
    }

    // Java: private void calculateMD4() throws IOException 
    pub fn calculateMD4(&mut self) {
      // Java: if (this.field_1_UID == null)
      if self.field_1_UID.is_none() {
         // Java: byte[] md4 = PictureUtil.getMD4(this.mPictureData);
         let md4: Vec<u8> = PictureUtil::getMD4(self.mPictureData);
         // Java: this.field_1_UID = md4;
         self.field_1_UID = md4;
      }
    }

    // Java: public byte[] getUID() 
    pub fn getUID(&mut self) -> Vec<u8> {
      // Java: if (this.field_1_UID == null)
      if self.field_1_UID.is_none() {
         // Java: try
         {
            // Java: this.calculateMD4();
            self.calculateMD4();
         }
         // Java: catch (IOException var2)
         {
             let var2: String = "exception".to_string();
            // Java: throw new RuntimeException("Can not calculate MD4");
            panic!("{}", RuntimeException::new("Can not calculate MD4"));
         }
      }
      // Java: return this.field_1_UID;
      return self.field_1_UID;
    }

    // Java: public byte[] getPrimaryUID() 
    pub fn getPrimaryUID(&mut self) -> Vec<u8> {
      // Java: return this.field_2_UID;
      return self.field_2_UID;
    }

    // Java: public void setPrimaryUID(byte[] primaryUID) 
    pub fn setPrimaryUID(&mut self, primaryUID: Vec<u8>) {
      // Java: this.field_2_UID = primaryUID;
      self.field_2_UID = primaryUID;
    }

    // Java: public Rect getBounds() 
    pub fn getBounds(&mut self) -> Rect {
      // Java: return new Rect(this.field_3_rcBounds_x1, this.field_3_rcBounds_y1, this.field_3_rcBounds_x2, this.field_3_rcBounds_y2);
      return Rect::new(self.field_3_rcBounds_x1, self.field_3_rcBounds_y1, self.field_3_rcBounds_x2, self.field_3_rcBounds_y2);
    }

    // Java: public void setBounds(Rect bounds) 
    pub fn setBounds(&mut self, bounds: Rect) {
      // Java: this.field_3_rcBounds_x1 = bounds.left;
      self.field_3_rcBounds_x1 = bounds.left;
      // Java: this.field_3_rcBounds_y1 = bounds.top;
      self.field_3_rcBounds_y1 = bounds.top;
      // Java: this.field_3_rcBounds_x2 = bounds.right;
      self.field_3_rcBounds_x2 = bounds.right;
      // Java: this.field_3_rcBounds_y2 = bounds.bottom;
      self.field_3_rcBounds_y2 = bounds.bottom;
    }

    // Java: public Point getSizeEMU() 
    pub fn getSizeEMU(&mut self) -> Point {
      // Java: return new Point(this.field_4_ptSize_w, this.field_4_ptSize_h);
      return Point::new(self.field_4_ptSize_w, self.field_4_ptSize_h);
    }

    // Java: public void setSizeEMU(Point ptSize) 
    pub fn setSizeEMU(&mut self, ptSize: Point) {
      // Java: this.field_4_ptSize_w = ptSize.x;
      self.field_4_ptSize_w = ptSize.x;
      // Java: this.field_4_ptSize_h = ptSize.y;
      self.field_4_ptSize_h = ptSize.y;
    }

    // Java: public byte[] getRemainingData() 
    pub fn getRemainingData(&mut self) -> Vec<u8> {
      // Java: return this.remainingData;
      return self.remainingData;
    }

    // Java: public String toString() 
    pub fn toString(&mut self) -> String {
      // Java: String extraData = "";
      let extraData: String = "";
      // Java: return this.getClass().getName() + ":" + '\n' + "  RecordId: 0x" + HexDump.toHex(this.getRecordId()) + '\n' + "  Options: 0x" + HexDump.toHex(this.getOptions()) + '\n' + "  UID: 0x" + HexDump.toHex(this.field_1_UID) + '\n' + (this.field_2_UID == null ? "" : "  UID2: 0x" + HexDump.toHex(this.field_2_UID) + '\n') + "  Uncompressed Size: " + HexDump.toHex(this.field_2_cb) + '\n' + "  Compressed Size: " + HexDump.toHex(this.field_5_cbSave) + '\n' + "  Compression: " + HexDump.toHex(this.field_6_fCompression) + '\n' + "  Filter: " + HexDump.toHex(this.field_7_fFilter) + '\n' + "  Extra Data:" + '\n' + extraData + (this.remainingData == null ? null : "\n Remaining Data: " + HexDump.toHex(this.remainingData, 32));
      return self.getClass().getName() + ":" + '\n' as u16 + "  RecordId: 0x" + HexDump::toHex(self.super_.getRecordId()) + '\n' as u16 + "  Options: 0x" + HexDump::toHex(self.super_.getOptions()) + '\n' as u16 + "  UID: 0x" + HexDump::toHex(self.field_1_UID) + '\n' as u16 + (if self.field_2_UID.is_none() { "" } else { "  UID2: 0x" + HexDump::toHex(self.field_2_UID) + '\n' as u16 }) + "  Uncompressed Size: " + HexDump::toHex(self.field_2_cb) + '\n' as u16 + "  Compressed Size: " + HexDump::toHex(self.field_5_cbSave) + '\n' as u16 + "  Compression: " + HexDump::toHex(self.field_6_fCompression) + '\n' as u16 + "  Filter: " + HexDump::toHex(self.field_7_fFilter) + '\n' as u16 + "  Extra Data:" + '\n' as u16 + extraData + (if self.remainingData.is_none() { None } else { "\n Remaining Data: " + HexDump::toHex(self.remainingData, 32) });
    }

    // Java: public short getSignature() 
    pub fn getSignature(&mut self) -> i16 {
      // Java: switch (this.getRecordId())
      match self.super_.getRecordId() {
         // case -4070
         -4070 => { return 15680; }
         // case -4069
         -4069 => { return 8544; }
         // case -4068
         -4068 => { return 21536; }
         // case _
         _ => {
    IOLog::w(TAG, "Unknown metafile: " + self.super_.getRecordId());
    return 0;
}
      }
    }

    // Java: private void uncompressPicture(byte[] pRawPictureData, String extension, OutputStream dst, int nCompressionMethod) throws IOException 
    pub fn uncompressPicture(&mut self, pRawPictureData: Vec<u8>, extension: String, dst: Box<dyn std::io::Write>, nCompressionMethod: i32) {
      // Java: OutputStream stream = dst;
      let stream: Box<dyn std::io::Write> = dst;
      // Java: if (nCompressionMethod == 0)
      if nCompressionMethod == 0 {
         // Java: InflaterInputStream in = new InflaterInputStream(new ByteArrayInputStream(pRawPictureData));
         let r#in: InflaterInputStream = InflaterInputStream::new(ByteArrayInputStream::new(pRawPictureData));
         // Java: if ("wmf".equals(extension))
         if "wmf".equals(extension) {
            // Java: AldusHeader aldus = new AldusHeader(this.field_4_ptSize_w, this.field_4_ptSize_h);
            let aldus: AldusHeader = AldusHeader::new(self.field_4_ptSize_w, self.field_4_ptSize_h);
            // Java: dst.write(aldus.serialize());
            dst.write(aldus.serialize());
            // Java: try
            {
               // Java: StreamUtil.copyStream(in, stream);
               StreamUtil::copyStream(r#in, stream);
            }
            // Java: catch (IOException e)
            {
                let e: String = "exception".to_string();
               // Java: IOAssert.fail(e.getMessage());
               IOAssert::fail(e.getMessage());
               // Java: IOLog.d(TAG, e.getMessage());
               IOLog::d(TAG, e.getMessage());
            }
         }
         // Java: else if (!"dib".equals(extension))
         else if !"dib".equals(extension) {
            // Java: StreamUtil.copyStream(in, dst);
            StreamUtil::copyStream(r#in, dst);
         }
      }
      // Java: else if ("wmf".equals(extension))
      else if "wmf".equals(extension) {
         // Java: AldusHeader aldus = new AldusHeader(this.field_4_ptSize_w, this.field_4_ptSize_h);
         let aldus: AldusHeader = AldusHeader::new(self.field_4_ptSize_w, self.field_4_ptSize_h);
         // Java: dst.write(aldus.serialize());
         dst.write(aldus.serialize());
         // Java: dst.write(pRawPictureData);
         dst.write(pRawPictureData);
      }
      // Java: else if ("dib".equals(extension))
      else if "dib".equals(extension) {
         // Java: BitmapFileHeader bfHeader = new BitmapFileHeader(pRawPictureData);
         let bfHeader: BitmapFileHeader = BitmapFileHeader::new(pRawPictureData);
         // Java: dst.write(bfHeader.serialize());
         dst.write(bfHeader.serialize());
         // Java: dst.write(pRawPictureData);
         dst.write(pRawPictureData);
      }
      // Java: else
      else {
         // Java: dst.write(pRawPictureData);
         dst.write(pRawPictureData);
      }
    }

}

impl std::ops::Deref for EscherMetafileBlip {
    type Target = EscherBlipRecord;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

// Java: public class Rect 
impl Rect {
    // Java: public Rect() 
    pub fn new() -> Rect {
      Self { left: Default::default(), top: Default::default(), right: Default::default(), bottom: Default::default() }
    }

    // Java: public Rect(int left, int top, int right, int bottom) 
    pub fn new__int_int_int_int(left: i32, top: i32, right: i32, bottom: i32) -> Rect {
      Self { left: left, top: top, right: right, bottom: bottom }
    }

}

// Java: public class Point 
impl Point {
    // Java: public Point() 
    pub fn new() -> Point {
      Self { x: Default::default(), y: Default::default() }
    }

    // Java: public Point(int x, int y) 
    pub fn new__int_int(x: i32, y: i32) -> Point {
      Self { x: x, y: y }
    }

}
