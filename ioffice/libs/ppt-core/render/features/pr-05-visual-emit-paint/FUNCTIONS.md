# pr-05-visual-emit-paint — Function 组合图

> 状态：**pass** ｜ 管线：[VISUAL_CONTENT_PIPELINE.md](../../../VISUAL_CONTENT_PIPELINE.md)

## §0 管线位置

```text
pp-15 media + pp-16 ARGB + pl-07 写回
  → [01] render_slide：pic → DrawImage（无 media → 占位无 fill Rect）
  → [02] hasFill 附着（修「只有框线」）
  → [03] Flutter RenderPainter.DrawImage（JPEG/PNG）
```

## §1 清单

| order | function_id | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|
| 1 | `01_function_emit_draw_image` | pic shape + rect | DrawImage 或占位 Rect | pr-02:03;pp-15 | 03 | pass |
| 2 | `02_function_emit_solid_fill` | LayerItem + hasFill | DrawRect.fill 非空 | pp-16;pl-07 | pr-04 JSON | pass |
| 3 | `03_function_painter_draw_image` | DrawImage JSON + bytes | 像素 | 01;pr-04:02 | 用户可见 | pass |

## §2 数据流

```text
geometry_commands 产无 fill 的 Rect
apply_fill_line(fill ARGB)
type==2 || fill blip → DrawImage 替换 Rect
FFI JSON 可增加 pages[].images[] 或 data-URI；MVP 允许 media_id + 宿主侧缓存
```

**裁决：** 23 页 JPEG 走 FFI 旁路表（`media_id` → bytes）比把整图塞进每条命令更合适；FUNCTION 01 锁 JSON 字段。

## §3 测试面

Rust：`render_slide` fixture pic。Dart：`RenderPainter` DrawImage + 最小 PNG 字节。

## §4 验收

Sample_ppt 首页 JSON 含 `DrawImage`；host 能看见图；无 dylib 时 golden 不含图不挡本 feature。

## §5 loop

`01 → 02 → 03`

## §6 类级

| Java | function |
|---|---|
| `GemoRender.drawFill` BlipFill | 01 |
| `DrawingRender` solid | 02 |
| Canvas.drawBitmap | 03 Flutter；无 Java 移植目标 |
