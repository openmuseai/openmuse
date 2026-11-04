// Java: public interface BATManaged
pub trait BATManaged {
    // Java: abstract int countBlocks()
    fn countBlocks(&self) -> i32;

    // Java: abstract void setStartBlock(int var1)
    fn setStartBlock(&mut self, var1: i32);
}
