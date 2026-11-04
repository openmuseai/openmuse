# tests/ — 与 `src/` 隔离的 function 测试

目录结构必须与 `src/` 相同：

```text
src/cn/wps/io/opc/OPCPackage.rs
tests/cn/wps/io/opc/OPCPackage.rs
```

禁止在 `src/` 写 `#[cfg(test)]`。每个 function 在 `Cargo.toml` 登记一条 `[[test]]`，
`name` = `function_id`（如 `02_function_unzip`）。

可选数据：`tests/fixtures/<function_id>/`。

约定见 `.cursor/skills/feature-function/SKILL.md`。
