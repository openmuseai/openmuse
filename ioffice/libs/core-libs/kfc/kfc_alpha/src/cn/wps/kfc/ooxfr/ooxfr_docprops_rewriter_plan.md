# 原 typed 属性 part

2026-10-29，R0b-36 改写中。按原 OpcrRelationship.getPackagePart 的分支缓存 OxfrAppPart/OxfrCorePart/OxfrCustomPart，向 OxfrPackage 暴露同一缓存对象。parse 继续原 HLXmlReader/PropertiesHandler，保持 partial state、字段默认值及重复 parse。原 OpcUtil.getDate 的 SimpleDateFormat 使用默认时区、lenient Calendar；不得以另一套属性解析器或严格 ISO 日期解析替代。

同步 parser 的 private inner handler 对 outer this 的绑定属于 native 生命周期适配，不改变 ET/OPC 字段与分支。完成有界检查和真机对照后再标记已验证范围；目录整体不提前完成。
R0b-36 已接回原三个 typed part 及 OpcrRelationship 对象缓存；App/Core 原私有 handlers 与重复 parse/异常部分状态经 PKM110 Android 16 对照。默认时区日期叶子仅接受已采集的 852 向量；HL 孤立 UTF-16 代理实体仍需完整模型字符串载体，未声明整个 reader/OPC 门面完成。报告位于 ss-core M4 evidence/m4-validation-r0b36.json。
