# OpenMuse CLI 插件实施计划与 Easel 验收

更新：2026-10-06。设计边界见[CLI 设计](OPENMUSE-CLI-PLUGIN-DESIGN.zh-CN.md)；Mobile 仍按[远程 Surface 计划](REMOTE-SURFACE-IMPLEMENTATION-PLAN.zh-CN.md)推进。

## 已完成：macOS 本机试验切片

| 能力 | 实现位置 | 验收结果 |
| --- | --- | --- |
| 可停用 CLI 插件 | `plugins/cli`、`distribution/openmuse_builtin_plugins` | 默认提供底部终端；停用它不阻断编辑器或 DSH |
| 终端布局 | `workbench_shell.dart`、`plugins/cli` | 可拖动高度、折叠，右上角 `+` 新建 shell tab；Mac zsh/bash，Linux bash，Windows PowerShell profile |
| 原生命令入口 | `bin/openmuse.dart`、Mac 打包脚本 | Debug App 中的 launcher 已重建；从截图同一工作目录运行 `openmuse list` 返回已安装插件 |
| 通用包安装协议 | Manifest v2、`plugin_package.dart` | catalog SHA 与 artifact digest 验证、平台检查、`runtime.prepare` 插件安装引导、失败不写 receipt、插件级卸载 |
| Easel 自有业务实现 | `plugins/easel` | 0.3.0 包含打包器、Python venv/Playwright/Chromium 安装脚本、视频封面/加工脚本、抖音 CLI 脚本；Host 无 Easel 业务导入或 UI 入口 |
| 命令发现与 DSH | `contributes.cli`、`commands --json`、`DshWorkspaceBinding` | JSON 包含命令、选项、输入/输出 schema、效果和 stdout/stderr/exit-code 契约；DSH 技能说明与 CLI PATH 由 DSH 插件提供 |
| DSH 端到端 | 本机 DSH headless 会话 | DSH 发现 7 条命令、启动视频加工、捕获阶段/退出码、用 ffprobe 复核、调用抖音离线 plan，均成功；未正式发布 |

真实安装曾在 uv Python 默认复制式 venv 失败；Easel 安装引导改为符号链接并自动清理损坏 venv，重试成功。现已在用户 OpenMuse 数据目录先卸载旧 Easel 设置，再安装 0.3.0；隔离环境中 Playwright 1.63.0、Chromium 与 Headless Shell 安装并通过 `easel douyin check`。素材 `/Users/mac/Documents/OpenMuse/Cap 2026-10-05 at 23.38.12.mp4` 经 DSH 生成 `openmuse-douyin.cover.png` 和 `openmuse-douyin.mp4`，成片 38.501 秒、1920×1080、H.264/AAC。DSH 会话 ID 为 `session-17e39e13-1cdd-4a02-bfa2-8ff57dea2554`，本机事件记录为 `/tmp/openmuse-dsh-easel-session.jsonl`。按最新用户指令仅验证流水线，不执行发布。

## 接下来的开发关卡

| 关卡 | 预计 | 实现内容 | 验收条件 |
| --- | --- | --- | --- |
| P1 Host CLI Gateway | 2–3 周 | 用本机 socket/Windows pipe 统一 CLI 和 UI 的设置、插件安装、停用与并发事务，Linux 用 headless Host；当前 CLI 仍直接读写 settings/receipt | CLI/UI 并发安装只有一份一致 receipt，CLI 插件故障不影响 Host |
| P2 Agent 命令授权与 Job | 3–4 周 | 让 `contributes.cli` 与 `agent_cli` 共享命令描述但分别计算 grant；增加参数校验、审计、Job ID、事件流、取消/恢复；DSH Bash 调用受 WSR Lease 约束 | 插件能力自动可发现，未授权副作用不可调用；DSH 和终端读取相同 Job 状态 |
| P3 Easel 完整发布链 | 4–6 周 | 账号授权远程 Surface、可审预览、发布确认、读回对账和幂等 receipt；把素材加工的更多 Easel 能力扩展为独立命令；准备锁文件/离线依赖闭包 | 测试账号 `素材→预览→确认→发布→读回`；断线 outcome_unknown 不自动重发；Mobile 见同一 Job |
| P4 多平台与布局 | 3–5 周 | 通用底部 dock、Linux/Windows CLI 包与 shell 真机验证、签名/撤销/SBOM、升级回滚 | Mac/Linux/Windows 真机安装、命令隔离、UI 恢复；无匹配 artifact 时命令不可发现 |

以上估算为净开发时间，平台账号审核、代码签名及真实发布窗口另计。当前安装引导从 PyPI 和 Playwright CDN 获取依赖，尚不支持离线发行；安装包仅有 SHA 校验，未有发行签名。Easel 命令由子进程运行，P2 的细粒度沙箱与统一 Job 仍待实现。

## 复现与测试

```bash
DART_BIN=/Users/mac/src/flutter/bin/dart PYTHON_BIN=/Users/mac/.local/bin/python3.11 scripts/test_openmuse_cli.sh
cd app/openmuse_host && /Users/mac/src/flutter/bin/flutter test test/plugin_distribution_test.dart test/workbench_shell_test.dart
cd ../../plugins/dsh-agent && /Users/mac/src/flutter/bin/flutter test test/dsh_workspace_binding_test.dart
```

第一条会在临时工作区完整下载并安装 Playwright/Chromium，随后验证命令发现、离线 plan、自检及短片加工；运行时间与网络有关。Flutter 测试使用无网络安装 fixture，重点验证包/设置/界面协议。实机 DSH 验收使用已有 OpenCode Go 凭据、headless profile 的模型配置及 `x-opencode-session` 会话头；缺模型凭据时会话会明确返回 `MISSING_CREDENTIAL`，不会伪装成功。
