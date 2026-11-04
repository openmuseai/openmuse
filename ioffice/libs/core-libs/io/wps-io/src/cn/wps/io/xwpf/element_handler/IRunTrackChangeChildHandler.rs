// Java: public interface IRunTrackChangeChildHandler
pub trait IRunTrackChangeChildHandler {
    // Java: abstract void setDelHandler(DelHandler var1)
    fn setDelHandler(&self, var1: &dyn std::any::Any);

    // Java: abstract void setInsHandler(InsHandler var1)
    fn setInsHandler(&self, var1: &dyn std::any::Any);

}
