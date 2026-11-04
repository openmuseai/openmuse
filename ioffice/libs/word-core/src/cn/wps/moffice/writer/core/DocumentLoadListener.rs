// java public interface DocumentLoadListener
pub trait DocumentLoadListener {
    // java  void beforeLoad()
    fn beforeLoad(&mut self);

    // java  void afterLoad()
    fn afterLoad(&mut self);

}
