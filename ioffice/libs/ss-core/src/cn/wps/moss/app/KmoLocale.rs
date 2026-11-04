// java package cn.wps.moss.app;
use std::sync::{LazyLock, RwLock};
// rust: Java Locale.toString storage; the host initializes Locale.getDefault from its process locale.
static LOCALE: LazyLock<RwLock<String>> = LazyLock::new(|| {
    RwLock::new(
        ["LC_ALL", "LC_MESSAGES", "LANG"]
            .iter()
            .filter_map(|k| std::env::var(k).ok())
            .find(|s| !s.is_empty())
            .unwrap_or_else(|| "en_US".into())
            .split(['.', '@'])
            .next()
            .unwrap()
            .replace('-', "_"),
    )
});
// java public class KmoLocale
pub struct KmoLocale;
impl KmoLocale {
    // java return mLocale;
    pub fn getDefault() -> String {
        LOCALE.read().unwrap().clone()
    }
    // java mLocale = locale;
    pub fn setLocale(locale: String) {
        *LOCALE.write().unwrap() = locale;
    }
}
