#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.kfs.tools.JNIUtils
use crate::kfs::tools::JNIUtils::*;
// Java: import cn.wps.moffice.kfs.tools.KfsLong
use crate::kfs::tools::KfsLong::*;

// Java field:    public static int O_RDONLY = 0;
pub const O_RDONLY: i32 = 0;
// Java field:    public static int O_WRONLY = 1;
pub const O_WRONLY: i32 = 1;
// Java field:    public static int O_RDWR = 2;
pub const O_RDWR: i32 = 2;
// Java field:    public static int O_CREAT = 64;
pub const O_CREAT: i32 = 64;
// Java field:    public static int O_TRUNC = 512;
pub const O_TRUNC: i32 = 512;
// Java field:    public static int O_APPEND = 1024;
pub const O_APPEND: i32 = 1024;
// Java field:    public static int O_EXCL = 128;
pub const O_EXCL: i32 = 128;
// Java field:    public static int O_SYNC = 1052672;
pub const O_SYNC: i32 = 1052672;
// Java field:    public static int SEEK_SET = 0;
pub const SEEK_SET: i32 = 0;
// Java field:    public static int SEEK_CUR = 1;
pub const SEEK_CUR: i32 = 1;
// Java field:    public static int SEEK_END = 2;
pub const SEEK_END: i32 = 2;
// Java field:    public static int F_RDLCK = 1;
pub const F_RDLCK: i32 = 1;
// Java field:    public static int F_UNLCK = 2;
pub const F_UNLCK: i32 = 2;
// Java field:    public static int F_WRLCK = 3;
pub const F_WRLCK: i32 = 3;

// Java: public class CfsCore
#[derive(Default)]
pub struct CfsCore {}

// Java: public class CfsCore
impl CfsCore {
    pub fn new() -> CfsCore { Self {} }

    // Java: public static void checkOffsetAndCount(int length, int offset, int byteCount)
    pub fn checkOffsetAndCount(length: i32, offset: i32, byteCount: i32) {
        if offset < 0 || byteCount < 0 || length - offset < byteCount {
            panic!("IndexOutOfBoundsException");
        }
    }

    // Java: public static int read(int fd, byte[] buffer, int offset, int count)
    pub fn read(fd: i32, buffer: &mut [u8], offset: i32, count: i32) -> i32 {
        // Java: KfsLong handle = JNIUtils.initialNativeHandle();
        let handle = JNIUtils::initialNativeHandle();
        // Java: Object object = nativeCfsRead(fd, count, handle);
        let data = Self::nativeCfsRead(fd, count, &handle);
        // Java: if (object == null) return -1;
        if data.is_empty() { return -1; }
        let read_cnt = data.len().min(count as usize);
        let off = offset as usize;
        buffer[off..off + read_cnt].copy_from_slice(&data[..read_cnt]);
        // Java: nativeFreeReadBuffer(handle.getValue());
        Self::nativeFreeReadBuffer(handle.getValue());
        read_cnt as i32
    }

    // Java: public static void write(int fd, byte[] buffer, int offset, int count)
    pub fn write(fd: i32, buffer: &[u8], offset: i32, count: i32) {
        let off = offset as usize;
        let cnt = count as usize;
        // Java: nativeCfsWrite(fd, buff);
        Self::nativeCfsWrite(fd, &buffer[off..off + cnt]);
    }

    // Java: public static native int nativeCfsOpen(byte[] var0, int var1);
    pub fn nativeCfsOpen(_path: &[u8], _flags: i32) -> i32 { 0 }
    // Java: public static native int nativeCfsSync(int var0);
    pub fn nativeCfsSync(_fd: i32) -> i32 { 0 }
    // Java: public static native Object nativeCfsRead(int var0, int var1, KfsLong var2);
    pub fn nativeCfsRead(_fd: i32, _count: i32, _handle: &KfsLong) -> Vec<u8> { Vec::new() }
    // Java: public static native void nativeCfsWrite(int var0, Object var1);
    pub fn nativeCfsWrite(_fd: i32, _data: &[u8]) {}
    // Java: public static native int nativeCfsSeek(int var0, long var1, int var3);
    pub fn nativeCfsSeek(_fd: i32, _offset: i64, _whence: i32) -> i32 { 0 }
    // Java: public static native long nativeCfsLength(int var0);
    pub fn nativeCfsLength(_fd: i32) -> i64 { 0 }
    // Java: public static native int nativeCfsClose(int var0);
    pub fn nativeCfsClose(_fd: i32) -> i32 { 0 }
    // Java: public static native int nativeCfsRename(byte[] var0, byte[] var1);
    pub fn nativeCfsRename(_old: &[u8], _new: &[u8]) -> i32 { 0 }
    // Java: public static native int nativeCfsTruncate(int var0, long var1);
    pub fn nativeCfsTruncate(_fd: i32, _length: i64) -> i32 { 0 }
    // Java: public static native void nativeFreeReadBuffer(long var0);
    pub fn nativeFreeReadBuffer(_handle: i64) {}
    // Java: public static native int nativeCfsAvaliable(int var0);
    pub fn nativeCfsAvaliable(_fd: i32) -> i32 { 0 }
    // Java: public static native boolean nativeCfsFdValid(int var0);
    pub fn nativeCfsFdValid(_fd: i32) -> bool { false }
}
