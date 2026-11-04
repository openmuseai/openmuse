# PDF test fixtures

| 文件 | 用途 | SHA-256 |
|---|---|---|
| `testfile_pdf_clip.pdf` | 5 页明文 Document→Page 契约 | `3975c975fe3e60ec535821e8fe2e2cc769f9fc3e4aff0f88164b8bea755d779b` |
| `password-user123.pdf` | 密码 pending/reopen 契约 | `d89d61608d98e9c26b2fc830e86eca4bb0b0988c70a2e20595ca678b6676466b` |
| `rtl-edit-corpus.pdf` | WT6c2a Hebrew 视觉枚举、方向矩形、逻辑 caret | `e98d91e4f8e2f710c0cbd576202bfccd3315e7be11bc7a979631b2f7f9d5101b` |

密码样本由固定的原 SO 通过 `PDFDocument.native_setPasswords` 生成，测试用户密码为 `user123`，owner 密码为 `owner123`，permission 为 `0x0F3C`。这些都是仓库内的固定测试数据，不得替换为真实密码或敏感文档。

`rtl-edit-corpus.pdf` 由 `scripts/generate_rtl_edit_corpus.py` 确定性生成，嵌入的
`fonts/NotoSansHebrew-WT6-subset.ttf` 是 Google Fonts 的 Noto Sans Hebrew 子集；
许可证归档在 `fonts/NotoSansHebrew-OFL.txt`。生成器和字体输入发生变化时必须同步
更新 SHA-256，并重新执行原/rebuild Android 对拍。
