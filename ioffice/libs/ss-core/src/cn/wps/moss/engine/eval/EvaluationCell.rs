// java package cn.wps.moss.engine.eval;
use crate::cn::wps::et::ss::{
    formula::{
        evaluator::EvalResult,
        ptg::Ptg::{Ptg, PtgKind},
    },
    interfaces::IEvalCell::IEvalCell,
};
use crate::cn::wps::moss::{
    app::KmoSheet::KmoSheet,
    engine::grid::{
        KmoCellData::KmoCellData, KmoCellValueStore::KmoCellValueStore,
        KmoFormulaData::KmoFormulaData,
    },
};
use crate::java_runtime::{Rc, RefCell};
// java public final class EvaluationCell implements IEvalCell
pub struct EvaluationCell {
    ksheet: Rc<RefCell<KmoSheet>>,
    kcell: RefCell<KmoCellData>,
    row: i32,
    col: i32,
}
impl EvaluationCell {
    // java this.ksheet=s;this.kcell=c;this.row=row;this.col=col;
    pub fn EvaluationCell(s: Rc<RefCell<KmoSheet>>, c: KmoCellData, row: i32, col: i32) -> Self {
        Self {
            ksheet: s,
            kcell: RefCell::new(c),
            row,
            col,
        }
    }
    // rust: shared adapter for Java getter branches which unwrap formulaCachaId.
    fn valueId(&self) -> i32 {
        let id = self.kcell.borrow().id;
        if KmoCellValueStore::identifyTypeByID(id) == 7 {
            self.ksheet
                .borrow()
                .getCellValueStore()
                .borrow()
                .formulaCachaId(id)
        } else {
            id
        }
    }
}
impl IEvalCell for EvaluationCell {
    fn checkRecalculationFlag(&self) -> EvalResult<Option<bool>> {
        let cell = self.kcell.borrow();
        let sheet = self.ksheet.borrow();
        let values = sheet.getCellValueStore();
        if cell.formulaType == 0 {
            return Ok(None);
        }
        if cell.formulaType == 2 {
            let mut f = KmoFormulaData::KmoFormulaData();
            values.borrow().formulaValue(cell.id, &mut f);
            let bytes = values.borrow().formulaTokens(f.getTokensId());
            let ptgs =
                Ptg::readTokens__stream(f.getTokensPlainSize(), &bytes, false, sheet.getVersion());
            if let [Ptg {
                kind: PtgKind::Exp { row, col, .. },
                ..
            }] = ptgs.as_slice()
            {
                let master = sheet.getCellData(*row, *col as i32);
                return Ok(Some(values.borrow().getFormulaCalcFlag(master.id) & 1 != 0));
            }
            return Ok(None);
        }
        let flag = values.borrow().getFormulaCalcFlag(cell.id) & 1 != 0;
        Ok(Some(flag))
    }
    fn getFinalCellType(&self) -> i32 {
        // java unwrap cache type, blank -> 3; numeric/string/bool/error -> 0/1/4/5.
        match KmoCellValueStore::identifyTypeByID(self.valueId()) {
            0 => 3,
            1 => 0,
            2 => 1,
            5 => 4,
            6 => 5,
            _ => panic!("Wrong type of id"),
        }
    }
    // java ksheet.getCellData(row,col,kcell);
    fn updateCell(&self) {
        *self.kcell.borrow_mut() = self.ksheet.borrow().getCellData(self.row, self.col);
    }
    fn getNumericCellValue(&self) -> f64 {
        let id = self.valueId();
        self.ksheet
            .borrow()
            .getCellValueStore()
            .borrow_mut()
            .doubleValue(id)
    }
    fn getStringCellValue(&self) -> Option<String> {
        let id = self.valueId();
        Some(self.ksheet
            .borrow()
            .getCellValueStore()
            .borrow_mut()
            .stringValue(id))
    }
    fn getBooleanCellValue(&self) -> bool {
        let id = self.valueId();
        self.ksheet
            .borrow()
            .getCellValueStore()
            .borrow()
            .booleanValue(id)
    }
    fn getErrorCellValue(&self) -> i8 {
        let id = self.valueId();
        self.ksheet
            .borrow()
            .getCellValueStore()
            .borrow()
            .errorValue(id) as i8
    }
    fn getSheetUid(&self) -> i32 {
        self.ksheet.borrow().getSheetUid()
    }
    fn getRowIndex(&self) -> i32 {
        self.row
    }
    fn getColIndex(&self) -> i32 {
        self.col
    }
}
