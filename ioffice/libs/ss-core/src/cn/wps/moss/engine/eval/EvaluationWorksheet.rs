// java package cn.wps.moss.engine.eval;
use super::EvaluationCell::EvaluationCell;
use crate::cn::wps::et::ss::interfaces::{IEvalCell::IEvalCell, IEvalWorksheet::IEvalWorksheet};
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::java_runtime::{Rc, RefCell};
// java public final class EvaluationWorksheet implements IEvalWorksheet
pub struct EvaluationWorksheet {
    ksheet: Rc<RefCell<KmoSheet>>,
}
impl EvaluationWorksheet {
    // java this.ksheet=s;
    pub fn EvaluationWorksheet(s: Rc<RefCell<KmoSheet>>) -> Self {
        Self { ksheet: s }
    }
}
impl IEvalWorksheet for EvaluationWorksheet {
    // java getFormulaIterator returns each real iterator entry, including null for non-formula cells and array dependents.
    fn getFormulaIterator(
        &self,
        area: &crate::cn::wps::et::ss::util::Area::Area,
    ) -> Option<Box<
        dyn Iterator<
            Item = Option<
                Rc<dyn crate::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase>,
            >,
        >,
    >> {
        Some(Box::new(FormulaIterator {
            sheet: self.ksheet.clone(),
            iter: self.ksheet.borrow().getCellIterator(
                area.getFirstRow(),
                area.getLastRow(),
                area.getFirstCol(),
                area.getLastCol(),
            ),
        }))
    }
    // java public Iterator<IEvalCell> getCellIterator(int fr,int lr,int fc,int lc){final KmoSheet.CellIterator iter=this.ksheet.getCellIterator(fr,lr,fc,lc);return new Iterator(){public EvaluationCell next(){iter.next();return new EvaluationCell(ksheet,iter.cellData(),iter.row(),iter.col());}...};}
    fn getCellIterator(&self,fr:i32,lr:i32,fc:i32,lc:i32)->Box<dyn Iterator<Item=Rc<dyn IEvalCell>>>{Box::new(CellIterator{sheet:self.ksheet.clone(),iter:self.ksheet.borrow().getCellIterator(fr,lr,fc,lc)})}
    // java return ksheet.getSheetUid();
    fn getUid(&self) -> i32 {
        self.ksheet.borrow().getSheetUid()
    }
    // java KmoCellData c=ksheet.getCellData(row,col); return new EvaluationCell(ksheet,c,row,col);
    fn getCell(&self, row: i32, col: i32) -> Rc<dyn IEvalCell> {
        let c = self.ksheet.borrow().getCellData(row, col);
        Rc::new(EvaluationCell::EvaluationCell(
            self.ksheet.clone(),
            c,
            row,
            col,
        ))
    }
}

struct FormulaIterator {
    sheet: Rc<RefCell<KmoSheet>>,
    iter: crate::cn::wps::moss::app::KmoSheet::CellIterator,
}
impl Iterator for FormulaIterator {
    type Item =
        Option<Rc<dyn crate::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase>>;
    fn next(&mut self) -> Option<Self::Item> {
        use crate::cn::wps::et::ss::formula::ptg::ExpPtg::ExpPtg;
        use crate::cn::wps::moss::engine::grid::KmoFormulaData::KmoFormulaData;
        if !self.iter.hasNext() {
            return None;
        }
        self.iter.next();
        let row = self.iter.row();
        let col = self.iter.col();
        let cell = self.iter.cellData();
        if cell.formulaType == 0 {
            return Some(None);
        }
        if cell.formulaType == 2 {
            let sheet = self.sheet.borrow();
            let values = sheet.getCellValueStore();
            let mut f = KmoFormulaData::KmoFormulaData();
            values.borrow().formulaValue(cell.id, &mut f);
            let bytes = values.borrow().formulaTokens(f.getTokensId());
            if bytes.len() == ExpPtg::getSize(sheet.getVersion()) as usize && bytes[0] == 1 {
                let rc = ExpPtg::getRowCol(sheet.getVersion(), &bytes);
                if rc[0] == row && rc[1] == col {
                    return Some(Some(Rc::new(super::EvaluationArrayFormula::EvaluationArrayFormula::EvaluationArrayFormula(self.sheet.clone(),row,col))));
                }
            }
            return Some(None);
        }
        Some(Some(Rc::new(
            super::EvaluationCellFormula::EvaluationCellFormula::EvaluationCellFormula(
                self.sheet.clone(),
                row,
                col,
            ),
        )))
    }
}

struct CellIterator{sheet:Rc<RefCell<KmoSheet>>,iter:crate::cn::wps::moss::app::KmoSheet::CellIterator}impl Iterator for CellIterator{type Item=Rc<dyn IEvalCell>;fn next(&mut self)->Option<Self::Item>{if !self.iter.hasNext(){return None;}self.iter.next();Some(Rc::new(EvaluationCell::EvaluationCell(self.sheet.clone(),{let c=self.iter.cellData();crate::cn::wps::moss::engine::grid::KmoCellData::KmoCellData{ixf:c.ixf,formulaType:c.formulaType,hasComment:c.hasComment,isPicInCell:c.isPicInCell,id:c.id}},self.iter.row(),self.iter.col())))}}
