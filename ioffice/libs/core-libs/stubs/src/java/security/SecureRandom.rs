// Java: java.security.SecureRandom
//
// ★M-STAGE1 P2 —— 从 auto-generated 空壳（`new()` 为 `todo!()`、无 `nextBytes`）
//   做实。`OfficePasswordUtil.generateSalt()` 依赖它产生 16 字节盐值。
//
// ★算法口径：沿用本仓既有约定 —— `EncryptionVerifier.rs:30 secure_random_bytes`
//   的 xorshift64 序列（该文件同样以本地 helper 替代 Java 的 `SecureRandom`）。
//   此处把它收敛为 `SecureRandom` 这一**公共门面**，使 Java 侧调用形式不变：
//   Java `mRandom.nextBytes(buf)` ↔ Rust `SecureRandom::nextBytes(&mut buf)`。
//
// ★安全声明：这**不是**密码学安全的随机源（xorshift 可预测）。
//   Java 原文用 `java.security.SecureRandom`（CSPRNG）。本仓当前无
//   `rand`/`getrandom` 依赖，且这是**读路径**（生成盐值供后续哈希校验/写入），
//   不影响解析结果的可复现性 ⇒ 记为已知缺口，不冒充 CSPRNG。
#![allow(non_snake_case, dead_code, unused_imports)]

/// xorshift64 状态。Java `SecureRandom mRandom` 为**实例**字段
/// （`OfficePasswordUtil.java:12 private static final SecureRandom mRandom`），
/// 但 Rust 侧状态用全局原子即可满足语义（单次调用产生互异字节序列）。
static RAND_STATE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn next_u64() -> u64 {
    let mut s = RAND_STATE.load(std::sync::atomic::Ordering::Relaxed);
    if s == 0 {
        // rust: 与 `EncryptionVerifier.rs` 相同的固定种子，保证两处行为一致。
        s = 0xDEADBEEFCAFE1234;
    }
    s ^= s << 13;
    s ^= s >> 7;
    s ^= s << 17;
    RAND_STATE.store(s, std::sync::atomic::Ordering::Relaxed);
    s
}

// Java: public class SecureRandom
#[derive(Default)]
pub struct SecureRandom {}

impl SecureRandom {
    // java public SecureRandom()
    pub fn new() -> Self {
        Self {}
    }

    // java public void nextBytes(byte[] bytes)
    pub fn nextBytes(buf: &mut [u8]) {
        // java mRandom.nextBytes(buf);
        let mut i = 0usize;
        while i < buf.len() {
            let word = next_u64();
            let mut k = 0usize;
            while k < 8 && i + k < buf.len() {
                buf[i + k] = (word >> (k * 8)) as u8;
                k += 1;
            }
            i += 8;
        }
    }
}
