// Java: cn.wps.moffice.writer.core.protection.ProtectionPassword
//
// ★M-STAGE1 P2 —— 从 auto-generated 空壳（11 处 `todo!()`、无字段）**做实**。
//   闭包链：SettingsImporter → DocumentProtectionIO → **ProtectionPassword**
//                                                   → OfficePasswordUtil（§26 已做实）
//
// Java 原文 91 行。依赖均已就位且为真实现：
//   `StringUtil.isEmpty`（util）、`OfficePasswordUtil`（本仓，§26 新建）、
//   `IncorrectPasswordException`（本 crate）。
//
// ★可判定性：本类行为全部可由 `OfficePasswordUtil` 复现 ⇒ 用例断言的是**真实值**，
//   而非"函数被调用了"（见 68_function_protection_password）。

// Java: import cn.wps.moffice.util.StringUtil
use util::moffice::util::StringUtil::StringUtil;
// Java: import cn.wps.moffice.writer.core.protection.OfficePasswordUtil
use crate::cn::wps::moffice::writer::core::protection::OfficePasswordUtil::OfficePasswordUtil;
// Java: import cn.wps.moffice.writer.core.protection.IncorrectPasswordException
use crate::cn::wps::moffice::writer::core::protection::IncorrectPasswordException::IncorrectPasswordException;

// java public class ProtectionPassword
// rust: `Clone` 供 `DocumentProtectionIO::getPassword` 按值返回（Java 为引用返回）。
#[derive(Default, Clone)]
pub struct ProtectionPassword {
    // java int key;
    pub(crate) key: i32,
    // java String hashValue;
    pub(crate) hashValue: Option<String>,
    // java String saltValue;
    pub(crate) saltValue: Option<String>,
    // java String hashType;
    pub(crate) hashType: Option<String>,
    // java int spinCount;
    pub(crate) spinCount: i32,
}

// java public class ProtectionPassword
impl ProtectionPassword {
    // java public static final String HASH_SHA1 = "sha1";
    pub const HASH_SHA1: &'static str = "sha1";
    // java public static final String HASH_SHA512 = "sha512";
    pub const HASH_SHA512: &'static str = "sha512";

    // java public ProtectionPassword()
    pub fn ProtectionPassword() -> Self {
        // java (无语句：字段取 Java 默认值 key=0, hashValue=null, saltValue=null,
        //        hashType=null, spinCount=0)
        ProtectionPassword {
            key: 0,
            hashValue: None,
            saltValue: None,
            hashType: None,
            spinCount: 0,
        }
    }

    // java public ProtectionPassword(int key, String hashValue, String saltValue,
    //                                String hashType, int spinCount)
    // rust: Java 参数可空 ⇒ `Option<String>`（原文直接赋值，无空断言）。
    pub fn ProtectionPassword__int_string_string_string_int(
        key: i32,
        hashValue: Option<String>,
        saltValue: Option<String>,
        hashType: Option<String>,
        spinCount: i32,
    ) -> Self {
        // java this.key = key;
        // java this.hashValue = hashValue;
        // java this.saltValue = saltValue;
        // java this.hashType = hashType;
        // java this.spinCount = spinCount;
        ProtectionPassword {
            key,
            hashValue,
            saltValue,
            hashType,
            spinCount,
        }
    }

    // java public int getKey()
    pub fn getKey(&self) -> i32 {
        // java return this.key;
        return self.key;
    }

    // java public String getHash()
    pub fn getHash(&self) -> Option<String> {
        // java return this.hashValue;
        return self.hashValue.clone();
    }

    // java public String getSalt()
    pub fn getSalt(&self) -> Option<String> {
        // java return this.saltValue;
        return self.saltValue.clone();
    }

    // java public String getHashType()
    pub fn getHashType(&self) -> Option<String> {
        // java return this.hashType;
        return self.hashType.clone();
    }

    // java public int getSpinCount()
    pub fn getSpinCount(&self) -> i32 {
        // java return this.spinCount;
        return self.spinCount;
    }

    // java public boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // java return this.key == 0 && StringUtil.isEmpty(this.hashValue);
        return self.key == 0 && StringUtil::isEmpty(self.hashValue.as_deref());
    }

    // java public void enforce(String password)
    pub fn enforce(&mut self, password: &str) {
        // java this.reset();
        self.reset();
        // java this.key = OfficePasswordUtil.generateKey(password);
        self.key = OfficePasswordUtil::generateKey(Some(password));
    }

    // java public void clear(String password) throws IncorrectPasswordException
    // rust: Java 受检异常 `IncorrectPasswordException` ⇒ Rust `Result`。
    pub fn clear(&mut self, password: &str) -> Result<(), IncorrectPasswordException> {
        // java int key = OfficePasswordUtil.generateKey(password);
        let key: i32 = OfficePasswordUtil::generateKey(Some(password));
        // java boolean ok;
        let ok: bool;
        // java if (!StringUtil.isEmpty(this.hashValue))
        if !StringUtil::isEmpty(self.hashValue.as_deref()) {
            // java ok = OfficePasswordUtil.hashPassword(key, this.hashType,
            //        this.spinCount, this.saltValue).equals(this.hashValue);
            // rust: Java 传 null 时其内部 equalsIgnoreCase 会 NPE；Rust 侧以空串表达
            //   该不可达分支（本类正常流程下 ensureHash 已填满这三项）。
            let computed = OfficePasswordUtil::hashPassword__int(
                key,
                self.hashType.as_deref().unwrap_or(""),
                self.spinCount,
                self.saltValue.as_deref().unwrap_or(""),
            );
            ok = computed == self.hashValue.clone().unwrap_or_default();
        } else {
            // java ok = key == this.key;
            ok = key == self.key;
        }

        // java if (!ok)
        if !ok {
            // java throw new IncorrectPasswordException();
            return Err(IncorrectPasswordException::IncorrectPasswordException());
        } else {
            // java this.reset();
            self.reset();
            return Ok(());
        }
    }

    // java public void ensureHash()
    pub fn ensureHash(&mut self) {
        // java if (StringUtil.isEmpty(this.hashValue))
        if StringUtil::isEmpty(self.hashValue.as_deref()) {
            // java if (this.key == 0)
            if self.key == 0 {
                // java throw new IllegalStateException("Key should not be empty");
                panic!("Key should not be empty");
            }

            // java this.hashType = "sha1";
            self.hashType = Some(Self::HASH_SHA1.to_string());
            // java this.spinCount = 10000;
            self.spinCount = 10000;
            // java this.saltValue = OfficePasswordUtil.generateSaltString();
            self.saltValue = Some(OfficePasswordUtil::generateSaltString());
            // java this.hashValue = OfficePasswordUtil.hashPassword(this.key, this.hashType,
            //        this.spinCount, this.saltValue);
            self.hashValue = Some(OfficePasswordUtil::hashPassword__int(
                self.key,
                self.hashType.as_deref().unwrap_or(""),
                self.spinCount,
                self.saltValue.as_deref().unwrap_or(""),
            ));
        }
    }

    // java private void reset()
    fn reset(&mut self) {
        // java this.key = 0;
        self.key = 0;
        // java this.hashValue = null;
        self.hashValue = None;
        // java this.saltValue = null;
        self.saltValue = None;
        // java this.hashType = null;
        self.hashType = None;
        // java this.spinCount = 0;
        self.spinCount = 0;
    }
}
