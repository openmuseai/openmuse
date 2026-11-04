# wr-25-line-compress

> status: **pass**  
> Java：`LineCompressEnv` 记账 + `LineCompressTool.compress` 压缩后再 hang / rollback。

**I：** 已排满但 `compress` 后仍可能 fit 的行（可压缩空白/标点 + overflow graph）。  
**O：** 压缩后的行宽或 hang 成功；否则 graphPos 回退 → wr-12 rollback。
