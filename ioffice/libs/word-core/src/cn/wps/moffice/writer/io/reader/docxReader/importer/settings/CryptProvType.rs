// Java: cn.wps.moffice.writer.io.reader.docxReader.importer.settings.CryptProvType
//
// ★M-STAGE1 P2 —— 全仓缺失的 Java 枚举，由 `SettingsImporter.convertProvType`
//   与 `setWriteProtection` 的加密 provider 判定使用。
//
// Java 原文（3 变体 + `IntEnumMap` 静态表）：
// ```java
// public enum CryptProvType { RSAAES, RSAFULL, CUSTOM;
//    private static IntEnumMap<CryptProvType> sMap;
//    public void setVal(int v) { sMap.put(v, this); }
//    public static boolean isInitialized() { return null != sMap; }
//    public static void initialize() { sMap = new IntEnumMap(); }
//    public static void unInitialize() { sMap = null; }
//    public static CryptProvType fromValue(int v) { return (CryptProvType)sMap.get(v); }
// }
// ```
//
// rust: Java 的 `sMap` 是**类级可变静态**（`initialize` 时创建、`setVal` 时填充）。
//   Rust 侧以 `Mutex<Option<Vec<Option<CryptProvType>>>>` 表达"可为 null 的表"，
//   语义等价：`isInitialized` = `Some`，`unInitialize` = `None`。
use std::sync::Mutex;

// Java: public enum CryptProvType
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum CryptProvType {
    #[default]
    RSAAES,
    RSAFULL,
    CUSTOM,
}

/// Java: private static IntEnumMap<CryptProvType> sMap;
/// rust: `None` 表示 Java 的 `sMap == null`；`Some(v)` 表示已 `initialize`。
static S_MAP: Mutex<Option<Vec<(i32, CryptProvType)>>> = Mutex::new(None);

impl CryptProvType {
    // java public void setVal(int v)
    pub fn setVal(&self, v: i32) {
        // java sMap.put(v, this);
        let mut guard = S_MAP.lock().unwrap();
        if let Some(map) = guard.as_mut() {
            map.retain(|(k, _)| *k != v);
            map.push((v, *self));
        }
    }

    // java public static boolean isInitialized()
    pub fn isInitialized() -> bool {
        // java return null != sMap;
        return S_MAP.lock().unwrap().is_some();
    }

    // java public static void initialize()
    pub fn initialize() {
        // java sMap = new IntEnumMap();
        *S_MAP.lock().unwrap() = Some(Vec::new());
    }

    // java public static void unInitialize()
    pub fn unInitialize() {
        // java sMap = null;
        *S_MAP.lock().unwrap() = None;
    }

    // java public static CryptProvType fromValue(int v)
    pub fn fromValue(v: i32) -> Option<CryptProvType> {
        // java return (CryptProvType)sMap.get(v);
        let guard = S_MAP.lock().unwrap();
        return match guard.as_ref() {
            None => None,
            Some(map) => map.iter().find(|(k, _)| *k == v).map(|(_, t)| *t),
        };
    }
}
