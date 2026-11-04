// Java: public class SectorChain
pub struct SectorChain {
    // Java field: int BLOCK_SIZE_SHIFT
    BLOCK_SIZE_SHIFT: i32,
    // Java field: int BLOCK_SIZE
    BLOCK_SIZE: i32,
    // Java field: int[][] mBlocks
    pub(crate) mBlocks: Vec<Vec<i32>>,
    // Java field: int mBlockCount
    pub(crate) mBlockCount: i32,
    // Java field: int mLastBlockSize
    mLastBlockSize: i32,
}

// Java: public class SectorChain
impl SectorChain {
    // Java: SectorChain()
    pub fn new() -> Self {
      // Java: super();

      // Java: this.mBlocks[0] = new int[256];

      todo!()
    }

    // Java: void push_back(int value)
    pub fn push_back(&mut self, _value: i32) {
      // Java: if (this.mLastBlockSize >= 256)

      {
         // Java: int nNewBlock = this.mBlockCount >> 1;

         // Java: if (nNewBlock < 8)

         {
            // Java: nNewBlock = 8;

         }
         // Java: this.reserveBlocks(this.mBlockCount + nNewBlock);

         // Java: ++this.mBlockCount;

         // Java: this.mLastBlockSize = 0;

         // Java: this.mBlocks[this.mBlockCount - 1] = new int[256];

      }
      // Java: this.mBlocks[this.mBlockCount - 1][this.mLastBlockSize++] = value;

    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return (this.mBlockCount - 1 << 8) + this.mLastBlockSize;

      todo!()
    }

    // Java: int get(int index)
    pub fn get(&self, _index: i32) -> i32 {
      // Java: int iBlock = index >> 8;

      // Java: int iBlockOffset = index & 255;

      // Java: return this.mBlocks[iBlock][iBlockOffset];

      todo!()
    }

    // Java: void set(int index, int value)
    pub fn set(&mut self, _index: i32, _value: i32) {
      // Java: int iBlock = index >> 8;

      // Java: int iBlockOffset = index & 255;

      // Java: this.mBlocks[iBlock][iBlockOffset] = value;

    }

    // Java: int back()
    pub fn back(&self) -> i32 {
      // Java: return this.mBlocks[this.mBlockCount - 1][this.mLastBlockSize - 1];

      todo!()
    }

    // Java: boolean empty()
    pub fn empty(&self) -> bool {
      // Java: return this.mBlockCount < 2 && this.mLastBlockSize == 0;

      todo!()
    }

    // Java: void reserve(int n)
    pub fn reserve(&mut self, _n: i32) {
      // Java: int nDesiredBlocks = n + 256 - 1 >> 8;

      // Java: this.reserveBlocks(nDesiredBlocks);

    }

    // Java: void reserveBlocks(int nBlocks)
    pub fn reserveBlocks(&mut self, _nBlocks: i32) {
      // Java: if (nBlocks > this.mBlocks.length)

      {
         // Java: int[][] pNew = new int[nBlocks][];

         // Java: System.arraycopy(this.mBlocks, 0, pNew, 0, this.mBlocks.length);

         // Java: this.mBlocks = pNew;

      }
    }

}
