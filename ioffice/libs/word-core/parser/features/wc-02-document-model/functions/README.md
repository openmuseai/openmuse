# functions/ — 本 feature 的最小功能单元

索引与串联说明在上一级 [FUNCTIONS.md](../FUNCTIONS.md)。

目录名 `{NN}_function_*`：`NN` 为依赖拓扑序号，`ls` 顺序即编译/测试顺序。

每个目录含：

- `FUNCTION.md` — I/O、上下游、类图、调用关系、**测试方案**（§3）

可执行测试**不**在本目录。它们与 `src` 隔离，镜像落在对应 crate：

```text
libs/word-core/src/<relpath>/<Type>.rs      实现
libs/word-core/tests/<relpath>/<Type>.rs    测试
```

运行：`cargo test --manifest-path libs/word-core/Cargo.toml --test <function_id>`。

改写-测试-回归 loop：`.cursor/skills/feature-function/SKILL.md`。

**09 例外**：`SubDocument` 按 I/O 切，不按类切。开始 `09_function_subdocument_own` 前先读 [docs/09-subdocument-own-scope.md](../docs/09-subdocument-own-scope.md)，不要顺着 ctor 去改 `KRange`。
