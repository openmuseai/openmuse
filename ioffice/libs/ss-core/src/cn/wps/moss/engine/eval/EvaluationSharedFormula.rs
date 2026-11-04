// java package cn.wps.moss.engine.eval;

use crate::java_runtime::RefCell;
use crate::java_runtime::Rc;

use crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg;
use crate::cn::wps::et::ss::interfaces::IEvalSharedFormula::IEvalSharedFormula;
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::engine::grid::ExpFmlManager::{ExpFml, ExpFmlManager};

// java public final class EvaluationSharedFormula implements IEvalSharedFormula
pub struct EvaluationSharedFormula {
    // java private final KmoSheet ksheet;
    ksheet: Rc<RefCell<KmoSheet>>,
    // java private final int firstRow;
    firstRow: i32,
    // java private final int lastRow;
    lastRow: i32,
    // java private final int firstCol;
    firstCol: i32,
    // java private final int lastCol;
    lastCol: i32,
    // java private ExpFmlManager.ExpFml shrfml = null;
    shrfml: RefCell<Option<Rc<ExpFml>>>,
}

impl EvaluationSharedFormula {
    // java public EvaluationSharedFormula(KmoSheet ksheet, int firstRow, int lastRow, int firstCol, int lastCol)
    pub fn EvaluationSharedFormula(ksheet: Rc<RefCell<KmoSheet>>, firstRow: i32, lastRow: i32, firstCol: i32, lastCol: i32) -> Self {
        Self { ksheet, firstRow, lastRow, firstCol, lastCol, shrfml: RefCell::new(None) }
    }

    // java public KmoSheet getKSheet()
    pub fn getKSheet(&self) -> Rc<RefCell<KmoSheet>> { Rc::clone(&self.ksheet) }

    // java public int getSheetUid()
    pub fn getSheetUid(&self) -> i32 { self.ksheet.borrow().getSheetUid() }
    // java public int getSheetIndex()
    pub fn getSheetIndex(&self) -> i32 { self.ksheet.borrow().getSheetIndex() }
    // java public int getFmlRow()
    pub fn getFmlRow(&self) -> i32 { self.firstRow }
    // java public int getFmlCol()
    pub fn getFmlCol(&self) -> i32 { self.firstCol }
    // java public int getFmlLastRow()
    pub fn getFmlLastRow(&self) -> i32 { self.lastRow }
    // java public int getFmlLastCol()
    pub fn getFmlLastCol(&self) -> i32 { self.lastCol }

    // java public Ptg[] getTokens()
    pub fn getTokens(&self) -> Vec<Ptg> {
        // java if (this.shrfml == null)
        if self.shrfml.borrow().is_none() {
            // java this.shrfml = this.ksheet.getExpFmlManager().query(this.firstRow, this.firstCol, true);
            let shrfml = self.ksheet.borrow().getExpFmlManager().borrow().query(self.firstRow, self.firstCol, true);
            // java if (this.shrfml == null) throw new IllegalStateException(...);
            let shrfml = shrfml.unwrap_or_else(|| panic!("Can NOT find shared formula at postion row:{} col:{}", self.firstRow, self.firstCol));
            *self.shrfml.borrow_mut() = Some(shrfml);
        }
        let sheet = self.ksheet.borrow();
        let shrfml = self.shrfml.borrow();
        // java return ExpFmlManager.adjustSharedFormula(this.shrfml, this.firstRow, this.firstCol, this.ksheet.getBook().getSpreadsheetVersion());
        ExpFmlManager::adjustSharedFormula(shrfml.as_ref().unwrap(), self.firstRow, self.firstCol, sheet.getBook().getSpreadsheetVersion())
    }

    // java public boolean equals(Object o)
    pub fn equals(&self, other: &dyn IEvalSharedFormula) -> bool {
        // java if (this == o) return true;
        if std::ptr::eq(self as &dyn IEvalSharedFormula, other) { return true; }
        // java else if (!(this instanceof EvaluationSharedFormula)) return false;
        // The Java condition checks `this`, so the cast of `o` still runs.
        let x = other.as_any().downcast_ref::<EvaluationSharedFormula>().expect("ClassCastException: EvaluationSharedFormula");
        // java return x.ksheet == this.ksheet && x.firstRow == this.firstRow && x.lastRow == this.lastRow;
        Rc::ptr_eq(&self.ksheet, &x.ksheet) && self.firstRow == x.firstRow && self.lastRow == x.lastRow
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
        // java return uid << 8 ^ this.firstRow << 16 ^ this.lastRow;
        (self.getSheetUid() << 8) ^ (self.firstRow << 16) ^ self.lastRow
    }
}

impl IEvalSharedFormula for EvaluationSharedFormula {
    fn getSheetUid(&self) -> i32 { self.getSheetUid() }
    fn getSheetIndex(&self) -> i32 { self.getSheetIndex() }
    fn getFmlRow(&self) -> i32 { self.getFmlRow() }
    fn getFmlCol(&self) -> i32 { self.getFmlCol() }
    fn getFmlLastRow(&self) -> i32 { self.getFmlLastRow() }
    fn getFmlLastCol(&self) -> i32 { self.getFmlLastCol() }
    fn getTokens(&self) -> Vec<Ptg> { self.getTokens() }
    fn equals(&self, other: &dyn IEvalSharedFormula) -> bool { self.equals(other) }
    fn hashCode(&self) -> i32 { self.hashCode() }
    fn as_any(&self) -> &dyn std::any::Any { self }
}
