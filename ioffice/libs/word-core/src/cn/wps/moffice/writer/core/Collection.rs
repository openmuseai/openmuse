// java public interface Collection
pub trait Collection<T> {
    // java  int count()
    fn count(&self) -> i32;

    // java  T item(int var1)
    fn item(&self, var1: i32) -> &T;

}
