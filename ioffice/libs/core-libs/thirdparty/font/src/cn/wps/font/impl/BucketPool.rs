// Java field: int EACH_HOLD
const EACH_HOLD: usize = 4;

// Java: public class BucketPool
pub struct BucketPool {
    // Java field: char[][] buckets
    buckets: Vec<Option<Vec<u16>>>,
}

// Java: public class BucketPool
impl BucketPool {
    pub fn new() -> Self {
        BucketPool {
            buckets: (0..32).map(|_| None).collect(),
        }
    }

    // Java: char[] alloc(int size)
    pub fn alloc(&mut self, size: usize) -> Vec<u16> {
        // Java: int start = ((size >> 1) - 1) * 4;
        let start = ((size >> 1) - 1) * EACH_HOLD;
        // Java: for(int i = 0; i < 4; ++i)
        for i in 0..EACH_HOLD {
            // Java: if (this.buckets[start + i] != null)
            if self.buckets[start + i].is_some() {
                // Java: char[] bucket = this.buckets[start + i];
                // Java: this.buckets[start + i] = null;
                let bucket = self.buckets[start + i].take().unwrap();
                // Java: return bucket;
                return bucket;
            }
        }
        // Java: return new char[size];
        vec![0u16; size]
    }

    // Java: void free(char[] bucket)
    pub fn free(&mut self, bucket: Vec<u16>) {
        // Java: int start = ((bucket.length >> 1) - 1) * 4;
        let start = ((bucket.len() >> 1) - 1) * EACH_HOLD;
        // Java: for(int i = 0; i < 4; ++i)
        for i in 0..EACH_HOLD {
            // Java: if (this.buckets[start + i] == null)
            if self.buckets[start + i].is_none() {
                // Java: this.buckets[start + i] = bucket;
                self.buckets[start + i] = Some(bucket);
                // Java: return;
                return;
            }
        }
    }
}
