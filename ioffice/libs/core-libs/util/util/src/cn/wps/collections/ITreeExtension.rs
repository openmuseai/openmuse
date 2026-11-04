// Java: public interface ITreeExtension
pub trait ITreeExtension {
    // Java field: int MASK_CREATEHEAD
    // Java field: int MASK_INSERTED
    // Java field: int MASK_ERASED
    // Java field: int MASK_BEFORESWAP
    // Java field: int MASK_BEFOREERASE
    // Java field: int MASK_BEFOREROTATE
    // Java field: int MASK_ALL

    // Java: abstract void onCreateHead(Tree var1, Node var2)
    fn onCreateHead(&mut self, tree: *mut u8, node: *mut u8);

    // Java: abstract void onBinaryTreeInsertedLeft(Tree var1, Node var2, Node var3, int var4, Object var5)
    fn onBinaryTreeInsertedLeft(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8, var4: i32, var5: Option<&dyn std::any::Any>);

    // Java: abstract void onBinaryTreeInsertedRight(Tree var1, Node var2, Node var3, int var4, Object var5)
    fn onBinaryTreeInsertedRight(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8, var4: i32, var5: Option<&dyn std::any::Any>);

    // Java: abstract void beforeSwap(Tree var1, Node var2, Node var3)
    fn beforeSwap(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8);

    // Java: abstract void beforeErase(Tree var1, Node var2, Node var3)
    fn beforeErase(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8);

    // Java: abstract void onBinaryTreeErased(Tree var1, Node var2, Node var3, Node var4)
    fn onBinaryTreeErased(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8, var4: *mut u8);

    // Java: abstract void beforeRotateLeft(Tree var1, Node var2, Node var3)
    fn beforeRotateLeft(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8);

    // Java: abstract void beforeRotateRight(Tree var1, Node var2, Node var3)
    fn beforeRotateRight(&mut self, tree: *mut u8, var2: *mut u8, var3: *mut u8);

    // Java: abstract ITreeCreator getTreeCreator()
    fn getTreeCreator(&self) -> Option<Box<dyn std::any::Any>>;
}

pub const MASK_CREATEHEAD: i32 = 1;
pub const MASK_INSERTED: i32 = 2;
pub const MASK_ERASED: i32 = 4;
pub const MASK_BEFORESWAP: i32 = 8;
pub const MASK_BEFOREERASE: i32 = 16;
pub const MASK_BEFOREROTATE: i32 = 32;
pub const MASK_ALL: i32 = 63;
