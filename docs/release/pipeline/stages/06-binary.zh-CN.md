# 06-binary · 最终发行包验证

负责人：测试与构建负责人。状态与证据单独保存在每次发布的 `stages/06-binary.json`。

## 输入与目标

直接检查将要上传的签名包。此节点专门防止“Debug/Profile 正常，归档坏了”的问题。

前置节点：05-build。

## 执行步骤

- 核对包名、版本、构建号、Team、证书指纹、运行地区及英文配置；确认 main 入口。
- iOS：运行 `python3 scripts/release/verify-ios-bundle.py /path/to/Runner.app`，检查相册/麦克风/本地网络用途说明、六个 Notes FFI 导出、assets 原生可执行污染；再验签与 Profile。
- Android：最终 Manifest 的 SDK/权限、ABI、原生库页对齐和渠道签名检查。不要仅检查源码中的 target SDK 变量。
- 用实际 Release/Archive 包安装到真机，打开随包指南，确认图文/链接，编辑保存并连续重开。
- QA 重签与测试 target 只用于验证，不上传；重签产物不能替代正式 IPA 身份记录。
- dSYM 警告保留并跟进，不将“上传成功”解释为 Apple 后处理或人工审核通过。

## 输出与验收

最终包校验结果、实际 SHA-256、签名证书/公钥摘要、Release 真机验收。此案例 Android14 被用户从 Play 安装验证；iOS16 的变化为隐私元数据，iOS15 的图片保持原有来源标签。

证据不足保持 partial/pending；通过后由负责人填写 passed、owner 和 evidence。后续修改使本节点结论失效时，重新打开相关节点。

[返回流水线入口](../README.zh-CN.md)
