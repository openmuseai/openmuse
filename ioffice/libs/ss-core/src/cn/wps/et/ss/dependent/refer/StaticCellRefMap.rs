// java package cn.wps.et.ss.dependent.refer;

use crate::java_runtime::Rc;

// java import cn.wps.et.ss.calcchain.CalcChain/FormulaHelper;
use crate::cn::wps::et::ss::calcchain::FormulaHelper::FormulaHelper;
// java import cn.wps.et.ss.dependent.refer.base.CellRefMap;
use crate::cn::wps::et::ss::dependent::refer::base::CellRefMap::CellRefMap;
// java import cn.wps.et.ss.formula.ptg.Ptg;
use crate::cn::wps::et::ss::formula::ptg::Ptg::{col_index, Ptg, PtgKind};
// java import cn.wps.et.ss.interfaces.IEvalFormulaBase/IEvalWorkbook;
use crate::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
use crate::cn::wps::et::ss::interfaces::IEvalWorkbook::IEvalWorkbook;
// java import cn.wps.et.ss.util.level.PointLevelMap;
use crate::cn::wps::et::ss::util::level::PointLevelMap::PointLevelMap;
// java import cn.wps.et.ss.util.Area;
use crate::cn::wps::et::ss::util::Area::Area;
// java import cn.wps.moss.engine.eval.EvaluationName;
use crate::cn::wps::moss::engine::eval::EvaluationName::EvaluationName;
// java import org.apache.poi.ss.SpreadsheetVersion;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

use super::FormulaRef::FormulaRef;

// java public final class StaticCellRefMap extends CellRefMap<IEvalFormulaBase>
pub struct StaticCellRefMap {
    base: CellRefMap<FormulaRef>,
}

impl StaticCellRefMap {
    // java public StaticCellRefMap(SpreadsheetVersion sv)
    pub fn StaticCellRefMap(sv: SpreadsheetVersion) -> Self {
        // java super(sv);
        let mut result = Self { base: CellRefMap::CellRefMap(sv) };
        let proxy = Self { base: result.base.snapshot_view() };
        result.base.bindSnapshotOwner(Box::new(proxy));
        result
    }

    // java public void registerFml(IEvalFormulaBase fml, IEvalWorkbook oc, boolean inTrans)
    pub fn registerFml(&mut self, fml: &Rc<dyn IEvalFormulaBase>, oc: &dyn IEvalWorkbook, in_trans: bool) {
        // java this.registerTokens(fml.getTokens(), fml, oc, inTrans);
        self.registerTokens(&fml.getTokens(), fml, oc, in_trans);
    }

    // java public void registerTokens(Ptg[] ptgs, IEvalFormulaBase fml, IEvalWorkbook oc, boolean inTrans)
    pub fn registerTokens(&mut self, ptgs: &[Ptg], fml: &Rc<dyn IEvalFormulaBase>, oc: &dyn IEvalWorkbook, in_trans: bool) {
        // java for (int i = 0, len = ptgs.length; i < len; ++i) switch (ptg.getSid())
        for ptg in ptgs {
            match &ptg.kind {
                // java case 35: expand IEvalName definition with replaceRelativeNamePtgs; recurse without isSame guard.
                PtgKind::Name { index } => {
                    if let Some(name) = oc.getNameByNamePtg(*index) {
                        if name.hasFormula() {
                            let tokens = FormulaHelper::replaceRelativeNamePtgs(
                                &crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), oc.getSpreadsheetVersion());
                            self.registerTokens(&tokens, fml, oc, in_trans);
                        }
                    }
                }
                // java case 36: addToMap(fml.getSheetUid(), rptg.getRow(), rptg.getColumn(), fml, inTrans).
                PtgKind::Ref { row, col_raw, relative: false, .. } => {
                    self.base.addToMap(fml.getSheetUid(), *row, col_index(*col_raw), FormulaRef(Rc::clone(fml)), in_trans);
                }
                // java case 57: try { expand local IEvalName } catch(Throwable e) { IOLog.d(...); }
                PtgKind::NameX { sheet_ref, name_number } => {
                    super::FormulaRef::catch_java_throwable(|| {
                        if let Some(name) = oc.getNameByNameXPtg(*sheet_ref, *name_number - 1) {
                            // java if (aname instanceof IEvalName)
                            if let Some(name) = name.as_any().downcast_ref::<EvaluationName>() {
                                if name.hasFormula() {
                                    let tokens = FormulaHelper::replaceRelativeNamePtgs(
                                        &crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), oc.getSpreadsheetVersion());
                                    self.registerTokens(&tokens, fml, oc, in_trans);
                                }
                            }
                        }
                    });
                }
                // java case 58: try { Ref3DPtg over the external sheet span } catch(Throwable e) { IOLog.d(...); }
                PtgKind::Ref3D { extern_index, row, col_raw, .. } => {
                    super::FormulaRef::catch_java_throwable(|| {
                        if let Some(sheet) = oc.getExtSheet(*extern_index) {
                            // java if (extSheet != null && extSheet.getBookType() != 4)
                            if sheet.getBookType() != 4 {
                                let (first, last) = (sheet.getFirstSheetIndex(), sheet.getLastSheetIndex());
                                // java if (0 <= first && first <= last) for (j = first; j <= last; ++j) addToMap(sheetIndex2SheetUid(j), ...).
                                if 0 <= first && first <= last {
                                    for index in first..=last {
                                        self.base.addToMap(oc.sheetIndex2SheetUid(index), *row, col_index(*col_raw), FormulaRef(Rc::clone(fml)), in_trans);
                                    }
                                // java else if (first == -2) addToMap(fml.getSheetUid(), ...).
                                } else if first == -2 {
                                    self.base.addToMap(fml.getSheetUid(), *row, col_index(*col_raw), FormulaRef(Rc::clone(fml)), in_trans);
                                }
                            }
                        }
                    });
                }
                _ => {}
            }
        }
    }

    // java public void unregisterFml(IEvalFormulaBase fml, IEvalWorkbook oc)
    pub fn unregisterFml(&mut self, fml: &Rc<dyn IEvalFormulaBase>, oc: &dyn IEvalWorkbook) {
        // java this.unregisterTokens(fml.getTokens(), fml, oc);
        self.unregisterTokens(&fml.getTokens(), fml, oc);
    }

    // java void unregisterTokens(Ptg[] ptgs, IEvalFormulaBase fml, IEvalWorkbook oc)
    pub fn unregisterTokens(&mut self, ptgs: &[Ptg], fml: &Rc<dyn IEvalFormulaBase>, oc: &dyn IEvalWorkbook) {
        // java for each ptg, switch (ptg.getSid())
        for ptg in ptgs {
            match &ptg.kind {
                // java case 35: expand name and recursively unregister tokens.
                PtgKind::Name { index } => {
                    if let Some(name) = oc.getNameByNamePtg(*index) {
                        if name.hasFormula() {
                            let tokens = FormulaHelper::replaceRelativeNamePtgs(
                                &crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), oc.getSpreadsheetVersion());
                            self.unregisterTokens(&tokens, fml, oc);
                        }
                    }
                }
                // java case 36: removeFromMap(fml.getSheetUid(), row, col, fml).
                PtgKind::Ref { row, col_raw, relative: false, .. } => {
                    self.base.removeFromMap(fml.getSheetUid(), *row, col_index(*col_raw), &FormulaRef(Rc::clone(fml)));
                }
                // java case 57: resolve IEvalName and recursively unregister; this branch has no try/catch.
                PtgKind::NameX { sheet_ref, name_number } => {
                    if let Some(name) = oc.getNameByNameXPtg(*sheet_ref, *name_number - 1) {
                        if let Some(name) = name.as_any().downcast_ref::<EvaluationName>() {
                            if name.hasFormula() {
                                let tokens = FormulaHelper::replaceRelativeNamePtgs(
                                    &crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), oc.getSpreadsheetVersion());
                                self.unregisterTokens(&tokens, fml, oc);
                            }
                        }
                    }
                }
                // java case 58: remove references for each local sheet or the anonymous current sheet.
                PtgKind::Ref3D { extern_index, row, col_raw, .. } => {
                    if let Some(sheet) = oc.getExtSheet(*extern_index) {
                        if sheet.getBookType() != 4 {
                            let (first, last) = (sheet.getFirstSheetIndex(), sheet.getLastSheetIndex());
                            if 0 <= first && first <= last {
                                for index in first..=last {
                                    self.base.removeFromMap(oc.sheetIndex2SheetUid(index), *row, col_index(*col_raw), &FormulaRef(Rc::clone(fml)));
                                }
                            } else if first == -2 {
                                self.base.removeFromMap(fml.getSheetUid(), *row, col_index(*col_raw), &FormulaRef(Rc::clone(fml)));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // java public void queryReferred(int sheetUid, int row, int col, Collection<IEvalFormulaBase> out)
    pub fn queryReferred(&self, sheet_uid: i32, row: i32, col: i32, out: &mut Vec<Rc<dyn IEvalFormulaBase>>) {
        let mut references = Vec::new();
        self.base.queryReferred(sheet_uid, row, col, &mut references);
        out.extend(references.into_iter().map(|reference| reference.0));
    }

    // java inherited CellRefMap.queryReferred(int sheetUid, Area area, Collection<IEvalFormulaBase> out)
    pub fn queryReferred__area(&self, sheet_uid: i32, area: &Area, out: &mut Vec<Rc<dyn IEvalFormulaBase>>) {
        let mut references = Vec::new();
        self.base.queryReferred__area(sheet_uid, area, &mut references);
        out.extend(references.into_iter().map(|reference| reference.0));
    }

    // java inherited CellRefMap.queryAll(Collection<IEvalFormulaBase> out)
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

    // java public void reverseOp(boolean isUndo, CellRefMap.Backup<IEvalFormulaBase> data)
    fn reverseOp(&mut self, is_undo: bool) {
        // java CalcChain.recalcLock.lock(); try { ... } finally { unlock(); }
        let _lock = crate::cn::wps::et::ss::calcchain::recalcLock.lock();
        let mut base = self.base.state.borrow_mut();
        let sv = base.sv;
        let backup = base.atom.rawData();
        let (op_type, sheet_uid, row, col, data) =
            (backup.opType, backup.sheetUid, backup.row, backup.col, backup.data.clone());
        // java if (data.opType != 0)
        if op_type == 0 { return; }
        // java invalid opType throws IllegalArgumentException.
        assert!(op_type == 1 || op_type == 2, "IllegalArgumentException");
        let data = data.expect("Backup.data");
        // java ADD+undo or DEL+redo removes; DEL+undo or ADD+redo inserts.
        if (op_type == 1 && is_undo) || (op_type == 2 && !is_undo) {
            if let Some(points) = base.points.get_mut(sheet_uid) {
                // java plm.remove(row, col, data.data); data.data.setNeedRecalculationFlag(true);
                points.remove(row, col, &data);
                data.0.setNeedRecalculationFlag(true);
            }
        } else {
            // java if (plm == null) create and put; plm.insert(row, col, data.data); setNeedRecalculationFlag(true).
            if base.points.get(sheet_uid).is_none() {
                base.points.put(sheet_uid, PointLevelMap::PointLevelMap(sv));
            }
            base.points.get_mut(sheet_uid).unwrap().insert(row, col, data.clone());
            data.0.setNeedRecalculationFlag(true);
        }
    }
}

// rust: AtomImage retains the most-derived Java object and dispatches its overrides.
impl kfc::cn::wps::memor4j::AtomSnapshot::KmoAtom for StaticCellRefMap {
    fn willSwap(&mut self, is_undo: bool) { self.willSwap(is_undo); }
    fn swapData(&mut self, data: &mut Box<dyn kfc::cn::wps::memor4j::AtomSnapshot::KmoAtomData>) {
        self.base.state.borrow_mut().atom.swapSnapshotData(data);
    }
    fn didFinishSwapping(&mut self, is_undo: bool) { self.didFinishSwapping(is_undo); }
    fn isNeedSupportNonSequence(&self) -> bool { false }
    fn cloneAtom(&self) -> kfc::cn::wps::memor4j::AtomSnapshot::SharedAtom {
        panic!("non-sequence clone is not used by StaticCellRefMap")
    }
}
