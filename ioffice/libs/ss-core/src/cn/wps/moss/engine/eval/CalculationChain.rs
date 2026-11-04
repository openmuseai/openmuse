// java package cn.wps.moss.engine.eval;
use super::{EvaluationCellFormula::EvaluationCellFormula, EvaluationWorkbook::EvaluationWorkbook};
use crate::cn::wps::et::ss::{
    calcchain::{
        CalcChain::{CalcChain, ICalcChainListener, ICalculationCallback},
        CellFmlEvaluator::CellFmlEvaluator,
    },
    formula::ptg::Ptg::Ptg,
    interfaces::IEvalWorkbook::IEvalWorkbook,
};
use crate::cn::wps::moss::{
    app::KmoBook::KmoBook, engine::grid::KmoCellValueStore::KmoCellValueStore,
};
use crate::java_runtime::{Cell, RefCell};
use std::sync::{Arc, Weak};
// java public class CalculationChain
pub struct CalculationChain {
    kbook: Weak<KmoBook>,
    pub ew: Arc<EvaluationWorkbook>,
    pub cfe: Arc<CellFmlEvaluator>,
    pub calcChain: Arc<CalcChain>,
    initialized: Cell<bool>,
    fullCalculated: Cell<bool>,
    repealed: Cell<bool>,
    calcChainActive: Cell<bool>,
    evaluationCallback: RefCell<
        Option<Arc<dyn crate::cn::wps::et::ss::formula::IEvaluationCallback::IEvaluationCallback>>,
    >,
}
// java initCallback creates this IEvaluationCallback, whose methods dispatch to the same CalcChain.
struct EvaluationCallback(Weak<CalcChain>);
impl crate::cn::wps::et::ss::formula::IEvaluationCallback::IEvaluationCallback
    for EvaluationCallback
{
    fn evaluation(
        &self,
        uid: i32,
        row: i32,
        col: i32,
        needCalc: bool,
    ) -> crate::cn::wps::et::ss::formula::evaluator::EvalResult<
        Option<crate::cn::wps::et::ss::formula::eval::ValueRef>,
    > {
        self.0
            .upgrade()
            .expect("CalcChain")
            .evaluateOutside(uid, row, col, needCalc)
    }
    fn addRef(&self, uid: i32, row: i32, col: i32) -> bool {
        self.0.upgrade().expect("CalcChain").addRef(uid, row, col)
    }
}
struct AffinityCallback(
    Arc<dyn crate::cn::wps::moss::app::ThreadAffinityCallback::ThreadAffinityCallback>,
);
impl crate::cn::wps::et::ss::calcchain::listener::UnitThreadCallback::UnitThreadCallback
    for AffinityCallback
{
    // java beforeRun(): affinityCallback.enableBigCores();
    fn beforeRun(&self) {
        self.0.enableBigCores();
    }
}
impl CalculationChain {
    // java ew = new EvaluationWorkbook(kbook); cfe = new CellFmlEvaluator(ew); calcChain = new CalcChain(cfe,ew,calcChainListener); initCallback(); ew.setEvaluateCallback(evaluationCallback);
    pub fn CalculationChain(
        book: &Arc<KmoBook>,
        calcChainListener: Arc<dyn ICalcChainListener>,
    ) -> Self {
        let ew = Arc::new(EvaluationWorkbook::EvaluationWorkbook(book));
        let cfe = Arc::new(CellFmlEvaluator::CellFmlEvaluator(ew.clone()));
        let cc = Arc::new(CalcChain::CalcChain(
            cfe.clone(),
            ew.clone(),
            calcChainListener,
        ));
        let result = Self {
            kbook: Arc::downgrade(book),
            ew,
            cfe,
            calcChain: cc,
            initialized: Cell::new(false),
            fullCalculated: Cell::new(false),
            repealed: Cell::new(false),
            calcChainActive: Cell::new(true),
            evaluationCallback: RefCell::new(None),
        };
        // java initCallback(); ew.setEvaluateCallback(evaluationCallback);
        result.initCallback();
        result
            .ew
            .setEvaluateCallback(result.evaluationCallback.borrow().as_ref().unwrap().clone());
        result
    }
    // java evaluationCallback = new IEvaluationCallback(){ evaluation -> evaluateOutside; addRef -> addRef; };
    pub fn initCallback(&self) {
        *self.evaluationCallback.borrow_mut() = Some(Arc::new(EvaluationCallback(Arc::downgrade(
            &self.calcChain,
        ))));
    }
    // java if (affinityCallback!=null) calcChain.setUnitThreadCallback(new UnitThreadCallback(){beforeRun(){enableBigCores();}});
    pub fn setThreadAffinityCallback(
        &self,
        callback: Option<
            Arc<dyn crate::cn::wps::moss::app::ThreadAffinityCallback::ThreadAffinityCallback>,
        >,
    ) {
        if let Some(callback) = callback {
            self.calcChain
                .setUnitThreadCallback(Some(Arc::new(AffinityCallback(callback))));
        }
    }
    // java public void init()
    pub fn init(&self) {
        if self.repealed.get() || self.initialized.get() || !self.calcChainActive.get() {
            return;
        }
        self.initialized.set(true);
        let book = self.kbook.upgrade().unwrap();
        let start = std::time::Instant::now();
        let sv = book.getSpreadsheetVersion();
        for i in 0..book.sheetCount() {
            let sheet = book.sheetAt(i);
            let mut iter =
                sheet
                    .borrow()
                    .getCellIterator(0, sv.getLastRowIndex(), 0, sv.getLastColumnIndex());
            while iter.hasNext() {
                iter.next();
                let row = iter.row();
                let col = iter.col();
                let cell = iter.cellData().clone();
                // java if (ksheet.getCellType(iter)==7)
                if KmoCellValueStore::identifyTypeByID(cell.id) == 7 {
                    if cell.formulaType == 2 {
                        // java read ExpPtg; register only when this cell is the array master.
                        let values = sheet.borrow().getCellValueStore();
                        let mut f=crate::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData::KmoFormulaData();
                        values.borrow().formulaValue(cell.id, &mut f);
                        let bytes = values.borrow().formulaTokens(f.getTokensId());
                        let ptgs =
                            crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::readTokens__stream(
                                f.getTokensPlainSize(),
                                &bytes,
                                false,
                                sv,
                            );
                        match ptgs.first().map(|p| &p.kind) {
                            Some(crate::cn::wps::et::ss::formula::ptg::Ptg::PtgKind::Exp {
                                row: r,
                                col: c,
                                ..
                            }) => {
                                if *r == row && *c as i32 == col {
                                    self.calcChain.registerFml(Arc::new(super::EvaluationArrayFormula::EvaluationArrayFormula::EvaluationArrayFormula(sheet.clone(),row,col)),false);
                                }
                            }
                            _ => panic!("ClassCastException ExpPtg"),
                        }
                    } else {
                        self.calcChain.registerFml(
                            Arc::new(EvaluationCellFormula::EvaluationCellFormula(
                                sheet.clone(),
                                row,
                                col,
                            )),
                            false,
                        );
                    }
                }
                // java elapsed > 22000 only repeals on a constrained-memory host.
                // Desktop host has no Android small-heap limitation.
                let _elapsed = start.elapsed();
            }
            // java ksheet.rawFormulas = null; XLSX consumes CellFmlBackupHandler records before reader_callback.
        }
    }
    // java registerFormula/unregisterFormula's coordinate overloads inspect the real formula type before journalling.
    pub fn registerFormula(&self, sheet: i32, row: i32, col: i32) {
        if !self.repealed.get() && self.calcChainActive.get() {
            if let Some(f) = self.formulaAt(sheet, row, col) {
                self.interrupt();
                self.calcChain.registerFml(f, true);
            }
        }
    }
    pub fn unregisterFormula(&self, sheet: i32, row: i32, col: i32) {
        if !self.repealed.get() && self.calcChainActive.get() {
            if let Some(f) = self.formulaAt(sheet, row, col) {
                self.interrupt();
                self.calcChain.unregisterFml(&f);
            }
        }
    }
    // rust: shared adapter for the two identical Java coordinate inspection blocks.
    fn formulaAt(
        &self,
        index: i32,
        row: i32,
        col: i32,
    ) -> Option<Arc<dyn crate::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase>>
    {
        let book = self.kbook.upgrade().unwrap();
        let sheet = book.sheetAt(index);
        let cell = sheet.borrow().getCellData(row, col);
        if cell.formulaType == 1 || cell.formulaType == 3 {
            return Some(Arc::new(EvaluationCellFormula::EvaluationCellFormula(
                sheet, row, col,
            )));
        }
        if cell.formulaType == 2 {
            let values = sheet.borrow().getCellValueStore();
            let mut f =
                crate::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData::KmoFormulaData(
                );
            values.borrow().formulaValue(cell.id, &mut f);
            let bytes = values.borrow().formulaTokens(f.getTokensId());
            let ptgs = Ptg::readTokens__stream(
                f.getTokensPlainSize(),
                &bytes,
                false,
                book.getSpreadsheetVersion(),
            );
            if let [Ptg {
                kind: crate::cn::wps::et::ss::formula::ptg::Ptg::PtgKind::Exp { row: r, col: c, .. },
                ..
            }] = ptgs.as_slice()
            {
                if *r == row && *c as i32 == col {
                    return Some(Arc::new(super::EvaluationArrayFormula::EvaluationArrayFormula::EvaluationArrayFormula(sheet,row,col)));
                }
            }
        }
        None
    }
    pub fn registerFormula__formula(
        &self,
        f: Arc<dyn crate::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase>,
    ) {
        if !self.repealed.get() && self.calcChainActive.get() {
            self.interrupt();
            self.calcChain.registerFml(f, true);
        }
    }
    pub fn unregisterFormula__formula(
        &self,
        f: &Arc<dyn crate::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase>,
    ) {
        if !self.repealed.get() && self.calcChainActive.get() {
            self.interrupt();
            self.calcChain.unregisterFml(f);
        }
    }
    pub fn registerFormula__area(&self, sheet: i32, fr: i32, lr: i32, fc: i32, lc: i32) {
        if !self.repealed.get() && self.calcChainActive.get() {
            self.interrupt();
            self.calcChain.registerFml__area(
                sheet,
                &crate::cn::wps::et::ss::util::Area::Area::Area(fr, lr, fc, lc),
                true,
            );
        }
    }
    pub fn unregisterFormula__area(&self, sheet: i32, fr: i32, lr: i32, fc: i32, lc: i32) {
        if !self.repealed.get() && self.calcChainActive.get() {
            self.interrupt();
            self.calcChain.unregisterFml__area(
                sheet,
                &crate::cn::wps::et::ss::util::Area::Area::Area(fr, lr, fc, lc),
            );
        }
    }
    pub fn markVolatile(&self) {
        if !self.repealed.get() && self.calcChainActive.get() {
            if !self.initialized.get() {
                self.init();
            }
            self.interrupt();
            self.calcChain.markRecalculation__area(-1, None, false);
        }
    }
    pub fn markRecalculationByName(&self, index: i32) {
        if !self.repealed.get() && self.calcChainActive.get() {
            if !self.initialized.get() {
                self.init();
            }
            self.interrupt();
            self.calcChain.markRecalculationByName(index);
        }
    }
    pub fn markRecalculation__area(
        &self,
        sheet: i32,
        fr: i32,
        lr: i32,
        fc: i32,
        lc: i32,
        markCurrent: bool,
    ) {
        if !self.repealed.get() && self.calcChainActive.get() {
            if !self.initialized.get() {
                self.init();
            }
            self.interrupt();
            self.calcChain.markRecalculation__area(
                sheet,
                Some(&crate::cn::wps::et::ss::util::Area::Area::Area(
                    fr, lr, fc, lc,
                )),
                markCurrent,
            );
        }
    }
    pub fn markSubtotalRecalculation(&self, sheet: i32, fr: i32, lr: i32, fc: i32, lc: i32) {
        if !self.repealed.get() && self.calcChainActive.get() {
            if !self.initialized.get() {
                self.init();
            }
            self.interrupt();
            self.calcChain.markSubtotalCalculation(
                sheet,
                &crate::cn::wps::et::ss::util::Area::Area::Area(fr, lr, fc, lc),
            );
        }
    }
    pub fn isInterrupted(&self) -> bool {
        self.calcChain.isInterrupting()
    }
    pub fn isInterruptMark(&self) -> bool {
        self.calcChain.isInterruptMark()
    }
    pub fn interruptMark(&self) {
        self.calcChain.interruptMark();
    }
    pub fn isInCalulation__delay(&self, delay: i64) -> bool {
        self.calcChain.isInCalculating__delay(delay)
    }
    pub fn getCalcType(&self) -> Option<&'static str> {
        match self.calcChain.getCalcType() {
            1 => Some("auto"),
            16 => Some("updateData"),
            256 => Some("recal"),
            _ => None,
        }
    }
    // java public IEvalWorkbook getEvalWorkbook() { return ew; }
    pub fn getEvalWorkbook(&self) -> Arc<dyn IEvalWorkbook> {
        self.ew.clone()
    }
    // java isRepealed()/active()/turnOff()
    pub fn isRepealed(&self) -> bool {
        self.repealed.get()
    }
    pub fn active(&self) {
        self.calcChainActive.set(true);
    }
    pub fn turnOff(&self) {
        self.calcChainActive.set(false);
    }
    // java interrupt(): calcChain.interrupt(); ew.setInterruptMark(true);
    pub fn interrupt(&self) {
        self.calcChain.interrupt();
        self.ew.setInterruptMark(true);
    }
    pub fn resumeInterruptMark(&self) {
        self.calcChain.resumeMarkInTransaction();
    }
    // java forceMarkRecalculation: first call marks all, subsequent calls mark the requested cell/volatile graph.
    pub fn forceMarkRecalculation(&self, sheet: i32, row: i32, col: i32) {
        if !self.repealed.get() && self.calcChainActive.get() {
            if !self.initialized.get() {
                self.init();
            }
            if !self.calcChain.isInCalculating() {
                if !self.fullCalculated.get() {
                    self.fullCalculated.set(true);
                    self.calcChain.markAllRecalculation();
                } else {
                    self.calcChain.markRecalculation(sheet, row, col);
                }
            }
        }
    }
    pub fn markAllRecalculation(&self) {
        if !self.repealed.get() && self.calcChainActive.get() {
            if !self.initialized.get() {
                self.init();
            }
            self.interrupt();
            self.calcChain.markAllRecalculation();
        }
    }
    pub fn markRecalculation(&self, sheet: i32, row: i32, col: i32) {
        if !self.repealed.get() && self.calcChainActive.get() {
            if !self.initialized.get() {
                self.init();
            }
            self.interrupt();
            self.calcChain.markRecalculation(sheet, row, col);
        }
    }
    // java openCalc(): calcChain.setCalcType(1); calculateImpl(null);
    pub fn openCalc(&self) {
        self.calcChain.setCalcType(1);
        self.calculateImpl(None);
    }
    pub fn synCalculate(&self) {
        self.calculateImpl(None);
    }
    // java calculate(callback): new CalcRunnable; setCallback(callback); ThreadPool.execute(calcRunnable);
    pub fn calculate(self: &Arc<Self>) {
        self.calculate__callback(None);
    }
    // java new CalcRunnable(); setCallback(callback); ThreadPool.instance().execute(calcRunnable);
    pub fn calculate__callback(self: &Arc<Self>, callback: Option<Arc<dyn ICalculationCallback>>) {
        let mut runnable = CalcRunnable {
            calculationChain: self.clone(),
            callback: None,
            _bookRoot: self.kbook.upgrade(),
        };
        runnable.setCallback(callback);
        std::thread::spawn(move || runnable.run());
    }
    fn calculateImpl(&self, callback: Option<Arc<dyn ICalculationCallback>>) {
        if self.repealed.get() || !self.calcChainActive.get() {
            return;
        }
        let Some(book) = self.kbook.upgrade() else {
            return;
        };
        if !self.initialized.get() {
            self.init();
        }
        book.areaWatcher().borrow_mut().startMergeEvents();
        // java try { calcChain.calculate(callback); } catch(Exception) {} finally { areaWatcher.finishMergeEvents(); }
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.calcChain.calculate(callback)
        }));
        book.areaWatcher().borrow_mut().finishMergeEvents();
        // java catch (Exception e) logs and returns; the finally above still sends merged events.
        let _ = result;
    }
    pub fn getFormulaSize(&self) -> i32 {
        self.calcChain.getFormulaSize()
    }
    pub fn isInCalulation(&self) -> bool {
        self.calcChain.isInCalculating()
    }
}

// java class CalcRunnable implements Runnable; retained outer reference and per-call callback.
pub struct CalcRunnable {
    calculationChain: Arc<CalculationChain>,
    callback: Option<Arc<dyn ICalculationCallback>>,
    // Rust async root for the Java book retained by the enclosing Runnable/chain.
    _bookRoot: Option<Arc<KmoBook>>,
}
impl CalcRunnable {
    // java this.callback=callback;
    pub fn setCallback(&mut self, callback: Option<Arc<dyn ICalculationCallback>>) {
        self.callback = callback;
    }
    // java CalculationChain.this.calculateImpl(this.callback);
    pub fn run(&self) {
        self.calculationChain.calculateImpl(self.callback.clone());
    }
}
