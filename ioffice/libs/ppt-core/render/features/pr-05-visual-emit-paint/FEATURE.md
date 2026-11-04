# pr-05-visual-emit-paint — 色块/图片指令与 Flutter 绘制

> 层：render ｜ 状态：**pass** ｜ depends_on: pr-02:03; pp-15; pp-16; pr-04  
> 叶测已有 `DrawImage` 枚举；`render_slide` 与 Flutter painter **未接**。

## 目标

产品 `render_slide` 对 pic/blip 发出 `DrawImage`；对 `hasFill` 附着 ARGB；Flutter 解码 JPEG/PNG 画位图。SVG/WDP 本波占位矩形。

## I/O

| 输入 | 输出 |
|---|---|
| type=2 + media bytes | `DrawImage{x,y,w,h,media_id}` + envelope 可带 inline 或旁路 bytes 表 |
| `sp_pr.hasFill` | DrawRect.fill = ARGB（非 None） |
| Flutter commands | `DrawImage` 不再 skip |
