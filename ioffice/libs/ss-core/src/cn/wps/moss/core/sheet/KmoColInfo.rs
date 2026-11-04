// Java: cn.wps.moss.core.sheet.KmoColInfo.
use super::KmoColInfoData::KmoColInfoData;
#[derive(Clone, Debug)]
pub struct KmoColInfo {
    data: KmoColInfoData,
}

impl KmoColInfo {
    // Java: KmoColInfo(short min, short max, int width, short ixf, short option_flags)
    pub fn KmoColInfo(min: i16, max: i16, width: i32, ixf: i16, option_flags: i16) -> Self {
        Self { data: KmoColInfoData::KmoColInfoData(min, max, width, ixf, option_flags) }
    }

    // Java: KmoColInfo.newAncestorAtom(short min, short max, boolean hidden)
    pub fn newAncestorAtom__hidden(min: i16, max: i16, hidden: bool) -> Self {
        Self::KmoColInfo(min, max, 2560, 15, if hidden { 7 } else { 6 })
    }

    // java public static KmoColInfo newAncestorAtom(short minColIndex, short maxColIndex, int width, short xfIndex, short option_flags)
    pub fn newAncestorAtom(min: i16, max: i16, width: i32, xf_index: i16, option_flags: i16) -> Self {
        // java KmoColInfo info = new KmoColInfo(minColIndex, maxColIndex, width, xfIndex, option_flags);
        // java ((KmoColInfoData)info.rawData()).__forceOldest();
        // rust: no-stmt — KmoAtom 世代不在本轮。字段与构造器相同。
        Self::KmoColInfo(min, max, width, xf_index, option_flags)
    }

    // Java: getMinColIndex/getMaxColIndex/getColIndex/getWidth/getXFIndex/getOptionFlags/isHidden.
    pub fn getMinColIndex(&self) -> i16 { self.data.minColIndex }
    pub fn getMaxColIndex(&self) -> i16 { self.data.maxColIndex }
    pub fn getColIndex(&self) -> i16 { self.data.minColIndex }
    // java public int getColSpan()
    pub fn getColSpan(&self) -> i32 {
        // java return maxColIndex - minColIndex + 1;
        i32::from(self.data.maxColIndex) - i32::from(self.data.minColIndex) + 1
    }
    // java public void setColIndex(int min, int max)
    pub fn setColIndex__2(&mut self, min: i32, max: i32) {
        // java this.rawData().minColIndex = (short)min;
        self.data.minColIndex = min as i16;
        // java this.rawData().maxColIndex = (short)max;
        self.data.maxColIndex = max as i16;
    }
    pub fn getWidth(&self) -> i32 { self.data.width }
    pub fn getXFIndex(&self) -> i16 { self.data.xfIndex }
    pub fn getOptionFlags(&self) -> i16 { self.data.option_flags }
    pub fn isHidden(&self) -> bool { self.data.option_flags & 1 != 0 }
}
