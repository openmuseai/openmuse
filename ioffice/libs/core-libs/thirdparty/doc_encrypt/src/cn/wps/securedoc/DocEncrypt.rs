#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: public class DocEncrypt
pub struct DocEncrypt {
}

// Java: public class DocEncrypt
impl DocEncrypt {
    // Java: DocEncrypt()
    pub fn new() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static boolean encrypt(String sourcePath, String infoPath, String destPath, String key)
    pub fn encrypt(sourcePath: &str, infoPath: &str, destPath: &str, key: &str) -> bool {
      // Java: return native_encrypt(sourcePath, infoPath, destPath, key);
      Self::native_encrypt(sourcePath, infoPath, destPath, key)
    }

    // Java: static boolean decrypt(String sourcePath, String infoPath, String destPath, String key)
    pub fn decrypt(sourcePath: &str, infoPath: &str, destPath: &str, key: &str) -> bool {
      // Java: return native_decrypt(sourcePath, infoPath, destPath, key);
      Self::native_decrypt(sourcePath, infoPath, destPath, key)
    }

    // Java: static private static native boolean native_encrypt(String var0, String var1, String var2, String var3)
    fn native_encrypt(_var0: &str, _var1: &str, _var2: &str, _var3: &str) -> bool {
      todo!()
    }

    // Java: static private static native boolean native_decrypt(String var0, String var1, String var2, String var3)
    fn native_decrypt(_var0: &str, _var1: &str, _var2: &str, _var3: &str) -> bool {
      todo!()
    }

}
