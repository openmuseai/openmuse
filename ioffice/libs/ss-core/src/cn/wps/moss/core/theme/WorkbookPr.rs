#![allow(non_snake_case)]
// Java: public class WorkbookPr
pub struct WorkbookPr {
    defaultThemeVersion: String,
    chartTrackingRefBase: bool,
}

impl WorkbookPr {
    // Java: public WorkbookPr()
    pub fn WorkbookPr() -> Self {
        Self {
            defaultThemeVersion: "153222".to_string(),
            chartTrackingRefBase: false,
        }
    }

    // Java: public WorkbookPr(String ThemeVersion)
    pub fn WorkbookPr__theme(ThemeVersion: String) -> Self {
        Self {
            defaultThemeVersion: ThemeVersion,
            chartTrackingRefBase: false,
        }
    }

    // Java: public void setDefaultThemeVersion(String defaultThemeVersion)
    pub fn setDefaultThemeVersion(&mut self, defaultThemeVersion: String) {
        self.defaultThemeVersion = defaultThemeVersion;
    }

    // Java: public String getDefaultThemeVersion()
    pub fn getDefaultThemeVersion(&self) -> &str {
        &self.defaultThemeVersion
    }

    // Java: public void setChartTrackingRefBase(boolean val)
    pub fn setChartTrackingRefBase(&mut self, val: bool) {
        self.chartTrackingRefBase = val;
    }

    // Java: public boolean chartTrackingRefBase()
    pub fn chartTrackingRefBase(&self) -> bool {
        self.chartTrackingRefBase
    }
}
