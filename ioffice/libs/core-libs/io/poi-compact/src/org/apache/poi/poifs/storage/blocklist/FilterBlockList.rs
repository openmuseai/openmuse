// Java: import cn.wps.moffice.crash.FileDamagedException
// Java: import org.apache.poi.poifs.common.BlockBuf
use crate::org::apache::poi::poifs::common::BlockBuf::BlockBuf;
// Java: import org.apache.poi.poifs.storage.BlockAllocationTableReader
use crate::org::apache::poi::poifs::storage::BlockAllocationTableReader::BlockAllocationTableReader;
// Java: import org.apache.poi.util.IntList
use super::IBlockList::IBlockList;

// Java field: String TAG
pub(crate) const TAG: &str = "FilterBlockList";

// Java: public class FilterBlockList implements IBlockList
pub struct FilterBlockList {
    // Java field: IBlockList filteredList
    pub(crate) filteredList: Option<Box<dyn IBlockList>>,
    // Java field: IntList batEntries
    pub(crate) batEntries: Vec<i32>,
}

// Java: public class FilterBlockList implements IBlockList
impl FilterBlockList {
    // Java: FilterBlockList(IBlockList bl, BlockAllocationTableReader bat, int startBatEntry)
    pub fn new(bl: Box<dyn IBlockList>, bat: &BlockAllocationTableReader, startBatEntry: i32) -> Self {
        // Java: super();
        // Java: this.batEntries = bat.fetchBlockList(startBatEntry);
        let batEntries = bat.fetchBlockList(startBatEntry);
        // Java: this.filteredList = bl;
        FilterBlockList {
            filteredList: Some(bl),
            batEntries,
        }
    }

    // Java: void dispose()
    pub fn dispose_impl(&mut self) {
        // Java: if (null != this.filteredList)
        if let Some(ref mut fl) = self.filteredList {
            // Java: this.filteredList.dispose();
            fl.dispose();
            // Java: this.filteredList = null;
        }
        self.filteredList = None;
    }

    // Java: BlockBuf getBlockAt(int blockIndex)
    pub fn getBlockAt_impl(&mut self, blockIndex: i32) -> BlockBuf {
        // Java: return this.filteredList.getBlockAt(this.batEntries.get(blockIndex));
        let idx = self.batEntries[blockIndex as usize];
        self.filteredList.as_mut().unwrap().getBlockAt(idx)
    }

    // Java: int getBlockCount()
    pub fn getBlockCount_impl(&self) -> i32 {
        // Java: return this.batEntries.size();
        self.batEntries.len() as i32
    }

    // Java: int getBlockSize()
    pub fn getBlockSize_impl(&self) -> i32 {
        // Java: return this.filteredList.getBlockSize();
        self.filteredList.as_ref().unwrap().getBlockSize()
    }

    // Java: boolean readBlock(int blockIndex, BlockBuf buf)
    pub fn readBlock_impl(&mut self, blockIndex: i32, buf: BlockBuf) -> bool {
        // Java: try
        // Java: return this.filteredList.readBlock(this.batEntries.get(blockIndex), buf);
        let idx = blockIndex as usize;
        if idx >= self.batEntries.len() {
            // Java: catch (IndexOutOfBoundsException e)
            // Java: throw new FileDamagedException(e);
            panic!("FileDamagedException: IndexOutOfBoundsException");
        }
        let bat_idx = self.batEntries[idx];
        self.filteredList.as_mut().unwrap().readBlock(bat_idx, buf)
    }
}

impl IBlockList for FilterBlockList {
    fn getBlockAt(&mut self, block_index: i32) -> BlockBuf {
        self.getBlockAt_impl(block_index)
    }
    fn readBlock(&mut self, var1: i32, var2: BlockBuf) -> bool {
        self.readBlock_impl(var1, var2)
    }
    fn getBlockCount(&self) -> i32 {
        self.getBlockCount_impl()
    }
    fn getBlockSize(&self) -> i32 {
        self.getBlockSize_impl()
    }
    fn dispose(&mut self) {
        self.dispose_impl();
    }
}
