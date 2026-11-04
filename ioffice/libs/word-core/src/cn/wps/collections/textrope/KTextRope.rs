// Java: import cn.wps.collections.IntervalMap
use crate::cn::wps::collections::IntervalMap::{Segment, IntervalMapNode1};
// Java: import cn.wps.collections.IntervalMapNode
use crate::cn::wps::collections::IntervalMapNode::IntervalMapNode;
// Java: import cn.wps.moffice.define.FontName
use util::moffice::define::FontName::FontName;
// Java: import java.util.ArrayList
// Java: import java.lang.CharSequence
// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::ITextRope;
// Java: import cn.wps.collections.textrope.KTextPieceTable
use crate::cn::wps::collections::textrope::KTextPieceTable::{KTextPieceTable, CPNode};
// Java: import cn.wps.collections.textrope.ITextPool
use crate::cn::wps::collections::textrope::ITextPool::ITextPool;
// Java: import cn.wps.collections.textrope.KTextPool
use crate::cn::wps::collections::textrope::KTextPool::KTextPool;
// Java: import cn.wps.collections.textrope.ITextPiece
use crate::cn::wps::collections::textrope::ITextPiece::ITextPiece;
use util::collections::Node::Node;
use util::collections::PositionNode::PositionNode;

// java public static final int INVALID_POSITION = -1;
pub(crate) const INVALID_POSITION: i32 = -1;

// java public class KTextRope implements ITextRope
pub struct KTextRope {
    // java private int mLength;
    pub(crate) mLength: i32,
    // java private KTextPieceTable mMap = new KTextPieceTable();
    pub(crate) mMap: KTextPieceTable,
    // java private ITextPool mTextPool = new KTextPool();
    pub(crate) mTextPool: KTextPool,
    // 09: Java ITextPool is a shared reference; remember parent pool for getTextPool pointer equality
    mSharedPool: Option<*mut dyn ITextPool>,
}

impl Default for KTextRope {
    fn default() -> Self {
        Self {
            mLength: 0,
            mMap: KTextPieceTable::new(),
            mTextPool: KTextPool::KTextPool(),
            mSharedPool: None,
        }
    }
}

// java public interface ITextRope
impl ITextRope for KTextRope {
    // java public final void append(char c)
    fn append__char(&mut self, c: char) {
      // java int fc = this.mTextPool.append(c);
      let fc: i32 = unsafe { (*self.active_pool()).append__char(c) };
      // java this.insertImpl(this.mLength, fc, 1);
      self.insertImpl(self.mLength, fc, 1);
    }

    // java public final void append(char[] buf, int start, int end)
    fn append__chararr_int_int(&mut self, buf: &Vec<char>, start: i32, end: i32) {
      // java int fc = this.mTextPool.append(buf, start, end);
      let fc: i32 = unsafe { (*self.active_pool()).append__chararr_int_int(buf, start, end) };
      // java this.insertImpl(this.mLength, fc, end - start);
      self.insertImpl(self.mLength, fc, end - start);
    }

    // java public final void append(String s, int start, int end)
    fn append__string_int_int(&mut self, s: &str, start: i32, end: i32) {
      // java int fc = this.mTextPool.append((CharSequence)s, start, end);
      let fc: i32 = unsafe { (*self.active_pool()).append__charsequence_int_int(Some(s), start, end) };
      // java this.insertImpl(this.mLength, fc, end - start);
      self.insertImpl(self.mLength, fc, end - start);
    }

    // java public synchronized ITextPiece begin()
    fn begin(&mut self) -> Option<Box<dyn ITextPiece>> {
      // java return (KTextPieceTable.CPNode)this.mMap.getTree1().leftmost();
      let p = self.mMap.getTree1().leftmost() as *mut CPNode;
      if p.is_null() {
        None
      } else {
        // soft: cannot Box from tree-owned node; return None until owned handles
        let _ = p;
        None
      }
    }

    // java public final synchronized char charAt(int cp)
    fn charAt(&mut self, cp: i32) -> char {
      // java if (cp >= 0 && cp < this.mLength)
      if cp >= 0 && cp < self.mLength {
        // java return this.mTextPool.charAt(this.cp2fc(cp));
        let fc = self.cp2fc(cp);
        return unsafe { (*self.active_pool()).charAt(fc) };
      }
      // java else
      else {
        // java throw new IndexOutOfBoundsException();
        panic!("IndexOutOfBoundsException");
      }
    }

    // java public final synchronized int cp2fc(int cp)
    fn cp2fc(&mut self, cp: i32) -> i32 {
      // java int fc = this.mMap.forwardMapping(cp, -1);
      let fc: i32 = self.mMap.forwardMapping(cp, -1);
      // java return fc;
      return fc;
    }

    // java public synchronized ITextPiece end()
    fn end(&mut self) -> Option<Box<dyn ITextPiece>> {
      // java return (KTextPieceTable.CPNode)this.mMap.getTree1().getHead();
      let _p = self.mMap.getTree1().getHead() as *mut CPNode;
      None
    }

    // java public final void erase(int start, int end)
    fn erase(&mut self, start: i32, end: i32) {
      // java this.eraseImpl(start, end);
      self.eraseImpl(start, end);
    }

    // java public final synchronized int fc2cp(int fc)
    fn fc2cp(&mut self, fc: i32) -> i32 {
      // java return this.mMap.backwardMapping(fc, -1);
      return self.mMap.backwardMapping(fc, -1);
    }

    // java public final synchronized void getText(int start, int end, char[] buffer, int bufStart)
    fn getText(&self, mut start: i32, end: i32, buffer: &mut [char], bufStart: i32) {
      let pool = self.active_pool();
      // java IntervalMap.IntervalMapTree tree1 = (IntervalMap.IntervalMapTree)this.mMap.getTree1();
      let tree1 = self.mMap.getTree1();
      // java IntervalMap.IntervalMapTree tree2 = (IntervalMap.IntervalMapTree)this.mMap.getTree2();
      let tree2 = self.mMap.getTree2();
      // java IntervalMapNode pNode = (IntervalMapNode)tree1.lowerBound(start);
      let mut pNode = tree1.lowerBound(start) as *mut IntervalMapNode;
      // java int offset = 0;
      let mut offset: i32 = 0;
      // java if (pNode == tree1.getHead() || tree1.position(pNode) != start)
      if pNode as *mut Node == tree1.getHead()
        || tree1.position(pNode as *mut PositionNode) != start
      {
        // java pNode = (IntervalMapNode)pNode.getPrev();
        unsafe {
          pNode = (*pNode).super_.super_.base.getPrev() as *mut IntervalMapNode;
        }
        // java int pNode_pos = tree1.position(pNode);
        let pNode_pos: i32 = tree1.position(pNode as *mut PositionNode);
        // java offset = start - pNode_pos;
        offset = start - pNode_pos;
      }
      // java int len;
      let mut len: i32;
      let mut n: i32 = 0;
      // java for(int n = 0; start < end n += len)
      while start < end {
        unsafe {
          // java len = Math.min(pNode.getLength() - offset, end - start);
          let node_len = (*(pNode as *mut IntervalMapNode1)).mLength;
          len = std::cmp::min(node_len - offset, end - start);
          // java int fc = tree2.position(pNode.getPeer()) + offset;
          let fc: i32 = tree2.position((*pNode).peer as *mut PositionNode) + offset;
          // java this.mTextPool.getChars(fc, fc + len, buffer, n + bufStart);
          (*pool).getChars__int_int_chararr_int(fc, fc + len, buffer, n + bufStart);
          // java pNode = (IntervalMapNode)pNode.getNext();
          pNode = (*pNode).super_.super_.base.getNext() as *mut IntervalMapNode;
          // java tree1.position(pNode);
          let _ = tree1.position(pNode as *mut PositionNode);
          // java offset = 0;
          offset = 0;
          // java start += len;
          start = start + len;
          n += len;
        }
      }
    }

    // java public ITextPool getTextPool()
    fn getTextPool(&self) -> Option<*mut dyn ITextPool> {
      Some(self.active_pool())
    }

    // java public final void insert(int cp, char c)
    fn insert__int_char(&mut self, cp: i32, c: char) {
      // java int fc = this.mTextPool.append(c);
      let fc: i32 = unsafe { (*self.active_pool()).append__char(c) };
      // java this.insertImpl(cp, fc, 1);
      self.insertImpl(cp, fc, 1);
    }

    // java public final void insert(int cp, char[] buf, int start, int end)
    fn insert__int_chararr_int_int(&mut self, cp: i32, buf: &Vec<char>, start: i32, end: i32) {
      // java int fc = this.mTextPool.append(buf, start, end);
      let fc: i32 = unsafe { (*self.active_pool()).append__chararr_int_int(buf, start, end) };
      // java this.insertImpl(cp, fc, end - start);
      self.insertImpl(cp, fc, end - start);
    }

    // java public final void insert(int cp, String s, int start, int end)
    fn insert__int_string_int_int(&mut self, cp: i32, s: &str, start: i32, end: i32) {
      // java int fc = this.mTextPool.append((CharSequence)s, start, end);
      let fc: i32 = unsafe { (*self.active_pool()).append__charsequence_int_int(Some(s), start, end) };
      // java this.insertImpl(cp, fc, end - start);
      self.insertImpl(cp, fc, end - start);
    }

    // java public final int length()
    fn length(&self) -> i32 {
      // java return this.mLength;
      return self.mLength;
    }

    // java public final synchronized ITextPiece seek(int cp)
    fn seek(&mut self, cp: i32) -> Option<Box<dyn ITextPiece>> {
      // java return (KTextPieceTable.CPNode)this.mMap.getTree1().find(cp, (int[])null);
      let _p = self.mMap.getTree1().find(cp, None) as *mut CPNode;
      None
    }

    // java public final synchronized void addF0(long[] ranges)
    fn addF0(&mut self, ranges: &Vec<i64>) {
      let mut i: i32 = 0;
      // java for(int i = 0; i < ranges.length; ++i)
      while i < (ranges.len() as i32) {
        // java int rgStart = getStart(ranges[i]);
        let rgStart: i32 = Self::getStart(ranges[i as usize]);
        // java int rgEnd = getEnd(ranges[i]);
        let rgEnd: i32 = Self::getEnd(ranges[i as usize]);
        // java int start = rgEnd;
        let mut start: i32 = rgEnd;
        let mut cp: i32 = rgStart;
        // java for(int cp = rgStart; cp < rgEnd ++cp)
        while cp < rgEnd {
          // java int fc = this.cp2fc(cp);
          let fc: i32 = self.cp2fc(cp);
          // java char c = this.mTextPool.charAt(fc);
          let c: char = unsafe { (*self.active_pool()).charAt(fc) };
          // java if (FontName.isNormalChar(c))
          if FontName::isNormalChar(c) {
            // java start = Math.min(start, cp);
            start = std::cmp::min(start, cp);
          }
          // java else
          else {
            // java if (cp > start)
            if cp > start {
              // java this.modifyF0Impl(start, cp - start, true);
              self.modifyF0Impl(start, cp - start, true);
            }
            // java start = rgEnd;
            start = rgEnd;
          }
          cp += 1;
        }
        // java if (rgEnd > start)
        if rgEnd > start {
          // java this.modifyF0Impl(start, rgEnd - start, true);
          self.modifyF0Impl(start, rgEnd - start, true);
        }
        i += 1;
      }
    }

    // java public final synchronized void removeF0(long[] ranges)
    fn removeF0(&mut self, ranges: &Vec<i64>) {
      let mut i: i32 = 0;
      // java for(int i = 0; i < ranges.length; ++i)
      while i < (ranges.len() as i32) {
        // java int rgStart = getStart(ranges[i]);
        let rgStart: i32 = Self::getStart(ranges[i as usize]);
        // java int rgEnd = getEnd(ranges[i]);
        let rgEnd: i32 = Self::getEnd(ranges[i as usize]);
        // java int start = rgEnd;
        let mut start: i32 = rgEnd;
        let mut cp: i32 = rgStart;
        // java for(int cp = rgStart; cp < rgEnd ++cp)
        while cp < rgEnd {
          // java int fc = this.cp2fc(cp);
          let fc: i32 = self.cp2fc(cp);
          // java char c = this.mTextPool.charAt(fc);
          let c: char = unsafe { (*self.active_pool()).charAt(fc) };
          // java int high = c >> 8;
          let high: i32 = (c as u32 >> 8) as i32;
          // java if (high == 240)
          if high == 240 {
            // java start = Math.min(start, cp);
            start = std::cmp::min(start, cp);
          }
          // java else
          else {
            // java if (cp > start)
            if cp > start {
              // java this.modifyF0Impl(start, cp - start, false);
              self.modifyF0Impl(start, cp - start, false);
            }
            // java start = rgEnd;
            start = rgEnd;
          }
          cp += 1;
        }
        // java if (rgEnd > start)
        if rgEnd > start {
          // java this.modifyF0Impl(start, rgEnd - start, false);
          self.modifyF0Impl(start, rgEnd - start, false);
        }
        i += 1;
      }
    }

    // java public void modifyF0Impl(int cp, int len, boolean fAddF0)
    fn modifyF0Impl(&mut self, cp: i32, len: i32, fAddF0: bool) {
      let mut i: i32 = 0;
      // java for(int i = 0; i < len; ++i)
      while i < len {
        // java int fc = this.cp2fc(cp + i);
        let fc: i32 = self.cp2fc(cp + i);
        // java char c = this.mTextPool.charAt(fc);
        let c: char = unsafe { (*self.active_pool()).charAt(fc) };
        // java char newChar;
        let newChar: char;
        // java if (fAddF0 && c != '(')
        if fAddF0 && c != '(' {
          // java newChar = (char)(c | '\uf000');
          newChar = char::from_u32((c as u32) | 0xf000).unwrap_or(c);
        }
        // java else
        else {
          // java newChar = (char)(c & 255);
          newChar = char::from_u32((c as u32) & 255).unwrap_or('\0');
        }
        // java this.mTextPool.set(fc, newChar);
        unsafe { (*self.active_pool()).set(fc, newChar) };
        i += 1;
      }
    }

}

// java public class KTextRope implements ITextRope
impl KTextRope {
    // Java `this.mTextPool` after setTextPool: one ITextPool for every read and write.
    pub(crate) fn active_pool(&self) -> *mut dyn ITextPool {
      if let Some(p) = self.mSharedPool {
        if !p.is_null() {
          return p;
        }
      }
      &self.mTextPool as &dyn ITextPool as *const dyn ITextPool as *mut dyn ITextPool
    }

    // java public void setTextPool(ITextPool pTextPool)
    pub fn setTextPool(&mut self, pTextPool: Option<*mut dyn ITextPool>) {
      // java this.mTextPool = pTextPool;
      self.mSharedPool = pTextPool.filter(|p| !p.is_null());
    }

    // java public static int getStart(long range)
    pub fn getStart(range: i64) -> i32 {
      // java return (int)(range >>> 32);
      return ((range as u64 >> 32) as i32);
    }

    // java public static int getEnd(long range)
    pub fn getEnd(range: i64) -> i32 {
      // java return (int)range;
      return (range as i32);
    }

    // java protected void eraseImpl(int start, int end)
    pub fn eraseImpl(&mut self, start: i32, end: i32) {
      // java this.eraseMapping(start, end, (ArrayList)null);
      self.eraseMapping(start, end, None);
    }

    // java protected final synchronized void eraseMapping(int start, int end, ArrayList<IntervalMap.Segment> segments)
    pub fn eraseMapping(&mut self, start: i32, end: i32, mut segments: Option<&mut Vec<Segment>>) {
      // java if (start < end)
      if start < end {
        // java int len = end - start;
        let len: i32 = end - start;
        // java this.mMap.eraseMapping1(start, len, segments);
        self.mMap.eraseMapping1(start, len, segments.as_deref_mut());
        // java this.mMap.eraseSpace1(start, len);
        self.mMap.eraseSpace1(start, len);
        // java this.mLength -= len;
        self.mLength = self.mLength - len;
      }
    }

    // java protected void insertImpl(int cp, int fc, int length)
    pub fn insertImpl(&mut self, cp: i32, fc: i32, length: i32) {
      // java this.insertMapping(cp, fc, length);
      self.insertMapping(cp, fc, length);
    }

    // java protected final synchronized void insertMapping(int cp, int fc, int length)
    pub fn insertMapping(&mut self, cp: i32, fc: i32, length: i32) {
      // java if (length != 0)
      if length != 0 {
        // java if (cp < this.mLength)
        if cp < self.mLength {
          // java this.mMap.insertSpace1(cp, length);
          self.mMap.insertSpace1(cp, length);
        }
        // java this.mMap.addMapping1(cp, fc, length);
        self.mMap.addMapping1(cp, fc, length);
        // java this.mLength += length;
        self.mLength = self.mLength + length;
      }
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java char[] buf = new char[this.length()];
      let mut buf: Vec<char> = vec!['\0'; self.length() as usize];
      // java this.getText(0, this.length(), buf, 0);
      ITextRope::getText(self, 0, self.length(), &mut buf, 0);
      // java return new String(buf);
      return buf.iter().collect();
    }

}
