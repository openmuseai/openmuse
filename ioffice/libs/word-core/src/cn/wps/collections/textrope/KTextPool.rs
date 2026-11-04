// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import java.util.ArrayList
// Java: import cn.wps.collections.textrope.ITextPool
use crate::cn::wps::collections::textrope::ITextPool::*;
// Java: import java.lang.CharSequence
// soft: CharSequence → &str (matches ITextPool)

// java private static final int BLOCK_SIZE_SHIFT = 13;
const BLOCK_SIZE_SHIFT: i32 = 13;
// java private static final int BLOCK_SIZE = 8192;
const BLOCK_SIZE: i32 = 8192;

fn arraycopy_chars(src: &[char], src_pos: i32, dst: &mut [char], dst_pos: i32, length: i32) {
    let sp = src_pos as usize;
    let dp = dst_pos as usize;
    let n = length as usize;
    dst[dp..dp + n].copy_from_slice(&src[sp..sp + n]);
}

// java public class KTextPool implements ITextPool
#[derive(Default)]
pub struct KTextPool {
    // java private ArrayList<char[]> blocks = new ArrayList();
    blocks: Option<Vec<Vec<char>>>,
    // java private int lastBlockSize;
    lastBlockSize: i32,
}

// java public interface ITextPool
impl ITextPool for KTextPool {
    // java public int append(char c)
    fn append__char(&mut self, c: char) -> i32 {
      // java int len = this.length();
      let len: i32 = self.length();
      // java int nAvail = 8192 - this.lastBlockSize;
      let nAvail: i32 = 8192 - self.lastBlockSize;
      // java if (nAvail == 0)
      if nAvail == 0 {
        // java char[] block = new char[8192];
        let mut block: Vec<char> = vec!['\0'; 8192usize];
        // java this.blocks.add(block);
        // java block[0] = c;
        block[0usize] = c;
        if let Some(blocks) = self.blocks.as_mut() {
          blocks.push(block);
        }
        // java this.lastBlockSize = 1;
        self.lastBlockSize = 1;
      }
      // java else
      else {
        // java char[] block = (char[])this.blocks.get(this.blocks.size() - 1);
        // java block[this.lastBlockSize++] = c;
        if let Some(blocks) = self.blocks.as_mut() {
          let idx = blocks.len() - 1;
          let pos = self.lastBlockSize as usize;
          blocks[idx][pos] = c;
          self.lastBlockSize += 1;
        }
      }
      // java return len;
      return len;
    }

    // java public int append(char[] s, int start, int end)
    fn append__chararr_int_int(&mut self, s: &Vec<char>, start: i32, end: i32) -> i32 {
      // java int len;
      let mut len: i32;
      // java int nAdd;
      let mut nAdd: i32;
      let mut start = start;
      len = self.length();
      // java for(len = this.length() start < end this.lastBlockSize += nAdd)
      while start < end {
        // java char[] block = null;
        let mut block_is_new = false;
        // java int nAvail = 8192 - this.lastBlockSize;
        let mut nAvail: i32 = 8192 - self.lastBlockSize;
        // java if (nAvail == 0)
        if nAvail == 0 {
          // java block = new char[8192];
          // java this.lastBlockSize = 0;
          self.lastBlockSize = 0;
          // java this.blocks.add(block);
          if let Some(blocks) = self.blocks.as_mut() {
            blocks.push(vec!['\0'; 8192usize]);
          }
          // java nAvail = 8192;
          nAvail = 8192;
          block_is_new = true;
        }
        // java else
        else {
          // java block = (char[])this.blocks.get(this.blocks.size() - 1);
          let _ = block_is_new;
        }
        // java nAdd = Math.min(nAvail, end - start);
        nAdd = std::cmp::min(nAvail, end - start);
        // java System.arraycopy(s, start, block, this.lastBlockSize, nAdd);
        if let Some(blocks) = self.blocks.as_mut() {
          let bi = blocks.len() - 1;
          arraycopy_chars(s, start, &mut blocks[bi], self.lastBlockSize, nAdd);
        }
        // java start += nAdd;
        start = start + nAdd;
        self.lastBlockSize += nAdd;
      }
      // java return len;
      return len;
    }

    // java public int append(CharSequence s)
    fn append__charsequence(&mut self, s: &str) -> i32 {
      // java return this.append((CharSequence)s, 0, s.length());
      return self.append__charsequence_int_int(Some(s), 0, s.chars().count() as i32);
    }

    // java public int append(CharSequence s, int start, int end)
    fn append__charsequence_int_int(&mut self, s: Option<&str>, start: i32, end: i32) -> i32 {
      // java int len = this.length();
      let len: i32 = self.length();
      // java if (s == null)
      if s.is_none() {
        // java return len;
        return len;
      }
      // java else
      else {
        let s = s.unwrap();
        let mut start = start;
        // java while (start < end)
        while start < end {
          // java char[] block = null;
          // java int nAvail = 8192 - this.lastBlockSize;
          let mut nAvail: i32 = 8192 - self.lastBlockSize;
          // java if (nAvail == 0)
          if nAvail == 0 {
            // java block = new char[8192];
            // java this.lastBlockSize = 0;
            self.lastBlockSize = 0;
            // java this.blocks.add(block);
            if let Some(blocks) = self.blocks.as_mut() {
              blocks.push(vec!['\0'; 8192usize]);
            }
            // java nAvail = 8192;
            nAvail = 8192;
          }
          // java else
          else {
            // java block = (char[])this.blocks.get(this.blocks.size() - 1);
          }
          // java int nAdd = Math.min(nAvail, end - start);
          let nAdd: i32 = std::cmp::min(nAvail, end - start);
          // java getChars(s, start, start + nAdd, block, this.lastBlockSize);
          if let Some(blocks) = self.blocks.as_mut() {
            let bi = blocks.len() - 1;
            Self::getChars__charsequence_int_int_chararr_int(
              s,
              start,
              start + nAdd,
              &mut blocks[bi],
              self.lastBlockSize,
            );
          }
          // java start += nAdd;
          start = start + nAdd;
          // java this.lastBlockSize += nAdd;
          self.lastBlockSize = self.lastBlockSize + nAdd;
        }
        // java return len;
        return len;
      }
    }

    // java public void dispose()
    fn dispose(&mut self) {
      // java this.blocks.clear();
      if let Some(blocks) = self.blocks.as_mut() {
        blocks.clear();
      }
      // java this.blocks = null;
      self.blocks = None;
    }

    // java public char[] getChars(int start, int end)
    fn getChars__int_int(&self, start: i32, end: i32) -> Vec<char> {
      // java Assert.assertTrue("(start <= end) should be true.", start <= end);
      Assert::assertTrue_msg("(start <= end) should be true.", start <= end);
      // java char[] buf = new char[end - start];
      let mut buf: Vec<char> = vec!['\0'; (end - start) as usize];
      // java this.getChars(start, end, buf, 0);
      self.getChars__int_int_chararr_int(start, end, &mut buf, 0);
      // java return buf;
      return buf;
    }

    // java public void getChars(int start, int end, char[] buffer, int index)
    fn getChars__int_int_chararr_int(&self, start: i32, end: i32, buffer: &mut [char], index: i32) {
      // java if (buffer != null)
      {
        let mut start = start;
        let mut index = index;
        // java Assert.assertTrue("(end - start) <= (buffer.length - index) should be true", end - start <= buffer.length - index);
        Assert::assertTrue_msg(
          "(end - start) <= (buffer.length - index) should be true",
          end - start <= (buffer.len() as i32) - index,
        );
        // java while (start < end)
        while start < end {
          // java char[] block = (char[])this.blocks.get(start / 8192);
          let block = self
            .blocks
            .as_ref()
            .unwrap()
            .get((start / 8192) as usize)
            .unwrap();
          // java int ioffset = start % 8192;
          let ioffset: i32 = start % 8192;
          // java int nFill = Math.min(8192 - ioffset, end - start);
          let nFill: i32 = std::cmp::min(8192 - ioffset, end - start);
          // java System.arraycopy(block, ioffset, buffer, index, nFill);
          arraycopy_chars(block, ioffset, buffer, index, nFill);
          // java start += nFill;
          start = start + nFill;
          // java index += nFill;
          index = index + nFill;
        }
      }
    }

    // java public void set(int index, char c)
    fn set(&mut self, index: i32, c: char) {
      // java int iblock = index / 8192;
      let iblock: i32 = index / 8192;
      // java int ioffset = index % 8192;
      let ioffset: i32 = index % 8192;
      // java ((char[])this.blocks.get(iblock))[ioffset] = c;
      if let Some(blocks) = self.blocks.as_mut() {
        blocks[iblock as usize][ioffset as usize] = c;
      }
    }

    // java public char charAt(int index) — CharSequence, same object as set/append
    fn charAt(&self, index: i32) -> char {
      KTextPool::charAt(self, index)
    }

}

// java public class KTextPool implements ITextPool
impl KTextPool {
    // java public  KTextPool()
    pub fn KTextPool() -> Self {
      // java char[] block = new char[8192];
      let block: Vec<char> = vec!['\0'; 8192usize];
      // java this.blocks.add(block);
      // java this.lastBlockSize = 0;
      let mut this = Self {
        blocks: Some(Vec::new()),
        lastBlockSize: 0,
      };
      this.blocks.as_mut().unwrap().push(block);
      this
    }

    // java public int length()
    pub fn length(&self) -> i32 {
      // java int size = this.blocks.size();
      let size: i32 = (self.blocks.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java Assert.assertTrue("size should be greater than 0.", size > 0);
      Assert::assertTrue_msg("size should be greater than 0.", size > 0);
      // java return (size - 1 << 13) + this.lastBlockSize;
      return ((size - 1) << BLOCK_SIZE_SHIFT) + self.lastBlockSize;
    }

    // java public char charAt(int index)
    pub fn charAt(&self, index: i32) -> char {
      // java int iblock = index / 8192;
      let iblock: i32 = index / 8192;
      // java int ioffset = index % 8192;
      let ioffset: i32 = index % 8192;
      // java return ((char[])this.blocks.get(iblock))[ioffset];
      return self.blocks.as_ref().unwrap()[iblock as usize][ioffset as usize];
    }

    // java public CharSequence subSequence(int start, int end)
    pub fn subSequence(&mut self, start: i32, end: i32) -> Option<String> {
      let _ = (start, end);
      // java Assert.fail("Not implemented yet.");
      Assert::fail_msg("Not implemented yet.");
      // java return null;
      return None;
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java int MAX = 100;
      let MAX: i32 = 100;
      let _ = MAX;
      // java int len = this.length();
      let len: i32 = self.length();
      // java char[] buffer = this.getChars(0, Math.min(100, len));
      let buffer: Vec<char> = self.getChars__int_int(0, std::cmp::min(100, len));
      // java return len <= 100 ? new String(buffer) : new String(buffer) + "...";
      let s: String = buffer.iter().collect();
      return if len <= 100 { s } else { s + "..." };
    }

    // java private static void getChars(CharSequence s, int srcBegin, int srcEnd, char[] dst, int dstBegin)
    pub fn getChars__charsequence_int_int_chararr_int(
      s: &str,
      srcBegin: i32,
      srcEnd: i32,
      dst: &mut [char],
      dstBegin: i32,
    ) {
      // java if (s instanceof String)
      // soft: &str always string path — copy chars by index
      {
        // java ((String)s).getChars(srcBegin, srcEnd, dst, dstBegin);
        let chars: Vec<char> = s.chars().collect();
        let mut srcBegin = srcBegin;
        let mut dstBegin = dstBegin;
        // java else while path also covered by indexed copy
        // java while (srcBegin < srcEnd)
        while srcBegin < srcEnd {
          // java char c = s.charAt(srcBegin);
          let c: char = chars[srcBegin as usize];
          // java dst[dstBegin] = c;
          dst[dstBegin as usize] = c;
          // java ++srcBegin;
          srcBegin += 1;
          // java ++dstBegin;
          dstBegin += 1;
        }
      }
    }

}

// silence unused const warning when only BLOCK_SIZE_SHIFT used in length
#[allow(dead_code)]
const _BLOCK_SIZE: i32 = BLOCK_SIZE;
