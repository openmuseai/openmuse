use super::GridCache::GridCache;
// java public class GridRowCache extends GridCache
#[derive(Clone)]
pub struct GridRowCache {
    base: GridCache,
}
impl std::ops::Deref for GridRowCache {
    type Target = GridCache;
    fn deref(&self) -> &GridCache {
        &self.base
    }
}
impl std::ops::DerefMut for GridRowCache {
    fn deref_mut(&mut self) -> &mut GridCache {
        &mut self.base
    }
}
impl Default for GridRowCache {
    fn default() -> Self {
        Self::new()
    }
}
impl GridRowCache {
    // java public GridRowCache() {
    // java super(10);
    // java }
    pub fn new() -> Self {
        Self {
            base: GridCache::new(10),
        }
    }
    // java public void copyFrom(GridRowCache rowCache) {
    // java if (rowCache.mCache == null) {
    // java this.mCache = null;
    // java } else {
    // java int size = rowCache.mCache.length;
    // java if (this.mCache == null || this.mCache.length != size) {
    // java this.mCache = new int[size];
    // java }
    // java
    // java for(int i = 0; i < size; ++i) {
    // java this.mCache[i] = rowCache.mCache[i];
    // java }
    // java
    // java }
    // java }
    pub fn copyFrom(&mut self, src: &Self) {
        self.mCache = src.mCache.clone();
    }
    // java public void reset(int maxRowLimit, int[] mRows, int defaultRows) {
    // java int size = maxRowLimit / this.BLOCK_SIZE;
    // java if (this.mCache != null && this.mCache.length == size) {
    // java Arrays.fill(this.mCache, 0);
    // java } else {
    // java this.mCache = new int[size];
    // java }
    // java
    // java int rowCacheIndex = 0;
    // java if (mRows != null) {
    // java int rowsLen = mRows.length;
    // java int intRows = rowsLen / this.BLOCK_SIZE;
    // java int modRows = rowsLen % this.BLOCK_SIZE;
    // java int i = 0;
    // java
    // java for(int index = 0; i < intRows && index < rowsLen; ++i) {
    // java index = (i + 1) * this.BLOCK_SIZE;
    // java this.mCache[rowCacheIndex] = mRows[index < rowsLen ? index : rowsLen - 1];
    // java ++rowCacheIndex;
    // java }
    // java
    // java if (modRows > 0 && rowCacheIndex < size) {
    // java int[] var10000 = this.mCache;
    // java var10000[rowCacheIndex] += mRows[mRows.length - 1];
    // java var10000 = this.mCache;
    // java var10000[rowCacheIndex] += (this.BLOCK_SIZE - modRows + 1) * defaultRows;
    // java ++rowCacheIndex;
    // java }
    // java }
    // java
    // java int blockHeight = defaultRows * this.BLOCK_SIZE;
    // java if (rowCacheIndex == 0) {
    // java this.mCache[0] = blockHeight;
    // java ++rowCacheIndex;
    // java }
    // java
    // java for(int i = rowCacheIndex; i < this.mCache.length; ++i) {
    // java this.mCache[i] = this.mCache[i - 1] + blockHeight;
    // java }
    // java
    // java }
    pub fn reset(&mut self, max: i32, rows: Option<&[i32]>, default: i32) {
        let size = max / self.BLOCK_SIZE;
        let mut cache = vec![0; size as usize];
        let mut ci = 0usize;
        if let Some(rows) = rows {
            let len = rows.len();
            let n = len / self.BLOCK_SIZE as usize;
            let rem = len % self.BLOCK_SIZE as usize;
            let mut i = 0;
            let mut index = 0;
            while i < n && index < len {
                index = (i + 1) * self.BLOCK_SIZE as usize;
                cache[ci] = rows[index.min(len - 1)];
                ci += 1;
                i += 1;
            }
            if rem > 0 && ci < cache.len() {
                cache[ci] = cache[ci].wrapping_add(rows[len - 1]);
                cache[ci] = cache[ci]
                    .wrapping_add((self.BLOCK_SIZE - rem as i32 + 1).wrapping_mul(default));
                ci += 1;
            }
        }
        let block = default.wrapping_mul(self.BLOCK_SIZE);
        if ci == 0 {
            cache[0] = block;
            ci += 1;
        }
        for i in ci..cache.len() {
            cache[i] = cache[i - 1].wrapping_add(block);
        }
        self.mCache = Some(cache);
    }
}
