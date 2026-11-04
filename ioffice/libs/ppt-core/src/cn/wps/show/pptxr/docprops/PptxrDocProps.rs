#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

// Java: import cn.wps.base.log.IOLog
use compile_surface::IOLog;
// Java: import cn.wps.kfc.ooxfr.OxfrAppPart
use compile_surface::OxfrAppPart;
// Java: import cn.wps.kfc.ooxfr.OxfrCorePart
use compile_surface::OxfrCorePart;
// Java: import cn.wps.kfc.opcr.OpcUtil
use compile_surface::OpcUtil;
// Java: import cn.wps.show.data.docproperty.AppProperty
use compile_surface::AppProperty;
// Java: import cn.wps.show.data.docproperty.CoreProperty
use compile_surface::CoreProperty;
// Java: import java.io.IOException
use compile_surface::IOException;
// Java: import java.util.Date
use compile_surface::Date;

pub mod compile_surface {
    #[derive(Clone, Copy, Debug, Default)]
    pub struct IOException;
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Throwable;
    pub struct IOLog;
    impl IOLog {
        pub fn d(_tag: &str, _msg: &str, _e: Throwable) {}
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct AppProperty {
        pub application: Option<String>,
        pub appVersion: Option<String>,
        pub company: Option<String>,
        pub manager: Option<String>,
    }
    impl AppProperty {
        pub fn putApplication(&mut self, v: String) { self.application = Some(v); }
        pub fn putAppVersion(&mut self, v: String) { self.appVersion = Some(v); }
        pub fn putCompany(&mut self, v: String) { self.company = Some(v); }
        pub fn putManager(&mut self, v: String) { self.manager = Some(v); }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct CoreProperty {
        pub title: Option<String>,
        pub creator: Option<String>,
        pub lastModifiedBy: Option<String>,
        pub created: Option<String>,
        pub modified: Option<String>,
    }
    impl CoreProperty {
        pub fn putTitle(&mut self, v: String) { self.title = Some(v); }
        pub fn putCreator(&mut self, v: String) { self.creator = Some(v); }
        pub fn putLastModifiedBy(&mut self, v: String) { self.lastModifiedBy = Some(v); }
        pub fn putCreated(&mut self, v: String) { self.created = Some(v); }
        pub fn putModified(&mut self, v: String) { self.modified = Some(v); }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Date {
        pub millis: i64,
    }

    #[derive(Clone, Debug, Default)]
    pub struct OxfrAppPart {
        pub parse_ok: bool,
        pub appName: Option<String>,
        pub appVersion: Option<String>,
        pub company: Option<String>,
        pub manager: Option<String>,
    }
    impl OxfrAppPart {
        pub fn parse(&mut self) -> Result<(), Throwable> {
            if self.parse_ok { Ok(()) } else { Err(Throwable) }
        }
        pub fn getAppName(&self) -> Option<String> { self.appName.clone() }
        pub fn getAppVersion(&self) -> Option<String> { self.appVersion.clone() }
        pub fn getCompany(&self) -> Option<String> { self.company.clone() }
        pub fn getManager(&self) -> Option<String> { self.manager.clone() }
    }

    #[derive(Clone, Debug, Default)]
    pub struct OxfrCorePart {
        pub parse_ok: bool,
        pub title: Option<String>,
        pub creator: Option<String>,
        pub lastModifiedBy: Option<String>,
        pub created: Option<Date>,
        pub modified: Option<Date>,
    }
    impl OxfrCorePart {
        pub fn parse(&mut self) -> Result<(), Throwable> {
            if self.parse_ok { Ok(()) } else { Err(Throwable) }
        }
        pub fn getTitle(&self) -> Option<String> { self.title.clone() }
        pub fn getCreator(&self) -> Option<String> { self.creator.clone() }
        pub fn getLastModifiedBy(&self) -> Option<String> { self.lastModifiedBy.clone() }
        pub fn getCreated(&self) -> Option<Date> { self.created }
        pub fn getModified(&self) -> Option<Date> { self.modified }
    }

    pub struct OpcUtil;
    impl OpcUtil {
        pub fn getDateString(d: Date) -> String {
            let secs = d.millis.div_euclid(1000);
            let (y, mo, day, h, mi, s) = unix_secs_to_utc(secs);
            format!("{y:04}-{mo:02}-{day:02}T{h:02}:{mi:02}:{s:02}Z")
        }
    }

    fn unix_secs_to_utc(secs: i64) -> (i32, u32, u32, u32, u32, u32) {
        let days = secs.div_euclid(86400);
        let rem = secs.rem_euclid(86400) as u32;
        let h = rem / 3600;
        let mi = (rem % 3600) / 60;
        let s = rem % 60;
        let (y, mo, day) = civil_from_days(days);
        (y, mo, day, h, mi, s)
    }

    fn civil_from_days(z: i64) -> (i32, u32, u32) {
        let z = z + 719468;
        let era = if z >= 0 { z } else { z - 146096 } / 146097;
        let doe = (z - era * 146097) as u64;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe as i64 + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = if m <= 2 { y + 1 } else { y };
        (y as i32, m as u32, d as u32)
    }
}

// java private static final String TAG = null;
const TAG: &str = "";

// java public class PptxrDocProps
#[derive(Default)]
pub struct PptxrDocProps {
}

// java public class PptxrDocProps
impl PptxrDocProps {
    // java public static void readApp(AppProperty appProperty, OxfrAppPart part)
    pub fn readApp(appProperty: &mut AppProperty, part: Option<&mut OxfrAppPart>) {
      // java if (part != null)
      if let Some(part) = part {
            // java part.parse();
            let parse_res = part.parse();
            // java Throwable e
            if let Err(e) = parse_res {
              // java IOLog.d(TAG, "Throwable", e);
              IOLog::d(TAG, "Throwable", e);
            }
        // java String appName = part.getAppName();
        let appName: Option<String> = part.getAppName();
        // java if (appName != null)
        if let Some(appName) = appName {
          // java appProperty.putApplication(appName);
          appProperty.putApplication(appName);
        }
        // java String appVersion = part.getAppVersion();
        let appVersion: Option<String> = part.getAppVersion();
        // java if (appVersion != null)
        if let Some(appVersion) = appVersion {
          // java appProperty.putAppVersion(appVersion);
          appProperty.putAppVersion(appVersion);
        }
        // java String company = part.getCompany();
        let company: Option<String> = part.getCompany();
        // java if (company != null)
        if let Some(company) = company {
          // java appProperty.putCompany(company);
          appProperty.putCompany(company);
        }
        // java String manager = part.getManager();
        let manager: Option<String> = part.getManager();
        // java if (manager != null)
        if let Some(manager) = manager {
          // java appProperty.putManager(manager);
          appProperty.putManager(manager);
        }
      }
    }

    // java public static void readCore(CoreProperty coreProperty, OxfrCorePart part)
    pub fn readCore(coreProperty: &mut CoreProperty, part: Option<&mut OxfrCorePart>) {
      // java if (part != null)
      if let Some(part) = part {
            // java part.parse();
            let parse_res = part.parse();
            // java Throwable e
            if let Err(e) = parse_res {
              // java IOLog.d(TAG, "Throwable", e);
              IOLog::d(TAG, "Throwable", e);
              // java return;
              return;
            }
        // java String title = part.getTitle();
        let title: Option<String> = part.getTitle();
        // java if (title != null)
        if let Some(_title) = title {
          // java coreProperty.putTitle(part.getTitle());
          if let Some(t) = part.getTitle() {
            coreProperty.putTitle(t);
          }
        }
        // java String creator = part.getCreator();
        let creator: Option<String> = part.getCreator();
        // java if (creator != null)
        if let Some(creator) = creator {
          // java coreProperty.putCreator(creator);
          coreProperty.putCreator(creator);
        }
        // java String lastModifiedBy = part.getLastModifiedBy();
        let lastModifiedBy: Option<String> = part.getLastModifiedBy();
        // java if (lastModifiedBy != null)
        if let Some(lastModifiedBy) = lastModifiedBy {
          // java coreProperty.putLastModifiedBy(lastModifiedBy);
          coreProperty.putLastModifiedBy(lastModifiedBy);
        }
        // java Date created = part.getCreated();
        let created: Option<Date> = part.getCreated();
        // java if (created != null)
        if let Some(created) = created {
          // java coreProperty.putCreated(OpcUtil.getDateString(created));
          coreProperty.putCreated(OpcUtil::getDateString(created));
        }
        // java Date modified = part.getModified();
        let modified: Option<Date> = part.getModified();
        // java if (modified != null)
        if let Some(modified) = modified {
          // java coreProperty.putModified(OpcUtil.getDateString(modified));
          coreProperty.putModified(OpcUtil::getDateString(modified));
        }
      }
    }

    // java public static void readThumbsnail(OxfrCorePart part)
    pub fn readThumbsnail(part: Option<&mut OxfrCorePart>) {
        let _ = part;
    }

}
