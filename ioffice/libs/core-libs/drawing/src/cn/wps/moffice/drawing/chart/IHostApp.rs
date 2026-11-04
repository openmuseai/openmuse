#![allow(non_snake_case, dead_code, unused_imports)]

use std::sync::Arc;
use std::any::Any;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public interface IHostApp
pub trait IHostApp {
    // Java: abstract IMediaLib mediaLib()
    fn mediaLib(&self) -> Element;

    // Java: abstract IColorMapping colorMap()
    fn colorMap(&self) -> Element;

    // Java: abstract HostTheme hostTheme()
    fn hostTheme(&self) -> Element;

    // Java: abstract int appType()
    fn appType(&self) -> i32;

    // Java: abstract String docAppVersion()
    fn docAppVersion(&self) -> String;

    // Java: abstract String docAppName()
    fn docAppName(&self) -> String;

    // Java: abstract void destroy()
    fn destroy(&self);

}
