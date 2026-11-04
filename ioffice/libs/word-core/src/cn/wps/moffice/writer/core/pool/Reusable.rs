// java public interface Reusable
pub trait Reusable {
    // java  boolean reuseClean()
    fn reuseClean(&mut self) -> bool;

    // java  void reuseInit()
    fn reuseInit(&mut self);

}

// java public interface SafeRef
pub trait SafeRef<T> {
    // java  T get()
    fn get(&self) -> T;

    // java  boolean isEmtpy()
    fn isEmtpy(&self) -> bool;

}
