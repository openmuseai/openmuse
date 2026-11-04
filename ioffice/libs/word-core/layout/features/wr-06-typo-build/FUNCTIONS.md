# wr-06-typo-build — 布局→快照组装与失效

> 分析日期：2026-08-22
> Java 底稿：`writer/core/cache/TypoDocument.java`（`getSnapshot/newSnapshot/commitSnapshot/createEmptyTypoSnapshot/addSnapshotListener`）+ `TypoSnapshot.java`（fork/cleanFork/版本）
> 输入：分页结果（wr-05 页序列）
> 输出：`TypoSnapshot`（页对象集 + 版本）——渲染与对比的交付物
> 切分依据：I/O 边界（页事件流 → 快照提交；监听者通知）

## 1. Java 管线分析
`TypoDocument` 持有当前 `TypoSnapshot` 与版本：`createEmptyTypoSnapshot`（空快照 VER_EMPTY）→ 排版器写入页对象 → `newSnapshot()`（fork 新版本，VER_FIRSTCACHE）→ 布局完成后 `commitSnapshot(snapshot)`（置 VER_VALIDBEGIN 并通知 `ISnapshotListener`）。`getCurVersion` 供渲染侧判断快照变化。Rust 对标：`TypoDocument { snapshot: Option<TypoSnapshot>, version: i64, listeners: Vec<...> }`，`commit` 时版本+1 + 通知。

## 2. Function 清单
| order | function_id | 一句话 | 输入 | 输出 | 上游 | 下游 |
|------|-------------|--------|------|------|------|------|
| 1 | [01_function_typo_document](./functions/01_function_typo_document/FUNCTION.md) | TypoDocument 宿主：快照持有一版版+监听 | 页事件流/commit | 当前快照+version+通知 | wr-01; wr-05 | 02; wr-07 |
| 2 | [02_function_snapshot_build](./functions/02_function_snapshot_build/FUNCTION.md) | 页序列→快照（页列表+版本标记） | 页对象序列 | TypoSnapshot（VER_VALIDBEGIN） | wr-01; wr-05 | 01; wr-07 |

## 3. 依赖与编译面
src（loop 新建）：`libs/word-core/src/cn/wps/moffice/writer/cache/{TypoDocument,SnapshotBuild}.rs`。crate `libs/word-core`。

## 4. 验收顺序
```text
01 typo_document（初始 VER_EMPTY；commit→VER + 通知一次）
02 snapshot_build（空/多页快照；VER 置 VALIDBEGIN；fork 延后）
```