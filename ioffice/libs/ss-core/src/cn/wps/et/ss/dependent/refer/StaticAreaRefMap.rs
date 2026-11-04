// java package cn.wps.et.ss.dependent.refer;

use crate::java_runtime::Rc;

// java import cn.wps.et.ss.calcchain.CalcChain/FormulaHelper;
use crate::cn::wps::et::ss::calcchain::FormulaHelper::FormulaHelper;
// java import cn.wps.et.ss.dependent.refer.base.AreaRefMap;
use crate::cn::wps::et::ss::dependent::refer::base::AreaRefMap::AreaRefMap;
// java import cn.wps.et.ss.formula.ptg.Ptg;
use crate::cn::wps::et::ss::formula::ptg::Ptg::{col_index, Ptg, PtgKind};
// java import cn.wps.et.ss.interfaces.IEvalFormulaBase/IEvalWorkbook;
use crate::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
use crate::cn::wps::et::ss::interfaces::IEvalWorkbook::IEvalWorkbook;
// java import cn.wps.et.ss.util.Area;
use crate::cn::wps::et::ss::util::Area::Area;
// java import cn.wps.et.ss.util.level.LineLevelMap/RectLevelMap;
use crate::cn::wps::et::ss::util::level::LineLevelMap::LineLevelMap;
use crate::cn::wps::et::ss::util::level::RectLevelMap::RectLevelMap;
// java import cn.wps.moss.engine.eval.EvaluationName;
use crate::cn::wps::moss::engine::eval::EvaluationName::EvaluationName;
// java import org.apache.poi.ss.SpreadsheetVersion;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

use super::FormulaRef::FormulaRef;

// java public final class StaticAreaRefMap extends AreaRefMap<IEvalFormulaBase>
pub struct StaticAreaRefMap {
    base: AreaRefMap<FormulaRef>,
}

impl StaticAreaRefMap {
    // java public StaticAreaRefMap(SpreadsheetVersion sv)
    pub fn StaticAreaRefMap(sv: SpreadsheetVersion) -> Self {
        // java super(sv);
        let mut result = Self { base: AreaRefMap::AreaRefMap(sv) };
        let proxy = Self { base: result.base.snapshot_view() };
        result.base.bindSnapshotOwner(Box::new(proxy));
        result
    }

    // java public void registerFml(IEvalFormulaBase fml, IEvalWorkbook oc, boolean inTrans)
    pub fn registerFml(&mut self, fml: &Rc<dyn IEvalFormulaBase>, oc: &dyn IEvalWorkbook, in_trans: bool) {
        // java this.registerTokens(fml.getTokens(), fml, oc, inTrans);
        self.registerTokens(&fml.getTokens(), fml, oc, in_trans);
    }

    // java public void registerTokens(Ptg[] ptgs, IEvalFormulaBase fml, IEvalWorkbook cc, boolean inTrans)
    pub fn registerTokens(&mut self, ptgs: &[Ptg], fml: &Rc<dyn IEvalFormulaBase>, cc: &dyn IEvalWorkbook, in_trans: bool) {
        // java for (int i = 0, len = ptgs.length; i < len; ++i) switch (ptg.getSid())
        for ptg in ptgs {
            match &ptg.kind {
                // java case 35: expand IEvalName definition with replaceRelativeNamePtgs; recurse.
                PtgKind::Name { index } => {
                    if let Some(name) = cc.getNameByNamePtg(*index) {
                        if name.hasFormula() {
                            let tokens = FormulaHelper::replaceRelativeNamePtgs(
                                &crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), cc.getSpreadsheetVersion());
                            self.registerTokens(&tokens, fml, cc, in_trans);
                        }
                    }
                }
                // java case 37: new Area(firstRow,lastRow,firstColumn,lastColumn); addToMap(fml.getSheetUid(), area, fml, inTrans).
                PtgKind::Area { first_row, last_row, first_col_raw, last_col_raw, relative: false, .. } => {
                    let area = Area::Area(*first_row, *last_row, col_index(*first_col_raw), col_index(*last_col_raw));
                    self.base.addToMap(fml.getSheetUid(), &area, FormulaRef(Rc::clone(fml)), in_trans);
                }
                // java case 57: try { expand local IEvalName } catch(Throwable e) { IOLog.d(...); }
                PtgKind::NameX { sheet_ref, name_number } => {
                    super::FormulaRef::catch_java_throwable(|| {
                        if let Some(name) = cc.getNameByNameXPtg(*sheet_ref, *name_number - 1) {
                            if let Some(name) = name.as_any().downcast_ref::<EvaluationName>() {
                                if name.hasFormula() {
                                    let tokens = FormulaHelper::replaceRelativeNamePtgs(
                                        &crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), cc.getSpreadsheetVersion());
                                    self.registerTokens(&tokens, fml, cc, in_trans);
                                }
                            }
                        }
                    });
                }
                // java case 59: try { Area3DPtg over the external sheet span } catch(Throwable e) { IOLog.d(...); }
                PtgKind::Area3D { extern_index, first_row, last_row, first_col_raw, last_col_raw, .. } => {
                    super::FormulaRef::catch_java_throwable(|| {
                        if let Some(sheet) = cc.getExtSheet(*extern_index) {
                            // java if (extSheet != null && extSheet.getBookType() != 4)
                            if sheet.getBookType() != 4 {
                                let area = Area::Area(*first_row, *last_row, col_index(*first_col_raw), col_index(*last_col_raw));
                                let (first, last) = (sheet.getFirstSheetIndex(), sheet.getLastSheetIndex());
                                // java if (0 <= first && first <= last) for each sheet addToMap(sheetIndex2SheetUid(j), area, fml, inTrans).
                                if 0 <= first && first <= last {
                                    for index in first..=last {
                                        self.base.addToMap(cc.sheetIndex2SheetUid(index), &area, FormulaRef(Rc::clone(fml)), in_trans);
                                    }
                                // java else if (first == -2) addToMap(fml.getSheetUid(), area, fml, inTrans).
                                } else if first == -2 {
                                    self.base.addToMap(fml.getSheetUid(), &area, FormulaRef(Rc::clone(fml)), in_trans);
                                }
                            }
                        }
                    });
                }
                _ => {}
            }
        }
    }

    // java public void unregisterFml(IEvalFormulaBase fml, IEvalWorkbook cc)
    pub fn unregisterFml(&mut self, fml: &Rc<dyn IEvalFormulaBase>, cc: &dyn IEvalWorkbook) {
        // java this.unregisterTokens(fml.getTokens(), fml, cc);
        self.unregisterTokens(&fml.getTokens(), fml, cc);
    }

    // java void unregisterTokens(Ptg[] ptgs, IEvalFormulaBase fml, IEvalWorkbook cc)
    pub fn unregisterTokens(&mut self, ptgs: &[Ptg], fml: &Rc<dyn IEvalFormulaBase>, cc: &dyn IEvalWorkbook) {
        // java for each ptg, switch (ptg.getSid())
        for ptg in ptgs {
            match &ptg.kind {
                // java case 35: expand name definition and recursively unregister.
                PtgKind::Name { index } => {
                    if let Some(name) = cc.getNameByNamePtg(*index) {
                        if name.hasFormula() {
                            let tokens = FormulaHelper::replaceRelativeNamePtgs(
                                &crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), cc.getSpreadsheetVersion());
                            self.unregisterTokens(&tokens, fml, cc);
                        }
                    }
                }
                // java case 37: new Area(...); removeFromMap(fml.getSheetUid(), area, fml).
                PtgKind::Area { first_row, last_row, first_col_raw, last_col_raw, relative: false, .. } => {
                    let area = Area::Area(*first_row, *last_row, col_index(*first_col_raw), col_index(*last_col_raw));
                    self.base.removeFromMap(fml.getSheetUid(), &area, &FormulaRef(Rc::clone(fml)));
                }
                // java case 57: resolve local IEvalName and recursively unregister; no try/catch.
                PtgKind::NameX { sheet_ref, name_number } => {
                    if let Some(name) = cc.getNameByNameXPtg(*sheet_ref, *name_number - 1) {
                        if let Some(name) = name.as_any().downcast_ref::<EvaluationName>() {
                            if name.hasFormula() {
                                let tokens = FormulaHelper::replaceRelativeNamePtgs(
                                    &crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), cc.getSpreadsheetVersion());
                                self.unregisterTokens(&tokens, fml, cc);
                            }
                        }
                    }
                }
                // java case 59: remove area references over each local sheet or anonymous current sheet.
                PtgKind::Area3D { extern_index, first_row, last_row, first_col_raw, last_col_raw, .. } => {
                    if let Some(sheet) = cc.getExtSheet(*extern_index) {
                        if sheet.getBookType() != 4 {
                            let area = Area::Area(*first_row, *last_row, col_index(*first_col_raw), col_index(*last_col_raw));
                            let (first, last) = (sheet.getFirstSheetIndex(), sheet.getLastSheetIndex());
                            if 0 <= first && first <= last {
                                for index in first..=last {
                                    self.base.removeFromMap(cc.sheetIndex2SheetUid(index), &area, &FormulaRef(Rc::clone(fml)));
                                }
                            } else if first == -2 {
                                self.base.removeFromMap(fml.getSheetUid(), &area, &FormulaRef(Rc::clone(fml)));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // java public void queryAreaReferred(int sheetUid, int row, int col, Collection<IEvalFormulaBase> out)
    pub fn queryAreaReferred(&self, sheet_uid: i32, row: i32, col: i32, out: &mut Vec<Rc<dyn IEvalFormulaBase>>) {
        let mut references = Vec::new();
        self.base.queryAreaReferred(sheet_uid, row, col, &mut references);
        out.extend(references.into_iter().map(|reference| reference.0));
    }

    // java inherited AreaRefMap.queryAreaReferred(int sheetUid, Area area, Collection<IEvalFormulaBase> out)
    pub fn queryAreaReferred__area(&self, sheet_uid: i32, area: &Area, out: &mut Vec<Rc<dyn IEvalFormulaBase>>) {
        let mut references = Vec::new();
        self.base.queryAreaReferred__area(sheet_uid, area, &mut references);
        out.extend(references.into_iter().map(|reference| reference.0));
    }

    // java inherited AreaRefMap.queryAll(Collection<IEvalFormulaBase> out)
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

    // java public void reverseOp(boolean isUndo, AreaRefMap.Backup<IEvalFormulaBase> data)
    fn reverseOp(&mut self, is_undo: bool) {
        // java CalcChain.recalcLock.lock(); try { ... } finally { unlock(); }
        let _lock = crate::cn::wps::et::ss::calcchain::recalcLock.lock();
        let mut base = self.base.state.borrow_mut();
        let sv = base.sv;
        let backup = base.atom.rawData();
        let (op_type, sheet_uid, first_row, last_row, first_col, last_col, data) =
            (backup.opType, backup.sheetUid, backup.firstRow, backup.lastRow,
             backup.firstCol, backup.lastCol, backup.data.clone());
        // java if (data.opType != 0)
        if op_type == 0 { return; }
        // java invalid opType throws IllegalArgumentException.
        assert!(op_type == 1 || op_type == 2, "IllegalArgumentException");
        let data = data.expect("Backup.data");
        let remove = (op_type == 1 && is_undo) || (op_type == 2 && !is_undo);
        // java firstRow == lastRow precedes firstCol == lastCol.
        if first_row == last_row {
            if remove {
                if let Some(lines) = base.lines.get_mut(sheet_uid) {
                    // java removeHorizontalLine(...); setNeedRecalculationFlag(true).
                    lines.removeHorizontalLine(first_row, first_col, last_col, &data);
                    data.0.setNeedRecalculationFlag(true);
                }
            } else {
                // java create LineLevelMap if absent; insertHorizontalLine(...); setNeedRecalculationFlag(true).
                if base.lines.get(sheet_uid).is_none() { base.lines.put(sheet_uid, LineLevelMap::LineLevelMap(sv)); }
                base.lines.get_mut(sheet_uid).unwrap().insertHorizontalLine(first_row, first_col, last_col, data.clone());
                data.0.setNeedRecalculationFlag(true);
            }
        } else if first_col == last_col {
            if remove {
                if let Some(lines) = base.lines.get_mut(sheet_uid) {
                    // java removeVerticalLine(...); setNeedRecalculationFlag(true).
                    lines.removeVerticalLine(first_row, last_row, first_col, &data);
                    data.0.setNeedRecalculationFlag(true);
                }
            } else {
                // java create LineLevelMap if absent; insertVerticalLine(...); setNeedRecalculationFlag(true).
                if base.lines.get(sheet_uid).is_none() { base.lines.put(sheet_uid, LineLevelMap::LineLevelMap(sv)); }
                base.lines.get_mut(sheet_uid).unwrap().insertVerticalLine(first_row, last_row, first_col, data.clone());
                data.0.setNeedRecalculationFlag(true);
            }
        } else if remove {
            if let Some(rects) = base.rects.get_mut(sheet_uid) {
                // java rlm.remove(...); setNeedRecalculationFlag(true).
                rects.remove(first_row, last_row, first_col, last_col, &data);
                data.0.setNeedRecalculationFlag(true);
            }
        } else {
            // java create RectLevelMap if absent; insert(...); setNeedRecalculationFlag(true).
            if base.rects.get(sheet_uid).is_none() { base.rects.put(sheet_uid, RectLevelMap::RectLevelMap(sv)); }
            base.rects.get_mut(sheet_uid).unwrap().insert(first_row, last_row, first_col, last_col, data.clone());
            data.0.setNeedRecalculationFlag(true);
        }
    }
}

// rust: AtomImage retains the most-derived Java object and dispatches its overrides.
impl kfc::cn::wps::memor4j::AtomSnapshot::KmoAtom for StaticAreaRefMap {
    fn willSwap(&mut self, is_undo: bool) { self.willSwap(is_undo); }
    fn swapData(&mut self, data: &mut Box<dyn kfc::cn::wps::memor4j::AtomSnapshot::KmoAtomData>) {
        self.base.state.borrow_mut().atom.swapSnapshotData(data);
    }
    fn didFinishSwapping(&mut self, is_undo: bool) { self.didFinishSwapping(is_undo); }
    fn isNeedSupportNonSequence(&self) -> bool { false }
    fn cloneAtom(&self) -> kfc::cn::wps::memor4j::AtomSnapshot::SharedAtom {
        panic!("non-sequence clone is not used by StaticAreaRefMap")
    }
}
