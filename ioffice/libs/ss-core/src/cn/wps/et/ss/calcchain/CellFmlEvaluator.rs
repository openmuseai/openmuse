// java package cn.wps.et.ss.calcchain;
use super::{RecalcLock, ThreadCalcStack::ThreadCalcStack};
use crate::cn::wps::et::ss::{
    formula::{
        eval::{
            BoolEval::BoolEval, ErrorEval::ErrorEval, NumberEval::NumberEval,
            StringEval::StringEval, ValueRef,
        },
        evaluator::{
            EvalFailure, EvalResult, EvaluationContext::EvaluationContext,
            WorkbookBytesEvaluator::WorkbookBytesEvaluator, WorkbookEvaluator::WorkbookEvaluator,
        },
    },
    interfaces::IEvalFormulaBase::IEvalFormulaBase,
    interfaces::IEvalWorkbook::IEvalWorkbook,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
pub type ValuePos = Mutex<HashMap<i32, crate::java_runtime::ConcurrentLongMap::ConcurrentLongMap<ValueRef>>>;
// java public final class CellFmlEvaluator
pub struct CellFmlEvaluator {
    pub workbookEvaluator: Arc<WorkbookEvaluator>,
    pub workbookBytesEvaluator: WorkbookBytesEvaluator,
    workbook: Arc<dyn IEvalWorkbook>,
    pub threadCalcStack: ThreadCalcStack,
    // java HashMap<Integer, ConcurrentHashMap<Long, IValueEval>> valuePos
    pub valuePos: ValuePos,
    // rust: object monitors are independent of IEvalFormulaBase.lock().
    monitors: Mutex<HashMap<usize, Arc<RecalcLock>>>,
}
impl CellFmlEvaluator {
    // java this.workbookEvaluator = new WorkbookEvaluator(eb); this.workbookBytesEvaluator = new WorkbookBytesEvaluator(this.workbookEvaluator); this.workbook = eb;
    pub fn CellFmlEvaluator(eb: Arc<dyn IEvalWorkbook>) -> Self {
        let we = Arc::new(WorkbookEvaluator::WorkbookEvaluator(eb.clone()));
        Self {
            workbookEvaluator: we.clone(),
            workbookBytesEvaluator: WorkbookBytesEvaluator::WorkbookBytesEvaluator(we),
            workbook: eb,
            threadCalcStack: ThreadCalcStack::new(),
            valuePos: Mutex::new(HashMap::new()),
            monitors: Mutex::new(HashMap::new()),
        }
    }
    // java public boolean evaluateFormula(IEvalFormulaBase fml)
    pub fn evaluateFormula(&self, fml: &Arc<dyn IEvalFormulaBase>) -> EvalResult<bool> {
        self.threadCalcStack.addRef(fml);
        if self.threadCalcStack.checkCycl(fml) {
            return Ok(false);
        }
        // java synchronized(fml)
        let id = Arc::as_ptr(fml) as *const () as usize;
        let monitor = self
            .monitors
            .lock()
            .unwrap()
            .entry(id)
            .or_insert_with(|| Arc::new(RecalcLock::new()))
            .clone();
        let _guard = monitor.lock();
        if !fml.isNeedRecalculationFlagSet() {
            return Ok(false);
        }
        self.threadCalcStack.add(fml);
        // java exceptions escape BEFORE pop(); preserve that ordering.
        if fml.as_cell_formula().is_some() {
            self.evaluateCellFormula(fml)?;
        } else {
            self.evaluateArrayFormula(fml)?;
        }
        if fml.getRef().is_some_and(|r| r.isCycled()) {
            self.threadCalcStack.cyclFml(fml);
        }
        self.threadCalcStack.pop();
        Ok(true)
    }
    // java public void evaluateCellFormula(IEvalCellFormula cellfml)
    pub fn evaluateCellFormula(&self, fml: &Arc<dyn IEvalFormulaBase>) -> EvalResult<()> {
        let cell = fml
            .as_cell_formula()
            .ok_or(EvalFailure::Runtime("IEvalCellFormula expected"))?;
        let mut ec = EvaluationContext::EvaluationContext(
            self.workbookEvaluator.clone(),
            self.workbook.clone(),
            fml.getSheetIndex(),
            fml.getFmlRow(),
            fml.getFmlCol(),
            cell.isSharedFormula(),
        );
        let bytes = fml
            .getEncodedTokens()
            .ok_or(EvalFailure::Runtime("NullPointerException"))?;
        let mut stack = self.workbookBytesEvaluator.evaluateFormula(
            &mut ec,
            bytes,
            fml.getEncodedTokensLen(),
        )?;
        let mut v = stack
            .pop_front()
            .ok_or(EvalFailure::Runtime("NoSuchElementException"))?;
        if !stack.is_empty() {
            return Err(EvalFailure::Runtime("evaluation stack not empty"));
        }
        if !ec.getEvaluationWorkbook().isInterruptCalc() {
            v = WorkbookEvaluator::dereferenceResult(v, &ec)?;
            // The current formula-string store is UTF-8. Keep this explicit
            // boundary dirty until its Java String storage is ported.
            if let Some(s) = v.as_any().downcast_ref::<StringEval>() {
                if String::from_utf16(s.getStringValue()).is_err() {
                    return Err(EvalFailure::Unported("UTF-16 formula result store"));
                }
            }
            if !self.workbook.isFullPrecision() {
                v = self.workbook.getPrecisionValue(
                    v,
                    fml.getSheetIndex(),
                    fml.getFmlRow(),
                    fml.getFmlCol(),
                )?;
            }
            let key = crate::cn::wps::et::ss::util::RowColKeyHelper::RowColKeyHelper::makeLongKey(
                fml.getFmlRow(),
                fml.getFmlCol(),
            );
            self.valuePos
                .lock()
                .unwrap()
                .get_mut(&fml.getSheetUid())
                .ok_or(EvalFailure::Runtime("NullPointerException valuePos"))?
                .insert(key, v);
            fml.setNeedRecalculationFlag(ec.getEvaluationWorkbook().isInterruptCalc());
        }
        Ok(())
    }
    // java public void evaluateArrayFormula(IEvalArrayFormula arrayfml)
    pub fn evaluateArrayFormula(&self, fml: &Arc<dyn IEvalFormulaBase>) -> EvalResult<()> {
        let array = fml
            .as_array_formula()
            .ok_or(EvalFailure::Runtime("ClassCastException IEvalArrayFormula"))?;
        let mut ec = EvaluationContext::EvaluationContext__array(
            self.workbookEvaluator.clone(),
            self.workbook.clone(),
            fml.getSheetIndex(),
            fml.getFmlRow(),
            fml.getFmlCol(),
            array.getFmlLastRow(),
            array.getFmlLastCol(),
        );
        let bytes = fml
            .getEncodedTokens()
            .ok_or(EvalFailure::Runtime("NullPointerException tokens"))?;
        let mut stack = self.workbookBytesEvaluator.evaluateFormula(
            &mut ec,
            bytes,
            fml.getEncodedTokensLen(),
        )?;
        let v = stack
            .pop_front()
            .ok_or(EvalFailure::Runtime("NoSuchElementException"))?;
        if !stack.is_empty() {
            return Err(EvalFailure::Runtime("evaluation stack not empty"));
        }
        if !ec.getEvaluationWorkbook().isInterruptCalc() {
            array.setDirtyValue(Some(v), &self.valuePos, fml.getSheetUid(), &ec)?;
            fml.setNeedRecalculationFlag(ec.getEvaluationWorkbook().isInterruptCalc());
        }
        Ok(())
    }
    // java public void setFormulaResult(IEvalCellFormula fml, IValueEval ve)
    pub fn setFormulaResult(
        &self,
        fml: &Arc<dyn IEvalFormulaBase>,
        v: &ValueRef,
    ) -> EvalResult<()> {
        // java the base overload dispatches to distinct cell/array overloads.
        if let Some(array) = fml.as_array_formula() {
            return self.setFormulaResult__array(array, v);
        }
        if fml.isCycled() || fml.getRef().is_some_and(|r| r.isCycled()) {
            fml.setZeroResultIfBlank();
            return Ok(());
        }
        let f = fml
            .as_cell_formula()
            .ok_or(EvalFailure::Runtime("ClassCastException IEvalCellFormula"))?;
        if let Some(n) = v.as_any().downcast_ref::<NumberEval>() {
            f.setNumberResult(n.getNumberValue());
        } else if let Some(b) = v.as_any().downcast_ref::<BoolEval>() {
            f.setBoolResult(b.getBooleanValue());
        } else if let Some(s) = v.as_any().downcast_ref::<StringEval>() {
            let s = String::from_utf16(s.getStringValue())
                .map_err(|_| EvalFailure::Unported("UTF-16 formula result store"))?;
            f.setStringResult(&s);
        } else if let Some(e) = v.as_any().downcast_ref::<ErrorEval>() {
            f.setErrorResult(e.getErrorCode() as i8);
        } else {
            return Err(EvalFailure::Runtime("Unexpected eval class"));
        }
        Ok(())
    }
    // java public void setFormulaResult(IEvalArrayFormula fml,IValueEval ve)
    pub fn setFormulaResult__array(
        &self,
        fml: &dyn crate::cn::wps::et::ss::interfaces::IEvalArrayFormula::IEvalArrayFormula,
        value: &ValueRef,
    ) -> EvalResult<()> {
        if fml.isCycled() {
            fml.setZeroResultIfBlank();
            return Ok(());
        }
        for r in 0..=fml.getFmlLastRow() - fml.getFmlRow() {
            for c in 0..=fml.getFmlLastCol() - fml.getFmlCol() {
                let mut current = value.clone();
                // java IRefEval dereferences per member; the scalar branch reuses ve.
                if let Some(reference) = current.as_ref_eval() {
                    current = reference
                        .getInnerValueEval()?
                        .ok_or(EvalFailure::Runtime("NullPointerException array value"))?;
                }
                if current.as_any().is::<crate::cn::wps::et::ss::formula::eval::BlankEval::BlankEval>()||current.as_any().is::<crate::cn::wps::et::ss::formula::eval::MissingArgEval::MissingArgEval>(){current=Arc::new(NumberEval::ZERO);}
                let row = r + fml.getFmlRow();
                let col = c + fml.getFmlCol();
                if let Some(n) = current.as_any().downcast_ref::<NumberEval>() {
                    fml.setNumberResult(row, col, n.getNumberValue());
                } else if let Some(b) = current.as_any().downcast_ref::<BoolEval>() {
                    fml.setBoolResult(row, col, b.getBooleanValue());
                } else if let Some(s) = current.as_any().downcast_ref::<StringEval>() {
                    let s = String::from_utf16(s.getStringValue())
                        .map_err(|_| EvalFailure::Unported("UTF-16 formula result store"))?;
                    fml.setStringResult(row, col, &s);
                } else if let Some(e) = current.as_any().downcast_ref::<ErrorEval>() {
                    fml.setErrorResult(row, col, e.getErrorCode() as i8);
                } else {
                    return Err(EvalFailure::Runtime("Unexpected eval class"));
                }
            }
        }
        Ok(())
    }
    // java public void clean() { this.threadCalcStack.reSet(); }
    pub fn clean(&self) {
        self.threadCalcStack.reSet();
    }
}
