// java package cn.wps.et.ss.dependent;

use crate::java_runtime::Rc;
use crate::cn::wps::et::ss::calcchain::FormulaHelper::FormulaHelper;
use crate::cn::wps::et::ss::dependent::refer::base::AreaRefMap::AreaRefMap;
use crate::cn::wps::et::ss::dependent::refer::base::CellRefMap::CellRefMap;
use crate::cn::wps::et::ss::formula::ptg::Ptg::{col_index, col_rel, row_rel, Ptg, PtgKind};
use crate::cn::wps::et::ss::interfaces::IEvalSharedFormula::IEvalSharedFormula;
use crate::cn::wps::et::ss::interfaces::IEvalWorkbook::IEvalWorkbook;
use crate::cn::wps::et::ss::util::Area::Area;
use crate::cn::wps::moss::engine::eval::EvaluationName::EvaluationName;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;
use super::SharedFmlReferredArea::SharedFmlReferredArea;

// rust: Java CellRefMap uses IEvalSharedFormula.equals when removing a list entry.
#[derive(Clone)]
struct SharedFormulaRef(Rc<dyn IEvalSharedFormula>);

impl PartialEq for SharedFormulaRef {
    fn eq(&self, other: &Self) -> bool { self.0.equals(other.0.as_ref()) }
}

// java private static class SharedReference
#[derive(Clone)]
struct SharedReference {
    // java final Ptg ptg;
    ptg: Ptg,
    // java final IEvalSharedFormula fml;
    fml: Rc<dyn IEvalSharedFormula>,
    // java final Area referred;
    referred: Area,
}

impl PartialEq for SharedReference {
    // java SharedReference.equals compares fml, referred, then Ptg.equals.
    fn eq(&self, other: &Self) -> bool {
        self.fml.equals(other.fml.as_ref()) && self.referred == other.referred
            && Self::ptg_equals(&self.ptg, &other.ptg)
    }
}

impl SharedReference {
    // java RefPtg/Ref3DPtg/AreaPtg/Area3DPtg.equals compare semantic coordinates,
    // relative flags, operand class, and the 3D external sheet index.
    fn ptg_equals(left: &Ptg, right: &Ptg) -> bool {
        if left.class != right.class { return false; }
        match (&left.kind, &right.kind) {
            (PtgKind::Ref { row: ar, col_raw: ac, .. }, PtgKind::Ref { row: br, col_raw: bc, .. }) => ar == br && ac == bc,
            (PtgKind::Ref3D { extern_index: ai, row: ar, col_raw: ac, .. }, PtgKind::Ref3D { extern_index: bi, row: br, col_raw: bc, .. }) => ai == bi && ar == br && ac == bc,
            (PtgKind::Area { first_row: ar1, last_row: ar2, first_col_raw: ac1, last_col_raw: ac2, .. }, PtgKind::Area { first_row: br1, last_row: br2, first_col_raw: bc1, last_col_raw: bc2, .. }) => ar1 == br1 && ar2 == br2 && ac1 == bc1 && ac2 == bc2,
            (PtgKind::Area3D { extern_index: ai, first_row: ar1, last_row: ar2, first_col_raw: ac1, last_col_raw: ac2, .. }, PtgKind::Area3D { extern_index: bi, first_row: br1, last_row: br2, first_col_raw: bc1, last_col_raw: bc2, .. }) => ai == bi && ar1 == br1 && ar2 == br2 && ac1 == bc1 && ac2 == bc2,
            _ => false,
        }
    }
}

// java public class SharedFmlDependent
pub struct SharedFmlDependent {
    // java private final CellRefMap<IEvalSharedFormula> cellRefMap;
    cellRefMap: CellRefMap<SharedFormulaRef>,
    // java private final AreaRefMap<SharedReference> areaRefMap;
    areaRefMap: AreaRefMap<SharedReference>,
    // java private final SpreadsheetVersion sv;
    sv: SpreadsheetVersion,
}

impl SharedFmlDependent {
    // java public SharedFmlDependent(SpreadsheetVersion sv)
    pub fn SharedFmlDependent(sv: SpreadsheetVersion) -> Self {
        Self { cellRefMap: CellRefMap::CellRefMap(sv), areaRefMap: AreaRefMap::AreaRefMap(sv), sv }
    }

    // java public void registerFml(IEvalSharedFormula fml, IEvalWorkbook cw, boolean inTrans)
    pub fn registerFml(&mut self, fml: &Rc<dyn IEvalSharedFormula>, cw: &dyn IEvalWorkbook, in_trans: bool) {
        self.registerTokens(&fml.getTokens(), fml, cw, in_trans);
    }

    // java public void registerTokens(Ptg[] ptgs, IEvalSharedFormula fml, IEvalWorkbook cw, boolean inTrans)
    pub fn registerTokens(&mut self, ptgs: &[Ptg], fml: &Rc<dyn IEvalSharedFormula>, cw: &dyn IEvalWorkbook, in_trans: bool) {
        // java a shared formula spanning just one cell throws RuntimeException.
        assert!(fml.getFmlRow() != fml.getFmlLastRow() || fml.getFmlCol() != fml.getFmlLastCol());
        for ptg in ptgs {
            match &ptg.kind {
                // java case 35: expand local name and recurse.
                PtgKind::Name { index } => {
                    if let Some(name) = cw.getNameByNamePtg(*index) {
                        if name.hasFormula() {
                            let tokens = FormulaHelper::replaceRelativeNamePtgs(&crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), cw.getSpreadsheetVersion());
                            self.registerTokens(&tokens, fml, cw, in_trans);
                        }
                    }
                }
                // java case 36: registerSingleCellRef on the formula sheet.
                PtgKind::Ref { relative: false, .. } => self.registerSingleCellRef(fml.getSheetUid(), ptg, fml, in_trans),
                // java case 37: registerAreaRef on the formula sheet.
                PtgKind::Area { relative: false, .. } => self.registerAreaRef(fml.getSheetUid(), ptg, fml, in_trans),
                // java case 57: expand IEvalName resolved by NameXPtg.
                PtgKind::NameX { sheet_ref, name_number } => {
                    if let Some(name) = cw.getNameByNameXPtg(*sheet_ref, *name_number - 1) {
                        if let Some(name) = name.as_any().downcast_ref::<EvaluationName>() {
                            if name.hasFormula() {
                                let tokens = FormulaHelper::replaceRelativeNamePtgs(&crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), cw.getSpreadsheetVersion());
                                self.registerTokens(&tokens, fml, cw, in_trans);
                            }
                        }
                    }
                }
                // java case 58: skip Ref3DExtPtg, then use local sheetIndex2SheetUid.
                PtgKind::Ref3DExt { .. } => {}
                PtgKind::Ref3D { extern_index, .. } => {
                    if let Some(sheet) = cw.getExtSheet(*extern_index) {
                        if sheet.getBookType() != 4 {
                            let (first, last) = (sheet.getFirstSheetIndex(), sheet.getLastSheetIndex());
                            if 0 <= first && first <= last {
                                for index in first..=last { self.registerSingleCellRef(cw.sheetIndex2SheetUid(index), ptg, fml, in_trans); }
                            } else if first == -2 { self.registerSingleCellRef(fml.getSheetUid(), ptg, fml, in_trans); }
                        }
                    }
                }
                // java case 59: skip Area3DExtPtg; local Area3DPtg uses the sheet index directly.
                PtgKind::Area3DExt { .. } => {}
                PtgKind::Area3D { extern_index, .. } => {
                    if let Some(sheet) = cw.getExtSheet(*extern_index) {
                        if sheet.getBookType() != 4 {
                            let (first, last) = (sheet.getFirstSheetIndex(), sheet.getLastSheetIndex());
                            if 0 <= first && first <= last {
                                for index in first..=last { self.registerAreaRef(index, ptg, fml, in_trans); }
                            } else if first == -2 { self.registerAreaRef(fml.getSheetUid(), ptg, fml, in_trans); }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // java public void unregisterFml(IEvalSharedFormula fml, IEvalWorkbook cw)
    pub fn unregisterFml(&mut self, fml: &Rc<dyn IEvalSharedFormula>, cw: &dyn IEvalWorkbook) {
        assert!(fml.getFmlRow() != fml.getFmlLastRow() || fml.getFmlCol() != fml.getFmlLastCol());
        self.unregisterTokens(&fml.getTokens(), fml, cw);
    }

    // java private void unregisterTokens(Ptg[] ptgs, IEvalSharedFormula fml, IEvalWorkbook cw)
    fn unregisterTokens(&mut self, ptgs: &[Ptg], fml: &Rc<dyn IEvalSharedFormula>, cw: &dyn IEvalWorkbook) {
        for ptg in ptgs {
            match &ptg.kind {
                PtgKind::Name { index } => {
                    if let Some(name) = cw.getNameByNamePtg(*index) {
                        if name.hasFormula() {
                            let tokens = FormulaHelper::replaceRelativeNamePtgs(&crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), cw.getSpreadsheetVersion());
                            self.unregisterTokens(&tokens, fml, cw);
                        }
                    }
                }
                PtgKind::Ref { relative: false, .. } => self.unregisterSingleCellRef(fml.getSheetUid(), ptg, fml),
                PtgKind::Area { relative: false, .. } => self.unregisterAreaRef(fml.getSheetUid(), ptg, fml),
                PtgKind::NameX { sheet_ref, name_number } => {
                    if let Some(name) = cw.getNameByNameXPtg(*sheet_ref, *name_number - 1) {
                        if let Some(name) = name.as_any().downcast_ref::<EvaluationName>() {
                            if name.hasFormula() {
                                let tokens = FormulaHelper::replaceRelativeNamePtgs(&crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), cw.getSpreadsheetVersion());
                                self.unregisterTokens(&tokens, fml, cw);
                            }
                        }
                    }
                }
                // java case 58: 直接强转 Ref3DPtg；Ref3DExtPtg 在此抛 ClassCastException。
                PtgKind::Ref3DExt { .. } => panic!("ClassCastException: Ref3DExtPtg is not Ref3DPtg"),
                PtgKind::Ref3D { extern_index, .. } => {
                    if let Some(sheet) = cw.getExtSheet(*extern_index) {
                        if sheet.getBookType() != 4 {
                            let (first, last) = (sheet.getFirstSheetIndex(), sheet.getLastSheetIndex());
                            if 0 <= first && first <= last {
                                for index in first..=last { self.unregisterSingleCellRef(cw.sheetIndex2SheetUid(index), ptg, fml); }
                            } else if first == -2 { self.unregisterSingleCellRef(fml.getSheetUid(), ptg, fml); }
                        }
                    }
                }
                // java case 59: 直接强转 Area3DPtg；Area3DExtPtg 在此抛 ClassCastException。
                PtgKind::Area3DExt { .. } => panic!("ClassCastException: Area3DExtPtg is not Area3DPtg"),
                PtgKind::Area3D { extern_index, .. } => {
                    if let Some(sheet) = cw.getExtSheet(*extern_index) {
                        if sheet.getBookType() != 4 {
                            let (first, last) = (sheet.getFirstSheetIndex(), sheet.getLastSheetIndex());
                            if 0 <= first && first <= last {
                                for index in first..=last { self.unregisterAreaRef(index, ptg, fml); }
                            } else if first == -2 { self.unregisterAreaRef(fml.getSheetUid(), ptg, fml); }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // java private void registerSingleCellRef / unregisterSingleCellRef
    fn registerSingleCellRef(&mut self, sheet_uid: i32, ptg: &Ptg, fml: &Rc<dyn IEvalSharedFormula>, in_trans: bool) {
        self.changeSingleCellRef(sheet_uid, ptg, fml, Some(in_trans));
    }
    fn unregisterSingleCellRef(&mut self, sheet_uid: i32, ptg: &Ptg, fml: &Rc<dyn IEvalSharedFormula>) {
        self.changeSingleCellRef(sheet_uid, ptg, fml, None);
    }

    // rust: keeps the register/unregister branches paired in Java order.
    fn changeSingleCellRef(&mut self, sheet_uid: i32, ptg: &Ptg, fml: &Rc<dyn IEvalSharedFormula>, in_trans: Option<bool>) {
        let (row, col_raw) = match ptg.kind {
            PtgKind::Ref { row, col_raw, .. } | PtgKind::Ref3D { row, col_raw, .. } => (row, col_raw),
            _ => unreachable!(),
        };
        let col = col_index(col_raw);
        let (row_relative, col_relative) = (row_rel(col_raw), col_rel(col_raw));
        // java both absolute, or an axis with no shared span, uses CellRefMap.
        if (!row_relative && !col_relative)
            || (!row_relative && fml.getFmlCol() == fml.getFmlLastCol())
            || (!col_relative && fml.getFmlRow() == fml.getFmlLastRow()) {
            let value = SharedFormulaRef(Rc::clone(fml));
            if let Some(in_trans) = in_trans { self.cellRefMap.addToMap(sheet_uid, row, col, value, in_trans); }
            else { self.cellRefMap.removeFromMap(sheet_uid, row, col, &value); }
            return;
        }
        // java relative axes expand over the shared formula range, wrapping at sv bounds.
        let last_row = if row_relative { row + fml.getFmlLastRow() - fml.getFmlRow() } else { row };
        let last_col = if col_relative { col + fml.getFmlLastCol() - fml.getFmlCol() } else { col };
        for referred in self.referredAreas(row, last_row, col, last_col) {
            self.changeAreaEntry(sheet_uid, ptg, fml, referred, in_trans);
        }
    }

    // java private void registerAreaRef / unregisterAreaRef
    fn registerAreaRef(&mut self, sheet_uid: i32, ptg: &Ptg, fml: &Rc<dyn IEvalSharedFormula>, in_trans: bool) {
        self.changeAreaRef(sheet_uid, ptg, fml, Some(in_trans));
    }
    fn unregisterAreaRef(&mut self, sheet_uid: i32, ptg: &Ptg, fml: &Rc<dyn IEvalSharedFormula>) {
        self.changeAreaRef(sheet_uid, ptg, fml, None);
    }

    fn changeAreaRef(&mut self, sheet_uid: i32, ptg: &Ptg, fml: &Rc<dyn IEvalSharedFormula>, in_trans: Option<bool>) {
        let (first_row, last_row, first_col_raw, last_col_raw) = match ptg.kind {
            PtgKind::Area { first_row, last_row, first_col_raw, last_col_raw, .. }
            | PtgKind::Area3D { first_row, last_row, first_col_raw, last_col_raw, .. } => (first_row, last_row, first_col_raw, last_col_raw),
            _ => unreachable!(),
        };
        let (first_col, last_col) = (col_index(first_col_raw), col_index(last_col_raw));
        // java Math.max of the first/last endpoint after applying each relative flag.
        let row_span = fml.getFmlLastRow() - fml.getFmlRow();
        let col_span = fml.getFmlLastCol() - fml.getFmlCol();
        let end_row = (first_row + if row_rel(first_col_raw) { row_span } else { 0 })
            .max(last_row + if row_rel(last_col_raw) { row_span } else { 0 });
        let end_col = (first_col + if col_rel(first_col_raw) { col_span } else { 0 })
            .max(last_col + if col_rel(last_col_raw) { col_span } else { 0 });
        for referred in self.referredAreas(first_row, end_row, first_col, end_col) {
            self.changeAreaEntry(sheet_uid, ptg, fml, referred, in_trans);
        }
    }

    // java registerSingleCellRef/registerAreaRef split an axis at its spreadsheet limit.
    fn splitAxis(&self, first: i32, last: i32, last_index: i32, max: i32) -> Vec<(i32, i32)> {
        if last <= last_index { vec![(first, last)] }
        else { vec![(first, last_index), (0, last - max)] }
    }

    // java when both axes wrap, insert upper-left, lower-right, upper-right, lower-left.
    fn referredAreas(&self, first_row: i32, last_row: i32, first_col: i32, last_col: i32) -> Vec<Area> {
        let rows = self.splitAxis(first_row, last_row, self.sv.getLastRowIndex(), self.sv.getMaxRows());
        let cols = self.splitAxis(first_col, last_col, self.sv.getLastColumnIndex(), self.sv.getMaxColumns());
        let mut out = Vec::with_capacity(rows.len() * cols.len());
        let mut add = |r: usize, c: usize| out.push(Area::Area(rows[r].0, rows[r].1, cols[c].0, cols[c].1));
        add(0, 0);
        if rows.len() == 2 && cols.len() == 2 { add(1, 1); add(0, 1); add(1, 0); }
        else if rows.len() == 2 { add(1, 0); }
        else if cols.len() == 2 { add(0, 1); }
        out
    }

    fn changeAreaEntry(&mut self, sheet_uid: i32, ptg: &Ptg, fml: &Rc<dyn IEvalSharedFormula>, referred: Area, in_trans: Option<bool>) {
        let value = SharedReference { ptg: ptg.clone(), fml: Rc::clone(fml), referred: referred.clone() };
        if let Some(in_trans) = in_trans { self.areaRefMap.addToMap(sheet_uid, &referred, value, in_trans); }
        else { self.areaRefMap.removeFromMap(sheet_uid, &referred, &value); }
    }

    // java private static void convertRS1(ArrayList<IEvalSharedFormula> in, Collection<SharedFmlReferredArea> out)
    fn convertRS1(input: Vec<SharedFormulaRef>, out: &mut Vec<SharedFmlReferredArea>) {
        for SharedFormulaRef(fml) in input {
            let marked = Area::Area(fml.getFmlRow(), fml.getFmlLastRow(), fml.getFmlCol(), fml.getFmlLastCol());
            out.push(SharedFmlReferredArea::SharedFmlReferredArea(fml, marked));
        }
    }

    // java public void queryStaticReferred(int sheetUid, int row, int col, Collection out)
    pub fn queryStaticReferred(&self, sheet_uid: i32, row: i32, col: i32, out: &mut Vec<SharedFmlReferredArea>) {
        let mut cells = Vec::new();
        self.cellRefMap.queryReferred(sheet_uid, row, col, &mut cells);
        Self::convertRS1(cells, out);
        let mut areas = Vec::new();
        self.areaRefMap.queryAreaReferred(sheet_uid, row, col, &mut areas);
        self.convertRS2(areas, out, &Area::Area(row, row, col, col));
    }

    // java public void queryStaticReferred(int sheetUid, Area area, Collection out)
    pub fn queryStaticReferred__area(&self, sheet_uid: i32, area: &Area, out: &mut Vec<SharedFmlReferredArea>) {
        let mut cells = Vec::new();
        self.cellRefMap.queryReferred__area(sheet_uid, area, &mut cells);
        Self::convertRS1(cells, out);
        let mut areas = Vec::new();
        self.areaRefMap.queryAreaReferred__area(sheet_uid, area, &mut areas);
        self.convertRS2(areas, out, area);
    }

    // java private void convertRS2(ArrayList<SharedReference> in, Collection out, Area query)
    fn convertRS2(&self, input: Vec<SharedReference>, out: &mut Vec<SharedFmlReferredArea>, query: &Area) {
        for sr in input {
            let fml = &sr.fml;
            let (fr, lr, fc, lc) = (fml.getFmlRow(), fml.getFmlLastRow(), fml.getFmlCol(), fml.getFmlLastCol());
            let mut intersect = sr.referred.intersectWith(query).expect("SharedReference must intersect query");
            let marked = match sr.ptg.kind {
                PtgKind::Ref { row, col_raw, .. } | PtgKind::Ref3D { row, col_raw, .. } => {
                    let col = col_index(col_raw);
                    let (mut first_row, mut last_row) = if !row_rel(col_raw) { (fr, lr) }
                        else { let offset = fr - row; (intersect.getFirstRow() + offset, intersect.getLastRow() + offset) };
                    let (mut first_col, mut last_col) = if !col_rel(col_raw) { (fc, lc) }
                        else { let offset = fc - col; (intersect.getFirstCol() + offset, intersect.getLastCol() + offset) };
                    if row_rel(col_raw) { Self::wrapAxis(&mut first_row, &mut last_row, self.sv.getLastRowIndex(), self.sv.getMaxRows()); first_row = first_row.max(fr); last_row = last_row.min(lr); }
                    if col_rel(col_raw) { Self::wrapAxis(&mut first_col, &mut last_col, self.sv.getLastColumnIndex(), self.sv.getMaxColumns()); first_col = first_col.max(fc); last_col = last_col.min(lc); }
                    Area::Area(first_row, last_row, first_col, last_col)
                }
                PtgKind::Area { first_row, last_row, first_col_raw, last_col_raw, .. }
                | PtgKind::Area3D { first_row, last_row, first_col_raw, last_col_raw, .. } => {
                    let (first_col, last_col) = (col_index(first_col_raw), col_index(last_col_raw));
                    let max_referred = Area::Area(first_row, last_row + lr - fr, first_col, last_col + lc - fc);
                    if !max_referred.containsArea(&intersect) {
                        if intersect.getFirstRow() < max_referred.getFirstRow() { intersect.setFirstRow(intersect.getFirstRow() + self.sv.getMaxRows()); intersect.setLastRow(intersect.getLastRow() + self.sv.getMaxRows()); }
                        if intersect.getFirstCol() < max_referred.getFirstCol() { intersect.setFirstCol(intersect.getFirstCol() + self.sv.getMaxColumns()); intersect.setLastCol(intersect.getLastCol() + self.sv.getMaxColumns()); }
                    }
                    let (mut out_fr, mut out_lr) = Self::areaAxis(intersect.getFirstRow(), intersect.getLastRow(), first_row, last_row, row_rel(first_col_raw), row_rel(last_col_raw), fr, lr);
                    let (mut out_fc, mut out_lc) = Self::areaAxis(intersect.getFirstCol(), intersect.getLastCol(), first_col, last_col, col_rel(first_col_raw), col_rel(last_col_raw), fc, lc);
                    Self::wrapAxis(&mut out_fr, &mut out_lr, self.sv.getLastRowIndex(), self.sv.getMaxRows());
                    Self::wrapAxis(&mut out_fc, &mut out_lc, self.sv.getLastColumnIndex(), self.sv.getMaxColumns());
                    Area::Area(out_fr.max(fr), out_lr.min(lr), out_fc.max(fc), out_lc.min(lc))
                }
                _ => unreachable!(),
            };
            out.push(SharedFmlReferredArea::SharedFmlReferredArea(sr.fml, marked));
        }
    }

    // java convertRS2 adjusts an interval only when both ends lie outside one side.
    fn wrapAxis(first: &mut i32, last: &mut i32, last_index: i32, max: i32) {
        if *first < 0 && *last < 0 { *first += max; *last += max; }
        else if *first > last_index && *last > last_index { *first -= max; *last -= max; }
    }

    // java convertRS2 AreaPtgBase row and column branches have the same conditional structure.
    fn areaAxis(q_first: i32, q_last: i32, first: i32, last: i32, first_relative: bool, last_relative: bool, fml_first: i32, fml_last: i32) -> (i32, i32) {
        if !first_relative && !last_relative { (fml_first, fml_last) }
        else if !first_relative { (if q_first <= last { fml_first } else { fml_first + q_first - last }, fml_last) }
        else if !last_relative {
            if q_first <= last && last <= q_last { (fml_first, fml_last) }
            else if q_last < last { (fml_first, q_last + fml_first - first) }
            else { (q_first + fml_first - first, fml_last) }
        } else {
            let size = last - first + 1;
            let offset = fml_first - first;
            (q_first + 1 - size + offset, q_last + offset)
        }
    }
}
