// function_id=68_function_protection_password
// cargo test --manifest-path libs/word-core/Cargo.toml --test 68_function_protection_password
//
// ★M-STAGE1 P2 —— 闭包链第三层做实后的行为锁定。
//   链：SettingsImporter → DocumentProtectionIO → **ProtectionPassword**
//                                                → OfficePasswordUtil（§26）
//
// 该类型此前是 auto-generated 空壳（11 处 `todo!()`、无字段）。
//
// ★本用例断言的是**可由 `OfficePasswordUtil` 复现的真实值**：
//   `ensureHash` 生成的 hash 必须能用同一算法重新算出来并比对成功。
//   这是"接线 ≠ 生效"的可判定落点 —— 空壳/桩时代下列每一断言都会红。
#![allow(non_snake_case)]

use word_core::cn::wps::moffice::writer::core::protection::OfficePasswordUtil::OfficePasswordUtil;
use word_core::cn::wps::moffice::writer::core::protection::ProtectionPassword::ProtectionPassword;

// ───────────────── PP1：默认构造的 Java 默认值 ─────────────────

/// PP1：`new ProtectionPassword()` 后各字段须为 Java 默认值
/// （`key=0, hashValue=null, saltValue=null, hashType=null, spinCount=0`）。
#[test]
fn PP1_default_ctor_matches_java_defaults() {
    let p = ProtectionPassword::ProtectionPassword();
    assert_eq!(p.getKey(), 0, "key");
    assert_eq!(p.getHash(), None, "hashValue = null");
    assert_eq!(p.getSalt(), None, "saltValue = null");
    assert_eq!(p.getHashType(), None, "hashType = null");
    assert_eq!(p.getSpinCount(), 0, "spinCount");
}

// ───────────────── PP2：`isEmpty` 的 Java 双条件 ─────────────────

/// PP2：Java `isEmpty()` = `key == 0 && StringUtil.isEmpty(hashValue)`。
/// 两个条件**都**要满足；任一不满足即 `false`。
#[test]
fn PP2_isEmpty_requires_both_conditions() {
    // key=0 且 hash=null ⇒ empty
    let p = ProtectionPassword::ProtectionPassword();
    assert!(p.isEmpty(), "key=0 && hash=null ⇒ empty");

    // key != 0 ⇒ 非 empty（即便 hash 为 null）
    let p2 = ProtectionPassword::ProtectionPassword__int_string_string_string_int(
        12345, None, None, None, 0,
    );
    assert!(!p2.isEmpty(), "★key != 0 ⇒ 非 empty");

    // key=0 但 hash 非空 ⇒ 非 empty
    let p3 = ProtectionPassword::ProtectionPassword__int_string_string_string_int(
        0,
        Some("abc".to_string()),
        None,
        None,
        0,
    );
    assert!(!p3.isEmpty(), "★hash 非空 ⇒ 非 empty");

    // ★空串 hash 视同 null（StringUtil.isEmpty 语义）
    let p4 = ProtectionPassword::ProtectionPassword__int_string_string_string_int(
        0,
        Some(String::new()),
        None,
        None,
        0,
    );
    assert!(p4.isEmpty(), "空串 hash 等价于 null（StringUtil.isEmpty）");
}

// ───────────────── PP3：`enforce` = reset + generateKey ─────────────────

/// PP3：`enforce(password)` 先 `reset()` 再设 `key = generateKey(password)`。
/// 因此调用后 hash/salt/hashType/spinCount 必须**全部清空**。
#[test]
fn PP3_enforce_resets_then_sets_key() {
    let mut p = ProtectionPassword::ProtectionPassword__int_string_string_string_int(
        999,
        Some("stale".to_string()),
        Some("stale".to_string()),
        Some("sha512".to_string()),
        777,
    );

    p.enforce("secret");

    let expected = OfficePasswordUtil::generateKey(Some("secret"));
    assert_eq!(p.getKey(), expected, "★key 应等于 generateKey(password)");
    assert_eq!(p.getHash(), None, "★reset 后 hash 必须清空");
    assert_eq!(p.getSalt(), None, "★reset 后 salt 必须清空");
    assert_eq!(p.getHashType(), None, "★reset 后 hashType 必须清空");
    assert_eq!(p.getSpinCount(), 0, "★reset 后 spinCount 必须清空");
}

// ───────────────── PP4：`ensureHash` 的 Java 常量 ─────────────────

/// PP4：`ensureHash()` 须写入 Java 指定的常量：
/// `hashType="sha1"`、`spinCount=10000`、非空 salt、非空 hash。
#[test]
fn PP4_ensureHash_uses_java_constants() {
    let mut p = ProtectionPassword::ProtectionPassword();
    p.enforce("secret");
    p.ensureHash();

    assert_eq!(
        p.getHashType(),
        Some(ProtectionPassword::HASH_SHA1.to_string()),
        "★hashType 必须是 sha1"
    );
    assert_eq!(p.getSpinCount(), 10000, "★spinCount 必须是 10000");

    let salt = p.getSalt().expect("salt 必须非空");
    assert_eq!(salt.len(), 24, "盐值应为 16 字节的 Base64（24 字符）");

    let hash = p.getHash().expect("hash 必须非空");
    assert_eq!(hash.len(), 28, "SHA1 Base64 应为 28 字符");
    assert!(!p.isEmpty(), "ensureHash 后不应为空");
}

// ───────────────── PP5：★hash 可被独立重算 ⇒ 真实算法 ─────────────────

/// PP5：**核心可用性断言**。`ensureHash` 产生的 hash 必须能用
/// `OfficePasswordUtil.hashPassword(key, hashType, spinCount, salt)`
/// **独立重算出来并逐字符相等**。
///
/// 这证明 hash 不是占位串，而且三要素（key/hashType/spinCount/salt）
/// 确实构成了自洽的哈希输入。
#[test]
fn PP5_hash_is_reproducible_from_its_own_parameters() {
    let mut p = ProtectionPassword::ProtectionPassword();
    p.enforce("my-password");
    p.ensureHash();

    let recomputed = OfficePasswordUtil::hashPassword__int(
        p.getKey(),
        &p.getHashType().unwrap(),
        p.getSpinCount(),
        &p.getSalt().unwrap(),
    );

    assert_eq!(
        Some(recomputed),
        p.getHash(),
        "★hash 必须可由 (key, hashType, spinCount, salt) 独立重算 —— 空壳必红"
    );
}

// ───────────────── PP6：`clear` 正确口令 ⇒ Ok + reset ─────────────────

/// PP6：用正确口令 `clear` 须返回 `Ok`，且对象被 `reset`。
#[test]
fn PP6_clear_with_correct_password_succeeds_and_resets() {
    let mut p = ProtectionPassword::ProtectionPassword();
    p.enforce("right");
    p.ensureHash();

    let r = p.clear("right");
    assert!(r.is_ok(), "★正确口令 ⇒ Ok（Java 不抛异常）");

    // Java: 成功分支调用 this.reset()
    assert_eq!(p.getKey(), 0, "★成功后必须 reset");
    assert_eq!(p.getHash(), None, "★成功后 hash 清空");
    assert_eq!(p.getSpinCount(), 0, "★成功后 spinCount 清零");
    assert!(p.isEmpty(), "reset 后应回到 empty 状态");
}

// ───────────────── PP7：★错误口令 ⇒ Err 且**不** reset ─────────────────

/// PP7：错误口令必须返回 `Err`（Java 抛 `IncorrectPasswordException`），
/// 且**不**执行 reset（Java 的 else 分支才 reset）⇒ 原字段必须保持。
#[test]
fn PP7_clear_with_wrong_password_errors_without_resetting() {
    let mut p = ProtectionPassword::ProtectionPassword();
    p.enforce("right");
    p.ensureHash();

    let saved_key = p.getKey();
    let saved_hash = p.getHash();

    let r = p.clear("wrong");
    assert!(r.is_err(), "★错误口令 ⇒ Err");

    assert_eq!(p.getKey(), saved_key, "★失败路径不得 reset：key 保持");
    assert_eq!(p.getHash(), saved_hash, "★失败路径不得 reset：hash 保持");
    assert!(!p.isEmpty(), "对象仍应保持非空状态");
}

/// PP7b：`hashValue` 为空时，`clear` 走 Java 的 `key == this.key` 分支。
#[test]
fn PP7b_clear_without_hash_compares_key_only() {
    let mut p = ProtectionPassword::ProtectionPassword();
    p.enforce("right");
    // 故意不调 ensureHash ⇒ hashValue 仍为 null ⇒ 走 key 比较分支

    assert!(p.getHash().is_none(), "前置：hash 为空");
    assert!(p.clear("right").is_ok(), "★hash 为空 ⇒ 仅比 key，应 Ok");

    // 重新 enforce 后用错口令
    let mut p2 = ProtectionPassword::ProtectionPassword();
    p2.enforce("right");
    assert!(p2.clear("wrong").is_err(), "★key 不同 ⇒ Err");
}

// ───────────────── PP8：`ensureHash` 的幂等性与 panic 条件 ─────────────────

/// PP8：`ensureHash` 在 hash 已存在时**不得**覆盖（Java 的 if 守卫）。
#[test]
fn PP8_ensureHash_is_idempotent_when_hash_exists() {
    let mut p = ProtectionPassword::ProtectionPassword();
    p.enforce("pw");
    p.ensureHash();

    let first_hash = p.getHash();
    let first_salt = p.getSalt();

    p.ensureHash(); // 第二次：hash 已存在 ⇒ 直接跳过

    assert_eq!(p.getHash(), first_hash, "★hash 已存在 ⇒ 不得重新生成");
    assert_eq!(p.getSalt(), first_salt, "★salt 不得改变");
}

/// PP8b：`key == 0` 且 `hash` 为空时，`ensureHash` 必须 panic
/// （Java 抛 `IllegalStateException("Key should not be empty")`）。
#[test]
#[should_panic(expected = "Key should not be empty")]
fn PP8b_ensureHash_panics_when_key_empty() {
    let mut p = ProtectionPassword::ProtectionPassword();
    // key=0 且 hash=null ⇒ 进入 key==0 分支 ⇒ panic
    p.ensureHash();
}
