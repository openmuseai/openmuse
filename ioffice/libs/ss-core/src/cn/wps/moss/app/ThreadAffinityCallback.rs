// java package cn.wps.moss.app;
// java public interface ThreadAffinityCallback
pub trait ThreadAffinityCallback: Send + Sync {
    fn enableBigCores(&self);
}
