# wr-01-typo-cache — Function 组合图

> 切分依据：I/O 边界（页结构 → 行结构 → 快照容器 → 行迭代）。不切竞技场内部 `alloc`。  
> 类级蓝皮书：[STRUCTURE_ANALYSIS.md](./STRUCTURE_ANALYSIS.md)

```text
01_function_typo_page         页：cp 区间 + 边距 + 行宿主
02_function_typo_line         行：几何 + cp + flags
03_function_typo_snapshot     快照：页集合 + version
04_function_typo_iterator     页→行迭代 + contains(cp) 逆查
```

热路径：

```text
TypoSnapshot::new()
  → add_page(TypoPage { start, end, margins })
  → page.push_line(TypoTextLine { y, h, start_cp, end_cp, flags })
  → snapshot.page(0).lines() / iterator
  → line.contains 或 page.contains(cp)
```

**钉死：** 半开区间；JSON 字段与 Java getter 同名（`startCP`/`endCP`/`marginLeft`…）。

crate：`libs/word-core`。测试：`tests/cn/wps/moffice/writer/cache/<Type>.rs`。`[[test]]` name = function_id。
