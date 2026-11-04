# wr-07-render-command — 平台无关指令

Java 对照：`IRenderDrawer` + `InstractionBuilder.drawText/Line/Rect` + `TypoTextRender`。  
**不移植** `RenderDrawer`（Android Paint）。

```mermaid
flowchart LR
  S[TypoSnapshot] --> W[walk pages/lines/runs]
  W --> E[emit RenderCommand]
  E --> J[Stage3 JSON]
```

must：BeginPage/EndPage、FillRect 页背景、DrawText、DrawLine（下划线最小）。  
stub：DrawPath/Bitmap/Shape。  
defer：instruction_cache 的 int-block / persist。

坐标：twip。color：ARGB8888。
