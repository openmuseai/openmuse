// Java field: int NOT_CACHED
pub const NOT_CACHED: i32 = i32::MIN;
// Java field: byte NO_INDEX
pub const NO_INDEX: i8 = -1;

// Java: public class KerningPairCache
pub struct KerningPairCache {
    // Java field: int cacheIndex
    pub(crate) cacheIndex: i32,
    // Java field: int[] kerningTable
    pub(crate) kerningTable: Option<Vec<i32>>,
    // Java field: boolean makeCacheErrorOnce
    pub(crate) makeCacheErrorOnce: bool,
}

// Java: public class KerningPairCache
impl KerningPairCache {
    // Java: KerningPairCache()
    pub fn new() -> Self {
        // Java: super();
        KerningPairCache {
            cacheIndex: -1,
            kerningTable: None,
            makeCacheErrorOnce: false,
        }
    }

    // Java: int find(char left, char right)
    pub fn find(&mut self, left: u16, right: u16) -> i32 {
        // Java: if (this.makeCacheErrorOnce)
        if self.makeCacheErrorOnce {
            // Java: return Integer.MIN_VALUE;
            return i32::MIN;
        }
        // Java: else
        {
            // Java: this.cacheIndex = this.toIndex(left, right);
            self.cacheIndex = self.toIndex(left, right);
            // Java: if (this.cacheIndex != -1)
            if self.cacheIndex != -1 {
                // Java: if (this.kerningTable == null && !this.allocateCache())
                if self.kerningTable.is_none() && !self.allocateCache() {
                    // Java: this.makeCacheErrorOnce = true;
                    self.makeCacheErrorOnce = true;
                    // Java: this.cacheIndex = -1;
                    self.cacheIndex = -1;
                    // Java: return Integer.MIN_VALUE;
                    return i32::MIN;
                }
                // Java: else
                else {
                    // Java: return this.kerningTable[this.cacheIndex];
                    if let Some(ref table) = self.kerningTable {
                        return table[self.cacheIndex as usize];
                    }
                    return i32::MIN;
                }
            }
            // Java: else
            else {
                // Java: return Integer.MIN_VALUE;
                i32::MIN
            }
        }
    }

    // Java: void put(int kerning)
    pub fn put(&mut self, kerning: i32) {
        // Java: if (this.cacheIndex != -1)
        if self.cacheIndex != -1 {
            // Java: this.kerningTable[this.cacheIndex] = kerning;
            if let Some(ref mut table) = self.kerningTable {
                table[self.cacheIndex as usize] = kerning;
            }
        }
    }

    // Java: abstract boolean allocateCache()
    pub fn allocateCache(&mut self) -> bool {
        false
    }

    // Java: abstract int toIndex(char var1, char var2)
    pub fn toIndex(&self, _var1: u16, _var2: u16) -> i32 {
        -1
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.kerningTable = null;
        self.kerningTable = None;
    }
}
