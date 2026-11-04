// Java: import cn.wps.kfc.impl.arrayList.WIntArrayList
use util::kfc::r#impl::arrayList::WIntArrayList::WIntArrayList;

// java private static final int COMPRESSABLE_SPACE_TYPE_COUNT = 3;
const COMPRESSABLE_SPACE_TYPE_COUNT: i32 = 3;

// java public final class LineCompressEnv
pub struct LineCompressEnv {
    // java public int firstExcessSpaceGraphIndex;
    pub firstExcessSpaceGraphIndex: i32,
    // java public int firstExcessPunctGraphIndex;
    pub firstExcessPunctGraphIndex: i32,
    // java private int mMaxSpaceCompressableSpaceSum;
    mMaxSpaceCompressableSpaceSum: i32,
    // java private int mMaxPunctCompressableSpaceSum;
    mMaxPunctCompressableSpaceSum: i32,
    // java private int mMaxOtherCompressableSpaceSum;
    mMaxOtherCompressableSpaceSum: i32,
    // java private int mUsedSpaceCompressableSpaceSum;
    mUsedSpaceCompressableSpaceSum: i32,
    // java private int mUsedPunctCompressableSpaceSum;
    mUsedPunctCompressableSpaceSum: i32,
    // java private int mUsedOtherCompressableSpaceSum;
    mUsedOtherCompressableSpaceSum: i32,
    // java private WIntArrayList mCompressableGraphIndexes = new WIntArrayList();
    mCompressableGraphIndexes: Option<WIntArrayList>,
    // java private WIntArrayList mMaxCompressableSpaces = new WIntArrayList();
    mMaxCompressableSpaces: Option<WIntArrayList>,
    // java private WIntArrayList mCompressedGraphIndexes = new WIntArrayList();
    mCompressedGraphIndexes: Option<WIntArrayList>,
    // java private WIntArrayList mUsedCompressableSpaces = new WIntArrayList();
    mUsedCompressableSpaces: Option<WIntArrayList>,
    // java private WIntArrayList mCompressBeforeIndexes = new WIntArrayList();
    mCompressBeforeIndexes: Option<WIntArrayList>,
    // java private WIntArrayList mCompressBeforeValues = new WIntArrayList();
    mCompressBeforeValues: Option<WIntArrayList>,
    // java private WIntArrayList mCompressAfterIndexes = new WIntArrayList();
    mCompressAfterIndexes: Option<WIntArrayList>,
    // java private WIntArrayList mCompressAfterValues = new WIntArrayList();
    mCompressAfterValues: Option<WIntArrayList>,
    // java private int mFinalCompress;
    mFinalCompress: i32,
}

// java public final class LineCompressEnv
impl LineCompressEnv {
    // java field inits: new WIntArrayList() on eight lists (implicit ctor)
    pub fn LineCompressEnv() -> Self {
        Self {
            firstExcessSpaceGraphIndex: 0,
            firstExcessPunctGraphIndex: 0,
            mMaxSpaceCompressableSpaceSum: 0,
            mMaxPunctCompressableSpaceSum: 0,
            mMaxOtherCompressableSpaceSum: 0,
            mUsedSpaceCompressableSpaceSum: 0,
            mUsedPunctCompressableSpaceSum: 0,
            mUsedOtherCompressableSpaceSum: 0,
            mCompressableGraphIndexes: Some(WIntArrayList::new()),
            mMaxCompressableSpaces: Some(WIntArrayList::new()),
            mCompressedGraphIndexes: Some(WIntArrayList::new()),
            mUsedCompressableSpaces: Some(WIntArrayList::new()),
            mCompressBeforeIndexes: Some(WIntArrayList::new()),
            mCompressBeforeValues: Some(WIntArrayList::new()),
            mCompressAfterIndexes: Some(WIntArrayList::new()),
            mCompressAfterValues: Some(WIntArrayList::new()),
            mFinalCompress: 0,
        }
    }

    // java public void reset()
    pub fn reset(&mut self) {
      // java this.firstExcessPunctGraphIndex = Integer.MIN_VALUE;
      self.firstExcessPunctGraphIndex = i32::MIN;
      // java this.firstExcessSpaceGraphIndex = Integer.MIN_VALUE;
      self.firstExcessSpaceGraphIndex = i32::MIN;
      // java this.mMaxSpaceCompressableSpaceSum = 0;
      self.mMaxSpaceCompressableSpaceSum = 0;
      // java this.mMaxPunctCompressableSpaceSum = 0;
      self.mMaxPunctCompressableSpaceSum = 0;
      // java this.mMaxOtherCompressableSpaceSum = 0;
      self.mMaxOtherCompressableSpaceSum = 0;
      // java this.mUsedSpaceCompressableSpaceSum = 0;
      self.mUsedSpaceCompressableSpaceSum = 0;
      // java this.mUsedPunctCompressableSpaceSum = 0;
      self.mUsedPunctCompressableSpaceSum = 0;
      // java this.mUsedOtherCompressableSpaceSum = 0;
      self.mUsedOtherCompressableSpaceSum = 0;
      // java this.mCompressableGraphIndexes.resetQuick();
      self.mCompressableGraphIndexes.as_mut().unwrap().resetQuick();
      // java this.mMaxCompressableSpaces.resetQuick();
      self.mMaxCompressableSpaces.as_mut().unwrap().resetQuick();
      // java this.mCompressedGraphIndexes.resetQuick();
      self.mCompressedGraphIndexes.as_mut().unwrap().resetQuick();
      // java this.mUsedCompressableSpaces.resetQuick();
      self.mUsedCompressableSpaces.as_mut().unwrap().resetQuick();
      // java this.mCompressBeforeIndexes.resetQuick();
      self.mCompressBeforeIndexes.as_mut().unwrap().resetQuick();
      // java this.mCompressBeforeValues.resetQuick();
      self.mCompressBeforeValues.as_mut().unwrap().resetQuick();
      // java this.mCompressAfterIndexes.resetQuick();
      self.mCompressAfterIndexes.as_mut().unwrap().resetQuick();
      // java this.mCompressAfterValues.resetQuick();
      self.mCompressAfterValues.as_mut().unwrap().resetQuick();
      // java this.mFinalCompress = 0;
      self.mFinalCompress = 0;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mCompressableGraphIndexes = null;
      self.mCompressableGraphIndexes = None;
      // java this.mMaxCompressableSpaces = null;
      self.mMaxCompressableSpaces = None;
      // java this.mCompressedGraphIndexes = null;
      self.mCompressedGraphIndexes = None;
      // java this.mCompressBeforeIndexes = null;
      self.mCompressBeforeIndexes = None;
      // java this.mCompressBeforeValues = null;
      self.mCompressBeforeValues = None;
      // java this.mCompressAfterIndexes = null;
      self.mCompressAfterIndexes = None;
      // java this.mCompressAfterValues = null;
      self.mCompressAfterValues = None;
    }

    // java private void doAddCompressableSpace(int graphIdx, int space, int punct, int other)
    pub fn doAddCompressableSpace(&mut self, graphIdx: i32, space: i32, punct: i32, other: i32) {
      // java this.mMaxSpaceCompressableSpaceSum += space;
      self.mMaxSpaceCompressableSpaceSum = self.mMaxSpaceCompressableSpaceSum + space;
      // java this.mMaxPunctCompressableSpaceSum += punct;
      self.mMaxPunctCompressableSpaceSum = self.mMaxPunctCompressableSpaceSum + punct;
      // java this.mMaxOtherCompressableSpaceSum += other;
      self.mMaxOtherCompressableSpaceSum = self.mMaxOtherCompressableSpaceSum + other;
      // java WIntArrayList list = this.mMaxCompressableSpaces;
      let list = self.mMaxCompressableSpaces.as_ref();
      let _ = list;
      // java WIntArrayList graphIndexes = this.mCompressableGraphIndexes;
      let graphIndexes = self.mCompressableGraphIndexes.as_ref();
      // java int idx = graphIndexes.binarySearch(graphIdx);
      let idx: i32 = graphIndexes.unwrap().binarySearch(graphIdx);
      // java if (idx >= 0)
      if idx >= 0 {
        // java int spaceSumIdx = idx * 3;
        let spaceSumIdx: i32 = idx * 3;
        let list = self.mMaxCompressableSpaces.as_mut().unwrap();
        // java list.set(spaceSumIdx, this.mMaxSpaceCompressableSpaceSum);
        list.set(spaceSumIdx, self.mMaxSpaceCompressableSpaceSum);
        // java list.set(spaceSumIdx + 1, this.mMaxPunctCompressableSpaceSum);
        list.set(spaceSumIdx + 1, self.mMaxPunctCompressableSpaceSum);
        // java list.set(spaceSumIdx + 2, this.mMaxOtherCompressableSpaceSum);
        list.set(spaceSumIdx + 2, self.mMaxOtherCompressableSpaceSum);
      }
      // java else
      else {
        // java int insertIdx = -idx - 1;
        let insertIdx: i32 = -idx - 1;
        // java graphIndexes.insert(insertIdx, graphIdx);
        self.mCompressableGraphIndexes.as_mut().unwrap().insert(insertIdx, graphIdx);
        // java int spaceSumIdx = insertIdx * 3;
        let spaceSumIdx: i32 = insertIdx * 3;
        let list = self.mMaxCompressableSpaces.as_mut().unwrap();
        // java list.insert(spaceSumIdx, this.mMaxSpaceCompressableSpaceSum);
        list.insert(spaceSumIdx, self.mMaxSpaceCompressableSpaceSum);
        // java list.insert(spaceSumIdx + 1, this.mMaxPunctCompressableSpaceSum);
        list.insert(spaceSumIdx + 1, self.mMaxPunctCompressableSpaceSum);
        // java list.insert(spaceSumIdx + 2, this.mMaxOtherCompressableSpaceSum);
        list.insert(spaceSumIdx + 2, self.mMaxOtherCompressableSpaceSum);
      }
    }

    // java public void addSpaceCompressableSpace(int graphIdx, int space)
    pub fn addSpaceCompressableSpace(&mut self, graphIdx: i32, space: i32) {
      // java if (0 != space)
      if 0 != space {
        // java this.doAddCompressAfter(graphIdx, space);
        self.doAddCompressAfter(graphIdx, space);
      }
      // java this.doAddCompressableSpace(graphIdx, space, 0, 0);
      self.doAddCompressableSpace(graphIdx, space, 0, 0);
    }

    // java public void addPunctCompressableSpace(int graphIdx, int before, int after, int punct)
    pub fn addPunctCompressableSpace(&mut self, graphIdx: i32, before: i32, after: i32, punct: i32) {
      // java if (before != 0)
      if before != 0 {
        // java this.doAddCompressBefore(graphIdx, before);
        self.doAddCompressBefore(graphIdx, before);
      }
      // java if (after != 0)
      if after != 0 {
        // java this.doAddCompressAfter(graphIdx, after);
        self.doAddCompressAfter(graphIdx, after);
      }
      // java this.doAddCompressableSpace(graphIdx, 0, punct, 0);
      self.doAddCompressableSpace(graphIdx, 0, punct, 0);
    }

    // java public void addOtherCompressableSpace(int graphIdx, int other)
    pub fn addOtherCompressableSpace(&mut self, graphIdx: i32, other: i32) {
      // java if (0 != other)
      if 0 != other {
        // java this.doAddCompressAfter(graphIdx, other);
        self.doAddCompressAfter(graphIdx, other);
      }
      // java this.doAddCompressableSpace(graphIdx, 0, 0, other);
      self.doAddCompressableSpace(graphIdx, 0, 0, other);
    }

    // java public void restoreCompressState(int lastGraphIdx)
    pub fn restoreCompressState(&mut self, lastGraphIdx: i32) {
      // java int usedIdx = this.mCompressedGraphIndexes.binarySearch(lastGraphIdx);
      let mut usedIdx: i32 = self.mCompressedGraphIndexes.as_ref().unwrap().binarySearch(lastGraphIdx);
      // java if (usedIdx < 0)
      if usedIdx < 0 {
        // java usedIdx = Math.min(-usedIdx - 2, this.mCompressableGraphIndexes.size() - 1);
        usedIdx = (-usedIdx - 2).min(self.mCompressableGraphIndexes.as_ref().unwrap().size() - 1);
      }
      // java if (usedIdx < 0)
      if usedIdx < 0 {
        // java this.mUsedSpaceCompressableSpaceSum = 0;
        self.mUsedSpaceCompressableSpaceSum = 0;
        // java this.mUsedPunctCompressableSpaceSum = 0;
        self.mUsedPunctCompressableSpaceSum = 0;
        // java this.mUsedOtherCompressableSpaceSum = 0;
        self.mUsedOtherCompressableSpaceSum = 0;
      }
      // java else
      else {
        // java int idx = usedIdx * 3;
        let idx: i32 = usedIdx * 3;
        // java WIntArrayList list = this.mUsedCompressableSpaces;
        let list = self.mUsedCompressableSpaces.as_ref().unwrap();
        // java this.mUsedSpaceCompressableSpaceSum = list.get(idx);
        self.mUsedSpaceCompressableSpaceSum = list.get(idx);
        // java this.mUsedPunctCompressableSpaceSum = list.get(idx + 1);
        self.mUsedPunctCompressableSpaceSum = list.get(idx + 1);
        // java this.mUsedOtherCompressableSpaceSum = list.get(idx + 2);
        self.mUsedOtherCompressableSpaceSum = list.get(idx + 2);
      }
      // java int maxIdx = this.mCompressableGraphIndexes.binarySearch(lastGraphIdx);
      let mut maxIdx: i32 = self.mCompressableGraphIndexes.as_ref().unwrap().binarySearch(lastGraphIdx);
      // java if (maxIdx < 0)
      if maxIdx < 0 {
        // java maxIdx = Math.min(-maxIdx - 2, this.mCompressableGraphIndexes.size() - 1);
        maxIdx = (-maxIdx - 2).min(self.mCompressableGraphIndexes.as_ref().unwrap().size() - 1);
      }
      // java if (maxIdx < 0)
      if maxIdx < 0 {
        // java this.mMaxSpaceCompressableSpaceSum = 0;
        self.mMaxSpaceCompressableSpaceSum = 0;
        // java this.mMaxPunctCompressableSpaceSum = 0;
        self.mMaxPunctCompressableSpaceSum = 0;
        // java this.mMaxOtherCompressableSpaceSum = 0;
        self.mMaxOtherCompressableSpaceSum = 0;
      }
      // java else
      else {
        // java int idx = maxIdx * 3;
        let idx: i32 = maxIdx * 3;
        // java WIntArrayList list = this.mMaxCompressableSpaces;
        let list = self.mMaxCompressableSpaces.as_ref().unwrap();
        // java this.mMaxSpaceCompressableSpaceSum = list.get(idx);
        self.mMaxSpaceCompressableSpaceSum = list.get(idx);
        // java this.mMaxPunctCompressableSpaceSum = list.get(idx + 1);
        self.mMaxPunctCompressableSpaceSum = list.get(idx + 1);
        // java this.mMaxOtherCompressableSpaceSum = list.get(idx + 2);
        self.mMaxOtherCompressableSpaceSum = list.get(idx + 2);
      }
    }

    // java public void consumeCompressableSpaces(int compressGraphIdx, int needSpace)
    pub fn consumeCompressableSpaces(&mut self, compressGraphIdx: i32, mut needSpace: i32) {
      // java if ((float)needSpace <= 0.0F)
      if (needSpace as f32) <= 0.0 {
        // java this.mCompressedGraphIndexes.add(compressGraphIdx);
        self.mCompressedGraphIndexes.as_mut().unwrap().add(compressGraphIdx);
        // java this.mUsedCompressableSpaces.add(0);
        self.mUsedCompressableSpaces.as_mut().unwrap().add(0);
        // java this.mUsedCompressableSpaces.add(0);
        self.mUsedCompressableSpaces.as_mut().unwrap().add(0);
        // java this.mUsedCompressableSpaces.add(0);
        self.mUsedCompressableSpaces.as_mut().unwrap().add(0);
      }
      // java else
      else {
        // java int usedSpace = 0;
        let mut usedSpace: i32 = 0;
        // java int usedPunct = 0;
        let mut usedPunct: i32 = 0;
        // java int usedOther = 0;
        let mut usedOther: i32 = 0;
        // java if (0.0F != (float)needSpace)
        if 0.0 != (needSpace as f32) {
          // java usedSpace = Math.min(this.mMaxSpaceCompressableSpaceSum - this.mUsedSpaceCompressableSpaceSum, needSpace);
          usedSpace = (self.mMaxSpaceCompressableSpaceSum - self.mUsedSpaceCompressableSpaceSum).min(needSpace);
          // java this.mUsedSpaceCompressableSpaceSum += usedSpace;
          self.mUsedSpaceCompressableSpaceSum = self.mUsedSpaceCompressableSpaceSum + usedSpace;
          // java needSpace -= usedSpace;
          needSpace = needSpace - usedSpace;
        }
        // java if (0.0F != (float)needSpace)
        if 0.0 != (needSpace as f32) {
          // java usedPunct = Math.min(this.mMaxPunctCompressableSpaceSum - this.mUsedPunctCompressableSpaceSum, needSpace);
          usedPunct = (self.mMaxPunctCompressableSpaceSum - self.mUsedPunctCompressableSpaceSum).min(needSpace);
          // java this.mUsedPunctCompressableSpaceSum += usedPunct;
          self.mUsedPunctCompressableSpaceSum = self.mUsedPunctCompressableSpaceSum + usedPunct;
          // java needSpace -= usedPunct;
          needSpace = needSpace - usedPunct;
        }
        // java if (0.0F != (float)needSpace)
        if 0.0 != (needSpace as f32) {
          // java usedOther = Math.min(this.mMaxOtherCompressableSpaceSum - this.mUsedOtherCompressableSpaceSum, needSpace);
          usedOther = (self.mMaxOtherCompressableSpaceSum - self.mUsedOtherCompressableSpaceSum).min(needSpace);
          // java this.mUsedOtherCompressableSpaceSum += usedOther;
          self.mUsedOtherCompressableSpaceSum = self.mUsedOtherCompressableSpaceSum + usedOther;
          // java int var10000 = needSpace - usedOther;
          let mut var10000: i32 = needSpace - usedOther;
        }
        // java this.mCompressedGraphIndexes.add(compressGraphIdx);
        self.mCompressedGraphIndexes.as_mut().unwrap().add(compressGraphIdx);
        // java this.mUsedCompressableSpaces.add(this.mUsedSpaceCompressableSpaceSum);
        self.mUsedCompressableSpaces.as_mut().unwrap().add(self.mUsedSpaceCompressableSpaceSum);
        // java this.mUsedCompressableSpaces.add(this.mUsedPunctCompressableSpaceSum);
        self.mUsedCompressableSpaces.as_mut().unwrap().add(self.mUsedPunctCompressableSpaceSum);
        // java this.mUsedCompressableSpaces.add(this.mUsedOtherCompressableSpaceSum);
        self.mUsedCompressableSpaces.as_mut().unwrap().add(self.mUsedOtherCompressableSpaceSum);
      }
    }

    // java public int getUsedCompressableSpaceSum()
    pub fn getUsedCompressableSpaceSum(&self) -> i32 {
      // java return this.mUsedSpaceCompressableSpaceSum + this.mUsedPunctCompressableSpaceSum + this.mUsedOtherCompressableSpaceSum;
      return self.mUsedSpaceCompressableSpaceSum + self.mUsedPunctCompressableSpaceSum + self.mUsedOtherCompressableSpaceSum;
    }

    // java public int getLeftedCompressableSpaceSum()
    pub fn getLeftedCompressableSpaceSum(&self) -> i32 {
      // java int maxSpaceSum = this.mMaxSpaceCompressableSpaceSum + this.mMaxPunctCompressableSpaceSum + this.mMaxOtherCompressableSpaceSum;
      let maxSpaceSum: i32 = self.mMaxSpaceCompressableSpaceSum + self.mMaxPunctCompressableSpaceSum + self.mMaxOtherCompressableSpaceSum;
      // java int usedSpaceSum = this.getUsedCompressableSpaceSum();
      let usedSpaceSum: i32 = self.getUsedCompressableSpaceSum();
      // java return maxSpaceSum - usedSpaceSum;
      return maxSpaceSum - usedSpaceSum;
    }

    // java public boolean isFirstCompress()
    pub fn isFirstCompress(&self) -> bool {
      // java return this.mCompressedGraphIndexes.isEmpty();
      return self.mCompressedGraphIndexes.as_ref().unwrap().isEmpty();
    }

    // java public int getFirstCompressedGraphIndex()
    pub fn getFirstCompressedGraphIndex(&self) -> i32 {
      // java return this.isFirstCompress() ? Integer.MIN_VALUE : this.mCompressedGraphIndexes.get(0);
      return (if self.isFirstCompress() { i32::MIN } else { self.mCompressedGraphIndexes.as_ref().unwrap().get(0) });
    }

    // java public int getUsedSpaceCompressableSpaceSum()
    pub fn getUsedSpaceCompressableSpaceSum(&self) -> i32 {
      // java return this.mUsedSpaceCompressableSpaceSum;
      return self.mUsedSpaceCompressableSpaceSum;
    }

    // java public int getUsedPunctCompressableSpaceSum()
    pub fn getUsedPunctCompressableSpaceSum(&self) -> i32 {
      // java return this.mUsedPunctCompressableSpaceSum;
      return self.mUsedPunctCompressableSpaceSum;
    }

    // java public int getUsedOtherCompressableSpaceSum()
    pub fn getUsedOtherCompressableSpaceSum(&self) -> i32 {
      // java return this.mUsedOtherCompressableSpaceSum;
      return self.mUsedOtherCompressableSpaceSum;
    }

    // java public int getMaxSpaceCompressableSpaceSum()
    pub fn getMaxSpaceCompressableSpaceSum(&self) -> i32 {
      // java return this.mMaxSpaceCompressableSpaceSum;
      return self.mMaxSpaceCompressableSpaceSum;
    }

    // java public int getMaxPunctCompressableSpaceSum()
    pub fn getMaxPunctCompressableSpaceSum(&self) -> i32 {
      // java return this.mMaxPunctCompressableSpaceSum;
      return self.mMaxPunctCompressableSpaceSum;
    }

    // java public int getMaxOtherCompressableSpaceSum()
    pub fn getMaxOtherCompressableSpaceSum(&self) -> i32 {
      // java return this.mMaxOtherCompressableSpaceSum;
      return self.mMaxOtherCompressableSpaceSum;
    }

    // java public int getCompressBefore(int graphIndex)
    pub fn getCompressBefore(&self, graphIndex: i32) -> i32 {
      // java int index = this.mCompressBeforeIndexes.binarySearch(graphIndex);
      let index: i32 = self.mCompressBeforeIndexes.as_ref().unwrap().binarySearch(graphIndex);
      // java return index >= 0 ? this.mCompressBeforeValues.get(index) : 0;
      return (if index >= 0 { self.mCompressBeforeValues.as_ref().unwrap().get(index) } else { 0 });
    }

    // java private void doAddCompressBefore(int graphIndex, int before)
    pub fn doAddCompressBefore(&mut self, graphIndex: i32, before: i32) {
      // java if (this.mCompressBeforeIndexes.isEmpty())
      if self.mCompressBeforeIndexes.as_ref().unwrap().isEmpty() {
        // java this.mCompressBeforeIndexes.add(graphIndex);
        self.mCompressBeforeIndexes.as_mut().unwrap().add(graphIndex);
        // java this.mCompressBeforeValues.add(before);
        self.mCompressBeforeValues.as_mut().unwrap().add(before);
      }
      // java else
      else {
        // java int index = this.mCompressBeforeIndexes.binarySearch(graphIndex);
        let mut index: i32 = self.mCompressBeforeIndexes.as_ref().unwrap().binarySearch(graphIndex);
        // java if (index >= 0)
        if index >= 0 {
          // java this.mCompressBeforeValues.setQuick(index, this.mCompressBeforeValues.get(index) + before);
          let sum = self.mCompressBeforeValues.as_ref().unwrap().get(index) + before;
          self.mCompressBeforeValues.as_mut().unwrap().setQuick(index, sum);
        }
        // java else
        else {
          // java index = -index - 1;
          index = -index - 1;
          // java this.mCompressBeforeIndexes.insert(index, graphIndex);
          self.mCompressBeforeIndexes.as_mut().unwrap().insert(index, graphIndex);
          // java this.mCompressBeforeValues.insert(index, before);
          self.mCompressBeforeValues.as_mut().unwrap().insert(index, before);
        }
      }
    }

    // java public int getCompressAfter(int graphIndex)
    pub fn getCompressAfter(&self, graphIndex: i32) -> i32 {
      // java int index = this.mCompressAfterIndexes.binarySearch(graphIndex);
      let index: i32 = self.mCompressAfterIndexes.as_ref().unwrap().binarySearch(graphIndex);
      // java return index >= 0 ? this.mCompressAfterValues.get(index) : 0;
      return (if index >= 0 { self.mCompressAfterValues.as_ref().unwrap().get(index) } else { 0 });
    }

    // java private void doAddCompressAfter(int graphIndex, int after)
    pub fn doAddCompressAfter(&mut self, graphIndex: i32, after: i32) {
      // java if (this.mCompressAfterIndexes.isEmpty())
      if self.mCompressAfterIndexes.as_ref().unwrap().isEmpty() {
        // java this.mCompressAfterIndexes.add(graphIndex);
        self.mCompressAfterIndexes.as_mut().unwrap().add(graphIndex);
        // java this.mCompressAfterValues.add(after);
        self.mCompressAfterValues.as_mut().unwrap().add(after);
      }
      // java else
      else {
        // java int index = this.mCompressAfterIndexes.binarySearch(graphIndex);
        let mut index: i32 = self.mCompressAfterIndexes.as_ref().unwrap().binarySearch(graphIndex);
        // java if (index >= 0)
        if index >= 0 {
          // java this.mCompressAfterValues.setQuick(index, this.mCompressAfterValues.get(index) + after);
          let sum = self.mCompressAfterValues.as_ref().unwrap().get(index) + after;
          self.mCompressAfterValues.as_mut().unwrap().setQuick(index, sum);
        }
        // java else
        else {
          // java index = -index - 1;
          index = -index - 1;
          // java this.mCompressAfterIndexes.insert(index, graphIndex);
          self.mCompressAfterIndexes.as_mut().unwrap().insert(index, graphIndex);
          // java this.mCompressAfterValues.insert(index, after);
          self.mCompressAfterValues.as_mut().unwrap().insert(index, after);
        }
      }
    }

    // java public void setFinalCompress(int finalCompress)
    pub fn setFinalCompress(&mut self, finalCompress: i32) {
      // java this.mFinalCompress = finalCompress;
      self.mFinalCompress = finalCompress;
    }

    // java public int getFinalCompress()
    pub fn getFinalCompress(&self) -> i32 {
      // java return this.mFinalCompress;
      return self.mFinalCompress;
    }

}
