// Java: import cn.wps.moffice.transaction.Action
use util::moffice::transaction::Action::*;

// java public interface ITextEditor
pub trait ITextEditor {
    // java  void waitSync()
    fn waitSync(&mut self);

    // java  void saveSelection(Action var1, String var2)
    fn saveSelection(&mut self, var1: &Action, var2: &str);

    // java  void restoreSelection(Action var1, String var2)
    fn restoreSelection(&mut self, var1: &Action, var2: &str);

    // java  void mergeSelectionStore(Action var1, Action var2)
    fn mergeSelectionStore(&mut self, var1: &Action, var2: &Action);

    // java  void beforeSectionChanged()
    fn beforeSectionChanged(&mut self);

    // java  void afterSectionChanged()
    fn afterSectionChanged(&mut self);

}
