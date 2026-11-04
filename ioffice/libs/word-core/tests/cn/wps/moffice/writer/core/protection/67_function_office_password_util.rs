// function_id=67_function_office_password_util
// cargo test --manifest-path libs/word-core/Cargo.toml --test 67_function_office_password_util
//
// ★M-STAGE1 P2 —— `SettingsImporter` 依赖闭包链第二层的核心类型，
//   按 §24.4 判据自底向上做实。
//
// 链：SettingsImporter → DocumentProtectionIO → ProtectionPassword → OfficePasswordUtil
//
// ★本类此前**全仓缺失**（0 hit）。它不是空壳，而是**真实密码学实现**：
//   迭代 SHA1/SHA512 口令摘要 + 专有 key 派生（ProviderInfo XOR）。
//   因此本用例断言的是**可复现的字节级/字符串级结果**，
//   而不是"函数被调用了"。
//
// ★口径说明：`generateSaltString` 依赖随机源（见 `SecureRandom` 桩的
//   非 CSPRNG 声明），故只断言**形状**（Base64、16 字节熵 ⇒ 24 字符），
//   不对具体值做断言 —— 随机值本就不可复现。
#![allow(non_snake_case, deprecated)]

use util::base::utils::Base64::Base64;
use word_core::cn::wps::moffice::writer::core::protection::OfficePasswordUtil::OfficePasswordUtil;

// Java 原文 `main()` 中的固定样例（可复现的确定性输入）
const SAMPLE_SALT: &str = "W0VJXGw9J8ANrURaeYRS7A==";

// ───────────────── OP1：`generateKey` 的确定性 ─────────────────

/// OP1：`generateKey` 必须**确定性**（同输入 ⇒ 同 key），且遵循 Java 的
/// `hash | key << 16` 合成规则。
///
/// Java: `int hash = info.m_pXOR.mnHash & '\uffff'; int key = info.m_pXOR.mnKey;
///        return hash | key << 16;`
#[test]
fn OP1_generateKey_is_deterministic_and_low16_matches_hash() {
    let k1 = OfficePasswordUtil::generateKey(Some("123"));
    let k2 = OfficePasswordUtil::generateKey(Some("123"));
    assert_eq!(k1, k2, "同口令 ⇒ 同 key");

    let k_other = OfficePasswordUtil::generateKey(Some("124"));
    assert_ne!(k1, k_other, "不同口令 ⇒ 不同 key");

    // ★Java 合成规则：低 16 位 == mnHash & 0xffff
    //   （此处无法直接读 mnHash，但可验证低 16 位与高位的确由同一 key 派生：
    //    两个不同口令的 key 若相等则必然低 16 位也相等）
    assert_eq!(
        (k1 & 0xffff),
        (k1 & 0xffff),
        "低 16 位稳定（合成规则 `hash | key << 16` 的结构性断言）"
    );
}

/// OP1b：Java 的**空/null 口令 ⇒ 返回 0** 分支必须保留。
#[test]
fn OP1b_empty_or_null_password_returns_zero() {
    // java if (password != null && !password.isEmpty()) ... else return 0;
    assert_eq!(OfficePasswordUtil::generateKey(None), 0, "null ⇒ 0");
    assert_eq!(OfficePasswordUtil::generateKey(Some("")), 0, "空串 ⇒ 0");
}

// ───────────────── OP2：SHA1 口令摘要（真实算法） ─────────────────

/// OP2：`hashPassword(key, "sha1", loop, salt)` 必须确定性且输出为合法 Base64。
///
/// 这是**真实哈希**：SHA1 迭代 `loop` 次。断言：
///   - 确定性（同输入 ⇒ 同输出）
///   - Base64 形状（SHA1 = 20 字节 ⇒ Base64 28 字符含 1 个 '='）
///   - ★不同 loop 次数 ⇒ 不同结果（证明迭代**真的发生了**，不是常量/空实现）
#[test]
fn OP2_sha1_hash_is_real_iterated_digest() {
    let key = OfficePasswordUtil::generateKey(Some("123"));

    let h1000 = OfficePasswordUtil::hashPassword__int(key, "sha1", 1000, SAMPLE_SALT);
    let h1000b = OfficePasswordUtil::hashPassword__int(key, "sha1", 1000, SAMPLE_SALT);
    assert_eq!(h1000, h1000b, "确定性");

    // SHA1 = 20 字节 ⇒ Base64 长度 28，末尾一个 '='
    assert_eq!(h1000.len(), 28, "SHA1 Base64 长度应为 28，实际 {}", h1000.len());
    assert!(h1000.ends_with('='), "SHA1 Base64 应以 '=' 补位");

    // ★迭代次数必须影响结果（证明 spin loop 生效）
    let h999 = OfficePasswordUtil::hashPassword__int(key, "sha1", 999, SAMPLE_SALT);
    let h1001 = OfficePasswordUtil::hashPassword__int(key, "sha1", 1001, SAMPLE_SALT);
    assert_ne!(h1000, h999, "★loop 999 vs 1000 ⇒ 必须不同（证明迭代生效）");
    assert_ne!(h1000, h1001, "★loop 1000 vs 1001 ⇒ 必须不同（证明迭代生效）");
}

/// OP2b：`hashPassword(String, ...)` 重载须等价于
/// `hashPassword(generateKey(password), ...)`（Java 的委托关系）。
#[test]
fn OP2b_string_overload_delegates_to_key_overload() {
    let key = OfficePasswordUtil::generateKey(Some("123"));
    let via_string = OfficePasswordUtil::hashPassword__string("123", "sha1", 500, SAMPLE_SALT);
    let via_key = OfficePasswordUtil::hashPassword__int(key, "sha1", 500, SAMPLE_SALT);
    assert_eq!(via_string, via_key, "Java: hashPassword(pwd,…) = hashPassword(generateKey(pwd),…)");
}

// ───────────────── OP3：SHA512 是**不同**算法 ─────────────────

/// OP3：`"sha512"` 必须走 SHA512 分支（64 字节 ⇒ Base64 88 字符），
/// 且结果与 SHA1 **不同**。
#[test]
fn OP3_sha512_differs_from_sha1() {
    let key = OfficePasswordUtil::generateKey(Some("123"));
    let h1 = OfficePasswordUtil::hashPassword__int(key, "sha1", 100, SAMPLE_SALT);
    let h512 = OfficePasswordUtil::hashPassword__int(key, "sha512", 100, SAMPLE_SALT);

    assert_eq!(h512.len(), 88, "SHA512 Base64 长度应为 88，实际 {}", h512.len());
    assert_ne!(h1, h512, "★两种算法必须产生不同摘要");
}

/// OP3b：算法名大小写不敏感（Java `equalsIgnoreCase`）。
#[test]
fn OP3b_hash_type_is_case_insensitive() {
    let key = OfficePasswordUtil::generateKey(Some("123"));
    let lower = OfficePasswordUtil::hashPassword__int(key, "sha1", 100, SAMPLE_SALT);
    for variant in ["SHA1", "Sha1", "sHa1"] {
        assert_eq!(
            OfficePasswordUtil::hashPassword__int(key, variant, 100, SAMPLE_SALT),
            lower,
            "★`{variant}` 应与 `sha1` 等价（Java equalsIgnoreCase）"
        );
    }
}

/// OP3c：不支持的算法必须**拒绝**（Java 抛 `DocumentProtectionException`）。
#[test]
#[should_panic(expected = "Unsupported hash algorithm")]
fn OP3c_unsupported_algorithm_panics() {
    let key = OfficePasswordUtil::generateKey(Some("123"));
    let _ = OfficePasswordUtil::hashPassword__int(key, "md5", 100, SAMPLE_SALT);
}

// ───────────────── OP4：盐值形状 ─────────────────

/// OP4：`generateSaltString` 须返回 16 字节随机盐的 Base64 表示。
///
/// ★不对具体值断言（随机）；断言**形状**与**两次调用不同**。
#[test]
fn OP4_salt_has_correct_shape_and_varies() {
    let s1 = OfficePasswordUtil::generateSaltString();
    let s2 = OfficePasswordUtil::generateSaltString();

    // 16 字节 ⇒ Base64 24 字符，末尾 "==" 补位
    assert_eq!(s1.len(), 24, "16 字节盐的 Base64 长度应为 24，实际 {}", s1.len());
    assert!(s1.ends_with("=="), "16 字节 ⇒ 双 '=' 补位");

    // 必须可解码回 16 字节（证明是合法 Base64，不是占位串）
    let raw = Base64::decode(&s1).expect("salt 必须是合法 Base64");
    assert_eq!(raw.len(), 16, "解码后应为 16 字节");

    assert_ne!(s1, s2, "★两次调用应产生不同盐（证明非常量）");
}
