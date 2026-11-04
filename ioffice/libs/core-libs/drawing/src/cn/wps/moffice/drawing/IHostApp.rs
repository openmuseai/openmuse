#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.moffice.drawing.chart.HostTheme
// Java: import cn.wps.moffice.drawing.media.IMediaLib
// Java: import cn.wps.moffice.drawing.style.color.IColorMapping

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

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

}

// Java: public class HOST_APP
pub struct HOST_APP {
}

// Java field: int WRITER
pub const WRITER: i32 = 1;
// Java field: int SPREADSHEET
pub const SPREADSHEET: i32 = 2;
// Java field: int PRESENTATION
pub const PRESENTATION: i32 = 3;

// Java: public class HOST_APP
impl HOST_APP {
    // Java: HOST_APP()
    pub fn new() -> Self {
      // Java: super();
      HOST_APP {}
    }

    // Java: static boolean isWriter(int hostAppType)
    pub fn isWriter(hostAppType: i32) -> bool {
      // Java: return hostAppType == 1;
      hostAppType == 1
    }

    // Java: static boolean isSpreadsheet(int hostAppType)
    pub fn isSpreadsheet(hostAppType: i32) -> bool {
      // Java: return hostAppType == 2;
      hostAppType == 2
    }

    // Java: static boolean isPresentation(int hostAppType)
    pub fn isPresentation(hostAppType: i32) -> bool {
      // Java: return hostAppType == 3;
      hostAppType == 3
    }
}
