# docProps 原类续行

2026-10-29，R0b-32 已完成本段真机对照，整个门面/目录继续实施。沿原 `KmoBook.getAppSettings → KmoAppSettings → XlsxwApp/XlsxwCore/XlsxwCustom → OxfwPackage → KFS`，逐类、逐字段、逐方法手工移植；保留 `// java` 与原控制流。原日期引用、ICV、null/错误类型、原 map 遍历和异常前 XML 的 1,081 组属性、21 个状态阶段及 24 个 ZIP 零差，见 M4 里程碑 08 与 verify-r0b32.py。

本段以 PKM110 / Android 16 原实现为观察源；原 UUID 保持随机，观察生成字节供 native 熵叶重放，时钟只重放原 Date 构造值。原始随机熵未观察，不扩称随机源等价。

尚未接受整个 `KmoAppSettings` Java 对象门面：StringMap 仅非 null String 键 put/get/遍历，没有完整 Map 删除/null 键/迭代器并发修改语义；Java String 引用及任意 UTF-16、原 book 生命周期外引用不在已验证范围。Property 只覆盖构造值及访问器，不包含原二进制读取。整个 ET 保存、完整 raw、execute/export 仍未接通。下一步继续原 FontsLabel/SstLabel、其他 label、XlsxWriter 与 saveImpl。

本段 settings 是观察器和测试直接创建/赋值的原对象，不证明已有文件属性导入后再保存的保真；原 xlsxr/docProps 三类和 handler/docProps/PropertiesHandler 也必须沿原依赖补齐，不能靠新建默认 settings 接受保存。
