// java package cn.wps.et.ss.formula.renderer;
use super::RendererOption::RendererOption;
use crate::{cn::wps::et::ss::{interfaces::IEvalWorkbook::IEvalWorkbook,formula::{IWorkbookDependentPtg::IWorkbookDependentPtg,ptg::Ptg::{PtgArray,PtgKind}}},org::apache::poi::ss::util::CellRangeAddressBase::CellRangeAddressBase};
use std::{collections::VecDeque,panic::{catch_unwind,AssertUnwindSafe,resume_unwind}};
// java public class FormulaRenderer {
pub struct FormulaRenderer;
// java private static final String TAG=null;
const TAG:Option<&str>=None;
// rust: native transport distinguishes Java Exception from Error and native
// boundary failures. Original workbook-dependent catch(Throwable) stays separate.
pub(crate) fn javaException(e:&(dyn std::any::Any+Send))->bool{if e.is::<crate::java_runtime::RuntimeException::RuntimeException>(){return true;}let text=e.downcast_ref::<String>().map(String::as_str).or_else(||e.downcast_ref::<&str>().copied()).unwrap_or("");["RuntimeException","NullPointerException","ClassCastException","IllegalArgumentException","IllegalStateException","IndexOutOfBoundsException","ArrayIndexOutOfBoundsException","StringIndexOutOfBoundsException","NoSuchElementException","NumberFormatException","ArithmeticException","UnsupportedOperationException","NegativeArraySizeException"].iter().any(|n|text.starts_with(n))}
fn javaChar(ch:u16)->char{char::from_u32(ch as u32).expect("native UTF16 char output boundary: isolated surrogate")}
impl FormulaRenderer{
    // java public static String toFormulaStringWithException(Ptg[] ptgs,IEvalWorkbook workbook){
    pub fn toFormulaStringWithException(ptgs:Option<&PtgArray>,workbook:Option<&dyn IEvalWorkbook>)->Option<String>{
        // java try{return toFormulaString(workbook,ptgs,RendererOption.createDefaultOption());}catch(Exception var4){
        let option=RendererOption::createDefaultOption();let result=catch_unwind(AssertUnwindSafe(||Self::toFormulaString(workbook,ptgs,Some(&option.borrow()))));match result{Ok(result)=>return result,Err(e)=>if !javaException(e.as_ref()){resume_unwind(e);}}
        // java if(ptgs!=null&&ptgs.length>0){Ptg vPtg=ptgs[0];
        if let Some(ptgs)=ptgs{if !ptgs.borrow().is_empty(){let value=ptgs.borrow()[0].clone();if let Some(value)=value{let value=value.borrow();
            // java if(vPtg instanceof IntPtg){return String.valueOf(((IntPtg)vPtg).getValue());}
            if matches!(value.kind,PtgKind::Int(_)){return Some(value.getIntValue().to_string());}
            // java if(vPtg instanceof StringPtg){return ((StringPtg)vPtg).getValue();}
            if matches!(value.kind,PtgKind::Str(_)|PtgKind::StrRaw{..}){return Some(value.getStringValue());}
            // java if(vPtg instanceof NumberPtg){return String.valueOf(((NumberPtg)vPtg).getValue());}
            if matches!(value.kind,PtgKind::Number(_)){return Some(kfc::java_runtime::FloatingDecimal::FloatingDecimal::toJavaFormatString(value.getNumberValue()));}
        }}}
        // java return null;
        None
    }
    // java public static String toFormulaString(IEvalWorkbook book,Ptg[] ptgs,RendererOption opt){return toFormulaString(book,ptgs,opt,null,null,false);}
    pub fn toFormulaString(book:Option<&dyn IEvalWorkbook>,ptgs:Option<&PtgArray>,opt:Option<&RendererOption>)->Option<String>{Self::toFormulaString__addresses(book,ptgs,opt,None,None,false)}
    // java public static String toFormulaString(IEvalWorkbook book,Ptg[] ptgs,RendererOption opt,CellRangeAddressBase formulaAddress,CellRangeAddressBase originAddress,boolean isName){
    pub fn toFormulaString__addresses(book:Option<&dyn IEvalWorkbook>,ptgs:Option<&PtgArray>,opt:Option<&RendererOption>,formulaAddress:Option<&CellRangeAddressBase>,originAddress:Option<&CellRangeAddressBase>,isName:bool)->Option<String>{
        // java boolean isRC=formulaAddress!=null;if(ptgs!=null&&ptgs.length!=0){LinkedList<String> stack=new LinkedList();
        let isRC=formulaAddress.is_some();let ptgs=ptgs.filter(|p|!p.borrow().is_empty()).expect("IllegalArgumentException: ptgs must not be null");let mut stack=VecDeque::<Option<String>>::new();
        // java for(int i=0;i<ptgs.length;++i){Ptg ptg=ptgs[i];
        let count=ptgs.borrow().len();for i in 0..count{let ptg=ptgs.borrow()[i].clone().expect("NullPointerException");let ptg=ptg.borrow();
            // java if(ptg.getSid()!=38&&ptg.getSid()!=41&&ptg.getSid()!=39){
            if ptg.getSid()!=38&&ptg.getSid()!=41&&ptg.getSid()!=39{
                // java if(ptg.getSid()==21){String contents=stack.removeFirst();stack.addFirst("("+contents+")");}
                if ptg.getSid()==21{let contents=stack.pop_front().expect("NoSuchElementException");stack.push_front(Some(format!("({})",contents.as_deref().unwrap_or("null"))));}
                // java else if(ptg.getSid()==25){AttrPtg attrPtg=(AttrPtg)ptg;
                else if ptg.getSid()==25{
                    // java if(!attrPtg.isOptimizedIf()&&!attrPtg.isOptimizedChoose()&&!attrPtg.isSkip()&&!attrPtg.isSpace()&&!attrPtg.isSemiVolatile()){
                    if !ptg.isOptimizedIf()&&!ptg.isOptimizedChoose()&&!ptg.isSkip()&&!ptg.isSpace()&&!ptg.isSemiVolatile(){
                        // java if(attrPtg.isSum()){String[] operands=getOperands(stack,attrPtg.getNumberOfOperands());stack.addFirst(attrPtg.toFormulaString(operands));}
                        if ptg.isSum(){let operands=Self::getOperands(&mut stack,ptg.getAttrOperandCount());stack.push_front(ptg.toAttrFormulaString__operands(Some(&operands)));}
                        // java else if(!attrPtg.isNull()){throw new RuntimeException("Unexpected tAttr: "+attrPtg.toString());}
                        else if !ptg.isNull(){panic!("RuntimeException: Unexpected tAttr: {}",ptg.attrToString());}
                    }
                // java else if(ptg instanceof IWorkbookDependentPtg){IWorkbookDependentPtg optg=(IWorkbookDependentPtg)ptg;try{
                }else if ptg.isWorkbookDependentPtg(){
                    let result=catch_unwind(AssertUnwindSafe(||{
                        // java if(isRC){stack.addFirst(optg.toRCFormulaString(book,opt,formulaAddress,originAddress,book.getSpreadsheetVersion(),isName));}
                        if isRC{let version=book.expect("NullPointerException").getSpreadsheetVersion();stack.push_front(Some(IWorkbookDependentPtg::toRCFormulaString(&*ptg,book,opt,formulaAddress,originAddress,version,isName)));}
                        // java else{stack.addFirst(optg.toFormulaString(book,opt));}
                        else{stack.push_front(Some(IWorkbookDependentPtg::toFormulaString(&*ptg,book,opt)));}
                    }));
                    // java catch(Throwable e){IOLog.e(TAG,"Throwable",e);}
                    if result.is_err(){eprintln!("{:?}: Throwable",TAG);}
                // java else if(ptg instanceof ArrayPtg){stack.addFirst(((ArrayPtg)ptg).toFormulaString(opt.DECIMAL_SEPARATOR,opt.CONST_ARRAY_SEPARATOR,opt.CONST_ARRAY_ROW_SEPARATOR));}
                }else if matches!(ptg.kind,PtgKind::Array{..}){let opt=opt.expect("NullPointerException");stack.push_front(Some(ptg.toArrayFormulaString__separators(javaChar(opt.DECIMAL_SEPARATOR),javaChar(opt.CONST_ARRAY_SEPARATOR),javaChar(opt.CONST_ARRAY_ROW_SEPARATOR))));
                // java else if(ptg instanceof NumberPtg){stack.addFirst(((NumberPtg)ptg).toFormulaString(opt.DECIMAL_SEPARATOR));}
                }else if matches!(ptg.kind,PtgKind::Number(_)){stack.push_front(Some(ptg.toNumberFormulaString__separator(javaChar(opt.expect("NullPointerException").DECIMAL_SEPARATOR))));
                // java else if(!(ptg instanceof OperationPtg)){if(isRC){stack.addFirst(ptg.toRCFormulaString(formulaAddress,originAddress,book.getSpreadsheetVersion(),isName));}else{stack.addFirst(ptg.toFormulaString());}}
                }else if !ptg.isOperationPtg(){if isRC{let version=book.expect("NullPointerException").getSpreadsheetVersion();stack.push_front(ptg.toRCFormulaString(formulaAddress,originAddress,version,isName));}else{stack.push_front(ptg.toFormulaString());}
                // java else{OperationPtg o=(OperationPtg)ptg;if(o.getNumberOfOperands()>=0){String[] operands=getOperands(stack,o.getNumberOfOperands());stack.addFirst(o.toFormulaString(operands,opt.PARAMETER_SEPARATOR,opt.UNION_SEPARATOR));}}
                }else if ptg.getNumberOfOperands()>=0{let operands=Self::getOperands(&mut stack,ptg.getNumberOfOperands());let opt=opt.expect("NullPointerException");stack.push_front(ptg.toFormulaString__operands(Some(&operands),javaChar(opt.PARAMETER_SEPARATOR),javaChar(opt.UNION_SEPARATOR)));}
            }
        }
        // java if(stack.isEmpty()){throw new IllegalStateException("Stack underflow");}else{String result=stack.removeFirst();if(!stack.isEmpty()){throw new IllegalStateException("too much stuff left on the stack");}else{return result;}}
        assert!(!stack.is_empty(),"IllegalStateException: Stack underflow");let result=stack.pop_front().unwrap();assert!(stack.is_empty(),"IllegalStateException: too much stuff left on the stack");result
    }
    // java private static String[] getOperands(LinkedList<String> stack,int nOperands){
    fn getOperands(stack:&mut VecDeque<Option<String>>,nOperands:i32)->Vec<Option<String>>{
        // java String[] operands=new String[nOperands];for(int j=nOperands-1;j>=0;--j){if(stack.isEmpty()){String msg="Too few arguments supplied to operation. Expected ("+nOperands+") operands but got ("+(nOperands-j-1)+")";throw new IllegalStateException(msg);}operands[j]=stack.removeFirst();}return operands;
        assert!(nOperands>=0,"NegativeArraySizeException");let mut operands=vec![None;nOperands as usize];for j in(0..nOperands as usize).rev(){assert!(!stack.is_empty(),"IllegalStateException: Too few arguments supplied to operation. Expected ({nOperands}) operands but got ({})",nOperands-j as i32-1);operands[j]=stack.pop_front().unwrap();}operands
    }
}

#[cfg(test)]
#[path="FormulaRenderer.review.rs"]
mod review;
