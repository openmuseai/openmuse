// Java: cn.wps.moffice.writer.core.protection.ProtectionType
//
// ★M-STAGE1 P2 —— 从 auto-generated **空枚举**（无任何变体）做实。
//   闭包链：SettingsImporter → DocumentProtectionIO → KDocumentProtection
//                                                   → **ProtectionType**
//
// Java 原文 8 行，5 个变体，无字段/方法。
// ★该空枚举此前一直"能编译"，因为它从未被登记；登记 `KDocumentProtection`
//   后其 `ProtectionType::READONLY` / `FORMS` 等引用立即暴露为 E0599
//   —— 又一个"未被登记 ⇒ 缺陷被掩盖"的实例（同 §26.3 的两个异常类）。

// java public enum ProtectionType
// rust: 变体名与 Java 逐字一致（含 Java 的全大写命名习惯）。
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum ProtectionType {
    // java NONE,
    #[default]
    NONE,
    // java READONLY,
    READONLY,
    // java COMMENTS,
    COMMENTS,
    // java TRACKEDCHANGES,
    TRACKEDCHANGES,
    // java FORMS;
    FORMS,
}

// java public enum ProtectionType
impl ProtectionType {
}
