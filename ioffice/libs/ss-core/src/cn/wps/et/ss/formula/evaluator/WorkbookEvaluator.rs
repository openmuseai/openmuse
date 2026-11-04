// java package cn.wps.et.ss.formula.evaluator;
use super::{EvalResult, EvaluationContext::EvaluationContext};
use crate::cn::wps::et::ss::formula::eval::{
    BlankEval::BlankEval, MissingArgEval::MissingArgEval, NumberEval::NumberEval, ValueRef,
};
use crate::cn::wps::et::ss::interfaces::IEvalWorkbook::IEvalWorkbook;
use std::sync::Arc;
// java public final class WorkbookEvaluator
pub struct WorkbookEvaluator {
    _workbook: Arc<dyn IEvalWorkbook>,
}
impl WorkbookEvaluator {
    // java this._workbook = workbook;
    pub fn WorkbookEvaluator(workbook: Arc<dyn IEvalWorkbook>) -> Self {
        Self {
            _workbook: workbook,
        }
    }
    // java return this._workbook;
    pub fn getWorkbook(&self) -> &Arc<dyn IEvalWorkbook> {
        &self._workbook
    }
    // java getCellCachedValue(IEvalCell cell,IEvalWorkbook workbook)
    pub fn getCellCachedValue(
        cell: Option<Arc<dyn crate::cn::wps::et::ss::interfaces::IEvalCell::IEvalCell>>,
        workbook: Option<&Arc<dyn IEvalWorkbook>>,
    ) -> EvalResult<Option<ValueRef>> {
        use crate::cn::wps::et::ss::formula::eval::{
            BoolEval::BoolEval, ErrorEval::ErrorEval, StringEval::StringEval,
        };
        let Some(cell) = cell else {
            return Ok(Some(Arc::new(BlankEval::instance)));
        };
        if let Some(book) = workbook {
            if book.isInterruptCalc() {
                return Ok(None);
            }
            if book.getEvaluateCallback().is_some() {
                if let Some(ret) = cell.checkRecalculationFlag()? {
                    if let Some(v) = book.evaluateOutside(
                        cell.getSheetUid(),
                        cell.getRowIndex(),
                        cell.getColIndex(),
                        ret,
                    )? {
                        return Ok(Some(v));
                    }
                }
            }
        }
        Ok(Some(match cell.getFinalCellType() {
            0 => Arc::new(NumberEval::NumberEval(cell.getNumericCellValue())),
            1 => Arc::new(StringEval::StringEval__nullable(cell.getStringCellValue().as_deref())),
            3 => Arc::new(BlankEval::instance),
            4 => Arc::new(BoolEval::valueOf(cell.getBooleanCellValue())),
            5 => Arc::new(ErrorEval::valueOf(cell.getErrorCellValue() as i32)),
            _ => return Err(super::EvalFailure::Runtime("Unexpected cell final type")),
        }))
    }
    // java public static IValueEval dereferenceResult(IValueEval evaluationResult, EvaluationContext ec)
    pub fn dereferenceResult(value: ValueRef, ec: &EvaluationContext) -> EvalResult<ValueRef> {
        let value =
            match super::super::eval::OperandResolver::OperandResolver::getSingleValue(&value, ec)?
            {
                Ok(v) => v,
                Err(e) => return Ok(Arc::new(e)),
            };
        // java getSingleValue's scalar/reference branches; errors caught here return their original singleton.
        // java if (value == BlankEval.instance || value instanceof MissingArgEval) return NumberEval.ZERO;
        if value.as_any().is::<BlankEval>() || value.as_any().is::<MissingArgEval>() {
            Ok(Arc::new(NumberEval::ZERO))
        } else {
            Ok(value)
        }
    }
}
impl WorkbookEvaluator{
// java public IValueEval evaluateFormula(EvaluationContext ec, Ptg[] ptgs) {
// java LinkedList<IValueEval> stack = new LinkedList();
// java
// java for(int i = 0; i < ptgs.length; ++i) {
// java Ptg ptg = ptgs[i];
// java if (ptg.getSid() == 25) {
// java AttrPtg attrPtg = (AttrPtg)ptg;
// java if (attrPtg.isSum()) {
// java ptg = FuncVarPtg.SUM;
// java } else {
// java if (attrPtg.isOptimizedChoose()) {
// java IValueEval arg0 = (IValueEval)stack.removeFirst();
// java if (!(arg0 instanceof ArrayEval) && (!ec.isArrayFormula() || !(arg0 instanceof IAreaEval))) {
// java int[] jumpTable = attrPtg.getJumpTable();
// java int nChoices = jumpTable.length;
// java
// java int dist;
// java try {
// java int switchIndex = Choose.evaluateFirstArg(arg0, ec);
// java if (switchIndex >= 1 && switchIndex <= nChoices) {
// java dist = jumpTable[switchIndex - 1];
// java } else {
// java stack.addFirst(ErrorEval.VALUE_INVALID);
// java dist = attrPtg.getChooseFuncOffset() + 4;
// java }
// java } catch (EvaluationException e) {
// java stack.addFirst(e.getErrorEval());
// java dist = attrPtg.getChooseFuncOffset() + 4;
// java }
// java
// java dist -= nChoices * 2 + 2;
// java i += countTokensToBeSkipped(ptgs, i, dist);
// java continue;
// java }
// java
// java ptgs = cancelOptChoose(ptgs, i);
// java --i;
// java stack.addFirst(arg0);
// java continue;
// java }
// java
// java if (attrPtg.isOptimizedIf()) {
// java IValueEval arg0 = (IValueEval)stack.removeFirst();
// java if (!(arg0 instanceof ArrayEval) && (!ec.isArrayFormula() || !(arg0 instanceof IAreaEval))) {
// java boolean evaluatedPredicate;
// java try {
// java evaluatedPredicate = IfFunc.evaluateFirstArg(arg0, ec);
// java } catch (EvaluationException e) {
// java stack.addFirst(e.getErrorEval());
// java int dist = attrPtg.getData();
// java i += countTokensToBeSkipped(ptgs, i, dist);
// java attrPtg = (AttrPtg)ptgs[i];
// java dist = attrPtg.getData() + 1;
// java i += countTokensToBeSkipped(ptgs, i, dist);
// java continue;
// java }
// java
// java if (!evaluatedPredicate) {
// java int dist = attrPtg.getData();
// java i += countTokensToBeSkipped(ptgs, i, dist);
// java if (i + 1 < ptgs.length && ptgs[i].getSid() == 25 && ptgs[i + 1].getSid() == 34) {
// java AttrPtg ap = (AttrPtg)ptgs[i];
// java FuncVarPtg fp = (FuncVarPtg)ptgs[i + 1];
// java if (ap.isSkip() && ap.getData() + 1 == 4 && fp.getFunctionIndex() == 1) {
// java ++i;
// java stack.addFirst(BoolEval.FALSE);
// java }
// java }
// java }
// java continue;
// java }
// java
// java ptgs = cancelOptIf(ptgs, i);
// java --i;
// java stack.addFirst(arg0);
// java continue;
// java }
// java
// java if (attrPtg.isSkip()) {
// java int dist = attrPtg.getData() + 1;
// java i += countTokensToBeSkipped(ptgs, i, dist);
// java if (stack.peek() == MissingArgEval.instance) {
// java stack.removeFirst();
// java stack.addFirst(BlankEval.instance);
// java }
// java continue;
// java }
// java }
// java }
// java
// java if (!(ptg instanceof ControlPtg) && ptg.getSid() != 41 && ptg.getSid() != 38 && ptg.getSid() != 39) {
// java IValueEval opResult = null;
// java if (ptg instanceof OperationPtg) {
// java OperationPtg optg = (OperationPtg)ptg;
// java if (optg.getSid() == 16) {
// java IValueEval last = (IValueEval)stack.removeFirst();
// java stack.addFirst(new UnionEval((IValueEval)stack.removeFirst(), last));
// java continue;
// java }
// java
// java int numops = optg.getNumberOfOperands();
// java IValueEval[] ops = new IValueEval[numops];
// java
// java for(int j = numops - 1; j >= 0; --j) {
// java IValueEval p = (IValueEval)stack.removeFirst();
// java ops[j] = p;
// java }
// java
// java if (i == ptgs.length - 1) {
// java if (optg == AddPtg.instance) {
// java opResult = TwoOperandNumericOperation.RoundAddEval.evaluate(ptg.getPtgClass(), ops, ec);
// java } else if (optg == SubtractPtg.instance) {
// java opResult = TwoOperandNumericOperation.RoundSubtractEval.evaluate(ptg.getPtgClass(), ops, ec);
// java }
// java }
// java
// java if (opResult == null) {
// java opResult = OperationEvaluatorFactory.evaluate(optg, ops, ec);
// java }
// java } else {
// java opResult = this.getEvalForOperandPtg(ptg, ec);
// java }
// java
// java if (opResult == null) {
// java throw new RuntimeException("Evaluation result must not be null");
// java }
// java
// java stack.addFirst(opResult);
// java }
// java }
// java
// java IValueEval value = !stack.isEmpty() ? (IValueEval)stack.removeFirst() : null;
// java if (!stack.isEmpty()) {
// java throw new IllegalStateException("evaluation stack not empty");
// java } else {
// java return value;
// java }
// java }
pub fn evaluateFormula(&self,ec:&mut EvaluationContext,input:&[crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg])->EvalResult<Option<ValueRef>> {
use crate::cn::wps::et::ss::formula::{ptg::Ptg::{Ptg,PtgKind},functions::{IfFunc::IfFunc,base::IFunction::IFunction,builtin::TwoOperandNumericOperation::TwoOperandNumericOperation as N},eval::{ErrorEval::ErrorEval,BoolEval::BoolEval,BlankEval::BlankEval,MissingArgEval::MissingArgEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}}};use super::{EvalFailure,OperationEvaluatorFactory::OperationEvaluatorFactory};let mut ptgs=input.to_vec();let mut stack=std::collections::VecDeque::<ValueRef>::new();let mut i=0;while i<ptgs.len(){let mut ptg=ptgs[i].clone();if let PtgKind::Attr{options,data,..}=ptg.kind{if options&16!=0{let meta=crate::cn::wps::et::ss::formula::ptg::Ptg::function_by_name("SUM").unwrap();ptg=Ptg{ java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),kind:PtgKind::FuncVar{meta,nargs:1},class:32};}else{if options&4!=0{return Err(EvalFailure::Unported("AttrPtg choose jump table"));}if options&2!=0{let arg=stack.pop_front().ok_or(EvalFailure::Runtime("NoSuchElementException"))?;if !arg.as_any().is::<ArrayEval>()&&(!ec.isArrayFormula()||arg.as_area_eval().is_none()){match IfFunc::evaluateFirstArg(&arg,ec)?{Err(e)=>{stack.push_front(Arc::new(e));i+=Self::countTokensToBeSkipped(&ptgs,i,data as i32)?;let PtgKind::Attr{data,..}=ptgs[i].kind else{return Err(EvalFailure::Runtime("ClassCastException AttrPtg"));};i+=Self::countTokensToBeSkipped(&ptgs,i,data as i32+1)?;},Ok(false)=>{i+=Self::countTokensToBeSkipped(&ptgs,i,data as i32)?;if i+1<ptgs.len(){if let(PtgKind::Attr{options,data,..},PtgKind::FuncVar{meta,..})=(&ptgs[i].kind,&ptgs[i+1].kind){if options&8!=0&&*data as i32+1==4&&meta.getIndex()==1{i+=1;stack.push_front(Arc::new(BoolEval::FALSE));}}}},Ok(true)=>{}}i+=1;continue;}ptgs=Self::cancelOptIf(&ptgs,i)?;stack.push_front(arg);continue;}
if options&8!=0{i+=Self::countTokensToBeSkipped(&ptgs,i,data as i32+1)?;if stack.front().is_some_and(|v|v.as_any().is::<MissingArgEval>()){stack.pop_front();stack.push_front(Arc::new(BlankEval::instance));}i+=1;continue;}}}
if matches!(ptg.kind,PtgKind::Paren|PtgKind::Attr{..}|PtgKind::MemFunc{..}|PtgKind::MemArea{..}|PtgKind::MemAreaInitial{..}|PtgKind::MemErr{..}){i+=1;continue;}
if matches!(ptg.kind,PtgKind::Union){let last=stack.pop_front().ok_or(EvalFailure::Runtime("NoSuchElementException"))?;let first=stack.pop_front().ok_or(EvalFailure::Runtime("NoSuchElementException"))?;stack.push_front(Arc::new(UnionEval::new(first,last)));i+=1;continue;}
let nargs=match &ptg.kind{PtgKind::Add|PtgKind::Subtract|PtgKind::Multiply|PtgKind::Divide|PtgKind::Power|PtgKind::Equal|PtgKind::NotEqual|PtgKind::GreaterEqual|PtgKind::GreaterThan|PtgKind::LessEqual|PtgKind::LessThan|PtgKind::Concat|PtgKind::Range|PtgKind::Intersection=>Some(2),PtgKind::UnaryPlus|PtgKind::UnaryMinus|PtgKind::Percent=>Some(1),PtgKind::Func{meta}=>Some(meta.getMinParams()as usize),PtgKind::FuncVar{nargs,..}|PtgKind::FuncVarExt{nargs,..}=>Some(*nargs as usize),_=>None};
let value=if let Some(n)=nargs{let mut args=vec![];for _ in 0..n{args.push(stack.pop_front().ok_or(EvalFailure::Runtime("NoSuchElementException"))?);}args.reverse();if i==ptgs.len()-1&&matches!(ptg.kind,PtgKind::Add){N::RoundAddEval.evaluate(ptg.class as i8,&args,ec)?}else if i==ptgs.len()-1&&matches!(ptg.kind,PtgKind::Subtract){N::RoundSubtractEval.evaluate(ptg.class as i8,&args,ec)?}else{OperationEvaluatorFactory::evaluate(&ptg,&args,ec)?}}else{self.getEvalForOperandPtg(&ptg,ec)?};stack.push_front(value);i+=1;}
let value=stack.pop_front();if !stack.is_empty(){return Err(EvalFailure::Runtime("evaluation stack not empty"));}Ok(value)
}
// java private static int countTokensToBeSkipped(Ptg[] ptgs, int startIndex, int distInBytes) {
// java int remBytes = distInBytes;
// java int index = startIndex;
// java
// java while(remBytes != 0) {
// java ++index;
// java if (ptgs[index].getSid() == 32) {
// java remBytes -= 8;
// java } else if (ptgs[index].getSid() == 38) {
// java remBytes -= 7;
// java } else {
// java remBytes -= ptgs[index].getSize();
// java }
// java
// java if (remBytes < 0) {
// java throw new RuntimeException("Bad skip distance (wrong token size calculation).");
// java }
// java
// java if (index >= ptgs.length) {
// java throw new RuntimeException("Skip distance too far (ran out of formula tokens).");
// java }
// java }
// java
// java return index - startIndex;
// java }
pub fn countTokensToBeSkipped(ptgs:&[crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg],start:usize,dist:i32)->EvalResult<usize> {
let mut remaining=dist;let mut index=start;while remaining!=0{index+=1;let p=ptgs.get(index).ok_or(super::EvalFailure::Runtime("Skip distance too far"))?;remaining-=match p.getSid(){32=>8,38=>7,_=>p.getSize()};if remaining<0{return Err(super::EvalFailure::Runtime("Bad skip distance"));}}Ok(index-start)
}
// java private static Ptg[] cancelOptIf(Ptg[] oldptgs, int ifFuncBegin) {
// java AttrPtg attrPtg = (AttrPtg)oldptgs[ifFuncBegin];
// java int dist = attrPtg.getData();
// java int skip1Index = ifFuncBegin + countTokensToBeSkipped(oldptgs, ifFuncBegin, dist);
// java AttrPtg skip1 = (AttrPtg)oldptgs[skip1Index];
// java Ptg nextPtg = oldptgs[skip1Index + 1];
// java if (nextPtg.getSid() == 34 && ((FuncVarPtg)nextPtg).getFunctionIndex() == 1) {
// java Ptg[] ret = new Ptg[oldptgs.length - 2];
// java System.arraycopy(oldptgs, 0, ret, 0, ifFuncBegin);
// java System.arraycopy(oldptgs, ifFuncBegin + 1, ret, ifFuncBegin, skip1Index - ifFuncBegin - 1);
// java System.arraycopy(oldptgs, skip1Index + 1, ret, skip1Index - 1, oldptgs.length - skip1Index - 1);
// java return ret;
// java } else {
// java Ptg[] ret = new Ptg[oldptgs.length - 3];
// java int skip2Index = skip1Index + countTokensToBeSkipped(oldptgs, skip1Index, skip1.getData() + 1) - 1;
// java System.arraycopy(oldptgs, 0, ret, 0, ifFuncBegin);
// java System.arraycopy(oldptgs, ifFuncBegin + 1, ret, ifFuncBegin, skip1Index - ifFuncBegin - 1);
// java System.arraycopy(oldptgs, skip1Index + 1, ret, skip1Index - 1, skip2Index - skip1Index - 1);
// java System.arraycopy(oldptgs, skip2Index + 1, ret, skip2Index - 2, oldptgs.length - skip2Index - 1);
// java return ret;
// java }
// java }
pub fn cancelOptIf(old:&[crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg],begin:usize)->EvalResult<Vec<crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg>> {
use crate::cn::wps::et::ss::formula::ptg::Ptg::PtgKind;let PtgKind::Attr{data,..}=old[begin].kind else{return Err(super::EvalFailure::Runtime("ClassCastException AttrPtg"));};let skip=begin+Self::countTokensToBeSkipped(old,begin,data as i32)?;let PtgKind::Attr{data,..}=old[skip].kind else{return Err(super::EvalFailure::Runtime("ClassCastException AttrPtg"));};let next=&old[skip+1];let mut removed=vec![begin,skip];if !matches!(&next.kind,PtgKind::FuncVar{meta,..}if meta.getIndex()==1){let skip2=skip+Self::countTokensToBeSkipped(old,skip,data as i32+1)?-1;removed.push(skip2);}Ok(old.iter().enumerate().filter(|(i,_)|!removed.contains(i)).map(|(_,p)|p.clone()).collect())
}
// java private IValueEval getEvalForOperandPtg(Ptg ptg, EvaluationContext ec) {
// java switch (ptg.getSid()) {
// java case 1:
// java throw new RuntimeException("ExpPtg currently not supported");
// java case 2:
// java case 3:
// java case 4:
// java case 5:
// java case 6:
// java case 7:
// java case 8:
// java case 9:
// java case 10:
// java case 11:
// java case 12:
// java case 13:
// java case 14:
// java case 15:
// java case 16:
// java case 17:
// java case 18:
// java case 19:
// java case 20:
// java case 21:
// java case 24:
// java case 25:
// java case 26:
// java case 27:
// java case 33:
// java case 34:
// java case 38:
// java case 39:
// java case 40:
// java case 41:
// java case 44:
// java case 45:
// java case 46:
// java case 47:
// java case 48:
// java case 49:
// java case 50:
// java case 51:
// java case 52:
// java case 53:
// java case 54:
// java case 55:
// java case 56:
// java default:
// java if (ptg instanceof UnknownPtg) {
// java throw new RuntimeException("UnknownPtg not allowed");
// java }
// java
// java throw new RuntimeException("Unexpected ptg class (" + ptg.getClass().getName() + ")");
// java case 22:
// java return MissingArgEval.instance;
// java case 23:
// java return new StringEval(((StringPtg)ptg).getValue());
// java case 28:
// java return ErrorEval.valueOf(((ErrPtg)ptg).getErrorCode());
// java case 29:
// java return BoolEval.valueOf(((BoolPtg)ptg).getValue());
// java case 30:
// java return new NumberEval((double)((IntPtg)ptg).getValue());
// java case 31:
// java return new NumberEval(((NumberPtg)ptg).getValue());
// java case 32:
// java return new ArrayEval((ArrayPtg)ptg);
// java case 35:
// java NamePtg namePtg = (NamePtg)ptg;
// java IEvalName nameRecord = this._workbook.getNameByNamePtg(namePtg.getIndex());
// java if (nameRecord != null && nameRecord.isFunctionName()) {
// java return new UDFNameEval(nameRecord.getNameText());
// java } else {
// java if (nameRecord != null && nameRecord.hasFormula()) {
// java return this.evaluateNameFormula(nameRecord.getNameDefinition(), ptg.getPtgClass(), ec);
// java }
// java
// java return ErrorEval.NAME_INVALID;
// java }
// java case 36:
// java RefPtg rptg = (RefPtg)ptg;
// java return ec.getRefEval(rptg.getRow(), rptg.getColumn());
// java case 37:
// java AreaPtg aptg = (AreaPtg)ptg;
// java return ec.getAreaEval(aptg.getFirstRow(), aptg.getFirstColumn(), aptg.getLastRow(), aptg.getLastColumn(), aptg.getPtgClass());
// java case 42:
// java case 43:
// java case 60:
// java case 61:
// java return ErrorEval.REF_INVALID;
// java case 57:
// java NameXPtg nptg = (NameXPtg)ptg;
// java return ec.getNameXEval(nptg.getSheetRefIndex(), nptg.getNameIndex(), ptg.getPtgClass());
// java case 58: {
// java Ref3DPtg ref3d = (Ref3DPtg)ptg;
// java return ec.getRef3DEval(ref3d.getRow(), ref3d.getColumn(), ref3d.getExternSheetIndex());
// java }
// java case 59: {
// java Area3DPtg area3d = (Area3DPtg)ptg;
// java return ec.getArea3DEval(area3d.getFirstRow(), area3d.getFirstColumn(), area3d.getLastRow(), area3d.getLastColumn(), area3d.getExternSheetIndex(), area3d.getPtgClass());
// java }
// java }
// java }
pub fn getEvalForOperandPtg(&self,ptg:&crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg,ec:&mut EvaluationContext)->EvalResult<ValueRef> {
use crate::cn::wps::et::ss::formula::{ptg::Ptg::{PtgKind,col_index},eval::{MissingArgEval::MissingArgEval,StringEval::StringEval,ErrorEval::ErrorEval,BoolEval::BoolEval,NumberEval::NumberEval,advance::ArrayEval::ArrayEval}};use super::EvalFailure;Ok(match &ptg.kind{PtgKind::Exp{..}=>return Err(EvalFailure::Runtime("ExpPtg currently not supported")),PtgKind::MissingArg=>Arc::new(MissingArgEval::instance),PtgKind::Str(_)|PtgKind::StrRaw{..}=>Arc::new(StringEval::StringEval__ptg(ptg)),PtgKind::Err(v)=>Arc::new(ErrorEval::valueOf(*v as i32)),PtgKind::Bool(v)=>Arc::new(BoolEval::valueOf(*v)),PtgKind::Int(v)=>Arc::new(NumberEval::NumberEval(*v as f64)),PtgKind::Number(v)=>Arc::new(NumberEval::NumberEval(*v)),PtgKind::Array{..}=>Arc::new(ArrayEval::fromPtg(ptg)),PtgKind::Ref{row,col_raw,relative:false,..}=>ec.getRefEval(*row,col_index(*col_raw)),PtgKind::Area{first_row,last_row,first_col_raw,last_col_raw,relative:false,..}=>ec.getAreaEval(*first_row,col_index(*first_col_raw),*last_row,col_index(*last_col_raw),ptg.class as i8),PtgKind::RefError|PtgKind::AreaErr|PtgKind::DeletedRef3D{..}|PtgKind::DeletedArea3D{..}=>Arc::new(ErrorEval::REF_INVALID),PtgKind::Name{index}=>{if let Some(name)=self._workbook.getNameByNamePtg(*index){if name.isFunctionName(){return Ok(Arc::new(super::super::eval::UDFNameEval::UDFNameEval::new(name.getNameText().to_string())));}if name.hasFormula(){return self.evaluateNameFormula(&crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()),ptg.class as i8,ec)?.ok_or(EvalFailure::Runtime("Evaluation result must not be null"));}}Arc::new(ErrorEval::NAME_INVALID)},PtgKind::NameX{sheet_ref,name_number}=>return ec.getNameXEval(*sheet_ref,*name_number-1,ptg.class as i8),
// java case 58: Ref3DPtg ref3d = (Ref3DPtg)ptg; return ec.getRef3DEval(ref3d.getRow(), ref3d.getColumn(), ref3d.getExternSheetIndex());
PtgKind::Ref3D{row,col_raw,extern_index,..}=>return ec.getRef3DEval(*row,col_index(*col_raw),*extern_index),
// java case 59: Area3DPtg area3d = (Area3DPtg)ptg; return ec.getArea3DEval(area3d.getFirstRow(), area3d.getFirstColumn(), area3d.getLastRow(), area3d.getLastColumn(), area3d.getExternSheetIndex(), area3d.getPtgClass());
PtgKind::Area3D{first_row,last_row,first_col_raw,last_col_raw,extern_index,..}=>return ec.getArea3DEval(*first_row,col_index(*first_col_raw),*last_row,col_index(*last_col_raw),*extern_index,ptg.class as i8),_=>return Err(EvalFailure::Runtime("Unexpected ptg class"))})
}
// java public IValueEval evaluateNameFormula(Ptg[] ptgs, byte rav, EvaluationContext ec) {
// java Ptg[] namePtgs = FormulaHelper.replaceRelativeNamePtgs(ptgs, ec.getRowIndex(), ec.getColumnIndex(), this._workbook.getSpreadsheetVersion());
// java boolean nameIsArray = rav == 64;
// java boolean oldState = ec.isArrayFormula();
// java ec.setArrayFormula(nameIsArray);
// java
// java IValueEval var8;
// java try {
// java IValueEval ret = this.evaluateFormula(ec, namePtgs);
// java if (nameIsArray && ret instanceof IAreaEval) {
// java ((IAreaEval)ret).setRAV((byte)64);
// java }
// java
// java var8 = ret;
// java } finally {
// java ec.setArrayFormula(oldState);
// java }
// java
// java return var8;
// java }
pub fn evaluateNameFormula(&self,ptgs:&[crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg],rav:i8,ec:&mut EvaluationContext)->EvalResult<Option<ValueRef>> {
let ptgs=crate::cn::wps::et::ss::calcchain::FormulaHelper::FormulaHelper::replaceRelativeNamePtgs(ptgs,ec.getRowIndex(),ec.getColumnIndex(),self._workbook.getSpreadsheetVersion());let name_array=rav==64;let old=ec.isArrayFormula();ec.setArrayFormula(name_array);let ret=self.evaluateFormula(ec,&ptgs);if name_array{if let Ok(Some(ret))=&ret{if let Some(a)=ret.as_area_eval(){a.setRAV(64);}}}ec.setArrayFormula(old);ret
}
}
