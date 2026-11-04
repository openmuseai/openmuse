// java package cn.wps.et.ss.formula.ptg;
use crate::{cn::wps::et::ss::{interfaces::IEvalWorkbook::IEvalWorkbook,formula::{SheetNameFormatter::SheetNameFormatter,renderer::RendererOption::RendererOption}},java_runtime::Character,org::apache::poi::ss::usermodel::ErrorConstants::ErrorConstants};
// java public final class ExternSheetNameResolver {
pub struct ExternSheetNameResolver;
impl ExternSheetNameResolver{
    // java private ExternSheetNameResolver() {}
    fn ExternSheetNameResolver()->Self{Self}
    // java public static String prependSheetName(IEvalWorkbook book,int field_1_index_extern_sheet,String cellRefText,RendererOption opt){
    pub fn prependSheetName(book:Option<&dyn IEvalWorkbook>,field_1_index_extern_sheet:i32,cellRefText:Option<&str>,opt:Option<&RendererOption>)->String{
        // java StringBuilder sb=new StringBuilder();IEvalExtSheet extSheet=book.getExtSheet(field_1_index_extern_sheet);
        let mut sb=String::new();let extSheet=book.expect("NullPointerException").getExtSheet(field_1_index_extern_sheet);
        // java if(extSheet.getBookType()==2){
        if extSheet.as_ref().expect("NullPointerException").getBookType()==2{
            // java String bookName=extSheet.getBookName();String sheetName=extSheet.getSheetName(0);if(bookName==null){bookName="";}
            let bookName=extSheet.as_ref().unwrap().getBookName();let sheetName=extSheet.as_ref().unwrap().getSheetName(0);let bookName=bookName.unwrap_or_default();
            // java SheetNameFormatter.appendFormat(sb,bookName,sheetName);sb.append('!');sb.append(cellRefText);return sb.toString();
            SheetNameFormatter::appendFormat__workbook(&mut sb,Some(&bookName),sheetName.as_deref());sb.push('!');sb.push_str(cellRefText.unwrap_or("null"));return sb;
        }
        // java if(extSheet.getBookType()==4){
        if extSheet.as_ref().unwrap().getBookType()==4{
            // java String bookName=extSheet.getBookName();if(opt.formulaType==6||opt.formulaType==7){
            let mut bookName=extSheet.as_ref().unwrap().getBookName();let opt=opt.expect("NullPointerException");if opt.formulaType==6||opt.formulaType==7{
                // java if(opt.ooxmlBookRefs==null){return ErrorConstants.getText(23);}boolean found=false;
                let Some(refs)=opt.ooxmlBookRefs.as_ref()else{return ErrorConstants::getText(23);};let mut found=false;
                // java for(int i=0;i<opt.ooxmlBookRefs.length;++i){if(opt.ooxmlBookRefs[i].equalsIgnoreCase(bookName)){bookName=Integer.toString(i+1);found=true;break;}}
                let refs=refs.borrow();for i in 0..refs.len(){let r=refs[i].as_deref().expect("NullPointerException");if bookName.as_ref().is_some_and(|n|Character::equalsIgnoreCase(&r.encode_utf16().collect::<Vec<_>>(),&n.encode_utf16().collect::<Vec<_>>())){bookName=Some((i+1).to_string());found=true;break;}}
                // java if(!found){return ErrorConstants.getText(23);}
                if !found{return ErrorConstants::getText(23);}
            }
            // java String sheetName=extSheet.getSheetName(extSheet.getFirstSheetIndex());SheetNameFormatter.appendFormat(sb,bookName,sheetName);
            let sheetName=extSheet.as_ref().unwrap().getSheetName(extSheet.as_ref().unwrap().getFirstSheetIndex());SheetNameFormatter::appendFormat__workbook(&mut sb,bookName.as_deref(),sheetName.as_deref());
        // java else if(extSheet.getFirstSheetIndex()==-1){if(opt.formulaType==6||opt.formulaType==7){return ErrorConstants.getText(23);}sb.append("#REF");}
        }else if extSheet.as_ref().unwrap().getFirstSheetIndex()==-1{let opt=opt.expect("NullPointerException");if opt.formulaType==6||opt.formulaType==7{return ErrorConstants::getText(23);}sb.push_str("#REF");
        // java else if(extSheet.getFirstSheetIndex()>=0){String sheetName=extSheet.getSheetName(extSheet.getFirstSheetIndex());
        }else if extSheet.as_ref().unwrap().getFirstSheetIndex()>=0{let sheetName=extSheet.as_ref().unwrap().getSheetName(extSheet.as_ref().unwrap().getFirstSheetIndex());
            // java if(sheetName!=null&&sheetName.length()!=0){SheetNameFormatter.appendFormat(sb,sheetName);}
            if sheetName.as_ref().is_some_and(|s|!s.is_empty()){SheetNameFormatter::appendFormat(&mut sb,sheetName.as_deref());}
            // java else{if(opt.formulaType==6||opt.formulaType==7){return ErrorConstants.getText(23);}sb.append("#REF");}
            else{let opt=opt.expect("NullPointerException");if opt.formulaType==6||opt.formulaType==7{return ErrorConstants::getText(23);}sb.push_str("#REF");}
        }
        // java sb.append('!');sb.append(cellRefText);return sb.toString();
        sb.push('!');sb.push_str(cellRefText.unwrap_or("null"));sb
    }
}
