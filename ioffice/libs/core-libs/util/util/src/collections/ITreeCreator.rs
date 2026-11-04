// Java: public interface ITreeCreator extends INodeVisitor
pub trait ITreeCreator {
    // Java: abstract void createTree(Tree var1, Node var2, INodeProvider var3)
    fn createTree(&self, Tree var1, Node var2, INodeProvider var3);

}
