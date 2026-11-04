// Java: cn.wps.moffice.writer.core.protection.OfficePasswordUtil
//
// ★M-STAGE1 P2 —— 新建类型。全仓此前缺失（0 hit），是 `SettingsImporter`
//   依赖闭包链上的一环：
//     `SettingsImporter` → `DocumentProtectionIO` → `ProtectionPassword` → 本类
//   按 §24.4 判据：闭包内类型须**自底向上**逐个做实。
//
// Java 原文 172 行。其依赖**均已存在且为真实现**：
//   `Base64`（util）、`ProviderInfo` + `MSCodec_Xor95`（poi-compact）、
//   `DigestSHA1` / `DigestSHA512`（poi-compact，含真实 `updateHash`）。
//
// ★本轮为 `MSCodec_Xor95` 补了 `getMnKey()` / `getMnHash()` 两个公开只读访问器 ——
//   Java 中 `info.m_pXOR.mnKey/mnHash` 为包内可见，Rust 的 `pub(crate)` 无法跨 crate，
//   故在**属主 crate**（poi-compact）内补访问器，而非在本 crate 复制字段。

// Java: import cn.wps.crypt.provider.ProviderInfo
use io_poi_compact::cn::wps::crypt::provider::ProviderInfo::*;
// Java: import cn.wps.encrypt.Base64
use util::base::utils::Base64::*;
// Java: import java.security.MessageDigest  (以 DigestSHA1/DigestSHA512 落地)
use io_poi_compact::cn::wps::crypt::sal::digest::DigestSHA1::DigestSHA1;
use io_poi_compact::cn::wps::crypt::sal::digest::DigestSHA512::DigestSHA512;
// Java: import java.security.SecureRandom
use stubs::java::security::SecureRandom::SecureRandom;

// Java: import cn.wps.moffice.writer.core.protection.DocumentProtectionException
use crate::cn::wps::moffice::writer::core::protection::DocumentProtectionException::DocumentProtectionException;

// Java: public class OfficePasswordUtil
pub struct OfficePasswordUtil {
    // java private static final SecureRandom mRandom = new SecureRandom();
    mRandom: SecureRandom,
}

impl OfficePasswordUtil {
    // Java: private static final byte[] alpha;  static { alpha = "0123456789ABCDEF".getBytes(UTF_8); }
    // rust: 静态常量表，直接用 `const` 表达（Java 的 static 块只是初始化）。
    const alpha: [u8; 16] = *b"0123456789ABCDEF";

    // java public static byte[] generateSalt()
    pub fn generateSalt() -> Vec<u8> {
        // java byte[] buf = new byte[16];
        let mut buf = vec![0u8; 16];
        // java mRandom.nextBytes(buf);
        SecureRandom::nextBytes(&mut buf);
        // java return buf;
        return buf;
    }

    // java public static String generateSaltString()
    pub fn generateSaltString() -> String {
        // java return Base64.byteArrayToBase64(generateSalt());
        return Base64::encode(&Self::generateSalt()).unwrap_or_default();
    }

    // java static void print(byte[] buf)
    pub fn print(buf: &[u8]) {
        // java for(byte b : buf)
        for b in buf.iter() {
            // java System.out.println(b & 255);
            println!("{}", (*b as i32) & 255);
        }

        // java System.out.println("=====");
        println!("=====");
    }

    // java private static byte[] SHA1HashPassword(byte[] password, byte[] salt, int spin)
    fn SHA1HashPassword(password: &[u8], salt: &[u8], spin: i32) -> Vec<u8> {
        // java return calculatePasswordDigest(password, salt, spin, "SHA1");
        return Self::calculatePasswordDigest(password, salt, spin, true);
    }

    // java private static byte[] SHA512HashPassword(byte[] password, byte[] salt, int spin)
    fn SHA512HashPassword(password: &[u8], salt: &[u8], spin: i32) -> Vec<u8> {
        // java return calculatePasswordDigest(password, salt, spin, "SHA512");
        return Self::calculatePasswordDigest(password, salt, spin, false);
    }

    // java private static byte[] calculatePasswordDigest(byte[] password, byte[] salt,
    //                                                    int spin, String digestAlgorithm)
    // rust: Java 用 `MessageDigest.getInstance(算法名)`；Rust 侧以**布尔选路**替代字符串查表，
    //   两条分支各自调用真实的 `DigestSHA1` / `DigestSHA512`，行为与 Java 等价。
    fn calculatePasswordDigest(password: &[u8], salt: &[u8], spin: i32, is_sha1: bool) -> Vec<u8> {
        // java try
        // java byte[] iter = new byte[4];
        let mut iter = [0u8; 4];
        // java MessageDigest digest = MessageDigest.getInstance(digestAlgorithm);
        // java digest.update(salt);
        // java digest.update(password);
        // java byte[] hash = digest.digest();
        let mut hash: Vec<u8>;
        if is_sha1 {
            let mut digest = DigestSHA1::new();
            digest.update_data(salt, 0, salt.len() as i32);
            digest.update_data(password, 0, password.len() as i32);
            let mut out = vec![0u8; 20];
            digest.get_digest(&mut out);
            hash = out;
        } else {
            let mut digest = DigestSHA512::new();
            digest.update_data(salt, 0, salt.len() as i32);
            digest.update_data(password, 0, password.len() as i32);
            let mut out = vec![0u8; 64];
            digest.get_digest(&mut out);
            hash = out;
        }

        // java for(int i = 0; i < spin; ++i) {
        let mut i: i32 = 0;
        while i < spin {
            // java digest.reset();
            // java iter[0] = (byte)(i & 255);
            iter[0] = (i & 255) as u8;
            // java iter[1] = (byte)(i >>> 8 & 255);
            iter[1] = ((i >> 8) & 255) as u8;
            // java iter[2] = (byte)(i >>> 16 & 255);
            iter[2] = ((i >> 16) & 255) as u8;
            // java iter[3] = (byte)(i >>> 24 & 255);
            iter[3] = ((i >> 24) & 255) as u8;
            // java digest.update(hash);
            // java digest.update(iter);
            // java hash = digest.digest();
            if is_sha1 {
                let mut digest = DigestSHA1::new();
                digest.update_data(&hash, 0, hash.len() as i32);
                digest.update_data(&iter, 0, iter.len() as i32);
                let mut out = vec![0u8; 20];
                digest.get_digest(&mut out);
                hash = out;
            } else {
                let mut digest = DigestSHA512::new();
                digest.update_data(&hash, 0, hash.len() as i32);
                digest.update_data(&iter, 0, iter.len() as i32);
                let mut out = vec![0u8; 64];
                digest.get_digest(&mut out);
                hash = out;
            }
            i += 1;
        }

        // java catch (GeneralSecurityException e) {
        // java throw new RuntimeException("No digest algorithm found!", e);
        // java }
        // rust: Rust 无受检异常；上列为 Java 异常路径，本实现不产生该异常
        //   （算法由**静态选路**确定，不经字符串查表）⇒ 该路径不可达。
        // java return hash;
        return hash;
    }

    // java /** @deprecated */
    // java @Deprecated
    // java public static String generateHash(String password, byte[] salt, int spin, String algorithm)
    // ★Java 标注 `@Deprecated`；仍按原文实现（不得因弃用而省略逻辑）。
    #[deprecated]
    pub fn generateHash(password: &str, salt: &[u8], spin: i32, algorithm: &str) -> String {
        // java byte[] bytes = new byte[password.length() * 2];
        let chars: Vec<char> = password.chars().collect();
        let mut bytes = vec![0u8; chars.len() * 2];

        // java for(int i = 0; i < password.length(); ++i) {
        let mut i: usize = 0;
        while i < chars.len() {
            // java bytes[i * 2] = (byte)password.charAt(i);
            bytes[i * 2] = chars[i] as u8;
            i += 1;
        }

        // java byte[] encryptedBytes;
        let encryptedBytes: Vec<u8>;
        // java if ("sha1".equalsIgnoreCase(algorithm))
        if algorithm.eq_ignore_ascii_case("sha1") {
            // java encryptedBytes = SHA1HashPassword(bytes, salt, spin);
            encryptedBytes = Self::SHA1HashPassword(&bytes, salt, spin);
        } else {
            // java if (!"sha512".equalsIgnoreCase(algorithm)) throw new DocumentProtectionException(...)
            if !algorithm.eq_ignore_ascii_case("sha512") {
                // java throw new DocumentProtectionException("Unsupported hash algorithm: " + algorithm);
                panic!("Unsupported hash algorithm: {}", algorithm);
            }

            // java encryptedBytes = SHA512HashPassword(bytes, salt, spin);
            encryptedBytes = Self::SHA512HashPassword(&bytes, salt, spin);
        }

        // java return Base64.byteArrayToBase64(encryptedBytes);
        return Base64::encode(&encryptedBytes).unwrap_or_default();
    }

    // java private static String hashPassword_SHA1(String password, String salt, int loop)
    fn hashPassword_SHA1__string(password: &str, salt: &str, r#loop: i32) -> String {
        // java int x = generateKey(password);
        let x: i32 = Self::generateKey(Some(password));
        // java byte[] password_bytes = _getPasswordBytes(x);
        let password_bytes = Self::_getPasswordBytes(x);
        // java byte[] salt_bytes = Base64.base64ToByteArray(salt);
        let salt_bytes = Base64::decode(salt).unwrap_or_default();
        // java byte[] h = SHA1HashPassword(password_bytes, salt_bytes, loop);
        let h = Self::SHA1HashPassword(&password_bytes, &salt_bytes, r#loop);
        // java return Base64.byteArrayToBase64(h);
        return Base64::encode(&h).unwrap_or_default();
    }

    // java private static String hashPassword_SHA1(int password, String salt, int loop)
    fn hashPassword_SHA1__int(password: i32, salt: &str, r#loop: i32) -> String {
        // java byte[] password_bytes = _getPasswordBytes(password);
        let password_bytes = Self::_getPasswordBytes(password);
        // java byte[] salt_bytes = Base64.base64ToByteArray(salt);
        let salt_bytes = Base64::decode(salt).unwrap_or_default();
        // java byte[] h = SHA1HashPassword(password_bytes, salt_bytes, loop);
        let h = Self::SHA1HashPassword(&password_bytes, &salt_bytes, r#loop);
        // java return Base64.byteArrayToBase64(h);
        return Base64::encode(&h).unwrap_or_default();
    }

    // java public static String hashPassword(int key, String hashType, int loop, String salt)
    pub fn hashPassword__int(key: i32, hashType: &str, r#loop: i32, salt: &str) -> String {
        // java byte[] password_bytes = _getPasswordBytes(key);
        let password_bytes = Self::_getPasswordBytes(key);
        // java byte[] salt_bytes = Base64.base64ToByteArray(salt);
        let salt_bytes = Base64::decode(salt).unwrap_or_default();
        // java byte[] h;
        let h: Vec<u8>;
        // java if ("sha1".equalsIgnoreCase(hashType))
        if hashType.eq_ignore_ascii_case("sha1") {
            // java h = SHA1HashPassword(password_bytes, salt_bytes, loop);
            h = Self::SHA1HashPassword(&password_bytes, &salt_bytes, r#loop);
        } else {
            // java if (!"sha512".equalsIgnoreCase(hashType)) throw new DocumentProtectionException(...)
            if !hashType.eq_ignore_ascii_case("sha512") {
                // java throw new DocumentProtectionException("Unsupported hash algorithm: " + hashType);
                panic!("Unsupported hash algorithm: {}", hashType);
            }

            // java h = SHA512HashPassword(password_bytes, salt_bytes, loop);
            h = Self::SHA512HashPassword(&password_bytes, &salt_bytes, r#loop);
        }

        // java return Base64.byteArrayToBase64(h);
        return Base64::encode(&h).unwrap_or_default();
    }

    // java public static void main1(String[] args)
    // rust: 命令行演示入口（非产品管线），按 DEFERRED_NON_FUNCTION 处理；
    //   全部 Java 语句以 `// java` 逐条保留（§3.8 要求语句级完整）。
    pub fn main1(_args: Vec<String>) {
        // java byte[] salt = Base64.base64ToByteArray("W0VJXGw9J8ANrURaeYRS7A==");
        let salt: Vec<u8> = Base64::decode("W0VJXGw9J8ANrURaeYRS7A==").unwrap_or_default();
        // java for(byte b : salt)
        for b in salt.iter() {
            // java System.out.println(String.format("0x%02x", b & 255));
            println!("0x{:02x}", (*b as i32) & 255);
        }
        // java String password = "123";
        let password = "123";
        // java int x = generateKey(password);
        let x: i32 = Self::generateKey(Some(password));
        // java byte[] password_bytes = _getPasswordBytes(x);
        let password_bytes: Vec<u8> = Self::_getPasswordBytes(x);
        // java byte[] h = SHA1HashPassword(password_bytes, salt, 100000);
        let h: Vec<u8> = Self::SHA1HashPassword(&password_bytes, &salt, 100000);
        // java System.out.println(Base64.byteArrayToBase64(h));
        println!("{}", Base64::encode(&h).unwrap_or_default());

        // rust: 上列语句已按 Java 逐条落地；`main1` 为命令行演示入口，
        //   不参与产品管线（Java 侧同样不被任何生产代码调用）。
        //   按 DEFERRED_NON_FUNCTION 约定保留末尾 todo 以标记"非管线方法"。
        todo!("DEFERRED_NON_FUNCTION")
    }

    // java public static void main(String[] args)
    // rust: 命令行演示入口（非产品管线），按 DEFERRED_NON_FUNCTION 处理。
    pub fn main(_args: Vec<String>) {
        // java String h = hashPassword("123", "sha1", 100000, "W0VJXGw9J8ANrURaeYRS7A==");
        let h: String = Self::hashPassword__string("123", "sha1", 100000, "W0VJXGw9J8ANrURaeYRS7A==");
        // java System.out.println(h);
        println!("{}", h);
        todo!("DEFERRED_NON_FUNCTION")
    }

    // java public static String hashPassword(String password, String hashType, int loop, String salt)
    pub fn hashPassword__string(password: &str, hashType: &str, r#loop: i32, salt: &str) -> String {
        // java return hashPassword(generateKey(password), hashType, loop, salt);
        return Self::hashPassword__int(Self::generateKey(Some(password)), hashType, r#loop, salt);
    }

    // java private static byte[] _getPasswordBytes(int key)
    fn _getPasswordBytes(key: i32) -> Vec<u8> {
        // java byte[] buf = new byte[16];
        let mut buf = vec![0u8; 16];

        // java for(int i = 0; i < 4; ++i) {
        let mut i: i32 = 0;
        while i < 4 {
            // java byte x = (byte)(key >>> i * 8 & 255);
            let x: u8 = ((key as u32 >> (i * 8)) & 255) as u8;
            // java buf[i * 4] = alpha[x >>> 4 & 15];
            buf[(i * 4) as usize] = Self::alpha[((x >> 4) & 15) as usize];
            // java buf[i * 4 + 1] = 0;
            buf[(i * 4 + 1) as usize] = 0;
            // java buf[i * 4 + 2] = alpha[x & 15];
            buf[(i * 4 + 2) as usize] = Self::alpha[(x & 15) as usize];
            // java buf[i * 4 + 3] = 0;
            buf[(i * 4 + 3) as usize] = 0;
            i += 1;
        }

        // java return buf;
        return buf;
    }

    // java public static int generateKey(String password)
    // rust: Java 参数可空 ⇒ `Option<&str>`（Java 的 `null == password` 分支保留）。
    pub fn generateKey(password: Option<&str>) -> i32 {
        // java if (password != null && !password.isEmpty())
        if let Some(pwd) = password {
            if !pwd.is_empty() {
                // java ProviderInfo info = new ProviderInfo();
                let mut info = ProviderInfo::new();
                // java info.init_XOR(ProviderInfo.CRYPTDOCUMENT_WPS, password);
                info.init_XOR(CRYPTDOCUMENT_WPS, pwd);
                // rust: `pub(crate)` 字段跨 crate 不可见 ⇒ 走本轮在属主 crate 补的只读访问器。
                let xor = info.m_pXOR.as_ref().expect("m_pXOR after init_XOR");
                // java int hash = info.m_pXOR.mnHash & '\uffff';
                let hash: i32 = (xor.getMnHash() as i32) & 0xffff;
                // java int key = info.m_pXOR.mnKey;
                let key: i32 = xor.getMnKey() as i32;
                // java return hash | key << 16;
                return hash | (key << 16);
            } else {
                // java return 0;
                return 0;
            }
        } else {
            // java return 0;
            return 0;
        }
    }
}
