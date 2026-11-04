// Java: import java.security.SecureRandom
// Java: import java.util.UUID
#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
use std::hash::{BuildHasher, RandomState};
use std::sync::atomic::{AtomicU64, Ordering};

// java public final class UUID implements Serializable, Comparable<UUID>
//
// rust: ★M-STAGE2 P1-D1 —— 由空桩做实。
//   Java 的 `UUID.randomUUID()` 语义（JDK）：
//     `return new UUID(randomBytes(16));` —— 16 字节随机数，随后
//     `new UUID(mostSigBits, leastSigBits)` 强制 version=4 与 variant=IETF：
//       mostSigBits  = (mostSigBits  & 0xFFFFFFFFFFFF0FFFL) | 0x0000000000004000L;
//       leastSigBits = (leastSigBits & 0x3FFFFFFFFFFFFFFFL) | 0x8000000000000000L;
//   `toString()` 语义：`digits(mostSigBits>>32,8) + "-" + digits(mostSigBits>>16,4)
//     + "-" + digits(mostSigBits,4) + "-" + digits(leastSigBits>>48,4) + "-"
//     + digits(leastSigBits,12)`（十六进制小写，不足位左补 0）。
//
// rust: 熵源说明 —— 本仓为纯 `std`（无 `rand` / `getrandom` 依赖）。
//   此处用 `RandomState`（由 OS 熵初始化的 SipHash 密钥）+ 单调计数器 + 时间
//   合成 128 位。对 Java 语义中"随机、不可预测"的要求而言分布质量足够，且不引入
//   新依赖；**不**用于密码学用途（Java 侧亦仅用于生成 paraId / 文件名）。
#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct UUID {
    pub(crate) mostSigBits: i64,
    pub(crate) leastSigBits: i64,
}

static COUNTER: AtomicU64 = AtomicU64::new(0);

// rust: `RandomState::new()` 每次调用由 OS 熵派生新密钥；`build_hasher()` 对
//   固定输入即得一个高质量 64 位值。
fn entropy64(salt: u64) -> u64 {
    use std::hash::Hasher;
    let rs = RandomState::new();
    let mut h = rs.build_hasher();
    h.write_u64(salt);
    h.finish()
}

// Java: public class UUID
impl UUID {
    pub fn new() -> Self {
        Self::default()
    }

    // java public static UUID randomUUID()
    pub fn randomUUID() -> Self {
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        // 128 位随机数（掺入计数器与时间以防同刻重复）
        let hi = entropy64(n ^ t);
        let lo = entropy64(hi ^ n.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let a = ((hi as u128) << 64) | (lo as u128);
        let b = ((entropy64(lo) as u128) << 64) | (entropy64((a as u64) ^ n) as u128);
        // Java: long mostSigBits = randomBytes[0..8]; long leastSigBits = randomBytes[8..16];
        let mut mostSigBits = a as i64;
        let mut leastSigBits = b as i64;
        // Java: mostSigBits &= 0xFFFFFFFFFFFF0FFFL; mostSigBits |= 0x0000000000004000L;
        mostSigBits = (mostSigBits & 0xFFFF_FFFF_FFFF_0FFF_u64 as i64) | 0x0000_0000_0000_4000_i64;
        // Java: leastSigBits &= 0x3FFFFFFFFFFFFFFFL; leastSigBits |= 0x8000000000000000L;
        leastSigBits =
            (leastSigBits & 0x3FFF_FFFF_FFFF_FFFF_i64) | (0x8000_0000_0000_0000_u64 as i64);
        Self {
            mostSigBits,
            leastSigBits,
        }
    }

    // java private static String digits(long val, int digits)
    fn digits(val: i64, digits: i32) -> String {
        // Java: long hi = 1L << (digits * 4);
        let hi: i64 = 1_i64 << (digits * 4);
        // Java: return Long.toHexString(hi | (val & (hi - 1))).substring(1);
        let v = (hi | (val & (hi - 1))) as u64;
        format!("{:x}", v).chars().skip(1).collect()
    }

    // java public String toString()
    pub fn toString(&self) -> String {
        format!(
            "{}-{}-{}-{}-{}",
            Self::digits(self.mostSigBits >> 32, 8),
            Self::digits(self.mostSigBits >> 16, 4),
            Self::digits(self.mostSigBits, 4),
            Self::digits(self.leastSigBits >> 48, 4),
            Self::digits(self.leastSigBits, 12)
        )
    }

    // java public long getMostSignificantBits()
    pub fn getMostSignificantBits(&self) -> i64 {
        self.mostSigBits
    }

    // java public long getLeastSignificantBits()
    pub fn getLeastSignificantBits(&self) -> i64 {
        self.leastSigBits
    }

    // java public int version()
    pub fn version(&self) -> i32 {
        ((self.mostSigBits >> 12) & 0x0f) as i32
    }

    // java public int variant()
    pub fn variant(&self) -> i32 {
        // Java: (int) ((leastSigBits >>> (64 - (leastSigBits >>> 62))) & (leastSigBits >> 63));
        let ls = self.leastSigBits as u64;
        let shift = 64u32.wrapping_sub((ls >> 62) as u32);
        ((ls >> shift) & ((self.leastSigBits >> 63) as u64)) as i32
    }
}
