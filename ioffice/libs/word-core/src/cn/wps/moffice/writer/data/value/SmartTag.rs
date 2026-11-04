// Java: cn.wps.moffice.writer.data.value.SmartTag
//
// ★M-STAGE1 P2 —— 新建类型。全仓此前缺失（0 hit），是 `SmartTagTypeImporter`
//   依赖闭包中的一环。
//
// Java 原文仅 3 个 **public 字段**、无方法、无构造器：
// ```java
// public class SmartTag {
//    public String mName = null;
//    public String mNamespaceuri = null;
//    public String mUrl = null;
// }
// ```
// rust: Java 三字段均为 `public` 且初值 `null` ⇒ Rust 侧为 `pub Option<String>`，
//   保持"可为空"这一规格的一部分（与 `CompatSetting` 同类处置）。

// java public class SmartTag
#[derive(Default, Clone)]
pub struct SmartTag {
    // java public String mName = null;
    pub mName: Option<String>,
    // java public String mNamespaceuri = null;
    pub mNamespaceuri: Option<String>,
    // java public String mUrl = null;
    pub mUrl: Option<String>,
}
