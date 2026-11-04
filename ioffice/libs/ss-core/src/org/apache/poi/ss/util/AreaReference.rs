// java package org.apache.poi.ss.util;
use super::{CellReference::CellReference,CellRangeAddressBase::CellRangeAddressBase};
use crate::{java_runtime::Rc,org::apache::poi::ss::SpreadsheetVersion::{SpreadsheetVersion,CUR_VERSION}};
use util::cn::wps::base::io::TypeConversion::TypeConversion;
// java public class AreaReference {
pub struct AreaReference{
    // java private final CellReference _firstCell;
    _firstCell:Rc<CellReference>,
    // java private final CellReference _lastCell;
    _lastCell:Rc<CellReference>,
    // java private final boolean _isSingleCell;
    _isSingleCell:bool,
}
#[cfg(test)]
#[path="AreaReference.review.rs"]
mod review;
// java private static final char SHEET_NAME_DELIMITER='!';
const SHEET_NAME_DELIMITER:char='!';
// java private static final char CELL_DELIMITER=':';
const CELL_DELIMITER:char=':';
// java private static final char SPECIAL_NAME_DELIMITER='\'';
const SPECIAL_NAME_DELIMITER:char='\'';
impl AreaReference{
    // java public AreaReference(String reference,SpreadsheetVersion version){
    pub fn new(reference:&str,version:SpreadsheetVersion)->Result<Self,String>{
        // java if(!isContiguous(reference)){throw new IllegalArgumentException("References passed to the AreaReference must be contiguous, use generateContiguous(ref) if you have non-contiguous references");}
        if !Self::isContiguous(reference){return Err("IllegalArgumentException: References must be contiguous".into());}
        // java String[] parts=separateAreaRefs(reference);String part0=parts[0];
        let parts=Self::separateAreaRefs(reference)?;let part0=&parts[0];
        // java if(parts.length==1){this._firstCell=new CellReference(part0);this._lastCell=this._firstCell;this._isSingleCell=true;}
        if parts.len()==1{let _firstCell=Rc::new(CellReference::fromString(part0)?);return Ok(Self{_lastCell:_firstCell.clone(),_firstCell,_isSingleCell:true});}
        // java else if(parts.length!=2){throw new IllegalArgumentException("Bad area ref '"+reference+"'");}
        if parts.len()!=2{return Err("IllegalArgumentException: Bad area ref".into());}
        // java String part1=parts[1];if(this.isPlainColumn(part0)){
        let part1=&parts[1];if Self::isPlainColumn(part0){
            // java if(!this.isPlainColumn(part1)){throw new RuntimeException("Bad area ref '"+reference+"'");}
            if !Self::isPlainColumn(part1){return Err("RuntimeException: Bad area ref".into());}
            // java boolean firstIsAbs=CellReference.isPartAbsolute(part0);boolean lastIsAbs=CellReference.isPartAbsolute(part1);
            let firstIsAbs=CellReference::isPartAbsolute(Some(part0));let lastIsAbs=CellReference::isPartAbsolute(Some(part1));
            // java int col0=CellReference.convertColStringToIndex(part0);int col1=CellReference.convertColStringToIndex(part1);
            let col0=CellReference::convertColStringToIndex(part0)?;let col1=CellReference::convertColStringToIndex(part1)?;
            // java this._firstCell=new CellReference(0,col0,true,firstIsAbs);this._lastCell=new CellReference(version.getMaxRows()-1,col1,true,lastIsAbs);this._isSingleCell=false;
            Ok(Self{_firstCell:Rc::new(CellReference::CellReference__absolute(0,col0,true,firstIsAbs)),_lastCell:Rc::new(CellReference::CellReference__absolute(version.getMaxRows()-1,col1,true,lastIsAbs)),_isSingleCell:false})
        // java else if(this.isPlainRow(part0)){
        }else if Self::isPlainRow(part0){
            // java if(!this.isPlainRow(part1)){throw new RuntimeException("Bad area ref '"+reference+"'");}
            if !Self::isPlainRow(part1){return Err("RuntimeException: Bad area ref".into());}
            // java boolean firstIsAbs=CellReference.isPartAbsolute(part0);boolean lastIsAbs=CellReference.isPartAbsolute(part1);
            let firstIsAbs=CellReference::isPartAbsolute(Some(part0));let lastIsAbs=CellReference::isPartAbsolute(Some(part1));
            // java Integer tmp0=TypeConversion.string2Integer(part0);tmp0=null==tmp0?1:tmp0;int row0=tmp0-1;
            let row0=TypeConversion::string2Integer(part0).unwrap_or(1).wrapping_sub(1);
            // java Integer tmp1=TypeConversion.string2Integer(part1);tmp1=null==tmp1?1:tmp1;int row1=tmp1-1;
            let row1=TypeConversion::string2Integer(part1).unwrap_or(1).wrapping_sub(1);
            // java this._firstCell=new CellReference(row0,0,firstIsAbs,true);this._lastCell=new CellReference(row1,version.getMaxColumns()-1,lastIsAbs,true);this._isSingleCell=false;
            Ok(Self{_firstCell:Rc::new(CellReference::CellReference__absolute(row0,0,firstIsAbs,true)),_lastCell:Rc::new(CellReference::CellReference__absolute(row1,version.getMaxColumns()-1,lastIsAbs,true)),_isSingleCell:false})
        }else{
            // java this._firstCell=new CellReference(part0);this._lastCell=new CellReference(part1);this._isSingleCell=part0.equals(part1);
            Ok(Self{_firstCell:Rc::new(CellReference::fromString(part0)?),_lastCell:Rc::new(CellReference::fromString(part1)?),_isSingleCell:part0==part1})
        }
    }
    // java private boolean isPlainColumn(String refPart){for(int i=refPart.length()-1;i>=0;--i){int ch=refPart.charAt(i);if((ch!=36||i!=0)&&(ch<65||ch>90)){return false;}}return true;}
    pub fn isPlainColumn(refPart:&str)->bool{let units:Vec<_>=refPart.encode_utf16().collect();for i in(0..units.len()).rev(){let ch=units[i];if(ch!=36||i!=0)&&!(65..=90).contains(&ch){return false;}}true}
    // java private boolean isPlainRow(String refPart){for(int i=refPart.length()-1;i>=0;--i){int ch=refPart.charAt(i);if((ch!=36||i!=0)&&(ch<48||ch>57)){return false;}}return true;}
    pub fn isPlainRow(refPart:&str)->bool{let units:Vec<_>=refPart.encode_utf16().collect();for i in(0..units.len()).rev(){let ch=units[i];if(ch!=36||i!=0)&&!(48..=57).contains(&ch){return false;}}true}
    // java public AreaReference(CellReference topLeft,CellReference botRight){
    pub fn fromCells(topLeft:Rc<CellReference>,botRight:Rc<CellReference>)->Self{
        // java boolean swapRows=topLeft.getRow()>botRight.getRow();boolean swapCols=topLeft.getCol()>botRight.getCol();
        let swapRows=topLeft.getRow()>botRight.getRow();let swapCols=topLeft.getCol()>botRight.getCol();
        // java if(!swapRows&&!swapCols){this._firstCell=topLeft;this._lastCell=botRight;}
        if !swapRows&&!swapCols{return Self{_firstCell:topLeft,_lastCell:botRight,_isSingleCell:false};}
        // java if(swapRows){firstRow=botRight.getRow();firstRowAbs=botRight.isRowAbsolute();lastRow=topLeft.getRow();lastRowAbs=topLeft.isRowAbsolute();}
        // java else{firstRow=topLeft.getRow();firstRowAbs=topLeft.isRowAbsolute();lastRow=botRight.getRow();lastRowAbs=botRight.isRowAbsolute();}
        let(firstRow,firstRowAbs,lastRow,lastRowAbs)=if swapRows{(botRight.getRow(),botRight.isRowAbsolute(),topLeft.getRow(),topLeft.isRowAbsolute())}else{(topLeft.getRow(),topLeft.isRowAbsolute(),botRight.getRow(),botRight.isRowAbsolute())};
        // java if(swapCols){firstColumn=botRight.getCol();firstColAbs=botRight.isColAbsolute();lastColumn=topLeft.getCol();lastColAbs=topLeft.isColAbsolute();}
        // java else{firstColumn=topLeft.getCol();firstColAbs=topLeft.isColAbsolute();lastColumn=botRight.getCol();lastColAbs=botRight.isColAbsolute();}
        let(firstColumn,firstColAbs,lastColumn,lastColAbs)=if swapCols{(botRight.getCol(),botRight.isColAbsolute(),topLeft.getCol(),topLeft.isColAbsolute())}else{(topLeft.getCol(),topLeft.isColAbsolute(),botRight.getCol(),botRight.isColAbsolute())};
        // java this._firstCell=new CellReference(firstRow,firstColumn,firstRowAbs,firstColAbs);this._lastCell=new CellReference(lastRow,lastColumn,lastRowAbs,lastColAbs);this._isSingleCell=false;
        Self{_firstCell:Rc::new(CellReference::CellReference__absolute(firstRow,firstColumn as i32,firstRowAbs,firstColAbs)),_lastCell:Rc::new(CellReference::CellReference__absolute(lastRow,lastColumn as i32,lastRowAbs,lastColAbs)),_isSingleCell:false}
    }
    // java public static boolean isContiguous(String reference){return reference.indexOf(44)==-1;}
    pub fn isContiguous(reference:&str)->bool{!reference.contains(',')}
    // java public static AreaReference getWholeRow(String start,String end,SpreadsheetVersion version){return new AreaReference("$A"+start+":$"+CellReference.convertNumToColString(version.getLastColumnIndex())+end,version);}
    pub fn getWholeRow(start:Option<&str>,end:Option<&str>,version:SpreadsheetVersion)->Result<Self,String>{Self::new(&format!("$A{}:${}{}",start.unwrap_or("null"),CellReference::convertNumToColString(version.getLastColumnIndex()),end.unwrap_or("null")),version)}
    // java public static AreaReference getWholeColumn(String start,String end,SpreadsheetVersion version){return new AreaReference(start+"$1:"+end+"$"+version.getMaxRows(),version);}
    pub fn getWholeColumn(start:Option<&str>,end:Option<&str>,version:SpreadsheetVersion)->Result<Self,String>{Self::new(&format!("{}$1:{}${}",start.unwrap_or("null"),end.unwrap_or("null"),version.getMaxRows()),version)}
    // java public static boolean isWholeColumnReference(CellReference topLeft,CellReference botRight){return topLeft.getRow()==0&&botRight.getRow()==SpreadsheetVersion.CUR_VERSION.getLastRowIndex()&&topLeft.isRowAbsolute()==botRight.isRowAbsolute();}
    pub fn isWholeColumnReference(topLeft:&CellReference,botRight:&CellReference)->bool{topLeft.getRow()==0&&botRight.getRow()==CUR_VERSION.lock().unwrap().getLastRowIndex()&&topLeft.isRowAbsolute()==botRight.isRowAbsolute()}
    // java public static boolean isWholeRowReference(CellReference topLeft,CellReference botRight){return topLeft.getCol()==0&&botRight.getCol()==SpreadsheetVersion.CUR_VERSION.getLastColumnIndex()&&topLeft.isColAbsolute()==botRight.isColAbsolute();}
    pub fn isWholeRowReference(topLeft:&CellReference,botRight:&CellReference)->bool{topLeft.getCol()==0&&botRight.getCol()as i32==CUR_VERSION.lock().unwrap().getLastColumnIndex()&&topLeft.isColAbsolute()==botRight.isColAbsolute()}
    // java public static AreaReference[] generateContiguous(String reference,SpreadsheetVersion version){ArrayList<AreaReference> refs=new ArrayList();StringTokenizer st=new StringTokenizer(reference,",");while(st.hasMoreTokens()){refs.add(new AreaReference(st.nextToken(),version));}return refs.toArray(new AreaReference[refs.size()]);}
    pub fn generateContiguous(reference:&str,version:SpreadsheetVersion)->Result<Vec<Self>,String>{reference.split(',').filter(|s|!s.is_empty()).map(|s|Self::new(s,version)).collect()}
    // java public boolean isSingleCell(){return this._isSingleCell;}
    pub fn isSingleCell(&self)->bool{self._isSingleCell}
    // java public CellReference getFirstCell(){return this._firstCell;}
    pub fn getFirstCell(&self)->&CellReference{&self._firstCell}
    // java public CellReference getLastCell(){return this._lastCell;}
    pub fn getLastCell(&self)->&CellReference{&self._lastCell}
    // java public CellReference[] getAllReferencedCells(){
    pub fn getAllReferencedCells(&self)->Vec<Rc<CellReference>>{
        // java if(this._isSingleCell){return new CellReference[]{this._firstCell};}
        if self._isSingleCell{return vec![self._firstCell.clone()];}
        // java int minRow=Math.min(this._firstCell.getRow(),this._lastCell.getRow());int maxRow=Math.max(this._firstCell.getRow(),this._lastCell.getRow());
        let minRow=self._firstCell.getRow().min(self._lastCell.getRow());let maxRow=self._firstCell.getRow().max(self._lastCell.getRow());
        // java int minCol=Math.min(this._firstCell.getCol(),this._lastCell.getCol());int maxCol=Math.max(this._firstCell.getCol(),this._lastCell.getCol());String sheetName=this._firstCell.getSheetName();ArrayList<CellReference> refs=new ArrayList();
        let minCol=self._firstCell.getCol().min(self._lastCell.getCol())as i32;let maxCol=self._firstCell.getCol().max(self._lastCell.getCol())as i32;let sheetName=self._firstCell.getSheetName();let mut refs=vec![];
        // java for(int row=minRow;row<=maxRow;++row){for(int col=minCol;col<=maxCol;++col){CellReference ref=new CellReference(sheetName,row,col,this._firstCell.isRowAbsolute(),this._firstCell.isColAbsolute());refs.add(ref);}}
        let mut row=minRow;while row<=maxRow{let mut col=minCol;while col<=maxCol{refs.push(Rc::new(CellReference::CellReference__sheet(sheetName.map(str::to_string),row,col,self._firstCell.isRowAbsolute(),self._firstCell.isColAbsolute())));col=col.wrapping_add(1);}row=row.wrapping_add(1);}
        // java return refs.toArray(new CellReference[refs.size()]);
        refs
    }
    // java public String formatAsString(){StringBuilder sb=new StringBuilder(32);
    pub fn formatAsString(&self)->String{
        let mut sb=String::with_capacity(32);
        // java if(isWholeRowReference(this._firstCell,this._lastCell)){
        if Self::isWholeRowReference(&self._firstCell,&self._lastCell){
            // java if(this._firstCell.isRowAbsolute()){sb.append('$');}sb.append(this._firstCell.getRow()+1);sb.append(':');if(this._lastCell.isRowAbsolute()){sb.append('$');}sb.append(this._lastCell.getRow()+1);return sb.toString();
            if self._firstCell.isRowAbsolute(){sb.push('$');}sb.push_str(&self._firstCell.getRow().wrapping_add(1).to_string());sb.push(':');if self._lastCell.isRowAbsolute(){sb.push('$');}sb.push_str(&self._lastCell.getRow().wrapping_add(1).to_string());sb
        // java else if(isWholeColumnReference(this._firstCell,this._lastCell)){
        }else if Self::isWholeColumnReference(&self._firstCell,&self._lastCell){
            // java if(this._firstCell.isColAbsolute()){sb.append('$');}sb.append(CellReference.convertNumToColString(this._firstCell.getCol()));sb.append(':');if(this._lastCell.isColAbsolute()){sb.append('$');}sb.append(CellReference.convertNumToColString(this._lastCell.getCol()));return sb.toString();
            if self._firstCell.isColAbsolute(){sb.push('$');}sb.push_str(&CellReference::convertNumToColString(self._firstCell.getCol()as i32));sb.push(':');if self._lastCell.isColAbsolute(){sb.push('$');}sb.push_str(&CellReference::convertNumToColString(self._lastCell.getCol()as i32));sb
        }else{
            // java sb.append(this._firstCell.formatAsString());if(!this._isSingleCell){sb.append(':');if(this._lastCell.getSheetName()==null){sb.append(this._lastCell.formatAsString());}else{this._lastCell.appendCellReference(sb);}}return sb.toString();
            sb.push_str(&self._firstCell.formatAsString());if !self._isSingleCell{sb.push(':');if self._lastCell.getSheetName().is_none(){sb.push_str(&self._lastCell.formatAsString());}else{self._lastCell.appendCellReference(&mut sb);}}sb
        }
    }
    // java public String formatAsRCString(CellRangeAddressBase formulaAddress,CellRangeAddressBase originAddress){StringBuilder sb=new StringBuilder(32);
    pub fn formatAsRCString(&self,formulaAddress:Option<&CellRangeAddressBase>,originAddress:Option<&CellRangeAddressBase>)->String{
        let mut sb=String::with_capacity(32);
        // java if(isWholeRowReference(this._firstCell,this._lastCell)){
        if Self::isWholeRowReference(&self._firstCell,&self._lastCell){
            // java sb.append('R');if(!this._firstCell.isRowAbsolute()){sb.append('[');sb.append(this._firstCell.getRow()-formulaAddress.getFirstRow());sb.append(']');}else{sb.append(this._firstCell.getRow()-originAddress.getFirstRow()+1);}
            sb.push('R');if !self._firstCell.isRowAbsolute(){sb.push('[');sb.push_str(&self._firstCell.getRow().wrapping_sub(formulaAddress.expect("NullPointerException").getFirstRow()).to_string());sb.push(']');}else{sb.push_str(&self._firstCell.getRow().wrapping_sub(originAddress.expect("NullPointerException").getFirstRow()).wrapping_add(1).to_string());}
            // java sb.append(':');sb.append('R');if(!this._lastCell.isRowAbsolute()){sb.append('[');sb.append(this._lastCell.getRow()-formulaAddress.getFirstRow());sb.append(']');}else{sb.append(this._lastCell.getRow()-originAddress.getFirstRow()+1);}return sb.toString();
            sb.push(':');sb.push('R');if !self._lastCell.isRowAbsolute(){sb.push('[');sb.push_str(&self._lastCell.getRow().wrapping_sub(formulaAddress.expect("NullPointerException").getFirstRow()).to_string());sb.push(']');}else{sb.push_str(&self._lastCell.getRow().wrapping_sub(originAddress.expect("NullPointerException").getFirstRow()).wrapping_add(1).to_string());}sb
        // java else if(isWholeColumnReference(this._firstCell,this._lastCell)){
        }else if Self::isWholeColumnReference(&self._firstCell,&self._lastCell){
            // java sb.append('C');if(!this._firstCell.isColAbsolute()){sb.append('[');sb.append(this._firstCell.getCol()-formulaAddress.getFirstColumn());sb.append(']');}else{sb.append(this._firstCell.getCol()-originAddress.getFirstColumn()+1);}
            sb.push('C');if !self._firstCell.isColAbsolute(){sb.push('[');sb.push_str(&(self._firstCell.getCol()as i32).wrapping_sub(formulaAddress.expect("NullPointerException").getFirstColumn()).to_string());sb.push(']');}else{sb.push_str(&(self._firstCell.getCol()as i32).wrapping_sub(originAddress.expect("NullPointerException").getFirstColumn()).wrapping_add(1).to_string());}
            // java sb.append(':');sb.append('C');if(!this._lastCell.isColAbsolute()){sb.append('[');sb.append(this._lastCell.getCol()-formulaAddress.getFirstColumn());sb.append(']');}else{sb.append(this._lastCell.getCol()-originAddress.getFirstColumn()+1);}return sb.toString();
            sb.push(':');sb.push('C');if !self._lastCell.isColAbsolute(){sb.push('[');sb.push_str(&(self._lastCell.getCol()as i32).wrapping_sub(formulaAddress.expect("NullPointerException").getFirstColumn()).to_string());sb.push(']');}else{sb.push_str(&(self._lastCell.getCol()as i32).wrapping_sub(originAddress.expect("NullPointerException").getFirstColumn()).wrapping_add(1).to_string());}sb
        }else{
            // java sb.append(this._firstCell.formatAsRCString(formulaAddress,originAddress));if(!this._isSingleCell){sb.append(':');if(this._lastCell.getSheetName()==null){sb.append(this._lastCell.formatAsRCString(formulaAddress,originAddress));}}return sb.toString();
            sb.push_str(&self._firstCell.formatAsRCString(formulaAddress,originAddress));if !self._isSingleCell{sb.push(':');if self._lastCell.getSheetName().is_none(){sb.push_str(&self._lastCell.formatAsRCString(formulaAddress,originAddress));}}sb
        }
    }
    // java public String toString(){StringBuffer sb=new StringBuffer(64);sb.append(this.getClass().getName()).append(" [");sb.append(this.formatAsString());sb.append("]");return sb.toString();}
    pub fn toString(&self)->String{let mut sb=String::with_capacity(64);sb.push_str("org.apache.poi.ss.util.AreaReference [");sb.push_str(&self.formatAsString());sb.push(']');sb}
    // java private static String[] separateAreaRefs(String reference){
    pub fn separateAreaRefs(reference:&str)->Result<Vec<String>,String>{
        // java int len=reference.length();int delimiterPos=-1;boolean insideDelimitedName=false;for(int i=0;i<len;++i){switch(reference.charAt(i)){
        // rust: byte positions of ASCII delimiters index the same original String boundaries.
        let chars:Vec<_>=reference.char_indices().collect();let(mut delimiterPos,mut insideDelimitedName,mut i)=(None,false,0);
        while i<chars.len(){let(pos,ch)=chars[i];match ch{
            // java case '\'':if(!insideDelimitedName){insideDelimitedName=true;}else{if(i>=len-1){throw new IllegalArgumentException(...);}if(reference.charAt(i+1)=='\''){++i;}else{insideDelimitedName=false;}}break;
            SPECIAL_NAME_DELIMITER=>{if !insideDelimitedName{insideDelimitedName=true;}else{if i>=chars.len()-1{return Err("IllegalArgumentException: Area ends with special name delimiter".into());}if chars[i+1].1==SPECIAL_NAME_DELIMITER{i+=1;}else{insideDelimitedName=false;}}},
            // java case ':':if(!insideDelimitedName){if(delimiterPos>=0){throw new IllegalArgumentException(...);}delimiterPos=i;}
            CELL_DELIMITER=>if !insideDelimitedName{if delimiterPos.is_some(){return Err("IllegalArgumentException: More than one cell delimiter".into());}delimiterPos=Some(pos);},_=>{}
        }i+=1;}
        // java if(delimiterPos<0){return new String[]{reference};}
        let Some(delimiterPos)=delimiterPos else{return Ok(vec![reference.into()]);};
        // java String partA=reference.substring(0,delimiterPos);String partB=reference.substring(delimiterPos+1);if(partB.indexOf(33)>=0){throw new RuntimeException(...);}
        let partA=&reference[..delimiterPos];let partB=&reference[delimiterPos+1..];if partB.contains(SHEET_NAME_DELIMITER){return Err("RuntimeException: Unexpected ! in second cell reference".into());}
        // java int plingPos=partA.lastIndexOf(33);if(plingPos<0){return new String[]{partA,partB};}else{String sheetName=partA.substring(0,plingPos+1);return new String[]{partA,sheetName+partB};}
        if let Some(plingPos)=partA.rfind(SHEET_NAME_DELIMITER){Ok(vec![partA.into(),format!("{}{partB}",&partA[..=plingPos])])}else{Ok(vec![partA.into(),partB.into()])}
    }
}
