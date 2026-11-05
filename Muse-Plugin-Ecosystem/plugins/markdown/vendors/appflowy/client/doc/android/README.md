# Android 构建、打包与生产推送

Android 客户端是 Flutter APK（arm64-v8a）。**不内嵌 Node / DSH sidecar**；DeepSeek 面板走公网 `https://dsh.openmuseai.com`，账号走 `https://openmuseai.com`（见 `config/openmuse-cloud.json`）。

流程：**本机打包 → 真机人工验证 → 手动一键推送**。脚本不会在打包后自动上传。生产 nginx 的 `/downloads/` 指向 `/opt/openmuse-downloads`，上传是 **按目录合并拷贝**，不会清空 macOS / Windows 等其它平台。

官网 catalog 已列出：

`https://openmuseai.com/downloads/android/OpenMuse-0.11.4-android-arm64.apk`

版本号仍是 `0.11.4` 时，只需替换该文件，不必重建官网。

## 用户拿到什么

打包完成后，在 `frontend/client/dist/android/`：

| 产物 | 用途 |
|---|---|
| `OpenMuse-android-release.apk` | 本机构建别名 |
| `OpenMuse-{version}-android-arm64.apk` | 下载页 / 生产文件名 |
| `OpenMuse-android.apk` | 短别名，内容相同 |

用户在 Android 上允许未知来源后安装 APK。Release 仍用 debug 签名（见 `android/app/build.gradle`），同一台设备覆盖安装没问题；换正式签名会变成另一个应用。

`irondash_engine_context` 0.5.5 的 CargoKit 会找新的 `com.flutter.gradle.FlutterPlugin`，本仓库仍用旧版 `flutter.gradle`，Gradle 会跳过该插件。`build-android-client.sh` 会单独编 `libirondash_engine_context_native.so` 放进 app `jniLibs`。

## 打包机要求

- macOS 或 Linux
- Python 3.10+
- Flutter **3.27.x**（`FLUTTER_HOME` 或 `~/sdks/flutters/flutter`）
- JDK **17**（`JAVA_HOME`；macOS 可用 `/usr/libexec/java_home -v 17`）
- Android SDK + NDK（优先 `ANDROID_HOME/ndk/24.0.8215888`，与 `android/app/build.gradle` 对齐）
- `cargo-ndk`、`rustup target add aarch64-linux-android`
- Rust 1.85、cargo-make、`protoc-gen-dart`

不需要本机 Node 闭包。Release 默认注入 `config/openmuse-cloud.json`。

## 一条命令（打包）

在仓库根目录（`Muse-Clients`）：

```bash
python frontend/client/scripts/pack-android-client.py
# 或
./frontend/client/scripts/pack-android-client.sh
```

只重编 Flutter、复用已有 rust-lib / packages：

```bash
python frontend/client/scripts/pack-android-client.py --skip-core --skip-packages
```

Debug（连本机 Cloud，不注入生产 dart-define）：

```bash
python frontend/client/scripts/pack-android-client.py --debug
```

打包结束会跑 `verify-android-apk.py`（arm64 原生库 + `openmuseai.com` / `dsh.openmuseai.com` 字符串）。

## 人工验证

USB 调试打开后：

```bash
./frontend/client/scripts/install-android-client.sh
# 或指定 catalog 名
./frontend/client/scripts/install-android-client.sh \
  frontend/client/dist/android/OpenMuse-0.11.4-android-arm64.apk
```

建议核对：

1. 冷启动能进首页，不闪退
2. 登录走 `openmuseai.com/gotrue`，不是 localhost
3. DeepSeek 面板能打开公网 DSH（`dsh.openmuseai.com`）
4. 文档能打开、能保存

测试账号不要提交进仓库。

## 一键推送到生产（验证 OK 之后）

需要本机有 gitignored 的 `openmuse-io/scripts/deploy.env`（`OPENMUSE_DEPLOY_HOST` 等）。**不要**为此跑 Cloud 的 `deploy-infra.sh`。

```bash
python frontend/client/scripts/push-android-client.py
# 或
./frontend/client/scripts/push-android-client.sh
```

脚本会：

1. 再跑一遍 APK 校验
2. 拷到 `Muse-WebSite/public/downloads/android/OpenMuse-{version}-android-arm64.apk`
3. 暂时移开其它平台目录，只上传 `android/`
4. 调用 `Muse-WebSite/scripts/deploy.py --env production downloads`（远端 merge）

完成后可直接下载：

```bash
curl -sI https://openmuseai.com/downloads/android/OpenMuse-0.11.4-android-arm64.apk
```

应看到 `200` 且 `Content-Type` 为 APK。官网下载页同一 URL；未改 catalog 路径时不必重新部署 Next.js。

指定文件：

```bash
python frontend/client/scripts/push-android-client.py --apk path/to.apk
```

## 脚本

| 脚本 | 作用 |
|---|---|
| [`pack-android-client.py`](../../scripts/pack-android-client.py) | Release APK + catalog 文件名 + 校验 |
| [`pack-android-client.sh`](../../scripts/pack-android-client.sh) | 转调 Python |
| [`build-android-client.sh`](../../scripts/build-android-client.sh) | Flutter + cargo-ndk；被 pack 调用 |
| [`build-mobile-apk.sh`](../../scripts/build-mobile-apk.sh) | `flutter build apk --target-platform android-arm64` |
| [`install-android-client.sh`](../../scripts/install-android-client.sh) | `adb install -r` |
| [`verify-android-apk.py`](../../scripts/verify-android-apk.py) | 原生库与生产域名检查 |
| [`push-android-client.py`](../../scripts/push-android-client.py) | 人工确认后上传生产 `/downloads/android/` |
| [`lib/android-cargokit.sh`](../../scripts/lib/android-cargokit.sh) | 补编 irondash 原生库（CargoKit 与旧 Flutter Gradle 不兼容时） |

## 签名与包名

- 默认 `applicationId` 见 `android/app/build.gradle`（OpenMuse 品牌包名）。覆盖安装旧 `io.appflowy.appflowy` 数据需 `--application-id`（仅 `build-android-client.sh`）。
- Play 商店 / 正式签名不在本流水线内；当前产物适合官网 sidecar 下载与内部验证。
