#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use std::sync::{Mutex, OnceLock};

// Java: import cn.wps.base.assertion.Assert

// Java field: String ST_SONAME
pub(crate) const ST_SONAME: &str = "wpscn_st_convert";
// Java field: int max_char
pub(crate) const MAX_CHAR: i32 = 40864;
// Java field: int min_char
pub(crate) const MIN_CHAR: i32 = 19968;
// Java field: int max_word_len
pub(crate) const MAX_WORD_LEN: i32 = 7;

// Java field: CnSTConvertCallback sCnSTConvertCallback
// Java field: boolean sInitJni
static S_INIT_JNI: OnceLock<Mutex<bool>> = OnceLock::new();

// Java: public interface CnSTConvertCallback
pub trait CnSTConvertCallback {
    // Java: void initPlugin()
    fn initPlugin(&self);
}

// Java: public class CnSTConvert
pub struct CnSTConvert {
    // Java field: int max_char
    // Java field: int min_char
    // Java field: int max_word_len
}

// Java: public class CnSTConvert
impl CnSTConvert {
    // Java: static void init(CnSTConvertCallback callback)
    pub fn init_callback(_callback: Box<dyn CnSTConvertCallback>) {
        // Java: sCnSTConvertCallback = callback;
    }

    // Java: static void loadSo()
    fn loadSo() {
        // Java: if (!sInitJni)
        {
            // Java: synchronized(CnSTConvert.class)
            {
                // Java: if (!sInitJni)
                {
                    // Java: String libName = "wpscn_st_convert";
                    // Java: Assert.assertNotNull("cannot be null, must invoke init()", sCnSTConvertCallback);
                    // Java: if (null != sCnSTConvertCallback)
                    {
                        // Java: sCnSTConvertCallback.initPlugin();
                    }
                    // Java: try
                    {
                        // Java: System.loadLibrary("wpscn_st_convert");
                        // Java: sInitJni = true;
                    }
                    // Java: catch (Throwable var4)
                    {
                        // Java: sInitJni = false;
                    }
                }
            }
        }
    }

    // Java: CnSTConvert()
    pub fn new() -> Self {
        // Java: super();
        // Java: loadSo();
        Self::loadSo();
        CnSTConvert {}
    }

    // Java: int getMaxWordLen()
    pub fn getMaxWordLen(&self) -> i32 {
        // Java: return 7;
        7
    }

    // Java: int getParticipleLeft(int participle)
    pub fn getParticipleLeft(&self, participle: i32) -> i32 {
        // Java: return (participle & -65536) >> 16;
        (participle & -65536i32) >> 16
    }

    // Java: int getParticipleRight(int participle)
    pub fn getParticipleRight(&self, participle: i32) -> i32 {
        // Java: return participle & '\uffff';
        participle & 0xFFFF
    }

    // Java: boolean isConvertable(char c)
    pub fn isConvertable(&self, c: char) -> bool {
        // Java: return c >= 19968 && c <= '龠';
        let cv = c as u32;
        cv >= 19968 && cv <= 40864
    }

    // Java: boolean quickSkip(_buff: &[char], int start, int len)
    pub fn quickSkip(&self, buff: &[char], start: i32, len: i32) -> bool {
        // Java: for(int end = start + len; start < end; ++start)
        let end = start + len;
        let mut idx = start;
        while idx < end {
            // Java: char c = buff[start];
            let c = buff[idx as usize];
            // Java: if (c >= 19968 && c <= '龠')
            let cv = c as u32;
            if cv >= 19968 && cv <= 40864 {
                // Java: return false;
                return false;
            }
            idx += 1;
        }
        // Java: return true;
        true
    }

    // Java: void s2t(_buff: &[char], int start, int len, _outbuff: &[char], _participle: &[i32])
    pub fn s2t_buf(&self, buff: &[char], start: i32, len: i32, outbuff: &mut [char], participle: &mut [i32]) {
        // Java: if (null != buff && start >= 0 && start + len <= buff.length && null != outbuff && null != participle && participle.length >= 2)
        if start >= 0 && (start + len) as usize <= buff.len() && participle.len() >= 2 {
            // Java: this.nS2T(buff, start, len, outbuff, participle);
            self.nS2T(buff, start, len, outbuff, participle);
        }
        // Java: else
        else {
            // Java: throw new IllegalArgumentException();
            panic!("IllegalArgumentException");
        }
    }

    // Java: void t2s(_buff: &[char], int start, int len, _outbuff: &[char], _participle: &[i32])
    pub fn t2s_buf(&self, buff: &[char], start: i32, len: i32, outbuff: &mut [char], participle: &mut [i32]) {
        // Java: if (null != buff && start >= 0 && start + len <= buff.length && null != outbuff && null != participle && participle.length >= 2)
        if start >= 0 && (start + len) as usize <= buff.len() && participle.len() >= 2 {
            // Java: this.nT2S(buff, start, len, outbuff, participle);
            self.nT2S(buff, start, len, outbuff, participle);
        }
        // Java: else
        else {
            // Java: throw new IllegalArgumentException();
            panic!("IllegalArgumentException");
        }
    }

    // Java: String s2t(String str)
    pub fn s2t_str(&self, str: &str) -> String {
        // Java: if (null == str)
        // Java: throw new IllegalArgumentException();
        // Java: else
        // Java: return this.nS2TString(str);
        self.nS2TString(str)
    }

    // Java: String t2s(String str)
    pub fn t2s_str(&self, str: &str) -> String {
        // Java: if (null == str)
        // Java: throw new IllegalArgumentException();
        // Java: else
        // Java: return this.nT2SString(str);
        self.nT2SString(str)
    }

    // Java: int s2tCount(_buff: &[char], int start, int len)
    pub fn s2tCount_buf(&self, buff: &[char], start: i32, len: i32) -> i32 {
        // Java: if (null != buff && start >= 0 && start + len <= buff.length)
        if start >= 0 && (start + len) as usize <= buff.len() {
            // Java: return this.nS2TCount(buff, start, len);
            self.nS2TCount(buff, start, len)
        }
        // Java: else
        else {
            // Java: throw new IllegalArgumentException();
            panic!("IllegalArgumentException");
        }
    }

    // Java: int t2sCount(_buff: &[char], int start, int len)
    pub fn t2sCount_buf(&self, buff: &[char], start: i32, len: i32) -> i32 {
        // Java: if (null != buff && start >= 0 && start + len <= buff.length)
        if start >= 0 && (start + len) as usize <= buff.len() {
            // Java: return this.nT2SCount(buff, start, len);
            self.nT2SCount(buff, start, len)
        }
        // Java: else
        else {
            // Java: throw new IllegalArgumentException();
            panic!("IllegalArgumentException");
        }
    }

    // Java: int s2tCount(String str)
    pub fn s2tCount_str(&self, str: &str) -> i32 {
        // Java: if (null == str)
        // Java: throw new IllegalArgumentException();
        // Java: else
        // Java: return this.nS2TCountString(str);
        self.nS2TCountString(str)
    }

    // Java: int t2sCount(String str)
    pub fn t2sCount_str(&self, str: &str) -> i32 {
        // Java: if (null == str)
        // Java: throw new IllegalArgumentException();
        // Java: else
        // Java: return this.nT2SCountString(str);
        self.nT2SCountString(str)
    }

    // Java: private native void nS2T(_var1: &[char], int var2, int var3, _var4: &[char], _var5: &[i32])
    fn nS2T(&self, _var1: &[char], _var2: i32, _var3: i32, _var4: &mut [char], _var5: &mut [i32]) {
    }

    // Java: private native void nT2S(_var1: &[char], int var2, int var3, _var4: &[char], _var5: &[i32])
    fn nT2S(&self, _var1: &[char], _var2: i32, _var3: i32, _var4: &mut [char], _var5: &mut [i32]) {
    }

    // Java: private native String nS2TString(String var1)
    fn nS2TString(&self, _var1: &str) -> String {
        String::new()
    }

    // Java: private native String nT2SString(String var1)
    fn nT2SString(&self, _var1: &str) -> String {
        String::new()
    }

    // Java: private native int nS2TCountString(String var1)
    fn nS2TCountString(&self, _var1: &str) -> i32 {
        0
    }

    // Java: private native int nT2SCountString(String var1)
    fn nT2SCountString(&self, _var1: &str) -> i32 {
        0
    }

    // Java: private native int nS2TCount(_var1: &[char], int var2, int var3)
    fn nS2TCount(&self, _var1: &[char], _var2: i32, _var3: i32) -> i32 {
        0
    }

    // Java: private native int nT2SCount(_var1: &[char], int var2, int var3)
    fn nT2SCount(&self, _var1: &[char], _var2: i32, _var3: i32) -> i32 {
        0
    }
}
