// Java: public class BaseHash
pub struct BaseHash {
    // Java field: int blockSize
    pub(crate) block_size: i32,
    // Java field: byte[] buffer
    pub(crate) buffer: Vec<u8>,
    // Java field: long count
    pub(crate) count: i64,
    // Java field: int hashSize
    pub(crate) hash_size: i32,
    // Java field: String name
    pub(crate) name: String,
}

pub trait BaseHashExt {
    fn clone_hash(&self) -> BaseHash;
    fn getResult(&self) -> Vec<u8>;
    fn padBuffer(&self) -> Vec<u8>;
    fn resetContext(&mut self);
    fn selfTest(&self) -> bool;
    fn transform(&mut self, data: &[u8], offset: i32);
}

// Java: public class BaseHash
impl BaseHash {
    // Java: BaseHash(String name, int hashSize, int blockSize)
    pub fn new(name: &str, hash_size: i32, block_size: i32) -> Self {
        // Java: super();
        // Java: this.name = name;
        // Java: this.hashSize = hashSize;
        // Java: this.blockSize = blockSize;
        // Java: this.buffer = new byte[blockSize];
        // Java: this.resetContext();
        BaseHash {
            name: name.to_string(),
            hash_size,
            block_size,
            buffer: vec![0u8; block_size as usize],
            count: 0,
        }
    }

    // Java: int blockSize()
    pub fn blockSize(&self) -> i32 {
        // Java: return this.blockSize;
        self.block_size
    }

    // Java: abstract Object clone()

    // Java: byte[] digest()
    pub fn digest(&mut self, ext: &mut dyn BaseHashExt) -> Vec<u8> {
        // Java: byte[] tail = this.padBuffer();
        let tail = ext.padBuffer();
        // Java: this.update(tail, 0, tail.length);
        let tail_len = tail.len() as i32;
        self.update_range(ext, &tail, 0, tail_len);
        // Java: byte[] result = this.getResult();
        let result = ext.getResult();
        // Java: this.reset();
        self.reset(ext);
        // Java: return result;
        result
    }

    // Java: abstract byte[] getResult()

    // Java: int hashSize()
    pub fn hashSize(&self) -> i32 {
        // Java: return this.hashSize;
        self.hash_size
    }

    // Java: String name()
    pub fn name(&self) -> &str {
        // Java: return this.name;
        &self.name
    }

    // Java: abstract byte[] padBuffer()

    // Java: void reset()
    pub fn reset(&mut self, ext: &mut dyn BaseHashExt) {
        // Java: this.count = 0L;
        self.count = 0;
        // Java: for(int i = 0; i < this.blockSize; this.buffer[i++] = 0)
        for i in 0..self.block_size as usize {
            self.buffer[i] = 0;
        }
        // Java: this.resetContext();
        ext.resetContext();
    }

    // Java: abstract void resetContext()

    // Java: abstract boolean selfTest()

    // Java: abstract void transform(byte[] var1, int var2)

    // Java: void update(byte b)
    pub fn update_byte(&mut self, ext: &mut dyn BaseHashExt, b: u8) {
        // Java: int i = (int)(this.count % (long)this.blockSize);
        let i = (self.count % self.block_size as i64) as usize;
        // Java: ++this.count;
        self.count += 1;
        // Java: this.buffer[i] = b;
        self.buffer[i] = b;
        // Java: if (i == this.blockSize - 1)
        if i == (self.block_size - 1) as usize {
            // Java: this.transform(this.buffer, 0);
            let buf_copy = self.buffer.clone();
            ext.transform(&buf_copy, 0);
        }
    }

    // Java: void update(byte[] b)
    pub fn update_bytes(&mut self, ext: &mut dyn BaseHashExt, b: &[u8]) {
        // Java: this.update(b, 0, b.length);
        let len = b.len() as i32;
        self.update_range(ext, b, 0, len);
    }

    // Java: void update(byte[] b, int offset, int len)
    pub fn update_range(&mut self, ext: &mut dyn BaseHashExt, b: &[u8], offset: i32, len: i32) {
        // Java: int n = (int)(this.count % (long)this.blockSize);
        let mut n = (self.count % self.block_size as i64) as usize;
        // Java: this.count += (long)len;
        self.count += len as i64;
        // Java: int partLen = this.blockSize - n;
        let part_len = self.block_size as usize - n;
        // Java: int i = 0;
        let mut i: usize = 0;
        // Java: if (len >= partLen)
        if len as usize >= part_len {
            // Java: System.arraycopy(b, offset, this.buffer, n, partLen);
            self.buffer[n..n + part_len].copy_from_slice(&b[offset as usize..offset as usize + part_len]);
            // Java: this.transform(this.buffer, 0);
            let buf_copy = self.buffer.clone();
            ext.transform(&buf_copy, 0);
            // Java: for(i = partLen; i + this.blockSize - 1 < len; i += this.blockSize)
            i = part_len;
            while i + self.block_size as usize - 1 < len as usize {
                // Java: this.transform(b, offset + i);
                ext.transform(b, (offset as usize + i) as i32);
                i += self.block_size as usize;
            }
            // Java: n = 0;
            n = 0;
        }
        // Java: if (i < len)
        if i < len as usize {
            // Java: System.arraycopy(b, offset + i, this.buffer, n, len - i);
            let remaining = len as usize - i;
            self.buffer[n..n + remaining].copy_from_slice(&b[offset as usize + i..offset as usize + i + remaining]);
        }
    }
}
