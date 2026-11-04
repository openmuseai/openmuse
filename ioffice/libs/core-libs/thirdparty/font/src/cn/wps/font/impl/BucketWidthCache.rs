// Java: import java.util.Arrays
use super::BucketPool::BucketPool;

// Java field: int BUCKETS
const BUCKETS: usize = 512;
// Java field: int MAX_BUCKET_SIZE
pub(crate) const MAX_BUCKET_SIZE: usize = 8;
// Java field: float NOT_CACHED
const NOT_CACHED: f32 = -1.0;
// Java field: char MAX_ASCII
const MAX_ASCII: u16 = 0x7F;

// Java: public class BucketWidthCache
pub struct BucketWidthCache {
    // Java field: char[][] buckets
    buckets: Vec<Option<Vec<u16>>>,
    // Java field: BucketPool bucketPool
    bucketPool: BucketPool,
    // Java field: float[] asciiWidths
    pub(crate) asciiWidths: Vec<f32>,
    // Java field: int abandon
    abandon: usize,
}

// Java: public class BucketWidthCache
impl BucketWidthCache {
    // Java: BucketWidthCache()
    pub fn new() -> Self {
        // Java: super();
        // Java: Arrays.fill(this.asciiWidths, -1.0F);
        // Java: this.abandon = 0;
        BucketWidthCache {
            buckets: (0..BUCKETS).map(|_| None).collect(),
            bucketPool: BucketPool::new(),
            asciiWidths: vec![-1.0f32; 128],
            abandon: 0,
        }
    }

    // Java: float findWidth(char ch)
    pub fn findWidth(&self, ch: u16) -> f32 {
        // Java: if (ch <= 127)
        if ch <= MAX_ASCII {
            // Java: return this.asciiWidths[ch];
            return self.asciiWidths[ch as usize];
        }
        // Java: else
        {
            // Java: char[] bucket = this.buckets[ch & 511];
            let bucket = &self.buckets[(ch as usize) & (BUCKETS - 1)];
            // Java: if (bucket != null)
            if let Some(ref bucket) = bucket {
                // Java: for(int i = bucket.length - 2; i >= 0; i -= 2)
                let mut i = bucket.len() as i32 - 2;
                while i >= 0 {
                    // Java: if (bucket[i] == ch)
                    if bucket[i as usize] == ch {
                        // Java: return (float)bucket[i + 1];
                        return bucket[(i + 1) as usize] as f32;
                    }
                    i -= 2;
                }
            }
            // Java: return -1.0F;
            NOT_CACHED
        }
    }

    // Java: int nextAbandon()
    fn nextAbandon(&mut self) -> usize {
        // Java: int ans = this.abandon << 1;
        let ans = self.abandon << 1;
        // Java: this.abandon = this.abandon + 1 & 7;
        self.abandon = (self.abandon + 1) & 7;
        // Java: return ans;
        ans
    }

    // Java: void addWidth(char ch, float width)
    pub fn addWidth(&mut self, ch: u16, width: f32) {
        // Java: if (ch <= 127)
        if ch <= MAX_ASCII {
            // Java: this.asciiWidths[ch] = width;
            self.asciiWidths[ch as usize] = width;
        }
        // Java: else
        else {
            // Java: char[] bucket = this.buckets[ch & 511];
            let bucket_idx = (ch as usize) & (BUCKETS - 1);
            // Java: int i;
            let i: usize;
            // Java: if (bucket == null)
            if self.buckets[bucket_idx].is_none() {
                // Java: i = 0;
                i = 0;
                // Java: bucket = this.bucketPool.alloc(2);
                let bucket = self.bucketPool.alloc(2);
                // Java: this.buckets[ch & 511] = bucket;
                self.buckets[bucket_idx] = Some(bucket);
            }
            // Java: else if (bucket.length < 16)
            else if self.buckets[bucket_idx].as_ref().unwrap().len() < 16 {
                // Java: i = bucket.length;
                i = self.buckets[bucket_idx].as_ref().unwrap().len();
                // Java: char[] newBucket = this.bucketPool.alloc(bucket.length + 2);
                let mut new_bucket = self.bucketPool.alloc(i + 2);
                // Java: System.arraycopy(bucket, 0, newBucket, 0, bucket.length);
                let old_bucket = self.buckets[bucket_idx].take().unwrap();
                new_bucket[..old_bucket.len()].copy_from_slice(&old_bucket);
                // Java: this.bucketPool.free(bucket);
                self.bucketPool.free(old_bucket);
                // Java: bucket = this.buckets[ch & 511] = newBucket;
                self.buckets[bucket_idx] = Some(new_bucket);
            }
            // Java: else
            else {
                // Java: i = this.nextAbandon();
                i = self.nextAbandon();
            }
            // Java: bucket[i] = ch;
            if let Some(ref mut bucket) = self.buckets[bucket_idx] {
                bucket[i] = ch;
                // Java: bucket[i + 1] = (char)((int)width);
                bucket[i + 1] = width as u16;
            }
        }
    }

    // Java: void addWidth(char[] text, int textPos, float[] widths, int widthPos, int length)
    pub fn addWidth_batch(&mut self, text: &[u16], text_pos: usize, widths: &[f32], width_pos: usize, length: usize) {
        // Java: int i = textPos;
        let mut wp = width_pos;
        // Java: for(int n = textPos + length; i < n; ++i)
        for i in text_pos..(text_pos + length) {
            // Java: this.addWidth(text[i], widths[widthPos++]);
            self.addWidth(text[i], widths[wp]);
            wp += 1;
        }
    }
}
