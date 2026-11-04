// java package cn.wps.et.ss.calcchain;
use super::{
    listener::{
        ICalculationListener::ICalculationListener, ListenerAdapter::ListenerAdapter,
        UnitThreadCallback::UnitThreadCallback,
    },
    threadpool::UnitThread::UnitThread,
};
use std::{
    collections::{HashMap, VecDeque},
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc, Mutex,
    },
};
// java public static int ProcessCount = Runtime.getRuntime().availableProcessors();
pub static ProcessCount: AtomicI32 = AtomicI32::new(1);
use super::marktype::{
    MarkType::MarkType, MarkTypeAll::MarkTypeAll, MarkTypeAreaParam::MarkTypeAreaParam,
    MarkTypeAreaSub::MarkTypeAreaSub, MarkTypeName::MarkTypeName, MarkTypeSingle::MarkTypeSingle,
};
use super::{recalcLock, CalcChainNode::CalcChainNode, CellFmlEvaluator::CellFmlEvaluator};
use crate::cn::wps::et::ss::util::Area::Area;
use crate::cn::wps::et::ss::{
    dependent::{
        refer::base::CellRefMap::CellRefMap, SharedFmlReferredArea::SharedFmlReferredArea,
    },
    formula::{
        eval::{OperandResolver::constRecognisor, ValueRef},
        evaluator::{EvalFailure, EvalResult},
        ptg::Ptg::Ptg,
    },
    interfaces::{IEvalFormulaBase::IEvalFormulaBase, IEvalWorkbook::IEvalWorkbook},
    util::{
        primeval::objmap::LongObjectHashMap::LongObjectHashMap, RowColKeyHelper::RowColKeyHelper,
    },
};
use crate::java_runtime::{Cell, RefCell};
use kfc::cn::wps::memor4j::{
    AtomSnapshot::{KmoAtom as SnapshotAtom, KmoAtomData as SnapshotData, SharedAtom},
    KmoAtom::KmoAtom,
    KmoAtomData::{KmoAtomData, KmoAtomDataSub},
};
pub(crate) type Node = Arc<RefCell<CalcChainNode>>;
// java public interface ICalcChainListener
pub trait ICalcChainListener: Send + Sync {
    fn onChange(&self, sheet: i32, row: i32, col: i32, ptgs: &[Ptg], mode: i32);
}
// java public interface ICalculationCallback
pub trait ICalculationCallback: Send + Sync {
    fn onStart(&self);
    fn onEnd(&self);
    // rust: Java instanceof ICalculationProgressCallback.
    fn onProgress(&self, _sheet: i32, _row: i32, _col: i32) -> bool {
        false
    }
}
// java protected static class Backup extends KmoAtomData
struct Backup {
    base: KmoAtomData,
    fmls: Vec<Arc<dyn IEvalFormulaBase>>,
    opType: i32,
}
impl Backup {
    fn new() -> Self {
        Self {
            base: KmoAtomData::new(),
            fmls: Vec::with_capacity(2),
            opType: 0,
        }
    }
}
impl KmoAtomDataSub for Backup {
    fn as_base(&self) -> &KmoAtomData {
        &self.base
    }
    fn as_base_mut(&mut self) -> &mut KmoAtomData {
        &mut self.base
    }
    // java Backup ret = new Backup(); ret.__forceOldest(); return ret;
    fn copy(&self) -> Self {
        let mut b = Self::new();
        b.base.__forceOldest();
        b
    }
}
// java class CalcMark
#[derive(Default)]
struct CalcMark {
    interrupted: bool,
    hasCycle: bool,
    calcSum: i64,
    checkedFml: i32,
    attemptedCount: i32,
    calcedFml: i32,
}
impl CalcMark {
    // java reSet deliberately leaves checkedFml intact and assigns calcedFml twice.
    fn reSet(&mut self) {
        self.interrupted = false;
        self.hasCycle = false;
        self.calcSum = 0;
        self.calcedFml = 0;
        self.attemptedCount = 0;
        self.calcedFml = 0;
    }
}
struct Links {
    fmlIdPos: LongObjectHashMap<Node>,
    fmlPos: LongObjectHashMap<Node>,
    calcChainHead: Option<Node>,
    calcChainTail: Option<Node>,
    fmlMaxIndex: i32,
    volatileFmls: Vec<Arc<dyn IEvalFormulaBase>>,
}
struct Core {
    atom: RefCell<KmoAtom<Backup>>,
    links: RefCell<Links>,
    evaluator: Arc<CellFmlEvaluator>,
    calculationWorkbook: Arc<dyn IEvalWorkbook>,
    calcChainListener: Arc<dyn ICalcChainListener>,
    cm: Mutex<CalcMark>,
    needRecalc: Cell<bool>,
    interrupting: Cell<bool>,
    inCalculating: Cell<bool>,
    calcStartTime: Cell<i64>,
    interruptMark: Cell<bool>,
    calcType: Cell<i32>,
    threadPools: Mutex<HashMap<usize, Arc<UnitThread>>>,
    unitThreadCallback: RefCell<Option<Arc<dyn UnitThreadCallback>>>,
    listener: RefCell<Option<Arc<ListenerAdapter>>>,
    evaluationGaps: Mutex<Vec<(i32, i32, i32, &'static str)>>,
    flagsLock: Mutex<()>,
    historyMarkList: RefCell<Vec<Arc<dyn MarkType>>>,
    callback: RefCell<Option<Arc<dyn ICalculationCallback>>>,
}
// java public final class CalcChain extends KmoAtom<CalcChain.Backup>
pub struct CalcChain {
    core: Arc<Core>,
    owner: Option<SharedAtom>,
}
impl CalcChain {
    // java constructor: super(new Backup()); retain evaluator, workbook and listener.
    pub fn CalcChain(
        eval: Arc<CellFmlEvaluator>,
        cw: Arc<dyn IEvalWorkbook>,
        listener: Arc<dyn ICalcChainListener>,
    ) -> Self {
        // java OperandResolver.constRecognisor.setLangeuage(cw.getLanguageType()); setUsingDate1904(cw.isUsing1904Date());
        {
            let mut r = constRecognisor.lock().unwrap();
            r.setLangeuage(cw.getLanguageType());
            r.setUsingDate1904(cw.isUsing1904Date());
        }
        let core = Arc::new(Core {
            atom: RefCell::new(KmoAtom::new(Backup::new())),
            links: RefCell::new(Links {
                fmlIdPos: LongObjectHashMap::LongObjectHashMap(),
                fmlPos: LongObjectHashMap::LongObjectHashMap(),
                calcChainHead: None,
                calcChainTail: None,
                fmlMaxIndex: 0,
                volatileFmls: Vec::new(),
            }),
            evaluator: eval,
            calculationWorkbook: cw,
            calcChainListener: listener,
            cm: Mutex::new(CalcMark::default()),
            needRecalc: Cell::new(false),
            interrupting: Cell::new(false),
            inCalculating: Cell::new(false),
            calcStartTime: Cell::new(0),
            interruptMark: Cell::new(false),
            calcType: Cell::new(0),
            threadPools: Mutex::new(HashMap::new()),
            unitThreadCallback: RefCell::new(None),
            listener: RefCell::new(None),
            evaluationGaps: Mutex::new(Vec::new()),
            flagsLock: Mutex::new(()),
            historyMarkList: RefCell::new(Vec::new()),
            callback: RefCell::new(None),
        });
        let owner: SharedAtom = Arc::new(Mutex::new(Box::new(Self {
            core: core.clone(),
            owner: None,
        })));
        core.atom.borrow_mut().bindSnapshotOwner(&owner);
        Self {
            core,
            owner: Some(owner),
        }
    }
    pub(crate) fn threadPools(&self) -> &Mutex<HashMap<usize, Arc<UnitThread>>> {
        &self.core.threadPools
    }
    pub(crate) fn nodeAtIndex(&self, index: i32) -> Option<Node> {
        self.core.links.borrow().fmlIdPos.get(index as i64).cloned()
    }
    pub(crate) fn maxIndex(&self) -> i32 {
        self.core.links.borrow().fmlMaxIndex
    }
    // java setUnitThreadCallback()/setListener()
    pub fn setUnitThreadCallback(&self, c: Option<Arc<dyn UnitThreadCallback>>) {
        *self.core.unitThreadCallback.borrow_mut() = c;
    }
    pub fn setListener(&self, l: Arc<dyn ICalculationListener>) {
        *self.core.listener.borrow_mut() = Some(Arc::new(ListenerAdapter::ListenerAdapter(l)));
    }
    fn cellCalculated(&self) {
        if let Some(l) = &*self.core.listener.borrow() {
            l.cellCalculated();
        }
    }
    // java private synchronized void appendNode(CalcChainNode ccn)
    fn appendNode(&self, ccn: &Node) {
        let fml = {
            let mut l = self.core.links.borrow_mut();
            let mut n = ccn.borrow_mut();
            n.nextCalcChainNode = None;
            n.preCalcChainNode = l.calcChainTail.as_ref().map(Arc::downgrade);
            if let Some(tail) = &l.calcChainTail {
                let mut t = tail.borrow_mut();
                t.nextCalcChainNode = Some(ccn.clone());
                n.index = t.index + 1;
            }
            l.calcChainTail = Some(ccn.clone());
            if l.calcChainHead.is_none() {
                l.calcChainHead = Some(ccn.clone());
            }
            l.fmlMaxIndex += 1;
            l.fmlIdPos.put(n.index as i64, ccn.clone());
            n.formula.clone()
        };
        self.core.calcChainListener.onChange(
            fml.getSheetIndex(),
            fml.getFmlRow(),
            fml.getFmlCol(),
            &fml.getTokens(),
            1,
        );
    }
    // java private synchronized void removeNode(CalcChainNode ccn)
    fn removeNode(&self, ccn: &Node) {
        let fml = {
            let mut l = self.core.links.borrow_mut();
            let mut n = ccn.borrow_mut();
            let pre = n.preCalcChainNode.as_ref().and_then(|p| p.upgrade());
            let next = n.nextCalcChainNode.clone();
            if let Some(p) = &pre {
                p.borrow_mut().nextCalcChainNode = next.clone();
            }
            if let Some(v) = &next {
                v.borrow_mut().preCalcChainNode = pre.as_ref().map(Arc::downgrade);
            }
            if l.calcChainHead
                .as_ref()
                .is_some_and(|h| Arc::ptr_eq(h, ccn))
            {
                l.calcChainHead = next;
            }
            if l.calcChainTail
                .as_ref()
                .is_some_and(|t| Arc::ptr_eq(t, ccn))
            {
                l.calcChainTail = pre;
            }
            n.nextCalcChainNode = None;
            n.preCalcChainNode = None;
            l.fmlIdPos.remove(n.index as i64);
            n.formula.clone()
        };
        self.core.calcChainListener.onChange(
            fml.getSheetIndex(),
            fml.getFmlRow(),
            fml.getFmlCol(),
            &fml.getTokens(),
            2,
        );
    }
    // java public void registerFml(IEvalFormulaBase fml, boolean inTrans)
    pub fn registerFml(&self, fml: Arc<dyn IEvalFormulaBase>, inTrans: bool) {
        self.interrupt();
        self.interruptMark();
        let _lock = recalcLock.lock();
        let _resume = ResumeMark(self);
        if inTrans {
            let mut atom = self.core.atom.borrow_mut();
            atom.beforeWrite();
            let mut b = atom.rawData_mut();
            b.opType = 1;
            b.fmls.push(fml.clone());
        }
        let ccn = Arc::new(RefCell::new(CalcChainNode::CalcChainNode(fml.clone())));
        let id = ccn.borrow().nodeId;
        let old = self.core.links.borrow_mut().fmlPos.put(id, ccn.clone());
        if old.is_none() {
            self.appendNode(&ccn);
        }
        let ptgs = fml.getTokens();
        if super::FormulaHelper::FormulaHelper::checkVolatile(
            &ptgs,
            fml.as_ref(),
            self.core.calculationWorkbook.as_ref(),
        ) {
            self.core.links.borrow_mut().volatileFmls.push(fml.clone());
        }
        self.core
            .calculationWorkbook
            .getDependent()
            .expect("Dependent")
            .borrow_mut()
            .registerFml(&fml, &ptgs, self.core.calculationWorkbook.as_ref(), inTrans);
        // java finally { resumeMark(); recalcLock.unlock(); }
        self.resumeMark();
    }
    // java public void unregisterFml(IEvalFormulaBase fml)
    pub fn unregisterFml(&self, fml: &Arc<dyn IEvalFormulaBase>) {
        self.interrupt();
        self.interruptMark();
        let _lock = recalcLock.lock();
        let _resume = ResumeMark(self);
        {
            let mut a = self.core.atom.borrow_mut();
            a.beforeWrite();
            a.rawData_mut().opType = 2;
            a.rawData_mut().fmls.push(fml.clone());
        }
        let old = self
            .core
            .links
            .borrow_mut()
            .fmlPos
            .remove(CellRefMap::cellPos2Long(
                fml.getSheetUid(),
                fml.getFmlRow(),
                fml.getFmlCol(),
            ));
        if let Some(n) = old {
            self.removeNode(&n);
        }
        self.core.links.borrow_mut().volatileFmls.retain(|v| {
            !(v.getSheetUid() == fml.getSheetUid()
                && v.getFmlRow() == fml.getFmlRow()
                && v.getFmlCol() == fml.getFmlCol())
        });
        self.core
            .calculationWorkbook
            .getDependent()
            .unwrap()
            .borrow_mut()
            .unregisterFml(fml, self.core.calculationWorkbook.as_ref());
        self.resumeMark();
    }
    // java registerFml(sheetIndex,Area,inTrans) journals one Backup containing all iterator formulas.
    pub fn registerFml__area(&self, sheet: i32, area: &Area, inTrans: bool) {
        self.interrupt();
        self.interruptMark();
        let _lock = recalcLock.lock();
        let _resume = ResumeMark(self);
        if inTrans {
            let mut atom = self.core.atom.borrow_mut();
            atom.beforeWrite();
            atom.rawData_mut().opType = 1;
        }
        for f in self
            .core
            .calculationWorkbook
            .getSheet(sheet)
            .getFormulaIterator(area)
            .expect("NullPointerException")
        {
            if let Some(f) = f {
                if inTrans {
                    self.core
                        .atom
                        .borrow_mut()
                        .rawData_mut()
                        .fmls
                        .push(f.clone());
                }
                let n = Arc::new(RefCell::new(CalcChainNode::CalcChainNode(f.clone())));
                let id = n.borrow().nodeId;
                let old = self.core.links.borrow_mut().fmlPos.put(id, n.clone());
                if old.is_none() {
                    self.appendNode(&n);
                }
                let ptgs = f.getTokens();
                if super::FormulaHelper::FormulaHelper::checkVolatile(
                    &ptgs,
                    f.as_ref(),
                    self.core.calculationWorkbook.as_ref(),
                ) {
                    self.core.links.borrow_mut().volatileFmls.push(f.clone());
                }
                self.core
                    .calculationWorkbook
                    .getDependent()
                    .unwrap()
                    .borrow_mut()
                    .registerFml(&f, &ptgs, self.core.calculationWorkbook.as_ref(), inTrans);
            }
        }
    }
    // java unregisterFml(sheetIndex,Area) walks the linked chain, retaining next before removal.
    pub fn unregisterFml__area(&self, sheet: i32, area: &Area) {
        self.interrupt();
        self.interruptMark();
        let _lock = recalcLock.lock();
        let _resume = ResumeMark(self);
        {
            let mut atom = self.core.atom.borrow_mut();
            atom.beforeWrite();
            atom.rawData_mut().opType = 2;
        }
        let uid = self.core.calculationWorkbook.sheetIndex2SheetUid(sheet);
        let mut node = self.core.links.borrow().calcChainHead.clone();
        while let Some(n) = node {
            let (next, f) = {
                let n = n.borrow();
                (n.nextCalcChainNode.clone(), n.formula.clone())
            };
            node = next;
            if f.getSheetUid() == uid && area.contains(f.getFmlRow(), f.getFmlCol()) {
                self.core
                    .atom
                    .borrow_mut()
                    .rawData_mut()
                    .fmls
                    .push(f.clone());
                let old = self
                    .core
                    .links
                    .borrow_mut()
                    .fmlPos
                    .remove(CellRefMap::cellPos2Long(uid, f.getFmlRow(), f.getFmlCol()));
                if let Some(old) = old {
                    self.removeNode(&old);
                }
                self.core
                    .calculationWorkbook
                    .getDependent()
                    .unwrap()
                    .borrow_mut()
                    .unregisterFml(&f, self.core.calculationWorkbook.as_ref());
            }
        }
        self.core
            .links
            .borrow_mut()
            .volatileFmls
            .retain(|f| !(f.getSheetUid() == uid && area.contains(f.getFmlRow(), f.getFmlCol())));
    }
    // java public CalcChainNode getNode(int sheetId, int row, int col)
    pub fn getNode(&self, uid: i32, row: i32, col: i32) -> Option<Node> {
        self.core
            .links
            .borrow()
            .fmlPos
            .get(CellRefMap::cellPos2Long(uid, row, col))
            .cloned()
    }
    // java node == null ? null : node.formula;
    pub fn getFml(&self, uid: i32, row: i32, col: i32) -> Option<Arc<dyn IEvalFormulaBase>> {
        self.getNode(uid, row, col)
            .map(|n| n.borrow().formula.clone())
    }
    // java public void markAllRecalculation()
    pub fn markAllRecalculation(&self) {
        self.interrupt();
        let _lock = recalcLock.lock();
        self.resumeMark();
        self.markAllRecalculationImpl();
    }
    // java public void markAllRecalculationImpl()
    pub fn markAllRecalculationImpl(&self) {
        let _finally = MarkAllFinally(self);
        let mut n = self.core.links.borrow().calcChainHead.clone();
        while let Some(current) = n {
            if self.isInterruptMark() {
                break;
            }
            let current = current.borrow();
            current.formula.setNeedRecalculationFlag(true);
            n = current.nextCalcChainNode.clone();
        }
        if self.core.links.borrow().calcChainHead.is_some() {
            self.core.needRecalc.set(true);
        }
    }
    // java _push_to_stack: shared-formula query expands the original formula rectangle.
    fn push_to_stack(
        &self,
        supported: Vec<Arc<dyn IEvalFormulaBase>>,
        shared: Vec<SharedFmlReferredArea>,
        stack: &mut VecDeque<Arc<dyn IEvalFormulaBase>>,
    ) {
        for f in supported {
            if self.isInterruptMark() {
                return;
            }
            if f.isNeedRecalculationFlagSet() {
                self.core.needRecalc.set(true);
            } else {
                f.setNeedRecalculationFlag(true);
                self.core.needRecalc.set(true);
                stack.push_front(f);
            }
        }
        for q in shared {
            for r in q.fml.getFmlRow()..=q.fml.getFmlLastRow() {
                for c in q.fml.getFmlCol()..=q.fml.getFmlLastCol() {
                    if self.isInterruptMark() {
                        break;
                    }
                    if let Some(f) = self.getFml(q.fml.getSheetUid(), r, c) {
                        if !f.isNeedRecalculationFlagSet() {
                            f.setNeedRecalculationFlag(true);
                            self.core.needRecalc.set(true);
                            stack.push_front(f);
                        }
                    }
                }
            }
        }
    }
    // java _walk_stack's IEvalCellFormula branch.
    fn walk_stack(&self, stack: &mut VecDeque<Arc<dyn IEvalFormulaBase>>) {
        while !stack.is_empty() && !self.isInterruptMark() {
            let f = stack.pop_front().unwrap();
            let (mut out, mut shared) = (Vec::new(), Vec::new());
            let dep = self.core.calculationWorkbook.getDependent().unwrap();
            if f.as_cell_formula().is_some() {
                dep.borrow().queryStaticReferred(
                    f.getSheetUid(),
                    f.getFmlRow(),
                    f.getFmlCol(),
                    &mut out,
                    &mut shared,
                );
            } else if let Some(a) = f.as_array_formula() {
                let area = crate::cn::wps::et::ss::util::Area::Area::Area(
                    f.getFmlRow(),
                    a.getFmlLastRow(),
                    f.getFmlCol(),
                    a.getFmlLastCol(),
                );
                dep.borrow().queryStaticReferred__area(
                    f.getSheetUid(),
                    &area,
                    &mut out,
                    &mut shared,
                );
            } else {
                panic!("Unexpected data type");
            }
            self.push_to_stack(out, shared, stack);
        }
    }
    // java markRecalculation(int sheetIndex,int row,int col)
    pub fn markRecalculation(&self, sheet: i32, row: i32, col: i32) {
        self.interrupt();
        let _lock = recalcLock.lock();
        self.resumeMark();
        self.markRecalculationImpl(sheet, row, col);
        self.markHistory();
    }
    pub fn markRecalculationImpl(&self, sheet: i32, row: i32, col: i32) {
        let mut mt = MarkTypeSingle::new();
        mt.setSheetIndex(sheet);
        mt.setRow(row);
        mt.setCol(col);
        let _finally = HistoryMark(self, Arc::new(mt));
        let mut stack = VecDeque::new();
        if !self.core.needRecalc.get() {
            let vol = self.core.links.borrow().volatileFmls.clone();
            let count = vol.len();
            for f in vol {
                if self.isInterruptMark() {
                    return;
                }
                f.setNeedRecalculationFlag(true);
                stack.push_front(f);
            }
            if count > 0 {
                self.core.needRecalc.set(true);
            }
        }
        if sheet >= 0 && row >= 0 && col >= 0 {
            // java getFormula returns a new cell formula, not the registered chain node.
            if let Some(f) = self.core.calculationWorkbook.getFormula(sheet, row, col) {
                f.setNeedRecalculationFlag(true);
                self.core.needRecalc.set(true);
                stack.push_front(f);
            } else {
                let (mut out, mut shared) = (Vec::new(), Vec::new());
                self.core
                    .calculationWorkbook
                    .getDependent()
                    .unwrap()
                    .borrow()
                    .queryStaticReferred(
                        self.core.calculationWorkbook.sheetIndex2SheetUid(sheet),
                        row,
                        col,
                        &mut out,
                        &mut shared,
                    );
                self.push_to_stack(out, shared, &mut stack);
            }
        }
        self.walk_stack(&mut stack);
    }
    // java public void markAllRecalculationNoLock()
    pub fn markAllRecalculationNoLock(&self) {
        self.interrupt();
        self.markAllRecalculationImpl();
    }
    pub fn markRecalculationNoLock(&self, sheet: i32, row: i32, col: i32) {
        self.interrupt();
        self.markRecalculationImpl(sheet, row, col);
    }
    // java handlerMarkType: deduplicate using the query object's equals, preserving AreaParam(null)'s unusual behavior.
    pub fn handlerMarkType(&self, mt: Arc<dyn MarkType>) {
        let mut list = self.core.historyMarkList.borrow_mut();
        let index = list.iter().position(|v| mt.equals(Some(v.as_ref())));
        if self.isInterruptMark() {
            if index.is_none() {
                list.push(mt);
            }
        } else if let Some(i) = index {
            list.remove(i);
        }
    }
    // java markHistory snapshots historyMarkList before invoking each reMark(this).
    pub fn markHistory(&self) {
        if !self.isInterruptMark() {
            let history = self.core.historyMarkList.borrow().clone();
            for mt in history {
                mt.reMark(self);
            }
        }
    }
    pub fn markRecalculationByName(&self, index: i32) {
        if index >= 0 {
            self.interrupt();
            let _lock = recalcLock.lock();
            self.resumeMark();
            self.markRecalculationByNameImpl(index);
            self.markHistory();
        }
    }
    pub fn markRecalculationByNameNoLock(&self, index: i32) {
        if index >= 0 {
            self.interrupt();
            self.markRecalculationByNameImpl(index);
        }
    }
    fn markRecalculationByNameImpl(&self, index: i32) {
        let mut mt = MarkTypeName::new();
        mt.setIndex(index);
        let _finally = HistoryMark(self, Arc::new(mt));
        let mut stack = VecDeque::new();
        let mut out = Vec::new();
        self.core
            .calculationWorkbook
            .getDependent()
            .unwrap()
            .borrow()
            .queryNameReferred(index, &mut out);
        self.push_to_stack(out, Vec::new(), &mut stack);
        self.walk_stack(&mut stack);
    }
    // java markRecalculation(sheetIndex,Area,markCurrentRange)
    pub fn markRecalculation__area(&self, sheet: i32, area: Option<&Area>, markCurrent: bool) {
        self.interrupt();
        let _lock = recalcLock.lock();
        self.resumeMark();
        self.markRecalculationImpl__area(sheet, area, markCurrent);
        self.markHistory();
    }
    pub fn markRecalculationNoLock__area(
        &self,
        sheet: i32,
        area: Option<&Area>,
        markCurrent: bool,
    ) {
        self.interrupt();
        self.markRecalculationImpl__area(sheet, area, markCurrent);
    }
    fn markRecalculationImpl__area(&self, sheet: i32, area: Option<&Area>, markCurrent: bool) {
        let mut mt = MarkTypeAreaParam::new();
        mt.setSheetIndex(sheet);
        mt.setArea(area.cloned());
        mt.setMarkCurrentRange(markCurrent);
        let _finally = HistoryMark(self, Arc::new(mt));
        let mut stack = VecDeque::new();
        if !self.core.needRecalc.get() || area.is_none() {
            let vol = self.core.links.borrow().volatileFmls.clone();
            let count = vol.len();
            for f in vol {
                if self.isInterruptMark() {
                    return;
                }
                f.setNeedRecalculationFlag(true);
                stack.push_front(f);
            }
            if count > 0 {
                self.core.needRecalc.set(true);
            }
        }
        if let Some(area) = area {
            let uid = self.core.calculationWorkbook.sheetIndex2SheetUid(sheet);
            if markCurrent {
                if (area.acreage() as i64)
                    < self.core.links.borrow().fmlPos.iterator().count() as i64
                {
                    let iter = self
                        .core
                        .calculationWorkbook
                        .getSheet(sheet)
                        .getFormulaIterator(area).expect("NullPointerException");
                    for f in iter {
                        if self.isInterruptMark() {
                            return;
                        }
                        if let Some(f) = f {
                            f.setNeedRecalculationFlag(true);
                            self.core.needRecalc.set(true);
                        }
                    }
                } else {
                    let mut node = self.core.links.borrow().calcChainHead.clone();
                    while let Some(n) = node {
                        if self.isInterruptMark() {
                            return;
                        }
                        let n = n.borrow();
                        let f = &n.formula;
                        if f.getSheetUid() == uid && area.contains(f.getFmlRow(), f.getFmlCol()) {
                            f.setNeedRecalculationFlag(true);
                            self.core.needRecalc.set(true);
                        }
                        node = n.nextCalcChainNode.clone();
                    }
                }
            }
            let (mut out, mut shared) = (Vec::new(), Vec::new());
            self.core
                .calculationWorkbook
                .getDependent()
                .unwrap()
                .borrow()
                .queryStaticReferred__area(uid, area, &mut out, &mut shared);
            self.push_to_stack(out, shared, &mut stack);
            self.walk_stack(&mut stack);
        }
    }
    pub fn markSubtotalCalculation(&self, sheet: i32, area: &Area) {
        self.interrupt();
        let _lock = recalcLock.lock();
        self.resumeMark();
        self.markSubtotalCalculationImpl(sheet, area);
        self.markHistory();
    }
    pub fn markSubtotalCalculationNoLock(&self, sheet: i32, area: &Area) {
        self.interrupt();
        self.markSubtotalCalculationImpl(sheet, area);
    }
    fn markSubtotalCalculationImpl(&self, sheet: i32, area: &Area) {
        let mut mt = MarkTypeAreaSub::new();
        mt.setArea(Some(area.clone()));
        mt.setSheetIndex(sheet);
        let _finally = HistoryMark(self, Arc::new(mt));
        let mut stack = VecDeque::new();
        if !self.core.needRecalc.get() {
            let vol = self.core.links.borrow().volatileFmls.clone();
            let count = vol.len();
            for f in vol {
                if self.isInterruptMark() {
                    return;
                }
                f.setNeedRecalculationFlag(true);
                stack.push_front(f);
            }
            if count > 0 {
                self.core.needRecalc.set(true);
            }
        }
        let uid = self.core.calculationWorkbook.sheetIndex2SheetUid(sheet);
        let (mut supported, mut shared) = (Vec::new(), Vec::new());
        self.core
            .calculationWorkbook
            .getDependent()
            .unwrap()
            .borrow()
            .queryStaticReferred__area(uid, area, &mut supported, &mut shared);
        for f in supported {
            if self.isInterruptMark() {
                return;
            }
            if !f.isNeedRecalculationFlagSet()
                && super::FormulaHelper::FormulaHelper::hasSubtotal(&f.getTokens())
            {
                f.setNeedRecalculationFlag(true);
                self.core.needRecalc.set(true);
                stack.push_front(f);
            }
        }
        for q in shared {
            if self.isInterruptMark() {
                return;
            }
            if super::FormulaHelper::FormulaHelper::hasSubtotal(&q.fml.getTokens()) {
                let sheet = q.fml.getSheetIndex();
                for r in q.marked.getFirstRow()..=q.marked.getLastRow() {
                    for c in q.marked.getFirstCol()..=q.marked.getLastCol() {
                        if self.isInterruptMark() {
                            return;
                        }
                        if let Some(f) = self.core.calculationWorkbook.getFormula(sheet, r, c) {
                            if !f.isNeedRecalculationFlagSet() {
                                f.setNeedRecalculationFlag(true);
                                self.core.needRecalc.set(true);
                                stack.push_front(f);
                            }
                        }
                    }
                }
            }
        }
        self.walk_stack(&mut stack);
    }
    // java public IValueEval evaluateOutside(int uid,int r,int c,boolean needCalc)
    pub fn evaluateOutside(
        &self,
        uid: i32,
        row: i32,
        col: i32,
        needCalc: bool,
    ) -> EvalResult<Option<ValueRef>> {
        if self.isInterrupting() {
            return Ok(None);
        }
        if needCalc {
            let Some(f) = self.getFml(uid, row, col) else {
                return Ok(None);
            };
            if self.core.evaluator.evaluateFormula(&f)? {
                self.checkInputPos(&f)?;
            }
        }
        Ok(self
            .core
            .evaluator
            .valuePos
            .lock()
            .unwrap()
            .get(&uid)
            .and_then(|l| l.get(&RowColKeyHelper::makeLongKey(row, col)))
            .cloned())
    }
    // java evaluator.threadCalcStack.addRef(getFml(...)); return true;
    pub fn addRef(&self, uid: i32, row: i32, col: i32) -> bool {
        self.core
            .evaluator
            .threadCalcStack
            .addRef__nullable(self.getFml(uid, row, col));
        true
    }
    fn checkInputPos(&self, f: &Arc<dyn IEvalFormulaBase>) -> EvalResult<()> {
        if self.core.callback.borrow().as_ref().is_some_and(|c| {
            f.as_cell_formula().is_some()
                && c.onProgress(f.getSheetIndex(), f.getFmlRow(), f.getFmlCol())
        }) {
            let value = self
                .core
                .evaluator
                .valuePos
                .lock()
                .unwrap()
                .get(&f.getSheetUid())
                .and_then(|l| l.get(&RowColKeyHelper::makeLongKey(f.getFmlRow(), f.getFmlCol())))
                .cloned();
            if let Some(v) = value {
                use crate::cn::wps::et::ss::formula::eval::{
                    BoolEval::BoolEval, ErrorEval::ErrorEval, NumberEval::NumberEval,
                    StringEval::StringEval,
                };
                let cell = f.as_cell_formula().unwrap();
                // java checkInputPos writes the scalar directly, without setFormulaResult's cycle predicate.
                if let Some(n) = v.as_any().downcast_ref::<NumberEval>() {
                    cell.setNumberResult(n.getNumberValue());
                } else if let Some(b) = v.as_any().downcast_ref::<BoolEval>() {
                    cell.setBoolResult(b.getBooleanValue());
                } else if let Some(s) = v.as_any().downcast_ref::<StringEval>() {
                    let s = String::from_utf16(s.getStringValue())
                        .map_err(|_| EvalFailure::Unported("UTF-16 result store"))?;
                    cell.setStringResult(&s);
                } else if let Some(e) = v.as_any().downcast_ref::<ErrorEval>() {
                    cell.setErrorResult(e.getErrorCode() as i8);
                }
            }
        }
        Ok(())
    }
    // java public void calculate(ICalculationCallback callback)
    pub fn calculate(&self, callback: Option<Arc<dyn ICalculationCallback>>) {
        self.core.inCalculating.set(true);
        self.core.calcStartTime.set(now());
        if let Some(c) = &callback {
            self.setCalcType(16);
            c.onStart();
        }
        if !self.core.needRecalc.get() {
            self.core.inCalculating.set(false);
            self.core.calcStartTime.set(-1);
            self.setCalcType(0);
            return;
        }
        let _lock = recalcLock.lock();
        *self.core.callback.borrow_mut() = callback.clone();
        self.core.interrupting.set(false);
        let evaluated = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            for i in 0..self.core.calculationWorkbook.getSheetCount() {
                self.core
                    .evaluator
                    .valuePos
                    .lock()
                    .unwrap()
                    .entry(self.core.calculationWorkbook.sheetIndex2SheetUid(i))
                    .or_default();
            }
            self.core.calculationWorkbook.setInterruptMark(false);
            if let Some(l) = &*self.core.listener.borrow() {
                l.beginCalculation(self.core.links.borrow().fmlPos.iterator().count() as i32);
            }
            self.clearCalcMark();
            self.startMultiThreadsCalculate();
            if let Some(l) = &*self.core.listener.borrow() {
                l.finishCalculation();
            }
        }));
        // java finally/success share this cleanup sequence, including writeDirty BEFORE resetting flags.
        let history = self.core.needRecalc.get();
        let interrupted = self.core.cm.lock().unwrap().interrupted;
        self.core.needRecalc.set(interrupted);
        // java this try/catch(Exception) encloses both writeDirty and callback.onEnd.
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !self.core.needRecalc.get() {
                self.writeDirty();
            }
            if history {
                if let Some(c) = callback {
                    c.onEnd();
                }
            }
        }));
        {
            let _flags = self.core.flagsLock.lock().unwrap();
            self.core.inCalculating.set(false);
            self.core.calcStartTime.set(-1);
            self.core.interrupting.set(interrupted);
            self.setCalcType(0);
        }
        drop(_lock);
        self.core.cm.lock().unwrap().reSet();
        self.clearEvalCache();
        if let Err(error) = evaluated {
            std::panic::resume_unwind(error);
        }
        // java if (cm.hasCycle) throw new CircleReferenceException(...); cm.reSet above intentionally cleared this flag.
        if self.core.cm.lock().unwrap().hasCycle {
            panic!("circle reference detected");
        }
    }
    // java private void writeDirty()
    fn writeDirty(&self) {
        let values = self.core.evaluator.valuePos.lock().unwrap().clone();
        for (uid, child) in values {
            for (key, value) in child {
                self.core.calculationWorkbook.writeDirty(
                    &value,
                    uid,
                    RowColKeyHelper::getRowFromKey(key),
                    RowColKeyHelper::getColFromKey(key),
                );
            }
        }
        self.core.evaluator.valuePos.lock().unwrap().clear();
    }
    fn clearCalcMark(&self) {
        self.core.evaluationGaps.lock().unwrap().clear();
        let mut n = self.core.links.borrow().calcChainHead.clone();
        while let Some(current) = n {
            let mut c = current.borrow_mut();
            c.interrupted = false;
            c.cycled = false;
            c.refNode = None;
            c.formula.setCycled(false);
            c.formula.setRef(None);
            n = c.nextCalcChainNode.clone();
        }
        self.core.evaluator.clean();
    }
    // java fillUnitArray uses fmlMaxIndex and keeps holes left by removeNode.
    fn fillUnitArray(&self, processCount: usize) -> Vec<[i32; 2]> {
        let l = self.core.links.borrow();
        let size = l.fmlPos.iterator().count();
        let count = if size / 2 < processCount {
            size
        } else {
            processCount
        };
        ProcessCount.store(count as i32, Ordering::SeqCst);
        if count == 0 {
            return Vec::new();
        }
        let unitSize = (l.fmlMaxIndex - 1) / count as i32;
        let mut out = Vec::new();
        for i in 0..count {
            if i == 0 {
                out.push([0, unitSize]);
            } else {
                let start = (out[i - 1][1] + 1).min(l.fmlMaxIndex);
                out.push([start, (start + unitSize).min(l.fmlMaxIndex)]);
            }
        }
        out
    }
    fn startMultiThreadsCalculate(&self) {
        let count = std::thread::available_parallelism().map_or(1, |n| n.get());
        ProcessCount.store(count as i32, Ordering::SeqCst);
        let ranges = self.fillUnitArray(count);
        std::thread::scope(|scope| {
            let mut futures = Vec::new();
            for [mut start, mut end] in ranges {
                let mut first = self.nodeAtIndex(start);
                let mut last = self.nodeAtIndex(end);
                while first.is_none() && start < end {
                    start += 1;
                    first = self.nodeAtIndex(start);
                }
                while last.is_none() && end > start {
                    end -= 1;
                    last = self.nodeAtIndex(end);
                }
                // java UnitThread thread=new UnitThread(this,startNode,endNode); setUnitThreadCallback; pool.submit(thread);
                let chain = Arc::new(Self {
                    core: self.core.clone(),
                    owner: None,
                });
                let thread = Arc::new(UnitThread::UnitThread(chain, first, last));
                thread.setUnitThreadCallback(self.core.unitThreadCallback.borrow().clone());
                futures.push(scope.spawn(move || thread.run()));
            }
            // java future.get catches InterruptedException/ExecutionException.
            for f in futures {
                let _ = f.join();
            }
        });
    }
    // java calculateUnit(CalcChainNode start,CalcChainNode end,UnitThread unitWork)
    pub(crate) fn calculateUnit(
        &self,
        mut start: Option<Node>,
        mut end: Option<Node>,
        unit: &UnitThread,
    ) {
        unit.meetInterrupt.set(false);
        let mut current = start
            .clone()
            .or_else(|| self.core.links.borrow().calcChainHead.clone());
        'next: while let Some(n) = current.clone() {
            if self.isInterrupting() {
                self.core.cm.lock().unwrap().interrupted = true;
                unit.meetBreak.set(true);
                break;
            }
            if ProcessCount.load(Ordering::SeqCst) > 1 {
                let old = start
                    .as_ref()
                    .expect("NullPointerException start")
                    .borrow()
                    .index;
                let new = unit
                    .start
                    .borrow()
                    .as_ref()
                    .expect("NullPointerException unit.start")
                    .borrow()
                    .index;
                if old != new {
                    start = unit.start.borrow().clone();
                    end = unit.end.borrow().clone();
                    current = start.clone();
                    continue;
                }
                if unit.end.borrow().is_none() {
                    continue;
                }
                if end.as_ref().unwrap().borrow().index
                    != unit.end.borrow().as_ref().unwrap().borrow().index
                {
                    end = unit.end.borrow().clone();
                }
                if n.borrow().index > end.as_ref().unwrap().borrow().index {
                    break;
                }
            }
            let reference = n.borrow().refNode.as_ref().and_then(|r| r.upgrade());
            if reference.is_some_and(|r| r.borrow().formula.isNeedRecalculationFlagSet()) {
                unit.meetInterrupt.set(true);
                current = n.borrow().nextCalcChainNode.clone();
                continue;
            }
            let f = n.borrow().formula.clone();
            let t = now();
            if !f.isNeedRecalculationFlagSet() {
                current = n.borrow().nextCalcChainNode.clone();
                self.cellCalculated();
                continue;
            }
            self.core.cm.lock().unwrap().attemptedCount += 1;
            // java catch(RuntimeException) is per formula. Rust adapters for
            // unchecked Java exceptions unwind; catch them at this same site,
            // so a bad cell does not terminate the rest of its UnitThread.
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.core
                    .evaluator
                    .evaluateFormula(&f)
                    .and_then(|_| self.checkInputPos(&f))
            }))
            .unwrap_or(Err(EvalFailure::Runtime("Java RuntimeException")));
            self.core.cm.lock().unwrap().calcSum += now() - t;
            match result {
                Ok(()) => self.core.cm.lock().unwrap().calcedFml += 1,
                // java catch (NotCalculatedException nce)
                Err(EvalFailure::NotCalculated(e)) => {
                    unit.meetInterrupt.set(true);
                    let up = self.getNode(e.getSheetUid(), e.getRow(), e.getCol());
                    e.recycle();
                    if let Some(up) = up {
                        n.borrow_mut().interrupted = true;
                        n.borrow_mut().refNode = Some(Arc::downgrade(&up));
                        if up.borrow().cycled {
                            n.borrow_mut().cycled = true;
                            f.setZeroResultIfBlank();
                            f.setNeedRecalculationFlag(false);
                            current = n.borrow().nextCalcChainNode.clone();
                            self.cellCalculated();
                            continue;
                        }
                        if up.borrow().interrupted {
                            let mut reference =
                                up.borrow().refNode.as_ref().and_then(|r| r.upgrade());
                            while let Some(r) = reference.clone() {
                                if Arc::ptr_eq(&r, &n) {
                                    break;
                                }
                                reference = r.borrow().refNode.as_ref().and_then(|v| v.upgrade());
                            }
                            if reference.is_none() {
                                current = n.borrow().nextCalcChainNode.clone();
                                continue;
                            }
                            if reference.as_ref().is_some_and(|r| Arc::ptr_eq(r, &n)) {
                                self.core.cm.lock().unwrap().hasCycle = true;
                                let mut r = n.clone();
                                loop {
                                    r.borrow_mut().cycled = true;
                                    let cf = r.borrow().formula.clone();
                                    cf.setZeroResultIfBlank();
                                    cf.setNeedRecalculationFlag(false);
                                    self.cellCalculated();
                                    let next =
                                        r.borrow().refNode.as_ref().and_then(|v| v.upgrade());
                                    if next.as_ref().is_none_or(|v| Arc::ptr_eq(v, &n)) {
                                        continue 'next;
                                    }
                                    r = next.unwrap();
                                }
                            }
                        }
                    }
                    current = n.borrow().nextCalcChainNode.clone();
                    continue;
                }
                // java catch(RuntimeException) {}. Implementation boundaries remain separate from spreadsheet errors.
                Err(EvalFailure::Runtime(_)) => (),
                Err(EvalFailure::Unported(reason)) => self
                    .core
                    .evaluationGaps
                    .lock()
                    .unwrap()
                    .push((f.getSheetUid(), f.getFmlRow(), f.getFmlCol(), reason)),
            }
            n.borrow_mut().interrupted = false;
            n.borrow_mut().refNode = None;
            current = n.borrow().nextCalcChainNode.clone();
            self.cellCalculated();
        }
    }
    // java flagsLock sections
    pub fn interrupt(&self) {
        let _f = self.core.flagsLock.lock().unwrap();
        if self.core.inCalculating.get() {
            self.core.interrupting.set(true);
        }
    }
    pub fn interruptMark(&self) {
        let _f = self.core.flagsLock.lock().unwrap();
        self.core.interruptMark.set(true);
    }
    pub fn resumeMark(&self) {
        let _f = self.core.flagsLock.lock().unwrap();
        self.core.interruptMark.set(false);
    }
    pub fn resumeMarkInTransaction(&self) {
        let _f = self.core.flagsLock.lock().unwrap();
        self.core.interruptMark.set(false);
        self.core.needRecalc.set(true);
    }
    pub fn isInterruptMark(&self) -> bool {
        self.core.interruptMark.get()
    }
    pub fn isInterrupting(&self) -> bool {
        self.core.interrupting.get()
    }
    pub fn isInCalculating(&self) -> bool {
        self.core.inCalculating.get()
    }
    // java return inCalculating && calcStartTime>0 && now-calcStartTime>delay;
    pub fn isInCalculating__delay(&self, delay: i64) -> bool {
        self.isInCalculating()
            && self.core.calcStartTime.get() > 0
            && now() - self.core.calcStartTime.get() > delay
    }
    pub fn setCalcType(&self, t: i32) {
        self.core.calcType.set(t);
    }
    pub fn getCalcType(&self) -> i32 {
        self.core.calcType.get()
    }
    pub fn getFormulaSize(&self) -> i32 {
        self.core.links.borrow().fmlIdPos.iterator().count() as i32
    }
    // rust: diagnostic boundary for the next Java class port; never used as a value/cached result.
    pub fn unportedEvaluationBranches(&self) -> Vec<(i32, i32, i32, &'static str)> {
        self.core.evaluationGaps.lock().unwrap().clone()
    }
    // java willSwap/isUndo and didFinishSwapping/!isUndo invoke reverseOp against the current backup.
    fn reverseOp(&self, isUndo: bool) {
        let _lock = recalcLock.lock();
        let (op, fmls) = {
            let a = self.core.atom.borrow();
            (a.rawData().opType, a.rawData().fmls.clone())
        };
        if op == 0 {
            return;
        }
        assert!(op == 1 || op == 2, "IllegalArgumentException");
        for f in fmls {
            let id = CellRefMap::cellPos2Long(f.getSheetUid(), f.getFmlRow(), f.getFmlCol());
            if (op == 1 && isUndo) || (op == 2 && !isUndo) {
                let old = self.core.links.borrow_mut().fmlPos.remove(id);
                if let Some(n) = old {
                    self.removeNode(&n);
                    self.markRecalculationImpl(f.getSheetIndex(), f.getFmlRow(), f.getFmlCol());
                }
                let mut l = self.core.links.borrow_mut();
                if let Some(i) = l.volatileFmls.iter().position(|v| v.equals(f.as_ref())) {
                    l.volatileFmls.remove(i);
                }
            } else {
                let n = Arc::new(RefCell::new(CalcChainNode::CalcChainNode(f.clone())));
                let old = self.core.links.borrow_mut().fmlPos.put(id, n.clone());
                if old.is_none() {
                    self.appendNode(&n);
                    self.markRecalculationImpl(f.getSheetIndex(), f.getFmlRow(), f.getFmlCol());
                }
                if super::FormulaHelper::FormulaHelper::checkVolatile(
                    &f.getTokens(),
                    f.as_ref(),
                    self.core.calculationWorkbook.as_ref(),
                ) {
                    self.core.links.borrow_mut().volatileFmls.push(f);
                }
            }
        }
    }
}
impl SnapshotAtom for CalcChain {
    fn willSwap(&mut self, undo: bool) {
        if undo {
            self.reverseOp(undo);
        }
    }
    fn swapData(&mut self, data: &mut Box<dyn SnapshotData>) {
        self.core.atom.borrow_mut().swapSnapshotData(data);
    }
    fn didFinishSwapping(&mut self, undo: bool) {
        if !undo {
            self.reverseOp(undo);
        }
    }
    fn isNeedSupportNonSequence(&self) -> bool {
        false
    }
    fn cloneAtom(&self) -> SharedAtom {
        panic!("CalcChain non-sequence clone is not used")
    }
}
fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

struct ResumeMark<'a>(&'a CalcChain);
impl Drop for ResumeMark<'_> {
    fn drop(&mut self) {
        self.0.resumeMark();
    }
}

struct HistoryMark<'a>(&'a CalcChain, Arc<dyn MarkType>);
impl Drop for HistoryMark<'_> {
    fn drop(&mut self) {
        self.0.handlerMarkType(self.1.clone());
    }
}
struct MarkAllFinally<'a>(&'a CalcChain);
impl Drop for MarkAllFinally<'_> {
    fn drop(&mut self) {
        let mt: Arc<dyn MarkType> = Arc::new(MarkTypeAll);
        if self.0.isInterruptMark() {
            let mut l = self.0.core.historyMarkList.borrow_mut();
            l.clear();
            l.push(mt);
        } else {
            self.0.handlerMarkType(mt);
        }
    }
}

impl CalcChain{
// java private void clearEvalCache(){AreaNumberCollector.clearCache();CountifCache.clearCache();CountifRankCache.clearCache();LargeSmallCache.clearCache();RankCache.clearCache();VlookupCache.clearCache();HlookupCache.clearCache();}
fn clearEvalCache(&self){use crate::cn::wps::et::ss::formula::evalcache;evalcache::AreaNumberCollector::AreaNumberCollector::clearCache();evalcache::CountifCache::CountifCache::clearCache();evalcache::CountifRankCache::CountifRankCache::clearCache();evalcache::LargeSmallCache::LargeSmallCache::clearCache();
// rust: RANK/VLOOKUP/HLOOKUP functions and their caches are outside the compiled evaluator closure.
}}
