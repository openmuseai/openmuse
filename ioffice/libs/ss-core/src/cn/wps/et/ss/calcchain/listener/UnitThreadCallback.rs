// java public interface UnitThreadCallback { void beforeRun(); }
pub trait UnitThreadCallback: Send + Sync {
    fn beforeRun(&self);
}
