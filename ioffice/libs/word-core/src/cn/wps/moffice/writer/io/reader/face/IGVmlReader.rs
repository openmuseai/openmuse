// java public interface IGVmlReader
pub trait IGVmlReader {
    // java  int read()
    fn read(&mut self) -> i32;

    // java  void dispose()
    fn dispose(&mut self);

}
