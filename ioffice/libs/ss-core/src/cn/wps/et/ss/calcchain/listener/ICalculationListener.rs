// java public interface ICalculationListener
pub trait ICalculationListener: Send + Sync {
    fn notifyBegin(&self, total: i32);
    fn notifyProcess(&self, current: i32);
    fn notifyFinished(&self);
}
