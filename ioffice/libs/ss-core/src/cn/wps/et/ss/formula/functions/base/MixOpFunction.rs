use crate::cn::wps::et::ss::formula::{eval::{ValueRef,BlankEval::BlankEval,ErrorEval::ErrorEval,LazyAreaEval::LazyAreaEval,advance::{ArrayEval::ArrayEval,AreaArrayEval::AreaArrayEval}},evaluator::{EvalResult,EvaluationContext::EvaluationContext}};use std::sync::Arc;
// java public abstract class MixOpFunction implements IFunction
pub trait MixOpFunction:Send+Sync{fn valueArgRequired(&self,argPos:usize,args:&[ValueRef],ec:&EvaluationContext)->bool;fn evaluateOnce(&self,rav:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>;fn shouldMarkArray(&self)->bool{false}fn shouldClip(&self)->bool{true}}
impl<T:MixOpFunction> super::IFunction::IFunction for T{fn evaluate(&self,rav:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>{evaluate(self,rav,args,ec)}}
// java public final IValueEval evaluate(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java if (ec.getEvaluationWorkbook().isInterruptCalc()) {
// java return null;
// java } else {
// java boolean hasArray = false;
// java boolean simpleArgsCalc = true;
// java int args_len = args.length;
// java
// java for(int i = 0; i < args_len; ++i) {
// java IValueEval arg = args[i];
// java if (arg instanceof AreaArrayEval) {
// java hasArray = true;
// java simpleArgsCalc = false;
// java AreaArrayEval areaArrayEval = (AreaArrayEval)arg;
// java
// java for(int r = 0; r < areaArrayEval.getRowCount(); ++r) {
// java for(int c = 0; c < areaArrayEval.getColCount(); ++c) {
// java IAreaEval area = null;
// java
// java try {
// java area = areaArrayEval.getArea(r, c);
// java this.commonClips(area, ec);
// java } catch (EvaluationException e) {
// java e.printStackTrace();
// java }
// java }
// java }
// java } else {
// java if (arg instanceof IAreaEval) {
// java this.commonClips((IAreaEval)arg, ec);
// java }
// java
// java if (this.valueArgRequired(i, args, ec)) {
// java if (arg instanceof ArrayEval) {
// java hasArray = true;
// java simpleArgsCalc = false;
// java } else if (arg instanceof IAreaEval) {
// java if (ec.isArrayFormula() || ((IAreaEval)arg).getRAV() == 64 || this.shouldMarkArray()) {
// java hasArray = true;
// java }
// java
// java simpleArgsCalc = false;
// java }
// java }
// java }
// java }
// java
// java if (simpleArgsCalc) {
// java return this.evaluateOnce(funcRAV, args, ec);
// java } else {
// java int arrayRowCount = 1;
// java int arrayColCount = 1;
// java int orgArrayRowCount = -1;
// java int orgArrayColCount = -1;
// java
// java for(int i = 0; i < args_len; ++i) {
// java IValueEval arg = args[i];
// java if (arg instanceof AreaArrayEval) {
// java AreaArrayEval ae = (AreaArrayEval)arg;
// java if (ae.getRowCount() > arrayRowCount) {
// java arrayRowCount = ae.getRowCount();
// java }
// java
// java if (ae.getColCount() > arrayColCount) {
// java arrayColCount = ae.getColCount();
// java }
// java } else if (this.valueArgRequired(i, args, ec)) {
// java if (arg instanceof ArrayEval) {
// java ArrayEval ae = (ArrayEval)arg;
// java if (ae.getRowCount() > arrayRowCount) {
// java arrayRowCount = ae.getRowCount();
// java if (orgArrayRowCount == -1) {
// java orgArrayRowCount = Math.max(ae.getOrgRowCount(), ae.getRowCount());
// java } else if (ae.getOrgRowCount() > orgArrayRowCount) {
// java orgArrayRowCount = ae.getOrgRowCount();
// java }
// java }
// java
// java if (ae.getColCount() > arrayColCount) {
// java arrayColCount = ae.getColCount();
// java if (ae.getOrgColCount() > orgArrayColCount) {
// java orgArrayColCount = ae.getOrgColCount();
// java }
// java }
// java } else if (arg instanceof IAreaEval) {
// java IAreaEval ae = (IAreaEval)arg;
// java if (hasArray) {
// java if (ae.getHeight() > arrayRowCount) {
// java orgArrayRowCount = ae.getOrgHeight();
// java arrayRowCount = ae.getHeight();
// java }
// java
// java if (ae.getWidth() > arrayColCount) {
// java orgArrayColCount = ae.getOrgWidth();
// java arrayColCount = ae.getWidth();
// java }
// java }
// java }
// java }
// java }
// java
// java if (ec.isEndOfCalc()) {
// java int resultRowCount = ec.getLastRowIndex() - ec.getRowIndex() + 1;
// java int resultColCount = ec.getLastColIndex() - ec.getColumnIndex() + 1;
// java arrayRowCount = resultRowCount < arrayRowCount ? resultRowCount : resultRowCount;
// java arrayColCount = resultColCount < arrayColCount ? resultColCount : arrayColCount;
// java }
// java
// java IValueEval[] result = new IValueEval[arrayRowCount * arrayColCount];
// java IValueEval[] simpleArgs = new IValueEval[args_len];
// java
// java for(int i = 0; i < arrayRowCount && !ec.getEvaluationWorkbook().isInterruptCalc(); ++i) {
// java for(int j = 0; j < arrayColCount && !ec.getEvaluationWorkbook().isInterruptCalc(); ++j) {
// java for(int k = 0; k < args_len && !ec.getEvaluationWorkbook().isInterruptCalc(); ++k) {
// java IValueEval arg = args[k];
// java simpleArgs[k] = arg;
// java if (arg instanceof AreaArrayEval) {
// java AreaArrayEval aae = (AreaArrayEval)arg;
// java if (this.valueArgRequired(k, args, ec)) {
// java simpleArgs[k] = ErrorEval.VALUE_INVALID;
// java } else if (i < aae.getRowCount() && j < aae.getColCount()) {
// java try {
// java simpleArgs[k] = aae.getArea(i, j);
// java } catch (EvaluationException e) {
// java simpleArgs[k] = e.getErrorEval();
// java }
// java } else {
// java simpleArgs[k] = ErrorEval.NA;
// java }
// java } else if (this.valueArgRequired(k, args, ec)) {
// java if (arg instanceof ArrayEval) {
// java ArrayEval ae = (ArrayEval)arg;
// java if (i < ae.getRowCount() && j < ae.getColCount()) {
// java simpleArgs[k] = ae.getValue(i, j);
// java } else if (ae.getRowCount() == 1 && j < ae.getColCount()) {
// java simpleArgs[k] = ae.getValue(0, j);
// java } else if (i < ae.getRowCount() && ae.getColCount() == 1) {
// java simpleArgs[k] = ae.getValue(i, 0);
// java } else if (ae.getRowCount() == 1 && ae.getColCount() == 1) {
// java simpleArgs[k] = ae.getValue(0, 0);
// java } else if (i < ae.getOrgRowCount() && j < ae.getOrgColCount()) {
// java simpleArgs[k] = BlankEval.instance;
// java } else if (ae.getOrgRowCount() <= 1 && j < ae.getOrgColCount()) {
// java simpleArgs[k] = BlankEval.instance;
// java } else if (i < ae.getOrgRowCount() && ae.getOrgColCount() <= 1) {
// java simpleArgs[k] = BlankEval.instance;
// java } else if (ae.getOrgRowCount() == 1 && ae.getOrgColCount() == 1) {
// java simpleArgs[k] = BlankEval.instance;
// java } else {
// java simpleArgs[k] = ErrorEval.NA;
// java }
// java } else if (arg instanceof IAreaEval) {
// java IAreaEval ae = (IAreaEval)arg;
// java if (!ec.isArrayFormula() && ae.getRAV() != 64 && !hasArray) {
// java if (ae.getHeight() == 1 && ae.getWidth() == 1) {
// java simpleArgs[k] = ae.getRelativeValue(0, 0);
// java } else if (ae.getHeight() == 1 && ae.containsColumn(ec.getColumnIndex())) {
// java simpleArgs[k] = ae.getAbsoluteValue(ae.getFirstRow(), ec.getColumnIndex());
// java } else if (ae.containsRow(ec.getRowIndex()) && ae.getWidth() == 1) {
// java simpleArgs[k] = ae.getAbsoluteValue(ec.getRowIndex(), ae.getFirstColumn());
// java } else if (ae.getOrgHeight() == 1 && ae.getOrgWidth() == 1) {
// java simpleArgs[k] = ae.getRelativeValue(0, 0);
// java } else if (ae.getOrgHeight() <= 1 && ae.getFirstColumn() <= ec.getColumnIndex() && ae.getOrgLastCol() >= ec.getColumnIndex()) {
// java simpleArgs[k] = ae.getAbsoluteValue(ae.getFirstRow(), ec.getColumnIndex());
// java } else if (ae.getFirstRow() <= ec.getRowIndex() && ae.getOrgLastRow() >= ec.getColumnIndex() && ae.getOrgWidth() == 1) {
// java simpleArgs[k] = ae.getAbsoluteValue(ec.getRowIndex(), ae.getFirstColumn());
// java } else {
// java simpleArgs[k] = ErrorEval.VALUE_INVALID;
// java }
// java } else if (i < ae.getHeight() && j < ae.getWidth()) {
// java simpleArgs[k] = ae.getRelativeValue(i, j);
// java } else if (ae.getHeight() == 1 && j < ae.getWidth()) {
// java simpleArgs[k] = ae.getRelativeValue(0, j);
// java } else if (i < ae.getHeight() && ae.getWidth() == 1) {
// java simpleArgs[k] = ae.getRelativeValue(i, 0);
// java } else if (ae.getHeight() == 1 && ae.getWidth() == 1) {
// java simpleArgs[k] = ae.getRelativeValue(0, 0);
// java } else if (i < ae.getOrgHeight() && j < ae.getOrgWidth()) {
// java simpleArgs[k] = ae.getRelativeValue(i, j);
// java } else if (ae.getOrgHeight() == 1 && j < ae.getOrgWidth()) {
// java simpleArgs[k] = ae.getRelativeValue(0, j);
// java } else if (i < ae.getOrgHeight() && ae.getOrgWidth() == 1) {
// java simpleArgs[k] = ae.getRelativeValue(i, 0);
// java } else if (ae.getOrgHeight() == 1 && ae.getOrgWidth() == 1) {
// java simpleArgs[k] = ae.getRelativeValue(0, 0);
// java } else {
// java simpleArgs[k] = ErrorEval.NA;
// java }
// java }
// java }
// java }
// java
// java result[i * arrayColCount + j] = this.evaluateOnce(funcRAV, simpleArgs, ec);
// java }
// java }
// java
// java if (hasArray) {
// java if (result.length > 1 && result[0] instanceof IAreaEval) {
// java return new AreaArrayEval(arrayRowCount, arrayColCount, result);
// java } else {
// java for(int i = 0; i < result.length; ++i) {
// java if (result[i] instanceof ArrayEval) {
// java result[i] = ((ArrayEval)result[i]).getFirstValue();
// java }
// java }
// java
// java ArrayEval arrayEval = new ArrayEval(arrayRowCount, arrayColCount, orgArrayRowCount, orgArrayColCount, result);
// java return arrayEval;
// java }
// java } else {
// java return result[0];
// java }
// java }
// java }
// java }
pub fn evaluate(f:&dyn MixOpFunction,rav:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>{
if ec.getEvaluationWorkbook().isInterruptCalc(){return Err(crate::cn::wps::et::ss::formula::evaluator::EvalFailure::Runtime("null interrupted evaluation"));}
let(mut has_array,mut simple)=(false,true);
for(i,a)in args.iter().enumerate(){if let Some(aa)=a.as_any().downcast_ref::<AreaArrayEval>(){has_array=true;simple=false;for r in 0..aa.getRowCount(){for c in 0..aa.getColCount(){if let Ok(a)=aa.getArea(r,c){clips(f,&a,ec);}}}}else{if a.as_area_eval().is_some(){clips(f,a,ec);}if f.valueArgRequired(i,args,ec){if a.as_any().is::<ArrayEval>(){has_array=true;simple=false;}else if let Some(a)=a.as_area_eval(){if ec.isArrayFormula()||a.getRAV()==64||f.shouldMarkArray(){has_array=true;}simple=false;}}}}
if simple{return f.evaluateOnce(rav,args,ec);}
let(mut rows,mut cols,mut org_rows,mut org_cols)=(1,1,-1,-1);
for(i,a)in args.iter().enumerate(){if let Some(aa)=a.as_any().downcast_ref::<AreaArrayEval>(){rows=rows.max(aa.getRowCount());cols=cols.max(aa.getColCount());}else if f.valueArgRequired(i,args,ec){if let Some(aa)=a.as_any().downcast_ref::<ArrayEval>(){if aa.getRowCount()>rows{rows=aa.getRowCount();if org_rows== -1{org_rows=aa.getOrgRowCount().max(aa.getRowCount());}else if aa.getOrgRowCount()>org_rows{org_rows=aa.getOrgRowCount();}}if aa.getColCount()>cols{cols=aa.getColCount();if aa.getOrgColCount()>org_cols{org_cols=aa.getOrgColCount();}}}else if let Some(a)=a.as_area_eval(){if has_array{if a.getHeight()>rows{org_rows=a.base().orgLastRow.get()-a.getFirstRow()+1;rows=a.getHeight();}if a.getWidth()>cols{org_cols=a.base().orgLastColumn.get()-a.getFirstColumn()+1;cols=a.getWidth();}}}}}
if ec.isEndOfCalc(){let rr=ec.getLastRowIndex()-ec.getRowIndex()+1;let cc=ec.getLastColIndex()-ec.getColumnIndex()+1;rows=rr;cols=cc.min(cols);}
let mut result=vec![];
for r in 0..rows{for c in 0..cols{let mut simple_args=vec![];for(k,arg)in args.iter().enumerate(){let mut value=arg.clone();if let Some(aa)=arg.as_any().downcast_ref::<AreaArrayEval>(){value=if f.valueArgRequired(k,args,ec){Arc::new(ErrorEval::VALUE_INVALID)}else if r<aa.getRowCount()&&c<aa.getColCount(){aa.getArea(r,c).unwrap_or_else(|e|Arc::new(e))}else{Arc::new(ErrorEval::NA)};}else if f.valueArgRequired(k,args,ec){if let Some(a)=arg.as_any().downcast_ref::<ArrayEval>(){value=if r<a.getRowCount()&&c<a.getColCount(){a.getValue(r,c)}else if a.getRowCount()==1&&c<a.getColCount(){a.getValue(0,c)}else if r<a.getRowCount()&&a.getColCount()==1{a.getValue(r,0)}else if a.getRowCount()==1&&a.getColCount()==1{a.getValue(0,0)}else if(r<a.getOrgRowCount()&&c<a.getOrgColCount())||(a.getOrgRowCount()<=1&&c<a.getOrgColCount())||(r<a.getOrgRowCount()&&a.getOrgColCount()<=1)||(a.getOrgRowCount()==1&&a.getOrgColCount()==1){Arc::new(BlankEval::instance)}else{Arc::new(ErrorEval::NA)};}else if let Some(a)=arg.as_area_eval(){let or=a.base().orgLastRow.get()-a.getFirstRow()+1;let oc=a.base().orgLastColumn.get()-a.getFirstColumn()+1;
value=if !ec.isArrayFormula()&&a.getRAV()!=64&&!has_array{let row=ec.getRowIndex();let col=ec.getColumnIndex();if a.getHeight()==1&&a.getWidth()==1{a.getRelativeValue(0,0)?}else if a.getHeight()==1&&a.containsColumn(col){a.getAbsoluteValue(a.getFirstRow(),col)?}else if a.containsRow(row)&&a.getWidth()==1{a.getAbsoluteValue(row,a.getFirstColumn())?}else if or==1&&oc==1{a.getRelativeValue(0,0)?}else if or<=1&&a.getFirstColumn()<=col&&a.base().orgLastColumn.get()>=col{a.getAbsoluteValue(a.getFirstRow(),col)?}else if a.getFirstRow()<=row&&a.base().orgLastRow.get()>=col&&oc==1{a.getAbsoluteValue(row,a.getFirstColumn())?}else{Arc::new(ErrorEval::VALUE_INVALID)}}else if r<a.getHeight()&&c<a.getWidth(){a.getRelativeValue(r,c)?}else if a.getHeight()==1&&c<a.getWidth(){a.getRelativeValue(0,c)?}else if r<a.getHeight()&&a.getWidth()==1{a.getRelativeValue(r,0)?}else if a.getHeight()==1&&a.getWidth()==1{a.getRelativeValue(0,0)?}else if r<or&&c<oc{a.getRelativeValue(r,c)?}else if or==1&&c<oc{a.getRelativeValue(0,c)?}else if r<or&&oc==1{a.getRelativeValue(r,0)?}else if or==1&&oc==1{a.getRelativeValue(0,0)?}else{Arc::new(ErrorEval::NA)};}}
simple_args.push(value);}result.push(f.evaluateOnce(rav,&simple_args,ec)?);}}
if has_array{if result.len()>1&&result[0].as_area_eval().is_some(){Ok(Arc::new(AreaArrayEval::new(rows,cols,Arc::new(result))))}else{for v in &mut result{if let Some(a)=v.as_any().downcast_ref::<ArrayEval>(){*v=a.getFirstValue();}}Ok(Arc::new(ArrayEval::withOriginal(rows,cols,org_rows,org_cols,Arc::new(result))))}}else{Ok(result[0].clone())}}
// java private void commonClips(IAreaEval arg, EvaluationContext ec) {
// java if (this.shouldClip()) {
// java this.valueArgClips(arg, ec);
// java } else {
// java arg.setOrgLastRow(arg.getLastRow());
// java arg.setOrgLastCol(arg.getLastColumn());
// java }
// java
// java }
fn clips(f:&dyn MixOpFunction,a:&ValueRef,ec:&EvaluationContext){let Some(a)=a.as_area_eval()else{return;};if f.shouldClip(){let wb=if let Some(lazy)=a.as_any().downcast_ref::<LazyAreaEval>(){lazy.sheetRefEvaluator().getBook()}else{ec.getEvaluationWorkbook().clone()};if let Some(range)=wb.getUseRange(a.getSheetIndex()){let v=ec.getEvaluationWorkbook().getSpreadsheetVersion();a.clip(a.getFirstRow(),a.getFirstColumn(),(range.getLastRow()+1).min(v.getMaxRows()),(range.getLastColumn()+1).min(v.getMaxColumns()));}}else{a.base().orgLastRow.set(a.getLastRow());a.base().orgLastColumn.set(a.getLastColumn());}}
