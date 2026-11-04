# Java→Rust Pipeline MCP Workflow

When the user asks to "run the pipeline", "convert Java files", "rewrite files", or similar, follow this workflow using the `java2rust-pipeline` MCP server tools.

## File Management

每个 Rust crate 目录下包含：

- `rewrite_config.json` — crate 配置（Java/Rust 路径映射、CSV 文件名等）
- `rewriting_files.csv` — 当前正在重写的文件（state: pending/steady/review/fail）
- `rewrite_modules.csv` — 模块级计划：按依赖顺序排列的代码目录列表
- `rewriter_history_steady.csv` — 已确认完成的文件记录
- `rewriter_history_review.csv` — 待人工 review 的文件记录

流水线自动从 `rewriting_files.csv` 按顺序取下一个待处理文件。

## Workflow

### 1. Start

调用 `pipeline_start` 启动流水线。`config` 参数可选，未传则自动从 CWD 向上检测 `rewrite_config.json`：
```
pipeline_start({})
pipeline_start({ config: "<absolute_path_to_rewrite_config.json>" })
```

指定文件：`pipeline_start({ javaFile: "<path>" })`
强制重跑：`pipeline_start({ restart: true })`

### 2. Rewrite the Rust file

根据 `pipeline_start` 返回的模板路径，编辑对应 `.rs` 文件：
- Replace `// TODO_RUST_CODE` markers with equivalent Rust implementations
- Follow the rewrite rules from `REWRITE_GUIDE.md`
- Preserve all `// Java:` comments exactly as they are
- Keep struct/field/method names identical to Java originals
- Apply proper type mappings (int→i32, long→i64, String→String, null→Option<T>)

### 3. Validate & Continue

改写完成后调用 `pipeline_continue`：
```
pipeline_continue({})
pipeline_continue({ config: "<path>", rustPath: "<path>" })
```

该工具会：
1. 运行一致性检查（验证 `// Java:` 注释完整保留）
2. 运行 `cargo check`（编译检查）
3. 若通过 → 标记完成，自动启动下一个文件
4. 若失败 → 返回错误信息和修复指令

### 4. Fix errors (if check fails)

根据返回的错误信息修复 Rust 文件，然后再次调用 `pipeline_continue`，直到通过。

### 5. Loop

重复步骤 2-4，直到所有文件完成。

## Key Rules for Rewriting

1. **Never delete `// Java:` comments** — they are structural markers mapping Java→Rust
2. **Only fill `// TODO_RUST_CODE` markers** — don't restructure the template
3. **Preserve method signatures** — only implement the body
4. **Use `Arc<T>` for shared objects** (not `.clone()` for pool patterns)
5. **`synchronized` → `Mutex`**, `static final` → `const`/`OnceLock`
6. **If unsure about translation, stop and ask** — don't guess complex logic

## Error Recovery

- If a file fails validation after 5 attempts, skip it and move to the next file
- Report skipped files to the user at the end
