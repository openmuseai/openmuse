// java package cn.wps.et.ss.dependent;

use crate::java_runtime::Rc;
use crate::cn::wps::et::ss::dependent::refer::StaticAreaRefMap::StaticAreaRefMap;
use crate::cn::wps::et::ss::dependent::refer::StaticCellRefMap::StaticCellRefMap;
use crate::cn::wps::et::ss::dependent::refer::StaticNameRefMap::StaticNameRefMap;
use crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg;
use crate::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
use crate::cn::wps::et::ss::interfaces::IEvalWorkbook::IEvalWorkbook;
use crate::cn::wps::et::ss::util::Area::Area;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;
use super::SharedFmlDependent::SharedFmlDependent;
use super::SharedFmlReferredArea::SharedFmlReferredArea;

// java public class Dependent
pub struct Dependent {
    // java private final StaticCellRefMap cellRefMap;
    cellRefMap: StaticCellRefMap,
    // java private final StaticAreaRefMap areaRefMap;
    areaRefMap: StaticAreaRefMap,
    // java private final SharedFmlDependent sharedFmlDep;
    sharedFmlDep: SharedFmlDependent,
    // java private final StaticNameRefMap nameRefMap;
    nameRefMap: StaticNameRefMap,
}

impl Dependent {
    // java public Dependent(SpreadsheetVersion sv)
    pub fn Dependent(sv: SpreadsheetVersion) -> Self {
        Self { cellRefMap: StaticCellRefMap::StaticCellRefMap(sv), areaRefMap: StaticAreaRefMap::StaticAreaRefMap(sv), sharedFmlDep: SharedFmlDependent::SharedFmlDependent(sv), nameRefMap: StaticNameRefMap::StaticNameRefMap(sv) }
    }

    // java private void registerName(IEvalFormulaBase fml, Ptg[] ptgs, IEvalWorkbook cw, boolean inTrans)
    fn registerName(&mut self, fml: &Rc<dyn IEvalFormulaBase>, ptgs: &[Ptg], cw: &dyn IEvalWorkbook, in_trans: bool) {
        self.nameRefMap.registerName__tokens(ptgs, fml, cw, in_trans);
    }

    // java private void unregisterName(IEvalFormulaBase fml, IEvalWorkbook cw)
    fn unregisterName(&mut self, fml: &Rc<dyn IEvalFormulaBase>, cw: &dyn IEvalWorkbook) {
        self.nameRefMap.unregisterName(fml, cw);
    }

    // java public void registerFml(IEvalFormulaBase fml, Ptg[] ptgs, IEvalWorkbook cw, boolean inTrans)
    pub fn registerFml(&mut self, fml: &Rc<dyn IEvalFormulaBase>, ptgs: &[Ptg], cw: &dyn IEvalWorkbook, in_trans: bool) {
        // java registerName runs before the shared-formula branch.
        self.registerName(fml, ptgs, cw, in_trans);
        let mut done = false;
        // java if (fml instanceof IEvalCellFormula)
        if let Some(cf) = fml.as_cell_formula() {
            if cf.isSharedFormula() {
                if cf.isMasterSharedFormula() {
                    if let Some(df) = cf.getSharedFormula() {
                        if df.getFmlLastRow() > df.getFmlRow() || df.getFmlLastCol() > df.getFmlCol() {
                            let df: Rc<dyn crate::cn::wps::et::ss::interfaces::IEvalSharedFormula::IEvalSharedFormula> = Rc::from(df);
                            self.sharedFmlDep.registerTokens(ptgs, &df, cw, in_trans);
                            done = true;
                        }
                    }
                } else { done = true; }
            }
        }
        // java if (!done) { cellRefMap.registerTokens(...); areaRefMap.registerTokens(...); }
        if !done {
            self.cellRefMap.registerTokens(ptgs, fml, cw, in_trans);
            self.areaRefMap.registerTokens(ptgs, fml, cw, in_trans);
        }
    }

    // java public void unregisterFml(IEvalFormulaBase fml, IEvalWorkbook cw)
    pub fn unregisterFml(&mut self, fml: &Rc<dyn IEvalFormulaBase>, cw: &dyn IEvalWorkbook) {
        self.unregisterName(fml, cw);
        let mut done = false;
        if let Some(cf) = fml.as_cell_formula() {
            if cf.isSharedFormula() {
                if cf.isMasterSharedFormula() {
                    if let Some(df) = cf.getSharedFormula() {
                        if df.getFmlLastRow() > df.getFmlRow() || df.getFmlLastCol() > df.getFmlCol() {
                            let df: Rc<dyn crate::cn::wps::et::ss::interfaces::IEvalSharedFormula::IEvalSharedFormula> = Rc::from(df);
                            self.sharedFmlDep.unregisterFml(&df, cw);
                            done = true;
                        }
                    }
                } else { done = true; }
            }
        }
        if !done {
            self.cellRefMap.unregisterFml(fml, cw);
            self.areaRefMap.unregisterFml(fml, cw);
        }
    }

    // java public void queryNameReferred(int nameNumber, Collection<IEvalFormulaBase> out)
    pub fn queryNameReferred(&self, name_number: i32, out: &mut Vec<Rc<dyn IEvalFormulaBase>>) {
        self.nameRefMap.queryReferred(name_number, out);
    }

    // java public void queryStaticReferred(int sheetUid, int row, int col, Collection out, Collection out2)
    pub fn queryStaticReferred(&self, sheet_uid: i32, row: i32, col: i32, out: &mut Vec<Rc<dyn IEvalFormulaBase>>, out2: &mut Vec<SharedFmlReferredArea>) {
        self.cellRefMap.queryReferred(sheet_uid, row, col, out);
        self.areaRefMap.queryAreaReferred(sheet_uid, row, col, out);
        self.sharedFmlDep.queryStaticReferred(sheet_uid, row, col, out2);
    }

    // java public void queryStaticReferred(int sheetUid, Area area, Collection out, Collection out2)
    pub fn queryStaticReferred__area(&self, sheet_uid: i32, area: &Area, out: &mut Vec<Rc<dyn IEvalFormulaBase>>, out2: &mut Vec<SharedFmlReferredArea>) {
        self.cellRefMap.queryReferred__area(sheet_uid, area, out);
        self.areaRefMap.queryAreaReferred__area(sheet_uid, area, out);
        self.sharedFmlDep.queryStaticReferred__area(sheet_uid, area, out2);
    }

    // java public StaticCellRefMap getStaticCellRefMap()
    pub fn getStaticCellRefMap(&self) -> &StaticCellRefMap { &self.cellRefMap }
    // java public StaticAreaRefMap getStaticAreaRefMap()
    pub fn getStaticAreaRefMap(&self) -> &StaticAreaRefMap { &self.areaRefMap }
}
