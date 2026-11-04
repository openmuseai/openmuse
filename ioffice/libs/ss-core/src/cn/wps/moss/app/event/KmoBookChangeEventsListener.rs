// java public interface KmoBookChangeEventsListener {
pub trait KmoBookChangeEventsListener: Send + Sync {
    fn bookDidFinishInitializing(&self);
    fn bookSettingsDidFinishChanging(&self);
    fn bookContentsDidFinishChanging(&self);
    fn sheetDidFinishActivating(&self);
}
