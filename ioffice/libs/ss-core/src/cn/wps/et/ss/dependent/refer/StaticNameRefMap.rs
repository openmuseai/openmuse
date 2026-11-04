// java package cn.wps.et.ss.dependent.refer;

use crate::java_runtime::Rc;

// java import cn.wps.et.ss.calcchain.CalcChain/FormulaHelper;
use crate::cn::wps::et::ss::calcchain::FormulaHelper::FormulaHelper;
// java import cn.wps.et.ss.dependent.refer.base.NameRefMap;
use crate::cn::wps::et::ss::dependent::refer::base::NameRefMap::NameRefMap;
// java import cn.wps.et.ss.formula.ptg.Ptg;
use crate::cn::wps::et::ss::formula::ptg::Ptg::{Ptg, PtgKind};
// java import cn.wps.et.ss.interfaces.IEvalFormulaBase/IEvalWorkbook;
use crate::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
use crate::cn::wps::et::ss::interfaces::IEvalWorkbook::IEvalWorkbook;
// java import cn.wps.moss.engine.eval.EvaluationName;
use crate::cn::wps::moss::engine::eval::EvaluationName::EvaluationName;
// java import org.apache.poi.ss.SpreadsheetVersion;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

use super::FormulaRef::FormulaRef;

// java public class StaticNameRefMap extends NameRefMap<IEvalFormulaBase>
pub struct StaticNameRefMap {
    base: NameRefMap<FormulaRef>,
}

impl StaticNameRefMap {
    // java public StaticNameRefMap(SpreadsheetVersion sv)
    pub fn StaticNameRefMap(sv: SpreadsheetVersion) -> Self {
        // java super(sv);
        let mut result = Self { base: NameRefMap::NameRefMap(sv) };
        let proxy = Self { base: result.base.snapshot_view() };
        result.base.bindSnapshotOwner(Box::new(proxy));
        result
    }

    // java public void registerName(IEvalFormulaBase fml, IEvalWorkbook oc, boolean inTrans)
    pub fn registerName(&mut self, fml: &Rc<dyn IEvalFormulaBase>, oc: &dyn IEvalWorkbook, in_trans: bool) {
        // java this.registerName(fml.getTokens(), fml, oc, inTrans);
        self.registerName__tokens(&fml.getTokens(), fml, oc, in_trans);
    }

    // java public void registerName(Ptg[] ptgs, IEvalFormulaBase fml, IEvalWorkbook oc, boolean inTrans)
    pub fn registerName__tokens(&mut self, ptgs: &[Ptg], fml: &Rc<dyn IEvalFormulaBase>, oc: &dyn IEvalWorkbook, in_trans: bool) {
        // java for (int i = 0, len = ptgs.length; i < len; ++i) switch (ptg.getSid())
        for ptg in ptgs {
            match &ptg.kind {
                // java case 35: addToMap(namePtg.getIndex(), fml, inTrans); then recurse into defined name if not isSame.
                PtgKind::Name { index } => {
                    self.base.addToMap(*index, FormulaRef(Rc::clone(fml)), in_trans);
                    if let Some(name) = oc.getNameByNamePtg(*index) {
                        if name.hasFormula() {
                            let name_ptgs = FormulaHelper::replaceRelativeNamePtgs(
                                &crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), oc.getSpreadsheetVersion());
                            // java if (!FormulaHelper.isSame(namePtgs, ptgs)) this.registerName(namePtgs, fml, oc, inTrans);
                            if !FormulaHelper::isSame(Some(&name_ptgs), Some(ptgs)) {
                                self.registerName__tokens(&name_ptgs, fml, oc, in_trans);
                            }
                        }
                    }
                }
                // java case 57: try { if (bookType != 4 && bookType != 3) add and recurse for IEvalName; } catch(Throwable e) { IOLog.d(...); }
                PtgKind::NameX { sheet_ref, name_number } => {
                    super::FormulaRef::catch_java_throwable(|| {
                        let book_type = oc.getExtSheet(*sheet_ref).map(|sheet| sheet.getBookType()).unwrap_or(2);
                        if book_type != 4 && book_type != 3 {
                            let name_index = *name_number - 1;
                            self.base.addToMap(name_index, FormulaRef(Rc::clone(fml)), in_trans);
                            if let Some(name) = oc.getNameByNameXPtg(*sheet_ref, name_index) {
                                // java if (aname instanceof IEvalName)
                                if let Some(name) = name.as_any().downcast_ref::<EvaluationName>() {
                                    if name.hasFormula() {
                                        let name_ptgs = FormulaHelper::replaceRelativeNamePtgs(
                                            &crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), oc.getSpreadsheetVersion());
                                        if !FormulaHelper::isSame(Some(&name_ptgs), Some(ptgs)) {
                                            self.registerName__tokens(&name_ptgs, fml, oc, in_trans);
                                        }
                                    }
                                }
                            }
                        }
                    });
                }
                // java default: no name edge.
                _ => {}
            }
        }
    }

    // java public void unregisterName(IEvalFormulaBase fml, IEvalWorkbook oc)
    pub fn unregisterName(&mut self, fml: &Rc<dyn IEvalFormulaBase>, oc: &dyn IEvalWorkbook) {
        // java this.unregisterName(fml.getTokens(), fml, oc);
        self.unregisterName__tokens(&fml.getTokens(), fml, oc);
    }

    // java void unregisterName(Ptg[] ptgs, IEvalFormulaBase fml, IEvalWorkbook oc)
    pub fn unregisterName__tokens(&mut self, ptgs: &[Ptg], fml: &Rc<dyn IEvalFormulaBase>, oc: &dyn IEvalWorkbook) {
        // java for each ptg, switch (ptg.getSid())
        for ptg in ptgs {
            match &ptg.kind {
                // java case 35: removeFromMap(namePtg.getIndex(), fml); no recursion.
                PtgKind::Name { index } => { self.base.removeFromMap(*index, &FormulaRef(Rc::clone(fml))); }
                // java case 57: if (bookType != 4 && bookType != 3) removeFromMap(nptg.getNameIndex(), fml).
                PtgKind::NameX { sheet_ref, name_number } => {
                    let book_type = oc.getExtSheet(*sheet_ref).map(|sheet| sheet.getBookType()).unwrap_or(2);
                    if book_type != 4 && book_type != 3 {
                        self.base.removeFromMap(*name_number - 1, &FormulaRef(Rc::clone(fml)));
                    }
                }
                _ => {}
            }
        }
    }

    // java public void queryReferred(int nameNumber, Collection<IEvalFormulaBase> out)
    pub fn queryReferred(&self, name_number: i32, out: &mut Vec<Rc<dyn IEvalFormulaBase>>) {
        let mut references = Vec::new();
        self.base.queryReferred(name_number, &mut references);
        out.extend(references.into_iter().map(|reference| reference.0));
    }

    // java inherited NameRefMap.queryAll(Collection<IEvalFormulaBase> out)
    pub fn queryAll(&self, out: &mut Vec<Rc<dyn IEvalFormulaBase>>) {
        let mut references = Vec::new();
        self.base.queryAll(&mut references);
        out.extend(references.into_iter().map(|reference| reference.0));
    }

    // java public void willSwap(boolean isUndo)
    pub fn willSwap(&mut self, is_undo: bool) {
        // java if (isUndo) reverseOp(isUndo, (Backup)rawData());
        if is_undo { self.reverseOp(is_undo); }
    }

    // java public void didFinishSwapping(boolean isUndo)
    pub fn didFinishSwapping(&mut self, is_undo: bool) {
        // java if (!isUndo) reverseOp(isUndo, (Backup)rawData());
        if !is_undo { self.reverseOp(is_undo); }
    }

    // java public void reverseOp(boolean isUndo, NameRefMap.Backup<IEvalFormulaBase> data)
    fn reverseOp(&mut self, is_undo: bool) {
        // java CalcChain.recalcLock.lock(); try { ... } finally { unlock(); }
        let _lock = crate::cn::wps::et::ss::calcchain::recalcLock.lock();
        let mut base = self.base.state.borrow_mut();
        let backup = base.atom.rawData();
        let (op_type, name_number, data) = (backup.opType, backup.nameNumber, backup.data.clone());
        // java if (data.opType != 0)
        if op_type == 0 { return; }
        // java invalid opType throws IllegalArgumentException.
        assert!(op_type == 1 || op_type == 2, "IllegalArgumentException");
        let data = data.expect("Backup.data");
        // java ADD+undo or DEL+redo removes, otherwise inserts.
        if (op_type == 1 && is_undo) || (op_type == 2 && !is_undo) {
            // java if (objects != null) { objects.remove(data.data); data.data.setNeedRecalculationFlag(true); }
            if let Some(objects) = base.map.get_mut(name_number as i64) {
                if let Some(index) = objects.iter().position(|item| &data == item) { objects.remove(index); }
                data.0.setNeedRecalculationFlag(true);
            }
        } else {
            // java if (objects == null) create; objects.add(data.data); setNeedRecalculationFlag(true).
            if base.map.get(name_number as i64).is_none() { base.map.put(name_number as i64, Vec::new()); }
            base.map.get_mut(name_number as i64).unwrap().push(data.clone());
            data.0.setNeedRecalculationFlag(true);
        }
    }
}

// rust: AtomImage retains the most-derived Java object and dispatches its overrides.
impl kfc::cn::wps::memor4j::AtomSnapshot::KmoAtom for StaticNameRefMap {
    fn willSwap(&mut self, is_undo: bool) { self.willSwap(is_undo); }
    fn swapData(&mut self, data: &mut Box<dyn kfc::cn::wps::memor4j::AtomSnapshot::KmoAtomData>) {
        self.base.state.borrow_mut().atom.swapSnapshotData(data);
    }
    fn didFinishSwapping(&mut self, is_undo: bool) { self.didFinishSwapping(is_undo); }
    fn isNeedSupportNonSequence(&self) -> bool { false }
    fn cloneAtom(&self) -> kfc::cn::wps::memor4j::AtomSnapshot::SharedAtom {
        panic!("non-sequence clone is not used by StaticNameRefMap")
    }
}
