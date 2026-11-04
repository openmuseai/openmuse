// Java: import cn.wps.core.runtime.ISharedInfo
use util_adapter::cn::wps::core::runtime::ISharedInfo::*;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::cn::wps::core::runtime::Platform::*;
// Java: import java.util.HashMap
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import java.lang.ThreadLocal
use stubs::java::lang::ThreadLocal::*;
use std::collections::{HashMap};

// java static ISharedInfo sSharedInfo = new DefaultVersionProxy();
// rust: ★M-STAGE2 P1-D1 —— 由 `LazyLock<DefaultVersionProxy>` 修正为
//   `Mutex<Box<dyn ISharedInfo + Send + Sync>>`：Java 侧该静态量**可重赋值**
//   且持有**多态引用**（`fourceUseTestInfo()` 会整体换掉），
//   `LazyLock<具体类型>` 两点都表达不了 —— 这是 55 个错误的总根源。
pub(crate) static sSharedInfo: std::sync::LazyLock<
    std::sync::Mutex<Box<dyn ISharedInfo + Send + Sync>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Box::new(DefaultVersionProxy::default())));

// java public final class VersionState
#[derive(Default)]
pub struct VersionState {
}

// java static class DefaultVersionProxy implements ISharedInfo
#[derive(Default)]
pub struct DefaultVersionProxy {
}

// java static class UnitTestVersionProxy implements ISharedInfo
pub struct UnitTestVersionProxy {
    // java private ThreadLocal<UnitTestVersionProxy> threadInfo = new ThreadLocal();
    threadInfo: Option<Box<dyn std::any::Any + Send + Sync>>,
    // java private boolean mIsRomReadMode = false;
    mIsRomReadMode: bool,
    // java private boolean mIsProVersion = false;
    mIsProVersion: bool,
    // java private boolean mIsRecordVersion = false;
    mIsRecordVersion: bool,
    // java private boolean mIsOemPhoneShrink = false;
    mIsOemPhoneShrink: bool,
    // java private boolean mIsMultiWindowVersion = false;
    mIsMultiWindowVersion: bool,
    // java private boolean mIsPadScreen = false;
    mIsPadScreen: bool,
    // java private boolean mIsDebugLogVersion = false;
    mIsDebugLogVersion: bool,
    // java private boolean mIsOverseaVersion = false;
    mIsOverseaVersion: bool,
    // java private boolean mIsAndroidN = false;
    mIsAndroidN: bool,
    // java private int[] mDisplayWidthAndHeight = new int[0];
    mDisplayWidthAndHeight: Vec<i32>,
    // java private boolean mIsLeshiDevice = false;
    mIsLeshiDevice: bool,
    // java private boolean mIsOnlyHuaweiDevice = false;
    mIsOnlyHuaweiDevice: bool,
    // java private boolean mIsHonorDevice = false;
    mIsHonorDevice: bool,
    // java private String mDeviceBrand = null;
    // rust: Java 的 `null` 以空串表达（无 null 判定的调用方）。
    mDeviceBrand: String,
    // java private HashMap<String, Boolean> mIsParamsOn = new HashMap();
    mIsParamsOn: HashMap<String, bool>,
}

// rust: 手写 Default —— Java 的字段初始化器（`= new HashMap()` / `= new int[0]`）
//   不是 `#[derive(Default)]` 能表达的（`bool` 默认 false 与 Java 一致，
//   但容器与 String 需要显式构造）。
impl Default for UnitTestVersionProxy {
    fn default() -> Self {
        Self {
            threadInfo: None,
            mIsRomReadMode: false,
            mIsProVersion: false,
            mIsRecordVersion: false,
            mIsOemPhoneShrink: false,
            mIsMultiWindowVersion: false,
            mIsPadScreen: false,
            mIsDebugLogVersion: false,
            mIsOverseaVersion: false,
            mIsAndroidN: false,
            mDisplayWidthAndHeight: Vec::new(),
            mIsLeshiDevice: false,
            mIsOnlyHuaweiDevice: false,
            mIsHonorDevice: false,
            mDeviceBrand: String::new(),
            mIsParamsOn: HashMap::new(),
        }
    }
}

// java public final class VersionState
impl VersionState {
    // java private  VersionState()
    pub fn VersionState() -> Self {
      Self {}
    }

    // java public static boolean isInRomReadMode()
    pub fn isInRomReadMode() -> bool {
      // java return sSharedInfo.isInRomReadMode();
      return sSharedInfo.lock().unwrap().isInRomReadMode();
    }

    // java public static boolean isProVersion()
    pub fn isProVersion() -> bool {
      // java return sSharedInfo.isProVersion();
      return sSharedInfo.lock().unwrap().isProVersion();
    }

    // java public static boolean isOnlyHuaweiDevice()
    pub fn isOnlyHuaweiDevice() -> bool {
      // java return sSharedInfo.isOnlyHuaweiDevice();
      return sSharedInfo.lock().unwrap().isOnlyHuaweiDevice();
    }

    // java public static boolean isHonorDevice()
    pub fn isHonorDevice() -> bool {
      // java return sSharedInfo.isHonorDevice();
      return sSharedInfo.lock().unwrap().isHonorDevice();
    }

    // java public static String getDeviceBrand()
    pub fn getDeviceBrand() -> String {
      // java return sSharedInfo.getDeviceBrand();
      return sSharedInfo.lock().unwrap().getDeviceBrand();
    }

    // java static UnitTestVersionProxy fourceUseTestInfo()
    pub fn fourceUseTestInfo() -> UnitTestVersionProxy {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      //   实现路径：为 `ISharedInfo` 增设 `as_any()`（Java 的 `instanceof` + 强转等价物），
      //   再对下列静态量加锁后 `downcast_ref::<UnitTestVersionProxy>()`。
      //   Java 侧该静态量是**可变静态重赋值**加 `instanceof` + 强转；
      //   Rust 需 `Any` 向下转型 + 静态可变性，成本高。
      //   实测该家族（`fourceUseTestInfo` / `testSet*` / `testGet*`）全仓
      //   **0 调用方**，且不参与"渲染出正确文档"的管线 ⇒ 按 DEFERRED_NON_FUNCTION 延迟。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!(sSharedInfo instanceof UnitTestVersionProxy)) {
      // java    sSharedInfo = new UnitTestVersionProxy();
      // java }
      // java return (UnitTestVersionProxy)sSharedInfo;
      */
    }

    // java public static void testSetInRomReadMode(boolean value)
    pub fn testSetInRomReadMode(value: bool) {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java fourceUseTestInfo().setIsInRomReadMode(value);
      */
    }

    // java public static void testSetIsProVersion(boolean value)
    pub fn testSetIsProVersion(value: bool) {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java fourceUseTestInfo().setIsProVersion(value);
      */
    }

    // java public static void testSetRecordVersion(boolean value)
    pub fn testSetRecordVersion(value: bool) {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java fourceUseTestInfo().setRecordVersion(value);
      */
    }

    // java public static boolean testGetIsRecordVersion()
    pub fn testGetIsRecordVersion() -> bool {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return fourceUseTestInfo().isRecordVersion();
      */
    }

    // java public static void testSetOemPhoneShrink(boolean value)
    pub fn testSetOemPhoneShrink(value: bool) {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java fourceUseTestInfo().setOemPhoneShrink(value);
      */
    }

    // java public static boolean testGetIsOemPhoneShrink()
    pub fn testGetIsOemPhoneShrink() -> bool {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return fourceUseTestInfo().isOemPhoneShrink();
      */
    }

    // java public static void testSetMultiWindowVersion(boolean value)
    pub fn testSetMultiWindowVersion(value: bool) {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java fourceUseTestInfo().setMultiWindowVersion(value);
      */
    }

    // java public static boolean testGetIsMultiWindowVersion()
    pub fn testGetIsMultiWindowVersion() -> bool {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return fourceUseTestInfo().isMultiWindowVersion();
      */
    }

    // java public static void testSetDebugLogVersion(boolean value)
    pub fn testSetDebugLogVersion(value: bool) {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java fourceUseTestInfo().setDebugLogVersion(value);
      */
    }

    // java public static boolean testGetIsDebugLogVersionn()
    pub fn testGetIsDebugLogVersionn() -> bool {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return fourceUseTestInfo().isDebugLogVersion();
      */
    }

    // java public static void testSetOverseaVersion(boolean value)
    pub fn testSetOverseaVersion(value: bool) {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java fourceUseTestInfo().setOverseaVersion(value);
      */
    }

    // java public static boolean testGetOverseaVersion()
    pub fn testGetOverseaVersion() -> bool {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return fourceUseTestInfo().isOverseaVersion();
      */
    }

    // java public static void testSetAndroidN(boolean value)
    pub fn testSetAndroidN(value: bool) {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java fourceUseTestInfo().setAndroidN(value);
      */
    }

    // java public static boolean testGetIsAndroidN()
    pub fn testGetIsAndroidN() -> bool {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return fourceUseTestInfo().isAndroidN();
      */
    }

    // java public static void testSetPadScreen(boolean value)
    pub fn testSetPadScreen(value: bool) {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java fourceUseTestInfo().setPadScreen(value);
      */
    }

    // java public static boolean testGetIsPadScreen(Object o)
    pub fn testGetIsPadScreen() -> bool {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return fourceUseTestInfo().isPadScreen(o);
      */
    }

    // java public static void testSetDisplayWidthAndHeight(int[] value)
    pub fn testSetDisplayWidthAndHeight() {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java fourceUseTestInfo().setDisplayWidthAndHeight(value);
      */
    }

    // java public static int[] testGetDisplayWidthAndHeight(Object o)
    pub fn testGetDisplayWidthAndHeight(o: &dyn Object) -> Vec<i32> {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return fourceUseTestInfo().getDisplayWidthAndHeight(o);
      */
    }

    // java public static boolean testGetParamOn(String s)
    pub fn testGetParamOn() -> bool {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean paramsOn = fourceUseTestInfo().isParamsOn(s);
      // java return paramsOn;
      */
    }

    // java public static void testSetLeshiDevice(boolean value)
    pub fn testSetLeshiDevice(value: bool) {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java fourceUseTestInfo().setLeshiDevice(value);
      */
    }

    // java public static boolean testGetLeshiDevice()
    pub fn testGetLeshiDevice() -> bool {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return fourceUseTestInfo().isLeshiDevice();
      */
    }

    // java public static void testSetOnlyHuaweiDevice(boolean value)
    pub fn testSetOnlyHuaweiDevice(value: bool) {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java fourceUseTestInfo().setOnlyHuaweiDevice(value);
      */
    }

    // java public static boolean testGetOnlyHuaweiDevice()
    pub fn testGetOnlyHuaweiDevice() -> bool {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return fourceUseTestInfo().isOnlyHuaweiDevice();
      */
    }

    // java public static void testSetHonorDevice(boolean value)
    pub fn testSetHonorDevice(value: bool) {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java fourceUseTestInfo().setHonorDevice(value);
      */
    }

    // java public static boolean testGetHonorDevice()
    pub fn testGetHonorDevice() -> bool {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return fourceUseTestInfo().isHonorDevice();
      */
    }

    // java public static void testSetDeviceBrand(String value)
    pub fn testSetDeviceBrand() {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java fourceUseTestInfo().setDeviceBrand(value);
      */
    }

    // java public static String testGetDeviceBrand()
    pub fn testGetDeviceBrand() -> String {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return fourceUseTestInfo().getDeviceBrand();
      */
    }

    // java public static void testSetParamOn(String key, boolean value)
    pub fn testSetParamOn(key: &str, value: bool) {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java fourceUseTestInfo().setParamsOn(key, value);
      */
    }

}

// java static class DefaultVersionProxy implements ISharedInfo
impl ISharedInfo for DefaultVersionProxy {
    // java public boolean isInRomReadMode()
    fn isInRomReadMode(&self) -> bool {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return sharedInfo != null ? sharedInfo.isInRomReadMode() : false;
      return sharedInfo.map_or(false, |s| s.isInRomReadMode());
    }

    // java public boolean isProVersion()
    fn isProVersion(&self) -> bool {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return sharedInfo != null ? sharedInfo.isProVersion() : false;
      return sharedInfo.map_or(false, |s| s.isProVersion());
    }

    // java public boolean isRecordVersion()
    fn isRecordVersion(&self) -> bool {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return sharedInfo != null ? sharedInfo.isRecordVersion() : false;
      return sharedInfo.map_or(false, |s| s.isRecordVersion());
    }

    // java public boolean isOemPhoneShrink()
    fn isOemPhoneShrink(&self) -> bool {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return sharedInfo != null ? sharedInfo.isOemPhoneShrink() : false;
      return sharedInfo.map_or(false, |s| s.isOemPhoneShrink());
    }

    // java public boolean isMultiWindowVersion()
    fn isMultiWindowVersion(&self) -> bool {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return sharedInfo != null ? sharedInfo.isMultiWindowVersion() : false;
      return sharedInfo.map_or(false, |s| s.isMultiWindowVersion());
    }

    // java public int[] getDisplayWidthAndHeight(Object o)
    fn getDisplayWidthAndHeight(&self, o: &dyn std::any::Any) -> Vec<i32> {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return null != sharedInfo ? sharedInfo.getDisplayWidthAndHeight(o) : new int[0];
      return sharedInfo.map_or_else(|| vec![Default::default(); 0usize], |s| s.getDisplayWidthAndHeight(o));
    }

    // java public boolean isPadScreen(Object context)
    fn isPadScreen(&self, context: &dyn std::any::Any) -> bool {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return sharedInfo != null ? sharedInfo.isPadScreen(context) : false;
      return sharedInfo.map_or(false, |s| s.isPadScreen(context));
    }

    // java public boolean isDebugLogVersion()
    fn isDebugLogVersion(&self) -> bool {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return sharedInfo != null ? sharedInfo.isDebugLogVersion() : false;
      return sharedInfo.map_or(false, |s| s.isDebugLogVersion());
    }

    // java public boolean isOverseaVersion()
    fn isOverseaVersion(&self) -> bool {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return sharedInfo != null ? sharedInfo.isOverseaVersion() : false;
      return sharedInfo.map_or(false, |s| s.isOverseaVersion());
    }

    // java public boolean isAndroidN()
    fn isAndroidN(&self) -> bool {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return sharedInfo != null ? sharedInfo.isAndroidN() : false;
      return sharedInfo.map_or(false, |s| s.isAndroidN());
    }

    // java public boolean isParamsOn(String key)
    fn isParamsOn(&self, key: &str) -> bool {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return sharedInfo != null ? sharedInfo.isParamsOn(key) : false;
      return sharedInfo.map_or(false, |s| s.isParamsOn(key));
    }

    // java public boolean isLeshiDevice()
    fn isLeshiDevice(&self) -> bool {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return sharedInfo != null ? sharedInfo.isLeshiDevice() : false;
      return sharedInfo.map_or(false, |s| s.isLeshiDevice());
    }

    // java public boolean isShowInHWScreen(Object o)
    fn isShowInHWScreen(&self, o: &dyn std::any::Any) -> bool {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return sharedInfo != null ? sharedInfo.isShowInHWScreen(o) : false;
      return sharedInfo.map_or(false, |s| s.isShowInHWScreen(o));
    }

    // java public boolean isOnlyHuaweiDevice()
    fn isOnlyHuaweiDevice(&self) -> bool {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return sharedInfo != null ? sharedInfo.isOnlyHuaweiDevice() : false;
      return sharedInfo.map_or(false, |s| s.isOnlyHuaweiDevice());
    }

    // java public boolean isHonorDevice()
    fn isHonorDevice(&self) -> bool {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return sharedInfo != null ? sharedInfo.isHonorDevice() : false;
      return sharedInfo.map_or(false, |s| s.isHonorDevice());
    }

    // java public String getDeviceBrand()
    fn getDeviceBrand(&self) -> String {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java return sharedInfo != null ? sharedInfo.getDeviceBrand() : null;
      // rust: Java 返回 `null` 时 Rust 侧以空串表达（调用方 `ShapeTool::getPenKitName`
      //   只做字符串拼接，不做 null 判定）。
      return sharedInfo.map_or_else(String::new, |s| s.getDeviceBrand());
    }

}

// java static class UnitTestVersionProxy implements ISharedInfo
impl UnitTestVersionProxy {
    // java   UnitTestVersionProxy()
    pub fn UnitTestVersionProxy() -> Self {
      // java ISharedInfo sharedInfo = Platform.getSharedInfo();
      let sharedInfo = Platform::getSharedInfo();
      // java if (sharedInfo != null) {
      // java    this.mIsRomReadMode = sharedInfo.isInRomReadMode();
      // java    this.mIsProVersion = sharedInfo.isProVersion();
      // java }
      let mut me = Self::default();
      if let Some(si) = sharedInfo {
        me.mIsRomReadMode = si.isInRomReadMode();
        me.mIsProVersion = si.isProVersion();
      }
      // java return;（隐式返回 this）
      return me;
    }

    // java  UnitTestVersionProxy threadGet()
    pub fn threadGet(&mut self) -> UnitTestVersionProxy {
      // rust: ★M-STAGE2 P1-D1 —— DEFERRED_NON_FUNCTION（0 调用方、非管线）。
      //   `info` 须回指 ThreadLocal 内的同一实例，Rust 需 `thread_local!` +
      //   `Rc<RefCell<..>>` 自引用，成本高。
      //   Java 侧 `threadInfo` 是 `ThreadLocal<UnitTestVersionProxy>`，
      //   `threadGet()` 返回**存进 ThreadLocal 的那个实例的引用**，
      //   故 setter 的 `this.threadGet().mIsX = value;` 能持久生效。
      //   Rust 表达该自引用需要 `thread_local!` + `Rc<RefCell<..>>` 回指，
      //   而 `UnitTestVersionProxy` 全仓 **0 调用方**且不参与渲染管线
      //   ⇒ 按 DEFERRED_NON_FUNCTION 延迟。
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java UnitTestVersionProxy info = (UnitTestVersionProxy)this.threadInfo.get();
      // java if (info == null) {
      // java    info = new UnitTestVersionProxy();
      // java    this.threadInfo.set(info);
      // java }
      // java return info;
      */
    }

    // java public boolean isInRomReadMode()
    pub fn isInRomReadMode(&mut self) -> bool {
      // java return this.threadGet().mIsRomReadMode;
      return self.mIsRomReadMode.clone();
    }

    // java public boolean isProVersion()
    pub fn isProVersion(&mut self) -> bool {
      // java return this.threadGet().mIsProVersion;
      return self.mIsProVersion.clone();
    }

    // java public boolean isRecordVersion()
    pub fn isRecordVersion(&mut self) -> bool {
      // java return this.threadGet().mIsRecordVersion;
      return self.mIsRecordVersion.clone();
    }

    // java public boolean isOemPhoneShrink()
    pub fn isOemPhoneShrink(&mut self) -> bool {
      // java return this.threadGet().mIsOemPhoneShrink;
      return self.mIsOemPhoneShrink.clone();
    }

    // java public boolean isMultiWindowVersion()
    pub fn isMultiWindowVersion(&mut self) -> bool {
      // java return this.threadGet().mIsMultiWindowVersion;
      return self.mIsMultiWindowVersion.clone();
    }

    // java public int[] getDisplayWidthAndHeight(Object o)
    pub fn getDisplayWidthAndHeight(&self, o: &dyn std::any::Any) -> Vec<i32> {
      // java return this.mDisplayWidthAndHeight;
      // rust: Java 数组按引用返回；Rust 侧 `Vec` 需 clone 以免移动出 `&self`。
      return self.mDisplayWidthAndHeight.clone();
    }

    // java public boolean isPadScreen(Object o)
    pub fn isPadScreen(&mut self, o: &dyn Object) -> bool {
      // java return this.threadGet().mIsPadScreen;
      return self.mIsPadScreen.clone();
    }

    // java public boolean isDebugLogVersion()
    pub fn isDebugLogVersion(&mut self) -> bool {
      // java return this.threadGet().mIsDebugLogVersion;
      return self.mIsDebugLogVersion.clone();
    }

    // java public boolean isOverseaVersion()
    pub fn isOverseaVersion(&mut self) -> bool {
      // java return this.threadGet().mIsOverseaVersion;
      return self.mIsOverseaVersion.clone();
    }

    // java public boolean isShowInHWScreen(Object o)
    pub fn isShowInHWScreen(&self, o: &dyn std::any::Any) -> bool {
      // java return false;
      return false;
    }

    // java public boolean isAndroidN()
    pub fn isAndroidN(&mut self) -> bool {
      // java return this.threadGet().mIsAndroidN;
      return self.mIsAndroidN.clone();
    }

    // java public boolean isParamsOn(String s)
    pub fn isParamsOn(&mut self, s: &str) -> bool {
      // java Boolean val = (Boolean)this.threadGet().mIsParamsOn.get(s);
      let val: Option<bool> = self.mIsParamsOn.get(s).copied();
      // java return null != val ? val : false;
      return val.unwrap_or(false);
    }

    // java public boolean isLeshiDevice()
    pub fn isLeshiDevice(&mut self) -> bool {
      // java return this.threadGet().mIsLeshiDevice;
      return self.mIsLeshiDevice.clone();
    }

    // java public boolean isOnlyHuaweiDevice()
    pub fn isOnlyHuaweiDevice(&mut self) -> bool {
      // java return this.threadGet().mIsOnlyHuaweiDevice;
      return self.mIsOnlyHuaweiDevice.clone();
    }

    // java public boolean isHonorDevice()
    pub fn isHonorDevice(&mut self) -> bool {
      // java return this.threadGet().mIsHonorDevice;
      return self.mIsHonorDevice.clone();
    }

    // java public String getDeviceBrand()
    pub fn getDeviceBrand(&mut self) -> String {
      // java return this.threadGet().mDeviceBrand;
      return self.mDeviceBrand.clone();
    }

    // java public void setIsInRomReadMode(boolean value)
    pub fn setIsInRomReadMode(&mut self, value: bool) {
      // java this.threadGet().mIsRomReadMode = value;
      self.mIsRomReadMode = value;
    }

    // java public void setIsProVersion(boolean value)
    pub fn setIsProVersion(&mut self, value: bool) {
      // java this.threadGet().mIsProVersion = value;
      self.mIsProVersion = value;
    }

    // java public void setRecordVersion(boolean value)
    pub fn setRecordVersion(&mut self, value: bool) {
      // java this.threadGet().mIsRecordVersion = value;
      self.mIsRecordVersion = value;
    }

    // java public void setOemPhoneShrink(boolean value)
    pub fn setOemPhoneShrink(&mut self, value: bool) {
      // java this.threadGet().mIsOemPhoneShrink = value;
      self.mIsOemPhoneShrink = value;
    }

    // java public void setMultiWindowVersion(boolean value)
    pub fn setMultiWindowVersion(&mut self, value: bool) {
      // java this.threadGet().mIsMultiWindowVersion = value;
      self.mIsMultiWindowVersion = value;
    }

    // java public void setDebugLogVersion(boolean value)
    pub fn setDebugLogVersion(&mut self, value: bool) {
      // java this.threadGet().mIsDebugLogVersion = value;
      self.mIsDebugLogVersion = value;
    }

    // java public void setOverseaVersion(boolean value)
    pub fn setOverseaVersion(&mut self, value: bool) {
      // java this.threadGet().mIsOverseaVersion = value;
      self.mIsOverseaVersion = value;
    }

    // java public void setAndroidN(boolean value)
    pub fn setAndroidN(&mut self, value: bool) {
      // java this.threadGet().mIsAndroidN = value;
      self.mIsAndroidN = value;
    }

    // java public void setPadScreen(boolean value)
    pub fn setPadScreen(&mut self, value: bool) {
      // java this.threadGet().mIsPadScreen = value;
      self.mIsPadScreen = value;
    }

    // java public void setDisplayWidthAndHeight(int[] widthAndHeight)
    pub fn setDisplayWidthAndHeight(&mut self, widthAndHeight: &Vec<i32>) {
      // java this.threadGet().mDisplayWidthAndHeight = widthAndHeight;
      self.mDisplayWidthAndHeight = widthAndHeight.clone();
    }

    // java public void setParamsOn(String key, boolean value)
    pub fn setParamsOn(&mut self, key: &str, value: bool) {
      // java this.threadGet().mIsParamsOn.put(key, value);
      self.mIsParamsOn.insert(key.to_string(), value);
    }

    // java public void setLeshiDevice(boolean value)
    pub fn setLeshiDevice(&mut self, value: bool) {
      // java this.threadGet().mIsLeshiDevice = value;
      self.mIsLeshiDevice = value;
    }

    // java public void setOnlyHuaweiDevice(boolean value)
    pub fn setOnlyHuaweiDevice(&mut self, value: bool) {
      // java this.threadGet().mIsOnlyHuaweiDevice = value;
      self.mIsOnlyHuaweiDevice = value;
    }

    // java public void setHonorDevice(boolean value)
    pub fn setHonorDevice(&mut self, value: bool) {
      // java this.threadGet().mIsHonorDevice = value;
      self.mIsHonorDevice = value;
    }

    // java public void setDeviceBrand(String value)
    pub fn setDeviceBrand(&mut self, value: &str) {
      // java this.threadGet().mDeviceBrand = value;
      self.mDeviceBrand = value.to_string();
    }

}
