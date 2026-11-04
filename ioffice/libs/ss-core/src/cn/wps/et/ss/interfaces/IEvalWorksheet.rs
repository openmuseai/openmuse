// java public interface IEvalWorksheet
use super::IEvalCell::IEvalCell;
use std::sync::Arc;
pub trait IEvalWorksheet: Send + Sync {
    fn getFormulaIterator(
        &self,
        area: &crate::cn::wps::et::ss::util::Area::Area,
    ) -> Option<Box<dyn Iterator<Item = Option<Arc<dyn super::IEvalFormulaBase::IEvalFormulaBase>>>>> {
        panic!("IEvalWorksheet.getFormulaIterator")
    }
    // java Iterator<IEvalCell> getCellIterator(int firstRow,int lastRow,int firstCol,int lastCol);
    fn getCellIterator(&self,_fr:i32,_lr:i32,_fc:i32,_lc:i32)->Box<dyn Iterator<Item=Arc<dyn IEvalCell>>>{panic!("IEvalWorksheet.getCellIterator")}
    fn getUid(&self) -> i32;
    fn getCell(&self, row: i32, col: i32) -> Arc<dyn IEvalCell>;
}
