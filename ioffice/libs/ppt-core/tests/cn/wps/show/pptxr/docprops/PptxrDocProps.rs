use ppt_core::cn::wps::show::pptxr::docprops::PptxrDocProps::{
    compile_surface::{AppProperty, CoreProperty, Date, OxfrAppPart, OxfrCorePart},
    PptxrDocProps,
};

#[test]
fn t01_null_parts_skip() {
    let mut app = AppProperty::default();
    let mut core = CoreProperty::default();
    PptxrDocProps::readApp(&mut app, None);
    PptxrDocProps::readCore(&mut core, None);
    PptxrDocProps::readThumbsnail(None);
    assert_eq!(app, AppProperty::default());
    assert_eq!(core, CoreProperty::default());
}

#[test]
fn t02_fields_put_or_skip_null() {
    let mut app = AppProperty::default();
    let mut part = OxfrAppPart {
        parse_ok: true,
        appName: Some("WPS".into()),
        appVersion: None,
        company: Some("Kingsoft".into()),
        manager: None,
    };
    PptxrDocProps::readApp(&mut app, Some(&mut part));
    assert_eq!(app.application.as_deref(), Some("WPS"));
    assert_eq!(app.appVersion, None);
    assert_eq!(app.company.as_deref(), Some("Kingsoft"));
    assert_eq!(app.manager, None);

    let mut core = CoreProperty::default();
    let mut cpart = OxfrCorePart {
        parse_ok: true,
        title: Some("T".into()),
        creator: None,
        lastModifiedBy: Some("me".into()),
        created: Some(Date { millis: 0 }),
        modified: None,
    };
    PptxrDocProps::readCore(&mut core, Some(&mut cpart));
    assert_eq!(core.title.as_deref(), Some("T"));
    assert_eq!(core.creator, None);
    assert_eq!(core.lastModifiedBy.as_deref(), Some("me"));
    assert_eq!(core.created.as_deref(), Some("1970-01-01T00:00:00Z"));
    assert_eq!(core.modified, None);
}

#[test]
fn t03_read_app_parse_error_still_puts() {
    let mut app = AppProperty::default();
    let mut part = OxfrAppPart {
        parse_ok: false,
        appName: Some("after-throw".into()),
        appVersion: Some("1".into()),
        company: Some("c".into()),
        manager: Some("m".into()),
    };
    PptxrDocProps::readApp(&mut app, Some(&mut part));
    assert_eq!(app.application.as_deref(), Some("after-throw"));
    assert_eq!(app.appVersion.as_deref(), Some("1"));
    assert_eq!(app.company.as_deref(), Some("c"));
    assert_eq!(app.manager.as_deref(), Some("m"));
}

#[test]
fn t04_read_core_parse_error_returns() {
    let mut core = CoreProperty::default();
    let mut part = OxfrCorePart {
        parse_ok: false,
        title: Some("T".into()),
        creator: Some("c".into()),
        lastModifiedBy: Some("m".into()),
        created: Some(Date { millis: 0 }),
        modified: Some(Date { millis: 0 }),
    };
    PptxrDocProps::readCore(&mut core, Some(&mut part));
    assert_eq!(core, CoreProperty::default());
}

#[test]
fn t05_date_string_utc_java_format() {
    let mut core = CoreProperty::default();
    let mut part = OxfrCorePart {
        parse_ok: true,
        title: None,
        creator: None,
        lastModifiedBy: None,
        created: Some(Date { millis: 1_609_459_200_000 }),
        modified: Some(Date { millis: 0 }),
    };
    PptxrDocProps::readCore(&mut core, Some(&mut part));
    assert_eq!(core.created.as_deref(), Some("2021-01-01T00:00:00Z"));
    assert_eq!(core.modified.as_deref(), Some("1970-01-01T00:00:00Z"));
}
