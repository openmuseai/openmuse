// Java: import java.util.Set
use std::collections::HashSet;

// Java: public interface IMergeKmoBookDocument
pub trait IMergeKmoBookDocument {
    // Java: abstract Set<Integer> getNotEmptySelectIndex(IDocument var1, Set<Integer> var2)
    fn getNotEmptySelectIndex(&self, _var1: &dyn std::any::Any, _var2: &HashSet<i32>) -> HashSet<i32>;

    // Java: abstract boolean hasEmptySheet(IDocument var1, boolean var2, Set<Integer> var3)
    fn hasEmptySheet(&self, _var1: &dyn std::any::Any, _var2: bool, _var3: &HashSet<i32>) -> bool;

    // Java: abstract boolean isAllSheetEmpty(IDocument var1, boolean var2, Set<Integer> var3, boolean var4)
    fn isAllSheetEmpty(&self, _var1: &dyn std::any::Any, _var2: bool, _var3: &HashSet<i32>, _var4: bool) -> bool;

    // Java: abstract Set<Integer> getSelectedIndex(IDocument var1, boolean var2, Set<Integer> var3, Set<Integer> var4)
    fn getSelectedIndex(&self, _var1: &dyn std::any::Any, _var2: bool, _var3: &HashSet<i32>, _var4: &HashSet<i32>) -> HashSet<i32>;

    // Java: abstract Set<Integer> getNoVeryHiddenSelectedIndex(IDocument var1, Set<Integer> var2)
    fn getNoVeryHiddenSelectedIndex(&self, _var1: &dyn std::any::Any, _var2: &HashSet<i32>) -> HashSet<i32>;

    // Java: abstract String getSelectSheetName(IDocument var1, boolean var2, Set<Integer> var3)
    fn getSelectSheetName(&self, _var1: &dyn std::any::Any, _var2: bool, _var3: &HashSet<i32>) -> String;

}
