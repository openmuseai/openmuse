// Java: public interface IWorkSheetImporter
pub trait IWorkSheetImporter {
    // Java: abstract void setSheetIndex(int var1)
    fn setSheetIndex(&self, var1: i32);

    // Java: abstract void importCellValue()
    fn importCellValue(&self);

    // Java: abstract void importUsedRange()
    fn importUsedRange(&self);

    // Java: abstract void importFormula()
    fn importFormula(&self);

    // Java: abstract void importAllFormula()
    fn importAllFormula(&self);

    // Java: abstract void importFilter()
    fn importFilter(&self);

    // Java: abstract void importRowInfo()
    fn importRowInfo(&self);

    // Java: abstract void importSort()
    fn importSort(&self);

    // Java: abstract void importPageSetting()
    fn importPageSetting(&self);

    // Java: abstract void importView()
    fn importView(&self);

    // Java: abstract void importProtection()
    fn importProtection(&self);

}
