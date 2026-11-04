// java public static enum Type
#[derive(Copy, Clone, Debug)]
pub enum Type {
    BULLET,
    NUMBER,
}

// java public interface ListLevel
pub trait ListLevel {
    // java  int getIndex()
    fn getIndex(&self) -> i32;

    // java  Type getType()
    fn getType(&self) -> Type;

    // java  int getStartAt()
    fn getStartAt(&self) -> i32;

    // java  void setStartAt(int var1)
    fn setStartAt(&mut self, var1: i32);

}

// java public static enum Type
impl Type {
}
