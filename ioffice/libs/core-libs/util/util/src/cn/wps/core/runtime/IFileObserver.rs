// Java: public interface IFileObserver
pub trait IFileObserver {
    // Java: abstract void start()
    fn start(&self);

    // Java: abstract void stop()
    fn stop(&self);

}
