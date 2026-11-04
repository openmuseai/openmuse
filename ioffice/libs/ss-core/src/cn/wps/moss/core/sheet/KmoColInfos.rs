// Java: cn.wps.moss.core.sheet.KmoColInfos extends SparseIndexMap<KmoColInfo>.
use std::collections::BTreeMap;
use super::KmoColInfo::KmoColInfo;

#[derive(Default)]
pub struct KmoColInfos {
    // SparseIndexMap.put keys each range by its first column.
    blocks: BTreeMap<i32, KmoColInfo>,
}

impl KmoColInfos {
    pub fn KmoColInfos() -> Self { Self::default() }

    // Java: getColInfo(int colIndex): exact key, last range, then ascending iterator.
    pub fn getColInfo(&self, col_index: i32) -> Option<&KmoColInfo> {
        if let Some(info) = self.blocks.get(&col_index) { return Some(info); }
        let (_, last) = self.blocks.last_key_value()?;
        if col_index >= i32::from(last.getMinColIndex()) && col_index <= i32::from(last.getMaxColIndex()) {
            return Some(last);
        }
        if col_index < i32::from(last.getMaxColIndex()) {
            return self.blocks.values().find(|info| col_index >= i32::from(info.getMinColIndex()) && col_index <= i32::from(info.getMaxColIndex()));
        }
        None
    }

    // Java: initFillColInfo(int min, int max, int width, short ixf, short option_flags).
    pub fn initFillColInfo(&mut self, min: i32, max: i32, width: i32, ixf: i16, option_flags: i16) {
        let info = KmoColInfo::KmoColInfo(min as i16, max as i16, width, ixf, option_flags);
        self.blocks.insert(i32::from(info.getColIndex()), info);
    }

    // Java: initFillColInfo(int min, int max, boolean hidden).
    pub fn initFillColInfo__hidden(&mut self, min: i32, max: i32, hidden: bool) {
        self.blocks.insert(min, KmoColInfo::newAncestorAtom__hidden(min as i16, max as i16, hidden));
    }

    // Java: getLastCol().
    pub fn getLastCol(&self) -> i32 {
        self.blocks.last_key_value().map(|(_, info)| i32::from(info.getMaxColIndex())).unwrap_or(-1)
    }

    // java public int getFirstCol()
    pub fn getFirstCol(&self) -> i32 {
        // java return this.getFirstIndex();
        self.blocks.first_key_value().map(|(key, _)| *key).unwrap_or(-1)
    }

    // java public void setColInfo(KmoColInfo colInfo)
    pub fn setColInfo(&mut self, col_info: KmoColInfo) {
        // java colInfo.setOptHiddenRowColManager(this.kmoSheet);
        // rust: no-stmt — 构造器还没有 KmoSheet。隐藏属性在 sheet 为空时不改标志。
        // java this.put(colInfo.getColIndex(), colInfo);
        self.blocks.insert(i32::from(col_info.getColIndex()), col_info);
    }

    // java public KmoColInfo fitAndGetColInfo(int colIndex)
    pub fn fitAndGetColInfo(&mut self, col_index: i32) -> Option<KmoColInfo> {
        // java KmoColInfo colInfo = this.getColInfo(colIndex);
        let col_info = self.getColInfo(col_index)?.clone();
        // java this.checkSplitInfo(colInfo, (short)colIndex);
        self.checkSplitInfo(&col_info, col_index as i16);
        // java return colInfo;
        self.getColInfo(col_index).cloned()
    }

    // java public void checkSplitInfo(KmoColInfo colInfo, short col)
    pub fn checkSplitInfo(&mut self, col_info: &KmoColInfo, col: i16) {
        // java if (colInfo != null && colInfo.getColSpan() > 1 && col >= min && col <= max)
        if col_info.getColSpan() <= 1 {
            return;
        }
        let min = col_info.getMinColIndex();
        let max = col_info.getMaxColIndex();
        if col < min || col > max {
            return;
        }
        let width = col_info.getWidth();
        let xf = col_info.getXFIndex();
        let flags = col_info.getOptionFlags();
        self.blocks.remove(&i32::from(min));
        let mut narrowed = col_info.clone();
        // java colInfo.setColIndex(col, col);
        narrowed.setColIndex__2(i32::from(col), i32::from(col));
        if col == min {
            // java right = KmoColInfo.newAncestorAtom((short)(col + 1), max, width, xf, flags);
            let right = KmoColInfo::newAncestorAtom(col + 1, max, width, xf, flags);
            // java right.setOptHiddenRowColManager(this.kmoSheet);
            // rust: no-stmt
            self.blocks.insert(i32::from(narrowed.getColIndex()), narrowed);
            self.blocks.insert(i32::from(right.getColIndex()), right);
        } else if col == max {
            let left = KmoColInfo::newAncestorAtom(min, col - 1, width, xf, flags);
            self.blocks.insert(i32::from(left.getColIndex()), left);
            self.blocks.insert(i32::from(narrowed.getColIndex()), narrowed);
        } else {
            let left = KmoColInfo::newAncestorAtom(min, col - 1, width, xf, flags);
            let right = KmoColInfo::newAncestorAtom(col + 1, max, width, xf, flags);
            self.blocks.insert(i32::from(left.getColIndex()), left);
            self.blocks.insert(i32::from(narrowed.getColIndex()), narrowed);
            self.blocks.insert(i32::from(right.getColIndex()), right);
        }
    }
}

impl KmoColInfos{
// java IndexMapIterator<KmoColInfo> iterator()
pub fn iterator(&self)->impl Iterator<Item=&KmoColInfo>{self.blocks.values()}
}
