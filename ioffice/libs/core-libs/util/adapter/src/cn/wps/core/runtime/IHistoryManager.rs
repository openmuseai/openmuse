// Java: public interface IHistoryManager
pub trait IHistoryManager {
    // Java: abstract String getChannelFromPersistence()
    fn getChannelFromPersistence(&self) -> String;

}
