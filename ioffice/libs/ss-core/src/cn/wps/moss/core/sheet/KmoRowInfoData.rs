use kfc::cn::wps::memor4j::KmoAtomData::{KmoAtomData,KmoAtomDataSub};
use util::org::apache::poi::util::{BitField::BitField,BitFieldFactory::BitFieldFactory};
use std::sync::LazyLock;
// java class KmoRowInfoData extends KmoAtomData {
pub(crate) struct KmoRowInfoData{
    base:KmoAtomData,
    // java protected int rowIndex;
    pub rowIndex:i32,
    // java protected short raw_xfIndex;
    pub raw_xfIndex:i16,
    // java protected short height;
    pub height:i16,
    // java protected short option_flag;
    pub option_flag:i16,
    // java protected int maxHeightCell = -1;
    pub maxHeightCell:i32,
    // java protected int minHeightCell = -1;
    pub minHeightCell:i32,
}
// java static final BitField outlineLevel = BitFieldFactory.getInstance(7);
pub(crate) static outlineLevel:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(7));
// java static final BitField colapsed = BitFieldFactory.getInstance(16);
pub(crate) static colapsed:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(16));
// java static final BitField isHidden = BitFieldFactory.getInstance(32);
pub(crate) static isHidden:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(32));
// java static final BitField isUnsynced = BitFieldFactory.getInstance(64);
pub(crate) static isUnsynced:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(64));
// java static final BitField isFormatted = BitFieldFactory.getInstance(128);
pub(crate) static isFormatted:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(128));
// java static final BitField ixf = BitFieldFactory.getInstance(4095);
pub(crate) static ixf:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(4095));
// java static final BitField exAsc = BitFieldFactory.getInstance(4096);
pub(crate) static exAsc:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(4096));
// java static final BitField exDsc = BitFieldFactory.getInstance(8192);
pub(crate) static exDsc:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(8192));
impl KmoRowInfoData{
    // java KmoRowInfoData(int row) { this.rowIndex = row; this.raw_xfIndex = 15; this.option_flag = 256; }
    pub fn new(row:i32)->Self{Self{base:KmoAtomData::new(),rowIndex:row,raw_xfIndex:15,height:0,option_flag:256,maxHeightCell:-1,minHeightCell:-1}}
    // java KmoRowInfoData(int row, short rowHeight, short ixf, short outline, boolean colapse, boolean hidden, boolean unsyned, boolean ghostdirty, boolean exAsc, boolean exDsc) {
    pub fn new__attrs(row:i32,rowHeight:i16,ixfValue:i16,outline:i16,colapse:bool,hidden:bool,unsyned:bool,ghostdirty:bool,asc:bool,dsc:bool)->Self{
        // java this.rowIndex = row; this.height = rowHeight; this.raw_xfIndex = 0;
        let mut data=Self{base:KmoAtomData::new(),rowIndex:row,raw_xfIndex:0,height:rowHeight,option_flag:0,maxHeightCell:-1,minHeightCell:-1};
        // java this.raw_xfIndex = KmoRowInfoData.ixf.setShortValue(this.raw_xfIndex, ixf);
        data.raw_xfIndex=ixf.setShortValue(data.raw_xfIndex,ixfValue);
        // java this.raw_xfIndex = KmoRowInfoData.exAsc.setShortBoolean(this.raw_xfIndex, exAsc);
        data.raw_xfIndex=exAsc.setShortBoolean(data.raw_xfIndex,asc);
        // java this.raw_xfIndex = KmoRowInfoData.exDsc.setShortBoolean(this.raw_xfIndex, exDsc);
        data.raw_xfIndex=exDsc.setShortBoolean(data.raw_xfIndex,dsc);
        // java this.option_flag = 256;
        data.option_flag=256;
        // java this.option_flag = outlineLevel.setShortValue(this.option_flag, outline);
        data.option_flag=outlineLevel.setShortValue(data.option_flag,outline);
        // java this.option_flag = (short)colapsed.setBoolean(this.option_flag, colapse);
        data.option_flag=colapsed.setBoolean(data.option_flag as i32,colapse)as i16;
        // java this.option_flag = (short)isHidden.setBoolean(this.option_flag, hidden);
        data.option_flag=isHidden.setBoolean(data.option_flag as i32,hidden)as i16;
        // java this.option_flag = (short)isUnsynced.setBoolean(this.option_flag, unsyned);
        data.option_flag=isUnsynced.setBoolean(data.option_flag as i32,unsyned)as i16;
        // java this.option_flag = (short)isFormatted.setBoolean(this.option_flag, ghostdirty);
        data.option_flag=isFormatted.setBoolean(data.option_flag as i32,ghostdirty)as i16;data
    }
    // java KmoRowInfoData(int row, short rowHeight, short raw_ixf, short option_flags) {
    pub fn new__raw(row:i32,rowHeight:i16,raw_ixf:i16,option_flags:i16)->Self{
        // java this.rowIndex = row; this.height = rowHeight; this.raw_xfIndex = raw_ixf; this.option_flag = option_flags;
        Self{base:KmoAtomData::new(),rowIndex:row,raw_xfIndex:raw_ixf,height:rowHeight,option_flag:option_flags,maxHeightCell:-1,minHeightCell:-1}
    }
    // java public KmoAtomData copy() {
    pub fn copy(&self)->Self{
        // java KmoRowInfoData info = new KmoRowInfoData(this.rowIndex);
        let mut info=Self::new(self.rowIndex);
        // java info.raw_xfIndex = this.raw_xfIndex; info.height = this.height; info.option_flag = this.option_flag; return info;
        info.raw_xfIndex=self.raw_xfIndex;info.height=self.height;info.option_flag=self.option_flag;info
    }
    // java public boolean equals(Object o) {
    pub fn equals(&self,o:Option<&dyn std::any::Any>)->bool{
        // java if (o == this) { return true; } else if (o == null) { return false; }
        let Some(o)=o else{return false;};if o.downcast_ref::<Self>().is_some_and(|o|std::ptr::eq(self,o)){return true;}
        // java else { KmoRowInfoData data = (KmoRowInfoData)o;
        let data=o.downcast_ref::<Self>().expect("ClassCastException");
        // java return this.raw_xfIndex == data.raw_xfIndex && this.height == data.height && this.option_flag == data.option_flag;
        self.raw_xfIndex==data.raw_xfIndex&&self.height==data.height&&self.option_flag==data.option_flag
    }
}
impl KmoAtomDataSub for KmoRowInfoData{fn as_base(&self)->&KmoAtomData{&self.base}fn as_base_mut(&mut self)->&mut KmoAtomData{&mut self.base}fn copy(&self)->Self{KmoRowInfoData::copy(self)}}
