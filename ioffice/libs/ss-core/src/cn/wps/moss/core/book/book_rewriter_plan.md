# book 原类续行

2026-10-29，R0b-32 已手工移植 KmoAppSettings 原 23 个实例字段、8 个常量与 53 个方法，包括 ICV 状态、nullable 字符串、日期/字节数组引用和阅读布局。调用方沿原 KmoBook 三个 getter/setter 切片进入原 docProps writer。

真机 21 个状态阶段及属性 ZIP 对照，源码/参数/字段顺序验证见 M4 `fixtures/verify-r0b32.py`。这不是整个目录、整个 Java Map/String 对象门面或 BIFF Property 二进制读取的完成标记；相关边界见 M4 docProps 计划和机器报告。继续原 ET 保存与完整 raw/事务管线，M4 未验收。
