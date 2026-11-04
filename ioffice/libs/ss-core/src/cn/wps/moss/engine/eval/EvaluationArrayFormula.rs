// java package cn.wps.moss.engine.eval;
use super::EvaluationPresent::EvaluationPresent;
use crate::cn::wps::et::ss::{
    calcchain::CellFmlEvaluator::ValuePos,
    formula::{
        eval::{
            BlankEval::BlankEval, MissingArgEval::MissingArgEval, NumberEval::NumberEval, ValueRef,
        },
        evaluator::{EvalFailure, EvalResult, EvaluationContext::EvaluationContext},
        ptg::Ptg::Ptg,
    },
    interfaces::{IEvalArrayFormula::IEvalArrayFormula, IEvalFormulaBase::IEvalFormulaBase},
    util::RowColKeyHelper::RowColKeyHelper,
};
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::java_runtime::{Cell, Rc, RefCell};
// java public final class EvaluationArrayFormula implements IEvalArrayFormula
pub struct EvaluationArrayFormula {
    ksheet: Rc<RefCell<KmoSheet>>,
    rowIndex: i32,
    colIndex: i32,
    lastRowIndex: i32,
    lastColIndex: i32,
    encodedTokens: Option<Vec<u8>>,
    encodedTokenLen: i32,
    evaluationPresent: EvaluationPresent,
    nodeBuff: [u8; 7],
    isCycled: Cell<bool>,
    refFml: RefCell<Option<Rc<dyn IEvalFormulaBase>>>,
}
impl EvaluationArrayFormula {
    // java query(rowIndex,colIndex,false); null uses the master rectangle and null tokens.
    pub fn EvaluationArrayFormula(sheet: Rc<RefCell<KmoSheet>>, row: i32, col: i32) -> Self {
        let f = sheet
            .borrow()
            .getExpFmlManager()
            .borrow()
            .query(row, col, false);
        let (lr, lc, bytes, len) = if let Some(f) = f {
            (
                f.lastRow,
                f.lastCol,
                Some(f.getEncodedTokens()),
                f.getEncodedTokensLen(),
            )
        } else {
            (row, col, None, 0)
        };
        Self {
            ksheet: sheet,
            rowIndex: row,
            colIndex: col,
            lastRowIndex: lr,
            lastColIndex: lc,
            encodedTokens: bytes,
            encodedTokenLen: len,
            evaluationPresent: EvaluationPresent::new(),
            nodeBuff: [0; 7],
            isCycled: Cell::new(false),
            refFml: RefCell::new(None),
        }
    }
    // java getDirtyValue scalar/ref branches; arrays/areas have no concrete value class in this compilation scope yet.
    pub fn getDirtyValue(
        &self,
        _row: i32,
        _col: i32,
        value: Option<ValueRef>,
    ) -> EvalResult<Option<ValueRef>> {
        let Some(mut v) = value else {
            return Ok(None);
        };
        if self.isCycled() {
            return Ok(Some(Rc::new(NumberEval::ZERO)));
        }
        if let Some(r) = v.as_ref_eval() {
            v = r
                .getInnerValueEval()?
                .ok_or(EvalFailure::Runtime("NullPointerException dirty value"))?;
        }
        if v.as_any().is::<BlankEval>() || v.as_any().is::<MissingArgEval>() {
            v = Rc::new(NumberEval::ZERO);
        }
        Ok(Some(v))
    }
}
impl IEvalFormulaBase for EvaluationArrayFormula {
    fn getSheetUid(&self) -> i32 {
        self.ksheet.borrow().getSheetUid()
    }
    fn getSheetIndex(&self) -> i32 {
        self.ksheet.borrow().getSheetIndex()
    }
    fn getFmlRow(&self) -> i32 {
        self.rowIndex
    }
    fn getFmlCol(&self) -> i32 {
        self.colIndex
    }
    fn getTokens(&self) -> Vec<Ptg> {
        self.encodedTokens.as_ref().map_or_else(Vec::new, |b| {
            Ptg::readTokens__stream(
                self.encodedTokenLen,
                b,
                false,
                self.ksheet.borrow().getVersion(),
            )
        })
    }
    fn getEncodedTokens(&self) -> Option<&[u8]> {
        self.encodedTokens.as_deref()
    }
    fn getEncodedTokensLen(&self) -> i32 {
        self.encodedTokenLen
    }
    fn setNeedRecalculationFlag(&self, f: bool) {
        EvaluationPresent::setNeedRecalculationFlag(
            &self.ksheet.borrow(),
            self.rowIndex,
            self.colIndex,
            f,
        );
    }
    fn isNeedRecalculationFlagSet(&self) -> bool {
        EvaluationPresent::isNeedRecalculationFlagSet(
            &self.ksheet.borrow(),
            self.rowIndex,
            self.colIndex,
        )
    }
    // java for(r=rowIndex;r<lastRowIndex;++r) for(c=colIndex;c<lastColIndex;++c) setZeroResultIfBlank(...);
    fn setZeroResultIfBlank(&self) {
        let sheet = self.ksheet.borrow();
        for r in self.rowIndex..self.lastRowIndex {
            for c in self.colIndex..self.lastColIndex {
                EvaluationPresent::setZeroResultIfBlank(&sheet, r, c);
            }
        }
    }
    fn equals(&self, other: &dyn IEvalFormulaBase) -> bool {
        other.as_any().downcast_ref::<Self>().is_some_and(|f| {
            Rc::ptr_eq(&self.ksheet, &f.ksheet)
                && self.rowIndex == f.rowIndex
                && self.colIndex == f.colIndex
        })
    }
    fn hashCode(&self) -> i32 {
        (self.getSheetUid() << 8) ^ (self.rowIndex << 16) ^ self.colIndex
    }
    fn lock(&self) {
        self.evaluationPresent.lock();
    }
    fn islock(&self) -> bool {
        self.evaluationPresent.isLock()
    }
    fn unlock(&self) {
        self.evaluationPresent.unlock();
    }
    fn isCycled(&self) -> bool {
        self.isCycled.get()
    }
    fn setCycled(&self, f: bool) {
        self.isCycled.set(f);
    }
    fn setRef(&self, f: Option<Rc<dyn IEvalFormulaBase>>) {
        *self.refFml.borrow_mut() = f;
    }
    fn getRef(&self) -> Option<Rc<dyn IEvalFormulaBase>> {
        self.refFml.borrow().clone()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_array_formula(&self) -> Option<&dyn IEvalArrayFormula> {
        Some(self)
    }
}
impl IEvalArrayFormula for EvaluationArrayFormula {
    fn getFmlLastRow(&self) -> i32 {
        self.lastRowIndex
    }
    fn getFmlLastCol(&self) -> i32 {
        self.lastColIndex
    }
    fn setStringResult(&self, r: i32, c: i32, v: &str) {
        EvaluationPresent::setStringResult(&self.ksheet.borrow(), r, c, v);
    }
    fn setNumberResult(&self, r: i32, c: i32, v: f64) {
        EvaluationPresent::setNumberResult(&self.ksheet.borrow(), r, c, v);
    }
    fn setErrorResult(&self, r: i32, c: i32, v: i8) {
        EvaluationPresent::setErrorResult(&self.ksheet.borrow(), r, c, v);
    }
    fn setBoolResult(&self, r: i32, c: i32, v: bool) {
        EvaluationPresent::setBoolResult(&self.ksheet.borrow(), r, c, v);
    }
    fn setDirtyValue(
        &self,
        value: Option<ValueRef>,
        positions: &ValuePos,
        _uid: i32,
        _ec: &EvaluationContext,
    ) -> EvalResult<()> {
        let Some(v) = value else {
            return Ok(());
        };
        if self.isCycled() {
            self.setZeroResultIfBlank();
            return Ok(());
        }
        // java scalar broadcast over the inclusive formula rectangle; refs are dereferenced separately for each member.
        for r in 0..=self.lastRowIndex - self.rowIndex {
            for c in 0..=self.lastColIndex - self.colIndex {
                let mut current = v.clone();
                if let Some(reference) = current.as_ref_eval() {
                    current = reference
                        .getInnerValueEval()?
                        .ok_or(EvalFailure::Runtime("NullPointerException array reference"))?;
                }
                if current.as_any().is::<BlankEval>() || current.as_any().is::<MissingArgEval>() {
                    current = Rc::new(NumberEval::ZERO);
                }
                if let Some(text) = current
                    .as_any()
                    .downcast_ref::<crate::cn::wps::et::ss::formula::eval::StringEval::StringEval>(
                ) {
                    if String::from_utf16(text.getStringValue()).is_err() {
                        return Err(EvalFailure::Unported("UTF-16 formula result store"));
                    }
                }
                positions
                    .lock()
                    .unwrap()
                    .get_mut(&self.getSheetUid())
                    .ok_or(EvalFailure::Runtime("NullPointerException valuePos"))?
                    .insert(
                        RowColKeyHelper::makeLongKey(r + self.rowIndex, c + self.colIndex),
                        current,
                    );
            }
        }
        Ok(())
    }
}
