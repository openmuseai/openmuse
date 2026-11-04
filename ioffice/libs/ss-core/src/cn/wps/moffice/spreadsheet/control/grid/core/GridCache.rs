// java public abstract class GridCache
#[derive(Clone)]
pub struct GridCache {
    pub BLOCK_SHIFT: i32,
    pub BLOCK_SIZE: i32,
    pub mCache: Option<Vec<i32>>,
}
impl GridCache {
    // java public GridCache(int blockShift) {
    // java this.BLOCK_SHIFT = blockShift;
    // java this.BLOCK_SIZE = 1 << blockShift;
    // java }
    pub fn new(shift: i32) -> Self {
        Self {
            BLOCK_SHIFT: shift,
            BLOCK_SIZE: 1i32.wrapping_shl(shift as u32),
            mCache: None,
        }
    }
    // java public CacheInfo search(int y) {
    // java CacheInfo info = new CacheInfo();
    // java this.search(y, info);
    // java return info;
    // java }
    pub fn search(&self, y: i32) -> CacheInfo {
        let mut info = CacheInfo::default();
        self.search__info(y, &mut info);
        info
    }
    // java public void search(int y, CacheInfo info) {
    // java if (this.mCache != null) {
    // java int index = this.binarySearch(y, this.mCache);
    // java info.set(index, this.mCache, this.BLOCK_SHIFT);
    // java }
    // java }
    pub fn search__info(&self, y: i32, info: &mut CacheInfo) {
        if let Some(c) = &self.mCache {
            info.set(self.binarySearch(y, c), c, self.BLOCK_SHIFT);
        }
    }
    // java private int binarySearch(int pos, int[] cache) {
    // java if (pos == cache[0]) {
    // java return 0;
    // java } else {
    // java int lo = 0;
    // java int hi = cache.length - 1;
    // java
    // java while(lo <= hi) {
    // java int mid = lo + hi >>> 1;
    // java int midValCmp = cache[mid] - pos;
    // java if (midValCmp < 0) {
    // java lo = mid + 1;
    // java } else {
    // java if (midValCmp <= 0) {
    // java return mid;
    // java }
    // java
    // java hi = mid - 1;
    // java }
    // java }
    // java
    // java return lo;
    // java }
    // java }
    pub fn binarySearch(&self, pos: i32, cache: &[i32]) -> i32 {
        if pos == cache[0] {
            return 0;
        }
        let (mut lo, mut hi) = (0i32, cache.len() as i32 - 1);
        while lo <= hi {
            let mid = ((lo.wrapping_add(hi)) as u32 >> 1) as i32;
            let cmp = cache[mid as usize].wrapping_sub(pos);
            if cmp < 0 {
                lo = mid + 1;
            } else {
                if cmp <= 0 {
                    return mid;
                }
                hi = mid - 1;
            }
        }
        lo
    }
    // java public static int search(int pos, int[] cache) {
    // java if (pos == cache[0]) {
    // java return 0;
    // java } else {
    // java int lo = 0;
    // java int hi = cache.length - 1;
    // java
    // java while(lo <= hi) {
    // java int mid = lo + hi >>> 1;
    // java int midValCmp = cache[mid] - pos;
    // java if (midValCmp < 0) {
    // java lo = mid + 1;
    // java } else {
    // java if (midValCmp <= 0) {
    // java return mid;
    // java }
    // java
    // java hi = mid - 1;
    // java }
    // java }
    // java
    // java return lo > 0 ? lo - 1 : lo;
    // java }
    // java }
    pub fn search__static(pos: i32, cache: &[i32]) -> i32 {
        if pos == cache[0] {
            return 0;
        }
        let (mut lo, mut hi) = (0i32, cache.len() as i32 - 1);
        while lo <= hi {
            let mid = ((lo.wrapping_add(hi)) as u32 >> 1) as i32;
            let cmp = cache[mid as usize].wrapping_sub(pos);
            if cmp < 0 {
                lo = mid + 1;
            } else {
                if cmp <= 0 {
                    return mid;
                }
                hi = mid - 1;
            }
        }
        if lo > 0 {
            lo - 1
        } else {
            lo
        }
    }
    // java public int startPos(int rcIndex) {
    // java int index = rcIndex >> this.BLOCK_SHIFT;
    // java if (index >= this.mCache.length) {
    // java index = this.mCache.length - 1;
    // java }
    // java
    // java return index > 0 ? this.mCache[index - 1] : 0;
    // java }
    pub fn startPos(&self, rc: i32) -> i32 {
        let cache = self.mCache.as_ref().expect("NullPointerException");
        let mut index = rc >> self.BLOCK_SHIFT;
        if index >= cache.len() as i32 {
            index = cache.len() as i32 - 1;
        }
        if index > 0 {
            cache[index as usize - 1]
        } else {
            0
        }
    }
    // java public int startIndex(int rcIndex) {
    // java int index = rcIndex >> this.BLOCK_SHIFT;
    // java if (index >= this.mCache.length) {
    // java index = this.mCache.length - 1;
    // java }
    // java
    // java return index << this.BLOCK_SHIFT;
    // java }
    pub fn startIndex(&self, rc: i32) -> i32 {
        let cache = self.mCache.as_ref().expect("NullPointerException");
        let mut index = rc >> self.BLOCK_SHIFT;
        if index >= cache.len() as i32 {
            index = cache.len() as i32 - 1;
        }
        index << self.BLOCK_SHIFT
    }
}
// java public static class CacheInfo
#[derive(Clone, Debug)]
pub struct CacheInfo {
    pub mStartIndex: i32,
    pub mEndIndex: i32,
    pub mStartPos: i32,
    pub mEndPos: i32,
}
impl Default for CacheInfo {
    fn default() -> Self {
        Self {
            mStartIndex: -1,
            mEndIndex: -1,
            mStartPos: 0,
            mEndPos: 0,
        }
    }
}
impl CacheInfo {
    // java public void set(int index, int[] cache, int shift) {
    // java if (index + 1 >= cache.length) {
    // java index = cache.length - 1;
    // java }
    // java
    // java this.mStartIndex = index << shift;
    // java this.mEndIndex = index + 1 << shift;
    // java this.mStartPos = index > 0 ? cache[index - 1] : 0;
    // java this.mEndPos = cache[index];
    // java }
    pub fn set(&mut self, mut index: i32, cache: &[i32], shift: i32) {
        if index + 1 >= cache.len() as i32 {
            index = cache.len() as i32 - 1;
        }
        self.mStartIndex = index << shift;
        self.mEndIndex = (index + 1) << shift;
        self.mStartPos = if index > 0 {
            cache[index as usize - 1]
        } else {
            0
        };
        self.mEndPos = cache[index as usize];
    }
}
