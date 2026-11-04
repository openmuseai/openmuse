// Java: public interface ITreeBalancer
pub trait ITreeBalancer {
    // Java: abstract void onCreateHead(Tree var1, Node var2)
    fn onCreateHead(&mut self, tree: *mut u8, node: *mut u8);

    // Java: abstract void onBinaryTreeInserted(Tree var1, Node var2)
    fn onBinaryTreeInserted(&mut self, tree: *mut u8, node: *mut u8);

    // Java: abstract void beforeSwap(Tree var1, Node var2, Node var3)
    fn beforeSwap(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8);

    // Java: abstract void onBinaryTreeErased(Tree var1, Node var2, Node var3, Node var4)
    fn onBinaryTreeErased(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8, var4: *mut u8);

    // Java: abstract ITreeCreator getTreeCreator()
    fn getTreeCreator(&self) -> Option<Box<dyn std::any::Any>>;
}
