// Java: import java.io.IOException
// Java: import org.apache.poi.ddf.DefaultEscherRecordFactory
use crate::org::apache::poi::ddf::DefaultEscherRecordFactory::DefaultEscherRecordFactory;
// Java: import org.apache.poi.ddf.EscherBSERecord
use crate::org::apache::poi::ddf::EscherBSERecord::EscherBSERecord;
// Java: import org.apache.poi.ddf.EscherContainerRecord
use crate::org::apache::poi::ddf::EscherContainerRecord::EscherContainerRecord;
// Java: import org.apache.poi.ddf.EscherOptRecord
use crate::org::apache::poi::ddf::EscherOptRecord::EscherOptRecord;
// Java: import org.apache.poi.ddf.EscherRecord
use crate::org::apache::poi::ddf::EscherRecord::EscherRecord;
// Java: import org.apache.poi.ddf.EscherRecordFactory
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;

use super::PICF::{PICF, PICF_SIZE};

// Java field: int PICF_OFFSET
pub const PICF_OFFSET: i32 = 0;
// Java field: int PICT_HEADER_OFFSET
pub const PICT_HEADER_OFFSET: i32 = 4;
// Java field: int MFPMM_OFFSET
pub const MFPMM_OFFSET: i32 = 6;
// Java field: int PICF_SHAPE_OFFSET
pub const PICF_SHAPE_OFFSET: i32 = 14;
// Java field: int PICMD_OFFSET
pub const PICMD_OFFSET: i32 = 28;
// Java field: int DGGTREES_OFFSET
pub const DGGTREES_OFFSET: i32 = 68;
// Java field: int UNKNOWN_HEADER_SIZE
pub const UNKNOWN_HEADER_SIZE: i32 = 73;
// Java field: byte[] GIF
pub static GIF: &[u8] = &[71, 73, 70];
// Java field: byte[] PNG
pub static PNG: &[u8] = &[137u8, 80, 78, 71, 13, 10, 26, 10];
// Java field: byte[] JPG
pub static JPG: &[u8] = &[0xFFu8, 0xD8];
// Java field: byte[] BMP
pub static BMP: &[u8] = &[66, 77];
// Java field: byte[] TIFF
pub static TIFF: &[u8] = &[73, 73, 42, 0];
// Java field: byte[] TIFF1
pub static TIFF1: &[u8] = &[77, 77, 0, 42];
// Java field: byte[] EMF
pub static EMF: &[u8] = &[1, 0, 0, 0];
// Java field: byte[] WMF1
pub static WMF1: &[u8] = &[0xD7u8, 0xCD, 0xC6, 0x9A, 0, 0];
// Java field: byte[] WMF2
pub static WMF2: &[u8] = &[1, 0, 9, 0, 0, 3];
// Java field: byte[] IHDR
pub static IHDR: &[u8] = &[73, 72, 68, 82];
// Java field: byte[] COMPRESSED1
pub static COMPRESSED1: &[u8] = &[0xFEu8, 120, 0xDA];
// Java field: byte[] COMPRESSED2
pub static COMPRESSED2: &[u8] = &[0xFEu8, 120, 0x9C];
// Java field: byte[] COMPRESSED3
pub static COMPRESSED3: &[u8] = &[0xFEu8, 120, 1];

// Java: public class Picture
pub struct Picture {
    // Java field: PICF picf
    pub(crate) picf: Option<PICF>,
    // Java field: EscherContainerRecord spContainer
    pub(crate) spContainer: Option<EscherContainerRecord>,
    // Java field: EscherBSERecord bse
    pub(crate) bse: Option<EscherBSERecord>,
    // Java field: EscherBSERecord fillBse
    pub(crate) fillBse: Option<EscherBSERecord>,
    // Java field: EscherBSERecord lineFillBse
    pub(crate) lineFillBse: Option<EscherBSERecord>,
}

// Java: public class Picture
impl Picture {
    // Java: Picture(int dataBlockStartOfsset, DocumentInputStream _dataStream, boolean fillBytes)
    pub fn new(data_block_start_offset: i32, data_stream: &mut DocumentInputStream, _fill_bytes: bool) -> Self {
      // Java: super();
      let mut this = Picture {
          picf: None,
          spContainer: None,
          bse: None,
          fillBse: None,
          lineFillBse: None,
      };
      // Java: _dataStream.seek((long)(dataBlockStartOfsset + 0));
      data_stream.seek((data_block_start_offset + 0) as i64);
      // Java: byte[] data = new byte[PICF.SIZE];
      let mut data = vec![0u8; PICF_SIZE as usize];
      // Java: _dataStream.read(data);
      data_stream.readFully(&mut data);
      // Java: this.picf = new PICF(data, 0);
      this.picf = Some(PICF::new_from_buf(&data, 0));
      // Java: int offset = dataBlockStartOfsset + 68;
      let mut offset = data_block_start_offset + 68;
      // Java: EscherRecordFactory recordFactory = new DefaultEscherRecordFactory();
      let record_factory = DefaultEscherRecordFactory::new();
      // Java: _dataStream.seek((long)offset);
      data_stream.seek(offset as i64);
      // Java: if (this.picf.mfp.mm == 102)
      if this.picf.as_ref().map_or(false, |p| p.mfp.mm == 102)
      {
         // Java: int pathLength = _dataStream.readUByte();
         let path_length = data_stream.readUByte() as i32;
         // Java: offset += pathLength + 1;
         offset += path_length + 1;
      }
      // Java: EscherRecord escherRecord = recordFactory.createRecord(_dataStream, offset);
      let escher_record = record_factory.createRecord_stream(data_stream, offset);
      // Java: if (escherRecord instanceof EscherContainerRecord)
      if escher_record.isContainerRecord()
      {
         // Java: this.spContainer = (EscherContainerRecord)recordFactory.createRecord(_dataStream, offset);
         let mut sp_container = EscherContainerRecord::new();
         // Java: int spContainerBytes = this.spContainer.fillFields(_dataStream, offset, recordFactory);
         let sp_container_bytes = sp_container.fillFields_stream(data_stream, offset, &record_factory);
         // Java: if (this.spContainer.hasChildOfType((short)-4085))
         if sp_container.hasChildOfType(-4085)
         {
            // Java: offset += spContainerBytes;
            offset += sp_container_bytes;
            // Java: EscherOptRecord opt = (EscherOptRecord)this.spContainer.getChildById((short)-4085);
            // Java: opt.fillProps();
            // Java: if (opt.getEscherPropertyById(260) != null)
            // stubbed: EscherOptRecord extraction from container not yet supported
            {
               // Java: EscherRecord record = recordFactory.createRecord(_dataStream, offset);
               let record = record_factory.createRecord_stream(data_stream, offset);
               // Java: if (record instanceof EscherBSERecord)
               if record.getRecordId() == crate::org::apache::poi::ddf::EscherBSERecord::RECORD_ID
               {
                  // Java: this.bse = (EscherBSERecord)recordFactory.createRecord(_dataStream, offset);
                  let mut bse = EscherBSERecord::new();
                  // Java: offset += this.bse.fillFields(_dataStream, offset, recordFactory);
                  offset += bse.base.fillFields_stream(data_stream, offset, &record_factory).unwrap_or(0);
                  this.bse = Some(bse);
               }
               // Java: else
               else
               {
                  // Java: offset += record.fillFields(_dataStream, offset, recordFactory);
                  offset += record.getRemainingBytes() + 8;
               }
            }
            // Java: if (opt.getEscherPropertyById(390) != null)
            // stubbed
            {
               // Java: EscherRecord record = recordFactory.createRecord(_dataStream, offset);
               let record = record_factory.createRecord_stream(data_stream, offset);
               // Java: if (record instanceof EscherBSERecord)
               if record.getRecordId() == crate::org::apache::poi::ddf::EscherBSERecord::RECORD_ID
               {
                  // Java: this.fillBse = (EscherBSERecord)recordFactory.createRecord(_dataStream, offset);
                  let mut fill_bse = EscherBSERecord::new();
                  // Java: offset += this.fillBse.fillFields(_dataStream, offset, recordFactory);
                  offset += fill_bse.base.fillFields_stream(data_stream, offset, &record_factory).unwrap_or(0);
                  this.fillBse = Some(fill_bse);
               }
               // Java: else
               else
               {
                  // Java: offset += record.fillFields(_dataStream, offset, recordFactory);
                  offset += record.getRemainingBytes() + 8;
               }
            }
            // Java: if (opt.getEscherPropertyById(453) != null)
            // stubbed
            {
               // Java: EscherRecord record = recordFactory.createRecord(_dataStream, offset);
               let record = record_factory.createRecord_stream(data_stream, offset);
               // Java: if (record instanceof EscherBSERecord)
               if record.getRecordId() == crate::org::apache::poi::ddf::EscherBSERecord::RECORD_ID
               {
                  // Java: this.lineFillBse = (EscherBSERecord)recordFactory.createRecord(_dataStream, offset);
                  let mut line_fill_bse = EscherBSERecord::new();
                  // Java: int var10000 = offset + this.lineFillBse.fillFields(_dataStream, offset, recordFactory);
                  let _var10000 = offset + line_fill_bse.base.fillFields_stream(data_stream, offset, &record_factory).unwrap_or(0);
                  this.lineFillBse = Some(line_fill_bse);
               }
               // Java: else
               else
               {
                  // Java: int var15 = offset + record.fillFields(_dataStream, offset, recordFactory);
                  let _var15 = offset + record.getRemainingBytes() + 8;
               }
            }
         }
         this.spContainer = Some(sp_container);
      }
      this
    }

    // Java: PICF getPicf()
    pub fn getPicf(&self) -> Option<&PICF> {
      // Java: return this.picf;
      self.picf.as_ref()
    }

    // Java: EscherContainerRecord getSpContainerRecord()
    pub fn getSpContainerRecord(&self) -> Option<&EscherContainerRecord> {
      // Java: return this.spContainer;
      self.spContainer.as_ref()
    }

    // Java: EscherBSERecord getBSERecord()
    pub fn getBSERecord(&self) -> Option<&EscherBSERecord> {
      // Java: return this.bse;
      self.bse.as_ref()
    }

    // Java: EscherBSERecord getFillBlipBSERecord()
    pub fn getFillBlipBSERecord(&self) -> Option<&EscherBSERecord> {
      // Java: return this.fillBse;
      self.fillBse.as_ref()
    }

    // Java: EscherBSERecord getLineFillBseRecord()
    pub fn getLineFillBseRecord(&self) -> Option<&EscherBSERecord> {
      // Java: return this.lineFillBse;
      self.lineFillBse.as_ref()
    }

}
