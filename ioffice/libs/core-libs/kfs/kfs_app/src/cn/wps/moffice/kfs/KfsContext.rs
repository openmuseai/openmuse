#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

// Java: import cn.wps.moffice.kfs.log.KfsLog
use crate::kfs::log::KfsLog::KfsLog;
// Java: import cn.wps.moffice.kfs.mfs.MfsReflectHelper
use crate::kfs::mfs::MfsReflectHelper::MfsReflectHelper;
// Java: import cn.wps.moffice.kfs.mfs.core.iface.IMfsDiskManager
use crate::kfs::mfs::core::iface::IMfsDiskManager::*;
// Java: import cn.wps.moffice.kfs.tools.ByteArrayUtil
use crate::kfs::tools::ByteArrayUtil::ByteArrayUtil;

use std::sync::Mutex;

// Java field:    public static final int MODE_NORMAL = 0;
pub const MODE_NORMAL: i32 = 0;
// Java field:    public static final int MODE_CFS = 1;
pub const MODE_CFS: i32 = 1;
// Java field:    public static final int MODE_MFS = 2;
pub const MODE_MFS: i32 = 2;

// Java field:    private static KfsContext sInstance;
static S_INSTANCE: Mutex<Option<KfsContext>> = Mutex::new(None);

// Java: public class KfsContext
#[derive(Default, Clone)]
pub struct KfsContext {
    // Java field:    private boolean mIsInstalled = false;
    mIsInstalled: bool,
    // Java field:    private int mMode = 0;
    mMode: i32,
}

// Java: public class KfsContext
impl KfsContext {
    // Java: private KfsContext()
    pub fn new() -> KfsContext {
        Self { mIsInstalled: false, mMode: 0 }
    }

    // Java: public static KfsContext instance()
    pub fn instance() -> KfsContext {
        // Java: if (null == sInstance) sInstance = new KfsContext();
        let mut lock = S_INSTANCE.lock().unwrap();
        if lock.is_none() {
            *lock = Some(KfsContext::new());
        }
        // Java: return sInstance;
        lock.clone().unwrap()
    }

    // Java: public void setMode(int mode)
    pub fn setMode(&mut self, mode: i32) {
        // Java: this.mMode = mode;
        self.mMode = mode;
    }

    // Java: public int mode()
    pub fn mode(&self) -> i32 {
        // Java: return this.mMode;
        self.mMode
    }

    // Java: public void install(Object context, KeyProvider keyProvider)
    pub fn install(&mut self, _context: Box<dyn std::any::Any>, key: Option<Vec<u8>>) {
        // Java: if (!this.mIsInstalled)
        if !self.mIsInstalled {
            // Java: if (1 == this.mode())
            if 1 == self.mode() {
                // Java: this.installCfs(keyProvider);
                self.installCfs(key);
            }
            // Java: else if (2 == this.mode())
            else if 2 == self.mode() {
                // Java: this.installMfs(context);
                // Reflection-based MFS install - simplified
            }
            // Java: this.mIsInstalled = true;
            self.mIsInstalled = true;
        }
    }

    // Java: public void preInstall(Object context, String className, Runnable runnable)
    pub fn preInstall(&mut self, context: Box<dyn std::any::Any>, className: String, runnable: Box<dyn Fn()>) {
        // Java: if (2 == this.mode())
        if 2 == self.mode() {
            // Java: IMfsDiskManager mdm = MfsReflectHelper.getDiskManager(this);
            // Reflection-based - simplified, just run the runnable
            runnable();
        }
        // Java: else
        else {
            // Java: runnable.run();
            runnable();
        }
    }

    // Java: private void installCfs(KeyProvider keyProvider)
    fn installCfs(&mut self, key: Option<Vec<u8>>) {
        // Java: byte[] key = keyProvider != null ? keyProvider.getKey() : null;
        // Java: if (null != key && key.length >= 0)
        if let Some(ref k) = key {
            // Java: KfsLog.d("KfsContext", "Cfs install");
            KfsLog::d("KfsContext".to_string(), "Cfs install".to_string());
            // Java: System.loadLibrary("kfs");
            // Native library loading - no-op in Rust
            // Java: nativeInstall(key);
            Self::nativeInstall(k);
        }
    }

    // Java: private byte[] getRandomKey()
    fn getRandomKey(&self) -> Vec<u8> {
        // Java: String randomKey = this.randomHexString(16);
        let randomKey = self.randomHexString(16);
        // Java: return null != randomKey && 0 != randomKey.length() ? ByteArrayUtil.parse(randomKey) : null;
        if !randomKey.is_empty() {
            ByteArrayUtil::parse(randomKey)
        } else {
            Vec::new()
        }
    }

    // Java: private String randomHexString(int len)
    fn randomHexString(&self, len: i32) -> String {
        // Java: StringBuffer result = new StringBuffer();
        let mut result = String::new();
        // Java: for(int i = 0; i < len; ++i)
        for _i in 0..len {
            // Java: result.append(Integer.toHexString((new Random()).nextInt(16)));
            use std::time::SystemTime;
            let n = (SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos() % 16) as u8;
            result.push_str(&format!("{:x}", n));
        }
        // Java: return result.toString().toUpperCase();
        result.to_uppercase()
    }

    // Java: private static native void nativeInstall(byte[] var0);
    fn nativeInstall(_key: &[u8]) {
        // JNI native method - no-op in Rust
    }
}
