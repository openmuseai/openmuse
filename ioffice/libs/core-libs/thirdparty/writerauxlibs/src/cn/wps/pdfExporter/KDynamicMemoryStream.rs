// Java: import java.io.IOException
use std::sync::Mutex;

// Java field: int MINBLOCKSIZE
pub(crate) const MINBLOCKSIZE: i32 = 512;

#[allow(non_upper_case_globals)]
static mEmptyBlock: Mutex<Option<Box<Block>>> = Mutex::new(None);

// Java: public class KDynamicMemoryStream extends KStream
pub struct KDynamicMemoryStream {
    // Java field: Block head
    pub(crate) head: Option<Box<Block>>,
    // Java field: Block tail
    tail: *mut Block,
    // Java field: Block mEmptyBlock
    pub(crate) length: i32,
}

// Java: public class Block
pub struct Block {
    // Java field: Block next
    pub(crate) next: Option<Box<Block>>,
    // Java field: byte[] buf
    pub(crate) buf: Vec<u8>,
    // Java field: int start
    pub(crate) start: i32,
}

// Java: public class KDynamicMemoryStream extends KStream
impl KDynamicMemoryStream {
    // Java: KDynamicMemoryStream()
    pub fn new() -> Self {
      // Java: super();
      Self {
          head: None,
          tail: std::ptr::null_mut(),
          length: 0,
      }
    }

    // Java: static Block getBlock(int size)
    pub fn getBlock(size: i32) -> Option<Box<Block>> {
      // Java: if (size == 512 && mEmptyBlock != null)
      if size == 512
      {
         let mut pool = mEmptyBlock.lock().unwrap();
         if pool.is_some() {
             // Java: Block block = mEmptyBlock;
             let mut block = pool.take().unwrap();
             // Java: mEmptyBlock = mEmptyBlock.next;
             *pool = block.next.take();
             // Java: block.next = null;
             block.next = None;
             // Java: block.start = 0;
             block.start = 0;
             // Java: return block;
             return Some(block);
         }
      }
      // Java: else
      {
         // Java: return null;
         return None;
      }
    }

    // Java: static void releaseBlock(Block block)
    pub fn releaseBlock(mut block: Box<Block>) {
      // Java: if (block.buf.length == 512)
      if block.buf.len() == 512
      {
         let mut pool = mEmptyBlock.lock().unwrap();
         // Java: block.next = mEmptyBlock;
         block.next = pool.take().map(|b| b);
         // Java: mEmptyBlock = block;
         *pool = Some(block);
      }
    }

    // Java: static void clearBlock()
    pub fn clearBlock() {
      // Java: mEmptyBlock = null;
      let mut pool = mEmptyBlock.lock().unwrap();
      *pool = None;
    }

    // Java: Block newBlock(int size)
    pub fn newBlock(&self, size: i32) -> Box<Block> {
      // Java: Block block = getBlock(size);
      let block = Self::getBlock(size);
      // Java: if (block == null)
      if let Some(block) = block
      {
         return block;
      }
      {
         // Java: block = new Block(size);
         return Box::new(Block::new(size));
      }
      // Java: return block;
    }

    // Java: void write(byte[] data, int start, int count)
    pub fn write_bytes_range(&mut self, data: &[u8], mut start: i32, mut count: i32) {
      // Java: this.length += count;
      self.length += count;
      // Java: if (this.tail != null)
      if !self.tail.is_null()
      {
         let tail = unsafe { &mut *self.tail };
         // Java: int avail = this.tail.avail();
         let avail = tail.avail();
         // Java: if (avail > 0)
         if avail > 0
         {
            // Java: int size = Math.min(avail, count);
            let size = avail.min(count);
            // Java: this.tail.append(data, start, size);
            tail.append(data, start, size);
            // Java: if (size == count)
            if size == count
            {
               // Java: return;
               return;
            }
            // Java: start += size;
            start += size;
            // Java: count -= size;
            count -= size;
         }
      }
      // Java: int size = Math.max(count, 512);
      let size = count.max(512);
      // Java: Block block = this.newBlock(size);
      let mut block = self.newBlock(size);
      // Java: block.append(data, start, count);
      block.append(data, start, count);
      // Java: if (this.tail != null)
      if !self.tail.is_null()
      {
         let tail = unsafe { &mut *self.tail };
         // Java: this.tail.next = block;
         tail.next = Some(block);
         // Java: this.tail = block;
         self.tail = tail.next.as_mut().unwrap().as_mut() as *mut Block;
      }
      // Java: else
      else
      {
         // Java: this.head = this.tail = block;
         self.head = Some(block);
         self.tail = self.head.as_mut().unwrap().as_mut() as *mut Block;
      }
    }

    // Java: byte[] copyData()
    pub fn copyData(&self) -> Vec<u8> {
      // Java: byte[] buf = new byte[this.length];
      let mut buf = vec![0u8; self.length as usize];
      // Java: Block block = this.head;
      let mut block = self.head.as_ref();
      // Java: for(int pos = 0; block != null; block = block.next)
      let mut pos: usize = 0;
      while let Some(b) = block
      {
         // Java: System.arraycopy(block.buf, 0, buf, pos, block.start);
         let len = b.start as usize;
         buf[pos..pos + len].copy_from_slice(&b.buf[..len]);
         // Java: pos += block.start;
         pos += len;
         block = b.next.as_ref();
      }
      // Java: return buf;
      buf
    }

    // Java: Block getHead()
    pub fn getHead(&self) -> Option<&Block> {
      // Java: return this.head;
      self.head.as_deref()
    }

    // Java: int read(byte[] buf, int offset, int start, int count)
    pub fn read_to_buf_range(&self, buf: &mut [u8], offset: i32, mut start: i32, mut count: i32) -> i32 {
      // Java: if (start >= this.length)
      if start >= self.length
      {
         // Java: return 0;
         return 0;
      }
      // Java: else
      {
         // Java: Block block;
         // Java: for(block = this.head; block != null && start >= block.start; block = block.next)
         let mut block_ref = self.head.as_ref();
         while let Some(b) = block_ref {
             if start < b.start { break; }
         {
            // Java: start -= block.start;
            start -= b.start;
         }
             block_ref = b.next.as_ref();
         }
         // Java: int read;
         let mut read: i32 = 0;
         // Java: for(read = 0; count > 0 && block != null; block = block.next)
         while count > 0 && block_ref.is_some()
         {
            let b = block_ref.unwrap();
            // Java: int size = Math.min(block.start - start, count);
            let size = (b.start - start).min(count);
            // Java: System.arraycopy(block.buf, start, buf, offset + read, size);
            let dst_start = (offset + read) as usize;
            let src_start = start as usize;
            let sz = size as usize;
            buf[dst_start..dst_start + sz].copy_from_slice(&b.buf[src_start..src_start + sz]);
            // Java: count -= size;
            count -= size;
            // Java: read += size;
            read += size;
            // Java: start = 0;
            start = 0;
            block_ref = b.next.as_ref();
         }
         // Java: return read;
         return read;
      }
    }

    // Java: byte read(int pos)
    pub fn read_at(&self, mut pos: i32) -> i8 {
      // Java: if (pos >= this.length)
      if pos >= self.length
      {
         // Java: return 0;
         return 0;
      }
      // Java: else
      {
         // Java: for(Block block = this.head; block != null; block = block.next)
         let mut block = self.head.as_ref();
         while let Some(b) = block
         {
            // Java: if (pos < block.start)
            if pos < b.start
            {
               // Java: return block.buf[pos];
               return b.buf[pos as usize] as i8;
            }
            // Java: pos -= block.start;
            pos -= b.start;
            block = b.next.as_ref();
         }
         // Java: return 0;
         return 0;
      }
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: Block next = this.head;
      // Java: while(next != null)
      while let Some(mut block) = self.head.take()
      {
         // Java: Block block = next;
         // Java: next = next.next;
         self.head = block.next.take();
         // Java: releaseBlock(block);
         Self::releaseBlock(block);
      }
      // Java: this.head = this.tail = null;
      self.head = None;
      self.tail = std::ptr::null_mut();
    }
}

// Java: public class Block
impl Block {
    // Java: Block(int size)
    pub fn new(size: i32) -> Self {
         // Java: super();
         // Java: this.buf = new byte[size];
         // Java: this.start = 0;
         // Java: this.next = null;
         Self {
             buf: vec![0u8; size as usize],
             start: 0,
             next: None,
         }
    }

    // Java: int avail()
    pub fn avail(&self) -> i32 {
         // Java: return this.buf.length - this.start;
         self.buf.len() as i32 - self.start
    }

    // Java: void append(byte[] data, int start, int count)
    pub fn append(&mut self, data: &[u8], start: i32, count: i32) {
         // Java: System.arraycopy(data, start, this.buf, this.start, count);
         let src_start = start as usize;
         let dst_start = self.start as usize;
         let cnt = count as usize;
         self.buf[dst_start..dst_start + cnt].copy_from_slice(&data[src_start..src_start + cnt]);
         // Java: this.start += count;
         self.start += count;
    }

    // Java: byte[] getBuffer()
    pub fn getBuffer(&self) -> &[u8] {
         // Java: return this.buf;
         &self.buf
    }

    // Java: int getLen()
    pub fn getLen(&self) -> i32 {
         // Java: return this.start;
         self.start
    }

    // Java: Block getNext()
    pub fn getNext(&self) -> Option<&Block> {
         // Java: return this.next;
         self.next.as_deref()
    }
}
