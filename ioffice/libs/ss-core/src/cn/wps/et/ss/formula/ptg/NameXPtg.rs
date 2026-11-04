// java public final class NameXPtg extends OperandPtg implements IWorkbookDependentPtg {
use super::Ptg::{Ptg,PtgKind};
use crate::{cn::wps::et::ss::{interfaces::IEvalWorkbook::IEvalWorkbook,formula::{renderer::RendererOption::RendererOption,SheetNameFormatter::SheetNameFormatter}},java_runtime::Character,org::apache::poi::ss::usermodel::ErrorConstants::ErrorConstants};
use util::cn::wps::moffice::util::StringUtil::StringUtil;
use crate::org::apache::poi::ss::{SpreadsheetVersion::SpreadsheetVersion,util::CellRangeAddressBase::CellRangeAddressBase};
impl Ptg{
    // java public String toString() { String retValue = "NameXPtg:[sheetRefIndex:" + this._sheetRefIndex + " , nameNumber:" + this._nameNumber + "]"; return retValue; }
    pub fn toNameXDebugString(&self)->String{let PtgKind::NameX{sheet_ref,name_number}=self.kind else{panic!("ClassCastException: NameXPtg")};format!("NameXPtg:[sheetRefIndex:{sheet_ref} , nameNumber:{name_number}]")}
    // java public String toFormulaString(IEvalWorkbook book,RendererOption opt){
    pub fn toNameXFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>)->String{
        let PtgKind::NameX{sheet_ref,name_number}=self.kind else{panic!("ClassCastException: NameXPtg")};let book=book.expect("NullPointerException");
        // java IEvalExtSheet extSheet=book.getExtSheet(this.getSheetRefIndex());IEvalNameBase aname=book.getNameByNameXPtg(this.getSheetRefIndex(),this.getNameIndex());
        let extSheet=book.getExtSheet(sheet_ref);let aname=book.getNameByNameXPtg(sheet_ref,name_number.wrapping_sub(1));
        // java int sheetIndex1Based=aname.getSheetIndexOneBased();StringBuilder sb=new StringBuilder();
        let aname=aname.as_ref().expect("NullPointerException");let sheetIndex1Based=aname.getSheetIndexOneBased();let mut sb=String::new();
        // java if(extSheet.getBookType()==3){return aname.getNameText();}
        let extSheet=extSheet.as_ref().expect("NullPointerException");if extSheet.getBookType()==3{return aname.getNameText();}
        // java else if(extSheet.getBookType()!=1&&extSheet.getBookType()!=2){
        if extSheet.getBookType()!=1&&extSheet.getBookType()!=2{
            // java IEvalExtName extName=(IEvalExtName)aname;String wbName=extSheet.getBookName();
            let extName=aname.as_ext_name().expect("ClassCastException: IEvalExtName");let mut wbName=extSheet.getBookName();
            // java if(opt.formulaType!=6&&opt.formulaType!=7){if(0==sheetIndex1Based){sb.append(wbName);}else{SheetNameFormatter.appendFormat(sb,wbName,extSheet.getSheetName(sheetIndex1Based-1));}sb.append('!').append(extName.getNameText());return sb.toString();}
            let opt=opt.expect("NullPointerException");if opt.formulaType!=6&&opt.formulaType!=7{if sheetIndex1Based==0{sb.push_str(wbName.as_deref().unwrap_or("null"));}else{let sheet=extSheet.getSheetName(sheetIndex1Based.wrapping_sub(1));SheetNameFormatter::appendFormat__workbook(&mut sb,wbName.as_deref(),sheet.as_deref());}sb.push('!');sb.push_str(&extName.getNameText());return sb;}
            // java else if(opt.ooxmlBookRefs==null){return ErrorConstants.getText(23);}else{boolean found=false;
            let Some(refs)=opt.ooxmlBookRefs.as_ref()else{return ErrorConstants::getText(23);};let mut found=false;
            // java for(int i=0;i<opt.ooxmlBookRefs.length;++i){if(opt.ooxmlBookRefs[i].equalsIgnoreCase(wbName)){wbName=Integer.toString(i+1);found=true;break;}}
            let refs=refs.borrow();for i in 0..refs.len(){let r=refs[i].as_deref().expect("NullPointerException");if wbName.as_ref().is_some_and(|n|Character::equalsIgnoreCase(&r.encode_utf16().collect::<Vec<_>>(),&n.encode_utf16().collect::<Vec<_>>())){wbName=Some((i+1).to_string());found=true;break;}}
            // java if(!found){return ErrorConstants.getText(23);}
            if !found{return ErrorConstants::getText(23);}
            // java if(0==sheetIndex1Based){sb.append('[').append(wbName).append(']');}else{SheetNameFormatter.appendFormat(sb,wbName,extSheet.getSheetName(sheetIndex1Based-1));}
            if sheetIndex1Based==0{sb.push('[');sb.push_str(wbName.as_deref().unwrap_or("null"));sb.push(']');}else{let sheet=extSheet.getSheetName(sheetIndex1Based.wrapping_sub(1));SheetNameFormatter::appendFormat__workbook(&mut sb,wbName.as_deref(),sheet.as_deref());}
            // java sb.append('!').append(extName.getNameText());return sb.toString();
            sb.push('!');sb.push_str(&extName.getNameText());sb
        }else{
            // java if(sheetIndex1Based>0){String sheetName=book.getSheetName(sheetIndex1Based-1);if(sheetName!=null&&sheetName.length()!=0){sb.append(sheetName).append('!');}else{sb.append("#REF!");}}
            if sheetIndex1Based>0{let sheetName=book.getSheetName(sheetIndex1Based.wrapping_sub(1));if let Some(sheetName)=sheetName.filter(|s|!s.is_empty()){sb.push_str(&sheetName);sb.push('!');}else{sb.push_str("#REF!");}}
            else{
                // java String wbName=StringUtil.getNamePart(book.getFileName());if(opt.formulaType!=6&&opt.formulaType!=7){
                let fileName=book.getFileName();let wbName=StringUtil::getNamePart(fileName.as_deref());let opt=opt.expect("NullPointerException");if opt.formulaType!=6&&opt.formulaType!=7{
                    // java if(0==sheetIndex1Based){sb.append(wbName);}else{SheetNameFormatter.appendFormat(sb,wbName,extSheet.getSheetName(sheetIndex1Based-1));}
                    if sheetIndex1Based==0{sb.push_str(&wbName);}else{let sheet=extSheet.getSheetName(sheetIndex1Based.wrapping_sub(1));SheetNameFormatter::appendFormat__workbook(&mut sb,Some(&wbName),sheet.as_deref());}
                // java else if(0==sheetIndex1Based){sb.append('[').append(0).append(']');}else{SheetNameFormatter.appendFormat(sb,wbName,extSheet.getSheetName(sheetIndex1Based-1));}
                }else if sheetIndex1Based==0{sb.push_str("[0]");}else{let sheet=extSheet.getSheetName(sheetIndex1Based.wrapping_sub(1));SheetNameFormatter::appendFormat__workbook(&mut sb,Some(&wbName),sheet.as_deref());}
                // java sb.append('!');
                sb.push('!');
            }
            // java sb.append(aname.getNameText());return sb.toString();
            sb.push_str(&aname.getNameText());sb
        }
    }
    // java public String toRCFormulaString(IEvalWorkbook book,RendererOption opt,CellRangeAddressBase formulaAddress,CellRangeAddressBase originAddress,SpreadsheetVersion version,boolean isName){return this.toFormulaString(book,opt);}
    pub fn toNameXRCFormulaString(&self,book:Option<&dyn IEvalWorkbook>,opt:Option<&RendererOption>,_formulaAddress:Option<&CellRangeAddressBase>,_originAddress:Option<&CellRangeAddressBase>,_version:SpreadsheetVersion,_isName:bool)->String{self.toNameXFormulaString(book,opt)}
    // java public String toFormulaString(){throw new RuntimeException("3D references need a workbook to determine formula text");}
    pub fn toNameXFormulaString__plain(&self)->String{panic!("RuntimeException: 3D references need a workbook to determine formula text")}
}
