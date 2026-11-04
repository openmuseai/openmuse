// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{Entry as IPapxEntry, IPapxTable};
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::*;

// java public final class ParagraphContext
#[derive(Default)]
pub struct ParagraphContext {
    // java public Integer mPositionInserted;
    pub mPositionInserted: Option<i32>,
    // java public String mParaId;
    pub mParaId: Option<String>,
    // java public IPapxTable.Entry mEntry;
    pub mEntry: Option<Box<dyn IPapxEntry>>,
}

// java public final class ParagraphContext
impl ParagraphContext {
    // java public  ParagraphContext(int position)
    pub fn ParagraphContext__int(position: i32) -> Self {
      // java this.mPositionInserted = position;
      Self { mPositionInserted: Some(position), mParaId: Default::default(), mEntry: Default::default() }
    }

    // java public  ParagraphContext(IPapxTable.Entry entry)
    pub fn ParagraphContext__entry(entry: Option<Box<dyn IPapxEntry>>) -> Self {
      // java IOAssert.assertNotNull("entry should not be null!", entry);
      assert!(entry.is_some(), "entry should not be null!");
      // java this.mEntry = entry;
      let this = Self { mPositionInserted: Default::default(), mParaId: Default::default(), mEntry: entry };
      this
    }

}
