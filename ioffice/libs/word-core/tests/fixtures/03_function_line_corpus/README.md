# wr-16 Java Stage2 corpus

`docx/` 是 wr-16 的五类纯文本输入；`java/` 是同名文档由 koffice
`TypoTextLine` 在布局完成后导出的整数 TWIP 行表。它们不是截图，也不是
Rust 生成后复制的期望值。

生成环境：

- Android 14 AOSP ARM64 emulator，fingerprint
  `Android/sdk_phone64_arm64/emu64a:14/UE1A.230829.036.A1/11228894:userdebug/test-keys`
- APK：`koffice/writer/writer-demo` 的 `cnDebug`
- 拉丁回退字体：`/system/fonts/Roboto-Regular.ttf`，SHA-256
  `9ca9debb09459bf4e3e7f826f5cd0f35f253902b85684921fce2ba3f28dd0f50`
- 中文字体：`/system/fonts/NotoSansCJK-Regular.ttc`，SHA-256
  `39fb47c543da50618ab99e8b9e5529e54566bdbef41719308165975f627d5c93`

Java 导出入口为 `LayoutTraceCollector.getStage2Json()`；demo receiver 将结果写成
`koffice_stage2.json`。Rust corpus runner 只消费冻结 JSON，不允许改 Java 的 CP
切分来迁就 Rust。
