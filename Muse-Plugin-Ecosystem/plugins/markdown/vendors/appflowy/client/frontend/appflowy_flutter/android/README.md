# Muse Android build

From the Muse workspace root, use `scripts/build-android-client.sh` with Flutter
3.27.4, JDK 17, an installed Android SDK / NDK, cargo-ndk 4.1.2, and the
`aarch64-linux-android` target installed for the repository's Rust toolchain.

```bash
export ANDROID_HOME=/path/to/Android/sdk
export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/27.0.12077973"
export JAVA_HOME=/path/to/jdk-17
export MUSE_DSH_PUBLIC_URL=https://dsh.example.com
./scripts/build-android-client.sh --debug --skip-packages
```

The wrapper uses the same NDK for Rust and Gradle, selects Java 17 for Gradle
without changing global Flutter settings, and verifies that the arm64 APK
contains `libdart_ffi.so` before copying it to
`dist/android/AppFlowy-android-debug.apk`. The APK contains no DSH Node runtime.
Use `--skip-core` only after rebuilding the current Rust sources.

The Debug APK is for development, not Play distribution. The upstream Release
configuration also uses the debug signing key until release signing is supplied.
Building the APK does not verify the remote DSH service or device-token handshake.
See `docs/development-plan-v2/09-web-android-product/ANDROID-IMPLEMENTATION.zh-CN.md`
in the Muse workspace for implementation status and remaining integration work.

## Upstream Android instructions

This is a guide on how to build the rust SDK for AppFlowy on android.
Compiling the sdk is easy it just needs a few tweaks.
When compiling for android we need the following pre-requisites:

- Android NDK Tools. (v24 has been tested).
- Cargo NDK. (@latest version).

**Getting the tools**
- Install cargo-ndk ```bash cargo install cargo-ndk```.
- [Download](https://developer.android.com/ndk/downloads/) Android NDK version 24.
- When downloading Android NDK you can get the compressed version as a standalone from the site.
    Or you can download it through [Android Studio](https://developer.android.com/studio).
- After downloading the two you need to set the environment variables. For Windows that's a separate process.
    On macOS and Linux the process is similar.
- The variables needed are '$ANDROID_NDK_HOME', this will point to where the NDK is located.
---

**Cargo Config File**
This code needs to be written in ~/.cargo/config, this helps cargo know where to locate the android tools(linker and archiver).
**NB** Keep in mind just replace 'user' with your own user name. Or just point it to the location of where you put the NDK.

```toml
[target.aarch64-linux-android]
ar = "/home/user/Android/Sdk/ndk/24.0.8215888/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
linker = "/home/user/Android/Sdk/ndk/24.0.8215888/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android29-clang"

[target.armv7-linux-androideabi]
ar = "/home/user/Android/Sdk/ndk/24.0.8215888/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
linker = "/home/user/Android/Sdk/ndk/24.0.8215888/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi29-clang"

[target.i686-linux-android]
ar = "/home/user/Android/Sdk/ndk/24.0.8215888/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
linker = "/home/user/Android/Sdk/ndk/24.0.8215888/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android29-clang"

[target.x86_64-linux-android]
ar = "/home/user/Android/Sdk/ndk/24.0.8215888/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
linker = "/home/user/Android/Sdk/ndk/24.0.8215888/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android29-clang"
```

**Clang Fix**
 In order to get clang to work properly with version 24 you need to create this file.
 libgcc.a, then add this one line.
 ```
 INPUT(-lunwind)
 ```

**Folder path: 'Android/Sdk/ndk/24.0.8215888/toolchains/llvm/prebuilt/linux-x86_64/lib64/clang/14.0.1/lib/linux'.**
After that you have to copy this file into three different folders namely aarch64, arm, i386 and x86_64.
We have to do this so we Android NDK can find clang on our system, if we used NDK 22 we wouldn't have to do this process.
Though using NDK v22 will not give us a lot of features to work with.
This GitHub [issue](https://github.com/fzyzcjy/flutter_rust_bridge/issues/419) explains the reason why we are doing this.

 ---

 **Android NDK**

 After installing the NDK tools for android you should export the PATH to your config file
 (.vimrc, .zshrc, .profile, .bashrc file), That way it can be found.

 ```vim
 export PATH=/home/sean/Android/Sdk/ndk/24.0.8215888
 ```
