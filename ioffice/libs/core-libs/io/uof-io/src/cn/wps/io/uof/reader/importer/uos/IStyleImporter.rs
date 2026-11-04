// Java: public interface IStyleImporter
pub trait IStyleImporter {
    // Java: abstract void importNumberFormat()
    fn importNumberFormat(&self);

    // Java: abstract void importFont()
    fn importFont(&self);

    // Java: abstract void importFill()
    fn importFill(&self);

    // Java: abstract void importBorder()
    fn importBorder(&self);

    // Java: abstract void importCellXf()
    fn importCellXf(&self);

}
