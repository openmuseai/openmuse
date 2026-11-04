use crate::java_runtime::Cell;
// java public class CellAttrRecord_BIFF2 {
// The real RecordInputStream constructor is a separate BIFF-reader dependency.
// No replacement constructor or invented input protocol is provided here.
pub struct CellAttrRecord_BIFF2{
    // java private short xfIndex;
    xfIndex:Cell<i16>,
    // java private short cellOption;
    cellOption:Cell<i16>,
    // java private short formatIndex;
    formatIndex:Cell<i16>,
    // java private short fontIndex;
    fontIndex:Cell<i16>,
    // java private short horAlign;
    horAlign:Cell<i16>,
    // java private short borderOption;
    borderOption:Cell<i16>,
    // java private boolean hasBackground;
    hasBackground:Cell<bool>,
}
impl CellAttrRecord_BIFF2{
    // java public short getXfIndex() { return this.xfIndex; }
    pub fn getXfIndex(&self)->i16{self.xfIndex.get()}
    // java public short getCellOption() { return this.cellOption; }
    pub fn getCellOption(&self)->i16{self.cellOption.get()}
    // java public short getFormatIndex() { return this.formatIndex; }
    pub fn getFormatIndex(&self)->i16{self.formatIndex.get()}
    // java public short getFontIndex() { return this.fontIndex; }
    pub fn getFontIndex(&self)->i16{self.fontIndex.get()}
    // java public short getHorAlign() { return this.horAlign; }
    pub fn getHorAlign(&self)->i16{self.horAlign.get()}
    // java public short getBorderOption() { return this.borderOption; }
    pub fn getBorderOption(&self)->i16{self.borderOption.get()}
    // java public void setBorderOption(short option) { this.borderOption = option; }
    pub fn setBorderOption(&self,option:i16){self.borderOption.set(option);}
    // java public boolean hasBackground() { return this.hasBackground; }
    pub fn hasBackground(&self)->bool{self.hasBackground.get()}
    // java public int hashCode() {
    pub fn hashCode(&self)->i32{
        // java int prime = 31; int result = 1;
        let _prime=31;let mut result=1i32;
        // java result = 31 * result + this.borderOption;
        result=31i32.wrapping_mul(result).wrapping_add(self.borderOption.get()as i32);
        // java result = 31 * result + this.cellOption;
        result=31i32.wrapping_mul(result).wrapping_add(self.cellOption.get()as i32);
        // java result = 31 * result + this.fontIndex;
        result=31i32.wrapping_mul(result).wrapping_add(self.fontIndex.get()as i32);
        // java result = 31 * result + this.formatIndex;
        result=31i32.wrapping_mul(result).wrapping_add(self.formatIndex.get()as i32);
        // java result = 31 * result + (this.hasBackground ? 1231 : 1237);
        result=31i32.wrapping_mul(result).wrapping_add(if self.hasBackground.get(){1231}else{1237});
        // java result = 31 * result + this.horAlign;
        result=31i32.wrapping_mul(result).wrapping_add(self.horAlign.get()as i32);
        // java result = 31 * result + this.xfIndex; return result;
        result=31i32.wrapping_mul(result).wrapping_add(self.xfIndex.get()as i32);result
    }
    // java public boolean equals(Object obj) {
    // Native type metadata carries the original instanceof test; derived BIFF2
    // classes can return the same base reference without flattening their fields.
    pub fn equals(&self,obj:Option<&dyn CellAttrClass>)->bool{
        // java if (obj == null) { return false; } else if (!(obj instanceof CellAttrRecord_BIFF2)) { return false; }
        let Some(obj)=obj else{return false;};let Some(cellAttr)=obj.asCellAttrRecord_BIFF2()else{return false;};
        // java return this.xfIndex == cellAttr.getXfIndex() && this.cellOption == cellAttr.cellOption && this.formatIndex == cellAttr.getFormatIndex() && this.fontIndex == cellAttr.getFontIndex() && this.horAlign == cellAttr.getHorAlign() && this.borderOption == cellAttr.getBorderOption() && this.hasBackground == cellAttr.hasBackground;
        self.xfIndex.get()==cellAttr.getXfIndex()&&self.cellOption.get()==cellAttr.cellOption.get()&&self.formatIndex.get()==cellAttr.getFormatIndex()&&self.fontIndex.get()==cellAttr.getFontIndex()&&self.horAlign.get()==cellAttr.getHorAlign()&&self.borderOption.get()==cellAttr.getBorderOption()&&self.hasBackground.get()==cellAttr.hasBackground.get()
    }
}
// rust: Native Java class metadata, not an alternative BIFF model.
pub trait CellAttrClass{fn asCellAttrRecord_BIFF2(&self)->Option<&CellAttrRecord_BIFF2>;}
impl CellAttrClass for CellAttrRecord_BIFF2{fn asCellAttrRecord_BIFF2(&self)->Option<&CellAttrRecord_BIFF2>{Some(self)}}
