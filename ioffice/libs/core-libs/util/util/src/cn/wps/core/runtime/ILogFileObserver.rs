// Java: public interface ILogFileObserver
pub trait ILogFileObserver {
    // Java: abstract void start()
    fn start(&self);

    // Java: abstract void stop()
    fn stop(&self);

}
