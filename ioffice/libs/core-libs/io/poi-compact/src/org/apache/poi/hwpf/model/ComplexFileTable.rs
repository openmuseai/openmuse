// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.poi.hwpf.model.io.HWPFFileSystem
use crate::org::apache::poi::hwpf::model::io::HWPFFileSystem::HWPFFileSystem;
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use super::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian
use super::TextPieceTable::TextPieceTable;
use super::PrcData::PrcData;

// Java field: byte TEXT_PIECE_TABLE_TYPE
pub const TEXT_PIECE_TABLE_TYPE: u8 = 2;
// Java field: byte GRPPRL_TYPE
const GRPPRL_TYPE: u8 = 1;

fn le_put_int(buf: &mut [u8], value: i32) {
    let bytes = value.to_le_bytes();
    buf[0] = bytes[0];
    buf[1] = bytes[1];
    buf[2] = bytes[2];
    buf[3] = bytes[3];
}

// Java: public class ComplexFileTable
pub struct ComplexFileTable {
    // Java field: TextPieceTable _tpt
    pub _tpt: TextPieceTable,
    // Java field: List<PrcData> prcDataList
    pub prcDataList: Option<Vec<PrcData>>,
}

// Java: public class ComplexFileTable
impl ComplexFileTable {
    // Java: ComplexFileTable(DocumentInputStream documentStream, DocumentInputStream tableStream, int offset, int fcMin)
    pub fn new(document_stream: DocumentInputStream, table_stream: &mut DocumentInputStream, mut offset: i32, fc_min: i32) -> Self {
      // Java: super();
      let mut prc_data_list: Option<Vec<PrcData>> = None;
      // Java: tableStream.seek((long)offset);
      table_stream.seek(offset as i64);
      // Java: while(tableStream.readByte() == 1)
      while table_stream.readByte() == 1
      {
         // Java: if (this.prcDataList == null)
         if prc_data_list.is_none()
         {
            // Java: this.prcDataList = new ArrayList();
            prc_data_list = Some(Vec::new());
         }
         // Java: ++offset;
         offset += 1;
         // Java: int skip = tableStream.readUShort();
         let skip = table_stream.readUShort();
         // Java: this.prcDataList.add(new PrcData(tableStream, offset));
         prc_data_list.as_mut().unwrap().push(PrcData::new(table_stream, offset));
         // Java: offset += 2 + skip;
         offset += 2 + skip as i32;
         // Java: tableStream.seek((long)offset);
         table_stream.seek(offset as i64);
      }
      // Java: tableStream.seek((long)offset);
      table_stream.seek(offset as i64);
      // Java: if (tableStream.readByte() != 2)
      if table_stream.readByte() != 2
      {
         // Java: throw new IOException("The text piece table is corrupted");
         panic!("The text piece table is corrupted");
      }
      // Java: else
      else
      {
         // Java: ++offset;
         offset += 1;
         // Java: int pieceTableSize = tableStream.readInt();
         let piece_table_size = table_stream.readInt();
         // Java: offset += 4;
         offset += 4;
         // Java: this._tpt = new TextPieceTable(documentStream, tableStream, offset, pieceTableSize, fcMin);
         let tpt = TextPieceTable::new(document_stream, table_stream.clone(), offset, piece_table_size, fc_min);
         ComplexFileTable {
            _tpt: tpt,
            prcDataList: prc_data_list,
         }
      }
    }

    // Java: TextPieceTable getTextPieceTable()
    pub fn getTextPieceTable(&self) -> &TextPieceTable {
      // Java: return this._tpt;
      &self._tpt
    }

    // Java: List<PrcData> getPrcDataList()
    pub fn getPrcDataList(&self) -> &Option<Vec<PrcData>> {
      // Java: return this.prcDataList;
      &self.prcDataList
    }

    // Java: void writeTo(HWPFFileSystem sys)
    pub fn writeTo(&mut self, sys: &mut HWPFFileSystem) {
      // Java: HWPFOutputStream docStream = sys.getStream("WordDocument");
      let doc_stream_ptr = sys.getStream("WordDocument").unwrap()
          as *mut crate::org::apache::poi::hwpf::model::io::HWPFFileSystem::HWPFOutputStream
          as *mut HWPFOutputStream;
      // Java: HWPFOutputStream tableStream = sys.getStream("0Table");
      let table_stream_ptr = sys.getStream("0Table").unwrap()
          as *mut crate::org::apache::poi::hwpf::model::io::HWPFFileSystem::HWPFOutputStream
          as *mut HWPFOutputStream;
      // Java: tableStream.write(2);
      let table_s = unsafe { &mut *table_stream_ptr };
      table_s.write_all(&[2u8]);
      // Java: byte[] table = this._tpt.writeTo(docStream);
      let doc_s = unsafe { &mut *doc_stream_ptr };
      let table = self._tpt.writeTo(doc_s);
      // Java: byte[] numHolder = new byte[4];
      let mut num_holder = [0u8; 4];
      // Java: LittleEndian.putInt(numHolder, table.length);
      le_put_int(&mut num_holder, table.len() as i32);
      // Java: tableStream.write(numHolder);
      let table_s = unsafe { &mut *table_stream_ptr };
      table_s.write_all(&num_holder);
      // Java: tableStream.write(table);
      table_s.write_all(&table);
    }

}
