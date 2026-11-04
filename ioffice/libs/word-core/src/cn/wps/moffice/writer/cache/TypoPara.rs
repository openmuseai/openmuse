#![allow(dead_code, non_snake_case)]

/// Compile-surface for TypoTextLine deferred signatures (wr-01 02).
/// getChar/getWidth/getWidths match TypoPara.java; backing Vecs until snapshot is wired.
#[derive(Clone)]
pub struct TypoPara {
    pub mGraphSize: i32,
    pub chars: Vec<char>,
    pub widths: Vec<i32>,
    // Java: snapshot pointer+4 owns an ordered TypoList of drawing handles.
    // Rust's compiled TypoPara mirror keeps the same order beside chars/widths.
    pub embeds: Vec<i32>,
    // java protected int mPointer;
    mPointer: i32,
    // Java TypoPara.mSnapshot. Null until alloc(snapshot).
    pub snapshot: *mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot,
    // rust: ★wr-29 05 —— CP→graph 映射 mirror（Java 存 para snapshot；Rust mirror 置此，
    //       GraphReader.commitReadText 经 addMap 同步维护）↑(cpStart,cpEnd,graphIndex,graphEnd)
    pub cp_map: Vec<(i32, i32, i32, i32)>,
}

impl Default for TypoPara {
    fn default() -> Self {
        TypoPara { mGraphSize: 0, chars: Vec::new(), widths: Vec::new(), embeds: Vec::new(), mPointer: 0, snapshot: std::ptr::null_mut(), cp_map: Vec::new() }
    }
}

impl TypoPara {
    // java public int getEmbed(int index)
    pub fn getEmbed(&self, index: i32) -> i32 {
      // Java: return 0 != pEmbeds ? TypoList.get(index, pEmbeds, snapshot) : 0;
      if !self.snapshot.is_null() && self.mPointer != 0 {
        let snap = unsafe { &mut *self.snapshot };
        let p_embeds = snap.get__int(self.mPointer + 4);
        if p_embeds != 0 {
          return crate::cn::wps::moffice::writer::cache::TypoList::TypoList::get__int_int_typosnapshot(index, p_embeds, snap);
        }
      }
      if index < 0 || (index as usize) >= self.embeds.len() { 0 } else { self.embeds[index as usize] }
    }

    // java public static read of snapshot slot pointer+4
    pub fn getEmbed__int_int_typosnapshot(index: i32, p_para: i32, snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
      if p_para == 0 { return 0; }
      let p_embeds = snapshot.get__int(p_para + 4);
      if p_embeds == 0 { return 0; }
      crate::cn::wps::moffice::writer::cache::TypoList::TypoList::get__int_int_typosnapshot(index, p_embeds, snapshot)
    }

    // java public void clearRedundantEmbeds(int limitCp)
    pub fn clearRedundantEmbeds(&mut self, limit_cp: i32) {
      if self.snapshot.is_null() || self.mPointer == 0 {
        return;
      }
      let snap = unsafe { &mut *self.snapshot };
      let p_embeds = snap.get__int(self.mPointer + 4);
      if p_embeds == 0 { return; }
      let mut index = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(p_embeds, snap) - 1;
      while index >= 0 {
        let embed = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::get__int_int_typosnapshot(index, p_embeds, snap);
        if crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::getCP__int_typosnapshot(embed, snap) < limit_cp {
          break;
        }
        crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing::removeShape__int_typosnapshot(embed, snap);
        crate::cn::wps::moffice::writer::cache::TypoList::TypoList::remove__int_int_typosnapshot(index, p_embeds, snap);
        index -= 1;
      }
    }

    // java public int getEmbedSize()
    pub fn getEmbedSize(&self) -> i32 {
      // Java: invalid para/absent pEmbeds gives 0; otherwise TypoList.size.
      if !self.snapshot.is_null() && self.mPointer != 0 {
        let snap = unsafe { &*self.snapshot };
        let p_embeds = snap.get__int(self.mPointer + 4);
        if p_embeds != 0 {
          return crate::cn::wps::moffice::writer::cache::TypoList::TypoList::size__int_typosnapshot(p_embeds, snap);
        }
      }
      self.embeds.len() as i32
    }

    // java public void addEmbed(int pDrawing)
    pub fn addEmbed(&mut self, pDrawing: i32) {
      // Java: TypoList.add appends pDrawing and stores the resulting list pointer.
      self.embeds.push(pDrawing);
      if self.snapshot.is_null() || self.mPointer == 0 {
        return;
      }
      let snap = unsafe { &mut *self.snapshot };
      let old = snap.get__int(self.mPointer + 4);
      let p_embeds = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::add__int_int_typosnapshot(pDrawing, old, snap);
      if p_embeds != old {
        snap.set__int_int(self.mPointer + 4, p_embeds);
      }
    }

    // rust: ★wr-29 05 —— CP 映射写入（Java GraphBuff.addMap 语义：连续 1:1 段合并）
    pub fn putMap(&mut self, cpStart: i32, cpLen: i32, graphIndex: i32, graphLen: i32) {
      if self.cp_map.is_empty() {
        // java mapData[0..3] = cpStart, cpStart+cpLen, graphIndex, graphIndex+graphLen; ++mMapSize;
        self.cp_map.push((cpStart, cpStart + cpLen, graphIndex, graphIndex + graphLen));
      } else {
        let last = *self.cp_map.last().unwrap();
        // java else if (cpLen==graphLen && 连续 && cpStart==lastEnd && graphIndex==lastGIEnd) { 合并 }
        if cpLen == graphLen
          && last.1 - last.0 == last.3 - last.2
          && cpStart == last.1
          && graphIndex == last.3 {
          let n = self.cp_map.len() - 1;
          self.cp_map[n] = (last.0, cpStart + cpLen, last.2, graphIndex + graphLen);
        } else {
          // java else { 新条目（扩容用 push） }
          self.cp_map.push((cpStart, cpStart + cpLen, graphIndex, graphIndex + graphLen));
        }
      }
    }

    // java public int getCP(int graphIndex)
    pub fn getCP(&self, graphIndex: i32) -> i32 {
      // java return getStartCP(graphIndex, this.mPointer, this.mSnapshot);
      for &(startCP, endCP, startGraphIndex, endGraphIndex) in &self.cp_map {
        if graphIndex >= startGraphIndex && graphIndex < endGraphIndex {
          if endCP - startCP == endGraphIndex - startGraphIndex {
            return startCP + graphIndex - startGraphIndex;
          }
          return startCP;
        }
      }
      -1
    }

    // java public int getNextCP(int graphIndex)
    // rust: ★wr-29 05 —— mirror 实现（Java TypoPara.java:585-611 语义；cpRef=0）
    pub fn getNextCP(&self, graphIndex: i32) -> i32 {
      // java for(...) { startGraphIndex = snapshot.get(pointerOfItem + 2); endGraphIndex; ... }
      for &(startCP, endCP, startGraphIndex, endGraphIndex) in &self.cp_map {
        if graphIndex >= startGraphIndex && graphIndex < endGraphIndex {
          // java if (endCP - startCP == endGraphIndex - startGraphIndex) { return startCP + graphIndex - startGraphIndex + cpRef + 1; }
          if endCP - startCP == endGraphIndex - startGraphIndex {
            return startCP + graphIndex - startGraphIndex + 1;
          }
          // java return endCP + cpRef;
          return endCP;
        }
      }
      // java return -1;
      -1
    }

    // java public int getPointer()
    pub fn getPointer(&self) -> i32 {
        // java return this.mPointer;
        self.mPointer
    }

    // java public int getWidth(int graphIndex)
    pub fn getWidth(&self, graphIndex: i32) -> i32 {
        if graphIndex >= self.mGraphSize {
            0
        } else {
            self.widths.get(graphIndex as usize).copied().unwrap_or(0)
        }
    }

    pub fn getWidth__int(&self, graphIndex: i32) -> i32 {
        self.getWidth(graphIndex)
    }

    // java public int hasAlloc()
    pub fn hasAlloc(&self) -> i32 {
        // java return this.mGraphSize;
        self.mGraphSize
    }

    // java public int allocGraphs(int len)
    pub fn allocGraphs(&mut self, len: i32) -> i32 {
        // java TypoSnapshot snapshot = this.mSnapshot;
        // java int oldGraphSize = this.mGraphSize;
        let oldGraphSize = self.mGraphSize;
        // java this.mGraphSize += len;
        self.mGraphSize += len;
        // java int extendMemSize = len * 2;
        let extend_mem_size = len.wrapping_mul(2);
        if let Some(snapshot) = unsafe { self.snapshot.as_mut() } {
            // java this.mPointer = snapshot.extendAlloc(this.mPointer, TypoMemoryObj.getMemSize(...), extendMemSize);
            let old_mem_size = crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj::getMemSize__int_typosnapshot(self.mPointer, snapshot);
            self.mPointer = snapshot.extendAlloc(self.mPointer, old_mem_size, extend_mem_size);
            // java snapshot.extend(mPointer + 1, extendMemSize);
            snapshot.extend(self.mPointer + 1, extend_mem_size);
            // java if (oldGraphSize != 0) snapshot.copy(old width plane, new width plane, oldGraphSize);
            if oldGraphSize != 0 {
                snapshot.copy__int_int_int(
                    self.mPointer + 5 + oldGraphSize,
                    self.mPointer + 5 + self.mGraphSize,
                    oldGraphSize,
                );
            }
        }
        self.chars.resize(self.mGraphSize as usize, '\0');
        self.widths.resize(self.mGraphSize as usize, 0);
        // java return oldGraphSize;
        oldGraphSize
    }

    // java public void putChars(char[] chars, int index, int length, int graphIndex)
    pub fn putChars__chararr_int_int_int(&mut self, chars: &[char], index: i32, length: i32, graphIndex: i32) {
        // java int maxGraphIndex = graphIndex + Math.min(length, chars.length - index);
        let maxGraphIndex = graphIndex + length.min(chars.len() as i32 - index);
        // java int curGraphIndex = graphIndex;
        let mut curGraphIndex = graphIndex;
        let mut i = index;
        // rust: compile-surface 连续块 —— 逐字符 copy
        while curGraphIndex < maxGraphIndex {
            // java data[i] = chars[index++];
            self.chars[curGraphIndex as usize] = chars[i as usize];
            if let Some(snapshot) = unsafe { self.snapshot.as_mut() } {
                snapshot.set__int_int(self.mPointer + 5 + curGraphIndex, chars[i as usize] as i32);
            }
            curGraphIndex += 1;
            i += 1;
        }
    }

    // java public void putChar(int graphIndex, char c)
    pub fn putChar(&mut self, graphIndex: i32, c: char) {
        // java snapshot.set(this.mPointer + 5 + graphIndex, c);
        self.chars[graphIndex as usize] = c;
        if let Some(snapshot) = unsafe { self.snapshot.as_mut() } {
            snapshot.set__int_int(self.mPointer + 5 + graphIndex, c as i32);
        }
    }

    // java public void putChars(ITextRope textRope, int start, int end, int graphIndex)
    pub fn putChars__textrope_int_int_int(&mut self, textRope: &dyn crate::cn::wps::collections::textrope::ITextRope::ITextRope, start: i32, end: i32, graphIndex: i32) {
        // java int maxEnd = Math.min(textRope.length(), end);
        let maxEnd = end.min(textRope.length());
        // java int curGraphIndex = graphIndex; int curStart = start;
        let mut curGraphIndex = graphIndex;
        let mut curStart = start;
        // java char[] buff = this.mCharCache;
        // rust: compile-surface —— 直接分块 getText 到 chars
        // java for(char[] buff = this.mCharCache; curStart < maxEnd; curGraphIndex += readThisTime)
        while curStart < maxEnd {
            // java readThisTime = Math.min(maxEnd - curStart, 256);
            let readThisTime = (maxEnd - curStart).min(256);
            // java textRope.getText(curStart, curStart + readThisTime, buff, 0);
            let mut buff = vec!['\0'; readThisTime as usize];
            textRope.getText(curStart, curStart + readThisTime, &mut buff, 0);
            // java this.putChars(buff, 0, readThisTime, curGraphIndex);
            self.putChars__chararr_int_int_int(&buff, 0, readThisTime, curGraphIndex);
            curStart += readThisTime;
            curGraphIndex += readThisTime;
        }
    }

    // java public void putWidth(int graphIndex, int width)
    pub fn putWidth(&mut self, graphIndex: i32, width: i32) {
      // java this.widths[graphIndex] = width;
      if graphIndex >= 0 && (graphIndex as usize) < self.mGraphSize as usize {
        self.widths[graphIndex as usize] = width;
        if let Some(snapshot) = unsafe { self.snapshot.as_mut() } {
          snapshot.set__int_int(self.mPointer + 5 + self.mGraphSize + graphIndex, width);
        }
      }
    }

    // java public void putWidths(int graphIndex, int graphLen, int[] widths)
    // rust: ★wr-29 04 —— 回写宽度平面（Java 直接写 snapshot 数组；compile-surface 以平面回写）
    pub fn putWidths(&mut self, graphIndex: i32, graphLen: i32, widths: &[i32]) {
        let start = graphIndex as usize;
        let end = (graphIndex + graphLen) as usize;
        if self.widths.len() < end {
            self.widths.resize(end, 0);
        }
        for i in 0..graphLen {
            self.widths[start + i as usize] = widths[i as usize];
            if let Some(snapshot) = unsafe { self.snapshot.as_mut() } {
                snapshot.set__int_int(self.mPointer + 5 + self.mGraphSize + graphIndex + i, widths[i as usize]);
            }
        }
    }

    // java public void getWidths(int index, int length, int[] widths, int bufIndex)
    pub fn getWidths(&self, index: i32, length: i32, widths: &mut [i32], bufIndex: i32) {
        let mut i = 0i32;
        while i < length {
            let dest = (bufIndex + i) as usize;
            if dest < widths.len() {
                widths[dest] = self.getWidth(index + i);
            }
            i += 1;
        }
    }

    // java public char getChar(int graphIndex)
    pub fn getChar(&self, graphIndex: i32) -> char {
        if graphIndex >= 0 && graphIndex < self.mGraphSize {
            self.chars.get(graphIndex as usize).copied().unwrap_or('\0')
        } else {
            panic!("TypoPara error graphIndex={} graphSize={}", graphIndex, self.mGraphSize);
        }
    }

    // java public int[] getWidthBlock(int graphIndex, int[] result)
    // rust: ★wr-29 04 —— compile-surface 连续块（宽度同 chars 平面）
    pub fn getWidthBlock(&self, graphIndex: i32, result: &mut [i32; 2]) -> &[i32] {
        // java int[] data = snapshot.get(blockPointer + curGraphIndex, this.mOffsets);
        // rust: result[0]=块内 offset(=graphIndex), result[1]=limit(=mGraphSize)
        result[0] = graphIndex;
        result[1] = self.mGraphSize;
        &self.widths
    }

    // java public void getChars(int graphIndex, int graphLength, char[] buf, int bufIndex)
    // rust: ★wr-29 04 —— 拷贝 chars 到外部 buf（calcHeight 用）
    // java public void processRTLGraphWidth(int graphIndex, int graphLength)
    pub fn processRTLGraphWidth(&mut self, graphIndex: i32, graphLength: i32) {
        // java snapshot.processRTLWidths: each width becomes -abs(width)
        let start = graphIndex.max(0) as usize;
        let end = (graphIndex.saturating_add(graphLength).max(0) as usize).min(self.widths.len());
        if start < end {
            for w in &mut self.widths[start..end] {
                *w = -w.abs();
            }
        }
    }

    pub fn getChars(&self, graphIndex: i32, graphLength: i32, buf: &mut [char], bufIndex: i32) {
        // java System.arraycopy(this.chars, graphIndex, buf, bufIndex, graphLength);
        let src = self.chars.get(graphIndex as usize..(graphIndex + graphLength) as usize);
        if let Some(src) = src {
            let dst = buf.get_mut(bufIndex as usize..(bufIndex + graphLength) as usize);
            if let Some(dst) = dst {
                dst.copy_from_slice(src);
            }
        }
    }

    // java public void fillWidths(int graphIndex, int graphLen, int width)
    // rust: ★wr-29 04 —— 常量宽填充宽度平面（addSpaceGraph/addZeroWidthSpaceGraph 用）
    pub fn fillWidths(&mut self, graphIndex: i32, graphLen: i32, width: i32) {
        // java Arrays.fill(this.widths, graphIndex, graphIndex + graphLen, width);
        let start = graphIndex as usize;
        let end = (graphIndex + graphLen) as usize;
        if self.widths.len() < end {
            self.widths.resize(end, 0);
        }
        for w in self.widths.iter_mut().take(end).skip(start) {
            *w = width;
        }
    }

    // java public int getSum(int graphIndex, int graphLength)
    pub fn getSum(&self, graphIndex: i32, graphLength: i32) -> i32 {
        // java return this.mSnapshot.getSum(this.mPointer + 5 + this.mGraphSize + graphIndex, graphLength);
        // rust: ★wr-29 08 —— 宽度平面求和（Java snapshot 块；compile-surface 同 putWidths 平面）
        let mut sum: i32 = 0;
        let mut i: i32 = 0;
        while i < graphLength {
            sum = sum.saturating_add(self.getWidth(graphIndex + i));
            i += 1;
        }
        sum
    }

    // java public int getAbsSum(int graphIndex, int graphLength)
    pub fn getAbsSum(&self, graphIndex: i32, graphLength: i32) -> i32 {
        // java return this.mSnapshot.getAbsSum(this.mPointer + 5 + this.mGraphSize + graphIndex, graphLength);
        self.getSum(graphIndex, graphLength).abs()
    }

    // java public int[] getCharBlock(int graphIndex, int[] result)
    pub fn getCharBlock(&self, graphIndex: i32, result: &mut [i32; 2]) -> &[char] {
        // java int[] data = snapshot.get(blockPointer + curGraphIndex, this.mOffsets);
        // rust: compile-surface 连续块 —— result[0]=块内 offset(=graphIndex), result[1]=limit(=mGraphSize)
        result[0] = graphIndex;
        result[1] = self.mGraphSize;
        &self.chars
    }

    // java public void clear()
    // java public void extendWidths(int graphIndex, int extendLen, int width)
    pub fn extendWidths(&mut self, graphIndex: i32, extendLen: i32, width: i32) {
      // Java: this.mSnapshot.extend(this.mPointer + 5 + this.mGraphSize + graphIndex, extendLen, spacePerChar);
      // Java TypoSnapshot.extend adds to each stored width; it does not replace it.
      if graphIndex >= 0 {
        for i in 0..extendLen {
          let gi = graphIndex + i;
          if gi >= 0 && (gi as usize) < self.mGraphSize as usize {
            self.widths[gi as usize] = self.widths[gi as usize].wrapping_add(width);
            if let Some(snapshot) = unsafe { self.snapshot.as_mut() } {
              snapshot.extend(self.mPointer + 5 + self.mGraphSize + gi, width);
            }
          }
        }
      }
    }

    pub fn clear(&mut self) {
        // java this.mSnapshot = null;
        self.snapshot = std::ptr::null_mut();
        // java this.mPointer = 0;
        self.mPointer = 0;
        self.mGraphSize = 0;
        self.chars.clear();
        self.widths.clear();
        self.embeds.clear();
        // Java drops the paragraph snapshot and its CP map together.
        self.cp_map.clear();
    }

    // java public int alloc(TypoSnapshot snapshot)
    pub fn alloc(&mut self, snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
        self.snapshot = snapshot as *mut _;
        // java this.mPointer = snapshot.alloc(5);
        self.mPointer = snapshot.alloc(5);
        // java snapshot.set(this.mPointer + 0, 17);
        snapshot.set__int_int(self.mPointer, 17);
        // java snapshot.set(this.mPointer + 1, 5);
        snapshot.set__int_int(self.mPointer + 1, 5);
        // java this.mSnapshot = snapshot;
        // java if (null == this.mCharCache)
        if self.chars.len() < 256 {
            // java this.mCharCache = new char[256];
            self.chars.resize(256, '\0');
        }
        // java this.mGraphSize = 0;
        self.mGraphSize = 0;
        self.embeds.clear();
        // Each Java TypoPara allocation starts a new CP→graph index space.
        self.cp_map.clear();
        // java return this.mPointer;
        self.mPointer
    }

    // java public void commit(int[] mapData, int mapSize, Document doc, int recommentRef)
    pub fn commit(
        &mut self,
        mapData: &mut [i32],
        mapSize: i32,
        doc: Option<*mut dyn crate::cn::wps::moffice::writer::core::Document::Document>,
        recommentRef: i32,
    ) {
        // java TypoSnapshot snapshot = this.mSnapshot;
        let snapshot = unsafe { self.snapshot.as_mut().expect("TypoPara.commit requires alloc") };
        // java int startCP = mapData[0];
        assert!(mapSize >= 0 && !mapData.is_empty() && mapData.len() >= mapSize as usize * 4);
        let startCP = mapData[0];
        // java int ref = recommentRef != 0 && snapshot.getCPRef(recommentRef) == startCP ? recommentRef : snapshot.addCPRef(doc, startCP);
        let cp_ref = if recommentRef != 0 && snapshot.getCPRef(recommentRef) == startCP {
            recommentRef
        } else {
            snapshot.addCPRef(doc.map(|ptr| unsafe { &*ptr as &dyn crate::cn::wps::moffice::writer::core::Document::Document }), startCP)
        };
        // java snapshot.set(this.mPointer + 2, ref);
        snapshot.set__int_int(self.mPointer + 2, cp_ref);
        // java for(int i = 0; i < mapSize; ++i) { mapData[i * 4 + 0] -= startCP; mapData[i * 4 + 1] -= startCP; }
        for i in 0..mapSize as usize {
            mapData[i * 4] = mapData[i * 4].wrapping_sub(startCP);
            mapData[i * 4 + 1] = mapData[i * 4 + 1].wrapping_sub(startCP);
        }
        // java int extendMem = mapSize * 4;
        let extendMem = mapSize.wrapping_mul(4);
        // java this.mPointer = snapshot.extendAlloc(...);
        let old_size = crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj::getMemSize__int_typosnapshot(self.mPointer, snapshot);
        self.mPointer = snapshot.extendAlloc(self.mPointer, old_size, extendMem);
        // java snapshot.set(this.mPointer + 1, 5 + this.mGraphSize * 2 + extendMem);
        snapshot.set__int_int(self.mPointer + 1, 5i32.wrapping_add(self.mGraphSize.wrapping_mul(2)).wrapping_add(extendMem));
        // java snapshot.set(this.mPointer + 3, this.mGraphSize);
        snapshot.set__int_int(self.mPointer + 3, self.mGraphSize);
        // java snapshot.set(this.mPointer + 5 + this.mGraphSize * 2, (int[])mapData, 0, extendMem);
        let map_start = self.mPointer + 5 + self.mGraphSize * 2;
        for (i, value) in mapData[..extendMem as usize].iter().enumerate() {
            snapshot.set__int_int(map_start + i as i32, *value);
        }
    }

    // java public void removeGraphs(int graphIndex, int graphLen)
    pub fn removeGraphs(&mut self, graphIndex: i32, graphLen: i32) {
        // java if (graphIndex < this.mGraphSize)
        if graphIndex < self.mGraphSize {
            // java this.mGraphSize = graphIndex; (truncating arm when graphIndex+graphLen >= mGraphSize)
            if graphIndex + graphLen >= self.mGraphSize {
                self.mGraphSize = graphIndex;
            } else {
                self.mGraphSize -= graphLen;
            }
        }
    }

    pub fn getCPRef__int_typosnapshot(pointer: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
        snapshot.get__int(pointer + 2)
    }

    pub fn getChar__int_int_typosnapshot(graph_index: i32, para_pointer: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> char {
        let graph_size = snapshot.get__int(para_pointer + 3);
        if graph_index >= 0 && graph_index < graph_size {
            char::from_u32(snapshot.get__int(para_pointer + 5 + graph_index) as u32).unwrap_or('\0')
        } else {
            panic!("TypoPara error graphIndex");
        }
    }

    pub fn getGraphIndexFromCP__int_int_typosnapshot(cp: i32, p_para: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
        Self::getGraphIndexFromCP__int_boolean_boolean_int_typosnapshot(cp, true, true, p_para, snapshot)
    }

    pub fn getGraphIndexFromCP__int_boolean_boolean_int_typosnapshot(cp: i32, candidate_after: bool, seek_item_end: bool, p_para: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
        let pointer_of_map = p_para + 5 + snapshot.get__int(p_para + 3) * 2;
        let max_pointer = p_para + snapshot.get__int(p_para + 1);
        let mut pointer_of_item = pointer_of_map;
        let mut after_index = i32::MAX;
        let mut before_index = i32::MIN;
        let ref_cp = snapshot.getCPRef(Self::getCPRef__int_typosnapshot(p_para, snapshot));
        while pointer_of_item < max_pointer {
            let start_cp = ref_cp + snapshot.get__int(pointer_of_item);
            let end_cp = ref_cp + snapshot.get__int(pointer_of_item + 1);
            let start_graph = snapshot.get__int(pointer_of_item + 2);
            let end_graph = snapshot.get__int(pointer_of_item + 3);
            if cp >= start_cp && cp < end_cp {
                if end_cp - start_cp == end_graph - start_graph {
                    return start_graph + cp - start_cp;
                }
                return if seek_item_end { end_graph - 1 } else { start_graph };
            }
            if cp < start_cp {
                after_index = start_graph;
                break;
            }
            if cp >= end_cp {
                before_index = end_graph - 1;
            }
            pointer_of_item += 4;
        }
        let index = if candidate_after { after_index } else { before_index };
        if index != i32::MAX && index != i32::MIN { index } else { -1 }
    }

    pub fn getStartCP__int_int_typosnapshot(index: i32, p_para: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
        let cp_ref = snapshot.getCPRef(Self::getCPRef__int_typosnapshot(p_para, snapshot));
        if cp_ref < 0 {
            return -1;
        }
        let pointer_of_map = p_para + 5 + snapshot.get__int(p_para + 3) * 2;
        let max_pointer = p_para + snapshot.get__int(p_para + 1);
        let mut pointer_of_item = pointer_of_map;
        while pointer_of_item < max_pointer {
            let start_graph = snapshot.get__int(pointer_of_item + 2);
            let end_graph = snapshot.get__int(pointer_of_item + 3);
            if index >= start_graph && index < end_graph {
                let start_cp = snapshot.get__int(pointer_of_item);
                let end_cp = snapshot.get__int(pointer_of_item + 1);
                if end_cp - start_cp == end_graph - start_graph {
                    return start_cp + index - start_graph + cp_ref;
                }
                return start_cp + cp_ref;
            }
            pointer_of_item += 4;
        }
        -1
    }

    pub fn getEndCP__int_int_typosnapshot(index: i32, p_para: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
        let cp_ref = snapshot.getCPRef(Self::getCPRef__int_typosnapshot(p_para, snapshot));
        if cp_ref < 0 {
            return -1;
        }
        let pointer_of_map = p_para + 5 + snapshot.get__int(p_para + 3) * 2;
        let max_pointer = p_para + snapshot.get__int(p_para + 1);
        let mut pointer_of_item = pointer_of_map;
        while pointer_of_item < max_pointer {
            let start_graph = snapshot.get__int(pointer_of_item + 2);
            let end_graph = snapshot.get__int(pointer_of_item + 3);
            if index >= start_graph && index < end_graph {
                let start_cp = snapshot.get__int(pointer_of_item);
                let end_cp = snapshot.get__int(pointer_of_item + 1);
                if end_cp - start_cp == end_graph - start_graph {
                    return start_cp + index - start_graph + cp_ref + 1;
                }
                return end_cp + cp_ref;
            }
            pointer_of_item += 4;
        }
        -1
    }

    pub fn getWidth__int_int_typosnapshot(graph_index: i32, pointer: i32, snapshot: &crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) -> i32 {
        if pointer != 0 && graph_index >= 0 {
            let graph_size = snapshot.get__int(pointer + 3);
            if graph_index >= graph_size { 0 } else { snapshot.get__int(pointer + 5 + graph_size + graph_index) }
        } else {
            0
        }
    }
}
