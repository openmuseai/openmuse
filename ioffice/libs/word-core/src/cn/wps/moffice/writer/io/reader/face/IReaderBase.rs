// java public interface IReaderBase
pub trait IReaderBase {
    // java  void read()
    fn read(&mut self);

    // java  void dumpAllPictures()
    fn dumpAllPictures(&mut self);

    // java  void dumpVbaMacro()
    fn dumpVbaMacro(&mut self);

}
