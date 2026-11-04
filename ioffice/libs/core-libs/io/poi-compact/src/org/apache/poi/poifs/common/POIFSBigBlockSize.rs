// Java: public class POIFSBigBlockSize
pub struct POIFSBigBlockSize {
    // Java field: int bigBlockSize
    pub(crate) bigBlockSize: i32,
    // Java field: short headerValue
    pub(crate) headerValue: i16,
}

// Java: public class POIFSBigBlockSize
impl POIFSBigBlockSize {
    // Java: POIFSBigBlockSize(int bigBlockSize, short headerValue)
    pub fn new(bigBlockSize: i32, headerValue: i16) -> Self {
        // Java: super();
        // Java: this.bigBlockSize = bigBlockSize;
        // Java: this.headerValue = headerValue;
        POIFSBigBlockSize {
            bigBlockSize,
            headerValue,
        }
    }

    // Java: int getBigBlockSize()
    pub fn getBigBlockSize(&self) -> i32 {
        // Java: return this.bigBlockSize;
        self.bigBlockSize
    }

    // Java: short getHeaderValue()
    pub fn getHeaderValue(&self) -> i16 {
        // Java: return this.headerValue;
        self.headerValue
    }

    // Java: int getPropertiesPerBlock()
    pub fn getPropertiesPerBlock(&self) -> i32 {
        // Java: return this.bigBlockSize / 128;
        self.bigBlockSize / 128
    }

    // Java: int getBATEntriesPerBlock()
    pub fn getBATEntriesPerBlock(&self) -> i32 {
        // Java: return this.bigBlockSize / 4;
        self.bigBlockSize / 4
    }

    // Java: int getXBATEntriesPerBlock()
    pub fn getXBATEntriesPerBlock(&self) -> i32 {
        // Java: return this.getBATEntriesPerBlock() - 1;
        self.getBATEntriesPerBlock() - 1
    }

    // Java: int getNextXBATChainOffset()
    pub fn getNextXBATChainOffset(&self) -> i32 {
        // Java: return this.getXBATEntriesPerBlock() * 4;
        self.getXBATEntriesPerBlock() * 4
    }
}
