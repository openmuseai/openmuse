# 原 docProps 读取链

2026-10-29，R0b-36 改写中。先恢复原 OPC typed part 缓存、OxfrApp/Core/CustomPart 及其原 handler，再接 ET XlsxrApp/Core/Custom、PropertiesHandler 和 XlsxReader 原调用点。保留原字段/方法顺序、重复 parse 的状态、partial failure 后继续赋值、catch Throwable/PasswordCancelException 和原日期默认时区。

现有 R0b-32～35 只证明直接创建 settings 的写出与 SST/持久化，不能作为导入既有属性再保存的验收。新增逻辑逐项 `// java`，完成原依赖和真机对照后才接受读取闭包。完整 raw、事务/依赖和整本 ET 保存仍持续实施。
R0b-36 对照通过：实际 typed part 缓存和原 XlsxReader 三个调用点已接回，52 夹具/51 属性包读写再读与原 Java 真机一致。日历与 ICU 适配限 zh-Hans-CN、四时区、852 向量；HL 孤立代理实体、任意 Java String 身份及触发 PasswordCancelException 未验。详见 `evidence/m4-validation-r0b36.json`；不代表全工作簿保存完成。
