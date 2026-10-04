# OpenMuse Mobile ASR 实施计划与验证记录

状态：P0 合同切片与 P1 Android 本地纵向切片已完成；iOS、模型发行、Remote Engine 和发布级性能门禁按后续阶段推进（2026-10-03）。

关联架构：[Mobile ASR Plugin 架构设计](./MOBILE-ASR-PLUGIN-ARCHITECTURE.zh-CN.md)

## 1. 本次交付边界

本次实现的目标不是把实验模型直接宣称为生产模型，而是完成一个可运行、可测试、符合 Plugin 边界的端到端纵向切片：

1. WorkBuddy 只依赖稳定的 `SpeechRecognitionPort`，不依赖 sherpa API；
2. `com.openmuse.speech.input` 作为 Built-in Plugin 管理权限、录音、会话与引擎；
3. 本地 sherpa-onnx Streaming Zipformer 在独立 isolate 运行；
4. partial 只替换一个临时草稿片段，final 保留在输入框中供用户编辑，不自动发送；
5. 长按入口使用真实麦克风 PCM16 流；受信任的 debug seam 可用 WAV 替代麦克风做确定性真机验证；
6. `localOnly`、`preferLocal`、`remoteOnly` 已进入稳定合同。当前插件实现 local Engine；`remoteOnly` 明确失败，不会静默上传或回退；
7. 正常构建不记录 transcript。只有同时启用 debug WAV seam 时输出 E2E 断言标记。

## 2. 工作分解与状态

| 阶段 | 交付物 | 状态 | 退出证据 |
| --- | --- | --- | --- |
| A. 分支与合同 | `codex/mobile-asr-plugin`；session/source/policy/event/error 合同 | 完成 | contract tests 通过 |
| B. Composer Core | 单临时片段 partial/final 替换、取消恢复 selection | 完成 | core tests 与 Widget test 通过 |
| C. Plugin | manifest、生命周期、single-active-session、录音权限与 PCM16 capture | 完成 | analyze/tests 通过 |
| D. Local Engine | sherpa worker isolate、Zipformer INT8、WAV/PCM 两种输入 | 完成 | Android arm64 实机 final 非空 |
| E. Host 接入 | Plugin Registry 装配、键盘/语音模式切换、长按/上滑取消、动画波形、错误恢复 | 完成 | Widget test 与真机 UI 断言通过 |
| F. 平台权限 | Android `RECORD_AUDIO`、iOS usage description | 完成 | manifest/plist 已配置 |
| G. 可重复 E2E | build/install/push fixture/launch/log/UI assertion 脚本 | 完成 | `tool/run_asr_e2e_android.sh` |
| H. 发布闭包 | signed model catalog、下载/回滚、SBOM/notices、iOS 真机 | 待 P2 | 发布门禁 |
| I. Remote Engine | 同合同 provider、显式 consent、scoped token、弱网策略 | 待 P3 | Remote TCK 与安全审计 |
| J. 体验与质量 | corpus、CER/WER、hotword、VAD、profile/soak/热功耗 | 待 P4 | 架构文档第 15–18 节门禁 |

## 3. 代码落点

| 模块 | 职责 |
| --- | --- |
| `packages/muse_speech_contract` | Host/Plugin 稳定合同；不依赖 Flutter 或模型 SDK |
| `packages/muse_speech_core` | 可测试的 Composer 草稿合并规则 |
| `plugins/speech-input` | Plugin 生命周期、capture、engine worker、manifest |
| `app/openmuse_mobile/lib/main.dart` | 编译期装配 Plugin 与 debug-only E2E source |
| `app/openmuse_mobile/lib/workbuddy/workbuddy_shell.dart` | 手势/session 映射与可编辑草稿呈现 |
| `app/openmuse_mobile/tool/run_asr_e2e_android.sh` | Android 真机确定性回归 |

数据流保持为：

```text
WorkBuddy gesture/debug WAV
  -> SpeechRecognitionPort
  -> speech-input session coordinator
  -> microphone PCM16 or trusted WAV
  -> sherpa worker isolate
  -> partial/final text event
  -> SpeechDraftBuffer
  -> editable Composer
```

原始 PCM 不进入 Plugin Registry 事件、DSH 协议或 Agent 消息；用户仍需显式点击发送。

## 4. Android E2E 复现

准备已解压的 `sherpa-onnx-streaming-zipformer-zh-14M-2023-02-23` 和一个 mono PCM WAV，然后执行：

```bash
cd app/openmuse_mobile
OPENMUSE_ASR_MODEL_DIR=/absolute/path/to/sherpa-onnx-streaming-zipformer-zh-14M-2023-02-23 \
OPENMUSE_ASR_TEST_AUDIO=/absolute/path/to/test.wav \
./tool/run_asr_e2e_android.sh
```

脚本执行以下端到端断言：

1. 构建带 arm64 sherpa runtime 的 debug APK；
2. 安装 APK，并将模型与 WAV 放入应用私有目录；
3. 冷启动 App，debug seam 自动创建真实 speech session；
4. 等待非空 `final`，若出现安全错误信息或超时则失败；
5. 使用 Android UIAutomator 确认 final 文本真实存在于 Composer，而非只停留在引擎日志。

## 5. 2026-10-03 真机验证记录

- 设备：ADB serial `3B65BB01B4H00000`，Android arm64，产品型号 `PKM110`；
- APK：Flutter debug，`io.openmuse.openmuse_mobile`；
- 模型：`sherpa-onnx-streaming-zipformer-zh-14M-2023-02-23`；
- 输入：模型包自带 `test_wavs/0.wav`，mono PCM16/16 kHz；
- 输出：`对我做了介绍那么我想说的是大家如果对我的研究感兴趣`；
- UI 断言：UIAutomator 在 Composer 文本输入节点读取到完全相同结果；
- 输入模式断言：点击左侧按钮后 TextField 与软键盘消失，中央区域切换为“按住说话”；再次点击可恢复键盘输入；
- 手势断言：真机长按显示青绿色动画波形层，上滑超过阈值后切换为红色“松手取消”，释放后回到待机且不保留识别片段；
- 登录回归：Mobile debug/release 默认认证端点统一为 `https://openmuseai.com/gotrue`，不再让物理设备访问自身的 `127.0.0.1:9999`；本地服务调试必须显式传入 dart-define；
- 进程断言：识别后进程仍存活；
- 模型和音频在设备私有目录的 SHA-256 与 Host 文件一致；
- 首次失败曾定位到模型类型配置：该模型 metadata 为 `zipformer`，使用 `zipformer2` 会要求不存在的 `query_head_dims` 并使 native runtime 退出；已修复并由成功回归覆盖。

### 语音模式真机回归

- 普通 Debug 构建现在默认从应用私有 `files/speech-model` 加载模型；`OPENMUSE_SPEECH_MODEL_DIR` 仍可显式覆盖。测试模型已通过上述 E2E 脚本安装到手机。
- 录音会先启动，模型冷启动期间的 PCM16 保存在有界缓冲区，待 worker 就绪后按顺序送入识别器；松手会等待录音流结束再完成解码。
- 识别期间 partial 显示在波形层；松手后显示“正在完成识别…”，非空 final 自动切回可编辑文本框且不拉起键盘。空 final 会显示“未识别到语音，请重试”，不再无提示地消失。
- 使用 Mac 扬声器播放同一 `test_wavs/0.wav`，由手机麦克风真实收音，真机 Composer 显示 `介绍那么我想说的是大家如果对我研究感兴趣`，发送按钮可用，软键盘保持关闭。麦克风回放的声学条件与文件注入不同，文字不要求逐字相同。

本记录证明“Mobile Host → Plugin → isolate/native engine → final event → Composer”的文件输入端到端链路已完成。它不替代发布前的真实麦克风噪声 corpus、iOS 真机、长时 soak 和 profile 性能测试。

## 6. 后续实施顺序

1. 模型发行：冻结 upstream revision/license/digest，选择随包或 signed catalog 下载，增加原子激活与回滚；
2. Android/iOS 实机矩阵：权限拒绝、route/interruption、前后台、100 utterance、30 分钟 soak；
3. Plugin SDK Service seam：把当前 typed Dart adapter 收敛到 `openmuse.speech.recognition@1` 的 Broker 注册与 TCK；
4. Remote provider：只在 consent 和组织策略通过后创建 session，禁止 utterance 中静默切换；
5. 质量优化：授权 corpus 上比较 greedy 与 modified beam/hotword，再决定 VAD、标点及候选模型。

任何阶段都不得以“提高成功率”为由绕过用户同意上传音频，也不得把 transcript、PCM 或 context 写入常规日志与遥测。
