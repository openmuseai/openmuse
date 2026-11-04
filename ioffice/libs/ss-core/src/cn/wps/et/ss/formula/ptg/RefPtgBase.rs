// java package cn.wps.et.ss.formula.ptg;
// rust: original formatting methods on the existing Ptg object carrier. The
// enum-to-coordinate match is a native field view, never a token clone. The
// complete RefPtgBase inheritance/field facade is still separately tracked.
use super::Ptg::{Ptg,PtgKind,row_rel,col_rel,col_index};
use crate::org::apache::poi::ss::{SpreadsheetVersion::SpreadsheetVersion,util::{CellReference::CellReference,CellRangeAddressBase::CellRangeAddressBase}};
impl Ptg{
    // rust: typed view of original field_1_row and field_2_col.
    fn refCoordinates(&self)->(i32,u16){match self.kind{PtgKind::Ref{row,col_raw,..}|PtgKind::Ref3D{row,col_raw,..}|PtgKind::Ref3DExt{row,col_raw,..}=>(row,col_raw),_=>panic!("ClassCastException: RefPtgBase")}}
    // Native mutable view of the same two original fields.
    fn refCoordinatesMut(&mut self)->(&mut i32,&mut u16){match &mut self.kind{PtgKind::Ref{row,col_raw,..}|PtgKind::Ref3D{row,col_raw,..}|PtgKind::Ref3DExt{row,col_raw,..}=>(row,col_raw),_=>panic!("ClassCastException: RefPtgBase")}}
    // java public final void setRow(int rowIndex) { this.field_1_row = rowIndex; }
    pub fn setRow(&mut self,rowIndex:i32){*self.refCoordinatesMut().0=rowIndex;}
    // java public final int getRow(){return this.field_1_row;}
    pub fn getRow(&self)->i32{self.refCoordinates().0}
    // java public final boolean isRowRelative(){return (this.field_2_col&32768)!=0;}
    pub fn isRowRelative(&self)->bool{row_rel(self.refCoordinates().1)}
    // java public final void setRowRelative(boolean rel) { if (rel) { this.field_2_col |= 32768; } else { this.field_2_col &= -32769; } }
    pub fn setRowRelative(&mut self,rel:bool){let field=self.refCoordinatesMut().1;if rel{*field|=32768;}else{*field&=!32768;}}
    // java public final boolean isColRelative(){return (this.field_2_col&16384)!=0;}
    pub fn isColRelative(&self)->bool{col_rel(self.refCoordinates().1)}
    // java public final void setColRelative(boolean rel) { if (rel) { this.field_2_col |= 16384; } else { this.field_2_col &= -16385; } }
    pub fn setColRelative(&mut self,rel:bool){let field=self.refCoordinatesMut().1;if rel{*field|=16384;}else{*field&=!16384;}}
    // java public final void setColumn(int col) { this.field_2_col &= -16384; this.field_2_col |= col & 16383; }
    pub fn setColumn(&mut self,col:i32){let field=self.refCoordinatesMut().1;*field&=!16383;*field|=(col&16383)as u16;}
    // java public final int getColumn(){return this.field_2_col&16383;}
    pub fn getColumn(&self)->i32{col_index(self.refCoordinates().1)as i32}
    // java protected final String formatReferenceAsString(){
    pub fn formatReferenceAsString(&self)->String{
        // java CellReference cr=new CellReference(this.getRow(),this.getColumn(),!this.isRowRelative(),!this.isColRelative());
        let cr=CellReference::CellReference__absolute(self.getRow(),self.getColumn(),!self.isRowRelative(),!self.isColRelative());
        // java return cr.formatAsString();
        cr.formatAsString()
    }
    // java protected final String formatRCReferenceAsString(CellRangeAddressBase formulaAddress,CellRangeAddressBase originAddress,SpreadsheetVersion version,boolean isName){
    pub fn formatRCReferenceAsString(&self,formulaAddress:Option<&CellRangeAddressBase>,originAddress:Option<&CellRangeAddressBase>,version:SpreadsheetVersion,isName:bool)->String{
        // java int row=this.getRow();int col=this.getColumn();
        let mut row=self.getRow();let mut col=self.getColumn();
        // java if(isName){if(this.isRowRelative()){row=getRelativeRow(0,row,version);}if(this.isColRelative()){col=getRelativeCol(0,col,version);}}
        if isName{if self.isRowRelative(){row=Self::getRelativeRow(0,row,version);}if self.isColRelative(){col=Self::getRelativeCol(0,col,version);}}
        // java CellReference cr=new CellReference(row,col,!this.isRowRelative(),!this.isColRelative());return cr.formatAsRCString(formulaAddress,originAddress);
        let cr=CellReference::CellReference__absolute(row,col,!self.isRowRelative(),!self.isColRelative());cr.formatAsRCString(formulaAddress,originAddress)
    }
}
