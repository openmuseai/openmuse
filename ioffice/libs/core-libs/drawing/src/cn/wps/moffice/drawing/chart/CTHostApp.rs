#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]

use std::sync::Arc;
use std::any::Any;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class CTHostApp implements IHostApp
pub struct CTHostApp {
    // Java field: IColorMapping mColorMapping
    pub(crate) mColorMapping: Element,
    // Java field: HostTheme mHostTheme
    pub(crate) mHostTheme: Element,
    // Java field: IMediaLib mMediaLib
    pub(crate) mMediaLib: Element,
    // Java field: String mDocAppVersion
    pub(crate) mDocAppVersion: Option<String>,
    // Java field: String mDocAppName
    pub(crate) mDocAppName: Option<String>,
}

// Java: public class CTHostApp implements IHostApp
impl CTHostApp {
    // Java: CTHostApp()
    pub fn new() -> Self {
      // Java: this((IMediaLib)null);
      Self::new_with_mediaLib(None)
    }

    // Java: CTHostApp(IMediaLib mediaLib)
    pub fn new_with_mediaLib(mediaLib: Element) -> Self {
      // Java: this(mediaLib, (String)null);
      Self::new_with_mediaLib_and_theme(mediaLib, None)
    }

    // Java: CTHostApp(IMediaLib mediaLib, String themePath)
    pub fn new_with_mediaLib_and_theme(mediaLib: Element, themePath: Option<String>) -> Self {
      // Java: super();
      // Java: this.mMediaLib = mediaLib;
      // Java: this.mHostTheme = new HostTheme();
      let mut hostTheme = super::HostTheme::HostTheme::new();
      // Java: this.mHostTheme.setTheme(themePath);
      if let Some(path) = themePath {
          hostTheme.setTheme(path);
      }
      CTHostApp {
          mColorMapping: None,
          mHostTheme: Some(Arc::new(std::sync::Mutex::new(hostTheme))),
          mMediaLib: mediaLib,
          mDocAppVersion: None,
          mDocAppName: None,
      }
    }

    // Java: void destroy()
    pub fn destroy(&mut self) {
      // Java: this.mColorMapping = null;
      self.mColorMapping = None;
      // Java: this.mHostTheme = null;
      self.mHostTheme = None;
      // Java: this.mMediaLib = null;
      self.mMediaLib = None;
    }

    // Java: void setMediaLib(IMediaLib mediaLib)
    pub fn setMediaLib(&mut self, mediaLib: Element) {
      // Java: this.mMediaLib = mediaLib;
      self.mMediaLib = mediaLib;
    }

    // Java: void setColorMapping(IColorMapping colorMapping)
    pub fn setColorMapping(&mut self, colorMapping: Element) {
      // Java: this.mColorMapping = colorMapping;
      self.mColorMapping = colorMapping;
    }

    // Java: void setDocAppVersion(String appVersion)
    pub fn setDocAppVersion(&mut self, appVersion: Option<String>) {
      // Java: this.mDocAppVersion = appVersion;
      self.mDocAppVersion = appVersion;
    }

    // Java: void setDocAppName(String appName)
    pub fn setDocAppName(&mut self, appName: Option<String>) {
      // Java: this.mDocAppName = appName;
      self.mDocAppName = appName;
    }

    // Java: String docAppVersion()
    pub fn docAppVersion(&self) -> String {
      // Java: return this.mDocAppVersion;
      self.mDocAppVersion.clone().unwrap_or_default()
    }

    // Java: String docAppName()
    pub fn docAppName(&self) -> String {
      // Java: return this.mDocAppName;
      self.mDocAppName.clone().unwrap_or_default()
    }

    // Java: IColorMapping colorMap()
    pub fn colorMap(&self) -> &Element {
      // Java: return this.mColorMapping;
      &self.mColorMapping
    }

    // Java: HostTheme hostTheme()
    pub fn hostTheme(&self) -> &Element {
      // Java: return this.mHostTheme;
      &self.mHostTheme
    }

    // Java: IMediaLib mediaLib()
    pub fn mediaLib(&self) -> &Element {
      // Java: return this.mMediaLib;
      &self.mMediaLib
    }

}
