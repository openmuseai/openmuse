// Java: import cn.wps.moffice.drawing.IHostApp
use drawing::cn::wps::moffice::drawing::IHostApp::*;
// Java: import cn.wps.moffice.drawing.chart.HostTheme
use drawing::cn::wps::moffice::drawing::chart::HostTheme::*;
// Java: import cn.wps.moffice.drawing.media.IMediaLib
use drawing::cn::wps::moffice::drawing::media::IMediaLib::*;
// Java: import cn.wps.moffice.drawing.style.color.IColorMapping
use drawing::cn::wps::moffice::drawing::style::color::IColorMapping::*;

// java public class SmartartHostApp implements IHostApp
#[derive(Default)]
pub struct SmartartHostApp {
    // java IMediaLib mMediaLib;
    pub(crate) mMediaLib: Option<Box<dyn IMediaLib>>,
    // java HostTheme mTheme;
    pub(crate) mTheme: Option<HostTheme>,
    // java String mAppVersion;
    pub(crate) mAppVersion: Option<String>,
    // java String mAppName;
    pub(crate) mAppName: Option<String>,
}

// java public interface IHostApp
impl IHostApp for SmartartHostApp {
    // java public IMediaLib mediaLib()
    fn mediaLib(&self) -> Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
      // java return this.mMediaLib;
      // IHostApp Element = Option<Arc<dyn Any+Send+Sync>>; concrete IMediaLib not object-safe here.
      None
    }

    // java public IColorMapping colorMap()
    fn colorMap(&self) -> Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
      // java return null;
      None
    }

    // java public HostTheme hostTheme()
    fn hostTheme(&self) -> Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
      // java return this.mTheme;
      None
    }

    // java public int appType()
    fn appType(&self) -> i32 {
      // java return 1;
      1
    }

    // java public String docAppVersion()
    fn docAppVersion(&self) -> String {
      // java return this.mAppVersion;
      return self.mAppVersion;
    }

    // java public String docAppName()
    fn docAppName(&self) -> String {
      // java return this.mAppName;
      return self.mAppName;
    }

}

// java public static class HOST_APP
impl HOST_APP {
    // java public  HOST_APP()
    pub fn HOST_APP() -> Self {
      // java super();
      Self {}
    }

    // java public static boolean isWriter(int hostAppType)
    pub fn isWriter(hostAppType: i32) -> bool {
      // java return hostAppType == 1;
      return hostAppType == 1;
    }

    // java public static boolean isSpreadsheet(int hostAppType)
    pub fn isSpreadsheet(hostAppType: i32) -> bool {
      // java return hostAppType == 2;
      return hostAppType == 2;
    }

    // java public static boolean isPresentation(int hostAppType)
    pub fn isPresentation(hostAppType: i32) -> bool {
      // java return hostAppType == 3;
      return hostAppType == 3;
    }

}

// java public class SmartartHostApp implements IHostApp
impl SmartartHostApp {
    // java public  SmartartHostApp(IMediaLib mediaLib, String theme, String docAppVersion, String docAppName)
    pub fn SmartartHostApp(mediaLib: &dyn IMediaLib, theme: &str, docAppVersion: &str, docAppName: &str) -> Self {
      // java this.mMediaLib = mediaLib;
      let mut this = Self { mMediaLib: mediaLib, mTheme: HostTheme::new(), mAppVersion: docAppVersion, mAppName: docAppName };
      // java this.mAppName = docAppName;
      // java this.mAppVersion = docAppVersion;
      // java this.mTheme = new HostTheme();
      // java this.mTheme.setTheme(theme);
      this.mTheme.setTheme(&theme);
      this
    }

}
