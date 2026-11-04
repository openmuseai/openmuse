// Java: import org.apache.poi.poifs.common.IBlockDocumentInputStream
use crate::org::apache::poi::poifs::common::IBlockDocumentInputStream::IBlockDocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;

pub use super::EscherRecord::EscherRecord;

// Java: public interface EscherRecordFactory
pub trait EscherRecordFactory {
    // Java: abstract EscherRecord createRecord(byte[] var1, int var2)
    fn createRecord_bytes(&self, data: &[u8], offset: i32) -> EscherRecord;

    // Java: abstract EscherRecord createRecord(DocumentInputStream var1, int var2)
    fn createRecord_stream(&self, stream: &mut DocumentInputStream, offset: i32) -> EscherRecord;

    // Java: abstract EscherRecord createRecord(DocumentInputStream var1, int var2, boolean var3)
    fn createRecord_stream_flag(&self, stream: &mut DocumentInputStream, offset: i32, flag: bool) -> EscherRecord;

    // Java: abstract EscherRecord createRecord(short var1)
    fn createRecord_short(&self, record_type: i16) -> EscherRecord;

    // Java: abstract EscherRecord createRecord(IBlockDocumentInputStream var1, int var2)
    fn createRecord_block(&self, stream: &mut dyn IBlockDocumentInputStream, offset: i32) -> EscherRecord;
}
