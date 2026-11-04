use crate::{java_runtime::{Rc,Weak,RefCell},cn::wps::moss::app::KmoSheet::KmoSheet};
use kfc::cn::wps::memor4j::{KmoAtom::KmoAtom,KmoAtomData::KmoAtomDataSub,AtomSnapshot::SharedAtom};
use super::KmoRowInfoData::{self as Data,KmoRowInfoData};
#[cfg(test)]
#[path="KmoRowInfo.review.rs"]
pub(crate) mod review;
// java public class KmoRowInfo extends KmoAtom<KmoRowInfoData> {
pub struct KmoRowInfo{
    atom:Rc<RefCell<KmoAtom<KmoRowInfoData>>>,_owner:SharedAtom,
    // java protected KmoSheet kmoSheet;
    kmoSheet:RefCell<Option<Weak<RefCell<KmoSheet>>>>,
}
impl KmoRowInfo{
    // java public static final short DEFAULT_OPTION_FLAG = 256;
    pub const DEFAULT_OPTION_FLAG:i16=256;
    // java public static KmoRowInfo newAncestorAtom(int index, short height, short ixf, short outline, boolean colapse, boolean hidden, boolean unsynced, boolean ghostdirty, boolean exAsc, boolean exDsc) {
    pub fn newAncestorAtom(index:i32,height:i16,ixf:i16,outline:i16,colapse:bool,hidden:bool,unsynced:bool,ghostdirty:bool,exAsc:bool,exDsc:bool)->Self{
        // java KmoRowInfo info = new KmoRowInfo(index, height, ixf, outline, colapse, hidden, unsynced, ghostdirty, exAsc, exDsc);
        let info=Self::new__attrs(index,height,ixf,outline,colapse,hidden,unsynced,ghostdirty,exAsc,exDsc);
        // java ((KmoRowInfoData)info.rawData()).__forceOldest(); return info;
        info.atom.borrow_mut().rawData_mut().as_base_mut().__forceOldest();info
    }
    // java KmoRowInfo(int rowIndex) { super(new KmoRowInfoData(rowIndex)); }
    pub fn new(rowIndex:i32)->Self{Self::bind(KmoRowInfoData::new(rowIndex))}
    // java private KmoRowInfo(int index, short height, short raw_ixf, short outline, boolean colapse, boolean hidden, boolean unsynced, boolean ghostdirty, boolean exAsc, boolean exDsc) {
    fn new__attrs(index:i32,height:i16,raw_ixf:i16,outline:i16,colapse:bool,hidden:bool,unsynced:bool,ghostdirty:bool,exAsc:bool,exDsc:bool)->Self{
        // java super(new KmoRowInfoData(index, height, raw_ixf, outline, colapse, hidden, unsynced, ghostdirty, exAsc, exDsc));
        Self::bind(KmoRowInfoData::new__attrs(index,height,raw_ixf,outline,colapse,hidden,unsynced,ghostdirty,exAsc,exDsc))
    }
    // java public KmoRowInfo(int index, short height, short raw_ixf, short option_flags) { super(new KmoRowInfoData(index, height, raw_ixf, option_flags)); }
    pub fn KmoRowInfo(index:i32,height:i16,raw_ixf:i16,option_flags:i16)->Self{Self::bind(KmoRowInfoData::new__raw(index,height,raw_ixf,option_flags))}
    // java public void setHiddenRowsManager(KmoSheet sheet) {
    pub fn setHiddenRowsManager(&self,sheet:Option<Rc<RefCell<KmoSheet>>>){
        // java this.kmoSheet = sheet;
        *self.kmoSheet.borrow_mut()=sheet.as_ref().map(Rc::downgrade);
        // java boolean isHide = KmoRowInfoData.isHidden.isSet(((KmoRowInfoData)this.rawData()).option_flag);
        let isHide=Data::isHidden.isSet(self.atom.borrow().rawData().option_flag as i32);
        // java this.kmoSheet.getHiddenRowsManager().setHidden(((KmoRowInfoData)this.rawData()).rowIndex, isHide);
        let row=self.atom.borrow().rawData().rowIndex;self.sheet().expect("NullPointerException").borrow().getHiddenRowsManager().setHidden(row,isHide);
        // java this.setHiddenAttr(isHide);
        self.setHiddenAttr(isHide);
    }
    // java public int getMaxHeightCell() { return ((KmoRowInfoData)this.rawData()).maxHeightCell; }
    pub fn getMaxHeightCell(&self)->i32{self.atom.borrow().rawData().maxHeightCell}
    // java public void setMaxHeightCell(int col) { this.beforeWrite(); ((KmoRowInfoData)this.rawData()).maxHeightCell = col; }
    pub fn setMaxHeightCell(&self,col:i32){let mut atom=self.atom.borrow_mut();atom.beforeWrite();atom.rawData_mut().maxHeightCell=col;}
    // java public int getMinHeightCell() { return ((KmoRowInfoData)this.rawData()).minHeightCell; }
    pub fn getMinHeightCell(&self)->i32{self.atom.borrow().rawData().minHeightCell}
    // java public void setMinHeightCell(int col) { this.beforeWrite(); ((KmoRowInfoData)this.rawData()).minHeightCell = col; }
    pub fn setMinHeightCell(&self,col:i32){let mut atom=self.atom.borrow_mut();atom.beforeWrite();atom.rawData_mut().minHeightCell=col;}
    // java public int getRowIndex() { return ((KmoRowInfoData)this.rawData()).rowIndex & 1048575; }
    pub fn getRowIndex(&self)->i32{self.atom.borrow().rawData().rowIndex&1048575}
    // java public void setRowIndex(int row) { ((KmoRowInfoData)this.rawData()).rowIndex = row; }
    pub fn setRowIndex(&self,row:i32){self.atom.borrow_mut().rawData_mut().rowIndex=row;}
    // java public void setXFIndex(short ixf) { this.beforeWrite(); ((KmoRowInfoData)this.rawData()).raw_xfIndex = KmoRowInfoData.ixf.setShortValue(((KmoRowInfoData)this.rawData()).raw_xfIndex, ixf); }
    pub fn setXFIndex(&self,ixf:i16){let mut atom=self.atom.borrow_mut();atom.beforeWrite();let value=Data::ixf.setShortValue(atom.rawData().raw_xfIndex,ixf);atom.rawData_mut().raw_xfIndex=value;}
    // java public void setRawXFIndex(short raw_ixf) { this.beforeWrite(); ((KmoRowInfoData)this.rawData()).raw_xfIndex = raw_ixf; }
    pub fn setRawXFIndex(&self,raw_ixf:i16){let mut atom=self.atom.borrow_mut();atom.beforeWrite();atom.rawData_mut().raw_xfIndex=raw_ixf;}
    // java public short getXFIndex() { return KmoRowInfoData.ixf.getShortValue(((KmoRowInfoData)this.rawData()).raw_xfIndex); }
    pub fn getXFIndex(&self)->i16{Data::ixf.getShortValue(self.atom.borrow().rawData().raw_xfIndex)}
    // java public short getRawXFIndex() { return ((KmoRowInfoData)this.rawData()).raw_xfIndex; }
    pub fn getRawXFIndex(&self)->i16{self.atom.borrow().rawData().raw_xfIndex}
    // java public void setHeight(short height) { this.beforeWrite(); ((KmoRowInfoData)this.rawData()).height = height; }
    pub fn setHeight(&self,height:i16){let mut atom=self.atom.borrow_mut();atom.beforeWrite();atom.rawData_mut().height=height;}
    // java public short getHeight() { return ((KmoRowInfoData)this.rawData()).height; }
    pub fn getHeight(&self)->i16{self.atom.borrow().rawData().height}
    // java public void resetHiddenAttr(KmoSheet sheet, boolean rowOperable) {
    pub fn resetHiddenAttr(&self,sheet:Option<&Rc<RefCell<KmoSheet>>>,rowOperable:bool){
        // java sheet.getOptHiddenRowColManager().setRowOperable(((KmoRowInfoData)this.rawData()).rowIndex, rowOperable);
        let sheet=sheet.expect("NullPointerException");let manager=sheet.borrow().getOptHiddenRowColManager();let row=self.atom.borrow().rawData().rowIndex;manager.setRowOperable(row,rowOperable);
    }
    // java public void setHiddenAttr(boolean hidden) {
    pub fn setHiddenAttr(&self,hidden:bool){
        // java if (hidden) { this.kmoSheet.getOptHiddenRowColManager().setRowOperable(((KmoRowInfoData)this.rawData()).rowIndex, true); }
        if hidden{let sheet=self.sheet().expect("NullPointerException");let manager=sheet.borrow().getOptHiddenRowColManager();let row=self.atom.borrow().rawData().rowIndex;manager.setRowOperable(row,true);}
    }
    // java public void setHiddenForFilter(boolean hidden) {
    pub fn setHiddenForFilter(&self,hidden:bool){
        // java if (this.kmoSheet != null) { this.kmoSheet.getHiddenRowsManager().setHidden(((KmoRowInfoData)this.rawData()).rowIndex, hidden); }
        if let Some(sheet)=self.sheet(){let manager=sheet.borrow().getHiddenRowsManager();let row=self.atom.borrow().rawData().rowIndex;manager.setHidden(row,hidden);}
    }
    // java public void setHidden(boolean hidden) {
    pub fn setHidden(&self,hidden:bool){
        // java if (this.kmoSheet != null) { this.kmoSheet.getHiddenRowsManager().setHidden(((KmoRowInfoData)this.rawData()).rowIndex, hidden); this.setHiddenAttr(hidden); }
        if let Some(sheet)=self.sheet(){let manager=sheet.borrow().getHiddenRowsManager();let row=self.atom.borrow().rawData().rowIndex;manager.setHidden(row,hidden);self.setHiddenAttr(hidden);}
        // java else { this.beforeWrite(); ((KmoRowInfoData)this.rawData()).option_flag = KmoRowInfoData.isHidden.setShortBoolean(((KmoRowInfoData)this.rawData()).option_flag, hidden); }
        else{let mut atom=self.atom.borrow_mut();atom.beforeWrite();let flag=Data::isHidden.setShortBoolean(atom.rawData().option_flag,hidden);atom.rawData_mut().option_flag=flag;}
    }
    // java public void setUnsynced(boolean unsynced) { this.beforeWrite(); ((KmoRowInfoData)this.rawData()).option_flag = KmoRowInfoData.isUnsynced.setShortBoolean(((KmoRowInfoData)this.rawData()).option_flag, unsynced); }
    pub fn setUnsynced(&self,unsynced:bool){let mut atom=self.atom.borrow_mut();atom.beforeWrite();let flag=Data::isUnsynced.setShortBoolean(atom.rawData().option_flag,unsynced);atom.rawData_mut().option_flag=flag;}
    // java public boolean isHidden() { return this.kmoSheet != null ? this.kmoSheet.getHiddenRowsManager().isHidden(((KmoRowInfoData)this.rawData()).rowIndex) : KmoRowInfoData.isHidden.isSet(((KmoRowInfoData)this.rawData()).option_flag); }
    pub fn isHidden(&self)->bool{if let Some(sheet)=self.sheet(){let row=self.atom.borrow().rawData().rowIndex;sheet.borrow().getHiddenRowsManager().isHidden(row)}else{Data::isHidden.isSet(self.atom.borrow().rawData().option_flag as i32)}}
    // java public boolean isUnsynced() { return KmoRowInfoData.isUnsynced.isSet(((KmoRowInfoData)this.rawData()).option_flag); }
    pub fn isUnsynced(&self)->bool{Data::isUnsynced.isSet(self.atom.borrow().rawData().option_flag as i32)}
    // java public boolean isFormated() { return KmoRowInfoData.isFormatted.isSet(((KmoRowInfoData)this.rawData()).option_flag); }
    pub fn isFormated(&self)->bool{Data::isFormatted.isSet(self.atom.borrow().rawData().option_flag as i32)}
    // java public void setIsFormatted(boolean isFormatted) { this.beforeWrite(); ((KmoRowInfoData)this.rawData()).option_flag = (short)KmoRowInfoData.isFormatted.setBoolean(((KmoRowInfoData)this.rawData()).option_flag, isFormatted); }
    pub fn setIsFormatted(&self,isFormatted:bool){let mut atom=self.atom.borrow_mut();atom.beforeWrite();let flag=Data::isFormatted.setBoolean(atom.rawData().option_flag as i32,isFormatted)as i16;atom.rawData_mut().option_flag=flag;}
    // java public short getOptionFlags() {
    pub fn getOptionFlags(&self)->i16{
        // java ((KmoRowInfoData)this.rawData()).option_flag = (short)KmoRowInfoData.isHidden.setBoolean(((KmoRowInfoData)this.rawData()).option_flag, this.isHidden());
        let flag=self.atom.borrow().rawData().option_flag;let flag=Data::isHidden.setBoolean(flag as i32,self.isHidden())as i16;self.atom.borrow_mut().rawData_mut().option_flag=flag;
        // java return ((KmoRowInfoData)this.rawData()).option_flag;
        self.atom.borrow().rawData().option_flag
    }
    // java public void setOptionFlags(short flag) {
    pub fn setOptionFlags(&self,flag:i16){
        // java this.beforeWrite(); ((KmoRowInfoData)this.rawData()).option_flag = flag;
        {let mut atom=self.atom.borrow_mut();atom.beforeWrite();atom.rawData_mut().option_flag=flag;}
        // java if (this.kmoSheet != null) { this.kmoSheet.getHiddenRowsManager().setHidden(((KmoRowInfoData)this.rawData()).rowIndex, KmoRowInfoData.isHidden.isSet(((KmoRowInfoData)this.rawData()).option_flag)); }
        if let Some(sheet)=self.sheet(){let manager=sheet.borrow().getHiddenRowsManager();let data=self.atom.borrow().rawData();manager.setHidden(data.rowIndex,Data::isHidden.isSet(data.option_flag as i32));}
    }
    // java public boolean equals(Object o) {
    pub fn equals(&self,o:Option<&dyn std::any::Any>)->bool{
        // java if (o == this) { return true; } else if (!(o instanceof KmoRowInfo)) { return false; }
        let Some(info)=o.and_then(|o|o.downcast_ref::<Self>())else{return false;};if std::ptr::eq(self,info){return true;}
        // java else { KmoRowInfo info = (KmoRowInfo)o; return this.isHidden() == info.isHidden() && ((KmoRowInfoData)this.rawData()).equals(info.rawData()); }
        self.isHidden()==info.isHidden()&&self.atom.borrow().rawData().equals(Some(&*info.atom.borrow().rawData()))
    }
    // java public int hashCode() {
    pub fn hashCode(&self)->i32{
        // java int result = 17;
        let mut result=17i32;let data=self.atom.borrow().rawData();
        // java result = 31 * result + ((KmoRowInfoData)this.rawData()).maxHeightCell;
        result=result.wrapping_mul(31).wrapping_add(data.maxHeightCell);
        // java result = 31 * result + ((KmoRowInfoData)this.rawData()).height;
        result=result.wrapping_mul(31).wrapping_add(data.height as i32);
        // java result = 31 * result + ((KmoRowInfoData)this.rawData()).minHeightCell;
        result=result.wrapping_mul(31).wrapping_add(data.minHeightCell);
        // java result = 31 * result + ((KmoRowInfoData)this.rawData()).option_flag;
        result=result.wrapping_mul(31).wrapping_add(data.option_flag as i32);
        // java result = 31 * result + ((KmoRowInfoData)this.rawData()).rowIndex; return result;
        result=result.wrapping_mul(31).wrapping_add(data.rowIndex);result
    }
    // java public KmoRowInfo copy() {
    pub fn copy(&self)->Self{
        // java KmoRowInfoData data = (KmoRowInfoData)this.rawData();
        let data=self.atom.borrow().rawData();let(row,height,xf,max,min)=(data.rowIndex,data.height,data.raw_xfIndex,data.maxHeightCell,data.minHeightCell);drop(data);
        // java KmoRowInfo ret = new KmoRowInfo(data.rowIndex, data.height, data.raw_xfIndex, this.getOptionFlags());
        let ret=Self::KmoRowInfo(row,height,xf,self.getOptionFlags());
        // java ((KmoRowInfoData)ret.rawData()).maxHeightCell = data.maxHeightCell; ((KmoRowInfoData)ret.rawData()).minHeightCell = data.minHeightCell;
        let mut data=ret.atom.borrow_mut().rawData_mut();data.maxHeightCell=max;data.minHeightCell=min;
        // java ((KmoRowInfoData)ret.rawData()).__forceOldest(); return ret;
        data.as_base_mut().__forceOldest();drop(data);ret
    }
    // java public KmoRowInfo mergeCopy(KmoSheet extSheet, KmoBook curBook) { return this.mergeCopy(-1, extSheet, curBook); }
    pub fn mergeCopy(&self,extSheet:Option<&KmoSheet>,curBook:Option<&crate::cn::wps::moss::app::KmoBook::KmoBook>)->Self{self.mergeCopy__row(-1,extSheet,curBook)}
    // java public KmoRowInfo mergeCopy(int newRowIndex, KmoSheet extSheet, KmoBook curBook) {
    pub fn mergeCopy__row(&self,newRowIndex:i32,extSheet:Option<&KmoSheet>,curBook:Option<&crate::cn::wps::moss::app::KmoBook::KmoBook>)->Self{
        // java KmoRowInfoData data = (KmoRowInfoData)this.rawData();
        let data=self.atom.borrow().rawData();let(row,height,max,min)=(data.rowIndex,data.height,data.maxHeightCell,data.minHeightCell);drop(data);
        // java short index = this.getXFIndex();
        let mut index=self.getXFIndex();
        // java if (this.getRowIndex() >= 0) {
        if self.getRowIndex()>=0{
            // java KmoXF oldXF = extSheet.getRowXF(this.getRowIndex());
            let oldXF=extSheet.expect("NullPointerException").getRowXF(self.getRowIndex());
            // java KmoXF newXF = KmoXF.createXF(oldXF);
            let newXF=Rc::new(crate::cn::wps::moss::app::xf::KmoXF::KmoXF::createXF(oldXF.as_deref()));
            // java index = (short)curBook.getXFManager().checkXFIndex(newXF);
            index=curBook.expect("NullPointerException").getXFManager().borrow().checkXFIndex(Some(newXF))as i16;
        }
        // java int rowIndex = newRowIndex > 0 ? newRowIndex : data.rowIndex;
        let rowIndex=if newRowIndex>0{newRowIndex}else{row};
        // java KmoRowInfo ret = new KmoRowInfo(rowIndex, data.height, index, this.getOptionFlags());
        let ret=Self::KmoRowInfo(rowIndex,height,index,self.getOptionFlags());
        // java ((KmoRowInfoData)ret.rawData()).maxHeightCell = data.maxHeightCell; ((KmoRowInfoData)ret.rawData()).minHeightCell = data.minHeightCell;
        let mut data=ret.atom.borrow_mut().rawData_mut();data.maxHeightCell=max;data.minHeightCell=min;
        // java ((KmoRowInfoData)ret.rawData()).__forceOldest(); return ret;
        data.as_base_mut().__forceOldest();drop(data);ret
    }
    // Native KmoAtom superclass ownership and sheet reference transport.
    fn bind(data:KmoRowInfoData)->Self{let(atom,owner)=crate::cn::wps::moss::app::xf::snapshot::bind(data);Self{atom,_owner:owner,kmoSheet:RefCell::new(None)}}
    fn sheet(&self)->Option<Rc<RefCell<KmoSheet>>>{self.kmoSheet.borrow().as_ref().map(|s|s.upgrade().expect("NullPointerException"))}
}
