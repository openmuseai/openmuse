// java package cn.wps.et.ss.formula.ptg;
// rust: formatting methods borrow original coordinate fields through the Ptg
// carrier; full AreaPtgBase inheritance/field facade is not claimed by this view.
use super::Ptg::{Ptg,PtgKind,row_rel,col_rel,col_index};
use crate::{java_runtime::Rc,org::apache::poi::ss::{SpreadsheetVersion::SpreadsheetVersion,util::{CellReference::CellReference,CellRangeAddressBase::CellRangeAddressBase,AreaReference::AreaReference}}};
impl Ptg{
    // rust: native typed view of the original four coordinate fields.
    fn areaCoordinates(&self)->(i32,i32,u16,u16){match self.kind{PtgKind::Area{first_row,last_row,first_col_raw,last_col_raw,..}|PtgKind::Area3D{first_row,last_row,first_col_raw,last_col_raw,..}|PtgKind::Area3DExt{first_row,last_row,first_col_raw,last_col_raw,..}=>(first_row,last_row,first_col_raw,last_col_raw),_=>panic!("ClassCastException: AreaPtgBase")}}
    // Native mutable view of the same four original coordinate fields.
    fn areaCoordinatesMut(&mut self)->(&mut i32,&mut i32,&mut u16,&mut u16){match &mut self.kind{PtgKind::Area{first_row,last_row,first_col_raw,last_col_raw,..}|PtgKind::Area3D{first_row,last_row,first_col_raw,last_col_raw,..}|PtgKind::Area3DExt{first_row,last_row,first_col_raw,last_col_raw,..}=>(first_row,last_row,first_col_raw,last_col_raw),_=>panic!("ClassCastException: AreaPtgBase")}}
    // java protected AreaPtgBase(int firstRow, int lastRow, int firstColumn, int lastColumn, boolean firstRowRelative, boolean lastRowRelative, boolean firstColRelative, boolean lastColRelative) {
    pub(crate) fn initializeAreaCoordinates(&mut self,firstRow:i32,lastRow:i32,firstColumn:i32,lastColumn:i32,firstRowRelative:bool,lastRowRelative:bool,firstColRelative:bool,lastColRelative:bool){
        // java if (lastRow >= firstRow) { this.setFirstRow(firstRow); this.setLastRow(lastRow); this.setFirstRowRelative(firstRowRelative); this.setLastRowRelative(lastRowRelative); }
        if lastRow>=firstRow{self.setFirstRow(firstRow);self.setLastRow(lastRow);self.setFirstRowRelative(firstRowRelative);self.setLastRowRelative(lastRowRelative);}
        // java else { this.setFirstRow(lastRow); this.setLastRow(firstRow); this.setFirstRowRelative(lastRowRelative); this.setLastRowRelative(firstRowRelative); }
        else{self.setFirstRow(lastRow);self.setLastRow(firstRow);self.setFirstRowRelative(lastRowRelative);self.setLastRowRelative(firstRowRelative);}
        // java if (lastColumn >= firstColumn) { this.setFirstColumn(firstColumn); this.setLastColumn(lastColumn); this.setFirstColRelative(firstColRelative); this.setLastColRelative(lastColRelative); }
        if lastColumn>=firstColumn{self.setFirstColumn(firstColumn);self.setLastColumn(lastColumn);self.setFirstColRelative(firstColRelative);self.setLastColRelative(lastColRelative);}
        // java else { this.setFirstColumn(lastColumn); this.setLastColumn(firstColumn); this.setFirstColRelative(lastColRelative); this.setLastColRelative(firstColRelative); }
        else{self.setFirstColumn(lastColumn);self.setLastColumn(firstColumn);self.setFirstColRelative(lastColRelative);self.setLastColRelative(firstColRelative);}
    }
    // java public final int getFirstRow(){return this.field_1_first_row;}
    pub fn getFirstRow(&self)->i32{self.areaCoordinates().0}
    // java public final void setFirstRow(int rowIx) { this.field_1_first_row = rowIx; }
    pub fn setFirstRow(&mut self,rowIx:i32){*self.areaCoordinatesMut().0=rowIx;}
    // java public final int getLastRow(){return this.field_2_last_row;}
    pub fn getLastRow(&self)->i32{self.areaCoordinates().1}
    // java public final void setLastRow(int rowIx) { this.field_2_last_row = rowIx; }
    pub fn setLastRow(&mut self,rowIx:i32){*self.areaCoordinatesMut().1=rowIx;}
    // java public final int getFirstColumn(){return this.field_3_first_column&16383;}
    pub fn getFirstColumn(&self)->i32{col_index(self.areaCoordinates().2)as i32}
    // java public final short getFirstColumnRaw(){return (short)this.field_3_first_column;}
    pub fn getFirstColumnRaw(&self)->i16{self.areaCoordinates().2 as i16}
    // java public final boolean isFirstRowRelative(){return (this.field_3_first_column&32768)!=0;}
    pub fn isFirstRowRelative(&self)->bool{row_rel(self.areaCoordinates().2)}
    // java public final void setFirstRowRelative(boolean rel) { if (rel) { this.field_3_first_column |= 32768; } else { this.field_3_first_column &= -32769; } }
    pub fn setFirstRowRelative(&mut self,rel:bool){let field=self.areaCoordinatesMut().2;if rel{*field|=32768;}else{*field&=!32768;}}
    // java public final boolean isFirstColRelative(){return (this.field_3_first_column&16384)!=0;}
    pub fn isFirstColRelative(&self)->bool{col_rel(self.areaCoordinates().2)}
    // java public final void setFirstColRelative(boolean rel) { if (rel) { this.field_3_first_column |= 16384; } else { this.field_3_first_column &= -16385; } }
    pub fn setFirstColRelative(&mut self,rel:bool){let field=self.areaCoordinatesMut().2;if rel{*field|=16384;}else{*field&=!16384;}}
    // java public final void setFirstColumn(int colIx) { this.field_3_first_column &= -16384; this.field_3_first_column |= colIx & 16383; }
    pub fn setFirstColumn(&mut self,colIx:i32){let field=self.areaCoordinatesMut().2;*field&=!16383;*field|=(colIx&16383)as u16;}
    // java public final int getLastColumn(){return this.field_4_last_column&16383;}
    pub fn getLastColumn(&self)->i32{col_index(self.areaCoordinates().3)as i32}
    // java public final short getLastColumnRaw(){return (short)this.field_4_last_column;}
    pub fn getLastColumnRaw(&self)->i16{self.areaCoordinates().3 as i16}
    // java public final boolean isLastRowRelative(){return (this.field_4_last_column&32768)!=0;}
    pub fn isLastRowRelative(&self)->bool{row_rel(self.areaCoordinates().3)}
    // java public final void setLastRowRelative(boolean rel) { if (rel) { this.field_4_last_column |= 32768; } else { this.field_4_last_column &= -32769; } }
    pub fn setLastRowRelative(&mut self,rel:bool){let field=self.areaCoordinatesMut().3;if rel{*field|=32768;}else{*field&=!32768;}}
    // java public final boolean isLastColRelative(){return (this.field_4_last_column&16384)!=0;}
    pub fn isLastColRelative(&self)->bool{col_rel(self.areaCoordinates().3)}
    // java public final void setLastColRelative(boolean rel) { if (rel) { this.field_4_last_column |= 16384; } else { this.field_4_last_column &= -16385; } }
    pub fn setLastColRelative(&mut self,rel:bool){let field=self.areaCoordinatesMut().3;if rel{*field|=16384;}else{*field&=!16384;}}
    // java public final void setLastColumn(int colIx) { this.field_4_last_column &= -16384; this.field_4_last_column |= colIx & 16383; }
    pub fn setLastColumn(&mut self,colIx:i32){let field=self.areaCoordinatesMut().3;*field&=!16383;*field|=(colIx&16383)as u16;}
    // java protected final String formatReferenceAsString(){
    pub fn formatAreaReferenceAsString(&self)->String{
        // java CellReference topLeft=new CellReference(this.getFirstRow(),this.getFirstColumn(),!this.isFirstRowRelative(),!this.isFirstColRelative());
        let topLeft=Rc::new(CellReference::CellReference__absolute(self.getFirstRow(),self.getFirstColumn(),!self.isFirstRowRelative(),!self.isFirstColRelative()));
        // java CellReference botRight=new CellReference(this.getLastRow(),this.getLastColumn(),!this.isLastRowRelative(),!this.isLastColRelative());
        let botRight=Rc::new(CellReference::CellReference__absolute(self.getLastRow(),self.getLastColumn(),!self.isLastRowRelative(),!self.isLastColRelative()));
        // java return !AreaReference.isWholeColumnReference(topLeft,botRight)&&!AreaReference.isWholeRowReference(topLeft,botRight)?topLeft.formatAsString()+":"+botRight.formatAsString():(new AreaReference(topLeft,botRight)).formatAsString();
        if !AreaReference::isWholeColumnReference(&topLeft,&botRight)&&!AreaReference::isWholeRowReference(&topLeft,&botRight){format!("{}:{}",topLeft.formatAsString(),botRight.formatAsString())}else{AreaReference::fromCells(topLeft,botRight).formatAsString()}
    }
    // java protected final String formatRCReferenceAsString(CellRangeAddressBase formulaAddress,CellRangeAddressBase originAddress,SpreadsheetVersion version,boolean isName){
    pub fn formatAreaRCReferenceAsString(&self,formulaAddress:Option<&CellRangeAddressBase>,originAddress:Option<&CellRangeAddressBase>,version:SpreadsheetVersion,isName:bool)->String{
        // java int firstRow=this.getFirstRow();int firstCol=this.getFirstColumn();int lastRow=this.getLastRow();int lastCol=this.getLastColumn();
        let mut firstRow=self.getFirstRow();let mut firstCol=self.getFirstColumn();let mut lastRow=self.getLastRow();let mut lastCol=self.getLastColumn();
        // java if(isName){
        if isName{
            // java if(this.isFirstRowRelative()){firstRow=getRelativeRow(0,firstRow,version);}if(this.isFirstColRelative()){firstCol=getRelativeCol(0,firstCol,version);}
            if self.isFirstRowRelative(){firstRow=Self::getRelativeRow(0,firstRow,version);}if self.isFirstColRelative(){firstCol=Self::getRelativeCol(0,firstCol,version);}
            // java if(this.isLastRowRelative()){lastRow=getRelativeRow(0,lastRow,version);}if(this.isLastColRelative()){lastCol=getRelativeCol(0,lastCol,version);}
            if self.isLastRowRelative(){lastRow=Self::getRelativeRow(0,lastRow,version);}if self.isLastColRelative(){lastCol=Self::getRelativeCol(0,lastCol,version);}
        }
        // java CellReference topLeft=new CellReference(firstRow,firstCol,!this.isFirstRowRelative(),!this.isFirstColRelative());
        let topLeft=Rc::new(CellReference::CellReference__absolute(firstRow,firstCol,!self.isFirstRowRelative(),!self.isFirstColRelative()));
        // java CellReference botRight=new CellReference(lastRow,lastCol,!this.isLastRowRelative(),!this.isLastColRelative());
        let botRight=Rc::new(CellReference::CellReference__absolute(lastRow,lastCol,!self.isLastRowRelative(),!self.isLastColRelative()));
        // java return !AreaReference.isWholeColumnReference(topLeft,botRight)&&!AreaReference.isWholeRowReference(topLeft,botRight)?topLeft.formatAsRCString(formulaAddress,originAddress)+":"+botRight.formatAsRCString(formulaAddress,originAddress):(new AreaReference(topLeft,botRight)).formatAsRCString(formulaAddress,originAddress);
        if !AreaReference::isWholeColumnReference(&topLeft,&botRight)&&!AreaReference::isWholeRowReference(&topLeft,&botRight){format!("{}:{}",topLeft.formatAsRCString(formulaAddress,originAddress),botRight.formatAsRCString(formulaAddress,originAddress))}else{AreaReference::fromCells(topLeft,botRight).formatAsRCString(formulaAddress,originAddress)}
    }
}
