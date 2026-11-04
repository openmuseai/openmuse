use crate::cn::wps::moffice::writer::cache::TypoPara::TypoPara;

// Java: public class OrgWidthCache
pub struct OrgWidthCache {
    // Java: private WIntArrayList graphIndexs, bufIndex, lens;
    graphIndexs: Vec<i32>,
    bufIndex: Vec<i32>,
    lens: Vec<i32>,
    // Java: private int[] mWidths = new int[32];
    mWidths: Vec<i32>,
    // Java: private int mBufSize = 0;
    mBufSize: i32,
}

impl OrgWidthCache {
    // Java: implicit default constructor.
    pub fn new() -> Self {
        Self { graphIndexs: Vec::new(), bufIndex: Vec::new(), lens: Vec::new(), mWidths: vec![0; 32], mBufSize: 0 }
    }

    // Java: public void save(int graphIndex, int graphLength, TypoPara typoPara)
    pub fn save(&mut self, graphIndex: i32, graphLength: i32, typoPara: &TypoPara) {
        // Java: int newSize = this.mBufSize + graphLength;
        let newSize = self.mBufSize + graphLength;
        // Java: if (newSize > this.mWidths.length) copy old buffer into new int[newSize].
        if newSize as usize > self.mWidths.len() { self.mWidths.resize(newSize as usize, 0); }
        // Java: typoPara.getWidths(graphIndex, graphLength, this.mWidths, this.mBufSize);
        typoPara.getWidths(graphIndex, graphLength, &mut self.mWidths, self.mBufSize);
        // Java: graphIndexs.add(graphIndex); bufIndex.add(mBufSize); lens.add(graphLength);
        self.graphIndexs.push(graphIndex);
        self.bufIndex.push(self.mBufSize);
        self.lens.push(graphLength);
        // Java: this.mBufSize = newSize;
        self.mBufSize = newSize;
    }

    // Java: public void checkSave(int graphIndex, int graphLength, TypoPara typoPara)
    pub fn checkSave(&mut self, graphIndex: i32, graphLength: i32, typoPara: &TypoPara) {
        // Java: if (graphIndexs.binarySearch(graphIndex) < 0) this.save(...);
        if self.graphIndexs.binary_search(&graphIndex).is_err() { self.save(graphIndex, graphLength, typoPara); }
    }

    // Java: public boolean restore(int graphIndex, TypoPara typoPara)
    pub fn restore(&self, graphIndex: i32, typoPara: &mut TypoPara) -> bool {
        // Java: boolean hasRestore = false;
        let mut hasRestore = false;
        // Java: for (int i = graphIndexs.size() - 1; i >= 0; --i)
        for i in (0..self.graphIndexs.len()).rev() {
            let curGraphIndex = self.graphIndexs[i];
            let curGraphEnd = curGraphIndex + self.lens[i];
            // Java: if (curGraphEnd > graphIndex)
            if curGraphEnd > graphIndex {
                let realGraphIndex = curGraphIndex.max(graphIndex);
                let realIndex = (self.bufIndex[i] + realGraphIndex - curGraphIndex) as usize;
                let len = (curGraphEnd - realGraphIndex) as usize;
                // Java: typoPara.putWidths(this.mWidths, realIndex, curGraphEnd-realGraphIndex, realGraphIndex);
                typoPara.putWidths(realGraphIndex, len as i32, &self.mWidths[realIndex..realIndex + len]);
                hasRestore = true;
            }
        }
        // Java: return hasRestore;
        hasRestore
    }

    // Java: public void clear()
    pub fn clear(&mut self) {
        // Java: graphIndexs.resetQuick(); bufIndex.resetQuick(); lens.resetQuick(); mBufSize = 0;
        self.graphIndexs.clear(); self.bufIndex.clear(); self.lens.clear(); self.mBufSize = 0;
    }
}
