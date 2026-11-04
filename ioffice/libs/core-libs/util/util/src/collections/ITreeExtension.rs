// Java: public interface ITreeExtension
pub trait ITreeExtension {
    // Java: abstract void onCreateHead(Tree var1, Node var2)
    fn onCreateHead(&self, Tree var1, Node var2);

    // Java: abstract void onBinaryTreeInsertedLeft(Tree var1, Node var2, Node var3, int var4, Object var5)
    fn onBinaryTreeInsertedLeft(&self, Tree var1, Node var2, Node var3, int var4, Object var5);

    // Java: abstract void onBinaryTreeInsertedRight(Tree var1, Node var2, Node var3, int var4, Object var5)
    fn onBinaryTreeInsertedRight(&self, Tree var1, Node var2, Node var3, int var4, Object var5);

    // Java: abstract void beforeSwap(Tree var1, Node var2, Node var3)
    fn beforeSwap(&self, Tree var1, Node var2, Node var3);

    // Java: abstract void beforeErase(Tree var1, Node var2, Node var3)
    fn beforeErase(&self, Tree var1, Node var2, Node var3);

    // Java: abstract void onBinaryTreeErased(Tree var1, Node var2, Node var3, Node var4)
    fn onBinaryTreeErased(&self, Tree var1, Node var2, Node var3, Node var4);

    // Java: abstract void beforeRotateLeft(Tree var1, Node var2, Node var3)
    fn beforeRotateLeft(&self, Tree var1, Node var2, Node var3);

    // Java: abstract void beforeRotateRight(Tree var1, Node var2, Node var3)
    fn beforeRotateRight(&self, Tree var1, Node var2, Node var3);

    // Java: abstract ITreeCreator getTreeCreator()
    fn getTreeCreator(&self) -> ITreeCreator;

}
