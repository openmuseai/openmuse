# Foundation F0 实施包

状态：**进行中**（F0.1 公共合同、F0.2 Host Resource Core、F0.3 DSH Seam 已完成；F0.4–F0.6 待实施）  
开始日期：2026-09-15

本目录是 F0 的评审与交付证据入口：

| 文件 | 用途 |
|---|---|
| [DESIGN.md](DESIGN.md) | 边界、用户行为、合同、状态、安全与技术指标 |
| [DEVELOPMENT.md](DEVELOPMENT.md) | F0.1–F0.6 工作包、路径、依赖、命令和退出条件 |
| [TEST-MATRIX.md](TEST-MATRIX.md) | requirement 到平台、fixture、测试类型和证据的映射 |
| [FIXTURES.md](FIXTURES.md) | golden fixture 设计、命名和扩展规则 |
| [RELEASE.md](RELEASE.md) | flag、兼容、灰度、回滚与观测计划 |
| [RESULTS.md](RESULTS.md) | 已执行验证、实际结果、阻塞项和 Gate 决定 |

F0 总 Gate：一个 fake Resource 能从 DSH 打开 fake Surface 并只产生一个 terminal receipt；所有取消/失败路径资源计数归零；旧 Sidebar 打开路径仍可回退。

当前完成合同层、Host Resource 逻辑内核与 DSH seam；子阶段证据：[F0.2 Host Resource](../F0.2-host-resource/README.md)、[F0.3 DSH Seam](../F0.3-dsh-seam/README.md)。**不得据此开启任何真实引擎 flag**。
